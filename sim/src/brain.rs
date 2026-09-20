//! A tiny MLP brain plus the genome that encodes it.
//! Learning happens only through inheritance and mutation (neuroevolution).

use crate::rng::Rng;

pub const N_IN: usize = 20;
pub const N_HID: usize = 16;
pub const N_ACT: usize = 5;
pub const N_OUT: usize = 2 + N_ACT; // move_x, move_y, then action scores
pub const N_WEIGHTS: usize = N_IN * N_HID + N_HID + N_HID * N_OUT + N_OUT;

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
}

impl Genome {
    pub fn random(rng: &mut Rng, marker: [f32; 3]) -> Genome {
        let weights = (0..N_WEIGHTS).map(|_| rng.normal() * 0.5).collect();
        Genome { weights, marker }
    }

    pub fn mutated(&self, rng: &mut Rng, p_mut: f32, sigma: f32) -> Genome {
        let mut g = self.clone();
        for w in g.weights.iter_mut() {
            if rng.f32() < p_mut {
                *w += rng.normal() * sigma;
            }
        }
        for m in g.marker.iter_mut() {
            *m = (*m + rng.normal() * 0.02).clamp(0.0, 1.0);
        }
        g
    }

    /// 1.0 = identical markers, 0.0 = maximally different.
    #[inline]
    pub fn kinship(&self, other: &Genome) -> f32 {
        let d = (0..3).map(|i| (self.marker[i] - other.marker[i]).powi(2)).sum::<f32>().sqrt();
        1.0 - d / 1.732
    }

    /// Forward pass. Returns (move_x, move_y, action).
    pub fn think(&self, input: &[f32; N_IN]) -> (f32, f32, Action) {
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
            if out[2 + a] > out[2 + best] {
                best = a;
            }
        }
        (fast_tanh(out[0]), fast_tanh(out[1]), Action::ALL[best])
    }
}

#[inline]
fn fast_tanh(x: f32) -> f32 {
    let x = x.clamp(-4.5, 4.5);
    let x2 = x * x;
    x * (27.0 + x2) / (27.0 + 9.0 * x2)
}
