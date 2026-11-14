//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Negation and subtraction against exact rational arithmetic and the applied
//! map's oracle.
//!
//! The position each tuple names is computed here in `Ratio`, from the signs and
//! the phase and nothing the step computes, and then handed to the apply tests'
//! oracle, which rounds and completes it from the canon's formulas. The step's
//! adapted answer and its overflow verdict have to equal the oracle's at every
//! tuple, every mode, every shipped policy and, for the one mode that reads it,
//! four dithers.
//!
//! Exhaustive over the members at every width from one to eight, both slot
//! families, at every phase of the shared list. Past eight bits the members
//! are too many to walk, so every width to sixty-four is fed the ten edges of
//! its range and of the index, members checked against the oracle and every
//! other slot checked to land inside the range.

use core::marker::PhantomData;
use core::ops::RangeInclusive;

use super::steps::{Difference, Negation, Step, Sum, copies, operand_count, operands, signed};
use super::{Ratio, coordinates, edges, signatures};
use crate::adapt::{DeclaredSignature, overflow_of, rounding_of};
use crate::addition::{difference_position, sub, subtraction_reach};
use crate::ambient::BinaryRationals;
use crate::apply::tests::the_oracle::{Point, Wide, answer};
use crate::apply::{Dither, Exact, Fraction, panic_on_overflow};
use crate::format::Format;
use crate::overflow::SHIPPED_POLICIES;
use crate::quantum::Constant;
use crate::rounding::{ALL_MODES, Mode};
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::symmetry::Reach;
use crate::tests::dispatch::{self, PerFormat, PerSignature};
use crate::tests::grid::Grid;
use crate::width::Bool;

/// The quarters of a slot the dither is drawn at: all four for the mode that
/// reads it, and one for every other, which ignores it.
fn quarters(mode: Mode) -> RangeInclusive<i64> {
    if mode == Mode::Stochastic { 0 ..= 3 } else { 0 ..= 0 }
}

/// The position `T` names for the tuple, in exact rationals: the signed sum of
/// the slots and the step's copies of the phase.
fn exact_position<T: Step, F: Format>(a: Slot, b: Slot) -> Ratio {
    Ratio::whole(signed::<T, Slot>((a, b), Slot::index))
        .plus(Ratio::phase_of::<F>().times(Ratio::whole(copies::<T>())))
}

/// A ratio of slots as the oracle's point: the slot below it and the remainder.
fn point(at: Ratio) -> Point {
    let rest = Ratio::of(at.num.rem_euclid(at.den), at.den);
    Point {
        whole: Wide::of(at.floor()),
        num:   rest.num,
        den:   rest.den,
    }
}

/// What a run of comparisons found: how many were made and how many disagreed.
#[derive(Clone, Copy, Default)]
struct Tally {
    checked: u64,
    wrong:   u64,
}

impl Tally {
    /// Both counts added.
    fn plus(self, other: Self) -> Self {
        Self {
            checked: self.checked + other.checked,
            wrong:   self.wrong + other.wrong,
        }
    }
}

/// One tuple at one signature, against the oracle, at each dither the mode
/// reads. Strict stops at the first disagreement and names it; otherwise the
/// disagreements are counted, which is what a planted step is run for.
fn agrees<T: Step, S: DeclaredSignature>(a: Slot, b: Slot, strict: bool) -> Tally {
    let min = <<S::Format as Format>::Slots as Slots>::MIN.index();
    let max = <<S::Format as Format>::Slots as Slots>::MAX.index();
    let mode = rounding_of::<S::Adaptation>();
    let policy = overflow_of::<S::Adaptation>();
    let at = point(exact_position::<T, S::Format>(a, b));
    let mut tally = Tally::default();
    for quarter in quarters(mode) {
        let dither = Dither::at(Fraction::of(quarter, 4));
        let want = answer(mode, policy, at, (quarter, 4), min, max);
        let got = (
            T::adapted::<S>(a, b, dither).index(),
            panic_on_overflow::<S>(T::position::<S::Format>(a, b), dither).get(),
        );
        if strict {
            assert_eq!(
                got,
                want,
                "{:?} at {a:?}, {b:?} under {mode:?}, {policy:?}, dither {quarter}/4, {:?}",
                T::NEGATED,
                coordinates::<S::Format>()
            );
        }
        tally.checked += 1;
        tally.wrong += u64::from(got != want);
    }
    tally
}

/// Every member tuple at one signature, against the oracle.
struct EveryMember<T> {
    step:   PhantomData<T>,
    strict: bool,
}

impl<T: Step> PerSignature for EveryMember<T> {
    type Out = Tally;

    #[inline(never)]
    fn run<S: DeclaredSignature>(&self) -> Tally {
        operands::<T, S::Format>()
            .map(|(a, b)| agrees::<T, S>(a, b, self.strict))
            .fold(Tally::default(), Tally::plus)
    }
}

/// The edge tuples at one signature: members against the oracle, every other
/// slot inside the range. Answers how many tuples it fed.
struct AtTheEdges<T>(PhantomData<T>);

impl<T: Step> PerSignature for AtTheEdges<T> {
    type Out = u64;

    #[inline(never)]
    fn run<S: DeclaredSignature>(&self) -> u64 {
        let min = <<S::Format as Format>::Slots as Slots>::MIN;
        let max = <<S::Format as Format>::Slots as Slots>::MAX;
        let unary = T::NEGATED.len() == 1;
        let mut fed = 0u64;
        // By position in the list rather than by value: at a narrow width the
        // edges repeat slots, and a unary step takes each edge once.
        for (i, a) in edges(min, max).into_iter().enumerate() {
            for (j, b) in edges(min, max).into_iter().enumerate() {
                if unary && j != i {
                    continue;
                }
                fed += 1;
                if a.is_within(min, max).get() && b.is_within(min, max).get() {
                    agrees::<T, S>(a, b, true);
                    continue;
                }
                for quarter in 0 ..= 3 {
                    let got = T::adapted::<S>(a, b, Dither::at(Fraction::of(quarter, 4)));
                    assert!(
                        got.is_within(min, max).get(),
                        "{:?} at {a:?}, {b:?} adapted to {got:?}, outside [{min:?}, {max:?}]",
                        T::NEGATED
                    );
                }
            }
        }
        fed
    }
}

/// A sweep of `T` over formats, every signature at each, counted against what
/// the formats owe.
#[derive(Default)]
struct Sweep<T> {
    step:    PhantomData<T>,
    edges:   bool,
    planted: bool,
    formats: u64,
    checked: u64,
    wrong:   u64,
    owed:    u64,
}

impl<T: Step> PerFormat for Sweep<T> {
    #[inline(never)]
    fn run<F: Format>(&mut self) {
        self.formats += 1;
        for mode in ALL_MODES {
            for policy in SHIPPED_POLICIES {
                if self.edges {
                    self.checked +=
                        dispatch::at::<F, AtTheEdges<T>>(mode, policy, &AtTheEdges(PhantomData));
                    self.owed += 10u64.pow(T::NEGATED.len() as u32);
                } else {
                    let every = EveryMember {
                        step:   PhantomData::<T>,
                        strict: !self.planted,
                    };
                    let tally = dispatch::at::<F, EveryMember<T>>(mode, policy, &every);
                    self.checked += tally.checked;
                    self.wrong += tally.wrong;
                    self.owed += operand_count::<T, F>() * (*quarters(mode).end() as u64 + 1);
                }
            }
        }
    }
}

/// Negation a copy of the phase short, `-a - phase`, under negation's signs.
///
/// The control for the sweep: the oracle reads the signs, so it expects two
/// copies, and the sweep has to report this step wherever the phase is not
/// zero and nowhere it is.
#[derive(Default)]
struct OneCopyShort;

impl Step for OneCopyShort {
    const NEGATED: &'static [Bool] = &[Bool::TRUE];

    fn position<F: Format>(a: Slot, _: Slot) -> Exact {
        difference_position::<F>(Slot::ZERO, a)
    }

    fn adapted<S: DeclaredSignature>(a: Slot, _: Slot, dither: Dither) -> Slot {
        sub::<S>(Slot::ZERO, a, dither)
    }

    fn reach<S: DeclaredSignature>() -> Reach {
        subtraction_reach::<S>()
    }
}

/// Every phase of the shared list over both slot families at every width from
/// one to eight.
fn every_narrow_format<P: PerFormat>(walk: &mut P) {
    macro_rules! widths {
        ($($w:literal),+) => {
            $(
                every_phase!(walk; Signed<$w>);
                every_phase!(walk; Unsigned<$w>);
            )+
        };
    }
    widths!(1, 2, 3, 4, 5, 6, 7, 8);
}

/// `T` against the oracle at every member tuple of every narrow format.
fn exhaustive<T: Step + Default>() {
    let mut walk = Sweep::<T>::default();
    every_narrow_format(&mut walk);
    assert_eq!(
        walk.formats,
        8 * 2 * super::phases() as u64,
        "{:?}",
        T::NEGATED
    );
    assert_eq!(walk.checked, walk.owed, "{:?}", T::NEGATED);
    assert_eq!(walk.wrong, 0, "{:?}", T::NEGATED);
}

#[test]
fn the_sweep_reports_a_negation_a_copy_of_the_phase_short() {
    // At phase zero the planted step is negation, so the sweep has to find
    // nothing there; at every other phase of the list it has to find it.
    let mut at_zero = Sweep::<OneCopyShort> {
        planted: true,
        ..Sweep::default()
    };
    at_zero.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, 0, 1>>();
    at_zero.run::<Grid<BinaryRationals, Constant<0>, Unsigned<4>, 0, 1>>();
    assert_eq!(at_zero.checked, at_zero.owed);
    assert_eq!(at_zero.wrong, 0);

    let mut walk = Sweep::<OneCopyShort> {
        planted: true,
        ..Sweep::default()
    };
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, -1, 1>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, 5, 1>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, 1, 2>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, 1, 3>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Unsigned<4>, -2, 3>>();
    assert_eq!(walk.checked, walk.owed);
    assert!(walk.wrong > 0, "the sweep missed a step a copy short");
}

/// `T` at the edges of every admitted width of both families.
fn at_every_width<T: Step + Default>() {
    let mut walk = Sweep::<T> {
        edges: true,
        ..Sweep::default()
    };
    dispatch::every_width(&mut walk);
    assert_eq!(walk.formats, 64 * 2, "{:?}", T::NEGATED);
    assert_eq!(walk.checked, walk.owed, "{:?}", T::NEGATED);
    assert_eq!(
        walk.owed,
        walk.formats * signatures() as u64 * 10u64.pow(T::NEGATED.len() as u32),
        "{:?}",
        T::NEGATED
    );
}

#[test]
fn negation_answers_as_the_oracle_does_at_every_member_up_to_eight_bits() {
    exhaustive::<Negation>();
}

#[test]
fn subtraction_answers_as_the_oracle_does_at_every_member_up_to_eight_bits() {
    exhaustive::<Difference>();
}

#[test]
fn negation_is_total_and_exact_at_the_edges_of_every_admitted_width() {
    at_every_width::<Negation>();
}

#[test]
fn subtraction_is_total_and_exact_at_the_edges_of_every_admitted_width() {
    at_every_width::<Difference>();
}

/// `T` over phases whose fractional part reduces to a denominator of `2^63`,
/// the largest a phase can declare: every member at a small range, and the edges
/// at both 62-bit ranges, the widest whose sums the rational oracle forms over a
/// denominator of `2^63`, under every mode and shipped policy.
fn at_the_least_denominator<T: Step + Default>() {
    let mut walk = Sweep::<T>::default();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, 1, { i64::MIN }>>();
    walk.run::<Grid<BinaryRationals, Constant<-2>, Unsigned<4>, -3, { i64::MIN }>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<3>, { i64::MAX }, { i64::MIN }>>();
    let mut edges = Sweep::<T> {
        edges: true,
        ..Sweep::default()
    };
    edges.run::<Grid<BinaryRationals, Constant<0>, Signed<62>, 1, { i64::MIN }>>();
    edges.run::<Grid<BinaryRationals, Constant<0>, Unsigned<62>, { i64::MAX }, { i64::MIN }>>();
    assert_eq!((walk.formats, edges.formats), (3, 2), "{:?}", T::NEGATED);
    assert_eq!(walk.checked, walk.owed, "{:?}", T::NEGATED);
    assert_eq!(edges.checked, edges.owed, "{:?}", T::NEGATED);
    assert!(walk.owed > 0 && edges.owed > 0);
}

#[test]
fn every_step_answers_as_the_oracle_does_over_a_phase_reduced_to_two_to_the_63() {
    at_the_least_denominator::<Sum>();
    at_the_least_denominator::<Negation>();
    at_the_least_denominator::<Difference>();
}
