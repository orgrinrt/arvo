//! Unit tests for `Bits<N>`: construction, masking, HasBitWidth,
//! BitAccess.
//!
//! Post-pass-5 the per-size helper surface (`Bits::new(u64)` /
//! `.bits()` / `from_raw_uN` / `to_raw_uN`) is gone. Tests now go
//! through `Bits::from_raw(<Hot as UContainerFor<N>>::T)` and
//! `Bits::to_raw()`. Container types are concrete per N: u8 for
//! 1..=8, u16 for 9..=16, u32 for 17..=32, u64 for 33..=64.

use arvo::{Bool, USize};
use arvo_bits::{BitAccess, HasBitWidth, Bits};

#[test]
fn from_raw_round_trips_u8_container() {
    // Bits<4> uses u8 container under Hot. The caller is responsible
    // for masking to 4 bits before construction.
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let b: Bits<4> = Bits::from_raw(0x0F_u8);
    // lint:allow(no-bare-numeric) reason: test assertion against raw container; tracked: #256
    assert_eq!(b.to_raw(), 0x0F_u8);
}

#[test]
fn from_raw_round_trips_u64_container() {
    // Bits<64> uses u64 container under Hot.
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let b: Bits<64> = Bits::from_raw(0xFFFF_FFFF_FFFF_FFFF_u64);
    // lint:allow(no-bare-numeric) reason: test assertion against raw container; tracked: #256
    assert_eq!(b.to_raw(), 0xFFFF_FFFF_FFFF_FFFF_u64);
}

#[test]
fn width_matches_n() {
    assert_eq!(<Bits<28> as HasBitWidth>::WIDTH, USize(28));
    assert_eq!(<Bits<64> as HasBitWidth>::WIDTH, USize(64));
}

#[test]
fn bit_access_read() {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let b: Bits<8> = Bits::from_raw(0b1010_0101_u8);
    assert_eq!(b.bit(USize(0)), Bool(true));
    assert_eq!(b.bit(USize(1)), Bool(false));
    assert_eq!(b.bit(USize(7)), Bool(true));
}

#[test]
fn bit_access_out_of_range_reads_false() {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let b: Bits<8> = Bits::from_raw(0xFF_u8);
    assert_eq!(b.bit(USize(8)), Bool::FALSE);
    assert_eq!(b.bit(USize(63)), Bool::FALSE);
    assert_eq!(b.bit(USize(1000)), Bool::FALSE);
}

#[test]
fn bit_access_set_clear_toggle() {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let b: Bits<8> = Bits::from_raw(0b0000_0000_u8);
    let set = b.with_bit_set(USize(3));
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(set.to_raw(), 0b0000_1000_u8);
    let cleared = set.with_bit_cleared(USize(3));
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(cleared.to_raw(), 0b0000_0000_u8);
    let toggled = set.with_bit_toggled(USize(3));
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(toggled.to_raw(), 0b0000_0000_u8);
}

#[test]
fn bit_access_out_of_range_mutators_noop() {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let b: Bits<8> = Bits::from_raw(0xAA_u8);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(b.with_bit_set(USize(100)).to_raw(), 0xAA_u8);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(b.with_bit_cleared(USize(100)).to_raw(), 0xAA_u8);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(b.with_bit_toggled(USize(100)).to_raw(), 0xAA_u8);
}

#[test]
fn equality() {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let a: Bits<16> = Bits::from_raw(0x1234_u16);
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let b: Bits<16> = Bits::from_raw(0x1234_u16);
    assert_eq!(a, b);
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let c: Bits<16> = Bits::from_raw(0x1235_u16);
    assert_ne!(a, c);
}

#[test]
fn default_is_zero() {
    let b: Bits<16> = Bits::default();
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(b.to_raw(), 0_u16);
}
