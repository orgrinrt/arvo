//! `BitAccess` coverage: `bit` / `with_bit_set` / `with_bit_cleared`
//! / `with_bit_toggled` across UFixed, IFixed, and the primitive
//! bridges.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::ifixed::IFixed;
use arvo::{FBits, IBits, ibits, fbits, USize};
use arvo::strategy::{Hot, Warm};
use arvo::ufixed::UFixed;
use arvo_bits::{BitAccess, BitPrim, Byte, IBitPrim};

#[test]
fn bitprim_u8_roundtrip() {
    let v: u8 = 0b0000_0000;
    let v = v.with_bit_set(3);
    assert!(v.get_bit(3));
    assert!(!v.get_bit(2));
    let v = v.with_bit_set(7);
    assert_eq!(v, 0b1000_1000);
    let v = v.with_bit_cleared(3);
    assert_eq!(v, 0b1000_0000);
    let v = v.with_bit_toggled(0);
    assert_eq!(v, 0b1000_0001);
    // Out-of-range idx: value unchanged, read returns false.
    assert!(!v.get_bit(42));
    assert_eq!(v.with_bit_set(42), 0b1000_0001);
    assert_eq!(v.with_bit_cleared(42), 0b1000_0001);
    assert_eq!(v.with_bit_toggled(42), 0b1000_0001);
}

#[test]
fn bitprim_u64_high_bits() {
    let v: u64 = 0;
    let v = v.with_bit_set(63);
    assert!(v.get_bit(63));
    assert_eq!(v, 1u64 << 63);
    let v = v.with_bit_toggled(63);
    assert_eq!(v, 0);
}

#[test]
fn ibitprim_signed_roundtrip() {
    let v: i16 = 0;
    let v = v.with_bit_set(15);
    // Setting bit 15 on i16 flips sign (two's complement).
    assert!(v < 0);
    assert!(v.get_bit(15));
    let v = v.with_bit_cleared(15);
    assert_eq!(v, 0);
}

#[test]
fn ufixed_bit_access_hot_u8() {
    // Byte = UFixed<ibits(8), fbits(0), Hot> — container u8.
    let b = Byte::<Hot>::from_raw(0b0001_1000);
    assert!(!b.bit(USize(0)).0);
    assert!(b.bit(USize(3)).0);
    assert!(b.bit(USize(4)).0);
    assert!(!b.bit(USize(7)).0);

    let set = b.with_bit_set(USize(0));
    assert_eq!(set.to_raw(), 0b0001_1001);
    let cleared = b.with_bit_cleared(USize(3));
    assert_eq!(cleared.to_raw(), 0b0001_0000);
    let toggled = b.with_bit_toggled(USize(7));
    assert_eq!(toggled.to_raw(), 0b1001_1000);
}

#[test]
fn ufixed_bit_access_warm_wider_container() {
    // UFixed<ibits(8), fbits(0), Warm> — container is u16 (2x).
    // Bit semantics are LSB-first regardless of container width.
    type W = UFixed<{ ibits(8) }, { FBits::ZERO }, Warm>;
    let v = W::from_raw(0u16).with_bit_set(USize(5));
    assert!(v.bit(USize(5)).0);
    assert_eq!(v.to_raw(), 1u16 << 5);

    // Out-of-logical-range idx >= WIDTH is allowed by BitPrim, but the
    // contract documents idx >= WIDTH as non-panicking self-unchanged
    // for mutators. BitPrim's width is the container width; logical
    // out-of-range within the container is still a valid bit write
    // and reads back what was written. This test pins the primitive
    // behavior, not the logical-width policy (which consumers layer on).
    let v = W::from_raw(0u16).with_bit_set(USize(10));
    assert!(v.bit(USize(10)).0);
}

#[test]
fn ifixed_bit_access_hot_i16() {
    // IFixed<ibits(7), fbits(8), Hot> — 1 + 7 + 8 = 16 logical bits,
    // container is i16.
    type I = IFixed<{ ibits(7) }, { fbits(8) }, Hot>;
    let x = I::from_raw(0i16).with_bit_set(USize(15));
    // Bit 15 is the sign bit for i16 — setting it flips the signed
    // interpretation but the bit-level read is a plain `true`.
    assert!(x.bit(USize(15)).0);
    assert!(x.to_raw() < 0);
    let x = x.with_bit_toggled(USize(15));
    assert_eq!(x.to_raw(), 0);
}
