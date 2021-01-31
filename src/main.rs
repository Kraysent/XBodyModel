use nbody::generators::{ Generator, plummer::Plummer };
use nbody::integrators::{ Integrator, simple_nbody::SimpleNBody };
use nbody::particles::Particle;
use nbody::profiler::Profiler;
use nbody::quantity::Units;
use std::process;

fn main()
{
    let r = 0.01 * Units::kpc;
    let m = 1e+9 * Units::MSun;
    let n = 100;

    let plummer = Plummer::new(r, n, m).unwrap_or_else(|err| 
    {
        eprintln!("Error during initialisation of particles: {}", err);
        process::exit(1);
    });
    
    let mut ps = plummer.generate().unwrap_or_else(|err| 
    {
        eprintln!("Error during generation of particles: {}", err);
        process::exit(1);
    });

    eprintln!("Legend: x y z vx vy vz m");
    for p in ps.particles.iter()
    {
        print_particle(&p);
    }

    let mut integr = SimpleNBody::new(&mut ps).unwrap_or_else(|err| 
    {
        eprintln!("Error during initialisation of integrator: {}", err);
        process::exit(1);
    });

    let dt = Units::yr * 1.0;

    {
        let _p = Profiler::new(None);

        for _ in 0..1000
        {
            integr.integrate(dt.value_in(Units::s));
        }
    }

    for p in integr.get_state().unwrap().particles
    {
        print_particle(&p);
    }
}

fn print_particle(p: &Particle)
{
    let pos = p.position.value_in(Units::m);
    let vel = p.velocity.value_in(Units::ms);
    let mass = p.mass.value_in(Units::kg);
    
    println!("{:.5e}\t{:.5e}\t{:.5e}\t{:.5e}\t{:.5e}\t{:.5e}\t{:.5e}", 
                pos.x, pos.y, pos.z,
                vel.x, vel.y, vel.z,
                mass
            );
}