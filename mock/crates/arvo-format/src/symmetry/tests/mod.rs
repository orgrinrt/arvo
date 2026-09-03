//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The laws of the two symmetries, derived from the map rather than read back.
//!
//! Every assertion here runs `adapt` and looks at what came back. The
//! classification a predicate reads is derived by walking the map over domains
//! that differ in one restriction at a time, so a wrong classification fails here
//! rather than being confirmed by a predicate that restates it. That is what
//! separates these arms from the four predicates this crate deleted for reaching
//! a declaration and stopping.
//!
//! The cross is every mode, every shipped policy, three range kinds and every
//! combination of what a reach can restrict. Three range kinds is the whole
//! classification the predicates read, because they ask only whether the lowest
//! admitted slot is below zero and whether it is the negation of the highest: one
//! range reaching negatives asymmetrically, one reaching none, and one symmetric
//! about zero. No shipped format is symmetric, so this module declares one, which
//! is also what an outside implementor does.

use crate::adapt::{Adapt, DeclaredSignature, Signature};
use crate::ambient::BinaryRationals;
use crate::apply::{Dither, Exact, Fraction, adapt};
use crate::format::{Format, Phase};
use crate::overflow::{Clamp, Policy, Saturate, Wrap};
use crate::points::{Integer, UFixed};
use crate::quantum::Constant;
use crate::rounding::{ALL_MODES, Ceil, Floor, HalfEven, HalfUp, Mode, Stochastic, TowardZero};
use crate::slots::{Slot, Slots};
use crate::symmetry::{
    Reach,
    adaptation_reflects,
    adaptation_relocates,
    completion_is_reflection_equivariant,
    completion_is_translation_homomorphic,
};
use crate::width::{Bool, Width};

mod the_classification;
mod the_cross;
mod the_reach;

/// A slot range symmetric about zero, which no shipped format has.
///
/// The shipped signed ranges are two's complement, so the lowest slot is one
/// below the negation of the highest. The completion's reflection region is a
/// question about a symmetric range, so the range is declared here, through the
/// open trait and against the same obligation any implementor meets.
struct SymmetricSlots;

impl Slots for SymmetricSlots {
    const MAX: Slot = Slot::at(3);
    const MIN: Slot = Slot::at(-3);
    const WIDTH: Width = Width::bits(3);
}

/// A format over that range, otherwise an integer.
struct SymmetricInteger;

impl Format for SymmetricInteger {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = SymmetricSlots;

    const PHASE: Phase = Phase::ZERO;
}

/// Which range a cell is measured over.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Which {
    /// `-4` to `3`, two's complement, reaching negatives and not symmetric.
    Signed,
    /// `0` to `7`, reaching no negative slot.
    Unsigned,
    /// `-3` to `3`, symmetric about zero.
    Symmetric,
    /// `-2^39` to `2^39 - 1`. Wide enough that the bands below never leave it,
    /// which is what isolates the rounding region from the completion.
    Wide,
}

/// The three the cross runs over. `Wide` is not one of them: it exists to
/// isolate a region rather than to be a range a law is stated at.
const RANGES: [Which; 3] = [Which::Signed, Which::Unsigned, Which::Symmetric];

/// Something to do at a declared signature the cell names.
///
/// One dispatch over the matrix rather than one per caller. The alternative is
/// the same four-by-six-by-three match written out five times, which is the pile
/// this crate refuses everywhere else.
trait PerSignature {
    /// What it answers with.
    type Out;

    /// Do it at one signature.
    fn run<S: DeclaredSignature>(&self) -> Self::Out;
}

/// The dispatch. Every arm is a real instantiation, so what comes back is what a
/// consumer declaring that signature gets.
fn at<P: PerSignature>(which: Which, mode: Mode, policy: Policy, what: &P) -> P::Out {
    macro_rules! at_policy {
        ($fmt:ty, $md:ty) => {
            match policy {
                Policy::Wrap => what.run::<Signature<$fmt, Adapt<$md, Wrap>>>(),
                Policy::Saturate => what.run::<Signature<$fmt, Adapt<$md, Saturate>>>(),
                Policy::Clamp => what.run::<Signature<$fmt, Adapt<$md, Clamp>>>(),
            }
        };
    }
    macro_rules! at_mode {
        ($fmt:ty) => {
            match mode {
                Mode::TowardZero => at_policy!($fmt, TowardZero),
                Mode::Floor => at_policy!($fmt, Floor),
                Mode::Ceil => at_policy!($fmt, Ceil),
                Mode::HalfUp => at_policy!($fmt, HalfUp),
                Mode::HalfEven => at_policy!($fmt, HalfEven),
                Mode::Stochastic => at_policy!($fmt, Stochastic),
            }
        };
    }
    match which {
        Which::Signed => at_mode!(Integer<3>),
        Which::Unsigned => at_mode!(UFixed<3, 0>),
        Which::Symmetric => at_mode!(SymmetricInteger),
        Which::Wide => at_mode!(Integer<40>),
    }
}

struct Adapting {
    exact:  Exact,
    dither: Dither,
}

impl PerSignature for Adapting {
    type Out = Slot;

    fn run<S: DeclaredSignature>(&self) -> Slot {
        adapt::<S>(self.exact, self.dither)
    }
}

struct Relocates(Reach);

impl PerSignature for Relocates {
    type Out = Bool;

    fn run<S: DeclaredSignature>(&self) -> Bool {
        adaptation_relocates::<S>(self.0)
    }
}

struct Reflects(Reach);

impl PerSignature for Reflects {
    type Out = Bool;

    fn run<S: DeclaredSignature>(&self) -> Bool {
        adaptation_reflects::<S>(self.0)
    }
}

struct CompletionRelocates(Reach);

impl PerSignature for CompletionRelocates {
    type Out = Bool;

    fn run<S: DeclaredSignature>(&self) -> Bool {
        completion_is_translation_homomorphic::<S>(self.0)
    }
}

struct CompletionReflects(Reach);

impl PerSignature for CompletionReflects {
    type Out = Bool;

    fn run<S: DeclaredSignature>(&self) -> Bool {
        completion_is_reflection_equivariant::<S>(self.0)
    }
}

/// The whole map at one cell.
fn adapt_at(which: Which, mode: Mode, policy: Policy, exact: Exact, dither: Dither) -> Slot {
    at(which, mode, policy, &Adapting {
        exact,
        dither,
    })
}

/// The bounds of a range, read off the format rather than written twice.
fn bounds(which: Which) -> (Slot, Slot) {
    fn of<S: Slots>() -> (Slot, Slot) {
        (S::MIN, S::MAX)
    }
    match which {
        Which::Signed => of::<<Integer<3> as Format>::Slots>(),
        Which::Unsigned => of::<<UFixed<3, 0> as Format>::Slots>(),
        Which::Symmetric => of::<<SymmetricInteger as Format>::Slots>(),
        Which::Wide => of::<<Integer<40> as Format>::Slots>(),
    }
}

/// The residues a sweep walks: on the grid, both sides of the midpoint, the
/// midpoint itself, and a denominator that cannot reach one.
///
/// A sweep over odd denominators alone would report every mode equivariant and
/// every arm in it would look reasonable, which is what the tie count in the
/// control is for.
///
/// A function rather than an item constant, for the reason the ratio
/// coordinate's suite gives: a const here is a coordinate spelled in the host's
/// own type, which the contract lint refuses in the one crate that is otherwise
/// allowed to name one.
fn residues() -> impl Iterator<Item = (i64, i64)> {
    [(0, 1), (1, 8), (1, 4), (1, 2), (3, 4), (7, 8), (1, 3)].into_iter()
}

/// The dither every arm uses.
///
/// Fixed, because relocation is a question about a function and the stochastic
/// mode is one only once its decision is pinned. One arm varies it and requires
/// the verdict not to move.
fn the_dither() -> Dither {
    dither(1, 2)
}

fn position(slot: i64, num: i64, den: i64) -> Exact {
    Exact::between(Slot::at(slot), Fraction::of(num, den))
}

fn dither(num: i64, den: i64) -> Dither {
    Dither::at(Fraction::of(num, den))
}

fn is_tie(num: i64, den: i64) -> bool {
    num * 2 == den
}

/// What a cell restricts, which is what a reach carries.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Restriction {
    negative_positions:    bool,
    negative_translations: bool,
    ties:                  bool,
}

const RESTRICTIONS: [Restriction; 8] = [
    Restriction {
        negative_positions:    true,
        negative_translations: true,
        ties:                  true,
    },
    Restriction {
        negative_positions:    true,
        negative_translations: true,
        ties:                  false,
    },
    Restriction {
        negative_positions:    true,
        negative_translations: false,
        ties:                  true,
    },
    Restriction {
        negative_positions:    true,
        negative_translations: false,
        ties:                  false,
    },
    Restriction {
        negative_positions:    false,
        negative_translations: true,
        ties:                  true,
    },
    Restriction {
        negative_positions:    false,
        negative_translations: true,
        ties:                  false,
    },
    Restriction {
        negative_positions:    false,
        negative_translations: false,
        ties:                  true,
    },
    Restriction {
        negative_positions:    false,
        negative_translations: false,
        ties:                  false,
    },
];

/// The reach one cell describes.
///
/// The band runs one span past each end of the range so a value reaches the
/// completion from outside on both sides, and the translations are the
/// representable slots, which is what a translation by a representable value is.
fn cell_reach(which: Which, r: Restriction) -> Reach {
    let (lo, hi) = bounds(which);
    let span = hi.index() - lo.index() + 1;
    let position_low = if r.negative_positions {
        lo.index() - span
    } else {
        (lo.index() - span).max(0)
    };
    let translation_low = if r.negative_translations { lo.index() } else { lo.index().max(0) };
    let reach = Reach::of(Slot::at(position_low), Slot::at(hi.index() + span))
        .translated_by(Slot::at(translation_low), hi);
    if r.ties { reach } else { reach.without_ties() }
}

/// What one cell's sweep found.
struct Cell {
    law:                bool,
    triples:            u64,
    excursions:         u64,
    ties:               u64,
    negative_positions: u64,
}

/// The relocation law over one cell, measured.
fn relocation_over(which: Which, mode: Mode, policy: Policy, r: Restriction) -> Cell {
    let reach = cell_reach(which, r);
    let (lo, hi) = bounds(which);
    let d = the_dither();
    let mut out = Cell {
        law:                true,
        triples:            0,
        excursions:         0,
        ties:               0,
        negative_positions: 0,
    };
    for (num, den) in residues() {
        if !r.ties && is_tie(num, den) {
            continue;
        }
        for n in reach.positions_low().index() ..= reach.positions_high().index() {
            let here = adapt_at(which, mode, policy, position(n, num, den), d);
            if n < lo.index() || n > hi.index() {
                out.excursions += 1;
            }
            if n < 0 {
                out.negative_positions += 1;
            }
            if is_tie(num, den) {
                out.ties += 1;
            }
            for c in reach.translations_low().index() ..= reach.translations_high().index() {
                out.triples += 1;
                let direct = adapt_at(which, mode, policy, position(n + c, num, den), d);
                let staged = adapt_at(
                    which,
                    mode,
                    policy,
                    Exact::on_grid(Slot::at(here.index() + c)),
                    d,
                );
                if direct != staged {
                    out.law = false;
                }
            }
        }
    }
    out
}

/// The reflection law over one cell, measured.
fn reflection_over(which: Which, mode: Mode, policy: Policy) -> (bool, u64) {
    let (lo, hi) = bounds(which);
    let span = hi.index() - lo.index() + 1;
    let d = the_dither();
    let mut held = true;
    let mut walked = 0u64;
    for (num, den) in residues() {
        for n in (lo.index() - span) ..= (hi.index() + span) {
            walked += 1;
            let here = adapt_at(which, mode, policy, position(n, num, den), d);
            let direct = adapt_at(which, mode, policy, position(-n, -num, den), d);
            let staged = adapt_at(
                which,
                mode,
                policy,
                Exact::on_grid(Slot::at(-here.index())),
                d,
            );
            if direct != staged {
                held = false;
            }
        }
    }
    (held, walked)
}

/// The reach the reflection cross describes, which carries no translation.
fn reflection_reach(which: Which) -> Reach {
    let (lo, hi) = bounds(which);
    let span = hi.index() - lo.index() + 1;
    Reach::of(Slot::at(lo.index() - span), Slot::at(hi.index() + span))
}

// The controls that say what a yes is a claim about, and the reach coordinate's
// own arms, are in `the_reach`.
