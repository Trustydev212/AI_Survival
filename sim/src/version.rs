//! Which world the rules make.
//!
//! Every number measured in this repository is a measurement of *a* world, and the rules of that
//! world have changed several times: the sea arrived, then materials and crafting, then hearing
//! strangers, then the ceiling on how much a world may know. Each change silently invalidated
//! every table measured before it, and more than one claim in docs/THEORY.md had to be withdrawn
//! for exactly that reason and no other.
//!
//! So the rules carry a version. Bump it in the same commit that changes how the world behaves,
//! say what changed in the note, and the lab will then refuse to compare arms measured on
//! different worlds instead of quietly averaging two different universes together. If a change
//! moves a behaviour fingerprint in tools/check.py and the version has not moved with it, that is
//! the mistake this file exists to catch.

/// The rules of the world. Bump on any change to how the world behaves.
pub const WORLD: u32 = 6;

/// What this version is, in one line, newest first in the list below.
pub const NOTE: &str = "room for 2048 things in one world, so a long run does not hit a ceiling";

/// The history, so an old report can be placed.
///
/// - v6: the ceiling went to 2048, after a world running free reached 437 by tick 66,000.
/// - v5: the ceiling on innovations went from 128 to 512, and names are coined per world.
/// - v4: an actor-critic learner and sideways transfer of what was learned (both off by default).
/// - v3: herds off by default; strangers heard as loudly as kin.
/// - v2: materials, crafting, gear, shelters, storehouses.
/// - v1: sea and boats.
pub const HISTORY: [&str; 6] = [
    "v1: sea and boats",
    "v2: materials, crafting, gear, shelters",
    "v3: herds off, strangers heard",
    "v4: actor-critic learning, know-how passed sideways",
    "v5: 512 innovations, names coined per world",
    "v6: 2048 innovations, so a world left running does not stop inventing",
];
