use xbody_model::vector::Vector3;

#[test]
fn vector3_add_vector3()
{
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(2., 6., 1.);
    
    let actual = v1 + v2;
    let expected = Vector3::new(3., 8., 4.);

    assert_eq!(actual, expected);
}

#[test]
fn vector3_addassign_vector3()
{
    let mut actual = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(2., 6., 1.);
    
    actual += v2;
    let expected = Vector3::new(3., 8., 4.);

    assert_eq!(actual, expected);
}

#[test]
fn vector3_sub_vector3()
{
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(2., 6., 1.);
    
    let actual = v1 - v2;
    let expected = Vector3::new(-1., -4., 2.);

    assert_eq!(actual, expected);
}

#[test]
fn vector3_subassign_vector3()
{
    let mut actual = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(2., 6., 1.);
    
    actual -= v2;
    let expected = Vector3::new(-1., -4., 2.);

    assert_eq!(actual, expected);
}

#[test]
fn vector3_mul_f64()
{
    let v = Vector3::new(2., 4., 6.);
    let x = 0.5;

    let actual = v * x;
    let expected = Vector3::new(1., 2., 3.);

    assert_eq!(actual, expected);
}

#[test]
fn vector3_mulassign_f64()
{
    let mut actual = Vector3::new(2., 4., 6.);
    let x = 0.5;

    actual *= x;
    let expected = Vector3::new(1., 2., 3.);

    assert_eq!(actual, expected);
}

#[test]
fn vector3_div_f64()
{
    let v = Vector3::new(2., 4., 6.);
    let x = 0.5;

    let actual = v / x;
    let expected = Vector3::new(4., 8., 12.);

    assert_eq!(actual, expected);
}

#[test]
fn vector3_divassign_f64()
{
    let mut actual = Vector3::new(2., 4., 6.);
    let x = 0.5;

    actual /= x;
    let expected = Vector3::new(4., 8., 12.);

    assert_eq!(actual, expected);
}

#[test]
fn f64_mul_vector3()
{
    let x = 2.;
    let v = Vector3::new(1., 2., 1.);

    let actual = x * v;
    let expected = Vector3::new(2., 4., 2.);

    assert_eq!(actual, expected);
}