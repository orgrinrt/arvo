#![no_std]
#![feature(adt_const_params)]
//! The door, holding the gate, declaring a width as its own type in the one
//! position the language refuses without it.
//!
//! `adt_const_params` rather than `min_adt_const_params`: the compiler's help
//! text on this pin names the second, the workspace register carries a row for
//! the first in its Allowed tier, and the second refuses a newtype whose field
//! is private. `k6` is where that was measured.

use core::marker::ConstParamTy;

/// A count of bits, with the field still private.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, ConstParamTy)]
pub struct Width(u32);

impl Width {
    /// A width from a count of bits.
    #[must_use]
    pub const fn bits(n: u32) -> Self { Self(n) }
    /// The count, for the one place a host contract needs it back.
    #[must_use]
    pub const fn count(self) -> u32 { self.0 }
}

/// A slot range of a declared width, with the width as the door's own type.
pub struct Signed<const BITS: Width>;

/// What a declaration of that width admits.
pub trait Slots {
    /// The width the declaration stated.
    const WIDTH: Width;
}

impl<const BITS: Width> Slots for Signed<BITS> {
    const WIDTH: Width = BITS;
}
