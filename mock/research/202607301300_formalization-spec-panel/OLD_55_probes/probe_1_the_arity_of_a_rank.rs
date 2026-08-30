//! Probe 1: what is the arity of `upward_rank`, and can the design read it?
//!
//! `arvo-graph`'s `upward_rank` (`mock/crates/arvo-graph/src/rank.rs:34-88`)
//! accumulates `rank[v] = weight[v] + max(rank[succ])` over a DAG. Expanded,
//! `rank[v]` is a SUM of weights along one root-to-`v` path. So it is a fold,
//! its operation is `+`, and its arity is the path node count.
//!
//! The design's fold surface publishes a grade projected from interior safety
//! (`49:464-473`), and interior safety is `Headroom >= Arity - 1`
//! (`47_probes/probe_3:93-99`), a TYPE-level comparison through the tower's own
//! `Cmp`. So `upward_rank` cannot publish a grade without naming its arity as a
//! type.
//!
//! CLAIM A. The arity is bounded and the bound is already in the signature: no
//! simple path in a DAG on `C` nodes has more than `cap_size(C::CAP)` of them,
//! so `Arity <= C`'s capacity and `ArityMinusOne <= capacity - 1`. The
//! algorithm crate does not have to invent a number; it already carries one.
//!
//! CLAIM B. It carries it as a CONST, not a type. `Capacity` exposes
//! `const CAP: Cap` (`mock/crates/arvo-tensor/src/capacity.rs:24`) and nothing
//! else about its size. This section compiles the attempt to state the
//! obligation from that const, and it fails.
//!
//! CLAIM C. The spine rule (`49:59-72`) says what to do: a quantity computed
//! and then required in a type is a type. `Capacity` owes a `Nat` face. With
//! it, the obligation states in one where-clause and the rank function's grade
//! projects exactly the way a fold's does.
//!
//! CLAIM D. The arrow only goes one way. A `Nat` face can be PROJECTED to the
//! `usize` an array length needs; a `usize` const generic cannot be lifted to a
//! `Nat`. So the fix is not "add a projection to `Dim<const N: usize>`", it is
//! "`Dim` carries the type and derives the const", and the two spellings are
//! kept in agreement by a forced const assertion rather than by hope.
//!
//! EXPECTED: sections A and C compile; section B is a committed refusal.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   --extern grade_lib=libgrade_lib.rlib probe_1_the_arity_of_a_rank.rs

#![allow(dead_code)]

use core::marker::PhantomData;
use tower::nat::{Cmp, Eq3, Gt, Lt, Nat, Pos, Pz, H, I, O};

// ---------------------------------------------------------------------------
// The design's interior-safety machinery, verbatim from 47_probes/probe_3
// (lines 101-132). Reproduced rather than imported because that probe is a
// leaf, not a library; nothing here is new.
// ---------------------------------------------------------------------------

pub struct Safe;
pub struct Unsafe;
pub trait Safety {}
impl Safety for Safe {}
impl Safety for Unsafe {}

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

pub trait InteriorSafety<ArityMinusOne> {
    type Out: Safety;
}
impl<Hd: Pos + Cmp<A>, A: Pos> InteriorSafety<A> for Hd
where
    <Hd as Cmp<A>>::Out: SafetyOf,
{
    type Out = <<Hd as Cmp<A>>::Out as SafetyOf>::Out;
}

// ---------------------------------------------------------------------------
// CLAIM B: today's `Capacity`, and the obligation that cannot be stated from it.
//
// This is `arvo-tensor`'s trait reduced to the two members the question needs.
// ---------------------------------------------------------------------------

pub trait CapacityAsShipped {
    type Array<T>;
    /// `arvo-tensor` spells this `Cap`; `usize` here so the probe needs no
    /// arvo dependency. The distinction is irrelevant to the question: both
    /// are consts.
    const CAP: usize;
}

pub struct DimAsShipped<const N: usize>;

impl<const N: usize> CapacityAsShipped for DimAsShipped<N> {
    type Array<T> = [T; N];
    const CAP: usize = N;
}

// The obligation `upward_rank` owes, written against the shipped shape. The
// arity is `C::CAP`; `InteriorSafety` wants a `Pos`. There is no route from one
// to the other, and the two spellings below are the two anyone would try.
//
// UNCOMMENT EITHER TO REPRODUCE. Both are recorded verbatim in OUTCOMES.md.
//
// pub fn rank_shipped_shape_1<C: CapacityAsShipped, Hd: Pos>()
// where
//     Hd: InteriorSafety<{ C::CAP - 1 }>,
// {}
//
// pub fn rank_shipped_shape_2<C: CapacityAsShipped, Hd: Pos>()
// where
//     Hd: InteriorSafety<<C as CapacityAsShipped>::CAP>,
// {}

// ---------------------------------------------------------------------------
// CLAIM C + D: the capacity carries a `Nat` face, and the const is derived
// FROM it rather than the other way round.
// ---------------------------------------------------------------------------

pub trait CapacityWithNat {
    type Array<T>;
    /// The spine rule's answer: the size is computed (by whoever declares the
    /// graph) and then has to appear in a type (the interior-safety
    /// obligation), so it is a type.
    ///
    /// `Pos`, not `Nat`, because `Cmp` is declared over `Pos` (`vu_nat.rs:153`)
    /// and interior safety is a `Cmp`. The cost is stated rather than hidden:
    /// `Pos` has no zero, so a zero-capacity container has no `Dim`. That is
    /// arguably right (a zero-node DAG has no fold to be safe about) and it is
    /// a real narrowing of `Capacity`'s domain, recorded in the file.
    type Dim: Pos;
    /// Still a const, because array indexing and loop bounds only READ it.
    /// Projected from the type, so the two cannot decorrelate.
    const CAP: usize = <Pz<Self::Dim> as Nat>::VAL as usize;
}

/// `Dim` carrying both spellings. The const generic stays because `[T; N]`
/// is the language's array-length grammar and `[T; <Pz<P> as Nat>::VAL]` is a
/// const expression in type position, which is the forbidden feature. The
/// const assertion below is what stops the two from drifting apart.
pub struct DimBoth<const N: usize, P>(PhantomData<P>);

impl<const N: usize, P: Pos> CapacityWithNat for DimBoth<N, P> {
    type Array<T> = [T; N];
    type Dim = P;
    const CAP: usize = N;
}

/// The agreement check, forced. An anonymous `const _` inside an impl over
/// generic parameters is not necessarily evaluated; a named associated const
/// that a consumer path touches is. `witness` is that path.
pub trait DimAgrees: CapacityWithNat {
    const AGREES: ();
    fn witness() {
        let () = Self::AGREES;
    }
}

impl<const N: usize, P: Pos> DimAgrees for DimBoth<N, P> {
    const AGREES: () = assert!(N as u64 == <Pz<P> as Nat>::VAL);
}

// ---------------------------------------------------------------------------
// The rank signature the design gets, with the obligation stated.
//
// `W` is the weight type. The crate may not name `Number`, so `W` arrives as a
// bare parameter; `Headroom` is what file 26's Stage G move 1 ("enrich the
// bound", `26:476-482`) puts on it, and move 2 would project it off `W` itself.
// Either way the obligation below is the same one.
// ---------------------------------------------------------------------------

use grade_lib::{Faithful, Grade};

/// The four-tuple the design's `FoldGrade` reads, with the rank fold's own
/// members. Reproduced from `47_probes/probe_3:143-184` at the two arms this
/// probe reaches.
pub struct Refuse;
pub struct ReduceModulo;
pub struct Signed;

pub trait FoldGrade {
    type Out: Grade;
}
impl<Top, Bot, Dom> FoldGrade for (Safe, Top, Bot, Dom) {
    type Out = Faithful;
}
impl<Dom> FoldGrade for (Unsafe, Refuse, Refuse, Dom) {
    type Out = grade_lib::RefusalsTransferred;
}
impl FoldGrade for (Unsafe, ReduceModulo, ReduceModulo, tower::nat::Z) {
    type Out = Faithful;
}

pub struct Ranked<G: Grade>(PhantomData<G>);

/// The obligation states. `ArityMinusOne` is the capacity's own `Nat`, less
/// one, because the longest simple path visits every node at most once.
pub fn upward_rank_typed<C, W, Hd, Top, Bot, Dom>(
) -> Ranked<<(<Hd as InteriorSafety<C::Dim>>::Out, Top, Bot, Dom) as FoldGrade>::Out>
where
    C: CapacityWithNat,
    Hd: Pos + InteriorSafety<C::Dim>,
    (<Hd as InteriorSafety<C::Dim>>::Out, Top, Bot, Dom): FoldGrade,
{
    Ranked(PhantomData)
}

// ---------------------------------------------------------------------------
// Call sites. A 64-node DAG needs six bits of headroom in the accumulator to
// keep the rank fold interior-safe, and the signature now says so.
// ---------------------------------------------------------------------------

type P64 = O<O<O<O<O<O<H>>>>>>; // 64
type P6 = O<I<H>>; // 6
type P70 = O<I<I<O<O<O<H>>>>>>; // 70

const _: () = assert!(<Pz<P64> as Nat>::VAL == 64);
const _: () = assert!(<Pz<P6> as Nat>::VAL == 6);
const _: () = assert!(<Pz<P70> as Nat>::VAL == 70);

pub type Cap64 = DimBoth<64, P64>;

/// Six bits of headroom against sixty-four nodes: NOT interior-safe, because
/// interior safety is about the COUNT of composed additions, not their bit
/// width. Refusing at both ends, so the grade published is
/// `RefusalsTransferred`, which is exactly what a `Precise` rank returns.
pub fn rank_precise_64() -> Ranked<grade_lib::RefusalsTransferred> {
    upward_rank_typed::<Cap64, (), P6, Refuse, Refuse, Signed>()
}

/// Seventy of headroom against sixty-four nodes: interior-safe, so the fold
/// publishes `Faithful` and the rank is the exact longest weighted path.
pub fn rank_precise_64_wide() -> Ranked<Faithful> {
    upward_rank_typed::<Cap64, (), P70, Refuse, Refuse, Signed>()
}

/// The agreement check is reachable, so a mismatched `DimBoth` cannot survive.
pub fn check_agreement() {
    <Cap64 as DimAgrees>::witness();
}
