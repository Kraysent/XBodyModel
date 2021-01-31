use std::cmp::PartialEq;
use crate::quantity::*;
use crate::vector::Vector3;

pub struct Particle
{
    position: VectorQuantity,
    velocity: VectorQuantity,
    mass: ScalarQuantity
}

impl Particle
{
    pub fn new(position: VectorQuantity, velocity: VectorQuantity, mass: ScalarQuantity) 
        -> Result<Particle, &'static str>
    {
        let mut res = Particle::empty();
        
        res.set_position(position)?;
        res.set_velocity(velocity)?;
        res.set_mass(mass)?;
        
        return Ok(res);
    }

    pub fn empty() -> Particle
    {
        return Particle
        {
            position: Vector3::null_vector() * Units::m,
            velocity: Vector3::null_vector() * Units::ms,
            mass: 1. * Units::kg
        }
    }

    pub fn set_position(&mut self, position: VectorQuantity) -> Result<(), &'static str>
    {
        let check_pos = |pos: &VectorQuantity| -> bool 
        {
            return pos.is_compatible(Units::m.convert());
        };

        if !check_pos(&position) {
            return Err("incorrect position");
        }
        
        self.position = position;

        return Ok(());
    }

    pub fn set_velocity(&mut self, velocity: VectorQuantity) -> Result<(), &'static str>
    {
        let check_vel = |vel: &VectorQuantity| -> bool
        {
            return vel.is_compatible(Units::ms.convert());
        };

        if !check_vel(&velocity) {
            return Err("incorrect velocity");
        }

        self.velocity = velocity;

        return Ok(());
    }

    pub fn set_mass(&mut self, mass: ScalarQuantity) -> Result<(), &'static str>
    {
        let check_mass = |m: &ScalarQuantity| -> bool
        {
            if m.is_compatible(Units::kg.convert()) {
                return *m >= 0. * Units::kg;
            }

            return false;
        };

        if !check_mass(&mass) {
            return Err("incorrect mass");
        }

        self.mass = mass;

        return Ok(());
    }

    pub fn get_position(&self) -> &VectorQuantity
    {
        return &self.position;
    }

    pub fn get_velocity(&self) -> &VectorQuantity
    {
        return &self.velocity;
    }

    pub fn get_mass(&self) -> &ScalarQuantity
    {
        return &self.mass;
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
