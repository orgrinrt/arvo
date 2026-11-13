//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The debug check in a release build. `Abort` aborts only where the debug
//! profile applies, so here the inputs that abort in `the_debug_check` complete
//! by the policy under `Abort` exactly as they do under `Quiet`: the check is a
//! constant branch that clears at lowering. Compiled only without
//! `debug_assertions`, so it runs under `cargo test --release`.

#![cfg(not(debug_assertions))]

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Exact, Slot};
use arvo_numeral::{Abort, Int, Quiet};

type WrapAborts = Int<4, Adapt<TowardZero, Wrap>>;
type SaturateAborts = Int<4, Adapt<TowardZero, Saturate>, Abort>;
type WrapQuiet = Int<4, Adapt<TowardZero, Wrap>, Quiet>;

/// One past the top of a 4-bit signed range.
const PAST_THE_TOP: i128 = 8;

#[test]
fn add_completes_under_abort_in_release() {
    let wrapped = WrapAborts::constant::<7>().add(WrapAborts::constant::<1>());
    assert_eq!(wrapped.slot().index(), -8);
    let saturated = SaturateAborts::constant::<7>().add(SaturateAborts::constant::<1>());
    assert_eq!(saturated.slot().index(), 7);
}

#[test]
fn adapted_completes_under_abort_in_release() {
    let wrapped = WrapAborts::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
    assert_eq!(wrapped.slot().index(), -8);
    let saturated = SaturateAborts::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
    assert_eq!(saturated.slot().index(), 7);
}

#[test]
fn sub_neg_and_mul_complete_under_abort_in_release() {
    assert_eq!(
        WrapAborts::constant::<-8>()
            .sub(WrapAborts::constant::<1>())
            .slot()
            .index(),
        7
    );
    assert_eq!(
        SaturateAborts::constant::<-8>()
            .sub(SaturateAborts::constant::<1>())
            .slot()
            .index(),
        -8
    );
    assert_eq!(WrapAborts::constant::<-8>().neg().slot().index(), -8);
    assert_eq!(SaturateAborts::constant::<-8>().neg().slot().index(), 7);
    assert_eq!(
        WrapAborts::constant::<4>()
            .mul(WrapAborts::constant::<-3>())
            .slot()
            .index(),
        4
    );
    assert_eq!(
        SaturateAborts::constant::<4>()
            .mul(SaturateAborts::constant::<-3>())
            .slot()
            .index(),
        -8
    );
}

#[test]
fn abort_and_quiet_agree_in_release() {
    let a = WrapAborts::constant::<7>().add(WrapAborts::constant::<1>());
    let q = WrapQuiet::constant::<7>().add(WrapQuiet::constant::<1>());
    assert_eq!(a.slot(), q.slot());
}
