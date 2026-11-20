//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! The array a capacity projects and the count `count` reads are one number.
//!
//! Over a spread of counts, from the empty capacity through powers of two and
//! the numbers either side of them to one past a page of `u16`, the projected
//! array holds exactly as many elements as the count says, for an element of
//! one byte and of several. Under `--cfg arvo_mutant_capacity_count` the count
//! is one past the array, and every test here that reads the count fails.

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

/// An element that is not `Copy`, numbering itself in the order it was made.
struct Made(u32);

#[test]
fn filled_with_calls_its_closure_once_per_element_in_order() {
    let mut next = 0u32;
    let made = <Dim<6> as Capacity>::filled_with(|| {
        next += 1;
        Made(next)
    });
    assert_eq!(
        next, 6,
        "the closure was called {next} times for six elements"
    );
    let order: [u32; 6] = core::array::from_fn(|i| made.as_ref()[i].0);
    assert_eq!(order, [1, 2, 3, 4, 5, 6]);
    let mut calls = 0u32;
    let none = <Dim<0> as Capacity>::filled_with(|| {
        calls += 1;
        Made(calls)
    });
    assert!(none.as_ref().is_empty());
    assert_eq!(calls, 0, "the empty capacity called its closure");
}

#[test]
fn a_nested_array_of_a_non_copy_element_is_built_in_place() {
    let grid = <Dim<2> as Capacity>::filled_with(|| <Dim<3> as Capacity>::filled_with(|| Made(4)));
    assert_eq!(grid.as_ref().len(), 2);
    assert!(grid.as_ref().iter().all(|row| row.as_ref().len() == 3));
    assert!(
        grid.as_ref()
            .iter()
            .flat_map(|row| row.as_ref())
            .all(|m| m.0 == 4)
    );
}

/// The count at zero, one and the host's largest `usize`, under one rounding
/// mode, one overflow policy and one check, read in a constant.
///
/// `Clamp` and `Stochastic` are left out because `USize` refuses them at build:
/// a value has no bound to clamp to and nowhere to carry a dither.
macro_rules! edges {
    ($r:ty, $o:ty, $k:ty) => {{
        type Ad = arvo_format::Adapt<$r, $o>;
        const ZERO: usize = count::<Dim<0>, Ad, $k>().host();
        const ONE: usize = count::<Dim<1>, Ad, $k>().host();
        const TOP: usize = count::<Dim<{ usize::MAX }>, Ad, $k>().host();
        assert_eq!((ZERO, ONE, TOP), (0, 1, usize::MAX));
    }};
}

macro_rules! every_policy {
    ($r:ty, $k:ty) => {
        edges!($r, arvo_format::overflow::Wrap, $k);
        edges!($r, arvo_format::overflow::Saturate, $k);
    };
}

macro_rules! every_mode {
    ($k:ty) => {
        every_policy!(arvo_format::rounding::TowardZero, $k);
        every_policy!(arvo_format::rounding::Floor, $k);
        every_policy!(arvo_format::rounding::Ceil, $k);
        every_policy!(arvo_format::rounding::HalfUp, $k);
        every_policy!(arvo_format::rounding::HalfEven, $k);
    };
}

#[test]
fn the_count_holds_at_its_edges_under_every_adaptation_and_check() {
    every_mode!(Quiet);
    every_mode!(arvo_numeral::Abort);
}
