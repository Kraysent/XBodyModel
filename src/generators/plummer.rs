use super::Generator;
use crate::particles::*;
use crate::quantity::{ScalarQuantity, Units};
use crate::vector::Vector3;
use rand::distributions::Standard;
use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use std::f64::consts::PI;

/// Struct that handles creation of sphere with Plummer distribution
#[allow(non_snake_case)]
pub struct Plummer {
    plummer_radius: f64,
    n: usize,
    m0: f64,
    G: f64,
}

impl Plummer {
    /// Creates new `Plummer` struct with given parameters:
    /// `r` - Plummer radius - a scale parameter that sets the size of the cluster core,
    /// `n` - number of particles,
    /// `m` - whole mass of the cluster.
    pub fn new(r: ScalarQuantity, n: usize, m: ScalarQuantity) -> Result<Plummer, &'static str> {
        let rad_check = |rad: &ScalarQuantity| -> bool {
            if rad.is_compatible(Units::m.convert()) {
                *rad >= (0. * Units::m)
            } else {
                false
            }
        };
        let n_check = |number: usize| -> bool { number != 0 };
        let m_check = |mass: &ScalarQuantity| -> bool {
            if mass.is_compatible(Units::kg.convert()) {
                *mass > (0.0 * Units::kg)
            } else {
                false
            }
        };

        if !rad_check(&r) {
            return Err("incorrect radius");
        }
        if !n_check(n) {
            return Err("incorrect number of particles");
        }
        if !m_check(&m) {
            return Err("incorrect mass");
        }

        return Ok(Plummer {
            plummer_radius: r.value_in(Units::m),
            n,
            m0: m.value_in(Units::kg),
            G: Units::G
                .convert()
                .value_in_q(Units::m.pow(3.) * Units::kg.pow(-1.) * Units::s.pow(-2.)),
        });
    }

    fn rho(&self, r: f64) -> f64 {
        return (3.0 * self.m0) / (4.0 * PI * self.m0.powi(3))
            * (1.0 + (r / self.plummer_radius).powi(2)).powf(-5.0 / 2.0);
    }

    fn spherical_to_cartesian(r: f64, phi: f64, theta: f64) -> Vector3 {
        let x = r * theta.sin() * phi.cos();
        let y = r * theta.sin() * phi.sin();
        let z = r * theta.cos();

        return Vector3 { x, y, z };
    }

    fn generate_positions(&self) -> Vec<Vector3> {
        let number_of_divisions: usize = (self.n as f32).powf(1.0 / 3.0) as usize;
        let max_r = 10.0 * self.plummer_radius;
        let division = 1.0 / (number_of_divisions as f64) * max_r;
        let mut descrete_rhos = vec![0.0_f64; number_of_divisions];

        for i in 0..number_of_divisions {
            descrete_rhos[i] = self.rho(division * (i as f64));
        }

        let sum: f64 = descrete_rhos.iter().sum();
        let mut probabilities = vec![0.0_f64; number_of_divisions + 1];

        for i in 0..number_of_divisions {
            probabilities[i] = descrete_rhos[i] / sum;
        }

        let mut generator = StdRng::from_entropy();
        let mut number_of_particles_in_cell = vec![0; number_of_divisions];

        for _ in 0..self.n {
            let mut num: f64 = generator.sample(Standard);

            for j in 0..number_of_divisions {
                if num > probabilities[j] {
                    num -= probabilities[j];
                } else {
                    number_of_particles_in_cell[j] += 1;
                    break;
                }
            }
        }

        let mut output = Vec::<Vector3>::new();

        for i in 0..number_of_divisions {
            let curr = number_of_particles_in_cell[i];

            if curr == 0 {
                continue;
            }

            for j in 1..curr + 1 {
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

    fn generate_velocities(&self, positions: Vec<f64>) -> Vec<Vector3> {
        let mut uniform_distr = StdRng::from_entropy();
        let mut output = Vec::new();

        for r in positions {
            let disp = self.G * self.m0 / (6.0 * (r.powi(2) + self.plummer_radius.powi(2)).sqrt());
            let normal_distr = Normal::new(0.0, disp.sqrt()).unwrap();

            let mag = normal_distr.sample(&mut rand::thread_rng()).abs() / 2.0;
            let phi: f64 = uniform_distr.sample(Standard);
            let phi = phi * 2.0 * PI;
            let theta: f64 = uniform_distr.sample(Standard);
            let theta = theta * PI;

            output.push(Self::spherical_to_cartesian(mag, phi, theta));
        }

        return output;
    }

    fn generate_particles(
        positions: Vec<Vector3>,
        velocities: Vec<Vector3>,
        masses: Vec<f64>,
    ) -> Result<ParticleSet, &'static str> {
        if (positions.len() != velocities.len()) || (positions.len() != masses.len()) {
            return Err("generation of positions, velocities or masses went wrong");
        }

        let mut output = ParticleSet::new()?;

        for i in 0..positions.len() {
            output.add_particle(Particle::new(
                positions[i] * Units::m,
                velocities[i] * Units::ms,
                masses[i] * Units::kg,
            )?);
        }

        return Ok(output);
    }
}

impl Generator for Plummer {
    fn generate(&self) -> Result<ParticleSet, &'static str> {
        let positions = self.generate_positions();
        let velocities =
            self.generate_velocities(positions.iter().map(|r| -> f64 { r.mag() }).collect());
        let masses = vec![self.m0 / (self.n as f64); self.n];

        return Self::generate_particles(positions, velocities, masses);
    }
}
