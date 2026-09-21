//! Binary snapshot stream for the browser viewer (viewer/index.html). Version 8.
//! Little endian throughout. The file is flushed after every frame so a viewer can
//! follow a run while it is still being computed.
//!
//! header: "AISV" u32 version(10) u16 width u16 height f32 max_food
//!         u32 len, then the sea as a bitmask (cell y*width+x is bit x%8 of byte (y*width+x)/8)
//! frame:  u32 frame_len (bytes that follow this field)
//!         u32 tick u8 era u8 keyframe u32 pop u32 stores
//!         f32 soil f32 climate f32 obedience f32 mean_known f32 season
//!         4 layers (food q0..31, cultivation q0..31, fertility q0..63, buildings: 0 none, else look 1..4 | storeys<<3 | span<<5), each: u32 len, then RLE pairs
//!           (value u8, run u8). A keyframe holds the layer itself; other frames hold layer XOR previous.
//!         pop agents of 28 bytes: u32 id u16 x*64 u16 y*64 u16 lineage u32 name u16 followers
//!           u8 flags u8 energy i8 mdx*100 i8 mdy*100 u8 under(0 none, 1..5 order) u8 action
//!           u8 gear (bit per slot held: tool, weapon, armour, boat, vessel, fire)
//!           u8 signal symbol (0..15: two dimensions in four bins each) | 16 if last reward was positive
//!           u8 plasticity (mean |plastic synapse| * 850, saturating) i8 reward*100 u8 heard symbol (0..15)
//!         stores of 14 bytes: u16 x*64 u16 y*64 f32 food u16 lineage u32 owner name id
//!         u32 herds, then herds of 6 bytes: u16 x*64 u16 y*64 u8 size*255 u8 hunters striking this tick
//! flags: 1 sick, 2 leader, 4 settled, 8 obeyed, 16 has custom, 32 afloat (in a boat)

use crate::agent::{Agent, N_SKILL};
use crate::craft;
use crate::herd::Herds;
use crate::innovation::Innovation;
use crate::store::Store;
use crate::world::World;
use std::io::{BufWriter, Write};

pub const KEYFRAME_EVERY: u32 = 16;

pub struct Snapshot {
    out: BufWriter<std::fs::File>,
    pub frames: u32,
    prev: [Vec<u8>; 4],
}

fn rle(out: &mut Vec<u8>, data: &[u8]) {
    let mut i = 0;
    while i < data.len() {
        let v = data[i];
        let mut run = 1;
        while i + run < data.len() && data[i + run] == v && run < 255 {
            run += 1;
        }
        out.push(v);
        out.push(run as u8);
        i += run;
    }
}

impl Snapshot {
    pub fn create(path: &str, world: &World) -> std::io::Result<Snapshot> {
        let mut out = BufWriter::new(std::fs::File::create(path)?);
        out.write_all(b"AISV")?;
        out.write_all(&10u32.to_le_bytes())?;
        out.write_all(&(world.width as u16).to_le_bytes())?;
        out.write_all(&(world.height as u16).to_le_bytes())?;
        out.write_all(&world.max_food.to_le_bytes())?;
        let n = world.width * world.height;
        let mut mask = vec![0u8; n.div_ceil(8)];
        for (i, w) in world.water.iter().enumerate() {
            if *w {
                mask[i / 8] |= 1 << (i % 8);
            }
        }
        out.write_all(&(mask.len() as u32).to_le_bytes())?;
        out.write_all(&mask)?;
        Ok(Snapshot { out, frames: 0, prev: [vec![0; n], vec![0; n], vec![0; n], vec![0; n]] })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn frame(
        &mut self, tick: u64, era: u8, world: &World, agents: &[Agent], stores: &[Store], herds: &Herds, soil: f32, obedience: f32,
        mean_known: f32, season: f32, settle_ticks: u16, custom_min: f32, registry: &[Innovation],
    ) -> std::io::Result<()> {
        let n = world.width * world.height;
        let key = self.frames % KEYFRAME_EVERY == 0;
        let mut body: Vec<u8> = Vec::with_capacity(n + agents.len() * 28 + 64);
        body.extend_from_slice(&(tick as u32).to_le_bytes());
        body.push(era);
        body.push(key as u8);
        body.extend_from_slice(&(agents.len() as u32).to_le_bytes());
        body.extend_from_slice(&(stores.len() as u32).to_le_bytes());
        for v in [soil, world.climate, obedience, mean_known, season] {
            body.extend_from_slice(&v.to_le_bytes());
        }
        // Quantised layers, delta-coded against the previous frame unless this is a keyframe.
        let food_scale = 31.0 / (world.max_food * 3.0);
        let mut cur = vec![0u8; n];
        for layer in 0..4 {
            for i in 0..n {
                cur[i] = match layer {
                    0 => (world.food[i] * food_scale).clamp(0.0, 31.0) as u8,
                    1 => (world.cultivation[i] * 31.0).clamp(0.0, 31.0) as u8,
                    2 => (world.fertility[i] * 63.0).clamp(0.0, 63.0) as u8,
                    // What is built here: three bits for what it is made of, two for how high it
                    // stands, two for how far it reaches. All three come from the recipe, so the
                    // skyline of a settlement is a picture of what its people worked out.
                    _ => match world.buildings[i] {
                        Some(b) => match registry.get(b.item as usize).and_then(|inn| inn.craft.as_ref()) {
                            Some(c) => {
                                let (span, storeys) = craft::shape_of(&c.props, c.n_parts as usize);
                                craft::look_of(&c.props) | (storeys << 3) | (span << 5)
                            }
                            None => 2 | (1 << 3),
                        },
                        None => 0,
                    },
                };
            }
            let mut packed = Vec::with_capacity(n / 8);
            if key {
                rle(&mut packed, &cur);
            } else {
                let prev = &self.prev[layer];
                let delta: Vec<u8> = cur.iter().zip(prev.iter()).map(|(c, p)| c ^ p).collect();
                rle(&mut packed, &delta);
            }
            body.extend_from_slice(&(packed.len() as u32).to_le_bytes());
            body.extend_from_slice(&packed);
            self.prev[layer].copy_from_slice(&cur);
        }
        for a in agents {
            body.extend_from_slice(&a.id.to_le_bytes());
            body.extend_from_slice(&((a.x * 64.0) as u16).to_le_bytes());
            body.extend_from_slice(&((a.y * 64.0) as u16).to_le_bytes());
            body.extend_from_slice(&(a.lineage as u16).to_le_bytes());
            body.extend_from_slice(&a.name.to_le_bytes());
            body.extend_from_slice(&a.followers.to_le_bytes());
            let mut flags = 0u8;
            if a.sick > 0 {
                flags |= 1;
            }
            if a.is_leader {
                flags |= 2;
            }
            if a.still >= settle_ticks {
                flags |= 4;
            }
            if a.obeyed {
                flags |= 8;
            }
            if a.custom.is_some() && a.custom_strength >= custom_min {
                flags |= 16;
            }
            if a.afloat {
                flags |= 32;
            }
            body.push(flags);
            body.push(a.energy.clamp(0.0, 255.0) as u8);
            body.push((a.mdx * 100.0).clamp(-127.0, 127.0) as i8 as u8);
            body.push((a.mdy * 100.0).clamp(-127.0, 127.0) as i8 as u8);
            body.push(a.under.map_or(0, |o| o as u8 + 1));
            body.push(a.last_action as u8);
            let mut gear = 0u8;
            for (k, g) in a.gear.iter().enumerate().take(6) {
                if g.is_some() {
                    gear |= 1 << k;
                }
            }
            body.push(gear);
            let bin = |v: f32| (((v + 1.0) * 2.0).floor() as u8).min(3);
            body.push(bin(a.signal[0]) * 4 + bin(a.signal[1]) + if a.reward > 0.05 { 16 } else { 0 });
            let plastic = a.plastic.iter().map(|p| p.abs()).sum::<f32>() / a.plastic.len().max(1) as f32;
            body.push((plastic * 850.0).clamp(0.0, 255.0) as u8);
            body.push((a.reward * 100.0).clamp(-127.0, 127.0) as i8 as u8);
            body.push(bin(a.heard[0]) * 4 + bin(a.heard[1]));
            // The craft a life has been spent on: which of the three skills stands highest, and
            // how high. Skills climb all through a life and nothing on screen ever showed it, so
            // a viewer watching people work saw no one getting better at anything.
            let mut best = 0usize;
            for k in 1..N_SKILL {
                if a.skill[k] > a.skill[best] {
                    best = k;
                }
            }
            let level = (a.skill[best] * 63.0).clamp(0.0, 63.0) as u8;
            body.push(if level < 13 { 0 } else { (best as u8 + 1) << 6 | level });
        }
        for s in stores {
            body.extend_from_slice(&((s.x * 64.0) as u16).to_le_bytes());
            body.extend_from_slice(&((s.y * 64.0) as u16).to_le_bytes());
            body.extend_from_slice(&s.food.to_le_bytes());
            body.extend_from_slice(&(s.lineage as u16).to_le_bytes());
            body.extend_from_slice(&s.owner.to_le_bytes());
        }
        let live: Vec<&crate::herd::Herd> = herds.list.iter().filter(|h| h.cooldown == 0).collect();
        body.extend_from_slice(&(live.len() as u32).to_le_bytes());
        for h in live {
            body.extend_from_slice(&((h.x * 64.0) as u16).to_le_bytes());
            body.extend_from_slice(&((h.y * 64.0) as u16).to_le_bytes());
            body.push((h.size * 255.0).clamp(0.0, 255.0) as u8);
            body.push(if h.struck { h.hunters.len().min(255) as u8 } else { 0 });
        }
        self.out.write_all(&(body.len() as u32).to_le_bytes())?;
        self.out.write_all(&body)?;
        self.out.flush()?;
        self.frames += 1;
        Ok(())
    }

    pub fn finish(mut self) -> std::io::Result<()> {
        self.out.flush()
    }
}

/// Names of every leader that ever held a name, plus the lineages that ever existed.
pub fn write_meta(
    path: &str, width: usize, height: usize, frames: u32, names: &[(u32, String)], eras: &[&str], seed: u64,
    ticks: u64, snapshot_every: u64,
) -> std::io::Result<()> {
    let mut f = BufWriter::new(std::fs::File::create(path)?);
    write!(f, "{{\"seed\":{seed},\"width\":{width},\"height\":{height},\"frames\":{frames},\"ticks\":{ticks},\"snapshot_every\":{snapshot_every},\"eras\":[")?;
    for (i, e) in eras.iter().enumerate() {
        write!(f, "{}\"{}\"", if i > 0 { "," } else { "" }, e)?;
    }
    write!(f, "],\"names\":{{")?;
    for (i, (id, name)) in names.iter().enumerate() {
        write!(f, "{}\"{}\":\"{}\"", if i > 0 { "," } else { "" }, id, name)?;
    }
    writeln!(f, "}}}}")?;
    f.flush()
}
