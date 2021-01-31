use std::cmp::PartialEq;
use crate::quantity::*;

pub struct Particle
{
    pub position: VectorQuantity,
    pub velocity: VectorQuantity,
    pub mass: ScalarQuantity
}

impl Particle
{
    pub fn new(position: VectorQuantity, velocity: VectorQuantity, mass: ScalarQuantity) 
        -> Result<Particle, &'static str>
    {
        let check_pos = |pos: &VectorQuantity| -> bool 
        {
            return pos.is_compatible(Units::m.convert());
        };
        let check_vel = |vel: &VectorQuantity| -> bool
        {
            return vel.is_compatible(Units::ms.convert());
        };
        let check_mass = |m: &ScalarQuantity| -> bool
        {
            if m.is_compatible(Units::kg.convert()) {
                return *m >= 0. * Units::kg;
            }

            return false;
        };
        
        if !check_pos(&position) {
            return Err("incorrect position");
        }
        if !check_vel(&velocity) {
            return Err("incorrect velocity");
        }
        if !check_mass(&mass) {
            return Err("incorrect mass");
        }

        return Ok(Particle {position, velocity, mass});
    }
}

impl PartialEq for Particle
{
    fn eq(&self, p: &Particle) -> bool 
    { 
        return (self.position == p.position) 
            && (self.velocity == p.velocity)
            && (self.mass == p.mass);
    }
}

pub struct ParticleSet
{
    pub particles: Vec<Particle>
}

impl ParticleSet
{
    pub fn new() -> Result<ParticleSet, &'static str>
    {
        return Ok(ParticleSet{ particles: Vec::new() });
    }

    pub fn add_particle(&mut self, p: Particle)
    {
        self.particles.push(p);
    }

    pub fn add_particles(&mut self, ps: ParticleSet)
    {
        for p in ps.particles
        {
            self.add_particle(p);
        }
    }
}
