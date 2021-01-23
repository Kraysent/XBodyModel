use nbody::quantity::{ Unit, ComplexUnit, Quantity, VectorQuantity };
use nbody::vector::Vector3;

#[test]
fn complex_unit_add_compatible_unit()
{
    let a = ComplexUnit::m;
    let b = Unit::Meter;

    let actual = a + b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 2.0;
    
    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn complex_unit_add_incompatible_unit()
{
    let a = ComplexUnit::pc;
    let b = Unit::Second;

    let _actual = a + b;
}

#[test]
fn complex_unit_sub_compatible_unit()
{
    let a = ComplexUnit::m;
    let b = Unit::Meter;

    let actual = a - b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 0.0;
    
    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn complex_unit_sub_incompatible_unit()
{
    let a = ComplexUnit::pc;
    let b = Unit::Second;

    let _actual = a - b;
}

#[test]
fn complex_unit_add_compatible_complex_unit()
{
    let a = ComplexUnit::m;
    let b = ComplexUnit::pc;

    let actual = a + b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 3.086e+16 + 1.0;
    
    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn complex_unit_add_incompatible_complex_unit()
{
    let a = ComplexUnit::pc;
    let b = ComplexUnit::s;

    let _actual = a + b;
}

#[test]
fn complex_unit_sub_compatible_complex_unit()
{
    let a = ComplexUnit::kpc;
    let b = ComplexUnit::pc;

    let actual = a - b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 3.086e+19 - 3.086e+16;
    
    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn complex_unit_sub_incompatible_complex_unit()
{
    let a = ComplexUnit::pc;
    let b = ComplexUnit::MSun;

    let _actual = a - b;
}

#[test]
fn complex_unit_add_compatible_quantity()
{
    let a = ComplexUnit::m;
    let mut b = Quantity::new();
    b.units.insert(Unit::Meter, 1);
    b.value = 2.35;

    let actual = a + b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 3.35;
    
    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn complex_unit_add_incompatible_quantity()
{
    let a = ComplexUnit::pc;
    let mut b = Quantity::new();
    b.units.insert(Unit::Second, 1);
    b.value = 5.5;

    let _actual = a + b;
}

#[test]
fn complex_unit_sub_compatible_quantity()
{
    let a = ComplexUnit::m;
    let mut b = Quantity::new();
    b.units.insert(Unit::Meter, 1);
    b.value = 2.35;

    let actual = a - b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 1.0 - 2.35;
    
    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn complex_unit_sub_incompatible_quantity()
{
    let a = ComplexUnit::pc;
    let mut b = Quantity::new();
    b.units.insert(Unit::Second, 1);
    b.value = 5.5;

    let _actual = a - b;
}

#[test]
fn complex_unit_mul_unit()
{
    let a = ComplexUnit::m;
    let b = Unit::Second;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Second, 1);
    
    assert_eq!(actual, expected);
}

#[test]
fn complex_unit_div_unit()
{
    let a = ComplexUnit::m;
    let b = Unit::Second;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Second, -1);
    
    assert_eq!(actual, expected);
}

#[test]
fn complex_unit_mul_complex_unit()
{
    let a = ComplexUnit::m;
    let b = ComplexUnit::MSun;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Kilogram, 1);
    expected.value = 1.989e+30;

    assert_eq!(actual, expected);
}

#[test]
fn complex_unit_div_complex_unit()
{
    let a = ComplexUnit::MSun;
    let b = ComplexUnit::m;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, -1);
    expected.units.insert(Unit::Kilogram, 1);
    expected.value = 1.989e+30;

    assert_eq!(actual, expected);
}

#[test]
fn complex_unit_mul_quantity()
{
    let a = ComplexUnit::m;
    let mut b = Quantity::new();
    b.units.insert(Unit::Second, 2);
    b.value = 2.6;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Second, 2);
    expected.units.insert(Unit::Meter, 1);
    expected.value = 2.6;

    assert_eq!(actual, expected);
}

#[test]
fn complex_unit_div_quantity()
{
    let a = ComplexUnit::m;
    let mut b = Quantity::new();
    b.units.insert(Unit::Second, 2);
    b.value = 2.6;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Second, -2);
    expected.units.insert(Unit::Meter, 1);
    expected.value = 1.0 / 2.6;

    assert_eq!(actual, expected);
}

#[test]
fn complex_unit_mul_f64()
{
    let a = ComplexUnit::m;
    let b = 6.5;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 6.5;

    assert_eq!(actual, expected);
}

#[test]
fn complex_unit_div_f64()
{
    let a = ComplexUnit::m;
    let b = 6.5;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 1.0 / 6.5;

    assert_eq!(actual, expected);
}

#[test]
fn complex_unit_mul_vector_quantity()
{
    let a = ComplexUnit::s;
    let v = Vector3::new(1.0, 2.0, 3.0);
    let mut b = VectorQuantity::from(v);
    b.units.insert(Unit::Kilogram, 2);

    let actual = a * b;
    let mut expected = VectorQuantity::from(v);
    expected.units.insert(Unit::Kilogram, 2);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
fn complex_unit_mul_vector()
{
    let a = ComplexUnit::s;
    let b = Vector3::new(1.0, 2.0, 3.0);

    let actual = a * b;
    let mut expected = VectorQuantity::from(b);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}