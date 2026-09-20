//! Dumps the world as a binary PPM image: food as green, agents as their marker colour.
//! No dependencies; convert with ImageMagick or any viewer, or feed to a future engine.

use crate::agent::Agent;
use crate::world::World;
use std::io::{BufWriter, Write};

pub fn write_ppm(path: &str, world: &World, agents: &[Agent], scale: usize) -> std::io::Result<()> {
    let w = world.width * scale;
    let h = world.height * scale;
    let mut px = vec![0u8; w * h * 3];
    for y in 0..world.height {
        for x in 0..world.width {
            let i = y * world.width + x;
            let fert = world.fertility[i];
            let food = world.food[i] / world.max_food;
            let cult = world.cultivation[i];
            let r = (18.0 + 30.0 * fert + 160.0 * cult) as u8;
            let g = (22.0 + 40.0 * fert + 150.0 * food.min(1.0)) as u8;
            let b = (28.0 + 10.0 * fert) as u8;
            fill(&mut px, w, x * scale, y * scale, scale, [r, g, b]);
        }
    }
    for a in agents {
        let m = a.genome.marker;
        let c = [(70.0 + 185.0 * m[0]) as u8, (70.0 + 185.0 * m[1]) as u8, (70.0 + 185.0 * m[2]) as u8];
        let x = (a.x as usize).min(world.width - 1) * scale;
        let y = (a.y as usize).min(world.height - 1) * scale;
        fill(&mut px, w, x, y, scale, c);
    }
    let mut out = BufWriter::new(std::fs::File::create(path)?);
    write!(out, "P6\n{} {}\n255\n", w, h)?;
    out.write_all(&px)?;
    Ok(())
}

#[inline]
fn fill(px: &mut [u8], w: usize, x0: usize, y0: usize, s: usize, c: [u8; 3]) {
    for dy in 0..s {
        for dx in 0..s {
            let o = ((y0 + dy) * w + x0 + dx) * 3;
            px[o..o + 3].copy_from_slice(&c);
        }
    }
}
