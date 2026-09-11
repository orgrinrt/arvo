//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the composed operation computes over the pairs the sweeps reach.
//!
//! Totality walks every admitted width of both slot families and feeds each
//! signature the edges of its range and the ends of the coordinate, where no
//! operand is a member. Where the true sum decides the answer, the answer is the
//! true sum's, which is the claim the saturated narrowing of the operand sum
//! makes and the one it would break if it moved a value that mattered.
//!
//! The rest are laws over small formats and every phase the shared list names:
//! the operation commutes, it is exact where the exact position is a member, the
//! identity's slot is neutral where the format has one, and the rounding axis
//! moves the answer exactly where the phase is not whole.

use notko::Maybe;

use super::{
    DITHERS,
    Ratio,
    Sums,
    Window,
    at_every_signature,
    coordinates,
    edges,
    member_count,
    members,
    signatures,
};
use crate::adapt::{Adapt, DeclaredSignature, Signature, overflow_of};
use crate::addition::{add, sum_position};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::Dither;
use crate::format::{Format, cancelling_slot, has_additive_identity};
use crate::overflow::{Policy, SHIPPED_POLICIES, Saturate, Wrap};
use crate::points::{Biased, Integer, UFixed};
use crate::quantum::{Constant, Magnitude};
use crate::rounding::{ALL_MODES, Floor, Mode};
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::dispatch::{self, PerFormat, PerSignature};
use crate::tests::grid::Grid;

// --- totality ------------------------------------------------------------------

/// Every fed pair at one signature: the answer is admitted, and where the true
/// sum decides it, it is the true sum's. Answers how many pairs it checked.
struct Total;

impl PerSignature for Total {
    type Out = u64;

    fn run<S: DeclaredSignature>(&self) -> u64 {
        let min = <<S::Format as Format>::Slots as Slots>::MIN;
        let max = <<S::Format as Format>::Slots as Slots>::MAX;
        let (lo, hi) = (min.index() as i128, max.index() as i128);
        let phase = Ratio::phase_of::<S::Format>();
        let policy = overflow_of::<S::Adaptation>();
        let mut checked = 0u64;
        for a in edges(min, max) {
            for b in edges(min, max) {
                for dither in DITHERS {
                    let got = add::<S>(a, b, dither);
                    assert!(
                        got.is_within(min, max).get(),
                        "{a:?} + {b:?} adapted to {got:?}, outside [{min:?}, {max:?}]"
                    );
                    checked += 1;
                    // The true position, in the wide domain, before any rounding.
                    let wide = (a.index() as i128) + (b.index() as i128) + phase.floor();
                    let got = got.index() as i128;
                    let both_members = a.is_within(min, max).get() && b.is_within(min, max).get();
                    match policy {
                        // Rounding returns the slot below the position or the one
                        // above it, so once both are past an end, the end is the
                        // answer whichever the mode picks.
                        Policy::Saturate | Policy::Clamp => {
                            if phase.is_whole() {
                                assert_eq!(got, wide.clamp(lo, hi), "{a:?} + {b:?}");
                            } else if wide + 1 < lo {
                                assert_eq!(got, lo, "{a:?} + {b:?}");
                            } else if wide >= hi {
                                assert_eq!(got, hi, "{a:?} + {b:?}");
                            }
                        },
                        // For members the wide sum never left the coordinate, so
                        // the reduction is the true one.
                        Policy::Wrap => {
                            if phase.is_whole() && both_members {
                                assert_eq!(got, lo + (wide - lo).rem_euclid(hi - lo + 1));
                            }
                        },
                    }
                }
            }
        }
        checked
    }
}

/// Every signature at one format, counted.
#[derive(Default)]
struct Walk {
    cells: usize,
    pairs: u64,
}

impl PerFormat for Walk {
    fn run<F: Format>(&mut self) {
        self.pairs += at_every_signature::<F, Total>(&Total);
        self.cells += signatures();
    }
}

#[test]
fn addition_is_total_over_every_admitted_width_mode_and_policy() {
    let mut walk = Walk::default();
    dispatch::every_width(&mut walk);
    assert_eq!(walk.cells, 62 * 2 * signatures());
    assert_eq!(
        walk.pairs,
        walk.cells as u64 * 10 * 10 * DITHERS.len() as u64
    );
}

#[test]
fn addition_is_total_at_the_widest_ranges_under_a_fractional_phase() {
    // The widest ranges are where the obligation's margin is smallest, and a
    // fractional phase is where the rounding can step one slot past the sum.
    let mut walk = Walk::default();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<62>, 1, 3>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<62>, -1, 2>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Unsigned<62>, -1, 3>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Unsigned<62>, 1, 2>>();
    assert_eq!(walk.cells, 4 * signatures());
    assert_eq!(
        walk.pairs,
        walk.cells as u64 * 10 * 10 * DITHERS.len() as u64
    );
}

// --- commutativity -------------------------------------------------------------

/// Both orders of every member pair and every edge pair, at one signature.
struct Commutes;

impl PerSignature for Commutes {
    type Out = u64;

    fn run<S: DeclaredSignature>(&self) -> u64 {
        let min = <<S::Format as Format>::Slots as Slots>::MIN;
        let max = <<S::Format as Format>::Slots as Slots>::MAX;
        let fed = || members::<S::Format>().chain(edges(min, max));
        let mut checked = 0u64;
        for a in fed() {
            for b in fed() {
                assert_eq!(
                    sum_position::<S::Format>(a, b),
                    sum_position::<S::Format>(b, a)
                );
                for dither in DITHERS {
                    assert_eq!(
                        add::<S>(a, b, dither),
                        add::<S>(b, a, dither),
                        "{a:?}, {b:?}"
                    );
                    checked += 1;
                }
            }
        }
        checked
    }
}

/// Commutativity at one format, with the count it owes.
#[derive(Default)]
struct CommutesAt {
    checked: u64,
    owed:    u64,
}

impl PerFormat for CommutesAt {
    fn run<F: Format>(&mut self) {
        self.checked += at_every_signature::<F, Commutes>(&Commutes);
        let fed = (member_count::<F>() + 10) as u64;
        self.owed += signatures() as u64 * fed * fed * DITHERS.len() as u64;
    }
}

#[test]
fn addition_commutes_over_every_pair_mode_policy_and_phase() {
    let mut walk = CommutesAt::default();
    every_phase!(walk; Signed<4>);
    every_phase!(walk; Unsigned<4>);
    every_phase!(walk; Window<-3, 2>);
    walk.run::<Biased<4, -2, 1>>();
    assert_eq!(walk.checked, walk.owed);
}

// --- exactness and the identity ------------------------------------------------

/// Where the exact position is a member, the answer is that member. Answers how
/// many such pairs it met.
struct Exactness;

impl PerSignature for Exactness {
    type Out = u64;

    fn run<S: DeclaredSignature>(&self) -> u64 {
        let min = <<S::Format as Format>::Slots as Slots>::MIN;
        let max = <<S::Format as Format>::Slots as Slots>::MAX;
        let mut exact = 0u64;
        for a in members::<S::Format>() {
            for b in members::<S::Format>() {
                let e = sum_position::<S::Format>(a, b);
                if e.is_on_grid().get() && e.slot().is_within(min, max).get() {
                    for dither in DITHERS {
                        assert_eq!(add::<S>(a, b, dither), e.slot(), "{a:?} + {b:?}");
                    }
                    exact += 1;
                }
            }
        }
        exact
    }
}

/// Exactness at one format, sorted by whether its phase is whole.
#[derive(Default)]
struct ExactAt {
    whole:      u32,
    fractional: u32,
}

impl PerFormat for ExactAt {
    fn run<F: Format>(&mut self) {
        let exact = at_every_signature::<F, Exactness>(&Exactness);
        if F::PHASE.is_whole_multiple().get() {
            assert!(exact > 0, "{:?} met no exact pair", coordinates::<F>());
            self.whole += 1;
        } else {
            // At a fractional phase no sum of two members is on the grid, so the
            // law has nothing to say there and the count says so.
            assert_eq!(exact, 0, "{:?}", coordinates::<F>());
            self.fractional += 1;
        }
    }
}

#[test]
fn addition_is_exact_where_the_exact_position_is_a_member() {
    let mut walk = ExactAt::default();
    every_phase!(walk; Signed<4>);
    every_phase!(walk; Unsigned<4>);
    every_phase!(walk; Window<-3, 2>);
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, 6, 2>>();
    assert_eq!(walk.whole, 3 * 3 + 1);
    assert_eq!(walk.fractional, 3 * 3);
}

/// Adding the identity's slot to every member, in both orders, at one signature.
struct Neutral(Slot);

impl PerSignature for Neutral {
    type Out = u64;

    fn run<S: DeclaredSignature>(&self) -> u64 {
        let mut checked = 0u64;
        for a in members::<S::Format>() {
            for dither in DITHERS {
                assert_eq!(add::<S>(a, self.0, dither), a, "{a:?} + identity");
                assert_eq!(add::<S>(self.0, a, dither), a, "identity + {a:?}");
                checked += 1;
            }
        }
        checked
    }
}

/// The identity law at one format that has an identity.
fn the_identity_is_neutral<F: Format>() -> u64 {
    assert!(has_additive_identity::<F>().get());
    let Maybe::Is(identity) = cancelling_slot::<F>(Magnitude::SMALLEST) else {
        panic!(
            "{:?} has an identity and no cancelling slot",
            coordinates::<F>()
        );
    };
    at_every_signature::<F, Neutral>(&Neutral(identity))
}

#[test]
fn the_identitys_slot_is_neutral_wherever_the_format_has_one() {
    let checked = the_identity_is_neutral::<Integer<4>>()
        + the_identity_is_neutral::<UFixed<4, 0>>()
        + the_identity_is_neutral::<Biased<4, 0, 2>>()
        + the_identity_is_neutral::<Grid<BinaryRationals, Constant<0>, Signed<4>, -2, 1>>()
        + the_identity_is_neutral::<Grid<DecimalRationals, Constant<3>, Signed<4>, 3, 1>>()
        + the_identity_is_neutral::<Grid<BinaryRationals, Constant<0>, Window<-3, 2>, 1, 1>>();
    let owed = signatures() as u64 * DITHERS.len() as u64 * (16 + 16 + 16 + 16 + 16 + 6);
    assert_eq!(checked, owed);
}

#[test]
fn no_sum_of_two_members_lands_on_a_half_step_grid() {
    type F = Biased<4, 0, 1>;
    assert!(!has_additive_identity::<F>().get());
    for a in members::<F>() {
        for b in members::<F>() {
            assert!(!sum_position::<F>(a, b).is_on_grid().get(), "{a:?} + {b:?}");
        }
    }
}

#[test]
fn a_phase_cancelled_outside_the_range_has_no_identity_and_wrapping_finds_a_neutral_slot() {
    // Phase nine over four signed bits: the value zero would sit at slot -9,
    // which is not in the range, so the format has no identity.
    type F = Grid<BinaryRationals, Constant<0>, Signed<4>, 9, 1>;
    assert!(!has_additive_identity::<F>().get());
    // Under saturation no slot is neutral.
    for z in members::<F>() {
        let moves_something = members::<F>()
            .any(|a| add::<Signature<F, Adapt<Floor, Saturate>>>(a, z, Dither::UNUSED) != a);
        assert!(moves_something, "{z:?} is neutral under saturation");
    }
    // Under wrapping slot 7 is, because 7 + 9 is the span. The identity law is
    // about the value zero, and a neutral slot of the wrapped operation is a
    // different thing the format can have without it.
    for a in members::<F>() {
        assert_eq!(
            add::<Signature<F, Adapt<Floor, Wrap>>>(a, Slot::at(7), Dither::UNUSED),
            a
        );
    }
}

// --- the rounding axis ---------------------------------------------------------

/// Every mode's table against the floor's, at one format and every policy.
#[derive(Default)]
struct Axis {
    whole:      u32,
    fractional: u32,
}

impl PerFormat for Axis {
    fn run<F: Format>(&mut self) {
        let name = coordinates::<F>();
        let whole = F::PHASE.is_whole_multiple().get();
        for policy in SHIPPED_POLICIES {
            let floor = dispatch::at::<F, Sums>(Mode::Floor, policy, &Sums(Dither::UNUSED));
            if whole {
                for mode in ALL_MODES {
                    for dither in DITHERS {
                        let other = dispatch::at::<F, Sums>(mode, policy, &Sums(dither));
                        assert!(
                            other == floor,
                            "{mode:?} moved a sum at {name:?}, {policy:?}"
                        );
                    }
                }
            } else {
                let ceil = dispatch::at::<F, Sums>(Mode::Ceil, policy, &Sums(Dither::UNUSED));
                assert!(
                    ceil != floor,
                    "floor and ceil agree at {name:?}, {policy:?}"
                );
            }
        }
        if whole {
            self.whole += 1;
        } else {
            self.fractional += 1;
        }
    }
}

#[test]
fn the_rounding_mode_moves_the_sum_exactly_where_the_phase_is_not_whole() {
    let mut walk = Axis::default();
    every_phase!(walk; Signed<4>);
    every_phase!(walk; Unsigned<4>);
    every_phase!(walk; Window<-3, 2>);
    walk.run::<Integer<4>>();
    walk.run::<Biased<4, 0, 1>>();
    assert_eq!(walk.whole, 3 * 3 + 1);
    assert_eq!(walk.fractional, 3 * 3 + 1);
}
