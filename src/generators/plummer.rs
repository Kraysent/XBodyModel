use super::Generator;
use crate::particles::*;
use crate::vector::Vector3;
use rand::prelude::*;
use rand::distributions::{ Standard };
use rand_distr::{ Normal, Distribution };
use std::f64::consts::PI;

pub const G: f64 = 6.67e-11;

pub struct Plummer
{
    plummer_radius: f64,
    n: usize,
    m0: f64,
}

impl Plummer
{
    pub fn new(plummer_radius: f64, n: usize, m0: Option<f64>) -> Result<Plummer, &'static str>
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

    fn generate_positions(&self) -> Vec::<Vector3>
    {
        let number_of_divisions: usize = (self.n as f32).powf(1.0 / 3.0) as usize;
        let max_r = 10.0 * self.plummer_radius;
        let division = 1.0 / (number_of_divisions as f64) * max_r;
        let mut descrete_rhos = vec![0.0_f64; number_of_divisions];

        for i in 0..number_of_divisions
        {
            descrete_rhos[i] = self.rho(division * (i as f64));
        }

        let sum: f64 = descrete_rhos.iter().sum();
        let mut probabilities = vec![0.0_f64; number_of_divisions + 1];

        for i in 0..number_of_divisions
        {
            probabilities[i] = descrete_rhos[i] / sum;
        }

        let mut generator = StdRng::from_entropy();
        let mut number_of_particles_in_cell = vec![0; number_of_divisions];

        for _ in 0..self.n
        {    
            let mut num: f64 = generator.sample(Standard);

            for j in 0..number_of_divisions
            {
                if num > probabilities[j]
                {
                    num -= probabilities[j];
                }
                else
                {
                    number_of_particles_in_cell[j] += 1;
                    break;
                }
            }
        }

        let mut output = Vec::<Vector3>::new();

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

                output.push(Self::spherical_to_cartesian(curr_r, phi, theta));
            }
        }

        return output;
    }

    fn generate_velocity(&self, r: f64) -> Vector3
    {
        let disp = G * self.m0 / (6.0 * (r.powi(2) + self.plummer_radius.powi(2)).sqrt());
        let normal_distr = Normal::new(0.0, disp.sqrt()).unwrap();
        let mut uniform_distr = StdRng::from_entropy();

        let mag = normal_distr.sample(&mut rand::thread_rng()).abs() / 2.0;
        let phi: f64 = uniform_distr.sample(Standard);
        let phi = phi * 2.0 * PI;
        let theta: f64 = uniform_distr.sample(Standard);
        let theta = theta * PI;

        return Self::spherical_to_cartesian(mag, phi, theta);
    }
}

impl Generator for Plummer
{
    fn generate(&self) -> Result<ParticleSet, &'static str> 
    {
        let mut output = ParticleSet::new()?;
        let positions = self.generate_positions();
        let masses = vec![self.m0 / (self.n as f64); self.n];

        if positions.len() != masses.len()
        {
            return Err("generation of positions or masses went wrong");
        }

        for i in 0..positions.len()
        {
            output.particles.push(
                Particle::new(
                    positions[i], 
                    self.generate_velocity(positions[i].mag()), 
                    masses[i]
                )?
            );
        }

        return Ok(output);
    }
}