// Probe: the composed exact product. `Number<I: Width, F: Width, Sign>`
// carries widths as TYPES (probe 03's binary numeral), `mul_full` is one
// generic function over every width pair, and width growth chains through
// repeated multiplication and through a MAC-shaped fold without any
// per-arity or per-width duplication. `quantize` narrows back to a target
// numeral, the boundary where section 3.3's Quantisation apparatus fires.
#![allow(dead_code)]

use core::marker::PhantomData;

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

pub trait AddWidth<Rhs: Width>: Width {
    type Out: Width;
}
impl<L: Width + AddC<R, B0>, R: Width> AddWidth<R> for L {
    type Out = <L as AddC<R, B0>>::Output;
}

// --- the numeral and the number ---------------------------------------
// unsigned only, for the sketch: I integer bits, F fractional bits, raw
// value stored as the widest native container the demo needs (i128).
#[derive(Clone, Copy)]
pub struct Number<I: Width, F: Width>(pub i128, PhantomData<(I, F)>);

impl<I: Width, F: Width> Number<I, F> {
    pub const fn from_raw(raw: i128) -> Self {
        Number(raw, PhantomData)
    }
}

// the exact product: total, no rounding, widths and fractional bits add.
// ONE function, generic over every (I1,F1) x (I2,F2) pair.
pub fn mul_full<I1, F1, I2, F2>(
    a: Number<I1, F1>,
    b: Number<I2, F2>,
) -> Number<<I1 as AddWidth<I2>>::Out, <F1 as AddWidth<F2>>::Out>
where
    I1: AddWidth<I2>,
    F1: AddWidth<F2>,
    I2: Width,
    F2: Width,
{
    // raw fixed-point values already carry their own scale; the exact
    // product of the raw integers IS the raw value of the wider numeral.
    Number::from_raw(a.0 * b.0)
}

// exact sum: same numeral in, same numeral out, no width growth (matches
// the draft's Growth::Exact framing for addition; kept trivial here).
pub fn add_exact<I: Width, F: Width>(a: Number<I, F>, b: Number<I, F>) -> Number<I, F> {
    Number::from_raw(a.0 + b.0)
}

// narrowing: the ONLY place a quantiser fires. Stubbed to floor-shift here;
// section 3.3's Quantisation apparatus is what would really sit here.
pub fn quantize<SrcI: Width, SrcF: Width, DstI: Width, DstF: Width>(
    a: Number<SrcI, SrcF>,
) -> Number<DstI, DstF> {
    let shift = SrcF::VALUE as i32 - DstF::VALUE as i32;
    let raw = if shift >= 0 {
        a.0 >> shift
    } else {
        a.0 << (-shift)
    };
    Number::from_raw(raw)
}

// --- literal bridge, as probe 04 -----------------------------------------
pub trait LiteralWidth<const N: u16> {
    type W: Width;
}
pub struct Lit;
macro_rules! literal_widths {
    ($($n:literal => $ty:ty),* $(,)?) => {
        $( impl LiteralWidth<$n> for Lit { type W = $ty; } )*
    };
}
literal_widths! {
    0 => UTerm,
    2 => UInt<UInt<UTerm, B1>, B0>,
    3 => UInt<UInt<UTerm, B1>, B1>,
    4 => UInt<UInt<UInt<UTerm, B1>, B0>, B0>,
    5 => UInt<UInt<UInt<UTerm, B1>, B0>, B1>,
    6 => UInt<UInt<UInt<UTerm, B1>, B1>, B0>,
    9 => UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>,
}
pub type WidthOf<const N: u16> = <Lit as LiteralWidth<N>>::W;
pub type UFixedExact<const I: u16, const F: u16> = Number<WidthOf<I>, WidthOf<F>>;

const fn assert_eq_u16(a: u16, b: u16) {
    assert!(a == b);
}

fn main() {
    // Q(2,2) x Q(2,2) -> Q(4,4), exact, no rounding.
    let a: UFixedExact<2, 2> = Number::from_raw(0b1101); // 3.25 at F=2
    let b: UFixedExact<2, 2> = Number::from_raw(0b0110); // 1.5 at F=2
    let ab = mul_full(a, b); // width: I=4, F=4
    const _: () = assert_eq_u16(<WidthOf<2> as AddWidth<WidthOf<2>>>::Out::VALUE, 4);
    println!("ab.0 (raw, Q4.4) = {}", ab.0);

    // chain: (a * b) * c widens AGAIN. Q(4,4) x Q(2,2) -> Q(6,6).
    let c: UFixedExact<2, 2> = Number::from_raw(0b0100); // 1.0 at F=2
    let abc = mul_full(ab, c);
    println!("abc.0 (raw, Q6.6) = {}", abc.0);

    // MAC-shaped fold: sum of three exact products, all same wide numeral,
    // ONE quantisation at the very end. add_exact needs no width parameter
    // because the interior numeral does not change across the fold.
    let p1 = mul_full(
        Number::<WidthOf<3>, WidthOf<3>>::from_raw(5),
        Number::<WidthOf<3>, WidthOf<3>>::from_raw(7),
    );
    let p2 = mul_full(
        Number::<WidthOf<3>, WidthOf<3>>::from_raw(2),
        Number::<WidthOf<3>, WidthOf<3>>::from_raw(9),
    );
    let acc = add_exact(p1, p2); // width: I=6, F=6, exact integer addition
    let acc_raw = acc.0;
    let stored: UFixedExact<9, 9> = quantize(acc); // the single narrowing site
    println!("MAC acc.0 = {}, stored.0 = {}", acc_raw, stored.0);
}

#[no_mangle]
pub extern "C" fn probe_mul_full_2_2(
    a: UFixedExact<2, 2>,
    b: UFixedExact<2, 2>,
) -> UFixedExact<4, 4> {
    mul_full(a, b)
}

// --- SystemC / MATLAB firing-site test: ONE quantize, two call patterns ---
// SystemC-shaped: exact intermediates, one quantisation at assignment.
pub fn systemc_style<I1: Width, F1: Width, I2: Width, F2: Width, DI: Width, DF: Width>(
    a: Number<I1, F1>,
    b: Number<I2, F2>,
) -> Number<DI, DF>
where
    I1: AddWidth<I2>,
    F1: AddWidth<F2>,
    I2: Width,
    F2: Width,
    <I1 as AddWidth<I2>>::Out: Width,
    <F1 as AddWidth<F2>>::Out: Width,
{
    quantize(mul_full(a, b)) // the whole expression is exact until this line
}

// MATLAB ProductMode=KeepLSB(W)-shaped: same map, called immediately after
// every product rather than deferred to the eventual store. Not a second
// definition of quantisation; the SAME `quantize` function, closer to the
// multiply. Nothing about quantize's own body differs between the two.
pub fn matlab_product_mode<I1: Width, F1: Width, I2: Width, F2: Width, DI: Width, DF: Width>(
    a: Number<I1, F1>,
    b: Number<I2, F2>,
) -> Number<DI, DF>
where
    I1: AddWidth<I2>,
    F1: AddWidth<F2>,
    I2: Width,
    F2: Width,
    <I1 as AddWidth<I2>>::Out: Width,
    <F1 as AddWidth<F2>>::Out: Width,
{
    quantize(mul_full(a, b)) // identical body; the distinction is call-site placement, not a second quantiser
}
