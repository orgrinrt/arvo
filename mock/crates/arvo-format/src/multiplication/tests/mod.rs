//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The product's suite: the step and its refusals, the oracle sweep, and the
//! law rows' cells.
//!
//! Every check reads its expected answer from `Ratio`, the crate's shared
//! oracle, or from the applied map's oracle in `apply`, never from the step.

use crate::adapt::{Adapt, Signature};
use crate::ambient::BinaryRationals;
use crate::format::Format;
use crate::overflow::{Saturate, Wrap};
use crate::quantum::{Constant, Quantum};
use crate::rounding::Floor;
use crate::slots::{Slot, Slots};
use crate::tests::grid::Grid;
use crate::tests::ratio::Ratio;

mod the_laws;
mod the_oracle;
mod the_step;

/// A binary format over `S` at the quantum `2^E` and zero phase.
type At<S, const E: i32> = Grid<BinaryRationals, Constant<E>, S, 0, 1>;

/// Floor and saturation, the signature the law rows' saturating cells name.
type Sat<F> = Signature<F, Adapt<Floor, Saturate>>;

/// Floor and wrapping, the signature the law rows' wrapping cells name.
type Wr<F> = Signature<F, Adapt<Floor, Wrap>>;

/// The quantum of a constant binary format.
fn quantum<F: Format>() -> Ratio {
    let e = <F::Quantum as Quantum>::BASE.power();
    if e >= 0 { Ratio::whole(1i128 << e) } else { Ratio::of(1, 1i128 << -e) }
}

/// The exact position the product of two members names: the slots' product
/// times the quantum, as a rational.
fn exact_product<F: Format>(a: Slot, b: Slot) -> Ratio {
    Ratio::whole(a.index())
        .times(Ratio::whole(b.index()))
        .times(quantum::<F>())
}

/// Every member of a format's range, lowest first.
fn members<F: Format>() -> impl Iterator<Item = Slot> {
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    (min ..= max).map(Slot::at)
}
