//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quotient's suite: the divisor's two doors and the conditions, the oracle
//! sweep up to eight bits with its planted step, and the edge sweep at widths
//! nine to 64 that places the quotient at the divisor's extremes.
//!
//! Every check reads its expected answer from `Ratio`, from the applied map's
//! oracle in `apply`, or, past eight bits, from `Big`, never from the step.

use crate::format::{Format, radix};
use crate::quantum::Quantum;
use crate::slots::{Slot, Slots};
use crate::tests::ratio::Ratio;

mod the_divisor;
mod the_edges;
mod the_far_quotient;
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

/// The exact position the quotient of two members names: the quotient of their
/// values over the quantum, less the phase, as a rational.
fn exact_quotient<F: Format>(a: Slot, b: Slot) -> Ratio {
    value::<F>(a)
        .over(value::<F>(b))
        .over(quantum::<F>())
        .minus(Ratio::phase_of::<F>())
}

/// Every member of a format's range, lowest first.
fn members<F: Format>() -> impl Iterator<Item = Slot> {
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    (min ..= max).map(Slot::at)
}

/// Every member whose value is not zero, lowest first.
fn divisors<F: Format>() -> impl Iterator<Item = Slot> {
    members::<F>().filter(|&b| value::<F>(b).num != 0)
}
