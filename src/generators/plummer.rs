use super::Generator;
use crate::particles::*;
use crate::vector::Vector3;

pub struct Plummer
{
    plummer_radius: f64
}

impl Plummer
{
    pub fn new(plummer_radius: f64) -> Result<Plummer, &'static str>
    {
        if plummer_radius <= 0.0
        {
            return Err("radius is non-positive");
        }
        
        return Ok(Plummer { plummer_radius });
    }
}

impl Generator for Plummer
{
    fn generate(&self) -> Result<ParticleSet, &'static str> 
    {
        let mut ps = ParticleSet::new()?;

        ps.add_particle(
            Particle::new(Vector3{ x: self.plummer_radius, y: 0.0, z: 0.0 }, Vector3{ x: 0.0, y: 0.0, z: 0.0 }, 5.0)?
        );
        ps.add_particle(
            Particle::new(Vector3{ x: 0.0, y: self.plummer_radius, z: 0.0 }, Vector3{ x: 0.0, y: 0.0, z: 0.0 }, 5.0)?
        );
        ps.add_particle(
            Particle::new(Vector3{ x: 0.0, y: 0.0, z: 0.0 }, Vector3{ x: 0.0, y: 0.0, z: 0.0 }, 5.0)?
        );

        return Ok(ps);
    }
}