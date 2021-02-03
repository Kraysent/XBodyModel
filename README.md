# XBodyModel
Library that creates core for computations on sets of particles in gravitational N-body problem

## Quantities
To create scalar quanitity use:
```
use nbody::quantity::Units;
...
let q1 = 5.0 * Units::m; // 5 meters
let q2 = 2.5 * Units::kms; // 2.5 kilometers per second
let q3 = 1. * Units::m * Units::kg; 1 meter-kilogram
let q4 = 1. * Units::G; // 1 gravitational constant
```

To create vector quantity use:
```
let vq1 = Vector3::new(2., 4., 1.) * Units::m; // vector with components (2 meters, 4 meters, 1 meter)
```

## Generators
Generator is the structure that has a goal of creating the set of particles that obeys some hardcoded (or not) rule (for example, density profile).
### Implemented generators: 
* [Plummer sphere](/src/generators/plummer.rs) - sphere with density profile of [Plummer model](https://en.wikipedia.org/wiki/Plummer_model)

## Integrators
Integrator is the structure that has a goal af integrating given set of particles using some algorithm.
### Implemented integrators: 
* [SimpleNBody](/src/integrators/simple_nbody.rs) - integrator that uses simple direct-summation algorithm and [Euler method](https://en.wikipedia.org/wiki/Euler_method) for integration.

## Examples
### Minimal working example
Create plummer sphere and then integrate it for 1'000 years with the step of 1 year:
```
use xbody_model::quantity::Units;
use xbody_model::generators::{Generator, simple_nbody::SimpleNBody};
use xbody_model::integrators::{Integrator, plummer::Plummer};

fn main() -> Result<(), String> {
    let r = 0.01 * Units::kpc;
    let m = 1e+9 * Units::MSun;
    let n = 100;
    
    // Create new instance of Plummer sphere
    let plummer = Plummer::new(r, n, m)?;
    // Generate actual particle set
    let mut ps = plummer.generate()?;
    // Create new instance of SimpleNBody integrator
    let mut integrator = SimpleNBody::new(&mut ps)?;

    let dt = 1.0 * Units::yr;
    
    for i in 0..1000 {
        integrator.evolve(&(dt * (i as f64)))?;
    }

    return Ok(());
}
```

You can also use functions like `Particle::get_position`, `Particle::get_velocity`, `Particle::get_mass` to get parameters of each particle at every instant. 
To iterate over particles in integrator use
```
for p in integrator.get_state().particles.iter() { ... }
``` 