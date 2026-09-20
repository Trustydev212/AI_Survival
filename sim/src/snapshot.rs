//! Binary snapshot stream for the browser viewer (viewer/index.html). Version 4.
//! Little endian throughout. The file is flushed after every frame so a viewer can
//! follow a run while it is still being computed.
//!
//! header: "AISV" u32 version(4) u16 width u16 height f32 max_food
//!         u32 len, then the sea as a bitmask (cell y*width+x is bit x%8 of byte (y*width+x)/8)
//! frame:  u32 frame_len (bytes that follow this field)
//!         u32 tick u8 era u8 keyframe u32 pop u32 stores
//!         f32 soil f32 climate f32 obedience f32 mean_known f32 season
//!         3 layers (food q0..31, cultivation q0..31, fertility q0..63), each: u32 len, then RLE pairs
//!           (value u8, run u8). A keyframe holds the layer itself; other frames hold layer XOR previous.
//!         pop agents of 22 bytes: u32 id u16 x*64 u16 y*64 u16 lineage u32 name u16 followers
//!           u8 flags u8 energy i8 mdx*100 i8 mdy*100 u8 under(0 none, 1..5 order) u8 action
//!         stores of 14 bytes: u16 x*64 u16 y*64 f32 food u16 lineage u32 owner name id
//! flags: 1 sick, 2 leader, 4 settled, 8 obeyed, 16 has custom

use crate::agent::Agent;
use crate::store::Store;
use crate::world::World;
use std::io::{BufWriter, Write};

pub const KEYFRAME_EVERY: u32 = 16;

pub struct Snapshot {
    out: BufWriter<std::fs::File>,
    pub frames: u32,
    prev: [Vec<u8>; 3],
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
        out.write_all(&4u32.to_le_bytes())?;
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
        Ok(Snapshot { out, frames: 0, prev: [vec![0; n], vec![0; n], vec![0; n]] })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn frame(
        &mut self, tick: u64, era: u8, world: &World, agents: &[Agent], stores: &[Store], soil: f32, obedience: f32,
        mean_known: f32, season: f32, settle_ticks: u16, custom_min: f32,
    ) -> std::io::Result<()> {
        let n = world.width * world.height;
        let key = self.frames % KEYFRAME_EVERY == 0;
        let mut body: Vec<u8> = Vec::with_capacity(n + agents.len() * 22 + 64);
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
        for layer in 0..3 {
            for i in 0..n {
                cur[i] = match layer {
                    0 => (world.food[i] * food_scale).clamp(0.0, 31.0) as u8,
                    1 => (world.cultivation[i] * 31.0).clamp(0.0, 31.0) as u8,
                    _ => (world.fertility[i] * 63.0).clamp(0.0, 63.0) as u8,
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
            body.push(flags);
            body.push(a.energy.clamp(0.0, 255.0) as u8);
            body.push((a.mdx * 100.0).clamp(-127.0, 127.0) as i8 as u8);
            body.push((a.mdy * 100.0).clamp(-127.0, 127.0) as i8 as u8);
            body.push(a.under.map_or(0, |o| o as u8 + 1));
            body.push(a.last_action as u8);
        }
        for s in stores {
            body.extend_from_slice(&((s.x * 64.0) as u16).to_le_bytes());
            body.extend_from_slice(&((s.y * 64.0) as u16).to_le_bytes());
            body.extend_from_slice(&s.food.to_le_bytes());
            body.extend_from_slice(&(s.lineage as u16).to_le_bytes());
            body.extend_from_slice(&s.owner.to_le_bytes());
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
