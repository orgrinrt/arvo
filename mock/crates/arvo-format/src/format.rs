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
//! enumeration anybody has to widen.

use notko::Maybe;

use crate::ambient::Ambient;
use crate::quantum::{exponent_at, magnitude_in_range, Quantum};
use crate::slots::{slot_in_range, Slots};

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

    /// The numerator of the phase, in units of the quantum at magnitude zero.
    ///
    /// Carried explicitly and never assumed zero. Together with the denominator
    /// it decides whether the set carries an additive identity at all, and
    /// therefore whether the identity adaptation ever occurs, so a design that
    /// hardcoded it would have closed a case the canon leaves open.
    ///
    /// The units matter and are the reason the answer is not a property of the
    /// phase alone: the phase is fixed in the quantum at magnitude zero, and the
    /// quantum at every other magnitude is a different size, so the same phase is
    /// a different number of steps at each of them.
    const PHASE_NUM: i64;

    /// The denominator of the phase, in units of the quantum at magnitude zero.
    ///
    /// One for an unbiased grid, two for the half-step bias. Never zero, and
    /// `ADMITTED` is what makes that a check rather than a request.
    const PHASE_DEN: i64;

    /// What an implementor owes, checked rather than asked for.
    ///
    /// The phase denominator is nonzero. A denominator of zero names no position
    /// on the grid, so the phase coordinate does not denote and the membership
    /// predicate has nothing to be asked about.
    ///
    /// **It fires at codegen, not at `cargo check`**, so `cargo build` refuses
    /// and `cargo check` does not, which is why the predicates below stay total
    /// and answer a zero denominator rather than assuming it was refused.
    ///
    /// ```compile_fail
    /// use arvo_format::ambient::BinaryRationals;
    /// use arvo_format::format::{has_additive_identity, Format};
    /// use arvo_format::quantum::Constant;
    /// use arvo_format::slots::Signed;
    ///
    /// struct NoDenominator;
    ///
    /// impl Format for NoDenominator {
    ///     type Ambient = BinaryRationals;
    ///     type Quantum = Constant<0>;
    ///     type Slots = Signed<8>;
    ///     const PHASE_NUM: i64 = 1;
    ///     const PHASE_DEN: i64 = 0;
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
    /// use arvo_format::format::{has_additive_identity, Format};
    /// use arvo_format::quantum::Constant;
    /// use arvo_format::slots::Signed;
    ///
    /// struct HalfStep;
    ///
    /// impl Format for HalfStep {
    ///     type Ambient = BinaryRationals;
    ///     type Quantum = Constant<0>;
    ///     type Slots = Signed<8>;
    ///     const PHASE_NUM: i64 = 1;
    ///     const PHASE_DEN: i64 = 2;
    /// }
    ///
    /// fn main() {
    ///     assert!(!has_additive_identity::<HalfStep>());
    /// }
    /// ```
    ///
    /// It refuses what the assertion below names and nothing further.
    const ADMITTED: () = {
        assert!(
            Self::PHASE_DEN != 0,
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
pub const fn is_admissible_format<F: Format>() -> bool {
    F::PHASE_DEN != 0
}

/// Whether the coordinates name a member of the format's representable set.
///
/// The affine predicate, evaluated. Free rather than a method so it is reachable
/// at stage zero without the trait being const.
#[must_use]
pub const fn contains<F: Format>(slot: i64, magnitude: u32) -> bool {
    magnitude_in_range::<F::Quantum>(magnitude) && slot_in_range::<F::Slots>(slot)
}

/// How far the scaling below runs before a later step can add nothing.
///
/// Exact rather than approximate. The running product is carried in `i128`, which
/// holds magnitudes below `2^127`, and at a radix of at least two the product at
/// least doubles per step, so starting from one it leaves that range within 127
/// steps and every step past this answers `Isnt` regardless.
///
/// A radix below two never scales the product at all, and the loop is skipped
/// outright there rather than bounded, so the bound is about the reachable case
/// and the unreachable one is answered before it.
///
/// So the bounded search returns what an unbounded one would, at every radix, and
/// the two tests named `the_search_bound_*` are that derivation pinned.
const MAGNITUDE_SEARCH_BOUND: u32 = 127;

/// The slot that cancels the phase at one magnitude, if an admitted one does.
///
/// `Isnt` means no integer slot cancels the phase there, or that the slot which
/// would is too large to be carried, which is the same answer for the purpose: a
/// slot that far out is in no admitted range.
///
/// The arithmetic is exact and in integers. From `phase + i * quantum(m) = 0`
/// with `phase = (PHASE_NUM / PHASE_DEN) * radix^BASE` and
/// `quantum(m) = radix^(BASE + SLOPE * m)`, dividing through by `radix^BASE`
/// and clearing the denominator gives
///
/// ```text
///   i * PHASE_DEN * radix^(SLOPE * m) = -PHASE_NUM
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
pub const fn cancelling_slot<F: Format>(magnitude: u32) -> Maybe<i64> {
    let () = <F as Format>::ADMITTED;
    let () = <F::Ambient as Ambient>::ADMITTED;

    if F::PHASE_DEN == 0 {
        return Maybe::Isnt;
    }
    // A zero phase is cancelled by slot zero at every magnitude, whatever the
    // quantum does, so the scaling below would reach the same answer more slowly.
    if F::PHASE_NUM == 0 {
        return Maybe::Is(0);
    }

    let radix = <F::Ambient as Ambient>::RADIX as i128;
    let steps = (<F::Quantum as Quantum>::SLOPE as i128) * (magnitude as i128);
    let mut num = -(F::PHASE_NUM as i128);
    let mut den = F::PHASE_DEN as i128;

    // A radix below two scales nothing: at one the quantum is the same at every
    // magnitude and at zero it is not a number at a negative exponent. Both are
    // refused in a binary by `Ambient::ADMITTED` and both can still reach a
    // check-time evaluation, so the answer is the one at magnitude zero rather
    // than a division by a radix that is not there.
    if radix >= 2 {
        let mut taken = 0u32;
        if steps > 0 {
            while (taken as i128) < steps && taken < MAGNITUDE_SEARCH_BOUND {
                if den > i128::MAX / radix || den < i128::MIN / radix {
                    return Maybe::Isnt;
                }
                den *= radix;
                taken += 1;
            }
        } else if steps < 0 {
            while (taken as i128) < -steps && taken < MAGNITUDE_SEARCH_BOUND {
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
    Maybe::Is(slot as i64)
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
/// In the constant family this reduces to the whole-multiple rule, because there
/// is one quantum and one magnitude. It does not reduce that way in the
/// magnitude-indexed family, where the quantum names a different value at each
/// magnitude.
///
/// A format admitting no magnitude has an empty set, so it carries nothing, the
/// identity included. A denominator of zero names no position and is answered as
/// no identity rather than divided by.
#[must_use]
pub const fn has_additive_identity<F: Format>() -> bool {
    let () = <F as Format>::ADMITTED;
    let () = <F::Quantum as Quantum>::ADMITTED;

    if F::PHASE_DEN == 0 {
        return false;
    }

    let magnitudes = <F::Quantum as Quantum>::MAGNITUDES;
    let bound = if magnitudes < MAGNITUDE_SEARCH_BOUND {
        magnitudes
    } else {
        MAGNITUDE_SEARCH_BOUND
    };

    let mut magnitude = 0u32;
    while magnitude < bound {
        match cancelling_slot::<F>(magnitude) {
            Maybe::Is(slot) => {
                if slot_in_range::<F::Slots>(slot) {
                    return true;
                }
            }
            Maybe::Isnt => {}
        }
        magnitude += 1;
    }
    false
}

/// The exponent of the step at a magnitude, for the format's quantum law.
#[must_use]
pub const fn step_exponent<F: Format>(magnitude: u32) -> i32 {
    exponent_at::<F::Quantum>(magnitude)
}

/// The radix the format's ambient domain counts in.
#[must_use]
pub const fn radix<F: Format>() -> u32 {
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
pub const fn smallest_step_exponent<F: Format>() -> i32 {
    // Forced before the subtraction below, which underflows at a magnitude count
    // of zero. The obligation is what removes that case from a binary.
    let () = <F::Quantum as Quantum>::ADMITTED;
    let last = <F::Quantum as Quantum>::MAGNITUDES - 1;
    let at_zero = exponent_at::<F::Quantum>(0);
    let at_last = exponent_at::<F::Quantum>(last);
    if at_zero < at_last {
        at_zero
    } else {
        at_last
    }
}
