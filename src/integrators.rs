use crate::particles::ParticleSet;
use crate::quantity::ScalarQuantity;

/// Trait that is used to handle structures that are responsible for numerical integration of `ParticleSet`
pub trait Integrator {
    /// This method returns state of the system after the last `evolve` call (or just initialisation of integrator).
    /// 
    /// `return`: ParticleSet with particles representing current state or the error message 
    fn get_state(&self) -> Result<ParticleSet, &'static str>;
    /// Integrates the whole system up to `time`.
    /// 
    /// ## Arguments
    /// 
    /// * `time`: a `ScalarQuantity` equivalent to seconds  
    fn evolve(&mut self, time: &ScalarQuantity) -> Result<(), String>;
}

/// This module contains the simplest implementation of `Integrator` trait.
pub mod simple_nbody;
