/// This module contains trait and its implementations that are responsible for creating sets of particles.
pub mod generators;
/// This module containd trait and its implementations that are responsible for integrating sets of particles.
pub mod integrators;
/// This module contains definition of `Particle` and `ParticleSet` structures that respresent material points.
pub mod particles;
/// This module contains definition of `Profiler` structure that is used for time measurements.
pub mod profiler;
/// This module contains definitions of different units and quantities and operations on them.
pub mod quantity;
/// This module contains definition of `Vector3` structure that represents 3D vector.
pub mod vector;
