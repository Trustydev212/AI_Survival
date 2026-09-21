//! What survives the end of a world.
//!
//! When everyone dies the world starts again, and until now it started with brains drawn at
//! random: every civilisation began from nothing and the millions of ticks a long run buys were
//! spent learning the same first lessons over and over. Nothing accumulated.
//!
//! The ark is the one thing carried across. It holds a few of the brains that did best, judged by
//! the only measure the world itself keeps, how many children they left, and the founders of the
//! next world are drawn from it with mutation rather than from noise. Selection then reaches
//! across the deaths of whole civilisations instead of stopping at each one.
//!
//! It is deliberately small and deliberately mixed. A handful of brains, taken from different
//! lineages, and only part of each new world founded from them: an ark that held everything would
//! make every world a copy of the last, which would answer the question by removing it.

use crate::agent::Agent;
use crate::brain::{Genome, N_LEARN, N_TEMPER};
use crate::rng::Rng;
use std::io::{Read, Write};

/// How many brains are kept. Small on purpose: this is a memory, not a breeding programme.
pub const CAPACITY: usize = 24;
/// What share of a new world's founding tribes come from the ark rather than from noise.
pub const FOUNDED_FROM_ARK: f32 = 0.5;

#[derive(Clone)]
pub struct Saved {
    pub genome: Genome,
    /// How many children this brain left, which is the world's own measure of doing well.
    pub children: u16,
    /// Which civilisation it belonged to, and how old the world was when it was noticed.
    pub generation: u32,
    pub tick: u64,
}

#[derive(Default)]
pub struct Ark {
    pub kept: Vec<Saved>,
}

impl Ark {
    /// Look over the living and keep any brain better than the worst one held. Called now and
    /// then, not every tick: the point is to notice the good ones before the world ends, because
    /// at the end there is nobody left to ask.
    pub fn consider(&mut self, agents: &[Agent], generation: u32, tick: u64) {
        for a in agents {
            if a.children == 0 {
                continue;
            }
            // One brain per lineage, so the ark does not fill up with one successful family.
            if let Some(slot) = self.kept.iter_mut().find(|s| s.genome.marker == a.genome.marker) {
                if a.children > slot.children {
                    *slot = Saved { genome: a.genome.clone(), children: a.children, generation, tick };
                }
                continue;
            }
            if self.kept.len() < CAPACITY {
                self.kept.push(Saved { genome: a.genome.clone(), children: a.children, generation, tick });
            } else if let Some(worst) = self.kept.iter_mut().min_by_key(|s| s.children) {
                if a.children > worst.children {
                    *worst = Saved { genome: a.genome.clone(), children: a.children, generation, tick };
                }
            }
        }
    }

    /// A brain to found a tribe with, or nothing if the ark is empty. Picked with a bias towards
    /// the ones that left most children, but never only the best: a world founded by one mind is
    /// not a world.
    pub fn draw(&self, rng: &mut Rng) -> Option<Genome> {
        if self.kept.is_empty() {
            return None;
        }
        let total: u32 = self.kept.iter().map(|s| s.children as u32 + 1).sum();
        let mut pick = (rng.f32() * total as f32) as u32;
        for s in &self.kept {
            let weight = s.children as u32 + 1;
            if pick < weight {
                return Some(s.genome.clone());
            }
            pick -= weight;
        }
        self.kept.last().map(|s| s.genome.clone())
    }

    pub fn best(&self) -> u16 {
        self.kept.iter().map(|s| s.children).max().unwrap_or(0)
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut f = std::io::BufWriter::new(std::fs::File::create(path)?);
        f.write_all(b"AISA")?;
        f.write_all(&(self.kept.len() as u32).to_le_bytes())?;
        for s in &self.kept {
            f.write_all(&s.children.to_le_bytes())?;
            f.write_all(&s.generation.to_le_bytes())?;
            f.write_all(&s.tick.to_le_bytes())?;
            f.write_all(&(s.genome.weights.len() as u32).to_le_bytes())?;
            for w in &s.genome.weights {
                f.write_all(&w.to_le_bytes())?;
            }
            for v in s.genome.marker {
                f.write_all(&v.to_le_bytes())?;
            }
            for v in s.genome.temper {
                f.write_all(&v.to_le_bytes())?;
            }
            for v in s.genome.learn {
                f.write_all(&v.to_le_bytes())?;
            }
        }
        Ok(())
    }

    pub fn load(path: &str) -> Ark {
        let mut bytes = Vec::new();
        if std::fs::File::open(path).and_then(|mut f| f.read_to_end(&mut bytes)).is_err() || bytes.len() < 8 || &bytes[..4] != b"AISA" {
            return Ark::default();
        }
        let mut at = 4usize;
        let mut take4 = |at: &mut usize| {
            let v = u32::from_le_bytes(bytes[*at..*at + 4].try_into().unwrap());
            *at += 4;
            v
        };
        let n = take4(&mut at) as usize;
        let mut kept = Vec::with_capacity(n);
        for _ in 0..n {
            if at + 14 > bytes.len() {
                break;
            }
            let children = u16::from_le_bytes(bytes[at..at + 2].try_into().unwrap());
            at += 2;
            let generation = take4(&mut at);
            let tick = u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap());
            at += 8;
            let nw = take4(&mut at) as usize;
            let need = nw * 4 + (3 + N_TEMPER + N_LEARN) * 4;
            if at + need > bytes.len() {
                break;
            }
            let mut f32s = |at: &mut usize, count: usize| -> Vec<f32> {
                let v = (0..count).map(|i| f32::from_le_bytes(bytes[*at + i * 4..*at + i * 4 + 4].try_into().unwrap())).collect();
                *at += count * 4;
                v
            };
            let weights = f32s(&mut at, nw);
            let m = f32s(&mut at, 3);
            let t = f32s(&mut at, N_TEMPER);
            let l = f32s(&mut at, N_LEARN);
            let mut marker = [0.0; 3];
            marker.copy_from_slice(&m);
            let mut temper = [0.0; N_TEMPER];
            temper.copy_from_slice(&t);
            let mut learn = [0.0; N_LEARN];
            learn.copy_from_slice(&l);
            kept.push(Saved { genome: Genome { weights, marker, temper, learn }, children, generation, tick });
        }
        Ark { kept }
    }
}
