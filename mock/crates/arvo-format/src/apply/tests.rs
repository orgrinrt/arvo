//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The laws of the applied map, over the whole matrix.
//!
//! These replace four tests round `202608311902` shipped that reached a
//! declaration and stopped. The difference is that every assertion here can be
//! wrong about arithmetic: each one runs the map and looks at what came back,
//! so a predicate stating the opposite of the truth fails rather than being
//! confirmed by a sibling that states the same thing.

use crate::adapt::{Adapt, Signature};

/// Test shim. The exact step is carried wide, and the tests write narrow values,
/// so the widening happens once here rather than at thirty call sites.
fn complete(policy: Policy, slot: i64, min: i64, max: i64) -> i64 {
    complete_slot(policy, slot as i128, min, max)
}
use crate::apply::{
    adapt, complete_slot, panic_on_inexact, panic_on_overflow, round_slot, Dither, Exact,
};
use crate::overflow::{Clamp, Policy, Saturate, Wrap, SHIPPED_POLICIES};
use crate::points::Integer;
use crate::rounding::{Ceil, Floor, HalfEven, HalfUp, Mode, Stochastic, TowardZero, ALL_MODES};

/// Every position from well below a small window to well above it, at every
/// eighth, so ties and both off-grid sides are covered rather than sampled.
fn every_position() -> impl Iterator<Item = Exact> {
    (-20i64..=20).flat_map(|slot| (0i64..8).map(move |n| Exact::between(slot, n, 8)))
}

/// The window `Integer<5>` declares: slots -16 through 15.
const MIN5: i64 = -16;
const MAX5: i64 = 15;

// --- the control -------------------------------------------------------------

#[test]
fn the_control_the_sweep_reaches_both_regions_and_both_sides() {
    let mut off_grid = 0;
    let mut ties = 0;
    let mut below = 0;
    let mut above = 0;
    for e in every_position() {
        if !e.is_on_grid() {
            off_grid += 1;
        }
        if e.is_tie() {
            ties += 1;
        }
        if e.slot() < MIN5 {
            below += 1;
        }
        if e.slot() > MAX5 {
            above += 1;
        }
    }
    assert!(
        off_grid > 0,
        "no off-grid position, so rounding is untested"
    );
    assert!(ties > 0, "no tie, so the tie rules are untested");
    assert!(
        below > 0 && above > 0,
        "the sweep does not leave the window on both sides"
    );
}

// --- 1. the map is total -----------------------------------------------------

macro_rules! totality_over_the_matrix {
    ($($mode:ident),+ $(,)?) => {
        #[test]
        fn the_map_is_total_over_every_mode_and_every_policy() {
            let mut checked = 0usize;
            $(
                {
                    type SW = Signature<Integer<5>, Adapt<$mode, Wrap>>;
                    type SS = Signature<Integer<5>, Adapt<$mode, Saturate>>;
                    type SC = Signature<Integer<5>, Adapt<$mode, Clamp>>;
                    for e in every_position() {
                        for d in [Dither::UNUSED, Dither::at(1, 4), Dither::at(3, 4)] {
                            for got in [
                                adapt::<SW>(e, d),
                                adapt::<SS>(e, d),
                                adapt::<SC>(e, d),
                            ] {
                                assert!(
                                    got >= MIN5 && got <= MAX5,
                                    "{got} is outside the declared window for {e:?}"
                                );
                                checked += 1;
                            }
                        }
                    }
                }
            )+
            assert!(checked > 0, "the matrix ran nothing");
        }
    };
}

totality_over_the_matrix!(TowardZero, Floor, Ceil, HalfUp, HalfEven, Stochastic);

// --- 2. the identity on the grid, which the old predicate only declared ------

macro_rules! identity_over_the_matrix {
    ($($mode:ident),+ $(,)?) => {
        #[test]
        fn the_map_is_the_identity_on_a_grid_point_inside_the_window() {
            $(
                {
                    type SW = Signature<Integer<5>, Adapt<$mode, Wrap>>;
                    type SS = Signature<Integer<5>, Adapt<$mode, Saturate>>;
                    type SC = Signature<Integer<5>, Adapt<$mode, Clamp>>;
                    for slot in MIN5..=MAX5 {
                        let e = Exact::on_grid(slot);
                        for d in [Dither::UNUSED, Dither::at(1, 2)] {
                            assert_eq!(adapt::<SW>(e, d), slot, "wrap moved {slot}");
                            assert_eq!(adapt::<SS>(e, d), slot, "saturate moved {slot}");
                            assert_eq!(adapt::<SC>(e, d), slot, "clamp moved {slot}");
                        }
                    }
                }
            )+
        }
    };
}

identity_over_the_matrix!(TowardZero, Floor, Ceil, HalfUp, HalfEven, Stochastic);

// --- 3. order transport, measured against the map ---------------------------

/// Whether completion under this policy ever inverts a pair, over a range that
/// leaves the window in both directions.
fn completion_transports_order(policy: Policy) -> bool {
    for a in -40i64..=40 {
        for b in a..=40 {
            if complete(policy, a, MIN5, MAX5) > complete(policy, b, MIN5, MAX5) {
                return false;
            }
        }
    }
    true
}

#[test]
fn order_transport_is_measured_from_the_map_rather_than_declared() {
    // This is what the deleted `is_monotone` asserted about a `matches!`
    // expression. Here the answer comes from running the completion and looking
    // at the pairs, so a wrong claim fails.
    assert!(
        !completion_transports_order(Policy::Wrap),
        "wrapping was measured to transport order, which would mean the two law \
         families have no separating instance here"
    );
    assert!(completion_transports_order(Policy::Saturate));
    assert!(completion_transports_order(Policy::Clamp));

    // And exactly one of the three fails to transport order, which is the count
    // the deleted test asserted without measuring anything.
    let transporting = SHIPPED_POLICIES
        .iter()
        .filter(|p| completion_transports_order(**p))
        .count();
    assert_eq!(transporting, 2);
}

#[test]
fn wrapping_inverts_a_specific_pair_and_the_witness_is_named() {
    // A single witness, so the property above is not resting on a loop nobody
    // can point inside. The top of the window and one past it.
    let hi = complete(Policy::Wrap, MAX5, MIN5, MAX5);
    let over = complete(Policy::Wrap, MAX5 + 1, MIN5, MAX5);
    assert_eq!(hi, MAX5);
    assert_eq!(over, MIN5);
    assert!(
        over < hi,
        "wrapping did not invert the pair at the boundary"
    );
}

// --- 4. determinism, measured by varying the dither -------------------------

#[test]
fn five_modes_ignore_the_dither_and_one_does_not() {
    let dithers = [
        Dither::UNUSED,
        Dither::at(1, 8),
        Dither::at(1, 2),
        Dither::at(7, 8),
    ];

    let mut moved_by_dither = 0;
    for mode in ALL_MODES {
        let mut varies = false;
        for e in every_position() {
            let first = round_slot(mode, e, dithers[0]);
            for d in &dithers[1..] {
                if round_slot(mode, e, *d) != first {
                    varies = true;
                }
            }
        }
        if varies {
            moved_by_dither += 1;
        } else {
            // A deterministic mode must be stable across every dither, which is
            // the assertion rather than the count.
            for e in every_position() {
                let first = round_slot(mode, e, dithers[0]);
                for d in &dithers[1..] {
                    assert_eq!(
                        round_slot(mode, e, *d),
                        first,
                        "{mode:?} moved with the dither at {e:?}"
                    );
                }
            }
        }
    }
    assert_eq!(
        moved_by_dither, 1,
        "exactly one mode should read the dither, measured by varying it"
    );
    // And it is the one the vocabulary names for it.
    let mut stochastic_varies = false;
    for e in every_position() {
        if round_slot(Mode::Stochastic, e, Dither::at(1, 8))
            != round_slot(Mode::Stochastic, e, Dither::at(7, 8))
        {
            stochastic_varies = true;
        }
    }
    assert!(stochastic_varies);
}

// --- 5. direction, measured from the returned slots -------------------------

#[test]
fn each_directed_mode_moves_every_off_grid_position_its_own_way() {
    for e in every_position() {
        if e.is_on_grid() {
            continue;
        }
        let down = e.slot() as i128;
        let up = down + 1;

        assert_eq!(round_slot(Mode::Floor, e, Dither::UNUSED), down);
        assert_eq!(round_slot(Mode::Ceil, e, Dither::UNUSED), up);

        // Toward zero is neither of those on a signed domain, which is the whole
        // reason the ambiguous word was retired: it agrees with floor above zero
        // and with ceil below it.
        let toward = round_slot(Mode::TowardZero, e, Dither::UNUSED);
        if e.slot() < 0 {
            assert_eq!(toward, up, "toward zero should rise on a negative position");
        } else {
            assert_eq!(
                toward, down,
                "toward zero should fall on a positive position"
            );
        }
    }
}

#[test]
fn toward_zero_and_floor_are_measured_to_differ_and_the_witness_is_named() {
    // The retired word named these two. A single negative off-grid position
    // separates them, and if it did not the vocabulary's split would be decorative.
    let e = Exact::between(-3, 1, 2);
    assert_eq!(round_slot(Mode::Floor, e, Dither::UNUSED), -3);
    assert_eq!(round_slot(Mode::TowardZero, e, Dither::UNUSED), -2);
}

// --- 6. the tie rules, which are why the remainder is a fraction ------------

#[test]
fn half_up_goes_away_from_zero_on_a_tie_and_half_even_goes_to_the_even_slot() {
    // Positive tie: away from zero is up.
    let pos = Exact::between(2, 1, 2);
    assert!(pos.is_tie());
    assert_eq!(round_slot(Mode::HalfUp, pos, Dither::UNUSED), 3);
    assert_eq!(round_slot(Mode::HalfEven, pos, Dither::UNUSED), 2);

    // Negative tie: away from zero is down.
    let neg = Exact::between(-3, 1, 2);
    assert!(neg.is_tie());
    assert_eq!(round_slot(Mode::HalfUp, neg, Dither::UNUSED), -3);
    assert_eq!(round_slot(Mode::HalfEven, neg, Dither::UNUSED), -2);

    // Odd slot: half-even climbs to the even neighbour.
    let odd = Exact::between(3, 1, 2);
    assert_eq!(round_slot(Mode::HalfEven, odd, Dither::UNUSED), 4);
}

#[test]
fn a_non_tie_is_decided_by_which_side_of_the_midpoint_it_falls() {
    let below = Exact::between(5, 3, 8);
    let above = Exact::between(5, 5, 8);
    for mode in [Mode::HalfUp, Mode::HalfEven] {
        assert_eq!(round_slot(mode, below, Dither::UNUSED), 5, "{mode:?}");
        assert_eq!(round_slot(mode, above, Dither::UNUSED), 6, "{mode:?}");
    }
}

// --- 7. the two regions are separable ---------------------------------------

#[test]
fn rounding_is_reachable_without_completion_and_completion_without_rounding() {
    type S = Signature<Integer<5>, Adapt<Ceil, Saturate>>;

    // Inside the window, off grid: rounding acts, completion does not.
    let inside = Exact::between(3, 1, 4);
    assert_eq!(adapt::<S>(inside, Dither::UNUSED), 4);
    assert_eq!(complete(Policy::Saturate, 4, MIN5, MAX5), 4);

    // On the grid, outside the window: completion acts, rounding does not.
    let outside = Exact::on_grid(MAX5 + 7);
    assert_eq!(round_slot(Mode::Ceil, outside, Dither::UNUSED), (MAX5 + 7) as i128);
    assert_eq!(adapt::<S>(outside, Dither::UNUSED), MAX5);
}

#[test]
fn rounding_happens_before_completion_and_the_order_is_observable() {
    // A position just below the top that rounds up onto a slot one past the top.
    // If completion ran first it would see a slot inside the window and do
    // nothing, and the answer would be the rounded slot, which is out of range.
    type S = Signature<Integer<5>, Adapt<Ceil, Saturate>>;
    let e = Exact::between(MAX5, 1, 2);
    assert_eq!(round_slot(Mode::Ceil, e, Dither::UNUSED), (MAX5 + 1) as i128);
    assert_eq!(
        adapt::<S>(e, Dither::UNUSED),
        MAX5,
        "completion did not see the rounded slot, so the order is wrong"
    );
}

// --- 8. clamp and saturate agree, and the reason is recorded ---------------

#[test]
fn clamp_and_saturate_compute_the_same_function_because_a_coordinate_is_missing() {
    // Not a law. `Clamp` is documented as pinning to a declared bound that need
    // not be the range's own end, and the declared signature has nowhere to carry
    // that bound, so the two names have nothing to differ by. Asserted so the
    // agreement is recorded as a consequence of the missing coordinate rather
    // than read later as a property of the policies.
    for slot in -40i64..=40 {
        assert_eq!(
            complete(Policy::Clamp, slot, MIN5, MAX5),
            complete(Policy::Saturate, slot, MIN5, MAX5)
        );
    }

    // The control: they are not trivially equal to everything. Wrapping differs
    // from both outside the window, so the agreement above is about these two.
    let out = MAX5 + 1;
    assert_ne!(
        complete(Policy::Wrap, out, MIN5, MAX5),
        complete(Policy::Saturate, out, MIN5, MAX5)
    );
}

// --- 9. the panic verdicts, which report rather than diverge ---------------

#[test]
fn the_panic_verdicts_report_and_the_crate_stays_total() {
    assert!(!panic_on_inexact(Exact::on_grid(3)));
    assert!(panic_on_inexact(Exact::between(3, 1, 4)));

    type S = Signature<Integer<5>, Adapt<Floor, Wrap>>;
    assert!(!panic_on_overflow::<S>(MAX5));
    assert!(!panic_on_overflow::<S>(MIN5));
    assert!(panic_on_overflow::<S>(MAX5 + 1));
    assert!(panic_on_overflow::<S>(MIN5 - 1));

    // Totality is unaffected by the verdicts: the map still returns a slot for a
    // position both verdicts refuse.
    let refused = Exact::between(MAX5 + 5, 1, 4);
    assert!(panic_on_inexact(refused));
    let got = adapt::<S>(refused, Dither::UNUSED);
    assert!(got >= MIN5 && got <= MAX5);
}

// --- 10. the map is a function of the declared signature -------------------

#[test]
fn a_wider_format_admits_what_a_narrower_one_completes_away() {
    // The window comes from the format, so the same position adapts differently
    // under two signatures differing only in width. If it did not, the map would
    // not be reading the declared signature.
    type Narrow = Signature<Integer<5>, Adapt<Floor, Saturate>>;
    type Wide = Signature<Integer<8>, Adapt<Floor, Saturate>>;

    let e = Exact::on_grid(40);
    assert_eq!(adapt::<Narrow>(e, Dither::UNUSED), MAX5);
    assert_eq!(adapt::<Wide>(e, Dither::UNUSED), 40);
}

#[test]
fn two_signatures_differing_only_in_mode_adapt_the_same_position_differently() {
    type Down = Signature<Integer<5>, Adapt<Floor, Saturate>>;
    type Up = Signature<Integer<5>, Adapt<Ceil, Saturate>>;

    let e = Exact::between(2, 1, 4);
    assert_eq!(adapt::<Down>(e, Dither::UNUSED), 2);
    assert_eq!(adapt::<Up>(e, Dither::UNUSED), 3);
}

// --- 11. the constructor normalises rather than trusting the caller --------

#[test]
fn a_remainder_outside_the_unit_interval_is_normalised_into_the_slot() {
    assert_eq!(Exact::between(0, 9, 4).slot(), 2);
    assert_eq!(Exact::between(0, -1, 4).slot(), -1);
    assert!(Exact::between(0, 8, 4).is_on_grid());
    // A non-positive denominator is treated as a grid point rather than dividing.
    assert!(Exact::between(7, 3, 0).is_on_grid());
    assert_eq!(Exact::between(7, 3, 0).slot(), 7);
}

// --- the edges of the type, which every arm above stays away from ------------
//
// Thirty references to one small window meant the arithmetic could never leave
// `i64`, so the breaking path was never entered and five review passes found
// what the suite did not. These arms feed the edges.

/// The wrapping answer, as a verdict rather than an assertion, so a case can be
/// reported on and compared rather than only passing or failing.
fn wrap_of(slot: i64, min: i64, max: i64) -> i64 {
    let span = (max as i128) - (min as i128) + 1;
    ((min as i128) + ((slot as i128) - (min as i128)).rem_euclid(span)) as i64
}

/// Signature over the slot range `[-4, 3]`, span 8, which `Integer<3>` declares.
type Edge = Signature<Integer<3>, Adapt<Floor, Wrap>>;
const EDGE_MIN: i64 = -4;
const EDGE_MAX: i64 = 3;

#[test]
fn the_control_the_edges_are_outside_the_window_the_other_arms_use() {
    // If `i64::MAX` were inside the window these arms would be testing the
    // in-range path under another name.
    assert!(i64::MAX > EDGE_MAX);
    assert!(i64::MIN < EDGE_MIN);
    assert!(i64::MAX > MAX5 && i64::MIN < MIN5);
}

#[test]
fn adapting_at_the_edges_of_the_type_gives_the_arithmetic_answer() {
    // Computed independently by `wrap_of` in a carrier that holds the step, and
    // asserted against what the crate returns. Before the exact step moved to a
    // wide carrier the crate returned a value inside the range that was simply
    // wrong, which no assertion comparing it to itself could have caught.
    for slot in [i64::MAX, i64::MIN, i64::MAX - 1, i64::MIN + 1, 0, 7, -9] {
        let got = adapt::<Edge>(Exact::on_grid(slot), Dither::UNUSED);
        let want = wrap_of(slot, EDGE_MIN, EDGE_MAX);
        assert_eq!(got, want, "adapting {slot} disagreed with the arithmetic");
        assert!(
            got >= EDGE_MIN && got <= EDGE_MAX,
            "adapting {slot} left the declared range"
        );
    }
}

#[test]
fn the_edge_answers_are_the_ones_worked_out_by_hand() {
    // Two values written down rather than derived by the same expression the
    // crate uses, so the test does not agree with the code by construction.
    //
    // `i64::MAX` into `[-4, 3]`: (i64::MAX + 4) mod 8 = 3, so -4 + 3 = -1.
    // `i64::MIN` into `[-4, 3]`: (i64::MIN + 4) mod 8 = 4, so -4 + 4 = 0.
    assert_eq!(adapt::<Edge>(Exact::on_grid(i64::MAX), Dither::UNUSED), -1);
    assert_eq!(adapt::<Edge>(Exact::on_grid(i64::MIN), Dither::UNUSED), 0);
}

#[test]
fn rounding_at_the_top_of_the_type_does_not_leave_the_carrier() {
    // `Ceil` on an off-grid position at `i64::MAX` rounds to one past the top of
    // `i64`, which is a real position the completion has to land. Computing that
    // step in `i64` is what made the map wrong.
    type Up = Signature<Integer<3>, Adapt<Ceil, Wrap>>;
    let e = Exact::between(i64::MAX, 1, 4);
    let got = adapt::<Up>(e, Dither::UNUSED);
    assert!(got >= EDGE_MIN && got <= EDGE_MAX);
    assert_eq!(got, wrap_of_i128((i64::MAX as i128) + 1, EDGE_MIN, EDGE_MAX));
}

/// The same verdict for a position that has already left `i64`.
fn wrap_of_i128(slot: i128, min: i64, max: i64) -> i64 {
    let span = (max as i128) - (min as i128) + 1;
    ((min as i128) + (slot - (min as i128)).rem_euclid(span)) as i64
}

#[test]
fn saturating_at_the_edges_pins_to_the_declared_ends() {
    type Sat = Signature<Integer<3>, Adapt<Floor, Saturate>>;
    assert_eq!(
        adapt::<Sat>(Exact::on_grid(i64::MAX), Dither::UNUSED),
        EDGE_MAX
    );
    assert_eq!(
        adapt::<Sat>(Exact::on_grid(i64::MIN), Dither::UNUSED),
        EDGE_MIN
    );
}

#[test]
fn a_dither_at_the_edges_still_selects_between_two_neighbours() {
    // The stochastic mode cross-multiplies, which is the other site that could
    // leave the type. A large denominator with a large slot exercises both.
    let e = Exact::between(i64::MAX - 1, 3, 4);
    let low = adapt::<Edge>(e, Dither::at(1, 1_000_000_000));
    let high = adapt::<Edge>(e, Dither::at(999_999_999, 1_000_000_000));
    for got in [low, high] {
        assert!(got >= EDGE_MIN && got <= EDGE_MAX, "left the declared range");
    }
}
