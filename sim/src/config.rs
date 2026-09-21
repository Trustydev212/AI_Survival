//! All tunable world/agent rules in one place. This is the "harness":
//! the fixed physics of the world that brains must adapt to.

#[derive(Clone, Debug)]
pub struct Config {
    pub seed: u64,
    pub seeds: Option<(u64, u64)>,
    pub quiet: bool,
    pub profile: bool,
    /// Worker threads inside one world. Results do not depend on this.
    pub threads: usize,
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
    pub soil_drain: f32,
    pub soil_recovery: f32,

    // body
    pub max_energy: f32,
    pub start_energy: f32,
    pub base_cost: f32,
    pub move_cost: f32,
    pub rest_factor: f32,
    pub speed: f32,
    pub max_age: u32,
    pub vision: f32,
    /// At most this many neighbours are looked at per tick; crowds beyond it blur together.
    pub max_neighbours: u32,
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
    pub farm_boost: f32,
    pub cult_gain: f32,
    pub settle_ticks: u16,
    pub cult_decay: f32,

    // luck and disasters
    pub p_windfall: f32,
    pub p_accident: f32,
    pub p_drought: f32,
    pub p_golden: f32,
    pub p_plague: f32,
    pub drought_climate: f32,
    pub golden_climate: f32,
    pub plague_len: u16,
    pub p_infect: f32,
    pub sick_drain: f32,
    pub immune_len: u16,
    pub p_flood: f32,
    pub p_wildfire: f32,
    pub p_harsh_winter: f32,
    pub p_bounty: f32,

    // learning within a life
    pub skill_gain: f32,
    pub p_imitate: f32,
    pub imitate_rate: f32,

    // leadership
    pub leader_min_followers: u16,
    pub leader_min_prestige: f32,
    pub prestige_decay: f32,
    pub defiance_cost: f32,
    pub raid_bonus: f32,
    pub hold_bonus: f32,
    pub pool_bonus: f32,
    pub march_saving: f32,
    pub conserve_saving: f32,
    /// Control switch: leaders still form, but no order ever reaches anyone.
    pub no_orders: bool,
    pub defection_cost: f32,

    // the sea
    /// Summed "sea" effect an agent needs before it can move onto water.
    pub sea_threshold: f32,
    /// Energy cost multiplier while afloat.
    pub sea_cost: f32,
    /// Energy lost per tick when afloat without seafaring knowledge (forgotten boats).
    pub drown_drain: f32,
    /// Fishing while afloat, as a fraction of the land gather rate (does not deplete).
    pub fish_yield: f32,

    // making things
    /// Energy spent on one craft action.
    pub craft_cost: f32,
    /// Units of a material an agent can carry.
    pub mat_cap: u8,
    /// Chance per gather action of picking up a unit of each material lying on the cell, scaled by how much is there.
    pub p_pickup: f32,
    /// A made thing must reach this in its best use to count as a discovery; below it the attempt is junk.
    pub craft_min: f32,
    /// Chance that a working experiment is actually noticed and understood (a recipe, not a fluke).
    pub p_insight: f32,
    /// Life lost per tick of use, as a multiple of one.
    pub wear: f32,
    /// Share of a shelter's strength that shields the people at home in winter (energy) and in fights.
    pub shelter_warmth: f32,

    // minds
    /// Multiplier on within-life learning rates; 0 turns learning off (a control).
    pub learn_scale: f32,
    /// Learn by actor-critic with eligibility traces instead of the Hebbian rule. The Hebbian
    /// brain ties what it is doing to the reward arriving at that instant; this one keeps a
    /// fading record of recent choices, prices the present with a critic it grows itself, and
    /// learns from the gap between the two. It also has to act at random sometimes, or it would
    /// never find out what the choices it avoids are worth.
    pub grad_rule: bool,
    /// How far ahead a mind counts the future (0 = only this tick matters).
    pub gamma: f32,
    /// How long the trace of a choice lasts; with gamma it sets how far back a reward reaches.
    pub trace_lambda: f32,
    /// Step sizes for the two learners: the one that chooses, and the one that prices.
    /// Both are normalised by their own trace size, so they mean the same at any trace length.
    pub actor_rate: f32,
    pub critic_rate: f32,
    /// How wildly the gradient brain samples its actions. Low is near the old "take the best".
    pub policy_temp: f32,
    /// How much of a model's *learned* mind an imitator takes (0 = know-how never spreads).
    pub know_rate: f32,
    /// Multiplier on signal inputs; 0 makes everyone deaf (a control).
    pub hear_scale: f32,
    /// How much of strangers' calls gets through (1 = as loud as kin; 0 = only kin are heard, which the
    /// lab found to cost worlds dearly: hearing strangers triples knowledge and things per head).
    pub hear_strangers: f32,
    /// Energy per tick spent calling, per unit of signal magnitude: lying is not free.
    pub sig_cost: f32,
    /// Controls: customs never speak; nobody can craft.
    pub no_customs: bool,
    pub no_crafting: bool,

    // herds: prey that takes several people at once
    /// Herds per 10,000 cells of map. Off by default: the lab found herds a costly distraction that
    /// halves settlement without teaching anyone to hunt together (docs/lab/herds.md); 3.5 turns them on.
    pub herd_density: f32,
    /// Meat in a full herd, split among the hunters who struck the killing tick.
    pub herd_food: f32,
    /// Strength that must land in one tick to bring a full herd down (about three ordinary people).
    pub hunt_threshold: f32,
    pub hunt_range: f32,
    pub hunt_cost: f32,
    /// Distinct hunters that must have struck lately; one person can never do it alone.
    pub hunt_min_hands: usize,
    pub herd_regrow: f32,
    pub herd_respawn: u32,

    // customs
    pub custom_gain: f32,
    pub custom_decay: f32,
    pub custom_min: f32,

    // perception
    pub region_side: usize,
    pub region_refresh: u64,

    // storehouses
    pub store_min_followers: u16,
    pub store_range: f32,
    pub store_cap: f32,
    pub store_decay: f32,
    pub loot: f32,

    // viewer output
    pub snapshot_every: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            seed: 42,
            seeds: None,
            quiet: false,
            profile: false,
            threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
            width: 192,
            height: 192,
            agents: 1000,
            tribes: 20,
            max_agents: 0,
            min_pop: 0,
            ticks: 20_000,
            log_every: 500,
            image_every: 0,
            out_dir: "out".to_string(),
            wrap: true,

            max_food: 10.0,
            regrow: 0.08,
            season_len: 2000.0,
            soil_drain: 0.001,
            soil_recovery: 0.0002,

            max_energy: 100.0,
            start_energy: 60.0,
            base_cost: 0.15,
            move_cost: 0.12,
            rest_factor: 0.4,
            speed: 1.0,
            max_age: 3000,
            vision: 6.0,
            max_neighbours: 96,
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

            p_discover: 0.000001,
            p_learn: 0.01,
            learn_range: 2.5,
            farm_boost: 6.0,
            cult_gain: 0.006,
            settle_ticks: 5,
            cult_decay: 0.99,

            p_windfall: 0.0001,
            p_accident: 0.00005,
            p_drought: 0.15,
            p_golden: 0.15,
            p_plague: 0.12,
            drought_climate: 0.35,
            golden_climate: 1.5,
            plague_len: 300,
            p_infect: 0.02,
            sick_drain: 0.25,
            immune_len: 4000,
            p_flood: 0.08,
            p_wildfire: 0.08,
            p_harsh_winter: 0.08,
            p_bounty: 0.10,

            skill_gain: 0.002,
            p_imitate: 0.002,
            imitate_rate: 0.1,

            leader_min_followers: 5,
            leader_min_prestige: 5.0,
            prestige_decay: 0.999,
            defiance_cost: 0.002,
            raid_bonus: 12.0,
            hold_bonus: 1.5,
            pool_bonus: 1.5,
            march_saving: 0.3,
            conserve_saving: 0.5,
            no_orders: false,
            defection_cost: 0.03,
            sea_threshold: 0.25,
            sea_cost: 1.6,
            drown_drain: 3.0,
            fish_yield: 0.6,
            craft_cost: 1.5,
            mat_cap: 8,
            p_pickup: 0.35,
            craft_min: 0.12,
            p_insight: 0.02,
            wear: 1.0,
            shelter_warmth: 0.35,
            learn_scale: 1.0,
            grad_rule: false,
            gamma: 0.95,
            trace_lambda: 0.9,
            actor_rate: 0.1,
            critic_rate: 0.1,
            policy_temp: 0.1,
            know_rate: 0.0,
            hear_scale: 1.0,
            hear_strangers: 1.0,
            sig_cost: 0.02,
            no_customs: false,
            no_crafting: false,
            herd_density: 0.0,
            herd_food: 260.0,
            hunt_threshold: 150.0,
            hunt_range: 2.0,
            hunt_cost: 1.5,
            hunt_min_hands: 2,
            herd_regrow: 0.0006,
            herd_respawn: 1500,

            custom_gain: 0.01,
            custom_decay: 0.9995,
            custom_min: 0.2,

            region_side: 12,
            region_refresh: 50,

            store_min_followers: 10,
            store_range: 8.0,
            store_cap: 3000.0,
            store_decay: 0.9997,
            loot: 40.0,

            snapshot_every: 0,
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
            if key == "--quiet" {
                c.quiet = true;
                i += 1;
                continue;
            }
            if key == "--profile" {
                c.profile = true;
                i += 1;
                continue;
            }
            if key == "--no-orders" {
                c.no_orders = true;
                i += 1;
                continue;
            }
            if key == "--no-customs" {
                c.no_customs = true;
                i += 1;
                continue;
            }
            if key == "--no-crafting" {
                c.no_crafting = true;
                i += 1;
                continue;
            }
            if key == "--gradient" {
                c.grad_rule = true;
                i += 1;
                continue;
            }
            let val = args.get(i + 1).ok_or_else(|| format!("missing value for {key}"))?;
            macro_rules! set {
                ($field:ident) => {
                    c.$field = val.parse().map_err(|_| format!("bad value for {key}: {val}"))?
                };
            }
            match key {
                "--seed" => set!(seed),
                "--seeds" => {
                    let (a, b) = val.split_once('-').ok_or_else(|| format!("--seeds wants A-B, got {val}"))?;
                    let a: u64 = a.parse().map_err(|_| format!("bad seeds range {val}"))?;
                    let b: u64 = b.parse().map_err(|_| format!("bad seeds range {val}"))?;
                    if b < a {
                        return Err(format!("bad seeds range {val}"));
                    }
                    c.seeds = Some((a, b));
                }
                "--threads" => set!(threads),
                "--sea-threshold" => set!(sea_threshold),
                "--sea-cost" => set!(sea_cost),
                "--drown-drain" => set!(drown_drain),
                "--fish-yield" => set!(fish_yield),
                "--craft-cost" => set!(craft_cost),
                "--mat-cap" => set!(mat_cap),
                "--p-pickup" => set!(p_pickup),
                "--craft-min" => set!(craft_min),
                "--p-insight" => set!(p_insight),
                "--wear" => set!(wear),
                "--shelter-warmth" => set!(shelter_warmth),
                "--learn-scale" => set!(learn_scale),
                "--gamma" => set!(gamma),
                "--trace-lambda" => set!(trace_lambda),
                "--actor-rate" => set!(actor_rate),
                "--critic-rate" => set!(critic_rate),
                "--policy-temp" => set!(policy_temp),
                "--know-rate" => set!(know_rate),
                "--hear-scale" => set!(hear_scale),
                "--hear-strangers" => set!(hear_strangers),
                "--sig-cost" => set!(sig_cost),
                "--herd-density" => set!(herd_density),
                "--herd-food" => set!(herd_food),
                "--hunt-threshold" => set!(hunt_threshold),
                "--hunt-range" => set!(hunt_range),
                "--hunt-cost" => set!(hunt_cost),
                "--hunt-min-hands" => set!(hunt_min_hands),
                "--herd-regrow" => set!(herd_regrow),
                "--herd-respawn" => set!(herd_respawn),
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
                "--soil-drain" => set!(soil_drain),
                "--soil-recovery" => set!(soil_recovery),
                "--base-cost" => set!(base_cost),
                "--max-age" => set!(max_age),
                "--max-neighbours" => set!(max_neighbours),
                "--attack-damage" => set!(attack_damage),
                "--steal" => set!(steal),
                "--p-mut" => set!(p_mut),
                "--sigma" => set!(sigma),
                "--p-discover" => set!(p_discover),
                "--p-learn" => set!(p_learn),
                "--farm-boost" => set!(farm_boost),
                "--cult-gain" => set!(cult_gain),
                "--cult-decay" => set!(cult_decay),
                "--settle-ticks" => set!(settle_ticks),
                "--p-drought" => set!(p_drought),
                "--p-golden" => set!(p_golden),
                "--p-plague" => set!(p_plague),
                "--p-infect" => set!(p_infect),
                "--p-windfall" => set!(p_windfall),
                "--p-accident" => set!(p_accident),
                "--p-flood" => set!(p_flood),
                "--p-wildfire" => set!(p_wildfire),
                "--p-harsh-winter" => set!(p_harsh_winter),
                "--p-bounty" => set!(p_bounty),
                "--p-imitate" => set!(p_imitate),
                "--skill-gain" => set!(skill_gain),
                "--leader-min-followers" => set!(leader_min_followers),
                "--raid-bonus" => set!(raid_bonus),
                "--hold-bonus" => set!(hold_bonus),
                "--march-saving" => set!(march_saving),
                "--conserve-saving" => set!(conserve_saving),
                "--custom-gain" => set!(custom_gain),
                "--custom-decay" => set!(custom_decay),
                "--store-min-followers" => set!(store_min_followers),
                "--store-range" => set!(store_range),
                "--store-cap" => set!(store_cap),
                "--store-decay" => set!(store_decay),
                "--snapshot-every" => set!(snapshot_every),
                "--region-side" => set!(region_side),
                "--region-refresh" => set!(region_refresh),
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
  --seeds A-B       run every seed from A to B in parallel and print an outcome table
  --quiet           no per-window rows or live events (implied by --seeds)
  --threads N       worker threads inside one world (all cores); results never depend on it
  --profile         print the share of time spent in each phase at the end
  --agents N        initial agents (1000)
  --tribes N        founding tribes, agents spawn clustered per tribe (20)
  --ticks N         ticks to run (20000); a run ends early on extinction
  --width/--height  map size in cells (192)
  --max-agents N    hard population cap; 0 = none, the land is the only limit (0)
  --min-pop N       below this, random immigrants arrive; 0 = extinction is final (0)
  --log-every N     stats interval (500)
  --image-every N   write out/frame_XXXXXX.ppm every N ticks (0 = off)
  --out DIR         output directory (out)
  --wrap 0|1        toroidal map (1)
  --regrow F        food regrowth per tick (0.08)
  --max-food F      food cap per fully fertile cell (10)
  --season-len F    ticks per year (2000)
  --soil-drain F    fertility lost per unit harvested (0.001); --soil-recovery F regained per rested tick (0.0002)
  --base-cost F     energy burned per tick (0.15)
  --max-age N       ticks before dying of old age (3000)
  --max-neighbours N   neighbours an agent looks at per tick before the crowd blurs (96)
  --attack-damage F --steal F --p-mut F --sigma F
  --p-discover F    per agent-tick chance of inventing something while working (1e-6)
  --p-learn F       chance per tick of learning an innovation from an adjacent kin (0.01)
  --farm-boost F    regrowth multiplier of a fully tended cell (6); innovations raise it
  --cult-gain F     tending per still tick (0.006); --cult-decay F kept per untended tick (0.99)
  --settle-ticks N  ticks an agent must stay still before its tending takes effect (5)
  --p-drought F --p-golden F --p-harsh-winter F --p-plague F   yearly weather chances
  --p-flood F --p-wildfire F --p-bounty F                       yearly regional events
  --p-infect F      plague spread per contact-tick (0.02)
  --p-windfall F --p-accident F   per agent-tick personal luck
  --p-imitate F     per contact-tick chance of copying a richer kin's brain (0.002); 0 disables
  --gradient        learn by actor-critic with traces instead of the Hebbian rule
  --gamma F         how far ahead a gradient mind counts the future (0.95)
  --trace-lambda F  how long a choice stays creditable (0.9)
  --actor-rate F    step length of the chooser along its trace (0.1; above 0.3 worlds collapse)
  --critic-rate F   fraction of its own error the pricer corrects each tick (0.1)
  --policy-temp F   how wildly a gradient mind samples actions (0.1)
  --know-rate F     how much learned know-how an imitator takes (0; 0.3 makes culture)
  --skill-gain F    skill gained per practice (0.002)
  --leader-min-followers N   kin needed to count as a leader (5)
  --raid-bonus F --hold-bonus F --march-saving F --conserve-saving F
                    what obeying each kind of order is worth (12 / 1.5 / 0.3 / 0.5)
  --no-orders       control run: leaders still form but their orders reach no one
  --custom-gain F   how fast an obeyed order becomes a custom (0.01); --custom-decay F per tick (0.9995)
  --store-min-followers N   settled followers a named leader needs to raise a storehouse (10)
  --store-range F --store-cap F --store-decay F   reach (8), capacity (3000), spoilage kept per tick (0.9997)
  --snapshot-every N   write out/snap_seedN.bin every N ticks for the browser viewer (0 = off)
  --region-side N   world cells per side of a perception region (12)
  --region-refresh N   ticks between coarse-map refreshes (50)
";
