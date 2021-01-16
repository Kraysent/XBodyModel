use nbody::generators::{ Generator, plummer::Plummer };
use nbody::integrators::{ Integrator, simple_nbody::SimpleNBody };
use nbody::particles::Particle;
use nbody::profiler::Profiler;
use std::process;

fn main()
{
    let plummer = Plummer::new(1.0).unwrap_or_else(|err| 
    {
        eprintln!("Error during initialisation of particles: {}", err);
        process::exit(1);
    });
    
    let mut ps = plummer.generate().unwrap_or_else(|err| 
    {
        eprintln!("Error during generation of particles: {}", err);
        process::exit(1);
    });

    let mut integr = SimpleNBody::new(&mut ps).unwrap_or_else(|err| 
    {
        eprintln!("Error during initialisation of integrator: {}", err);
        process::exit(1);
    });
    
    // TODO: use iterators for this
    for p in integr.particles.particles.iter()
    {
        print_particle(&p);
    }

    {
        let _p = Profiler::new(Some("integration"));

        for _ in 0..300000
        {
            integr.integrate(0.01);
        }
    }

    for p in integr.particles.particles.iter()
    {
        print_particle(&p);
    }
}

fn print_particle(p: &Particle)
{
    println!("Particle in ({:?}) with vel ({:?}) and mass {}", 
             p.position, p.velocity, p.mass);
}