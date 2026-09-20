use crate::brain::{Action, Genome, N_ACT};

/// Behaviour profile dimensions: the five action frequencies plus mobility.
pub const N_PROFILE: usize = N_ACT + 1;

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
}

impl Agent {
    #[inline]
    pub fn record(&mut self, action: Action, moved: f32) {
        const DECAY: f32 = 0.98;
        for v in self.profile.iter_mut() {
            *v *= DECAY;
        }
        self.profile[action as usize] += 1.0 - DECAY;
        // moved is in [0, sqrt 2]; normalise so every dimension lives in [0, 1].
        self.profile[N_ACT] += (1.0 - DECAY) * moved * 0.7071;
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
