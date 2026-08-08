// q12. Does the three-input map erase? Clause four of the acceptance criterion
// at SETTLED.md:65-71 is that it "erase on lowering to be exactly what you
// describe". Every previous check in this stretch read shapes and sizes; this
// one reads instructions, on a body that does arithmetic rather than one that
// returns a size.
//
// Toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), pin nightly-2026-05-28.
// Features: none. Edition 2024.
// Build: rustc +nightly-2026-05-28 --edition 2024 -O --emit asm --crate-type lib \
//          q12_erasure_asm.rs --out-dir build
#![no_std]
#![allow(dead_code)]
include!("q07_body_inc.rs");
include!("q12_door_inc.rs");

#[repr(transparent)]
pub struct V<T: Derived>(<T as Derived>::Container);

// a 13.3 fixed-point multiply-accumulate at Hot, and its bare-primitive twin
pub type Money = UFixed<13, 3, Unsigned, Hot>;
pub type MoneyCold = UFixed<13, 3, Unsigned, Cold>;
pub type MoneySigned = UFixed<13, 3, Signed, Hot>;

#[unsafe(no_mangle)]
pub extern "C" fn q12_arvo_hot(a: V<Money>, b: V<Money>, c: V<Money>) -> u16 {
    a.0.wrapping_mul(b.0).wrapping_add(c.0)
}
#[unsafe(no_mangle)]
pub extern "C" fn q12_arvo_cold(a: V<MoneyCold>, b: V<MoneyCold>, c: V<MoneyCold>) -> u16 {
    a.0.wrapping_mul(b.0).wrapping_add(c.0)
}
#[unsafe(no_mangle)]
pub extern "C" fn q12_arvo_signed(a: V<MoneySigned>, b: V<MoneySigned>, c: V<MoneySigned>) -> i16 {
    a.0.wrapping_mul(b.0).wrapping_add(c.0)
}
#[unsafe(no_mangle)]
pub extern "C" fn q12_native_u16(a: u16, b: u16, c: u16) -> u16 {
    a.wrapping_mul(b).wrapping_add(c)
}
#[unsafe(no_mangle)]
pub extern "C" fn q12_native_i16(a: i16, b: i16, c: i16) -> i16 {
    a.wrapping_mul(b).wrapping_add(c)
}
