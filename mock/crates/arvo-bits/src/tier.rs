//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The bit tier: the contracts a bit container meets, and their impls for `Bits<N>`.
//!
//! Each trait is `const`, and each impl for `Bits<N>` calls the inherent method
//! of the same name and does nothing else. A consumer generic over the tier
//! enables `const_trait_impl` where it evaluates the tier in a constant; an
//! ordinary generic function needs nothing. Another container, an array of
//! `Bits<64>` for a wider set among them, implements these in its own crate.
//!
//! An implementor of `BitAccess` or `BitSequence` owes a refusal at build of
//! an index or an amount at or past its `WIDTH`, by an assertion of its own over
//! its own width, held by refusal cases of its own. No trait can enforce that,
//! so it is stated on each trait as the obligation it is.

use arvo_format::{Bool, Width};

use crate::Bits;

/// The declared width of a bit container.
///
/// The one thing a bound generic over the tier has to give back, since the
/// container's own width is not in scope there.
pub const trait BitWidth {
    /// The declared width.
    const WIDTH: Width;
}

/// One bit read and written, counting from the low bit at zero.
///
/// An index at or past the width is refused at build. That is an obligation on
/// the implementor, met for `Bits<N>` by an assertion inside each method; the
/// trait cannot enforce it.
pub const trait BitAccess: BitWidth + Copy {
    /// Whether bit `I` is set.
    fn bit<const I: u32>(self) -> Bool;

    /// This value with bit `I` set to `to` and every other bit as it was.
    fn with_bit<const I: u32>(self, to: Bool) -> Self;
}

/// The logic of two values at one width, and the emptiness test.
///
/// Intersection is `and`, union is `or`, and whether two sets meet is
/// `a.and(b).is_empty()`. `not` flips every bit of the width `BitWidth` gives.
pub const trait BitLogic: BitWidth + Copy {
    /// The bitwise and.
    fn and(self, other: Self) -> Self;

    /// The bitwise or.
    fn or(self, other: Self) -> Self;

    /// The bitwise exclusive or.
    fn xor(self, other: Self) -> Self;

    /// Every bit of the width flipped.
    fn not(self) -> Self;

    /// Whether no bit is set.
    fn is_empty(self) -> Bool;
}

/// The bits moved within the width by an amount fixed at build.
///
/// Up is toward the high bit. A shift drops what leaves the width and a
/// rotation brings it in at the other end. An amount at or past the width is
/// refused at build, and zero is the identity. The refusal is an obligation on
/// the implementor, as on `BitAccess`; the width is `BitWidth`'s.
pub const trait BitSequence: BitWidth + Copy {
    /// Every bit moved `K` places up, zeros brought in.
    fn shift_up<const K: u32>(self) -> Self;

    /// Every bit moved `K` places down, zeros brought in.
    fn shift_down<const K: u32>(self) -> Self;

    /// Every bit moved `K` places up, the top brought in at the bottom.
    fn rotate_up<const K: u32>(self) -> Self;

    /// Every bit moved `K` places down, the bottom brought in at the top.
    fn rotate_down<const K: u32>(self) -> Self;
}

const impl<const N: u32> BitWidth for Bits<N> {
    // Forced, so the width of `Bits<0>` or `Bits<65>` is refused like the type.
    const WIDTH: Width = {
        let () = Self::ADMITTED;
        Width::bits(N)
    };
}

const impl<const N: u32> BitAccess for Bits<N> {
    fn bit<const I: u32>(self) -> Bool {
        Bits::<N>::bit::<I>(self)
    }

    fn with_bit<const I: u32>(self, to: Bool) -> Self {
        Bits::<N>::with_bit::<I>(self, to)
    }
}

const impl<const N: u32> BitLogic for Bits<N> {
    fn and(self, other: Self) -> Self {
        Bits::<N>::and(self, other)
    }

    fn or(self, other: Self) -> Self {
        Bits::<N>::or(self, other)
    }

    fn xor(self, other: Self) -> Self {
        Bits::<N>::xor(self, other)
    }

    fn not(self) -> Self {
        Bits::<N>::not(self)
    }

    fn is_empty(self) -> Bool {
        Bits::<N>::is_empty(self)
    }
}

const impl<const N: u32> BitSequence for Bits<N> {
    fn shift_up<const K: u32>(self) -> Self {
        Bits::<N>::shift_up::<K>(self)
    }

    fn shift_down<const K: u32>(self) -> Self {
        Bits::<N>::shift_down::<K>(self)
    }

    fn rotate_up<const K: u32>(self) -> Self {
        Bits::<N>::rotate_up::<K>(self)
    }

    fn rotate_down<const K: u32>(self) -> Self {
        Bits::<N>::rotate_down::<K>(self)
    }
}
