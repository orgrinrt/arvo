//! Probe 3: the trait-level gcd, built. It is Stein's binary algorithm,
//! which is also what the named prior art uses (`typenum-1.20.1/src/uint.rs`
//! lines 1467 to 1528; I read it rather than assuming Euclid, and the
//! assumption would have been wrong). The difference is not the algorithm,
//! it is what the encoding makes free.
//!
//! On the value-unique encoding of probe 2, three of Stein's five steps are
//! pure impl selection with no computation at all:
//!
//!   halve an even number      `O<P>` -> `P`                  structural
//!   double the result         `P`    -> `O<P>`               structural
//!   test parity               match the outer constructor    structural
//!
//! and the odd/odd step loses its halving entirely, because for odd
//! `x = 2a+1` and `y = 2b+1` the quantity Stein needs, `(x - y) / 2`, is
//! exactly `a - b`: a subtraction of the two operands' own tails, with no
//! shift after it. typenum's odd/odd impl instead computes `Max` and `Min`
//! (each named twice in its where-clause and twice more in its output type),
//! subtracts at full width, and recurses into the even/odd case to do the
//! halving, which is one traversal this shape does not perform.
//!
//! Nothing here trims. `NSub` builds its result through `Dbl`/`DblInc`,
//! which are the smart constructors of probe 2, so a borrow that cancels the
//! leading digits produces `Z` at the point it happens rather than a padded
//! chain to be repaired later. That is the whole of typenum's
//! `Invert -> TrimTrailingZeros -> Invert` (private.rs:35-36, 304-310),
//! absorbed into constructor choice.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_3_stein_gcd_on_the_value_unique_encoding.rs
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).
//! Correctness: 28 binary gcd instantiations plus 4 three- and four-argument
//! folds, each asserted against a hand-computed value, including the
//! classical Euclid pair (1071, 462) -> 21, the coprime cases the reduction
//! actually needs, and file 34's own biased-MAC numbers.

#![allow(dead_code)]
#![no_std]

use core::marker::PhantomData;

// --- the value-unique encoding (probe 2, unchanged) ---

pub trait Pos {
    const VAL: u64;
}
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

impl Pos for H {
    const VAL: u64 = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: u64 = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: u64 = 2 * P::VAL + 1;
}

pub trait Nat {
    const VAL: u64;
}
pub struct Z;
pub struct Pz<P>(PhantomData<P>);

impl Nat for Z {
    const VAL: u64 = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: u64 = P::VAL;
}

pub trait Dbl {
    type Out: Nat;
}
impl Dbl for Z {
    type Out = Z;
}
impl<P: Pos> Dbl for Pz<P> {
    type Out = Pz<O<P>>;
}

pub trait DblInc {
    type Out: Nat;
}
impl DblInc for Z {
    type Out = Pz<H>;
}
impl<P: Pos> DblInc for Pz<P> {
    type Out = Pz<I<P>>;
}

/// The one direction that is partial: a `Nat` is a `Pos` only when nonzero,
/// and `Z` has no impl. Every use below is guarded by a strict comparison.
pub trait AsPos {
    type Out: Pos;
}
impl<P: Pos> AsPos for Pz<P> {
    type Out = P;
}

// --- comparison, LSB-first with a tie-break on the low digit ---

pub trait Ord3 {}
pub struct Lt;
pub struct Eq3;
pub struct Gt;
impl Ord3 for Lt {}
impl Ord3 for Eq3 {}
impl Ord3 for Gt {}

/// `Tie<T>` resolves a comparison of the high parts: equal high parts defer
/// to `T`, the low digit's own verdict; unequal high parts win outright.
pub trait Tie<T> {
    type Out: Ord3;
}
impl<T: Ord3> Tie<T> for Eq3 {
    type Out = T;
}
impl<T: Ord3> Tie<T> for Lt {
    type Out = Lt;
}
impl<T: Ord3> Tie<T> for Gt {
    type Out = Gt;
}

pub trait Cmp<Rhs> {
    type Out: Ord3;
}
impl Cmp<H> for H {
    type Out = Eq3;
}
impl<B: Pos> Cmp<O<B>> for H {
    type Out = Lt;
}
impl<B: Pos> Cmp<I<B>> for H {
    type Out = Lt;
}
impl<A: Pos> Cmp<H> for O<A> {
    type Out = Gt;
}
impl<A: Pos> Cmp<H> for I<A> {
    type Out = Gt;
}
impl<A: Pos + Cmp<B>, B: Pos> Cmp<O<B>> for O<A> {
    type Out = <A as Cmp<B>>::Out;
}
impl<A: Pos + Cmp<B>, B: Pos> Cmp<I<B>> for I<A> {
    type Out = <A as Cmp<B>>::Out;
}
impl<A: Pos + Cmp<B>, B: Pos> Cmp<I<B>> for O<A>
where
    <A as Cmp<B>>::Out: Tie<Lt>,
{
    type Out = <<A as Cmp<B>>::Out as Tie<Lt>>::Out;
}
impl<A: Pos + Cmp<B>, B: Pos> Cmp<O<B>> for I<A>
where
    <A as Cmp<B>>::Out: Tie<Gt>,
{
    type Out = <<A as Cmp<B>>::Out as Tie<Gt>>::Out;
}

// --- decrement and subtraction on Nat, built through the smart ---
// --- constructors so no repair pass exists ---

pub trait Dec {
    type Out: Nat;
}
impl Dec for Pz<H> {
    type Out = Z;
}
impl<P: Pos> Dec for Pz<O<P>>
where
    Pz<P>: Dec,
    <Pz<P> as Dec>::Out: DblInc,
{
    type Out = <<Pz<P> as Dec>::Out as DblInc>::Out;
}
impl<P: Pos> Dec for Pz<I<P>> {
    type Out = Pz<O<P>>;
}

/// `NSub` is partial: there is no impl where the result would be negative,
/// so a caller that has not established `self >= rhs` gets an unsatisfied
/// bound rather than a wrong answer.
pub trait NSub<Rhs> {
    type Out: Nat;
}
impl NSub<Z> for Z {
    type Out = Z;
}
impl<A: Pos> NSub<Z> for Pz<A> {
    type Out = Pz<A>;
}
impl NSub<Pz<H>> for Pz<H> {
    type Out = Z;
}
impl<A: Pos> NSub<Pz<H>> for Pz<O<A>>
where
    Pz<O<A>>: Dec,
{
    type Out = <Pz<O<A>> as Dec>::Out;
}
impl<A: Pos> NSub<Pz<H>> for Pz<I<A>> {
    type Out = Pz<O<A>>;
}
impl<A: Pos, B: Pos> NSub<Pz<O<B>>> for Pz<O<A>>
where
    Pz<A>: NSub<Pz<B>>,
    <Pz<A> as NSub<Pz<B>>>::Out: Dbl,
{
    type Out = <<Pz<A> as NSub<Pz<B>>>::Out as Dbl>::Out;
}
impl<A: Pos, B: Pos> NSub<Pz<I<B>>> for Pz<I<A>>
where
    Pz<A>: NSub<Pz<B>>,
    <Pz<A> as NSub<Pz<B>>>::Out: Dbl,
{
    type Out = <<Pz<A> as NSub<Pz<B>>>::Out as Dbl>::Out;
}
impl<A: Pos, B: Pos> NSub<Pz<O<B>>> for Pz<I<A>>
where
    Pz<A>: NSub<Pz<B>>,
    <Pz<A> as NSub<Pz<B>>>::Out: DblInc,
{
    type Out = <<Pz<A> as NSub<Pz<B>>>::Out as DblInc>::Out;
}
impl<A: Pos, B: Pos> NSub<Pz<I<B>>> for Pz<O<A>>
where
    Pz<A>: NSub<Pz<B>>,
    <Pz<A> as NSub<Pz<B>>>::Out: Dbl,
    <<Pz<A> as NSub<Pz<B>>>::Out as Dbl>::Out: Dec,
{
    type Out = <<<Pz<A> as NSub<Pz<B>>>::Out as Dbl>::Out as Dec>::Out;
}

/// The tail difference Stein's odd/odd step needs, as a `Pos`. Guarded: it
/// is only ever projected where the comparison already said the two tails
/// differ, so the `Z` case it has no impl for cannot arise.
pub type TailDiff<A, B> = <<Pz<A> as NSub<Pz<B>>>::Out as AsPos>::Out;

// --- Stein's binary gcd ---

pub trait Gcd<Rhs> {
    type Out: Pos;
}

/// gcd(1, y) = 1.
impl<B: Pos> Gcd<B> for H {
    type Out = H;
}
/// gcd(x, 1) = 1, split across the two non-`H` constructors so it does not
/// overlap the impl above.
impl<A: Pos> Gcd<H> for O<A> {
    type Out = H;
}
impl<A: Pos> Gcd<H> for I<A> {
    type Out = H;
}
/// Both even: gcd(2a, 2b) = 2 gcd(a, b). The halving is `O<A> -> A`, free.
impl<A: Pos + Gcd<B>, B: Pos> Gcd<O<B>> for O<A> {
    type Out = O<<A as Gcd<B>>::Out>;
}
/// One even: drop its factor of two. Free again.
impl<A: Pos + Gcd<I<B>>, B: Pos> Gcd<I<B>> for O<A> {
    type Out = <A as Gcd<I<B>>>::Out;
}
impl<A: Pos, B: Pos> Gcd<O<B>> for I<A>
where
    I<A>: Gcd<B>,
{
    type Out = <I<A> as Gcd<B>>::Out;
}
/// Both odd: compare the tails and take one step. `(x - y)/2` for odd x, y
/// is `a - b` on the tails, so there is no halving here at all.
impl<A: Pos + Cmp<B>, B: Pos> Gcd<I<B>> for I<A>
where
    <A as Cmp<B>>::Out: OddStep<A, B>,
{
    type Out = <<A as Cmp<B>>::Out as OddStep<A, B>>::Out;
}

/// The three-way branch of the odd/odd case, dispatched on the ordering.
pub trait OddStep<A, B> {
    type Out: Pos;
}
impl<A: Pos, B: Pos> OddStep<A, B> for Eq3 {
    type Out = I<A>;
}
impl<A: Pos, B: Pos> OddStep<A, B> for Gt
where
    Pz<A>: NSub<Pz<B>>,
    <Pz<A> as NSub<Pz<B>>>::Out: AsPos,
    TailDiff<A, B>: Gcd<I<B>>,
{
    type Out = <TailDiff<A, B> as Gcd<I<B>>>::Out;
}
impl<A: Pos, B: Pos> OddStep<A, B> for Lt
where
    Pz<B>: NSub<Pz<A>>,
    <Pz<B> as NSub<Pz<A>>>::Out: AsPos,
    TailDiff<B, A>: Gcd<I<A>>,
{
    type Out = <TailDiff<B, A> as Gcd<I<A>>>::Out;
}

// --- values, for the assertions ---

pub type P1 = H;
pub type P2 = O<H>;
pub type P3 = I<H>;
pub type P4 = O<O<H>>;
pub type P5 = I<O<H>>;
pub type P6 = O<I<H>>;
pub type P7 = I<I<H>>;
pub type P8 = O<O<O<H>>>;
pub type P12 = O<O<I<H>>>;
pub type P13 = I<O<I<H>>>;
pub type P15 = I<I<I<H>>>;
pub type P20 = O<O<I<O<H>>>>;
pub type P21 = I<O<I<O<H>>>>;
pub type P24 = O<O<O<I<H>>>>;
pub type P35 = I<I<O<O<O<H>>>>>;
pub type P45 = I<O<I<I<O<H>>>>>;
pub type P63 = I<I<I<I<I<H>>>>>;
pub type P96 = O<O<O<O<O<I<H>>>>>>;
pub type P100 = O<O<I<O<O<I<H>>>>>>;
pub type P144 = O<O<O<O<I<O<O<H>>>>>>>;
pub type P255 = I<I<I<I<I<I<I<H>>>>>>>;
pub type P462 = O<I<I<I<O<O<I<I<H>>>>>>>>;
pub type P1071 = I<I<I<I<O<I<O<O<O<O<H>>>>>>>>>>;

const _: () = assert!(P462::VAL == 462);
const _: () = assert!(P1071::VAL == 1071);

// --- CLAIM A: correctness, against hand-computed values ---

pub type G<A, B> = <A as Gcd<B>>::Out;

// even/even
const _: () = assert!(<G<P12, P8> as Pos>::VAL == 4);
const _: () = assert!(<G<P8, P12> as Pos>::VAL == 4);
const _: () = assert!(<G<P24, P144> as Pos>::VAL == 24);
const _: () = assert!(<G<P100, P96> as Pos>::VAL == 4);
// even/odd and odd/even
const _: () = assert!(<G<P12, P15> as Pos>::VAL == 3);
const _: () = assert!(<G<P15, P12> as Pos>::VAL == 3);
const _: () = assert!(<G<P20, P15> as Pos>::VAL == 5);
const _: () = assert!(<G<P144, P63> as Pos>::VAL == 9);
// odd/odd
const _: () = assert!(<G<P15, P35> as Pos>::VAL == 5);
const _: () = assert!(<G<P35, P15> as Pos>::VAL == 5);
const _: () = assert!(<G<P45, P63> as Pos>::VAL == 9);
const _: () = assert!(<G<P255, P15> as Pos>::VAL == 15);
const _: () = assert!(<G<P21, P35> as Pos>::VAL == 7);
// the classical Euclid pair
const _: () = assert!(<G<P1071, P462> as Pos>::VAL == 21);
const _: () = assert!(<G<P462, P1071> as Pos>::VAL == 21);
// coprime, which is the case a reduced-fraction check actually asks about
const _: () = assert!(<G<P13, P7> as Pos>::VAL == 1);
const _: () = assert!(<G<P7, P13> as Pos>::VAL == 1);
const _: () = assert!(<G<P3, P4> as Pos>::VAL == 1);
const _: () = assert!(<G<P2, P3> as Pos>::VAL == 1);
// with one, and with equals
const _: () = assert!(<G<P1, P462> as Pos>::VAL == 1);
const _: () = assert!(<G<P462, P1> as Pos>::VAL == 1);
const _: () = assert!(<G<P13, P13> as Pos>::VAL == 13);
const _: () = assert!(<G<P12, P12> as Pos>::VAL == 12);
// divides
const _: () = assert!(<G<P6, P12> as Pos>::VAL == 6);

// --- CLAIM B: the gcd's output is a canonical spelling, so the result ---
// --- inhabits the type a consumer writes directly. This is the ---
// --- obligation itself, checked on the gcd rather than assumed of it. ---

pub fn same_type<T>(_: PhantomData<T>, _: PhantomData<T>) {}

pub fn gcd_output_inhabits_the_written_numeral() {
    same_type(PhantomData::<G<P12, P8>>, PhantomData::<P4>);
    same_type(PhantomData::<G<P1071, P462>>, PhantomData::<P21>);
    same_type(PhantomData::<G<P255, P15>>, PhantomData::<P15>);
    same_type(PhantomData::<G<P13, P7>>, PhantomData::<P1>);
}

// --- CLAIM C: gcd is commutative and associative AS TYPES. Neither is a ---
// --- fresh mathematical claim; both are type-level statements that are ---
// --- only well formed because the encoding is value-unique, which is ---
// --- exactly the obligation probe 5b in file 34 raised. ---

pub fn gcd_is_commutative_as_types() {
    same_type(PhantomData::<G<P12, P8>>, PhantomData::<G<P8, P12>>);
    same_type(PhantomData::<G<P1071, P462>>, PhantomData::<G<P462, P1071>>);
    same_type(PhantomData::<G<P21, P35>>, PhantomData::<G<P35, P21>>);
}

pub fn gcd_is_associative_as_types() {
    same_type(
        PhantomData::<G<G<P12, P8>, P20>>,
        PhantomData::<G<P12, G<P8, P20>>>,
    );
    same_type(
        PhantomData::<G<G<P144, P24>, P63>>,
        PhantomData::<G<P144, G<P24, P63>>>,
    );
}

// --- CLAIM D: the four-argument gcd the design's own formulas need. ---
// File 31 section 4.6 settles `adjustment = gcd(A1A2, A1B2, A2B1)`; file 34
// section 2.3 adds `B1B2` for the biased-MAC accumulator. Both are folds of
// the binary operator, and associativity (claim C) is what makes the fold's
// bracketing irrelevant as a type, not only as a value.

pub type Gcd3<A, B, C> = G<G<A, B>, C>;
pub type Gcd4<A, B, C, D> = G<G<G<A, B>, C>, D>;

const _: () = assert!(<Gcd3<P24, P144, P63> as Pos>::VAL == 3);
const _: () = assert!(<Gcd4<P24, P144, P63, P100> as Pos>::VAL == 1);
// file 34's own worked biased-MAC case, verbatim from
// `34_probes/probe_3_biased_products_break_pairwise_closure.rs:50-51,69,120`:
// A = 4, B = 2 squared, so the monomials are A1A2 = 16, A1B2 = 8, A2B1 = 8,
// B1B2 = 4. Its `MUL_ADJ` is gcd(16, 8, 8) = 8 and its `ACC_ADJ` is
// gcd(16, 8, 8, 4) = 4. Both reproduced here at the type level.
pub type P16 = O<O<O<O<H>>>>;
const _: () = assert!(<Gcd3<P16, P8, P8> as Pos>::VAL == 8);
const _: () = assert!(<Gcd4<P16, P8, P8, P4> as Pos>::VAL == 4);
