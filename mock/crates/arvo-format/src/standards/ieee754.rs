//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! IEEE 754's interchange formats, as one composition at the standard's own
//! parameters.
//!
//! The standard states every format by a radix, a precision `p` and a largest
//! exponent `emax`, with `emin = 1 - emax`. `Interchange` is that statement as a
//! format, the radix carried by the ambient domain, and binary and decimal are
//! the same composition at two ambients. Nothing here is a case of its own: the
//! formats in the standard's table are aliases at its numbers.
//!
//! Three things the standard carries have no member here, and each is a test
//! rather than a sentence. An infinity, which is its answer past the largest
//! finite member under round to nearest and under the direction away from zero.
//! The signed zero, which the value set does not tell from zero. And binary128
//! and decimal128, whose significands are wider than the slot ladder.

use core::marker::PhantomData;

use crate::ambient::{Ambient, BinaryRationals, DecimalRationals};
use crate::format::{Format, Phase};
use crate::quantum::{Exponent, MagnitudeCount, Quantum};
use crate::slots::{Slot, Slots};
use crate::width::Width;

/// The exponent range of a format at precision `P` and largest exponent `EMAX`,
/// as the quantum law it names.
///
/// The magnitude-indexed family. The smallest step is the radix to
/// `emin - (p - 1)`, which is `2 - emax - p`, and it rises one exponent per
/// magnitude up to the step of the top binade, `emax - (p - 1)`, so there are
/// `2 emax` magnitudes. Both are expressions in the parameters, so both are
/// computed here in the impl body, where an arbitrary expression is legal, rather
/// than in a const generic argument, where it is not.
pub struct ExponentRange<const P: u32, const EMAX: u32>;

impl<const P: u32, const EMAX: u32> Quantum for ExponentRange<P, EMAX> {
    const BASE: Exponent = Exponent::of(2 - (EMAX as i32) - (P as i32));
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(2 * EMAX);
    const SLOPE: Exponent = Exponent::ONE;
}

/// The significand of a format at precision `P` over the ambient `A`, as the
/// slot range it names.
///
/// Symmetric at `radix^p - 1`, because the standard's significand is sign and
/// magnitude rather than two's complement, and declared one bit wider than the
/// magnitude needs, for the sign. A precision whose bound passes the slot index
/// saturates at its top, and the width that follows is past 64 either way, so the
/// range's own obligation refuses it where it is forced.
pub struct Significand<A, const P: u32>(PhantomData<A>);

/// `radix^p - 1`, or the slot index's top where the power passes it.
const fn largest_significand(radix: u32, p: u32) -> i128 {
    let mut power: i128 = 1;
    let mut taken = 0;
    while taken < p {
        power = match power.checked_mul(radix as i128) {
            Some(next) => next,
            None => return i128::MAX,
        };
        taken += 1;
    }
    power - 1
}

/// The bits a significand bound needs, with one more for the sign.
const fn signed_width_of(largest: i128) -> Width {
    Width::bits(i128::BITS - largest.leading_zeros() + 1)
}

impl<A: Ambient, const P: u32> Slots for Significand<A, P> {
    const MAX: Slot = Slot::at(largest_significand(A::RADIX.base(), P));
    const MIN: Slot = Slot::at(-largest_significand(A::RADIX.base(), P));
    const WIDTH: Width = signed_width_of(largest_significand(A::RADIX.base(), P));
}

/// An IEEE 754 interchange format over the ambient `A`, at precision `P` and
/// largest exponent `EMAX`.
///
/// The union over magnitudes of each magnitude's multiples of its step, within
/// the significand, is exactly the standard's finite values: the subnormals are
/// the smallest magnitude's, with no case naming them, and every normal value is
/// its own binade's.
pub struct Interchange<A, const P: u32, const EMAX: u32>(PhantomData<A>);

impl<A: Ambient, const P: u32, const EMAX: u32> Format for Interchange<A, P, EMAX> {
    type Ambient = A;
    type Quantum = ExponentRange<P, EMAX>;
    type Slots = Significand<A, P>;

    // Every finite member is a significand times a power of the radix, so the
    // grid runs through zero.
    const PHASE: Phase = Phase::ZERO;
}

/// A binary interchange format.
pub type Binary<const P: u32, const EMAX: u32> = Interchange<BinaryRationals, P, EMAX>;

/// A decimal interchange format.
pub type Decimal<const P: u32, const EMAX: u32> = Interchange<DecimalRationals, P, EMAX>;

/// binary16: `p = 11`, `emax = 15`.
pub type Binary16 = Binary<11, 15>;

/// binary32: `p = 24`, `emax = 127`.
pub type Binary32 = Binary<24, 127>;

/// binary64: `p = 53`, `emax = 1023`.
pub type Binary64 = Binary<53, 1023>;

/// decimal32: `p = 7`, `emax = 96`.
pub type Decimal32 = Decimal<7, 96>;

/// decimal64: `p = 16`, `emax = 384`.
pub type Decimal64 = Decimal<16, 384>;

/// A format under a rounding-direction attribute, which is the declared signature
/// the pair names.
///
/// The overflow policy is the consumer's. The standard's answer past the largest
/// finite member is an infinity under round to nearest and under the direction
/// away from zero, the format has no member there, and choosing a policy here
/// would hide that rather than state it.
pub type Attributed<F, R, O> = crate::adapt::Signature<F, crate::adapt::Adapt<R, O>>;

/// The rounding-direction attributes, under the standard's names.
///
/// Four of the five land on a shipped mode. `roundTiesToAway` is MATLAB's Round
/// under another name: a tie away from zero is not one of the six modes, and the
/// canon reaches it as the alias the MATLAB parity suite writes.
pub mod rounding_direction {
    pub use crate::rounding::{
        Ceil as RoundTowardPositive,
        Floor as RoundTowardNegative,
        HalfEven as RoundTiesToEven,
        TowardZero as RoundTowardZero,
    };
}
