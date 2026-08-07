//! P7. The tight product form, built.
//!
//! P5 derived it and P6 characterised its firing region. The predicate
//!
//!     2^W1 + 2^W2 - 2  >=  2^(W1+W2-1)
//!
//! looks like it needs exponentials of type-level naturals, which would be a
//! wall. It does not. It is equivalent to `min(W1, W2) == 1` for W1, W2 >= 1,
//! proved below and independently measured over the whole box in P6.
//!
//!   W1 = 1: LHS = 2 + 2^W2 - 2 = 2^W2, RHS = 2^W2. Equal, so it fires.
//!   W1 >= 2 (and W2 >= W1): RHS = 2^(W1-1) * 2^W2 >= 2 * 2^W2 = 2^W2 + 2^W2,
//!           and 2^W2 > 2^W1 - 2 since W2 >= W1, so RHS > LHS. It does not fire.
//!
//! So the tight product numeral is:
//!
//!     F_out = F1 + F2
//!     W_out = W1 + W2, minus one when either operand's total width is 1
//!     I_out = W_out - F_out
//!
//! which is a sum, an equality test against one, and a conditional decrement.
//! This file builds exactly that and asserts it against values P5 computed
//! independently in Python from exact rational value sets.
//!
//! Zero `#![feature(...)]` gates. Default solver. No forbidden construct.

#![no_std]
#![no_main]

pub struct End;
pub struct Zero<T>(core::marker::PhantomData<T>);
pub struct One<T>(core::marker::PhantomData<T>);

pub trait Nat {
    const VAL: u32;
}
impl Nat for End {
    const VAL: u32 = 0;
}
impl<T: Nat> Nat for Zero<T> {
    const VAL: u32 = 2 * T::VAL;
}
impl<T: Nat> Nat for One<T> {
    const VAL: u32 = 2 * T::VAL + 1;
}

// --- successor and predecessor ---
pub trait Succ {
    type Out: Nat;
}
impl Succ for End {
    type Out = One<End>;
}
impl<T: Nat> Succ for Zero<T> {
    type Out = One<T>;
}
impl<T: Nat + Succ> Succ for One<T> {
    type Out = Zero<<T as Succ>::Out>;
}

/// Predecessor. Total on the naturals this design admits, because the only
/// place it is used is on a total width already known to be at least one.
pub trait Pred {
    type Out: Nat;
}
impl Pred for One<End> {
    type Out = End; // 1 - 1 = 0, and 0's unique spelling is End
}
impl<T: Nat> Pred for One<Zero<T>> {
    type Out = Zero<Zero<T>>;
}
impl<T: Nat> Pred for One<One<T>> {
    type Out = Zero<One<T>>;
}
impl<T: Nat + Pred> Pred for Zero<T>
where
    <T as Pred>::Out: Nat,
{
    type Out = One<<T as Pred>::Out>;
}

// --- addition, the ripple-carry fold ---
pub trait AddN<R> {
    type Out: Nat;
}
impl AddN<End> for End {
    type Out = End;
}
impl<T: Nat> AddN<Zero<T>> for End {
    type Out = Zero<T>;
}
impl<T: Nat> AddN<One<T>> for End {
    type Out = One<T>;
}
impl<T: Nat> AddN<End> for Zero<T> {
    type Out = Zero<T>;
}
impl<T: Nat> AddN<End> for One<T> {
    type Out = One<T>;
}
impl<A: Nat + AddN<B>, B: Nat> AddN<Zero<B>> for Zero<A> {
    type Out = Zero<<A as AddN<B>>::Out>;
}
impl<A: Nat + AddN<B>, B: Nat> AddN<One<B>> for Zero<A> {
    type Out = One<<A as AddN<B>>::Out>;
}
impl<A: Nat + AddN<B>, B: Nat> AddN<Zero<B>> for One<A> {
    type Out = One<<A as AddN<B>>::Out>;
}
impl<A: Nat + AddN<B>, B: Nat> AddN<One<B>> for One<A>
where
    <A as AddN<B>>::Out: Succ,
{
    type Out = Zero<<<A as AddN<B>>::Out as Succ>::Out>;
}

// --- the saving predicate: is this total width exactly one? ---
pub struct Yes;
pub struct No;

pub trait IsOne {
    type Out;
}
impl IsOne for End {
    type Out = No;
}
impl IsOne for One<End> {
    type Out = Yes;
}
impl<T: Nat> IsOne for One<Zero<T>> {
    type Out = No;
}
impl<T: Nat> IsOne for One<One<T>> {
    type Out = No;
}
impl<T: Nat> IsOne for Zero<T> {
    type Out = No; // any even natural is 0 or >= 2, never 1
}

/// Either operand being width one is enough, so the two verdicts join with Or.
pub trait Or<R> {
    type Out;
}
impl Or<Yes> for Yes {
    type Out = Yes;
}
impl Or<No> for Yes {
    type Out = Yes;
}
impl Or<Yes> for No {
    type Out = Yes;
}
impl Or<No> for No {
    type Out = No;
}

/// The conditional decrement. This is the entire correction.
pub trait MaybeDec<W> {
    type Out: Nat;
}
impl<W: Nat + Pred> MaybeDec<W> for Yes {
    type Out = <W as Pred>::Out;
}
impl<W: Nat> MaybeDec<W> for No {
    type Out = W;
}

// --- subtraction, needed only to recover I from W and F ---
pub trait SubN<R> {
    type Out: Nat;
}
impl<A: Nat> SubN<End> for A {
    type Out = A;
}
impl<A: Nat + Pred, B: Nat> SubN<One<B>> for A
where
    <A as Pred>::Out: SubN<Zero<B>>,
    <<A as Pred>::Out as SubN<Zero<B>>>::Out: Nat,
{
    type Out = <<A as Pred>::Out as SubN<Zero<B>>>::Out;
}
impl<A: Nat, B: Nat> SubN<Zero<One<B>>> for A
where
    A: Pred,
    <A as Pred>::Out: Pred,
    <<A as Pred>::Out as Pred>::Out: SubN<Zero<Zero<B>>>,
    <<<A as Pred>::Out as Pred>::Out as SubN<Zero<Zero<B>>>>::Out: Nat,
{
    type Out = <<<A as Pred>::Out as Pred>::Out as SubN<Zero<Zero<B>>>>::Out;
}
impl<A: Nat> SubN<Zero<End>> for A {
    type Out = A;
}
impl<A: Nat, B: Nat> SubN<Zero<Zero<B>>> for A
where
    A: SubN<Zero<B>>,
    <A as SubN<Zero<B>>>::Out: SubN<Zero<B>>,
    <<A as SubN<Zero<B>>>::Out as SubN<Zero<B>>>::Out: Nat,
{
    // 2 * (2b) subtracted as (2b) twice
    type Out = <<A as SubN<Zero<B>>>::Out as SubN<Zero<B>>>::Out;
}

// ---------------------------------------------------------------------------
// The numeral, and the two product formulas side by side.
// ---------------------------------------------------------------------------

pub struct Num<I, F>(core::marker::PhantomData<(I, F)>);

pub trait Widths {
    const I: u32;
    const F: u32;
    const W: u32;
}
impl<I: Nat, F: Nat> Widths for Num<I, F> {
    const I: u32 = I::VAL;
    const F: u32 = F::VAL;
    const W: u32 = I::VAL + F::VAL;
}

/// What the record's shape implies: sum the two coordinates.
pub trait MulNaive<R> {
    type Out;
}
impl<I1: Nat + AddN<I2>, F1: Nat + AddN<F2>, I2: Nat, F2: Nat> MulNaive<Num<I2, F2>>
    for Num<I1, F1>
{
    type Out = Num<<I1 as AddN<I2>>::Out, <F1 as AddN<F2>>::Out>;
}

/// The tight form. F is the sum; W is the sum less one when either operand's
/// total width is one; I is W minus F.
pub trait MulTight<R> {
    type Out;
}
impl<I1, F1, I2, F2> MulTight<Num<I2, F2>> for Num<I1, F1>
where
    I1: Nat + AddN<F1>,
    I2: Nat + AddN<F2>,
    F1: Nat + AddN<F2>,
    F2: Nat,
    <I1 as AddN<F1>>::Out: IsOne + AddN<<I2 as AddN<F2>>::Out>,
    <I2 as AddN<F2>>::Out: IsOne,
    <<I1 as AddN<F1>>::Out as IsOne>::Out: Or<<<I2 as AddN<F2>>::Out as IsOne>::Out>,
    <<<I1 as AddN<F1>>::Out as IsOne>::Out as Or<
        <<I2 as AddN<F2>>::Out as IsOne>::Out,
    >>::Out: MaybeDec<<<I1 as AddN<F1>>::Out as AddN<<I2 as AddN<F2>>::Out>>::Out>,
    <<<<I1 as AddN<F1>>::Out as IsOne>::Out as Or<
        <<I2 as AddN<F2>>::Out as IsOne>::Out,
    >>::Out as MaybeDec<
        <<I1 as AddN<F1>>::Out as AddN<<I2 as AddN<F2>>::Out>>::Out,
    >>::Out: SubN<<F1 as AddN<F2>>::Out>,
    <F1 as AddN<F2>>::Out: Nat,
{
    type Out = Num<
        <<<<<I1 as AddN<F1>>::Out as IsOne>::Out as Or<
            <<I2 as AddN<F2>>::Out as IsOne>::Out,
        >>::Out as MaybeDec<
            <<I1 as AddN<F1>>::Out as AddN<<I2 as AddN<F2>>::Out>>::Out,
        >>::Out as SubN<<F1 as AddN<F2>>::Out>>::Out,
        <F1 as AddN<F2>>::Out,
    >;
}

// literals
type N0 = End;
type N1 = One<End>;
type N2 = Zero<One<End>>;
type N3 = One<One<End>>;
type N4 = Zero<Zero<One<End>>>;
type N5 = One<Zero<One<End>>>;
type N7 = One<One<One<End>>>;

// ---------------------------------------------------------------------------
// The assertions. Every expected value below was computed in Python by P5 from
// exact rational value sets, independently of this file's arithmetic.
// ---------------------------------------------------------------------------

// U<3,5> x U<2,1>: W1 = 8, W2 = 3, neither is one, so no saving.
//   naive (5, 6) ; tight the same.
type A_N = <Num<N3, N5> as MulNaive<Num<N2, N1>>>::Out;
type A_T = <Num<N3, N5> as MulTight<Num<N2, N1>>>::Out;
const _: () = assert!(
    <A_N as Widths>::I == 5 && <A_N as Widths>::F == 6,
    "A naive"
);
const _: () = assert!(
    <A_T as Widths>::I == 5 && <A_T as Widths>::F == 6,
    "A tight"
);

// U<0,1> x U<1,0>: W1 = 1, so the saving fires.
//   naive (1, 1) = 2 bits total ; tight (0, 1) = 1 bit total.
// P5's enumeration gives least = U<0,1> for this pair; P2 names it as the
// first non-degenerate overshoot.
type B_N = <Num<N0, N1> as MulNaive<Num<N1, N0>>>::Out;
type B_T = <Num<N0, N1> as MulTight<Num<N1, N0>>>::Out;
const _: () = assert!(
    <B_N as Widths>::I == 1 && <B_N as Widths>::F == 1,
    "B naive"
);
const _: () = assert!(
    <B_T as Widths>::I == 0 && <B_T as Widths>::F == 1,
    "B tight"
);
const _: () = assert!(
    <B_N as Widths>::W == <B_T as Widths>::W + 1,
    "B saves one bit"
);

// U<0,1> x U<1,7>: the container-boundary example from P5 Q4.
//   naive total 9 bits ; tight total 8 bits.
type C_N = <Num<N0, N1> as MulNaive<Num<N1, N7>>>::Out;
type C_T = <Num<N0, N1> as MulTight<Num<N1, N7>>>::Out;
const _: () = assert!(<C_N as Widths>::W == 9, "C naive total");
const _: () = assert!(<C_T as Widths>::W == 8, "C tight total");

// U<4,4> x U<3,5>: W1 = 8, W2 = 8, no saving, tight equals naive.
type D_N = <Num<N4, N4> as MulNaive<Num<N3, N5>>>::Out;
type D_T = <Num<N4, N4> as MulTight<Num<N3, N5>>>::Out;
const _: () = assert!(<D_N as Widths>::W == 16 && <D_T as Widths>::W == 16, "D");
const _: () = assert!(
    <D_T as Widths>::I == 7 && <D_T as Widths>::F == 9,
    "D tight"
);

#[no_mangle]
pub extern "C" fn probe_entry() -> u32 {
    <A_T as Widths>::W + <B_T as Widths>::W + <C_T as Widths>::W + <D_T as Widths>::W
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
