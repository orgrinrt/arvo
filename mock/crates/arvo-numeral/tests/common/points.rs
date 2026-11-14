//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Formats declared by the tests, the way a crate outside `arvo-format` would
//! declare one.
//!
//! The shipped points are binary. The radix is a coordinate of a format and the
//! trait is open, so a value is held over a decimal format as well, and over one
//! at a radix the format crate names no ambient for. These are those formats:
//! the unsigned decimal ambient, the two ternary ones, and a point over any
//! ambient and slot family at a constant exponent.

use core::marker::PhantomData;

use arvo_format::quantum::Constant;
use arvo_format::{Ambient, Bool, Format, Phase, Radix, Slots};

/// The non-negative rationals at radix ten.
pub struct UnsignedDecimalRationals;

impl Ambient for UnsignedDecimalRationals {
    const RADIX: Radix = Radix::DECIMAL;
    const SIGNED: Bool = Bool::FALSE;
}

/// The rationals at radix three, an odd radix sharing no factor with two.
pub struct TernaryRationals;

impl Ambient for TernaryRationals {
    const RADIX: Radix = Radix::of(3);
    const SIGNED: Bool = Bool::TRUE;
}

/// The non-negative rationals at radix three.
pub struct UnsignedTernaryRationals;

impl Ambient for UnsignedTernaryRationals {
    const RADIX: Radix = Radix::of(3);
    const SIGNED: Bool = Bool::FALSE;
}

/// The slot family `Sl` in the ambient `A`, each slot the radix to the `EXP`.
pub struct Scaled<A, Sl, const EXP: i32>(PhantomData<(A, Sl)>);

impl<A: Ambient, Sl: Slots, const EXP: i32> Format for Scaled<A, Sl, EXP> {
    type Ambient = A;
    type Quantum = Constant<EXP>;
    type Slots = Sl;

    const PHASE: Phase = Phase::ZERO;
}
