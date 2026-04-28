//! Warm const-bound positive test.
//!
//! `UFixed<I, F, Warm>` at `I + F = 32` is valid. Declaring and using
//! such a type must compile and behave normally at runtime.

#![no_std]

use arvo::{FBits, IBits, ibits, fbits};
use arvo::strategy::Warm;
use arvo::ufixed::UFixed;

#[test]
fn warm_at_32_bits_compiles_and_runs() {
    // 16 integer bits + 16 fractional bits = 32 total logical bits.
    // Warm container is u64 (2x).
    type U = UFixed<{ ibits(16) }, { fbits(16) }, Warm>;
    let a = U::from_raw(0x0001_0000); // 1.0 in 16.16.
    let b = U::from_raw(0x0002_0000); // 2.0 in 16.16.
    let sum = a + b;
    assert_eq!(sum.to_raw(), 0x0003_0000);
}

#[test]
fn warm_at_boundary_32_int() {
    // All-integer Warm at I=32. Container is u64 (Warm 2x at BITS 17..=32).
    type U = UFixed<{ ibits(32) }, { FBits::ZERO }, Warm>;
    let a = U::from_raw(1_000_000);
    let b = U::from_raw(2_000_000);
    let sum = a + b;
    assert_eq!(sum.to_raw(), 3_000_000u64);
}
