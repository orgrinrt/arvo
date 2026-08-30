// p6_core.  Definitions for p6 and its two negative controls.  No inner
// attributes here, so the file can be included from more than one crate root.
// See p6_sufficiency_as_a_bound.rs for the hypothesis this file serves.

use core::marker::PhantomData;

// ---------------------------------------------------------------- type-level nat
// Peano, because the check is on fraction widths whose realistic range is small
// and because a unary nat makes the Le relation two impls rather than a table.
// The encoding is scaffolding for the check; nothing here is a design decision
// about how a width should be spelled.

pub trait Nat {
    const VAL: u32;
}

pub struct Z;
pub struct Sk<N>(PhantomData<N>);

impl Nat for Z {
    const VAL: u32 = 0;
}
impl<N: Nat> Nat for Sk<N> {
    const VAL: u32 = N::VAL + 1;
}

// The order, inductively.  Two impls, no enumeration of widths anywhere.
pub trait Le<Other> {}
impl<N: Nat> Le<N> for Z {}
impl<A, B> Le<Sk<B>> for Sk<A> where A: Le<B> {}

// ------------------------------------------------------------------- numerals

pub trait Numeral {
    /// Fraction width, as a nat, so it can appear in a bound.
    type FracW: Nat;
    /// Integer width, as an ordinary const, since nothing below bounds on it.
    const I: i32;
}

pub struct U<const I: i32, F>(PhantomData<F>);

impl<const I: i32, F: Nat> Numeral for U<I, F> {
    type FracW = F;
    const I: i32 = I;
}

type N0 = Z;
type N1 = Sk<Z>;
type N2 = Sk<Sk<Z>>;
type N3 = Sk<Sk<Sk<Z>>>;

// ------------------------------------------------------ ARM A: the post-mono assert

struct SufficientAssert<A, P>(PhantomData<(A, P)>);

impl<A: Numeral, P: Numeral> SufficientAssert<A, P> {
    const CHECK: () = assert!(
        <A::FracW as Nat>::VAL >= <P::FracW as Nat>::VAL,
        "accumulator grid is coarser than the element grid"
    );
}

pub fn fold_arm_a<A: Numeral, P: Numeral>(seed: u64, xs: &[u64]) -> u64 {
    let () = SufficientAssert::<A, P>::CHECK;
    let mut acc = seed;
    for &x in xs {
        acc = acc.saturating_add(x);
    }
    acc
}

// ------------------------------------------------------- ARM B: the bound

/// A carries a grid at least as fine as P's.  Not a new axis: it is the existing
/// inclusion relation restricted to the grid coordinate, which is the only
/// coordinate the sufficiency condition turned out to touch.
pub trait GridSufficientFor<P> {}

impl<A, P> GridSufficientFor<P> for A
where
    A: Numeral,
    P: Numeral,
    P::FracW: Le<A::FracW>,
{
}

pub fn fold_arm_b<A, P>(seed: u64, xs: &[u64]) -> u64
where
    A: Numeral + GridSufficientFor<P>,
    P: Numeral,
{
    let mut acc = seed;
    for &x in xs {
        acc = acc.saturating_add(x);
    }
    acc
}

// ------------------------------------------------------ the unguarded baseline
// Same body, no bound.  If arm B erases, these two lower identically.

#[inline(never)]
pub fn fold_bare(seed: u64, xs: &[u64]) -> u64 {
    let mut acc = seed;
    for &x in xs {
        acc = acc.saturating_add(x);
    }
    acc
}

// --------------------------------------------------------------- exercised calls

type AccFine = U<3, N3>;
type ElemCoarse = U<1, N1>;
type ElemEqual = U<1, N3>;

#[inline(never)]
pub fn call_a(seed: u64, xs: &[u64]) -> u64 {
    fold_arm_a::<AccFine, ElemCoarse>(seed, xs)
}

#[inline(never)]
pub fn call_b(seed: u64, xs: &[u64]) -> u64 {
    fold_arm_b::<AccFine, ElemCoarse>(seed, xs)
}

#[inline(never)]
pub fn call_b_equal(seed: u64, xs: &[u64]) -> u64 {
    fold_arm_b::<AccFine, ElemEqual>(seed, xs)
}

// -------------------------------------------------------------- static evidence
// The nat's value is a compile-time constant, so the whole relation is decided
// before codegen.  These force that.

pub const FRAC_ACC: u32 = <<AccFine as Numeral>::FracW as Nat>::VAL;
pub const FRAC_ELEM: u32 = <<ElemCoarse as Numeral>::FracW as Nat>::VAL;
const _: () = assert!(FRAC_ACC == 3);
const _: () = assert!(FRAC_ELEM == 1);
const _: () = assert!(FRAC_ACC >= FRAC_ELEM);
