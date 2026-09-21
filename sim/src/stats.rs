//! Windowed counters and population-level metrics, printed as a table and CSV.

use crate::agent::{Agent, EMO_NAMES, N_EMO, N_SKILL, SKILL_NAMES};
use crate::brain::{Action, N_ACT};
use crate::orders::{Order, N_ORDER};
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
    /// Times an agent moved from land onto the sea this window.
    pub voyages: u32,
    /// Herds brought down together, and hunts that failed for want of hands.
    pub hunts: u32,
    pub hunt_fails: u32,
    /// Times someone took a step towards what another had *learned*, not inherited.
    pub know_gifts: u32,
    pub craft_tries: u32,
    /// New things registered, things made, shelters raised, recipes found again independently.
    pub crafts: u32,
    pub made: u32,
    pub built: u32,
    pub rediscoveries: u32,
    pub forgotten_recipes: u32,
    /// Units of material given between kin.
    pub mat_gifts: u32,
    pub actions: [u32; N_ACT],
    pub discoveries: u32,
    pub learned: u32,
    pub droughts: u32,
    pub outbreaks: u32,
    pub infections: u32,
    pub windfalls: u32,
    pub accidents: u32,
    pub burned: u32,
    pub floods: u32,
    pub wildfires: u32,
    pub harsh_winters: u32,
    pub bounties: u32,
    pub imitations: u32,
    pub leader_deaths: u32,
    pub orders: [u32; N_ORDER],
    pub obeyed: u32,
    pub defied: u32,
    pub custom_acts: u32,
    pub custom_spread: u32,
    pub defections: u32,
    pub mergers: u32,
    pub fertile: u32,
    pub restrained: u32,
    pub stores_raised: u32,
    pub deposits: u32,
    pub deposited: f32,
    pub withdrawals: u32,
    pub winter_withdrawals: u32,
    pub looted: f32,
}

pub struct Metrics {
    pub tick: u64,
    pub season: f32,
    pub climate: f32,
    pub pop: usize,
    pub mean_energy: f32,
    pub mean_inv: f32,
    pub food: f32,
    pub soil: f32,
    pub lineages: usize,
    pub top_share: f32,
    pub action_entropy: f32,
    pub marker_spread: f32,
    pub gini: f32,
    pub strat: StrategyReport,
    pub innovations: usize,
    pub mean_known: f32,
    pub cultivated: usize,
    pub settled: f32,
    pub sick: f32,
    pub emotion: [f32; N_EMO],
    pub skill: [f32; N_SKILL],
    pub leaders: usize,
    pub max_followers: u16,
    /// Share of orders obeyed this window, and what leaders are calling for.
    pub obedience: f32,
    pub order_mix: [f32; N_ORDER],
    /// Share of agents holding each custom; and births per 1000 agent-ticks spent able to breed.
    pub custom_mix: [f32; N_ORDER],
    pub breed_rate: f32,
    pub stores: usize,
    pub stored: f32,
    /// Mean soil health of the regions agents actually live in.
    pub lived_soil: f32,
    pub level: usize,
    pub era: &'static str,
    /// Mean size of within-life synaptic change: how much brains learn rather than inherit.
    pub plastic: f32,
    /// Signals: entropy of what is said (bits), and how much what one hears predicts what one does (bits).
    pub sig_ent: f32,
    pub sig_mi: f32,
    /// How much what one says reflects one's own state (bits): meaning on the speaker's side.
    pub sig_meaning: f32,
    /// Mean size of the surprise a mind gets each tick (temporal-difference error), and the mean
    /// price it puts on the present. Both are 0 unless the gradient learner is on.
    pub td: f32,
    pub value: f32,
    /// Division of labour, 0..1: how much of the population's activity mix is explained by who
    /// does it (mutual information between person and action, over the entropy of actions).
    pub dol: f32,
    /// Shelters standing right now: what a viewer would actually see on the map.
    pub buildings: usize,
    /// Things per head, and the share of people holding at least one made thing.
    pub things: f32,
    pub equipped: f32,
    /// Mean heritable learning rate (x1000) and mean loudness of calls: where evolution pushes minds and mouths.
    pub learn_rate: f32,
    pub loudness: f32,
    pub w: Window,
}

/// Eras are read off how much a society knows and whether it has rooted itself.
/// The names are deliberately not human history.
pub const ERA_NAMES: [&str; 8] = ["wild", "kindled", "rooted", "woven", "layered", "soaring", "radiant", "beyond"];

pub fn level_of(mean_known: f32, settled: f32) -> usize {
    // The scale has widened twice. Recipes made knowledge plentiful, and then the ceiling on how
    // many innovations a world may hold at once went from 128 to 512, after which a thriving world
    // kept learning past 130 things per head instead of stopping at about 110. At one era per five
    // things every such world sat in the last era for most of its life, which told you nothing.
    let mut level = (mean_known / 25.0).floor() as usize;
    if settled >= 0.3 {
        level += 1;
    }
    level.min(ERA_NAMES.len() - 1)
}

#[allow(clippy::too_many_arguments)]
pub fn compute(
    tick: u64, season: f32, climate: f32, agents: &[Agent], food: f32, soil: f32, lived_soil: f32,
    innovations: usize, cultivated: usize, settled: f32, order_mix: [f32; N_ORDER], custom_mix: [f32; N_ORDER],
    stores: usize, stored: f32, buildings: usize, w: Window,
) -> Metrics {
    let pop = agents.len();
    let n = pop.max(1) as f32;
    let mean_known = agents.iter().map(|a| a.known_count() as f32).sum::<f32>() / n;
    let plastic = agents.iter().map(|a| a.plastic.iter().map(|p| p.abs()).sum::<f32>() / a.plastic.len().max(1) as f32).sum::<f32>() / n;
    let (sig_ent, sig_mi, sig_meaning) = signal_stats(agents);
    let dol = division_of_labour(agents);
    let td = agents.iter().map(|a| a.td.abs()).sum::<f32>() / n;
    let value = agents.iter().map(|a| a.v_prev).sum::<f32>() / n;
    let things = agents.iter().map(|a| a.gear.iter().filter(|g| g.is_some()).count() as f32).sum::<f32>() / n;
    let equipped = agents.iter().filter(|a| a.gear.iter().any(|g| g.is_some())).count() as f32 / n;
    let learn_rate = agents.iter().map(|a| a.genome.learn_rate() * 1000.0).sum::<f32>() / n;
    let loudness = agents.iter().map(|a| a.signal[0].abs() + a.signal[1].abs()).sum::<f32>() / n;
    let mean_energy = agents.iter().map(|a| a.energy).sum::<f32>() / n;
    let mean_inv = agents.iter().map(|a| a.inventory).sum::<f32>() / n;
    let sick = agents.iter().filter(|a| a.sick > 0).count() as f32 / n;
    let mut emotion = [0.0f32; N_EMO];
    let mut skill = [0.0f32; N_SKILL];
    for a in agents {
        for e in 0..N_EMO {
            emotion[e] += a.emotion[e];
        }
        for k in 0..N_SKILL {
            skill[k] += a.skill[k];
        }
    }
    for e in emotion.iter_mut() {
        *e /= n;
    }
    for k in skill.iter_mut() {
        *k /= n;
    }
    let under = (w.obeyed + w.defied).max(1) as f32;
    let obedience = w.obeyed as f32 / under;
    let breed_rate = 1000.0 * w.births as f32 / w.fertile.max(1) as f32;
    let leaders = agents.iter().filter(|a| a.is_leader).count();
    let max_followers = agents.iter().map(|a| a.followers).max().unwrap_or(0);

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
    let level = level_of(mean_known, settled);

    Metrics {
        tick,
        season,
        climate,
        pop,
        mean_energy,
        mean_inv,
        food,
        soil,
        lineages: by_lineage.len(),
        top_share: if pop > 0 { top / n } else { 0.0 },
        action_entropy: entropy,
        marker_spread: spread,
        gini,
        strat,
        innovations,
        mean_known,
        cultivated,
        settled,
        sick,
        emotion,
        skill,
        leaders,
        max_followers,
        obedience,
        order_mix,
        custom_mix,
        breed_rate,
        stores,
        stored,
        lived_soil,
        level,
        era: ERA_NAMES[level],
        plastic,
        sig_ent,
        sig_mi,
        sig_meaning,
        td,
        value,
        dol,
        buildings,
        things,
        equipped,
        learn_rate,
        loudness,
        w,
    }
}

pub fn print_header() {
    println!(
        "{:>7} {:>4} {:>5} {:>6} {:>4} {:>5} {:>4} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:<9} {:<12} {:>5} {:>5} {:<8} {}",
        "tick", "clim", "pop", "energy", "soil", "lsoil", "lin", "gini", "born", "starv", "kill", "plag", "attk",
        "share", "strat", "innov", "known", "field", "stay", "fear", "angr", "joy", "bond", "skil", "lead", "obey",
        "order", "custom", "breed", "store", "era", "dominant strategies"
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
        "{:>7} {:>4.2} {:>5} {:>6.1} {:>4.2} {:>5.2} {:>4} {:>5.2} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5.1} {:>5} {:>5.1} {:>5} {:>4.0} {:>4.2} {:>4.2} {:>4.2} {:>4.2} {:>4.2} {:>4} {:>4.2} {:<9} {:<12} {:>5.1} {:>5} {:<8} {}",
        m.tick, m.climate, m.pop, m.mean_energy, m.soil, m.lived_soil, m.lineages, m.gini, m.w.births, m.w.starved,
        m.w.killed, m.w.plague_deaths, m.w.attacks, m.w.shares, m.strat.effective, m.innovations, m.mean_known,
        m.cultivated, m.settled * 100.0, m.emotion[0], m.emotion[1], m.emotion[2], m.emotion[3],
        (m.skill[0] + m.skill[1] + m.skill[2]) / 3.0, m.leaders, m.obedience, dominant_order(&m.order_mix),
        dominant_order(&m.custom_mix), m.breed_rate, m.stores, m.era, strats.join(" ")
    );
}

/// The call most leaders are making, with its share, e.g. "hold 62%".
pub fn dominant_order(mix: &[f32; N_ORDER]) -> String {
    let mut best = 0;
    for o in 1..N_ORDER {
        if mix[o] > mix[best] {
            best = o;
        }
    }
    if mix[best] <= 0.0 {
        return "-".to_string();
    }
    format!("{} {:.0}%", Order::ALL[best].name(), mix[best] * 100.0)
}

pub fn csv_header(out: &mut impl Write) -> std::io::Result<()> {
    let mut c: Vec<String> = [
        "tick", "season", "climate", "pop", "mean_energy", "mean_inventory", "food", "soil_health", "lived_soil",
        "obedience", "lineages", "top_lineage_share", "gini", "action_entropy", "marker_spread", "strategy_entropy",
        "effective_strategies", "births", "starved", "aged", "killed", "plague_deaths", "attacks", "attack_wins",
        "shares", "immigrants", "innovations", "mean_known", "discoveries", "learned", "cultivated_cells",
        "settled_share", "sick_share", "droughts", "outbreaks", "infections", "windfalls", "accidents",
        "fields_burned", "floods", "wildfires", "harsh_winters", "bounties", "imitations", "leaders",
        "max_followers", "leader_deaths", "level", "custom_acts", "custom_spread", "defections", "mergers",
        "breed_rate", "stores", "stored", "deposits", "withdrawals", "winter_withdrawals", "looted",
        "plastic", "signal_entropy", "signal_mi", "things_per_head", "equipped_share", "craft_tries", "crafts", "made", "built",
        "rediscoveries", "forgotten_recipes", "material_gifts", "voyages", "learn_rate", "loudness", "hunts", "hunt_fails", "signal_meaning", "division_of_labour", "td_error", "mean_value", "know_gifts", "buildings",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    c.extend(EMO_NAMES.iter().map(|e| format!("mean_{e}")));
    c.extend(SKILL_NAMES.iter().map(|e| format!("skill_{e}")));
    c.extend(Order::ALL.iter().map(|o| format!("order_{}", o.name())));
    c.extend(Order::ALL.iter().map(|o| format!("custom_{}", o.name())));
    c.extend(Action::ALL.iter().map(|a| a.name().to_string()));
    c.push("era".to_string());
    writeln!(out, "{}", c.join(","))
}

pub fn csv_row(out: &mut impl Write, m: &Metrics) -> std::io::Result<()> {
    let mut f: Vec<String> = Vec::with_capacity(70);
    let mut n = |v: f32| f.push(format!("{v:.4}"));
    n(m.tick as f32);
    n(m.season);
    n(m.climate);
    n(m.pop as f32);
    n(m.mean_energy);
    n(m.mean_inv);
    n(m.food);
    n(m.soil);
    n(m.lived_soil);
    n(m.obedience);
    n(m.lineages as f32);
    n(m.top_share);
    n(m.gini);
    n(m.action_entropy);
    n(m.marker_spread);
    n(m.strat.entropy);
    n(m.strat.effective);
    let w = &m.w;
    for v in [
        w.births, w.starved, w.aged, w.killed, w.plague_deaths, w.attacks, w.attack_wins, w.shares, w.immigrants,
    ] {
        n(v as f32);
    }
    n(m.innovations as f32);
    n(m.mean_known);
    n(w.discoveries as f32);
    n(w.learned as f32);
    n(m.cultivated as f32);
    n(m.settled);
    n(m.sick);
    for v in [
        w.droughts, w.outbreaks, w.infections, w.windfalls, w.accidents, w.burned, w.floods, w.wildfires,
        w.harsh_winters, w.bounties, w.imitations,
    ] {
        n(v as f32);
    }
    n(m.leaders as f32);
    n(m.max_followers as f32);
    n(w.leader_deaths as f32);
    n(m.level as f32);
    n(w.custom_acts as f32);
    n(w.custom_spread as f32);
    n(w.defections as f32);
    n(w.mergers as f32);
    n(m.breed_rate);
    n(m.stores as f32);
    n(m.stored);
    n(w.deposits as f32);
    n(w.withdrawals as f32);
    n(w.winter_withdrawals as f32);
    n(w.looted);
    n(m.plastic);
    n(m.sig_ent);
    n(m.sig_mi);
    n(m.things);
    n(m.equipped);
    for v in [w.craft_tries, w.crafts, w.made, w.built, w.rediscoveries, w.forgotten_recipes, w.mat_gifts, w.voyages] {
        n(v as f32);
    }
    n(m.learn_rate);
    n(m.loudness);
    n(w.hunts as f32);
    n(w.hunt_fails as f32);
    n(m.sig_meaning);
    n(m.dol);
    n(m.td);
    n(m.value);
    n(w.know_gifts as f32);
    n(m.buildings as f32);
    for v in m.emotion {
        n(v);
    }
    for v in m.skill {
        n(v);
    }
    for v in m.order_mix {
        n(v);
    }
    for v in m.custom_mix {
        n(v);
    }
    for v in w.actions {
        n(v as f32);
    }
    writeln!(out, "{},{}", f.join(","), m.era)
}

/// Quantise a signal into one of 16 symbols (4 bins per dimension).
fn symbol(sig: &[f32; crate::brain::N_SIG]) -> usize {
    let b = |v: f32| (((v + 1.0) * 2.0).floor() as usize).min(3);
    b(sig[0]) * 4 + b(sig[1])
}

/// The speaker's situation, in six classes: hungry, middling or full, and afraid or not.
/// This is what a call could be *about* before anyone has agreed on what it means.
fn state_class(a: &Agent) -> usize {
    state_class_of(a.energy, a.emotion[0])
}

fn state_class_of(energy: f32, fear: f32) -> usize {
    let e = if energy < 35.0 { 0 } else if energy < 70.0 { 1 } else { 2 };
    e + if fear > 0.3 { 3 } else { 0 }
}

/// Mutual information between two symbol streams from their joint counts, in bits, with the
/// Miller-Madow correction (small samples inflate the plug-in estimate).
fn mutual_information<const X: usize, const Y: usize>(joint: &[[f32; Y]; X], n: f32) -> f32 {
    let mut px = [0f32; X];
    let mut py = [0f32; Y];
    for (x, row) in joint.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            px[x] += c;
            py[y] += c;
        }
    }
    let mut mi = 0.0;
    for (x, row) in joint.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            let pxy = c / n;
            if pxy > 0.0 {
                mi += pxy * (pxy / ((px[x] / n) * (py[y] / n))).log2();
            }
        }
    }
    let kx = px.iter().filter(|c| **c > 0.0).count() as f32;
    let ky = py.iter().filter(|c| **c > 0.0).count() as f32;
    let bias = (kx * ky - kx - ky + 1.0).max(0.0) / (2.0 * n * std::f32::consts::LN_2);
    (mi - bias).max(0.0)
}

/// Signals, measured from both ends of the channel:
/// entropy of what is said; how much what one *hears* predicts what one does next
/// (comprehension); and how much what one *says* reflects one's own state (meaning).
/// A call can be meaningful without being understood, and "understood" without meaning
/// anything, when neighbours merely share a situation; language needs both.
pub fn signal_stats(agents: &[Agent]) -> (f32, f32, f32) {
    if agents.len() < 20 {
        return (0.0, 0.0, 0.0);
    }
    let mut said = [0f32; 16];
    let mut heard_act = [[0f32; N_ACT]; 16];
    let mut said_state = [[0f32; 6]; 16];
    for a in agents {
        let s = symbol(&a.signal);
        said[s] += 1.0;
        said_state[s][state_class(a)] += 1.0;
        heard_act[symbol(&a.heard)][a.last_action as usize] += 1.0;
    }
    let n = agents.len() as f32;
    let ent = -said.iter().filter(|c| **c > 0.0).map(|c| (c / n) * (c / n).log2()).sum::<f32>();
    (ent, mutual_information(&heard_act, n), mutual_information(&said_state, n))
}

/// Division of labour in the sense of Gorelick and others: the mutual information between
/// *who* and *what is done*, over the entropy of what is done. 0 when everyone lives the same
/// mix of actions; 1 when each person does one thing and different people do different things.
/// Uses each agent's decayed action profile, so it reads roles, not single moments.
pub fn division_of_labour(agents: &[Agent]) -> f32 {
    if agents.len() < 20 {
        return 0.0;
    }
    let entropy = |p: &[f32; N_ACT]| -> f32 { -p.iter().filter(|x| **x > 0.0).map(|x| x * x.log2()).sum::<f32>() };
    let mut pooled = [0f32; N_ACT];
    let mut inner = 0.0;
    let mut counted = 0.0;
    for a in agents {
        let total: f32 = a.profile[..N_ACT].iter().sum();
        if total <= 0.0 {
            continue;
        }
        let mut p = [0f32; N_ACT];
        for k in 0..N_ACT {
            p[k] = a.profile[k] / total;
            pooled[k] += p[k];
        }
        inner += entropy(&p);
        counted += 1.0;
    }
    if counted == 0.0 {
        return 0.0;
    }
    for x in pooled.iter_mut() {
        *x /= counted;
    }
    let h = entropy(&pooled);
    if h <= 0.0 {
        return 0.0;
    }
    ((h - inner / counted) / h).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_streams_carry_no_information() {
        // Every symbol paired with every action equally often: hearing tells you nothing.
        let joint = [[10.0f32; 4]; 4];
        let mi = mutual_information(&joint, 160.0);
        assert!(mi < 0.05, "independent streams reported {mi} bits");
    }

    #[test]
    fn a_perfect_code_carries_its_bits() {
        // Each symbol always followed by its own action: two bits, less the small-sample correction.
        let mut joint = [[0.0f32; 4]; 4];
        for k in 0..4 {
            joint[k][k] = 100.0;
        }
        let mi = mutual_information(&joint, 400.0);
        assert!(mi > 1.8, "a perfect four-way code reported only {mi} bits");
    }

    #[test]
    fn hunger_and_fear_make_six_states() {
        // The classes the speaker-side measure reads must not collapse or overflow.
        assert!(state_class_of(10.0, 0.0) != state_class_of(50.0, 0.0));
        assert!(state_class_of(10.0, 0.0) != state_class_of(10.0, 0.9));
        assert_eq!(state_class_of(90.0, 0.9), 5);
        assert_eq!(state_class_of(10.0, 0.0), 0);
    }
}
