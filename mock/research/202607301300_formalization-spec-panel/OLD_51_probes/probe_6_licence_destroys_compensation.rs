// The licence's scope boundary: the interior-safety condition licenses
// reassociation for `fold`, whose accumulator is wide enough that
// regrouping is a value-preserving no-op (49 section 1.8). It must never
// be granted to `fold_compensated`, whose entire mechanism (49 section
// 1.5, "the one genuinely shaped fold") is error feedback on a SPECIFIC,
// unreassociated rounding sequence: the classic Kahan step computes
// `(sum + y) - sum - y`, which is algebraically zero as a real-number
// identity and numerically the lost rounding error as a floating-point
// one. `reassoc` is licensed to treat the two readings as
// interchangeable, because algebraically they are. This probe compiles
// the textbook failure directly on this pin: the strict version recovers
// the lost bits; the algebraic version is optimised down to `y - y` and
// always returns zero.

#![allow(dead_code)]
#![feature(float_algebraic)]

use std::hint::black_box;

#[inline(never)]
#[no_mangle]
pub fn kahan_step_strict(sum: f32, y: f32) -> f32 {
    let t = sum + y;
    (t - sum) - y
}

#[inline(never)]
#[no_mangle]
pub fn kahan_step_algebraic(sum: f32, y: f32) -> f32 {
    let t = sum.algebraic_add(y);
    t.algebraic_sub(sum).algebraic_sub(y)
}

fn main() {
    let sum: f32 = black_box(1.0);
    let y: f32 = black_box(2.0f32.powi(-30)); // far below f32's rounding granularity at 1.0
    let strict = kahan_step_strict(sum, y);
    let algebraic = kahan_step_algebraic(sum, y);
    println!("y                            = {:e}", y);
    println!("strict compensation term    = {:e}", strict);
    println!("algebraic compensation term = {:e}", algebraic);
    assert_eq!(strict, -y, "strict recovers exactly the lost bits");
    assert_eq!(
        algebraic, 0.0,
        "algebraic reassociates the compensation away to nothing"
    );
}

// measured, aarch64-apple-darwin, `-O -C opt-level=3`:
//   y                            = 9.313226e-10
//   strict compensation term    = -9.313226e-10
//   algebraic compensation term = 0e0
//
// asm, kahan_step_strict:      fadd s2,s0,s1 / fsub s0,s2,s0 / fsub s0,s0,s1
// asm, kahan_step_algebraic:   fsub s0,s1,s1      (constant-folded to y - y)
