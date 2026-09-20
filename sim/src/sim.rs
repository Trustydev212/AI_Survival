//! One tick of the world: weather, regrow, sense, think, act, contact, metabolise, luck, die.

use crate::agent::*;
use crate::brain::{Action, Genome, N_IN, N_MEM};
use crate::config::Config;
use crate::events::EventLog;
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
    /// Hall of fame: name id -> (peak followers, lineage, tick of peak).
    pub hall: std::collections::HashMap<u32, (u16, u32, u64)>,
    spatial: SpatialHash,
    followers: Vec<u16>,
    apprentice: Vec<(u32, [f32; N_SKILL])>,
    imitations: Vec<(u32, u32)>,
    next_name: u32,
    learned: Vec<(u32, u16)>,
    infected: Vec<u32>,
    decisions: Vec<Decision>,
    next_lineage: u32,
}

impl Sim {
    pub fn new(cfg: Config, events: EventLog) -> Sim {
        let mut rng = Rng::new(cfg.seed);
        let world = World::generate(
            cfg.width, cfg.height, cfg.max_food, cfg.regrow, cfg.season_len, cfg.farm_boost, cfg.cult_decay, &mut rng,
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
            hall: std::collections::HashMap::new(),
            spatial,
            followers: Vec::new(),
            apprentice: Vec::new(),
            imitations: Vec::new(),
            next_name: 0,
            learned: Vec::new(),
            infected: Vec::new(),
            decisions: Vec::with_capacity(cfg.max_agents),
            next_lineage: 0,
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
                let mut a = self.make_agent(x, y, genome, lineage);
                a.tech = self.cfg.start_tech;
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
            tech: 0,
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

    /// Luck at world scale: once a year the dice decide drought, plenty or plague.
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
        }
        else if roll < self.cfg.p_drought + self.cfg.p_golden + self.cfg.p_harsh_winter {
            self.world.climate = 0.5;
            self.climate_until = self.tick + year;
            self.window.harsh_winters += 1;
            self.events.fire(self.tick, "", "harsh year: a long, bitter winter".to_string());
        }
        // Regional luck: floods and wildfires strike a place, bounty blesses one.
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
            // Nearest neighbour and local crowd composition.
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
            input[20] = if a.knows(TOOLS) { 1.0 } else { 0.0 };
            input[21] = if a.knows(FARMING) { 1.0 } else { 0.0 };
            input[22] = if a.knows(WEAPONS) { 1.0 } else { 0.0 };
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
            input[37] = if a.knows(METAL) { 1.0 } else { 0.0 };
            input[38] = if a.knows(WALLS) { 1.0 } else { 0.0 };
            input[39] = if a.knows(WRITING) { 1.0 } else { 0.0 };
            // Loyalty: keep the current leader unless a rival is clearly better.
            if prev_score > 0.0 && prev_score >= own_score.max(cfg.leader_min_prestige) && prev_score * 1.25 >= leader_score {
                leader = prev_leader;
            }
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

    /// Discovery chance scaled by mood: fed, content minds invent; writing compounds it.
    #[inline]
    fn invent_p(&self, a: &Agent, base: f32) -> f32 {
        let mut p = base * (0.5 + 2.0 * a.emotion[JOY]);
        if a.knows(WRITING) {
            p *= 2.0;
        }
        p
    }

    /// Who leads whom this tick: a follower's chosen leader gets a follower; enough
    /// followers make a leader, who is named the first time it happens.
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
                    let cell = self.world.idx(self.agents[i].x, self.agents[i].y);
                    let mut rate = cfg.gather_rate;
                    if self.agents[i].knows(TOOLS) {
                        rate *= cfg.tools_gather_mult;
                    }
                    if self.agents[i].knows(METAL) {
                        rate *= cfg.metal_mult;
                    }
                    rate *= 1.0 + 0.5 * self.agents[i].skill[SK_GATHER];
                    let take = self.world.harvest(self.agents[i].x, self.agents[i].y, rate, cfg.wrap);
                    let a = &mut self.agents[i];
                    a.train(SK_GATHER, cfg.skill_gain);
                    a.energy += take;
                    if a.energy > cfg.max_energy {
                        a.inventory = (a.inventory + a.energy - cfg.max_energy).min(cfg.inv_cap);
                        a.energy = cfg.max_energy;
                    }
                    let p_discover = cfg.p_discover;
                    self.tend_if_settled(i);
                    // Discovery while working: tools from bare hands, farming from tools on good land,
                    // irrigation from long years on the same field.
                    let a = &self.agents[i];
                    let fert = self.world.fertility[cell];
                    let p = self.invent_p(a, p_discover);
                    if !a.knows(TOOLS) && self.rng.f32() < p {
                        self.discover(i, 0);
                    } else if a.knows(TOOLS) && !a.knows(FARMING) && fert > 0.5 && self.rng.f32() < p * 0.5 {
                        self.discover(i, 1);
                    } else if a.knows(FARMING) && !a.knows(IRRIGATION) && a.still >= 200 && self.rng.f32() < p * 0.5 {
                        self.discover(i, 5);
                    }
                }
                Action::Attack => {
                    if d.target != u32::MAX {
                        self.resolve_attack(i, d.target as usize);
                        let a = &self.agents[i];
                        let p = self.invent_p(a, self.cfg.p_discover);
                        if a.knows(TOOLS) && !a.knows(WEAPONS) && self.rng.f32() < p * 20.0 {
                            self.discover(i, 2);
                        }
                    }
                }
                Action::Share => {
                    if d.target != u32::MAX {
                        self.resolve_share(i, d.target as usize);
                        // Writing is born where settled kin exchange a lot.
                        let a = &self.agents[i];
                        let p = self.invent_p(a, self.cfg.p_discover);
                        if a.knows(FARMING) && !a.knows(WRITING) && a.still >= self.cfg.settle_ticks && a.emotion[BOND] > 0.5
                            && self.rng.f32() < p * 5.0
                        {
                            self.discover(i, 8);
                        }
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
                    let cfg = &self.cfg;
                    let a = &self.agents[i];
                    let p = self.invent_p(a, cfg.p_discover);
                    if a.knows(TOOLS) && !a.knows(COOKING) && self.rng.f32() < p * 0.5 {
                        self.discover(i, 3);
                    } else if a.knows(TOOLS) && a.knows(COOKING) && !a.knows(METAL) && a.still >= cfg.settle_ticks
                        && self.rng.f32() < p * 0.5
                    {
                        self.discover(i, 4);
                    } else if a.knows(COOKING) && !a.knows(MEDICINE) && a.sick > 0 && self.rng.f32() < p * 10.0 {
                        self.discover(i, 7);
                    }
                }
            }
        }
        self.agents.extend(births);
    }

    fn tend_if_settled(&mut self, i: usize) {
        let cfg = &self.cfg;
        let a = &mut self.agents[i];
        if a.knows(FARMING) && a.still >= cfg.settle_ticks {
            let irrigate = a.knows(IRRIGATION);
            let mut gain = if irrigate { cfg.cult_gain * 2.0 } else { cfg.cult_gain };
            gain *= 1.0 + a.skill[SK_FARM];
            a.train(SK_FARM, cfg.skill_gain);
            let (ax, ay) = (a.x, a.y);
            if !a.has_home {
                a.has_home = true;
                a.home_x = ax;
                a.home_y = ay;
            }
            self.world.tend(ax, ay, gain, irrigate, cfg.wrap);
        }
    }

    /// Fighting strength: energy, arms, walls when defending at home, and mood.
    fn strength(&self, a: &Agent, defending: bool) -> f32 {
        let mut s = a.energy + 20.0 * a.skill[SK_FIGHT];
        // Fighting beside a leader who just charged: coordinated assault.
        if !defending && a.leader != NO_LEADER {
            if let Some(l) = self.agents.get(a.leader as usize) {
                if l.last_action == Action::Attack {
                    s += 10.0;
                }
            }
        }
        if a.knows(WEAPONS) {
            s += 20.0;
        }
        if a.knows(METAL) {
            s += 15.0;
        }
        if defending && a.knows(WALLS) && a.still >= self.cfg.settle_ticks {
            s += 25.0;
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
        // Walls: a settled, armed farmer under siege may think of building them.
        {
            let d = &self.agents[j];
            if d.knows(FARMING) && d.knows(WEAPONS) && !d.knows(WALLS) && d.still >= self.cfg.settle_ticks
                && self.rng.f32() < self.cfg.p_discover * 20.0
            {
                self.discover(j, 6);
            }
        }
        let cfg = &self.cfg;
        if self.rng.f32() < p_win {
            self.window.attack_wins += 1;
            let att = &self.agents[i];
            let mut mult = if att.knows(WEAPONS) { cfg.weapon_mult } else { 1.0 };
            mult *= 1.0 + 0.5 * att.emotion[ANGER];
            let raider = att.knows(WEAPONS) && att.knows(METAL);
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
            // Metal-armed raiders torch the fields of settled victims without walls.
            let (vx, vy, v_settled, v_walls) = {
                let v = &self.agents[j];
                (v.x, v.y, v.still >= cfg.settle_ticks, v.knows(WALLS))
            };
            if raider && v_settled && !v_walls {
                let lost = self.world.burn(vx, vy, cfg.wrap);
                if lost > 0.5 {
                    self.window.burned += 1;
                    self.events.fire(self.tick, "first_burn", "first fields burned by metal-armed raiders".to_string());
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
        let amount = cfg.share_amount * (1.0 + self.agents[i].emotion[BOND]);
        let give = self.agents[i].inventory.min(amount);
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

    /// Everything that passes between neighbours: knowledge and disease.
    fn contact(&mut self) {
        let cfg = &self.cfg;
        let n = self.decisions.len(); // agents present in the spatial hash this tick
        let range2 = cfg.learn_range * cfg.learn_range;
        self.learned.clear();
        self.infected.clear();
        self.apprentice.clear();
        self.imitations.clear();
        let mut rng = self.rng.clone();
        for i in 0..n {
            let a = &self.agents[i];
            let can_learn = a.tech != ALL_TECH;
            let can_catch = a.sick == 0 && a.immune == 0;
            let wealth = a.energy + a.inventory;
            let mut gained = 0u16;
            let mut caught = false;
            let mut skills = [0.0f32; N_SKILL];
            let mut model = NO_LEADER;
            let agents = &self.agents;
            self.spatial.for_each_near(a.x, a.y, |j| {
                if j == i || j >= n {
                    return;
                }
                let o = &agents[j];
                let missing = if can_learn { o.tech & !a.tech & !gained } else { 0 };
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
                    if a.knows(WRITING) || o.knows(WRITING) {
                        p *= 3.0;
                    }
                    if is_leader {
                        p *= 2.0;
                    }
                    for &bit in &TECH_BITS {
                        if missing & bit != 0 && rng.f32() < p {
                            gained |= bit;
                        }
                    }
                }
                if contagious {
                    let p = if a.knows(MEDICINE) { cfg.p_infect * 0.5 } else { cfg.p_infect };
                    if rng.f32() < p {
                        caught = true;
                    }
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
        for &(i, bits) in &self.learned {
            self.agents[i as usize].tech |= bits;
            for (t, &bit) in TECH_BITS.iter().enumerate() {
                if bits & bit != 0 {
                    self.window.learned[t] += 1;
                }
            }
        }
        for &i in &self.infected {
            let a = &mut self.agents[i as usize];
            a.sick = if a.knows(MEDICINE) { cfg.plague_len / 2 } else { cfg.plague_len };
            self.window.infections += 1;
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
            if a.knows(COOKING) {
                cost *= 1.0 - cfg.cooking_saving;
            }
            if a.sick > 0 {
                cost += if a.knows(MEDICINE) { cfg.sick_drain * 0.5 } else { cfg.sick_drain };
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

    /// Keeps the experiment alive after a collapse: fresh random brains drift in.
    fn immigrate(&mut self) {
        if self.agents.len() >= self.cfg.min_pop {
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

    fn discover(&mut self, i: usize, tech: usize) {
        let a = &mut self.agents[i];
        a.tech |= TECH_BITS[tech];
        a.feel(JOY, 0.3);
        a.prestige += 2.0;
        self.window.discoveries[tech] += 1;
        let (lineage, x, y) = (a.lineage, a.x, a.y);
        self.events.discovery(self.tick, tech, lineage, x, y);
    }

    pub fn settled_share(&self) -> f32 {
        let n = self.agents.len().max(1) as f32;
        self.agents.iter().filter(|a| a.still >= self.cfg.settle_ticks).count() as f32 / n
    }

    pub fn tech_share(&self) -> [f32; N_TECH] {
        let mut out = [0.0; N_TECH];
        let n = self.agents.len().max(1) as f32;
        for (t, &bit) in TECH_BITS.iter().enumerate() {
            out[t] = self.agents.iter().filter(|a| a.knows(bit)).count() as f32 / n;
        }
        out
    }

    pub fn take_window(&mut self) -> Window {
        std::mem::take(&mut self.window)
    }
}
