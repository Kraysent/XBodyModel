use crate::particles::ParticleSet;
use crate::quantity::ScalarQuantity;

pub trait Integrator
{
    fn integrate(&mut self, dt: ScalarQuantity);
    fn get_state(&self) -> Result<ParticleSet, &'static str>;
}

pub mod simple_nbody;