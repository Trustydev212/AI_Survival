//! A tiny recurrent MLP brain plus the genome that encodes it and the agent's temperament.
//! Learning happens only through inheritance and mutation (neuroevolution).

use crate::rng::Rng;

pub const N_MEM: usize = 4;
pub const N_IN: usize = 41;
pub const N_HID: usize = 16;
pub const N_ACT: usize = 5;
pub const N_OUT: usize = 3 + N_ACT + N_MEM; // move_x, move_y, go/stay, action scores, memory
pub const N_WEIGHTS: usize = N_IN * N_HID + N_HID + N_HID * N_OUT + N_OUT;
pub const N_TEMPER: usize = 8; // 4 emotion decay genes, 4 emotion sensitivity genes

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

    /// 1.0 = identical markers, 0.0 = maximally different.
    #[inline]
    pub fn kinship(&self, other: &Genome) -> f32 {
        let d = (0..3).map(|i| (self.marker[i] - other.marker[i]).powi(2)).sum::<f32>().sqrt();
        1.0 - d / 1.732
    }

    /// Forward pass. Returns (move_x, move_y, action, memory). Movement is zero when the
    /// go/stay output is negative, so staying put is one sign flip away from roaming.
    pub fn think(&self, input: &[f32; N_IN]) -> (f32, f32, Action, [f32; N_MEM]) {
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
            mem[m] = fast_tanh(out[3 + N_ACT + m]);
        }
        if out[2] <= 0.0 {
            return (0.0, 0.0, Action::ALL[best], mem);
        }
        (fast_tanh(out[0]), fast_tanh(out[1]), Action::ALL[best], mem)
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
