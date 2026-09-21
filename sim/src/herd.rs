//! Herds: big prey that no one can bring down alone.
//!
//! A herd wanders the land, grows back slowly, and falls only when enough strength lands
//! on it in the same tick. A lone attacker pays the cost and scares it off. Meat is split
//! among everyone who struck. This is the one task in the world that needs people to act
//! together at the same moment, which is exactly what a call could be for.

use crate::rng::Rng;
use crate::world::World;

pub struct Herd {
    pub x: f32,
    pub y: f32,
    /// 0..1: how much meat is on it. Grows back; 0 right after a kill.
    pub size: f32,
    pub dx: f32,
    pub dy: f32,
    /// Ticks until a killed herd reappears somewhere else.
    pub cooldown: u32,
    /// Strength landed on it lately, and by whom. Hits fade by half each tick, so blows a
    /// tick or two apart still add up; blows minutes apart do not.
    pub hits: f32,
    pub hunters: Vec<u32>,
    /// Whether anyone struck this tick.
    pub struck: bool,
}

#[derive(Default)]
pub struct Herds {
    pub list: Vec<Herd>,
}

impl Herds {
    pub fn spawn(n: usize, world: &World, rng: &mut Rng) -> Herds {
        let mut list = Vec::with_capacity(n);
        for _ in 0..n {
            let (x, y) = grazing_spot(world, rng);
            let (dx, dy) = wander(rng);
            list.push(Herd { x, y, size: 0.6 + 0.4 * rng.f32(), dx, dy, cooldown: 0, hits: 0.0, hunters: Vec::new(), struck: false });
        }
        Herds { list }
    }

    /// Herds graze, drift, grow back, and reappear after being taken.
    pub fn step(&mut self, world: &World, rng: &mut Rng, regrow: f32, wrap: bool) {
        let (w, h) = (world.width as f32, world.height as f32);
        for herd in self.list.iter_mut() {
            herd.hits *= 0.5;
            herd.struck = false;
            if herd.hits < 5.0 {
                herd.hits = 0.0;
                herd.hunters.clear();
            }
            if herd.cooldown > 0 {
                herd.cooldown -= 1;
                if herd.cooldown == 0 {
                    let (x, y) = grazing_spot(world, rng);
                    herd.x = x;
                    herd.y = y;
                    herd.size = 0.3;
                }
                continue;
            }
            herd.size = (herd.size + regrow).min(1.0);
            if rng.f32() < 0.03 {
                let (dx, dy) = wander(rng);
                herd.dx = dx;
                herd.dy = dy;
            }
            let nx = place(herd.x + herd.dx, w, wrap);
            let ny = place(herd.y + herd.dy, h, wrap);
            if world.is_water(nx, ny) {
                herd.dx = -herd.dx;
                herd.dy = -herd.dy;
            } else {
                herd.x = nx;
                herd.y = ny;
            }
        }
    }

    /// A failed hunt scares the herd some way off.
    pub fn flee(&mut self, i: usize, world: &World, rng: &mut Rng, wrap: bool) {
        let (w, h) = (world.width as f32, world.height as f32);
        let herd = &mut self.list[i];
        for _ in 0..6 {
            let (dx, dy) = wander(rng);
            let nx = place(herd.x + dx * 30.0, w, wrap);
            let ny = place(herd.y + dy * 30.0, h, wrap);
            if !world.is_water(nx, ny) {
                herd.x = nx;
                herd.y = ny;
                break;
            }
        }
    }

    /// Nearest live herd to a point: (index, dx, dy, squared distance).
    pub fn nearest(&self, x: f32, y: f32, w: f32, h: f32, wrap: bool) -> Option<(usize, f32, f32, f32)> {
        let mut best: Option<(usize, f32, f32, f32)> = None;
        for (i, herd) in self.list.iter().enumerate() {
            if herd.cooldown > 0 {
                continue;
            }
            let dx = delta(x, herd.x, w, wrap);
            let dy = delta(y, herd.y, h, wrap);
            let d2 = dx * dx + dy * dy;
            if best.is_none_or(|b| d2 < b.3) {
                best = Some((i, dx, dy, d2));
            }
        }
        best
    }
}

fn grazing_spot(world: &World, rng: &mut Rng) -> (f32, f32) {
    for _ in 0..200 {
        let x = rng.range(world.width);
        let y = rng.range(world.height);
        let i = y * world.width + x;
        if !world.water[i] && world.base_fertility[i] > 0.4 {
            return (x as f32 + 0.5, y as f32 + 0.5);
        }
    }
    (world.width as f32 / 2.0, world.height as f32 / 2.0)
}

fn wander(rng: &mut Rng) -> (f32, f32) {
    let a = rng.f32() * std::f32::consts::TAU;
    (a.cos() * 0.15, a.sin() * 0.15)
}

#[inline]
fn place(v: f32, size: f32, wrap: bool) -> f32 {
    if wrap {
        v.rem_euclid(size)
    } else {
        v.clamp(0.0, size - 0.001)
    }
}

#[inline]
fn delta(a: f32, b: f32, size: f32, wrap: bool) -> f32 {
    let mut d = b - a;
    if wrap {
        if d > size * 0.5 {
            d -= size;
        } else if d < -size * 0.5 {
            d += size;
        }
    }
    d
}
