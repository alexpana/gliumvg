extern crate gliumvg;

macro_rules! assert_eq_f32 {
    ($left:expr, $right:expr) => (assert!((($left) - ($right)).abs() < 0.001));
}

#[test]
fn clamp() {
    use gliumvg::math::clamp;

    assert_eq!(clamp(10, 0, 100), 10);
    assert_eq!(clamp(10, 100, 1000), 100);
    assert_eq!(clamp(10, 0, 9), 9);

    assert_eq!(clamp::<f32>(10.0, 100.0, 1000.0), 100.0);
}

#[test]
fn modf() {
    use gliumvg::math::modf;
    use std::f32::NAN;

    assert_eq!(modf(5.1, 3.0), 2.1);
    assert!(modf(NAN, 3.0).is_nan());
}

#[test]
fn cross() {
    use gliumvg::math::cross;

    assert_eq_f32!(cross(1.0, 0.0, 0.0, 1.0), 1.0);
    assert_eq_f32!(cross(0.0, 1.0, 1.0, 0.0), -1.0);
    assert_eq_f32!(cross(0.4, 2.0, 0.6, 0.9), -0.84);
}