use crate::brain::{Action, Genome, N_ACT};

/// Behaviour profile dimensions: the five action frequencies plus mobility.
pub const N_PROFILE: usize = N_ACT + 1;

/// Culture: knowledge held by an agent, learned by discovery or from neighbours.
/// Never inherited through genes.
pub const TOOLS: u8 = 1;
pub const FARMING: u8 = 2;
pub const WEAPONS: u8 = 4;
pub const COOKING: u8 = 8;
pub const N_TECH: usize = 4;
pub const TECH_NAMES: [&str; N_TECH] = ["tools", "farming", "weapons", "cooking"];
pub const TECH_BITS: [u8; N_TECH] = [TOOLS, FARMING, WEAPONS, COOKING];

#[derive(Clone)]
pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub energy: f32,
    pub inventory: f32,
    pub age: u32,
    pub lineage: u32,
    pub genome: Genome,
    pub attacked_timer: u8,
    pub last_action: Action,
    pub children: u16,
    /// Exponentially decayed history of what this agent actually does.
    /// This is its observed strategy, whatever its genes say.
    pub profile: [f32; N_PROFILE],
    pub tech: u8,
    /// Consecutive ticks spent (nearly) still. Farming only works when settled.
    pub still: u16,
}

impl Agent {
    #[inline]
    pub fn knows(&self, bit: u8) -> bool {
        self.tech & bit != 0
    }

    #[inline]
    pub fn record(&mut self, action: Action, moved: f32) {
        const DECAY: f32 = 0.98;
        for v in self.profile.iter_mut() {
            *v *= DECAY;
        }
        self.profile[action as usize] += 1.0 - DECAY;
        // moved is in [0, sqrt 2]; normalise so every dimension lives in [0, 1].
        self.profile[N_ACT] += (1.0 - DECAY) * moved * 0.7071;
        if moved < 0.3 {
            self.still = self.still.saturating_add(1);
        } else {
            self.still = 0;
        }
    }
}

/// What a brain decided this tick, resolved after all brains have run
/// so that decision order never leaks information.
#[derive(Clone, Copy)]
pub struct Decision {
    pub mx: f32,
    pub my: f32,
    pub action: Action,
    pub target: u32, // nearest agent index at decision time, u32::MAX if none
}
