// The licence, half 2: the bundle `algebraic_add`/`algebraic_mul` grants
// is `reassoc nsz arcp contract` (measured in probe 4's llvm-ir), not
// `reassoc` alone. `contract` licenses fusing an adjacent multiply and
// add into one hardware `fmadd`, single rounding, the same value
// `.mul_add()` computes directly. The design's own droplist already
// states this is "a different operation, not a permission" (49:921-923,
// about `f64::mul_add` specifically); this probe shows the algebraic
// intrinsics silently reach the identical fused value through a route
// that is not spelled `mul_add` anywhere in the source, on a witness
// where fused and unfused disagree in the low bit.

#![allow(dead_code)]
#![feature(float_algebraic)]

use std::hint::black_box;

#[inline(never)]
pub fn mac_algebraic(a: f32, b: f32, c: f32) -> f32 {
    c.algebraic_add(a.algebraic_mul(b))
}
#[inline(never)]
pub fn mac_plain(a: f32, b: f32, c: f32) -> f32 {
    c + a * b
}
#[inline(never)]
pub fn mac_fma(a: f32, b: f32, c: f32) -> f32 {
    a.mul_add(b, c)
}

fn main() {
    // a witness where separate-rounding and single-rounding disagree,
    // found by sweeping a*a - a*a-shaped triples near 1.0 (a brute
    // search over 2000 candidates; the first disagreement is used).
    let a: f32 = black_box(1.0000001192);
    let b: f32 = black_box(1.0000001192);
    let c: f32 = black_box(-1.0000002384);

    let plain = mac_plain(a, b, c);
    let alg = mac_algebraic(a, b, c);
    let fma = mac_fma(a, b, c);

    println!("plain bits = {:#010x}", plain.to_bits());
    println!("alg   bits = {:#010x}", alg.to_bits());
    println!("fma   bits = {:#010x}", fma.to_bits());
    assert_ne!(
        alg.to_bits(),
        plain.to_bits(),
        "algebraic must differ from plain on this witness"
    );
    assert_eq!(
        alg.to_bits(),
        fma.to_bits(),
        "algebraic must match the fused value, not the plain one"
    );
    println!("confirmed: algebraic_mul + algebraic_add delivers the fused (mul_add) value, not the separately-rounded one");
}

// measured, aarch64-apple-darwin, `-O -C opt-level=3`, black_box defeats
// constant folding:
//   plain bits = 0x00000000
//   alg   bits = 0x28800000
//   fma   bits = 0x28800000
//   alg == plain: false   alg == fma: true
//
// asm for mac_algebraic: single `fmadd s0, s1, s0, s2`.
// asm for mac_plain:     `fmul s0, s0, s1` then `fadd s0, s2, s0`, two
// instructions, two roundings.
