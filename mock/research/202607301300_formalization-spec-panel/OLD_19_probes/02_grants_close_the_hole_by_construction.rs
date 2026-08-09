//! PROBE 2: the fix for probe 1's hole, built, and rebuilt twice more after
//! each attempt was itself found broken by compiling it. See the trailing
//! note for the full three-attempt history; it is kept because this dive's
//! own discipline is to keep the audit trail, not to silently overwrite a
//! wrong probe with a right one.
//!
//! Leroy's principle, applied one axis over from where he built it:
//! "either prove the connection, or make the two definitions ONE definition
//! so there is no connection to prove" (`10_leroy...md:70-73`). Stop letting
//! a liberty be declared twice (once as data a body may ignore, once as what
//! a body does). Declare it once, as an associated truth marker on the
//! grade, and derive a body's ability to exist AND the view relation from
//! that one declaration, through an UNCONDITIONAL blanket that computes a
//! total answer, refused at the consuming bound rather than at the impl.
//!
//! Build:
//!   rustc -O 02_grants_close_the_hole_by_construction.rs -o p2 && ./p2
//!   rustc --cfg under_claim 02_grants_close_the_hole_by_construction.rs      # expect error
//!   rustc --cfg bad_grant_attempt 02_grants_close_the_hole_by_construction.rs # expect E0119

#![allow(dead_code)]

// ---- the truth-marker family this dive already uses (07/09/10/17/18) ------

pub struct True;
pub struct False;
pub trait TruthMarker {
    const VALUE: bool;
}
impl TruthMarker for True {
    const VALUE: bool = true;
}
impl TruthMarker for False {
    const VALUE: bool = false;
}
/// Only `True` proves. `False: IsTrue` deliberately does not exist, which is
/// where a refusal comes from in this shape: never from a missing impl of
/// the relation trait itself, always from a missing `IsTrue` at the
/// consuming bound.
pub trait IsTrue {}
impl IsTrue for True {}

/// The TOTAL implication table, both rows, as an associated-type function
/// rather than a partial relation. This is what makes the blanket below
/// unconditional: every `(Source, Target)` pair has an answer.
pub trait ImpliesOutput<Target> {
    type Out: TruthMarker;
}
impl ImpliesOutput<True> for True {
    type Out = True;
}
impl ImpliesOutput<False> for True {
    type Out = False;
}
impl ImpliesOutput<True> for False {
    type Out = True;
}
impl ImpliesOutput<False> for False {
    type Out = True;
}

// ---- (1) ONE declaration point per grade per liberty -----------------------

pub struct Reassoc; // the one liberty this probe models; a real design has four

pub struct Strict;
pub struct Relaxed;

/// A grade states, once, per liberty, whether it grants it. This is the
/// single authored fact. Nothing else in the file restates it.
pub trait CGrade {
    type ReassocGrant: TruthMarker;
    const NAME: &'static str;
}
impl CGrade for Strict {
    type ReassocGrant = False;
    const NAME: &'static str = "Strict";
}
impl CGrade for Relaxed {
    type ReassocGrant = True;
    const NAME: &'static str = "Relaxed";
}

/// Whether a grade grants a liberty is a derived consequence of the one
/// declaration above, consumed through `IsTrue` at the body's own bound.
/// There is no second, independently authored `Grants` impl for a body to
/// check against a stale copy of.
pub trait Grants<Lib> {
    type Ok: TruthMarker;
}
impl<L: CGrade> Grants<Reassoc> for L {
    type Ok = L::ReassocGrant;
}

// ---- (2) a liberty-gated body can now ONLY exist where the marker holds ----

fn sum4_relaxed<L>(xs: [f64; 4]) -> f64
where
    L: Grants<Reassoc>,
    L::Ok: IsTrue,
{
    (xs[0] + xs[2]) + (xs[1] + xs[3])
}

fn sum4_strict(xs: [f64; 4]) -> f64 {
    ((xs[0] + xs[1]) + xs[2]) + xs[3]
}

#[cfg(under_claim)]
fn reproduce_probe_1_mistake(xs: [f64; 4]) -> f64 {
    // probe 1's under-claiming case, honestly: regroup under Strict, which
    // declares `ReassocGrant = False`. `Grants<Reassoc>::Ok` for `Strict`
    // computes `False`, and `False: IsTrue` does not exist, so the bound at
    // `sum4_relaxed` is unsatisfiable for this instantiation.
    sum4_relaxed::<Strict>(xs)
}

// ---- (3) the view relation, as a TOTAL, UNCONDITIONAL blanket -------------

/// `A` viewed at `G` is the implication reading of the same `ReassocGrant`
/// declaration (folded, in a real design, over the closed liberty set the
/// way `AddAssoc`'s own derivation folds over its lemma members,
/// `14_dolan...md:270-296`), computed for EVERY `(A, G)` pair rather than
/// conditioned per pair. The blanket below has no `where` clause narrowing
/// which pairs it covers, which is exactly what closes the hole probe 2's
/// second attempt left open (see the trailing note): an unconditional
/// blanket leaves no gap for a hand-authored concrete impl to fill, so
/// authoring `impl ViewOf<Strict> for Relaxed {}` collides with it directly.
pub trait ViewOf<G: CGrade>: CGrade {
    type Ok: TruthMarker;
}
impl<A: CGrade, G: CGrade> ViewOf<G> for A
where
    A::ReassocGrant: ImpliesOutput<G::ReassocGrant>,
{
    type Ok = <A::ReassocGrant as ImpliesOutput<G::ReassocGrant>>::Out;
}

#[cfg(bad_grant_attempt)]
impl ViewOf<Strict> for Relaxed {
    type Ok = True;
}
// E0119, conflicting implementations: the blanket above ALREADY answers
// `Relaxed` viewed at `Strict` (with `Ok = False`), for every substitution,
// with no `where` clause excluding this one. There is no gap left to author
// a second, wrong answer into.

fn licensed<A, G>() -> bool
where
    A: CGrade + ViewOf<G>,
    G: CGrade,
    <A as ViewOf<G>>::Ok: IsTrue,
{
    true
}

fn main() {
    let xs = [1.0e16f64, -1.0e16, 1.0, 1.0];
    println!("(1) ONE declaration per grade: Strict::ReassocGrant = False, Relaxed::ReassocGrant = True.");
    println!();
    println!(
        "(2) sum4_relaxed requires Grants<Reassoc>::Ok: IsTrue, derived from that one declaration:"
    );
    println!(
        "    sum4_relaxed::<Relaxed>({:?}) = {}",
        xs,
        sum4_relaxed::<Relaxed>(xs)
    );
    println!("    sum4_strict({:?}) = {}", xs, sum4_strict(xs));
    println!("    sum4_relaxed::<Strict>(..) does not exist as a callable instantiation;");
    println!("    build with --cfg under_claim to see the unsatisfied-bound error that proves it.");
    println!();
    println!(
        "(3) Relaxed viewed at Relaxed: licensed = {}",
        licensed::<Relaxed, Relaxed>()
    );
    println!("    Relaxed viewed at Strict has Ok = False and cannot satisfy IsTrue.");
    println!("    build with --cfg bad_grant_attempt for the E0119 that shows the blanket");
    println!("    leaves no room for a hand-authored lie, unlike the first two attempts.");
}

// -----------------------------------------------------------------------
// Audit trail: three attempts, two of them wrong, each wrong in a
// different way, each found by compiling it rather than by re-reading it.
// This dive's own record (07 -> 09 -> 10 for the recovery-map witness) said
// to expect exactly this, and it happened again here, one axis over.
//
// ATTEMPT 1 (not committed). `ViewC` as three hand-authored concrete impls,
// `impl ViewC<Strict> for Strict {}` and so on, exactly mirroring
// `17_probes/06`'s own shape, with `Grants<Lib>` as a separate, also
// hand-authored marker-trait impl set. This reproduced probe 1's hole
// exactly one level down: nothing connected the `Grants` impls to the
// `ViewC` impls, so a correct-looking set of three `ViewC` impls and a
// correct-looking set of four `Grants` impls could each individually look
// right while silently disagreeing with each other, and a fourth `ViewC`
// impl (`Relaxed: ViewC<Strict>`) could simply be hand-added, compiling
// clean, because nothing about "three correct impls were written by hand"
// stops a fourth incorrect one from being written by hand the same way.
//
// ATTEMPT 2 (not committed). Collapsed `Grants` and `ViewC` onto ONE shared
// declaration (`ReassocGrant`) and derived `ViewC<G> for A` as a blanket
// CONDITIONED on `A::ReassocGrant: Implies<G::ReassocGrant>`, where
// `Implies` was a PARTIAL relation (three impls out of four rows; the
// `True: Implies<False>` row deliberately absent, mirroring probe 1's own
// `ViewC` shape one layer down). The claim was that this made the lie
// unwritable, because the blanket "already covers every pair". It does not.
// Verified: `impl ViewC<Strict> for Relaxed {}` alongside that blanket
// compiles with no error at all (`rustc --cfg bad_grant_attempt`, exit 0,
// no diagnostic). The reason is a real and generalisable Rust coherence
// fact, not a mistake specific to this probe: a blanket impl CONDITIONED on
// a where-clause only covers the substitutions where the where-clause is
// satisfied; rustc's coherence checker can and does determine, for fully
// concrete (non-generic) associated-type projections, that a where-clause
// is UNSATISFIED for a given pair, and it then treats that pair as outside
// the blanket's coverage, leaving room for a hand-authored concrete impl to
// fill exactly that gap. A conditional blanket does not close a design off
// from a hand-authored lie; it only automates the honest cases and leaves
// every case its own condition excludes exactly as open as an unauthored
// trait was.
//
// ATTEMPT 3, this file. Made the blanket UNCONDITIONAL: it computes a total
// answer (`Ok: TruthMarker`, either `True` or `False`) for every pair with
// no `where` clause narrowing coverage, and moved the refusal to the
// CONSUMING bound (`Ok: IsTrue`) rather than to the relation trait's own
// impl set. Verified: the same hand-authored `impl ViewOf<Strict> for
// Relaxed {}` now fails with E0119, because the blanket already answers
// every `(A, G)` pair and there is no gap. This is the same shape Thread C
// itself already uses for `AddAssoc` (a computed truth value, consumed
// through `Proves<C>`/`IsTrue`, never a partial relation with impls left
// out on purpose): the lesson generalises past this one probe. ANY relation
// in this design expressed as a blanket impl CONDITIONED on a where-clause,
// with the negative case represented by the where-clause's absence rather
// than by a computed `False`, is vulnerable to exactly attempt 2's hole,
// and the fix is the same every time: compute a total answer, refuse at the
// consumer.
