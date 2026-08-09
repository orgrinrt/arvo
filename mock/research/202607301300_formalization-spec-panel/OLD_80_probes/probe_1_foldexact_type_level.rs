// Probe 1, file 80. The exact fold-width closed form, built at the type level.
//
// Hypothesis: `foldexact(P, A) = bitlen(A * (2^P - 1))`, the exact minimal width for a
// fold of A operands of P bits each (file 64 section 1 states the closed form and does
// not build it), is expressible over the sealed `Nat`/`Pos` grammar with zero feature
// gates: no `generic_const_exprs`, no `min_generic_const_args`, associated-type
// structural recursion only, exactly the construction family `VAL`/`Cmp`/`Gcd`/`Dec`
// already use (`68:654-657`, `79` section 5).
//
// Every instantiated cell is checked at compile time against a u128 ground truth
// computed independently by const fn, and against `foldnum(P, A) = P + ceil(log2 A)`
// (file 55's formula) for the sufficiency and the at-most-one-bit-loose bound file 64
// characterised. The named loose cells at p = 8 (257, 513, 514, 1025, 2049) and p = 11
// (2049) are asserted loose by exactly one bit; every power-of-two arity is asserted
// exactly tight. The result is also placed in type position (`Acc<W>`) to establish
// that the computed width is a type, per the spine rule, not merely a const.
//
// Compile: rustc --edition 2021 --crate-type=lib --emit=metadata (pinned toolchain,
// run from inside the repo tree). Outcome and timing in OUTCOMES.md.
#![no_std]
#![allow(dead_code)]

use core::marker::PhantomData;

// ---- the sealed grammar, standalone stand-in (same shape every prior probe uses) ----

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

// ---- successor: the carry chain ----

pub trait Succ: Pos {
    type Out: Pos;
}
impl Succ for H {
    type Out = O<H>;
}
impl<P: Pos> Succ for O<P> {
    type Out = I<P>;
}
impl<P: Succ> Succ for I<P> {
    type Out = O<P::Out>;
}

// ---- addition: nine constructor pairs, standard binary full adder ----

pub trait AddP<Rhs: Pos>: Pos {
    type Out: Pos;
}
impl AddP<H> for H {
    type Out = O<H>;
}
impl<Q: Pos> AddP<O<Q>> for H {
    type Out = I<Q>;
}
impl<Q: Succ> AddP<I<Q>> for H {
    type Out = O<Q::Out>;
}
impl<P: Pos> AddP<H> for O<P> {
    type Out = I<P>;
}
impl<P: AddP<Q>, Q: Pos> AddP<O<Q>> for O<P> {
    type Out = O<<P as AddP<Q>>::Out>;
}
impl<P: AddP<Q>, Q: Pos> AddP<I<Q>> for O<P> {
    type Out = I<<P as AddP<Q>>::Out>;
}
impl<P: Succ> AddP<H> for I<P> {
    type Out = O<P::Out>;
}
impl<P: AddP<Q>, Q: Pos> AddP<O<Q>> for I<P> {
    type Out = I<<P as AddP<Q>>::Out>;
}
impl<P: AddP<Q>, Q: Pos> AddP<I<Q>> for I<P>
where
    <P as AddP<Q>>::Out: Succ,
{
    // (2P+1) + (2Q+1) = 2(P+Q+1)
    type Out = O<<<P as AddP<Q>>::Out as Succ>::Out>;
}

// ---- multiplication: shift-and-add over the left operand's bits ----

pub trait MulP<Rhs: Pos>: Pos {
    type Out: Pos;
}
impl<B: Pos> MulP<B> for H {
    type Out = B;
}
impl<P: MulP<B>, B: Pos> MulP<B> for O<P> {
    type Out = O<<P as MulP<B>>::Out>;
}
impl<P: MulP<B>, B: Pos> MulP<B> for I<P>
where
    O<<P as MulP<B>>::Out>: AddP<B>,
{
    // (2P+1)B = 2(PB) + B
    type Out = <O<<P as MulP<B>>::Out> as AddP<B>>::Out;
}

// ---- predecessor on Pos >= 2 (file 79's construction, rebuilt) ----

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
    // 2(2Q+1) - 1 = 4Q + 1
    type Out = I<O<Q>>;
}

// ---- 2^P - 1 as P ones: recursion on the value of P through DecP ----

pub trait AllOnes: Pos {
    type Out: Pos;
}
impl AllOnes for H {
    type Out = H;
}
impl<Q: Pos> AllOnes for O<Q>
where
    O<Q>: DecP,
    <O<Q> as DecP>::Out: AllOnes,
{
    type Out = I<<<O<Q> as DecP>::Out as AllOnes>::Out>;
}
impl<Q: Pos> AllOnes for I<Q>
where
    O<Q>: AllOnes,
{
    // Dec(I<Q>) = O<Q>, and AllOnes recurses through O<Q>'s own impl directly.
    type Out = I<<O<Q> as AllOnes>::Out>;
}

// ---- bit length: structural depth plus one per constructor ----

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

// ---- the exact fold width, assembled: bitlen(A * (2^P - 1)) ----

pub trait FoldExact<A: Pos>: Pos {
    type Out: Pos;
}
impl<P, A> FoldExact<A> for P
where
    P: AllOnes,
    A: MulP<<P as AllOnes>::Out>,
    <A as MulP<<P as AllOnes>::Out>>::Out: BitLen,
{
    type Out = <<A as MulP<<P as AllOnes>::Out>>::Out as BitLen>::Out;
}

// The width is a type: it takes a type position with no const machinery at all.
pub struct Acc<W: Pos>(PhantomData<W>);
pub type FoldAcc<P, A> = Acc<<P as FoldExact<A>>::Out>;

// ---- ground truth, u128, computed independently ----

const fn bitlen_u128(mut n: u128) -> u32 {
    let mut b = 0;
    while n > 0 {
        n >>= 1;
        b += 1;
    }
    b
}
/// Exact minimal bits for a sum of `a` operands of `p` bits each: bitlen(a * (2^p - 1)).
const fn exact_bits(p: u32, a: u128) -> u32 {
    bitlen_u128(a * (((1u128) << p) - 1))
}
/// File 55's formula: p + ceil(log2 a).
const fn foldnum(p: u32, a: u128) -> u32 {
    let cl = if a <= 1 { 0 } else { bitlen_u128(a - 1) };
    p + cl
}

// ---- the matrix ----

// arity literals (outermost constructor is the least significant bit)
type C1 = H;
type C2 = O<H>;
type C3 = I<H>;
type C4 = O<O<H>>;
type C5 = I<O<H>>;
type C7 = I<I<H>>;
type C8 = O<C4>;
type C16 = O<C8>;
type C64 = O<O<O<O<O<O<H>>>>>>;
type C100 = O<O<I<O<O<I<H>>>>>>;
type C256 = O<O<C64>>;
type C257 = I<O<O<O<O<O<O<O<H>>>>>>>>;
type C513 = I<O<O<O<O<O<O<O<O<H>>>>>>>>>;
type C514 = O<C257>;
type C1024 = O<O<C256>>;
type C1025 = I<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>;
type C2048 = O<C1024>;
type C2049 = I<C1024>;
type C4096 = O<C2048>;

// precision literals
type P1 = H;
type P2 = O<H>;
type P3 = I<H>;
type P8 = O<O<O<H>>>;
type P11 = I<I<O<H>>>;
type P16 = O<O<O<O<H>>>>;

macro_rules! cell {
    ($($P:ty , $A:ty);* $(;)?) => {
        const _: () = {
            $(
                {
                    let p = <$P as Nat>::VAL as u32;
                    let a = <$A as Nat>::VAL;
                    let tl = <<$P as FoldExact<$A>>::Out as Nat>::VAL as u32;
                    let exact = exact_bits(p, a);
                    // the type-level form computes the exact closed form
                    assert!(tl == exact);
                    // file 55's formula is sufficient and at most one bit loose
                    let fnum = foldnum(p, a);
                    assert!(exact <= fnum);
                    assert!(fnum - exact <= 1);
                }
            )*
        };
    };
}

// Every cell instantiated is checked against the independent ground truth; the sweep
// covers every behaviour class file 64 characterised: A = 1, powers of two (always
// tight), tight non-powers (3 at p = 2, the counterexample to the naive converse),
// and the loose band (257, 513, 514, 1025, 2049 at p = 8; 2049 at p = 11).
cell!(
    P1, C1; P1, C2; P1, C3; P1, C4; P1, C5; P1, C7; P1, C8; P1, C16; P1, C64;
    P1, C100; P1, C256; P1, C257; P1, C513; P1, C514; P1, C1024; P1, C1025;
    P1, C2048; P1, C2049; P1, C4096;
    P2, C1; P2, C2; P2, C3; P2, C4; P2, C5; P2, C7; P2, C8; P2, C16; P2, C64;
    P2, C100; P2, C256; P2, C257; P2, C513; P2, C514; P2, C1024; P2, C1025;
    P2, C2048; P2, C2049; P2, C4096;
    P3, C1; P3, C2; P3, C3; P3, C4; P3, C5; P3, C7; P3, C8; P3, C16; P3, C64;
    P3, C100; P3, C256; P3, C257; P3, C513; P3, C514; P3, C1024; P3, C1025;
    P3, C2048; P3, C2049; P3, C4096;
    P8, C1; P8, C2; P8, C3; P8, C4; P8, C5; P8, C7; P8, C8; P8, C16; P8, C64;
    P8, C100; P8, C256; P8, C257; P8, C513; P8, C514; P8, C1024; P8, C1025;
    P8, C2048; P8, C2049; P8, C4096;
    P11, C1; P11, C2; P11, C3; P11, C4; P11, C5; P11, C7; P11, C8; P11, C16;
    P11, C64; P11, C100; P11, C256; P11, C257; P11, C513; P11, C514; P11, C1024;
    P11, C1025; P11, C2048; P11, C2049; P11, C4096;
    P16, C1; P16, C2; P16, C3; P16, C4; P16, C5; P16, C7; P16, C8; P16, C16;
    P16, C64; P16, C100; P16, C256; P16, C257; P16, C513; P16, C514; P16, C1024;
    P16, C1025; P16, C2048; P16, C2049; P16, C4096;
);

// The characterised loose cells, asserted loose by exactly one bit, so a later
// tightening of foldnum cannot silently mask a regression in either formula.
macro_rules! loose_by_one {
    ($($P:ty , $A:ty);* $(;)?) => {
        const _: () = {
            $(
                {
                    let p = <$P as Nat>::VAL as u32;
                    let a = <$A as Nat>::VAL;
                    assert!(foldnum(p, a) - exact_bits(p, a) == 1);
                    // and the type-level form delivers the tight width
                    let tl = <<$P as FoldExact<$A>>::Out as Nat>::VAL as u32;
                    assert!(tl + 1 == foldnum(p, a));
                }
            )*
        };
    };
}
loose_by_one!(P8, C257; P8, C513; P8, C514; P8, C1025; P8, C2049; P11, C2049);

// Powers of two are exactly tight at every precision instantiated.
macro_rules! tight {
    ($($P:ty , $A:ty);* $(;)?) => {
        const _: () = {
            $(
                assert!(
                    foldnum(<$P as Nat>::VAL as u32, <$A as Nat>::VAL)
                        == exact_bits(<$P as Nat>::VAL as u32, <$A as Nat>::VAL)
                );
            )*
        };
    };
}
tight!(
    P1, C2; P2, C4; P3, C8; P8, C16; P8, C64; P8, C256; P8, C1024; P8, C4096;
    P11, C2048; P16, C1024;
    // and the counterexample to the naive converse: a tight NON-power (64:71-76)
    P2, C3;
);

// The spine-rule half: the computed width sits in type position, no gates.
pub type Fold257At8 = FoldAcc<P8, C257>;
const _: () = {
    // 257 operands of 8 bits: exact width is 16 (foldnum says 17)
    assert!(<<P8 as FoldExact<C257>>::Out as Nat>::VAL == 16);
};
