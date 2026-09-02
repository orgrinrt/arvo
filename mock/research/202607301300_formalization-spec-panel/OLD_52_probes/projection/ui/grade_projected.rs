//! Probe 3: the published grade projected instead of declared.
//!
//! File 37's transfer rule is right and this probe does not touch it: a
//! regrouping publishes exactly what its law fails to preserve, and tolerance
//! is a transfer rather than a waiver (`40:300-303`). What this probe changes
//! is who writes the published value down.
//!
//! In `37_probes/probe_4_view_as_a_return_type_and_the_transfer.rs` the
//! consumer writes it: `regroup_fold::<0, 0, 1, 4, 0, 1>(xs)`, where the last
//! parameter is a bitmask the caller must compute by hand and the const
//! assertion checks. File 37 states why (`37:452-456`): computing the grade in
//! return position is an expression over a generic const parameter in type
//! position, which is the `generic_const_exprs` wall the droplist records.
//!
//! CLAIM A. That wall is a consequence of the grade being a CONST parameter.
//! With the grade a TYPE, the return position is an ordinary associated-type
//! projection, which needs no unstable feature at all, and the caller declares
//! nothing. This is the same move op ratified for every number in the design at
//! the tenth checkpoint; the grade is the one quantity the design left behind
//! in const-land.
//!
//! CLAIM B. The composition with no law at any view (clamping) becomes an
//! unsatisfied trait bound rather than a const-eval panic, so its message is a
//! designed `#[diagnostic::on_unimplemented]` attached to the exact obligation,
//! and it fires before monomorphisation rather than during const evaluation.
//!
//! CLAIM C. Overstating stays available and stops being accidental. In the
//! const form, overstating is a different digit; here it is an explicit
//! `.weaken()` bounded on the lattice order, so it is greppable.
//!
//! CLAIM D. Interior safety is computed from the numerals rather than passed as
//! a hand-computed headroom integer, using the tower's own `Cmp` (`vu_nat.rs:153`).
//!
//! The fold body here is a stand-in (i32 addition); this probe is about the
//! signature the consumer sees, which is the object under review.
//!
//! EXPECTED: COMPILES CLEAN. Probes 3b and 3c are its committed refusals.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_3_the_grade_is_projected.rs

#![allow(dead_code)]

use core::marker::PhantomData;
use tower::nat::{Cmp, Eq3, Gt, Lt, Nat, Pos, Pz, H, I, O};

// ---------------------------------------------------------------------------
// The key, as types. Every member is one file 37's own const fn already reads.
// ---------------------------------------------------------------------------

pub struct Refuse;
pub struct Clamp;
pub struct ReduceModulo;
pub struct SubstituteZero;

pub struct Unsigned;
pub struct Signed;

// ---------------------------------------------------------------------------
// The grade, as named types rather than a bitmask. Same four points, same
// meaning, and a call site now says which one it is.
// ---------------------------------------------------------------------------

pub trait Grade {
    /// File 37's bitmask, kept as a const so the two encodings are provably
    /// the same object rather than two statements that will decorrelate.
    const BITS: u8;
}

/// The regrouping agrees with the sequential fold on every generator class.
pub struct Faithful;
/// It may refuse where the sequential fold returned, or return where it refused.
pub struct RefusalsTransferred;
/// It may carry a different multiset of quantisation events.
pub struct EventsTransferred;
/// Both.
pub struct BothTransferred;

impl Grade for Faithful {
    const BITS: u8 = 0;
}
impl Grade for RefusalsTransferred {
    const BITS: u8 = 1;
}
impl Grade for EventsTransferred {
    const BITS: u8 = 2;
}
impl Grade for BothTransferred {
    const BITS: u8 = 3;
}

// ---------------------------------------------------------------------------
// CLAIM D: interior safety, computed from the numerals.
//
// Interior safety is `headroom >= arity - 1` (`40:328-336`). Both sides are
// `Nat`s in the ratified encoding, so this is the tower's own `Cmp`, and the
// consumer passes the two numerals it already has rather than an integer it
// worked out on paper.
// ---------------------------------------------------------------------------

pub struct Safe;
pub struct Unsafe;

pub trait Safety {}
impl Safety for Safe {}
impl Safety for Unsafe {}

/// `Headroom` compared against `ArityMinusOne`, both as positive magnitudes.
pub trait InteriorSafety<ArityMinusOne> {
    type Out: Safety;
}

/// A `Gt` or `Eq3` comparison is interior-safe; `Lt` is not.
pub trait SafetyOf {
    type Out: Safety;
}
impl SafetyOf for Gt {
    type Out = Safe;
}
impl SafetyOf for Eq3 {
    type Out = Safe;
}
impl SafetyOf for Lt {
    type Out = Unsafe;
}

impl<Hd: Pos + Cmp<A>, A: Pos> InteriorSafety<A> for Hd
where
    <Hd as Cmp<A>>::Out: SafetyOf,
{
    type Out = <<Hd as Cmp<A>>::Out as SafetyOf>::Out;
}

// ---------------------------------------------------------------------------
// CLAIMS A and B: the law, as an impl-selected projection.
//
// The body is file 37's `add_assoc_view` transcribed. Every arm that returns a
// `Finest(view)` becomes an impl whose `Out` is that view's unpreserved set;
// the `Never` arm becomes the ABSENCE of an impl, so the composition with no
// law at any view is refused by trait solving with a designed message.
// ---------------------------------------------------------------------------

#[diagnostic::on_unimplemented(
    message = "this composition's fold has no associativity law at any view",
    label = "regrouping this fold changes the delivered value",
    note = "no published grade makes the regrouping honest, because the values themselves diverge",
    note = "widen the accumulator until the fold is interior-safe, or do not regroup"
)]
pub trait FoldGrade {
    type Out: Grade;
}

/// Interior-safe, whatever the resolutions are: one quantisation fires, at the
/// root, on a grouping-independent argument, so every component agrees.
impl<Top, Bot, Dom> FoldGrade for (Safe, Top, Bot, Dom) {
    type Out = Faithful;
}

/// Unsigned wrapping at both ends: the event count is the exact total over the
/// modulus, hence grouping-independent (file 37 probe 3, CLAIM A).
impl FoldGrade for (Unsafe, ReduceModulo, ReduceModulo, Unsigned) {
    type Out = Faithful;
}

/// Signed wrapping: reductions cancel in the value and are both counted in the
/// grade, so event multiplicities may disagree (file 37 probe 3, CLAIM C).
impl FoldGrade for (Unsafe, ReduceModulo, ReduceModulo, Signed) {
    type Out = EventsTransferred;
}

/// Refusing at both ends: values agree wherever both groupings return,
/// definedness does not, and there are no events to disagree about. This is
/// `Precise` below interior safety.
impl<Dom> FoldGrade for (Unsafe, Refuse, Refuse, Dom) {
    type Out = RefusalsTransferred;
}

/// Refusing at one end, reducing at the other: both classes live.
impl<Dom> FoldGrade for (Unsafe, Refuse, ReduceModulo, Dom) {
    type Out = BothTransferred;
}
impl<Dom> FoldGrade for (Unsafe, ReduceModulo, Refuse, Dom) {
    type Out = BothTransferred;
}

// No impl for any composition involving `Clamp` or `SubstituteZero` below
// interior safety. Clamping is a retraction: it preserves order and not the
// operation, so the values diverge and no view recovers the law. That absence
// IS the refusal, and probe 3b is it.

// ---------------------------------------------------------------------------
// The combinator. The consumer's whole call is the four facts it already knows
// plus its data. The grade is projected.
// ---------------------------------------------------------------------------

pub struct Folded<G: Grade>(pub i32, PhantomData<G>);

impl Folded<Faithful> {
    /// A result that was never regrouped, so its grade is `Faithful` by
    /// construction rather than by projection. This constructor has to live
    /// here, inside the perimeter, because `Folded`'s marker field is private:
    /// a consumer cannot mint a `Folded<Faithful>` from outside, which is the
    /// point. A sequential-fold combinator is therefore something arvo ships or
    /// something nobody has (probe 5, shape B).
    pub const fn sequential(v: i32) -> Self {
        Folded(v, PhantomData)
    }
}

impl<G: Grade> Folded<G> {
    /// CLAIM C: overstating stays available and is explicit. A `Folded<G>` may
    /// be weakened to any grade at least as permissive, never to a stricter one.
    pub fn weaken<To: Grade>(self) -> Folded<To>
    where
        G: WeakerThan<To>,
    {
        Folded(self.0, PhantomData)
    }
}

/// The lattice order on grades, as a bound: `Self`'s published classes are a
/// subset of `To`'s.
pub trait WeakerThan<To> {}
impl WeakerThan<Faithful> for Faithful {}
impl WeakerThan<RefusalsTransferred> for Faithful {}
impl WeakerThan<EventsTransferred> for Faithful {}
impl WeakerThan<BothTransferred> for Faithful {}
impl WeakerThan<RefusalsTransferred> for RefusalsTransferred {}
impl WeakerThan<BothTransferred> for RefusalsTransferred {}
impl WeakerThan<EventsTransferred> for EventsTransferred {}
impl WeakerThan<BothTransferred> for EventsTransferred {}
impl WeakerThan<BothTransferred> for BothTransferred {}

/// A regrouping fold. `Hd` is the accumulator's headroom and `Am1` the arity
/// less one, both numerals the consumer already declared; `Top`/`Bot` are the
/// numeral's own resolutions and `Dom` its sign domain, all three of which the
/// numeral type carries in the real design and are spelled out here because
/// this probe has no `Number<N, S>` to read them off.
pub const fn regroup_fold<Top, Bot, Dom, Hd, Am1>(
    xs: &[i32],
) -> Folded<<(<Hd as InteriorSafety<Am1>>::Out, Top, Bot, Dom) as FoldGrade>::Out>
where
    Hd: Pos + InteriorSafety<Am1>,
    Am1: Pos,
    (<Hd as InteriorSafety<Am1>>::Out, Top, Bot, Dom): FoldGrade,
{
    let mut acc = 0i32;
    let mut i = 0;
    while i < xs.len() {
        acc += xs[i];
        i += 1;
    }
    Folded(acc, PhantomData)
}

// ---------------------------------------------------------------------------
// Call sites: file 37's own five compositions, with nothing declared.
// ---------------------------------------------------------------------------

type Arity4Minus1 = I<H>; // 3
type NoHeadroom = H; // 1, so 1 < 3: not interior-safe
type AmpleHeadroom = O<O<H>>; // 4, so 4 >= 3: interior-safe

pub fn unsigned_wrap(xs: &[i32]) -> Folded<Faithful> {
    regroup_fold::<ReduceModulo, ReduceModulo, Unsigned, NoHeadroom, Arity4Minus1>(xs)
}

pub fn signed_wrap(xs: &[i32]) -> Folded<EventsTransferred> {
    regroup_fold::<ReduceModulo, ReduceModulo, Signed, NoHeadroom, Arity4Minus1>(xs)
}

pub fn precise_below(xs: &[i32]) -> Folded<RefusalsTransferred> {
    regroup_fold::<Refuse, Refuse, Signed, NoHeadroom, Arity4Minus1>(xs)
}

pub fn precise_safe(xs: &[i32]) -> Folded<Faithful> {
    regroup_fold::<Refuse, Refuse, Signed, AmpleHeadroom, Arity4Minus1>(xs)
}

pub fn mixed(xs: &[i32]) -> Folded<BothTransferred> {
    regroup_fold::<Refuse, ReduceModulo, Signed, NoHeadroom, Arity4Minus1>(xs)
}

// ---------------------------------------------------------------------------
// The consumer contract, unchanged from file 37: an ordinary type.
// ---------------------------------------------------------------------------

pub fn needs_faithful_definedness(f: Folded<Faithful>) -> i32 {
    f.0
}

pub fn ok_at_interior_safety(xs: &[i32]) -> i32 {
    needs_faithful_definedness(precise_safe(xs))
}
pub fn ok_unsigned(xs: &[i32]) -> i32 {
    needs_faithful_definedness(unsigned_wrap(xs))
}

/// And the explicit overstatement, which a consumer writes when it genuinely
/// wants one signature for several folds.
pub fn deliberately_weakened(xs: &[i32]) -> Folded<BothTransferred> {
    precise_below(xs).weaken::<BothTransferred>()
}

// The two encodings of the grade agree, so the named types and file 37's
// bitmask are one object.
const _: () = assert!(<Faithful as Grade>::BITS == 0);
const _: () = assert!(<RefusalsTransferred as Grade>::BITS == 1);
const _: () = assert!(<EventsTransferred as Grade>::BITS == 2);
const _: () = assert!(<BothTransferred as Grade>::BITS == 3);

// The safety computation, checked against the numerals it reads.
const _: () = assert!(<Pz<AmpleHeadroom> as Nat>::VAL == 4);
const _: () = assert!(<Pz<Arity4Minus1> as Nat>::VAL == 3);
const _: () = assert!(<Pz<NoHeadroom> as Nat>::VAL == 1);
