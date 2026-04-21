//! Smoke tests for the `bitfield!` macro.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(macro_metavar_expr_concat)]
#![allow(incomplete_features)]

use arvo_bits::{bitfield, Bit, Bits, Hot};

bitfield! {
    /// Interned string handle layout for smoke-testing.
    pub struct StrHandle: 32 {
        /// 1 = runtime-interned, 0 = compile-time.
        origin: 1 at 31,
        /// Reserved flag bits.
        reserved: 3 at 28,
        /// 28-bit interned identity.
        id: 28 at 0,
    }
}

#[test]
fn size_matches_4_bytes() {
    assert_eq!(core::mem::size_of::<StrHandle>(), 4);
}

#[test]
fn new_is_zero() {
    let h = StrHandle::new();
    assert_eq!(h.to_bits().bits(), 0);
    assert_eq!(h.origin().bits(), 0);
    assert_eq!(h.reserved().bits(), 0);
    assert_eq!(h.id().bits(), 0);
}

#[test]
fn round_trip_origin_and_id() {
    let h = StrHandle::new()
        .with_origin(Bit::<Hot>::new(1))
        .with_id(Bits::<28, Hot>::new(0x1234));
    assert_eq!(h.origin().bits(), 1);
    assert_eq!(h.id().bits(), 0x1234);
    assert_eq!(h.reserved().bits(), 0);
}

#[test]
fn setting_one_field_preserves_others() {
    let h = StrHandle::new()
        .with_origin(Bit::<Hot>::new(1))
        .with_id(Bits::<28, Hot>::new(0x42))
        .with_reserved(Bits::<3, Hot>::new(0b101));
    assert_eq!(h.origin().bits(), 1);
    assert_eq!(h.reserved().bits(), 0b101);
    assert_eq!(h.id().bits(), 0x42);
}

#[test]
fn setter_truncates_to_subrange_width() {
    // Pass a value whose low bits fit but whose stored Bits<W>
    // would have already masked. Confirm the setter doesn't
    // corrupt other fields via over-shift.
    let h = StrHandle::new()
        .with_id(Bits::<28, Hot>::new(0xFFFF_FFFF)); // Bits<28>::new masks to 28 bits
    assert_eq!(h.id().bits(), 0x0FFF_FFFF);
    assert_eq!(h.origin().bits(), 0);
    assert_eq!(h.reserved().bits(), 0);
}

#[test]
fn from_bits_to_bits_round_trip() {
    let raw = Bits::<32, Hot>::new(0x8000_0042);
    let h = StrHandle::from_bits(raw);
    assert_eq!(h.to_bits().bits(), 0x8000_0042);
    assert_eq!(h.origin().bits(), 1);
    assert_eq!(h.id().bits(), 0x42);
}

// Bits<N> size assertions — belt-and-suspenders on the container
// dispatch covered by bits.rs tests.
#[test]
fn bits_sizes_match_container() {
    assert_eq!(core::mem::size_of::<Bits<8, Hot>>(), 1);
    assert_eq!(core::mem::size_of::<Bits<16, Hot>>(), 2);
    assert_eq!(core::mem::size_of::<Bits<28, Hot>>(), 4);
    assert_eq!(core::mem::size_of::<Bits<32, Hot>>(), 4);
    assert_eq!(core::mem::size_of::<Bits<64, Hot>>(), 8);
}
