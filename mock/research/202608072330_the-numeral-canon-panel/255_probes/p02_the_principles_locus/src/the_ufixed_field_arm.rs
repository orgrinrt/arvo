//! The second arm that does not compile, kept out of the build on purpose.
//!
//! A consumer following `mock/PRINCIPLES.md.tmpl:220` puts `UFixed` where a
//! `u32` field would go and expects the field to hold a 32-bit value. The const
//! assertion states that expectation and const evaluation refuses it, because
//! the type is a format declaration and not a numeral.

use core::mem::size_of;

use arvo_format::points::UFixed;

pub struct ErrorCode(UFixed<32, 0>);

pub const THE_FIELD_HOLDS_A_CODE: () = {
    assert!(size_of::<ErrorCode>() == 4);
};
