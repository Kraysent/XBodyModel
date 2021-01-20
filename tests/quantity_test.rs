use nbody::quantity::{ Unit, Quantity };

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
fn multiply_quantity_by_scalar()
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
fn divide_quantity_by_scalar()
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
