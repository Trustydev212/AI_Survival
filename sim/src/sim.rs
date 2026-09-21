//! One tick of the world: weather, regrow, sense, think, act, contact, metabolise, luck, die.

use crate::agent::*;
use crate::brain::{act_probs, learn_td, trace_step, Action, Genome, N_ACT, N_CRITIC, N_HID, N_IN, N_MEM, N_OUT, N_PLASTIC, N_SIG, N_TRACE};
use crate::config::Config;
use crate::craft::{self, Ing, Process, Slot, M_BONE, NO_ING, N_MAT, N_PROP, N_SLOT};
use crate::world::Building;
use crate::events::EventLog;
use crate::herd::Herds;
use crate::innovation::*;
use crate::orders::{Order, N_ORDER};
use crate::region::RegionGrid;
use crate::store::Stores;
use crate::rng::Rng;
use crate::spatial::SpatialHash;
use crate::stats::Window;
use crate::world::World;

pub struct Sim {
    pub cfg: Config,
    pub rng: Rng,
    pub world: World,
    pub agents: Vec<Agent>,
    pub tick: u64,
    pub window: Window,
    pub events: EventLog,
    pub climate_until: u64,
    /// Every innovation this world has produced, in order of discovery.
    pub innovations: Vec<Innovation>,
    /// Hall of fame: name id -> (peak followers, lineage, tick of peak).
    pub hall: std::collections::HashMap<u32, (u16, u32, u64)>,
    /// Coarse view of the land, so brains can see beyond the cell they stand on.
    pub regions: RegionGrid,
    /// Village storehouses.
    pub stores: Stores,
    /// Big prey that takes several hunters at once.
    pub herds: Herds,
    pub hunts_total: u32,
    /// Seconds spent per phase, for --profile.
    pub profile: [f64; 5],
    spatial: SpatialHash,
    followers: Vec<u16>,
    /// Followers lost this window, by (old leader name, new leader name).
    pub defected: std::collections::HashMap<(u32, u32), u32>,
    learned: Vec<(u32, u128)>,
    /// Recipe signature (process, sorted parts) -> innovation index, so the same thing is never registered twice.
    recipes: std::collections::HashMap<(u8, [Ing; 3]), u16>,
    /// Registry indices freed by forgotten recipes.
    free_slots: Vec<u16>,
    infected: Vec<u32>,
    apprentice: Vec<(u32, [f32; N_SKILL])>,
    imitations: Vec<(u32, u32)>,
    decisions: Vec<Decision>,
    next_lineage: u32,
    next_name: u32,
    next_id: u32,
}

impl Sim {
    pub fn new(cfg: Config, events: EventLog) -> Sim {
        let mut rng = Rng::new(cfg.seed);
        let world = World::generate(
            cfg.width, cfg.height, cfg.max_food, cfg.regrow, cfg.season_len, cfg.farm_boost, cfg.cult_decay,
            cfg.soil_drain, cfg.soil_recovery, &mut rng,
        );
        let spatial = SpatialHash::new(cfg.width as f32, cfg.height as f32, cfg.vision, cfg.wrap);
        let regions = RegionGrid::new(&world, cfg.region_side);
        let mut sim = Sim {
            rng,
            world,
            regions,
            stores: Stores::default(),
            herds: Herds::default(),
            hunts_total: 0,
            profile: [0.0; 5],
            agents: Vec::with_capacity(cfg.agents * 4),
            tick: 0,
            window: Window::default(),
            events,
            climate_until: 0,
            innovations: Vec::new(),
            hall: std::collections::HashMap::new(),
            spatial,
            followers: Vec::new(),
            defected: std::collections::HashMap::new(),
            learned: Vec::new(),
            recipes: std::collections::HashMap::new(),
            free_slots: Vec::new(),
            infected: Vec::new(),
            apprentice: Vec::new(),
            imitations: Vec::new(),
            decisions: Vec::with_capacity(cfg.agents * 4),
            next_lineage: 0,
            next_name: 0,
            next_id: 0,
            cfg,
        };
        sim.spawn_tribes();
        let n_herds = (sim.cfg.herd_density * (sim.cfg.width * sim.cfg.height) as f32 / 10_000.0).round() as usize;
        sim.herds = Herds::spawn(n_herds, &sim.world, &mut sim.rng);
        sim
    }

    /// Founding tribes: each has a home spot on fertile land, a shared marker
    /// colour and a shared ancestral brain with individual variation.
    fn spawn_tribes(&mut self) {
        let tribes = self.cfg.tribes.max(1);
        let per_tribe = self.cfg.agents / tribes;
        for _ in 0..tribes {
            let (hx, hy) = self.fertile_spot();
            let marker = [self.rng.f32(), self.rng.f32(), self.rng.f32()];
            let ancestor = Genome::random(&mut self.rng, marker);
            let lineage = self.new_lineage();
            for _ in 0..per_tribe {
                let (nx, ny) = (self.rng.normal(), self.rng.normal());
                let mut x = self.place(hx + nx * 4.0, self.cfg.width);
                let mut y = self.place(hy + ny * 4.0, self.cfg.height);
                if self.world.is_water(x, y) {
                    x = hx;
                    y = hy;
                }
                let genome = ancestor.mutated(&mut self.rng, 0.5, 0.3);
                let a = self.make_agent(x, y, genome, lineage);
                self.agents.push(a);
            }
        }
    }

    fn fertile_spot(&mut self) -> (f32, f32) {
        for _ in 0..200 {
            let x = self.rng.range(self.cfg.width);
            let y = self.rng.range(self.cfg.height);
            if self.world.fertility[y * self.cfg.width + x] > 0.6 {
                return (x as f32 + 0.5, y as f32 + 0.5);
            }
        }
        (self.cfg.width as f32 / 2.0, self.cfg.height as f32 / 2.0)
    }

    /// Place a coordinate inside the map, wrapping or clamping per config.
    #[inline]
    fn place(&self, v: f32, size: usize) -> f32 {
        Geo { wrap: self.cfg.wrap }.place(v, size)
    }

    /// Shortest signed delta from a to b along one axis.
    #[inline]
    fn delta(&self, a: f32, b: f32, size: usize) -> f32 {
        Geo { wrap: self.cfg.wrap }.delta(a, b, size)
    }

    fn new_lineage(&mut self) -> u32 {
        self.next_lineage += 1;
        self.next_lineage
    }

    fn make_agent(&mut self, x: f32, y: f32, genome: Genome, lineage: u32) -> Agent {
        self.next_id += 1;
        let id = self.next_id;
        let genome_charisma = genome.charisma();
        let mut decay = [0.0; N_EMO];
        let mut sens = [0.0; N_EMO];
        for e in 0..N_EMO {
            decay[e] = genome.emo_decay(e);
            sens[e] = genome.emo_sensitivity(e);
        }
        Agent {
            id,
            x,
            y,
            mdx: 0.0,
            mdy: 0.0,
            energy: self.cfg.start_energy,
            inventory: 0.0,
            age: 0,
            lineage,
            genome,
            attacked_timer: 0,
            last_action: Action::Rest,
            children: 0,
            profile: [0.0; N_PROFILE],
            known: 0,
            caps: [0.0; N_EFFECT],
            mats: [0; N_MAT],
            gear: [Gear::NONE; N_SLOT],
            sheltered: 0.0,
            can_make: false,
            afloat: false,
            still: 0,
            emotion: [0.0; N_EMO],
            memory: [0.0; N_MEM],
            plastic: vec![0.0; N_PLASTIC],
            critic: vec![0.0; N_CRITIC],
            trace: vec![0.0; N_TRACE],
            vtrace: vec![0.0; N_CRITIC],
            v_prev: 0.0,
            td: 0.0,
            last_hidden: [0.0; N_HID],
            last_out: [0.0; N_OUT],
            signal: [0.0; N_SIG],
            heard: [0.0; N_SIG],
            reward: 0.0,
            prev_wealth: 0.0,
            prev_mood: 0.0,
            has_home: false,
            home_x: 0.0,
            home_y: 0.0,
            sick: 0,
            immune: 0,
            skill: [0.0; N_SKILL],
            prestige: 0.0,
            leader: NO_LEADER,
            followers: 0,
            is_leader: false,
            tenure: 0,
            name: 0,
            order: Order::Hold,
            order_dx: 0.0,
            order_dy: 0.0,
            under: None,
            obeyed: false,
            custom: None,
            custom_dx: 0.0,
            custom_dy: 0.0,
            custom_strength: 0.0,
            charisma: genome_charisma,
            emo_decay: decay,
            emo_sens: sens,
        }
    }

    pub fn step(&mut self) {
        let t0 = std::time::Instant::now();
        self.weather();
        self.herds.step(&self.world, &mut self.rng, self.cfg.herd_regrow, self.cfg.wrap);
        let season = self.world.season(self.tick);
        self.world.regrow(season);
        self.world.regrow_mats();
        self.world.age_buildings();
        if self.tick % self.cfg.region_refresh == 0 {
            self.regions.refresh(&self.world, &self.agents, self.cfg.wrap);
        }
        self.tend_stores();
        self.spatial.rebuild(self.agents.iter().map(|a| (a.x, a.y)));
        let t1 = std::time::Instant::now();
        self.sense_and_think(season);
        let t2 = std::time::Instant::now();
        self.act();
        let t3 = std::time::Instant::now();
        self.contact();
        let t4 = std::time::Instant::now();
        self.metabolise_and_die();
        self.immigrate();
        if self.tick % 500 == 250 {
            self.retire_forgotten();
        }
        let t5 = std::time::Instant::now();
        let p = &mut self.profile;
        p[0] += (t1 - t0).as_secs_f64();
        p[1] += (t2 - t1).as_secs_f64();
        p[2] += (t3 - t2).as_secs_f64();
        p[3] += (t4 - t3).as_secs_f64();
        p[4] += (t5 - t4).as_secs_f64();
        self.tick += 1;
    }

    pub const PHASES: [&'static str; 5] = ["world", "sense", "act", "contact", "metabolise"];

    /// Stores spoil a little every tick and are forgotten when long unused and empty.
    fn tend_stores(&mut self) {
        let cfg = &self.cfg;
        let before = self.stores.list.len();
        for s in self.stores.list.iter_mut() {
            s.food *= cfg.store_decay;
            s.idle += 1;
        }
        self.stores.list.retain(|s| !(s.food < 1.0 && s.idle > 2000));
        if self.stores.list.len() != before || self.tick % cfg.region_refresh == 0 {
            self.stores.rebuild_index(&self.regions);
        }
    }

    /// Luck at world scale: once a year the dice decide drought, plenty, hard winters, plague,
    /// and where floods, fires or bounty strike.
    fn weather(&mut self) {
        if self.tick >= self.climate_until {
            self.world.climate = 1.0;
        }
        let year = self.cfg.season_len as u64;
        if self.tick == 0 || self.tick % year != 0 {
            return;
        }
        let roll = self.rng.f32();
        let half = year / 2;
        if roll < self.cfg.p_drought {
            self.world.climate = self.cfg.drought_climate;
            self.climate_until = self.tick + half;
            self.window.droughts += 1;
            self.events.fire(self.tick, "", format!("drought: rains fail for {} ticks", half));
        } else if roll < self.cfg.p_drought + self.cfg.p_golden {
            self.world.climate = self.cfg.golden_climate;
            self.climate_until = self.tick + half;
            self.events.fire(self.tick, "", "a year of plenty: everything grows faster".to_string());
        } else if roll < self.cfg.p_drought + self.cfg.p_golden + self.cfg.p_harsh_winter {
            self.world.climate = 0.5;
            self.climate_until = self.tick + year;
            self.window.harsh_winters += 1;
            self.events.fire(self.tick, "", "harsh year: a long, bitter winter".to_string());
        }
        let r2 = self.rng.f32();
        if r2 < self.cfg.p_flood {
            let (cx, cy) = self.fertile_spot();
            let (cx, cy) = (cx as usize, cy as usize);
            self.world.for_region(cx, cy, 15, |w, i| {
                w.cultivation[i] *= 0.1;
                w.food[i] *= 0.3;
            });
            self.strike_agents(cx as f32, cy as f32, 15.0, 10.0);
            self.window.floods += 1;
            self.events.fire(self.tick, "", format!("flood: fields and stores swept away around ({cx}, {cy})"));
        } else if r2 < self.cfg.p_flood + self.cfg.p_wildfire {
            let (cx, cy) = self.fertile_spot();
            let (cx, cy) = (cx as usize, cy as usize);
            self.world.for_region(cx, cy, 15, |w, i| {
                w.food[i] = 0.0;
                w.cultivation[i] *= 0.5;
                if w.buildings[i].is_some_and(|b| b.flammable) {
                    w.buildings[i] = None;
                }
            });
            self.strike_agents(cx as f32, cy as f32, 15.0, 15.0);
            self.window.wildfires += 1;
            self.events.fire(self.tick, "", format!("wildfire: the land burns around ({cx}, {cy})"));
        } else if r2 < self.cfg.p_flood + self.cfg.p_wildfire + self.cfg.p_bounty {
            let (cx, cy) = self.fertile_spot();
            let (cx, cy) = (cx as usize, cy as usize);
            self.world.for_region(cx, cy, 15, |w, i| {
                w.food[i] = (w.food[i] + 8.0 * w.fertility[i]).min(w.max_food * 3.0);
            });
            self.window.bounties += 1;
            self.events.fire(self.tick, "", format!("bounty: a great herd or harvest appears around ({cx}, {cy})"));
        }
        if self.rng.f32() < self.cfg.p_plague && self.agents.len() > 10 {
            let mut seeded = 0;
            for _ in 0..3 {
                let i = self.rng.range(self.agents.len());
                let a = &mut self.agents[i];
                if a.sick == 0 && a.immune == 0 {
                    a.sick = self.cfg.plague_len;
                    seeded += 1;
                }
            }
            if seeded > 0 {
                self.window.outbreaks += 1;
                self.events.fire(self.tick, "", "plague: a sickness appears".to_string());
            }
        }
    }

    /// Everyone within radius of a disaster loses energy and takes fright.
    fn strike_agents(&mut self, cx: f32, cy: f32, radius: f32, damage: f32) {
        let (w, h) = (self.cfg.width, self.cfg.height);
        let wrap = self.cfg.wrap;
        let d = |a: f32, b: f32, size: usize| {
            let mut d = b - a;
            if wrap {
                let s = size as f32;
                if d > s * 0.5 { d -= s } else if d < -s * 0.5 { d += s }
            }
            d
        };
        for a in self.agents.iter_mut() {
            let dx = d(a.x, cx, w);
            let dy = d(a.y, cy, h);
            if dx * dx + dy * dy <= radius * radius {
                a.energy -= damage;
                a.feel(FEAR, 0.4);
                a.feel(JOY, -0.2);
            }
        }
    }

    fn sense_and_think(&mut self, season: f32) {
        let n = self.agents.len();
        self.decisions.clear();
        self.decisions.resize(n, Decision::default());
        let cfg = &self.cfg;
        let world = &self.world;
        let agents = &self.agents;
        let spatial = &self.spatial;
        let regions = &self.regions;
        let stores = &self.stores;
        let herds = &self.herds;
        let threads = cfg.threads.max(1);
        if threads == 1 || n < 2 * CHUNK {
            for (i, d) in self.decisions.iter_mut().enumerate() {
                *d = decide(i, cfg, world, agents, spatial, regions, stores, herds, season);
            }
            return;
        }
        // Fixed-size chunks handed out to a pool: the result never depends on thread count.
        let next = std::sync::atomic::AtomicUsize::new(0);
        let slots: Vec<std::sync::Mutex<&mut [Decision]>> =
            self.decisions.chunks_mut(CHUNK).map(std::sync::Mutex::new).collect();
        std::thread::scope(|scope| {
            for _ in 0..threads {
                scope.spawn(|| loop {
                    let c = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if c >= slots.len() {
                        break;
                    }
                    let mut slot = slots[c].lock().unwrap();
                    for (k, d) in slot.iter_mut().enumerate() {
                        *d = decide(c * CHUNK + k, cfg, world, agents, spatial, regions, stores, herds, season);
                    }
                });
            }
        });
    }

    /// Who leads whom this tick: a follower's chosen leader gets a follower; enough
    /// followers make a leader, who earns a name after a long enough tenure.
    fn resolve_leaders(&mut self) {
        let n = self.decisions.len();
        self.followers.clear();
        self.followers.resize(n, 0);
        let mut moves: Vec<(usize, u32, u32)> = Vec::new(); // (agent, old leader, new leader)
        for (i, d) in self.decisions.iter().enumerate() {
            let old = self.agents[i].leader;
            if d.leader != NO_LEADER {
                self.followers[d.leader as usize] = self.followers[d.leader as usize].saturating_add(1);
                if old != NO_LEADER && old != d.leader && (old as usize) < n {
                    moves.push((i, old, d.leader));
                }
            }
            self.agents[i].leader = d.leader;
        }
        // Defections and mergers: people change leaders, and whole bands change hands.
        for (i, old, new) in moves {
            let (old_ok, new_ok) = (self.agents[old as usize].is_leader, self.agents[new as usize].is_leader);
            if !(old_ok && new_ok) {
                continue;
            }
            let was_leader = self.agents[i].is_leader && self.agents[i].name != 0;
            let (old_name, new_name) = (self.agents[old as usize].name, self.agents[new as usize].name);
            self.agents[old as usize].prestige -= self.cfg.defection_cost;
            self.agents[new as usize].prestige += self.cfg.defection_cost * 0.5;
            self.window.defections += 1;
            if old_name != 0 && new_name != 0 {
                *self.defected.entry((old_name, new_name)).or_default() += 1;
            }
            if was_leader && new_name != 0 {
                self.window.mergers += 1;
                let me = self.agents[i].name;
                let f = self.agents[i].followers;
                if f >= 10 {
                    let key = format!("merge:{me}:{new_name}");
                    self.events.fire_cooldown(self.tick, &key, 2000, format!("{} and {} followers went over to {}", name_of(me), f, name_of(new_name)));
                }
            }
        }
        let min = self.cfg.leader_min_followers;
        for i in 0..n {
            let f = self.followers[i];
            let a = &mut self.agents[i];
            a.followers = f;
            a.is_leader = f >= min;
            if !a.is_leader {
                a.tenure = 0;
                continue;
            }
            a.tenure = a.tenure.saturating_add(1);
            a.prestige += 0.005 * f as f32;
            if a.name == 0 && a.tenure >= 200 {
                self.next_name += 1;
                a.name = self.next_name;
                let text = format!("{} of lineage {} has led {} kin for 200 ticks", name_of(a.name), a.lineage, f);
                self.events.fire(self.tick, "first_leader", format!("first leader: {text}"));
            }
            if a.name != 0 && f >= self.cfg.store_min_followers && a.still >= self.cfg.settle_ticks {
                let (ax, ay) = (a.x, a.y);
                let has = self.stores.nearest(ax, ay, a, true, self.cfg.kin_threshold, self.cfg.store_range * 2.0, &self.regions, self.cfg.wrap, self.cfg.width, self.cfg.height).is_some();
                if !has {
                    let (name, lineage, marker) = (a.name, a.lineage, a.genome.marker);
                    self.stores.list.push(crate::store::Store { x: ax, y: ay, food: 0.0, owner: name, lineage, marker, idle: 0 });
                    self.stores.rebuild_index(&self.regions);
                    self.window.stores_raised += 1;
                    self.events.fire(self.tick, "first_store", format!("first storehouse: raised by {} of lineage {} at ({:.0}, {:.0})", name_of(name), lineage, ax, ay));
                }
            }
            let a = &mut self.agents[i];
            if a.name != 0 {
                let entry = self.hall.entry(a.name).or_insert((0, a.lineage, self.tick));
                if f > entry.0 {
                    *entry = (f, a.lineage, self.tick);
                    if f >= 40 {
                        let key = format!("great:{}", a.name);
                        self.events.fire(self.tick, &key, format!("great leader: {} of lineage {} now leads {} kin", name_of(a.name), a.lineage, f));
                    }
                }
            }
        }
    }

    /// Discovery chance: what you know, how content you are, and writing-like innovations all compound.
    #[inline]
    fn invent_p(&self, a: &Agent) -> f32 {
        self.cfg.p_discover * (0.5 + 2.0 * a.emotion[JOY]) * (1.0 + a.caps[E_INVENT]).clamp(0.1, 2.5)
    }

    fn act(&mut self) {
        self.resolve_leaders();
        let n = self.agents.len();
        let mut births: Vec<Agent> = Vec::new();
        for i in 0..n {
            let d = self.decisions[i];
            self.window.actions[d.action as usize] += 1;
            // Obedience is a choice, made by the same brain that chose the action.
            // A raid is only a raid when it falls on outsiders; striking kin is not obedience.
            let mut obeyed = match d.under {
                Some(o) => o.obeyed(d.action, d.mx, d.my, d.under_dx, d.under_dy),
                None => false,
            };
            if obeyed && d.under == Some(Order::Raid) {
                obeyed = d.target != u32::MAX
                    && self.agents[i].genome.kinship(&self.agents[d.target as usize].genome) < self.cfg.kin_threshold;
            }
            if let Some(o) = d.under {
                self.window.orders[o as usize] += 1;
                if obeyed {
                    self.window.obeyed += 1;
                } else {
                    self.window.defied += 1;
                }
                if !d.from_leader {
                    self.window.custom_acts += 1;
                }
            }
            // Restraint: could have bred, chose not to.
            if self.agents[i].energy >= self.cfg.repro_threshold {
                self.window.fertile += 1;
                if d.action != Action::Reproduce {
                    self.window.restrained += 1;
                }
            }
            {
                let cfg = &self.cfg;
                let a = &mut self.agents[i];
                a.last_action = d.action;
                a.memory = d.memory;
                a.signal = d.sig;
                a.heard = d.heard;
                a.last_hidden = d.hidden;
                a.last_out = d.out;
                a.prev_wealth = a.energy + a.inventory;
                a.prev_mood = a.emotion[JOY] - a.emotion[FEAR];
                a.order = d.order;
                a.order_dx = d.odx;
                a.order_dy = d.ody;
                a.under = d.under;
                a.obeyed = obeyed;
                a.record(d.action, (d.mx * d.mx + d.my * d.my).sqrt());
                // Actor-critic learning, one step behind: only now, with the next moment priced,
                // can the last choice be judged. Surprise = what came + what comes next is worth
                // - what this was thought to be worth. Then the traces fade and record this tick.
                if cfg.grad_rule {
                    if a.age > 0 {
                        let td = (a.reward + cfg.gamma * d.value - a.v_prev).clamp(-5.0, 5.0);
                        a.td = td;
                        let Agent { plastic, critic, trace, vtrace, .. } = a;
                        learn_td(plastic, critic, trace, vtrace, td, cfg.actor_rate, cfg.critic_rate);
                    }
                    a.v_prev = d.value;
                    let decay = cfg.gamma * cfg.trace_lambda;
                    let Agent { trace, vtrace, .. } = a;
                    trace_step(trace, vtrace, &d.hidden, &d.probs, d.action as usize, decay);
                }
                // Acting together binds people; defying a leader loosens the tie.
                if d.under.is_some() {
                    if obeyed {
                        a.feel(BOND, 0.02);
                    } else {
                        a.feel(BOND, -0.03);
                    }
                }
                // Internalisation: an order obeyed again and again becomes a custom,
                // one that will speak even when no leader is left to give it.
                if let (Some(o), true) = (d.under, d.from_leader) {
                    if obeyed {
                        if a.custom == Some(o) {
                            a.custom_strength = (a.custom_strength + cfg.custom_gain).min(1.0);
                            a.custom_dx = d.under_dx;
                            a.custom_dy = d.under_dy;
                        } else {
                            a.custom_strength -= cfg.custom_gain;
                            if a.custom_strength <= 0.0 {
                                a.custom = Some(o);
                                a.custom_strength = cfg.custom_gain;
                                a.custom_dx = d.under_dx;
                                a.custom_dy = d.under_dy;
                            }
                        }
                    } else if a.custom == Some(o) {
                        a.custom_strength -= 2.0 * cfg.custom_gain;
                    }
                } else if let (Some(o), false, false) = (d.under, d.from_leader, obeyed) {
                    // Breaking with one's own custom weakens it.
                    if a.custom == Some(o) {
                        a.custom_strength -= cfg.custom_gain;
                    }
                }
                a.custom_strength *= cfg.custom_decay;
                if a.custom_strength <= 0.0 {
                    a.custom = None;
                    a.custom_strength = 0.0;
                }
            }
            // A leader whose people ignore it loses standing.
            if !obeyed && d.from_leader && d.leader != NO_LEADER {
                if let Some(l) = self.agents.get_mut(d.leader as usize) {
                    l.prestige -= self.cfg.defiance_cost;
                }
            }

            // Where one stands: is there a roof here?
            {
                let (x, y) = (self.agents[i].x, self.agents[i].y);
                self.agents[i].sheltered = self.world.shelter_at(x, y);
            }
            // Movement (costed in metabolise). Marching together is cheaper than wandering.
            {
                let a = &self.agents[i];
                let (ox, oy) = (a.x, a.y);
                let mut nx = self.place(ox + d.mx * self.cfg.speed, self.cfg.width);
                let mut ny = self.place(oy + d.my * self.cfg.speed, self.cfg.height);
                let (mut mx, mut my) = (d.mx, d.my);
                // The sea is not walkable without boats: slide along the coast, or stop at it.
                let sailor = a.caps[E_SEA] >= self.cfg.sea_threshold;
                if self.world.is_water(nx, ny) && !sailor {
                    if !self.world.is_water(nx, oy) {
                        ny = oy;
                        my = 0.0;
                    } else if !self.world.is_water(ox, ny) {
                        nx = ox;
                        mx = 0.0;
                    } else {
                        nx = ox;
                        ny = oy;
                        mx = 0.0;
                        my = 0.0;
                    }
                }
                let afloat = self.world.is_water(nx, ny);
                let a = &mut self.agents[i];
                a.x = nx;
                a.y = ny;
                a.mdx = mx;
                a.mdy = my;
                let was = a.afloat;
                a.afloat = afloat;
                if afloat && !was {
                    self.window.voyages += 1;
                    let lineage = self.agents[i].lineage;
                    self.events.fire(self.tick, "first_sail", format!("first boat: lineage {} sets out to sea at ({:.0}, {:.0})", lineage, nx, ny));
                }
            }

            match d.action {
                Action::Gather => {
                    let cfg = &self.cfg;
                    let (ax, ay, rate, drain) = {
                        let a = &self.agents[i];
                        let rate = cfg.gather_rate * (1.0 + a.caps[E_GATHER]).max(0.2) * (1.0 + 0.5 * a.skill[SK_GATHER]);
                        (a.x, a.y, rate, (1.0 + a.caps[E_SOIL]).max(0.0))
                    };
                    // Fishing: the sea feeds those who can float on it, and it never wears out.
                    let afloat = self.agents[i].afloat;
                    let take = if afloat {
                        cfg.fish_yield * rate * (1.0 + self.agents[i].caps[E_SEA]).max(0.0)
                    } else {
                        self.world.harvest(ax, ay, rate, drain, cfg.wrap)
                    };
                    // Whatever lies around comes along: a unit of each material with a chance scaled by how much is there.
                    if !afloat {
                        let cell = self.world.idx(ax, ay);
                        let (cap, p_pick) = (cfg.mat_cap, cfg.p_pickup);
                        for k in 0..N_MAT {
                            let here = self.world.mats[cell][k];
                            if here >= 0.1 && self.agents[i].mats[k] < cap && self.rng.f32() < p_pick * here {
                                self.agents[i].mats[k] += 1;
                                self.world.mats[cell][k] -= 0.1;
                            }
                        }
                    }
                    let a = &mut self.agents[i];
                    a.train(SK_GATHER, cfg.skill_gain);
                    a.energy += take;
                    if a.energy > cfg.max_energy {
                        a.inventory = (a.inventory + a.energy - cfg.max_energy).min(inv_cap(cfg, a));
                        a.energy = cfg.max_energy;
                    }
                    self.tend_if_settled(i);
                }
                Action::Attack => {
                    if d.target != u32::MAX {
                        self.resolve_attack(i, d.target as usize);
                    } else {
                        self.hunt(i);
                    }
                }
                Action::Share => {
                    let mut pooled = false;
                    if obeyed && d.under == Some(Order::Pool) {
                        let cfg = &self.cfg;
                        let a = &self.agents[i];
                        if let Some(nb) = self.stores.nearest(a.x, a.y, a, true, cfg.kin_threshold, cfg.store_range, &self.regions, cfg.wrap, cfg.width, cfg.height) {
                            let give = a.inventory.min(cfg.share_amount * cfg.pool_bonus);
                            if give > 0.0 {
                                let st = &mut self.stores.list[nb.idx];
                                let room = (cfg.store_cap - st.food).max(0.0);
                                let put = give.min(room);
                                st.food += put;
                                st.idle = 0;
                                let a = &mut self.agents[i];
                                a.inventory -= put;
                                a.feel(BOND, 0.1);
                                a.prestige += 0.05;
                                self.window.deposits += 1;
                                self.window.deposited += put;
                                pooled = true;
                            }
                        }
                    }
                    if !pooled && d.target != u32::MAX {
                        self.resolve_share(i, d.target as usize);
                    }
                }
                Action::Reproduce => {
                    let cfg = &self.cfg;
                    let room = cfg.max_agents == 0 || n + births.len() < cfg.max_agents;
                    if self.agents[i].energy >= cfg.repro_threshold && room && !self.agents[i].afloat {
                        let child_genome = self.agents[i].genome.mutated(&mut self.rng, cfg.p_mut, cfg.sigma);
                        let (px, py) = (self.agents[i].x, self.agents[i].y);
                        let (nx, ny) = (self.rng.normal(), self.rng.normal());
                        let mut x = self.place(px + nx * 0.8, cfg.width);
                        let mut y = self.place(py + ny * 0.8, cfg.height);
                        if self.world.is_water(x, y) {
                            x = px;
                            y = py;
                        }
                        let a = &mut self.agents[i];
                        a.energy -= cfg.repro_cost;
                        a.children = a.children.saturating_add(1);
                        a.prestige += 0.5;
                        a.feel(JOY, 0.15);
                        let lineage = a.lineage;
                        let child_energy = cfg.child_energy;
                        let mut child = self.make_agent(x, y, child_genome, lineage);
                        child.energy = child_energy;
                        births.push(child);
                        self.window.births += 1;
                    }
                }
                Action::Craft => {
                    if self.cfg.no_crafting {
                        self.tend_if_settled(i); // a control: the urge is there, the hands are not
                    } else {
                        self.agents[i].energy -= self.cfg.craft_cost;
                        self.craft(i);
                    }
                }
                Action::Rest => {
                    self.tend_if_settled(i);
                    // Rationing under a conserve order: organised idleness costs less.
                    if obeyed && d.under == Some(Order::Conserve) {
                        self.agents[i].energy += self.cfg.base_cost * self.cfg.conserve_saving;
                    }
                }
            }

            // Things wear with use. A fire burns down whether or not it is tended.
            {
                let wear = self.cfg.wear;
                let attacked = self.agents[i].attacked_timer > 0;
                let afloat = self.agents[i].afloat;
                let a = &mut self.agents[i];
                let mut changed = false;
                for (slot, g) in a.gear.iter_mut().enumerate() {
                    if !g.is_some() {
                        continue;
                    }
                    let used = match slot {
                        s if s == Slot::Tool as usize => (d.action == Action::Gather && !afloat) as u8 as f32,
                        s if s == Slot::Weapon as usize => (d.action == Action::Attack) as u8 as f32,
                        s if s == Slot::Armour as usize => attacked as u8 as f32,
                        s if s == Slot::Boat as usize => afloat as u8 as f32,
                        s if s == Slot::Vessel as usize => 0.15,
                        _ => 1.0,
                    };
                    g.life -= used * wear;
                    if g.life <= 0.0 {
                        *g = Gear::NONE;
                        changed = true;
                    }
                }
                if changed {
                    refresh_caps(a, &self.innovations);
                }
            }
            if d.action == Action::Craft || (self.tick + self.agents[i].id as u64) % 16 == 0 {
                self.agents[i].can_make = self.makeable(i).is_some();
            }

            // Invention: any work can spark it; what you were doing shapes what you find.
            let p = self.invent_p(&self.agents[i]);
            if (self.innovations.len() < MAX_INNOVATIONS || !self.free_slots.is_empty()) && self.rng.f32() < p {
                self.discover(i, d.action);
            }
        }
        self.resolve_hunts();
        self.agents.extend(births);
    }

    fn tend_if_settled(&mut self, i: usize) {
        let cfg = &self.cfg;
        let a = &mut self.agents[i];
        if a.still >= cfg.settle_ticks {
            // People holding a place together work it better than each alone.
            let together = if a.obeyed && a.under == Some(Order::Hold) { cfg.hold_bonus } else { 1.0 };
            let gain = cfg.cult_gain * together * (1.0 + a.caps[E_FARM]).max(0.0) * (1.0 + a.skill[SK_FARM]);
            a.train(SK_FARM, cfg.skill_gain);
            let (ax, ay) = (a.x, a.y);
            if !a.has_home {
                a.has_home = true;
                a.home_x = ax;
                a.home_y = ay;
            }
            self.world.tend(ax, ay, gain, cfg.wrap);
        }
    }

    /// Strike at the nearest herd within reach. Whether it falls is decided once everyone has acted.
    fn hunt(&mut self, i: usize) {
        let (x, y) = (self.agents[i].x, self.agents[i].y);
        let Some((h, _, _, d2)) = self.herds.nearest(x, y, self.cfg.width as f32, self.cfg.height as f32, self.cfg.wrap) else { return };
        if d2 > self.cfg.hunt_range * self.cfg.hunt_range {
            return;
        }
        let s = self.strength(&self.agents[i], false);
        let herd = &mut self.herds.list[h];
        herd.hits += s;
        herd.struck = true;
        if !herd.hunters.contains(&(i as u32)) {
            herd.hunters.push(i as u32);
        }
        let a = &mut self.agents[i];
        a.energy -= self.cfg.hunt_cost;
        a.train(SK_FIGHT, self.cfg.skill_gain);
    }

    /// Herds struck this tick fall if enough strength landed together; otherwise they flee.
    fn resolve_hunts(&mut self) {
        let cfg_food = self.cfg.herd_food;
        let threshold = self.cfg.hunt_threshold;
        let max_energy = self.cfg.max_energy;
        let respawn = self.cfg.herd_respawn;
        let wrap = self.cfg.wrap;
        for h in 0..self.herds.list.len() {
            if !self.herds.list[h].struck {
                continue;
            }
            let size = self.herds.list[h].size;
            let hands = self.herds.list[h].hunters.len();
            if hands >= self.cfg.hunt_min_hands && self.herds.list[h].hits >= threshold * (0.5 + 0.5 * size) {
                let hunters = std::mem::take(&mut self.herds.list[h].hunters);
                let share = cfg_food * size / hunters.len() as f32;
                let (hx, hy) = (self.herds.list[h].x, self.herds.list[h].y);
                let lineage = self.agents[hunters[0] as usize].lineage;
                for &j in &hunters {
                    let a = &mut self.agents[j as usize];
                    a.energy += share;
                    if a.energy > max_energy {
                        a.inventory = (a.inventory + a.energy - max_energy).min(inv_cap(&self.cfg, a));
                        a.energy = max_energy;
                    }
                    a.feel(JOY, 0.3);
                    a.feel(BOND, 0.1);
                    a.prestige += 0.4;
                }
                let cell = self.world.idx(hx, hy);
                self.world.mats[cell][M_BONE] = (self.world.mats[cell][M_BONE] + 0.6).min(1.0);
                let herd = &mut self.herds.list[h];
                herd.size = 0.0;
                herd.hits = 0.0;
                herd.cooldown = respawn;
                self.window.hunts += 1;
                self.hunts_total += 1;
                self.events.fire(self.tick, "first_hunt", format!("first hunt: {} hunters of lineage {} brought down a herd together at ({:.0}, {:.0})", hunters.len(), lineage, hx, hy));
            } else if self.rng.f32() < 0.15 {
                // Not enough hands: sometimes the herd bolts and the effort is lost.
                self.window.hunt_fails += 1;
                self.herds.list[h].hunters.clear();
                self.herds.list[h].hits = 0.0;
                self.herds.flee(h, &self.world, &mut self.rng, wrap);
            }
        }
    }

    /// Fighting strength: energy, skill, what you know, walls-like defence at home, and mood.
    fn strength(&self, a: &Agent, defending: bool) -> f32 {
        let mut s = a.energy + 20.0 * a.skill[SK_FIGHT] + 30.0 * a.caps[E_ATTACK];
        if defending && a.still >= self.cfg.settle_ticks {
            s += 30.0 * a.caps[E_DEFENSE] + 25.0 * a.sheltered;
        }
        // A raid called and answered: everyone striking on the same word hits harder.
        if !defending && a.obeyed && a.under == Some(Order::Raid) {
            s += self.cfg.raid_bonus;
        }
        s + 15.0 * a.emotion[ANGER] - 15.0 * a.emotion[FEAR]
    }

    fn resolve_attack(&mut self, i: usize, j: usize) {
        let (ax, ay) = (self.agents[i].x, self.agents[i].y);
        let (dx, dy) = (
            self.delta(ax, self.agents[j].x, self.cfg.width),
            self.delta(ay, self.agents[j].y, self.cfg.height),
        );
        if dx * dx + dy * dy > self.cfg.attack_range * self.cfg.attack_range {
            return;
        }
        self.window.attacks += 1;
        {
            let gain = self.cfg.skill_gain * 2.0;
            let a = &mut self.agents[i];
            a.energy -= self.cfg.attack_cost;
            a.train(SK_FIGHT, gain);
        }
        let sa = self.strength(&self.agents[i], false);
        let sd = self.strength(&self.agents[j], true);
        let p_win = 1.0 / (1.0 + (-(sa - sd) / 25.0).exp());
        let cfg = &self.cfg;
        if self.rng.f32() < p_win {
            self.window.attack_wins += 1;
            let att = &self.agents[i];
            let mult = (1.0 + 0.5 * att.caps[E_ATTACK]).max(0.5) * (1.0 + 0.5 * att.emotion[ANGER]);
            let raider = att.caps[E_ATTACK] >= 0.4;
            let stolen = self.agents[j].inventory.min(cfg.steal * mult);
            {
                let v = &mut self.agents[j];
                v.inventory -= stolen;
                v.energy -= cfg.attack_damage * mult;
                v.attacked_timer = 30;
                v.feel(FEAR, 0.5);
                v.feel(ANGER, 0.3);
                v.feel(JOY, -0.3);
            }
            // Well-armed raiders torch the fields of settled victims who cannot defend them.
            let (vx, vy, v_settled, v_walled) = {
                let v = &self.agents[j];
                (v.x, v.y, v.still >= cfg.settle_ticks, v.caps[E_DEFENSE] >= 0.4)
            };
            if raider && v_settled {
                let cell = self.world.idx(vx, vy);
                if let Some(b) = self.world.buildings[cell] {
                    if b.flammable && self.rng.f32() < 0.4 {
                        self.world.buildings[cell] = None;
                        self.window.burned += 1;
                        self.events.fire(self.tick, "first_house_burned", "first house burned by raiders".to_string());
                    }
                }
            }
            if raider && v_settled && !v_walled {
                let lost = self.world.burn(vx, vy, cfg.wrap);
                if lost > 0.5 {
                    self.window.burned += 1;
                    self.events.fire(self.tick, "first_burn", "first fields burned by raiders".to_string());
                }
            }
            // A raider who wins beside a rival storehouse carries some of it off.
            let mut looted = 0.0;
            if raider {
                let att = &self.agents[i];
                if let Some(nb) = self.stores.nearest(vx, vy, att, false, cfg.kin_threshold, cfg.store_range, &self.regions, cfg.wrap, cfg.width, cfg.height) {
                    let st = &mut self.stores.list[nb.idx];
                    looted = st.food.min(cfg.loot);
                    st.food -= looted;
                    st.idle = 0;
                    if looted > 0.0 {
                        self.window.looted += looted;
                        self.events.fire(self.tick, "first_loot", "first storehouse looted by raiders".to_string());
                    }
                }
            }
            let a = &mut self.agents[i];
            a.energy += stolen + cfg.attack_damage * mult * 0.5;
            a.inventory = (a.inventory + looted).min(inv_cap(cfg, a));
            if a.energy > cfg.max_energy {
                a.inventory = (a.inventory + a.energy - cfg.max_energy).min(inv_cap(cfg, a));
                a.energy = cfg.max_energy;
            }
            a.feel(JOY, 0.1);
            a.feel(ANGER, -0.1);
            a.prestige += 0.3;
        } else {
            let a = &mut self.agents[i];
            a.energy -= cfg.attack_damage * 0.5;
            a.attacked_timer = 30;
            a.feel(FEAR, 0.2);
            let v = &mut self.agents[j];
            v.feel(ANGER, 0.1);
        }
    }

    fn resolve_share(&mut self, i: usize, j: usize) {
        let cfg = &self.cfg;
        let (dx, dy) = (
            self.delta(self.agents[i].x, self.agents[j].x, cfg.width),
            self.delta(self.agents[i].y, self.agents[j].y, cfg.height),
        );
        if dx * dx + dy * dy > cfg.share_range * cfg.share_range {
            return;
        }
        if self.agents[i].genome.kinship(&self.agents[j].genome) < cfg.kin_threshold {
            return;
        }
        let g = &self.agents[i];
        let pooled = if g.obeyed && g.under == Some(Order::Pool) { cfg.pool_bonus } else { 1.0 };
        let amount = cfg.share_amount * pooled * (1.0 + g.emotion[BOND]) * (1.0 + g.caps[E_SHARE]).max(0.2);
        let give = g.inventory.min(amount);
        if give <= 0.0 {
            return;
        }
        {
            let g = &mut self.agents[i];
            g.inventory -= give;
            g.feel(BOND, 0.1);
            g.prestige += 0.05;
        }
        // A gift of matter too: one unit of what the giver has plenty of and the other has none.
        let gift = {
            let (g, t) = (&self.agents[i], &self.agents[j]);
            (0..N_MAT).filter(|&m| g.mats[m] >= 2 && t.mats[m] == 0).max_by_key(|&m| g.mats[m])
        };
        if let Some(m) = gift {
            self.agents[i].mats[m] -= 1;
            self.agents[j].mats[m] += 1;
            self.window.mat_gifts += 1;
        }
        let t = &mut self.agents[j];
        t.inventory = (t.inventory + give).min(inv_cap(cfg, t));
        t.feel(JOY, 0.2);
        t.feel(BOND, 0.2);
        self.window.shares += 1;
    }

    /// Everything that passes between neighbours: knowledge, skill, habits and disease.
    fn contact(&mut self) {
        let n = self.decisions.len(); // agents present in the spatial hash this tick
        let all_known: u128 = if self.innovations.len() >= 128 { u128::MAX } else { (1u128 << self.innovations.len()) - 1 };
        self.learned.clear();
        self.infected.clear();
        self.apprentice.clear();
        self.imitations.clear();
        let cfg = &self.cfg;
        let agents = &self.agents;
        let spatial = &self.spatial;
        let seed = cfg.seed ^ self.tick.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let chunks = n.div_ceil(CHUNK);
        let threads = cfg.threads.max(1);
        let mut results: Vec<Vec<(u32, Contact)>> = Vec::with_capacity(chunks);
        if threads == 1 || n < 2 * CHUNK {
            for c in 0..chunks {
                results.push(contact_chunk(c, n, seed, all_known, cfg, agents, spatial));
            }
        } else {
            let slots: Vec<std::sync::Mutex<Option<Vec<(u32, Contact)>>>> = (0..chunks).map(|_| std::sync::Mutex::new(None)).collect();
            let next = std::sync::atomic::AtomicUsize::new(0);
            std::thread::scope(|scope| {
                for _ in 0..threads {
                    scope.spawn(|| loop {
                        let c = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if c >= chunks {
                            break;
                        }
                        let r = contact_chunk(c, n, seed, all_known, cfg, agents, spatial);
                        *slots[c].lock().unwrap() = Some(r);
                    });
                }
            });
            for slot in slots {
                results.push(slot.into_inner().unwrap().unwrap_or_default());
            }
        }
        for (i, r) in results.into_iter().flatten() {
            if r.gained != 0 {
                self.learned.push((i, r.gained));
            }
            if r.caught {
                self.infected.push(i);
            }
            if r.skills.iter().any(|g| *g > 0.0) {
                self.apprentice.push((i, r.skills));
            }
            if r.model != NO_LEADER {
                self.imitations.push((i, r.model));
            }
            if let Some((o, dx, dy)) = r.custom {
                let a = &mut self.agents[i as usize];
                if a.custom == Some(o) {
                    a.custom_strength = (a.custom_strength + 0.05).min(1.0);
                } else {
                    a.custom = Some(o);
                    a.custom_strength = 0.1;
                    a.custom_dx = dx;
                    a.custom_dy = dy;
                }
                self.window.custom_spread += 1;
            }
        }
        for &(i, bits) in &self.learned {
            let a = &mut self.agents[i as usize];
            a.known |= bits;
            refresh_caps(a, &self.innovations);
            self.window.learned += bits.count_ones();
        }
        for &i in &self.infected {
            let a = &mut self.agents[i as usize];
            let resist = (1.0 + a.caps[E_RESIST]).max(0.2);
            a.sick = ((cfg.plague_len as f32 / resist) as u16).max(30);
            self.window.infections += 1;
        }
        for &(i, g) in &self.apprentice {
            let a = &mut self.agents[i as usize];
            for k in 0..N_SKILL {
                a.train(k, g[k]);
            }
        }
        for &(i, j) in &self.imitations {
            let model = self.agents[j as usize].genome.clone();
            self.agents[i as usize].genome.imitate(&model, cfg.imitate_rate);
            // Know-how, not only nature: with --know-rate above zero the learner also takes a
            // step towards what the model has *learned* in its own life. Nothing is inherited
            // by this; it spreads sideways, between the living, and dies with the last holder
            // unless someone else picks it up. That is what makes it culture rather than blood.
            if cfg.know_rate > 0.0 {
                let (plastic, critic) = {
                    let m = &self.agents[j as usize];
                    (m.plastic.clone(), m.critic.clone())
                };
                let a = &mut self.agents[i as usize];
                for (w, m) in a.plastic.iter_mut().zip(plastic.iter()) {
                    *w += (m - *w) * cfg.know_rate;
                }
                for (w, m) in a.critic.iter_mut().zip(critic.iter()) {
                    *w += (m - *w) * cfg.know_rate;
                }
                self.window.know_gifts += 1;
            }
            self.window.imitations += 1;
        }
    }

    fn metabolise_and_die(&mut self) {
        let cfg = &self.cfg;
        let n = self.decisions.len();
        let winter = self.world.season(self.tick) < 0.5;
        let seed = cfg.seed ^ self.tick.wrapping_mul(0xA24B_AED4_963E_E407);
        let total = self.agents.len();
        let threads = cfg.threads.max(1);
        let decisions = &self.decisions;
        let mut hungry_parts: Vec<Vec<u32>> = Vec::new();
        let mut luck_parts: Vec<(u32, u32)> = Vec::new();
        if threads == 1 || total < 2 * CHUNK {
            for (c, slice) in self.agents.chunks_mut(CHUNK).enumerate() {
                let (h, l) = metabolise_chunk(c, slice, n, seed, cfg, decisions, winter);
                hungry_parts.push(h);
                luck_parts.push(l);
            }
        } else {
            let slots: Vec<std::sync::Mutex<(Option<&mut [Agent]>, Vec<u32>, (u32, u32))>> =
                self.agents.chunks_mut(CHUNK).map(|sl| std::sync::Mutex::new((Some(sl), Vec::new(), (0, 0)))).collect();
            let next = std::sync::atomic::AtomicUsize::new(0);
            std::thread::scope(|scope| {
                for _ in 0..threads {
                    scope.spawn(|| loop {
                        let c = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if c >= slots.len() {
                            break;
                        }
                        let mut g = slots[c].lock().unwrap();
                        let slice = g.0.take().unwrap();
                        let (h, l) = metabolise_chunk(c, slice, n, seed, cfg, decisions, winter);
                        g.1 = h;
                        g.2 = l;
                    });
                }
            });
            for slot in slots {
                let (_, h, l) = slot.into_inner().unwrap();
                hungry_parts.push(h);
                luck_parts.push(l);
            }
        }
        let hungry: Vec<u32> = hungry_parts.into_iter().flatten().collect();
        for (w, a) in luck_parts {
            self.window.windfalls += w;
            self.window.accidents += a;
        }
        // The hungry draw on their village's store.
        for &i in &hungry {
            let a = &self.agents[i as usize];
            if let Some(nb) = self.stores.nearest(a.x, a.y, a, true, cfg.kin_threshold, cfg.store_range, &self.regions, cfg.wrap, cfg.width, cfg.height) {
                let st = &mut self.stores.list[nb.idx];
                let take = st.food.min(cfg.eat_amount);
                if take > 0.0 {
                    st.food -= take;
                    st.idle = 0;
                    let a = &mut self.agents[i as usize];
                    a.energy += take;
                    a.feel(BOND, 0.02);
                    self.window.withdrawals += 1;
                    if winter {
                        self.window.winter_withdrawals += 1;
                    }
                }
            }
        }
        // Emotional contagion: followers drift toward what their leader feels.
        for i in 0..n {
            let l = self.agents[i].leader;
            if l == NO_LEADER || l as usize >= self.agents.len() {
                continue;
            }
            let le = self.agents[l as usize].emotion;
            let a = &mut self.agents[i];
            for e in [FEAR, ANGER, BOND] {
                a.emotion[e] += (le[e] - a.emotion[e]) * 0.05;
            }
        }
        let mut fallen: Vec<(u32, u16, u32)> = Vec::new();
        let w = &mut self.window;
        let world = &mut self.world;
        self.agents.retain(|a| {
            if a.name != 0 && a.followers >= 40 && (a.energy <= 0.0 || a.age > cfg.max_age) {
                fallen.push((a.name, a.followers, a.lineage));
            }
            if a.energy <= 0.0 || a.age > cfg.max_age {
                // Bones stay where people fall.
                let cell = world.idx(a.x, a.y);
                world.mats[cell][M_BONE] = (world.mats[cell][M_BONE] + 0.3).min(1.0);
            }
            if a.energy <= 0.0 {
                if a.attacked_timer > 0 {
                    w.killed += 1;
                } else if a.sick > 0 {
                    w.plague_deaths += 1;
                } else {
                    w.starved += 1;
                }
                false
            } else if a.age > cfg.max_age {
                w.aged += 1;
                false
            } else {
                true
            }
        });
        for (name, followers, lineage) in fallen {
            self.window.leader_deaths += 1;
            self.events.fire(self.tick, "", format!("leader {} of lineage {} died, leaving {} followers", name_of(name), lineage, followers));
        }
    }

    /// Optional crutch: with min_pop > 0, fresh random brains drift in after a collapse.
    fn immigrate(&mut self) {
        if self.cfg.min_pop == 0 || self.agents.len() >= self.cfg.min_pop {
            return;
        }
        let missing = self.cfg.min_pop - self.agents.len();
        for _ in 0..missing.min(10) {
            let (x, y) = self.fertile_spot();
            let marker = [self.rng.f32(), self.rng.f32(), self.rng.f32()];
            let genome = Genome::random(&mut self.rng, marker);
            let lineage = self.new_lineage();
            let a = self.make_agent(x, y, genome, lineage);
            self.agents.push(a);
            self.window.immigrants += 1;
        }
    }

    /// A recipe nobody living knows, nobody holds, no building stands on and no other recipe
    /// needs is lost: its place in the registry is freed for something new. Practices are
    /// never retired (they are cheap to keep and shape the era count).
    fn retire_forgotten(&mut self) {
        if self.innovations.len() < MAX_INNOVATIONS / 2 {
            return;
        }
        let mut alive = 0u128;
        for a in &self.agents {
            alive |= a.known;
            for g in a.gear.iter() {
                if g.is_some() {
                    alive |= 1u128 << g.item;
                }
            }
        }
        for b in self.world.buildings.iter().flatten() {
            alive |= 1u128 << b.item;
        }
        for inn in &self.innovations {
            if let Some(c) = &inn.craft {
                for &part in &c.parts[..c.n_parts as usize] {
                    if part as usize >= N_MAT {
                        alive |= 1u128 << (part as usize - N_MAT);
                    }
                }
            }
        }
        let mut freed = Vec::new();
        for (idx, inn) in self.innovations.iter_mut().enumerate() {
            if inn.craft.is_some() && alive & (1u128 << idx) == 0 && !inn.name.is_empty() {
                freed.push(idx);
            }
        }
        for &idx in &freed {
            let inn = &mut self.innovations[idx];
            if let Some(c) = &inn.craft {
                let mut sig = [NO_ING; 3];
                sig[..c.n_parts as usize].copy_from_slice(&c.parts[..c.n_parts as usize]);
                self.recipes.remove(&(c.process as u8, sig));
            }
            inn.name.clear(); // a hole: reused by the next discovery
            self.free_slots.push(idx as u16);
        }
        if !freed.is_empty() {
            self.window.forgotten_recipes += freed.len() as u32;
        }
    }

    /// Where the next innovation goes: a freed hole first, else the end of the registry.
    fn next_slot(&mut self) -> Option<usize> {
        if let Some(idx) = self.free_slots.pop() {
            return Some(idx as usize);
        }
        if self.innovations.len() < MAX_INNOVATIONS {
            Some(self.innovations.len())
        } else {
            None
        }
    }

    /// Properties of an ingredient: a raw material or a made thing.
    fn ing_props(&self, ing: Ing) -> Option<[f32; N_PROP]> {
        let k = ing as usize;
        if k < N_MAT {
            return Some(craft::RAW[k]);
        }
        self.innovations.get(k - N_MAT).and_then(|inn| inn.craft.as_ref().map(|c| c.props))
    }

    /// Raw materials needed to make innovation `idx` from what agent `i` carries, or None
    /// if some part is neither held nor makeable (its recipe unknown).
    fn needs(&self, i: usize, idx: usize, depth: u8) -> Option<[u8; N_MAT]> {
        if depth > 4 {
            return None;
        }
        let c = self.innovations.get(idx)?.craft.as_ref()?;
        let a = &self.agents[i];
        let mut need = [0u8; N_MAT];
        for &part in &c.parts[..c.n_parts as usize] {
            let k = part as usize;
            if k < N_MAT {
                need[k] = need[k].saturating_add(1);
            } else {
                let sub = (k - N_MAT) as u16;
                if a.gear.iter().any(|g| g.item == sub) {
                    continue;
                }
                if a.known & (1u128 << (k - N_MAT)) == 0 {
                    return None;
                }
                let n = self.needs(i, k - N_MAT, depth + 1)?;
                for m in 0..N_MAT {
                    need[m] = need[m].saturating_add(n[m]);
                }
            }
        }
        Some(need)
    }

    fn can_afford(&self, i: usize, idx: usize) -> bool {
        match self.needs(i, idx, 0) {
            Some(need) => (0..N_MAT).all(|m| self.agents[i].mats[m] >= need[m]),
            None => false,
        }
    }

    /// The most useful known thing this agent lacks and could make now.
    fn makeable(&self, i: usize) -> Option<usize> {
        let a = &self.agents[i];
        let (x, y) = (a.x, a.y);
        let mut best: Option<(usize, f32)> = None;
        let mut bits = a.known;
        while bits != 0 {
            let idx = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            let Some(inn) = self.innovations.get(idx) else { continue };
            let Some(c) = inn.craft.as_ref() else { continue };
            if c.slot == Slot::Shelter {
                if a.still < 50 || self.world.is_water(x, y) || self.world.shelter_at(x, y) >= inn.effects[E_SHELTER] {
                    continue;
                }
            } else {
                let g = a.gear[c.slot as usize];
                if g.is_some() {
                    let held = self.innovations[g.item as usize].effects.iter().map(|v| v.abs()).sum::<f32>();
                    let this = inn.effects.iter().map(|v| v.abs()).sum::<f32>();
                    let worn = g.life < 0.25 * self.innovations[g.item as usize].craft.as_ref().map_or(1.0, |c| c.life);
                    if !worn && this <= held {
                        continue;
                    }
                }
            }
            if !self.can_afford(i, idx) {
                continue;
            }
            let score = inn.effects.iter().map(|v| v.abs()).sum::<f32>();
            if best.is_none_or(|b| score > b.1) {
                best = Some((idx, score));
            }
        }
        best.map(|b| b.0)
    }

    /// Work what you carry: make the best known thing you lack, or try something new.
    fn craft(&mut self, i: usize) {
        self.window.craft_tries += 1;
        if let Some(idx) = self.makeable(i) {
            self.make(i, idx);
            return;
        }
        // Experiment. Fire is a process only for those who have a fire in hand.
        let has_fire = self.agents[i].gear[Slot::Fire as usize].is_some();
        let procs: Vec<Process> = Process::ALL.iter().copied().filter(|p| *p != Process::Fire || has_fire).collect();
        let process = procs[self.rng.range(procs.len())];
        let (lo, hi) = process.arity();
        let k = lo + self.rng.range(hi - lo + 1);
        // The pool: raw materials in the bag and things in hand (never the fire itself).
        let mut pool: Vec<(Ing, u8)> = Vec::new();
        {
            let a = &self.agents[i];
            for m in 0..N_MAT {
                if a.mats[m] > 0 {
                    pool.push((m as Ing, a.mats[m]));
                }
            }
            for (s, g) in a.gear.iter().enumerate() {
                if g.is_some() && s != Slot::Fire as usize {
                    pool.push((N_MAT as Ing + g.item, 1));
                }
            }
        }
        if pool.is_empty() {
            return;
        }
        let mut parts: Vec<Ing> = Vec::with_capacity(k);
        for _ in 0..k {
            let (ing, _) = pool[self.rng.range(pool.len())];
            parts.push(ing);
        }
        for &(ing, avail) in &pool {
            if parts.iter().filter(|p| **p == ing).count() as u8 > avail {
                return; // not enough of it
            }
        }
        parts.sort_unstable();
        let mut sig = [NO_ING; 3];
        sig[..parts.len()].copy_from_slice(&parts);
        let key = (process as u8, sig);
        if let Some(&idx) = self.recipes.get(&key) {
            // Someone, somewhere, made this before. An independent rediscovery.
            let idx = idx as usize;
            let a = &mut self.agents[i];
            if a.known & (1u128 << idx) == 0 {
                a.known |= 1u128 << idx;
                refresh_caps(a, &self.innovations);
                self.window.rediscoveries += 1;
            }
            self.make(i, idx);
            return;
        }
        let props: Vec<[f32; N_PROP]> = parts.iter().filter_map(|p| self.ing_props(*p)).collect();
        if props.len() != parts.len() {
            return;
        }
        let Some(made) = craft::compose(process, &props) else { return };
        let (_, _, score) = craft::effects_of(&made, parts.len(), craft::bodies_in(&props));
        if score < self.cfg.craft_min {
            return; // nothing came of it
        }
        // Doing the right thing is not the same as understanding it. Insight is rare, and comes
        // easier to the content and to those who already know how to look.
        let insight = self.cfg.p_insight * (0.5 + 2.0 * self.agents[i].emotion[JOY]) * (1.0 + self.agents[i].caps[E_INVENT]).clamp(0.1, 2.5);
        if self.rng.f32() >= insight {
            return;
        }
        let Some(id) = self.next_slot() else { return };
        let (lineage, x, y) = {
            let a = &self.agents[i];
            (a.lineage, a.x, a.y)
        };
        let mut depth = 1u8;
        let mut cost = [0u8; N_MAT];
        for &part in &parts {
            let pk = part as usize;
            if pk < N_MAT {
                cost[pk] = cost[pk].saturating_add(1);
            } else if let Some(c) = self.innovations[pk - N_MAT].craft.as_ref() {
                depth = depth.max(c.depth + 1);
                for m in 0..N_MAT {
                    cost[m] = cost[m].saturating_add(c.cost[m]);
                }
            }
        }
        let inn = Innovation::crafted(id, process, &parts, &props, made, depth, cost, self.tick, lineage);
        let slot = inn.craft.as_ref().unwrap().slot;
        let text = format!("crafted: {} by lineage {} at ({:.0}, {:.0})", inn.describe(&self.innovations), lineage, x, y);
        let first_key = format!("first_{}", craft::SLOT_NAMES[slot as usize]);
        let first = !self.innovations.iter().any(|o| !o.name.is_empty() && o.craft.as_ref().is_some_and(|c| c.slot == slot));
        if id < self.innovations.len() {
            self.innovations[id] = inn;
        } else {
            self.innovations.push(inn);
        }
        self.recipes.insert(key, id as u16);
        self.window.discoveries += 1;
        self.window.crafts += 1;
        self.events.fire(self.tick, "", text);
        if first {
            let what = match slot {
                Slot::Tool => "a tool: something to dig and cut with",
                Slot::Weapon => "a weapon: something made to hurt",
                Slot::Armour => "a shield: something to hide behind",
                Slot::Boat => "a boat: something that floats and carries",
                Slot::Vessel => "a vessel: something that holds and keeps",
                Slot::Fire => "fire: struck from stone, it cooks and warms",
                Slot::Shelter => "a shelter: walls and a roof",
            };
            self.events.fire(self.tick, &first_key, format!("first {}: lineage {} made {} at ({:.0}, {:.0})", craft::SLOT_NAMES[slot as usize], lineage, what, x, y));
        }
        {
            let a = &mut self.agents[i];
            a.known |= 1u128 << id;
            a.feel(JOY, 0.3);
            a.prestige += 2.0;
        }
        self.make(i, id);
    }

    /// Make innovation `idx` for agent `i`, consuming parts (making sub-parts first) and
    /// putting the thing in its slot, or on the ground as a shelter.
    fn make(&mut self, i: usize, idx: usize) {
        let Some(c) = self.innovations.get(idx).and_then(|inn| inn.craft.clone()) else { return };
        for &part in &c.parts[..c.n_parts as usize] {
            let k = part as usize;
            if k < N_MAT {
                let a = &mut self.agents[i];
                if a.mats[k] == 0 {
                    return;
                }
                a.mats[k] -= 1;
            } else {
                let sub = (k - N_MAT) as u16;
                let a = &mut self.agents[i];
                if let Some(g) = a.gear.iter_mut().find(|g| g.item == sub) {
                    *g = Gear::NONE;
                } else {
                    self.make(i, k - N_MAT);
                    let a = &mut self.agents[i];
                    match a.gear.iter_mut().find(|g| g.item == sub) {
                        Some(g) => *g = Gear::NONE,
                        None => return,
                    }
                }
            }
        }
        self.window.made += 1;
        let effects = self.innovations[idx].effects;
        if c.slot == Slot::Shelter {
            let (x, y) = {
                let a = &self.agents[i];
                (a.x, a.y)
            };
            if self.world.is_water(x, y) || self.agents[i].still < 50 {
                return; // known now, built once they stay somewhere
            }
            let cell = self.world.idx(x, y);
            let shelter = effects[E_SHELTER];
            if self.world.buildings[cell].is_none_or(|b| b.shelter < shelter) {
                self.world.buildings[cell] = Some(Building { item: idx as u16, life: c.life, shelter, flammable: c.props[craft::P_HEAT] < 0.5 });
                self.window.built += 1;
            }
            let a = &mut self.agents[i];
            a.prestige += 0.5;
            a.feel(JOY, 0.1);
            return;
        }
        let a = &mut self.agents[i];
        a.gear[c.slot as usize] = Gear { item: idx as u16, life: c.life };
        a.feel(JOY, 0.05);
        refresh_caps(a, &self.innovations);
    }

    /// A new innovation is born into the world and known first by its discoverer.
    fn discover(&mut self, i: usize, doing: Action) {
        let Some(id) = self.next_slot() else { return };
        let (tier, settled, sick, lineage, x, y) = {
            let a = &self.agents[i];
            ((1 + a.known_count() / 4).min(6) as u8, a.still >= self.cfg.settle_ticks, a.sick > 0, a.lineage, a.x, a.y)
        };
        let Some(inn) = Innovation::practice(&mut self.rng, id, tier, doing, settled, sick, self.tick, lineage) else { return };
        let text = format!("innovation: {} by lineage {} at ({:.0}, {:.0})", inn.describe(&self.innovations), lineage, x, y);
        if id < self.innovations.len() {
            self.innovations[id] = inn;
        } else {
            self.innovations.push(inn);
        }
        let a = &mut self.agents[i];
        a.known |= 1u128 << id;
        refresh_caps(a, &self.innovations);
        a.feel(JOY, 0.3);
        a.prestige += 2.0;
        self.window.discoveries += 1;
        self.events.fire(self.tick, "", text);
    }

    /// What the leaders of this world are currently calling for, weighted by following.
    pub fn order_mix(&self) -> [f32; N_ORDER] {
        let mut mix = [0.0f32; N_ORDER];
        let mut total = 0.0;
        for a in &self.agents {
            if a.is_leader {
                mix[a.order as usize] += a.followers as f32;
                total += a.followers as f32;
            }
        }
        if total > 0.0 {
            for m in mix.iter_mut() {
                *m /= total;
            }
        }
        mix
    }

    /// Share of agents holding each custom firmly enough for it to speak.
    pub fn custom_mix(&self) -> [f32; N_ORDER] {
        let mut mix = [0.0f32; N_ORDER];
        let n = self.agents.len().max(1) as f32;
        for a in &self.agents {
            if let (Some(c), true) = (a.custom, a.custom_strength >= self.cfg.custom_min) {
                mix[c as usize] += 1.0 / n;
            }
        }
        mix
    }

    pub fn settled_share(&self) -> f32 {
        let n = self.agents.len().max(1) as f32;
        self.agents.iter().filter(|a| a.still >= self.cfg.settle_ticks).count() as f32 / n
    }

    pub fn take_window(&mut self) -> Window {
        std::mem::take(&mut self.window)
    }
}

/// Agents are handled in fixed chunks so parallel runs stay deterministic.
const CHUNK: usize = 256;

/// Map geometry shared by the free per-agent functions.
#[derive(Clone, Copy)]
struct Geo {
    wrap: bool,
}

impl Geo {
    #[inline]
    fn place(self, v: f32, size: usize) -> f32 {
        let s = size as f32;
        if self.wrap {
            let r = v.rem_euclid(s);
            if r >= s { 0.0 } else { r }
        } else {
            v.clamp(0.0, s - 0.001)
        }
    }

    #[inline]
    fn delta(self, a: f32, b: f32, size: usize) -> f32 {
        let d = b - a;
        if !self.wrap {
            return d;
        }
        let s = size as f32;
        if d > s * 0.5 {
            d - s
        } else if d < -s * 0.5 {
            d + s
        } else {
            d
        }
    }
}

/// One agent senses its surroundings and its brain decides. Pure: reads the world, writes nothing.
fn decide(
    i: usize, cfg: &Config, world: &World, agents: &[Agent], spatial: &SpatialHash, regions: &RegionGrid,
    stores: &Stores, herds: &Herds, season: f32,
) -> Decision {
    let geo = Geo { wrap: cfg.wrap };
    let vision2 = cfg.vision * cfg.vision;
    let a = &agents[i];
        // Nearest neighbour, crowd composition, and the most prestigious kin in sight.
        let mut best_d2 = f32::MAX;
        let mut nearest = u32::MAX;
        let mut crowd = 0.0f32;
        let mut kin = 0.0f32;
        let mut foe = 0.0f32;
        let mut sick_near = 0.0f32;
        let mut kin_sig = [0.0f32; N_SIG];
        let mut foe_sig = [0.0f32; N_SIG];
        let own_score = a.prestige * (0.5 + a.charisma);
        let mut leader = NO_LEADER;
        let mut leader_score = own_score.max(cfg.leader_min_prestige);
        let prev_leader = a.leader;
        let mut prev_score = 0.0f32;
        let mut seen = 0u32;
        spatial.for_each_near_until(a.x, a.y, |j| {
            if j == i {
                return true;
            }
            let o = &agents[j];
            let dx = geo.delta(a.x, o.x, cfg.width);
            let dy = geo.delta(a.y, o.y, cfg.height);
            let d2 = dx * dx + dy * dy;
            if d2 > vision2 {
                return true;
            }
            seen += 1;
            crowd += 1.0;
            if a.is_kin(o, cfg.kin_threshold) {
                kin += 1.0;
                kin_sig[0] += o.signal[0];
                kin_sig[1] += o.signal[1];
                let score = o.prestige * (0.5 + o.charisma);
                if j as u32 == prev_leader {
                    prev_score = score;
                }
                if score > leader_score {
                    leader_score = score;
                    leader = j as u32;
                }
            } else {
                foe += 1.0;
                foe_sig[0] += o.signal[0];
                foe_sig[1] += o.signal[1];
            }
            if o.sick > 0 {
                sick_near = 1.0;
            }
            if d2 < best_d2 {
                best_d2 = d2;
                nearest = j as u32;
            }
            seen < cfg.max_neighbours
        });
        // Loyalty: keep the current leader unless a rival is clearly better. How much
        // better depends on the follower's attachment, so bands do not flip all at once.
        let margin = 1.1 + 0.5 * a.emotion[BOND];
        if prev_score > 0.0 && prev_score >= own_score.max(cfg.leader_min_prestige) && prev_score * margin >= leader_score {
            leader = prev_leader;
        }

        // Food gradient from four sample points at vision/2.
        let r = cfg.vision * 0.5;
        let here = world.idx(a.x, a.y);
        let sample = |x: f32, y: f32| world.food[world.idx(geo.place(x, cfg.width), geo.place(y, cfg.height))];
        let gx = (sample(a.x + r, a.y) - sample(a.x - r, a.y)) / cfg.max_food;
        let gy = (sample(a.x, a.y + r) - sample(a.x, a.y - r)) / cfg.max_food;

        let mut input = [0.0f32; N_IN];
        input[0] = a.energy / cfg.max_energy;
        input[1] = a.age as f32 / cfg.max_age as f32;
        input[2] = a.inventory / cfg.inv_cap;
        input[3] = world.food[here] / cfg.max_food;
        input[4] = gx;
        input[5] = gy;
        if nearest != u32::MAX {
            let o = &agents[nearest as usize];
            let d = best_d2.sqrt();
            input[6] = 1.0;
            input[7] = geo.delta(a.x, o.x, cfg.width) / cfg.vision;
            input[8] = geo.delta(a.y, o.y, cfg.height) / cfg.vision;
            input[9] = d / cfg.vision;
            input[10] = a.genome.kinship(&o.genome);
            input[11] = (o.energy - a.energy) / cfg.max_energy;
            input[12] = o.inventory / cfg.inv_cap;
        }
        input[13] = (crowd / 10.0).min(2.0);
        input[14] = (kin / 10.0).min(2.0);
        input[15] = (foe / 10.0).min(2.0);
        input[16] = season;
        input[17] = if a.attacked_timer > 0 { 1.0 } else { 0.0 };
        input[18] = world.fertility[here];
        input[19] = 1.0;
        // What I can do, summed from what I know.
        input[20] = a.caps[E_GATHER];
        input[21] = a.caps[E_FARM];
        input[22] = a.caps[E_ATTACK];
        input[23] = world.cultivation[here];
        input[24] = a.emotion[FEAR];
        input[25] = a.emotion[ANGER];
        input[26] = a.emotion[JOY];
        input[27] = a.emotion[BOND];
        input[28..32].copy_from_slice(&a.memory);
        if a.has_home {
            input[32] = 1.0;
            input[33] = (geo.delta(a.x, a.home_x, cfg.width) / cfg.vision).clamp(-2.0, 2.0);
            input[34] = (geo.delta(a.y, a.home_y, cfg.height) / cfg.vision).clamp(-2.0, 2.0);
        }
        input[35] = if a.sick > 0 { 1.0 } else { 0.0 };
        input[36] = sick_near;
        input[37] = a.caps[E_DEFENSE];
        input[38] = a.caps[E_METABOLISM];
        input[39] = (a.known_count() as f32 / 10.0).min(2.0);
        input[40] = world.climate - 1.0;
        if leader != NO_LEADER {
            let l = &agents[leader as usize];
            input[41] = 1.0;
            input[42] = geo.delta(a.x, l.x, cfg.width) / cfg.vision;
            input[43] = geo.delta(a.y, l.y, cfg.height) / cfg.vision;
            input[44] = if l.last_action == Action::Attack { 1.0 } else { 0.0 };
            input[45] = if l.last_action == Action::Share { 1.0 } else { 0.0 };
        }
        input[46] = (a.prestige / 20.0).min(2.0);
        input[47] = a.charisma;
        input[48] = if a.is_leader { 1.0 } else { 0.0 };
        input[49] = a.skill[SK_GATHER];
        input[50] = a.skill[SK_FIGHT];
        input[51] = a.skill[SK_FARM];
        input[52] = (a.followers as f32 / 10.0).min(2.0);
        // The land beyond arm's reach: is this region tired, is anywhere better?
        let r = regions.index(a.x, a.y);
        input[53] = regions.soil[r];
        input[54] = regions.food[r];
        input[55] = regions.crowd[r];
        input[56] = regions.best_dx[r];
        input[57] = regions.best_dy[r];
        input[58] = regions.best_gain[r];
        // What I have been told to do, by my leader or, if I lead, by myself.
        // An order only carries from someone the group already recognises as a leader.
        // With no leader in sight, a custom the agent has internalised speaks instead.
        let (under, under_dx, under_dy, from_leader) = if cfg.no_orders {
            (None, 0.0, 0.0, false)
        } else if leader != NO_LEADER && agents[leader as usize].is_leader {
            let l = &agents[leader as usize];
            (Some(l.order), l.order_dx, l.order_dy, true)
        } else if a.is_leader {
            (Some(a.order), a.order_dx, a.order_dy, true)
        } else if let (Some(c), true, false) = (a.custom, a.custom_strength >= cfg.custom_min, cfg.no_customs) {
            (Some(c), a.custom_dx, a.custom_dy, false)
        } else {
            (None, 0.0, 0.0, false)
        };
        if let Some(o) = under {
            input[59 + o as usize] = 1.0;
            input[64] = under_dx;
            input[65] = under_dy;
        }
    // The village storehouse, if my kin have raised one within reach.
    if let Some(nb) = stores.nearest(a.x, a.y, a, true, cfg.kin_threshold, cfg.store_range, regions, cfg.wrap, cfg.width, cfg.height) {
        input[66] = 1.0;
        input[67] = nb.dx / cfg.vision;
        input[68] = nb.dy / cfg.vision;
        input[69] = (stores.list[nb.idx].food / cfg.store_cap).min(2.0);
    }

    // The sea: how close the coast is in each direction (1 = next cell, 0 = none within reach), and how
    // much of this region is water. Sailing is not invented; the brain only learns where the land ends.
    {
        const REACH: i32 = 4;
        let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];
        for (k, (dx, dy)) in dirs.iter().enumerate() {
            let mut v = 0.0;
            for step in 1..=REACH {
                let x = geo.place(a.x + (*dx * step) as f32, cfg.width);
                let y = geo.place(a.y + (*dy * step) as f32, cfg.height);
                if world.is_water(x, y) {
                    v = 1.0 - (step - 1) as f32 / REACH as f32;
                    break;
                }
            }
            input[70 + k] = v;
        }
        input[74] = regions.water[regions.index(a.x, a.y)];
        input[75] = a.caps[E_SEA];
    }
    // What is in hand: materials, the things held (life left), the roof overhead, and whether a
    // known recipe could be made right now.
    {
        for k in 0..N_MAT {
            input[76 + k] = a.mats[k] as f32 / cfg.mat_cap.max(1) as f32;
        }
        for s in 0..6 {
            let g = a.gear[s];
            input[82 + s] = if g.is_some() { (g.life / 400.0).min(1.5) } else { 0.0 };
        }
        input[88] = a.sheltered.min(1.5);
        input[89] = a.can_make as u8 as f32;
    }
    // How the last tick went, and what others are saying: the average call of kin and of
    // strangers in sight, and the call of the nearest one.
    // Only kin are heard clearly; strangers' calls get through by `hear_strangers` (0 by default).
    let heard = if nearest != u32::MAX {
        let o = &agents[nearest as usize];
        let gain = if a.is_kin(o, cfg.kin_threshold) { 1.0 } else { cfg.hear_strangers };
        [o.signal[0] * gain, o.signal[1] * gain]
    } else {
        [0.0; N_SIG]
    };
    {
        input[90] = a.reward;
        let (nk, nf) = (kin.max(1.0), foe.max(1.0));
        let hs = cfg.hear_scale;
        input[91] = hs * kin_sig[0] / nk;
        input[92] = hs * kin_sig[1] / nk;
        input[93] = hs * cfg.hear_strangers * foe_sig[0] / nf;
        input[94] = hs * cfg.hear_strangers * foe_sig[1] / nf;
        input[95] = hs * heard[0];
        input[96] = hs * heard[1];
    }
    // The nearest herd: is there one, where, and how much is on it.
    if let Some((h, dx, dy, d2)) = herds.nearest(a.x, a.y, cfg.width as f32, cfg.height as f32, cfg.wrap) {
        if d2 <= cfg.vision * cfg.vision * 4.0 {
            input[97] = 1.0;
            input[98] = (dx / cfg.vision).clamp(-2.0, 2.0);
            input[99] = (dy / cfg.vision).clamp(-2.0, 2.0);
            input[100] = herds.list[h].size;
        }
    }

        let mut t = a.genome.think(&input, &a.plastic, &a.critic);
        // A brain that learns from consequences has to take chances: an action never tried
        // teaches nothing. So under the gradient rule the action is drawn from the scores
        // rather than simply taken as the highest. The draw is a hash of who and when, so the
        // world still replays the same on any number of threads.
        let mut probs = [0.0f32; N_ACT];
        if cfg.grad_rule {
            probs = act_probs(&t.out, cfg.policy_temp);
            let mut u = roll(a.id as u64, a.age as u64);
            let mut pick = N_ACT - 1;
            for k in 0..N_ACT {
                if u < probs[k] {
                    pick = k;
                    break;
                }
                u -= probs[k];
            }
            t.action = Action::ALL[pick];
        }
        // Dead zone: a weak movement signal means "stay", so standing still is a stable choice.
        if t.mx * t.mx + t.my * t.my < 0.09 {
            t.mx = 0.0;
            t.my = 0.0;
        }
    Decision {
        mx: t.mx,
        my: t.my,
        action: t.action,
        target: nearest,
        memory: t.memory,
        leader,
        order: t.order,
        odx: t.odx,
        ody: t.ody,
        under,
        under_dx,
        under_dy,
        from_leader,
        sig: t.sig,
        heard,
        hidden: t.hidden,
        out: t.out,
        value: t.value,
        probs,
    }
}

/// What one agent picked up from its neighbours this tick.
#[derive(Clone, Copy, Default)]
struct Contact {
    gained: u128,
    caught: bool,
    skills: [f32; N_SKILL],
    model: u32,
    custom: Option<(Order, f32, f32)>,
}

/// Contacts for one fixed chunk of agents, with a chunk-local RNG so the outcome
/// is the same whatever the thread count.
fn contact_chunk(
    c: usize, n: usize, seed: u64, all_known: u128, cfg: &Config, agents: &[Agent], spatial: &SpatialHash,
) -> Vec<(u32, Contact)> {
    let geo = Geo { wrap: cfg.wrap };
    let range2 = cfg.learn_range * cfg.learn_range;
    let mut rng = Rng::new(seed ^ (c as u64 + 1).wrapping_mul(0xD6E8_FEB8_6659_FD93));
    let mut out = Vec::new();
    for i in c * CHUNK..((c + 1) * CHUNK).min(n) {
        let a = &agents[i];
        let can_learn = a.known != all_known;
        let can_catch = a.sick == 0 && a.immune == 0;
        let wealth = a.energy + a.inventory;
        let resist = (1.0 + a.caps[E_RESIST]).max(0.2);
        let mut gained = 0u128;
        let mut caught = false;
        let mut skills = [0.0f32; N_SKILL];
        let mut model = NO_LEADER;
        let mut custom: Option<(Order, f32, f32)> = None;
        let mut seen = 0u32;
        spatial.for_each_near_until(a.x, a.y, |j| {
            if j == i || j >= n {
                return true;
            }
            let o = &agents[j];
            let missing = if can_learn { o.known & !a.known & !gained } else { 0 };
            let contagious = can_catch && !caught && o.sick > 0;
            let dx = geo.delta(a.x, o.x, cfg.width);
            let dy = geo.delta(a.y, o.y, cfg.height);
            if dx * dx + dy * dy > range2 {
                return true;
            }
            seen += 1;
            let kin = a.is_kin(o, cfg.kin_threshold);
            let is_leader = a.leader == j as u32;
            if kin {
                // Apprenticeship: watching a more skilled relative rubs off.
                for k in 0..N_SKILL {
                    if o.skill[k] > a.skill[k] + 0.1 {
                        skills[k] += cfg.skill_gain * 0.25;
                    }
                }
                // Customs pass between kin: a firmly held way of doing things is catching.
                if o.custom.is_some() && o.custom_strength > a.custom_strength + 0.2 && rng.f32() < cfg.p_learn * (0.5 + a.emotion[BOND]) {
                    custom = Some((o.custom.unwrap(), o.custom_dx, o.custom_dy));
                }
                // Imitation: copy a little of a clearly more successful relative's mind.
                if model == NO_LEADER && o.energy + o.inventory > 1.5 * wealth {
                    let p = if is_leader { cfg.p_imitate * 3.0 } else { cfg.p_imitate };
                    if rng.f32() < p {
                        model = j as u32;
                    }
                }
            }
            if missing == 0 && !contagious {
                return true;
            }
            if missing != 0 {
                let mut p = if kin { cfg.p_learn } else { cfg.p_learn * 0.25 };
                p *= 0.5 + a.emotion[BOND];
                p *= (1.0 + a.caps[E_TEACH] + o.caps[E_TEACH]).max(0.2);
                if is_leader {
                    p *= 2.0;
                }
                let mut bits = missing;
                while bits != 0 {
                    let b = bits & bits.wrapping_neg();
                    bits &= bits - 1;
                    if rng.f32() < p {
                        gained |= b;
                    }
                }
            }
            if contagious && rng.f32() < cfg.p_infect / resist {
                caught = true;
            }
            seen < cfg.max_neighbours
        });
        if gained != 0 || caught || model != NO_LEADER || custom.is_some() || skills.iter().any(|g| *g > 0.0) {
            out.push((i as u32, Contact { gained, caught, skills, model, custom }));
        }
    }
    out
}

/// Per-agent upkeep for one fixed chunk: costs, sickness, luck, mood. Pure except for the
/// chunk-local RNG, so the outcome never depends on the thread count.
fn metabolise_chunk(
    c: usize, slice: &mut [Agent], n: usize, seed: u64, cfg: &Config, decisions: &[Decision], winter: bool,
) -> (Vec<u32>, (u32, u32)) {
    let mut rng = Rng::new(seed ^ (c as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let mut hungry = Vec::new();
    let (mut windfalls, mut accidents) = (0u32, 0u32);
for (k, a) in slice.iter_mut().enumerate() {
    let i = c * CHUNK + k;
        let (moved, resting) = if i < n {
            let d = &decisions[i];
            ((d.mx * d.mx + d.my * d.my).sqrt(), d.action == Action::Rest)
        } else {
            (0.0, false) // newborns this tick
        };
        let mut move_cost = cfg.move_cost;
        if a.obeyed && a.under == Some(Order::Move) {
            move_cost *= 1.0 - cfg.march_saving;
        }
        let mut cost = cfg.base_cost + move_cost * moved;
        if resting {
            cost *= cfg.rest_factor;
        }
        cost *= (1.0 + a.caps[E_METABOLISM]).max(0.3);
        if winter && a.sheltered > 0.0 {
            cost *= 1.0 - cfg.shelter_warmth * a.sheltered.min(1.0);
        }
        // Calling costs: a loud signal is a real expense, so meaning has to earn its keep.
        cost += cfg.sig_cost * (a.signal[0].abs() + a.signal[1].abs());
        if a.afloat {
            cost *= cfg.sea_cost;
            if a.caps[E_SEA] < cfg.sea_threshold {
                cost += cfg.drown_drain; // the boats were forgotten under them
            }
        }
        if a.sick > 0 {
            cost += cfg.sick_drain / (1.0 + a.caps[E_RESIST]).max(0.2);
            a.sick -= 1;
            if a.sick == 0 {
                a.immune = cfg.immune_len;
            }
            a.feel(FEAR, 0.01);
            a.feel(JOY, -0.02);
        } else if a.immune > 0 {
            a.immune -= 1;
        }
        a.energy -= cost;
        if a.energy < cfg.eat_below && a.inventory > 0.0 {
            let eat = a.inventory.min(cfg.eat_amount);
            a.inventory -= eat;
            a.energy += eat;
        } else if a.energy < cfg.eat_below {
            hungry.push(i as u32);
        }

        // Personal luck.
        let roll = rng.f32();
        if roll < cfg.p_windfall {
            a.inventory = (a.inventory + 20.0).min(cfg.inv_cap);
            a.feel(JOY, 0.3);
            windfalls += 1;
        } else if roll < cfg.p_windfall + cfg.p_accident {
            a.energy -= 15.0;
            a.feel(FEAR, 0.2);
            accidents += 1;
        }

        // Slow emotional drift from circumstances, then temperament-driven fading.
        if a.energy < 20.0 {
            a.feel(ANGER, 0.02);
            a.feel(JOY, -0.02);
        } else if a.energy > 70.0 {
            a.feel(JOY, 0.02);
        }
        for e in 0..N_EMO {
            a.emotion[e] *= a.emo_decay[e];
        }
        a.prestige *= cfg.prestige_decay;

        a.age += 1;
        if a.attacked_timer > 0 {
            a.attacked_timer -= 1;
        }
        // How the tick went, and what the brain makes of it.
        if i < n {
            // Reward: fortune gained, and feeling better. What feels good is itself heritable
            // (emotion genes), so evolution shapes what a mind learns from.
            let mood = a.emotion[JOY] - a.emotion[FEAR];
            let r = ((a.energy + a.inventory - a.prev_wealth) / 8.0 + (mood - a.prev_mood) * 2.0).clamp(-1.0, 1.0);
            a.reward = r;
            if !cfg.grad_rule {
                let Agent { genome, plastic, last_hidden, last_out, .. } = a;
                genome.learn(plastic, last_hidden, last_out, r, cfg.learn_scale);
            }
        }
    }
    (hungry, (windfalls, accidents))
}

/// Capabilities: every practice known plus every thing held.
pub fn refresh_caps(a: &mut Agent, registry: &[Innovation]) {
    let mut caps = capabilities(a.known, registry);
    for g in a.gear.iter() {
        if g.is_some() {
            if let Some(inn) = registry.get(g.item as usize) {
                for d in 0..N_EFFECT {
                    caps[d] += inn.effects[d];
                }
            }
        }
    }
    a.caps = caps;
}

/// How much food an agent can carry: the bag, plus whatever vessel it holds.
#[inline]
fn inv_cap(cfg: &Config, a: &Agent) -> f32 {
    cfg.inv_cap * (1.0 + a.caps[E_STORE]).max(0.5)
}


/// A deterministic coin in [0, 1) from who is asking and when. Same answer on any thread count,
/// so a world drawn this way replays exactly.
#[inline]
fn roll(id: u64, age: u64) -> f32 {
    let mut z = id.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ age.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    ((z >> 40) as f32) / 16_777_216.0
}
