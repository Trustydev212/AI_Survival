//! One tick of the world: weather, regrow, sense, think, act, contact, metabolise, luck, die.

use crate::agent::*;
use crate::brain::{Action, Genome, N_IN, N_MEM};
use crate::config::Config;
use crate::events::EventLog;
use crate::innovation::*;
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
    spatial: SpatialHash,
    followers: Vec<u16>,
    learned: Vec<(u32, u64)>,
    infected: Vec<u32>,
    apprentice: Vec<(u32, [f32; N_SKILL])>,
    imitations: Vec<(u32, u32)>,
    decisions: Vec<Decision>,
    next_lineage: u32,
    next_name: u32,
}

impl Sim {
    pub fn new(cfg: Config, events: EventLog) -> Sim {
        let mut rng = Rng::new(cfg.seed);
        let world = World::generate(
            cfg.width, cfg.height, cfg.max_food, cfg.regrow, cfg.season_len, cfg.farm_boost, cfg.cult_decay,
            cfg.soil_drain, cfg.soil_recovery, &mut rng,
        );
        let spatial = SpatialHash::new(cfg.width as f32, cfg.height as f32, cfg.vision, cfg.wrap);
        let mut sim = Sim {
            rng,
            world,
            agents: Vec::with_capacity(cfg.max_agents),
            tick: 0,
            window: Window::default(),
            events,
            climate_until: 0,
            innovations: Vec::new(),
            hall: std::collections::HashMap::new(),
            spatial,
            followers: Vec::new(),
            learned: Vec::new(),
            infected: Vec::new(),
            apprentice: Vec::new(),
            imitations: Vec::new(),
            decisions: Vec::with_capacity(cfg.max_agents),
            next_lineage: 0,
            next_name: 0,
            cfg,
        };
        sim.spawn_tribes();
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
                let x = self.place(hx + nx * 4.0, self.cfg.width);
                let y = self.place(hy + ny * 4.0, self.cfg.height);
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
        let s = size as f32;
        if self.cfg.wrap {
            let r = v.rem_euclid(s);
            if r >= s { 0.0 } else { r }
        } else {
            v.clamp(0.0, s - 0.001)
        }
    }

    /// Shortest signed delta from a to b along one axis.
    #[inline]
    fn delta(&self, a: f32, b: f32, size: usize) -> f32 {
        let d = b - a;
        if !self.cfg.wrap {
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

    fn new_lineage(&mut self) -> u32 {
        self.next_lineage += 1;
        self.next_lineage
    }

    fn make_agent(&self, x: f32, y: f32, genome: Genome, lineage: u32) -> Agent {
        Agent {
            x,
            y,
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
            still: 0,
            emotion: [0.0; N_EMO],
            memory: [0.0; N_MEM],
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
        }
    }

    pub fn step(&mut self) {
        self.weather();
        let season = self.world.season(self.tick);
        self.world.regrow(season);
        self.spatial.rebuild(self.agents.iter().map(|a| (a.x, a.y)));
        self.sense_and_think(season);
        self.act();
        self.contact();
        self.metabolise_and_die();
        self.immigrate();
        self.tick += 1;
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
        let cfg = &self.cfg;
        let world = &self.world;
        let agents = &self.agents;
        let vision2 = cfg.vision * cfg.vision;
        self.decisions.clear();
        for (i, a) in agents.iter().enumerate() {
            // Nearest neighbour, crowd composition, and the most prestigious kin in sight.
            let mut best_d2 = f32::MAX;
            let mut nearest = u32::MAX;
            let mut crowd = 0.0f32;
            let mut kin = 0.0f32;
            let mut foe = 0.0f32;
            let mut sick_near = 0.0f32;
            let own_score = a.prestige * (0.5 + a.genome.charisma());
            let mut leader = NO_LEADER;
            let mut leader_score = own_score.max(cfg.leader_min_prestige);
            let prev_leader = a.leader;
            let mut prev_score = 0.0f32;
            self.spatial.for_each_near(a.x, a.y, |j| {
                if j == i {
                    return;
                }
                let o = &agents[j];
                let dx = self.delta(a.x, o.x, cfg.width);
                let dy = self.delta(a.y, o.y, cfg.height);
                let d2 = dx * dx + dy * dy;
                if d2 > vision2 {
                    return;
                }
                crowd += 1.0;
                if a.genome.kinship(&o.genome) >= cfg.kin_threshold {
                    kin += 1.0;
                    let score = o.prestige * (0.5 + o.genome.charisma());
                    if j as u32 == prev_leader {
                        prev_score = score;
                    }
                    if score > leader_score {
                        leader_score = score;
                        leader = j as u32;
                    }
                } else {
                    foe += 1.0;
                }
                if o.sick > 0 {
                    sick_near = 1.0;
                }
                if d2 < best_d2 {
                    best_d2 = d2;
                    nearest = j as u32;
                }
            });
            // Loyalty: keep the current leader unless a rival is clearly better.
            if prev_score > 0.0 && prev_score >= own_score.max(cfg.leader_min_prestige) && prev_score * 1.25 >= leader_score {
                leader = prev_leader;
            }

            // Food gradient from four sample points at vision/2.
            let r = cfg.vision * 0.5;
            let here = world.idx(a.x, a.y);
            let sample = |x: f32, y: f32| world.food[world.idx(self.place(x, cfg.width), self.place(y, cfg.height))];
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
                input[7] = self.delta(a.x, o.x, cfg.width) / cfg.vision;
                input[8] = self.delta(a.y, o.y, cfg.height) / cfg.vision;
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
                input[33] = (self.delta(a.x, a.home_x, cfg.width) / cfg.vision).clamp(-2.0, 2.0);
                input[34] = (self.delta(a.y, a.home_y, cfg.height) / cfg.vision).clamp(-2.0, 2.0);
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
                input[42] = self.delta(a.x, l.x, cfg.width) / cfg.vision;
                input[43] = self.delta(a.y, l.y, cfg.height) / cfg.vision;
                input[44] = if l.last_action == Action::Attack { 1.0 } else { 0.0 };
                input[45] = if l.last_action == Action::Share { 1.0 } else { 0.0 };
            }
            input[46] = (a.prestige / 20.0).min(2.0);
            input[47] = a.genome.charisma();
            input[48] = if a.is_leader { 1.0 } else { 0.0 };
            input[49] = a.skill[SK_GATHER];
            input[50] = a.skill[SK_FIGHT];
            input[51] = a.skill[SK_FARM];
            input[52] = (a.followers as f32 / 10.0).min(2.0);

            let (mut mx, mut my, action, memory) = a.genome.think(&input);
            // Dead zone: a weak movement signal means "stay", so standing still is a stable choice.
            if mx * mx + my * my < 0.09 {
                mx = 0.0;
                my = 0.0;
            }
            self.decisions.push(Decision { mx, my, action, target: nearest, memory, leader });
        }
    }

    /// Who leads whom this tick: a follower's chosen leader gets a follower; enough
    /// followers make a leader, who earns a name after a long enough tenure.
    fn resolve_leaders(&mut self) {
        let n = self.decisions.len();
        self.followers.clear();
        self.followers.resize(n, 0);
        for (i, d) in self.decisions.iter().enumerate() {
            self.agents[i].leader = d.leader;
            if d.leader != NO_LEADER {
                self.followers[d.leader as usize] = self.followers[d.leader as usize].saturating_add(1);
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
            {
                let a = &mut self.agents[i];
                a.last_action = d.action;
                a.memory = d.memory;
                a.record(d.action, (d.mx * d.mx + d.my * d.my).sqrt());
            }

            // Movement (costed in metabolise).
            {
                let a = &self.agents[i];
                let nx = self.place(a.x + d.mx * self.cfg.speed, self.cfg.width);
                let ny = self.place(a.y + d.my * self.cfg.speed, self.cfg.height);
                let a = &mut self.agents[i];
                a.x = nx;
                a.y = ny;
            }

            match d.action {
                Action::Gather => {
                    let cfg = &self.cfg;
                    let (ax, ay, rate, drain) = {
                        let a = &self.agents[i];
                        let rate = cfg.gather_rate * (1.0 + a.caps[E_GATHER]).max(0.2) * (1.0 + 0.5 * a.skill[SK_GATHER]);
                        (a.x, a.y, rate, (1.0 + a.caps[E_SOIL]).max(0.0))
                    };
                    let take = self.world.harvest(ax, ay, rate, drain, cfg.wrap);
                    let a = &mut self.agents[i];
                    a.train(SK_GATHER, cfg.skill_gain);
                    a.energy += take;
                    if a.energy > cfg.max_energy {
                        a.inventory = (a.inventory + a.energy - cfg.max_energy).min(cfg.inv_cap);
                        a.energy = cfg.max_energy;
                    }
                    self.tend_if_settled(i);
                }
                Action::Attack => {
                    if d.target != u32::MAX {
                        self.resolve_attack(i, d.target as usize);
                    }
                }
                Action::Share => {
                    if d.target != u32::MAX {
                        self.resolve_share(i, d.target as usize);
                    }
                }
                Action::Reproduce => {
                    let cfg = &self.cfg;
                    if self.agents[i].energy >= cfg.repro_threshold && n + births.len() < cfg.max_agents {
                        let child_genome = self.agents[i].genome.mutated(&mut self.rng, cfg.p_mut, cfg.sigma);
                        let (px, py) = (self.agents[i].x, self.agents[i].y);
                        let (nx, ny) = (self.rng.normal(), self.rng.normal());
                        let x = self.place(px + nx * 0.8, cfg.width);
                        let y = self.place(py + ny * 0.8, cfg.height);
                        let a = &mut self.agents[i];
                        a.energy -= cfg.repro_cost;
                        a.children = a.children.saturating_add(1);
                        a.prestige += 0.5;
                        let lineage = a.lineage;
                        let mut child = self.make_agent(x, y, child_genome, lineage);
                        child.energy = cfg.child_energy;
                        births.push(child);
                        self.window.births += 1;
                    }
                }
                Action::Rest => {
                    self.tend_if_settled(i);
                }
            }

            // Invention: any work can spark it; what you were doing shapes what you find.
            let p = self.invent_p(&self.agents[i]);
            if self.innovations.len() < MAX_INNOVATIONS && self.rng.f32() < p {
                self.discover(i, d.action);
            }
        }
        self.agents.extend(births);
    }

    fn tend_if_settled(&mut self, i: usize) {
        let cfg = &self.cfg;
        let a = &mut self.agents[i];
        if a.still >= cfg.settle_ticks {
            let gain = cfg.cult_gain * (1.0 + a.caps[E_FARM]).max(0.0) * (1.0 + a.skill[SK_FARM]);
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

    /// Fighting strength: energy, skill, what you know, walls-like defence at home, and mood.
    fn strength(&self, a: &Agent, defending: bool) -> f32 {
        let mut s = a.energy + 20.0 * a.skill[SK_FIGHT] + 30.0 * a.caps[E_ATTACK];
        if defending && a.still >= self.cfg.settle_ticks {
            s += 30.0 * a.caps[E_DEFENSE];
        }
        // Fighting beside a leader who just charged: coordinated assault.
        if !defending && a.leader != NO_LEADER {
            if let Some(l) = self.agents.get(a.leader as usize) {
                if l.last_action == Action::Attack {
                    s += 10.0;
                }
            }
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
            if raider && v_settled && !v_walled {
                let lost = self.world.burn(vx, vy, cfg.wrap);
                if lost > 0.5 {
                    self.window.burned += 1;
                    self.events.fire(self.tick, "first_burn", "first fields burned by raiders".to_string());
                }
            }
            let a = &mut self.agents[i];
            a.energy += stolen + cfg.attack_damage * mult * 0.5;
            if a.energy > cfg.max_energy {
                a.inventory = (a.inventory + a.energy - cfg.max_energy).min(cfg.inv_cap);
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
        let amount = cfg.share_amount * (1.0 + g.emotion[BOND]) * (1.0 + g.caps[E_SHARE]).max(0.2);
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
        let t = &mut self.agents[j];
        t.inventory = (t.inventory + give).min(cfg.inv_cap);
        t.feel(JOY, 0.2);
        t.feel(BOND, 0.2);
        self.window.shares += 1;
    }

    /// Everything that passes between neighbours: knowledge, skill, habits and disease.
    fn contact(&mut self) {
        let cfg = &self.cfg;
        let n = self.decisions.len(); // agents present in the spatial hash this tick
        let range2 = cfg.learn_range * cfg.learn_range;
        let all_known: u64 = if self.innovations.len() >= 64 { u64::MAX } else { (1u64 << self.innovations.len()) - 1 };
        self.learned.clear();
        self.infected.clear();
        self.apprentice.clear();
        self.imitations.clear();
        let mut rng = self.rng.clone();
        for i in 0..n {
            let a = &self.agents[i];
            let can_learn = a.known != all_known;
            let can_catch = a.sick == 0 && a.immune == 0;
            let wealth = a.energy + a.inventory;
            let resist = (1.0 + a.caps[E_RESIST]).max(0.2);
            let mut gained = 0u64;
            let mut caught = false;
            let mut skills = [0.0f32; N_SKILL];
            let mut model = NO_LEADER;
            let agents = &self.agents;
            self.spatial.for_each_near(a.x, a.y, |j| {
                if j == i || j >= n {
                    return;
                }
                let o = &agents[j];
                let missing = if can_learn { o.known & !a.known & !gained } else { 0 };
                let contagious = can_catch && !caught && o.sick > 0;
                let dx = self.delta(a.x, o.x, cfg.width);
                let dy = self.delta(a.y, o.y, cfg.height);
                if dx * dx + dy * dy > range2 {
                    return;
                }
                let kin = a.genome.kinship(&o.genome) >= cfg.kin_threshold;
                let is_leader = a.leader == j as u32;
                if kin {
                    // Apprenticeship: watching a more skilled relative rubs off.
                    for k in 0..N_SKILL {
                        if o.skill[k] > a.skill[k] + 0.1 {
                            skills[k] += cfg.skill_gain * 0.25;
                        }
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
                    return;
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
            });
            if gained != 0 {
                self.learned.push((i as u32, gained));
            }
            if caught {
                self.infected.push(i as u32);
            }
            if skills.iter().any(|g| *g > 0.0) {
                self.apprentice.push((i as u32, skills));
            }
            if model != NO_LEADER {
                self.imitations.push((i as u32, model));
            }
        }
        self.rng = rng;
        for &(i, bits) in &self.learned {
            let a = &mut self.agents[i as usize];
            a.known |= bits;
            a.caps = capabilities(a.known, &self.innovations);
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
            self.window.imitations += 1;
        }
    }

    fn metabolise_and_die(&mut self) {
        let cfg = &self.cfg;
        let n = self.decisions.len();
        let mut rng = self.rng.clone();
        for (i, a) in self.agents.iter_mut().enumerate() {
            let (moved, resting) = if i < n {
                let d = &self.decisions[i];
                ((d.mx * d.mx + d.my * d.my).sqrt(), d.action == Action::Rest)
            } else {
                (0.0, false) // newborns this tick
            };
            let mut cost = cfg.base_cost + cfg.move_cost * moved;
            if resting {
                cost *= cfg.rest_factor;
            }
            cost *= (1.0 + a.caps[E_METABOLISM]).max(0.3);
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
            }

            // Personal luck.
            let roll = rng.f32();
            if roll < cfg.p_windfall {
                a.inventory = (a.inventory + 20.0).min(cfg.inv_cap);
                a.feel(JOY, 0.3);
                self.window.windfalls += 1;
            } else if roll < cfg.p_windfall + cfg.p_accident {
                a.energy -= 15.0;
                a.feel(FEAR, 0.2);
                self.window.accidents += 1;
            }

            // Slow emotional drift from circumstances, then temperament-driven fading.
            if a.energy < 20.0 {
                a.feel(ANGER, 0.02);
                a.feel(JOY, -0.02);
            } else if a.energy > 70.0 {
                a.feel(JOY, 0.02);
            }
            for e in 0..N_EMO {
                a.emotion[e] *= a.genome.emo_decay(e);
            }
            a.prestige *= cfg.prestige_decay;

            a.age += 1;
            if a.attacked_timer > 0 {
                a.attacked_timer -= 1;
            }
        }
        self.rng = rng;
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
        self.agents.retain(|a| {
            if a.name != 0 && a.followers >= 40 && (a.energy <= 0.0 || a.age > cfg.max_age) {
                fallen.push((a.name, a.followers, a.lineage));
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

    /// A new innovation is born into the world and known first by its discoverer.
    fn discover(&mut self, i: usize, doing: Action) {
        let id = self.innovations.len();
        let (tier, settled, sick, lineage, x, y) = {
            let a = &self.agents[i];
            ((1 + a.known_count() / 4).min(6) as u8, a.still >= self.cfg.settle_ticks, a.sick > 0, a.lineage, a.x, a.y)
        };
        let inn = Innovation::generate(&mut self.rng, id, tier, doing, settled, sick, self.tick, lineage);
        let text = format!("innovation: {} by lineage {} at ({:.0}, {:.0})", inn.describe(), lineage, x, y);
        self.innovations.push(inn);
        let a = &mut self.agents[i];
        a.known |= 1u64 << id;
        a.caps = capabilities(a.known, &self.innovations);
        a.feel(JOY, 0.3);
        a.prestige += 2.0;
        self.window.discoveries += 1;
        self.events.fire(self.tick, "", text);
    }

    pub fn settled_share(&self) -> f32 {
        let n = self.agents.len().max(1) as f32;
        self.agents.iter().filter(|a| a.still >= self.cfg.settle_ticks).count() as f32 / n
    }

    pub fn take_window(&mut self) -> Window {
        std::mem::take(&mut self.window)
    }
}
