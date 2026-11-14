//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The product's suite: the step and its conditions, the oracle sweep, the edge
//! sweep at widths nine to 64, and the law rows' cells.
//!
//! Every check reads its expected answer from `Ratio`, the crate's shared
//! oracle, from the applied map's oracle in `apply`, or, past eight bits, from
//! the host's unsigned 128-bit integer, never from the step.

use crate::adapt::{Adapt, Signature};
use crate::ambient::BinaryRationals;
use crate::format::{Format, radix};
use crate::overflow::{Saturate, Wrap};
use crate::quantum::{Constant, Quantum};
use crate::rounding::Floor;
use crate::slots::{Slot, Slots};
use crate::tests::grid::Grid;
use crate::tests::ratio::Ratio;

mod the_edges;
mod the_inverse_round_trip;
mod the_laws;
mod the_oracle;
mod the_step;
mod the_wrap_by_every_mode;
mod the_wrap_past_the_index;

/// A binary format over `S` at the quantum `2^E` and zero phase.
type At<S, const E: i32> = Grid<BinaryRationals, Constant<E>, S, 0, 1>;

/// Floor and saturation, the signature the law rows' saturating cells name.
type Sat<F> = Signature<F, Adapt<Floor, Saturate>>;

/// Floor and wrapping, the signature the law rows' wrapping cells name.
type Wr<F> = Signature<F, Adapt<Floor, Wrap>>;

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

/// The exact position the product of two members names: the product of their
/// values over the quantum, less the phase, as a rational.
fn exact_product<F: Format>(a: Slot, b: Slot) -> Ratio {
    value::<F>(a)
        .times(value::<F>(b))
        .over(quantum::<F>())
        .minus(Ratio::phase_of::<F>())
}

/// Every member of a format's range, lowest first.
fn members<F: Format>() -> impl Iterator<Item = Slot> {
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    (min ..= max).map(Slot::at)
}
