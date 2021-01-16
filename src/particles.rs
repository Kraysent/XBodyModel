use std::cmp::PartialEq;
use crate::vector::Vector3;

pub struct Particle
{
    pub position: Vector3,
    pub velocity: Vector3,
    pub mass: f64
}

impl Particle
{
    pub fn new(position: Vector3, velocity: Vector3, mass: f64) -> Result<Particle, &'static str>
    {
        if mass <= 0.0
        {
            return Err("particle mass is non-positive");
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
