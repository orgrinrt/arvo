//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! IEEE 754's interchange formats, one composition at the standard's numbers.
//!
//! The standard states every format by a radix, a precision `p` and a largest
//! exponent `emax`, with `emin = 1 - emax`. `Interchange` is that statement as a
//! format, the radix carried by the ambient domain, and binary and decimal are
//! the same composition at two ambients. Nothing here is a case of its own: the
//! formats in the standard's table are aliases at its numbers.
//!
//! Four things the standard carries have no member or no arm here, and each is a
//! test and a ticket. An infinity, which is its answer past the largest finite
//! member under round to nearest and under the direction away from zero. The
//! signed zero, which the value set does not tell from zero. Binary128 and
//! decimal128, whose significands are wider than the slot ladder. And putting a
//! value on the format at all, since conversion and the operations refuse a
//! floating format.

// FIXME(arvo_no_arithmetic_runs_through_both_sides_of_the_ieee_754_p): the
// parity is of the adaptation of a value; a sum, product or quotient over an
// interchange format waits on a magnitude coordinate in the applied map.
// FIXME(arvo_nothing_puts_a_value_onto_an_ieee_754_interchange_forma): conversion
// onto and off a format here refuses as well, so the parity suites land a value
// by a search in the test tree.

use core::marker::PhantomData;

use crate::ambient::{Ambient, BinaryRationals, DecimalRationals};
use crate::format::{Format, Phase};
use crate::quantum::{Exponent, MagnitudeCount, Quantum};
use crate::slots::{Slot, Slots};
use crate::width::{Bool, Width};

/// The exponent range at precision `P` and largest exponent `EMAX`.
///
/// The quantum law it names, the magnitude-indexed family. The smallest step is
/// the radix to `emin - (p - 1)`, which is `2 - emax - p`, and it rises one
/// exponent per magnitude up to the step of the top binade, `emax - (p - 1)`, so
/// there are `2 emax` magnitudes. Both are expressions in the parameters, so both
/// are computed here in the impl body, where an arbitrary expression is legal,
/// rather than in a const generic argument, where it is not.
pub struct ExponentRange<const P: u32, const EMAX: u32>;

impl<const P: u32, const EMAX: u32> Quantum for ExponentRange<P, EMAX> {
    const BASE: Exponent = Exponent::of(2 - (EMAX as i32) - (P as i32));
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(2 * EMAX);
    const SLOPE: Exponent = Exponent::ONE;
}

/// The significand at precision `P` over the ambient `A`.
///
/// The slot range it names, symmetric at `radix^p - 1`, because the standard's
/// significand is sign and magnitude rather than two's complement, and declared
/// one bit wider than the magnitude needs, for the sign. Its bounds are
/// `SignificandBounds`, which carries the slot range's own obligation; this type
/// adds what sign and magnitude owes, and its `ADMITTED` forces both.
pub struct Significand<A, const P: u32>(PhantomData<A>);

/// The bounds of a significand, under the slot range's own obligation only.
///
/// A precision whose bound passes the slot index saturates at its top, and the
/// width that follows is past 64 either way, so the obligation refuses it where
/// it is forced.
// FIXME(arvo_binary128_and_decimal128_need_a_slot_ladder_past_64_bit): binary128
// and decimal128 need 114 bits and are refused here until the ladder widens.
pub struct SignificandBounds<A, const P: u32>(PhantomData<A>);

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

impl<A: Ambient, const P: u32> Slots for SignificandBounds<A, P> {
    const MAX: Slot = Slot::at(largest_significand(A::RADIX.base(), P));
    const MIN: Slot = Slot::at(-largest_significand(A::RADIX.base(), P));
    const WIDTH: Width = signed_width_of(largest_significand(A::RADIX.base(), P));
}

/// Whether a significand over `A` at precision `P` is sign and magnitude.
///
/// The verdict beside the significand's obligation, asking the same question
/// without forcing it: the ambient carries values below zero, which the
/// symmetric range names, and the precision holds at least one digit.
#[must_use]
pub const fn is_sign_and_magnitude<A: Ambient, const P: u32>() -> Bool {
    A::SIGNED.and(Bool::of(P >= 1))
}

// FIXME(arvo_the_slots_verdict_cannot_see_an_overridden_admitted): the free
// verdict `slots::is_admissible` reads the bounds and not this override, so it
// answers true for a significand this obligation refuses.
impl<A: Ambient, const P: u32> Slots for Significand<A, P> {
    /// Sign and magnitude first, then the slot range's own obligation over the
    /// bounds, so both are met wherever the range's obligation is forced.
    const ADMITTED: () = {
        assert!(
            A::SIGNED.get(),
            "an IEEE 754 significand is sign and magnitude, so its ambient domain must carry values \
             below zero; this ambient is unsigned"
        );
        assert!(
            P >= 1,
            "an IEEE 754 significand holds at least one digit; a precision of zero holds only zero"
        );
        <SignificandBounds<A, P> as Slots>::ADMITTED
    };
    const MAX: Slot = <SignificandBounds<A, P> as Slots>::MAX;
    const MIN: Slot = <SignificandBounds<A, P> as Slots>::MIN;
    const WIDTH: Width = <SignificandBounds<A, P> as Slots>::WIDTH;
}

/// An IEEE 754 interchange format over `A`, at precision `P` and emax `EMAX`.
///
/// The union over magnitudes of each magnitude's multiples of its step, within
/// the significand, is exactly the standard's finite values: the subnormals are
/// the smallest magnitude's, with no case naming them, and every normal value is
/// its own binade's.
// FIXME(arvo_the_ieee_754_infinities_and_signed_zero_have_no_member): no member
// stands for an infinity or for the negative zero, so the overflow parity holds
// only where the standard's answer is finite and a zero is compared as a value.
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

/// A format under a rounding-direction attribute, as a declared signature.
///
/// The overflow policy is the consumer's. The standard's answer past the largest
/// finite member is an infinity under round to nearest and under the direction
/// away from zero, the format has no member there, and choosing a policy here
/// would hide that rather than state it.
pub type Attributed<F, R, O> = crate::adapt::Signature<F, crate::adapt::Adapt<R, O>>;

/// The rounding-direction attributes, under the standard's names.
///
/// Four of the five land on a shipped mode, as type aliases rather than renamed
/// re-exports: a re-export under another name becomes the path the compiler
/// prints for the mode, so a consumer of `TowardZero` who never read IEEE 754
/// would meet `RoundTowardZero` in a diagnostic. `roundTiesToAway` is MATLAB's
/// Round under another name: a tie away from zero is not one of the six modes,
/// and the canon reaches it as the alias the MATLAB parity suite writes.
pub mod rounding_direction {
    /// `roundTowardPositive`.
    pub type RoundTowardPositive = crate::rounding::Ceil;

    /// `roundTowardNegative`.
    pub type RoundTowardNegative = crate::rounding::Floor;

    /// `roundTiesToEven`.
    pub type RoundTiesToEven = crate::rounding::HalfEven;

    /// `roundTowardZero`.
    pub type RoundTowardZero = crate::rounding::TowardZero;
}
