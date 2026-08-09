// Probe: does the Widening axis collapse into ordinary numeral typing once
// mul_full targets a real product numeral and quantize is the only
// narrower? Extends 25_probes/05_composed_exact_product.rs's width-arithmetic
// machinery (unmodified, reused by inclusion below) with the three preset
// shapes the old Widening axis distinguished (None, InContainer,
// PerOperation), expressed with no axis at all: only which primitive is
// called and which target numeral quantize aims at.
#![allow(dead_code)]

use core::marker::PhantomData;

// --- width arithmetic, verbatim from 25_probes/03 + 05 (typenum-shaped) ---
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

// --- the numeral (raw i64 this time: native register width matters for the codegen check) ---
#[repr(transparent)]
pub struct Number<I: Width, F: Width>(pub i64, PhantomData<(I, F)>);
impl<I: Width, F: Width> Clone for Number<I, F> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<I: Width, F: Width> Copy for Number<I, F> {}
impl<I: Width, F: Width> Number<I, F> {
    pub const fn from_raw(raw: i64) -> Self {
        Number(raw, PhantomData)
    }
}

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
    Number::from_raw(a.0.wrapping_mul(b.0))
}

pub fn quantize_wrap<SrcI: Width, SrcF: Width, DstI: Width, DstF: Width>(
    a: Number<SrcI, SrcF>,
) -> Number<DstI, DstF> {
    // Hot's own resolution: ReduceModulo. narrowing by masking, matching
    // what a plain wrapping op on the ORIGINAL width already computes.
    let shift = SrcF::VALUE as i64 - DstF::VALUE as i64;
    let raw = if shift >= 0 {
        a.0 >> shift
    } else {
        a.0 << (-shift)
    };
    Number::from_raw(raw)
}

pub trait LiteralWidth<const N: u16> {
    type W: Width;
}
pub struct Lit;
macro_rules! literal_widths {
    ($($n:literal => $ty:ty),* $(,)?) => { $( impl LiteralWidth<$n> for Lit { type W = $ty; } )* };
}
literal_widths! {
    0 => UTerm,
    16 => UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>,
    32 => UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>,
}
pub type WidthOf<const N: u16> = <Lit as LiteralWidth<N>>::W;
pub type NumOf<const I: u16, const F: u16> = Number<WidthOf<I>, WidthOf<F>>;

const fn assert_eq_u16(a: u16, b: u16) {
    assert!(a == b);
}
// the Lattner gap, stated in the new vocabulary: does asking for the
// widened width of a 16-bit numeral, doubled, actually land at 32? this is
// the ONLY fact "Widening" used to gate; here it is a plain type equation,
// always inhabited, no compatibility predicate needed.
const _: () = assert_eq_u16(<WidthOf<16> as AddWidth<WidthOf<16>>>::Out::VALUE, 32);

// === the three old Widening instances, expressed with no axis at all ===

// Widening::None (Hot): no software-visible wide intermediate at all.
// this is NOT "mul_full then quantize with a target the same as the
// source"; it is a DIFFERENT, narrower primitive that never forms the
// wide numeral in the first place. one instruction on every ISA arvo
// targets (a plain wrapping multiply already discards the unrepresented
// high bits; there is nothing to keep room for).
#[no_mangle]
pub extern "C" fn hot_mul_direct(a: NumOf<16, 0>, b: NumOf<16, 0>) -> NumOf<16, 0> {
    Number::from_raw(a.0.wrapping_mul(b.0))
}

// the SAME preset, but forced through the composite mul_full + quantize
// path anyway (as if a consumer, or a future combinator, always called the
// exact primitive and immediately narrowed with the ReduceModulo
// resolution). if Widening::None ever bought codegen that this composite
// form cannot reach on its own, this function's disassembly will differ
// from hot_mul_direct's.
#[no_mangle]
pub extern "C" fn hot_mul_via_full_then_quantize(a: NumOf<16, 0>, b: NumOf<16, 0>) -> NumOf<16, 0> {
    let wide = mul_full(a, b); // NumOf<32,0>, a real, named numeral type
    quantize_wrap(wide) // narrows back to NumOf<16,0>, ReduceModulo
}

// Widening::InContainer-equivalent: the ORIGINAL numeral's own container is
// over-allocated (StoredWidth::DoubleLogical), independent of any op. the
// product numeral (32 bits) simply fits inside that over-allocation with no
// separate "where does headroom come from" fact needed; it is a property
// of NumOf<16,0>'s own Lowering, orthogonal to which arithmetic op runs.
#[repr(transparent)]
pub struct OverAllocated16(pub Number<WidthOf<16>, WidthOf<0>>); // physical container: i64, logical width: 16

// Widening::PerOperation-equivalent (Precise): a fresh, wider accumulator
// numeral is named explicitly per call site. this is exactly mul_full's
// own return type; "per operation" was always just "the type mul_full
// hands back", now visible instead of implied by an axis.
#[no_mangle]
pub extern "C" fn precise_mul_widens(a: NumOf<16, 0>, b: NumOf<16, 0>) -> NumOf<32, 0> {
    mul_full(a, b)
}

fn main() {
    let a = NumOf::<16, 0>::from_raw(1234);
    let b = NumOf::<16, 0>::from_raw(5678);
    let direct = hot_mul_direct(a, b);
    let composite = hot_mul_via_full_then_quantize(a, b);
    assert_eq!(
        direct.0, composite.0,
        "Hot direct and composite disagree on VALUE"
    );
    println!(
        "direct = {}, composite = {}, wide = {}",
        direct.0,
        composite.0,
        precise_mul_widens(a, b).0
    );
    println!("OK: value agreement holds; disassembly comparison follows in the harness script");
}
