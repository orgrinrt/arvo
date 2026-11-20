//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The exact position of a product, a quotient or a conversion, formed in the
//! host's 128-bit integer where the range leaves room for it.
//!
//! At a zero phase and a constant quantum each of the three sits at a ratio of
//! slot products and powers of the radix. With `u = r^max(e, 0)` and
//! `v = r^max(-e, 0)`: the product of the members at `a` and `b` at `a b u / v`,
//! the quotient of `a` by the divisor at `b` at `a v / (b u)`, and the member of
//! `F` at `a` on `T`'s grid at `a u_F v_T / (v_F u_T)`. Where the largest
//! numerator a member reaches and the denominator fit the host's unsigned
//! 128-bit integer and their quotient is below `2^127`, the ratio is formed here,
//! its floor and its residue, in the machine's 64-bit integer where the terms fit
//! there, and the format's `adapt` rounds and completes it.
//! Elsewhere the format's own position, formed four 64-bit limbs wide, is taken.
//!
//! Which of the two an instantiation takes is a constant of it, read from the
//! range, the radix and the exponent. The bound is over the range because a value
//! holds only members, and the divisors a value takes, a constant and a member of
//! a range on its grid, are in the range too. The format's position is total over
//! every slot, so the same bound there would need a test of the operands.

use core::marker::PhantomData;

use arvo_format::quantum::is_constant_family;
use arvo_format::{
    Divisor,
    Exact,
    Format,
    Quantum,
    Remainder,
    Slot,
    Slots,
    converted_position,
    product_position,
    quotient_position,
    radix,
};
use notko::Maybe;

/// A ratio of two magnitudes in slot units, and the sign of the ratio. The
/// denominator is above zero.
#[derive(Clone, Copy)]
struct Ratio {
    negative: bool,
    num:      u128,
    den:      u128,
}

impl Ratio {
    /// The exact position at the ratio: the floor toward negative infinity, and
    /// the residue to the slot above it.
    ///
    /// Reached only where the bound holds, so the quotient is below `2^127` and
    /// the floor and the slot under it are in the index. `narrow` says the
    /// bound keeps both terms in 64 bits, a constant of the instantiation, so
    /// the division is the machine's 64-bit one there. The residue is not
    /// reduced: a value only rounds and completes the position, and neither
    /// reads its terms apart from their ratio.
    #[inline(always)]
    const fn exact(self, narrow: bool) -> Exact {
        let (whole, rest) = if narrow {
            let (num, den) = (self.num as u64, self.den as u64);
            ((num / den) as i128, (num % den) as u128)
        } else {
            ((self.num / self.den) as i128, self.num % self.den)
        };
        if rest == 0 {
            return Exact::on_grid(Slot::at(if self.negative { -whole } else { whole }));
        }
        // The floor of a negative ratio is one below the negated quotient
        // wherever something is left over, and the residue is then counted from
        // the slot above.
        let (floor, residue) = if !self.negative {
            (whole, rest)
        } else if cfg!(arvo_mutant_room) {
            (-whole, self.den - rest)
        } else {
            (-whole - 1, self.den - rest)
        };
        Exact::at(Slot::at(floor), Remainder::of(residue, self.den))
    }
}

/// The radix to the power `k`, or nothing where it passes 128 bits.
const fn power(radix: u128, k: u32) -> Maybe<u128> {
    let mut at = 1u128;
    let mut i = 0;
    while i < k {
        match at.checked_mul(radix) {
            Some(next) => at = next,
            None => return Maybe::Isnt,
        }
        i += 1;
    }
    Maybe::Is(at)
}

/// The product of two magnitudes, or nothing where either is lost or the
/// product passes 128 bits.
const fn times(a: Maybe<u128>, b: Maybe<u128>) -> Maybe<u128> {
    match (a, b) {
        (Maybe::Is(a), Maybe::Is(b)) => {
            match a.checked_mul(b) {
                Some(at) => Maybe::Is(at),
                None => Maybe::Isnt,
            }
        },
        _ => Maybe::Isnt,
    }
}

/// Whether a ratio whose numerator reaches `num` over a denominator of at least
/// `den` is formed here: both are held and their quotient is below `2^127`.
const fn holds(num: Maybe<u128>, den: Maybe<u128>) -> bool {
    match (num, den) {
        // The mutant admits a quotient up to `2^128`, one radix power past the
        // edge at radix two, where the floor no longer fits the index.
        (Maybe::Is(_), Maybe::Is(den)) if cfg!(arvo_mutant_room_edge) => den > 0,
        (Maybe::Is(num), Maybe::Is(den)) => den > 0 && num / den < (1u128 << 127),
        _ => false,
    }
}

/// Whether a numerator reaching `num` and a denominator reaching `den` both fit
/// the machine's 64-bit integer.
const fn narrow(num: Maybe<u128>, den: Maybe<u128>) -> bool {
    match (num, den) {
        (Maybe::Is(num), Maybe::Is(den)) => num <= u64::MAX as u128 && den <= u64::MAX as u128,
        _ => false,
    }
}

/// The constants of `F` the ratio reads.
struct Sides<F>(PhantomData<F>);

impl<F: Format> Sides<F> {
    /// `v` where the product or the quotient is formed here, and one otherwise.
    const DOWN: u128 = match Self::POWERS.1 {
        Maybe::Is(v) => v,
        Maybe::Isnt => 1,
    };
    /// Whether the phase is zero and the quantum constant, which every value's
    /// format is and the ratio presumes.
    const PLAIN: bool = F::PHASE.is_zero().get() && is_constant_family::<F::Quantum>().get();
    /// `u` and `v`, the radix to the two halves of the quantum's exponent.
    const POWERS: (Maybe<u128>, Maybe<u128>) = {
        let r = radix::<F>().base() as u128;
        let e = <F::Quantum as Quantum>::BASE.power();
        if e >= 0 {
            (power(r, e.unsigned_abs()), Maybe::Is(1))
        } else {
            (Maybe::Is(1), power(r, e.unsigned_abs()))
        }
    };
    /// Whether the product of two members is formed here.
    const PRODUCT: bool = Self::PLAIN
        && holds(
            times(times(Self::REACH, Self::REACH), Self::POWERS.0),
            Self::POWERS.1,
        );
    /// Whether the product's terms fit 64 bits.
    const PRODUCT_NARROW: bool = narrow(
        times(times(Self::REACH, Self::REACH), Self::POWERS.0),
        Self::POWERS.1,
    );
    /// Whether the quotient of a member by a divisor in the range is formed here.
    const QUOTIENT: bool = Self::PLAIN
        && matches!(times(Self::REACH, Self::POWERS.0), Maybe::Is(_))
        && holds(times(Self::REACH, Self::POWERS.1), Self::POWERS.0);
    /// Whether the quotient's terms fit 64 bits.
    const QUOTIENT_NARROW: bool = narrow(
        times(Self::REACH, Self::POWERS.1),
        times(Self::REACH, Self::POWERS.0),
    );
    /// The largest magnitude a slot of the range takes.
    const REACH: Maybe<u128> = {
        let lo = <F::Slots as Slots>::MIN.index().unsigned_abs();
        let hi = <F::Slots as Slots>::MAX.index().unsigned_abs();
        Maybe::Is(if lo > hi { lo } else { hi })
    };
    /// `u` where the product or the quotient is formed here, and one otherwise.
    const UP: u128 = match Self::POWERS.0 {
        Maybe::Is(u) => u,
        Maybe::Isnt => 1,
    };
}

/// The constants of the crossing from `F` onto `T` the ratio reads.
struct Crossing<F, T>(PhantomData<(F, T)>);

impl<F: Format, T: Format> Crossing<F, T> {
    /// `v_F u_T`, the denominator.
    const DIVISOR: Maybe<u128> = times(Sides::<F>::POWERS.1, Sides::<T>::POWERS.0);
    /// Whether the conversion of a member is formed here.
    const HOLDS: bool = Sides::<F>::PLAIN
        && Sides::<T>::PLAIN
        && holds(times(Sides::<F>::REACH, Self::SCALE), Self::DIVISOR);
    /// Whether the conversion's terms fit 64 bits.
    const NARROW: bool = narrow(times(Sides::<F>::REACH, Self::SCALE), Self::DIVISOR);
    /// `v_F u_T` where the conversion is formed here, and one otherwise.
    const OVER: u128 = match Self::DIVISOR {
        Maybe::Is(d) => d,
        Maybe::Isnt => 1,
    };
    /// `u_F v_T`, what a slot of `F` is scaled by.
    const SCALE: Maybe<u128> = times(Sides::<F>::POWERS.0, Sides::<T>::POWERS.1);
    /// `u_F v_T` where the conversion is formed here, and one otherwise.
    const SCALED: u128 = match Self::SCALE {
        Maybe::Is(k) => k,
        Maybe::Isnt => 1,
    };
}

/// The exact position of the product of the members at `a` and `b` of `F`.
#[inline]
pub(crate) const fn product<F: Format>(a: Slot, b: Slot) -> Exact {
    if const { Sides::<F>::PRODUCT } {
        let (a, b) = (a.index(), b.index());
        Ratio {
            negative: (a < 0) != (b < 0),
            num:      a.unsigned_abs() * b.unsigned_abs() * Sides::<F>::UP,
            den:      Sides::<F>::DOWN,
        }
        .exact(const { Sides::<F>::PRODUCT_NARROW })
    } else {
        product_position::<F>(a, b)
    }
}

/// The exact position of the quotient of the member at `a` of `F` by `by`.
#[inline]
pub(crate) const fn quotient<F: Format>(a: Slot, by: Divisor<F>) -> Exact {
    if const { Sides::<F>::QUOTIENT } {
        let (a, b) = (a.index(), by.slot().index());
        Ratio {
            negative: (a < 0) != (b < 0),
            num:      a.unsigned_abs() * Sides::<F>::DOWN,
            den:      b.unsigned_abs() * Sides::<F>::UP,
        }
        .exact(const { Sides::<F>::QUOTIENT_NARROW })
    } else {
        quotient_position::<F>(a, by)
    }
}

/// The exact position of the member at `a` of `F` on the grid of `T`.
#[inline]
pub(crate) const fn converted<F: Format, T: Format>(a: Slot) -> Exact {
    if const { Crossing::<F, T>::HOLDS } {
        let a = a.index();
        Ratio {
            negative: a < 0,
            num:      a.unsigned_abs() * Crossing::<F, T>::SCALED,
            den:      Crossing::<F, T>::OVER,
        }
        .exact(const { Crossing::<F, T>::NARROW })
    } else {
        converted_position::<F, T>(a)
    }
}

/// The edges in the tests' own arithmetic, the file the shape suites read.
#[cfg(test)]
#[path = "../tests/common/edges.rs"]
#[allow(dead_code)]
mod edges;

/// The radix-ten and radix-three ambients and the format over them that the
/// suites declare, read where they are kept.
#[cfg(test)]
#[path = "../tests/common/points.rs"]
mod points;

#[cfg(test)]
mod bound {
    //! The bound shown true one exponent inside its edge and false one past it.

    use arvo_format::ambient::{BinaryRationals, DecimalRationals, UnsignedBinaryRationals};
    use arvo_format::points::{Fixed, UFixed};
    use arvo_format::slots::{Signed, Unsigned};

    use super::points::{
        Scaled,
        TernaryRationals,
        UnsignedDecimalRationals,
        UnsignedTernaryRationals,
    };
    use super::{Crossing, Sides, edges};

    /// The product, the quotient and the conversion from exponent zero, each
    /// formed here at the last exponent the tests' arithmetic puts inside the
    /// host's integer and not one past it, over one ambient and range.
    macro_rules! at_the_edge {
        ($ambient:ty, $slots:ident, $w:literal, $signed:literal, $radix:literal) => {{
            const P: i32 = edges::product_room($w, $signed, $radix);
            const Q: i32 = edges::quotient_room($w, $signed, $radix);
            const C: i32 = -edges::host_bound(edges::widest($w, $signed) as u128, $radix);
            type At<const E: i32> = Scaled<$ambient, $slots<$w>, E>;
            let at = ($w, $signed, $radix);
            assert!(
                Sides::<At<P>>::PRODUCT,
                "product refused at its edge {}, {:?}",
                P,
                at
            );
            assert!(
                !Sides::<At<{ P - 1 }>>::PRODUCT,
                "product past its edge {}, {:?}",
                P,
                at
            );
            assert!(
                Sides::<At<Q>>::QUOTIENT,
                "quotient refused at its edge {}, {:?}",
                Q,
                at
            );
            assert!(
                !Sides::<At<{ Q - 1 }>>::QUOTIENT,
                "quotient past its edge {}, {:?}",
                Q,
                at
            );
            assert!(
                Crossing::<At<0>, At<C>>::HOLDS,
                "conversion refused at its edge {}, {:?}",
                C,
                at
            );
            assert!(
                !Crossing::<At<0>, At<{ C - 1 }>>::HOLDS,
                "conversion past its edge {}, {:?}",
                C,
                at
            );
        }};
    }

    /// Both ranges of one width at the three radixes.
    macro_rules! at_width {
        ($($w:literal),*) => {$(
            at_the_edge!(BinaryRationals, Signed, $w, true, 2);
            at_the_edge!(UnsignedBinaryRationals, Unsigned, $w, false, 2);
            at_the_edge!(DecimalRationals, Signed, $w, true, 10);
            at_the_edge!(UnsignedDecimalRationals, Unsigned, $w, false, 10);
            at_the_edge!(TernaryRationals, Signed, $w, true, 3);
            at_the_edge!(UnsignedTernaryRationals, Unsigned, $w, false, 3);
        )*};
    }

    #[test]
    fn the_three_are_formed_here_to_the_rooms_edge_at_every_radix() {
        at_width!(8, 16, 32, 64);
    }

    #[test]
    fn the_product_is_formed_here_down_to_where_the_radix_power_passes_128_bits() {
        // `2^127` is the last power of two a 128-bit integer holds, and the
        // square of any 64-bit member over it stays below `2^127`.
        assert!(Sides::<Fixed<64, -1>>::PRODUCT);
        assert!(Sides::<UFixed<64, -1>>::PRODUCT);
        assert!(Sides::<Fixed<64, -127>>::PRODUCT);
        assert!(Sides::<UFixed<64, -127>>::PRODUCT);
        assert!(!Sides::<Fixed<64, -128>>::PRODUCT);
        assert!(!Sides::<UFixed<64, -128>>::PRODUCT);
        assert!(Sides::<Fixed<8, -127>>::PRODUCT);
        assert!(!Sides::<Fixed<8, -128>>::PRODUCT);
    }

    #[test]
    fn the_product_of_the_64_bit_widths_is_the_formats_at_a_positive_exponent() {
        assert!(!Sides::<UFixed<64, 1>>::PRODUCT);
        assert!(!Sides::<Fixed<64, 2>>::PRODUCT);
        assert!(Sides::<Fixed<8, 2>>::PRODUCT);
    }

    #[test]
    fn the_quotient_is_formed_here_down_to_its_edge_and_not_past_it() {
        // `|a| v / u` is `(2^64 - 1) 2^k` unsigned and `2^63 2^k` signed, below
        // `2^127` up to `k = 63` for both.
        assert!(Sides::<UFixed<64, -63>>::QUOTIENT);
        assert!(!Sides::<UFixed<64, -64>>::QUOTIENT);
        assert!(Sides::<Fixed<64, -63>>::QUOTIENT);
        assert!(!Sides::<Fixed<64, -64>>::QUOTIENT);
    }

    #[test]
    fn the_terms_take_the_64_bit_division_where_they_fit_it_and_not_past() {
        assert!(Sides::<UFixed<16, -8>>::QUOTIENT_NARROW);
        assert!(Sides::<UFixed<16, -8>>::PRODUCT_NARROW);
        assert!(Sides::<UFixed<32, 0>>::PRODUCT_NARROW);
        assert!(!Sides::<UFixed<33, 0>>::PRODUCT_NARROW);
        assert!(!Sides::<UFixed<64, -1>>::QUOTIENT_NARROW);
        assert!(Crossing::<UFixed<16, -8>, UFixed<16, -4>>::NARROW);
        assert!(!Crossing::<UFixed<64, 0>, UFixed<64, -1>>::NARROW);
    }

    #[test]
    fn the_conversion_is_formed_here_where_the_scaled_range_stays_under_2_127() {
        assert!(Crossing::<UFixed<64, 0>, UFixed<64, -63>>::HOLDS);
        assert!(!Crossing::<UFixed<64, 0>, UFixed<64, -64>>::HOLDS);
        assert!(Crossing::<UFixed<8, -100>, UFixed<8, 0>>::HOLDS);
    }
}
