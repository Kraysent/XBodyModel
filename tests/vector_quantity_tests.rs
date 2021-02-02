use xbody_model::quantity::Units;
use xbody_model::vector::Vector3;

#[test]
fn vquantity_mul_squantity() {
    let vq = Vector3::new(1., 5., 2.) * Units::m;
    let q = 5. * Units::s;

    let actual = vq * q;
    let expected = Vector3::new(5., 25., 10.) * Units::m * Units::s;

    assert_eq!(actual, expected);
}

#[test]
fn vquantity_mulassign_squantity() {
    let mut actual = Vector3::new(1., 5., 2.) * Units::m;
    let q = 5. * Units::s;

    actual *= q;
    let expected = Vector3::new(5., 25., 10.) * Units::m * Units::s;

    assert_eq!(actual, expected);
}

#[test]
fn vquantity_div_squantity() {
    let vq = Vector3::new(1., 5., 2.) * Units::m;
    let q = 1. * Units::s;

    let actual = vq / q;
    let expected = Vector3::new(1., 5., 2.) * Units::m * Units::s.pow(-1.);

    assert_eq!(actual, expected);
}

#[test]
fn vquantity_divassign_squantity() {
    let mut actual = Vector3::new(1., 5., 2.) * Units::m;
    let q = 1. * Units::s;

    actual /= q;
    let expected = Vector3::new(1., 5., 2.) * Units::m * Units::s.pow(-1.);

    assert_eq!(actual, expected);
}

#[test]
fn vquantity_add_vquantity() {
    let vq1 = Vector3::new(1., 2., 3.) * Units::m;
    let vq2 = Vector3::new(3., 2., 3.) * Units::m;

    let actual = vq1 + vq2;
    let expected = Vector3::new(4., 4., 6.) * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn vquantity_addassign_vquantity() {
    let mut actual = Vector3::new(1., 2., 3.) * Units::m;
    let vq2 = Vector3::new(3., 2., 3.) * Units::m;

    actual += vq2;
    let expected = Vector3::new(4., 4., 6.) * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn vquantity_sub_vquantity() {
    let vq1 = Vector3::new(1., 2., 3.) * Units::m;
    let vq2 = Vector3::new(3., 2., 3.) * Units::m;

    let actual = vq1 - vq2;
    let expected = Vector3::new(-2., 0., 0.) * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn vquantity_subassign_vquantity() {
    let mut actual = Vector3::new(1., 2., 3.) * Units::m;
    let vq2 = Vector3::new(3., 2., 3.) * Units::m;

    actual -= vq2;
    let expected = Vector3::new(-2., 0., 0.) * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn f64_mul_vquantity() {
    let x = 5.;
    let vq = Vector3::new(1., 1., 1.) * Units::m;

    let actual = x * vq;
    let expected = Vector3::new(5., 5., 5.) * Units::m;

    assert_eq!(actual, expected);
}

#[test]
fn vquantity_mul_f64() {
    let vq = Vector3::new(2., 4., 2.) * Units::kg;
    let x = 2.;

    let actual = vq * x;
    let expected = Vector3::new(4., 8., 4.) * Units::kg;

    assert_eq!(actual, expected);
}

#[test]
fn vquantity_mulassign_f64() {
    let mut actual = Vector3::new(2., 4., 2.) * Units::kg;
    let x = 2.;

    actual *= x;
    let expected = Vector3::new(4., 8., 4.) * Units::kg;

    assert_eq!(actual, expected);
}

#[test]
fn vquantity_div_f64() {
    let vq = Vector3::new(2., 4., 2.) * Units::kg;
    let x = 2.;

    let actual = vq / x;
    let expected = Vector3::new(1., 2., 1.) * Units::kg;

    assert_eq!(actual, expected);
}

#[test]
fn vquantity_divassign_f64() {
    let mut actual = Vector3::new(2., 4., 2.) * Units::kg;
    let x = 2.;

    actual /= x;
    let expected = Vector3::new(1., 2., 1.) * Units::kg;

    assert_eq!(actual, expected);
}
