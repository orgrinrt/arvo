//! Arithmetic ops delegate to the wrapped IEEE float.
//!
//! Round-trips exact-representable values through each of Add, Sub,
//! Mul, Div, Neg on both wrappers at both widths.

use arvo::{FastFloat, StrictFloat};

// --- FastFloat<f32> ----

#[test]
fn fast_f32_add() {
    let a = FastFloat::new(2.0f32);
    let b = FastFloat::new(3.0f32);
    assert_eq!((a + b).into_inner(), 5.0);
}

#[test]
fn fast_f32_sub() {
    let a = FastFloat::new(5.0f32);
    let b = FastFloat::new(3.0f32);
    assert_eq!((a - b).into_inner(), 2.0);
}

#[test]
fn fast_f32_mul() {
    let a = FastFloat::new(2.0f32);
    let b = FastFloat::new(3.0f32);
    assert_eq!((a * b).into_inner(), 6.0);
}

#[test]
fn fast_f32_div() {
    let a = FastFloat::new(6.0f32);
    let b = FastFloat::new(3.0f32);
    assert_eq!((a / b).into_inner(), 2.0);
}

#[test]
fn fast_f32_neg() {
    let a = FastFloat::new(3.0f32);
    assert_eq!((-a).into_inner(), -3.0);
}

// --- FastFloat<f64> ----

#[test]
fn fast_f64_add() {
    let a = FastFloat::new(2.0f64);
    let b = FastFloat::new(3.0f64);
    assert_eq!((a + b).into_inner(), 5.0);
}

#[test]
fn fast_f64_sub() {
    let a = FastFloat::new(5.0f64);
    let b = FastFloat::new(3.0f64);
    assert_eq!((a - b).into_inner(), 2.0);
}

#[test]
fn fast_f64_mul() {
    let a = FastFloat::new(2.0f64);
    let b = FastFloat::new(3.0f64);
    assert_eq!((a * b).into_inner(), 6.0);
}

#[test]
fn fast_f64_div() {
    let a = FastFloat::new(6.0f64);
    let b = FastFloat::new(3.0f64);
    assert_eq!((a / b).into_inner(), 2.0);
}

#[test]
fn fast_f64_neg() {
    let a = FastFloat::new(3.0f64);
    assert_eq!((-a).into_inner(), -3.0);
}

// --- StrictFloat<f32> ----

#[test]
fn strict_f32_add() {
    let a = StrictFloat::new(2.0f32);
    let b = StrictFloat::new(3.0f32);
    assert_eq!((a + b).into_inner(), 5.0);
}

#[test]
fn strict_f32_sub() {
    let a = StrictFloat::new(5.0f32);
    let b = StrictFloat::new(3.0f32);
    assert_eq!((a - b).into_inner(), 2.0);
}

#[test]
fn strict_f32_mul() {
    let a = StrictFloat::new(2.0f32);
    let b = StrictFloat::new(3.0f32);
    assert_eq!((a * b).into_inner(), 6.0);
}

#[test]
fn strict_f32_div() {
    let a = StrictFloat::new(6.0f32);
    let b = StrictFloat::new(3.0f32);
    assert_eq!((a / b).into_inner(), 2.0);
}

#[test]
fn strict_f32_neg() {
    let a = StrictFloat::new(3.0f32);
    assert_eq!((-a).into_inner(), -3.0);
}

// --- StrictFloat<f64> ----

#[test]
fn strict_f64_add() {
    let a = StrictFloat::new(2.0f64);
    let b = StrictFloat::new(3.0f64);
    assert_eq!((a + b).into_inner(), 5.0);
}

#[test]
fn strict_f64_sub() {
    let a = StrictFloat::new(5.0f64);
    let b = StrictFloat::new(3.0f64);
    assert_eq!((a - b).into_inner(), 2.0);
}

#[test]
fn strict_f64_mul() {
    let a = StrictFloat::new(2.0f64);
    let b = StrictFloat::new(3.0f64);
    assert_eq!((a * b).into_inner(), 6.0);
}

#[test]
fn strict_f64_div() {
    let a = StrictFloat::new(6.0f64);
    let b = StrictFloat::new(3.0f64);
    assert_eq!((a / b).into_inner(), 2.0);
}

#[test]
fn strict_f64_neg() {
    let a = StrictFloat::new(3.0f64);
    assert_eq!((-a).into_inner(), -3.0);
}
