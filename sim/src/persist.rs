//! Saving a world and picking it up again.
//!
//! Every run used to end with the world thrown away. That put a ceiling on the whole project:
//! nothing could be watched for longer than one command, so every number in docs/THEORY.md was
//! measured over a window chosen by hand, and section 15 showed those windows decide the answer.
//! A world that can be put down and picked up again has no such window.
//!
//! The format is plain little-endian, written and read in the same order, with no padding and no
//! dependencies. It is not portable across versions: a saved world carries the rules version and
//! refuses to load into a different one, because the fields would line up and the meaning would
//! not.

use crate::agent::{Agent, Gear};
use crate::brain::{Action, Genome};
use crate::craft::{Craft, Ing, Process, Slot};
use crate::config::Config;
use crate::herd::{Herd, Herds};
use crate::innovation::{Innovation, Known, KNOWN_WORDS};
use crate::orders::Order;
use crate::rng::Rng;
use crate::sim::Sim;
use crate::store::{Store, Stores};
use crate::version;
use crate::world::{Building, World};
use std::io::{Read, Write};

const MAGIC: &[u8; 4] = b"AISW";

struct W<'a>(&'a mut dyn Write);
impl W<'_> {
    fn u8(&mut self, v: u8) { let _ = self.0.write_all(&[v]); }
    fn u16(&mut self, v: u16) { let _ = self.0.write_all(&v.to_le_bytes()); }
    fn u32(&mut self, v: u32) { let _ = self.0.write_all(&v.to_le_bytes()); }
    fn u64(&mut self, v: u64) { let _ = self.0.write_all(&v.to_le_bytes()); }
    fn f32(&mut self, v: f32) { let _ = self.0.write_all(&v.to_le_bytes()); }
    fn bl(&mut self, v: bool) { self.u8(v as u8); }
    fn fs(&mut self, v: &[f32]) { for x in v { self.f32(*x); } }
    fn bs(&mut self, v: &[u8]) { let _ = self.0.write_all(v); }
    fn vf(&mut self, v: &[f32]) { self.u32(v.len() as u32); self.fs(v); }
    fn s(&mut self, v: &str) { self.u32(v.len() as u32); self.bs(v.as_bytes()); }
    fn ord(&mut self, v: Option<Order>) { self.u8(v.map_or(0, |o| o as u8 + 1)); }
}

struct R<'a>(&'a [u8], usize);
impl R<'_> {
    fn take(&mut self, n: usize) -> &[u8] { let s = &self.0[self.1..self.1 + n]; self.1 += n; s }
    fn u8(&mut self) -> u8 { self.take(1)[0] }
    fn u16(&mut self) -> u16 { u16::from_le_bytes(self.take(2).try_into().unwrap()) }
    fn u32(&mut self) -> u32 { u32::from_le_bytes(self.take(4).try_into().unwrap()) }
    fn u64(&mut self) -> u64 { u64::from_le_bytes(self.take(8).try_into().unwrap()) }
    fn f32(&mut self) -> f32 { f32::from_le_bytes(self.take(4).try_into().unwrap()) }
    fn bl(&mut self) -> bool { self.u8() == 1 }
    fn fs(&mut self, v: &mut [f32]) { for x in v.iter_mut() { *x = self.f32(); } }
    fn vf(&mut self) -> Vec<f32> { let n = self.u32() as usize; (0..n).map(|_| self.f32()).collect() }
    fn s(&mut self) -> String { let n = self.u32() as usize; String::from_utf8_lossy(self.take(n)).into_owned() }
    fn ord(&mut self) -> Option<Order> { let v = self.u8(); if v == 0 { None } else { Some(Order::ALL[v as usize - 1]) } }
}

fn put_agent(w: &mut W, a: &Agent) {
    w.u32(a.id); w.f32(a.x); w.f32(a.y); w.f32(a.mdx); w.f32(a.mdy);
    w.f32(a.energy); w.f32(a.inventory); w.u32(a.age); w.u32(a.lineage);
    w.vf(&a.genome.weights); w.fs(&a.genome.marker); w.fs(&a.genome.temper); w.fs(&a.genome.learn);
    w.u8(a.attacked_timer); w.u8(a.last_action as u8); w.u16(a.children);
    w.fs(&a.profile);
    for word in a.known.0 { w.u64(word); }
    w.fs(&a.caps); w.bs(&a.mats);
    for g in a.gear.iter() { w.u16(g.item); w.f32(g.life); }
    w.f32(a.sheltered); w.bl(a.can_make); w.u16(a.still);
    w.fs(&a.emotion); w.fs(&a.memory);
    w.vf(&a.plastic); w.vf(&a.critic); w.vf(&a.trace); w.vf(&a.vtrace); w.f32(a.v_prev); w.f32(a.td);
    w.fs(&a.last_hidden); w.fs(&a.last_out); w.fs(&a.signal); w.fs(&a.heard);
    w.f32(a.reward); w.f32(a.prev_wealth); w.f32(a.prev_mood);
    w.bl(a.has_home); w.f32(a.home_x); w.f32(a.home_y);
    w.u16(a.sick); w.bl(a.afloat); w.u16(a.immune); w.fs(&a.skill);
    w.f32(a.prestige); w.u32(a.leader); w.u16(a.followers); w.bl(a.is_leader); w.u16(a.tenure); w.u32(a.name);
    w.u8(a.order as u8); w.f32(a.order_dx); w.f32(a.order_dy);
    w.ord(a.under); w.bl(a.obeyed); w.ord(a.custom);
    w.f32(a.custom_dx); w.f32(a.custom_dy); w.f32(a.custom_strength); w.f32(a.charisma);
    w.fs(&a.emo_decay); w.fs(&a.emo_sens);
}

fn get_agent(r: &mut R, proto: &Agent) -> Agent {
    let mut a = proto.clone();
    a.id = r.u32(); a.x = r.f32(); a.y = r.f32(); a.mdx = r.f32(); a.mdy = r.f32();
    a.energy = r.f32(); a.inventory = r.f32(); a.age = r.u32(); a.lineage = r.u32();
    a.genome = Genome { weights: r.vf(), marker: [0.0; 3], temper: [0.0; crate::brain::N_TEMPER], learn: [0.0; crate::brain::N_LEARN] };
    r.fs(&mut a.genome.marker); r.fs(&mut a.genome.temper); r.fs(&mut a.genome.learn);
    a.attacked_timer = r.u8(); a.last_action = Action::ALL[r.u8() as usize]; a.children = r.u16();
    r.fs(&mut a.profile);
    let mut words = [0u64; KNOWN_WORDS];
    for word in words.iter_mut() { *word = r.u64(); }
    a.known = Known(words);
    r.fs(&mut a.caps);
    for m in a.mats.iter_mut() { *m = r.u8(); }
    for g in a.gear.iter_mut() { *g = Gear { item: r.u16(), life: r.f32() }; }
    a.sheltered = r.f32(); a.can_make = r.bl(); a.still = r.u16();
    r.fs(&mut a.emotion); r.fs(&mut a.memory);
    a.plastic = r.vf(); a.critic = r.vf(); a.trace = r.vf(); a.vtrace = r.vf(); a.v_prev = r.f32(); a.td = r.f32();
    r.fs(&mut a.last_hidden); r.fs(&mut a.last_out); r.fs(&mut a.signal); r.fs(&mut a.heard);
    a.reward = r.f32(); a.prev_wealth = r.f32(); a.prev_mood = r.f32();
    a.has_home = r.bl(); a.home_x = r.f32(); a.home_y = r.f32();
    a.sick = r.u16(); a.afloat = r.bl(); a.immune = r.u16(); r.fs(&mut a.skill);
    a.prestige = r.f32(); a.leader = r.u32(); a.followers = r.u16(); a.is_leader = r.bl(); a.tenure = r.u16(); a.name = r.u32();
    a.order = Order::ALL[r.u8() as usize]; a.order_dx = r.f32(); a.order_dy = r.f32();
    a.under = r.ord(); a.obeyed = r.bl(); a.custom = r.ord();
    a.custom_dx = r.f32(); a.custom_dy = r.f32(); a.custom_strength = r.f32(); a.charisma = r.f32();
    r.fs(&mut a.emo_decay); r.fs(&mut a.emo_sens);
    a
}

/// Write the whole world to a file. Everything not written here is rebuilt on load, which is
/// why the spatial index, the region grid and the per-tick scratch lists are absent.
pub fn save(sim: &Sim, path: &str) -> std::io::Result<()> {
    let f = std::fs::File::create(path)?;
    let mut buf = std::io::BufWriter::new(f);
    let w = &mut W(&mut buf);
    w.bs(MAGIC);
    w.u32(version::WORLD);
    w.u64(sim.tick);
    w.u64(sim.rng.state());
    w.u64(sim.climate_until);
    w.u32(sim.hunts_total);
    w.u32(sim.next_id);
    // world
    w.u16(sim.world.width as u16); w.u16(sim.world.height as u16);
    w.f32(sim.world.climate); w.f32(sim.world.max_food);
    w.vf(&sim.world.food); w.vf(&sim.world.cultivation); w.vf(&sim.world.fertility); w.vf(&sim.world.base_fertility);
    for b in &sim.world.water { w.bl(*b); }
    for cell in &sim.world.mats { w.fs(cell); }
    for cell in &sim.world.mats_cap { w.fs(cell); }
    w.u32(sim.world.buildings.len() as u32);
    for b in &sim.world.buildings {
        match b {
            None => w.u8(0),
            Some(b) => { w.u8(1); w.u16(b.item); w.f32(b.life); w.f32(b.shelter); w.bl(b.flammable); }
        }
    }
    // innovations
    w.u32(sim.innovations.len() as u32);
    for i in &sim.innovations {
        w.s(&i.name); w.u8(i.tier); w.fs(&i.effects); w.u64(i.born_tick); w.u32(i.lineage);
        match &i.craft {
            None => w.u8(0),
            Some(c) => {
                w.u8(1); w.u8(c.process as u8);
                for p in c.parts { w.u16(p); }
                w.u8(c.n_parts); w.fs(&c.props); w.u8(c.slot as u8); w.f32(c.life);
                for m in c.cost { w.u8(m); }
                w.u8(c.depth);
            }
        }
    }
    // agents
    w.u32(sim.agents.len() as u32);
    for a in &sim.agents { put_agent(w, a); }
    // stores, herds, hall of fame
    w.u32(sim.stores.list.len() as u32);
    for s in &sim.stores.list { w.f32(s.x); w.f32(s.y); w.f32(s.food); w.u32(s.owner); w.u32(s.lineage); w.fs(&s.marker); w.u32(s.idle); }
    w.u32(sim.herds.list.len() as u32);
    for h in &sim.herds.list { w.f32(h.x); w.f32(h.y); w.f32(h.size); w.f32(h.dx); w.f32(h.dy); w.u32(h.cooldown); }
    w.u32(sim.hall.len() as u32);
    for (k, v) in &sim.hall { w.u32(*k); w.u16(v.0); w.u32(v.1); w.u64(v.2); }
    // Which recipe sits in which registry slot, and which slots retirement freed. Without these
    // a world picked up again treats things it already knows as brand new.
    w.u32(sim.recipes.len() as u32);
    for ((proc, parts), slot) in &sim.recipes { w.u8(*proc); for p in parts { w.u16(*p); } w.u16(*slot); }
    w.u32(sim.free_slots.len() as u32);
    for s in &sim.free_slots { w.u16(*s); }
    w.u32(sim.defected.len() as u32);
    for ((a, b), c) in &sim.defected { w.u32(*a); w.u32(*b); w.u32(*c); }
    Ok(())
}

/// Read a world back. The config given is used for the rules; the file supplies the state.
pub fn load(cfg: &Config, path: &str, events: crate::events::EventLog) -> std::io::Result<Sim> {
    let mut bytes = Vec::new();
    std::fs::File::open(path)?.read_to_end(&mut bytes)?;
    let r = &mut R(&bytes, 0);
    if r.take(4) != MAGIC {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "not a saved world"));
    }
    let v = r.u32();
    if v != version::WORLD {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("saved on world v{v}, this build is v{}: the rules differ, so the state would not mean the same thing", version::WORLD),
        ));
    }
    let mut sim = Sim::new(cfg.clone(), events);
    sim.tick = r.u64();
    sim.rng = Rng::from_state(r.u64());
    sim.climate_until = r.u64();
    sim.hunts_total = r.u32();
    sim.next_id = r.u32();
    let (w_, h_) = (r.u16() as usize, r.u16() as usize);
    if w_ != sim.world.width || h_ != sim.world.height {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "saved map is a different size"));
    }
    sim.world.climate = r.f32(); sim.world.max_food = r.f32();
    sim.world.food = r.vf(); sim.world.cultivation = r.vf(); sim.world.fertility = r.vf(); sim.world.base_fertility = r.vf();
    for i in 0..w_ * h_ { sim.world.water[i] = r.bl(); }
    for i in 0..sim.world.mats.len() { let mut c = [0.0; crate::craft::N_MAT]; r.fs(&mut c); sim.world.mats[i] = c; }
    for i in 0..sim.world.mats_cap.len() { let mut c = [0.0; crate::craft::N_MAT]; r.fs(&mut c); sim.world.mats_cap[i] = c; }
    let nb = r.u32() as usize;
    sim.world.buildings = (0..nb).map(|_| if r.u8() == 0 { None } else {
        Some(Building { item: r.u16(), life: r.f32(), shelter: r.f32(), flammable: r.bl() })
    }).collect();
    let ni = r.u32() as usize;
    sim.innovations = (0..ni).map(|_| {
        let name = r.s(); let tier = r.u8();
        let mut effects = [0.0; crate::innovation::N_EFFECT];
        r.fs(&mut effects);
        let born_tick = r.u64(); let lineage = r.u32();
        let craft = if r.u8() == 0 { None } else {
            let process = Process::ALL[r.u8() as usize];
            let mut parts = [0 as Ing; 3];
            for p in parts.iter_mut() { *p = r.u16(); }
            let n_parts = r.u8();
            let mut props = [0.0; crate::craft::N_PROP];
            r.fs(&mut props);
            let slot = Slot::ALL[r.u8() as usize];
            let life = r.f32();
            let mut cost = [0u8; crate::craft::N_MAT];
            for m in cost.iter_mut() { *m = r.u8(); }
            Some(Craft { process, parts, n_parts, props, slot, life, cost, depth: r.u8() })
        };
        Innovation { name, tier, effects, born_tick, lineage, craft }
    }).collect();
    let na = r.u32() as usize;
    let proto = sim.agents[0].clone();
    sim.agents = (0..na).map(|_| get_agent(r, &proto)).collect();
    let ns = r.u32() as usize;
    sim.stores = Stores::default();
    for _ in 0..ns {
        let (x, y, food, owner, lineage) = (r.f32(), r.f32(), r.f32(), r.u32(), r.u32());
        let mut marker = [0.0; 3];
        r.fs(&mut marker);
        sim.stores.list.push(Store { x, y, food, owner, lineage, marker, idle: r.u32() });
    }
    let nh = r.u32() as usize;
    sim.herds = Herds { list: (0..nh).map(|_| Herd {
        x: r.f32(), y: r.f32(), size: r.f32(), dx: r.f32(), dy: r.f32(), cooldown: r.u32(),
        hits: 0.0, hunters: Vec::new(), struck: false,
    }).collect() };
    let nhall = r.u32() as usize;
    sim.hall.clear();
    for _ in 0..nhall { let k = r.u32(); sim.hall.insert(k, (r.u16(), r.u32(), r.u64())); }
    let nrec = r.u32() as usize;
    sim.recipes.clear();
    for _ in 0..nrec {
        let proc = r.u8();
        let mut parts = [0 as Ing; 3];
        for p in parts.iter_mut() { *p = r.u16(); }
        sim.recipes.insert((proc, parts), r.u16());
    }
    let nfree = r.u32() as usize;
    sim.free_slots = (0..nfree).map(|_| r.u16()).collect();
    let ndef = r.u32() as usize;
    sim.defected.clear();
    for _ in 0..ndef { let a = r.u32(); let b = r.u32(); sim.defected.insert((a, b), r.u32()); }
    sim.rebuild_after_load();
    Ok(sim)
}

/// Everything a saved world does not carry, rebuilt from what it does.
impl Sim {
    pub fn rebuild_after_load(&mut self) {
        let reg = std::mem::take(&mut self.innovations);
        for a in self.agents.iter_mut() {
            crate::sim::refresh_caps(a, &reg);
        }
        self.innovations = reg;
        self.refresh_indices();
        // The region view is rebuilt on a schedule, not every tick, so a world picked up between
        // refreshes would read a blank map of where the good land is. Refresh it now.
        self.regions.refresh(&self.world, &self.agents, self.cfg.wrap);
    }
}
