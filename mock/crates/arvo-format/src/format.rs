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
    exponent_at, magnitude_in_range, Exponent, Magnitude, MagnitudeCount, Quantum,
};
use crate::slots::{slot_in_range, Slot, Slots};
use crate::width::{Bool, Width};

/// The grid's offset from zero, in units of the quantum at magnitude zero.
///
/// A ratio, so the half-step bias is exact rather than approximated, and one
/// coordinate rather than two consts an implementor can put out of step with each
/// other. The parameterisation carries a phase, singular, and this is it.
///
/// **It holds the pair it was declared with.** Nothing here normalises the sign
/// or reduces the fraction. Both questions asked of it, whether the ratio is a
/// whole number and what that whole number is, are independent of the sign of the
/// denominator, and no other reader consults it, so an invariant that a positive
/// denominator would buy has no buyer. What it would cost is real: two writable
/// pairs have no normalisation inside the width the coordinates are declared in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Phase {
    num: i64,
    den: i64,
}

impl Phase {
    /// No offset, which leaves the additive identity at slot zero.
    pub const ZERO: Self = Self { num: 0, den: 1 };

    /// A phase of `num` over `den`.
    ///
    /// Total, and lossless on every pair. The numerator and denominator come back
    /// as they went in, so `of(3, i64::MIN)` is exactly three over the least
    /// value and not a tiny positive wearing its sign, and `of(i64::MIN, -7)` is
    /// exactly that ratio and not its negation.
    ///
    /// **A denominator of zero is not refused here and is not reinterpreted
    /// either.** It names no position on the grid, so the phase does not denote,
    /// and that is a condition on the format rather than on the pair:
    /// `Format::ADMITTED` refuses it where the coordinates are declared together,
    /// which is the same shape `Slots`, `Quantum` and `Ambient` already carry.
    /// Reading it as a denominator of one, which an earlier constructor did, is
    /// the one thing that cannot be right, because one over zero and one over one
    /// are different positions and only one of them exists.
    #[must_use]
    pub const fn of(num: i64, den: i64) -> Self {
        Self { num, den }
    }

    /// A phase counted in half steps, which is the biased grid's shape.
    #[must_use]
    pub const fn halves(num: i64) -> Self {
        Self { num, den: 2 }
    }

    /// The numerator, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn numerator(self) -> i64 {
        self.num
    }

    /// The denominator, for the one place a host contract needs it back.
    ///
    /// The second unwrap door. Two, rather than one, because the pair is what the
    /// coordinate is and handing back a single number would mean dividing.
    #[must_use]
    pub const fn denominator(self) -> i64 {
        self.den
    }

    /// Whether the pair names a position at all.
    ///
    /// The condition `Format::ADMITTED` refuses, written over the coordinate so
    /// the obligation and the verdict read the same predicate rather than
    /// restating it.
    #[must_use]
    pub const fn denotes(self) -> Bool {
        Bool::of(self.den != 0)
    }

    /// Whether the grid sits at zero.
    ///
    /// Not the question that decides the additive identity, which is the one
    /// below. A grid at zero has zero on it at slot zero; a grid one whole step
    /// along has zero on it one slot down, and this answers no about the second.
    #[must_use]
    pub const fn is_zero(self) -> Bool {
        Bool::of(self.num == 0)
    }

    /// Whether the offset is a whole number of quanta at magnitude zero.
    ///
    /// A phase of one whole step shifts the grid onto itself, so zero stays on it
    /// at a shifted slot, and only a fractional part takes it off there.
    ///
    /// **This is the constant family's answer and not the general one.** The
    /// phase is stated in units of the quantum at magnitude zero, and a law whose
    /// step moves with the magnitude names a different quantum at each one, so a
    /// ratio with a fractional part here can be a whole number of steps higher up.
    /// `has_additive_identity` is the question over the whole magnitude range;
    /// this is one coordinate of it, kept because the reduction is what the
    /// constant family's arms are about.
    ///
    /// **Computed one domain wider than the coordinates.** The least value over
    /// minus one overflows a remainder taken in the declared width, and it is a
    /// pair an implementor can write.
    #[must_use]
    pub const fn is_whole_multiple(self) -> Bool {
        if self.den == 0 {
            return Bool::FALSE;
        }
        Bool::of((self.num as i128) % (self.den as i128) == 0)
    }
}

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
    /// use arvo_format::format::{has_additive_identity, Format, Phase};
    /// use arvo_format::quantum::Constant;
    /// use arvo_format::slots::Signed;
    ///
    /// struct HalfStep;
    ///
    /// impl Format for HalfStep {
    ///     type Ambient = BinaryRationals;
    ///     type Quantum = Constant<0>;
    ///     type Slots = Signed<8>;
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
/// approximate: the running product in `cancelling_slot` is carried in `i128`,
/// which holds magnitudes below `2^127`, and at a radix of at least two the
/// product at least doubles per scaling step, so starting from one it leaves that
/// range within `SCALING_WIDTH` steps and every magnitude past this answers
/// `Isnt` regardless.
///
/// A radix below two never scales the product at all, and that case is answered
/// before the loop rather than bounded, so this is about the reachable case only.
///
/// So the bounded search returns what an unbounded one would, at every radix, and
/// the tests named `the_search_bound_*` are that derivation pinned.
const MAGNITUDE_SEARCH_BOUND: MagnitudeCount = MagnitudeCount::of(SCALING_WIDTH.count());

/// The width the cancellation carries its running product in, less its sign bit.
///
/// A count of bits, which is what bounds the scaling: at a radix of at least two
/// the product at least doubles per step, so from a magnitude of one it leaves a
/// domain of this many value bits within this many steps. That is the whole
/// derivation, and it is why the number is a property of the intermediate rather
/// than a figure somebody picked.
///
/// **A count of bits is not a count of magnitudes**, which is why the search bound
/// is derived from this rather than written beside it as a second literal. One
/// bounds the arithmetic inside one step of the walk and the other bounds the
/// walk, on the same reading the quantum law keeps an index and an extent apart.
///
/// **Inside the loop it is a termination proof rather than a decision.** The
/// overflow guard fires at or before this many steps for every radix of at least
/// two and every nonzero starting pair, so the loop never leaves through the
/// bound, and if it ever could the answer would be a partially scaled one.
const SCALING_WIDTH: Width = Width::bits(127);

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
/// any magnitude. The power scales whichever side keeps both whole, and each
/// scaling step is refused before it would leave the width rather than after, so
/// an `Isnt` is always a decided answer and never a wrap.
///
/// **The division is carried one width up on purpose.** The same equation solved
/// in the width the coordinates are declared in has two pairs it cannot answer:
/// the least numerator over minus one overflows the remainder, and that numerator
/// over one produces a quotient whose negation overflows. Both are reachable
/// through the open trait, both diverge rather than answer, and diverging on the
/// value path is what `ruling::never_a_runtime_check_and_one_lowered_path`
/// forbids. One width up there is no such pair, and the range check at the end is
/// what turns a quotient no slot index can hold into an answer rather than a wrap.
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
    let mut num = -(F::PHASE.numerator() as i128);
    let mut den = F::PHASE.denominator() as i128;

    // A radix below two scales nothing: at one the quantum is the same at every
    // magnitude and at zero it is not a number at a negative exponent. Both are
    // refused in a binary by `Ambient::ADMITTED` and both can still reach a
    // check-time evaluation, so the answer is the one at magnitude zero rather
    // than a division by a radix that is not there.
    if radix >= 2 {
        let mut taken = 0u32;
        if steps > 0 {
            while (taken as i128) < steps && taken < SCALING_WIDTH.count() {
                if den > i128::MAX / radix || den < i128::MIN / radix {
                    return Maybe::Isnt;
                }
                den *= radix;
                taken += 1;
            }
        } else if steps < 0 {
            while (taken as i128) < -steps && taken < SCALING_WIDTH.count() {
                if num > i128::MAX / radix || num < i128::MIN / radix {
                    return Maybe::Isnt;
                }
                num *= radix;
                taken += 1;
            }
        }
    }

    if den == 0 || num % den != 0 {
        return Maybe::Isnt;
    }
    let slot = num / den;
    if slot < i64::MIN as i128 || slot > i64::MAX as i128 {
        return Maybe::Isnt;
    }
    Maybe::Is(Slot::at(slot as i64))
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
            }
            Maybe::Isnt => {}
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
