use nbody::quantity::{ Unit, Quantity, ComplexUnit };

#[test]
fn create_quantity_from_unit()
{
    let actual = Unit::Meter * 10.0;

    let mut expected = Quantity::new();
    expected.value = 10.0;
    expected.units.insert(Unit::Meter, 1);
    
    assert_eq!(actual, expected);
}

#[test]
fn sum_summable_quantities()
{
    let actual = Unit::Meter * 10.0 + Unit::Meter * 5.0;

    let mut expected = Quantity::new();
    expected.value = 15.0;
    expected.units.insert(Unit::Meter, 1);
    
    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn sum_unsummable_quantities()
{
    let a = Unit::Meter * 10.0;
    let b = Unit::Second * 5.0;

    let _actual = a + b;
}

#[test]
fn sub_summable_quantities()
{
    let actual = Unit::Meter * 10.0 - Unit::Meter * 5.0;

    let mut expected = Quantity::new();
    expected.value = 5.0;
    expected.units.insert(Unit::Meter, 1);
    
    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn sub_unsummable_quantities()
{
    let a = Unit::Meter * 10.0;
    let b = Unit::Second * 5.0;

    let _actual = a - b;
}

#[test]
fn multiply_quantity_by_number()
{
    let a = Unit::Meter * 10.0;
    let b = 3.0;

    let actual = a * b;

    let mut expected = Quantity::new();
    expected.value = 30.0;
    expected.units.insert(Unit::Meter, 1);

    assert_eq!(actual, expected);
}

#[test]
fn multiply_quantity_by_unit()
{
    let a = Unit::Kilogram * 2.0;
    let b = Unit::Meter;

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.value = 2.0;
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Kilogram, 1);

    assert_eq!(actual, expected);
}

#[test]
fn multiply_different_quantities() 
{
    let a = Unit::Meter * 10.0;
    let b = Unit::Second * 5.0;

    let actual = a * b;

    let mut expected = Quantity::new();
    expected.value = 50.0;
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
fn multiply_same_quantities() 
{
    let a = Unit::Meter * 8.0;
    let b = Unit::Meter * 5.0;

    let actual = a * b;

    let mut expected = Quantity::new();
    expected.value = 40.0;
    expected.units.insert(Unit::Meter, 2);

    assert_eq!(actual, expected);
}

#[test]
fn divide_quantity_by_number()
{
    let a = Unit::Meter * 10.0;
    let b = 2.0;

    let actual = a / b;

    let mut expected = Quantity::new();
    expected.value = 5.0;
    expected.units.insert(Unit::Meter, 1);

    assert_eq!(actual, expected);
}

#[test]
fn divide_quantity_by_unit()
{
    let a = Unit::Kilogram * 2.0;
    let b = Unit::Meter;

    let actual = a / b;
    let mut expected = Quantity::new();
    expected.value = 2.0;
    expected.units.insert(Unit::Meter, -1);
    expected.units.insert(Unit::Kilogram, 1);

    assert_eq!(actual, expected);
}

#[test]
fn divide_different_quantities() 
{
    let a = Unit::Meter * 10.0;
    let b = Unit::Second * 5.0;

    let actual = a / b;

    let mut expected = Quantity::new();
    expected.value = 2.0;
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Second, -1);

    assert_eq!(actual, expected);
}

#[test]
fn divide_same_quantities() 
{
    let a = Unit::Meter * 20.0;
    let b = Unit::Meter * 5.0;

    let actual = a / b;

    let mut expected = Quantity::new();
    expected.value = 4.0;

    assert_eq!(actual, expected);
}

#[test]
fn multiply_complex_quantities()
{
    let a = Quantity::new() * 5.0 * Unit::Kilogram * Unit::Meter / Unit::Second;
    let b = Quantity::new() * 10.0 * Unit::Kilogram.pow(-4) * Unit::Meter * Unit::Second.pow(2);

    let actual = a * b;
    let mut expected = Quantity::new();
    expected.value = 50.0;
    expected.units.insert(Unit::Kilogram, -3);
    expected.units.insert(Unit::Meter, 2);
    expected.units.insert(Unit::Second, 1);
    
    assert_eq!(actual, expected);
}

#[test]
fn value_in_complex_units()
{
    let a = Unit::Meter * 3.086e+20;
    let b = Unit::Kilogram * 2.0 * 1.989e+29;

    let actuala = a.value_in(ComplexUnit::kpc.convert());
    let expecteda = 10.0;

    let actualb = b.value_in(ComplexUnit::MSun.convert());
    let expectedb = 0.2;

    assert_eq!(actuala, expecteda);
    assert_eq!(actualb, expectedb);
}

#[test]
fn quantity_pow()
{
    let actual = (ComplexUnit::m * 10.0).pow(2);
    let expected = Unit::Meter * Unit::Meter * 100.0;

    assert_eq!(actual, expected);
}

#[test]
fn complex_quantities_in_complex_units()
{
    let a = ComplexUnit::J * 1.0;

    let actual = a.value_in(ComplexUnit::MSun * ComplexUnit::pc.pow(2) * ComplexUnit::yr.pow(-2));
    let expected = 1.989e+30 * 3.086e+16 * 3.086e+16 / 31536000.0 / 31536000.0;
    let expected = 1.0 / expected;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn value_in_wrong_units()
{
    let a = Unit::Meter * 3.086e+20;

    let _x = a.value_in(ComplexUnit::MSun.convert());
}

#[test]
fn multiply_complex_unit_by_number()
{
    let actual = ComplexUnit::MSun * 10.0;
    let expected = Unit::Kilogram * 1.989e+31;

    assert_eq!(actual, expected);
}

#[test]
fn multiply_complex_unit_by_quantity()
{
    let a = ComplexUnit::m * 10.0;
    let b = ComplexUnit::kpc;

    let actual = b * a;
    let expected = Unit::Meter.pow(2) * 3.086e+20;

    assert_eq!(actual, expected);
}