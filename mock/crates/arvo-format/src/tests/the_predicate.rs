//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Membership, the quantum law, the slot range, the phase's own contract and the
//! radix, over the whole matrix.
//!
//! The laws the design names about what is in a format's representable set.
//! Choosing which instantiations to include is choosing what not to find out, so
//! the width sweeps run every width the slot ranges admit rather than the powers
//! of two somebody would reach for. Where a law is structural the assertion is a
//! compile-time one, because a runtime check of a compile-time property tests the
//! test rather than the property.
//!
//! Whether zero is in the set is one question and it is in `the_identity`, which
//! is its own law and its own file. What is here is the predicate over coordinates
//! a caller supplies.

use crate::ambient::{Ambient, BinaryRationals, DecimalRationals, Radix, UnsignedBinaryRationals};
use crate::format::{contains, radix, smallest_step_exponent, Phase};
use crate::points::{Biased, Floating, Integer, UFixed};
use crate::quantum::{
    exponent_at, is_constant_family, Constant, Exponent, Indexed, Magnitude, MagnitudeCount,
    Quantum,
};
use crate::slots::{slot_count, slot_in_range, Signed, Slot, SlotCount, Unsigned};
use crate::width::Bool;

// --- the control -------------------------------------------------------------
//
// Every sweep below rests on the slot ranges genuinely differing across widths.
// If they do not, every "holds at every width" result is one width reported many
// times.

#[test]
fn the_control_the_widths_produce_different_slot_ranges() {
    assert_ne!(slot_count::<Unsigned<3>>(), slot_count::<Unsigned<4>>());
    assert_ne!(slot_count::<Signed<3>>(), slot_count::<Signed<4>>());
    assert_ne!(
        slot_count::<Unsigned<13>>(),
        slot_count::<Unsigned<14>>(),
        "a non-power-of-two width has to be distinguishable too, or the sweep is \
         only testing the shapes a bare primitive already had"
    );
}

// --- membership is decidable at const time, for every point ------------------
//
// Asserted as `const` bindings rather than runtime calls. A runtime assertion
// would pass even if the function were not const, which is the property under
// test.

const _INTEGER_MEMBERSHIP: Bool = contains::<Integer<8>>(Slot::ZERO, Magnitude::SMALLEST);
const _UFIXED_MEMBERSHIP: Bool = contains::<UFixed<13, -4>>(Slot::ZERO, Magnitude::SMALLEST);
const _BIASED_MEMBERSHIP: Bool = contains::<Biased<7, -2, 1>>(Slot::ZERO, Magnitude::SMALLEST);
const _FLOATING_MEMBERSHIP: Bool =
    contains::<Floating<11, -14, 30>>(Slot::ZERO, Magnitude::SMALLEST);

#[test]
fn membership_is_decidable_at_const_time_for_every_point() {
    // The four bindings above are the assertion; if any were not const this file
    // would not compile. This body checks they are also correct rather than
    // merely evaluable, which the bindings alone do not say.
    assert!(_INTEGER_MEMBERSHIP.get());
    assert!(_UFIXED_MEMBERSHIP.get());
    assert!(_BIASED_MEMBERSHIP.get());
    assert!(_FLOATING_MEMBERSHIP.get());
}

// --- the slot range is exactly what the width declares, at every width -------

macro_rules! sweep_unsigned_widths {
    ($($w:literal),+ $(,)?) => {
        #[test]
        fn an_unsigned_slot_range_is_exactly_what_its_width_declares() {
            $(
                assert_eq!(
                    slot_count::<Unsigned<$w>>(),
                    SlotCount::of(1i64 << $w),
                    "width {} admits the wrong number of slots", $w
                );
                assert!(slot_in_range::<Unsigned<$w>>(Slot::ZERO).get());
                assert!(slot_in_range::<Unsigned<$w>>(Slot::at((1i64 << $w) - 1)).get());
                assert!(!slot_in_range::<Unsigned<$w>>(Slot::at(1i64 << $w)).get());
                assert!(!slot_in_range::<Unsigned<$w>>(Slot::at(-1)).get());
            )+
        }
    };
}

macro_rules! sweep_signed_widths {
    ($($w:literal),+ $(,)?) => {
        #[test]
        fn a_signed_slot_range_is_exactly_what_its_width_declares() {
            $(
                assert_eq!(
                    slot_count::<Signed<$w>>(),
                    SlotCount::of(1i64 << $w),
                    "width {} admits the wrong number of slots", $w
                );
                assert!(slot_in_range::<Signed<$w>>(Slot::ZERO).get());
                assert!(slot_in_range::<Signed<$w>>(Slot::at(-(1i64 << ($w - 1)))).get());
                assert!(slot_in_range::<Signed<$w>>(Slot::at((1i64 << ($w - 1)) - 1)).get());
                assert!(!slot_in_range::<Signed<$w>>(Slot::at(1i64 << ($w - 1))).get());
                assert!(!slot_in_range::<Signed<$w>>(Slot::at(-(1i64 << ($w - 1)) - 1)).get());
            )+
        }
    };
}

// Every width the design admits, 1 through 62, not the powers of two and not a
// convenient prefix. The bound is the set of impls `slots` writes, and above it
// no impl exists, so these are the widths that exist.
//
// The previous cut of this stopped at 32 while the ladder reached 64 and this
// file claimed to run every width the slot ranges admit. The unswept half is
// where the count overflowed.
sweep_unsigned_widths!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62
);

sweep_signed_widths!(
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62
);

// --- the two families differ on the quantum law and nowhere else -------------

#[test]
fn the_constant_family_has_one_magnitude_and_the_indexed_family_has_many() {
    assert!(is_constant_family::<Constant<0>>().get());
    assert!(is_constant_family::<Constant<-4>>().get());
    assert!(!is_constant_family::<Indexed<-14, 30>>().get());

    assert_eq!(<Constant<0> as Quantum>::MAGNITUDES, MagnitudeCount::of(1));
    assert_eq!(<Constant<-7> as Quantum>::MAGNITUDES, MagnitudeCount::of(1));
    assert_eq!(
        <Indexed<-14, 30> as Quantum>::MAGNITUDES,
        MagnitudeCount::of(30)
    );
}

#[test]
fn a_constant_quantum_does_not_move_with_magnitude_and_an_indexed_one_does() {
    // Constant: the exponent is the same at every magnitude it admits.
    assert_eq!(
        exponent_at::<Constant<-4>>(Magnitude::SMALLEST),
        Exponent::of(-4)
    );

    // Indexed: one exponent per magnitude step, which is the floating shape.
    assert_eq!(
        exponent_at::<Indexed<-14, 30>>(Magnitude::SMALLEST),
        Exponent::of(-14)
    );
    assert_eq!(
        exponent_at::<Indexed<-14, 30>>(Magnitude::at(1)),
        Exponent::of(-13)
    );
    assert_eq!(
        exponent_at::<Indexed<-14, 30>>(Magnitude::at(29)),
        Exponent::of(15)
    );

    // And the two genuinely differ, which is the control for the pair above.
    assert_ne!(
        exponent_at::<Indexed<-14, 30>>(Magnitude::SMALLEST),
        exponent_at::<Indexed<-14, 30>>(Magnitude::at(1))
    );
}

// --- subnormals fall out of the smallest magnitude, unnamed ------------------

#[test]
fn the_smallest_step_is_the_smallest_magnitudes_and_nothing_names_it() {
    // The floating point's smallest step is the one at its lowest magnitude, and
    // the values that step admits are what the conventions call subnormal. No
    // branch in this crate selects them and no name in it mentions them.
    assert_eq!(
        smallest_step_exponent::<Floating<11, -14, 30>>(),
        Exponent::of(-14)
    );
    assert_eq!(
        smallest_step_exponent::<Floating<24, -126, 254>>(),
        Exponent::of(-126)
    );

    // For the constant family the smallest step is the only step.
    assert_eq!(smallest_step_exponent::<UFixed<13, -4>>(), Exponent::of(-4));
    assert_eq!(smallest_step_exponent::<Integer<8>>(), Exponent::ZERO);
}

// --- the phase's own contract ------------------------------------------------

#[test]
fn a_phase_never_carries_a_denominator_that_cannot_divide() {
    // The condition an earlier doc comment stated and nothing held. Over the
    // inputs that reach the coercion as well as the ones that do not, because a
    // sweep of only the good ones would pass against a constructor doing nothing.
    for den in [i64::MIN, -9i64, -1, 0, 1, 2, 5, i64::MAX] {
        for num in [i64::MIN, -3, 0, 3, i64::MAX] {
            let p = Phase::of(num, den);
            assert!(
                p.denominator() > 0,
                "Phase::of({num}, {den}) produced a denominator of {}",
                p.denominator()
            );
            // And the value, which the sign assertion alone never asked about.
            // A negative denominator carries one, so normalising it has to move
            // the sign rather than change the magnitude. Without this the arm
            // passed against a constructor that answered `3/1` for `of(3, -7)`.
            if den < 0 && den != i64::MIN && num != i64::MIN {
                assert_eq!(
                    (p.numerator(), p.denominator()),
                    (-num, -den),
                    "Phase::of({num}, {den}) has to be exactly {}/{}",
                    -num,
                    -den
                );
            }
        }
    }
    // The two pairs `i64::MIN` puts out of reach, named rather than swept over.
    // Both are lossy on purpose and the doc says so; pinning them here is what
    // stops the loss spreading to the pairs that do normalise.
    //
    // **The pair, not the denominator.** Asserting only the sign of the
    // denominator was the same defect twelve lines above says a sign assertion
    // alone carries: `of(3, i64::MIN)` names a tiny negative and answers `+3/1`,
    // and `of(i64::MIN, -7)` names a large positive and answers negative. So
    // neither the sign nor the magnitude survives on these two, and the
    // assertion says which rather than implying the sign is kept.
    assert_eq!(
        (
            Phase::of(3, i64::MIN).numerator(),
            Phase::of(3, i64::MIN).denominator()
        ),
        (3, 1)
    );
    assert_eq!(
        (
            Phase::of(i64::MIN, -7).numerator(),
            Phase::of(i64::MIN, -7).denominator()
        ),
        (i64::MIN, 1)
    );
    // The exactly-representable negative case, which is the whole finding.
    assert_eq!(Phase::of(3, -7).numerator(), -3);
    assert_eq!(Phase::of(3, -7).denominator(), 7);
    // A zero denominator names no value, so it is read as one and the numerator
    // is kept. The control that keeps the branch above from swallowing this one.
    assert_eq!(
        (Phase::of(3, 0).numerator(), Phase::of(3, 0).denominator()),
        (3, 1)
    );
    // The control: a usable denominator is kept rather than replaced, so the
    // coercion is about the cases that need it and not about all of them.
    assert_eq!(Phase::of(3, 7).denominator(), 7);
    assert_eq!(Phase::of(3, 7).numerator(), 3);
    // The half-step shape the biased point uses, and the zero the other three do.
    assert_eq!(Phase::halves(1).denominator(), 2);
    assert_eq!(Phase::halves(1).numerator(), 1);
}

// --- the radix is a coordinate and is not hardcoded --------------------------

#[test]
fn the_radix_comes_from_the_ambient_domain() {
    assert_eq!(radix::<Integer<8>>(), Radix::BINARY);
    assert_eq!(<DecimalRationals as Ambient>::RADIX, Radix::DECIMAL);
    assert_ne!(
        <BinaryRationals as Ambient>::RADIX,
        <DecimalRationals as Ambient>::RADIX,
        "if these agree the radix is not a coordinate and every claim about it is \
         a claim about one value"
    );
    // The bases behind the two names, so the constants are not two spellings of
    // one number nobody looked at.
    assert_eq!(Radix::BINARY.base(), 2);
    assert_eq!(Radix::DECIMAL.base(), 10);
    assert!(!Radix::BINARY.equals(Radix::DECIMAL).get());
    assert!(Radix::BINARY.equals(Radix::of(2)).get());
}

#[test]
fn signedness_is_a_property_of_the_domain_and_not_of_a_carrier() {
    assert!(<BinaryRationals as Ambient>::SIGNED.get());
    assert!(!<UnsignedBinaryRationals as Ambient>::SIGNED.get());
}
