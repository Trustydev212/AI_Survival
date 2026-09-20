//! Open-ended innovations. Nothing here is named after human history: each world
//! generates its own discoveries from its seed. An innovation is a bundle of effects,
//! biased toward what its discoverer was doing, and it always carries a price.

use crate::agent::name_of;
use crate::brain::Action;
use crate::rng::Rng;

/// Effect dimensions. Positive is better except METABOLISM and SOIL, where positive hurts.
pub const E_GATHER: usize = 0; // harvest rate
pub const E_METABOLISM: usize = 1; // energy burned per tick (+ is worse)
pub const E_ATTACK: usize = 2; // strength when attacking
pub const E_DEFENSE: usize = 3; // strength when defending at home
pub const E_FARM: usize = 4; // field tending and yield
pub const E_RESIST: usize = 5; // disease resistance
pub const E_TEACH: usize = 6; // knowledge transmission
pub const E_INVENT: usize = 7; // discovery rate
pub const E_SHARE: usize = 8; // amount given when sharing
pub const E_SOIL: usize = 9; // extra soil exhaustion per harvest (+ is worse)
pub const E_SEA: usize = 10; // seafaring: boats once it passes the threshold in config
pub const N_EFFECT: usize = 11;
pub const EFFECT_NAMES: [&str; N_EFFECT] =
    ["gather", "metabolism", "attack", "defense", "farm", "resist", "teach", "invent", "share", "soil", "sea"];

pub const MAX_INNOVATIONS: usize = 64;

#[derive(Clone)]
pub struct Innovation {
    pub name: String,
    pub tier: u8,
    pub effects: [f32; N_EFFECT],
    pub born_tick: u64,
    pub lineage: u32,
}

impl Innovation {
    /// Generate a new innovation. `doing` biases which benefit appears; `tier` scales it.
    /// `coastal` discoverers (within a few cells of the sea) sometimes find ways onto the water instead.
    #[allow(clippy::too_many_arguments)]
    pub fn generate(rng: &mut Rng, id: usize, tier: u8, doing: Action, settled: bool, sick: bool, coastal: bool, tick: u64, lineage: u32) -> Innovation {
        let mut effects = [0.0f32; N_EFFECT];
        let scale = 0.15 * (tier as f32).powf(0.8);
        let magnitude = |rng: &mut Rng| scale * (0.7 + 0.6 * rng.f32());

        // Primary benefit follows the work being done.
        let pool: &[usize] = match doing {
            Action::Gather if settled => &[E_FARM, E_FARM, E_GATHER, E_DEFENSE, E_SOIL, E_SOIL],
            Action::Gather => &[E_GATHER, E_GATHER, E_METABOLISM, E_FARM],
            Action::Attack => &[E_ATTACK, E_ATTACK, E_DEFENSE],
            Action::Share => &[E_TEACH, E_SHARE, E_INVENT],
            Action::Reproduce => &[E_RESIST, E_SHARE],
            Action::Rest if sick => &[E_RESIST, E_RESIST, E_METABOLISM],
            Action::Rest => &[E_METABOLISM, E_DEFENSE, E_INVENT, E_RESIST],
        };
        let mut primary = pool[rng.range(pool.len())];
        if coastal && rng.f32() < 0.35 {
            primary = E_SEA;
        }
        let m = magnitude(rng);
        // Metabolism and soil are costs, so a benefit there is a reduction.
        effects[primary] += if primary == E_METABOLISM || primary == E_SOIL { -m } else { m };

        // Sometimes a second, smaller benefit.
        if rng.f32() < 0.3 {
            let all = [E_GATHER, E_ATTACK, E_DEFENSE, E_FARM, E_RESIST, E_TEACH, E_INVENT, E_SHARE];
            let d = all[rng.range(all.len())];
            effects[d] += magnitude(rng) * 0.5;
        }

        // Every innovation has a price: it burns more energy, or it wears the land.
        let price = m * (0.4 + 0.6 * rng.f32());
        let land_biased = matches!(primary, E_GATHER | E_FARM);
        let p_soil_price = if primary == E_SOIL || primary == E_SEA { 0.0 } else if land_biased { 0.75 } else { 0.35 };
        if rng.f32() < p_soil_price {
            effects[E_SOIL] += price;
        } else {
            effects[E_METABOLISM] += price;
        }

        Innovation { name: name_of(1_000_000 + id as u32), tier, effects, born_tick: tick, lineage }
    }

    /// Short label like "Kesh (t2: +farm .21, +soil .12)".
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();
        for d in 0..N_EFFECT {
            let v = self.effects[d];
            if v.abs() >= 0.01 {
                parts.push(format!("{}{} {:.2}", if v > 0.0 { "+" } else { "-" }, EFFECT_NAMES[d], v.abs()));
            }
        }
        format!("{} (t{}: {})", self.name, self.tier, parts.join(", "))
    }
}

/// Sum the effects of every innovation an agent knows.
pub fn capabilities(known: u64, registry: &[Innovation]) -> [f32; N_EFFECT] {
    let mut caps = [0.0f32; N_EFFECT];
    let mut bits = known;
    while bits != 0 {
        let i = bits.trailing_zeros() as usize;
        bits &= bits - 1;
        if let Some(inn) = registry.get(i) {
            for d in 0..N_EFFECT {
                caps[d] += inn.effects[d];
            }
        }
    }
    caps
}
