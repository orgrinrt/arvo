//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The bit operations on `Bits<N>`, each an inherent `const fn`.
//!
//! The traits in `tier` call these and do nothing else, so a consumer naming
//! `Bits<N>` needs no feature of the toolchain to reach them.

use arvo_format::Bool;

use crate::Bits;

/// An index or an amount `I` below the width `N`, checked where it is used.
///
/// Evaluated where `ADMITTED` is named, inside the method that takes `I`, so a
/// call with `I` at or past `N` is refused at `cargo build` through that
/// method and through anything generic that reaches it, the shape
/// `Bits::ADMITTED` has.
struct Within<const N: u32, const I: u32>;

impl<const N: u32, const I: u32> Within<N, I> {
    // The mutant `arvo_mutant_bits_index` admits the width itself, which the
    // `compile_fail` cases have to catch.
    const ADMITTED: () = assert!(
        if cfg!(arvo_mutant_bits_index) { I <= N } else { I < N },
        "a bit index or a shift amount at or past the declared width; bits are counted from zero \
         at the low bit, so the last index of `Bits<N>` is `N - 1`, and a shift or rotation by `N` \
         or more is no move within the width"
    );
}

impl<const N: u32> Bits<N> {
    /// Whether bit `I` is set, counting from the low bit at zero.
    #[must_use]
    pub const fn bit<const I: u32>(self) -> Bool {
        let () = Within::<N, I>::ADMITTED;
        Bool::of((self.0 >> I) & 1 == 1)
    }

    /// This value with bit `I` set to `to` and every other bit as it was.
    #[must_use]
    pub const fn with_bit<const I: u32>(self, to: Bool) -> Self {
        let () = Within::<N, I>::ADMITTED;
        // The mutant `arvo_mutant_bits_with_bit` sets without clearing first.
        let kept = if cfg!(arvo_mutant_bits_with_bit) { self.0 } else { self.0 & !(1u64 << I) };
        Self(kept | ((to.get() as u64) << I))
    }

    /// The bitwise and of two values at this width.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// The bitwise or of two values at this width.
    #[must_use]
    pub const fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// The bitwise exclusive or of two values at this width.
    ///
    /// Both sides are within `N` bits, so the result is too and needs no mask.
    /// A consumer xoring two widths casts one of them first.
    #[must_use]
    pub const fn xor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Every bit of the width flipped.
    ///
    /// Masked, since the host's complement also sets the carrier's bits above
    /// the width.
    #[must_use]
    pub const fn not(self) -> Self {
        // The mutant `arvo_mutant_bits_not` leaves the bits above the width set.
        if cfg!(arvo_mutant_bits_not) { Self(!self.0) } else { Self::masked(!self.0) }
    }

    /// Whether no bit is set.
    ///
    /// `a.and(b).is_empty()` is the intersection test of two sets.
    #[must_use]
    pub const fn is_empty(self) -> Bool {
        Bool::of(self.0 == 0)
    }

    /// Every bit moved `K` places toward the high bit, what leaves the width
    /// dropped and zeros brought in.
    #[must_use]
    pub const fn shift_up<const K: u32>(self) -> Self {
        let () = Within::<N, K>::ADMITTED;
        // The mutant `arvo_mutant_bits_shift` leaves what leaves the width in the
        // carrier.
        if cfg!(arvo_mutant_bits_shift) {
            Self(self.0 << K)
        } else {
            Self::masked(self.0 << K)
        }
    }

    /// Every bit moved `K` places toward the low bit, what leaves the width
    /// dropped and zeros brought in.
    #[must_use]
    pub const fn shift_down<const K: u32>(self) -> Self {
        let () = Within::<N, K>::ADMITTED;
        Self(self.0 >> K)
    }

    /// Every bit moved `K` places toward the high bit, what leaves the top of
    /// the width brought in at the bottom.
    ///
    /// Within `N` bits rather than within the carrier, so a rotation of a
    /// `Bits<28>` wraps at bit twenty-seven.
    #[must_use]
    pub const fn rotate_up<const K: u32>(self) -> Self {
        let () = Within::<N, K>::ADMITTED;
        // The mutant `arvo_mutant_bits_rotate` rotates within the carrier at every
        // width, masked afterwards.
        if cfg!(arvo_mutant_bits_rotate) {
            Self::masked(self.0.rotate_left(K))
        } else if N == 64 {
            Self(self.0.rotate_left(K))
        } else {
            // `K < N < 64`, so neither shift reaches the carrier's width, and at
            // `K == 0` the second term is the value shifted by `N`, which is zero.
            Self::masked((self.0 << K) | (self.0 >> (N - K)))
        }
    }

    /// Every bit moved `K` places toward the low bit, what leaves the bottom
    /// of the width brought in at the top.
    #[must_use]
    pub const fn rotate_down<const K: u32>(self) -> Self {
        let () = Within::<N, K>::ADMITTED;
        if N == 64 {
            Self(self.0.rotate_right(K))
        } else {
            Self::masked((self.0 >> K) | (self.0 << (N - K)))
        }
    }
}
