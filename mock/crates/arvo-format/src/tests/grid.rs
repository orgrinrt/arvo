//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A format with every coordinate free, for the suites that sweep one.
//!
//! The identity's suite sweeps the four axes the cancellation depends on and
//! addition's suite sweeps the quantum exponent, the radix and the phase, and
//! both want the same instrument. It lived in the first of those and is here so
//! the second reuses it rather than declaring a second format of the same shape.

use core::marker::PhantomData;

use crate::ambient::Ambient;
use crate::format::{Format, Phase};
use crate::quantum::Quantum;
use crate::slots::Slots;

/// A format with every coordinate free, which is what a sweep needs.
///
/// The shipped points each pin at least two of the four axes a format is
/// declared over. This pins none.
///
/// The phase arrives as two const generic parameters and becomes a `Phase` in the
/// impl body. An associated const takes an arbitrary expression, so the coordinate
/// carrying a type of its own costs the sweep nothing; only a const generic
/// parameter is restricted to the host's types, and that is the position
/// `tests/ui/an_arvo_type_as_a_const_parameter.rs` pins.
pub(crate) struct Grid<A, Q, S, const PN: i64, const PD: i64>(PhantomData<(A, Q, S)>);

impl<A: Ambient, Q: Quantum, S: Slots, const PN: i64, const PD: i64> Format
    for Grid<A, Q, S, PN, PD>
{
    type Ambient = A;
    type Quantum = Q;
    type Slots = S;

    const PHASE: Phase = Phase::of(PN, PD);
}
