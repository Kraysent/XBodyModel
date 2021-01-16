use super::Generator;
use crate::particles::*;
use crate::vector::Vector3;
use rand::prelude::*;
use rand::distributions::Standard;
use std::f64::consts::PI;

pub struct Plummer
{
    plummer_radius: f64,
    n: u32,
    m0: f64,
}

impl Plummer
{
    pub fn new(plummer_radius: f64, n: u32, m0: Option<f64>) -> Result<Plummer, &'static str>
    {
        if plummer_radius <= 0.0
        {
            return Err("radius is non-positive");
        }
        else if n == 0
        {
            return Err("no particles given");
        }
        
        return Ok(Plummer { plummer_radius, n, m0: m0.unwrap_or(1.0) });
    }

    fn rho(&self, r: f64) -> f64
    {
        return (3.0 * self.m0) / (4.0 * PI * self.m0.powi(3)) * (1.0 + (r / self.plummer_radius).powi(2)).powf(-5.0 / 2.0);
    }

    fn spherical_to_cartesian(r: f64, phi: f64, theta: f64) -> Vector3
    {
        let x = r * theta.sin() * phi.cos();
        let y = r * theta.sin() * phi.sin();
        let z = r * theta.cos();

        return Vector3{ x, y, z };
    }
}

impl Generator for Plummer
{
    fn generate(&self) -> Result<ParticleSet, &'static str> 
    {
        let number_of_divisions: usize = (self.n as f32).powf(1.0 / 3.0) as usize;
        let mut h = vec![0.0_f64; number_of_divisions];
        let max_val = 10.0 * self.plummer_radius;
        let division = 1.0 / (number_of_divisions as f64) * max_val;

        for i in 0..number_of_divisions
        {
            h[i] = self.rho(division * (i as f64));
        }

        let mut p = vec![0.0_f64; number_of_divisions + 1];
        let sum: f64 = h.iter().sum();
        
        for i in 0..number_of_divisions
        {
            p[i] = h[i] / sum;
        }

        p[number_of_divisions] = 1.0;

        let mut generator = StdRng::from_entropy();
        let mut number_of_particles_in_cell = vec![0; number_of_divisions];

        for _ in 0..self.n
        {
            let num: f64 = generator.sample(Standard);

            for j in 0..number_of_divisions
            {
                if p[j] < num && p[j + 1] > num
                {
                    number_of_particles_in_cell[j] += 1;
                }
            }
        }

        let mut output = ParticleSet::new()?;

        for i in 0..number_of_divisions
        {
            let curr = number_of_particles_in_cell[i];

            if curr == 0
            {
                continue;
            }

            for j in 1..curr + 1
            {
                let phi: f64 = generator.sample(Standard);
                let phi = phi * 2.0 * PI;
                let theta: f64 = generator.sample(Standard);
                let theta = theta * PI;
                let curr_r = division * (i as f64) + division * (j as f64) / (curr as f64);

                output.particles.push(
                    Particle::new(
                        Self::spherical_to_cartesian(curr_r, phi, theta), 
                        Vector3::null_vector(), 
                        self.m0 / (self.n as f64)
                    )?
                );
            }
        }

        return Ok(output);
    }
}