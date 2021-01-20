## These are things that need implementation:
### Nearest:
* add epsilon (smoothing factor) to forces count in SimpleNBody generator
* add Runge-Kutta method for numerical integration in SimpleNBody (most likely would use `enum` for this)
* use quantities in computations
* add vector quantities

### In more distant future:
* add more integrators (Barnes-Hut algorithm at least) and generators (Solar system at least)
* use multithreading to increase perfomance 
* use C++ code to increase perfomance 
* use GPU (primarily, CUDA or OpenCL) to increase perfomance
* add some UI (probably, using python's matplotlib or SDL in C++)
* add unit tests for integrators and generators
* add documentation