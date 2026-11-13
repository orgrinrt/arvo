//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The debug assertion under the plain overflow markers, observed at each site
//! that carries it: `add` and `adapted`, under `Wrap` and under
//! `Saturate`. The panic is the specified behaviour in a debug build, so each is
//! a `should_panic` test, and each site has a twin under the completing markers
//! that takes the same input, completes it and does not assert.

#![cfg(debug_assertions)]

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Exact, Slot};
use arvo_numeral::{Int, Saturating, Wrapping};

type UnderWrap = Int<4, Adapt<TowardZero, Wrap>>;
type UnderSaturate = Int<4, Adapt<TowardZero, Saturate>>;
type UnderWrapping = Int<4, Adapt<TowardZero, Wrapping>>;
type UnderSaturating = Int<4, Adapt<TowardZero, Saturating>>;

/// One past the top of the signed 4-bit range.
const PAST_THE_TOP: i128 = 8;

#[test]
#[should_panic(expected = "the sum leaves the signature's range")]
fn add_asserts_under_wrap() {
    let _ = UnderWrap::constant::<7>().add(UnderWrap::constant::<1>());
}

#[test]
#[should_panic(expected = "the sum leaves the signature's range")]
fn add_asserts_under_saturate() {
    let _ = UnderSaturate::constant::<7>().add(UnderSaturate::constant::<1>());
}

#[test]
fn add_completes_under_the_completing_markers() {
    let wrapped = UnderWrapping::constant::<7>().add(UnderWrapping::constant::<1>());
    assert_eq!(wrapped.slot().index(), -8);
    let saturated = UnderSaturating::constant::<7>().add(UnderSaturating::constant::<1>());
    assert_eq!(saturated.slot().index(), 7);
}

#[test]
#[should_panic(expected = "the position leaves the signature's range")]
fn adapted_asserts_under_wrap() {
    let _ = UnderWrap::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
}

#[test]
#[should_panic(expected = "the position leaves the signature's range")]
fn adapted_asserts_under_saturate() {
    let _ = UnderSaturate::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
}

#[test]
fn adapted_completes_under_the_completing_markers() {
    let wrapped = UnderWrapping::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
    assert_eq!(wrapped.slot().index(), -8);
    let saturated = UnderSaturating::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
    assert_eq!(saturated.slot().index(), 7);
}

#[test]
fn the_plain_markers_do_not_assert_on_what_fits() {
    assert_eq!(
        UnderWrap::constant::<7>()
            .add(UnderWrap::constant::<-1>())
            .slot()
            .index(),
        6
    );
    assert_eq!(
        UnderSaturate::constant::<-8>()
            .add(UnderSaturate::constant::<1>())
            .slot()
            .index(),
        -7
    );
    assert_eq!(
        UnderWrap::adapted(Exact::on_grid(Slot::at(7)))
            .slot()
            .index(),
        7
    );
    assert_eq!(
        UnderSaturate::adapted(Exact::on_grid(Slot::at(-8)))
            .slot()
            .index(),
        -8
    );
}
