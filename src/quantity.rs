use std::ops::{ Mul, Div, Add, Sub };
use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::vector::Vector3;

//-------------------------------Unit-------------------------------//

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Unit
{
    Meter,
    Second,
    Kilogram
}

impl Unit
{
    pub fn pow(self, x: i32) -> Quantity
    {
        let mut a = Quantity::new();
        a.units.insert(self, x);

        return a;
    }
}

impl Add<Unit> for Unit
{
    type Output = Quantity;

    fn add(self, rhs: Unit) -> Quantity
    {
        return Quantity::new() * self + Quantity::new() * rhs;
    }
}

impl Sub<Unit> for Unit
{
    type Output = Quantity;

    fn sub(self, rhs: Unit) -> Quantity
    {
        return Quantity::new() * self - Quantity::new() * rhs;
    }
}

impl Add<ComplexUnit> for Unit
{
    type Output = Quantity;

    fn add(self, rhs: ComplexUnit) -> Quantity
    {
        return Quantity::new() * self + Quantity::new() * rhs;
    }
}

impl Sub<ComplexUnit> for Unit
{
    type Output = Quantity;

    fn sub(self, rhs: ComplexUnit) -> Quantity
    {
        return Quantity::new() * self - Quantity::new() * rhs;
    }
}

impl Add<Quantity> for Unit
{
    type Output = Quantity;

    fn add(self, rhs: Quantity) -> Quantity
    {
        return Quantity::new() * self + rhs;
    }
}

impl Sub<Quantity> for Unit
{
    type Output = Quantity;

    fn sub(self, rhs: Quantity) -> Quantity
    {
        return Quantity::new() * self - rhs;
    }
}

impl Mul<Unit> for Unit
{
    type Output = Quantity;

    fn mul(self, rhs: Unit) -> Quantity
    { 
        return Quantity::new() * rhs * self;
    }
}

impl Div<Unit> for Unit
{
    type Output = Quantity;

    fn div(self, rhs: Unit) -> Quantity
    {
        return Quantity::new() * self / rhs;
    }
}

impl Mul<ComplexUnit> for Unit
{
    type Output = Quantity;

    fn mul(self, rhs: ComplexUnit) -> Quantity
    {
        return Quantity::new() * self * rhs;
    }
}

impl Div<ComplexUnit> for Unit
{
    type Output = Quantity;

    fn div(self, rhs: ComplexUnit) -> Quantity
    {
        return Quantity::new() * self / rhs;
    }
}

impl Mul<Quantity> for Unit
{
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Quantity
    { 
        return Quantity::new() * self * rhs;
    }
}

impl Div<Quantity> for Unit
{
    type Output = Quantity;

    fn div(self, rhs: Quantity) -> Quantity
    { 
        return Quantity::new() * self / rhs;
    }
}

impl Mul<VectorQuantity> for Unit
{
    type Output = VectorQuantity;

    fn mul(self, rhs: VectorQuantity) -> VectorQuantity
    {
        return rhs * self;
    }
}

impl Mul<f64> for Unit
{
    type Output = Quantity;

    fn mul(self, rhs: f64) -> Quantity
    {
        return Quantity::new() * self * rhs;
    }
}

impl Div<f64> for Unit
{
    type Output = Quantity;

    fn div(self, rhs: f64) -> Quantity
    {
        return Quantity::new() * self / rhs;
    }
}

impl Mul<Vector3> for Unit
{
    type Output = VectorQuantity;

    fn mul(self, rhs: Vector3) -> VectorQuantity
    {
        return VectorQuantity::from(rhs) * self;
    }
}

//-------------------------------ComplexUnit-------------------------------//

#[allow(non_camel_case_types)]
pub enum ComplexUnit
{
    m, pc, kpc,
    s, yr, Myr,
    kg, MSun,
    J
}

impl ComplexUnit
{
    pub fn convert(&self) -> Quantity
    {
        match self
        {
            Self::m => Unit::Meter * 1.0,
            Self::pc => Unit::Meter * 3.086e+16,
            Self::kpc => Unit::Meter * 3.086e+19,
            Self::s => Unit::Second * 1.0,
            Self::yr => Unit::Second * 365.0 * 86400.0,
            Self::Myr => Unit::Second * 1e+6 * 365.0 * 86400.0,
            Self::kg => Unit::Kilogram * 1.0,
            Self::MSun => Unit::Kilogram * 1.989e+30,
            Self::J => Unit::Kilogram * Unit::Meter.pow(2) * Unit::Second.pow(-2) * 1.0
        }
    }

    pub fn pow(&self, x: i32) -> Quantity
    {
        return self.convert().pow(x);
    }
}

impl Add<Unit> for ComplexUnit
{
    type Output = Quantity;

    fn add(self, rhs: Unit) -> Quantity
    {
        return Quantity::new() * self + Quantity::new() * rhs; 
    }
}

impl Sub<Unit> for ComplexUnit
{
    type Output = Quantity;

    fn sub(self, rhs: Unit) -> Quantity
    {
        return Quantity::new() * self - Quantity::new() * rhs; 
    }
}

impl Add<ComplexUnit> for ComplexUnit
{
    type Output = Quantity;

    fn add(self, rhs: ComplexUnit) -> Quantity
    {
        return Quantity::new() * self + Quantity::new() * rhs; 
    }
}

impl Sub<ComplexUnit> for ComplexUnit
{
    type Output = Quantity;

    fn sub(self, rhs: ComplexUnit) -> Quantity
    {
        return Quantity::new() * self - Quantity::new() * rhs; 
    }
}

impl Add<Quantity> for ComplexUnit
{
    type Output = Quantity;

    fn add(self, rhs: Quantity) -> Quantity
    {
        return Quantity::new() * self + rhs; 
    }
}

impl Sub<Quantity> for ComplexUnit
{
    type Output = Quantity;

    fn sub(self, rhs: Quantity) -> Quantity
    {
        return Quantity::new() * self - rhs; 
    }
}

impl Mul<Unit> for ComplexUnit
{
    type Output = Quantity;

    fn mul(self, rhs: Unit) -> Quantity
    {
        return Quantity::new() * self * rhs;
    }
}

impl Div<Unit> for ComplexUnit
{
    type Output = Quantity;

    fn div(self, rhs: Unit) -> Quantity
    {
        return Quantity::new() * self / rhs;
    }
}

impl Mul<ComplexUnit> for ComplexUnit
{
    type Output = Quantity;

    fn mul(self, rhs: ComplexUnit) -> Quantity
    {
        return Quantity::new() * self * rhs;
    }
}

impl Div<ComplexUnit> for ComplexUnit
{
    type Output = Quantity;

    fn div(self, rhs: ComplexUnit) -> Quantity
    {
        return Quantity::new() * self / rhs;
    }
}

impl Mul<Quantity> for ComplexUnit
{
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Quantity
    {
        return Quantity::new() * self * rhs;
    }
}

impl Div<Quantity> for ComplexUnit
{
    type Output = Quantity;

    fn div(self, rhs: Quantity) -> Quantity
    {
        return Quantity::new() * self / rhs;
    }
}

impl Mul<VectorQuantity> for ComplexUnit
{
    type Output = VectorQuantity;

    fn mul(self, rhs: VectorQuantity) -> VectorQuantity
    {
        return rhs * self;
    }
}

impl Mul<f64> for ComplexUnit
{
    type Output = Quantity;

    fn mul(self, rhs: f64) -> Quantity
    {
        return Quantity::new() * self * rhs;
    }
}

impl Div<f64> for ComplexUnit
{
    type Output = Quantity;

    fn div(self, rhs: f64) -> Quantity
    {
        return Quantity::new() * self / rhs;
    }
}

impl Mul<Vector3> for ComplexUnit
{
    type Output = VectorQuantity;

    fn mul(self, rhs: Vector3) -> VectorQuantity
    {
        return VectorQuantity::from(rhs) * self;
    }
}

//-------------------------------Quantity-------------------------------//

#[derive(Debug)]
pub struct Quantity
{
    pub value: f64,
    pub units: HashMap<Unit, i32>
}

impl Quantity
{
    pub fn new() -> Quantity
    {
        let mut units: HashMap<Unit, i32> = HashMap::new();
        units.insert(Unit::Meter, 0);
        units.insert(Unit::Second, 0);
        units.insert(Unit::Kilogram, 0);

        return Quantity 
        {
            value: 1.0,
            units
        };
    }

    pub fn pow(&self, x: i32) -> Quantity
    {
        let mut res = Quantity::new();

        res.units.insert(Unit::Meter, self.units[&Unit::Meter] * x);
        res.units.insert(Unit::Second, self.units[&Unit::Second] * x);
        res.units.insert(Unit::Kilogram, self.units[&Unit::Kilogram] * x);

        res = res * self.value.powi(x);

        return res;
    }

    pub fn value_in(&self, quantity: Quantity) -> f64
    {
        if quantity.units != self.units
        {
            panic!("trying to take value in incompatible units");
        }

        return self.value / quantity.value;
    }
}

impl PartialEq for Quantity
{
    fn eq(&self, rhs: &Quantity) -> bool 
    { 
        return (self.value == rhs.value) &&
               (self.units == rhs.units); 
    }
}

impl Add<Unit> for Quantity
{
    type Output = Quantity;

    fn add(self, rhs: Unit) -> Quantity
    {
        return self + Quantity::new() * rhs;
    }
}

impl Sub<Unit> for Quantity
{
    type Output = Quantity;

    fn sub(self, rhs: Unit) -> Quantity
    {
        return self - Quantity::new() * rhs;
    }
}

impl Add<ComplexUnit> for Quantity
{
    type Output = Quantity;

    fn add(self, rhs: ComplexUnit) -> Quantity
    {
        return self + Quantity::new() * rhs;
    }
}

impl Sub<ComplexUnit> for Quantity
{
    type Output = Quantity;

    fn sub(self, rhs: ComplexUnit) -> Quantity
    {
        return self - Quantity::new() * rhs;
    }
}

impl Add<Quantity> for Quantity
{
    type Output = Quantity;

    fn add(self, rhs: Quantity) -> Quantity 
    { 
        if self.units == rhs.units
        {
            return Quantity
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

impl Sub<Quantity> for Quantity
{
    type Output = Quantity;

    fn sub(self, rhs: Quantity) -> Quantity 
    { 
        if self.units == rhs.units
        {
            return Quantity
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

impl Mul<Unit> for Quantity
{
    type Output = Quantity;

    fn mul(self, rhs: Unit) -> Quantity
    {
        let mut rhs_q = Quantity::new();
        rhs_q.units.insert(rhs, 1);

        return self * rhs_q;
    }
}

impl Div<Unit> for Quantity
{
    type Output = Quantity;

    fn div(self, rhs: Unit) -> Quantity
    {
        let mut rhs_q = Quantity::new();
        rhs_q.units.insert(rhs, 1);

        return self / rhs_q;
    }
}

impl Mul<ComplexUnit> for Quantity
{
    type Output = Quantity;

    fn mul(self, rhs: ComplexUnit) -> Quantity
    {
        return self * rhs.convert();
    }
}

impl Div<ComplexUnit> for Quantity
{
    type Output = Quantity;

    fn div(self, rhs: ComplexUnit) -> Quantity
    {
        return self / (Quantity::new() * rhs);
    }
}

impl Mul<Quantity> for Quantity
{
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Quantity
    {
        let mut res = Quantity::new();

        res.units.insert(Unit::Meter, self.units[&Unit::Meter] + rhs.units[&Unit::Meter]);
        res.units.insert(Unit::Second, self.units[&Unit::Second] + rhs.units[&Unit::Second]);
        res.units.insert(Unit::Kilogram, self.units[&Unit::Kilogram] + rhs.units[&Unit::Kilogram]);
        res.value = self.value * rhs.value;

        return res;
    }
}

impl Div<Quantity> for Quantity
{
    type Output = Quantity;

    fn div(self, rhs: Quantity) -> Quantity
    {
        let mut res = Quantity::new();

        res.units.insert(Unit::Meter, self.units[&Unit::Meter] - rhs.units[&Unit::Meter]);
        res.units.insert(Unit::Second, self.units[&Unit::Second] - rhs.units[&Unit::Second]);
        res.units.insert(Unit::Kilogram, self.units[&Unit::Kilogram] - rhs.units[&Unit::Kilogram]);
        res.value = self.value / rhs.value;

        return res;
    }
}

impl Mul<VectorQuantity> for Quantity
{
    type Output = VectorQuantity;

    fn mul(self, rhs: VectorQuantity) -> VectorQuantity
    {
        return rhs * self;
    }
}

impl Mul<f64> for Quantity
{
    type Output = Quantity;

    fn mul(self, rhs: f64) -> Quantity
    {
        return Quantity
        {
            value: self.value * rhs,
            units: self.units
        }
    }
}

impl Div<f64> for Quantity
{
    type Output = Quantity;

    fn div(self, rhs: f64) -> Quantity
    {
        return Quantity
        {
            value: self.value / rhs,
            units: self.units
        }
    }
}

impl Mul<Vector3> for Quantity
{
    type Output = VectorQuantity;

    fn mul(self, rhs: Vector3) -> VectorQuantity
    {
        return VectorQuantity::from(rhs) * self;
    }
}

//-------------------------------VectorQuantity-------------------------------//

#[derive(Debug, PartialEq)]
pub struct VectorQuantity
{
    pub value: Vector3,
    pub units: HashMap<Unit, i32>
}

impl VectorQuantity
{
    pub fn from(v: Vector3) -> VectorQuantity
    {
        let mut units = HashMap::new();

        units.insert(Unit::Meter, 0);
        units.insert(Unit::Second, 0);
        units.insert(Unit::Kilogram, 0);

        return VectorQuantity { value: v, units }
    }

    pub fn mag(self) -> Quantity
    {
        return Quantity
        {
            value: self.value.mag(),
            units: self.units
        }
    }
}

impl Add<VectorQuantity> for VectorQuantity
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

impl Sub<VectorQuantity> for VectorQuantity
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

impl Mul<Unit> for VectorQuantity
{
    type Output = VectorQuantity;

    fn mul(self, rhs: Unit) -> VectorQuantity
    {
        return self * (Quantity::new() * rhs);
    }
}

impl Div<Unit> for VectorQuantity
{
    type Output = VectorQuantity;

    fn div(self, rhs: Unit) -> VectorQuantity
    {
        return self / (Quantity::new() * rhs);
    }
}

impl Mul<ComplexUnit> for VectorQuantity
{
    type Output = VectorQuantity;

    fn mul(self, rhs: ComplexUnit) -> VectorQuantity
    {
        return self * (Quantity::new() * rhs);
    }
}

impl Div<ComplexUnit> for VectorQuantity
{
    type Output = VectorQuantity;

    fn div(self, rhs: ComplexUnit) -> VectorQuantity
    {
        return self / (Quantity::new() * rhs);
    }
}

impl Mul<Quantity> for VectorQuantity
{
    type Output = VectorQuantity;

    fn mul(self, rhs: Quantity) -> VectorQuantity
    {
        let mut units = HashMap::new();
        units.insert(Unit::Meter, self.units[&Unit::Meter] + rhs.units[&Unit::Meter]);
        units.insert(Unit::Second, self.units[&Unit::Second] + rhs.units[&Unit::Second]);
        units.insert(Unit::Kilogram, self.units[&Unit::Kilogram] + rhs.units[&Unit::Kilogram]);

        return VectorQuantity
        {
            value: self.value * rhs.value,
            units
        }
    }
}

impl Div<Quantity> for VectorQuantity
{
    type Output = VectorQuantity;

    fn div(self, rhs: Quantity) -> VectorQuantity
    {
        let mut units = HashMap::new();
        units.insert(Unit::Meter, self.units[&Unit::Meter] - rhs.units[&Unit::Meter]);
        units.insert(Unit::Second, self.units[&Unit::Second] - rhs.units[&Unit::Second]);
        units.insert(Unit::Kilogram, self.units[&Unit::Kilogram] - rhs.units[&Unit::Kilogram]);

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
