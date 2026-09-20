//! Windowed counters and population-level metrics, printed as a table and CSV.

use crate::agent::{Agent, EMO_NAMES, N_EMO, N_TECH, TECH_BITS, TECH_NAMES};
use crate::brain::{Action, N_ACT};
use crate::strategy::{self, StrategyReport};
use std::collections::HashMap;
use std::io::Write;

#[derive(Default, Clone)]
pub struct Window {
    pub births: u32,
    pub starved: u32,
    pub aged: u32,
    pub killed: u32,
    pub plague_deaths: u32,
    pub attacks: u32,
    pub attack_wins: u32,
    pub shares: u32,
    pub immigrants: u32,
    pub actions: [u32; N_ACT],
    pub discoveries: [u32; N_TECH],
    pub learned: [u32; N_TECH],
    pub droughts: u32,
    pub outbreaks: u32,
    pub infections: u32,
    pub windfalls: u32,
    pub accidents: u32,
    pub burned: u32,
}

pub struct Metrics {
    pub tick: u64,
    pub season: f32,
    pub climate: f32,
    pub pop: usize,
    pub mean_energy: f32,
    pub mean_inv: f32,
    pub food: f32,
    pub lineages: usize,
    pub top_share: f32,
    pub action_entropy: f32,
    pub marker_spread: f32,
    pub gini: f32,
    pub strat: StrategyReport,
    pub tech: [f32; N_TECH],
    pub cultivated: usize,
    pub settled: f32,
    pub sick: f32,
    pub emotion: [f32; N_EMO],
    pub era: &'static str,
    pub w: Window,
}

/// Eras are read off the state of society, never scripted.
pub fn era_name(tech: &[f32; N_TECH], settled: f32) -> &'static str {
    if tech[8] >= 0.5 {
        "Age of Writing"
    } else if tech[4] >= 0.5 {
        "Metal Age"
    } else if tech[1] >= 0.5 && settled >= 0.3 {
        "Village Age"
    } else if tech[1] >= 0.5 {
        "Dawn of Farming"
    } else if tech[0] >= 0.5 {
        "Tool Age"
    } else {
        "Stone Age"
    }
}

pub fn compute(tick: u64, season: f32, climate: f32, agents: &[Agent], food: f32, cultivated: usize, settled: f32, w: Window) -> Metrics {
    let pop = agents.len();
    let n = pop.max(1) as f32;
    let mut tech = [0.0f32; N_TECH];
    for (t, &bit) in TECH_BITS.iter().enumerate() {
        tech[t] = agents.iter().filter(|a| a.knows(bit)).count() as f32 / n;
    }
    let mean_energy = agents.iter().map(|a| a.energy).sum::<f32>() / n;
    let mean_inv = agents.iter().map(|a| a.inventory).sum::<f32>() / n;
    let sick = agents.iter().filter(|a| a.sick > 0).count() as f32 / n;
    let mut emotion = [0.0f32; N_EMO];
    for a in agents {
        for e in 0..N_EMO {
            emotion[e] += a.emotion[e];
        }
    }
    for e in emotion.iter_mut() {
        *e /= n;
    }

    let mut by_lineage: HashMap<u32, u32> = HashMap::new();
    for a in agents {
        *by_lineage.entry(a.lineage).or_default() += 1;
    }
    let top = by_lineage.values().copied().max().unwrap_or(0) as f32;

    let total_actions: u32 = w.actions.iter().sum();
    let mut entropy = 0.0f32;
    if total_actions > 0 {
        for &c in &w.actions {
            if c > 0 {
                let p = c as f32 / total_actions as f32;
                entropy -= p * p.log2();
            }
        }
    }

    let mut spread = 0.0;
    if pop > 1 {
        let mut mean = [0.0f32; 3];
        for a in agents {
            for k in 0..3 {
                mean[k] += a.genome.marker[k];
            }
        }
        for m in mean.iter_mut() {
            *m /= n;
        }
        let var: f32 = agents
            .iter()
            .map(|a| (0..3).map(|k| (a.genome.marker[k] - mean[k]).powi(2)).sum::<f32>())
            .sum::<f32>()
            / n;
        spread = var.sqrt();
    }

    // Gini over wealth (energy + inventory)
    let mut wealth: Vec<f32> = agents.iter().map(|a| a.energy + a.inventory).collect();
    wealth.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let sum: f32 = wealth.iter().sum();
    let mut gini = 0.0;
    if pop > 1 && sum > 0.0 {
        let mut acc = 0.0f32;
        for (i, v) in wealth.iter().enumerate() {
            acc += (2.0 * (i as f32 + 1.0) - n - 1.0) * v;
        }
        gini = acc / (n * sum);
    }

    let strat = strategy::analyse(agents);
    let era = era_name(&tech, settled);

    Metrics {
        tick,
        season,
        climate,
        pop,
        mean_energy,
        mean_inv,
        food,
        lineages: by_lineage.len(),
        top_share: if pop > 0 { top / n } else { 0.0 },
        action_entropy: entropy,
        marker_spread: spread,
        gini,
        strat,
        tech,
        cultivated,
        settled,
        sick,
        emotion,
        era,
        w,
    }
}

pub fn print_header() {
    println!(
        "{:>7} {:>4} {:>5} {:>6} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>4} {:>4} {:>4} {:>4} {:<16} {}",
        "tick", "clim", "pop", "energy", "lin", "gini", "born", "starv", "kill", "plag", "attk", "share", "strat", "tool%", "farm%", "metl%", "writ%", "field", "stay%", "fear", "angr", "joy", "bond", "era", "dominant strategies"
    );
}

pub fn print_row(m: &Metrics) {
    let counted: usize = m.strat.strategies.iter().map(|s| s.count).sum::<usize>().max(1);
    let strats: Vec<String> = m
        .strat
        .strategies
        .iter()
        .take(3)
        .map(|s| format!("[{:.0}% {}]", 100.0 * s.count as f32 / counted as f32, strategy::describe(&s.centroid)))
        .collect();
    println!(
        "{:>7} {:>4.2} {:>5} {:>6.1} {:>5} {:>5.2} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5.1} {:>5.0} {:>5.0} {:>5.0} {:>5.0} {:>5} {:>5.0} {:>4.2} {:>4.2} {:>4.2} {:>4.2} {:<16} {}",
        m.tick,
        m.climate,
        m.pop,
        m.mean_energy,
        m.lineages,
        m.gini,
        m.w.births,
        m.w.starved,
        m.w.killed,
        m.w.plague_deaths,
        m.w.attacks,
        m.w.shares,
        m.strat.effective,
        m.tech[0] * 100.0,
        m.tech[1] * 100.0,
        m.tech[4] * 100.0,
        m.tech[8] * 100.0,
        m.cultivated,
        m.settled * 100.0,
        m.emotion[0],
        m.emotion[1],
        m.emotion[2],
        m.emotion[3],
        m.era,
        strats.join(" ")
    );
}

pub fn csv_header(out: &mut impl Write) -> std::io::Result<()> {
    let acts: Vec<&str> = Action::ALL.iter().map(|a| a.name()).collect();
    let techs: Vec<String> = TECH_NAMES.iter().map(|t| format!("{t}_share")).collect();
    let emos: Vec<String> = EMO_NAMES.iter().map(|e| format!("mean_{e}")).collect();
    writeln!(
        out,
        "tick,season,climate,pop,mean_energy,mean_inventory,food,lineages,top_lineage_share,gini,action_entropy,marker_spread,strategy_entropy,effective_strategies,births,starved,aged,killed,plague_deaths,attacks,attack_wins,shares,immigrants,cultivated_cells,settled_share,sick_share,droughts,outbreaks,infections,windfalls,accidents,fields_burned,era,{},{},{}",
        emos.join(","),
        techs.join(","),
        acts.join(",")
    )
}

pub fn csv_row(out: &mut impl Write, m: &Metrics) -> std::io::Result<()> {
    let acts: Vec<String> = m.w.actions.iter().map(|c| c.to_string()).collect();
    let techs: Vec<String> = m.tech.iter().map(|t| format!("{t:.4}")).collect();
    let emos: Vec<String> = m.emotion.iter().map(|e| format!("{e:.4}")).collect();
    writeln!(
        out,
        "{},{:.3},{:.2},{},{:.2},{:.2},{:.1},{},{:.4},{:.4},{:.4},{:.4},{:.4},{:.3},{},{},{},{},{},{},{},{},{},{},{:.4},{:.4},{},{},{},{},{},{},{},{},{},{}",
        m.tick,
        m.season,
        m.climate,
        m.pop,
        m.mean_energy,
        m.mean_inv,
        m.food,
        m.lineages,
        m.top_share,
        m.gini,
        m.action_entropy,
        m.marker_spread,
        m.strat.entropy,
        m.strat.effective,
        m.w.births,
        m.w.starved,
        m.w.aged,
        m.w.killed,
        m.w.plague_deaths,
        m.w.attacks,
        m.w.attack_wins,
        m.w.shares,
        m.w.immigrants,
        m.cultivated,
        m.settled,
        m.sick,
        m.w.droughts,
        m.w.outbreaks,
        m.w.infections,
        m.w.windfalls,
        m.w.accidents,
        m.w.burned,
        m.era,
        emos.join(","),
        techs.join(","),
        acts.join(",")
    )
}
