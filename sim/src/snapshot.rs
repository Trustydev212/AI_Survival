//! Binary snapshot stream for the browser viewer (viewer/index.html).
//! One file per run: a header, then one frame every N ticks. Little endian throughout.
//!
//! header: "AISV" u32 version u16 width u16 height f32 max_food
//! frame:  u32 tick u8 era u32 pop u32 stores f32 soil f32 climate f32 obedience f32 mean_known f32 season
//!         then width*height u8 food, width*height u8 cultivation, width*height u8 fertility,
//!         then pop agents of 17 bytes: u16 x*64 u16 y*64 u8 r g b u8 flags u16 lineage u32 name u16 followers u8 energy
//!         then stores of 16 bytes: u16 x*64 u16 y*64 f32 food u8 r g b u8 pad u32 owner name id
//! flags: 1 sick, 2 leader, 4 settled, 8 obeyed, 16 has custom

use crate::agent::Agent;
use crate::store::Store;
use crate::world::World;
use std::io::{BufWriter, Write};

pub struct Snapshot {
    out: BufWriter<std::fs::File>,
    pub frames: u32,
}

impl Snapshot {
    pub fn create(path: &str, world: &World) -> std::io::Result<Snapshot> {
        let mut out = BufWriter::new(std::fs::File::create(path)?);
        out.write_all(b"AISV")?;
        out.write_all(&2u32.to_le_bytes())?;
        out.write_all(&(world.width as u16).to_le_bytes())?;
        out.write_all(&(world.height as u16).to_le_bytes())?;
        out.write_all(&world.max_food.to_le_bytes())?;
        Ok(Snapshot { out, frames: 0 })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn frame(
        &mut self, tick: u64, era: u8, world: &World, agents: &[Agent], stores: &[Store], soil: f32, obedience: f32,
        mean_known: f32, season: f32, settle_ticks: u16, custom_min: f32,
    ) -> std::io::Result<()> {
        let o = &mut self.out;
        o.write_all(&(tick as u32).to_le_bytes())?;
        o.write_all(&[era])?;
        o.write_all(&(agents.len() as u32).to_le_bytes())?;
        o.write_all(&(stores.len() as u32).to_le_bytes())?;
        for v in [soil, world.climate, obedience, mean_known, season] {
            o.write_all(&v.to_le_bytes())?;
        }
        let n = world.width * world.height;
        let mut buf = Vec::with_capacity(n * 3);
        let scale = 255.0 / (world.max_food * 3.0);
        for i in 0..n {
            buf.push((world.food[i] * scale).clamp(0.0, 255.0) as u8);
        }
        for i in 0..n {
            buf.push((world.cultivation[i] * 255.0).clamp(0.0, 255.0) as u8);
        }
        for i in 0..n {
            buf.push((world.fertility[i] * 255.0).clamp(0.0, 255.0) as u8);
        }
        o.write_all(&buf)?;
        let mut ab = Vec::with_capacity(agents.len() * 17);
        for a in agents {
            ab.extend_from_slice(&((a.x * 64.0) as u16).to_le_bytes());
            ab.extend_from_slice(&((a.y * 64.0) as u16).to_le_bytes());
            let m = a.genome.marker;
            ab.push((70.0 + 185.0 * m[0]) as u8);
            ab.push((70.0 + 185.0 * m[1]) as u8);
            ab.push((70.0 + 185.0 * m[2]) as u8);
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
            ab.push(flags);
            ab.extend_from_slice(&(a.lineage as u16).to_le_bytes());
            ab.extend_from_slice(&a.name.to_le_bytes());
            ab.extend_from_slice(&a.followers.to_le_bytes());
            ab.push(a.energy.clamp(0.0, 255.0) as u8);
        }
        o.write_all(&ab)?;
        let mut sb = Vec::with_capacity(stores.len() * 16);
        for s in stores {
            sb.extend_from_slice(&((s.x * 64.0) as u16).to_le_bytes());
            sb.extend_from_slice(&((s.y * 64.0) as u16).to_le_bytes());
            sb.extend_from_slice(&s.food.to_le_bytes());
            sb.push((70.0 + 185.0 * s.marker[0]) as u8);
            sb.push((70.0 + 185.0 * s.marker[1]) as u8);
            sb.push((70.0 + 185.0 * s.marker[2]) as u8);
            sb.push((s.lineage & 0xFF) as u8);
            sb.extend_from_slice(&s.owner.to_le_bytes());
        }
        o.write_all(&sb)?;
        self.frames += 1;
        Ok(())
    }

    pub fn finish(mut self) -> std::io::Result<()> {
        self.out.flush()
    }
}

/// Names of every leader that ever held a name, for the viewer's labels.
pub fn write_meta(path: &str, width: usize, height: usize, frames: u32, names: &[(u32, String)], eras: &[&str], seed: u64) -> std::io::Result<()> {
    let mut f = BufWriter::new(std::fs::File::create(path)?);
    write!(f, "{{\"seed\":{seed},\"width\":{width},\"height\":{height},\"frames\":{frames},\"eras\":[")?;
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
