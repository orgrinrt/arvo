//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A value as the count of a type-level list, on the const path.
//!
//! Under `hlist-const` a list's length and a position's index are constants in
//! a value. This file names no feature of the toolchain: reading a length as a
//! constant needs no gate where it is read. Run with `--features hlist-const`.

#![cfg(feature = "hlist-const")]

mod count;

use count::{
    Byte,
    Eighths,
    Host,
    L0,
    L3,
    L4,
    L5,
    P2,
    TwoBitsSaturate,
    TwoBitsWrap,
    the_counts_as_calls,
};
use notko_hlist::{Length, Position};

const NONE: Host = <L0 as Length<Host>>::LEN;
const FOUR: Host = <L4 as Length<Host>>::LEN;
const THREE: Byte = <L3 as Length<Byte>>::LEN;
const THIRD: Byte = <P2 as Position<Byte>>::INDEX;
const THREE_WHOLE: Eighths = <L3 as Length<Eighths>>::LEN;
const WRAPPED: TwoBitsWrap = <L5 as Length<TwoBitsWrap>>::LEN;
const HELD: TwoBitsSaturate = <L5 as Length<TwoBitsSaturate>>::LEN;

/// A length where the language wants a constant: an array's.
const CELLS: [u8; FOUR.host()] = [0; FOUR.host()];

#[test]
fn a_length_is_a_constant_in_a_value() {
    assert_eq!(NONE.host(), 0);
    assert_eq!(FOUR.host(), 4);
    assert_eq!(CELLS.len(), 4);
    assert_eq!(THREE.slot().index(), 3);
    assert_eq!(THIRD.slot().index(), 2);
    assert_eq!(THREE_WHOLE.slot().index(), 24);
    assert_eq!(WRAPPED.slot().index(), 1);
    assert_eq!(HELD.slot().index(), 3);
}

#[test]
fn a_length_is_the_same_as_a_call() {
    the_counts_as_calls();
}

fn pair(refused: &str, twin: &str) {
    let t = trybuild::TestCases::new();
    t.compile_fail(format!("tests/ui_count/{refused}.rs"));
    t.pass(format!("tests/ui_count/{twin}.rs"));
}

#[test]
fn a_length_past_the_range_under_abort_is_refused() {
    pair(
        "a_length_past_the_range_under_abort_is_refused",
        "a_length_at_the_end_of_the_range_under_abort_compiles",
    );
}

#[test]
fn a_length_in_a_format_without_a_one_is_refused() {
    pair(
        "a_length_in_a_format_without_a_one_is_refused",
        "a_length_in_a_format_with_a_one_compiles",
    );
}
