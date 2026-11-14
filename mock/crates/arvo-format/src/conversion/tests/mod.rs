//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion's suite: every member up to eight bits onto targets across
//! exponents, families, phases and radixes against the applied map's oracle,
//! with a planted step, and the range ends at widths nine to 64 against `Big`.
//!
//! Every check reads its expected answer from `Ratio`, from the applied map's
//! oracle in `apply`, or from `Big`, never from the step.

use crate::format::{Format, radix};
use crate::quantum::Quantum;
use crate::slots::{Slot, Slots};
use crate::tests::ratio::Ratio;

mod the_edges;
mod the_far_conversion;
mod the_oracle;

/// The quantum of a constant format, `r^e`.
fn quantum<F: Format>() -> Ratio {
    let r = radix::<F>().base() as i128;
    let e = <F::Quantum as Quantum>::BASE.power();
    if e >= 0 {
        Ratio::whole(r.pow(e as u32))
    } else {
        Ratio::of(1, r.pow(e.unsigned_abs()))
    }
}

/// The value of the member at `slot`, `(phase + slot) * q`.
fn value<F: Format>(slot: Slot) -> Ratio {
    Ratio::phase_of::<F>()
        .plus(Ratio::whole(slot.index()))
        .times(quantum::<F>())
}

/// The exact position on the grid of `T` the member of `F` at `a` names.
fn exact_conversion<F: Format, T: Format>(a: Slot) -> Ratio {
    value::<F>(a)
        .over(quantum::<T>())
        .minus(Ratio::phase_of::<T>())
}

/// Every member of a format's range, lowest first.
fn members<F: Format>() -> impl Iterator<Item = Slot> {
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    (min ..= max).map(Slot::at)
}
