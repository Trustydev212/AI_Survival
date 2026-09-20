//! A coarse map over the world so brains can perceive the state of a whole region,
//! not just the cell under their feet. Without this an agent cannot tell a tired
//! patch from a dying land, and cannot leave before the land dies.

use crate::agent::Agent;
use crate::world::World;

pub struct RegionGrid {
    pub cols: usize,
    pub rows: usize,
    pub side: usize,
    /// Mean fertility as a fraction of what the region could be. 1 = pristine.
    pub soil: Vec<f32>,
    /// Mean standing food as a fraction of the cap.
    pub food: Vec<f32>,
    /// Agents per region, normalised by a comfortable density.
    pub crowd: Vec<f32>,
    /// Direction to the most promising region within reach, and how much better it is.
    pub best_dx: Vec<f32>,
    pub best_dy: Vec<f32>,
    pub best_gain: Vec<f32>,
    /// Fraction of the region that is sea (fixed at generation).
    pub water: Vec<f32>,
}

/// How attractive a region is to live in.
#[inline]
fn promise(food: f32, soil: f32, crowd: f32) -> f32 {
    food + 0.5 * soil - 0.5 * crowd
}

impl RegionGrid {
    pub fn new(world: &World, side: usize) -> RegionGrid {
        let side = side.max(1);
        let cols = world.width.div_ceil(side);
        let rows = world.height.div_ceil(side);
        let n = cols * rows;
        let mut water = vec![0.0f32; n];
        let mut cells = vec![0.0f32; n];
        for y in 0..world.height {
            for x in 0..world.width {
                let r = (y / side).min(rows - 1) * cols + (x / side).min(cols - 1);
                cells[r] += 1.0;
                if world.water[y * world.width + x] {
                    water[r] += 1.0;
                }
            }
        }
        for r in 0..n {
            water[r] = if cells[r] > 0.0 { water[r] / cells[r] } else { 0.0 };
        }
        RegionGrid {
            cols,
            rows,
            side,
            soil: vec![1.0; n],
            food: vec![0.0; n],
            crowd: vec![0.0; n],
            best_dx: vec![0.0; n],
            best_dy: vec![0.0; n],
            best_gain: vec![0.0; n],
            water,
        }
    }

    #[inline]
    pub fn index(&self, x: f32, y: f32) -> usize {
        let rx = ((x as usize) / self.side).min(self.cols - 1);
        let ry = ((y as usize) / self.side).min(self.rows - 1);
        ry * self.cols + rx
    }

    pub fn refresh(&mut self, world: &World, agents: &[Agent], wrap: bool) {
        let n = self.cols * self.rows;
        let mut soil_sum = vec![0.0f32; n];
        let mut base_sum = vec![0.0f32; n];
        let mut food_sum = vec![0.0f32; n];
        let mut cap_sum = vec![0.0f32; n];
        for y in 0..world.height {
            let ry = (y / self.side).min(self.rows - 1);
            for x in 0..world.width {
                let rx = (x / self.side).min(self.cols - 1);
                let r = ry * self.cols + rx;
                let i = y * world.width + x;
                soil_sum[r] += world.fertility[i];
                base_sum[r] += world.base_fertility[i];
                food_sum[r] += world.food[i];
                cap_sum[r] += world.max_food * world.base_fertility[i];
            }
        }
        for r in 0..n {
            self.soil[r] = if base_sum[r] > 0.01 { soil_sum[r] / base_sum[r] } else { 1.0 };
            self.food[r] = if cap_sum[r] > 0.01 { (food_sum[r] / cap_sum[r]).min(2.0) } else { 0.0 };
            self.crowd[r] = 0.0;
        }
        let per_region = (self.side * self.side) as f32;
        // One agent per 25 cells counts as a full region.
        let comfortable = (per_region / 25.0).max(1.0);
        for a in agents {
            let r = self.index(a.x, a.y);
            self.crowd[r] += 1.0;
        }
        for c in self.crowd.iter_mut() {
            *c = (*c / comfortable).min(3.0);
        }

        // Where to go from here: the best of the 8 neighbours, if better than home.
        for ry in 0..self.rows {
            for rx in 0..self.cols {
                let r = ry * self.cols + rx;
                let home = promise(self.food[r], self.soil[r], self.crowd[r]);
                let mut best = home;
                let (mut bdx, mut bdy) = (0.0f32, 0.0f32);
                for dy in -1..=1isize {
                    for dx in -1..=1isize {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let (nx, ny) = (rx as isize + dx, ry as isize + dy);
                        let (nx, ny) = if wrap {
                            (nx.rem_euclid(self.cols as isize), ny.rem_euclid(self.rows as isize))
                        } else {
                            if nx < 0 || ny < 0 || nx >= self.cols as isize || ny >= self.rows as isize {
                                continue;
                            }
                            (nx, ny)
                        };
                        let o = ny as usize * self.cols + nx as usize;
                        let p = promise(self.food[o], self.soil[o], self.crowd[o]);
                        if p > best {
                            best = p;
                            bdx = dx as f32;
                            bdy = dy as f32;
                        }
                    }
                }
                let len = (bdx * bdx + bdy * bdy).sqrt().max(1.0);
                self.best_dx[r] = bdx / len;
                self.best_dy[r] = bdy / len;
                self.best_gain[r] = (best - home).min(2.0);
            }
        }
    }

    /// Mean soil health of the regions where agents actually are.
    pub fn inhabited_soil(&self, agents: &[Agent]) -> f32 {
        if agents.is_empty() {
            return 1.0;
        }
        let mut sum = 0.0;
        for a in agents {
            sum += self.soil[self.index(a.x, a.y)];
        }
        sum / agents.len() as f32
    }
}
