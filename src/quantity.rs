use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign };
use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::vector::Vector3;

//-------------------------------SI-------------------------------//

#[derive(Debug, PartialEq, Eq, Hash)]
enum SI
{
    Meter,
    Second,
    Kilogram
}

impl SI
{
    pub fn pow(self, x: f64) -> ScalarQuantity
    {
        let mut a = ScalarQuantity::new();
        a.units.insert(self, x);

        return a;
    }
}

impl Mul<SI> for f64
{
    type Output = ScalarQuantity;

    fn mul(self, rhs: SI) -> ScalarQuantity
    {
        let mut sq = ScalarQuantity::new();
        sq.units.insert(rhs, 1.);
        sq.value = self;

        return sq;
    }
}

//-------------------------------Units-------------------------------//

#[allow(non_camel_case_types)]
pub enum Units
{
    m, pc, kpc,
    s, yr, Myr,
    kg, MSun,
    J,
    G
}

impl Units
{
    pub fn convert(&self) -> ScalarQuantity
    {
        match self
        {
            // meter
            Self::m => 1.0 * SI::Meter,
            // parsec
            Self::pc => 3.086e+16 * SI::Meter,
            // kiloparsec
            Self::kpc => 3.086e+19 * SI::Meter,
            // second
            Self::s => 1.0 * SI::Second,
            // year
            Self::yr => 365.0 * 86400.0 * SI::Second,
            // megayear
            Self::Myr => 1e+6 * 365.0 * 86400.0 * SI::Second,
            // kilogram
            Self::kg => 1.0 * SI::Kilogram,
            // mass of the sun
            Self::MSun => 1.989e+30 * SI::Kilogram,
            // joule
            Self::J => 1.0 * SI::Kilogram * SI::Meter.pow(2.) * SI::Second.pow(-2.),
            // gravitational constant
            Self::G => 6.67e-11 * SI::Meter.pow(3.) * SI::Kilogram.pow(-1.) * SI::Second.pow(-2.),
        }
    }

    pub fn pow(&self, x: f64) -> ScalarQuantity
    {
        return self.convert().pow(x);
    }
}

impl Mul<ScalarQuantity> for Units
{
    type Output = ScalarQuantity;

    fn mul(self, rhs: ScalarQuantity) -> ScalarQuantity
    {
        return ScalarQuantity::new() * self * rhs;
    }
}

impl Div<ScalarQuantity> for Units
{
    type Output = ScalarQuantity;

    fn div(self, rhs: ScalarQuantity) -> ScalarQuantity
    {
        return ScalarQuantity::new() * self / rhs;
    }
}

impl Mul<VectorQuantity> for Units
{
    type Output = VectorQuantity;

    fn mul(self, rhs: VectorQuantity) -> VectorQuantity
    {
        return rhs * self;
    }
}

impl Mul<f64> for Units
{
    type Output = ScalarQuantity;

    fn mul(self, rhs: f64) -> ScalarQuantity
    {
        return ScalarQuantity::new() * self * rhs;
    }
}

impl Div<f64> for Units
{
    type Output = ScalarQuantity;

    fn div(self, rhs: f64) -> ScalarQuantity
    {
        return ScalarQuantity::new() * self / rhs;
    }
}

impl Mul<Vector3> for Units
{
    type Output = VectorQuantity;

    fn mul(self, rhs: Vector3) -> VectorQuantity
    {
        return VectorQuantity::from(rhs) * self;
    }
}

impl Mul<Units> for f64
{
    type Output = ScalarQuantity;

    fn mul(self, rhs: Units) -> ScalarQuantity
    {
        return self * rhs.convert();
    }
}

//-------------------------------ScalarQuantity-------------------------------//

#[derive(Debug, PartialEq)]
pub struct ScalarQuantity
{
    value: f64,
    units: HashMap<SI, f64>
}

impl ScalarQuantity
{
    pub fn new() -> ScalarQuantity
    {
        let mut units = HashMap::new();
        units.insert(SI::Meter, 0.);
        units.insert(SI::Second, 0.);
        units.insert(SI::Kilogram, 0.);

        return ScalarQuantity 
        {
            value: 1.0,
            units
        };
    }

    pub fn pow(&self, x: f64) -> ScalarQuantity
    {
        let mut res = ScalarQuantity::new();

        res.units.insert(SI::Meter, self.units[&SI::Meter] * x);
        res.units.insert(SI::Second, self.units[&SI::Second] * x);
        res.units.insert(SI::Kilogram, self.units[&SI::Kilogram] * x);

        res = res * self.value.powf(x);

        return res;
    }

    pub fn value_in(&self, quantity: ScalarQuantity) -> f64
    {
        if quantity.units != self.units
        {
            panic!("trying to take value in incompatible units");
        }

        return self.value / quantity.value;
    }
}

impl Add for ScalarQuantity
{
    type Output = ScalarQuantity;

    fn add(self, rhs: ScalarQuantity) -> ScalarQuantity 
    { 
        if self.units == rhs.units
        {
            return ScalarQuantity
            {
                value: self.value + rhs.value,
                units: self.units
            };
        }
        else
        {
            panic!("trying to sum incompatible units");
        }
    }
}

impl AddAssign for ScalarQuantity
{
    fn add_assign(&mut self, rhs: ScalarQuantity)
    {
        if self.units != rhs.units
        {
            panic!("trying to sun incompatible units");
        }

        self.value += rhs.value;
    }
}

impl Sub for ScalarQuantity
{
    type Output = ScalarQuantity;

    fn sub(self, rhs: ScalarQuantity) -> ScalarQuantity 
    { 
        if self.units == rhs.units
        {
            return ScalarQuantity
            {
                value: self.value - rhs.value,
                units: self.units
            };
        }
        else
        {
            panic!("trying to sum incompatible units");
        }
    }
}

impl SubAssign for ScalarQuantity
{
    fn sub_assign(&mut self, rhs: ScalarQuantity)
    {
        if self.units != rhs.units
        {
            panic!("trying to sun incompatible units");
        }

        self.value -= rhs.value;
    }
}

impl Mul for ScalarQuantity
{
    type Output = ScalarQuantity;

    fn mul(self, rhs: ScalarQuantity) -> ScalarQuantity
    {
        let mut res = ScalarQuantity::new();

        res.units.insert(SI::Meter, self.units[&SI::Meter] + rhs.units[&SI::Meter]);
        res.units.insert(SI::Second, self.units[&SI::Second] + rhs.units[&SI::Second]);
        res.units.insert(SI::Kilogram, self.units[&SI::Kilogram] + rhs.units[&SI::Kilogram]);
        res.value = self.value * rhs.value;

        return res;
    }
}

impl MulAssign for ScalarQuantity
{
    fn mul_assign(&mut self, rhs: ScalarQuantity)
    {
        self.units.insert(SI::Meter, self.units[&SI::Meter] + rhs.units[&SI::Meter]);
        self.units.insert(SI::Second, self.units[&SI::Second] + rhs.units[&SI::Second]);
        self.units.insert(SI::Kilogram, self.units[&SI::Kilogram] + rhs.units[&SI::Kilogram]);
        self.value *= rhs.value;
    }
}

impl Div for ScalarQuantity
{
    type Output = ScalarQuantity;

    fn div(self, rhs: ScalarQuantity) -> ScalarQuantity
    {
        let mut res = ScalarQuantity::new();

        res.units.insert(SI::Meter, self.units[&SI::Meter] - rhs.units[&SI::Meter]);
        res.units.insert(SI::Second, self.units[&SI::Second] - rhs.units[&SI::Second]);
        res.units.insert(SI::Kilogram, self.units[&SI::Kilogram] - rhs.units[&SI::Kilogram]);
        res.value = self.value / rhs.value;

        return res;
    }
}

impl DivAssign for ScalarQuantity
{
    fn div_assign(&mut self, rhs: ScalarQuantity)
    {
        self.units.insert(SI::Meter, self.units[&SI::Meter] - rhs.units[&SI::Meter]);
        self.units.insert(SI::Second, self.units[&SI::Second] - rhs.units[&SI::Second]);
        self.units.insert(SI::Kilogram, self.units[&SI::Kilogram] - rhs.units[&SI::Kilogram]);
        self.value /= rhs.value;
    }
}

impl Mul<Units> for ScalarQuantity
{
    type Output = ScalarQuantity;

    fn mul(self, rhs: Units) -> ScalarQuantity
    {
        return self * rhs.convert();
    }
}

impl MulAssign<Units> for ScalarQuantity
{
    fn mul_assign(&mut self, rhs: Units)
    {
        *self *= rhs.convert();
    }
}

impl Div<Units> for ScalarQuantity
{
    type Output = ScalarQuantity;

    fn div(self, rhs: Units) -> ScalarQuantity
    {
        return self / (ScalarQuantity::new() * rhs);
    }
}

impl DivAssign<Units> for ScalarQuantity
{
    fn div_assign(&mut self, rhs: Units)
    {
        *self /= rhs.convert();
    }
}

impl Mul<f64> for ScalarQuantity
{
    type Output = ScalarQuantity;

    fn mul(self, rhs: f64) -> ScalarQuantity
    {
        return ScalarQuantity
        {
            value: self.value * rhs,
            units: self.units
        }
    }
}

impl MulAssign<f64> for ScalarQuantity
{
    fn mul_assign(&mut self, rhs: f64)
    {
        self.value *= rhs;
    }
}

impl Div<f64> for ScalarQuantity
{
    type Output = ScalarQuantity;

    fn div(self, rhs: f64) -> ScalarQuantity
    {
        return ScalarQuantity
        {
            value: self.value / rhs,
            units: self.units
        }
    }
}

impl DivAssign<f64> for ScalarQuantity
{
    fn div_assign(&mut self, rhs: f64)
    {
        self.value /= rhs;
    }
}

impl Mul<Vector3> for ScalarQuantity
{
    type Output = VectorQuantity;

    fn mul(self, rhs: Vector3) -> VectorQuantity
    {
        return VectorQuantity::from(rhs) * self;
    }
}

impl Mul<VectorQuantity> for ScalarQuantity
{
    type Output = VectorQuantity;

    fn mul(self, rhs: VectorQuantity) -> VectorQuantity
    {
        return rhs * self;
    }
}

impl Mul<ScalarQuantity> for f64
{
    type Output = ScalarQuantity;

    fn mul(self, rhs: ScalarQuantity) -> ScalarQuantity
    {
        return ScalarQuantity
        {
            value: self * rhs.value,
            units: rhs.units
        }
    }
}

impl Div<ScalarQuantity> for f64
{
    type Output = ScalarQuantity;

    fn div(self, rhs: ScalarQuantity) -> ScalarQuantity
    {
        return (ScalarQuantity::new() * self) / rhs;
    }
}

impl Mul<ScalarQuantity> for Vector3
{
    type Output = VectorQuantity;

    fn mul(self, rhs: ScalarQuantity) -> VectorQuantity
    {
        return VectorQuantity::from(self) * rhs;
    }
}

impl Div<ScalarQuantity> for Vector3
{
    type Output = VectorQuantity;

    fn div(self, rhs: ScalarQuantity) -> VectorQuantity
    {
        return VectorQuantity::from(self) / rhs;
    }
}

//-------------------------------VectorQuantity-------------------------------//

#[derive(Debug, PartialEq)]
pub struct VectorQuantity
{
    value: Vector3,
    units: HashMap<SI, f64>
}

impl VectorQuantity
{
    pub fn from(v: Vector3) -> VectorQuantity
    {
        let mut units = HashMap::new();

        units.insert(SI::Meter, 0.);
        units.insert(SI::Second, 0.);
        units.insert(SI::Kilogram, 0.);

        return VectorQuantity { value: v, units }
    }

    pub fn mag(self) -> ScalarQuantity
    {
        return ScalarQuantity
        {
            value: self.value.mag(),
            units: self.units
        }
    }
}

impl Add for VectorQuantity
{
    type Output = VectorQuantity;

    fn add(self, rhs: VectorQuantity) -> VectorQuantity
    {
        if self.units == rhs.units
        {
            return VectorQuantity
            {
                value: self.value + rhs.value,
                units: self.units
            };
        }
        else
        {
            panic!("trying to sum incompatible units");
        }
    }
}

impl Sub for VectorQuantity
{
    type Output = VectorQuantity;

    fn sub(self, rhs: VectorQuantity) -> VectorQuantity
    {
        if self.units == rhs.units
        {
            return VectorQuantity
            {
                value: self.value - rhs.value,
                units: self.units
            };
        }
        else
        {
            panic!("trying to sum incompatible units");
        }
    }
}

impl Mul<Units> for VectorQuantity
{
    type Output = VectorQuantity;

    fn mul(self, rhs: Units) -> VectorQuantity
    {
        return self * (ScalarQuantity::new() * rhs);
    }
}

impl Div<Units> for VectorQuantity
{
    type Output = VectorQuantity;

    fn div(self, rhs: Units) -> VectorQuantity
    {
        return self / (ScalarQuantity::new() * rhs);
    }
}

impl Mul<ScalarQuantity> for VectorQuantity
{
    type Output = VectorQuantity;

    fn mul(self, rhs: ScalarQuantity) -> VectorQuantity
    {
        let mut units = HashMap::new();
        units.insert(SI::Meter, self.units[&SI::Meter] + rhs.units[&SI::Meter]);
        units.insert(SI::Second, self.units[&SI::Second] + rhs.units[&SI::Second]);
        units.insert(SI::Kilogram, self.units[&SI::Kilogram] + rhs.units[&SI::Kilogram]);

        return VectorQuantity
        {
            value: self.value * rhs.value,
            units
        }
    }
}

impl Div<ScalarQuantity> for VectorQuantity
{
    type Output = VectorQuantity;

    fn div(self, rhs: ScalarQuantity) -> VectorQuantity
    {
        let mut units = HashMap::new();
        units.insert(SI::Meter, self.units[&SI::Meter] - rhs.units[&SI::Meter]);
        units.insert(SI::Second, self.units[&SI::Second] - rhs.units[&SI::Second]);
        units.insert(SI::Kilogram, self.units[&SI::Kilogram] - rhs.units[&SI::Kilogram]);

        return VectorQuantity
        {
            value: self.value / rhs.value,
            units
        }
    }
}

impl Mul<f64> for VectorQuantity
{
    type Output = VectorQuantity;

    fn mul(self, rhs: f64) -> VectorQuantity
    {
        return VectorQuantity
        {
            value: self.value * rhs,
            units: self.units
        }
    }
}

impl Div<f64> for VectorQuantity
{
    type Output = VectorQuantity;

    fn div(self, rhs: f64) -> VectorQuantity
    {
        return VectorQuantity
        {
            value: self.value / rhs,
            units: self.units
        }
    }
}

impl Mul<Units> for Vector3
{
    type Output = VectorQuantity;

    fn mul(self, rhs: Units) -> VectorQuantity
    {
        return VectorQuantity::from(self) * rhs;
    }
}

impl Div<Units> for Vector3
{
    type Output = VectorQuantity;

    fn div(self, rhs: Units) -> VectorQuantity
    {
        return VectorQuantity::from(self) /  rhs;
    }
}