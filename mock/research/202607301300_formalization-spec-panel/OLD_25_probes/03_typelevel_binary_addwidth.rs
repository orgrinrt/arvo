// Probe: type-level binary numerals + a from-scratch ripple-carry AddWidth,
// using ZERO unstable features. This is the typenum technique, hand-built
// here (rather than pulled from crates.io) so the mechanism itself is
// verified in this sketch rather than trusted by citation.
#![allow(dead_code)]

use core::marker::PhantomData;

// --- bits -------------------------------------------------------------
pub trait Bit {
    const VAL: u16;
}
pub struct B0;
pub struct B1;
impl Bit for B0 {
    const VAL: u16 = 0;
}
impl Bit for B1 {
    const VAL: u16 = 1;
}

// full adder truth table as a trait: self + rhs + carry-in -> (sum, carry-out)
pub trait FullAdd<Rhs: Bit, Cin: Bit> {
    type Sum: Bit;
    type Cout: Bit;
}
impl FullAdd<B0, B0> for B0 {
    type Sum = B0;
    type Cout = B0;
}
impl FullAdd<B1, B0> for B0 {
    type Sum = B1;
    type Cout = B0;
}
impl FullAdd<B0, B1> for B0 {
    type Sum = B1;
    type Cout = B0;
}
impl FullAdd<B1, B1> for B0 {
    type Sum = B0;
    type Cout = B1;
}
impl FullAdd<B0, B0> for B1 {
    type Sum = B1;
    type Cout = B0;
}
impl FullAdd<B1, B0> for B1 {
    type Sum = B0;
    type Cout = B1;
}
impl FullAdd<B0, B1> for B1 {
    type Sum = B0;
    type Cout = B1;
}
impl FullAdd<B1, B1> for B1 {
    type Sum = B1;
    type Cout = B1;
}

// --- type-level unsigned width, binary, little bit first from the leaf --
pub trait Width {
    const VALUE: u16;
}
pub struct UTerm;
pub struct UInt<Hi, Lo>(PhantomData<(Hi, Lo)>);

impl Width for UTerm {
    const VALUE: u16 = 0;
}
impl<Hi: Width, Lo: Bit> Width for UInt<Hi, Lo> {
    const VALUE: u16 = Hi::VALUE * 2 + Lo::VAL;
}

// increment by a single bit, ripples a carry through
pub trait IncBy<C: Bit> {
    type Output: Width;
}
impl IncBy<B0> for UTerm {
    type Output = UTerm;
}
impl IncBy<B1> for UTerm {
    type Output = UInt<UTerm, B1>;
}
impl<Hi: Width, Lo: Bit> IncBy<B0> for UInt<Hi, Lo> {
    type Output = UInt<Hi, Lo>;
}
impl<Hi, Lo> IncBy<B1> for UInt<Hi, Lo>
where
    Hi: Width,
    Lo: Bit + FullAdd<B1, B0>,
    Hi: IncBy<<Lo as FullAdd<B1, B0>>::Cout>,
{
    type Output =
        UInt<<Hi as IncBy<<Lo as FullAdd<B1, B0>>::Cout>>::Output, <Lo as FullAdd<B1, B0>>::Sum>;
}

// add with carry-in, the recursive ripple-carry adder
pub trait AddC<Rhs: Width, Cin: Bit> {
    type Output: Width;
}
impl<C: Bit> AddC<UTerm, C> for UTerm
where
    UTerm: IncBy<C>,
{
    type Output = <UTerm as IncBy<C>>::Output;
}
impl<Hi: Width, Lo: Bit, C: Bit> AddC<UTerm, C> for UInt<Hi, Lo>
where
    Self: IncBy<C>,
{
    type Output = <Self as IncBy<C>>::Output;
}
impl<Hi: Width, Lo: Bit, C: Bit> AddC<UInt<Hi, Lo>, C> for UTerm
where
    UInt<Hi, Lo>: IncBy<C>,
{
    type Output = <UInt<Hi, Lo> as IncBy<C>>::Output;
}
impl<Hi1, Lo1, Hi2, Lo2, C> AddC<UInt<Hi2, Lo2>, C> for UInt<Hi1, Lo1>
where
    Hi1: Width,
    Lo1: Bit + FullAdd<Lo2, C>,
    Hi2: Width,
    Lo2: Bit,
    C: Bit,
    Hi1: AddC<Hi2, <Lo1 as FullAdd<Lo2, C>>::Cout>,
{
    type Output = UInt<
        <Hi1 as AddC<Hi2, <Lo1 as FullAdd<Lo2, C>>::Cout>>::Output,
        <Lo1 as FullAdd<Lo2, C>>::Sum,
    >;
}

// the public surface: AddWidth, no carry parameter, zero unstable features
pub trait AddWidth<Rhs: Width>: Width {
    type Out: Width;
}
impl<L: Width + AddC<R, B0>, R: Width> AddWidth<R> for L {
    type Out = <L as AddC<R, B0>>::Output;
}

// --- exhaustive check on a 2-bit model (0..=3), every pair, compile-time --
type U0 = UTerm;
type U1 = UInt<UTerm, B1>;
type U2 = UInt<UInt<UTerm, B1>, B0>;
type U3 = UInt<UInt<UTerm, B1>, B1>;

const fn assert_eq_u16(a: u16, b: u16) {
    assert!(a == b);
}

const _: () = assert_eq_u16(<U0 as AddWidth<U0>>::Out::VALUE, 0);
const _: () = assert_eq_u16(<U0 as AddWidth<U1>>::Out::VALUE, 1);
const _: () = assert_eq_u16(<U0 as AddWidth<U2>>::Out::VALUE, 2);
const _: () = assert_eq_u16(<U0 as AddWidth<U3>>::Out::VALUE, 3);
const _: () = assert_eq_u16(<U1 as AddWidth<U0>>::Out::VALUE, 1);
const _: () = assert_eq_u16(<U1 as AddWidth<U1>>::Out::VALUE, 2);
const _: () = assert_eq_u16(<U1 as AddWidth<U2>>::Out::VALUE, 3);
const _: () = assert_eq_u16(<U1 as AddWidth<U3>>::Out::VALUE, 4);
const _: () = assert_eq_u16(<U2 as AddWidth<U0>>::Out::VALUE, 2);
const _: () = assert_eq_u16(<U2 as AddWidth<U1>>::Out::VALUE, 3);
const _: () = assert_eq_u16(<U2 as AddWidth<U2>>::Out::VALUE, 4);
const _: () = assert_eq_u16(<U2 as AddWidth<U3>>::Out::VALUE, 5);
const _: () = assert_eq_u16(<U3 as AddWidth<U0>>::Out::VALUE, 3);
const _: () = assert_eq_u16(<U3 as AddWidth<U1>>::Out::VALUE, 4);
const _: () = assert_eq_u16(<U3 as AddWidth<U2>>::Out::VALUE, 5);
const _: () = assert_eq_u16(<U3 as AddWidth<U3>>::Out::VALUE, 6);

// --- a realistic-shaped case: I1=13,F1=3 times I2=7,F2=2 -> I=20,F=5 ------
type UB0 = UTerm;
type UB1 = UInt<UTerm, B1>;
type UB2 = UInt<UB1, B0>;
type UB3 = UInt<UB1, B1>;
type UB4 = UInt<UB2, B0>;
type UB5 = UInt<UB2, B1>;
type UB6 = UInt<UB3, B0>;
type UB7 = UInt<UB3, B1>;
type UB13 = UInt<UB6, B1>; // 6*2+1=13
type UB20 = UInt<UB10, B0>;
type UB10 = UInt<UB5, B0>;

const _: () = assert_eq_u16(<UB13 as AddWidth<UB7>>::Out::VALUE, 20);
const _: () = assert_eq_u16(<UB3 as AddWidth<UB2>>::Out::VALUE, 5);

fn main() {
    println!("2+3 = {}", <U2 as AddWidth<U3>>::Out::VALUE);
    println!("13+7 = {}", <UB13 as AddWidth<UB7>>::Out::VALUE);
}
