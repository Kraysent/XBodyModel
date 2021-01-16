use std::cmp::PartialEq;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};

#[derive(Copy, Clone, Debug)]
pub struct Vector3
{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3
{
    pub fn dot(&self, v: &Vector3) -> f64
    {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }

    pub fn mag(&self) -> f64
    {
        return self.dot(self).sqrt();
    }

    pub fn unit(&self) -> Vector3
    {
        let m = self.mag();

        return Vector3 { 
            x: self.x / m,
            y: self.y / m,
            z: self.z / m
        };
    }

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

impl SubAssign for Vector3
{
    fn sub_assign(&mut self, rhs: Vector3) 
    { 
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;        
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