use std::ops::{ Mul, Div, Add, Sub };
use std::cmp::{ Eq, PartialEq };
use std::collections::HashMap;

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

impl Mul<f64> for Unit
{
    type Output = Quantity;

    fn mul(self, rhs: f64) -> Quantity
    {
        return Quantity::new() * self * rhs;
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

impl Mul<Quantity> for Unit
{
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Quantity
    { 
        return rhs * self;
    }
}

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

impl Mul<f64> for ComplexUnit
{
    type Output = Quantity;

    fn mul(self, rhs: f64) -> Quantity
    {
        return self.convert() * rhs;
    }
}

impl Mul<ComplexUnit> for ComplexUnit
{
    type Output = Quantity;

    fn mul(self, rhs: ComplexUnit) -> Quantity
    {
        return self.convert() * rhs;
    }
}

impl Mul<Quantity> for ComplexUnit
{
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Quantity
    {
        return self.convert() * rhs;
    }
}

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

impl Add for Quantity
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

impl Sub for Quantity
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

impl Mul<Unit> for Quantity
{
    type Output = Quantity;

    fn mul(self, rhs: Unit) -> Quantity
    {
        let mut rhs_q = Quantity::new();
        rhs_q.value = 1.0;
        rhs_q.units.insert(rhs, 1);

        return self * rhs_q;
    }
}

impl Mul<ComplexUnit> for Quantity
{
    type Output = Quantity;

    fn mul(self, rhs: ComplexUnit) -> Quantity
    {
        return rhs * self;
    }
}

impl Mul<Quantity> for Quantity
{
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Quantity
    {
        let mut units = HashMap::new();

        units.insert(Unit::Meter, self.units[&Unit::Meter] + rhs.units[&Unit::Meter]);
        units.insert(Unit::Second, self.units[&Unit::Second] + rhs.units[&Unit::Second]);
        units.insert(Unit::Kilogram, self.units[&Unit::Kilogram] + rhs.units[&Unit::Kilogram]);

        return Quantity
        {
            value: self.value * rhs.value,
            units
        };
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

impl Div<Quantity> for Quantity
{
    type Output = Quantity;

    fn div(self, rhs: Quantity) -> Quantity
    {
        let mut units = HashMap::new();

        units.insert(Unit::Meter, self.units[&Unit::Meter] - rhs.units[&Unit::Meter]);
        units.insert(Unit::Second, self.units[&Unit::Second] - rhs.units[&Unit::Second]);
        units.insert(Unit::Kilogram, self.units[&Unit::Kilogram] - rhs.units[&Unit::Kilogram]);

        return Quantity
        {
            value: self.value / rhs.value,
            units
        };
    }
}

impl Div<Unit> for Quantity
{
    type Output = Quantity;

    fn div(self, rhs: Unit) -> Quantity
    {
        let mut rhs_q = Quantity::new();
        rhs_q.value = 1.0;
        rhs_q.units.insert(rhs, 1);

        return self / rhs_q;
    }
}
