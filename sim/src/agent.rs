use crate::brain::{Action, Genome, N_ACT, N_MEM};

/// Behaviour profile dimensions: the five action frequencies plus mobility.
pub const N_PROFILE: usize = N_ACT + 1;

/// Culture: knowledge held by an agent, learned by discovery or from neighbours.
/// Never inherited through genes. Higher techs need lower ones plus social conditions.
pub const TOOLS: u16 = 1;
pub const FARMING: u16 = 2;
pub const WEAPONS: u16 = 4;
pub const COOKING: u16 = 8;
pub const METAL: u16 = 16;
pub const IRRIGATION: u16 = 32;
pub const WALLS: u16 = 64;
pub const MEDICINE: u16 = 128;
pub const WRITING: u16 = 256;
pub const N_TECH: usize = 9;
pub const ALL_TECH: u16 = 0x1FF;
pub const TECH_NAMES: [&str; N_TECH] =
    ["tools", "farming", "weapons", "cooking", "metal", "irrigation", "walls", "medicine", "writing"];
pub const TECH_BITS: [u16; N_TECH] = [TOOLS, FARMING, WEAPONS, COOKING, METAL, IRRIGATION, WALLS, MEDICINE, WRITING];

/// Emotions: fast internal state in [0, 1], driven by events, decaying at a heritable rate.
pub const FEAR: usize = 0;
pub const ANGER: usize = 1;
pub const JOY: usize = 2;
pub const BOND: usize = 3;
pub const N_EMO: usize = 4;
pub const EMO_NAMES: [&str; N_EMO] = ["fear", "anger", "joy", "bond"];

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
    pub profile: [f32; N_PROFILE],
    pub tech: u16,
    /// Consecutive ticks spent still. Farming only works when settled.
    pub still: u16,
    pub emotion: [f32; N_EMO],
    /// Recurrent memory: written by the brain, read back next tick. Its "thoughts".
    pub memory: [f32; N_MEM],
    pub has_home: bool,
    pub home_x: f32,
    pub home_y: f32,
    /// Ticks of illness left; 0 = healthy.
    pub sick: u16,
    /// Ticks of immunity left after recovering.
    pub immune: u16,
}

impl Agent {
    #[inline]
    pub fn knows(&self, bit: u16) -> bool {
        self.tech & bit != 0
    }

    #[inline]
    pub fn feel(&mut self, e: usize, amount: f32) {
        let s = self.genome.emo_sensitivity(e);
        self.emotion[e] = (self.emotion[e] + amount * s).clamp(0.0, 1.0);
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
    pub memory: [f32; N_MEM],
}
