//! Does arvo ship anything that holds a numeral's value?
//!
//! The question the five obligation rows presume an answer to. Each of them asks
//! for a primitive at an API position, and a primitive at an API position has to
//! be able to hold the number that position carries.
//!
//! What this checks, and the control is the point: the four shipped points of the
//! ratified parameterisation are all zero sized, so each declares which values
//! exist and holds none of them, while the crate's own coordinate newtypes are
//! not zero sized, which is what shows the check can tell the two apart.

#![no_std]

use arvo_format::points::{Biased, Floating, Integer, UFixed};
use arvo_format::{Bool, Slot, Width};
use core::mem::size_of;

/// Every shipped point of the parameterisation is zero sized.
///
/// A declaration of a value set, not a member of one. Asserted over all four
/// points and at several widths each, because a law asserted at one shape is a
/// law measured at one shape.
pub const POINTS_CARRY_NO_VALUE: () = {
    assert!(size_of::<Integer<8>>() == 0);
    assert!(size_of::<Integer<13>>() == 0);
    assert!(size_of::<Integer<32>>() == 0);
    assert!(size_of::<Integer<64>>() == 0);
    assert!(size_of::<UFixed<8, -4>>() == 0);
    assert!(size_of::<UFixed<13, -7>>() == 0);
    assert!(size_of::<UFixed<32, 0>>() == 0);
    assert!(size_of::<Biased<8, 0, 1>>() == 0);
    assert!(size_of::<Biased<16, -3, 2>>() == 0);
    assert!(size_of::<Floating<10, -14, 30>>() == 0);
    assert!(size_of::<Floating<23, -126, 254>>() == 0);
};

/// The control. The coordinate newtypes are not zero sized, so the check above
/// is not simply true of every name arvo exports.
///
/// Without this, `POINTS_CARRY_NO_VALUE` would pass equally against a crate that
/// happened to make everything a marker, and would prove nothing about the split
/// between a declaration and a value.
pub const CONTROL_THE_COORDINATES_DO_CARRY_A_VALUE: () = {
    assert!(size_of::<Width>() > 0);
    assert!(size_of::<Bool>() > 0);
    assert!(size_of::<Slot>() > 0);
};

/// The second control, and the one that says what kind of thing the coordinates
/// are. Each is a coordinate of a declaration rather than a numeral a consumer
/// computes in: `Width` counts bits, `Slot` indexes the representable set.
///
/// Stated as sizes because that is what a probe can reach. The category claim it
/// supports is in the file, not here.
pub const CONTROL_THE_COORDINATES_ARE_HOST_WIDTH_NEWTYPES: () = {
    assert!(size_of::<Width>() == size_of::<u32>());
    assert!(size_of::<Bool>() == size_of::<bool>());
    assert!(size_of::<Slot>() == size_of::<i64>());
};
