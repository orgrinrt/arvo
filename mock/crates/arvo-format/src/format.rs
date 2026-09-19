//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A format: an ambient domain and a representable set, and membership in it.
//!
//! The set is a constant of the type. A value set that depends on other data is
//! not a format but storage, which is the line the canon draws and the reason
//! every coordinate here is an associated item rather than a field.
//!
//! The concept is closed and the inventory is open: a new numeral joins by
//! implementing `Format` and supplying the coordinates, and nothing here is an
//! enumeration anybody has to widen. Every coordinate it supplies carries a type
//! this crate owns, so a numeral declared elsewhere is written in the same types
//! as the ones shipped here and is not refused by the lints that read every crate
//! but this one.
//!
//! Membership is one predicate over two coordinates, a slot and a magnitude, so a
//! question about one particular value is that predicate at the coordinates
//! denoting it. `has_additive_identity` below is `contains` asked where zero would
//! sit, and it quantifies over both because `contains` does. The alternative
//! already happened twice: a conjunction over the same coordinates, short one of
//! them, disagreeing with `contains` about the same question.

use notko::Maybe;

use crate::ambient::{Ambient, Radix};
use crate::quantum::{
    Exponent,
    Magnitude,
    MagnitudeCount,
    Quantum,
    exponent_at,
    magnitude_in_range,
};
use crate::slots::{Slot, Slots, slot_in_range};
use crate::width::{Bool, Width};

mod phase;

pub use phase::Phase;

/// A representable set, together with the domain it sits in.
///
/// Membership is one affine predicate over these coordinates: a value is in the
/// set exactly when it is `phase + slot * quantum(magnitude)` for some admitted
/// slot and magnitude.
pub trait Format {
    /// The domain the set is drawn from.
    type Ambient: Ambient;

    /// How the step changes with magnitude.
    type Quantum: Quantum;

    /// Which multiples of the step are admitted.
    type Slots: Slots;

    /// The grid's offset, in units of the quantum at magnitude zero.
    ///
    /// Carried explicitly and never assumed zero. Whether some admitted magnitude
    /// and slot cancel it decides whether the set carries an additive identity at
    /// all, and therefore whether the identity adaptation ever occurs, so a design
    /// that hardcoded it would have closed a case the canon leaves open.
    ///
    /// The units matter and are the reason the answer is not a property of the
    /// phase alone: it is fixed in the quantum at magnitude zero, and the quantum
    /// at every other magnitude is a different size, so the same phase is a
    /// different number of steps at each of them.
    const PHASE: Phase;

    /// What an implementor owes, checked rather than asked for.
    ///
    /// The phase denominator is nonzero. A denominator of zero names no position
    /// on the grid, so the phase coordinate does not denote and the membership
    /// predicate has nothing to be asked about.
    ///
    /// **It fires where it is forced and nowhere else**, which is
    /// `cancelling_slot` and `has_additive_identity` and nothing else in the
    /// crate. `adapt` forces the slot range's obligation rather than this one.
    /// So a coordinate read off the impl, and `contains`, both reach a value
    /// without meeting it, and an implementor writing an empty `ADMITTED` over
    /// this default meets it and finds nothing there. `contains` is not a route
    /// that forces nothing: it goes through `magnitude_in_range` and
    /// `slot_in_range`, so it forces the quantum's obligation and the slot
    /// range's, and never this one or the ambient's.
    /// That is why the predicates below stay total and answer a zero
    /// denominator rather than assuming it was refused.
    ///
    /// ```compile_fail
    /// use arvo_format::ambient::BinaryRationals;
    /// use arvo_format::format::{has_additive_identity, Format, Phase};
    /// use arvo_format::quantum::Constant;
    /// use arvo_format::slots::Signed;
    ///
    /// struct NoDenominator;
    ///
    /// impl Format for NoDenominator {
    ///     type Ambient = BinaryRationals;
    ///     type Quantum = Constant<0>;
    ///     type Slots = Signed<8>;
    ///     const PHASE: Phase = Phase::of(1, 0);
    /// }
    ///
    /// fn main() {
    ///     let _ = has_additive_identity::<NoDenominator>();
    /// }
    /// ```
    ///
    /// The control, which is what says the refusal above is this obligation: the
    /// same shape with a denominator of two builds.
    ///
    /// ```
    /// use arvo_format::ambient::BinaryRationals;
    /// use arvo_format::format::{Format, Phase, has_additive_identity};
    /// use arvo_format::quantum::Constant;
    /// use arvo_format::slots::Signed;
    ///
    /// struct HalfStep;
    ///
    /// impl Format for HalfStep {
    ///     type Ambient = BinaryRationals;
    ///     type Quantum = Constant<0>;
    ///     type Slots = Signed<8>;
    ///
    ///     const PHASE: Phase = Phase::of(1, 2);
    /// }
    ///
    /// fn main() {
    ///     assert!(!has_additive_identity::<HalfStep>().get());
    /// }
    /// ```
    ///
    /// It refuses what the assertion below names and nothing further.
    const ADMITTED: () = {
        assert!(
            Self::PHASE.denotes().get(),
            "a phase denominator of zero names no position on the grid, so the phase does not \
             denote a value the set could contain"
        );
    };
}

/// Whether a format meets what the contract asks of it.
///
/// The verdict form, so a construction that compiles and is wrong can be reported
/// on without forcing the const that would refuse it.
#[must_use]
pub const fn is_admissible_format<F: Format>() -> Bool {
    F::PHASE.denotes()
}

/// Whether the coordinates name a member of the format's representable set.
///
/// The affine predicate, evaluated. Free rather than a method so it is reachable
/// at stage zero without the trait being const.
#[must_use]
pub const fn contains<F: Format>(slot: Slot, magnitude: Magnitude) -> Bool {
    magnitude_in_range::<F::Quantum>(magnitude).and(slot_in_range::<F::Slots>(slot))
}

/// How far the magnitude search runs before a later magnitude can add nothing.
///
/// A count of magnitudes, which is what the outer search walks. Exact rather than
/// approximate. A magnitude moves the scaling by at least one step when the slope
/// is not zero, and past `SCALING_STEPS` steps every scaling in `cancelling_slot`
/// has already been decided `Isnt`, so every magnitude past this answers `Isnt`
/// regardless. Where the slope is zero every magnitude is the same equation and
/// the first one answers for all of them.
///
/// A radix below two never scales at all, and that case is answered before the
/// loop rather than bounded, so this is about the reachable case only.
///
/// So the bounded search returns what an unbounded one would, at every radix, and
/// the tests named `the_search_bound_*` are that derivation pinned.
const MAGNITUDE_SEARCH_BOUND: MagnitudeCount = MagnitudeCount::of(SCALING_STEPS.count() + 1);

/// The most scaling steps `cancelling_slot` can take before its answer is decided.
///
/// A count of steps, carried as the sum of two widths less their sign bits,
/// because at the smallest radix every step moves one bit. The phase is
/// reduced to lowest terms before it is scaled, so while the numerator scales the
/// denominator only ever loses the radix's common factor, and a denominator the
/// phase declares is at most `2^63` in size, so it can lose one at most 63 times.
/// Once it is one, every step multiplies the numerator by the radix, at least
/// two, from at least one, so within 127 steps it leaves the slot index. A step
/// that neither shrinks the denominator nor finds it at one means the radix
/// shares no factor with it, and no number of steps makes the phase whole.
///
/// Scaling the denominator instead ends sooner: at most 64 steps take it past
/// every numerator a phase can declare, after which no division is exact.
///
/// A count of steps is not a count of magnitudes, which is why the search bound
/// is derived from this rather than written beside it as a second literal.
const SCALING_STEPS: Width = PHASE_DENOMINATOR_WIDTH.add(SLOT_INDEX_WIDTH);

/// A phase's denominator width, less its sign bit.
const PHASE_DENOMINATOR_WIDTH: Width = Width::bits(i64::BITS - 1);

/// The slot index's width, less its sign bit.
const SLOT_INDEX_WIDTH: Width = Width::bits(i128::BITS - 1);

/// Euclid's algorithm over a non-negative and a positive operand.
pub(crate) const fn greatest_common_divisor(a: i128, b: i128) -> i128 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// The slot that cancels the phase at one magnitude, if an admitted one does.
///
/// `Isnt` means no integer slot cancels the phase there, or that the slot which
/// would is too large to be carried, which is the same answer for the purpose: a
/// slot that far out is in no admitted range.
///
/// The arithmetic is exact and in integers. From `phase + i * quantum(m) = 0`
/// with `phase = (n / d) * radix^BASE` and `quantum(m) = radix^(BASE + SLOPE * m)`,
/// dividing through by `radix^BASE` and clearing the denominator gives
///
/// ```text
///   i * d * radix^(SLOPE * m) = -n
/// ```
///
/// so `BASE` leaves the problem entirely and the answer does not depend on it at
/// any magnitude. The pair is taken in lowest terms, the power scales whichever
/// side keeps both whole, and a scaling step that would leave the index is
/// refused as it is taken rather than after, so an `Isnt` is always a decided
/// answer and never a wrap.
///
/// The division is carried one width up from the phase on purpose. The same
/// equation solved in the width the phase is declared in has two pairs it cannot
/// answer: the least numerator over minus one overflows the remainder, and that
/// numerator over one produces a quotient whose negation overflows. Both are
/// reachable through the open trait, both diverge rather than answer, and
/// diverging on the value path is what
/// `ruling::never_a_runtime_check_and_one_lowered_path` forbids. The wider width
/// is the slot index's own, so a quotient is a slot as it stands; scaling can
/// still bring the numerator to that width's least value, and the division is
/// checked so that pair over minus one is an `Isnt` rather than a wrap.
///
/// Named rather than folded into the predicate below because that predicate is a
/// conjunction, and a law over the conjunction alone cannot say which half a
/// failure came from.
#[must_use]
pub const fn cancelling_slot<F: Format>(magnitude: Magnitude) -> Maybe<Slot> {
    let () = <F as Format>::ADMITTED;
    let () = <F::Ambient as Ambient>::ADMITTED;

    if !F::PHASE.denotes().get() {
        return Maybe::Isnt;
    }
    // A zero phase is cancelled by slot zero at every magnitude, whatever the
    // quantum does, so the scaling below would reach the same answer more slowly.
    if F::PHASE.is_zero().get() {
        return Maybe::Is(Slot::ZERO);
    }

    let radix = <F::Ambient as Ambient>::RADIX.base() as i128;
    let steps = (<F::Quantum as Quantum>::SLOPE.power() as i128) * (magnitude.index() as i128);
    // Both are at most `2^63` in size, so neither sign flip nor the reduction
    // can leave the index. Lowest terms first, so a numerator scaled up is never
    // carrying a factor the denominator would have cancelled, which is what lets
    // an exact quotient the index holds be found even where the unreduced
    // product would not fit.
    let common = greatest_common_divisor(
        (F::PHASE.numerator() as i128).abs(),
        (F::PHASE.denominator() as i128).abs(),
    );
    let mut num = -(F::PHASE.numerator() as i128) / common;
    let mut den = (F::PHASE.denominator() as i128) / common;

    // A radix below two scales nothing: at one the quantum is the same at every
    // magnitude and at zero it is not a number at a negative exponent. Both are
    // refused in a binary by `Ambient::ADMITTED` and both can still reach a
    // check-time evaluation, so the answer is the one at magnitude zero rather
    // than a division by a radix that is not there.
    //
    // Every loop below leaves through one of its answers within
    // `SCALING_STEPS`, whatever `steps` is, which `SCALING_STEPS` derives.
    if radix >= 2 {
        let mut taken: i128 = 0;
        if steps > 0 {
            while taken < steps {
                // The pair is in lowest terms and the numerator is not zero, so
                // a denominator past it divides nothing, and it only grows.
                if den.abs() > num.abs() {
                    return Maybe::Isnt;
                }
                den = match den.checked_mul(radix) {
                    Some(scaled) => scaled,
                    None => return Maybe::Isnt,
                };
                taken += 1;
            }
        } else if steps < 0 {
            while taken < -steps {
                let shared = greatest_common_divisor(den.abs(), radix);
                if shared == 1 && den.abs() != 1 {
                    // The radix shares no factor with what is left of the
                    // denominator, so no number of steps makes the phase whole.
                    return Maybe::Isnt;
                }
                den /= shared;
                num = match num.checked_mul(radix / shared) {
                    Some(scaled) => scaled,
                    None => return Maybe::Isnt,
                };
                taken += 1;
            }
        }
    }

    // Checked, because scaling can leave the numerator at the least value the
    // index holds, and over a denominator of minus one that quotient is one past
    // the top. No slot holds it, so the overflow is the `Isnt` it would be.
    match (num.checked_rem(den), num.checked_div(den)) {
        (Some(0), Some(slot)) => Maybe::Is(Slot::at(slot)),
        _ => Maybe::Isnt,
    }
}

/// Whether the format's representable set carries an additive identity.
///
/// The membership predicate asked at the value zero, rather than a rule of its
/// own. Zero is in the set exactly when some admitted magnitude and some admitted
/// slot cancel the phase, so the existential runs over both coordinates, which is
/// the same pair membership quantifies over.
///
/// Two consequences follow that a rule about the phase alone does not carry. A
/// whole phase whose cancelling slot falls outside the slot range leaves no
/// identity on the grid. And where the quantum shrinks as the magnitude rises, a
/// phase with a fractional part at magnitude zero is a whole number of steps at a
/// higher magnitude, so it does leave the identity on the grid.
///
/// In the constant family this reduces to `Phase::is_whole_multiple`, because
/// there is one quantum and one magnitude. It does not reduce that way in the
/// magnitude-indexed family, where the quantum names a different value at each
/// magnitude.
///
/// A format admitting no magnitude has an empty set, so it carries nothing, the
/// identity included. A denominator of zero names no position and is answered as
/// no identity rather than divided by.
#[must_use]
pub const fn has_additive_identity<F: Format>() -> Bool {
    let () = <F as Format>::ADMITTED;
    let () = <F::Quantum as Quantum>::ADMITTED;

    if !F::PHASE.denotes().get() {
        return Bool::FALSE;
    }

    let declared = <F::Quantum as Quantum>::MAGNITUDES.count();
    let bound = MAGNITUDE_SEARCH_BOUND.count();
    let searched = if declared < bound { declared } else { bound };

    let mut index = 0u32;
    while index < searched {
        match cancelling_slot::<F>(Magnitude::at(index)) {
            Maybe::Is(slot) => {
                if slot_in_range::<F::Slots>(slot).get() {
                    return Bool::TRUE;
                }
            },
            Maybe::Isnt => {},
        }
        index += 1;
    }
    Bool::FALSE
}

/// The exponent of the step at a magnitude, for the format's quantum law.
#[must_use]
pub const fn step_exponent<F: Format>(magnitude: Magnitude) -> Exponent {
    exponent_at::<F::Quantum>(magnitude)
}

/// The radix the format's ambient domain counts in.
#[must_use]
pub const fn radix<F: Format>() -> Radix {
    let () = <F::Ambient as Ambient>::ADMITTED;
    <F::Ambient as Ambient>::RADIX
}

/// Whether the smallest magnitude's slots reach down to the grid's own step.
///
/// This is what makes subnormals fall out rather than be a case: at the smallest
/// magnitude the step is the smallest step the law produces, and the values it
/// admits are exactly those the floating conventions call subnormal. Nothing
/// names them and no branch selects them.
#[must_use]
pub const fn smallest_step_exponent<F: Format>() -> Exponent {
    let last = <F::Quantum as Quantum>::MAGNITUDES.largest();
    let at_zero = exponent_at::<F::Quantum>(Magnitude::SMALLEST);
    let at_last = exponent_at::<F::Quantum>(last);
    at_zero.min(at_last)
}
