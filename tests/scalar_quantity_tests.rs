use nbody::quantity::Units;
use nbody::vector::Vector3;

#[test]
fn squantity_add_squantity()
{
    let q1 = 2. * Units::m;
    let q2 = 3. * Units::m;

    let actual = q1 + q2;
    let expected = 5. * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_addassign_squantity()
{
    let mut actual = 2. * Units::m;
    let q2 = 3. * Units::m;

    actual += q2;
    let expected = 5. * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_sub_squantity()
{
    let q1 = 2. * Units::m;
    let q2 = 3. * Units::m;

    let actual = q1 - q2;
    let expected = -1. * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_subassign_squantity()
{
    let mut actual = 2. * Units::m;
    let q2 = 3. * Units::m;

    actual -= q2;
    let expected = -1. * Units::m;

    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn squantity_add_incompatible_squantity()
{
    let q1 = 2. * Units::m;
    let q2 = 3. * Units::s;

    let _actual = q1 + q2;
}

#[test]
#[should_panic]
fn squantity_addassign_incompatible_squantity()
{
    let mut actual = 2. * Units::m;
    let q2 = 3. * Units::s;

    actual += q2;
}

#[test]
#[should_panic]
fn squantity_sub_incompatible_squantity()
{
    let q1 = 2. * Units::m;
    let q2 = 3. * Units::s;

    let _actual = q1 - q2;
}

#[test]
#[should_panic]
fn squantity_subassign_incompatible_squantity()
{
    let mut actual = 2. * Units::m;
    let q2 = 3. * Units::s;

    actual -= q2;
}

#[test]
fn squantity_mul_f64()
{
    let x = 3. * Units::s;
    let y = 2.;

    let actual = x * y;
    let expected = 6. * Units::s;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_mulassign_f64()
{
    let mut actual = 3. * Units::s;
    let y = 2.;

    actual *= y;
    let expected = 6. * Units::s;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_mul_units()
{
    let x = 3. * Units::s;
    let y = Units::m;

    let actual = x * y;
    let expected = 3. * Units::s * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_mulassign_units()
{
    let mut actual = 3. * Units::s;
    let y = Units::m;

    actual *= y;
    let expected = 3. * Units::s * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_mul_squantity()
{
    let x = 3. * Units::s;
    let y = 2. * Units::m;

    let actual = x * y;
    let expected = 6. * Units::s * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_mulassign_squantity()
{
    let mut actual = 3. * Units::s;
    let y = 2. * Units::m;

    actual *= y;
    let expected = 6. * Units::s * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_div_f64()
{
    let x = 3. * Units::s;
    let y = 2.;

    let actual = x / y;
    let expected = 1.5 * Units::s;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_divassign_f64()
{
    let mut actual = 3. * Units::s;
    let y = 2.;

    actual /= y;
    let expected = 1.5 * Units::s;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_div_units()
{
    let x = 3. * Units::s;
    let y = Units::m;

    let actual = x / y;
    let expected = 3. * Units::s * Units::m.pow(-1.);

    assert_eq!(actual, expected);
}

#[test]
fn squantity_divassign_units()
{
    let mut actual = 3. * Units::s;
    let y = Units::m;

    actual /= y;
    let expected = 3. * Units::s * Units::m.pow(-1.);

    assert_eq!(actual, expected);
}

#[test]
fn squantity_div_squantity()
{
    let x = 3. * Units::s;
    let y = 2. * Units::m;

    let actual = x / y;
    let expected = 1.5 * Units::s * Units::m.pow(-1.);

    assert_eq!(actual, expected);
}

#[test]
fn squantity_divassign_squantity()
{
    let mut actual = 3. * Units::s;
    let y = 2. * Units::m;

    actual /= y;
    let expected = 1.5 * Units::s * Units::m.pow(-1.);

    assert_eq!(actual, expected);
}

#[test]
fn squantity_mul_vquantity()
{
    let vq = Vector3::new(1., 2., 3.) * Units::m;
    let q = 3. * Units::s;

    let actual = q * vq;
    let expected = Vector3::new(3., 6., 9.) * Units::m * Units::s;

    assert_eq!(actual, expected);
}

#[test]
fn squantity_mul_vector3()
{
    let v = Vector3::new(1., 2., 1.);
    let q = 3. * Units::m;

    let actual = q * v;
    let expected = Vector3::new(3., 6., 3.) * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn vector3_mul_squantity()
{
    let v = Vector3::new(1., 2., 1.);
    let q = 3. * Units::m;

    let actual = v * q;
    let expected = Vector3::new(3., 6., 3.) * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn vector3_div_squantity()
{
    let v = Vector3::new(3., 6., 3.);
    let q = 3. * Units::m;

    let actual = v / q;
    let expected = Vector3::new(1., 2., 1.) * Units::m.pow(-1.);

    assert_eq!(actual, expected);
}

#[test]
fn f64_mul_squantity()
{
    let x = 6.;
    let q = 2. * Units::m;

    let actual = x * q;
    let expected = 12. * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn f64_div_squantity()
{
    let x = 6.;
    let q = 2. * Units::m;

    let actual = x / q;
    let expected = 3. * Units::m.pow(-1.);

    assert_eq!(actual, expected);
}

#[test]
fn squantity_cmp_squantity()
{
    let q1 = 2.0 * Units::m;
    let q2 = 3.0 * Units::m;
    let q3 = 2.0 * Units::m;

    assert_eq!(q1 > q2, false);
    assert_eq!(q2 > q1, true);
    assert_eq!(q2 < q1, false);
    assert_eq!(q1 < q2, true);
    assert_eq!(q3 >= q2, false);
    assert_eq!(q1 >= q3, true);
    assert_eq!(q2 <= q1, false);    
    assert_eq!(q1 <= q2, true);    
}