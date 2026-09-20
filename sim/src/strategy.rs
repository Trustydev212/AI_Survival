//! Strategy clustering: group agents by what they actually do, not by lineage.
//! Deterministic k-means on behaviour profiles; near-duplicate clusters are merged
//! so the count reflects genuinely different ways of living.

use crate::agent::{Agent, N_PROFILE};
use crate::brain::{Action, N_ACT};

pub struct Strategy {
    pub centroid: [f32; N_PROFILE],
    pub count: usize,
    pub mean_wealth: f32,
    pub mean_age: f32,
    pub lineages: usize,
}

pub struct StrategyReport {
    pub strategies: Vec<Strategy>, // sorted by count, descending
    pub entropy: f32,              // bits; 0 = everyone lives the same way
    pub effective: f32,            // 2^entropy = effective number of strategies
}

const K: usize = 8;
const ITERS: usize = 12;
const MERGE_DIST: f32 = 0.22;
const MIN_SHARE: f32 = 0.02;

pub fn analyse(agents: &[Agent]) -> StrategyReport {
    let n = agents.len();
    if n == 0 {
        return StrategyReport { strategies: Vec::new(), entropy: 0.0, effective: 0.0 };
    }
    // Ignore newborns with no history yet.
    let pts: Vec<(usize, [f32; N_PROFILE])> = agents
        .iter()
        .enumerate()
        .filter(|(_, a)| a.age >= 50)
        .map(|(i, a)| (i, a.profile))
        .collect();
    if pts.len() < K {
        return StrategyReport { strategies: Vec::new(), entropy: 0.0, effective: 0.0 };
    }

    // Deterministic init: evenly spaced sample points.
    let k = K.min(pts.len());
    let mut centroids: Vec<[f32; N_PROFILE]> = (0..k).map(|c| pts[c * pts.len() / k].1).collect();
    let mut assign = vec![0usize; pts.len()];
    for _ in 0..ITERS {
        for (pi, (_, p)) in pts.iter().enumerate() {
            let mut best = 0;
            let mut best_d = f32::MAX;
            for (ci, c) in centroids.iter().enumerate() {
                let d = dist2(p, c);
                if d < best_d {
                    best_d = d;
                    best = ci;
                }
            }
            assign[pi] = best;
        }
        let mut sums = vec![[0.0f32; N_PROFILE]; centroids.len()];
        let mut counts = vec![0usize; centroids.len()];
        for (pi, (_, p)) in pts.iter().enumerate() {
            let c = assign[pi];
            counts[c] += 1;
            for d in 0..N_PROFILE {
                sums[c][d] += p[d];
            }
        }
        for c in 0..centroids.len() {
            if counts[c] > 0 {
                for d in 0..N_PROFILE {
                    centroids[c][d] = sums[c][d] / counts[c] as f32;
                }
            }
        }
    }

    // Agglomerative merge: repeatedly join the closest pair of centroids while
    // they are within MERGE_DIST, so one way of living is not split into several.
    let mut counts = vec![0usize; centroids.len()];
    for &c in &assign {
        counts[c] += 1;
    }
    let mut merged_into: Vec<usize> = (0..centroids.len()).collect();
    loop {
        let mut best: Option<(usize, usize, f32)> = None;
        for a in 0..centroids.len() {
            if merged_into[a] != a || counts[a] == 0 {
                continue;
            }
            for b in 0..a {
                if merged_into[b] != b || counts[b] == 0 {
                    continue;
                }
                let d = dist2(&centroids[a], &centroids[b]).sqrt();
                if d < MERGE_DIST && best.map_or(true, |(_, _, bd)| d < bd) {
                    best = Some((a, b, d));
                }
            }
        }
        let Some((a, b, _)) = best else { break };
        let (na, nb) = (counts[a] as f32, counts[b] as f32);
        for d in 0..N_PROFILE {
            centroids[b][d] = (centroids[a][d] * na + centroids[b][d] * nb) / (na + nb);
        }
        counts[b] += counts[a];
        counts[a] = 0;
        merged_into[a] = b;
    }
    for pi in 0..assign.len() {
        let mut c = assign[pi];
        while merged_into[c] != c {
            c = merged_into[c];
        }
        assign[pi] = c;
    }

    // Build the report.
    let mut strategies: Vec<Strategy> = Vec::new();
    let mut index_of = vec![usize::MAX; centroids.len()];
    for c in 0..centroids.len() {
        if merged_into[c] != c {
            continue;
        }
        let members: Vec<usize> = (0..pts.len()).filter(|&pi| assign[pi] == c).map(|pi| pts[pi].0).collect();
        if members.is_empty() {
            continue;
        }
        let mut centroid = [0.0f32; N_PROFILE];
        let mut wealth = 0.0;
        let mut age = 0.0;
        let mut lin: Vec<u32> = Vec::with_capacity(members.len());
        for &i in &members {
            let a = &agents[i];
            for d in 0..N_PROFILE {
                centroid[d] += a.profile[d];
            }
            wealth += a.energy + a.inventory;
            age += a.age as f32;
            lin.push(a.lineage);
        }
        let m = members.len() as f32;
        for d in centroid.iter_mut() {
            *d /= m;
        }
        lin.sort_unstable();
        lin.dedup();
        index_of[c] = strategies.len();
        strategies.push(Strategy { centroid, count: members.len(), mean_wealth: wealth / m, mean_age: age / m, lineages: lin.len() });
    }
    strategies.sort_by(|a, b| b.count.cmp(&a.count));

    let total = pts.len() as f32;
    let mut entropy = 0.0f32;
    for s in &strategies {
        let p = s.count as f32 / total;
        if p >= MIN_SHARE {
            entropy -= p * p.log2();
        }
    }
    StrategyReport { effective: entropy.exp2(), strategies, entropy }
}

#[inline]
fn dist2(a: &[f32; N_PROFILE], b: &[f32; N_PROFILE]) -> f32 {
    (0..N_PROFILE).map(|i| (a[i] - b[i]).powi(2)).sum()
}

/// Short human label like "gather74 rest20 mobile" for a centroid.
pub fn describe(c: &[f32; N_PROFILE]) -> String {
    let mut acts: Vec<(usize, f32)> = (0..N_ACT).map(|i| (i, c[i])).collect();
    acts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let mut parts: Vec<String> = acts
        .iter()
        .filter(|(_, v)| *v >= 0.05)
        .take(3)
        .map(|(i, v)| format!("{}{:.0}", Action::ALL[*i].name(), v * 100.0))
        .collect();
    let mobility = c[N_ACT];
    let word = if mobility < 0.25 { "sedentary" } else if mobility < 0.55 { "roaming" } else { "nomadic" };
    parts.push(format!("{word}{:.0}", mobility * 100.0));
    parts.join(" ")
}
