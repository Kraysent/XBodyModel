use std::cmp::PartialEq;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

/// Represents 3-dimentional vector
#[derive(Copy, Clone, Debug)]
pub struct Vector3
{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3
{
    /// New vector with given coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Vector3
    {
        return Vector3 { x, y, z };
    }

    /// Dot product of two vectors.
    pub fn dot(&self, v: &Vector3) -> f64
    {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }

    /// Magnitude of the vector.
    pub fn mag(&self) -> f64
    {
        return self.dot(self).sqrt();
    }

    /// Vector that has the same direction but unit length.
    pub fn unit(&self) -> Vector3
    {
        let m = self.mag();

        return Vector3 { 
            x: self.x / m,
            y: self.y / m,
            z: self.z / m
        };
    }

    /// Vector with all coordinates equal to 0.
    pub fn null_vector() -> Vector3
    {
        return Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    }
}

impl PartialEq for Vector3
{
    fn eq(&self, v: &Vector3) -> bool
    {
        return (self.x == v.x)
            && (self.y == v.y)
            && (self.z == v.z);
    }
}

impl Add for Vector3
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self 
    { 
        Self { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl AddAssign for Vector3
{
    fn add_assign(&mut self, rhs: Vector3) 
    { 
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;        
    }
}

impl Sub for Vector3
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self 
    { 
        Self {
            x: self.x - rhs.x, 
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl SubAssign for Vector3
{
    fn sub_assign(&mut self, rhs: Vector3) 
    { 
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;        
    }
}

impl Mul<f64> for Vector3
{
    type Output = Self;

    fn mul(self, rhs: f64) -> Self 
    { 
        Self {
            x: self.x * rhs, 
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl MulAssign<f64> for Vector3
{
    fn mul_assign(&mut self, rhs: f64)
    {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vector3
{
    type Output = Self;

    fn div(self, rhs: f64) -> Self 
    { 
        Self {
            x: self.x / rhs, 
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl DivAssign<f64> for Vector3
{
    fn div_assign(&mut self, rhs: f64)
    {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Mul<Vector3> for f64
{
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3
    {
        return Vector3 
        {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}