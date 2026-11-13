//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The exact sum of a bounded number of members.
//!
//! The sum is held in a value of a second signature whose range the build has
//! checked against the capacity, so no step of it completes and the order of
//! the terms cannot matter. It comes back to the terms' signature once.

use core::marker::PhantomData;

use arvo_format::{Bool, Format, Quantum, Slots, radix};
use notko::Maybe;

use crate::admit::Sole;
use crate::check::{Abort, DebugCheck, Quiet};
use crate::num::{Num, SlotsOf};

/// The exact sum of at most `CAP` members of `S`, held in a value of `T`.
///
/// `T` is named by the consumer and checked by the build: its format sits on
/// the grid of `S`, the same radix and quantum exponent, and its range holds
/// `CAP` times each end of the range of `S`. Where it does not, anything that
/// builds an accumulator is refused as a failed constant.
///
/// The capacity is kept by the shapes of the functions that put terms in, and
/// nothing is counted at run time. What is guaranteed is over those functions:
/// `empty`, `of`, `take` and `plus` are the only ways to one, and the sum is
/// read through `total`, `finish` and `fits`.
pub struct Accumulator<S: Sole, T: Sole, const CAP: usize, C: DebugCheck = Abort>(
    Num<T, Quiet>,
    PhantomData<(S, C)>,
);

/// The interval the sums of `count` members of `S` lie in, as slot indices.
/// Nothing where it leaves the build's own integers.
const fn sums<S: Sole>(count: usize) -> Maybe<(i128, i128)> {
    let count = count as i128;
    match (
        count.checked_mul(<SlotsOf<S> as Slots>::MIN.index()),
        count.checked_mul(<SlotsOf<S> as Slots>::MAX.index()),
    ) {
        (Some(low), Some(high)) => Maybe::Is((low, high)),
        _ => Maybe::Isnt,
    }
}

/// Whether the interval is inside the slot range of `T`.
const fn holds<T: Sole>(interval: Maybe<(i128, i128)>) -> bool {
    match interval {
        Maybe::Is((low, high)) => {
            low >= <SlotsOf<T> as Slots>::MIN.index() && high <= <SlotsOf<T> as Slots>::MAX.index()
        },
        Maybe::Isnt => false,
    }
}

/// The interval the differences of a sum in `a` and a sum in `b` lie in.
const fn differences(a: Maybe<(i128, i128)>, b: Maybe<(i128, i128)>) -> Maybe<(i128, i128)> {
    match (a, b) {
        (Maybe::Is((a_low, a_high)), Maybe::Is((b_low, b_high))) => {
            match (a_low.checked_sub(b_high), a_high.checked_sub(b_low)) {
                (Some(low), Some(high)) => Maybe::Is((low, high)),
                _ => Maybe::Isnt,
            }
        },
        _ => Maybe::Isnt,
    }
}

impl<S: Sole, T: Sole, const CAP: usize, C: DebugCheck> Accumulator<S, T, CAP, C> {
    /// What the build asks of `T` on behalf of `S` and `CAP`.
    const ADMITTED: () = {
        let () = S::ADMITTED;
        let () = T::ADMITTED;
        assert!(
            radix::<S::Format>().base() == radix::<T::Format>().base()
                && <<S::Format as Format>::Quantum as Quantum>::BASE.power()
                    == <<T::Format as Format>::Quantum as Quantum>::BASE.power(),
            "the accumulator's signature is on another grid than its terms'. A term enters by its \
             slot index, so the two formats share their radix and their quantum exponent. Name an \
             accumulator format with the terms' fraction bits and more width"
        );
        assert!(
            holds::<T>(sums::<S>(CAP)),
            "the accumulator's range does not hold the capacity times each end of its terms' range, \
             so a sum of that many terms could leave it. Name a wider accumulator format, or a \
             smaller capacity and sum in blocks with `take`"
        );
    };

    const fn holding(sum: Num<T, Quiet>) -> Self {
        let () = Self::ADMITTED;
        Self(sum, PhantomData)
    }

    /// The sum of the terms, which the caller has bounded by the capacity.
    const fn sum(terms: &[Num<S, C>]) -> Self {
        let mut sum = Num::<T, Quiet>::at_index(0);
        let mut i = 0;
        while i < terms.len() {
            sum = sum.add(Num::at_index(terms[i].slot().index()));
            i += 1;
        }
        Self::holding(sum)
    }

    /// The accumulator of no terms, whose sum is zero.
    #[must_use]
    pub const fn empty() -> Self {
        Self::holding(Num::at_index(0))
    }

    /// The exact sum of an array of `N` terms.
    ///
    /// Refused at build where `N` is above the capacity.
    #[must_use]
    pub const fn of<const N: usize>(terms: &[Num<S, C>; N]) -> Self {
        const {
            assert!(
                N <= CAP,
                "`of` is given more terms than the accumulator's capacity. Name a capacity of at \
                 least the array's length, or sum a slice in blocks with `take`"
            );
        }
        Self::sum(terms)
    }

    /// The exact sum of the first terms of a slice, as many as the capacity
    /// or all of them where the slice is shorter, with the rest handed back.
    ///
    /// A slice longer than the capacity is summed a block at a time, each block
    /// exact, and what joins the blocks is the caller's to say: the signature's
    /// `add` on the finished values, or `plus` into a wider accumulator.
    ///
    /// Refused at build at a capacity of zero, where it would sum nothing and
    /// hand back the slice it was given.
    #[must_use]
    pub const fn take(terms: &[Num<S, C>]) -> (Self, &[Num<S, C>]) {
        const {
            assert!(
                CAP >= 1,
                "`take` at a capacity of zero sums nothing and hands back the slice it was given, so \
                 a loop over it would not end. Name a capacity of at least one"
            );
        }
        let count = if cfg!(feature = "mutant-accumulator") || terms.len() < CAP {
            terms.len()
        } else {
            CAP
        };
        let (block, rest) = terms.split_at(count);
        (Self::sum(block), rest)
    }

    /// The two sums joined: the exact sum of every term of both.
    ///
    /// Refused at build where the two capacities together are above this one.
    #[must_use]
    pub const fn plus<const A: usize, const B: usize>(
        a: Accumulator<S, T, A, C>,
        b: Accumulator<S, T, B, C>,
    ) -> Self {
        const {
            assert!(
                matches!(A.checked_add(B), Some(both) if both <= CAP),
                "`plus` joins two accumulators whose capacities together are above the capacity of \
                 the one it returns. Name a capacity of at least their sum"
            );
        }
        Self::holding(a.0.add(b.0))
    }

    /// The exact difference of two sums, as a value of `T`.
    ///
    /// A difference of two sums is no sum of members, so it is a value and not
    /// an accumulator. Refused at build where the range of `T` does not hold
    /// every difference of a sum of `A` terms and a sum of `B`, which over
    /// unsigned terms reaches below zero.
    #[must_use]
    pub const fn minus<const A: usize, const B: usize>(
        a: Accumulator<S, T, A, C>,
        b: Accumulator<S, T, B, C>,
    ) -> Num<T, C> {
        const {
            assert!(
                holds::<T>(differences(sums::<S>(A), sums::<S>(B))),
                "`minus` could leave the accumulator's range: the range does not hold every \
                 difference of sums at these two capacities. Name an accumulator format that \
                 reaches as far below zero as the subtracted sum goes, a signed one over unsigned \
                 terms"
            );
        }
        Num::from_carrier(a.0.sub(b.0).carrier())
    }

    /// The exact sum, as a value of `T`.
    #[must_use]
    pub const fn total(self) -> Num<T, C> {
        Num::from_carrier(self.0.carrier())
    }

    /// The sum as a value of `S`, completed once by the overflow policy of `S`
    /// where it has left the range of `S`.
    ///
    /// The same for every order the terms were taken in. Under the `Abort`
    /// check a debug build aborts where the sum has left the range, which
    /// `fits` asks ahead; under `Quiet` no build does.
    #[must_use]
    pub const fn finish(self) -> Num<S, C> {
        self.total().convert::<S>()
    }

    /// Whether the sum is a member of `S`, so `finish` needs no completion.
    #[must_use]
    pub const fn fits(self) -> Bool {
        self.total().fits_convert::<S>()
    }
}

impl<S: Sole, T: Sole, const CAP: usize, C: DebugCheck> Clone for Accumulator<S, T, CAP, C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: Sole, T: Sole, const CAP: usize, C: DebugCheck> Copy for Accumulator<S, T, CAP, C> {}
