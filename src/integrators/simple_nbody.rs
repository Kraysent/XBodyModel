use crate::particles::{ParticleSet, Particle};
use crate::vector::Vector3;
use super::Integrator;

pub const G: f64 = 6.67e-11;

pub struct SimpleNBody<'a>
{
    pub particles: &'a mut ParticleSet,
}

impl<'a> SimpleNBody<'a>
{
    pub fn new(particles: &'a mut ParticleSet) -> Result<SimpleNBody, &'static str>
    {
        return Ok(SimpleNBody { particles });
    }

    fn get_force(p1: &Particle, p2: &Particle) -> Vector3
    {
        let dist = p2.position - p1.position;

        return dist.unit() * G * p1.mass * p2.mass / (dist.dot(&dist));
    }
}

impl<'a> Integrator for SimpleNBody<'a>
{
    fn integrate(&mut self, dt: f64) 
    { 
        let mut force: Vector3;
        let particle_set = &self.particles.particles;
        let mut dvs = vec![Vector3::null_vector(); particle_set.len()];
        let mut drs = vec![Vector3::null_vector(); particle_set.len()];

        for i in 0..particle_set.len()
        {
            force = Vector3::null_vector();

            for j in 0..particle_set.len()
            {
                if particle_set[i] == particle_set[j]
                {
                    continue;
                }

                force += SimpleNBody::get_force(&particle_set[i], &particle_set[j]);
            }

            dvs[i] = force / particle_set[i].mass * dt;
        }

        // update velocities
        let particle_set = &mut self.particles.particles;

        for i in 0..particle_set.len()
        {
            particle_set[i].velocity += dvs[i];
        }

        // update positions 
        let particle_set = &self.particles.particles;

        for i in 0..particle_set.len()
        {
            drs[i] = particle_set[i].velocity * dt;
        }

        let particle_set = &mut self.particles.particles;

        for i in 0..particle_set.len()
        {
            particle_set[i].position += drs[i];
        }
    }
}