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
    NearTheBottom,
    NearTheTop,
    Ratio,
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
use crate::overflow::{Policy, Saturate, Wrap};
use crate::points::{Biased, Integer, UFixed};
use crate::quantum::{Constant, Magnitude};
use crate::rounding::Floor;
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::dispatch::{self, PerFormat, PerSignature};
use crate::tests::grid::Grid;

// --- totality ------------------------------------------------------------------

/// A sum of slot indices held in two halves, so it is exact past either end of
/// the index.
///
/// The operation computes in the index itself and saturates once where an
/// operand lies outside the range, so the oracle it is checked against has to
/// be wider than the index or it would share the saturation it is checking.
/// The high half is signed and the low half holds the bottom sixty-four bits,
/// so ordering the pair orders the sums.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Wide {
    high: i128,
    low:  u128,
}

impl Wide {
    /// How many bits the low half holds. A function rather than an item
    /// constant, for the reason `the_ratio_coordinate` gives.
    fn low_bits() -> u32 {
        64
    }

    /// The exact sum of the given indices.
    fn sum(terms: &[i128]) -> Self {
        let mask = u128::from(u64::MAX);
        let (mut high, mut low) = (0i128, 0u128);
        for &term in terms {
            high += term >> Self::low_bits();
            low += term as u128 & mask;
        }
        high += (low >> Self::low_bits()) as i128;
        Self {
            high,
            low: low & mask,
        }
    }

    /// One index, in the same halves.
    fn of(index: i128) -> Self {
        Self::sum(&[index])
    }

    /// The sum as an index, where the index holds it.
    fn index(self) -> Maybe<i128> {
        if i128::from(i64::MIN) <= self.high && self.high <= i128::from(i64::MAX) {
            Maybe::Is((self.high << Self::low_bits()) | self.low as i128)
        } else {
            Maybe::Isnt
        }
    }
}

#[test]
fn the_wide_sum_is_exact_past_both_ends_of_the_index() {
    // The ends and their neighbours, where a sum in the index itself overflows.
    assert_eq!(Wide::sum(&[i128::MAX, 1]).index(), Maybe::Isnt);
    assert_eq!(Wide::sum(&[i128::MIN, -1]).index(), Maybe::Isnt);
    assert_eq!(Wide::sum(&[i128::MAX, 1, -1]).index(), Maybe::Is(i128::MAX));
    assert_eq!(Wide::sum(&[i128::MIN, -1, 1]).index(), Maybe::Is(i128::MIN));
    assert_eq!(Wide::sum(&[i128::MIN, i128::MAX]).index(), Maybe::Is(-1));
    assert!(Wide::sum(&[i128::MAX, i128::MAX]) > Wide::of(i128::MAX));
    assert!(Wide::sum(&[i128::MIN, i128::MIN]) < Wide::of(i128::MIN));
    assert!(Wide::sum(&[i128::MIN, i128::MIN]) < Wide::sum(&[i128::MIN, i128::MIN, 1]));
    // Every small sum agrees with the index's own arithmetic, signs mixed.
    for a in -300i128 .. 300 {
        for b in [-(1i128 << 64), -1, 0, 1, 1 << 64, i128::MAX / 4] {
            assert_eq!(Wide::sum(&[a, b]).index(), Maybe::Is(a + b), "{a} + {b}");
            assert_eq!(
                Wide::sum(&[a, b]).cmp(&Wide::of(a)),
                (a + b).cmp(&a),
                "{a} + {b}"
            );
        }
    }
}

/// Every fed pair at one signature: the answer is admitted, and where the true
/// sum decides it, it is the true sum's. Answers how many pairs it checked.
struct Total;

impl PerSignature for Total {
    type Out = u64;

    fn run<S: DeclaredSignature>(&self) -> u64 {
        let min = <<S::Format as Format>::Slots as Slots>::MIN;
        let max = <<S::Format as Format>::Slots as Slots>::MAX;
        let (lo, hi) = (min.index(), max.index());
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
                    // The true position, exact, before any rounding.
                    let wide = Wide::sum(&[a.index(), b.index(), phase.floor()]);
                    let got = got.index();
                    let both_members = a.is_within(min, max).get() && b.is_within(min, max).get();
                    match policy {
                        // Rounding returns the slot below the position or the one
                        // above it, so once both are past an end, the end is the
                        // answer whichever the mode picks.
                        Policy::Saturate | Policy::Clamp => {
                            if phase.is_whole() {
                                let clamped = if wide < Wide::of(lo) {
                                    lo
                                } else if wide > Wide::of(hi) {
                                    hi
                                } else {
                                    wide.index().expect("a sum inside the range")
                                };
                                assert_eq!(got, clamped, "{a:?} + {b:?}");
                            } else if Wide::sum(&[a.index(), b.index(), phase.floor(), 1])
                                < Wide::of(lo)
                            {
                                assert_eq!(got, lo, "{a:?} + {b:?}");
                            } else if wide >= Wide::of(hi) {
                                assert_eq!(got, hi, "{a:?} + {b:?}");
                            }
                        },
                        // For members the sum never left the index, so the
                        // reduction is the true one.
                        Policy::Wrap => {
                            if phase.is_whole() && both_members {
                                let wide = wide.index().expect("a sum of two members");
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
    assert_eq!(walk.cells, 64 * 2 * signatures());
    assert_eq!(
        walk.pairs,
        walk.cells as u64 * 10 * 10 * DITHERS.len() as u64
    );
}

#[test]
fn addition_is_total_at_the_widest_ranges_under_a_fractional_phase() {
    // The widest ranges are where the obligation's margin is smallest, and a
    // fractional phase is where the rounding can step one slot past the sum. The
    // two ranges near the top of the index are where the sum of two members is
    // closest to leaving it.
    let mut walk = Walk::default();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<64>, 1, 3>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<64>, -1, 2>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Unsigned<64>, -1, 3>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Unsigned<64>, 1, 2>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, NearTheTop, 1, 2>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, NearTheBottom, 1, 2>>();
    assert_eq!(walk.cells, 6 * signatures());
    assert_eq!(
        walk.pairs,
        walk.cells as u64 * 10 * 10 * DITHERS.len() as u64
    );
}

#[test]
fn a_sum_past_the_index_saturates_once_so_a_third_term_cannot_bring_it_back() {
    // Two non-members at the top of the index and a phase whose whole part is
    // negative. Saturating step by step would pin the first sum at `i128::MAX`
    // and then take one off it; the design saturates the sum once, and the true
    // sum is far past the top, so the position is the top itself.
    type Down = Grid<BinaryRationals, Constant<0>, Signed<4>, -1, 2>;
    let top = Slot::at(i128::MAX);
    assert_eq!(sum_position::<Down>(top, top).slot(), top);

    // The mirror at the bottom, with a whole part that is positive.
    type Up = Grid<BinaryRationals, Constant<0>, Signed<4>, 3, 2>;
    let bottom = Slot::at(i128::MIN);
    assert_eq!(sum_position::<Up>(bottom, bottom).slot(), bottom);

    // Where it shows. Saturation and clamping pin both to the range's end, so the
    // difference is visible only under wrapping. `i128::MAX` is fifteen modulo
    // sixteen, so into `[-8, 7]` it lands on -8 + (15 + 8) mod 16, which is -1;
    // the position one below it would have landed on -2.
    assert_eq!(
        add::<Signature<Down, Adapt<Floor, Wrap>>>(top, top, Dither::UNUSED),
        Slot::at(-1)
    );
    // And `i128::MIN` is zero modulo sixteen, so it lands on -8 + 8, which is 0;
    // one above it would have landed on 1.
    assert_eq!(
        add::<Signature<Up, Adapt<Floor, Wrap>>>(bottom, bottom, Dither::UNUSED),
        Slot::ZERO
    );

    // The control: members sum exactly, with no saturation in reach.
    assert_eq!(
        sum_position::<Down>(Slot::at(7), Slot::at(7)).slot(),
        Slot::at(13)
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
//
// In `the_rounding_axis`, beside this file.
