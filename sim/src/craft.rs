//! Materials, processes and the things made from them.
//!
//! Nothing in this file names a human invention. It defines a small physics of matter:
//! six raw materials with measurable properties, five ways of working them, and rules
//! that turn the properties of a made thing into what it is good for. Boats, axes,
//! pots, walls and fire are not listed anywhere; they fall out of the rules when an
//! agent happens to bind a sharpened stone to a stick, hollow a log, or strike two
//! stones together. What a world discovers depends on what its people try.

use crate::innovation::{
    E_ATTACK, E_DEFENSE, E_FARM, E_GATHER, E_METABOLISM, E_RESIST, E_SEA, E_SHELTER, E_SOIL, E_STORE, N_EFFECT,
};

// ---------- raw materials
pub const M_WOOD: usize = 0;
pub const M_STONE: usize = 1;
pub const M_FIBRE: usize = 2;
pub const M_CLAY: usize = 3;
pub const M_ORE: usize = 4;
pub const M_BONE: usize = 5;
pub const N_MAT: usize = 6;
pub const MAT_NAMES: [&str; N_MAT] = ["wood", "stone", "fibre", "clay", "ore", "bone"];

// ---------- properties of matter, each in [0, 1]
pub const P_HARD: usize = 0; // resists deformation
pub const P_EDGE: usize = 1; // holds an edge
pub const P_BUOY: usize = 2; // floats
pub const P_FLEX: usize = 3; // bends without breaking
pub const P_HEAT: usize = 4; // survives fire (fired clay, metal)
pub const P_BIND: usize = 5; // can tie other things together
pub const P_MASS: usize = 6; // weight, relative to a stone
pub const P_HOLLOW: usize = 7; // has an inside that holds something
pub const P_FIRE: usize = 8; // is burning / gives heat
pub const P_HANDLE: usize = 9; // a rigid head on a flexible shaft
pub const P_BOUND: usize = 10; // already an assembly of parts
pub const N_PROP: usize = 11;

/// hard edge buoy flex heat bind mass hollow fire handle bound
pub const RAW: [[f32; N_PROP]; N_MAT] = [
    [0.40, 0.20, 0.90, 0.50, 0.10, 0.30, 0.40, 0.00, 0.0, 0.0, 0.0], // wood
    [0.90, 0.35, 0.00, 0.00, 0.80, 0.00, 0.90, 0.00, 0.0, 0.0, 0.0], // stone (a rock is not sharp until knapped)
    [0.05, 0.00, 0.30, 0.95, 0.00, 0.90, 0.10, 0.00, 0.0, 0.0, 0.0], // fibre
    [0.20, 0.00, 0.00, 0.60, 0.30, 0.50, 0.60, 0.00, 0.0, 0.0, 0.0], // clay (raw)
    [0.60, 0.30, 0.00, 0.10, 0.90, 0.00, 1.00, 0.00, 0.0, 0.0, 0.0], // ore
    [0.60, 0.30, 0.20, 0.20, 0.30, 0.20, 0.30, 0.00, 0.0, 0.0, 0.0], // bone
];

// ---------- processes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Process {
    Bind = 0,    // tie two or three things together
    Sharpen = 1, // grind or knap an edge onto one thing
    Hollow = 2,  // carve or dig out the inside of one thing
    Strike = 3,  // hit one hard thing with another
    Fire = 4,    // put one thing in a fire (needs a fire in hand)
}
pub const N_PROC: usize = 5;
impl Process {
    pub const ALL: [Process; N_PROC] = [Process::Bind, Process::Sharpen, Process::Hollow, Process::Strike, Process::Fire];
    pub fn name(self) -> &'static str {
        match self {
            Process::Bind => "bind",
            Process::Sharpen => "sharpen",
            Process::Hollow => "hollow",
            Process::Strike => "strike",
            Process::Fire => "fire",
        }
    }
    /// How many parts the process takes: (min, max).
    pub fn arity(self) -> (usize, usize) {
        match self {
            Process::Bind => (2, 3),
            Process::Strike => (2, 2),
            _ => (1, 1),
        }
    }
}

// ---------- what a made thing is for
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slot {
    Tool = 0,    // gathering and farming
    Weapon = 1,  // attack
    Armour = 2,  // defence
    Boat = 3,    // the sea
    Vessel = 4,  // carrying and keeping
    Fire = 5,    // heat: cooking and the fire process
    Shelter = 6, // not carried: built where its maker settles
}
pub const N_SLOT: usize = 7;
impl Slot {
    pub const ALL: [Slot; N_SLOT] = [Slot::Tool, Slot::Weapon, Slot::Armour, Slot::Boat, Slot::Vessel, Slot::Fire, Slot::Shelter];
}
pub const SLOT_NAMES: [&str; N_SLOT] = ["tool", "weapon", "armour", "boat", "vessel", "fire", "shelter"];
pub const NO_ITEM: u16 = u16::MAX;

/// An ingredient: a raw material (0..N_MAT) or a made thing (N_MAT + innovation index).
pub type Ing = u16;
pub const NO_ING: Ing = u16::MAX;

/// A recipe and the thing it makes. Stored inside an Innovation.
#[derive(Clone, Debug)]
pub struct Craft {
    pub process: Process,
    pub parts: [Ing; 3],
    pub n_parts: u8,
    pub props: [f32; N_PROP],
    pub slot: Slot,
    /// Ticks of use before it is worn out.
    pub life: f32,
    /// Raw materials consumed to make one, counting the parts' own recipes.
    pub cost: [u8; N_MAT],
    /// Nesting depth: raw = 0, a thing made from raw = 1, ...
    pub depth: u8,
}

/// The properties of a made thing from its process and parts. None when the process
/// does nothing to these parts (blunt things cannot be sharpened, rope cannot be hollowed).
pub fn compose(process: Process, parts: &[[f32; N_PROP]]) -> Option<[f32; N_PROP]> {
    let (lo, hi) = process.arity();
    if parts.len() < lo || parts.len() > hi {
        return None;
    }
    let mut p = [0.0f32; N_PROP];
    match process {
        Process::Sharpen => {
            let a = parts[0];
            // Only things not yet worked to an edge; a blade cannot be sharpened again, nor a bowl.
            if a[P_HARD] < 0.3 || a[P_EDGE] >= 0.6 * a[P_HARD] || a[P_FIRE] > 0.0 || a[P_HOLLOW] > 0.0 {
                return None;
            }
            p = a;
            p[P_EDGE] = (1.25 * (a[P_HARD] * a[P_EDGE].max(0.15)).sqrt()).min(1.0);
            p[P_MASS] = a[P_MASS] * 0.85;
            p[P_HANDLE] = a[P_HANDLE];
            if p[P_EDGE] <= a[P_EDGE] + 0.05 {
                return None;
            }
        }
        Process::Hollow => {
            let a = parts[0];
            // Blades and bowls cannot be hollowed; nor rope, nor fire.
            if a[P_HARD] < 0.15 || a[P_HOLLOW] > 0.0 || a[P_FIRE] > 0.0 || a[P_FLEX] > 0.8 || a[P_EDGE] >= 0.5 {
                return None;
            }
            p = a;
            p[P_HOLLOW] = 0.85;
            p[P_MASS] = a[P_MASS] * 0.6;
            p[P_HARD] = a[P_HARD] * 0.85;
        }
        Process::Strike => {
            let (a, b) = (parts[0], parts[1]);
            let hard = a[P_HARD].min(b[P_HARD]);
            if hard < 0.75 || a[P_HOLLOW] > 0.0 || b[P_HOLLOW] > 0.0 {
                return None;
            }
            // Sparks. The thing made is a fire: light, hot, short-lived.
            p[P_FIRE] = (hard * 1.05).min(1.0);
            p[P_HEAT] = 1.0;
            p[P_MASS] = 0.1;
        }
        Process::Bind => {
            let binder = parts.iter().map(|q| q[P_BIND]).fold(0.0, f32::max);
            if binder < 0.5 || parts.iter().any(|q| q[P_FIRE] > 0.0) {
                return None;
            }
            // At least one part must be something other than the binder itself.
            let bodies: Vec<&[f32; N_PROP]> = parts.iter().filter(|q| q[P_BIND] < 0.5 || q[P_HARD] >= 0.3).collect();
            if bodies.is_empty() {
                return None;
            }
            // Wrapping more rope around one already-bound thing makes nothing new.
            if bodies.len() == 1 && bodies[0][P_BOUND] > 0.0 {
                return None;
            }
            let mass: f32 = parts.iter().map(|q| q[P_MASS]).sum::<f32>() * 0.75;
            p[P_MASS] = mass.min(1.5);
            p[P_HARD] = bodies.iter().map(|q| q[P_HARD]).fold(0.0, f32::max) * (0.8 + 0.2 * binder);
            p[P_EDGE] = bodies.iter().map(|q| q[P_EDGE]).fold(0.0, f32::max);
            let total_mass: f32 = parts.iter().map(|q| q[P_MASS].max(0.05)).sum();
            p[P_BUOY] = parts.iter().map(|q| q[P_BUOY] * q[P_MASS].max(0.05)).sum::<f32>() / total_mass;
            p[P_FLEX] = parts.iter().map(|q| q[P_FLEX]).fold(1.0, f32::min);
            p[P_HEAT] = parts.iter().map(|q| q[P_HEAT]).fold(1.0, f32::min);
            p[P_BIND] = binder * 0.5;
            p[P_BOUND] = 1.0;
            p[P_HOLLOW] = parts.iter().map(|q| q[P_HOLLOW]).fold(0.0, f32::max) * (0.9 + 0.1 * binder);
            // A handle: a rigid, sharp or heavy head on a shaft that bends a little.
            let head = parts.iter().any(|q| q[P_HARD] >= 0.6 && q[P_HOLLOW] == 0.0);
            let shaft = parts.iter().any(|q| q[P_FLEX] >= 0.4 && q[P_FLEX] <= 0.7 && q[P_HARD] >= 0.3 && q[P_HARD] < 0.6);
            p[P_HANDLE] = if head && shaft { binder.min(1.0) } else { 0.0 };
            // Two or more heavy bodies bound together: a structure.
            if p[P_HANDLE] == 0.0 && bodies.len() >= 2 && p[P_MASS] >= 0.6 {
                p[P_HARD] = (p[P_HARD] * 1.1).min(1.0);
            }
        }
        Process::Fire => {
            let a = parts[0];
            if a[P_FIRE] > 0.0 {
                return None;
            }
            p = a;
            if a[P_HEAT] >= 0.85 && a[P_HARD] >= 0.5 && a[P_EDGE] > 0.0 && a[P_BUOY] == 0.0 && a[P_HOLLOW] == 0.0 {
                // ore-like: smelting
                p[P_HARD] = 0.95;
                p[P_EDGE] = (a[P_EDGE] + 0.5).min(0.95);
                p[P_FLEX] = 0.3;
                p[P_HEAT] = 1.0;
                p[P_MASS] = a[P_MASS] * 0.9;
            } else if a[P_BIND] >= 0.4 && a[P_FLEX] >= 0.4 && a[P_HARD] < 0.4 && a[P_BUOY] == 0.0 {
                // clay-like: firing
                p[P_HARD] = 0.7;
                p[P_FLEX] = 0.05;
                p[P_HEAT] = 0.95;
                p[P_BIND] = 0.0;
                p[P_MASS] = a[P_MASS] * 0.85;
            } else if a[P_BUOY] >= 0.6 && a[P_HEAT] < 0.3 && a[P_HOLLOW] == 0.0 {
                // wood-like: charcoal, a fire that lasts
                p = [0.0; N_PROP];
                p[P_FIRE] = 0.8;
                p[P_HEAT] = 1.0;
                p[P_MASS] = 0.2;
            } else {
                return None; // it just burns or does nothing
            }
        }
    }
    for v in p.iter_mut() {
        *v = v.clamp(0.0, 1.5);
    }
    Some(p)
}

/// What a thing with these properties is good for, in the innovation effect dimensions,
/// and the slot it belongs in. Weight is its price: everything carried burns energy.
pub fn effects_of(p: &[f32; N_PROP], n_parts: usize, n_bodies: usize) -> ([f32; N_EFFECT], Slot, f32) {
    let mut e = [0.0f32; N_EFFECT];
    let handle = 1.0 + 0.8 * p[P_HANDLE];
    let cutting = p[P_EDGE] * 0.55 * handle;
    e[E_GATHER] = (cutting + p[P_HOLLOW] * 0.15).min(1.2);
    e[E_FARM] = ((p[P_EDGE] * 0.35 + p[P_MASS] * 0.15) * handle).min(1.0);
    if p[P_EDGE] >= 0.3 || p[P_MASS] >= 0.7 {
        e[E_ATTACK] = ((p[P_EDGE] * 0.4 + p[P_MASS] * 0.3) * (1.0 + 0.4 * p[P_HANDLE])).min(1.2);
    }
    let bound = n_parts >= 2 && p[P_HANDLE] == 0.0;
    e[E_DEFENSE] = if bound { p[P_HARD] * 0.35 + p[P_FLEX] * 0.15 } else { p[P_HARD] * 0.12 };
    e[E_SEA] = if p[P_HOLLOW] >= 0.5 { p[P_BUOY] * p[P_HOLLOW] * (0.6 + 0.4 * p[P_BIND].min(1.0)) } else { 0.0 };
    e[E_STORE] = p[P_HOLLOW] * (0.35 + 0.65 * p[P_HEAT]);
    e[E_SHELTER] = if bound && n_bodies >= 2 && p[P_MASS] >= 0.6 { (p[P_MASS] * 0.3 + p[P_HARD] * 0.4 + p[P_BIND] * 0.3).min(1.2) } else { 0.0 };
    if p[P_FIRE] > 0.0 {
        e[E_RESIST] = p[P_FIRE] * 0.4; // cooked food, warmth
        e[E_METABOLISM] = -p[P_FIRE] * 0.15;
    }
    // Prices: carrying weight burns energy; digging tools wear the soil faster.
    e[E_METABOLISM] += p[P_MASS] * 0.18;
    e[E_SOIL] += e[E_FARM] * 0.3;

    let scores = [
        (Slot::Tool, e[E_GATHER].max(e[E_FARM])),
        (Slot::Weapon, e[E_ATTACK]),
        (Slot::Armour, e[E_DEFENSE]),
        (Slot::Boat, e[E_SEA]),
        (Slot::Vessel, e[E_STORE]),
        (Slot::Fire, p[P_FIRE]),
        (Slot::Shelter, e[E_SHELTER]),
    ];
    let mut best = scores[0];
    for s in scores.iter().skip(1) {
        if s.1 > best.1 {
            best = *s;
        }
    }
    // A thing kept in one slot only gives what that slot is for, plus its prices.
    let mut kept = [0.0f32; N_EFFECT];
    kept[E_METABOLISM] = e[E_METABOLISM];
    kept[E_SOIL] = e[E_SOIL];
    match best.0 {
        Slot::Tool => {
            kept[E_GATHER] = e[E_GATHER];
            kept[E_FARM] = e[E_FARM];
            kept[E_ATTACK] = e[E_ATTACK] * 0.5;
        }
        Slot::Weapon => {
            kept[E_ATTACK] = e[E_ATTACK];
            kept[E_GATHER] = e[E_GATHER] * 0.3;
        }
        Slot::Armour => kept[E_DEFENSE] = e[E_DEFENSE],
        Slot::Boat => {
            kept[E_SEA] = e[E_SEA];
            kept[E_STORE] = e[E_STORE] * 0.5;
        }
        Slot::Vessel => kept[E_STORE] = e[E_STORE],
        Slot::Fire => {
            kept[E_RESIST] = e[E_RESIST];
            kept[E_SOIL] = 0.0;
        }
        Slot::Shelter => {
            kept[E_SHELTER] = e[E_SHELTER];
            kept[E_DEFENSE] = e[E_DEFENSE];
            kept[E_METABOLISM] = 0.0; // not carried
            kept[E_SOIL] = 0.0;
        }
    }
    (kept, best.0, best.1)
}

/// How long a thing lasts in use, in ticks.
pub fn life_of(p: &[f32; N_PROP], slot: Slot) -> f32 {
    match slot {
        Slot::Fire => 120.0 + 400.0 * p[P_FIRE].max(0.0),
        Slot::Shelter => 4000.0 + 8000.0 * p[P_HARD],
        _ => 300.0 + 900.0 * p[P_HARD] * (1.0 - 0.3 * p[P_HOLLOW]),
    }
}

/// Tier for drawing and for names: 1 raw matter worked, 2 bound things, 3 fired things.
pub fn tier_of(p: &[f32; N_PROP], depth: u8) -> u8 {
    if p[P_HEAT] >= 0.9 && p[P_FIRE] == 0.0 {
        3
    } else if depth >= 2 {
        2
    } else {
        1
    }
}

/// Human-readable recipe, e.g. "bind(sharpen(stone), wood, fibre)".
pub fn recipe_text(process: Process, parts: &[Ing], name_of: &dyn Fn(Ing) -> String) -> String {
    let inner: Vec<String> = parts.iter().map(|p| name_of(*p)).collect();
    format!("{}({})", process.name(), inner.join(", "))
}

/// How many of these parts are bodies rather than binders (rope, raw clay).
pub fn bodies_in(parts: &[[f32; N_PROP]]) -> usize {
    parts.iter().filter(|q| q[P_BIND] < 0.5 || q[P_HARD] >= 0.3).count()
}

/// What a shelter looks like: 1 wood-like, 2 stone, clay or bone, 3 fired.
/// What a shelter looks like, decided by nothing but what it is made of. Five steps now, so that
/// a settlement seen from above tells you what its people have worked out: branches, then earth
/// and stone, then something fired, then something hard and heavy enough to carry its own weight
/// upward. Nobody writes an age of stone or an age of metal anywhere; the materials do it.
pub fn look_of(p: &[f32; N_PROP]) -> u8 {
    if p[P_HEAT] >= 0.9 && p[P_HARD] >= 0.8 && p[P_MASS] >= 0.7 && p[P_FIRE] == 0.0 {
        4 // worked metal: hard, heavy and unburnable
    } else if p[P_HEAT] >= 0.9 && p[P_FIRE] == 0.0 {
        3 // fired: brick and tile
    } else if p[P_BUOY] >= 0.4 {
        1 // branches and hide
    } else {
        2 // earth and stone
    }
}

/// How far a shelter reaches and how far up it goes, from its own weight and its own making.
/// A heavy thing built out of many heavy things holds a bigger floor and stacks higher; a lean-to
/// of branches does neither. Returns (half-width in cells, storeys).
pub fn shape_of(p: &[f32; N_PROP], bodies: usize) -> (u8, u8) {
    let heft = p[P_MASS] * (1.0 + p[P_HARD]) * (bodies as f32).sqrt();
    let span = if heft >= 3.4 { 2 } else if heft >= 1.7 { 1 } else { 0 };
    let storeys = if p[P_HARD] >= 0.8 && p[P_MASS] >= 0.7 && heft >= 3.0 {
        3
    } else if p[P_HARD] >= 0.6 && heft >= 2.0 {
        2
    } else {
        1
    };
    (span, storeys)
}
