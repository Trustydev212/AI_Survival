//! A tiny recurrent MLP brain plus the genome that encodes it and the agent's temperament.
//! Learning happens only through inheritance and mutation (neuroevolution).

use crate::orders::{Order, N_ORDER};
use crate::rng::Rng;

pub const N_MEM: usize = 4;
pub const N_IN: usize = 75;
pub const N_HID: usize = 16;
pub const N_ACT: usize = 5;
// move_x, move_y, go/stay, action scores, memory, order scores, order direction
pub const N_OUT: usize = 3 + N_ACT + N_MEM + N_ORDER + 2;
const O_MEM: usize = 3 + N_ACT;
const O_ORDER: usize = O_MEM + N_MEM;
const O_ORDER_DIR: usize = O_ORDER + N_ORDER;
pub const N_WEIGHTS: usize = N_IN * N_HID + N_HID + N_HID * N_OUT + N_OUT;
pub const N_TEMPER: usize = 9; // 4 emotion decay genes, 4 emotion sensitivity genes, 1 charisma gene

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Action {
    Gather = 0,
    Attack = 1,
    Share = 2,
    Reproduce = 3,
    Rest = 4,
}

impl Action {
    pub const ALL: [Action; N_ACT] = [Action::Gather, Action::Attack, Action::Share, Action::Reproduce, Action::Rest];
    pub fn name(self) -> &'static str {
        match self {
            Action::Gather => "gather",
            Action::Attack => "attack",
            Action::Share => "share",
            Action::Reproduce => "repro",
            Action::Rest => "rest",
        }
    }
}

#[derive(Clone)]
pub struct Genome {
    pub weights: Vec<f32>,
    /// Heritable "phenotype colour"; kin recognition is based on marker distance.
    pub marker: [f32; 3],
    /// Temperament: how fast each emotion fades and how strongly it is felt.
    pub temper: [f32; N_TEMPER],
}

impl Genome {
    pub fn random(rng: &mut Rng, marker: [f32; 3]) -> Genome {
        let weights = (0..N_WEIGHTS).map(|_| rng.normal() * 0.5).collect();
        let mut temper = [0.0; N_TEMPER];
        for t in temper.iter_mut() {
            *t = rng.normal();
        }
        Genome { weights, marker, temper }
    }

    pub fn mutated(&self, rng: &mut Rng, p_mut: f32, sigma: f32) -> Genome {
        let mut g = self.clone();
        for w in g.weights.iter_mut() {
            if rng.f32() < p_mut {
                *w += rng.normal() * sigma;
            }
        }
        for t in g.temper.iter_mut() {
            if rng.f32() < p_mut {
                *t += rng.normal() * sigma;
            }
        }
        for m in g.marker.iter_mut() {
            *m = (*m + rng.normal() * 0.02).clamp(0.0, 1.0);
        }
        g
    }

    /// Per-tick retention of emotion e, in [0.90, 0.999]: from hot-headed to brooding.
    #[inline]
    pub fn emo_decay(&self, e: usize) -> f32 {
        0.90 + 0.099 * sigmoid(self.temper[e])
    }

    /// How strongly emotion e responds to events, in [0.25, 4].
    #[inline]
    pub fn emo_sensitivity(&self, e: usize) -> f32 {
        self.temper[4 + e].exp().clamp(0.25, 4.0)
    }

    /// Natural pull on others, in [0, 1]. Prestige is multiplied by it when choosing whom to follow.
    #[inline]
    pub fn charisma(&self) -> f32 {
        sigmoid(self.temper[8])
    }

    /// Learn by imitation: pull this brain a fraction of the way toward a model's.
    pub fn imitate(&mut self, model: &Genome, rate: f32) {
        for (w, m) in self.weights.iter_mut().zip(model.weights.iter()) {
            *w += (m - *w) * rate;
        }
    }

    /// 1.0 = identical markers, 0.0 = maximally different.
    #[inline]
    pub fn kinship(&self, other: &Genome) -> f32 {
        let d = (0..3).map(|i| (self.marker[i] - other.marker[i]).powi(2)).sum::<f32>().sqrt();
        1.0 - d / 1.732
    }

    /// Forward pass. Movement is zero when the go/stay output is negative, so staying
    /// put is one sign flip away from roaming. Every brain also forms an order; it only
    /// reaches anyone if this agent happens to be a leader.
    pub fn think(&self, input: &[f32; N_IN]) -> Thought {
        let w = &self.weights;
        let mut hidden = [0.0f32; N_HID];
        let mut off = 0;
        for h in 0..N_HID {
            let mut acc = 0.0;
            for i in 0..N_IN {
                acc += w[off + h * N_IN + i] * input[i];
            }
            hidden[h] = acc;
        }
        off += N_IN * N_HID;
        for h in 0..N_HID {
            hidden[h] = fast_tanh(hidden[h] + w[off + h]);
        }
        off += N_HID;
        let mut out = [0.0f32; N_OUT];
        for o in 0..N_OUT {
            let mut acc = 0.0;
            for h in 0..N_HID {
                acc += w[off + o * N_HID + h] * hidden[h];
            }
            out[o] = acc;
        }
        off += N_HID * N_OUT;
        for o in 0..N_OUT {
            out[o] += w[off + o];
        }
        let mut best = 0;
        for a in 1..N_ACT {
            if out[3 + a] > out[3 + best] {
                best = a;
            }
        }
        let mut mem = [0.0f32; N_MEM];
        for m in 0..N_MEM {
            mem[m] = fast_tanh(out[O_MEM + m]);
        }
        let mut best_order = 0;
        for o in 1..N_ORDER {
            if out[O_ORDER + o] > out[O_ORDER + best_order] {
                best_order = o;
            }
        }
        let (odx, ody) = {
            let (x, y) = (fast_tanh(out[O_ORDER_DIR]), fast_tanh(out[O_ORDER_DIR + 1]));
            let len = (x * x + y * y).sqrt();
            if len < 0.01 { (0.0, 0.0) } else { (x / len, y / len) }
        };
        let order = Order::ALL[best_order];
        if out[2] <= 0.0 {
            return Thought { mx: 0.0, my: 0.0, action: Action::ALL[best], memory: mem, order, odx, ody };
        }
        Thought { mx: fast_tanh(out[0]), my: fast_tanh(out[1]), action: Action::ALL[best], memory: mem, order, odx, ody }
    }
}

#[inline]
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

#[inline]
fn fast_tanh(x: f32) -> f32 {
    let x = x.clamp(-4.5, 4.5);
    let x2 = x * x;
    x * (27.0 + x2) / (27.0 + 9.0 * x2)
}

/// One tick of a brain's output.
pub struct Thought {
    pub mx: f32,
    pub my: f32,
    pub action: Action,
    pub memory: [f32; N_MEM],
    pub order: Order,
    pub odx: f32,
    pub ody: f32,
}
