//! A history book: notable firsts, crises and era changes, detected as they happen.
//! Written to stdout as they occur and to out/events_seed<N>.txt.

use crate::agent::{Agent, N_TECH, TECH_BITS, TECH_NAMES};
use crate::stats::{Metrics, Window};
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
    tech_reached: [bool; N_TECH],
    era: &'static str,
    settled_peak: f32,
    writing_peak: f32,
    writer: Option<std::io::BufWriter<std::fs::File>>,
}

impl EventLog {
    pub fn new(path: Option<&str>) -> EventLog {
        let writer = path.and_then(|p| std::fs::File::create(p).ok()).map(std::io::BufWriter::new);
        EventLog {
            events: Vec::new(),
            fired: HashSet::new(),
            lineage_peak: HashMap::new(),
            tech_reached: [false; N_TECH],
            era: "Stone Age",
            settled_peak: 0.0,
            writing_peak: 0.0,
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

    /// Called once per stats window; derives crises, social firsts and era changes.
    pub fn check_window(&mut self, m: &Metrics, agents: &[Agent], window_len: u64) {
        let tick = m.tick;
        let w: &Window = &m.w;
        let pop = agents.len();
        let popf = pop.max(1) as f32;

        // Crises are judged relative to population and window length.
        let per_1000 = 1000.0 / window_len as f32;
        let starve_rate = w.starved as f32 / popf * per_1000;
        let kill_rate = w.killed as f32 / popf * per_1000;
        let plague_rate = w.plague_deaths as f32 / popf * per_1000;
        if starve_rate > 1.0 {
            self.fire(tick, "", format!("famine: {} starved ({:.0}% of population per 1000 ticks)", w.starved, starve_rate * 100.0));
        }
        if kill_rate > 0.5 {
            self.fire(tick, "", format!("war: {} killed in {} attacks ({:.0}% of population per 1000 ticks)", w.killed, w.attacks, kill_rate * 100.0));
        }
        if plague_rate > 0.2 {
            self.fire(tick, "", format!("plague toll: {} dead of sickness, {} infected this window", w.plague_deaths, w.infections));
        }
        if w.burned > 0 {
            self.fire(tick, "", format!("raids: {} fields burned this window", w.burned));
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
            let bit = TECH_BITS[t];
            let n = agents.iter().filter(|a| a.knows(bit)).count();
            if pop > 0 && n as f32 / popf >= 0.5 {
                self.tech_reached[t] = true;
                self.fire(tick, &format!("adopted:{t}"), format!("{} is now known by half the population", TECH_NAMES[t]));
            }
        }

        // Social firsts from the strategy clusters.
        let counted: usize = m.strat.strategies.iter().map(|s| s.count).sum::<usize>().max(1);
        for s in &m.strat.strategies {
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

        // Eras, collapses and dark ages.
        if m.era != self.era {
            self.fire(tick, "", format!("era: {} -> {}", self.era, m.era));
            self.era = m.era;
        }
        if m.settled > self.settled_peak {
            self.settled_peak = m.settled;
        }
        if self.settled_peak >= 0.5 && m.settled < 0.15 {
            self.fire(tick, "collapse", format!("collapse: villages abandoned, settled share fell from {:.0}% to {:.0}%", self.settled_peak * 100.0, m.settled * 100.0));
            self.settled_peak = m.settled;
            self.fired.remove("collapse");
        }
        if m.tech[8] > self.writing_peak {
            self.writing_peak = m.tech[8];
        }
        if self.writing_peak >= 0.5 && m.tech[8] < 0.2 {
            self.fire(tick, "", format!("dark age: writing fell from {:.0}% to {:.0}% of the population", self.writing_peak * 100.0, m.tech[8] * 100.0));
            self.writing_peak = m.tech[8];
        }
    }

    pub fn flush(&mut self) {
        if let Some(w) = self.writer.as_mut() {
            let _ = w.flush();
        }
    }
}
