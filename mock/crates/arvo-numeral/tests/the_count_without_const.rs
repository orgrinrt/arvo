//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A value as the count of a type-level list, on the plain path.
//!
//! Under `hlist` alone the count trait is a plain trait, a length is a call,
//! and nothing in the build is unstable. The same lists and the same checks as
//! the const target, off the shared module. Run with `--features hlist`.

#![cfg(all(feature = "hlist", not(feature = "hlist-const")))]

mod count;

use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::Uint;
use count::{L3, L4, the_counts_as_calls};
use notko_hlist::Length;

/// Two bits under the default check, which aborts in a debug build where a
/// count leaves the range. Only the plain path reads it, where a length is a
/// call.
type TwoBitsAbort = Uint<2, Adapt<TowardZero, Wrap>>;

#[test]
fn a_length_is_a_call_in_a_value() {
    the_counts_as_calls();
}

#[test]
fn the_default_check_passes_a_length_that_fits() {
    assert_eq!(<L3 as Length<TwoBitsAbort>>::len().slot().index(), 3);
}

/// Four cells in two bits. As a call the count is computed when it runs, so
/// the check is the debug build's abort, where the const path refuses at build.
#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "the sum leaves the signature's range")]
fn a_length_past_the_range_aborts_under_the_default_check() {
    let _ = <L4 as Length<TwoBitsAbort>>::len();
}
