//! Open-ended innovations. Nothing here is named after human history: each world
//! generates its own discoveries from its seed.
//!
//! Two kinds exist. A **practice** is an idea: a bundle of social or agricultural
//! effects biased toward what its discoverer was doing, always with a price. A **craft**
//! is a thing: a recipe found by working materials (see craft.rs), whose effects follow
//! from the physics of what it is made of. Knowing a craft is not the same as holding
//! it: the thing must be made, from materials, and it wears out.

use crate::agent::name_of;
use crate::brain::Action;
use crate::craft::{self, Craft, Ing, Process, Slot, MAT_NAMES, N_MAT, N_PROP};
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
pub const E_STORE: usize = 11; // carrying capacity
pub const E_SHELTER: usize = 12; // warmth and walls where one lives
pub const N_EFFECT: usize = 13;
pub const EFFECT_NAMES: [&str; N_EFFECT] =
    ["gather", "metabolism", "attack", "defense", "farm", "resist", "teach", "invent", "share", "soil", "sea", "store", "shelter"];

/// Knowledge is a bitset of fixed width, so a world holds at most this many innovations at once.
/// It used to be one u128, and a thriving world filled all 128 slots by about tick 6500 and then
/// could never invent anything again: development stopped dead while the society lived on. The
/// bitset is now eight words wide.
pub const KNOWN_WORDS: usize = 8;

/// What one mind knows: one bit per innovation in the world's registry.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Known(pub [u64; KNOWN_WORDS]);

impl Known {
    pub const EMPTY: Known = Known([0; KNOWN_WORDS]);
    #[inline]
    pub fn has(&self, i: usize) -> bool {
        i < MAX_INNOVATIONS && self.0[i >> 6] >> (i & 63) & 1 != 0
    }
    #[inline]
    pub fn set(&mut self, i: usize) {
        if i < MAX_INNOVATIONS {
            self.0[i >> 6] |= 1u64 << (i & 63);
        }
    }
    #[inline]
    pub fn unset(&mut self, i: usize) {
        if i < MAX_INNOVATIONS {
            self.0[i >> 6] &= !(1u64 << (i & 63));
        }
    }
    #[inline]
    pub fn count(&self) -> u32 {
        self.0.iter().map(|w| w.count_ones()).sum()
    }
    #[inline]
    pub fn any(&self) -> bool {
        self.0.iter().any(|w| *w != 0)
    }
    #[inline]
    pub fn union_with(&mut self, o: &Known) {
        for k in 0..KNOWN_WORDS {
            self.0[k] |= o.0[k];
        }
    }
    /// What `self` holds that neither `a` nor `b` does: what one could still teach the other.
    pub fn beyond(&self, a: &Known, b: &Known) -> Known {
        let mut out = Known::EMPTY;
        for k in 0..KNOWN_WORDS {
            out.0[k] = self.0[k] & !a.0[k] & !b.0[k];
        }
        out
    }
    /// The first `n` slots, all set: everything a world of `n` innovations could know.
    pub fn all_upto(n: usize) -> Known {
        let mut out = Known::EMPTY;
        for i in 0..n.min(MAX_INNOVATIONS) {
            out.set(i);
        }
        out
    }
    /// Every slot that is set, lowest first.
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        (0..KNOWN_WORDS).flat_map(move |k| {
            let mut w = self.0[k];
            std::iter::from_fn(move || {
                if w == 0 {
                    return None;
                }
                let b = w.trailing_zeros() as usize;
                w &= w - 1;
                Some(k * 64 + b)
            })
        })
    }
}

/// How many innovations a world can hold at once.
pub const MAX_INNOVATIONS: usize = KNOWN_WORDS * 64;

#[derive(Clone)]
pub struct Innovation {
    pub name: String,
    pub tier: u8,
    pub effects: [f32; N_EFFECT],
    pub born_tick: u64,
    pub lineage: u32,
    /// Some for a made thing, None for a practice.
    pub craft: Option<Craft>,
}

impl Innovation {
    /// A practice: an idea about farming, health, teaching, sharing or invention.
    /// `doing` biases which benefit appears; `tier` scales it. Tools and weapons are
    /// not ideas; they have to be made (see `crafted`).
    pub fn practice(rng: &mut Rng, id: usize, salt: u64, tier: u8, doing: Action, settled: bool, sick: bool, tick: u64, lineage: u32) -> Option<Innovation> {
        let mut effects = [0.0f32; N_EFFECT];
        let scale = 0.15 * (tier as f32).powf(0.8);
        let magnitude = |rng: &mut Rng| scale * (0.7 + 0.6 * rng.f32());

        let pool: &[usize] = match doing {
            Action::Gather if settled => &[E_FARM, E_FARM, E_RESIST, E_SOIL],
            Action::Gather => &[E_FARM, E_RESIST],
            Action::Share => &[E_TEACH, E_SHARE, E_INVENT],
            Action::Reproduce => &[E_RESIST, E_SHARE],
            Action::Rest if sick => &[E_RESIST, E_RESIST],
            Action::Rest => &[E_INVENT, E_TEACH, E_RESIST],
            Action::Attack | Action::Craft => return None,
        };
        let primary = pool[rng.range(pool.len())];
        let m = magnitude(rng);
        effects[primary] += if primary == E_SOIL { -m } else { m };
        if rng.f32() < 0.3 {
            let all = [E_FARM, E_RESIST, E_TEACH, E_INVENT, E_SHARE];
            let d = all[rng.range(all.len())];
            effects[d] += magnitude(rng) * 0.5;
        }
        // Every idea has a price: it burns more energy, or it wears the land.
        let price = m * (0.4 + 0.6 * rng.f32());
        let p_soil_price = if primary == E_SOIL { 0.0 } else if primary == E_FARM { 0.75 } else { 0.35 };
        if rng.f32() < p_soil_price {
            effects[E_SOIL] += price;
        } else {
            effects[E_METABOLISM] += price;
        }
        Some(Innovation { name: Self::coined(id, salt), tier, effects, born_tick: tick, lineage, craft: None })
    }

    /// A made thing, from a process applied to parts whose properties are given.
    #[allow(clippy::too_many_arguments)]
    pub fn crafted(id: usize, salt: u64, process: Process, parts: &[Ing], part_props: &[[f32; N_PROP]], props: [f32; N_PROP], depth: u8, cost: [u8; N_MAT], tick: u64, lineage: u32) -> Innovation {
        let (effects, slot, _) = craft::effects_of(&props, parts.len(), craft::bodies_in(part_props));
        let mut ps = [craft::NO_ING; 3];
        ps[..parts.len()].copy_from_slice(parts);
        let c = Craft { process, parts: ps, n_parts: parts.len() as u8, props, slot, life: craft::life_of(&props, slot), cost, depth };
        Innovation { name: Self::coined(id, salt), tier: craft::tier_of(&props, depth), effects, born_tick: tick, lineage, craft: Some(c) }
    }

    /// A name for a new thing. Each world coins its own: the same slot in two worlds gets two
/// different words, because the world's seed goes into the name. Without this every world's
/// first invention carried the same name and the whole thing read like a script it was not.
fn coined(id: usize, salt: u64) -> String {
    let mut z = (id as u64).wrapping_add(1).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ salt.wrapping_mul(0xD6E8_FEB8_6659_FD93);
    z ^= z >> 29;
    z = z.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z ^= z >> 32;
    name_of(1_000_000 + (z % 900_000_000) as u32)
}

/// Effects as text: "+farm 0.21, +soil 0.12".
    pub fn effects_text(&self) -> String {
        let mut parts = Vec::new();
        for d in 0..N_EFFECT {
            let v = self.effects[d];
            if v.abs() >= 0.01 {
                parts.push(format!("{}{} {:.2}", if v > 0.0 { "+" } else { "-" }, EFFECT_NAMES[d], v.abs()));
            }
        }
        parts.join(", ")
    }

    /// Short label like "Kesh (t2: +farm .21, +soil .12)" or
    /// "Kesh = bind(sharpen(stone), wood, fibre) -> tool (+gather .35, +metabolism .1)".
    pub fn describe(&self, registry: &[Innovation]) -> String {
        match &self.craft {
            None => format!("{} (t{}: {})", self.name, self.tier, self.effects_text()),
            Some(c) => {
                let named = |ing: Ing| ing_name(ing, registry);
                let parts = &c.parts[..c.n_parts as usize];
                format!("{} = {} -> {} (t{}: {})", self.name, craft::recipe_text(c.process, parts, &named), slot_name(c.slot), self.tier, self.effects_text())
            }
        }
    }
}

pub fn slot_name(s: Slot) -> &'static str {
    craft::SLOT_NAMES[s as usize]
}

/// The name of an ingredient: a raw material, or a made thing's recipe spelled out.
pub fn ing_name(ing: Ing, registry: &[Innovation]) -> String {
    let i = ing as usize;
    if i < N_MAT {
        return MAT_NAMES[i].to_string();
    }
    match registry.get(i - N_MAT) {
        Some(inn) => match &inn.craft {
            Some(c) => craft::recipe_text(c.process, &c.parts[..c.n_parts as usize], &|p| ing_name(p, registry)),
            None => inn.name.clone(),
        },
        None => "?".to_string(),
    }
}

/// Sum the effects of every practice an agent knows. Crafts count only when held (gear).
pub fn capabilities(known: &Known, registry: &[Innovation]) -> [f32; N_EFFECT] {
    let mut caps = [0.0f32; N_EFFECT];
    for i in known.iter() {
        if let Some(inn) = registry.get(i) {
            if inn.craft.is_none() {
                for d in 0..N_EFFECT {
                    caps[d] += inn.effects[d];
                }
            }
        }
    }
    caps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_holds_every_slot_it_should() {
        let mut k = Known::EMPTY;
        assert_eq!(k.count(), 0);
        assert!(!k.any());
        for i in [0, 1, 63, 64, 65, 127, 128, 300, MAX_INNOVATIONS - 1] {
            k.set(i);
            assert!(k.has(i), "slot {i} should be set");
        }
        assert_eq!(k.count(), 9);
        assert_eq!(k.iter().count(), 9);
        assert_eq!(k.iter().next(), Some(0));
        assert_eq!(k.iter().last(), Some(MAX_INNOVATIONS - 1));
    }

    #[test]
    fn known_ignores_slots_past_the_end() {
        // Out of range must be dropped, never wrap round onto slot 0 and hand someone
        // knowledge they never had.
        let mut k = Known::EMPTY;
        k.set(MAX_INNOVATIONS);
        k.set(MAX_INNOVATIONS + 64);
        assert_eq!(k.count(), 0);
        assert!(!k.has(MAX_INNOVATIONS));
        assert!(!k.has(0));
    }

    #[test]
    fn known_unset_and_union() {
        let mut a = Known::EMPTY;
        a.set(5);
        a.set(200);
        let mut b = Known::EMPTY;
        b.set(200);
        b.set(400);
        a.union_with(&b);
        assert_eq!(a.count(), 3);
        a.unset(200);
        assert!(!a.has(200));
        assert_eq!(a.count(), 2);
    }

    #[test]
    fn beyond_is_what_only_the_teacher_knows() {
        let mut teacher = Known::EMPTY;
        for i in [1, 70, 300] {
            teacher.set(i);
        }
        let mut learner = Known::EMPTY;
        learner.set(1);
        let mut already = Known::EMPTY;
        already.set(70);
        let left = teacher.beyond(&learner, &already);
        assert_eq!(left.iter().collect::<Vec<_>>(), vec![300]);
    }

    #[test]
    fn all_upto_is_exactly_that_many() {
        assert_eq!(Known::all_upto(0).count(), 0);
        assert_eq!(Known::all_upto(130).count(), 130);
        assert!(Known::all_upto(130).has(129));
        assert!(!Known::all_upto(130).has(130));
        assert_eq!(Known::all_upto(MAX_INNOVATIONS + 50).count(), MAX_INNOVATIONS as u32);
    }

    #[test]
    fn each_world_coins_its_own_words() {
        // The same slot in two worlds must not produce the same name, or identical-looking
        // histories make the thing read as a script.
        let a: Vec<String> = (0..8).map(|i| Innovation::coined(i, 3)).collect();
        let b: Vec<String> = (0..8).map(|i| Innovation::coined(i, 5)).collect();
        assert_ne!(a, b);
        assert_eq!(a, (0..8).map(|i| Innovation::coined(i, 3)).collect::<Vec<_>>());
        assert!(a.iter().all(|n| !n.is_empty()));
    }
}
