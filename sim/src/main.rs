mod agent;
mod ark;
mod brain;
mod config;
mod craft;
mod events;
mod herd;
mod innovation;
mod orders;
mod persist;
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

    if let Some(path) = cfg.eval_path.clone() {
        evaluate(&cfg, &path);
        return;
    }
    if cfg.forever {
        run_forever(cfg);
        return;
    }
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
    let log = events::EventLog::new(Some(&events_path), !cfg.quiet);
    let mut sim = match &cfg.load_path {
        Some(p) => persist::load(&cfg, p, log).unwrap_or_else(|e| {
            eprintln!("could not pick up {p}: {e}");
            std::process::exit(1);
        }),
        None => sim::Sim::new(cfg.clone(), log),
    };
    let start_tick = sim.tick;
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
    let mut peak_level = 0;
    let mut peak_pop = sim.agents.len();
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
        if t % cfg.log_every == 0 || sim.agents.is_empty() || t == start_tick + cfg.ticks {
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

    if let Some(path) = &cfg.save_path {
        if let Err(e) = persist::save(&sim, path) {
            eprintln!("could not put the world down at {path}: {e}");
        } else if !cfg.quiet {
            eprintln!("world saved to {path} at tick {}", sim.tick);
        }
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

/// A world with no chosen stopping point.
///
/// Every table in docs/THEORY.md was measured over a window someone picked, and section 15 showed
/// the window decides the answer: the same world is thriving at twenty thousand ticks and boom and
/// bust at sixty thousand. This mode picks no window. It runs until it is stopped, puts the world
/// down at intervals so nothing is lost, and when a civilisation dies out it writes down what that
/// one managed and begins another on fresh ground, counting the generations.
///
/// The chronicle is the record that survives the worlds: one line per civilisation, what it
/// reached and how long it lasted. That is the only thing here meant to be read years from now.
fn run_forever(cfg: Config) {
    let chronicle_path = format!("{}/chronicle.txt", cfg.out_dir);
    let mut generation = count_generations(&chronicle_path);
    let save_path = cfg.save_path.clone().unwrap_or_else(|| format!("{}/world.bin", cfg.out_dir));
    let mut sim = open_world(&cfg, &save_path, generation);
    let mut csv = generation_csv(&cfg, generation);
    let mut ticks_this_session = 0u64;
    let started_at = std::time::Instant::now();
    let started_tick = sim.tick;
    // Take a reading before the first tick. A shift that picks a world up has no statistics window
    // for another five hundred ticks, and the frames written before then used to carry the
    // placeholder: a world resumed looked, for its first few frames, like one that knew nothing.
    let mut last_metrics = (
        sim.world.soil_health(),
        0.0f32,
        if sim.agents.is_empty() { 0.0 } else { sim.agents.iter().map(|a| a.known_count() as f32).sum::<f32>() / sim.agents.len() as f32 },
    );
    // The live picture is a rolling window. A world with no end cannot keep every frame, so the
    // file starts over every so often and the viewer is shown the recent stretch.
    let mut snap = open_window(&cfg, &sim);
    eprintln!("world=v{} ({}) forever: generation {generation} from tick {}", version::WORLD, version::NOTE, sim.tick);
    loop {
        sim.step();
        ticks_this_session += 1;
        let t = sim.tick;
        if t % cfg.log_every == 0 || sim.agents.is_empty() {
            let window = sim.take_window();
            let m = stats::compute(
                t, sim.world.season(t), sim.world.climate, &sim.agents, sim.world.total_food(), sim.world.soil_health(),
                sim.regions.inhabited_soil(&sim.agents), sim.innovations.len(), sim.world.cultivated_cells(),
                sim.settled_share(), sim.order_mix(), sim.custom_mix(), sim.stores.list.len(), sim.stores.total_food(),
                sim.world.building_count(), window,
            );
            if !cfg.quiet {
                stats::print_row(&m);
            }
            stats::csv_row(&mut csv, &m).unwrap();
            let _ = csv.flush();
            last_metrics = (m.soil, m.obedience, m.mean_known);
            // Notice the brains that did well while their owners are still alive: at the end of a
            // civilisation there is nobody left to ask.
            sim.ark.consider(&sim.agents, generation, t);
            sim.events.check_window(&m, &sim.agents, &sim.innovations, cfg.log_every, &sim.defected);
            sim.defected.clear();
        }
        if let Some(sn) = snap.as_mut() {
            if cfg.snapshot_window > 0 && t % cfg.snapshot_window == 0 {
                snap = open_window(&cfg, &sim);
            } else if t % cfg.snapshot_every == 0 {
                let (soil, obey, known) = last_metrics;
                let era = stats::level_of(known, sim.settled_share()) as u8;
                let _ = sn.frame(t, era, &sim.world, &sim.agents, &sim.stores.list, &sim.herds, soil, obey, known,
                    sim.world.season(t), cfg.settle_ticks, cfg.custom_min, &sim.innovations);
                if sn.frames % 10 == 1 {
                    write_live_meta(&cfg, sn.frames, &sim, t);
                }
            }
        }
        if cfg.save_every > 0 && t % cfg.save_every == 0 {
            put_down(&sim, &save_path);
            let _ = sim.ark.save(&ark_path(&cfg));
            write_status(&cfg, &sim, generation, ticks_this_session as f64 / started_at.elapsed().as_secs_f64().max(0.001), started_tick);
        }
        if sim.agents.is_empty() {
            // Read the age back off its own record rather than off this shift, because a
            // civilisation usually outlives the shift that happened to be watching when it died.
            let (peak, best_known) = high_water(&cfg, generation);
            let (cause, detail) = postmortem(&cfg, generation);
            append_line(&format!("{}/postmortem.txt", cfg.out_dir),
                &format!("\n== generation {generation}, tick {t}: {cause} ==\n{detail}\nDeepest thing made: {}\n", deepest_made(&sim)));
            let line = format!(
                "generation {generation}\tseed {}\tlived {t} ticks\tfounded {} reached {} people\t{} things\t{:.1} known per head at its best\tark {} brains, best left {} children\tended at tick {t}: {cause}",
                seed_for(&cfg, generation), cfg.agents, peak, sim.innovations.iter().filter(|i| !i.name.is_empty()).count(),
                best_known, sim.ark.kept.len(), sim.ark.best(),
            );
            let _ = sim.ark.save(&ark_path(&cfg));
            append_line(&chronicle_path, &line);
            eprintln!("{line}");
            generation += 1;
            sim = open_world(&cfg, "", generation);
            csv = generation_csv(&cfg, generation);
            snap = open_window(&cfg, &sim);
            put_down(&sim, &save_path);
            continue;
        }
        if cfg.ticks > 0 && ticks_this_session >= cfg.ticks {
            put_down(&sim, &save_path);
            let _ = sim.ark.save(&ark_path(&cfg));
            write_status(&cfg, &sim, generation, ticks_this_session as f64 / started_at.elapsed().as_secs_f64().max(0.001), started_tick);
            eprintln!("stopping this shift at tick {t}, generation {generation}");
            return;
        }
    }
}

/// Start a fresh window of the live picture. The viewer always reads the same two names, so a
/// window starting over looks to it like a world being reloaded rather than a new address.
fn open_window(cfg: &Config, sim: &sim::Sim) -> Option<snapshot::Snapshot> {
    if cfg.snapshot_every == 0 {
        return None;
    }
    let path = format!("{}/live.bin", cfg.out_dir);
    match snapshot::Snapshot::create(&path, &sim.world) {
        Ok(s) => {
            write_live_meta(cfg, 0, sim, sim.tick);
            Some(s)
        }
        Err(e) => {
            eprintln!("could not open the live picture: {e}");
            None
        }
    }
}

fn write_live_meta(cfg: &Config, frames: u32, sim: &sim::Sim, tick: u64) {
    let mut names: Vec<(u32, String)> = sim.hall.keys().map(|id| (*id, agent::name_of(*id))).collect();
    names.sort();
    let meta = format!("{}/live.json", cfg.out_dir);
    let _ = snapshot::write_meta(&meta, cfg.width, cfg.height, frames, &names, &stats::ERA_NAMES, cfg.seed, tick, cfg.snapshot_every);
}

fn seed_for(cfg: &Config, generation: u32) -> u64 {
    cfg.seed.wrapping_add(generation as u64 * 7919)
}

/// Pick the world up if there is one to pick up, otherwise make one.
fn open_world(cfg: &Config, save_path: &str, generation: u32) -> sim::Sim {
    let mut c = cfg.clone();
    c.seed = seed_for(cfg, generation);
    let events_path = format!("{}/events_gen{generation}.txt", cfg.out_dir);
    let log = events::EventLog::new(Some(&events_path), !cfg.quiet);
    let ark = ark::Ark::load(&ark_path(cfg));
    if !save_path.is_empty() && std::path::Path::new(save_path).exists() {
        match persist::load(&c, save_path, log) {
            Ok(mut s) => {
                s.ark = ark;
                return s;
            }
            Err(e) => {
                eprintln!("could not pick up {save_path}: {e}\nstarting a fresh world instead");
                let log = events::EventLog::new(Some(&events_path), !cfg.quiet);
                return sim::Sim::new_founded(c.clone(), log, ark);
            }
        }
    }
    if !ark.kept.is_empty() {
        eprintln!("founding from {} brains carried over, best left {} children", ark.kept.len(), ark.best());
    }
    sim::Sim::new_founded(c, log, ark)
}

fn ark_path(cfg: &Config) -> String {
    format!("{}/ark.bin", cfg.out_dir)
}

fn generation_csv(cfg: &Config, generation: u32) -> BufWriter<std::fs::File> {
    let path = format!("{}/stats_gen{generation}.csv", cfg.out_dir);
    let fresh = !std::path::Path::new(&path).exists();
    let file = std::fs::OpenOptions::new().create(true).append(true).open(&path).expect("open stats");
    let mut w = BufWriter::new(file);
    if fresh {
        stats::csv_header(&mut w).unwrap();
    }
    w
}

/// A line anyone can read to see the world is alive: where it is, how fast, and what it has.
/// Written beside the world every time it is put down, and published with it.
fn write_status(cfg: &Config, sim: &sim::Sim, generation: u32, rate: f64, started: u64) {
    let known = if sim.agents.is_empty() { 0.0 } else { sim.agents.iter().map(|a| a.known_count() as f32).sum::<f32>() / sim.agents.len() as f32 };
    let body = format!(
        "{{\"world\":{},\"generation\":{generation},\"tick\":{},\"people\":{},\"things\":{},\"known_per_head\":{:.1},\"buildings\":{},\"ticks_per_second\":{:.0},\"this_shift\":{},\"written\":\"{}\"}}\n",
        version::WORLD, sim.tick, sim.agents.len(),
        sim.innovations.iter().filter(|i| !i.name.is_empty()).count(), known,
        sim.world.building_count(), rate, sim.tick.saturating_sub(started),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0),
    );
    let _ = std::fs::write(format!("{}/status.json", cfg.out_dir), body);
}

fn put_down(sim: &sim::Sim, path: &str) {
    // Write beside the real file and move it into place, so a stop half way through a save
    // leaves the previous world intact rather than half of two.
    let tmp = format!("{path}.part");
    if persist::save(sim, &tmp).is_ok() {
        let _ = std::fs::rename(&tmp, path);
    }
}

fn append_line(path: &str, line: &str) {
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{line}");
    }
}

/// The most people and the most known per head a generation ever reached, from its own record.
fn high_water(cfg: &Config, generation: u32) -> (usize, f32) {
    let path = format!("{}/stats_gen{generation}.csv", cfg.out_dir);
    let Ok(text) = std::fs::read_to_string(&path) else { return (0, 0.0) };
    let mut lines = text.lines();
    let Some(header) = lines.next() else { return (0, 0.0) };
    let cols: Vec<&str> = header.split(',').collect();
    let (Some(pi), Some(ki)) = (cols.iter().position(|c| *c == "pop"), cols.iter().position(|c| *c == "mean_known")) else {
        return (0, 0.0);
    };
    let mut peak = 0usize;
    let mut known = 0.0f32;
    for line in lines {
        let f: Vec<&str> = line.split(',').collect();
        if let Some(v) = f.get(pi).and_then(|v| v.parse::<f32>().ok()) {
            peak = peak.max(v as usize);
        }
        if let Some(v) = f.get(ki).and_then(|v| v.parse::<f32>().ok()) {
            known = known.max(v);
        }
    }
    (peak, known)
}

fn count_generations(path: &str) -> u32 {
    std::fs::read_to_string(path).map(|s| s.lines().filter(|l| l.starts_with("generation")).count() as u32).unwrap_or(0)
}

/// Why a civilisation ended, read off its own record.
///
/// "Everyone died" is not a cause. A world running for years is only worth watching if each
/// ending can be told apart from the others: starved on exhausted land is a different story from
/// killed each other, and both are different from simply stopping having children. All of it is
/// already in the generation's own statistics; nobody had ever read it back.
fn postmortem(cfg: &Config, generation: u32) -> (String, String) {
    let path = format!("{}/stats_gen{generation}.csv", cfg.out_dir);
    let Ok(text) = std::fs::read_to_string(&path) else {
        return ("no record".into(), String::new());
    };
    let mut lines = text.lines();
    let Some(header) = lines.next() else { return ("no record".into(), String::new()) };
    let cols: Vec<&str> = header.split(',').collect();
    let at = |name: &str| cols.iter().position(|c| *c == name);
    let rows: Vec<Vec<f32>> = lines
        .map(|l| l.split(',').map(|v| v.parse::<f32>().unwrap_or(0.0)).collect())
        .filter(|r: &Vec<f32>| r.len() == cols.len())
        .collect();
    if rows.len() < 4 {
        return ("died too young to tell".into(), String::new());
    }
    let get = |r: &Vec<f32>, name: &str| at(name).and_then(|i| r.get(i).copied()).unwrap_or(0.0);
    // The last fifth of a life is where the ending is written.
    let tail = &rows[rows.len() * 4 / 5..];
    let sum = |name: &str| tail.iter().map(|r| get(r, name)).sum::<f32>();
    let (starved, killed, plague, aged) = (sum("starved"), sum("killed"), sum("plague_deaths"), sum("aged"));
    let births = sum("births");
    let deaths = starved + killed + plague + aged;
    let peak = rows.iter().enumerate().max_by(|a, b| get(a.1, "pop").total_cmp(&get(b.1, "pop"))).map(|(i, r)| (i, get(r, "pop"))).unwrap_or((0, 0.0));
    let peak_tick = get(&rows[peak.0], "tick");
    let end_tick = get(rows.last().unwrap(), "tick");
    let soil_end = get(rows.last().unwrap(), "soil_health");
    let soil_peak = get(&rows[peak.0], "soil_health");
    let food_end = get(rows.last().unwrap(), "food");
    let cause = if deaths < 1.0 && births < 1.0 {
        "nobody left to have children"
    } else if starved >= killed.max(plague).max(aged) {
        if soil_end < 0.6 { "starved on land they had worn out" } else { "starved" }
    } else if killed >= plague.max(aged) {
        "killed one another"
    } else if plague >= aged {
        "plague"
    } else {
        "grew old with too few born"
    };
    let detail = format!(
        "peaked at {:.0} people on tick {:.0}, then {:.0} ticks of decline. In its last stretch: \
         {starved:.0} starved, {killed:.0} killed, {plague:.0} taken by plague, {aged:.0} died old, \
         {births:.0} born. Soil {:.0}% at its peak and {:.0}% at the end, {:.0} food left standing.",
        peak.1, peak_tick, end_tick - peak_tick, soil_peak * 100.0, soil_end * 100.0, food_end,
    );
    (cause.into(), detail)
}

/// The deepest thing a civilisation ever made: the one measure of how far its craft got.
fn deepest_made(sim: &sim::Sim) -> String {
    sim.innovations
        .iter()
        .filter(|i| !i.name.is_empty())
        .filter_map(|i| i.craft.as_ref().map(|c| (c.depth, i.name.clone(), i.describe(&sim.innovations))))
        .max_by_key(|(d, _, _)| *d)
        .map(|(d, name, recipe)| format!("{name} ({d} deep): {recipe}"))
        .unwrap_or_else(|| "nothing made".into())
}

/// Does a brain from this world actually do better than one drawn at random?
///
/// Nothing in the repository answered that. Evolution and within-life learning both ran, and both
/// were measured by how the societies turned out, which mixes the brain with its luck, its land
/// and its neighbours. This is the missing measurement: take brains out of the world that made
/// them, drop copies of each into the *same* fresh world nobody evolved in, and count what they
/// manage. A held-out test, in the ordinary sense.
fn evaluate(cfg: &Config, path: &str) {
    let ark = ark::Ark::load(path);
    if ark.kept.is_empty() {
        eprintln!("no brains in {path}");
        return;
    }
    let trials: Vec<u64> = (901..=905).collect();
    eprintln!(
        "world=v{} evaluating {} brains from {path} against {} random ones, {} trials each of {} ticks",
        version::WORLD, ark.kept.len(), ark.kept.len(), trials.len(), cfg.ticks
    );
    let mut rng = rng::Rng::new(12345);
    let random: Vec<brain::Genome> = (0..ark.kept.len())
        .map(|_| {
            let marker = [rng.f32(), rng.f32(), rng.f32()];
            brain::Genome::random(&mut rng, marker)
        })
        .collect();
    let trained: Vec<brain::Genome> = ark.kept.iter().map(|s| s.genome.clone()).collect();
    let score_all = |set: &[brain::Genome]| -> Vec<f32> {
        set.iter().map(|g| trials.iter().map(|s| score_brain(cfg, g, *s)).sum::<f32>() / trials.len() as f32).collect()
    };
    let a = score_all(&random);
    let b = score_all(&trained);
    let mean = |v: &[f32]| v.iter().sum::<f32>() / v.len().max(1) as f32;
    let sd = |v: &[f32]| {
        let m = mean(v);
        (v.iter().map(|x| (x - m).powi(2)).sum::<f32>() / v.len().max(1) as f32).sqrt()
    };
    println!("brains        trials  mean score  spread");
    println!("random        {:>6}  {:>10.1}  {:>6.1}", a.len() * trials.len(), mean(&a), sd(&a));
    println!("from the ark  {:>6}  {:>10.1}  {:>6.1}", b.len() * trials.len(), mean(&b), sd(&b));
    let lift = if mean(&a).abs() > 0.01 { (mean(&b) - mean(&a)) / mean(&a) * 100.0 } else { 0.0 };
    println!("\ndifference: {:+.1} ({:+.0}%)", mean(&b) - mean(&a), lift);
    println!("A brain is scored by how many people its line leaves behind: twenty copies of it are put");
    println!("into an empty world of their own and counted after {} ticks.", cfg.ticks);
}

/// One brain, one fresh world, one number: how many of its line are alive at the end.
fn score_brain(cfg: &Config, genome: &brain::Genome, seed: u64) -> f32 {
    let mut c = cfg.clone();
    c.seed = seed;
    c.width = 64;
    c.height = 64;
    c.agents = 20;
    c.tribes = 1;
    c.quiet = true;
    c.out_dir = std::env::temp_dir().to_string_lossy().into_owned();
    c.snapshot_every = 0;
    c.image_every = 0;
    let mut sim = sim::Sim::new(c.clone(), events::EventLog::new(None, false));
    // Everyone in the trial is a copy of the brain being judged, so the score is the brain's and
    // not its neighbours'. Small differences remain because the world is not still.
    for a in sim.agents.iter_mut() {
        let marker = a.genome.marker;
        a.genome = genome.clone();
        a.genome.marker = marker;
    }
    for _ in 0..cfg.ticks {
        sim.step();
        if sim.agents.is_empty() {
            return 0.0;
        }
    }
    sim.agents.len() as f32
}
