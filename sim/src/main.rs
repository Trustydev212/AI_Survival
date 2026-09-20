mod agent;
mod brain;
mod config;
mod events;
mod innovation;
mod render;
mod rng;
mod sim;
mod spatial;
mod stats;
mod strategy;
mod world;

use config::Config;
use std::io::{BufWriter, Write};
use std::time::Instant;

/// How one world ended.
struct Outcome {
    seed: u64,
    label: &'static str,
    ticks: u64,
    peak_pop: usize,
    final_pop: usize,
    innovations: usize,
    mean_known: f32,
    peak_level: usize,
    final_level: usize,
    soil: f32,
    settled: f32,
    top_leader: String,
    events: usize,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cfg = match Config::from_args(&args) {
        Ok(c) => c,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(if msg.starts_with("AI Survival") { 0 } else { 2 });
        }
    };
    std::fs::create_dir_all(&cfg.out_dir).expect("create out dir");

    if let Some((a, b)) = cfg.seeds {
        experiment(&cfg, a, b);
        return;
    }
    eprintln!(
        "seed={} agents={} tribes={} map={}x{} ticks={} brain={}-{}-{} ({} weights)",
        cfg.seed, cfg.agents, cfg.tribes, cfg.width, cfg.height, cfg.ticks,
        brain::N_IN, brain::N_HID, brain::N_OUT, brain::N_WEIGHTS
    );
    let start = Instant::now();
    let (o, sim) = run_one(cfg.clone());
    let secs = start.elapsed().as_secs_f64();
    eprintln!("done: {} ticks in {:.2}s ({:.0} ticks/s)", o.ticks, secs, o.ticks as f64 / secs);
    summarize(&sim);
    println!("\nOutcome: {} after {} ticks. Peak pop {}, final pop {}, {} innovations, {:.1} known per head, soil {:.0}%, era {} (peak {}).",
        o.label, o.ticks, o.peak_pop, o.final_pop, o.innovations, o.mean_known, o.soil * 100.0, stats::ERA_NAMES[o.final_level], stats::ERA_NAMES[o.peak_level]);
}

/// Run one world to its end. Returns the outcome and the finished sim for summaries.
fn run_one(cfg: Config) -> (Outcome, sim::Sim) {
    let csv_path = format!("{}/stats_seed{}.csv", cfg.out_dir, cfg.seed);
    let mut csv = BufWriter::new(std::fs::File::create(&csv_path).expect("create csv"));
    stats::csv_header(&mut csv).unwrap();
    let events_path = format!("{}/events_seed{}.txt", cfg.out_dir, cfg.seed);
    let mut sim = sim::Sim::new(cfg.clone(), events::EventLog::new(Some(&events_path), !cfg.quiet));
    if !cfg.quiet {
        stats::print_header();
    }

    let mut peak_pop = sim.agents.len();
    let mut peak_level = 0;
    let mut last: Option<stats::Metrics> = None;
    let mut extinct = false;
    let mut pops: Vec<usize> = Vec::new();
    for _ in 0..cfg.ticks {
        sim.step();
        let t = sim.tick;
        peak_pop = peak_pop.max(sim.agents.len());
        if cfg.image_every > 0 && t % cfg.image_every == 0 {
            let path = format!("{}/frame_{:06}.ppm", cfg.out_dir, t);
            render::write_ppm(&path, &sim.world, &sim.agents, 3).expect("write ppm");
        }
        if t % cfg.log_every == 0 || sim.agents.is_empty() {
            let window = sim.take_window();
            let m = stats::compute(
                t, sim.world.season(t), sim.world.climate, &sim.agents, sim.world.total_food(), sim.world.soil_health(),
                sim.innovations.len(), sim.world.cultivated_cells(), sim.settled_share(), window,
            );
            if !cfg.quiet {
                stats::print_row(&m);
            }
            stats::csv_row(&mut csv, &m).unwrap();
            sim.events.check_window(&m, &sim.agents, &sim.innovations, cfg.log_every);
            peak_level = peak_level.max(m.level);
            pops.push(m.pop);
            last = Some(m);
        }
        if sim.agents.is_empty() {
            sim.events.fire(t, "extinction", "extinction: the last agent died".to_string());
            extinct = true;
            break;
        }
    }
    csv.flush().unwrap();
    sim.events.flush();

    let m = last.expect("at least one stats window");
    // Seasons swing population within a year, so judge collapse on yearly means:
    // the best year against the last year.
    let per_year = ((cfg.season_len as u64 / cfg.log_every.max(1)) as usize).max(1);
    let year_mean = |w: &[usize]| w.iter().sum::<usize>() as f32 / w.len().max(1) as f32;
    let best_year = pops.windows(per_year.min(pops.len().max(1))).map(year_mean).fold(0.0f32, f32::max);
    let last_year = year_mean(&pops[pops.len().saturating_sub(per_year)..]);
    let label = if extinct {
        "extinct"
    } else if last_year < 0.25 * best_year {
        "collapsed"
    } else if m.level >= 3 && m.soil >= 0.5 {
        "flourishing"
    } else if m.level >= 3 {
        "flourishing on dying land"
    } else if m.level + 1 < peak_level {
        "fallen"
    } else {
        "surviving"
    };
    let top_leader = sim
        .hall
        .iter()
        .max_by_key(|(_, v)| v.0)
        .map(|(name, v)| format!("{} ({} followers)", agent::name_of(*name), v.0))
        .unwrap_or_else(|| "none".to_string());
    let o = Outcome {
        seed: cfg.seed,
        label,
        ticks: sim.tick,
        peak_pop,
        final_pop: m.pop,
        innovations: sim.innovations.len(),
        mean_known: m.mean_known,
        peak_level,
        final_level: m.level,
        soil: m.soil,
        settled: m.settled,
        top_leader,
        events: sim.events.events.len(),
    };
    (o, sim)
}

/// Many worlds at once: same rules, different dice. Prints an outcome table.
fn experiment(cfg: &Config, a: u64, b: u64) {
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(2);
    let seeds: Vec<u64> = (a..=b).collect();
    eprintln!("experiment: seeds {a}..={b} ({} worlds) on {threads} threads, {} ticks each", seeds.len(), cfg.ticks);
    let start = Instant::now();
    let outcomes = std::sync::Mutex::new(Vec::new());
    let next = std::sync::atomic::AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if k >= seeds.len() {
                    break;
                }
                let mut c = cfg.clone();
                c.seed = seeds[k];
                c.quiet = true;
                let (o, _) = run_one(c);
                eprintln!("  seed {:>4}: {:<26} {:>6} ticks, pop {:>4}, era {}", o.seed, o.label, o.ticks, o.final_pop, stats::ERA_NAMES[o.final_level]);
                outcomes.lock().unwrap().push(o);
            });
        }
    });
    let mut outcomes = outcomes.into_inner().unwrap();
    outcomes.sort_by_key(|o| o.seed);
    let secs = start.elapsed().as_secs_f64();

    println!("\n{:>5} {:<26} {:>6} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:<9} {:<9} {}",
        "seed", "outcome", "ticks", "peak", "final", "innov", "known", "soil%", "stay%", "era", "peak era", "greatest leader");
    for o in &outcomes {
        println!("{:>5} {:<26} {:>6} {:>5} {:>5} {:>5} {:>5.1} {:>5.0} {:>5.0} {:<9} {:<9} {}",
            o.seed, o.label, o.ticks, o.peak_pop, o.final_pop, o.innovations, o.mean_known, o.soil * 100.0, o.settled * 100.0,
            stats::ERA_NAMES[o.final_level], stats::ERA_NAMES[o.peak_level], o.top_leader);
    }
    let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for o in &outcomes {
        *counts.entry(o.label).or_default() += 1;
    }
    let summary: Vec<String> = counts.iter().map(|(k, v)| format!("{k}: {v}")).collect();
    println!("\n{} worlds in {:.0}s. {}", outcomes.len(), secs, summary.join(", "));

    let path = format!("{}/experiment_{}_{}.csv", cfg.out_dir, a, b);
    let mut f = BufWriter::new(std::fs::File::create(&path).expect("create experiment csv"));
    writeln!(f, "seed,outcome,ticks,peak_pop,final_pop,innovations,mean_known,soil_health,settled_share,final_level,peak_level,greatest_leader,events").unwrap();
    for o in &outcomes {
        writeln!(f, "{},{},{},{},{},{},{:.3},{:.4},{:.4},{},{},{},{}", o.seed, o.label, o.ticks, o.peak_pop, o.final_pop, o.innovations, o.mean_known, o.soil, o.settled, o.final_level, o.peak_level, o.top_leader, o.events).unwrap();
    }
    println!("written to {path}");
}

/// Print the dominant lineages, strategies, innovations, leaders and the history of one world.
fn summarize(sim: &sim::Sim) {
    use std::collections::HashMap;
    let mut by: HashMap<u32, (usize, [u32; brain::N_ACT], f32)> = HashMap::new();
    for a in &sim.agents {
        let e = by.entry(a.lineage).or_insert((0, [0; brain::N_ACT], 0.0));
        e.0 += 1;
        e.1[a.last_action as usize] += 1;
        e.2 += a.inventory;
    }
    let mut rows: Vec<_> = by.into_iter().collect();
    rows.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
    println!("\nTop lineages at end (pop, mean inventory, last-action mix):");
    for (lineage, (pop, acts, inv)) in rows.iter().take(8) {
        let mix: Vec<String> = brain::Action::ALL
            .iter()
            .map(|a| format!("{} {:.0}%", a.name(), 100.0 * acts[*a as usize] as f32 / *pop as f32))
            .collect();
        println!("  lineage {:>4}: pop {:>5}  inv {:>5.1}  {}", lineage, pop, inv / *pop as f32, mix.join("  "));
    }

    let r = strategy::analyse(&sim.agents);
    let total: usize = r.strategies.iter().map(|s| s.count).sum::<usize>().max(1);
    println!("\nStrategies at end: {} clusters, entropy {:.2} bits, {:.1} effective ways of living", r.strategies.len(), r.entropy, r.effective);
    println!("  {:>6} {:>6} {:>7} {:>6} {:>4}  {}", "share", "count", "wealth", "age", "lin", "profile");
    for s in &r.strategies {
        println!("  {:>5.1}% {:>6} {:>7.1} {:>6.0} {:>4}  {}", 100.0 * s.count as f32 / total as f32, s.count, s.mean_wealth, s.mean_age, s.lineages, strategy::describe(&s.centroid));
    }

    let n = sim.agents.len().max(1) as f32;
    println!("\nInnovations of this world ({}), with share of the living who know each:", sim.innovations.len());
    for (idx, inn) in sim.innovations.iter().enumerate() {
        let bit = 1u64 << idx;
        let share = sim.agents.iter().filter(|a| a.known & bit != 0).count() as f32 / n;
        println!("  tick {:>6}  {:>4.0}%  lineage {:>3}  {}", inn.born_tick, share * 100.0, inn.lineage, inn.describe());
    }

    let mut hall: Vec<_> = sim.hall.iter().map(|(name, v)| (*name, *v)).collect();
    hall.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
    println!("\nHall of fame (greatest followings):");
    for (name, (peak, lineage, tick)) in hall.iter().take(5) {
        println!("  {:<10} lineage {:>3}  {:>3} followers at tick {}", agent::name_of(*name), lineage, peak, tick);
    }

    let noisy = ["famine", "war", "plague toll", "raids", "drought", "a year of plenty", "plague:", "flood", "wildfire", "bounty", "harsh year", "leader ", "innovation:"];
    println!("\nHistory ({} events):", sim.events.events.len());
    for e in sim.events.events.iter().filter(|e| !noisy.iter().any(|p| e.text.starts_with(p))) {
        println!("  tick {:>6}: {}", e.tick, e.text);
    }
}
