use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::fmt::{ Display, Formatter, Result };
use crate::vector::Vector3;

//-------------------------------SI-------------------------------//

#[derive(PartialEq, Debug)]
struct SI
{
    meters: f64,
    seconds: f64,
    kilograms: f64
}

impl SI
{
    pub fn new(meters: f64, seconds: f64, kilograms: f64) -> SI
    {
        return SI { meters, seconds, kilograms };
    }

    pub fn empty() -> SI
    {
        return SI { meters: 0., seconds: 0., kilograms: 0. };
    }

    pub fn pow(&self, x: f64) -> SI
    {
        return SI 
        { 
            meters: self.meters * x, 
            seconds: self.seconds * x, 
            kilograms: self.kilograms * x
        };
    }
}

impl Add for SI
{
    type Output = SI;

    fn add(self, rhs: SI) -> SI
    {
        return SI
        {
            meters: self.meters + rhs.meters,
            seconds: self.seconds + rhs.seconds,
            kilograms: self.kilograms + rhs.kilograms,
        }
    }
}

impl AddAssign for SI
{
    fn add_assign(&mut self, rhs: SI)
    {
        self.meters += rhs.meters;
        self.seconds += rhs.seconds;
        self.kilograms += rhs.kilograms;        
    }
}

impl Sub for SI
{
    type Output = SI;

    fn sub(self, rhs: SI) -> SI
    {
        return SI
        {
            meters: self.meters - rhs.meters,
            seconds: self.seconds - rhs.seconds,
            kilograms: self.kilograms - rhs.kilograms,
        }
    }
}

impl SubAssign for SI
{
    fn sub_assign(&mut self, rhs: SI)
    {
        self.meters -= rhs.meters;
        self.seconds -= rhs.seconds;
        self.kilograms -= rhs.kilograms;        
    }
}

impl Mul<SI> for f64
{
    type Output = ScalarQuantity;

    fn mul(self, rhs: SI) -> ScalarQuantity
    {
        return ScalarQuantity
        {
            value: self,
            units: rhs
        };
    }
}

//-------------------------------Units-------------------------------//

#[allow(non_camel_case_types)]
pub enum Units
{
    m, pc, kpc,
    s, yr, Myr,
    kg, MSun,
    ms, kms,
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
            Self::m => 1.0 * SI::new(1., 0., 0.),
            // parsec
            Self::pc => 3.086e+16 * SI::new(1., 0., 0.),
            // kiloparsec
            Self::kpc => 3.086e+19 * SI::new(1., 0., 0.),
            // second
            Self::s => 1.0 * SI::new(0., 1., 0.),
            // year
            Self::yr => 365.0 * 86400.0 * SI::new(0., 1., 0.),
            // megayear
            Self::Myr => 1e+6 * 365.0 * 86400.0 * SI::new(0., 1., 0.),
            // kilogram
            Self::kg => 1.0 * SI::new(0., 0., 1.),
            // mass of the sun
            Self::MSun => 1.989e+30 * SI::new(0., 0., 1.),
            // meters per second
            Self::ms => 1. * SI::new(1., 0., -1.),
            // kilometers per second
            Self::kms => 1e+3 * SI::new(1., 0., -1.),
            // joule
            Self::J => 1.0 * SI::new(2., -2., 1.),
            // gravitational constant
            Self::G => 6.67e-11 * SI::new(3., -2., -1.),
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

//-------------------------------ScalarQuantity-------------------------------//

#[derive(PartialEq, Debug)]
pub struct ScalarQuantity
{
    value: f64,
    units: SI
}

impl ScalarQuantity
{
    pub fn new() -> ScalarQuantity
    {
        return ScalarQuantity 
        {
            value: 1.0,
            units: SI::empty()
        };
    }

    pub fn pow(&self, x: f64) -> ScalarQuantity
    {
        return ScalarQuantity
        {
            value: self.value.powf(x),
            units: self.units.pow(x)
        };
    }

    pub fn value_in_q(&self, quantity: ScalarQuantity) -> f64
    {
        if quantity.units != self.units
        {
            panic!("trying to take value in incompatible units");
        }

        return self.value / quantity.value;
    }

    pub fn value_in(&self, unit: Units) -> f64
    {
        return self.value_in_q(unit.convert());
    }

    pub fn is_compatible(&self, quantity: ScalarQuantity) -> bool
    {
        return self.units == quantity.units;
    }
}

impl Display for ScalarQuantity
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        return write!(f, "{:e} m^{} s^{} kg^{}", 
            self.value,
            self.units.meters,
            self.units.seconds,
            self.units.kilograms
        );
    }
}

impl Add for ScalarQuantity
{
    type Output = ScalarQuantity;

    fn add(self, rhs: ScalarQuantity) -> ScalarQuantity 
    { 
        if self.units != rhs.units
        {
            panic!("trying to sum incompatible units");
        }
        
        return ScalarQuantity
        {
            value: self.value + rhs.value,
            units: self.units
        };
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
        if self.units != rhs.units
        {
            panic!("trying to sum incompatible units");
            
        }

        return ScalarQuantity
        {
            value: self.value - rhs.value,
            units: self.units
        };
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
        return ScalarQuantity {
            value: self.value * rhs.value,
            units: self.units + rhs.units
        };
    }
}

impl MulAssign for ScalarQuantity
{
    fn mul_assign(&mut self, rhs: ScalarQuantity)
    {
        self.units += rhs.units;
        self.value *= rhs.value;
    }
}

impl Div for ScalarQuantity
{
    type Output = ScalarQuantity;

    fn div(self, rhs: ScalarQuantity) -> ScalarQuantity
    {
        return ScalarQuantity {
            value: self.value / rhs.value,
            units: self.units - rhs.units
        };
    }
}

impl DivAssign for ScalarQuantity
{
    fn div_assign(&mut self, rhs: ScalarQuantity)
    {
        self.units -= rhs.units;
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

impl PartialOrd for ScalarQuantity
{
    fn partial_cmp(&self, rhs: &ScalarQuantity) -> Option<Ordering>
    {
        if self.units != rhs.units
        {
            panic!("trying to sum incompatible units");
        }

        return self.value.partial_cmp(&rhs.value);
    }
}

//-------------------------------VectorQuantity-------------------------------//

#[derive(PartialEq, Debug)]
pub struct VectorQuantity
{
    value: Vector3,
    units: SI
}

impl VectorQuantity
{
    pub fn from(v: Vector3) -> VectorQuantity
    {
        return VectorQuantity { 
            value: v, 
            units: SI::empty() 
        };
    }

    pub fn mag(self) -> ScalarQuantity
    {
        return ScalarQuantity
        {
            value: self.value.mag(),
            units: self.units
        }
    }

    pub fn value_in_q(&self, quantity: ScalarQuantity) -> Vector3
    {
        if quantity.units != self.units
        {
            panic!("trying to take value in incompatible units");
        }

        return self.value / quantity.value;
    }

    pub fn value_in(&self, unit: Units) -> Vector3
    {
        return self.value_in_q(unit.convert());
    }

    pub fn is_compatible(&self, quantity: ScalarQuantity) -> bool
    {
        return self.units == quantity.units;
    }
}

impl Add for VectorQuantity
{
    type Output = VectorQuantity;

    fn add(self, rhs: VectorQuantity) -> VectorQuantity
    {
        if self.units != rhs.units
        {
            panic!("trying to sum incompatible units");
        }

        return VectorQuantity
        {
            value: self.value + rhs.value,
            units: self.units
        };
    }
}

impl AddAssign for VectorQuantity
{
    fn add_assign(&mut self, rhs: VectorQuantity)
    {
        if self.units != rhs.units
        {
            panic!("trying to sum incompatible units");
        }

        self.value += rhs.value;
    }
}

impl Sub for VectorQuantity
{
    type Output = VectorQuantity;

    fn sub(self, rhs: VectorQuantity) -> VectorQuantity
    {
        if self.units != rhs.units
        {
            panic!("trying to sum incompatible units");
        }

        return VectorQuantity
        {
            value: self.value - rhs.value,
            units: self.units
        };
    }
}

impl SubAssign for VectorQuantity
{
    fn sub_assign(&mut self, rhs: VectorQuantity)
    {
        if self.units != rhs.units
        {
            panic!("trying to sum incompatible units");
        }

        self.value -= rhs.value;
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
        return VectorQuantity {
            value: self.value * rhs.value,
            units: self.units + rhs.units
        };
    }
}

impl MulAssign<ScalarQuantity> for VectorQuantity
{
    fn mul_assign(&mut self, rhs: ScalarQuantity)
    {
        self.value *= rhs.value;
        self.units += rhs.units;
    }
}

impl Div<ScalarQuantity> for VectorQuantity
{
    type Output = VectorQuantity;

    fn div(self, rhs: ScalarQuantity) -> VectorQuantity
    {
        return VectorQuantity {
            value: self.value / rhs.value,
            units: self.units - rhs.units
        };
    }
}

impl DivAssign<ScalarQuantity> for VectorQuantity
{
    fn div_assign(&mut self, rhs: ScalarQuantity)
    {
        self.value /= rhs.value;
        self.units -= rhs.units;
    }
}

impl Mul<f64> for VectorQuantity
{
    type Output = VectorQuantity;

    fn mul(self, rhs: f64) -> VectorQuantity
    {
        return VectorQuantity {
            value: self.value * rhs,
            units: self.units
        }
    }
}

impl MulAssign<f64> for VectorQuantity
{
    fn mul_assign(&mut self, rhs: f64) 
    {
        self.value *= rhs;
    }
}

impl Div<f64> for VectorQuantity
{
    type Output = VectorQuantity;

    fn div(self, rhs: f64) -> VectorQuantity
    {
        return VectorQuantity {
            value: self.value / rhs,
            units: self.units
        }
    }
}

impl DivAssign<f64> for VectorQuantity
{
    fn div_assign(&mut self, rhs: f64) 
    {
        self.value /= rhs;
    }
}

impl Mul<VectorQuantity> for f64
{
    type Output = VectorQuantity;

    fn mul(self, rhs: VectorQuantity) -> VectorQuantity
    {
        return rhs * self;
    }
}