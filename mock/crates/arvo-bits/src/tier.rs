//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The bit tier: the contracts a bit container meets, and their impls for `Bits<N>`.
//!
//! Each trait is `const`, and each impl for `Bits<N>` calls the inherent method
//! of the same operation and does nothing else. A consumer generic over the
//! tier enables `const_trait_impl` where it evaluates the tier in a constant;
//! an ordinary generic function needs nothing. Another container, an array of
//! `Bits<64>` for a wider set among them, implements these in its own crate.
//!
//! The tier refuses an index or an amount at or past a container's width, once,
//! for every container. An implementor writes `BitAccessAt` and `BitSequenceBy`,
//! whose methods take a `Within` proof that only `Within::NEW` makes, and a
//! caller names `BitAccess` and `BitSequence`, each implemented once over every
//! implementor, making the proof. An impl of either by hand does not build, so
//! no container can take an index past its width.

use core::marker::PhantomData;

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

/// A proof that the index or amount `I` is below the width of the container `B`.
///
/// Made only by `NEW`, which asserts it inside a `const` item, so naming
/// `Within::<B, I>::NEW` with `I` at or past `B::WIDTH` is refused at
/// `cargo build`, through a call at run time and through anything generic that
/// reaches it alike. Nothing is checked at run time.
pub struct Within<B, const I: u32>(PhantomData<B>);

impl<B: BitWidth, const I: u32> Within<B, I> {
    /// The proof, refused at build where `I` is not below `B::WIDTH`.
    // The mutant `arvo_mutant_bits_index` admits the width itself. trybuild
    // removes `RUSTFLAGS` from its own build, so the `compile_fail` cases never
    // see it; the red run builds the index cases as examples instead.
    pub const NEW: Self = {
        let width = B::WIDTH.count();
        assert!(
            if cfg!(arvo_mutant_bits_index) { I <= width } else { I < width },
            "a bit index or a shift amount at or past the container's declared width; bits are \
             counted from zero at the low bit, so the last index of a container `w` bits wide is \
             `w - 1`, and a shift or rotation by `w` or more is no move within the width"
        );
        Self(PhantomData)
    };
}

/// One bit read and written at a proven index: the side an implementor writes.
pub const trait BitAccessAt: BitWidth + Copy {
    /// Whether bit `I` is set.
    fn bit_at<const I: u32>(self, at: Within<Self, I>) -> Bool;

    /// This value with bit `I` set to `to` and every other bit as it was.
    fn with_bit_at<const I: u32>(self, at: Within<Self, I>, to: Bool) -> Self;
}

/// One bit read and written, counting from the low bit at zero: the side a
/// caller names.
///
/// Implemented once, over every `BitAccessAt`, which makes the proof that `I`
/// is below the width, so an index at or past it is refused at build.
pub const trait BitAccess: BitAccessAt {
    /// Whether bit `I` is set.
    fn bit<const I: u32>(self) -> Bool;

    /// This value with bit `I` set to `to` and every other bit as it was.
    fn with_bit<const I: u32>(self, to: Bool) -> Self;
}

const impl<B: [const] BitAccessAt> BitAccess for B {
    fn bit<const I: u32>(self) -> Bool {
        self.bit_at(Within::<Self, I>::NEW)
    }

    fn with_bit<const I: u32>(self, to: Bool) -> Self {
        self.with_bit_at(Within::<Self, I>::NEW, to)
    }
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

/// The bits moved within the width by a proven amount: the side an
/// implementor writes.
pub const trait BitSequenceBy: BitWidth + Copy {
    /// Every bit moved `K` places up, zeros brought in.
    fn shift_up_by<const K: u32>(self, by: Within<Self, K>) -> Self;

    /// Every bit moved `K` places down, zeros brought in.
    fn shift_down_by<const K: u32>(self, by: Within<Self, K>) -> Self;

    /// Every bit moved `K` places up, the top brought in at the bottom.
    fn rotate_up_by<const K: u32>(self, by: Within<Self, K>) -> Self;

    /// Every bit moved `K` places down, the bottom brought in at the top.
    fn rotate_down_by<const K: u32>(self, by: Within<Self, K>) -> Self;
}

/// The bits moved within the width by an amount fixed at build: the side a
/// caller names.
///
/// Up is toward the high bit. A shift drops what leaves the width and a
/// rotation brings it in at the other end, and zero is the identity.
/// Implemented once, over every `BitSequenceBy`, which makes the proof that
/// `K` is below the width, so an amount at or past it is refused at build.
pub const trait BitSequence: BitSequenceBy {
    /// Every bit moved `K` places up, zeros brought in.
    fn shift_up<const K: u32>(self) -> Self;

    /// Every bit moved `K` places down, zeros brought in.
    fn shift_down<const K: u32>(self) -> Self;

    /// Every bit moved `K` places up, the top brought in at the bottom.
    fn rotate_up<const K: u32>(self) -> Self;

    /// Every bit moved `K` places down, the bottom brought in at the top.
    fn rotate_down<const K: u32>(self) -> Self;
}

const impl<B: [const] BitSequenceBy> BitSequence for B {
    fn shift_up<const K: u32>(self) -> Self {
        self.shift_up_by(Within::<Self, K>::NEW)
    }

    fn shift_down<const K: u32>(self) -> Self {
        self.shift_down_by(Within::<Self, K>::NEW)
    }

    fn rotate_up<const K: u32>(self) -> Self {
        self.rotate_up_by(Within::<Self, K>::NEW)
    }

    fn rotate_down<const K: u32>(self) -> Self {
        self.rotate_down_by(Within::<Self, K>::NEW)
    }
}

const impl<const N: u32> BitWidth for Bits<N> {
    // Forced, so the width of `Bits<0>` or `Bits<65>` is refused like the type.
    const WIDTH: Width = {
        let () = Self::ADMITTED;
        Width::bits(N)
    };
}

const impl<const N: u32> BitAccessAt for Bits<N> {
    fn bit_at<const I: u32>(self, _: Within<Self, I>) -> Bool {
        Bits::<N>::bit::<I>(self)
    }

    fn with_bit_at<const I: u32>(self, _: Within<Self, I>, to: Bool) -> Self {
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

const impl<const N: u32> BitSequenceBy for Bits<N> {
    fn shift_up_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        Bits::<N>::shift_up::<K>(self)
    }

    fn shift_down_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        Bits::<N>::shift_down::<K>(self)
    }

    fn rotate_up_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        Bits::<N>::rotate_up::<K>(self)
    }

    fn rotate_down_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        Bits::<N>::rotate_down::<K>(self)
    }
}
