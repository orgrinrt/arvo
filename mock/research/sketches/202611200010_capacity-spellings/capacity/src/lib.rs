//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! Arm A: a count as a type, its array projection, and its count as a constant.
//!
//! Two variants of the one spelling. `sealed` admits `Dim<N>` alone, so the
//! array a capacity projects and the count it reports come from the one `N`
//! and cannot disagree. `open` is the same trait any crate may implement,
//! kept here to measure what the retirement
//! `dl_capacity_checked_at_the_one_construction_door` names: with the trait
//! open, an implementor reports a count its array does not have, and nothing
//! at a construction door sees it, because nothing is constructed through one.
//!
//! The provider names no feature of the toolchain either. Under
//! `--cfg capacity_mutant` the sealed `Dim<N>` reports one more than `N`,
//! which the consumer's agreement check must refuse at build.

#![no_std]

use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;

/// The count a capacity reports, the stack's own platform-sized integer.
pub type Count = arvo_numeral::USize<Adapt<TowardZero, Wrap>>;

/// A capacity fixed by a literal inside its own type.
///
/// `N` is the one bare integer in the spelling, at a const generic position,
/// the "declare the budget once" site hilavitkutin's ratified row names.
pub struct Dim<const N: usize>;

/// The sealed variant: `Dim<N>` is the only capacity.
pub mod sealed {
    use super::{Count, Dim};

    mod seal {
        pub trait Sealed {}
    }

    impl<const N: usize> seal::Sealed for Dim<N> {}

    /// A count as typestate, with the array it sizes and the count as a constant.
    pub trait Capacity: seal::Sealed {
        /// An array of exactly the count's length.
        type Array<T>: AsRef<[T]> + AsMut<[T]>;
        /// The count, readable in a constant.
        const COUNT: Count;
        /// The array with every element `value`.
        fn filled<T: Copy>(value: T) -> Self::Array<T>;
    }

    impl<const N: usize> Capacity for Dim<N> {
        type Array<T> = [T; N];

        #[cfg(not(capacity_mutant))]
        const COUNT: Count = Count::from_host(N);
        #[cfg(capacity_mutant)]
        const COUNT: Count = Count::from_host(N + 1);

        fn filled<T: Copy>(value: T) -> [T; N] {
            [value; N]
        }
    }
}

/// The open variant: any crate may implement it.
pub mod open {
    use super::{Count, Dim};

    /// The same contract as `sealed::Capacity`, with no seal.
    pub trait Capacity {
        /// An array the implementor claims is the count's length.
        type Array<T>: AsRef<[T]> + AsMut<[T]>;
        /// The count the implementor claims.
        const COUNT: Count;
        /// The array with every element `value`.
        fn filled<T: Copy>(value: T) -> Self::Array<T>;
    }

    impl<const N: usize> Capacity for Dim<N> {
        type Array<T> = [T; N];

        const COUNT: Count = Count::from_host(N);

        fn filled<T: Copy>(value: T) -> [T; N] {
            [value; N]
        }
    }
}
