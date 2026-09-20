//! The map: a grid of cells with fixed fertility and regrowing food.
//! Fertility is clustered on purpose so that land is scarce and unequal.

use crate::rng::Rng;

pub struct World {
    pub width: usize,
    pub height: usize,
    pub fertility: Vec<f32>,
    pub food: Vec<f32>,
    /// How tended a cell is, 0..1. Raised by farmers standing on it, decays otherwise.
    pub cultivation: Vec<f32>,
    /// Irrigated cells keep growing through droughts.
    pub irrigated: Vec<bool>,
    /// Weather multiplier on regrowth: 1 normal, below 1 drought, above 1 a good year.
    pub climate: f32,
    pub farm_boost: f32,
    pub cult_decay: f32,
    pub max_food: f32,
    pub regrow: f32,
    pub season_len: f32,
}

impl World {
    pub fn generate(width: usize, height: usize, max_food: f32, regrow: f32, season_len: f32, farm_boost: f32, cult_decay: f32, rng: &mut Rng) -> World {
        let mut fertility = vec![0.0f32; width * height];
        // Two octaves of value noise, bilinearly interpolated.
        let octaves = [(24usize, 1.0f32), (8usize, 0.35f32)];
        for &(period, amp) in &octaves {
            let gw = width / period + 2;
            let gh = height / period + 2;
            let grid: Vec<f32> = (0..gw * gh).map(|_| rng.f32()).collect();
            for y in 0..height {
                for x in 0..width {
                    let fx = x as f32 / period as f32;
                    let fy = y as f32 / period as f32;
                    let x0 = fx.floor() as usize;
                    let y0 = fy.floor() as usize;
                    let tx = smooth(fx - x0 as f32);
                    let ty = smooth(fy - y0 as f32);
                    let g = |gx: usize, gy: usize| grid[gy * gw + gx];
                    let a = lerp(g(x0, y0), g(x0 + 1, y0), tx);
                    let b = lerp(g(x0, y0 + 1), g(x0 + 1, y0 + 1), tx);
                    fertility[y * width + x] += amp * lerp(a, b, ty);
                }
            }
        }
        // Normalise then threshold so ~40% of land is near-barren and patches are rich.
        let total_amp: f32 = octaves.iter().map(|o| o.1).sum();
        for f in fertility.iter_mut() {
            let v = *f / total_amp;
            let t = ((v - 0.42) / 0.30).clamp(0.0, 1.0);
            *f = t * t * (3.0 - 2.0 * t); // smoothstep
        }
        let food = fertility.iter().map(|f| f * max_food * 0.8).collect();
        let cultivation = vec![0.0; width * height];
        let irrigated = vec![false; width * height];
        World { width, height, fertility, food, cultivation, irrigated, climate: 1.0, farm_boost, cult_decay, max_food, regrow, season_len }
    }

    #[inline]
    pub fn idx(&self, x: f32, y: f32) -> usize {
        let xi = (x as usize).min(self.width - 1);
        let yi = (y as usize).min(self.height - 1);
        yi * self.width + xi
    }

    /// Seasonal multiplier in [0.2, 1.0]; winter is a real famine.
    pub fn season(&self, tick: u64) -> f32 {
        let phase = (tick as f32 / self.season_len) * std::f32::consts::TAU;
        0.2 + 0.8 * (0.5 + 0.5 * phase.sin())
    }

    pub fn regrow(&mut self, season: f32) {
        let r = self.regrow * season;
        let climate = self.climate;
        let irrigated_climate = climate.max(0.8);
        let max_food = self.max_food;
        let boost = self.farm_boost;
        let decay = self.cult_decay;
        for i in 0..self.food.len() {
            let fert = self.fertility[i];
            let cult = &mut self.cultivation[i];
            if *cult > 0.0 {
                *cult *= decay;
                if *cult < 0.001 {
                    *cult = 0.0;
                    self.irrigated[i] = false;
                }
            }
            let cap = max_food * fert * (1.0 + 2.0 * *cult);
            if cap <= 0.0 {
                continue;
            }
            let c = if self.irrigated[i] { irrigated_climate } else { climate };
            let food = &mut self.food[i];
            *food += r * c * fert * (1.0 + boost * *cult) * (1.0 - *food / cap);
            if *food > cap {
                *food = cap;
            }
        }
    }

    /// Raiders torch the 3x3 field around a spot. Returns cultivation destroyed.
    pub fn burn(&mut self, x: f32, y: f32, wrap: bool) -> f32 {
        let mut lost = 0.0;
        self.for_block(x, y, wrap, |w, i| {
            let before = w.cultivation[i];
            w.cultivation[i] *= 0.2;
            lost += before - w.cultivation[i];
        });
        lost
    }

    fn for_block(&mut self, x: f32, y: f32, wrap: bool, mut f: impl FnMut(&mut World, usize)) {
        let cx = (x as isize).min(self.width as isize - 1);
        let cy = (y as isize).min(self.height as isize - 1);
        for dy in -1..=1isize {
            for dx in -1..=1isize {
                let (mut nx, mut ny) = (cx + dx, cy + dy);
                if wrap {
                    nx = nx.rem_euclid(self.width as isize);
                    ny = ny.rem_euclid(self.height as isize);
                } else if nx < 0 || ny < 0 || nx >= self.width as isize || ny >= self.height as isize {
                    continue;
                }
                let i = ny as usize * self.width + nx as usize;
                f(self, i);
            }
        }
    }

    /// Take up to `rate` food from the agent's cell first, then from the 8 cells around it,
    /// but the ring only where it is a worked field (cultivated). Wild land is picked cell by cell.
    pub fn harvest(&mut self, x: f32, y: f32, rate: f32, wrap: bool) -> f32 {
        let cx = (x as isize).min(self.width as isize - 1);
        let cy = (y as isize).min(self.height as isize - 1);
        let mut left = rate;
        let mut got = 0.0;
        const ORDER: [(isize, isize); 9] = [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)];
        for (dx, dy) in ORDER {
            if left <= 0.0 {
                break;
            }
            let (mut nx, mut ny) = (cx + dx, cy + dy);
            if wrap {
                nx = nx.rem_euclid(self.width as isize);
                ny = ny.rem_euclid(self.height as isize);
            } else if nx < 0 || ny < 0 || nx >= self.width as isize || ny >= self.height as isize {
                continue;
            }
            let i = ny as usize * self.width + nx as usize;
            if (dx != 0 || dy != 0) && self.cultivation[i] < 0.2 {
                continue;
            }
            let take = self.food[i].min(left);
            self.food[i] -= take;
            left -= take;
            got += take;
        }
        got
    }

    /// A farmer tends the 3x3 block around them: full gain on their cell, half on the ring.
    /// An irrigating farmer also marks the cells as irrigated.
    pub fn tend(&mut self, x: f32, y: f32, gain: f32, irrigate: bool, wrap: bool) {
        let cx = (x as isize).min(self.width as isize - 1);
        let cy = (y as isize).min(self.height as isize - 1);
        for dy in -1..=1isize {
            for dx in -1..=1isize {
                let (mut nx, mut ny) = (cx + dx, cy + dy);
                if wrap {
                    nx = nx.rem_euclid(self.width as isize);
                    ny = ny.rem_euclid(self.height as isize);
                } else if nx < 0 || ny < 0 || nx >= self.width as isize || ny >= self.height as isize {
                    continue;
                }
                let i = ny as usize * self.width + nx as usize;
                if self.fertility[i] <= 0.05 {
                    continue; // nothing grows on barren rock
                }
                let g = if dx == 0 && dy == 0 { gain } else { gain * 0.5 };
                let c = &mut self.cultivation[i];
                *c = (*c + g).min(1.0);
                if irrigate {
                    self.irrigated[i] = true;
                }
            }
        }
    }

    pub fn cultivated_cells(&self) -> usize {
        self.cultivation.iter().filter(|c| **c > 0.2).count()
    }

    pub fn total_food(&self) -> f32 {
        self.food.iter().sum()
    }
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
#[inline]
fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}
