//! What `mock/PRINCIPLES.md.tmpl:220` tells a consumer, compiled.
//!
//! The rendered principles page, `docs/PRINCIPLES.md:229`, says that where a
//! bare `u8..u128` or `i8..i128` would sit, the consumer writes `UFixed` or
//! `IFixed`, and that both live in `arvo-format`. This crate is a consumer doing
//! exactly that.
//!
//! **What compiles here is the half that establishes the claim is false rather
//! than merely unbuilt**: `arvo_format::points::UFixed` exists and is zero sized
//! at every instantiation tried, so a field of that type holds no value. The two
//! arms that do not compile are beside this file, kept out of the build so the
//! crate builds and this control runs, with their stderr committed.

#![no_std]

use core::mem::size_of;

use arvo_format::Width;
use arvo_format::points::UFixed;

/// The type the principles page names holds nothing.
///
/// A 32-bit unsigned integer written the way the page says to write it, and the
/// field it declares is zero bytes wide.
pub const THE_NAMED_TYPE_HOLDS_NO_VALUE: () = {
    assert!(size_of::<UFixed<8, 0>>() == 0);
    assert!(size_of::<UFixed<32, 0>>() == 0);
    assert!(size_of::<UFixed<64, 0>>() == 0);
};

/// The control: a coordinate newtype from the same crate is not zero sized.
///
/// Without this, the assertion above passes equally against a crate whose every
/// export is a marker.
pub const A_COORDINATE_IS_NOT_ZERO_SIZED: () = {
    assert!(size_of::<Width>() == 4);
};
