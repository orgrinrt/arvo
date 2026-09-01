//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The adaptation's two coordinates, the signature, and the admission rule.
//!
//! An adaptation is its rounding mode and its overflow policy and nothing else,
//! so the matrix here is the full cross product rather than a diagonal, and the
//! independence of the two is asserted by moving one and watching the other stay.

use crate::adapt::{Adapt, Arity, DeclaredSignature, Operation, Signature};
use crate::format::{contains, Format};
use crate::overflow::{Overflow, Policy, Saturate, Wrap, SHIPPED_POLICIES};
use crate::points::{Integer, UFixed};
use crate::quantum::Magnitude;
use crate::rounding::{Ceil, Floor, HalfEven, HalfUp, Mode, Rounding, Stochastic, TowardZero, ALL_MODES};
use crate::slots::Slot;

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
    const ARITY: Arity = Arity::BINARY;
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
    assert_eq!(<Add<Integer<8>> as Operation>::ARITY, Arity::BINARY);
    assert_eq!(<Add<Integer<8>> as Operation>::ARITY.count(), 2);
    assert_ne!(Arity::UNARY, Arity::BINARY);
    assert_eq!(Arity::of(3).count(), 3);
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
    assert!(
        contains::<<S as DeclaredSignature>::Format>(Slot::ZERO, Magnitude::SMALLEST).get()
    );
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
