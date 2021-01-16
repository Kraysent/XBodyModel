use crate::particles::ParticleSet;

pub trait Generator
{
    fn generate(&self) -> Result<ParticleSet, &'static str>;
}   

pub mod plummer;