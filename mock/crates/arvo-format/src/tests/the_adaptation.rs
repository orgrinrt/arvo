//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The adaptation's two coordinates, the signature, and the admission rule.
//!
//! An adaptation is its rounding mode and its overflow policy and nothing else,
//! so the matrix here is the full cross product rather than a diagonal, and the
//! independence of the two is asserted by moving one and watching the other stay.
//!
//! The open-inventory arm runs its foreign policy through the map rather than
//! reading the constant back. Reading it back says a constant holds what its own
//! definition set, which is a law between two declarations and constrains neither.

use crate::adapt::{Adapt, Arity, DeclaredSignature, Operation, Signature};
use crate::apply::{adapt, Dither, Exact};
use crate::format::{contains, Format};
use crate::overflow::{Clamp, Overflow, Policy, Saturate, Wrap, SHIPPED_POLICIES};
use crate::points::{Integer, UFixed};
use crate::quantum::Magnitude;
use crate::rounding::{
    Ceil, Floor, HalfEven, HalfUp, Mode, Rounding, Stochastic, TowardZero, ALL_MODES,
};
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
struct DeclaredBound;

impl Overflow for DeclaredBound {
    const POLICY: Policy = Policy::Clamp;
}

/// A second one, deciding the other way.
///
/// One foreign member establishes that the map does not refuse an unknown type.
/// Two deciding differently establish that it reads what the declaration says,
/// which is the claim the name makes and the thing one member cannot separate.
struct DeclaredRing;

impl Overflow for DeclaredRing {
    const POLICY: Policy = Policy::Wrap;
}

/// The window `Integer<5>` declares: slots -16 through 15, so a span of 32.
const MIN5: Slot = Slot::at(-16);
const MAX5: Slot = Slot::at(15);

#[test]
fn the_overflow_inventory_admits_a_member_this_crate_does_not_know_about() {
    // The whole of this arm used to be the declared constant read back, four
    // lines under the impl that set it, with a doc saying the body was incidental.
    // What the name claims is that a policy declared outside this crate reaches
    // the crate's machinery, so the arms run it through the map.
    type Outside = Signature<Integer<5>, Adapt<Floor, DeclaredBound>>;
    type Ring = Signature<Integer<5>, Adapt<Floor, DeclaredRing>>;
    type Shipped = Signature<Integer<5>, Adapt<Floor, Clamp>>;

    // Positions well outside the window on both sides, so the completion region
    // answers and the rounding region has nothing to do.
    let above = Exact::on_grid(Slot::at(MAX5.index() + 25));
    let below = Exact::on_grid(Slot::at(MIN5.index() - 25));

    for (foreign, shipped) in [
        (
            adapt::<Outside>(above, Dither::UNUSED),
            adapt::<Shipped>(above, Dither::UNUSED),
        ),
        (
            adapt::<Outside>(below, Dither::UNUSED),
            adapt::<Shipped>(below, Dither::UNUSED),
        ),
    ] {
        assert_eq!(
            foreign, shipped,
            "a foreign policy naming the same value did not land where the shipped one does"
        );
    }
    assert_eq!(adapt::<Outside>(above, Dither::UNUSED), MAX5);
    assert_eq!(adapt::<Outside>(below, Dither::UNUSED), MIN5);

    // The two foreign members decide differently, which is what says the map
    // reads the declaration rather than answering one way for anything it does
    // not recognise.
    assert_ne!(
        adapt::<Outside>(above, Dither::UNUSED),
        adapt::<Ring>(above, Dither::UNUSED),
        "two foreign policies naming different values adapted the same way"
    );

    // Worked out by hand rather than by the expression the crate uses. The span
    // is 32, so 40 lands at ((40 + 16) mod 32) - 16, which is 24 - 16 = 8, and
    // -41 lands at ((-41 + 16) mod 32) - 16, which is 7 - 16 = -9.
    assert_eq!(adapt::<Ring>(above, Dither::UNUSED), Slot::at(8));
    assert_eq!(adapt::<Ring>(below, Dither::UNUSED), Slot::at(-9));

    // And the foreign path is total, like every other: both answers are in the
    // declared window rather than merely different from each other.
    for got in [
        adapt::<Outside>(above, Dither::UNUSED),
        adapt::<Outside>(below, Dither::UNUSED),
        adapt::<Ring>(above, Dither::UNUSED),
        adapt::<Ring>(below, Dither::UNUSED),
    ] {
        assert!(
            got.is_within(MIN5, MAX5).get(),
            "{got:?} left the declared window under a foreign policy"
        );
    }

    // The coordinate reads back through the crate's own accessor, which is where
    // a signature carries it rather than where the impl wrote it.
    assert_eq!(
        crate::adapt::overflow_of::<Adapt<Floor, DeclaredBound>>(),
        Policy::Clamp
    );
    assert_eq!(
        crate::adapt::overflow_of::<Adapt<Floor, DeclaredRing>>(),
        Policy::Wrap
    );

    // The control: inside the window the completion does nothing, so both foreign
    // members agree there and the disagreement above is about the region the
    // policy governs rather than about the policy being read at all.
    let inside = Exact::on_grid(Slot::at(3));
    assert_eq!(adapt::<Outside>(inside, Dither::UNUSED), Slot::at(3));
    assert_eq!(adapt::<Ring>(inside, Dither::UNUSED), Slot::at(3));
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
                    type A3 = Adapt<$r, Clamp>;
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
    assert!(contains::<<S as DeclaredSignature>::Format>(Slot::ZERO, Magnitude::SMALLEST).get());
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
