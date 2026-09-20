//! One tick of the world: regrow, sense, think, act, die, immigrate.

use crate::agent::{Agent, Decision};
use crate::brain::{Action, Genome, N_IN};
use crate::config::Config;
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
    spatial: SpatialHash,
    decisions: Vec<Decision>,
    next_lineage: u32,
}

impl Sim {
    pub fn new(cfg: Config) -> Sim {
        let mut rng = Rng::new(cfg.seed);
        let world = World::generate(cfg.width, cfg.height, cfg.max_food, cfg.regrow, cfg.season_len, &mut rng);
        let spatial = SpatialHash::new(cfg.width as f32, cfg.height as f32, cfg.vision, cfg.wrap);
        let mut sim = Sim {
            rng,
            world,
            agents: Vec::with_capacity(cfg.max_agents),
            tick: 0,
            window: Window::default(),
            spatial,
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
                self.agents.push(self.make_agent(x, y, genome, lineage));
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
        }
    }

    pub fn step(&mut self) {
        let season = self.world.season(self.tick);
        self.world.regrow(season);
        self.spatial.rebuild(self.agents.iter().map(|a| (a.x, a.y)));
        self.sense_and_think(season);
        self.act();
        self.metabolise_and_die();
        self.immigrate();
        self.tick += 1;
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
                } else {
                    foe += 1.0;
                }
                if d2 < best_d2 {
                    best_d2 = d2;
                    nearest = j as u32;
                }
            });

            // Food gradient from four sample points at vision/2.
            let r = cfg.vision * 0.5;
            let f_here = world.food[world.idx(a.x, a.y)];
            let sample = |x: f32, y: f32| world.food[world.idx(self.place(x, cfg.width), self.place(y, cfg.height))];
            let gx = (sample(a.x + r, a.y) - sample(a.x - r, a.y)) / cfg.max_food;
            let gy = (sample(a.x, a.y + r) - sample(a.x, a.y - r)) / cfg.max_food;

            let mut input = [0.0f32; N_IN];
            input[0] = a.energy / cfg.max_energy;
            input[1] = a.age as f32 / cfg.max_age as f32;
            input[2] = a.inventory / cfg.inv_cap;
            input[3] = f_here / cfg.max_food;
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
            input[18] = world.fertility[world.idx(a.x, a.y)];
            input[19] = 1.0;

            let (mx, my, action) = a.genome.think(&input);
            self.decisions.push(Decision { mx, my, action, target: nearest });
        }
    }

    fn act(&mut self) {
        let n = self.agents.len();
        let mut births: Vec<Agent> = Vec::new();
        for i in 0..n {
            let d = self.decisions[i];
            self.window.actions[d.action as usize] += 1;
            self.agents[i].last_action = d.action;

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
                    let a = &mut self.agents[i];
                    let cell = self.world.idx(a.x, a.y);
                    let take = self.world.food[cell].min(cfg.gather_rate);
                    self.world.food[cell] -= take;
                    a.energy += take;
                    if a.energy > cfg.max_energy {
                        a.inventory = (a.inventory + a.energy - cfg.max_energy).min(cfg.inv_cap);
                        a.energy = cfg.max_energy;
                    }
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
                        let lineage = a.lineage;
                        let mut child = self.make_agent(x, y, child_genome, lineage);
                        child.energy = cfg.child_energy;
                        births.push(child);
                        self.window.births += 1;
                    }
                }
                Action::Rest => {}
            }
        }
        self.agents.extend(births);
    }

    fn resolve_attack(&mut self, i: usize, j: usize) {
        let cfg = &self.cfg;
        let (ax, ay, ae) = {
            let a = &self.agents[i];
            (a.x, a.y, a.energy)
        };
        let (dx, dy, de) = {
            let t = &self.agents[j];
            (self.delta(ax, t.x, cfg.width), self.delta(ay, t.y, cfg.height), t.energy)
        };
        if dx * dx + dy * dy > cfg.attack_range * cfg.attack_range {
            return;
        }
        self.window.attacks += 1;
        self.agents[i].energy -= cfg.attack_cost;
        // Stronger fighter usually wins; sigmoid of energy difference.
        let p_win = 1.0 / (1.0 + (-(ae - de) / 25.0).exp());
        if self.rng.f32() < p_win {
            self.window.attack_wins += 1;
            let stolen = self.agents[j].inventory.min(cfg.steal);
            self.agents[j].inventory -= stolen;
            self.agents[j].energy -= cfg.attack_damage;
            self.agents[j].attacked_timer = 30;
            let a = &mut self.agents[i];
            a.energy += stolen + cfg.attack_damage * 0.5;
            if a.energy > cfg.max_energy {
                a.inventory = (a.inventory + a.energy - cfg.max_energy).min(cfg.inv_cap);
                a.energy = cfg.max_energy;
            }
        } else {
            self.agents[i].energy -= cfg.attack_damage * 0.5;
            self.agents[i].attacked_timer = 30;
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
        let give = self.agents[i].inventory.min(cfg.share_amount);
        if give <= 0.0 {
            return;
        }
        self.agents[i].inventory -= give;
        let t = &mut self.agents[j];
        t.inventory = (t.inventory + give).min(cfg.inv_cap);
        self.window.shares += 1;
    }

    fn metabolise_and_die(&mut self) {
        let cfg = &self.cfg;
        let n = self.decisions.len();
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
            a.energy -= cost;
            if a.energy < cfg.eat_below && a.inventory > 0.0 {
                let eat = a.inventory.min(cfg.eat_amount);
                a.inventory -= eat;
                a.energy += eat;
            }
            a.age += 1;
            if a.attacked_timer > 0 {
                a.attacked_timer -= 1;
            }
        }
        let w = &mut self.window;
        self.agents.retain(|a| {
            if a.energy <= 0.0 {
                if a.attacked_timer > 0 {
                    w.killed += 1;
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

    pub fn take_window(&mut self) -> Window {
        std::mem::take(&mut self.window)
    }
}
