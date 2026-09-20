mod agent;
mod brain;
mod config;
mod render;
mod rng;
mod sim;
mod spatial;
mod stats;
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

    let mut sim = sim::Sim::new(cfg.clone());
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
            let m = stats::compute(t, sim.world.season(t), &sim.agents, sim.world.total_food(), window);
            stats::print_row(&m);
            stats::csv_row(&mut csv, &m).unwrap();
        }
    }
    csv.flush().unwrap();

    let secs = start.elapsed().as_secs_f64();
    eprintln!(
        "done: {} ticks in {:.2}s ({:.0} ticks/s), final pop {}, csv: {}",
        cfg.ticks, secs, cfg.ticks as f64 / secs, sim.agents.len(), csv_path
    );
    summarize(&sim);
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
