//! A history book: notable firsts and crises, detected as they happen.
//! Written to stdout as they occur and to out/events_seed<N>.txt.

use crate::agent::{Agent, N_TECH, TECH_NAMES};
use crate::stats::Window;
use crate::strategy::StrategyReport;
use std::collections::{HashMap, HashSet};
use std::io::Write;

pub struct Event {
    pub tick: u64,
    pub text: String,
}

pub struct EventLog {
    pub events: Vec<Event>,
    fired: HashSet<String>,
    lineage_peak: HashMap<u32, f32>,
    lineage_seen: HashSet<u32>,
    tech_reached: [bool; N_TECH],
    writer: Option<std::io::BufWriter<std::fs::File>>,
}

impl EventLog {
    pub fn new(path: Option<&str>) -> EventLog {
        let writer = path.and_then(|p| std::fs::File::create(p).ok()).map(std::io::BufWriter::new);
        EventLog {
            events: Vec::new(),
            fired: HashSet::new(),
            lineage_peak: HashMap::new(),
            lineage_seen: HashSet::new(),
            tech_reached: [false; N_TECH],
            writer,
        }
    }

    /// Record an event once per key (a key of "" fires every time).
    pub fn fire(&mut self, tick: u64, key: &str, text: String) {
        if !key.is_empty() && !self.fired.insert(key.to_string()) {
            return;
        }
        println!("  ! tick {:>6}: {}", tick, text);
        if let Some(w) = self.writer.as_mut() {
            let _ = writeln!(w, "{}\t{}", tick, text);
        }
        self.events.push(Event { tick, text });
    }

    pub fn discovery(&mut self, tick: u64, tech: usize, lineage: u32, x: f32, y: f32) {
        let key = format!("discover:{}", tech);
        let text = format!("{} discovered for the first time by lineage {} at ({:.0}, {:.0})", TECH_NAMES[tech], lineage, x, y);
        self.fire(tick, &key, text);
    }

    /// Called once per stats window with the population, the window counters
    /// and the strategy report; derives crises and social firsts.
    pub fn check_window(&mut self, tick: u64, agents: &[Agent], w: &Window, strat: &StrategyReport, window_len: u64) {
        let pop = agents.len();
        let popf = pop.max(1) as f32;

        // Famine and war are judged relative to population and window length.
        let per_1000 = 1000.0 / window_len as f32;
        let starve_rate = w.starved as f32 / popf * per_1000;
        let kill_rate = w.killed as f32 / popf * per_1000;
        if starve_rate > 1.0 {
            self.fire(tick, "", format!("famine: {} starved ({:.0}% of population per 1000 ticks)", w.starved, starve_rate * 100.0));
        }
        if kill_rate > 0.5 {
            self.fire(tick, "", format!("war: {} killed in {} attacks ({:.0}% of population per 1000 ticks)", w.killed, w.attacks, kill_rate * 100.0));
        }

        // Lineages: dominance and extinction.
        let mut by: HashMap<u32, usize> = HashMap::new();
        for a in agents {
            *by.entry(a.lineage).or_default() += 1;
        }
        for (&lin, &n) in &by {
            let share = n as f32 / popf;
            let peak = self.lineage_peak.entry(lin).or_insert(0.0);
            if share > *peak {
                *peak = share;
            }
            self.lineage_seen.insert(lin);
            if share >= 0.5 {
                self.fire(tick, &format!("dominant:{lin}"), format!("lineage {} now holds {:.0}% of the population", lin, share * 100.0));
            }
        }
        let gone: Vec<(u32, f32)> = self
            .lineage_peak
            .iter()
            .filter(|(lin, peak)| **peak >= 0.05 && !by.contains_key(lin))
            .map(|(l, p)| (*l, *p))
            .collect();
        for (lin, peak) in gone {
            self.fire(tick, &format!("extinct:{lin}"), format!("lineage {} went extinct (peaked at {:.0}% of the population)", lin, peak * 100.0));
        }
        if by.len() == 1 && pop > 0 {
            self.fire(tick, "monoculture", format!("only one lineage left: {}", by.keys().next().unwrap()));
        }

        // Technology adoption milestones.
        for t in 0..N_TECH {
            if self.tech_reached[t] {
                continue;
            }
            let bit = crate::agent::TECH_BITS[t];
            let n = agents.iter().filter(|a| a.knows(bit)).count();
            if pop > 0 && n as f32 / popf >= 0.5 {
                self.tech_reached[t] = true;
                self.fire(tick, &format!("adopted:{t}"), format!("{} is now known by half the population", TECH_NAMES[t]));
            }
        }

        // Social firsts from the strategy clusters.
        let counted: usize = strat.strategies.iter().map(|s| s.count).sum::<usize>().max(1);
        for s in &strat.strategies {
            let share = s.count as f32 / counted as f32;
            if share < 0.03 {
                continue;
            }
            let c = &s.centroid;
            if c[crate::brain::Action::Attack as usize] >= 0.3 {
                self.fire(tick, "warriors", format!("a warrior class emerged: {:.0}% of agents spend over 30% of their time attacking", share * 100.0));
            }
            if c[crate::brain::N_ACT] < 0.25 {
                self.fire(tick, "settlement", format!("first settled way of life: {:.0}% of agents now barely move", share * 100.0));
            }
            if c[crate::brain::Action::Share as usize] >= 0.2 {
                self.fire(tick, "sharers", format!("a sharing culture emerged: {:.0}% of agents give food to kin over 20% of the time", share * 100.0));
            }
        }
    }

    pub fn flush(&mut self) {
        if let Some(w) = self.writer.as_mut() {
            let _ = w.flush();
        }
    }
}
