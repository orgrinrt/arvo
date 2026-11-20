//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! The capacity trait and its one implementor.

/// The capacity of `N` elements.
///
/// `N` is the one bare integer in the surface, at a const generic parameter.
/// A consumer declares its budget here once, as `type Units = Dim<256>`.
pub struct Dim<const N: usize>;

pub(crate) mod seal {
    use arvo_format::Adapt;
    use arvo_format::overflow::Wrap;
    use arvo_format::rounding::TowardZero;
    use arvo_numeral::{Quiet, USize};

    /// The type the length is held in. Nothing is computed on it here, so its
    /// adaptation is never applied; `count` hands it on under the consumer's.
    pub type Len = USize<Adapt<TowardZero, Wrap>, Quiet>;

    /// Admits `Dim<N>` alone, and holds the length the count is read from.
    ///
    /// The length is here rather than on `Capacity` so no host integer is
    /// public, and so the array and the count come from the one `N`.
    #[diagnostic::on_unimplemented(
        message = "`{Self}` is not a capacity: `Capacity` is implemented for `Dim<N>` alone",
        label = "not `Dim<N>`",
        note = "name the count as `Dim<N>`; the trait is sealed while the question `is_arvo_s_capacity_trait_sealed_to_its_one_marker` is open"
    )]
    pub trait Sealed {
        /// The number of elements, the `N` of `Dim<N>`.
        const LEN: Len;
    }
}

impl<const N: usize> seal::Sealed for Dim<N> {
    #[cfg(not(arvo_mutant_capacity_count))]
    const LEN: seal::Len = seal::Len::from_host(N);
    // The mutant: a count one past the array, which the agreement tests refuse.
    #[cfg(arvo_mutant_capacity_count)]
    const LEN: seal::Len = seal::Len::from_host(N.wrapping_add(1));
}

/// A count carried as typestate, with the array it sizes.
///
/// `Array<T>` is an array of exactly the count's length, read and written as
/// a slice. Nesting is a projection inside a projection, and no product of two
/// counts is computed; the outer array is built with `filled_with`.
///
/// The perimeter: `Dim<N>` is the only implementor, so for every capacity the
/// array's length and the count `count` reads are the same `N`.
// FIXME(is_arvo_s_capacity_trait_sealed_to_its_one_marker): sealed to `Dim<N>`
// while the question is open; unsealing breaks no consumer, sealing later would.
pub trait Capacity: seal::Sealed {
    /// An array of exactly the count's length of `T`.
    type Array<T>: AsRef<[T]> + AsMut<[T]>;

    /// The array with every element `value`.
    fn filled<T: Copy>(value: T) -> Self::Array<T>;

    /// The array whose elements are the values `f` returns, one call per
    /// element, in order from the first.
    ///
    /// No index is passed; a consumer that wants one counts in the closure.
    fn filled_with<T, F: FnMut() -> T>(f: F) -> Self::Array<T>;
}

impl<const N: usize> Capacity for Dim<N> {
    type Array<T> = [T; N];

    fn filled<T: Copy>(value: T) -> [T; N] {
        [value; N]
    }

    fn filled_with<T, F: FnMut() -> T>(mut f: F) -> [T; N] {
        core::array::from_fn(|_| f())
    }
}
