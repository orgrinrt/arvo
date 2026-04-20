//! Unit tests for `Bits<N>`: construction, masking, BitWidth,
//! BitAccess.

use arvo::{Bool, USize};
use arvo_bits::{BitAccess, BitWidth, Bits};

#[test]
fn new_masks_high_bits() {
    let b = Bits::<4>::new(0xFF);
    assert_eq!(b.bits(), 0x0F);
}

#[test]
fn new_full_width_preserves() {
    let b = Bits::<64>::new(0xFFFF_FFFF_FFFF_FFFF);
    assert_eq!(b.bits(), 0xFFFF_FFFF_FFFF_FFFF);
}

#[test]
fn width_matches_n() {
    assert_eq!(<Bits<28> as BitWidth>::WIDTH, USize(28));
    assert_eq!(<Bits<64> as BitWidth>::WIDTH, USize(64));
}

#[test]
fn bit_access_read() {
    let b = Bits::<8>::new(0b1010_0101);
    assert_eq!(b.bit(USize(0)), Bool(true));
    assert_eq!(b.bit(USize(1)), Bool(false));
    assert_eq!(b.bit(USize(7)), Bool(true));
}

#[test]
fn bit_access_out_of_range_reads_false() {
    let b = Bits::<8>::new(0xFF);
    assert_eq!(b.bit(USize(8)), Bool::FALSE);
    assert_eq!(b.bit(USize(63)), Bool::FALSE);
    assert_eq!(b.bit(USize(1000)), Bool::FALSE);
}

#[test]
fn bit_access_set_clear_toggle() {
    let b = Bits::<8>::new(0b0000_0000);
    let set = b.with_bit_set(USize(3));
    assert_eq!(set.bits(), 0b0000_1000);
    let cleared = set.with_bit_cleared(USize(3));
    assert_eq!(cleared.bits(), 0b0000_0000);
    let toggled = set.with_bit_toggled(USize(3));
    assert_eq!(toggled.bits(), 0b0000_0000);
}

#[test]
fn bit_access_out_of_range_mutators_noop() {
    let b = Bits::<8>::new(0xAA);
    assert_eq!(b.with_bit_set(USize(100)).bits(), 0xAA);
    assert_eq!(b.with_bit_cleared(USize(100)).bits(), 0xAA);
    assert_eq!(b.with_bit_toggled(USize(100)).bits(), 0xAA);
}

#[test]
fn equality() {
    let a = Bits::<16>::new(0x1234);
    let b = Bits::<16>::new(0x1234);
    assert_eq!(a, b);
    let c = Bits::<16>::new(0x1235);
    assert_ne!(a, c);
}

#[test]
fn default_is_zero() {
    let b: Bits<16> = Bits::default();
    assert_eq!(b.bits(), 0);
}
