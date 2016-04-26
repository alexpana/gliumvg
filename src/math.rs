pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        return min;
    }

    if value > max {
        return max;
    }

    return value;
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        b
    } else {
        a
    }
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn sqrt(x: f32) -> f32 {
    x.sqrt()
}

pub fn sin(x: f32) -> f32 {
    x.sin()
}

pub fn cos(x: f32) -> f32 {
    x.cos()
}

pub fn tan(x: f32) -> f32 {
    x.tan()
}

pub fn atan2(x: f32, y: f32) -> f32 {
    x.atan2(y)
}

pub fn acos(x: f32) -> f32 {
    x.acos()
}

pub fn abs(x: f32) -> f32 {
    x.abs()
}

pub fn sign(x: f32) -> f32 {
    x.signum()
}

pub fn cross(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    x1 * y2 - x2 * y1
}

///
/// The floating-point remainder of the division operation `x/y` calculated by this function
/// is exactly the value `x - n*y`, where `n` is `x/y` with its fractional part truncated.
///
/// See http://en.cppreference.com/w/c/numeric/math/fmod
///
/// Error Handling:
/// * If x is ±0 and y is not zero, ±0 is returned
/// * If x is ±∞ and y is not NaN, NaN is returned and FE_INVALID is raised
/// * If y is ±0 and x is not NaN, NaN is returned and FE_INVALID is raised
/// * If y is ±∞ and x is finite, x is returned.
/// * If either argument is NaN, NaN is returned
pub fn modf(x: f32, y: f32) -> f32 {
    use std::f32::{NAN, INFINITY, NEG_INFINITY};

    if x == 0.0 && y != 0.0 {
        return 0.0;
    }

    if x.is_infinite() && !y.is_nan() {
        return NAN
    }

    if !x.is_nan() && y == 0.0 {
        return NAN;
    }

    if x.is_finite() && y.is_infinite() {
        return x;
    }

    if x.is_nan() || y.is_nan() {
        return NAN;
    }


    let n = (x / y).trunc();
    return x - n * y;
}

//static float nvg__cross(float dx0, float dy0, float dx1, float dy1) { return dx1*dy0 - dx0*dy1; }