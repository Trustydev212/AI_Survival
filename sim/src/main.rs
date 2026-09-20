mod agent;
mod brain;
mod config;
mod events;
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
    let csv_path = format!("{}/stats_seed{}.csv", cfg.out_dir, cfg.seed);
    let mut csv = BufWriter::new(std::fs::File::create(&csv_path).expect("create csv"));
    stats::csv_header(&mut csv).unwrap();

    eprintln!(
        "seed={} agents={} tribes={} map={}x{} ticks={} brain={}-{}-{} ({} weights)",
        cfg.seed, cfg.agents, cfg.tribes, cfg.width, cfg.height, cfg.ticks,
        brain::N_IN, brain::N_HID, brain::N_OUT, brain::N_WEIGHTS
    );

    let events_path = format!("{}/events_seed{}.txt", cfg.out_dir, cfg.seed);
    let mut sim = sim::Sim::new(cfg.clone(), events::EventLog::new(Some(&events_path)));
    stats::print_header();
    let start = Instant::now();

    for _ in 0..cfg.ticks {
        sim.step();
        let t = sim.tick;
        if cfg.image_every > 0 && t % cfg.image_every == 0 {
            let path = format!("{}/frame_{:06}.ppm", cfg.out_dir, t);
            render::write_ppm(&path, &sim.world, &sim.agents, 3).expect("write ppm");
        }
        if t % cfg.log_every == 0 {
            let window = sim.take_window();
            let m = stats::compute(
                t, sim.world.season(t), sim.world.climate, &sim.agents, sim.world.total_food(),
                sim.world.cultivated_cells(), sim.settled_share(), window,
            );
            stats::print_row(&m);
            stats::csv_row(&mut csv, &m).unwrap();
            sim.events.check_window(&m, &sim.agents, cfg.log_every);
        }
    }
    csv.flush().unwrap();
    sim.events.flush();

    let secs = start.elapsed().as_secs_f64();
    eprintln!(
        "done: {} ticks in {:.2}s ({:.0} ticks/s), final pop {}, csv: {}",
        cfg.ticks, secs, cfg.ticks as f64 / secs, sim.agents.len(), csv_path
    );
    summarize(&sim);
    summarize_strategies(&sim);
    let share = sim.tech_share();
    println!(
        "\nTechnology at end: {}",
        agent::TECH_NAMES.iter().zip(share.iter()).map(|(n, s)| format!("{n} {:.0}%", s * 100.0)).collect::<Vec<_>>().join("  ")
    );
    let mut hall: Vec<_> = sim.hall.iter().map(|(name, v)| (*name, *v)).collect();
    hall.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
    println!("\nHall of fame (greatest followings):");
    for (name, (peak, lineage, tick)) in hall.iter().take(5) {
        println!("  {:<10} lineage {:>3}  {:>3} followers at tick {}", agent::name_of(*name), lineage, peak, tick);
    }
    println!("\nHistory ({} events, written to {}):", sim.events.events.len(), events_path);
    let noisy = ["famine", "war", "plague toll", "raids", "drought", "a year of plenty", "plague:", "flood", "wildfire", "bounty", "harsh year", "leader "];
    for e in sim.events.events.iter().filter(|e| !noisy.iter().any(|p| e.text.starts_with(p))) {
        println!("  tick {:>6}: {}", e.tick, e.text);
    }
}

fn summarize_strategies(sim: &sim::Sim) {
    let r = strategy::analyse(&sim.agents);
    let total: usize = r.strategies.iter().map(|s| s.count).sum::<usize>().max(1);
    println!(
        "\nStrategies at end: {} clusters, entropy {:.2} bits, {:.1} effective ways of living",
        r.strategies.len(),
        r.entropy,
        r.effective
    );
    println!("  {:>6} {:>6} {:>7} {:>6} {:>4}  {}", "share", "count", "wealth", "age", "lin", "profile");
    for s in &r.strategies {
        println!(
            "  {:>5.1}% {:>6} {:>7.1} {:>6.0} {:>4}  {}",
            100.0 * s.count as f32 / total as f32,
            s.count,
            s.mean_wealth,
            s.mean_age,
            s.lineages,
            strategy::describe(&s.centroid)
        );
    }
}

/// Print the dominant lineages and what their members tend to do.
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
}
