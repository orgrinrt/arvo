//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Which declared signatures a value is held for.

use arvo_format::overflow::{Overflow, Saturate, Wrap};
use arvo_format::quantum::is_constant_family;
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, Rounding, TowardZero};
use arvo_format::{Adaptation, Bool, DeclaredSignature, Format, Policy, Slots, Width, overflow_of};

use crate::map::Carried;

mod sealed {
    pub trait Policy {}
    pub trait Mode {}
}

/// Wrapping, and completion by it is the meaning of the operation.
///
/// The same `Policy::Wrap` as `arvo_format::overflow::Wrap`, without the debug
/// assertion: nothing asserts under this marker in any build profile.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Wrapping;

/// Saturation, and completion by it is the meaning of the operation.
///
/// The same `Policy::Saturate` as `arvo_format::overflow::Saturate`, without
/// the debug assertion: nothing asserts under this marker in any build profile.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Saturating;

impl Overflow for Wrapping {
    const POLICY: Policy = Policy::Wrap;
}

impl Overflow for Saturating {
    const POLICY: Policy = Policy::Saturate;
}

/// An overflow policy a value admits, and whether leaving the range asserts in
/// a debug build under it.
///
/// Sealed: the rung arithmetic computes wrapping and saturation and nothing
/// else. `Clamp` is not one, since with no bound to carry it is saturation
/// under another name.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not an overflow policy a value admits",
    note = "a value admits `Wrap` and `Saturate`, which assert in a debug build when a result leaves the range, and `Wrapping` and `Saturating`, which never do; `Clamp` with no bound is saturation under another name"
)]
pub trait ValuePolicy: Overflow + sealed::Policy {
    /// Whether a result leaving the range asserts in a debug build.
    const ASSERTS: Bool;
}

macro_rules! value_policy {
    ($($marker:ty => $asserts:expr),+ $(,)?) => {
        $(
            impl sealed::Policy for $marker {}
            impl ValuePolicy for $marker {
                const ASSERTS: Bool = $asserts;
            }
        )+
    };
}

value_policy! {
    Wrap => Bool::TRUE,
    Saturate => Bool::TRUE,
    Wrapping => Bool::FALSE,
    Saturating => Bool::FALSE,
}

/// A rounding mode a value admits: the five deterministic ones.
///
/// Sealed. `Stochastic` reads a dither, and a value has nowhere to carry one.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a rounding mode a value admits",
    note = "a value admits `TowardZero`, `Floor`, `Ceil`, `HalfUp` and `HalfEven`; `Stochastic` reads a dither, and a value has nowhere to carry one"
)]
pub trait ValueRounding: Rounding + sealed::Mode {}

macro_rules! value_rounding {
    ($($marker:ty),+ $(,)?) => {
        $(
            impl sealed::Mode for $marker {}
            impl ValueRounding for $marker {}
        )+
    };
}

value_rounding!(TowardZero, Floor, Ceil, HalfUp, HalfEven);

type SlotsOf<S> = <<S as DeclaredSignature>::Format as Format>::Slots;
type OverflowOf<S> = <<S as DeclaredSignature>::Adaptation as Adaptation>::Overflow;
type RoundingOf<S> = <<S as DeclaredSignature>::Adaptation as Adaptation>::Rounding;

/// A declared signature a value can be held for.
///
/// Implemented once, over every signature whose slot family is on the map,
/// whose overflow policy is a `ValuePolicy` and whose rounding mode is a
/// `ValueRounding`, so nothing outside this crate implements it.
pub trait Sole: DeclaredSignature<Format: Format<Slots: Carried>> {
    /// The declared width.
    const WIDTH: Width;

    /// The overflow policy.
    const POLICY: Policy;

    /// Whether a result leaving the range asserts in a debug build.
    const ASSERTS: Bool;

    /// The carrier arithmetic is the format's own only where the sum of two
    /// members is on the grid: a constant quantum and no phase. Every door
    /// forces this, so no value of another format exists.
    const ADMITTED: ();
}

impl<S> Sole for S
where
    S: DeclaredSignature,
    SlotsOf<S>: Carried,
    OverflowOf<S>: ValuePolicy,
    RoundingOf<S>: ValueRounding,
{
    const ADMITTED: () = assert!(
        <S::Format as Format>::PHASE.is_zero().get()
            && is_constant_family::<<S::Format as Format>::Quantum>().get(),
        "a value is held only for a format whose sum of two members is on its grid: a zero phase \
         and a constant quantum. This format has a phase or an indexed quantum, so its sums are \
         not slot sums; compute them with `arvo_format::add` on slots instead"
    );
    const ASSERTS: Bool = <OverflowOf<S> as ValuePolicy>::ASSERTS;
    const POLICY: Policy = overflow_of::<S::Adaptation>();
    const WIDTH: Width = <SlotsOf<S> as Slots>::WIDTH;
}
