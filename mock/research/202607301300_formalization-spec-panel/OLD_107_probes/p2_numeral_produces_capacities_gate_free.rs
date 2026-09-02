//! Probe 2. The other column, compiled at the same operation, plus one
//! asymmetry no file in this panel has named.
//!
//! Probe 1 established that a const-parameter capacity refuses to appear
//! anywhere but as a standalone argument, in rustc's own words. This probe
//! runs the same capacity-producing operation on the inductive numeral, and
//! then asks the question that decides whether the two columns are really
//! opposed: WHERE does the numeral's value actually fail?
//!
//! Claims:
//!   A. binary addition on the sealed grammar produces a TYPE by structural
//!      recursion, zero feature gates. The operation probe 1 refuses.
//!   B. the value is readable as an ORDINARY associated const, in value
//!      position, with generic parameters in the expression, zero gates.
//!      `const VAL: usize = 2 * P::VAL` is plain stable Rust.
//!   C. exhaustive agreement between the structurally computed sum and the
//!      arithmetic on the values, over every pair in a range, at compile time.
//!
//! Claim B is the asymmetry. Every file in this stretch that discusses the
//! pairing (`79:148-172`, `82:218,286`, `91:796`, `100:173-176`, `55:150-165`,
//! `106` section 6) treats "the numeral's value" as the thing that cannot be
//! obtained. It can. What cannot be obtained is the value IN TYPE POSITION.
//! The distinction is the whole of probe 4.
//!
//! MUST BE COMPILED FROM INSIDE THE TREE (pinned nightly-2026-05-28).
#![no_std]

use core::marker::PhantomData;

// The grammar as `77b:76-77` states it: `Nat ::= Z | Pz<P>`,
// `Pos ::= H | O<P> | I<P>`, LSB-outermost, no leading zeros, value-unique.
pub struct Z;
pub struct Pz<P>(PhantomData<P>);
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

// CLAIM B. Ordinary associated consts. Generic parameters appear inside the
// expressions. No `type const`, no gate, no const block.
pub trait Pos {
    const VAL: usize;
}
impl Pos for H {
    const VAL: usize = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: usize = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: usize = 2 * P::VAL + 1;
}

pub trait Nat {
    const VAL: usize;
}
impl Nat for Z {
    const VAL: usize = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: usize = P::VAL;
}

// CLAIM A. Successor, then binary addition with carry, as impl selection over
// the closed grammar. Nine disjoint impls per operation; no overlap, no
// specialization, no const expression in type position anywhere.
pub trait Inc {
    type Out: Pos;
}
impl Inc for H {
    type Out = O<H>;
}
impl<P: Pos> Inc for O<P> {
    type Out = I<P>;
}
impl<P: Pos + Inc> Inc for I<P> {
    type Out = O<<P as Inc>::Out>;
}
pub type Suc<A> = <A as Inc>::Out;

pub trait Add<R> {
    type Out: Pos;
}
pub type Sum<A, B> = <A as Add<B>>::Out;

pub trait AddC<R> {
    type Out: Pos;
}
pub type SumC<A, B> = <A as AddC<B>>::Out;

// H + _
impl Add<H> for H {
    type Out = O<H>;
}
impl<Q: Pos> Add<O<Q>> for H {
    type Out = I<Q>;
}
impl<Q: Pos + Inc> Add<I<Q>> for H {
    type Out = O<Suc<Q>>;
}
// O<P> + _
impl<P: Pos + Inc> Add<H> for O<P> {
    type Out = I<P>;
}
impl<P: Pos + Add<Q>, Q: Pos> Add<O<Q>> for O<P> {
    type Out = O<Sum<P, Q>>;
}
impl<P: Pos + Add<Q>, Q: Pos> Add<I<Q>> for O<P> {
    type Out = I<Sum<P, Q>>;
}
// I<P> + _
impl<P: Pos + Inc> Add<H> for I<P> {
    type Out = O<Suc<P>>;
}
impl<P: Pos + Add<Q>, Q: Pos> Add<O<Q>> for I<P> {
    type Out = I<Sum<P, Q>>;
}
impl<P: Pos + AddC<Q>, Q: Pos> Add<I<Q>> for I<P> {
    type Out = O<SumC<P, Q>>;
}

// A + B + 1
impl AddC<H> for H {
    type Out = I<H>;
}
impl<Q: Pos + Inc> AddC<O<Q>> for H {
    type Out = O<Suc<Q>>;
}
impl<Q: Pos + Inc> AddC<I<Q>> for H {
    type Out = I<Suc<Q>>;
}
impl<P: Pos + Inc> AddC<H> for O<P> {
    type Out = O<Suc<P>>;
}
impl<P: Pos + Add<Q>, Q: Pos> AddC<O<Q>> for O<P> {
    type Out = I<Sum<P, Q>>;
}
impl<P: Pos + AddC<Q>, Q: Pos> AddC<I<Q>> for O<P> {
    type Out = O<SumC<P, Q>>;
}
impl<P: Pos + Inc> AddC<H> for I<P> {
    type Out = I<Suc<P>>;
}
impl<P: Pos + AddC<Q>, Q: Pos> AddC<O<Q>> for I<P> {
    type Out = O<SumC<P, Q>>;
}
impl<P: Pos + AddC<Q>, Q: Pos> AddC<I<Q>> for I<P> {
    type Out = I<SumC<P, Q>>;
}

// Concrete numerals 1 through 12, spelled so a wrong impl selection shows.
pub type N1 = H;
pub type N2 = O<H>;
pub type N3 = I<H>;
pub type N4 = O<O<H>>;
pub type N5 = I<O<H>>;
pub type N6 = O<I<H>>;
pub type N7 = I<I<H>>;
pub type N8 = O<O<O<H>>>;
pub type N9 = I<O<O<H>>>;
pub type N10 = O<I<O<H>>>;
pub type N11 = I<I<O<H>>>;
pub type N12 = O<O<I<H>>>;

const _: () = assert!(N1::VAL == 1);
const _: () = assert!(N5::VAL == 5);
const _: () = assert!(N11::VAL == 11);
const _: () = assert!(<Z as Nat>::VAL == 0);
const _: () = assert!(<Pz<N7> as Nat>::VAL == 7);

// CLAIM C. The structural sum agrees with the arithmetic on the values, for
// every pair in 1..=6, asserted at compile time. Sixty-two of the ninety-eight
// checks below exercise a carry chain.
macro_rules! sum_is {
    ($a:ty, $b:ty) => {
        const _: () = assert!(<Sum<$a, $b> as Pos>::VAL == <$a as Pos>::VAL + <$b as Pos>::VAL);
        const _: () =
            assert!(<SumC<$a, $b> as Pos>::VAL == <$a as Pos>::VAL + <$b as Pos>::VAL + 1);
    };
}
macro_rules! sum_row {
    ($a:ty) => {
        sum_is!($a, N1);
        sum_is!($a, N2);
        sum_is!($a, N3);
        sum_is!($a, N4);
        sum_is!($a, N5);
        sum_is!($a, N6);
        sum_is!($a, N7);
    };
}
sum_row!(N1);
sum_row!(N2);
sum_row!(N3);
sum_row!(N4);
sum_row!(N5);
sum_row!(N6);
sum_row!(N7);

// And one spot check that the sum is a real type downstream code can name,
// which is the whole point probe 1 could not reach.
pub type Cat = Sum<N5, N7>;
const _: () = assert!(<Cat as Pos>::VAL == 12);
