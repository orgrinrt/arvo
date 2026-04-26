//! Alias-width sanity: each semantic alias resolves to the expected
//! logical width and participates in the bit-level surface.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::USize;
use arvo::strategy::{Hot, Warm};
use arvo_bits::{Bit, BitAccess, BitSequence, HasBitWidth, Byte, DWord, Nibble, QWord, Word};

#[test]
fn alias_logical_widths() {
    assert_eq!(<Bit<Hot> as HasBitWidth>::WIDTH.0, 1);
    assert_eq!(<Nibble<Hot> as HasBitWidth>::WIDTH.0, 4);
    assert_eq!(<Byte<Hot> as HasBitWidth>::WIDTH.0, 8);
    assert_eq!(<Word<Hot> as HasBitWidth>::WIDTH.0, 16);
    assert_eq!(<DWord<Hot> as HasBitWidth>::WIDTH.0, 32);
    assert_eq!(<QWord<Hot> as HasBitWidth>::WIDTH.0, 64);
}

#[test]
fn bit_alias_holds_one_bit() {
    let b = Bit::<Hot>::from_raw(1u8);
    assert!(b.bit(USize(0)).0);
    assert_eq!(b.count_ones().0, 1);
    let b = b.with_bit_cleared(USize(0));
    assert!(b.is_zero().0);
}

#[test]
fn byte_alias_roundtrip_through_access() {
    let b = Byte::<Hot>::from_raw(0);
    let b = b.with_bit_set(USize(0)).with_bit_set(USize(7));
    assert_eq!(b.to_raw(), 0b1000_0001);
    assert_eq!(b.count_ones().0, 2);
    assert_eq!(b.trailing_zeros().0, 0);
    assert_eq!(b.leading_zeros().0, 0);
}

#[test]
fn dword_warm_strategy_instantiable() {
    // Confirms aliases are usable with a non-default strategy by
    // instantiating the type parameter and exercising the bit surface.
    type DW = DWord<Warm>;
    assert_eq!(<DW as HasBitWidth>::WIDTH.0, 32);
    let d = DW::from_raw(0u64).with_bit_set(USize(4));
    assert!(d.bit(USize(4)).0);
    assert_eq!(d.count_ones().0, 1);
}
