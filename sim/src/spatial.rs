//! Uniform grid spatial hash so neighbour queries are O(k) instead of O(n).

pub struct SpatialHash {
    cell: f32,
    wrap: bool,
    cols: usize,
    rows: usize,
    heads: Vec<u32>,
    next: Vec<u32>,
}

const NONE: u32 = u32::MAX;

impl SpatialHash {
    pub fn new(width: f32, height: f32, cell: f32, wrap: bool) -> SpatialHash {
        // Exact bucket count so that wrapping bucket indices wraps the world.
        let cols = ((width / cell).ceil() as usize).max(1);
        let rows = ((height / cell).ceil() as usize).max(1);
        SpatialHash { cell, wrap, cols, rows, heads: vec![NONE; cols * rows], next: Vec::new() }
    }

    pub fn rebuild(&mut self, positions: impl ExactSizeIterator<Item = (f32, f32)>) {
        self.heads.iter_mut().for_each(|h| *h = NONE);
        self.next.clear();
        self.next.resize(positions.len(), NONE);
        for (i, (x, y)) in positions.enumerate() {
            let b = self.bucket(x, y);
            self.next[i] = self.heads[b];
            self.heads[b] = i as u32;
        }
    }

    #[inline]
    fn bucket(&self, x: f32, y: f32) -> usize {
        let cx = ((x / self.cell) as usize).min(self.cols - 1);
        let cy = ((y / self.cell) as usize).min(self.rows - 1);
        cy * self.cols + cx
    }

    /// Visit every index in the 3x3 block of buckets around (x, y).
    /// Callers filter by exact distance.
    #[inline]
    #[allow(dead_code)]
    pub fn for_each_near(&self, x: f32, y: f32, mut f: impl FnMut(usize)) {
        self.for_each_near_until(x, y, |j| {
            f(j);
            true
        });
    }

    /// Like for_each_near, but stops as soon as the callback returns false.
    #[inline]
    pub fn for_each_near_until(&self, x: f32, y: f32, mut f: impl FnMut(usize) -> bool) {
        let cx = ((x / self.cell) as isize).min(self.cols as isize - 1);
        let cy = ((y / self.cell) as isize).min(self.rows as isize - 1);
        for dy in -1..=1 {
            let mut by = cy + dy;
            if self.wrap {
                by = by.rem_euclid(self.rows as isize);
            } else if by < 0 || by >= self.rows as isize {
                continue;
            }
            for dx in -1..=1 {
                let mut bx = cx + dx;
                if self.wrap {
                    bx = bx.rem_euclid(self.cols as isize);
                } else if bx < 0 || bx >= self.cols as isize {
                    continue;
                }
                let mut i = self.heads[by as usize * self.cols + bx as usize];
                while i != NONE {
                    if !f(i as usize) {
                        return;
                    }
                    i = self.next[i as usize];
                }
            }
        }
    }
}
