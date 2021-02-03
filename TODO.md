## These are things that need implementation:
### Nearest:
* add epsilon (smoothing factor) to forces count in `SimpleNBody` generator
* add Runge-Kutta method for numerical integration in `SimpleNBody` (most likely would use `enum` for this)
* create tests for basic `ScalarQuantity` and `VectorQuantity` functions
* implement custom error class in order to get rid of `&'static str` or `String` in each `Result` 
* add `check_compatability` method to quantities

### In more distant future:
* add more integrators (Barnes-Hut algorithm at least) and generators (Solar system at least)
* use multithreading to increase perfomance 
* use C++ code to increase perfomance 
* use GPU (primarily, CUDA or OpenCL) to increase perfomance
* add CLI
* add some UI (probably, using python's matplotlib, SDL in C++ or even REST API)
* add unit tests for integrators and generators
* add documentation

## Completed things:
* implement all operations on quantities and prepare unit tests for each of them

## Thoughts
* It would be good to create a way that forces integrators and generators to use quantities of specific types; for example, radius of the Plummer sphere **must** be in meters (or equivalent) and in nothing else. If it would be in compile-time - it would be great.
* Probably I should move `SI` struct to `core` module and make something like `System` trait which should somehow deal with different systems like SI, CGS and so on 
* Maybe move Euler (or other) method to separate struct and call it from the integrators
* Probably should add `Quantity` trait and derive `ScalarQuantity` and `VectorQuantity` from it for methods like `is_compatible` and `value_in`