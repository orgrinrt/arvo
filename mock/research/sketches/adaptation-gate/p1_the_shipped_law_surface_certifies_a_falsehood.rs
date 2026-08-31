//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Do the overflow and rounding law tests shipped in round 202608311902 have any
//! grip on the truth, or do they only check a declaration against itself?
//!
//! Run: `rustc --edition 2021 -O --test p1_the_shipped_law_surface_certifies_a_falsehood.rs -o /tmp/a1 && /tmp/a1 --test-threads=1`
//!
//! The claim under test is about my own work. `arvo-format` ships
//! `is_monotone`, `is_identity_inside_range`, `is_deterministic` and `is_directed`
//! as `matches!` expressions over an enumeration, and ships tests asserting what
//! those expressions match. Nothing anywhere applies a `Policy` or a `Mode` to a
//! value: the only non-test use of `Policy` in the whole tree is an array literal.
//!
//! So the question is whether that pair can catch a declaration that is false
//! about arithmetic, or only one that disagrees with its own test.
//!
//! What must fail, stated before the run: `the_control_the_behaviour_tied_test_has_grip`
//! builds a real wrapping and a real saturating map over a small range and checks
//! order transport against them. If that control passes under the backwards
//! declaration, this probe proves nothing and the diagnosis is wrong.

#![allow(dead_code)]

// --- the shipped shape, transcribed -----------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Policy {
    Wrap,
    Saturate,
    Clamp,
}

pub const SHIPPED_POLICIES: [Policy; 3] = [Policy::Wrap, Policy::Saturate, Policy::Clamp];

/// Verbatim from `arvo-format/src/overflow.rs`.
pub const fn is_monotone_as_shipped(policy: Policy) -> bool {
    !matches!(policy, Policy::Wrap)
}

/// Verbatim from `arvo-format/src/overflow.rs`.
pub const fn is_identity_inside_range_as_shipped(policy: Policy) -> bool {
    matches!(policy, Policy::Wrap | Policy::Saturate | Policy::Clamp)
}

// --- 1. the purest case: a predicate true for every value that exists --------

#[test]
fn the_identity_predicate_is_true_for_every_variant_that_exists() {
    // `is_identity_inside_range` names all three variants of a three-variant
    // enumeration, so it is the constant `true` wearing a `matches!`. The shipped
    // test asserts it holds for all three, which is asserting that `true` is true.
    for p in SHIPPED_POLICIES {
        assert!(
            is_identity_inside_range_as_shipped(p),
            "the transcription is wrong if this fails"
        );
    }
    // And there is no value of the type for which it is false.
    let exhaustive_over_the_type = [Policy::Wrap, Policy::Saturate, Policy::Clamp];
    assert_eq!(
        exhaustive_over_the_type.len(),
        SHIPPED_POLICIES.len(),
        "if these differ the enumeration has a variant the shipped array omits, \
         and the predicate would have a false case after all"
    );
    let falsifying = exhaustive_over_the_type
        .into_iter()
        .filter(|p| !is_identity_inside_range_as_shipped(*p))
        .count();
    assert_eq!(
        falsifying, 0,
        "no input falsifies it, so the shipped assertion cannot fail for any reason \
         other than somebody editing the predicate"
    );
}

// --- 2. the closed loop: the declaration can be inverted and stay green ------

/// The same predicate, stated backwards. This says wrapping transports order and
/// saturation does not, which is **false** about arithmetic in both directions.
pub const fn is_monotone_backwards(policy: Policy) -> bool {
    matches!(policy, Policy::Wrap)
}

#[test]
fn the_shipped_shape_of_test_passes_against_the_backwards_declaration() {
    // The shipped test asserts a count and three memberships. Rewrite those to
    // agree with the backwards declaration, exactly as somebody editing both would,
    // and the suite is green while the crate now claims something false.
    let monotone = SHIPPED_POLICIES
        .iter()
        .filter(|p| is_monotone_backwards(**p))
        .count();
    assert_eq!(monotone, 1);
    assert!(is_monotone_backwards(Policy::Wrap));
    assert!(!is_monotone_backwards(Policy::Saturate));
    assert!(!is_monotone_backwards(Policy::Clamp));

    // Green, and the statement it certifies is wrong. That is the defect: the test
    // constrains the declaration and nothing constrains the declaration against
    // what a policy does, because nothing does anything with a policy.
}

// --- 3. the control, and the repair ------------------------------------------
//
// A behaviour-tied test has grip because it reaches something the code does.
// Apply the policy to a value and ask whether order survives.

const LO: i32 = -4;
const HI: i32 = 3;

fn apply(policy: Policy, v: i32) -> i32 {
    match policy {
        Policy::Wrap => {
            let span = HI - LO + 1;
            let mut r = (v - LO) % span;
            if r < 0 {
                r += span;
            }
            r + LO
        }
        Policy::Saturate | Policy::Clamp => {
            if v < LO {
                LO
            } else if v > HI {
                HI
            } else {
                v
            }
        }
    }
}

/// Whether applying the policy preserves the order of every pair in a range wide
/// enough to leave the representable window.
fn transports_order(policy: Policy) -> bool {
    for a in -12i32..=12 {
        for b in -12i32..=12 {
            if a <= b && apply(policy, a) > apply(policy, b) {
                return false;
            }
        }
    }
    true
}

#[test]
fn the_control_the_behaviour_tied_test_has_grip() {
    // Measured against the applied map rather than declared. If the backwards
    // declaration were true, these would agree with it. They do not.
    assert!(
        !transports_order(Policy::Wrap),
        "wrapping was measured to transport order, so the applied map is wrong and \
         this probe establishes nothing"
    );
    assert!(transports_order(Policy::Saturate));
    assert!(transports_order(Policy::Clamp));

    // And this is what the shipped surface cannot do: it disagrees with the
    // backwards declaration on every one of the three, from measurement.
    for p in SHIPPED_POLICIES {
        assert_ne!(
            transports_order(p),
            is_monotone_backwards(p),
            "the measured answer and the backwards declaration agree at {p:?}, which \
             would mean the backwards declaration was not backwards"
        );
        assert_eq!(
            transports_order(p),
            is_monotone_as_shipped(p),
            "the measured answer disagrees with the shipped declaration at {p:?}"
        );
    }
}

#[test]
fn the_identity_claim_becomes_falsifiable_once_it_is_applied() {
    // The other predicate, tied to behaviour. Every policy is the identity on a
    // value already inside the window, and now that is a measurement over the
    // window rather than a `matches!` over the variants.
    for p in SHIPPED_POLICIES {
        for v in LO..=HI {
            assert_eq!(
                apply(p, v),
                v,
                "{p:?} changed {v}, which was already representable"
            );
        }
    }

    // The control: outside the window they do not agree, so the property above is
    // about the window rather than about the map being the identity everywhere.
    assert_ne!(apply(Policy::Wrap, HI + 1), HI + 1);
    assert_ne!(apply(Policy::Saturate, HI + 1), HI + 1);
}

// --- what this establishes ---------------------------------------------------
//
// The shipped predicates are declarations no code reads. Their tests check the
// declarations against themselves, so an inverted declaration stays green once its
// own test is updated to match, and the crate then certifies a false statement
// about arithmetic. Tying the same properties to an applied map makes both
// falsifiable, and under that tying the shipped declarations turn out to be
// correct, which is worth saying: the defect is that nothing checked them, not
// that they were wrong.
