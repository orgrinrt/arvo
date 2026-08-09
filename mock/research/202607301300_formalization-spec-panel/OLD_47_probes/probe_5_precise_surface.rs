//! Probe 5: `Precise`'s combinator surface, written as the consumer's code
//! under each candidate answer.
//!
//! The open item (`40:632-637`): what `Precise` preserves is settled at every
//! accumulator width, and what is not settled is whether the shipped surface
//! offers only the definedness-faithful form, or offers the published-grade
//! form as well and lets the caller's type decide.
//!
//! Three shapes are available and the third is not, which is worth compiling
//! rather than asserting:
//!
//!   A. One combinator. It regroups, it publishes, and a consumer needing
//!      faithful definedness widens the accumulator until it is interior-safe.
//!      There is no other door.
//!   B. Two combinators, one regrouping and one sequential, the design's own
//!      `fold` / `fold_compensated` idiom applied a second time (`40:199-201`).
//!   C. One combinator with the grouping as a defaulted type parameter, so the
//!      common path stays `fold(xs)`. NOT AVAILABLE: a free function cannot
//!      default a generic parameter, compiled below.
//!
//! CLAIM A. Shape C does not exist in this language. `pub fn fold<G = Regrouped>`
//! is refused (`error: defaults for generic parameters are not allowed here`,
//! future-incompatible lint, issue #36887), and the same is true of an inherent
//! method. So the choice really is between A and B, and any proposal to keep
//! the ergonomic default has to pick B.
//!
//! CLAIM B. Under shape A a consumer who cannot widen has no expressible
//! program. Compiled: the accumulator sufficiency for a three-million-element
//! `Precise` fold over a 64-bit numeral wants 86 digits, and the consumer's own
//! storage numeral is what it is.
//!
//! CLAIM C. Under shape B the caller's type does decide, with no new mechanism:
//! the consumer writes `Folded<Faithful>` in their own signature and exactly one
//! of the two combinators typechecks.
//!
//! EXPECTED: COMPILES CLEAN (shape C's refusal is in probe 5b, committed
//! refusing).
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_5_precise_surface.rs

#![allow(dead_code)]

#[path = "probe_3_the_grade_is_projected.rs"]
mod mechanism;

use mechanism::{regroup_fold, Faithful, Folded, Grade, RefusalsTransferred, Refuse, Signed};
use tower::nat::{Nat, Pos, Pz, H, O};

// ---------------------------------------------------------------------------
// The workload. A column of 3,000,000 samples, folded. Two accumulators: one
// wide enough for interior safety, one not.
// ---------------------------------------------------------------------------

/// arity - 1, rounded up to the nearest magnitude that fits in a small probe:
/// what matters is that it exceeds the headroom in the second case.
type Arity = O<O<O<O<H>>>>; // 16
type NarrowHeadroom = O<H>; // 2
type WideHeadroom = O<O<O<O<O<H>>>>>; // 32

const _: () = assert!(<Pz<Arity> as Nat>::VAL == 16);
const _: () = assert!(<Pz<NarrowHeadroom> as Nat>::VAL == 2);
const _: () = assert!(<Pz<WideHeadroom> as Nat>::VAL == 32);

// ---------------------------------------------------------------------------
// SHAPE A: one combinator, and the widening is the only remedy.
// ---------------------------------------------------------------------------

pub mod shape_a {
    use super::*;

    /// The consumer whose contract needs definedness faithful to the sequential
    /// fold. This is the ordinary case: a monitoring path that must report a
    /// refusal exactly when the sample really is out of range, not when a
    /// grouping happened to overflow an interior accumulator.
    pub fn alarm_threshold(f: Folded<Faithful>) -> i32 {
        f.0
    }

    /// With a wide enough accumulator, the one combinator delivers it.
    pub fn wide_enough(xs: &[i32]) -> i32 {
        alarm_threshold(regroup_fold::<Refuse, Refuse, Signed, WideHeadroom, Arity>(
            xs,
        ))
    }

    /// Without one, the consumer's only move is to change their storage. The
    /// call that would express "do it sequentially, I will pay for it" is not
    /// in the surface at all: see `probe_5c`, committed refusing.
    pub fn not_wide_enough(xs: &[i32]) -> Folded<RefusalsTransferred> {
        regroup_fold::<Refuse, Refuse, Signed, NarrowHeadroom, Arity>(xs)
    }
}

// ---------------------------------------------------------------------------
// SHAPE B: two combinators. The second one is named for what it costs, not for
// what it delivers, because the property is already in the return type and the
// price is not visible anywhere else.
// ---------------------------------------------------------------------------

pub mod shape_b {
    use super::*;

    /// The regrouping fold, unchanged from probe 3. Vectorises, splits across
    /// morsels, publishes what its law does not preserve.
    pub use mechanism::regroup_fold as fold;

    /// The sequential fold. One accumulator, one order, no regrouping, so no
    /// law is invoked and nothing is transferred: it IS the sequential fold the
    /// transfer rule measures everything else against.
    ///
    /// The name says "sequential" rather than "faithful" on purpose. Its
    /// faithfulness is already in the return type, where the compiler enforces
    /// it; its cost is not stated anywhere else, and the cost is the reason a
    /// reviewer should stop on this call site.
    pub fn fold_sequential<Top, Bot, Dom, Hd, Am1>(xs: &[i32]) -> Folded<Faithful>
    where
        Hd: Pos,
        Am1: Pos,
    {
        let mut acc = 0i32;
        let mut i = 0;
        while i < xs.len() {
            acc += xs[i];
            i += 1;
        }
        Folded::sequential(acc)
    }

    /// CLAIM C: the caller's type decides, and exactly one combinator
    /// typechecks against it. Nothing new is introduced to make this work.
    pub fn alarm_threshold(f: Folded<Faithful>) -> i32 {
        f.0
    }

    /// The consumer who can widen takes the fast door.
    pub fn wide_enough(xs: &[i32]) -> i32 {
        alarm_threshold(fold::<Refuse, Refuse, Signed, WideHeadroom, Arity>(xs))
    }

    /// The consumer who cannot widen still has a program, and its cost is
    /// legible at the call site.
    pub fn cannot_widen(xs: &[i32]) -> i32 {
        alarm_threshold(fold_sequential::<
            Refuse,
            Refuse,
            Signed,
            NarrowHeadroom,
            Arity,
        >(xs))
    }

    /// And the consumer who wants the fast one and can live with the transfer
    /// says so in their own signature, which is the whole of the contract.
    pub fn tolerates_transfer(xs: &[i32]) -> Folded<RefusalsTransferred> {
        fold::<Refuse, Refuse, Signed, NarrowHeadroom, Arity>(xs)
    }
}

// The `Folded<Faithful>::sequential` constructor lives inside the mechanism
// (`probe_3:...`), not here, because `Folded`'s marker field is private. A
// consumer cannot mint a faithful result from outside the perimeter, so shape
// B is something arvo ships or something nobody has.

const _: () = assert!(<Faithful as Grade>::BITS == 0);
