//! Probe 1: does file 35's Widening-collapse measurement survive file 36's
//! encoding replacement?
//!
//! File 35's probe 1 measured that `mul_full` into a named product numeral,
//! followed by `quantize` with `ReduceModulo`, folds to the same code as a
//! direct wrapping multiply, and concluded the `Widening` axis bought no
//! codegen distinction. Its width machinery was `25_probes/03+05`'s
//! `UTerm`/`UInt` chain. File 36 then replaced that chain with the
//! value-unique `Z`/`Pz`/`H`/`O`/`I` encoding and stated that "nothing here
//! disturbs" file 35's shapes (`36:362-364`). That statement was reasoned,
//! not compiled: file 35's measurement and file 36's encoding have never
//! been in one compilation unit. This probe puts them there.
//!
//! Construction: the value-unique encoding and its 18-impl carry adder,
//! verbatim from `36_probes/probe_2_value_unique_naturals_by_construction.rs`
//! (`Pos`/`Nat`/`Succ`/`PAdd`/`NAdd`), carrying file 35's probe-1 shape on
//! top: `Number<I, F>` phantom over `i64`, `mul_full` computing its product
//! widths through `NAdd`, `quantize_wrap` narrowing with the `ReduceModulo`
//! resolution, and the direct wrapping multiply as the control.
//!
//! Checks:
//!   1. (value) direct and composite agree at runtime.
//!   2. (type)  the product width of 16 + 16 is the type spelling 32, as a
//!      type-equality demand, on the new encoding.
//!   3. (codegen) at `-C opt-level=3`, no LTO (file 34's corrected shape A
//!      for codegen-quality questions), the three bodies fold: inspected on
//!      the emitted asm by the harness commands recorded in OUTCOMES.md.
//!
//! Build: rustc --edition 2021 -C opt-level=3 --emit=asm -o probe_1.s \
//!        probe_1_collapse_survives_the_encoding_swap.rs  (plus a -O run
//!        of the binary for the value check)
//! Outcome: recorded in OUTCOMES.md.

#![allow(dead_code)]

use core::marker::PhantomData;

// --- the value-unique encoding, verbatim from 36_probes/probe_2 ---

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

pub trait Succ {
    type Out: Pos;
}
impl Succ for H {
    type Out = O<H>;
}
impl<P: Pos> Succ for O<P> {
    type Out = I<P>;
}
impl<P: Pos + Succ> Succ for I<P> {
    type Out = O<<P as Succ>::Out>;
}

pub struct C0;
pub struct C1;

pub trait PAdd<Rhs, C> {
    type Out: Pos;
}
impl PAdd<H, C0> for H {
    type Out = O<H>;
}
impl PAdd<H, C1> for H {
    type Out = I<H>;
}
impl<B: Pos> PAdd<O<B>, C0> for H {
    type Out = I<B>;
}
impl<B: Pos + Succ> PAdd<O<B>, C1> for H {
    type Out = O<<B as Succ>::Out>;
}
impl<B: Pos + Succ> PAdd<I<B>, C0> for H {
    type Out = O<<B as Succ>::Out>;
}
impl<B: Pos + Succ> PAdd<I<B>, C1> for H {
    type Out = I<<B as Succ>::Out>;
}
impl<A: Pos> PAdd<H, C0> for O<A> {
    type Out = I<A>;
}
impl<A: Pos + Succ> PAdd<H, C1> for O<A> {
    type Out = O<<A as Succ>::Out>;
}
impl<A: Pos + Succ> PAdd<H, C0> for I<A> {
    type Out = O<<A as Succ>::Out>;
}
impl<A: Pos + Succ> PAdd<H, C1> for I<A> {
    type Out = I<<A as Succ>::Out>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<O<B>, C0> for O<A> {
    type Out = O<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<O<B>, C1> for O<A> {
    type Out = I<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<I<B>, C0> for O<A> {
    type Out = I<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<I<B>, C1> for O<A> {
    type Out = O<<A as PAdd<B, C1>>::Out>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> PAdd<O<B>, C0> for I<A> {
    type Out = I<<A as PAdd<B, C0>>::Out>;
}
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<O<B>, C1> for I<A> {
    type Out = O<<A as PAdd<B, C1>>::Out>;
}
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<I<B>, C0> for I<A> {
    type Out = O<<A as PAdd<B, C1>>::Out>;
}
impl<A: Pos + PAdd<B, C1>, B: Pos> PAdd<I<B>, C1> for I<A> {
    type Out = I<<A as PAdd<B, C1>>::Out>;
}

pub trait NAdd<Rhs> {
    type Out: Nat;
}
impl NAdd<Z> for Z {
    type Out = Z;
}
impl<B: Pos> NAdd<Pz<B>> for Z {
    type Out = Pz<B>;
}
impl<A: Pos> NAdd<Z> for Pz<A> {
    type Out = Pz<A>;
}
impl<A: Pos + PAdd<B, C0>, B: Pos> NAdd<Pz<B>> for Pz<A> {
    type Out = Pz<<A as PAdd<B, C0>>::Out>;
}

// --- file 35's probe-1 shape, carried over unchanged in substance ---

#[repr(transparent)]
pub struct Number<In: Nat, F: Nat>(pub i64, PhantomData<(In, F)>);
impl<In: Nat, F: Nat> Clone for Number<In, F> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<In: Nat, F: Nat> Copy for Number<In, F> {}
impl<In: Nat, F: Nat> Number<In, F> {
    pub const fn from_raw(raw: i64) -> Self {
        Number(raw, PhantomData)
    }
}

pub fn mul_full<I1, F1, I2, F2>(
    a: Number<I1, F1>,
    b: Number<I2, F2>,
) -> Number<<I1 as NAdd<I2>>::Out, <F1 as NAdd<F2>>::Out>
where
    I1: Nat + NAdd<I2>,
    F1: Nat + NAdd<F2>,
    I2: Nat,
    F2: Nat,
{
    Number::from_raw(a.0.wrapping_mul(b.0))
}

pub fn quantize_wrap<SrcI: Nat, SrcF: Nat, DstI: Nat, DstF: Nat>(
    a: Number<SrcI, SrcF>,
) -> Number<DstI, DstF> {
    // hot's own resolution: ReduceModulo, matching what a plain wrapping op
    // on the original width already computes.
    let shift = SrcF::VAL as i64 - DstF::VAL as i64;
    let raw = if shift >= 0 {
        a.0 >> shift
    } else {
        a.0 << (-shift)
    };
    Number::from_raw(raw)
}

// widths 16 and 32 in the value-unique spelling.
// 16 = 0b10000 = O<O<O<O<H>>>>; 32 = 0b100000 = O<O<O<O<O<H>>>>>.
pub type W16 = Pz<O<O<O<O<H>>>>>;
pub type W32 = Pz<O<O<O<O<O<H>>>>>>;
pub type Num16 = Number<W16, Z>;
pub type Num32 = Number<W32, Z>;

// the type-level check: 16 + 16 lands, as a TYPE, on the unique spelling of
// 32. under the old encoding this equality held only for the spellings the
// adder happened to produce (36's probe 1b is the refusal); here there is
// one spelling, so the demand is exact.
const fn same_type<T>(_: PhantomData<T>, _: PhantomData<T>) {}
const _: () = same_type(PhantomData::<<W16 as NAdd<W16>>::Out>, PhantomData::<W32>);
const _: () = assert!(<W16 as NAdd<W16>>::Out::VAL == 32);

// === the three shapes file 35 measured, on the new encoding ===

#[no_mangle]
pub extern "C" fn hot_mul_direct(a: Num16, b: Num16) -> Num16 {
    Number::from_raw(a.0.wrapping_mul(b.0))
}

#[no_mangle]
pub extern "C" fn hot_mul_via_full_then_quantize(a: Num16, b: Num16) -> Num16 {
    let wide = mul_full(a, b); // Number<W32, Z>, the named product numeral
    quantize_wrap(wide)
}

#[no_mangle]
pub extern "C" fn precise_mul_widens(a: Num16, b: Num16) -> Num32 {
    mul_full(a, b)
}

fn main() {
    let a = Num16::from_raw(1234);
    let b = Num16::from_raw(5678);
    let direct = hot_mul_direct(a, b);
    let composite = hot_mul_via_full_then_quantize(a, b);
    assert_eq!(direct.0, composite.0, "direct and composite disagree");
    println!(
        "OK: direct = {}, composite = {}, wide = {}",
        direct.0,
        composite.0,
        precise_mul_widens(a, b).0
    );
}
