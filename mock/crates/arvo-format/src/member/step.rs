//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The exact step over members, in the host's integer where it fits.
//!
//! A slot landed on the grid, and the product, the quotient and the conversion
//! of members, each formed in the host's 128-bit integer where the range leaves
//! room for it.
//!
//! At a zero phase and a constant quantum each of the three sits at a ratio of
//! slot products and powers of the radix. With `u = r^max(e, 0)` and
//! `v = r^max(-e, 0)`: the product of the members at `a` and `b` at `a b u / v`,
//! the quotient of `a` by the divisor at `b` at `a v / (b u)`, and the member of
//! `F` at `a` on `T`'s grid at `a u_F v_T / (v_F u_T)`. Where the largest
//! numerator two members reach and the denominator fit the host's unsigned
//! 128-bit integer and their quotient is below `2^127`, the ratio is formed here,
//! its floor and its residue, in the machine's 64-bit integer where the terms fit
//! there. Elsewhere the slot step's position, formed four 64-bit limbs wide, is
//! taken.
//!
//! Which of the two an instantiation takes is a constant of it, read from the
//! range, the radix and the exponent. The bound is over the range because the
//! operands are members, and every divisor is in the range at a zero phase: a
//! constant is checked against it, `Divisor::member` is admitted only for a range
//! inside it, and `Divisor::of` is refused where the phase is whole. The slot steps
//! take any slot, so the same bound there would need a test of the operands.

use core::marker::PhantomData;

use notko::Maybe;

use super::{Member, MemberOf};
use crate::adapt::{Adaptation, DeclaredSignature};
use crate::apply::{Exact, Remainder, complete_index};
use crate::conversion::{Convertible, converted_position};
use crate::division::{Divisible, Divisor, quotient_position};
use crate::format::Format;
use crate::multiplication::{Multipliable, product_position};
use crate::overflow::Overflow;
use crate::placing::{Powers, powers};
use crate::quantum::is_constant_family;
use crate::slots::{Slot, Slots};
use crate::wide::Wide;
use crate::width::Bool;

/// A ratio of two magnitudes in slot units, and the sign of the ratio. The
/// denominator is above zero.
#[derive(Clone, Copy)]
struct Ratio {
    negative: bool,
    num:      u128,
    den:      u128,
}

impl Ratio {
    /// The exact position at the ratio: the floor toward negative infinity, and
    /// the residue to the slot above it.
    ///
    /// Reached only where the bound holds, so the quotient is below `2^127` and
    /// the floor and the slot under it are in the index. `narrow` says the
    /// bound keeps both terms in 64 bits, a constant of the instantiation, so
    /// the division is the machine's 64-bit one there. The residue is not
    /// reduced: the applied map reads it only against the half and the dither,
    /// and both read the ratio.
    #[inline(always)]
    const fn exact(self, narrow: bool) -> Exact {
        let (whole, rest) = if narrow {
            let (num, den) = (self.num as u64, self.den as u64);
            ((num / den) as i128, (num % den) as u128)
        } else {
            ((self.num / self.den) as i128, self.num % self.den)
        };
        if rest == 0 {
            return Exact::on_grid(Slot::at(if self.negative { -whole } else { whole }));
        }
        // The floor of a negative ratio is one below the negated quotient
        // wherever something is left over, and the residue is then counted from
        // the slot above.
        let (floor, residue) = if !self.negative {
            (whole, rest)
        } else if cfg!(arvo_mutant_member_step) {
            (-whole, self.den - rest)
        } else {
            (-whole - 1, self.den - rest)
        };
        Exact::at(Slot::at(floor), Remainder::of(residue, self.den))
    }
}

/// A magnitude in slot units, or a power of the radix, as the step's constants
/// hold it.
#[derive(Clone, Copy)]
struct Term(u128);

impl Term {
    /// One, the power at a zero exponent.
    const ONE: Self = Self(1);

    /// The magnitude.
    #[inline(always)]
    const fn get(self) -> u128 {
        self.0
    }

    /// The term, or one where it is lost, for a constant read only where the
    /// bound holds and so it is not lost.
    #[inline(always)]
    const fn or_one(term: Maybe<Self>) -> Self {
        match term {
            Maybe::Is(term) => term,
            Maybe::Isnt => Self::ONE,
        }
    }

    /// A power the four limbs hold, as the host's unsigned integer, or nothing
    /// where it is lost or passes `2^128 - 1`.
    #[inline(always)]
    const fn narrowed(power: Maybe<Wide>) -> Maybe<Self> {
        match power {
            Maybe::Is(power) => {
                match power.unsigned() {
                    Maybe::Is(at) => Maybe::Is(Self(at)),
                    Maybe::Isnt => Maybe::Isnt,
                }
            },
            Maybe::Isnt => Maybe::Isnt,
        }
    }
}

/// The product of two magnitudes, or nothing where either is lost or the
/// product passes 128 bits.
#[inline]
const fn times(a: Maybe<Term>, b: Maybe<Term>) -> Maybe<Term> {
    match (a, b) {
        (Maybe::Is(a), Maybe::Is(b)) => {
            match a.get().checked_mul(b.get()) {
                Some(at) => Maybe::Is(Term(at)),
                None => Maybe::Isnt,
            }
        },
        _ => Maybe::Isnt,
    }
}

/// Whether a ratio whose numerator reaches `num` over a denominator of at least
/// `den` is formed here: both are held and their quotient is below `2^127`.
#[inline]
const fn holds(num: Maybe<Term>, den: Maybe<Term>) -> Bool {
    Bool::of(match (num, den) {
        // The mutant admits a quotient up to `2^128`, past the edge at radix two,
        // where the floor no longer fits the index.
        (Maybe::Is(_), Maybe::Is(den)) if cfg!(arvo_mutant_member_step_edge) => den.get() > 0,
        (Maybe::Is(num), Maybe::Is(den)) => den.get() > 0 && num.get() / den.get() < (1u128 << 127),
        _ => false,
    })
}

/// Whether a numerator reaching `num` and a denominator reaching `den` both fit
/// the machine's 64-bit integer.
#[inline]
const fn narrow(num: Maybe<Term>, den: Maybe<Term>) -> Bool {
    Bool::of(match (num, den) {
        (Maybe::Is(num), Maybe::Is(den)) => {
            num.get() <= u64::MAX as u128 && den.get() <= u64::MAX as u128
        },
        _ => false,
    })
}

/// The constants of `F` the ratio reads.
pub(super) struct Sides<F>(PhantomData<F>);

impl<F: Format> Sides<F> {
    /// `v` where the product or the quotient is formed here, and one otherwise.
    const DOWN: Term = Term::or_one(Self::POWERS.1);
    /// Whether the phase is zero and the quantum constant, which the ratio
    /// presumes.
    const PLAIN: Bool = F::PHASE.is_zero().and(is_constant_family::<F::Quantum>());
    /// `u` and `v`, the radix to the two halves of the quantum's exponent, as the
    /// slot steps read them, in the host's unsigned integer.
    const POWERS: (Maybe<Term>, Maybe<Term>) = {
        let Powers {
            up,
            down,
        } = powers::<F>();
        (Term::narrowed(up), Term::narrowed(down))
    };
    /// Whether the product of two members is formed here.
    pub(super) const PRODUCT: Bool = Self::PLAIN.and(holds(
        times(times(Self::REACH, Self::REACH), Self::POWERS.0),
        Self::POWERS.1,
    ));
    /// Whether the product's terms fit 64 bits.
    pub(super) const PRODUCT_NARROW: Bool = narrow(
        times(times(Self::REACH, Self::REACH), Self::POWERS.0),
        Self::POWERS.1,
    );
    /// Whether the quotient of a member by a divisor in the range is formed here:
    /// its largest denominator, the widest member times `u`, is held, and the
    /// ratio is below `2^127`.
    pub(super) const QUOTIENT: Bool = Self::PLAIN
        // The mutant drops the check on the denominator, which above a zero
        // exponent is the only part of the bound that can fail.
        .and(Bool::of(
            cfg!(arvo_mutant_member_step_guard)
                || matches!(times(Self::REACH, Self::POWERS.0), Maybe::Is(_)),
        ))
        .and(holds(times(Self::REACH, Self::POWERS.1), Self::POWERS.0));
    /// Whether the quotient's terms fit 64 bits.
    pub(super) const QUOTIENT_NARROW: Bool = narrow(
        times(Self::REACH, Self::POWERS.1),
        times(Self::REACH, Self::POWERS.0),
    );
    /// The largest magnitude a slot of the range takes.
    const REACH: Maybe<Term> = {
        let lo = <F::Slots as Slots>::MIN.index().unsigned_abs();
        let hi = <F::Slots as Slots>::MAX.index().unsigned_abs();
        Maybe::Is(Term(if lo > hi { lo } else { hi }))
    };
    /// `u` where the product or the quotient is formed here, and one otherwise.
    const UP: Term = Term::or_one(Self::POWERS.0);
}

/// The constants of the crossing from `F` onto `T` the ratio reads.
pub(super) struct Crossing<F, T>(PhantomData<(F, T)>);

impl<F: Format, T: Format> Crossing<F, T> {
    /// `v_F u_T`, the denominator.
    const DIVISOR: Maybe<Term> = times(Sides::<F>::POWERS.1, Sides::<T>::POWERS.0);
    /// Whether the conversion of a member is formed here.
    pub(super) const HOLDS: Bool = Sides::<F>::PLAIN
        .and(Sides::<T>::PLAIN)
        .and(holds(times(Sides::<F>::REACH, Self::SCALE), Self::DIVISOR));
    /// Whether the conversion's terms fit 64 bits.
    pub(super) const NARROW: Bool = narrow(times(Sides::<F>::REACH, Self::SCALE), Self::DIVISOR);
    /// `v_F u_T` where the conversion is formed here, and one otherwise.
    const OVER: Term = Term::or_one(Self::DIVISOR);
    /// `u_F v_T`, what a slot of `F` is scaled by.
    const SCALE: Maybe<Term> = times(Sides::<F>::POWERS.0, Sides::<T>::POWERS.1);
    /// `u_F v_T` where the conversion is formed here, and one otherwise.
    const SCALED: Term = Term::or_one(Self::SCALE);
}

impl<F: Format> Member<F> {
    /// The member `adapted::<S>(Exact::on_grid(slot), _)` gives, for any dither.
    ///
    /// A position on the grid does not round, so only `S`'s policy acts, and a
    /// slot already in the range lands on itself. Total, so nothing is checked,
    /// and formed in the slot index by the completion `adapt` takes there as
    /// well. Refused at build time where the format's phase does not denote, and
    /// not yet admitted where its quantum is not constant.
    #[must_use]
    #[inline]
    pub const fn landed<S: DeclaredSignature<Format = F>>(slot: Slot) -> Self {
        let () = MemberOf::<F>::ADMITTED;
        let () = <F::Slots as Slots>::ADMITTED;
        let policy = <<S::Adaptation as Adaptation>::Overflow as Overflow>::POLICY;
        Self {
            slot:   complete_index(
                policy,
                slot.index(),
                <F::Slots as Slots>::MIN,
                <F::Slots as Slots>::MAX,
            ),
            format: PhantomData,
        }
    }

    /// The exact position of the product of this member and `rhs`.
    ///
    /// The position `product_position` gives for the two slots, formed in the
    /// host's integer where the bound over the range holds. Forces the product's
    /// obligation in both arms, so a product the format does not admit is
    /// refused at build with the slot step's message.
    #[must_use]
    #[inline]
    pub const fn product(self, rhs: Self) -> Exact {
        let () = Multipliable::<F>::ADMITTED;
        if const { Sides::<F>::PRODUCT.get() } {
            let (a, b) = (self.slot.index(), rhs.slot.index());
            Ratio {
                negative: (a < 0) != (b < 0),
                num:      a.unsigned_abs() * b.unsigned_abs() * Sides::<F>::UP.get(),
                den:      Sides::<F>::DOWN.get(),
            }
            .exact(const { Sides::<F>::PRODUCT_NARROW.get() })
        } else {
            product_position::<F>(self.slot, rhs.slot)
        }
    }

    /// The exact position of the quotient of this member by `by`.
    ///
    /// The position `quotient_position` gives, formed in the host's integer
    /// where the bound over the range holds. Forces the quotient's obligation in
    /// both arms.
    #[must_use]
    #[inline]
    pub const fn quotient(self, by: Divisor<F>) -> Exact {
        let () = Divisible::<F>::ADMITTED;
        if const { Sides::<F>::QUOTIENT.get() } {
            let (a, b) = (self.slot.index(), by.slot().index());
            Ratio {
                negative: (a < 0) != (b < 0),
                num:      a.unsigned_abs() * Sides::<F>::DOWN.get(),
                den:      b.unsigned_abs() * Sides::<F>::UP.get(),
            }
            .exact(const { Sides::<F>::QUOTIENT_NARROW.get() })
        } else {
            quotient_position::<F>(self.slot, by)
        }
    }

    /// The exact position of this member on `T`'s grid.
    ///
    /// The position `converted_position` gives, formed in the host's integer
    /// where the bound over the range holds. Forces the conversion's obligation
    /// in both arms.
    #[must_use]
    #[inline]
    pub const fn converted<T: Format>(self) -> Exact {
        let () = Convertible::<F, T>::ADMITTED;
        if const { Crossing::<F, T>::HOLDS.get() } {
            let a = self.slot.index();
            Ratio {
                negative: a < 0,
                num:      a.unsigned_abs() * Crossing::<F, T>::SCALED.get(),
                den:      Crossing::<F, T>::OVER.get(),
            }
            .exact(const { Crossing::<F, T>::NARROW.get() })
        } else {
            converted_position::<F, T>(self.slot)
        }
    }
}

#[cfg(test)]
mod tests;
