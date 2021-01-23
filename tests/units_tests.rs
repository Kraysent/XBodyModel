use nbody::quantity::{ Unit, ComplexUnit, Quantity, VectorQuantity };
use nbody::vector::Vector3;

#[test]
fn unit_add_compatible_unit()
{
    let a = Unit::Meter;
    let b = Unit::Meter;

    let actual = a + b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 2.0;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn unit_add_incompatible_unit()
{
    let a = Unit::Meter;
    let b = Unit::Second;

    let _actual = a + b;
}

#[test]
fn unit_sub_compatible_unit()
{
    let a = Unit::Meter;
    let b = Unit::Meter;

    let actual = a - b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 0.0;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn unit_sub_incompatible_unit()
{
    let a = Unit::Meter;
    let b = Unit::Second;

    let _actual = a - b;
}

#[test]
fn unit_add_compatible_complex_unit()
{
    let a = Unit::Meter;
    let b = ComplexUnit::pc;

    let actual = a + b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 3.086e+16 + 1.0;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn unit_add_incompatible_complex_unit()
{
    let a = Unit::Meter;
    let b = ComplexUnit::MSun;

    let _actual = a + b;
}

#[test]
fn unit_sub_compatible_complex_unit()
{
    let a = Unit::Meter;
    let b = ComplexUnit::m;

    let actual = a - b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 0.0;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn unit_sub_incompatible_complex_unit()
{
    let a = Unit::Meter;
    let b = ComplexUnit::s;

    let _actual = a - b;
}

#[test]
fn unit_add_compatible_quantity()
{
    let a = Unit::Meter;
    let mut b = Quantity::new();
    b.units.insert(Unit::Meter, 1);
    b.value = 2.0;

    let actual = a + b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = 3.0;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn unit_add_incompatible_quantity()
{
    let a = Unit::Meter;
    let mut b = Quantity::new();
    b.units.insert(Unit::Second, 1);
    b.value = 2.0;

    let _actual = a + b;
}

#[test]
fn unit_sub_compatible_quantity()
{
    let a = Unit::Meter;
    let mut b = Quantity::new();
    b.units.insert(Unit::Meter, 1);
    b.value = 2.0;

    let actual = a - b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 1);
    expected.value = -1.0;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn unit_sub_incompatible_quantity()
{
    let a = Unit::Meter;
    let mut b = Quantity::new();
    b.units.insert(Unit::Second, 1);
    b.value = 2.0;

    let _actual = a - b;
}

#[test]
fn unit_mul_unit()
{
    let a = Unit::Meter;
    let b = Unit::Meter;
    let c = Unit::Meter;
    let d = Unit::Second;

    let actual_same = a * b;
    let actual_diff = c * d;
    let mut expected_same = Quantity::new();
    expected_same.units.insert(Unit::Meter, 2);
    let mut expected_diff = Quantity::new();
    expected_diff.units.insert(Unit::Meter, 1);
    expected_diff.units.insert(Unit::Second, 1);

    assert_eq!(actual_same, expected_same);
    assert_eq!(actual_diff, expected_diff);
}

#[test]
fn unit_div_unit()
{
    let a = Unit::Meter;
    let b = Unit::Meter;
    let c = Unit::Meter;
    let d = Unit::Second;

    let actual_same = a / b;
    let actual_diff = c / d;
    let expected_same = Quantity::new();
    let mut expected_diff = Quantity::new();
    expected_diff.units.insert(Unit::Meter, 1);
    expected_diff.units.insert(Unit::Second, -1);

    assert_eq!(actual_same, expected_same);
    assert_eq!(actual_diff, expected_diff);
}

#[test]
fn unit_mul_complex_unit()
{
    let a = Unit::Meter;
    let b = ComplexUnit::kpc;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Meter, 2);
    expected.value = 3.086e+19;

    assert_eq!(actual, expected);
}

#[test]
fn unit_div_complex_unit()
{
    let a = Unit::Meter;
    let b = ComplexUnit::kpc;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.value = 1.0 / 3.086e+19;

    assert_eq!(actual, expected);
}

#[test]
fn unit_mul_quantity()
{
    let a = Unit::Second;
    let mut b = Quantity::new();
    b.units.insert(Unit::Kilogram, 2);
    b.value = 5.5;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Kilogram, 2);
    expected.units.insert(Unit::Second, 1);
    expected.value = 5.5;

    assert_eq!(actual, expected);
}

#[test]
fn unit_div_quantity()
{
    let a = Unit::Second;
    let mut b = Quantity::new();
    b.units.insert(Unit::Kilogram, 2);
    b.value = 5.5;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Kilogram, -2);
    expected.units.insert(Unit::Second, 1);
    expected.value = 1.0 / 5.5;

    assert_eq!(actual, expected);
}

#[test]
fn unit_mul_vector_quantity()
{
    let a = Unit::Second;
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
fn unit_mul_vector()
{
    let a = Unit::Second;
    let b = Vector3::new(1.0, 2.0, 3.0);

    let actual = a * b;
    let mut expected = VectorQuantity::from(b);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
fn unit_mul_f64()
{
    let a = Unit::Kilogram;
    let b = 2.3;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Kilogram, 1);
    expected.value = 2.3;

    assert_eq!(actual, expected);
}

#[test]
fn unit_div_f64()
{
    let a = Unit::Kilogram;
    let b = 2.3;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.units.insert(Unit::Kilogram, 1);
    expected.value = 1.0 / 2.3;

    assert_eq!(actual, expected);
}