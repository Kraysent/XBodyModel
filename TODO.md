## These are things that need implementation:
### Nearest:
* add epsilon (smoothing factor) to forces count in SimpleNBody generator
* add Runge-Kutta method for numerical integration in SimpleNBody (most likely would use `enum` for this)
* use quantities in computations
* create tests for basic `ScalarQuantity` and `VectorQuantity` functions
* implement some basic mathematical operations on quantitites (including sqrt and comparison)
* create tests for `VectorQuantity` operations

### In more distant future:
* add more integrators (Barnes-Hut algorithm at least) and generators (Solar system at least)
* use multithreading to increase perfomance 
* use C++ code to increase perfomance 
* use GPU (primarily, CUDA or OpenCL) to increase perfomance
* add CLI
* add some UI (probably, using python's matplotlib or SDL in C++)
* add unit tests for integrators and generators
* add documentation

## Completed things:
* implement all operations on quantities and prepare unit tests for each of them

## Thoughts
* Maybe timestep should be a property of integrator and it should evolve on some other time provided to the `Integrator.evolve` function
* It would be good to create a way that forces integrators and generators to use quantities of specific types; for example, radius of the Plummer sphere **must** be in meters and in nothing else. 
