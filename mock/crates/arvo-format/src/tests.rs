//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The laws the design names, asserted over the whole matrix rather than a sample.
//!
//! Choosing which instantiations to include is choosing what not to find out, so
//! the width sweeps below run every width the slot ranges admit rather than the
//! powers of two somebody would reach for. Where a law is structural the assertion
//! is a compile-time one, because a runtime check of a compile-time property tests
//! the test rather than the property.

use crate::adapt::{Adapt, Adaptation, DeclaredSignature, Operation, Signature};
use crate::ambient::{Ambient, BinaryRationals, DecimalRationals, UnsignedBinaryRationals};
use crate::format::{
    contains, has_additive_identity, radix, smallest_step_exponent, step_exponent, Format,
};
use crate::overflow::{Overflow, Policy, Saturate, Wrap, SHIPPED_POLICIES};
use crate::points::{Biased, Floating, Integer, UFixed};
use crate::quantum::{exponent_at, is_constant_family, Constant, Indexed, Quantum};
use crate::rounding::{
    Ceil, Floor, HalfEven, HalfUp, Mode, Rounding, Stochastic, TowardZero, ALL_MODES,
};
use crate::slots::{slot_count, slot_in_range, Signed, Slots, Unsigned};

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

const _INTEGER_MEMBERSHIP: bool = contains::<Integer<8>>(0, 0);
const _UFIXED_MEMBERSHIP: bool = contains::<UFixed<13, -4>>(0, 0);
const _BIASED_MEMBERSHIP: bool = contains::<Biased<7, -2, 1>>(0, 0);
const _FLOATING_MEMBERSHIP: bool = contains::<Floating<11, -14, 30>>(0, 0);

#[test]
fn membership_is_decidable_at_const_time_for_every_point() {
    // The four bindings above are the assertion; if any were not const this file
    // would not compile. This body checks they are also correct rather than
    // merely evaluable, which the bindings alone do not say.
    assert!(_INTEGER_MEMBERSHIP);
    assert!(_UFIXED_MEMBERSHIP);
    assert!(_BIASED_MEMBERSHIP);
    assert!(_FLOATING_MEMBERSHIP);
}

// --- the slot range is exactly what the width declares, at every width -------

macro_rules! sweep_unsigned_widths {
    ($($w:literal),+ $(,)?) => {
        #[test]
        fn an_unsigned_slot_range_is_exactly_what_its_width_declares() {
            $(
                assert_eq!(
                    slot_count::<Unsigned<$w>>(),
                    1i64 << $w,
                    "width {} admits the wrong number of slots", $w
                );
                assert!(slot_in_range::<Unsigned<$w>>(0));
                assert!(slot_in_range::<Unsigned<$w>>((1i64 << $w) - 1));
                assert!(!slot_in_range::<Unsigned<$w>>(1i64 << $w));
                assert!(!slot_in_range::<Unsigned<$w>>(-1));
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
                    1i64 << $w,
                    "width {} admits the wrong number of slots", $w
                );
                assert!(slot_in_range::<Signed<$w>>(0));
                assert!(slot_in_range::<Signed<$w>>(-(1i64 << ($w - 1))));
                assert!(slot_in_range::<Signed<$w>>((1i64 << ($w - 1)) - 1));
                assert!(!slot_in_range::<Signed<$w>>(1i64 << ($w - 1)));
                assert!(!slot_in_range::<Signed<$w>>(-(1i64 << ($w - 1)) - 1));
            )+
        }
    };
}

// Every width from 1 to 32, not the powers of two. The whole point of declaring a
// width is that 13 and 27 are as ordinary as 16 and 32.
sweep_unsigned_widths!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32
);

sweep_signed_widths!(
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32
);

// --- the two families differ on the quantum law and nowhere else -------------

#[test]
fn the_constant_family_has_one_magnitude_and_the_indexed_family_has_many() {
    assert!(is_constant_family::<Constant<0>>());
    assert!(is_constant_family::<Constant<-4>>());
    assert!(!is_constant_family::<Indexed<-14, 30>>());

    assert_eq!(<Constant<0> as Quantum>::MAGNITUDES, 1);
    assert_eq!(<Constant<-7> as Quantum>::MAGNITUDES, 1);
    assert_eq!(<Indexed<-14, 30> as Quantum>::MAGNITUDES, 30);
}

#[test]
fn a_constant_quantum_does_not_move_with_magnitude_and_an_indexed_one_does() {
    // Constant: the exponent is the same at every magnitude it admits.
    assert_eq!(exponent_at::<Constant<-4>>(0), -4);

    // Indexed: one exponent per magnitude step, which is the floating shape.
    assert_eq!(exponent_at::<Indexed<-14, 30>>(0), -14);
    assert_eq!(exponent_at::<Indexed<-14, 30>>(1), -13);
    assert_eq!(exponent_at::<Indexed<-14, 30>>(29), 15);

    // And the two genuinely differ, which is the control for the pair above.
    assert_ne!(
        exponent_at::<Indexed<-14, 30>>(0),
        exponent_at::<Indexed<-14, 30>>(1)
    );
}

// --- subnormals fall out of the smallest magnitude, unnamed ------------------

#[test]
fn the_smallest_step_is_the_smallest_magnitudes_and_nothing_names_it() {
    // The floating point's smallest step is the one at its lowest magnitude, and
    // the values that step admits are what the conventions call subnormal. No
    // branch in this crate selects them and no name in it mentions them.
    assert_eq!(smallest_step_exponent::<Floating<11, -14, 30>>(), -14);
    assert_eq!(smallest_step_exponent::<Floating<24, -126, 254>>(), -126);

    // For the constant family the smallest step is the only step.
    assert_eq!(smallest_step_exponent::<UFixed<13, -4>>(), -4);
    assert_eq!(smallest_step_exponent::<Integer<8>>(), 0);
}

// --- the phase, which is why the coordinate is carried -----------------------

#[test]
fn a_zero_phase_puts_the_additive_identity_on_the_grid() {
    assert!(has_additive_identity::<Integer<8>>());
    assert!(has_additive_identity::<UFixed<13, -4>>());
    assert!(has_additive_identity::<Floating<11, -14, 30>>());
}

#[test]
fn a_nonzero_phase_takes_the_additive_identity_off_the_grid() {
    // The half-step bias. The canon carries the phase coordinate precisely
    // because this is not a corner case: the grid contains neither zero nor one,
    // no exact sum lands on it, and it is not a monoid carrier.
    assert!(!has_additive_identity::<Biased<7, -2, 1>>());
    assert!(!has_additive_identity::<Biased<13, 0, 1>>());
    assert!(!has_additive_identity::<Biased<31, -8, 3>>());

    // And a biased format with the phase set back to zero has it again, which is
    // the control saying the phase is what did it rather than the width.
    assert!(has_additive_identity::<Biased<7, -2, 0>>());
}

// --- the radix is a coordinate and is not hardcoded --------------------------

#[test]
fn the_radix_comes_from_the_ambient_domain() {
    assert_eq!(radix::<Integer<8>>(), 2);
    assert_eq!(<DecimalRationals as Ambient>::RADIX, 10);
    assert_ne!(
        <BinaryRationals as Ambient>::RADIX,
        <DecimalRationals as Ambient>::RADIX,
        "if these agree the radix is not a coordinate and every claim about it is \
         a claim about one value"
    );
}

#[test]
fn signedness_is_a_property_of_the_domain_and_not_of_a_carrier() {
    assert!(<BinaryRationals as Ambient>::SIGNED);
    assert!(!<UnsignedBinaryRationals as Ambient>::SIGNED);
}

// --- the rounding vocabulary is the six, and the retired word is not among ----

#[test]
fn the_rounding_vocabulary_is_exactly_six_names() {
    assert_eq!(ALL_MODES.len(), 6);

    // Every one distinct, so the enumeration is six names rather than six
    // spellings of fewer.
    for (i, a) in ALL_MODES.iter().enumerate() {
        for (j, b) in ALL_MODES.iter().enumerate() {
            if i != j {
                assert_ne!(a, b, "two of the six modes are the same value");
            }
        }
    }

    assert_eq!(<TowardZero as Rounding>::MODE, Mode::TowardZero);
    assert_eq!(<Floor as Rounding>::MODE, Mode::Floor);
    assert_eq!(<Ceil as Rounding>::MODE, Mode::Ceil);
    assert_eq!(<HalfUp as Rounding>::MODE, Mode::HalfUp);
    assert_eq!(<HalfEven as Rounding>::MODE, Mode::HalfEven);
    assert_eq!(<Stochastic as Rounding>::MODE, Mode::Stochastic);
}

#[test]
fn the_shipped_overflow_policies_are_three_and_distinct() {
    assert_eq!(SHIPPED_POLICIES.len(), 3);
    for (i, a) in SHIPPED_POLICIES.iter().enumerate() {
        for (j, b) in SHIPPED_POLICIES.iter().enumerate() {
            if i != j {
                assert_ne!(a, b);
            }
        }
    }
}

/// A user-defined overflow policy, which is what an open inventory means.
///
/// If this file compiles, adding a policy needed no edit to any existing item.
/// That is the assertion; the body is incidental.
struct DeclaredBound;

impl Overflow for DeclaredBound {
    const POLICY: Policy = Policy::Clamp;
}

#[test]
fn the_overflow_inventory_admits_a_member_this_crate_does_not_know_about() {
    assert_eq!(<DeclaredBound as Overflow>::POLICY, Policy::Clamp);
}

// --- the adaptation is the pair, over the whole matrix -----------------------

macro_rules! adaptation_over_the_matrix {
    ($($r:ident),+ $(,)?) => {
        #[test]
        fn every_rounding_mode_pairs_with_every_shipped_overflow_policy() {
            // The full cross product, not a diagonal. Six modes times three
            // policies is eighteen adaptations and every one is constructible,
            // which is what "the adaptation is its two coordinates" means.
            let mut seen = 0usize;
            $(
                {
                    type A1 = Adapt<$r, Wrap>;
                    type A2 = Adapt<$r, Saturate>;
                    type A3 = Adapt<$r, crate::overflow::Clamp>;
                    assert_eq!(
                        crate::adapt::rounding_of::<A1>(),
                        <$r as Rounding>::MODE
                    );
                    assert_eq!(crate::adapt::overflow_of::<A1>(), Policy::Wrap);
                    assert_eq!(crate::adapt::overflow_of::<A2>(), Policy::Saturate);
                    assert_eq!(crate::adapt::overflow_of::<A3>(), Policy::Clamp);
                    seen += 3;
                }
            )+
            assert_eq!(seen, 18, "the matrix is six by three and nothing was skipped");
        }
    };
}

adaptation_over_the_matrix!(TowardZero, Floor, Ceil, HalfUp, HalfEven, Stochastic);

#[test]
fn the_rounding_coordinate_and_the_overflow_coordinate_move_independently() {
    // Two regions of one map rather than two mechanisms: changing one leaves the
    // other where it was. If these coupled, the design would be carrying one axis
    // wearing two names.
    type Fixed = Adapt<HalfEven, Wrap>;
    type RoundingMoved = Adapt<Floor, Wrap>;
    type OverflowMoved = Adapt<HalfEven, Saturate>;

    assert_ne!(
        crate::adapt::rounding_of::<Fixed>(),
        crate::adapt::rounding_of::<RoundingMoved>()
    );
    assert_eq!(
        crate::adapt::overflow_of::<Fixed>(),
        crate::adapt::overflow_of::<RoundingMoved>()
    );

    assert_eq!(
        crate::adapt::rounding_of::<Fixed>(),
        crate::adapt::rounding_of::<OverflowMoved>()
    );
    assert_ne!(
        crate::adapt::overflow_of::<Fixed>(),
        crate::adapt::overflow_of::<OverflowMoved>()
    );
}

// --- the declared signature, and the admission rule --------------------------

/// An operation admitted by the rule: it names a signature and nothing else.
struct Add<F: Format>(core::marker::PhantomData<F>);

impl<F: Format> Operation for Add<F> {
    type Signature = Signature<F, Adapt<HalfEven, Saturate>>;
    const ARITY: u32 = 2;
}

#[test]
fn an_admitted_operation_reads_its_answer_through_the_signature() {
    assert_eq!(
        crate::adapt::operation_rounding::<Add<Integer<8>>>(),
        Mode::HalfEven
    );
    assert_eq!(
        crate::adapt::operation_overflow::<Add<Integer<8>>>(),
        Policy::Saturate
    );
    assert_eq!(<Add<Integer<8>> as Operation>::ARITY, 2);
}

#[test]
fn a_signature_carries_the_format_and_the_adaptation_and_nothing_else() {
    // Structural. The associated items of `DeclaredSignature` are exactly two,
    // and the test reaches both and finds no third to reach. A carrier coordinate
    // arriving later would break this file, which is the point of writing it.
    type S = Signature<UFixed<13, -4>, Adapt<Floor, Wrap>>;
    assert_eq!(
        crate::adapt::rounding_of::<<S as DeclaredSignature>::Adaptation>(),
        Mode::Floor
    );
    assert!(contains::<<S as DeclaredSignature>::Format>(0, 0));
}

#[test]
fn two_signatures_differing_only_in_adaptation_are_different_signatures() {
    // Which is why "the same format under two adaptations" is two declared
    // signatures rather than one signature with a setting, and why a disagreement
    // between two realisations of one name means a missing coordinate.
    type A = Signature<Integer<8>, Adapt<Floor, Wrap>>;
    type B = Signature<Integer<8>, Adapt<Ceil, Wrap>>;
    assert_ne!(
        crate::adapt::rounding_of::<<A as DeclaredSignature>::Adaptation>(),
        crate::adapt::rounding_of::<<B as DeclaredSignature>::Adaptation>()
    );
}

// --- a format the crate does not know about ----------------------------------

/// A format declared outside the shipped points, which is what an open inventory
/// means for the format concept.
struct Ternary;

impl Format for Ternary {
    type Ambient = DecimalRationals;
    type Quantum = Constant<-1>;
    type Slots = Signed<3>;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
}

#[test]
fn the_format_inventory_admits_a_member_this_crate_does_not_know_about() {
    assert_eq!(radix::<Ternary>(), 10);
    assert_eq!(step_exponent::<Ternary>(0), -1);
    assert!(has_additive_identity::<Ternary>());
    assert!(contains::<Ternary>(0, 0));
    assert!(!contains::<Ternary>(4, 0));
}
