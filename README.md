## Currently available functionality on Quantities:
|                   	| Unit 	| ComplexUnit 	| Quantity 	| VectorQuantity 	|
|-------------------	|:----:	|:-----------:	|:--------:	|:--------------:	|
| +- Unit           	|   +  	|      +      	|     +    	|        x       	|
| +- ComplexUnit    	|   +  	|      +      	|     +    	|        x       	|
| +- Quantity       	|   +  	|      +      	|     +    	|        x       	|
| +- VectorQuantity 	|   x  	|      x      	|     x    	|        +       	|
| *  Unit           	|   +  	|      +      	|     +    	|        +       	|
| /  Unit           	|   +  	|      +      	|     +    	|        +       	|
| *  ComplexUnit    	|   +  	|      +      	|     +    	|        +       	|
| /  ComplexUnit    	|   +  	|      +      	|     +    	|        +       	|
| *  Quantity       	|   +  	|      +      	|     +    	|        +       	|
| /  Quantity       	|   +  	|      +      	|     +    	|        +       	|
| *  VectorQuantity 	|   +  	|      +      	|     +    	|        x       	|
| /  VectorQuantity 	|   x  	|      x      	|     x    	|        x       	|
| *  f64            	|   +  	|      +      	|     +    	|        +       	|
| /  f64            	|   +  	|      +      	|     +    	|        +       	|
| *  Vector3        	|   +  	|      +      	|     +    	|        x       	|
| /  Vector3        	|   x  	|      x      	|     x    	|        x       	| 

To create scalar quanitity use:
```
let q = Quantity::new() * (/* values in f64 and units */)
```
For example
```
let q1 = Quantity::new() * 5.0 * ComplexUnit::m.pow(2); // 5 meters^2
let q2 = Quantity::new() * ComplexUnit::kpc * 2.3; // 2.3 kiloparsec
let q3 = Quantity::new() * 5.0 * ComplexUnit::J / (2.0 * ComplexUnit::s); // 5 joules per second
```
To create vector quantity you can use same principle with VectorQuantity instead (take operations above into consideration)