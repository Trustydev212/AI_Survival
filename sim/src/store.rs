//! Village storehouses: food held in common, raised by a settled leader, filled by
//! followers who obey a pool order, drawn on by any kin who goes hungry, looted by
//! raiders, and slowly spoiling. The first institution that outlives a winter.

use crate::agent::Agent;
use crate::region::RegionGrid;

pub struct Store {
    pub x: f32,
    pub y: f32,
    pub food: f32,
    /// Name id of the leader who raised it.
    pub owner: u32,
    pub lineage: u32,
    /// Kin marker of the founder: who may draw on it.
    pub marker: [f32; 3],
    /// Ticks since anything was put in or taken out.
    pub idle: u32,
}

impl Store {
    #[inline]
    pub fn kinship(&self, a: &Agent) -> f32 {
        let m = &a.genome.marker;
        let d = (0..3).map(|i| (self.marker[i] - m[i]).powi(2)).sum::<f32>().sqrt();
        1.0 - d / 1.732
    }
}

/// Stores plus a per-region index so lookups stay cheap however many villages there are.
#[derive(Default)]
pub struct Stores {
    pub list: Vec<Store>,
    index: Vec<Vec<u16>>,
    cols: usize,
    rows: usize,
}

pub struct Nearby {
    pub idx: usize,
    pub dx: f32,
    pub dy: f32,
    pub dist: f32,
}

impl Stores {
    pub fn rebuild_index(&mut self, regions: &RegionGrid) {
        self.cols = regions.cols;
        self.rows = regions.rows;
        self.index.clear();
        self.index.resize(regions.cols * regions.rows, Vec::new());
        for (i, s) in self.list.iter().enumerate() {
            let r = regions.index(s.x, s.y);
            self.index[r].push(i as u16);
        }
    }

    /// Nearest store within `range` of (x, y) that `a` is kin to (or, with `kin` false,
    /// explicitly NOT kin to: what a raider looks for).
    pub fn nearest(
        &self, x: f32, y: f32, a: &Agent, kin: bool, kin_threshold: f32, range: f32, regions: &RegionGrid,
        wrap: bool, width: usize, height: usize,
    ) -> Option<Nearby> {
        if self.list.is_empty() || self.index.is_empty() {
            return None;
        }
        let r = regions.index(x, y);
        let (rx, ry) = ((r % self.cols) as isize, (r / self.cols) as isize);
        let mut best: Option<Nearby> = None;
        for dy in -1..=1isize {
            for dx in -1..=1isize {
                let (mut nx, mut ny) = (rx + dx, ry + dy);
                if wrap {
                    nx = nx.rem_euclid(self.cols as isize);
                    ny = ny.rem_euclid(self.rows as isize);
                } else if nx < 0 || ny < 0 || nx >= self.cols as isize || ny >= self.rows as isize {
                    continue;
                }
                for &si in &self.index[ny as usize * self.cols + nx as usize] {
                    let s = &self.list[si as usize];
                    let related = s.kinship(a) >= kin_threshold;
                    if related != kin {
                        continue;
                    }
                    let ddx = delta(x, s.x, width, wrap);
                    let ddy = delta(y, s.y, height, wrap);
                    let d = (ddx * ddx + ddy * ddy).sqrt();
                    if d <= range && best.as_ref().map_or(true, |b| d < b.dist) {
                        best = Some(Nearby { idx: si as usize, dx: ddx, dy: ddy, dist: d });
                    }
                }
            }
        }
        best
    }

    pub fn total_food(&self) -> f32 {
        self.list.iter().map(|s| s.food).sum()
    }
}

#[inline]
fn delta(a: f32, b: f32, size: usize, wrap: bool) -> f32 {
    let d = b - a;
    if !wrap {
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
