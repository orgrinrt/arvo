//! `BitSequence` coverage: `count_ones` / `count_zeros` /
//! `trailing_zeros` / `leading_zeros` / `is_zero`.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{FBits, IBits, ibits, fbits};
use arvo::strategy::{Hot, Warm};
use arvo::ufixed::UFixed;
use arvo_bits::{BitSequence, Byte, DWord, QWord};

#[test]
fn byte_count_ones_and_zeros() {
    let b = Byte::<Hot>::from_raw(0b1010_1100);
    assert_eq!(b.count_ones().0, 4);
    assert_eq!(b.count_zeros().0, 4);
    let zero = Byte::<Hot>::from_raw(0);
    assert_eq!(zero.count_ones().0, 0);
    assert_eq!(zero.count_zeros().0, 8);
    let full = Byte::<Hot>::from_raw(0xff);
    assert_eq!(full.count_ones().0, 8);
    assert_eq!(full.count_zeros().0, 0);
}

#[test]
fn byte_trailing_and_leading_zeros() {
    let b = Byte::<Hot>::from_raw(0b0001_0000);
    assert_eq!(b.trailing_zeros().0, 4);
    // Byte is Hot: container is u8, same width as logical. Leading
    // zeros on 0b0001_0000 is 3.
    assert_eq!(b.leading_zeros().0, 3);
}

#[test]
fn qword_high_bit_leading_zeros() {
    // QWord = UFixed<ibits(64), fbits(0), Hot>. 1 << 63 -> lz = 0.
    let q = QWord::<Hot>::from_raw(1u64 << 63);
    assert_eq!(q.leading_zeros().0, 0);
    assert_eq!(q.trailing_zeros().0, 63);
}

#[test]
fn warm_leading_zeros_accounts_for_wider_container() {
    // UFixed<ibits(8), fbits(0), Warm> -> logical width 8, container
    // is u16 (16 bits). A value of 1u16 << 7 has container leading
    // zeros = 8; logical leading zeros = 0 (bit 7 is the MSB of the
    // logical 8-bit window).
    type W = UFixed<{ ibits(8) }, { FBits::ZERO }, Warm>;
    let v = W::from_raw(1u16 << 7);
    assert_eq!(v.leading_zeros().0, 0);
    // Value 1: logical leading zeros = 7 (bit 0 set, 7 zeros above it
    // within the logical 8-bit window).
    let v = W::from_raw(1u16);
    assert_eq!(v.leading_zeros().0, 7);
}

#[test]
fn is_zero_across_strategies() {
    let b = Byte::<Hot>::from_raw(0);
    assert!(b.is_zero().0);
    let b = Byte::<Hot>::from_raw(1);
    assert!(!b.is_zero().0);

    // Warm: container wider than logical width; is_zero still checks
    // the container, which is correct because consumers keep values
    // within logical range.
    type W = UFixed<{ ibits(8) }, { FBits::ZERO }, Warm>;
    let v = W::from_raw(0u16);
    assert!(v.is_zero().0);
    let v = W::from_raw(1u16);
    assert!(!v.is_zero().0);
}

#[test]
fn dword_count_ones() {
    let d = DWord::<Hot>::from_raw(0xffff_0000u32);
    assert_eq!(d.count_ones().0, 16);
    assert_eq!(d.count_zeros().0, 16);
    let d = DWord::<Hot>::from_raw(0u32);
    assert_eq!(d.trailing_zeros().0, 32);
}
