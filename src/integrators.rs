use crate::particles::ParticleSet;
use crate::quantity::ScalarQuantity;

/// Trait that is used to handle structures that are responsible for numerical integration of `ParticleSet`
pub trait Integrator
{
    /// This method integrates the whole system on time `dt`.
    /// When you implement `integrate` function you **must** ensure that `dt` is in time units.
    fn integrate(&mut self, dt: &ScalarQuantity);
    /// This method returns state of the system after the last `integrate` call (or just initialisation of integrator).
    fn get_state(&self) -> Result<ParticleSet, &'static str>;
}

/// This module contains the simplest implementation of `Integrator` trait.
pub mod simple_nbody;