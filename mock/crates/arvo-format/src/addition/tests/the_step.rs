//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The exact step, what its obligation refuses, and the reach it supplies.
//!
//! The step's claim is that it reads the phase and the slot range and never the
//! scale. The denotation test is where that claim can fail: its oracle computes
//! each member's value with the quantum multiplied in, adds the values, and reads
//! the sum back as a position, over exponents on both sides of zero and two
//! radices. A step that added the phase in value units, which is what a design
//! reading the scale would compute, is kept beside it and has to disagree, and it
//! has to agree wherever the quantum is one, which is what says the disagreement
//! is the scale and nothing else.
//!
//! The refusals are asked through `is_addable`, which does not force the
//! obligation, and each condition is pinned on both sides of its boundary. The
//! forcing side is the compile-fail cases.

use core::marker::PhantomData;

use super::steps::{
    Difference,
    Negation,
    Step,
    Sum,
    copies,
    operand_count,
    operands,
    sign,
    signed,
};
use super::{NearTheBottom, NearTheTop, Ratio, Window, coordinates};
use crate::adapt::{Adapt, Signature};
use crate::addition::{
    addition_is_associative,
    addition_reach,
    difference_position,
    is_addable,
    negation_position,
    negation_reach,
    subtraction_reach,
    sum_position,
};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Exact, Fraction};
use crate::format::{Format, radix};
use crate::overflow::Saturate;
use crate::points::{Biased, Floating, Integer, UFixed};
use crate::quantum::{Constant, Quantum};
use crate::rounding::Floor;
use crate::slots::{Signed, Slot, Unsigned};
use crate::symmetry::Reach;
use crate::tests::dispatch::PerFormat;
use crate::tests::grid::Grid;
use crate::width::Bool;

// --- the denotation ------------------------------------------------------------

/// The quantum at the one magnitude a constant-quantum format has.
fn quantum<F: Format>() -> Ratio {
    let r = radix::<F>().base() as i128;
    let e = <F::Quantum as Quantum>::BASE.power();
    let p = r.pow(e.unsigned_abs());
    if e >= 0 { Ratio::whole(p) } else { Ratio::of(1, p) }
}

/// The value of the member at `slot`: the phase at the quantum, plus the slot's
/// multiple of it.
fn value<F: Format>(slot: Slot) -> Ratio {
    let q = quantum::<F>();
    Ratio::phase_of::<F>()
        .times(q)
        .plus(Ratio::whole(slot.index()).times(q))
}

/// The denotation of step `T` checked at one format, with the value-units step
/// beside it.
#[derive(Default)]
struct Denotation<T> {
    step:                 PhantomData<T>,
    with_mutant:          bool,
    formats:              u32,
    tuples:               u64,
    owed:                 u64,
    mutant_wrong:         u64,
    mutant_wrong_at_unit: u64,
}

impl<T: Step> PerFormat for Denotation<T> {
    fn run<F: Format>(&mut self) {
        let q = quantum::<F>();
        let phase = Ratio::phase_of::<F>();
        self.formats += 1;
        self.owed += operand_count::<T, F>();
        for (a, b) in operands::<T, F>() {
            let got = T::position::<F>(a, b);
            // Back from value space: a value is `phase * q + p * q`, so the
            // position of a value `v` is `v / q - phase`.
            let values =
                T::NEGATED
                    .iter()
                    .zip([a, b])
                    .fold(Ratio::whole(0), |sum, (&negated, slot)| {
                        sum.plus(value::<F>(slot).times(Ratio::whole(sign(negated))))
                    });
            let want = values.over(q).minus(phase).position();
            assert_eq!(
                got,
                want,
                "{:?} at {a:?}, {b:?} at {:?}",
                T::NEGATED,
                coordinates::<F>()
            );
            self.tuples += 1;
            if self.with_mutant {
                // The copies of the phase added in value units, the scale left in.
                let slots = Ratio::whole(signed::<T, Slot>((a, b), Slot::index));
                let mutant: Exact = slots
                    .plus(phase.times(q).times(Ratio::whole(copies::<T>())))
                    .position();
                if mutant != want {
                    self.mutant_wrong += 1;
                    if q == Ratio::whole(1) {
                        self.mutant_wrong_at_unit += 1;
                    }
                }
            }
        }
    }
}

/// Every exponent from minus eight to eight at one ambient, slot range and phase.
macro_rules! every_exponent {
    ($walk:ident; $ambient:ty, $slots:ty, $pn:literal, $pd:literal) => {
        every_exponent!(@ $walk; $ambient, $slots, $pn, $pd;
            -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8);
    };
    (@ $walk:ident; $ambient:ty, $slots:ty, $pn:literal, $pd:literal; $($e:literal),+) => {
        $( $walk.run::<Grid<$ambient, Constant<{ $e }>, $slots, { $pn }, { $pd }>>(); )+
    };
}

/// The denotation's phases at one ambient and slot range: whole, a half, and
/// thirds with the sign on either half of the pair, and zero.
macro_rules! denotation_phases {
    ($walk:ident; $ambient:ty, $slots:ty) => {
        every_exponent!($walk; $ambient, $slots, 2, 1);
        every_exponent!($walk; $ambient, $slots, 1, 2);
        every_exponent!($walk; $ambient, $slots, -4, 3);
        every_exponent!($walk; $ambient, $slots, 1, -3);
        every_exponent!($walk; $ambient, $slots, 0, 1);
    };
}

/// The denotation of `T` over every exponent, phase, ambient and two slot ranges,
/// with the value-units step as its control.
fn the_step_is_the_signed_sum_of_the_values<T: Step + Default>() {
    let mut walk = Denotation::<T> {
        with_mutant: true,
        ..Denotation::default()
    };
    denotation_phases!(walk; BinaryRationals, Signed<3>);
    denotation_phases!(walk; BinaryRationals, Unsigned<3>);
    denotation_phases!(walk; DecimalRationals, Signed<3>);
    denotation_phases!(walk; DecimalRationals, Unsigned<3>);

    assert_eq!(walk.formats, 2 * 2 * 5 * 17, "{:?}", T::NEGATED);
    assert_eq!(
        walk.tuples,
        u64::from(walk.formats) * 8u64.pow(T::NEGATED.len() as u32),
        "{:?}",
        T::NEGATED
    );
    // The control: the value-units step is wrong somewhere, and nowhere the
    // quantum is one, so what separates it from the step is the scale.
    assert!(
        walk.mutant_wrong > 0,
        "the value-units step agreed everywhere for {:?}",
        T::NEGATED
    );
    assert_eq!(walk.mutant_wrong_at_unit, 0, "{:?}", T::NEGATED);
}

#[test]
fn the_exact_step_is_the_sum_of_the_values_read_back_as_a_position() {
    the_step_is_the_signed_sum_of_the_values::<Sum>();
}

#[test]
fn the_negation_step_is_the_negated_value_read_back_as_a_position() {
    the_step_is_the_signed_sum_of_the_values::<Negation>();
}

#[test]
fn the_difference_step_is_the_difference_of_the_values_read_back_as_a_position() {
    the_step_is_the_signed_sum_of_the_values::<Difference>();
}

/// The denotation of `T` at the largest denominator the remainder admits.
fn the_step_holds_at_the_largest_denominator<T: Step + Default>() {
    // A denominator of the least value with an odd numerator reduces to 2^63,
    // the largest a phase can declare, and with an even one to 2^62. The
    // oracle's arithmetic carries both in the wide domain throughout. At minus
    // one and minus two copies the reduced denominator divides it, which is the
    // admission argument for the signed steps, checked here at its edge.
    let mut walk = Denotation::<T>::default();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, 2, { i64::MIN }>>();
    walk.run::<Grid<BinaryRationals, Constant<-3>, Signed<4>, -2, { i64::MIN }>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, { i64::MIN }, { i64::MIN }>>();
    walk.run::<Grid<DecimalRationals, Constant<2>, Unsigned<4>, { i64::MAX - 1 }, { i64::MIN }>>();
    walk.run::<Grid<BinaryRationals, Constant<0>, Signed<4>, 1, { i64::MIN }>>();
    walk.run::<Grid<BinaryRationals, Constant<-3>, Unsigned<4>, -3, { i64::MIN }>>();
    walk.run::<Grid<DecimalRationals, Constant<1>, Signed<4>, { i64::MAX }, { i64::MIN }>>();
    assert_eq!(walk.formats, 7, "{:?}", T::NEGATED);
    assert_eq!(walk.tuples, walk.owed, "{:?}", T::NEGATED);
    assert_eq!(
        walk.tuples,
        7 * 16u64.pow(T::NEGATED.len() as u32),
        "{:?}",
        T::NEGATED
    );
}

#[test]
fn every_step_holds_at_the_largest_denominator_the_remainder_admits() {
    the_step_holds_at_the_largest_denominator::<Sum>();
    the_step_holds_at_the_largest_denominator::<Negation>();
    the_step_holds_at_the_largest_denominator::<Difference>();
}

// --- the refusals --------------------------------------------------------------

/// A constant-quantum binary format over `S` with the phase `PN / PD`.
type At<S, const PN: i64, const PD: i64> = Grid<BinaryRationals, Constant<0>, S, PN, PD>;

#[test]
fn the_obligation_admits_every_shipped_constant_quantum_point() {
    assert!(is_addable::<Integer<1>>().get());
    assert!(is_addable::<Integer<63>>().get());
    assert!(is_addable::<Integer<64>>().get());
    assert!(is_addable::<UFixed<63, -8>>().get());
    assert!(is_addable::<UFixed<64, -8>>().get());
    assert!(is_addable::<Biased<64, 3, 1>>().get());
    assert!(is_addable::<Biased<64, 0, -7>>().get());
}

#[test]
fn the_obligation_refuses_a_magnitude_indexed_quantum() {
    assert!(!is_addable::<Floating<4, -3, 4>>().get());
    assert!(!is_addable::<Floating<24, -149, 254>>().get());
}

#[test]
fn the_obligation_refuses_a_phase_that_does_not_denote_and_holds_every_one_that_does() {
    // No denominator at all, at any numerator.
    assert!(!is_addable::<At<Signed<8>, 1, 0>>().get());
    assert!(!is_addable::<At<Signed<8>, 0, 0>>().get());
    assert!(!is_addable::<At<Signed<8>, { i64::MIN }, 0>>().get());
    // The least denominator under an odd numerator reduces to 2^63, which the
    // remainder holds, so every denoting phase is admitted.
    assert!(is_addable::<At<Signed<8>, 1, { i64::MIN }>>().get());
    assert!(is_addable::<At<Signed<8>, -3, { i64::MIN }>>().get());
    assert!(is_addable::<At<Signed<8>, { i64::MAX }, { i64::MIN }>>().get());
    assert!(is_addable::<At<Signed<8>, 2, { i64::MIN }>>().get());
    assert!(is_addable::<At<Signed<8>, { i64::MIN }, { i64::MIN }>>().get());
}

#[test]
fn every_shipped_range_carries_its_sums_at_every_phase() {
    // The widest ranges this crate ships, at the phases furthest from zero a
    // phase can declare. Twice the top of the unsigned 64-bit range plus
    // `2^63 - 1` is about `2^65`, sixty-odd bits inside the index.
    assert!(is_addable::<At<Unsigned<64>, { i64::MAX }, 1>>().get());
    assert!(is_addable::<At<Unsigned<64>, { i64::MIN }, 1>>().get());
    assert!(is_addable::<At<Signed<64>, { i64::MAX }, 1>>().get());
    assert!(is_addable::<At<Signed<64>, { i64::MIN }, 1>>().get());
    // The least value over minus one is 2^63, which the index holds.
    assert!(is_addable::<At<Signed<64>, { i64::MIN }, -1>>().get());
    assert!(is_addable::<At<Signed<8>, { i64::MIN }, -1>>().get());
    // And a fractional phase, whose ceiling is one slot past its whole part.
    assert!(is_addable::<At<Unsigned<64>, { i64::MAX }, 2>>().get());
}

#[test]
fn no_phase_admits_a_range_past_the_doubling_bound_itself() {
    // DESIGN:813 states the carry obligation keeps every range addition admits
    // inside `[-2^126, 2^126)`. `doubled_plus` checks `slot.checked_mul(2)`
    // before it ever adds the phase's `whole` or `ceiling`, so that multiply
    // alone is the binding constraint: `i128::MAX / 2` is `2^126 - 1`, the
    // widest a slot index holds doubled, and `i128::MIN / 2` is `-2^126`
    // exactly. No phase moves either edge outward, only inward, because a
    // positive `ceiling` or `whole` can still overflow the checked add once
    // the multiply is already at its own limit (`the_obligation_refuses_a_sum_
    // the_coordinate_cannot_carry_on_both_sides` pins that narrowing), and a
    // negative one has no headroom left to spend. Pinned here at the phase
    // furthest from zero in the direction that would extend the bound if
    // extending it were possible: `i64::MIN` does not move the top past
    // `2^126 - 1`, and `i64::MAX` does not move the bottom past `-2^126`.
    type NearTheFarTop = Window<{ (1i128 << 126) - 4 }, { (1i128 << 126) - 1 }>;
    type PastTheFarTop = Window<{ (1i128 << 126) - 3 }, { 1i128 << 126 }>;
    assert!(is_addable::<At<NearTheFarTop, { i64::MIN }, 1>>().get());
    assert!(!is_addable::<At<PastTheFarTop, { i64::MIN }, 1>>().get());

    type NearTheFarBottom = Window<{ -(1i128 << 126) }, { -(1i128 << 126) + 3 }>;
    type PastTheFarBottom = Window<{ -(1i128 << 126) - 1 }, { -(1i128 << 126) + 2 }>;
    assert!(is_addable::<At<NearTheFarBottom, { i64::MAX }, 1>>().get());
    assert!(!is_addable::<At<PastTheFarBottom, { i64::MAX }, 1>>().get());
}

#[test]
fn the_obligation_refuses_a_sum_the_coordinate_cannot_carry_on_both_sides() {
    // Only a range an outside crate places near the index's end reaches the
    // refusal. Twice the top here is `i128::MAX - 1`, so one whole step fits and
    // two do not, and a fractional phase rounds a slot past its whole part:
    // three halves has the whole part one does, and is refused for the ceiling.
    assert!(is_addable::<At<NearTheTop, 0, 1>>().get());
    assert!(is_addable::<At<NearTheTop, 1, 1>>().get());
    assert!(!is_addable::<At<NearTheTop, 2, 1>>().get());
    assert!(is_addable::<At<NearTheTop, 1, 2>>().get());
    assert!(!is_addable::<At<NearTheTop, 3, 2>>().get());
    // Twice the bottom is `i128::MIN`, so no step down fits.
    assert!(is_addable::<At<NearTheBottom, 0, 1>>().get());
    assert!(!is_addable::<At<NearTheBottom, -1, 1>>().get());
    // A negative fractional phase has the whole part below it, so a third below
    // zero is refused where zero is not.
    assert!(!is_addable::<At<NearTheBottom, -1, 3>>().get());
    // And one further in than the bottom window, a step down fits again.
    assert!(is_addable::<At<Window<{ i128::MIN / 2 + 1 }, { i128::MIN / 2 + 4 }>, -2, 1>>().get());
    assert!(!is_addable::<At<Window<{ i128::MIN / 2 + 1 }, { i128::MIN / 2 + 4 }>, -3, 1>>().get());
}

/// The verdicts, bound at check time: none of these forces a refusal, and each
/// is a `const fn` the backend sees as a constant.
const _ADDABLE: Bool = is_addable::<Integer<8>>();
const _REFUSED: Bool = is_addable::<Floating<4, -3, 4>>();
const _HALF_WAY: Exact = sum_position::<Biased<8, 0, 1>>(Slot::ZERO, Slot::ZERO);
const _REACH: Reach = addition_reach::<Signature<Integer<4>, Adapt<Floor, Saturate>>>();
const _VERDICT: Bool = addition_is_associative::<Signature<Integer<4>, Adapt<Floor, Saturate>>>();
const _NEGATED: Exact = negation_position::<Biased<8, 0, 1>>(Slot::ZERO);
const _DIFFERENCE: Exact = difference_position::<Biased<8, 0, 1>>(Slot::ZERO, Slot::ZERO);
const _NEGATION_REACH: Reach = negation_reach::<Signature<Integer<4>, Adapt<Floor, Saturate>>>();
const _SUBTRACTION_REACH: Reach =
    subtraction_reach::<Signature<Integer<4>, Adapt<Floor, Saturate>>>();

#[test]
fn the_step_the_reach_and_the_verdict_evaluate_at_check_time() {
    assert!(_ADDABLE.get());
    assert!(!_REFUSED.get());
    assert_eq!(_HALF_WAY, Exact::between(Slot::ZERO, Fraction::HALF));
    assert_eq!(_REACH.positions_low(), Slot::at(-16));
    assert_eq!(_REACH.positions_high(), Slot::at(14));
    assert!(!_VERDICT.get());
}

#[test]
fn the_signed_steps_and_their_reaches_evaluate_at_check_time() {
    // A phase of one half: minus two copies is a whole slot, minus one is a
    // half below zero.
    assert_eq!(_NEGATED, Exact::on_grid(Slot::at(-1)));
    assert_eq!(_DIFFERENCE, Exact::between(Slot::at(-1), Fraction::HALF));
    // Four signed bits run from -8 to 7, so a negation runs from -7 to 8 and a
    // difference from -15 to 15.
    assert_eq!(_NEGATION_REACH.positions_low(), Slot::at(-7));
    assert_eq!(_NEGATION_REACH.positions_high(), Slot::at(8));
    assert_eq!(_SUBTRACTION_REACH.positions_low(), Slot::at(-15));
    assert_eq!(_SUBTRACTION_REACH.positions_high(), Slot::at(15));
}

// --- the reach -----------------------------------------------------------------

/// The reach of step `T` compared with every position it produces, at one format.
#[derive(Default)]
struct ReachAgainstStep<T> {
    step:    PhantomData<T>,
    formats: u32,
    on_grid: u32,
    ties:    u32,
    off:     u32,
}

impl<T: Step> PerFormat for ReachAgainstStep<T> {
    fn run<F: Format>(&mut self) {
        let reach = T::reach::<Signature<F, Adapt<Floor, Saturate>>>();
        let (mut low, mut high) = (i128::MAX, i128::MIN);
        let (mut off, mut tie) = (false, false);
        for (a, b) in operands::<T, F>() {
            let e = T::position::<F>(a, b);
            low = low.min(e.slot().index());
            high = high.max(e.slot().index());
            off |= !e.is_on_grid().get();
            tie |= e.is_tie().get();
        }
        let name = (T::NEGATED, coordinates::<F>());
        assert_eq!(reach.positions_low(), Slot::at(low), "{name:?}");
        assert_eq!(reach.positions_high(), Slot::at(high), "{name:?}");
        assert_eq!(reach.reaches_off_the_grid().get(), off, "{name:?}");
        assert_eq!(reach.reaches_a_tie().get(), tie, "{name:?}");
        assert_eq!(reach.translations_low(), Slot::ZERO, "{name:?}");
        assert_eq!(reach.translations_high(), Slot::ZERO, "{name:?}");
        self.formats += 1;
        match (off, tie) {
            (false, _) => self.on_grid += 1,
            (true, true) => self.ties += 1,
            (true, false) => self.off += 1,
        }
    }
}

/// The reach of `T` against its positions over every grid state and both ends
/// of the index.
fn the_reach_is_the_positions_of<T: Step + Default>() {
    let mut walk = ReachAgainstStep::<T>::default();
    walk.run::<Integer<3>>();
    walk.run::<UFixed<3, 0>>();
    walk.run::<Biased<3, 0, 1>>();
    walk.run::<Biased<3, 0, -3>>();
    walk.run::<Biased<3, 0, 4>>();
    every_phase!(walk; Signed<3>);
    every_phase!(walk; Unsigned<3>);
    every_phase!(walk; Window<-2, 3>);
    every_phase!(walk; Window<1, 4>);
    walk.run::<At<Signed<3>, 5, 4>>();
    walk.run::<At<Signed<3>, -7, 6>>();
    walk.run::<At<Signed<3>, 2, -3>>();
    // Ranges at the index's ends, where the positions the step produces sit at
    // the very edge of what it holds.
    walk.run::<At<NearTheTop, 1, 1>>();
    walk.run::<At<NearTheTop, 1, 2>>();
    walk.run::<At<NearTheBottom, 0, 1>>();

    assert_eq!(walk.formats, 5 + 4 * super::phases() as u32 + 3 + 3);
    // Every grid state is reached, so each branch of the reach's grid state is
    // compared rather than one of them.
    assert!(
        walk.on_grid > 0 && walk.ties > 0 && walk.off > 0,
        "{:?}",
        T::NEGATED
    );
}

#[test]
fn the_reach_is_the_positions_the_step_produces() {
    the_reach_is_the_positions_of::<Sum>();
}

#[test]
fn the_negation_reach_is_the_positions_the_negation_step_produces() {
    the_reach_is_the_positions_of::<Negation>();
}

#[test]
fn the_subtraction_reach_is_the_positions_the_difference_step_produces() {
    the_reach_is_the_positions_of::<Difference>();
}
