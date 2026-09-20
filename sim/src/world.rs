//! The map: a grid of cells with fixed fertility and regrowing food.
//! Fertility is clustered on purpose so that land is scarce and unequal.

use crate::rng::Rng;

pub struct World {
    pub width: usize,
    pub height: usize,
    pub fertility: Vec<f32>,
    pub food: Vec<f32>,
    pub max_food: f32,
    pub regrow: f32,
    pub season_len: f32,
}

impl World {
    pub fn generate(width: usize, height: usize, max_food: f32, regrow: f32, season_len: f32, rng: &mut Rng) -> World {
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
        World { width, height, fertility, food, max_food, regrow, season_len }
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
        let max_food = self.max_food;
        for (food, fert) in self.food.iter_mut().zip(self.fertility.iter()) {
            let cap = max_food * fert;
            if cap <= 0.0 {
                continue;
            }
            *food += r * fert * (1.0 - *food / cap);
            if *food > cap {
                *food = cap;
            }
        }
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
