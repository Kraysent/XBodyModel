use crate::quantity::*;
use crate::vector::Vector3;
use std::cmp::PartialEq;

pub struct Particle {
    position: VectorQuantity,
    velocity: VectorQuantity,
    mass: ScalarQuantity,
}

impl Particle {
    pub fn new(
        position: VectorQuantity,
        velocity: VectorQuantity,
        mass: ScalarQuantity,
    ) -> Result<Particle, &'static str> {
        let mut res = Particle::empty();

        res.set_position(position)?;
        res.set_velocity(velocity)?;
        res.set_mass(mass)?;

        return Ok(res);
    }

    pub fn empty() -> Particle {
        return Particle {
            position: Vector3::null_vector() * Units::m,
            velocity: Vector3::null_vector() * Units::ms,
            mass: 1. * Units::kg,
        };
    }

    pub fn set_position(&mut self, position: VectorQuantity) -> Result<(), &'static str> {
        let check_pos = |pos: &VectorQuantity| -> bool {
            return pos.is_compatible(Units::m.convert());
        };

        if !check_pos(&position) {
            return Err("incorrect position");
        }

        self.position = position;

        return Ok(());
    }

    pub fn set_velocity(&mut self, velocity: VectorQuantity) -> Result<(), &'static str> {
        let check_vel = |vel: &VectorQuantity| -> bool {
            return vel.is_compatible(Units::ms.convert());
        };

        if !check_vel(&velocity) {
            return Err("incorrect velocity");
        }

        self.velocity = velocity;

        return Ok(());
    }

    pub fn set_mass(&mut self, mass: ScalarQuantity) -> Result<(), &'static str> {
        let check_mass = |m: &ScalarQuantity| -> bool {
            if m.is_compatible(Units::kg.convert()) {
                return *m >= 0. * Units::kg;
            }

            return false;
        };

        if !check_mass(&mass) {
            return Err("incorrect mass");
        }

        self.mass = mass;

        return Ok(());
    }

    pub fn get_position(&self) -> VectorQuantity {
        return self.position;
    }

    pub fn get_velocity(&self) -> VectorQuantity {
        return self.velocity;
    }

    pub fn get_mass(&self) -> ScalarQuantity {
        return self.mass;
    }
}

impl PartialEq for Particle {
    fn eq(&self, p: &Particle) -> bool {
        return (self.position == p.position)
            && (self.velocity == p.velocity)
            && (self.mass == p.mass);
    }
}

/// Represents the set of particles; it is needed in order
/// to be able to calculate things like energies and so on
pub struct ParticleSet {
    pub particles: Vec<Particle>,
}

impl ParticleSet {
    /// Creates new instance of a `ParticleSet`
    ///
    /// `return`: `ParticleSet` or the error with description
    pub fn new() -> Result<ParticleSet, &'static str> {
        return Ok(ParticleSet {
            particles: Vec::new(),
        });
    }

    /// Adds one particle to the set
    ///
    /// `p`: given particle
    pub fn add_particle(&mut self, p: Particle) {
        self.particles.push(p);
    }

    /// Adds one set of particles into another
    ///
    /// `ps`: given set of particles
    pub fn add_particles(&mut self, ps: ParticleSet) {
        for p in ps.particles {
            self.add_particle(p);
        }
    }

    /// Returns kinetic energy of the particle set;
    /// Complexity: O(N)
    /// `return`: ScalarQuantity equivalent to Units::J
    pub fn get_kinetic_energy(&self) -> ScalarQuantity {
        let mut result = 0. * Units::J;

        for p in self.particles.iter() {
            result += p.get_mass() * p.get_velocity().mag().pow(2.) / 2.;
        }

        return result;
    }

    /// Returns potential energy of the particle set;
    /// Complexity: O(N^2)
    /// `return`: ScalarQuantity equivalent to Units::J
    pub fn get_potential_energy(&self) -> ScalarQuantity {
        let mut result = 0. * Units::J;
        let set = &self.particles;

        for i in 0..set.len() {
            for j in 0..set.len() {
                if i == j {
                    continue;
                }

                let r = (set[i].get_position() - set[j].get_position()).mag();
                result -= Units::G * set[i].get_mass() * set[j].get_mass() / r;
            }
        }

        return result;
    }
}
