use nbody::generators::{ Generator, plummer::Plummer };
use nbody::integrators::{ Integrator, simple_nbody::SimpleNBody };
use nbody::particles::Particle;
use nbody::profiler::Profiler;
use nbody::quantity::Unit;
use std::process;

fn main()
{
    let r = Unit::kpc() * 0.01;
    let m = Unit::msun() * 1e+9;
    let n = 100;

    let plummer = Plummer::new(r.value, n, Some(m.value)).unwrap_or_else(|err| 
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

    let dt = Unit::Second * 365.0 * 86400.0;

    {
        let _p = Profiler::new(None);

        for _ in 0..1000
        {
            integr.integrate(dt.value);
        }
    }

    for p in integr.particles.particles.iter()
    {
        print_particle(&p);
    }
}

fn print_particle(p: &Particle)
{
    println!("{} {} {} {} {} {} {}", 
                p.position.x, p.position.y, p.position.z,
                p.velocity.x, p.velocity.y, p.velocity.z,
                p.mass
            );
}