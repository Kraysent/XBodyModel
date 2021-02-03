use std::process;
use xbody_model::generators::{plummer::Plummer, Generator};
use xbody_model::integrators::{simple_nbody::SimpleNBody, Integrator};
use xbody_model::particles::Particle;
use xbody_model::profiler::Profiler;
use xbody_model::quantity::Units;

fn main() -> Result<(), String> {
    let r = 0.01 * Units::kpc;
    let m = 1e+9 * Units::MSun;
    let n = 100;

    let plummer = Plummer::new(r, n, m).unwrap_or_else(|err| {
        eprintln!("Error during initialisation of particles: {}", err);
        process::exit(1);
    });

    let mut ps = plummer.generate().unwrap_or_else(|err| {
        eprintln!("Error during generation of particles: {}", err);
        process::exit(1);
    });

    eprintln!("Legend: x y z vx vy vz m");
    for p in ps.particles.iter() {
        print_particle(&p);
    }

    let mut integr = SimpleNBody::new(&mut ps).unwrap_or_else(|err| {
        eprintln!("Error during initialisation of integrator: {}", err);
        process::exit(1);
    });

    let dt = 1.0 * Units::yr;

    {
        let _p = Profiler::new(None);

        for i in 0..1000 {
            integr.evolve(&(dt * (i as f64)))?;
        }
    }

    for p in integr.get_state().unwrap().particles {
        print_particle(&p);
    }

    return Ok(());
}

fn print_particle(p: &Particle) {
    let pos = p.get_position(); //.value_in(Units::m);
    let vel = p.get_velocity(); //.value_in(Units::ms);
    let mass = p.get_mass(); //.value_in(Units::kg);

    println!(
        "{:e}\t{:e}\t{:e}",
        pos, vel, mass
    );
}
