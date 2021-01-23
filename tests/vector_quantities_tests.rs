use nbody::quantity::{ Unit, ComplexUnit, Quantity, VectorQuantity };
use nbody::vector::Vector3;

#[test]
fn vector_quantity_add_compatible_vector_quantity()
{
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(3.0, 2.0, 3.0);
    
    let mut vq1 = VectorQuantity::from(v1);
    vq1.units.insert(Unit::Meter, 2);
    vq1.units.insert(Unit::Second, 1);
    let mut vq2 = VectorQuantity::from(v2);
    vq2.units.insert(Unit::Meter, 2);
    vq2.units.insert(Unit::Second, 1);

    let actual = vq1 + vq2;
    let mut expected = VectorQuantity::from(v1 + v2);
    expected.units.insert(Unit::Meter, 2);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn vector_quantity_add_incompatible_vector_quantity()
{
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(3.0, 2.0, 3.0);
    
    let mut vq1 = VectorQuantity::from(v1);
    vq1.units.insert(Unit::Meter, 2);
    vq1.units.insert(Unit::Second, 1);
    let mut vq2 = VectorQuantity::from(v2);
    vq2.units.insert(Unit::Meter, 2);
    vq2.units.insert(Unit::Second, 1);
    vq2.units.insert(Unit::Kilogram, -1);

    let _actual = vq1 + vq2;
}

#[test]
fn vector_quantity_sub_compatible_vector_quantity()
{
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(3.0, 2.0, 3.0);
    
    let mut vq1 = VectorQuantity::from(v1);
    vq1.units.insert(Unit::Meter, 2);
    vq1.units.insert(Unit::Second, 1);
    let mut vq2 = VectorQuantity::from(v2);
    vq2.units.insert(Unit::Meter, 2);
    vq2.units.insert(Unit::Second, 1);

    let actual = vq1 - vq2;
    let mut expected = VectorQuantity::from(v1 - v2);
    expected.units.insert(Unit::Meter, 2);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn vector_quantity_sub_incompatible_vector_quantity()
{
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(3.0, 2.0, 3.0);
    
    let mut vq1 = VectorQuantity::from(v1);
    vq1.units.insert(Unit::Meter, 2);
    vq1.units.insert(Unit::Second, 1);
    let mut vq2 = VectorQuantity::from(v2);
    vq2.units.insert(Unit::Meter, 2);
    vq2.units.insert(Unit::Second, 1);
    vq2.units.insert(Unit::Kilogram, -1);

    let _actual = vq1 - vq2;
}

#[test]
fn vector_quantity_mul_unit()
{
    let v = Vector3::new(1.0, 2.0, 3.0);
    let q = Unit::Meter;

    let mut vq = VectorQuantity::from(v);
    vq.units.insert(Unit::Meter, 2);
    vq.units.insert(Unit::Second, 1);

    let actual = vq * q;
    let mut expected = VectorQuantity::from(v);
    expected.units.insert(Unit::Meter, 3);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
fn vector_quantity_div_unit()
{
    let v = Vector3::new(1.0, 2.0, 3.0);
    let q = Unit::Meter;

    let mut vq = VectorQuantity::from(v);
    vq.units.insert(Unit::Meter, 2);
    vq.units.insert(Unit::Second, 1);

    let actual = vq / q;
    let mut expected = VectorQuantity::from(v);
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
fn vector_quantity_mul_complex_unit()
{
    let v = Vector3::new(1.0, 2.0, 3.0);
    let q = ComplexUnit::m;

    let mut vq = VectorQuantity::from(v);
    vq.units.insert(Unit::Meter, 2);
    vq.units.insert(Unit::Second, 1);

    let actual = vq * q;
    let mut expected = VectorQuantity::from(v);
    expected.units.insert(Unit::Meter, 3);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
fn vector_quantity_div_complex_unit()
{
    let v = Vector3::new(1.0, 2.0, 3.0);
    let q = ComplexUnit::m;

    let mut vq = VectorQuantity::from(v);
    vq.units.insert(Unit::Meter, 2);
    vq.units.insert(Unit::Second, 1);

    let actual = vq / q;
    let mut expected = VectorQuantity::from(v);
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}


#[test]
fn vector_quantity_mul_quantity()
{
    let v = Vector3::new(1.0, 2.0, 3.0);
    let q = Quantity::new() * ComplexUnit::m * 3.0;

    let mut vq = VectorQuantity::from(v);
    vq.units.insert(Unit::Meter, 2);
    vq.units.insert(Unit::Second, 1);

    let actual = vq * q;
    let mut expected = VectorQuantity::from(v * 3.0);
    expected.units.insert(Unit::Meter, 3);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
fn vector_quantity_div_quantity()
{
    let v = Vector3::new(1.0, 2.0, 3.0);
    let q = Quantity::new() * ComplexUnit::m * 3.0;

    let mut vq = VectorQuantity::from(v);
    vq.units.insert(Unit::Meter, 2);
    vq.units.insert(Unit::Second, 1);

    let actual = vq / q;
    let mut expected = VectorQuantity::from(v / 3.0);
    expected.units.insert(Unit::Meter, 1);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
fn vector_quantity_mul_f64()
{
    let v = Vector3::new(1.0, 2.0, 3.0);
    let q = 3.0;

    let mut vq = VectorQuantity::from(v);
    vq.units.insert(Unit::Meter, 2);
    vq.units.insert(Unit::Second, 1);

    let actual = vq * q;
    let mut expected = VectorQuantity::from(v * 3.0);
    expected.units.insert(Unit::Meter, 2);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}

#[test]
fn vector_quantity_div_f64()
{
    let v = Vector3::new(1.0, 2.0, 3.0);
    let q = 3.0;

    let mut vq = VectorQuantity::from(v);
    vq.units.insert(Unit::Meter, 2);
    vq.units.insert(Unit::Second, 1);

    let actual = vq / q;
    let mut expected = VectorQuantity::from(v / 3.0);
    expected.units.insert(Unit::Meter, 2);
    expected.units.insert(Unit::Second, 1);

    assert_eq!(actual, expected);
}