use nbody::generators::{ Generator, plummer::Plummer };
use nbody::integrators::{ Integrator, simple_nbody::SimpleNBody };
use nbody::particles::Particle;
use nbody::profiler::Profiler;
use std::process;

fn main()
{
    let plummer = Plummer::new(2.0, 10_000, None).unwrap_or_else(|err| 
    {
        eprintln!("Error during initialisation of particles: {}", err);
        process::exit(1);
    });
    
    let ps = plummer.generate().unwrap_or_else(|err| 
    {
        eprintln!("Error during generation of particles: {}", err);
        process::exit(1);
    });

    eprintln!("Legend: x y z vx vy vz m");
    for p in ps.particles.iter()
    {
        print_particle(&p);
    }

    // let mut integr = SimpleNBody::new(&mut ps).unwrap_or_else(|err| 
    // {
    //     eprintln!("Error during initialisation of integrator: {}", err);
    //     process::exit(1);
    // });
}

fn print_particle(p: &Particle)
{
    println!("{} {} {} {} {} {} {}", 
                p.position.x, p.position.y, p.position.z,
                p.velocity.x, p.velocity.y, p.velocity.z,
                p.mass
            );
}