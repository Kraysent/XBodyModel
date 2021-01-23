use nbody::quantity::{ Unit, ComplexUnit, Quantity, VectorQuantity };
use nbody::vector::Vector3;

#[test]
fn quantity_add_compatible_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 1);
    a.value = 2.0;
    let b = Unit::Meter;

    let actual = a + b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 3.0;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn quantity_add_incompatible_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 2);
    a.value = 2.0;
    let b = Unit::Second;

    let _actual = a + b;
}

#[test]
fn quantity_sub_compatible_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 1);
    a.value = 2.0;
    let b = Unit::Meter;

    let actual = a - b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 1.0;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn quantity_sub_incompatible_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 2);
    a.value = 2.0;
    let b = Unit::Kilogram;

    let _actual = a - b;
}

#[test]
fn quantity_add_compatible_complex_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 1);
    a.value = 2.0;
    let b = ComplexUnit::m;

    let actual = a + b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 3.0;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn quantity_add_incompatible_complex_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 2);
    a.value = 2.0;
    let b = ComplexUnit::m;

    let _actual = a + b;
}

#[test]
fn quantity_sub_compatible_complex_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 1);
    a.value = 2.0;
    let b = ComplexUnit::m;

    let actual = a - b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 1.0;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn quantity_sub_incompatible_complex_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 2);
    a.value = 2.0;
    let b = ComplexUnit::kg;

    let _actual = a - b;
}

#[test]
fn quantity_add_compatible_quantity()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 2);
    a.value = 2.0;
    let mut b = Quantity::new();
    b.units.insert(Unit::Meter, 2);
    b.value = 3.5;

    let actual = a + b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 2);
    expected.value = 5.5;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn quantity_add_incompatible_quantity()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 2);
    a.value = 2.0;
    let mut b = Quantity::new();
    b.units.insert(Unit::Meter, 1);
    b.value = -1.0;

    let _actual = a + b;
}

#[test]
fn quantity_sub_compatible_quantity()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 1);
    a.value = 2.0;
    let mut b = Quantity::new();
    b.units.insert(Unit::Meter, 1);
    b.value = 5.0;

    let actual = a - b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = -3.0;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn quantity_sub_incompatible_quantity()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 2);
    a.value = 2.0;
    let mut b = Quantity::new();
    b.units.insert(Unit::Second, 1);
    b.value = 8.0;

    let _actual = a - b;
}

#[test]
fn quantity_mul_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 1);
    a.value = 2.0;
    let b = Unit::Second;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Second, 1);
    expected.value = 2.0;

    assert_eq!(actual, expected);
}

#[test]
fn quantity_div_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 1);
    a.value = 2.0;
    let b = Unit::Second;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Second, -1);
    expected.value = 2.0;

    assert_eq!(actual, expected);
}

#[test]
fn quantity_mul_complex_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 1);
    a.value = 2.0;
    let b = ComplexUnit::s;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Second, 1);
    expected.value = 2.0;

    assert_eq!(actual, expected);
}

#[test]
fn quantity_div_complex_unit()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 1);
    a.value = 2.0;
    let b = ComplexUnit::s;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Second, -1);
    expected.value = 2.0;

    assert_eq!(actual, expected);
}

#[test]
fn quantity_mul_quantity()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 2);
    a.value = 2.0;
    let mut b = Quantity::new();
    b.units.insert(Unit::Kilogram, 2);
    b.units.insert(Unit::Meter, -1);
    b.value = 3.5;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Kilogram, 2);
    expected.value = 7.0;

    assert_eq!(actual, expected);
}

#[test]
fn quantity_div_quantity()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 2);
    a.value = 2.0;
    let mut b = Quantity::new();
    b.units.insert(Unit::Kilogram, 2);
    b.units.insert(Unit::Meter, -1);
    b.value = 3.5;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 3);
    expected.units.insert(Unit::Kilogram, -2);
    expected.value = 2.0 / 3.5;

    assert_eq!(actual, expected);
}

#[test]
fn quantity_mul_f64()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 1);
    a.value = 2.0;
    let b = 2.0;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 4.0;

    assert_eq!(actual, expected);
}

#[test]
fn quantity_div_f64()
{
    let mut a = Quantity::new();
    a.units.insert(Unit::Meter, 1);
    a.value = 2.0;
    let b = 4.0;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 0.5;

    assert_eq!(actual, expected);
}

#[test]
fn quantity_mul_vector_quantity()
{
    let a = Quantity::new() * 5.0 * ComplexUnit::s;
    let v = Vector3::new(1.0, 2.0, 3.0);
    let mut b = VectorQuantity::from(v);
    b.units.insert(Unit::Kilogram, 2);

    let actual = a * b;
    let mut expected = VectorQuantity::from(v * 5.0);
    expected.units.insert(Unit::Kilogram, 2);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
fn quantity_mul_vector()
{
    let a = Quantity::new() * 5.0 * ComplexUnit::s;
    let b = Vector3::new(1.0, 2.0, 3.0);

    let actual = a * b;
    let mut expected = VectorQuantity::from(b * 5.0);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}