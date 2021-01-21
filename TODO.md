## These are things that need implementation:
### Nearest:
* add epsilon (smoothing factor) to forces count in SimpleNBody generator
* add Runge-Kutta method for numerical integration in SimpleNBody (most likely would use `enum` for this)
* use quantities in computations
* implement all quantities operations (see below) and prepare unit tests for each of them
* create tests for basic unit & complexunit & quantity finctions

### In more distant future:
* add more integrators (Barnes-Hut algorithm at least) and generators (Solar system at least)
* use multithreading to increase perfomance 
* use C++ code to increase perfomance 
* use GPU (primarily, CUDA or OpenCL) to increase perfomance
* add CLI
* add some UI (probably, using python's matplotlib or SDL in C++)
* add unit tests for integrators and generators
* add documentation

### Operations on quantities:
|                   	| Unit 	| ComplexUnit 	| Quantity 	| VectorQuantity 	|
|-------------------	|:----:	|:-----------:	|:--------:	|:--------------:	|
| +- Unit           	|   +  	|      +      	|     +    	|                	|
| +- ComplexUnit    	|   +  	|      +      	|     +    	|                	|
| +- Quantity       	|   +  	|      +      	|     +    	|                	|
| +- VectorQuantity 	|   x  	|      x      	|     x    	|                	|
| *  Unit           	|   +  	|      +      	|     +    	|                	|
| /  Unit           	|   +  	|      +      	|     +    	|                	|
| *  ComplexUnit    	|   +  	|      +      	|     +    	|                	|
| /  ComplexUnit    	|   +  	|      +      	|     +    	|                	|
| *  Quantity       	|   +  	|      +      	|     +    	|                	|
| /  Quantity       	|   +  	|      +      	|     +    	|                	|
| *  VectorQuantity 	|      	|             	|          	|                	|
| /  VectorQuantity 	|   x  	|      x      	|     x    	|        x       	|
| *  f64            	|   +  	|      +      	|     +    	|                	|
| /  f64            	|   +  	|      +      	|     +    	|                	|
| *  Vector3        	|      	|             	|          	|                	|
| /  Vector3        	|   x  	|      x      	|     x    	|        x       	|

## Thoughts
* Do operations on quantities need to panic in case of incompatability?