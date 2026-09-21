//! A history book: notable firsts, crises and era changes, detected as they happen.
//! Written to stdout as they occur (unless quiet) and to out/events_seed<N>.txt.

use crate::agent::Agent;
use crate::innovation::Innovation;
use crate::orders::{Order, N_ORDER};
use crate::stats::{Metrics, Window};
use std::collections::{HashMap, HashSet};
use std::io::Write;

pub struct Event {
    pub tick: u64,
    pub text: String,
}

pub struct EventLog {
    pub events: Vec<Event>,
    verbose: bool,
    fired: HashSet<String>,
    lineage_peak: HashMap<u32, f32>,
    era: &'static str,
    /// Last tick a cooled-down key fired, so repeating stories are told once in a while.
    cooldowns: HashMap<String, u64>,
    obey_peak: f32,
    custom: [bool; N_ORDER],
    settled_peak: f32,
    known_peak: f32,
    writer: Option<std::io::BufWriter<std::fs::File>>,
}

impl EventLog {
    pub fn new(path: Option<&str>, verbose: bool) -> EventLog {
        let writer = path.and_then(|p| std::fs::File::create(p).ok()).map(std::io::BufWriter::new);
        EventLog {
            events: Vec::new(),
            verbose,
            fired: HashSet::new(),
            lineage_peak: HashMap::new(),
            era: "wild",
            cooldowns: HashMap::new(),
            obey_peak: 0.0,
            custom: [false; N_ORDER],
            settled_peak: 0.0,
            known_peak: 0.0,
            writer,
        }
    }

    /// Record an event once per key (a key of "" fires every time).
    pub fn fire(&mut self, tick: u64, key: &str, text: String) {
        if !key.is_empty() && !self.fired.insert(key.to_string()) {
            return;
        }
        if self.verbose {
            println!("  ! tick {:>6}: {}", tick, text);
        }
        if let Some(w) = self.writer.as_mut() {
            let _ = writeln!(w, "{}\t{}\t{}", tick, kind_of(&text), text);
            let _ = w.flush();
        }
        self.events.push(Event { tick, text });
    }

    /// Record an event at most once per `cooldown` ticks for the given key.
    pub fn fire_cooldown(&mut self, tick: u64, key: &str, cooldown: u64, text: String) {
        if let Some(last) = self.cooldowns.get(key) {
            if tick < last + cooldown {
                return;
            }
        }
        self.cooldowns.insert(key.to_string(), tick);
        self.fire(tick, "", text);
    }

    /// Called once per stats window; derives crises, social firsts and era changes.
    pub fn check_window(
        &mut self, m: &Metrics, agents: &[Agent], innovations: &[Innovation], window_len: u64,
        defected: &std::collections::HashMap<(u32, u32), u32>,
    ) {
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
        if m.soil < 0.5 {
            self.fire(tick, "soil_half", format!("exhausted land: soil health fell to {:.0}% of its potential", m.soil * 100.0));
        }
        if m.soil < 0.25 {
            self.fire(tick, "soil_quarter", format!("dust: soil health fell to {:.0}% of its potential", m.soil * 100.0));
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

        // Innovations that reached half the population.
        for (idx, inn) in innovations.iter().enumerate() {
            if inn.name.is_empty() {
                continue;
            }
            let key = format!("adopted:{idx}-{}", inn.born_tick);
            if self.fired.contains(&key) {
                continue;
            }
            let bit = 1u128 << idx;
            let n = agents.iter().filter(|a| a.known & bit != 0).count();
            if pop > 0 && n as f32 / popf >= 0.5 {
                self.fire(tick, &key, format!("{} is now known by half the population", inn.describe(innovations)));
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

        // Obedience: the moment leading starts to mean something, and what it is used for.
        if m.obedience >= 0.5 {
            self.fire(tick, "obedience", format!("the word of leaders now carries: {:.0}% of orders are obeyed", m.obedience * 100.0));
        }
        // Calls that carry meaning: when what one hears from a neighbour predicts what one does next.
        if pop >= 300 && m.sig_mi >= 0.2 && m.sig_meaning >= 0.2 {
            self.fire(tick, "language", format!("calls begin to mean something: what one says reflects one's state ({:.2} bits) and hearing a neighbour predicts what one does next ({:.2} bits)", m.sig_meaning, m.sig_mi));
        }
        if pop >= 200 && m.plastic >= 0.05 {
            self.fire(tick, "learners", format!("minds that change within a life: synapses have drifted {:.3} on average since birth", m.plastic));
        }
        if m.obedience > self.obey_peak {
            self.obey_peak = m.obedience;
        }
        for o in 0..N_ORDER {
            if self.custom[o] || m.order_mix[o] < 0.5 || m.obedience < 0.4 {
                continue;
            }
            self.custom[o] = true;
            let what = match Order::ALL[o] {
                Order::Hold => "a custom of staying: leaders call their people to hold their ground and they listen",
                Order::Move => "a custom of migration: leaders call their people onward and they follow",
                Order::Raid => "a custom of raiding: leaders call their people to war and they answer",
                Order::Conserve => "a custom of restraint: leaders call their people off the land and they obey",
                Order::Pool => "a custom of pooling: leaders call for food to be shared and it is",
            };
            self.fire(tick, "", format!("{} ({:.0}% of orders, {:.0}% obeyed)", what, m.order_mix[o] * 100.0, m.obedience * 100.0));
        }

        // Customs: an order that has outlived the need for a leader to give it.
        for o in 0..N_ORDER {
            if m.custom_mix[o] >= 0.3 {
                let key = format!("custom:{o}");
                self.fire(tick, &key, format!("a {} custom took root: {:.0}% of people keep it with no leader present", Order::ALL[o].name(), m.custom_mix[o] * 100.0));
            }
        }
        // Rivalry: bands that changed hands this window.
        let mut big: Vec<(&(u32, u32), &u32)> = defected.iter().filter(|(_, n)| **n >= 15).collect();
        big.sort_by(|a, b| b.1.cmp(a.1));
        for ((from, to), n) in big.into_iter().take(3) {
            self.fire(tick, "", format!("rivalry: {} lost {} followers to {}", crate::agent::name_of(*from), n, crate::agent::name_of(*to)));
        }

        // Storehouses that carried people through a winter.
        if w.winter_withdrawals as f32 > popf {
            self.fire(tick, "", format!("granary: storehouses were drawn on {} times through the winter by a population of {}", w.winter_withdrawals, pop));
        }

        // Eras, collapses and forgetting.
        if m.era != self.era {
            self.fire(tick, "", format!("era: {} -> {} (mean knowledge {:.1}, settled {:.0}%)", self.era, m.era, m.mean_known, m.settled * 100.0));
            self.era = m.era;
        }
        if m.settled > self.settled_peak {
            self.settled_peak = m.settled;
        }
        if self.settled_peak >= 0.5 && m.settled < 0.15 {
            self.fire(tick, "", format!("collapse: settlements abandoned, settled share fell from {:.0}% to {:.0}%", self.settled_peak * 100.0, m.settled * 100.0));
            self.settled_peak = m.settled;
        }
        if m.mean_known > self.known_peak {
            self.known_peak = m.mean_known;
        }
        if self.known_peak >= 4.0 && m.mean_known < 0.5 * self.known_peak {
            self.fire(tick, "", format!("forgetting: mean knowledge fell from {:.1} to {:.1} innovations per head", self.known_peak, m.mean_known));
            self.known_peak = m.mean_known;
        }
    }

    pub fn flush(&mut self) {
        if let Some(w) = self.writer.as_mut() {
            let _ = w.flush();
        }
    }
}

/// A short machine-readable kind for each event, derived from how its text begins.
/// Viewers translate by kind and pull numbers and names out of the text.
pub fn kind_of(text: &str) -> &'static str {
    const KINDS: [(&str, &str); 36] = [
        ("famine:", "famine"), ("war:", "war"), ("plague toll:", "plague_toll"), ("plague:", "plague"),
        ("raids:", "raids"), ("drought:", "drought"), ("a year of plenty", "plenty"), ("harsh year", "harsh_year"),
        ("flood:", "flood"), ("wildfire:", "wildfire"), ("bounty:", "bounty"), ("exhausted land", "soil_half"),
        ("dust:", "soil_quarter"), ("lineage ", "lineage"), ("only one lineage", "monoculture"),
        ("innovation:", "innovation"), (" is now known by half", "adopted"), ("a warrior class", "warriors"),
        ("first settled", "settlement"), ("a sharing culture", "sharers"), ("the word of leaders", "obedience"), ("calls begin to mean", "language"), ("minds that change", "learners"),
        ("a custom of", "custom_order"), ("era:", "era"), ("collapse:", "collapse"), ("forgetting:", "forgetting"),
        ("first leader:", "first_leader"), ("great leader:", "great_leader"), ("leader ", "leader_died"),
        ("rivalry:", "rivalry"), ("first storehouse", "first_store"), ("first storehouse looted", "first_loot"),
        ("granary:", "granary"), ("extinction:", "extinction"), (" took root", "custom_root"),
    ];
    for (needle, kind) in KINDS {
        if text.starts_with(needle) || (needle.starts_with(' ') && text.contains(needle)) {
            return kind;
        }
    }
    if text.contains("went over to") {
        return "merger";
    }
    if text.contains("first fields burned") {
        return "first_burn";
    }
    if text.contains("sets out to sea") {
        return "first_sail";
    }
    for (slot, kind) in [("first tool:", "first_tool"), ("first weapon:", "first_weapon"), ("first armour:", "first_armour"), ("first boat:", "first_boat"), ("first vessel:", "first_vessel"), ("first fire:", "first_fire"), ("first shelter:", "first_shelter"), ("first house burned", "first_house_burned")] {
        if text.starts_with(slot) {
            return kind;
        }
    }
    if text.starts_with("crafted:") {
        return "craft";
    }
    if text.starts_with("first hunt:") {
        return "first_hunt";
    }
    "other"
}
