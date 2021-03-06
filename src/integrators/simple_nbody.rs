use super::Integrator;
use crate::particles::{Particle, ParticleSet};
use crate::quantity::{ScalarQuantity, Units};
use crate::vector::Vector3;

/// Integrator that uses direct summation and Euler method for numerical integration.
#[allow(non_snake_case)]
pub struct SimpleNBody {
    positions: Vec<Vector3>,
    velocities: Vec<Vector3>,
    masses: Vec<f64>,
    G: f64,
    timestep: f64,
    model_time: f64
}

impl SimpleNBody {
    /// Initialises integrator.
    pub fn new(particle_set: &ParticleSet) -> Result<SimpleNBody, &'static str> {
        let positions = particle_set
            .particles
            .iter()
            .map(|p| -> Vector3 { p.get_position().value_in(Units::m) })
            .collect();
        let velocities = particle_set
            .particles
            .iter()
            .map(|p| -> Vector3 { p.get_velocity().value_in(Units::ms) })
            .collect();
        let masses = particle_set
            .particles
            .iter()
            .map(|p| -> f64 { p.get_mass().value_in(Units::kg) })
            .collect();

        return Ok(SimpleNBody {
            positions,
            velocities,
            masses,
            G: Units::G
                .convert()
                .value_in_q(Units::m.pow(3.) * Units::kg.pow(-1.) * Units::s.pow(-2.)),
            timestep: 1.0,
            model_time: 0.0
        });
    }

    pub fn set_timestep(&mut self, dt: ScalarQuantity) -> Result<(), String> {
        if !dt.is_compatible(1.0 * Units::s) {
            return Err("dt has incompatible units".to_string());
        }

        self.timestep = dt.value_in(Units::s);

        return Ok(());
    }

    pub fn get_timestep(&self) -> ScalarQuantity {
        return self.timestep * Units::s;
    }

    fn integrate(&mut self, dt: f64) {
        let mut force: Vector3;
        let mut dvs = vec![Vector3::null_vector(); self.positions.len()];
        let mut drs = vec![Vector3::null_vector(); self.positions.len()];

        for i in 0..self.positions.len() {
            force = self.get_force_to_particle(self.positions[i], self.masses[i]);
            dvs[i] = force / self.masses[i] * dt;
        }

        self.update_velocities(dvs);

        for i in 0..self.positions.len() {
            drs[i] = self.velocities[i] * dt;
        }

        self.update_positions(drs);

        self.model_time += dt;
    }

    fn get_force(&self, pos1: Vector3, pos2: Vector3, m1: f64, m2: f64) -> Vector3 {
        let dist = pos1 - pos2;

        return self.G * m1 * m2 / (dist.dot(&dist)) * dist.unit();
    }

    fn update_velocities(&mut self, dvs: Vec<Vector3>) {
        for i in 0..self.velocities.len() {
            self.velocities[i] += dvs[i];
        }
    }

    fn update_positions(&mut self, drs: Vec<Vector3>) {
        for i in 0..self.positions.len() {
            self.positions[i] += drs[i];
        }
    }

    fn get_force_to_particle(&self, pos: Vector3, m: f64) -> Vector3 {
        let mut result = Vector3::null_vector();

        for i in 0..self.positions.len() {
            if self.positions[i] == pos {
                continue;
            }

            result += self.get_force(self.positions[i], pos, self.masses[i], m);
        }

        return result;
    }
}

impl Integrator for SimpleNBody {
    fn get_state(&self) -> Result<ParticleSet, &'static str> {
        let mut result = ParticleSet::new()?;

        for i in 0..self.positions.len() {
            let p = Particle::new(
                self.positions[i] * Units::m,
                self.velocities[i] * Units::ms,
                self.masses[i] * Units::kg,
            )?;

            result.add_particle(p);
        }

        return Ok(result);
    }

    fn evolve(&mut self, time: &ScalarQuantity) -> Result<(), String> {
        if !time.is_compatible(1. * Units::s) {
            return Err("time has wrong units".to_string());
        }

        let time = time.value_in(Units::s);

        while self.model_time <= (time - self.timestep) {
            self.integrate(self.timestep);
        }

        self.integrate(time - self.model_time);

        return Ok(());
    }
}
