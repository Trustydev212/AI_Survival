use crate::brain::{Action, Genome, N_ACT, N_MEM};
use crate::innovation::N_EFFECT;
use crate::orders::Order;

/// Behaviour profile dimensions: the five action frequencies plus mobility.
pub const N_PROFILE: usize = N_ACT + 1;

/// Culture: knowledge is a set of world-specific innovations (see innovation.rs),
/// learned by discovery or from neighbours. Never inherited through genes.

/// Emotions: fast internal state in [0, 1], driven by events, decaying at a heritable rate.
pub const FEAR: usize = 0;
pub const ANGER: usize = 1;
pub const JOY: usize = 2;
pub const BOND: usize = 3;
pub const N_EMO: usize = 4;
pub const EMO_NAMES: [&str; N_EMO] = ["fear", "anger", "joy", "bond"];

/// Skills grow with practice within one life and are never inherited.
pub const SK_GATHER: usize = 0;
pub const SK_FIGHT: usize = 1;
pub const SK_FARM: usize = 2;
pub const N_SKILL: usize = 3;
pub const SKILL_NAMES: [&str; N_SKILL] = ["gathering", "fighting", "farming"];

pub const NO_LEADER: u32 = u32::MAX;

/// Deterministic pronounceable name from an id, so leaders can be talked about.
pub fn name_of(id: u32) -> String {
    const ON: [&str; 12] = ["k", "t", "m", "r", "s", "n", "v", "l", "d", "b", "h", "z"];
    const VO: [&str; 6] = ["a", "e", "i", "o", "u", "ai"];
    const END: [&str; 8] = ["", "n", "r", "sh", "l", "k", "th", "m"];
    let mut x = id.wrapping_mul(2654435761) ^ 0x9E37;
    let mut out = String::new();
    let syllables = 2 + (x % 2) as usize;
    for i in 0..syllables {
        x = x.wrapping_mul(1103515245).wrapping_add(12345);
        let c = ON[(x >> 8) as usize % ON.len()];
        let v = VO[(x >> 16) as usize % VO.len()];
        if i == 0 {
            out.push_str(&c.to_uppercase());
        } else {
            out.push_str(c);
        }
        out.push_str(v);
    }
    out.push_str(END[(x >> 24) as usize % END.len()]);
    out
}

#[derive(Clone)]
pub struct Agent {
    /// Unique for the whole run, so a viewer can follow one life across frames.
    pub id: u32,
    pub x: f32,
    pub y: f32,
    /// Last movement, for drawing.
    pub mdx: f32,
    pub mdy: f32,
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
    /// Bitset of known innovations, indexed into the world's registry.
    pub known: u64,
    /// Summed effects of everything known; refreshed whenever `known` changes.
    pub caps: [f32; N_EFFECT],
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
    pub skill: [f32; N_SKILL],
    /// Standing among kin, earned by deeds and slowly forgotten.
    pub prestige: f32,
    /// Index of the leader this agent follows this tick, or NO_LEADER.
    pub leader: u32,
    pub followers: u16,
    pub is_leader: bool,
    /// Consecutive ticks as a leader. A name is earned, not given.
    pub tenure: u16,
    /// 0 = never led; otherwise the id its name is derived from.
    pub name: u32,
    /// The order this agent is currently giving. Only reaches anyone while it leads.
    pub order: Order,
    pub order_dx: f32,
    pub order_dy: f32,
    /// The order this agent was under when it last acted, and whether it obeyed.
    pub under: Option<Order>,
    pub obeyed: bool,
    /// A custom: an order internalised from leaders, kept and spread without them.
    pub custom: Option<Order>,
    pub custom_dx: f32,
    pub custom_dy: f32,
    pub custom_strength: f32,
    /// Cached from the genome at birth: these never change within a life.
    pub charisma: f32,
    pub emo_decay: [f32; N_EMO],
    pub emo_sens: [f32; N_EMO],
}

impl Agent {
    #[inline]
    pub fn known_count(&self) -> u32 {
        self.known.count_ones()
    }

    #[inline]
    pub fn train(&mut self, sk: usize, amount: f32) {
        self.skill[sk] = (self.skill[sk] + amount).min(1.0);
    }

    #[inline]
    pub fn feel(&mut self, e: usize, amount: f32) {
        self.emotion[e] = (self.emotion[e] + amount * self.emo_sens[e]).clamp(0.0, 1.0);
    }

    /// Is `other` close kin? Squared marker distance against the threshold, no sqrt.
    #[inline]
    pub fn is_kin(&self, other: &Agent, kin_threshold: f32) -> bool {
        let a = &self.genome.marker;
        let b = &other.genome.marker;
        let d2 = (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2);
        let lim = (1.0 - kin_threshold) * 1.732;
        d2 <= lim * lim
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
    pub leader: u32,
    /// The order this agent issues this tick.
    pub order: Order,
    pub odx: f32,
    pub ody: f32,
    /// The order it was under while deciding, and that order's direction.
    pub under: Option<Order>,
    pub under_dx: f32,
    pub under_dy: f32,
    /// Whether that order came from a living leader (true) or from custom (false).
    pub from_leader: bool,
}

impl Default for Decision {
    fn default() -> Self {
        Decision {
            mx: 0.0,
            my: 0.0,
            action: Action::Rest,
            target: u32::MAX,
            memory: [0.0; N_MEM],
            leader: NO_LEADER,
            order: Order::Hold,
            odx: 0.0,
            ody: 0.0,
            under: None,
            under_dx: 0.0,
            under_dy: 0.0,
            from_leader: false,
        }
    }
}
