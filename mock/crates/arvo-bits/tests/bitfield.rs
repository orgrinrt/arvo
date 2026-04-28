//! Smoke tests for the `bitfield!` macro.
//!
//! Post-pass-5/6: per-size helpers on `Bits` are gone. Tests use
//! `Bits::from_raw(<container>)` and `Bits::to_raw()` directly.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(macro_metavar_expr_concat)]
#![allow(incomplete_features)]

use arvo::bitfield;
use arvo_bits::{Bit, Bits, Hot};

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
    // Bits<32, Hot> uses u32 container; Bits<28, Hot> u32; Bits<3, Hot> u8;
    // Bits<1, Hot> u8.
    // lint:allow(no-bare-numeric) reason: test assertion against raw container; tracked: #256
    assert_eq!(h.to_bits().to_raw(), 0_u32);
    // lint:allow(no-bare-numeric) reason: test assertion against raw container; tracked: #256
    assert_eq!(h.origin().to_raw(), 0_u8);
    // lint:allow(no-bare-numeric) reason: test assertion against raw container; tracked: #256
    assert_eq!(h.reserved().to_raw(), 0_u8);
    // lint:allow(no-bare-numeric) reason: test assertion against raw container; tracked: #256
    assert_eq!(h.id().to_raw(), 0_u32);
}

#[test]
fn round_trip_origin_and_id() {
    let h = StrHandle::new()
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
        .with_origin(Bit::<Hot>::from_raw(1_u8))
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
        .with_id(Bits::<28, Hot>::from_raw(0x1234_u32));
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.origin().to_raw(), 1_u8);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.id().to_raw(), 0x1234_u32);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.reserved().to_raw(), 0_u8);
}

#[test]
fn setting_one_field_preserves_others() {
    let h = StrHandle::new()
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
        .with_origin(Bit::<Hot>::from_raw(1_u8))
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
        .with_id(Bits::<28, Hot>::from_raw(0x42_u32))
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
        .with_reserved(Bits::<3, Hot>::from_raw(0b101_u8));
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.origin().to_raw(), 1_u8);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.reserved().to_raw(), 0b101_u8);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.id().to_raw(), 0x42_u32);
}

#[test]
fn setter_truncates_to_subrange_width() {
    // Pre-pass-5 the `Bits<28>::new(0xFFFF_FFFF)` constructor would mask
    // the input to 28 bits implicitly. Post-deletion, the test must
    // mask itself before constructing.
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side mask + container literal; tracked: #256
    let id_value: Bits<28, Hot> = Bits::from_raw((0xFFFF_FFFF_u32) & 0x0FFF_FFFF_u32);
    let h = StrHandle::new().with_id(id_value);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.id().to_raw(), 0x0FFF_FFFF_u32);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.origin().to_raw(), 0_u8);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.reserved().to_raw(), 0_u8);
}

#[test]
fn from_bits_to_bits_round_trip() {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test-side raw container literal; tracked: #256
    let raw: Bits<32, Hot> = Bits::from_raw(0x8000_0042_u32);
    let h = StrHandle::from_bits(raw);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.to_bits().to_raw(), 0x8000_0042_u32);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.origin().to_raw(), 1_u8);
    // lint:allow(no-bare-numeric) reason: test assertion; tracked: #256
    assert_eq!(h.id().to_raw(), 0x42_u32);
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

#[test]
fn generated_field_masks_match_layout() {
    // lint:allow(no-bare-numeric) reason: test assertion against macro-emitted mask; tracked: #256
    assert_eq!(StrHandle::origin_MASK.to_raw(), 1_u32 << 31);
    // lint:allow(no-bare-numeric) reason: test assertion against macro-emitted mask; tracked: #256
    assert_eq!(StrHandle::reserved_MASK.to_raw(), 0b111_u32 << 28);
    // lint:allow(no-bare-numeric) reason: test assertion against macro-emitted mask; tracked: #256
    assert_eq!(StrHandle::id_MASK.to_raw(), 0x0FFF_FFFF_u32);
}
