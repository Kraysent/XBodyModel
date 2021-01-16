pub trait Integrator
{
    fn integrate(&mut self, dt: f64);
}

pub mod simple_nbody;