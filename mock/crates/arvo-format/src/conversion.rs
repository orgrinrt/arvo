//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A member of one format placed on another format's grid, in one step.
//!
//! A member of `F` at slot `a` is `(phase_F + a) q_F`, and on the grid of `T` it
//! sits at position `(phase_F + a) q_F / q_T - phase_T`, a function of the source
//! format and the target signature together. With each phase `n / d` in lowest
//! terms and, per format, `U = r^max(e, 0)` and `V = r^max(-e, 0)`, that position
//! is the numerator `(n_F + a d_F) U_F V_T d_T - n_T d_F V_F U_T` over
//! `D = d_F V_F U_T d_T`. The radixes are read separately, so the two formats
//! need not share one. It is one placement, never a chain through a format
//! between the two, so the result is rounded once.
//!
//! One obligation carries four conditions: both phases denote, and three that
//! are not yet admitted rather than refused, a magnitude-indexed quantum on
//! either side, a `D` past `2^128 - 1`, and a range end the slot index does not
//! hold on the target's grid. It is forced by the exact step, and
//! `is_convertible` asks the same without forcing it.

use core::marker::PhantomData;

use notko::Maybe;

use crate::adapt::{Arity, DeclaredSignature, Operation};
use crate::apply::{Dither, Exact, adapt};
use crate::format::Format;
use crate::placing::{
    Placed,
    Powers,
    Reduced,
    all,
    is_held,
    over,
    positioned,
    powers,
    reduced_phase,
    saturated,
    times,
};
use crate::quantum::is_constant_family;
use crate::slots::{Slot, Slots};
use crate::wide::Wide;
use crate::width::Bool;

/// Conversion from the format `F` onto the grid of the signature `S`.
///
/// It names the target signature it adapts under, the source format beside it,
/// and its arity, one, and carries nothing else.
pub struct Convert<F: Format, S: DeclaredSignature>(PhantomData<(F, S)>);

impl<F: Format, S: DeclaredSignature> Operation for Convert<F, S> {
    type Signature = S;

    const ARITY: Arity = Arity::UNARY;
}

/// What a pair of formats owes the conversion, checked rather than asked for.
struct Convertible<F, T>(PhantomData<(F, T)>);

impl<F: Format, T: Format> Convertible<F, T> {
    /// The four conditions, each asserted from `conditions`, which
    /// `is_convertible` folds. The phases are first because the others read
    /// their parts.
    const ADMITTED: () = {
        let [denote, constant, fits, carried] = conditions::<F, T>();
        assert!(
            denote.get(),
            "conversion is refused where either phase's denominator is zero: such a phase names \
             no member, so there is nothing to place or nowhere to place it"
        );
        // FIXME(arvo_quotient_and_conversion_over_a_magnitude_indexed_quantu): the
        // placement is a function of both formats here too. `Exact` and `adapt`
        // carry no magnitude coordinate to read it from or land it at.
        assert!(
            constant.get(),
            "conversion is not yet admitted over a magnitude-indexed quantum on either side: a slot \
             names a different value at each magnitude, and the applied map carries no magnitude \
             coordinate yet"
        );
        // FIXME(arvo_s_exact_position_carries_no_remainder_denominator_past): the
        // position is a function of the pair here too, and only its remainder's
        // denominator is past the unsigned 128 bits it is carried over.
        assert!(
            fits.get(),
            "conversion is not yet admitted where the remainder's denominator d_F V_F U_T d_T, with \
             d each phase's reduced denominator, U = r^max(e, 0) and V = r^max(-e, 0) per format, \
             passes 2^128 - 1: an exact position carries its remainder over an unsigned 128-bit \
             denominator"
        );
        // FIXME(arvo_quotient_and_conversion_positions_past_the_slot_index): the
        // positions at the range ends are functions of the pair, and only the slot
        // index's own integer is too narrow to hold them.
        assert!(
            carried.get(),
            "conversion is not yet admitted between these formats: a member at an end of the \
             source's range lands past the slot coordinate on the target's grid, so the position \
             it names is not one a slot index can hold yet"
        );
    };
    /// The step's constants, or nothing where a phase does not denote or `D`
    /// does not fit, settled once per pair.
    const STEP: Maybe<Step> = step::<F, T>();
}

/// What the step reads of the two formats.
#[derive(Clone, Copy)]
struct Step {
    /// The source's phase in lowest terms.
    source:  Reduced,
    /// `U_F V_T d_T`, or nothing when it leaves the four limbs, in which case any
    /// member whose factor is not zero is past the index.
    scale:   Maybe<Wide>,
    /// `n_T d_F V_F U_T`, the target's phase term, at most `2^63` times `D`.
    offset:  Wide,
    /// `D = d_F V_F U_T d_T`, from one up to `2^128 - 1`.
    divisor: u128,
}

/// The step's constants for the pair, or nothing where a phase does not denote
/// or `D` is past `2^128 - 1`.
#[inline]
const fn step<F: Format, T: Format>() -> Maybe<Step> {
    let (Maybe::Is(source), Maybe::Is(target)) = (reduced_phase::<F>(), reduced_phase::<T>())
    else {
        return Maybe::Isnt;
    };
    let Powers {
        up: up_f,
        down: down_f,
    } = powers::<F>();
    let Powers {
        up: up_t,
        down: down_t,
    } = powers::<T>();
    // `d_F V_F U_T`, then `D` as that times `d_T`.
    let Maybe::Is(fall) = times(times(Maybe::Is(Wide::of(source.denominator)), down_f), up_t)
    else {
        return Maybe::Isnt;
    };
    let divisor = match fall.times(Wide::of(target.denominator)) {
        Maybe::Is(v) => {
            match v.unsigned() {
                Maybe::Is(u) if u > 0 => u,
                _ => return Maybe::Isnt,
            }
        },
        Maybe::Isnt => return Maybe::Isnt,
    };
    // `d_F V_F U_T` is at most `D`, under `2^128`, and `|n_T|` at most `2^63`.
    let Maybe::Is(offset) = Wide::of(target.numerator).times(fall) else {
        return Maybe::Isnt;
    };
    Maybe::Is(Step {
        source,
        scale: times(times(up_f, down_t), Maybe::Is(Wide::of(target.denominator))),
        offset,
        divisor,
    })
}

/// The member at slot `a` placed on the target's grid by the step.
///
/// The numerator `(n_F + a d_F) U_F V_T d_T - n_T d_F V_F U_T` in four limbs,
/// then its floor over `D`. A numerator that leaves the limbs, or a floor that
/// leaves the index, is past it on the sign of `n_F + a d_F`; for a member the
/// obligation has ruled both out.
#[inline]
const fn placed(step: Step, a: i128) -> Placed {
    // Under `2^191` in magnitude for any slot, so it is formed; the arm is the
    // answer the step gives if it were not.
    let Maybe::Is(fa) = (match Wide::of(a).times(Wide::of(step.source.denominator)) {
        Maybe::Is(ad) => ad.plus(Wide::of(step.source.numerator)),
        Maybe::Isnt => Maybe::Isnt,
    }) else {
        return Placed::Past(a < 0);
    };
    let negative = fa.is_negative().get();
    let scaled = if fa.is_zero().get() {
        fa
    } else {
        match times(Maybe::Is(fa), step.scale) {
            Maybe::Is(p) => p,
            Maybe::Isnt => return Placed::Past(negative),
        }
    };
    match scaled.plus(step.offset.negated()) {
        Maybe::Is(numerator) => over(numerator, step.divisor),
        Maybe::Isnt => Placed::Past(negative),
    }
}

/// Whether every member of the source lands at a position a slot index holds on
/// the target's grid.
///
/// The position is affine and increasing in the slot, so the range's two ends
/// decide it. Nothing where `D` does not fit.
#[inline]
const fn conversion_is_carried<F: Format, T: Format>() -> Bool {
    let Maybe::Is(step) = Convertible::<F, T>::STEP else {
        return Bool::FALSE;
    };
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    Bool::of(is_held(placed(step, min)).get() && is_held(placed(step, max)).get())
}

/// The exact position of the member of `F` at slot `a` on the grid of `T`.
///
/// `(phase_F + a) q_F / q_T - phase_T`, in the target's slot units: on its grid,
/// or at the floor slot with the remainder over `d_F V_F U_T d_T` in lowest
/// terms. For a member the position is exact. For a slot outside the source's
/// range, a numerator past the four limbs or a position past the index saturates
/// at the end on the position's sign, so the step still answers.
///
/// Forces the conversion obligation, so a pair the conversion does not admit
/// stops the build here, at check time when the call is bound in a `const` item.
#[must_use]
#[inline]
pub const fn converted_position<F: Format, T: Format>(a: Slot) -> Exact {
    let () = Convertible::<F, T>::ADMITTED;
    match Convertible::<F, T>::STEP {
        Maybe::Is(step) => positioned(step.divisor, placed(step, a.index())),
        // The obligation above has refused this pair already.
        Maybe::Isnt => saturated(false),
    }
}

/// The member of `F` at slot `a`, placed on the grid of `S` and adapted under it.
///
/// The exact step, then `adapt` under the target signature, with the dither
/// passed through for the one mode that reads it. Total over every slot, with no
/// check and no panic.
#[must_use]
#[inline]
pub const fn convert<F: Format, S: DeclaredSignature>(a: Slot, dither: Dither) -> Slot {
    adapt::<S>(converted_position::<F, S::Format>(a), dither)
}

/// Whether the conversion from `F` onto `T` is admitted, asked without forcing
/// the obligation.
///
/// The obligation's own verdicts folded, so a test can hold a pair the
/// conversion does not admit and report on it rather than stop the build.
#[must_use]
#[inline]
pub const fn is_convertible<F: Format, T: Format>() -> Bool {
    all(conditions::<F, T>())
}

/// The obligation's four verdicts, in order: both phases denote, both quanta
/// are constant, `D` fits, the range ends are carried.
///
/// The one place they are stated, read by the obligation and by
/// `is_convertible`. The last two read the step and answer false where a phase
/// does not denote.
#[inline]
const fn conditions<F: Format, T: Format>() -> [Bool; 4] {
    [
        Bool::of(F::PHASE.denotes().get() && T::PHASE.denotes().get()),
        Bool::of(
            is_constant_family::<F::Quantum>().get() && is_constant_family::<T::Quantum>().get(),
        ),
        Bool::of(matches!(Convertible::<F, T>::STEP, Maybe::Is(_))),
        conversion_is_carried::<F, T>(),
    ]
}

/// The step at slot `a` without forcing the obligation, for the edge sweep,
/// which reads the step at pairs the obligation does not admit.
#[cfg(test)]
const fn unforced_position<F: Format, T: Format>(a: Slot) -> Maybe<Exact> {
    match Convertible::<F, T>::STEP {
        Maybe::Is(step) => Maybe::Is(positioned(step.divisor, placed(step, a.index()))),
        Maybe::Isnt => Maybe::Isnt,
    }
}

#[cfg(test)]
mod tests;
