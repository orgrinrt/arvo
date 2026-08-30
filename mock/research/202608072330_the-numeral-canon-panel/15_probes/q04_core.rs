// q04 core. The nat machinery, plus the three things it was missing:
// canonical natural subtraction, max/min, and the two tight shape rules from
// q02 and q03 written over TOTAL width rather than integer width.
//
// PROVENANCE. The nat itself (Z / O<N> / E<N>), Add, AddC, Cmp, OrElse, IfLe,
// CeilHalf, Buf and the Rung selector are lifted from 13_probes/p09_core.rs.
// That is a spike and is cited for what it proved: the algebra compiles with
// zero feature gates and its addition normalises over a 4225-pair matrix. Its
// names and arities are scaffolding, not decisions, and are kept only so a
// reader can diff the two files. Everything from `MkE` down is mine.
//
// Toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), pin nightly-2026-05-28.
// Features: none. No -Z flags. Default solver.

#![no_std]
#![allow(dead_code)]

pub struct Z;
pub struct O<N>(N);
pub struct E<N>(N);

// ---------------------------------------------------------------- addition ---
pub trait Add<R> {
    type Out;
}
pub trait AddC<R> {
    type Out;
}
pub type Sum<A, B> = <A as Add<B>>::Out;
pub type SumC<A, B> = <A as AddC<B>>::Out;

impl Add<Z> for Z {
    type Out = Z;
}
impl<B> Add<O<B>> for Z {
    type Out = O<B>;
}
impl<B> Add<E<B>> for Z {
    type Out = E<B>;
}
impl<A> Add<Z> for O<A> {
    type Out = O<A>;
}
impl<A> Add<Z> for E<A> {
    type Out = E<A>;
}
impl<A: AddC<B>, B> Add<O<B>> for O<A> {
    type Out = E<SumC<A, B>>;
}
impl<A: Add<B>, B> Add<E<B>> for O<A> {
    type Out = O<Sum<A, B>>;
}
impl<A: Add<B>, B> Add<O<B>> for E<A> {
    type Out = O<Sum<A, B>>;
}
impl<A: Add<B>, B> Add<E<B>> for E<A> {
    type Out = E<Sum<A, B>>;
}

impl AddC<Z> for Z {
    type Out = O<Z>;
}
impl<B: AddC<Z>> AddC<O<B>> for Z {
    type Out = E<SumC<B, Z>>;
}
impl<B> AddC<E<B>> for Z {
    type Out = O<B>;
}
impl<A: AddC<Z>> AddC<Z> for O<A> {
    type Out = E<SumC<A, Z>>;
}
impl<A> AddC<Z> for E<A> {
    type Out = O<A>;
}
impl<A: AddC<B>, B> AddC<O<B>> for O<A> {
    type Out = O<SumC<A, B>>;
}
impl<A: AddC<B>, B> AddC<E<B>> for O<A> {
    type Out = E<SumC<A, B>>;
}
impl<A: AddC<B>, B> AddC<O<B>> for E<A> {
    type Out = E<SumC<A, B>>;
}
impl<A: Add<B>, B> AddC<E<B>> for E<A> {
    type Out = O<Sum<A, B>>;
}

// ------------------------------------------------------ ordering, selectors ---
pub struct Lt;
pub struct Eqq;
pub struct Gt;

pub trait OrElse<D> {
    type Out;
}
impl<D> OrElse<D> for Eqq {
    type Out = D;
}
impl<D> OrElse<D> for Lt {
    type Out = Lt;
}
impl<D> OrElse<D> for Gt {
    type Out = Gt;
}

pub trait IfLe<T, F> {
    type Out;
}
impl<T, F> IfLe<T, F> for Lt {
    type Out = T;
}
impl<T, F> IfLe<T, F> for Eqq {
    type Out = T;
}
impl<T, F> IfLe<T, F> for Gt {
    type Out = F;
}

// strictly-greater selector, which the tight addition carry needs
pub trait IfGt<T, F> {
    type Out;
}
impl<T, F> IfGt<T, F> for Lt {
    type Out = F;
}
impl<T, F> IfGt<T, F> for Eqq {
    type Out = F;
}
impl<T, F> IfGt<T, F> for Gt {
    type Out = T;
}

// equality selector, which the tight product predicate needs
pub trait IfEq<T, F> {
    type Out;
}
impl<T, F> IfEq<T, F> for Lt {
    type Out = F;
}
impl<T, F> IfEq<T, F> for Eqq {
    type Out = T;
}
impl<T, F> IfEq<T, F> for Gt {
    type Out = F;
}

pub trait Cmp<R> {
    type Out;
}
pub type Ord2<A, B> = <A as Cmp<B>>::Out;

impl Cmp<Z> for Z {
    type Out = Eqq;
}
impl<B> Cmp<O<B>> for Z {
    type Out = Lt;
}
impl<B> Cmp<E<B>> for Z {
    type Out = Lt;
}
impl<A> Cmp<Z> for O<A> {
    type Out = Gt;
}
impl<A> Cmp<Z> for E<A> {
    type Out = Gt;
}
impl<A: Cmp<B>, B> Cmp<O<B>> for O<A> {
    type Out = Ord2<A, B>;
}
impl<A: Cmp<B>, B> Cmp<E<B>> for E<A> {
    type Out = Ord2<A, B>;
}
impl<A: Cmp<B>, B> Cmp<E<B>> for O<A>
where
    Ord2<A, B>: OrElse<Gt>,
{
    type Out = <Ord2<A, B> as OrElse<Gt>>::Out;
}
impl<A: Cmp<B>, B> Cmp<O<B>> for E<A>
where
    Ord2<A, B>: OrElse<Lt>,
{
    type Out = <Ord2<A, B> as OrElse<Lt>>::Out;
}

// ------------------------------------------- MINE FROM HERE: canonical even ---
// E<Z> is a second spelling of zero and it breaks Cmp, which reports E<Z> as
// greater than Z. Every construction below that builds an even node goes
// through MkE so the encoding stays canonical.
pub trait MkE {
    type Out;
}
pub type Ev<A> = <A as MkE>::Out;
impl MkE for Z {
    type Out = Z;
}
impl<N> MkE for O<N> {
    type Out = E<O<N>>;
}
impl<N> MkE for E<N> {
    type Out = E<E<N>>;
}

// ------------------------------------------------- natural subtraction, L-R ---
// Total only where R <= L. Where R > L there is deliberately no impl, so an
// underflow is a compile refusal rather than a wrapped value.
pub trait Sub<R> {
    type Out;
}
pub trait SubB<R> {
    type Out;
} // L - R - 1
pub type Dif<A, B> = <A as Sub<B>>::Out;
pub type DifB<A, B> = <A as SubB<B>>::Out;

impl Sub<Z> for Z {
    type Out = Z;
}
impl<A> Sub<Z> for O<A> {
    type Out = O<A>;
}
impl<A> Sub<Z> for E<A> {
    type Out = E<A>;
}
impl<A: Sub<B>, B> Sub<O<B>> for O<A>
where
    Dif<A, B>: MkE,
{
    type Out = Ev<Dif<A, B>>;
}
impl<A: Sub<B>, B> Sub<E<B>> for E<A>
where
    Dif<A, B>: MkE,
{
    type Out = Ev<Dif<A, B>>;
}
impl<A: Sub<B>, B> Sub<E<B>> for O<A> {
    type Out = O<Dif<A, B>>;
}
impl<A: SubB<B>, B> Sub<O<B>> for E<A> {
    type Out = O<DifB<A, B>>;
}

impl<A> SubB<Z> for O<A>
where
    A: MkE,
{
    type Out = Ev<A>;
}
impl<A: SubB<Z>> SubB<Z> for E<A> {
    type Out = O<DifB<A, Z>>;
}
impl<A: SubB<B>, B> SubB<O<B>> for O<A> {
    type Out = O<DifB<A, B>>;
}
impl<A: Sub<B>, B> SubB<E<B>> for O<A>
where
    Dif<A, B>: MkE,
{
    type Out = Ev<Dif<A, B>>;
}
impl<A: SubB<B>, B> SubB<O<B>> for E<A>
where
    DifB<A, B>: MkE,
{
    type Out = Ev<DifB<A, B>>;
}
impl<A: SubB<B>, B> SubB<E<B>> for E<A> {
    type Out = O<DifB<A, B>>;
}

// ------------------------------------------------------------- max and min ---
pub type Max<A, B> = <Ord2<A, B> as IfLe<B, A>>::Out;
pub type Min<A, B> = <Ord2<A, B> as IfLe<A, B>>::Out;

// ----------------------------------------------------- ceil to bytes, buffer --
pub trait CeilHalf {
    type Out;
}
pub type CH<A> = <A as CeilHalf>::Out;
impl CeilHalf for Z {
    type Out = Z;
}
impl<A: AddC<Z>> CeilHalf for O<A> {
    type Out = SumC<A, Z>;
}
impl<A> CeilHalf for E<A> {
    type Out = A;
}
pub type Bytes<W> = CH<CH<CH<W>>>;

pub struct Nil;
#[repr(C)]
pub struct One<T>(u8, T, T);
#[repr(C)]
pub struct Two<T>(T, T);

pub trait Buf {
    type Out;
}
pub type BufOf<A> = <A as Buf>::Out;
impl Buf for Z {
    type Out = Nil;
}
impl<A: Buf> Buf for O<A> {
    type Out = One<BufOf<A>>;
}
impl<A: Buf> Buf for E<A> {
    type Out = Two<BufOf<A>>;
}

pub type Rung<W, Bound, Narrow, Wide> = <Ord2<W, Bound> as IfLe<Narrow, Wide>>::Out;

pub trait Same<T> {}
impl<T> Same<T> for T {}
