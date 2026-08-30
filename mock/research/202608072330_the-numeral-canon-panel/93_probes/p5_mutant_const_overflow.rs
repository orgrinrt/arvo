//! P5 negative control. The compile-time refusal from p5_what_warm_imitates.rs,
//! on its own, so the refusal is demonstrated rather than asserted.
//!
//! Expected: this file DOES NOT COMPILE. Its committed output is the error.

const W: u32 = 13;
const DECLARED_MODULUS: u64 = 1u64 << W;

const fn add_or_refuse(a: u64, b: u64) -> u64 {
    let s = a + b;
    assert!(s < DECLARED_MODULUS, "overflow at the declared width");
    s
}

const BAD: u64 = add_or_refuse(8000, 8000);

fn main() {
    println!("{BAD}");
}
