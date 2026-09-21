mod agent;
mod brain;
mod config;
mod craft;
mod events;
mod herd;
mod innovation;
mod orders;
mod render;
mod region;
mod rng;
mod sim;
mod version;
mod snapshot;
mod spatial;
mod store;
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
    lived_soil: f32,
    obedience: f32,
    order: String,
    custom: String,
    breed_rate: f32,
    swing: f32,
    settled: f32,
    top_leader: String,
    events: usize,
    plastic: f32,
    sig_mi: f32,
    sig_meaning: f32,
    dol: f32,
    things: f32,
    equipped: f32,
    crafts: usize,
    learn_rate: f32,
    loudness: f32,
    hunts: u32,
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
        "world=v{} seed={} agents={} tribes={} map={}x{} ticks={} brain={}-{}-{} ({} weights)",
        version::WORLD, cfg.seed, cfg.agents, cfg.tribes, cfg.width, cfg.height, cfg.ticks,
        brain::N_IN, brain::N_HID, brain::N_OUT, brain::N_WEIGHTS
    );
    let start = Instant::now();
    let (o, sim) = run_one(cfg.clone());
    let secs = start.elapsed().as_secs_f64();
    eprintln!("done: {} ticks in {:.2}s ({:.0} ticks/s)", o.ticks, secs, o.ticks as f64 / secs);
    if cfg.profile {
        let total: f64 = sim.profile.iter().sum::<f64>().max(1e-9);
        let parts: Vec<String> = sim::Sim::PHASES.iter().zip(sim.profile.iter()).map(|(n, t)| format!("{n} {:.0}%", 100.0 * t / total)).collect();
        eprintln!("profile: {}", parts.join("  "));
    }
    summarize(&sim);
    println!("\nOutcome: {} after {} ticks. Peak pop {}, final pop {}, {} innovations, {:.1} known per head,",
        o.label, o.ticks, o.peak_pop, o.final_pop, o.innovations, o.mean_known);
    println!("soil {:.0}% overall and {:.0}% where people live, {:.0}% of orders obeyed (mostly {}), custom {}, {:.1} births per 1000 fertile ticks, swing x{:.1}, era {} (peak {}).",
        o.soil * 100.0, o.lived_soil * 100.0, o.obedience * 100.0, o.order, o.custom, o.breed_rate, o.swing, stats::ERA_NAMES[o.final_level], stats::ERA_NAMES[o.peak_level]);
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

    let mut snap = if cfg.snapshot_every > 0 {
        let path = format!("{}/snap_seed{}.bin", cfg.out_dir, cfg.seed);
        Some(snapshot::Snapshot::create(&path, &sim.world).expect("create snapshot"))
    } else {
        None
    };
    let mut last_metrics: (f32, f32, f32) = (1.0, 0.0, 0.0); // soil, obedience, mean_known
    let mut peak_pop = sim.agents.len();
    let mut peak_level = 0;
    let mut last: Option<stats::Metrics> = None;
    let mut extinct = false;
    let mut pops: Vec<usize> = Vec::new();
    for _ in 0..cfg.ticks {
        sim.step();
        let t = sim.tick;
        peak_pop = peak_pop.max(sim.agents.len());
        if let Some(sn) = snap.as_mut() {
            if t % cfg.snapshot_every == 0 || sim.agents.is_empty() {
                let (soil, obey, known) = last_metrics;
                let era = stats::level_of(known, sim.settled_share()) as u8;
                sn.frame(t, era, &sim.world, &sim.agents, &sim.stores.list, &sim.herds, soil, obey, known, sim.world.season(t), cfg.settle_ticks, cfg.custom_min, &sim.innovations)
                    .expect("write snapshot frame");
                if sn.frames % 20 == 1 {
                    let mut names: Vec<(u32, String)> = sim.hall.keys().map(|id| (*id, agent::name_of(*id))).collect();
                    names.sort();
                    let meta = format!("{}/meta_seed{}.json", cfg.out_dir, cfg.seed);
                    let _ = snapshot::write_meta(&meta, cfg.width, cfg.height, sn.frames, &names, &stats::ERA_NAMES, cfg.seed, t, cfg.snapshot_every);
                }
            }
        }
        if cfg.image_every > 0 && t % cfg.image_every == 0 {
            let path = format!("{}/frame_{:06}.ppm", cfg.out_dir, t);
            render::write_ppm(&path, &sim.world, &sim.agents, 3).expect("write ppm");
        }
        // A run shorter than one window used to end in a panic, which made a quick trial of
        // the sim impossible; the last tick always closes a window now.
        if t % cfg.log_every == 0 || sim.agents.is_empty() || t == cfg.ticks {
            let window = sim.take_window();
            let m = stats::compute(
                t, sim.world.season(t), sim.world.climate, &sim.agents, sim.world.total_food(), sim.world.soil_health(),
                sim.regions.inhabited_soil(&sim.agents), sim.innovations.len(), sim.world.cultivated_cells(),
                sim.settled_share(), sim.order_mix(), sim.custom_mix(), sim.stores.list.len(), sim.stores.total_food(), sim.world.building_count(), window,
            );
            if !cfg.quiet {
                stats::print_row(&m);
            }
            stats::csv_row(&mut csv, &m).unwrap();
            sim.events.check_window(&m, &sim.agents, &sim.innovations, cfg.log_every, &sim.defected);
            sim.defected.clear();
            peak_level = peak_level.max(m.level);
            last_metrics = (m.soil, m.obedience, m.mean_known);
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
    if let Some(sn) = snap {
        let frames = sn.frames;
        sn.finish().expect("finish snapshot");
        let mut names: Vec<(u32, String)> = sim.hall.keys().map(|id| (*id, agent::name_of(*id))).collect();
        names.sort();
        let meta = format!("{}/meta_seed{}.json", cfg.out_dir, cfg.seed);
        snapshot::write_meta(&meta, cfg.width, cfg.height, frames, &names, &stats::ERA_NAMES, cfg.seed, sim.tick, cfg.snapshot_every).expect("write meta");
    }

    let m = last.expect("at least one stats window");
    // Population swings within a year and between booms, so judge the end against the
    // best stretch of the run, both smoothed over several windows.
    let n = pops.len().max(1);
    let smooth = (((cfg.season_len as u64 / cfg.log_every.max(1)) as usize).max(4)).min(n);
    let mean_of = |w: &[usize]| w.iter().sum::<usize>() as f32 / w.len().max(1) as f32;
    let best = pops.windows(smooth).map(mean_of).fold(0.0f32, f32::max).max(1.0);
    let end = mean_of(&pops[pops.len().saturating_sub(smooth)..]);
    // How violently the population swings late in the run.
    let tail = &pops[pops.len() * 2 / 3..];
    let hi = tail.iter().copied().max().unwrap_or(1).max(1) as f32;
    let lo = tail.iter().copied().min().unwrap_or(1).max(1) as f32;
    let swing = hi / lo;
    let label = if extinct {
        "extinct"
    } else if end < 0.25 * best {
        "collapsed"
    } else if swing >= 4.0 {
        if m.soil >= 0.5 { "boom and bust" } else { "boom and bust on dying land" }
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
        lived_soil: m.lived_soil,
        obedience: m.obedience,
        order: stats::dominant_order(&m.order_mix),
        custom: stats::dominant_order(&m.custom_mix),
        breed_rate: m.breed_rate,
        swing,
        settled: m.settled,
        top_leader,
        events: sim.events.events.len(),
        plastic: m.plastic,
        sig_mi: m.sig_mi,
        sig_meaning: m.sig_meaning,
        dol: m.dol,
        things: m.things,
        equipped: m.equipped,
        crafts: sim.innovations.iter().filter(|i| i.craft.is_some() && !i.name.is_empty()).count(),
        learn_rate: m.learn_rate,
        loudness: m.loudness,
        hunts: sim.hunts_total,
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
                c.threads = 1;
                let (o, _) = run_one(c);
                eprintln!("  seed {:>4}: {:<26} {:>6} ticks, pop {:>4}, era {}", o.seed, o.label, o.ticks, o.final_pop, stats::ERA_NAMES[o.final_level]);
                outcomes.lock().unwrap().push(o);
            });
        }
    });
    let mut outcomes = outcomes.into_inner().unwrap();
    outcomes.sort_by_key(|o| o.seed);
    let secs = start.elapsed().as_secs_f64();

    println!("\n{:>5} {:<26} {:>6} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:<12} {:<12} {:>5} {:>5} {:<8} {}",
        "seed", "outcome", "ticks", "peak", "final", "innov", "known", "soil%", "lsoil", "stay%", "obey", "order", "custom", "breed", "swing", "era", "greatest leader");
    for o in &outcomes {
        println!("{:>5} {:<26} {:>6} {:>5} {:>5} {:>5} {:>5.1} {:>5.0} {:>5.0} {:>5.0} {:>5.2} {:<12} {:<12} {:>5.1} {:>5.1} {:<8} {}",
            o.seed, o.label, o.ticks, o.peak_pop, o.final_pop, o.innovations, o.mean_known, o.soil * 100.0,
            o.lived_soil * 100.0, o.settled * 100.0, o.obedience, o.order, o.custom, o.breed_rate, o.swing,
            stats::ERA_NAMES[o.final_level], o.top_leader);
    }
    let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for o in &outcomes {
        *counts.entry(o.label).or_default() += 1;
    }
    let summary: Vec<String> = counts.iter().map(|(k, v)| format!("{k}: {v}")).collect();
    println!("\n{} worlds in {:.0}s. {}", outcomes.len(), secs, summary.join(", "));

    let path = format!("{}/experiment_{}_{}.csv", cfg.out_dir, a, b);
    let mut f = BufWriter::new(std::fs::File::create(&path).expect("create experiment csv"));
    writeln!(f, "world,seed,outcome,ticks,peak_pop,final_pop,innovations,mean_known,soil_health,lived_soil,settled_share,obedience,dominant_order,dominant_custom,breed_rate,swing,final_level,peak_level,greatest_leader,events,plastic,signal_mi,things_per_head,equipped_share,crafts,learn_rate,loudness,hunts,signal_meaning,division_of_labour").unwrap();
    for o in &outcomes {
        writeln!(f, "{},{},{},{},{},{},{},{:.3},{:.4},{:.4},{:.4},{:.4},{},{},{:.4},{:.3},{},{},{},{},{:.4},{:.4},{:.3},{:.3},{},{:.3},{:.3},{},{:.4},{:.3}", version::WORLD, o.seed, o.label, o.ticks, o.peak_pop, o.final_pop, o.innovations, o.mean_known, o.soil, o.lived_soil, o.settled, o.obedience, o.order, o.custom, o.breed_rate, o.swing, o.final_level, o.peak_level, o.top_leader, o.events, o.plastic, o.sig_mi, o.things, o.equipped, o.crafts, o.learn_rate, o.loudness, o.hunts, o.sig_meaning, o.dol).unwrap();
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
        if inn.name.is_empty() {
            continue; // a recipe the world forgot
        }
        let share = sim.agents.iter().filter(|a| a.known.has(idx)).count() as f32 / n;
        println!("  tick {:>6}  {:>4.0}%  lineage {:>3}  {}", inn.born_tick, share * 100.0, inn.lineage, inn.describe(&sim.innovations));
    }

    {
        let crafts = sim.innovations.iter().filter(|i| i.craft.is_some() && !i.name.is_empty()).count();
        let mut held = [0usize; craft::N_SLOT];
        for a in &sim.agents {
            for (s, g) in a.gear.iter().enumerate() {
                if g.is_some() {
                    held[s] += 1;
                }
            }
        }
        let with_mats = sim.agents.iter().filter(|a| a.mats.iter().any(|m| *m > 0)).count();
        println!("\nThings: {} recipes and {} practices; {} shelters standing; {} of {} carry materials.", crafts, sim.innovations.len() - crafts, sim.world.building_count(), with_mats, sim.agents.len());
        let parts: Vec<String> = (0..craft::N_SLOT - 1).map(|s| format!("{} {}", craft::SLOT_NAMES[s], held[s])).collect();
        println!("Held now: {}", parts.join(", "));
    }

    let mut hall: Vec<_> = sim.hall.iter().map(|(name, v)| (*name, *v)).collect();
    hall.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
    println!("\nHall of fame (greatest followings):");
    for (name, (peak, lineage, tick)) in hall.iter().take(5) {
        println!("  {:<10} lineage {:>3}  {:>3} followers at tick {}", agent::name_of(*name), lineage, peak, tick);
    }

    let noisy = ["famine", "war", "plague toll", "raids", "drought", "a year of plenty", "plague:", "flood", "wildfire", "bounty", "harsh year", "leader ", "innovation:", "crafted:", "lineage ", "rivalry:"];
    println!("\nHistory ({} events):", sim.events.events.len());
    for e in sim.events.events.iter().filter(|e| !noisy.iter().any(|p| e.text.starts_with(p))) {
        println!("  tick {:>6}: {}", e.tick, e.text);
    }
}
