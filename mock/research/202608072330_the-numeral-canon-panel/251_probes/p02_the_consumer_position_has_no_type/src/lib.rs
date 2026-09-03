//! The consumer's own shipped position, written against arvo as it stands.
//!
//! `hilavitkutin-linking/src/error.rs:30` on `origin/dev` at `313d427a` reads
//! `LoadFailed { platform_code: USize }`, and `obligation::a_platform_sized_unsigned_integer_at_an_api_position`
//! quotes that crate's design for the need. So this writes the same field and
//! asks arvo for a type to put in it.
//!
//! **The arm that matters does not compile and is in `the_position.rs` beside
//! this file**, kept out of the build so this crate builds and its controls run.
//! What compiles here is the enumeration of what arvo does export, which is what
//! establishes that the refusal is an absence rather than a spelling mistake.

#![no_std]

use core::mem::size_of;

use arvo_format::points::{Integer, UFixed};
use arvo_format::{Bool, Slot, Width};

/// A platform error code is a value. The four points are the only numeral-shaped
/// things arvo exports, and none of them can hold one.
///
/// Stated as the size, because a zero-sized field carries no code whatever the
/// declaration around it says.
pub const NO_POINT_CAN_CARRY_A_CODE: () = {
    assert!(size_of::<Integer<32>>() == 0);
    assert!(size_of::<UFixed<32, 0>>() == 0);
};

/// What is left, and why each is the wrong thing rather than a near miss.
///
/// `Width` is a count of bits and `Slot` indexes a representable set, so putting
/// an errno in either is a category error rather than a width one. `Bool` holds
/// no number at all. The assertions pin the widths so a later reader can see the
/// range argument is not what decides this.
pub const WHAT_IS_LEFT_IS_A_COORDINATE_NOT_A_NUMERAL: () = {
    assert!(size_of::<Width>() == 4);
    assert!(size_of::<Slot>() == 8);
    assert!(size_of::<Bool>() == 1);
};

/// The control. `Width` would hold a `GetLastError` value on range alone, so the
/// refusal above is not a range finding and nobody should read it as one.
///
/// `GetLastError` is a `DWORD`, and `errno` is a C `int`; both fit in 32 bits.
pub const CONTROL_RANGE_IS_NOT_WHAT_REFUSES_THIS: () = {
    assert!(Width::bits(u32::MAX).count() == u32::MAX);
};
