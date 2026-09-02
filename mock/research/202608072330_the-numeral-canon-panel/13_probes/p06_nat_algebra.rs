// P6. A binary type-level nat, LSB first, canonical (no E<Z>), with addition.
// No features at all. The question: does type-level addition normalise, so that
// the sum of two widths is the SAME TYPE as the literal for that sum.
#![no_std]
#![allow(dead_code)]

pub struct Z; // 0
pub struct O<N>(N); // 2n + 1
pub struct E<N>(N); // 2n, with n != 0 so the form stays canonical

pub trait Nat {}
impl Nat for Z {}
impl<N: Nat> Nat for O<N> {}
impl<N: Nat> Nat for E<N> {}

pub trait Add<R> {
    type Out;
}
pub trait AddC<R> {
    type Out;
} // add with a carry of one

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

// literals, LSB first
pub type N0 = Z;
pub type N1 = O<Z>;
pub type N3 = O<O<Z>>;
pub type N5 = O<E<O<Z>>>;
pub type N6 = E<O<O<Z>>>;
pub type N13 = O<E<O<O<Z>>>>;
pub type N16 = E<E<E<E<O<Z>>>>>;
pub type N26 = E<O<E<O<O<Z>>>>>;
pub type N32 = E<E<E<E<E<O<Z>>>>>>;

// the identity that decides whether an operation output is namable through the
// same surface the consumer used for its inputs
pub trait Same<T> {}
impl<T> Same<T> for T {}

fn _mul_output_is_the_named_literal()
where
    Sum<N13, N13>: Same<N26>,
    Sum<N3, N3>: Same<N6>,
    Sum<N16, N16>: Same<N32>,
    Sum<N13, N3>: Same<N16>,
    Sum<N0, N5>: Same<N5>,
    Sum<N5, N0>: Same<N5>,
{
}
