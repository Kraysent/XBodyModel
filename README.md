## Currently available functionality on Quantities:
|     o     | SQ o SQ | SQ o VQ | VQ o SQ | VQ o VQ | f64 o SQ | SQ o f64 | V3 o SQ | SQ o V3 | f64 o VQ | VQ o f64 |
|:---------:|:-------:|:-------:|:-------:|:-------:|:--------:|:--------:|:-------:|:-------:|:--------:|:--------:|
|     +     |    +    |    x    |    x    |    +    |     x    |     x    |    x    |    x    |     x    |     x    |
|     +=    |    +    |    x    |    x    |    +    |     x    |     x    |    x    |    x    |     x    |     x    |
|     -     |    +    |    x    |    x    |    +    |     x    |     x    |    x    |    x    |     x    |     x    |
|     -=    |    +    |    x    |    x    |    +    |     x    |     x    |    x    |    x    |     x    |     x    |
|     *     |    +    |    +    |    +    |    x    |     +    |     +    |    +    |    +    |     +    |     +    |
|     *=    |    +    |    x    |    +    |    x    |     x    |     +    |    x    |    x    |     x    |     +    |
|     /     |    +    |    x    |    +    |    x    |     +    |     +    |    +    |    x    |     x    |     +    |
|     /=    |    +    |    x    |    +    |    x    |     x    |     +    |    x    |    x    |     x    |     +    |
| >,<,>=,<= |    +    |    x    |    x    |    x    |     x    |     x    |    x    |    x    |     x    |     x    |

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