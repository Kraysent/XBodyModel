use crate::vector::Vector3;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result, LowerExp};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

//-------------------------------SI-------------------------------//

#[derive(PartialEq, Debug, Clone, Copy)]
struct SI {
    meters: f64,
    seconds: f64,
    kilograms: f64,
}

impl SI {
    pub fn new(meters: f64, seconds: f64, kilograms: f64) -> SI {
        return SI {
            meters,
            seconds,
            kilograms,
        };
    }

    pub fn empty() -> SI {
        return SI {
            meters: 0.,
            seconds: 0.,
            kilograms: 0.,
        };
    }

    pub fn pow(&self, x: f64) -> SI {
        return SI {
            meters: self.meters * x,
            seconds: self.seconds * x,
            kilograms: self.kilograms * x,
        };
    }
}

impl Display for SI {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(
            f,
            "m^{} s^{} kg^{}",
            self.meters, self.seconds, self.kilograms
        );
    }
}

impl Add for SI {
    type Output = SI;

    fn add(self, rhs: SI) -> SI {
        return SI {
            meters: self.meters + rhs.meters,
            seconds: self.seconds + rhs.seconds,
            kilograms: self.kilograms + rhs.kilograms,
        };
    }
}

impl AddAssign for SI {
    fn add_assign(&mut self, rhs: SI) {
        self.meters += rhs.meters;
        self.seconds += rhs.seconds;
        self.kilograms += rhs.kilograms;
    }
}

impl Sub for SI {
    type Output = SI;

    fn sub(self, rhs: SI) -> SI {
        return SI {
            meters: self.meters - rhs.meters,
            seconds: self.seconds - rhs.seconds,
            kilograms: self.kilograms - rhs.kilograms,
        };
    }
}

impl SubAssign for SI {
    fn sub_assign(&mut self, rhs: SI) {
        self.meters -= rhs.meters;
        self.seconds -= rhs.seconds;
        self.kilograms -= rhs.kilograms;
    }
}

impl Mul<SI> for f64 {
    type Output = ScalarQuantity;

    fn mul(self, rhs: SI) -> ScalarQuantity {
        return ScalarQuantity {
            value: self,
            units: rhs,
        };
    }
}

//-------------------------------Units-------------------------------//

#[allow(non_camel_case_types)]
pub enum Units {
    m,      // meter
    pc,     // parsec
    kpc,    // kiloparsec
    s,      // second
    yr,     // year
    Myr,    // megayear
    kg,     // kilogram
    MSun,   // mass of the sun
    ms,     // meters per second
    kms,    // kilometers per second
    J,      // joules
    G,      // gravitational constant
}

impl Units {
    pub fn convert(&self) -> ScalarQuantity {
        match self {
            Self::m => 1.0 * SI::new(1., 0., 0.),
            Self::pc => 3.086e+16 * SI::new(1., 0., 0.),
            Self::kpc => 3.086e+19 * SI::new(1., 0., 0.),
            Self::s => 1.0 * SI::new(0., 1., 0.),
            Self::yr => 365.0 * 86400.0 * SI::new(0., 1., 0.),
            Self::Myr => 1e+6 * 365.0 * 86400.0 * SI::new(0., 1., 0.),
            Self::kg => 1.0 * SI::new(0., 0., 1.),
            Self::MSun => 1.989e+30 * SI::new(0., 0., 1.),
            Self::ms => 1. * SI::new(1., -1., 0.),
            Self::kms => 1e+3 * SI::new(1., 0., -1.),
            Self::J => 1.0 * SI::new(2., -2., 1.),
            Self::G => 6.67e-11 * SI::new(3., -2., -1.),
        }
    }

    pub fn pow(&self, x: f64) -> ScalarQuantity {
        return self.convert().pow(x);
    }
}

impl Mul<f64> for Units {
    type Output = ScalarQuantity;

    fn mul(self, rhs: f64) -> ScalarQuantity {
        return ScalarQuantity::new() * self * rhs;
    }
}

impl Div<f64> for Units {
    type Output = ScalarQuantity;

    fn div(self, rhs: f64) -> ScalarQuantity {
        return ScalarQuantity::new() * self / rhs;
    }
}

impl Mul<Vector3> for Units {
    type Output = VectorQuantity;

    fn mul(self, rhs: Vector3) -> VectorQuantity {
        return VectorQuantity::from(rhs) * self;
    }
}

impl Mul<Units> for f64 {
    type Output = ScalarQuantity;

    fn mul(self, rhs: Units) -> ScalarQuantity {
        return self * rhs.convert();
    }
}

impl Mul<Units> for Vector3 {
    type Output = VectorQuantity;

    fn mul(self, rhs: Units) -> VectorQuantity {
        return VectorQuantity::from(self) * rhs;
    }
}

impl Div<Units> for Vector3 {
    type Output = VectorQuantity;

    fn div(self, rhs: Units) -> VectorQuantity {
        return VectorQuantity::from(self) / rhs;
    }
}

//-------------------------------Quantity-------------------------------//

pub trait QuantityBound: 
    Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign 
    + Mul<f64, Output = Self> + MulAssign<f64> + Div<f64, Output = Self> + DivAssign<f64> 
    + PartialEq + Clone + Copy + Display + LowerExp
    where Self: Sized { }

impl<T> QuantityBound for T 
    where T: Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign 
    + Mul<f64, Output = Self> + MulAssign<f64> + Div<f64, Output = Self> + DivAssign<f64> 
    + PartialEq + Clone + Copy + Display + LowerExp { }

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Quantity<T> 
where T: QuantityBound {
    value: T,
    units: SI
}

pub type ScalarQuantity = Quantity<f64>;
pub type VectorQuantity = Quantity<Vector3>;

impl<T: QuantityBound> Quantity<T>{
    pub fn value_in_q(&self, q: Quantity<f64>) -> T {
        if self.units != q.units {
            panic!("trying to take value in incompatible units");
        }

        return self.value / q.value;
    }

    pub fn is_compatible<V: QuantityBound>(&self, q: Quantity<V>) -> bool {
        return self.units == q.units;
    }

    pub fn value_in(&self, unit: Units) -> T {
        return self.value_in_q(unit.convert());
    }
}

impl<T: QuantityBound> Display for Quantity<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(
            f,
            "{} {}",
            self.value, self.units
        );
    }
}

impl<T: QuantityBound> LowerExp for Quantity<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(
            f,
            "{:e} {}",
            self.value, self.units
        );
    }
}

impl<T: QuantityBound> Add for Quantity<T> {
    type Output = Quantity<T>;

    fn add(self, rhs: Quantity<T>) -> Quantity<T> {
        if self.units != rhs.units {
            panic!("trying to sum incompatible units");
        }

        return Quantity {
            value: self.value + rhs.value,
            units: rhs.units
        }
    }
}

impl<T: QuantityBound> AddAssign for Quantity<T> {
    fn add_assign(&mut self, rhs: Quantity<T>) {
        if self.units != rhs.units {
            panic!("trying to sum incompatible units");
        }

        self.value += rhs.value;
    }
}

impl<T: QuantityBound> Sub for Quantity<T> {
    type Output = Quantity<T>;

    fn sub(self, rhs: Quantity<T>) -> Quantity<T> {
        if self.units != rhs.units {
            panic!("trying to sum incompatible units");
        }

        return Quantity {
            value: self.value - rhs.value,
            units: rhs.units
        }
    }
}

impl<T: QuantityBound> SubAssign for Quantity<T> {
    fn sub_assign(&mut self, rhs: Quantity<T>) {
        if self.units != rhs.units {
            panic!("trying to sum incompatible units");
        }
        
        self.value -= rhs.value;
    }
}

impl<T: QuantityBound> Mul<f64> for Quantity<T> {
    type Output = Quantity<T>;

    fn mul(self, rhs: f64) -> Quantity<T> {
        return Quantity {
            value: self.value * rhs,
            units: self.units
        }
    }
}

impl<T: QuantityBound> MulAssign<f64> for Quantity<T> {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}

impl<T: QuantityBound> Div<f64> for Quantity<T> {
    type Output = Quantity<T>;

    fn div(self, rhs: f64) -> Quantity<T> {
        return Quantity {
            value: self.value / rhs,
            units: self.units
        }
    }
}

impl<T: QuantityBound> DivAssign<f64> for Quantity<T> {
    fn div_assign(&mut self, rhs: f64) {
        self.value /= rhs;
    }
}

impl<T: QuantityBound> Mul<ScalarQuantity> for Quantity<T> {
    type Output = Quantity<T>;

    fn mul(self, rhs: Quantity<f64>) -> Quantity<T> {
        return Quantity {
            value: self.value * rhs.value,
            units: self.units + rhs.units
        }
    }
}

impl<T: QuantityBound> MulAssign<ScalarQuantity> for Quantity<T> {
    fn mul_assign(&mut self, rhs: Quantity<f64>) {
        self.value *= rhs.value;
        self.units += rhs.units;
    }
}

impl<T: QuantityBound> Div<ScalarQuantity> for Quantity<T> {
    type Output = Quantity<T>;

    fn div(self, rhs: Quantity<f64>) -> Quantity<T> {
        return Quantity {
            value: self.value / rhs.value,
            units: self.units - rhs.units
        }
    }
}

impl<T: QuantityBound> DivAssign<ScalarQuantity> for Quantity<T> {
    fn div_assign(&mut self, rhs: Quantity<f64>) {
        self.value /= rhs.value;
        self.units -= rhs.units;
    }
}

impl<T: QuantityBound> Mul<Units> for Quantity<T> {
    type Output = Quantity<T>;

    fn mul(self, rhs: Units) -> Quantity<T> {
        return self * rhs.convert();
    }
}

impl<T: QuantityBound> MulAssign<Units> for Quantity<T> {
    fn mul_assign(&mut self, rhs: Units) {
        *self *= rhs.convert();
    }
}

impl<T: QuantityBound> Div<Units> for Quantity<T> {
    type Output = Quantity<T>;

    fn div(self, rhs: Units) -> Quantity<T> {
        return self / rhs.convert();
    }
}

impl<T: QuantityBound> DivAssign<Units> for Quantity<T> {
    fn div_assign(&mut self, rhs: Units) {
        *self /= rhs.convert();
    }
}

impl<T: QuantityBound> Mul<Quantity<T>> for f64 {
    type Output = Quantity<T>;

    fn mul(self, rhs: Quantity<T>) -> Quantity<T> {
        return Quantity {
            value: rhs.value * self,
            units: rhs.units
        }
    }
}

impl<T: QuantityBound> Mul<Quantity<T>> for Units {
    type Output = Quantity<T>;

    fn mul(self, rhs: Quantity<T>) -> Quantity<T> {
        return rhs * self;
    }
}

impl<T: QuantityBound + PartialOrd> PartialOrd for Quantity<T> {
    fn partial_cmp(&self, rhs: &Quantity<T>) -> Option<Ordering> {
        if self.units != rhs.units {
            panic!("trying to compare incompatible units");
        }

        return self.value.partial_cmp(&rhs.value);
    }
}

//-------------------------------ScalarQuantity-------------------------------//

impl ScalarQuantity {
    pub fn new() -> ScalarQuantity {
        return ScalarQuantity {
            value: 1.0,
            units: SI::empty(),
        };
    }

    pub fn pow(&self, x: f64) -> ScalarQuantity {
        return ScalarQuantity {
            value: self.value.powf(x),
            units: self.units.pow(x),
        };
    }
}

impl Mul<Vector3> for ScalarQuantity {
    type Output = VectorQuantity;

    fn mul(self, rhs: Vector3) -> VectorQuantity {
        return VectorQuantity::from(rhs) * self;
    }
}

impl Mul<VectorQuantity> for ScalarQuantity {
    type Output = VectorQuantity;

    fn mul(self, rhs: VectorQuantity) -> VectorQuantity {
        return rhs * self;
    }
}

impl Div<ScalarQuantity> for f64 {
    type Output = ScalarQuantity;

    fn div(self, rhs: ScalarQuantity) -> ScalarQuantity {
        return (ScalarQuantity::new() * self) / rhs;
    }
}

impl Mul<ScalarQuantity> for Vector3 {
    type Output = VectorQuantity;

    fn mul(self, rhs: ScalarQuantity) -> VectorQuantity {
        return VectorQuantity::from(self) * rhs;
    }
}

impl Div<ScalarQuantity> for Vector3 {
    type Output = VectorQuantity;

    fn div(self, rhs: ScalarQuantity) -> VectorQuantity {
        return VectorQuantity::from(self) / rhs;
    }
}

//-------------------------------VectorQuantity-------------------------------//

impl VectorQuantity {
    pub fn from(v: Vector3) -> VectorQuantity {
        return VectorQuantity {
            value: v,
            units: SI::empty(),
        };
    }

    pub fn mag(self) -> ScalarQuantity {
        return ScalarQuantity {
            value: self.value.mag(),
            units: self.units,
        };
    }
}
