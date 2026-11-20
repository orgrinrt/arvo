//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! The array a capacity projects and the count `count` reads are one number.
//!
//! Over a spread of counts, from the empty capacity through powers of two and
//! the numbers either side of them to one past a page of `u16`, the projected
//! array holds exactly as many elements as the count says, for an element of
//! one byte and of several. Under `--cfg arvo_mutant_capacity_count` the count
//! is one past the array, and every case below fails.

use arvo_capacity::{Capacity, Dim, count};
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::Quiet;

type A = Adapt<TowardZero, Wrap>;

/// The projected array's length beside the count, for one capacity.
fn lengths<C: Capacity>() -> (usize, usize) {
    let bytes = C::filled(0u8);
    let words = C::filled([0u64; 3]);
    assert_eq!(bytes.as_ref().len(), words.as_ref().len());
    assert_eq!(
        core::mem::size_of::<C::Array<u8>>(),
        bytes.as_ref().len(),
        "an array of bytes carries no padding, so its size is its length"
    );
    (bytes.as_ref().len(), count::<C, A, Quiet>().host())
}

macro_rules! agree {
    ($($n:literal),* $(,)?) => {
        $(
            let (array, counted) = lengths::<Dim<$n>>();
            assert_eq!(array, $n, "the array of Dim<{}> is not {} long", $n, $n);
            assert_eq!(counted, $n, "the count of Dim<{}> is not {}", $n, $n);
        )*
    };
}

#[test]
fn the_array_and_the_count_agree_over_a_spread_of_counts() {
    agree!(
        0, 1, 2, 3, 7, 8, 9, 15, 16, 17, 255, 256, 257, 1023, 1024, 4096, 65535, 65536
    );
}

/// The count read in a constant, where the build evaluates it.
const UNITS: usize = count::<Dim<256>, A, Quiet>().host();
const NONE: usize = count::<Dim<0>, A, Quiet>().host();

#[test]
fn the_count_reads_in_a_constant() {
    assert_eq!(UNITS, 256);
    assert_eq!(NONE, 0);
}

#[test]
fn a_filled_array_holds_its_value_in_every_element() {
    let a = <Dim<5> as Capacity>::filled(9u32);
    assert_eq!(a.as_ref(), &[9, 9, 9, 9, 9]);
    let empty = <Dim<0> as Capacity>::filled(9u32);
    assert!(empty.as_ref().is_empty());
}

#[test]
fn a_nested_projection_is_outer_by_inner_with_no_product_computed() {
    let mut grid = <Dim<3> as Capacity>::filled(<Dim<4> as Capacity>::filled(0u16));
    grid.as_mut()[2].as_mut()[3] = 7;
    assert_eq!(grid.as_ref().len(), 3);
    assert_eq!(grid.as_ref()[0].as_ref().len(), 4);
    assert_eq!(grid.as_ref()[2].as_ref()[3], 7);
    assert_eq!(grid.as_ref()[1].as_ref()[3], 0);
    assert_eq!(core::mem::size_of_val(&grid), 3 * 4 * 2);
}
