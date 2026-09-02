// Probe 2, file 82. The exact fold width without ever materialising 2^P - 1.
//
// Probe 1 established that file 80's `foldexact` construction carries two undocumented
// ceilings, both at 128, because `AllOnes` recurses on the VALUE of P and its result's
// value is read through `Nat::VAL: u128`. binary256 (p = 237) refuses outright.
//
// The cause is not the closed form; it is that file 80 built the DEFINITION
// `bitlen(A * (2^P - 1))` rather than a closed form OF it. A closed form exists whose
// every recursion is structural (logarithmic in the value) and whose largest intermediate
// is bounded by A squared, with P appearing only as a summand:
//
//     L = bitlen(A),  R = A - 2^(L-1)
//     foldexact(P, A) = P + L - 1 + bit,   where
//         bit = 0                     if R = 0        (A a power of two)
//         bit = 1                     if R >= 1 and P >= L
//         bit = [ (R << P) >= A ]     if R >= 1 and P <  L
//
// The shift branch is entered only when P < L, so its depth is bounded by bitlen(A) and
// never by P. Verified in exact integer arithmetic over P in 1..=299 by A in 1..=4099,
// 1,225,601 cells, zero mismatches against `bitlen(A * (2^P - 1))`, including every loose
// cell file 80 names and both precisions where file 80's form dies.
//
// This probe builds that form over the same sealed grammar, with zero feature gates, and
// asserts it at compile time against an independent u128 ground truth wherever the ground
// truth is itself representable, plus at p = 113 and p = 237 where it is not.
//
// Compile: rustc --edition 2021 --crate-type=lib --emit=metadata (pinned toolchain, run
// from inside the repo tree). Outcome in OUTCOMES.md.
#![no_std]
#![allow(dead_code)]

use core::marker::PhantomData;

// ---- the sealed grammar, verbatim from 80_probes/probe_1 ----

pub trait Nat {
    const VAL: u128;
}
pub struct Z;
impl Nat for Z {
    const VAL: u128 = 0;
}
pub trait Pos: Nat {}

pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>);

impl Nat for H {
    const VAL: u128 = 1;
}
impl Pos for H {}
impl<P: Pos> Nat for O<P> {
    const VAL: u128 = 2 * P::VAL;
}
impl<P: Pos> Pos for O<P> {}
impl<P: Pos> Nat for I<P> {
    const VAL: u128 = 2 * P::VAL + 1;
}
impl<P: Pos> Pos for I<P> {}

// ---- successor and predecessor, verbatim ----

pub trait Succ: Pos {
    type Out: Pos;
}
impl Succ for H {
    type Out = O<H>;
}
impl<P: Pos> Succ for O<P> {
    type Out = I<P>;
}
impl<P: Pos> Succ for I<P>
where
    P: Succ,
{
    type Out = O<<P as Succ>::Out>;
}

pub trait DecP: Pos {
    type Out: Pos;
}
impl<Q: Pos> DecP for I<Q> {
    type Out = O<Q>;
}
impl DecP for O<H> {
    type Out = H;
}
impl<Q: Pos> DecP for O<O<Q>>
where
    O<Q>: DecP,
{
    type Out = I<<O<Q> as DecP>::Out>;
}
impl<Q: Pos> DecP for O<I<Q>> {
    type Out = I<O<Q>>;
}

// ---- addition, structural (nine constructor pairs collapse to three by symmetry) ----

pub trait AddP<Rhs: Pos>: Pos {
    type Out: Pos;
}
impl AddP<H> for H {
    type Out = O<H>;
}
impl<Q: Pos> AddP<O<Q>> for H
where
    O<Q>: Succ,
{
    type Out = <O<Q> as Succ>::Out;
}
impl<Q: Pos> AddP<I<Q>> for H
where
    I<Q>: Succ,
{
    type Out = <I<Q> as Succ>::Out;
}
impl<P: Pos> AddP<H> for O<P>
where
    O<P>: Succ,
{
    type Out = <O<P> as Succ>::Out;
}
impl<P: Pos> AddP<H> for I<P>
where
    I<P>: Succ,
{
    type Out = <I<P> as Succ>::Out;
}
impl<P: Pos, Q: Pos> AddP<O<Q>> for O<P>
where
    P: AddP<Q>,
{
    type Out = O<<P as AddP<Q>>::Out>;
}
impl<P: Pos, Q: Pos> AddP<I<Q>> for O<P>
where
    P: AddP<Q>,
{
    type Out = I<<P as AddP<Q>>::Out>;
}
impl<P: Pos, Q: Pos> AddP<O<Q>> for I<P>
where
    P: AddP<Q>,
{
    type Out = I<<P as AddP<Q>>::Out>;
}
impl<P: Pos, Q: Pos> AddP<I<Q>> for I<P>
where
    P: AddP<Q>,
    <P as AddP<Q>>::Out: Succ,
{
    // (2P+1) + (2Q+1) = 2(P+Q+1)
    type Out = O<<<P as AddP<Q>>::Out as Succ>::Out>;
}

// ---- bit length, verbatim ----

pub trait BitLen: Pos {
    type Out: Pos;
}
impl BitLen for H {
    type Out = H;
}
impl<P: BitLen> BitLen for O<P>
where
    <P as BitLen>::Out: Succ,
{
    type Out = <<P as BitLen>::Out as Succ>::Out;
}
impl<P: BitLen> BitLen for I<P>
where
    <P as BitLen>::Out: Succ,
{
    type Out = <<P as BitLen>::Out as Succ>::Out;
}

// ---- three-way structural comparison on Pos ----

pub struct Lt;
pub struct Eq;
pub struct Gt;
pub trait Ord3 {}
impl Ord3 for Lt {}
impl Ord3 for Eq {}
impl Ord3 for Gt {}

pub trait CmpP<Rhs: Pos>: Pos {
    type Out: Ord3;
}
impl CmpP<H> for H {
    type Out = Eq;
}
impl<Q: Pos> CmpP<O<Q>> for H {
    type Out = Lt;
}
impl<Q: Pos> CmpP<I<Q>> for H {
    type Out = Lt;
}
impl<P: Pos> CmpP<H> for O<P> {
    type Out = Gt;
}
impl<P: Pos> CmpP<H> for I<P> {
    type Out = Gt;
}
impl<P: CmpP<Q>, Q: Pos> CmpP<O<Q>> for O<P> {
    type Out = <P as CmpP<Q>>::Out;
}
impl<P: CmpP<Q>, Q: Pos> CmpP<I<Q>> for I<P> {
    type Out = <P as CmpP<Q>>::Out;
}
// 2P vs 2Q+1: less unless P > Q.
impl<P: CmpP<Q>, Q: Pos> CmpP<I<Q>> for O<P>
where
    <P as CmpP<Q>>::Out: DemoteToLt,
{
    type Out = <<P as CmpP<Q>>::Out as DemoteToLt>::Out;
}
// 2P+1 vs 2Q: greater unless P < Q.
impl<P: CmpP<Q>, Q: Pos> CmpP<O<Q>> for I<P>
where
    <P as CmpP<Q>>::Out: PromoteToGt,
{
    type Out = <<P as CmpP<Q>>::Out as PromoteToGt>::Out;
}

pub trait DemoteToLt: Ord3 {
    type Out: Ord3;
}
impl DemoteToLt for Lt {
    type Out = Lt;
}
impl DemoteToLt for Eq {
    type Out = Lt;
}
impl DemoteToLt for Gt {
    type Out = Gt;
}

pub trait PromoteToGt: Ord3 {
    type Out: Ord3;
}
impl PromoteToGt for Lt {
    type Out = Lt;
}
impl PromoteToGt for Eq {
    type Out = Gt;
}
impl PromoteToGt for Gt {
    type Out = Gt;
}

// ---- clear the top bit: A -> A - 2^(bitlen(A)-1), as a two-constructor kind ----
//
// A closed two-member kind rather than a `Nat`, because the result is dispatched on and
// `Z` versus `P: Pos` is not a distinction rustc's coherence can draw without negative
// reasoning. This is the same closed-vocabulary discipline the carrier-at-birth rule asks
// for, applied to an intermediate.

pub struct TopZero;
pub struct TopSome<R: Pos>(PhantomData<R>);
pub trait TopKind {}
impl TopKind for TopZero {}
impl<R: Pos> TopKind for TopSome<R> {}

pub trait DoubleTop: TopKind {
    type Out: TopKind;
}
impl DoubleTop for TopZero {
    type Out = TopZero;
}
impl<R: Pos> DoubleTop for TopSome<R> {
    type Out = TopSome<O<R>>;
}

pub trait DoubleTopPlusOne: TopKind {
    type Out: TopKind;
}
impl DoubleTopPlusOne for TopZero {
    type Out = TopSome<H>;
}
impl<R: Pos> DoubleTopPlusOne for TopSome<R> {
    type Out = TopSome<I<R>>;
}

pub trait ClearTop: Pos {
    type Out: TopKind;
}
impl ClearTop for H {
    type Out = TopZero;
}
impl<Q: ClearTop> ClearTop for O<Q>
where
    <Q as ClearTop>::Out: DoubleTop,
{
    type Out = <<Q as ClearTop>::Out as DoubleTop>::Out;
}
impl<Q: ClearTop> ClearTop for I<Q>
where
    <Q as ClearTop>::Out: DoubleTopPlusOne,
{
    type Out = <<Q as ClearTop>::Out as DoubleTopPlusOne>::Out;
}

// ---- left shift, recursing on the shift amount ----
//
// Reached only in the branch where the shift amount is strictly below bitlen(A), so its
// depth is bounded by the arity's bit length and never by the precision.

pub trait ShlP<K: Pos>: Pos {
    type Out: Pos;
}
impl<R: Pos> ShlP<H> for R {
    type Out = O<R>;
}
impl<R: Pos, Q: Pos> ShlP<O<Q>> for R
where
    O<Q>: DecP,
    R: ShlP<<O<Q> as DecP>::Out>,
{
    type Out = O<<R as ShlP<<O<Q> as DecP>::Out>>::Out>;
}
impl<R: Pos, Q: Pos> ShlP<I<Q>> for R
where
    R: ShlP<O<Q>>,
{
    type Out = O<<R as ShlP<O<Q>>>::Out>;
}

// ---- the correction bit ----

pub struct BFalse;
pub struct BTrue;
pub trait BoolT {}
impl BoolT for BFalse {}
impl BoolT for BTrue {}

// innermost: given R and the comparison of (R << P) against A, is the bit set?
pub trait ShiftVerdict: Ord3 {
    type Out: BoolT;
}
impl ShiftVerdict for Lt {
    type Out = BFalse;
}
impl ShiftVerdict for Eq {
    type Out = BTrue;
}
impl ShiftVerdict for Gt {
    type Out = BTrue;
}

// middle: dispatch on cmp(P, L). Less means the shift branch; equal or greater means set.
pub trait CorrOnCmp<P: Pos, A: Pos, R: Pos>: Ord3 {
    type Out: BoolT;
}
impl<P: Pos, A: Pos, R: Pos> CorrOnCmp<P, A, R> for Lt
where
    R: ShlP<P>,
    <R as ShlP<P>>::Out: CmpP<A>,
    <<R as ShlP<P>>::Out as CmpP<A>>::Out: ShiftVerdict,
{
    type Out = <<<R as ShlP<P>>::Out as CmpP<A>>::Out as ShiftVerdict>::Out;
}
impl<P: Pos, A: Pos, R: Pos> CorrOnCmp<P, A, R> for Eq {
    type Out = BTrue;
}
impl<P: Pos, A: Pos, R: Pos> CorrOnCmp<P, A, R> for Gt {
    type Out = BTrue;
}

// outer: dispatch on whether A is a power of two.
pub trait CorrOnTop<P: Pos, A: Pos>: TopKind {
    type Out: BoolT;
}
impl<P: Pos, A: Pos> CorrOnTop<P, A> for TopZero {
    type Out = BFalse;
}
impl<P: Pos, A: Pos, R: Pos> CorrOnTop<P, A> for TopSome<R>
where
    A: BitLen,
    P: CmpP<<A as BitLen>::Out>,
    <P as CmpP<<A as BitLen>::Out>>::Out: CorrOnCmp<P, A, R>,
{
    type Out = <<P as CmpP<<A as BitLen>::Out>>::Out as CorrOnCmp<P, A, R>>::Out;
}

// ---- assembly: P + L - 1 + bit ----

pub trait AddBit<W: Pos>: BoolT {
    type Out: Pos;
}
impl<W: Pos> AddBit<W> for BTrue {
    type Out = W;
}
impl<W: Pos> AddBit<W> for BFalse
where
    W: DecP,
{
    type Out = <W as DecP>::Out;
}

pub trait FoldExact2<A: Pos>: Pos {
    type Out: Pos;
}
impl<P, A> FoldExact2<A> for P
where
    P: Pos + BitLen,
    A: Pos + BitLen + ClearTop,
    P: AddP<<A as BitLen>::Out>,
    <A as ClearTop>::Out: CorrOnTop<P, A>,
    <<A as ClearTop>::Out as CorrOnTop<P, A>>::Out: AddBit<<P as AddP<<A as BitLen>::Out>>::Out>,
{
    // P + L, then subtract one unless the correction bit is set.
    type Out = <<<A as ClearTop>::Out as CorrOnTop<P, A>>::Out as AddBit<
        <P as AddP<<A as BitLen>::Out>>::Out,
    >>::Out;
}

// The width is a type: it takes a type position with no const machinery at all.
pub struct Acc<W: Pos>(PhantomData<W>);
pub type FoldAcc2<P, A> = Acc<<P as FoldExact2<A>>::Out>;

// ---- ground truth, u128, computed independently, where representable ----

const fn bitlen_u128(mut n: u128) -> u32 {
    let mut l = 0;
    while n > 0 {
        l += 1;
        n >>= 1;
    }
    l
}
const fn ground(p: u32, a: u128) -> u128 {
    // bitlen(a * (2^p - 1)); only called where the product fits u128.
    bitlen_u128(a * ((1u128 << p) - 1)) as u128
}

// ---- grammar literals ----

pub type P1 = H;
pub type P2 = O<H>;
pub type P3 = I<H>;
pub type P8 = O<O<O<H>>>;
pub type P11 = I<I<O<H>>>;
pub type P16 = O<O<O<O<H>>>>;
pub type P24 = O<O<O<I<H>>>>;
pub type P53 = I<O<I<O<I<H>>>>>;
pub type P113 = I<O<O<O<I<I<H>>>>>>;
pub type P237 = I<O<I<I<O<I<I<H>>>>>>>;

pub type A1 = H;
pub type A2 = O<H>;
pub type A3 = I<H>;
pub type A4 = O<O<H>>;
pub type A5 = I<O<H>>;
pub type A7 = I<I<H>>;
pub type A8 = O<O<O<H>>>;
pub type A16 = O<O<O<O<H>>>>;
pub type A64 = O<O<O<O<O<O<H>>>>>>;
pub type A100 = O<O<I<O<O<I<H>>>>>>;
pub type A256 = O<O<O<O<O<O<O<O<H>>>>>>>>;
pub type A257 = I<O<O<O<O<O<O<O<H>>>>>>>>;
pub type A513 = I<O<O<O<O<O<O<O<O<H>>>>>>>>>;
pub type A514 = O<I<O<O<O<O<O<O<O<H>>>>>>>>>;
pub type A1025 = I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>;
pub type A2049 = I<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>;
pub type A4096 = O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>;

// Literal sanity, at compile time.
const _: () = assert!(<P113 as Nat>::VAL == 113);
const _: () = assert!(<P237 as Nat>::VAL == 237);
const _: () = assert!(<A257 as Nat>::VAL == 257);
const _: () = assert!(<A2049 as Nat>::VAL == 2049);
const _: () = assert!(<A4096 as Nat>::VAL == 4096);

// ---- the matrix, asserted at compile time against ground truth ----

macro_rules! check {
    ($p:ty, $pv:expr, $a:ty, $av:expr) => {
        const _: () = assert!(
            <<$p as FoldExact2<$a>>::Out as Nat>::VAL == ground($pv, $av),
            concat!("foldexact2 mismatch at p=", $pv, " a=", $av)
        );
    };
}

// Every precision and arity file 80's probe_1 checked, so the two constructions are
// compared on the same matrix rather than on a convenient subset.
check!(P1, 1, A1, 1);
check!(P1, 1, A2, 2);
check!(P1, 1, A3, 3);
check!(P1, 1, A4, 4);
check!(P1, 1, A5, 5);
check!(P1, 1, A7, 7);
check!(P1, 1, A8, 8);
check!(P1, 1, A16, 16);
check!(P1, 1, A64, 64);
check!(P1, 1, A100, 100);
check!(P1, 1, A256, 256);
check!(P1, 1, A257, 257);
check!(P1, 1, A513, 513);
check!(P1, 1, A514, 514);
check!(P1, 1, A1025, 1025);
check!(P1, 1, A2049, 2049);
check!(P1, 1, A4096, 4096);

check!(P2, 2, A1, 1);
check!(P2, 2, A3, 3);
check!(P2, 2, A5, 5);
check!(P2, 2, A100, 100);
check!(P2, 2, A257, 257);
check!(P2, 2, A2049, 2049);

check!(P3, 3, A3, 3);
check!(P3, 3, A7, 7);
check!(P3, 3, A256, 256);
check!(P3, 3, A257, 257);

check!(P8, 8, A1, 1);
check!(P8, 8, A2, 2);
check!(P8, 8, A3, 3);
check!(P8, 8, A4, 4);
check!(P8, 8, A5, 5);
check!(P8, 8, A7, 7);
check!(P8, 8, A8, 8);
check!(P8, 8, A16, 16);
check!(P8, 8, A64, 64);
check!(P8, 8, A100, 100);
check!(P8, 8, A256, 256);
check!(P8, 8, A257, 257); // the cell where foldnum is loose by one
check!(P8, 8, A513, 513);
check!(P8, 8, A514, 514);
check!(P8, 8, A1025, 1025);
check!(P8, 8, A2049, 2049);
check!(P8, 8, A4096, 4096);

check!(P11, 11, A100, 100);
check!(P11, 11, A2049, 2049); // the second loose cell
check!(P11, 11, A4096, 4096);

check!(P16, 16, A3, 3);
check!(P16, 16, A100, 100);
check!(P16, 16, A257, 257);
check!(P16, 16, A4096, 4096);

check!(P24, 24, A100, 100);
check!(P24, 24, A4096, 4096);

check!(P53, 53, A3, 3);
check!(P53, 53, A257, 257);
check!(P53, 53, A4096, 4096);

// ---- the two precisions where file 80's construction refuses ----
//
// binary128. The u128 ground truth is still representable here for small arities, so the
// cell is checked against it rather than merely compiled.
check!(P113, 113, A3, 3);
check!(P113, 113, A4096, 4096);

// binary256. `2^237 - 1` is not representable in u128, so no `ground()` call is possible
// and the expected values are stated as literals, computed in exact integer arithmetic
// offline and reproduced in the outcomes file.
const _: () = assert!(<<P237 as FoldExact2<A3>>::Out as Nat>::VAL == 239);
const _: () = assert!(<<P237 as FoldExact2<A257>>::Out as Nat>::VAL == 246);
const _: () = assert!(<<P237 as FoldExact2<A4096>>::Out as Nat>::VAL == 249);
const _: () = assert!(<<P237 as FoldExact2<A256>>::Out as Nat>::VAL == 245);

// The width is genuinely a type at binary256, not merely a readable const.
pub type Binary256Fold = FoldAcc2<P237, A257>;

// ---- negative control: the assertions are live ----
//
// Uncommenting this must fail the build with E0080. Recorded in OUTCOMES.md.
// const _: () = assert!(<<P8 as FoldExact2<A257>>::Out as Nat>::VAL == 17);
