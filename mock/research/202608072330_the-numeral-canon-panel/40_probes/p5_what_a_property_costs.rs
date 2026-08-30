// p5. What does 35's "state which properties the arithmetic has" cost, once a
// property depends on more than one axis?
//
// p3 implements every property on a single axis value, so a preset inherits it
// for free. That works because absorption, monotonicity and invertibility each
// depend on the overflow policy alone. Not every property does.
//
// 35 section 3.6 measures exact reassociability over 16.7M vectors and the
// result depends on TWO axes jointly:
//
//     unsigned + wrapping      0 failures     reassociable
//     unsigned + saturating    0 failures     reassociable
//     signed   + wrapping      0 failures     reassociable
//     signed   + saturating    11,760,675 of 16,777,216 (70.1%)   NOT
//
// The satisfying set is "everything except one cell", which is a disjunction.
// This probe asks what that costs to state under the forbidden-feature set,
// and whether the cost is a property of this one law or of a whole class.
//
//   arm base    : the conjunctive case (a property that is the AND of per-axis
//                 properties) and the disjunctive case (this one), both stated.
//   arm bad     : a signed saturating strategy handed to a routine requiring
//                 reassociability. MUST be refused.
//   arm negative: the cheap spelling, "holds unless signed and saturating",
//                 attempted with a blanket impl plus an exclusion. Expected to
//                 be refused, and the diagnostic is the finding.
//
//   rustc +nightly-2026-05-28 --edition 2021 p5_what_a_property_costs.rs \
//         --crate-type lib --out-dir out
//   ... --cfg bad        (expected: E0277)
//   ... --cfg negative   (expected: refused; message recorded)

#![allow(dead_code)]

use core::marker::PhantomData;

pub trait Overflow {}
pub struct Wrap;
pub struct Saturate;
impl Overflow for Wrap {}
impl Overflow for Saturate {}

pub trait Sign {}
pub struct Unsigned;
pub struct Signed;
impl Sign for Unsigned {}
impl Sign for Signed {}

pub trait Strategy {
    type Overflow: Overflow;
    type Sign: Sign;
}

pub struct Strat<O, G>(PhantomData<(O, G)>);
impl<O: Overflow, G: Sign> Strategy for Strat<O, G> {
    type Overflow = O;
    type Sign = G;
}

// ---------------------------------------------------------------------------
// The conjunctive case, for contrast. A property that is the AND of per-axis
// properties costs one impl per axis and composes by adding bounds. Adding an
// axis adds one impl.
// ---------------------------------------------------------------------------

pub trait AbsorbingTop {}
impl AbsorbingTop for Saturate {}

pub trait MonotoneAdd {}
impl MonotoneAdd for Saturate {}

/// Conjunctive: two per-axis facts, ANDed at the use site. No product is
/// enumerated anywhere.
pub fn tropical_fold<S>()
where
    S: Strategy,
    S::Overflow: AbsorbingTop + MonotoneAdd,
{
}

// ---------------------------------------------------------------------------
// The disjunctive case. The satisfying set is three of four cells, and there is
// no per-axis fact that implies it: neither Signed nor Saturate alone excludes
// reassociability, only their conjunction does. So the satisfying assignments
// are listed.
// ---------------------------------------------------------------------------

/// Splitting a reduction across lanes or cores changes the association order,
/// so a fold may only be split where the operation is associative.
/// 35 section 3.6, exhaustive at n=8 over 16,777,216 vectors per cell.
#[diagnostic::on_unimplemented(
    message = "this strategy's arithmetic is not exactly reassociable, so a reduction over it may not be split",
    label = "splitting across lanes or cores changes the association order",
    note = "signed saturating addition is the one combination that fails: 11,760,675 of 16,777,216 vectors at n=8 give a different answer under a different split"
)]
pub trait ExactlyReassociable {}

// Three impls, one per satisfying cell. This is the enumeration and it is the
// cost being priced.
impl ExactlyReassociable for Strat<Wrap, Unsigned> {}
impl ExactlyReassociable for Strat<Saturate, Unsigned> {}
impl ExactlyReassociable for Strat<Wrap, Signed> {}
// deliberately no impl for Strat<Saturate, Signed>.

pub fn splittable_fold<S: Strategy + ExactlyReassociable>() {}

pub fn positive_unsigned_saturating() {
    splittable_fold::<Strat<Saturate, Unsigned>>();
}

pub fn positive_signed_wrapping() {
    splittable_fold::<Strat<Wrap, Signed>>();
}

pub fn positive_conjunctive() {
    tropical_fold::<Strat<Saturate, Signed>>();
}

#[cfg(bad)]
pub fn negative_signed_saturating() {
    // The cell 35 measures at 70.1% split-dependence.
    splittable_fold::<Strat<Saturate, Signed>>();
}

// ---------------------------------------------------------------------------
// The cheap spelling that would avoid the enumeration: state the property once
// for every strategy and exclude the one cell. That is negative reasoning, and
// the forbidden-feature set has no route to it: full `specialization` is
// forbidden outright, `negative_impls` does not yet disarm coherence, and the
// only remaining shape is a second blanket impl that overlaps the first.
// ---------------------------------------------------------------------------

#[cfg(negative)]
mod cheap_spelling {
    use super::*;

    pub trait NotBoth {}

    // "Holds for everything..."
    impl<O: Overflow, G: Sign> NotBoth for Strat<O, G> {}

    // "...except this one", which is the overlap the coherence checker refuses.
    impl NotBoth for Strat<Saturate, Signed> {}
}

// ---------------------------------------------------------------------------
// How the cost scales, stated as a relation rather than measured, because it is
// combinatorics rather than an observation:
//
//   a property that is a CONJUNCTION of per-axis facts over k axes costs k
//   impls, and adding an axis adds one.
//
//   a property whose satisfying set is not a product of per-axis sets costs one
//   impl per satisfying assignment, which is up to prod(|A_i|) - 1, and adding
//   an axis multiplies it.
//
// So the reframing is cheap exactly for the properties that factor through the
// axes and expensive for the ones that do not, and which properties those are
// is a measurement rather than a matter of taste. The one non-factoring
// property measured so far is reassociability.
// ---------------------------------------------------------------------------
