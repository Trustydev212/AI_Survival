//! All tunable world/agent rules in one place. This is the "harness":
//! the fixed physics of the world that brains must adapt to.

#[derive(Clone, Debug)]
pub struct Config {
    pub seed: u64,
    pub width: usize,
    pub height: usize,
    pub agents: usize,
    pub tribes: usize,
    pub max_agents: usize,
    pub min_pop: usize,
    pub ticks: u64,
    pub log_every: u64,
    pub image_every: u64,
    pub out_dir: String,
    /// Toroidal map: no edges, no edge-hugging artefacts.
    pub wrap: bool,

    // world
    pub max_food: f32,
    pub regrow: f32,
    pub season_len: f32,

    // body
    pub max_energy: f32,
    pub start_energy: f32,
    pub base_cost: f32,
    pub move_cost: f32,
    pub rest_factor: f32,
    pub speed: f32,
    pub max_age: u32,
    pub vision: f32,
    pub inv_cap: f32,
    pub eat_below: f32,
    pub eat_amount: f32,

    // actions
    pub gather_rate: f32,
    pub attack_range: f32,
    pub attack_cost: f32,
    pub attack_damage: f32,
    pub steal: f32,
    pub share_amount: f32,
    pub share_range: f32,
    pub kin_threshold: f32,
    pub repro_threshold: f32,
    pub repro_cost: f32,
    pub child_energy: f32,

    // evolution
    pub p_mut: f32,
    pub sigma: f32,

    // culture
    pub p_discover: f32,
    pub p_learn: f32,
    pub learn_range: f32,
    pub tools_gather_mult: f32,
    pub farm_boost: f32,
    pub cult_gain: f32,
    pub settle_ticks: u16,
    pub cult_decay: f32,
    pub weapon_mult: f32,
    pub cooking_saving: f32,
    /// Tech bitmask founders start with (1 tools, 2 farming, 4 weapons, 8 cooking).
    pub start_tech: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            seed: 42,
            width: 192,
            height: 192,
            agents: 1000,
            tribes: 20,
            max_agents: 4000,
            min_pop: 60,
            ticks: 20_000,
            log_every: 500,
            image_every: 0,
            out_dir: "out".to_string(),
            wrap: true,

            max_food: 10.0,
            regrow: 0.08,
            season_len: 2000.0,

            max_energy: 100.0,
            start_energy: 60.0,
            base_cost: 0.15,
            move_cost: 0.12,
            rest_factor: 0.4,
            speed: 1.0,
            max_age: 3000,
            vision: 6.0,
            inv_cap: 100.0,
            eat_below: 50.0,
            eat_amount: 5.0,

            gather_rate: 2.0,
            attack_range: 1.5,
            attack_cost: 2.0,
            attack_damage: 15.0,
            steal: 20.0,
            share_amount: 8.0,
            share_range: 2.0,
            kin_threshold: 0.75,
            repro_threshold: 75.0,
            repro_cost: 40.0,
            child_energy: 35.0,

            p_mut: 0.08,
            sigma: 0.15,

            p_discover: 0.000002,
            p_learn: 0.01,
            learn_range: 2.5,
            tools_gather_mult: 1.5,
            farm_boost: 12.0,
            cult_gain: 0.02,
            settle_ticks: 5,
            cult_decay: 0.99,
            weapon_mult: 1.5,
            cooking_saving: 0.25,
            start_tech: 0,
        }
    }
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, String> {
        let mut c = Config::default();
        let mut i = 0;
        while i < args.len() {
            let key = args[i].as_str();
            if key == "--help" || key == "-h" {
                return Err(HELP.to_string());
            }
            let val = args.get(i + 1).ok_or_else(|| format!("missing value for {key}"))?;
            macro_rules! set {
                ($field:ident) => {
                    c.$field = val.parse().map_err(|_| format!("bad value for {key}: {val}"))?
                };
            }
            match key {
                "--seed" => set!(seed),
                "--width" => set!(width),
                "--height" => set!(height),
                "--agents" => set!(agents),
                "--tribes" => set!(tribes),
                "--max-agents" => set!(max_agents),
                "--min-pop" => set!(min_pop),
                "--ticks" => set!(ticks),
                "--log-every" => set!(log_every),
                "--image-every" => set!(image_every),
                "--out" => c.out_dir = val.clone(),
                "--wrap" => c.wrap = val == "1" || val == "true",
                "--regrow" => set!(regrow),
                "--max-food" => set!(max_food),
                "--season-len" => set!(season_len),
                "--base-cost" => set!(base_cost),
                "--max-age" => set!(max_age),
                "--attack-damage" => set!(attack_damage),
                "--steal" => set!(steal),
                "--p-mut" => set!(p_mut),
                "--sigma" => set!(sigma),
                "--p-discover" => set!(p_discover),
                "--p-learn" => set!(p_learn),
                "--farm-boost" => set!(farm_boost),
                "--settle-ticks" => set!(settle_ticks),
                "--start-tech" => set!(start_tech),
                "--cult-decay" => set!(cult_decay),
                _ => return Err(format!("unknown flag {key}\n{HELP}")),
            }
            i += 2;
        }
        Ok(c)
    }
}

pub const HELP: &str = "AI Survival headless sim

USAGE: sim [--flag value ...]

  --seed N          RNG seed (default 42)
  --agents N        initial agents (1000)
  --tribes N        founding tribes, agents spawn clustered per tribe (20)
  --ticks N         ticks to run (20000)
  --width/--height  map size in cells (192)
  --max-agents N    hard population cap (4000)
  --min-pop N       below this, random immigrants arrive (60)
  --log-every N     stats interval (500)
  --image-every N   write out/frame_XXXXXX.ppm every N ticks (0 = off)
  --out DIR         output directory (out)
  --wrap 0|1        toroidal map (1)
  --regrow F        food regrowth per tick (0.08)
  --max-food F      food cap per fully fertile cell (10)
  --season-len F    ticks per year (2000)
  --base-cost F     energy burned per tick (0.15)
  --max-age N       ticks before dying of old age (3000)
  --attack-damage F --steal F --p-mut F --sigma F
  --p-discover F    per agent-tick chance of discovering tools while gathering (2e-6);
                    farming and cooking are half as likely, weapons 20x per attack
  --p-learn F       chance per tick of learning a tech from an adjacent kin (0.01)
  --farm-boost F    regrowth multiplier of a fully cultivated cell (12)
  --cult-decay F    cultivation kept per tick when untended (0.99)
  --settle-ticks N  ticks an agent must stay still before its farming takes effect (5)
  --start-tech N    tech bitmask founders begin with: 1 tools, 2 farming, 4 weapons, 8 cooking (0)
";
