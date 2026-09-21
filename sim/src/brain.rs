//! A tiny recurrent MLP brain plus the genome that encodes it and the agent's temperament.
//! Learning happens only through inheritance and mutation (neuroevolution).

use crate::orders::{Order, N_ORDER};
use crate::rng::Rng;

pub const N_MEM: usize = 4;
/// Signals: a small vector every agent broadcasts each tick and neighbours can hear.
/// What it means, if anything, is up to evolution.
pub const N_SIG: usize = 2;
/// 101 senses, plus four about a place worth remembering: whether this mind holds one, which way
/// it lies, and whether something rich is within arm's reach right now. Nothing in this world has
/// ever needed a memory of *where*, which is most likely why calls never came to mean anything:
/// a call cannot carry what the speaker cannot hold.
pub const N_IN: usize = 105;
pub const N_HID: usize = 20;
pub const N_ACT: usize = 6;
// move_x, move_y, go/stay, action scores, memory, order scores, order direction, signal
pub const N_OUT: usize = 3 + N_ACT + N_MEM + N_ORDER + 2 + N_SIG + 1;
const O_MEM: usize = 3 + N_ACT;
const O_ORDER: usize = O_MEM + N_MEM;
const O_ORDER_DIR: usize = O_ORDER + N_ORDER;
const O_SIG: usize = O_ORDER_DIR + 2;
/// One gate: mark this spot as the place worth coming back to.
const O_MARK: usize = O_SIG + N_SIG;
pub const N_WEIGHTS: usize = N_IN * N_HID + N_HID + N_HID * N_OUT + N_OUT;
/// The plastic part: the hidden-to-output layer, which changes within one life.
pub const N_PLASTIC: usize = N_HID * N_OUT;
pub const N_TEMPER: usize = 9; // 4 emotion decay genes, 4 emotion sensitivity genes, 1 charisma gene
/// Learning genes: rate, and the three Hebbian coefficients (pre*post, pre, post).
pub const N_LEARN: usize = 4;
/// The critic: one weight per hidden unit plus a bias, estimating how good the present is.
/// It is born blank and learned within one life, never inherited, so the genome is untouched
/// and every world made before the critic existed still replays identically.
pub const N_CRITIC: usize = N_HID + 1;
/// Eligibility trace over the action readout: which of those weights pushed the recent choices.
pub const N_TRACE: usize = N_ACT * N_HID;
/// Where the action scores start inside the output vector.
pub const O_ACT: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Action {
    Gather = 0,
    Attack = 1,
    Share = 2,
    Reproduce = 3,
    Rest = 4,
    /// Work what you carry: make a known thing, or try something new.
    Craft = 5,
}

impl Action {
    pub const ALL: [Action; N_ACT] = [Action::Gather, Action::Attack, Action::Share, Action::Reproduce, Action::Rest, Action::Craft];
    pub fn name(self) -> &'static str {
        match self {
            Action::Gather => "gather",
            Action::Attack => "attack",
            Action::Share => "share",
            Action::Reproduce => "repro",
            Action::Rest => "rest",
            Action::Craft => "craft",
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
    /// How this brain learns within its life: heritable rate and Hebbian shape.
    pub learn: [f32; N_LEARN],
}

impl Genome {
    pub fn random(rng: &mut Rng, marker: [f32; 3]) -> Genome {
        let weights = (0..N_WEIGHTS).map(|_| rng.normal() * 0.5).collect();
        let mut temper = [0.0; N_TEMPER];
        for t in temper.iter_mut() {
            *t = rng.normal();
        }
        let mut learn = [0.0; N_LEARN];
        for l in learn.iter_mut() {
            *l = rng.normal();
        }
        Genome { weights, marker, temper, learn }
    }

    /// A child of two, in blocks rather than weight by weight.
    ///
    /// Until this existed every child was a mutated copy of one parent, so two lineages that had
    /// each worked out half of something could never put the halves together: evolution here had
    /// only mutation, which is the slowest search there is. Crossing in stretches rather than at
    /// random keeps whatever a run of weights was doing together instead of shredding it.
    pub fn crossed(&self, other: &Genome, rng: &mut Rng) -> Genome {
        let mut g = self.clone();
        let mut from_other = rng.f32() < 0.5;
        let mut run = 0usize;
        for (i, w) in g.weights.iter_mut().enumerate() {
            if run == 0 {
                run = 8 + (rng.f32() * 40.0) as usize;
                from_other = !from_other;
            }
            run -= 1;
            if from_other {
                *w = other.weights[i];
            }
        }
        for (i, t) in g.temper.iter_mut().enumerate() {
            if rng.f32() < 0.5 {
                *t = other.temper[i];
            }
        }
        for (i, l) in g.learn.iter_mut().enumerate() {
            if rng.f32() < 0.5 {
                *l = other.learn[i];
            }
        }
        // The kin marker is the average of the two, so a child of two families belongs a little to
        // both and reads as kin to neither entirely. That is how a new people begins.
        for k in 0..3 {
            g.marker[k] = (self.marker[k] + other.marker[k]) * 0.5;
        }
        g
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
        for l in g.learn.iter_mut() {
            if rng.f32() < p_mut {
                *l += rng.normal() * sigma;
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
    pub fn think(&self, input: &[f32; N_IN], plastic: &[f32], critic: &[f32]) -> Thought {
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
                acc += (w[off + o * N_HID + h] + plastic[o * N_HID + h]) * hidden[h];
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
        let sig = [fast_tanh(out[O_SIG]), fast_tanh(out[O_SIG + 1])];
        let mark = out[O_MARK] > 0.0;
        let (mx, my) = if out[2] <= 0.0 { (0.0, 0.0) } else { (fast_tanh(out[0]), fast_tanh(out[1])) };
        let mut value = critic[N_HID];
        for h in 0..N_HID {
            value += critic[h] * hidden[h];
        }
        Thought { mx, my, action: Action::ALL[best], memory: mem, order, odx, ody, sig, mark, hidden, out, value: value.clamp(-25.0, 25.0) }
    }

    /// The heritable learning rate this genome encodes (before the config scale).
    #[inline]
    pub fn learn_rate(&self) -> f32 {
        0.01 * sigmoid(self.learn[0] - 1.0)
    }

    /// Learning within a life: neuromodulated Hebbian plasticity on the output layer.
    /// `reward` is the tick's change in fortune, in [-1, 1]. The genes set how fast and
    /// in what shape synapses move; a brain can also inherit a rate near zero and not learn.
    pub fn learn(&self, plastic: &mut [f32], hidden: &[f32; N_HID], out: &[f32; N_OUT], reward: f32, scale: f32) {
        let eta = scale * self.learn_rate();
        if eta < 1e-4 || reward == 0.0 {
            return;
        }
        let (a, b, c) = (fast_tanh(self.learn[1]), fast_tanh(self.learn[2]) * 0.5, fast_tanh(self.learn[3]) * 0.5);
        let m = eta * reward;
        for o in 0..N_OUT {
            let post = fast_tanh(out[o]);
            for h in 0..N_HID {
                let pre = hidden[h];
                let p = &mut plastic[o * N_HID + h];
                *p = (*p + m * (a * pre * post + b * pre + c * post) - 0.001 * *p).clamp(-0.6, 0.6);
            }
        }
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

/// One tick of a brain's output, with the activations kept for learning.
pub struct Thought {
    pub mx: f32,
    pub my: f32,
    pub action: Action,
    pub memory: [f32; N_MEM],
    pub order: Order,
    pub odx: f32,
    pub ody: f32,
    pub sig: [f32; N_SIG],
    /// Whether this mind chose to remember where it is standing.
    pub mark: bool,
    pub hidden: [f32; N_HID],
    pub out: [f32; N_OUT],
    /// The critic's estimate of how good this moment is, in units of future reward.
    pub value: f32,
}

/// The policy: action scores turned into probabilities. A brain that learns by gradient has to
/// take chances, because a choice never taken teaches nothing. Temperature sets how wild it is;
/// as temperature goes to zero this becomes the old "pick the highest score".
pub fn act_probs(out: &[f32; N_OUT], temp: f32) -> [f32; N_ACT] {
    let t = temp.max(0.05);
    let mut p = [0.0f32; N_ACT];
    let mut top = f32::NEG_INFINITY;
    for a in 0..N_ACT {
        if out[O_ACT + a] > top {
            top = out[O_ACT + a];
        }
    }
    let mut sum = 0.0;
    for a in 0..N_ACT {
        p[a] = ((out[O_ACT + a] - top) / t).exp();
        sum += p[a];
    }
    for v in p.iter_mut() {
        *v /= sum;
    }
    p
}

/// One step of actor-critic learning with eligibility traces.
///
/// `td` is the surprise: reward that arrived plus what the next moment is worth, minus what this
/// moment was thought to be worth. Positive means better than expected. The traces remember which
/// weights argued for the recent choices, fading as they go, so a reward arriving late still finds
/// the choice that earned it. This is the piece plain Hebbian learning cannot do.
///
/// Both steps are normalised by the size of their own trace, which is what keeps this stable.
/// Without it the step grows with however long the trace happens to be, the critic overshoots,
/// its error never settles, and the noise it feeds the chooser makes everyone lock onto a single
/// action and starve. Measured, not guessed: the unnormalised version collapsed every world.
/// The critic takes a fixed fraction of the correction that would wipe out its own error
/// (`crit` = 0.1 means a tenth of it). The chooser takes a step of fixed length `actor` along
/// the direction its trace points, so no single loud moment can throw it.
#[allow(clippy::too_many_arguments)]
pub fn learn_td(plastic: &mut [f32], critic: &mut [f32], trace: &[f32], vtrace: &[f32], td: f32, actor: f32, crit: f32) {
    let vnorm2: f32 = vtrace.iter().map(|x| x * x).sum::<f32>().max(1e-3);
    let vstep = crit * td / vnorm2;
    for h in 0..N_CRITIC {
        critic[h] = (critic[h] + vstep * vtrace[h]).clamp(-8.0, 8.0);
    }
    let tnorm: f32 = trace.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-3);
    let astep = actor * td / tnorm;
    for a in 0..N_ACT {
        for h in 0..N_HID {
            let p = &mut plastic[(O_ACT + a) * N_HID + h];
            *p = (*p + astep * trace[a * N_HID + h]).clamp(-0.6, 0.6);
        }
    }
}

/// Fade the traces, then add what this tick's choice did.
///
/// For the chosen action the gradient of the log-probability is (1 - p) times the hidden activity;
/// for the others it is (0 - p) times it. Reading it plainly: raise what you did, lower what you
/// were also tempted by, in proportion to how surprised you would be to have done it.
pub fn trace_step(trace: &mut [f32], vtrace: &mut [f32], hidden: &[f32; N_HID], probs: &[f32; N_ACT], chosen: usize, decay: f32) {
    for a in 0..N_ACT {
        let g = if a == chosen { 1.0 - probs[a] } else { -probs[a] };
        for h in 0..N_HID {
            let t = &mut trace[a * N_HID + h];
            *t = (*t * decay + g * hidden[h]).clamp(-6.0, 6.0);
        }
    }
    for h in 0..N_HID {
        vtrace[h] = (vtrace[h] * decay + hidden[h]).clamp(-6.0, 6.0);
    }
    vtrace[N_HID] = (vtrace[N_HID] * decay + 1.0).clamp(-6.0, 6.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn out_with(scores: [f32; N_ACT]) -> [f32; N_OUT] {
        let mut out = [0.0; N_OUT];
        out[O_ACT..O_ACT + N_ACT].copy_from_slice(&scores);
        out
    }

    #[test]
    fn a_policy_is_a_distribution() {
        let p = act_probs(&out_with([0.2, -1.0, 3.0, 0.0, 0.5, -0.3]), 1.0);
        let sum: f32 = p.iter().sum();
        assert!((sum - 1.0).abs() < 1e-4, "probabilities summed to {sum}");
        assert!(p.iter().all(|v| *v > 0.0), "every action keeps a chance of being tried");
    }

    #[test]
    fn cold_policies_commit_and_warm_ones_hesitate() {
        let scores = [0.0, 0.0, 2.0, 0.0, 0.0, 0.0];
        let cold = act_probs(&out_with(scores), 0.05);
        let warm = act_probs(&out_with(scores), 2.0);
        assert!(cold[2] > 0.99, "a cold policy all but always takes the best");
        assert!(warm[2] < cold[2], "a warm policy spreads its bets");
    }

    #[test]
    fn one_lesson_never_moves_a_weight_further_than_its_step() {
        // The bug that killed every world: the update grew with the length of the trace, the
        // critic overshot, and the noise it fed back locked every agent onto one action. Both
        // steps are normalised now, so a single lesson is bounded however loud the moment was.
        let mut plastic = vec![0.0; N_HID * N_OUT];
        let mut critic = vec![0.0; N_CRITIC];
        let trace = vec![5.0; N_TRACE];
        let vtrace = vec![5.0; N_CRITIC];
        let (actor, crit) = (0.1, 0.1);
        learn_td(&mut plastic, &mut critic, &trace, &vtrace, 5.0, actor, crit);
        let biggest = plastic.iter().fold(0.0f32, |m, v| m.max(v.abs()));
        assert!(biggest <= actor + 1e-6, "one lesson moved a weight by {biggest}");
        assert!(critic.iter().all(|v| v.abs() <= 8.0), "the critic stays inside its bounds");
    }

    #[test]
    fn traces_fade_and_favour_what_was_done() {
        let mut trace = vec![0.0; N_TRACE];
        let mut vtrace = vec![0.0; N_CRITIC];
        let hidden = [1.0f32; N_HID];
        let probs = [1.0 / 6.0; N_ACT];
        trace_step(&mut trace, &mut vtrace, &hidden, &probs, 2, 0.855);
        assert!(trace[2 * N_HID] > 0.0, "the chosen action is argued for");
        assert!(trace[0] < 0.0, "the ones passed over are argued against");
        let chosen_before = trace[2 * N_HID];
        for _ in 0..40 {
            trace_step(&mut trace, &mut vtrace, &hidden, &probs, 0, 0.855);
        }
        assert!(trace[2 * N_HID] < chosen_before, "an old choice fades out of credit");
    }
}
