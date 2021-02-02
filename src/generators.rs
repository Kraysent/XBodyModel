use crate::particles::ParticleSet;

/// Trait that represents type of structures that are able to create sets of particles.
pub trait Generator {
    /// Actually generates the set of particles.
    fn generate(&self) -> Result<ParticleSet, &'static str>;
}

/// Creates particles according to [Plummer model](https://en.wikipedia.org/wiki/Plummer_model)
pub mod plummer;
