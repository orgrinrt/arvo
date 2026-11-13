//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The debug check, observed at each site that carries it: `add` and `adapted`,
//! under `Wrap` and under `Saturate`. Under `Abort` a debug build aborts when a
//! result leaves the range, so each of those is a `should_panic` test; each has
//! a twin under `Quiet` that takes the same input and completes it by the
//! policy. The check is no part of the signature, so the twins hold the same
//! member the same way.

#![cfg(debug_assertions)]

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Exact, Slot};
use arvo_numeral::{Abort, Int, Quiet};

type WrapAborts = Int<4, Adapt<TowardZero, Wrap>>;
type SaturateAborts = Int<4, Adapt<TowardZero, Saturate>, Abort>;
type WrapQuiet = Int<4, Adapt<TowardZero, Wrap>, Quiet>;
type SaturateQuiet = Int<4, Adapt<TowardZero, Saturate>, Quiet>;

/// One past the top of a 4-bit signed range.
const PAST_THE_TOP: i128 = 8;

#[test]
#[should_panic(expected = "the sum leaves the signature's range")]
fn add_aborts_under_wrap_with_the_default_check() {
    let _ = WrapAborts::constant::<7>().add(WrapAborts::constant::<1>());
}

#[test]
#[should_panic(expected = "the sum leaves the signature's range")]
fn add_aborts_under_saturate_with_abort_named() {
    let _ = SaturateAborts::constant::<7>().add(SaturateAborts::constant::<1>());
}

#[test]
#[should_panic(expected = "the position leaves the signature's range")]
fn adapted_aborts_under_wrap_with_the_default_check() {
    let _ = WrapAborts::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
}

#[test]
#[should_panic(expected = "the position leaves the signature's range")]
fn adapted_aborts_under_saturate_with_abort_named() {
    let _ = SaturateAborts::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
}

#[test]
fn add_completes_under_quiet() {
    let wrapped = WrapQuiet::constant::<7>().add(WrapQuiet::constant::<1>());
    assert_eq!(wrapped.slot().index(), -8);
    let saturated = SaturateQuiet::constant::<7>().add(SaturateQuiet::constant::<1>());
    assert_eq!(saturated.slot().index(), 7);
}

#[test]
fn adapted_completes_under_quiet() {
    let wrapped = WrapQuiet::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
    assert_eq!(wrapped.slot().index(), -8);
    let saturated = SaturateQuiet::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
    assert_eq!(saturated.slot().index(), 7);
}

#[test]
fn abort_does_not_abort_on_what_fits() {
    assert_eq!(
        WrapAborts::constant::<7>()
            .add(WrapAborts::constant::<-1>())
            .slot()
            .index(),
        6
    );
    assert_eq!(
        SaturateAborts::constant::<-8>()
            .add(SaturateAborts::constant::<1>())
            .slot()
            .index(),
        -7
    );
    assert_eq!(
        WrapAborts::adapted(Exact::on_grid(Slot::at(7)))
            .slot()
            .index(),
        7
    );
    assert_eq!(
        SaturateAborts::adapted(Exact::on_grid(Slot::at(-8)))
            .slot()
            .index(),
        -8
    );
}

#[test]
fn the_check_changes_neither_the_member_nor_its_carrier() {
    use core::mem::size_of;
    assert_eq!(size_of::<WrapAborts>(), size_of::<WrapQuiet>());
    assert_eq!(
        WrapAborts::constant::<-3>().slot(),
        WrapQuiet::constant::<-3>().slot()
    );
}
