// Probe 3: At<N, Q>'s exponent typechecks as an ordinary instance of the
// sealed exponent vocabulary. Closes the gap file 85 section 4 names: "nobody
// has yet built At<N, Q>'s exponent as a literal instance of
// EZero | EPos<P> | ENeg<P> and watched it typecheck."
//
// Model tower per the consolidation's trait table (78 section 1.23): the
// sealed Pos grammar, the sealed signed-exponent vocabulary, and an
// ExponentForm with a ranged member (a float's bounds) and a fixed member
// (fixed point's constant exponent function, the founding one-formalisation
// identity). At<M, Q> reuses M's radix, precision and domain members and picks
// Fixed<Q>; it mints no vocabulary. quantise's typed form is generic over the
// target and compiles against it.
//
// Compile-only, zero feature gates.
#![no_std]

use core::marker::PhantomData;

// -- the sealed magnitude grammar (model of 78:611-615) ---------------------
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);
pub trait Pos {
    const VAL: i64;
}
impl Pos for H {
    const VAL: i64 = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: i64 = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: i64 = 2 * P::VAL + 1;
}

// -- the sealed signed exponent (model of 78:615, Exponent ::= ...) ---------
pub struct EZero;
pub struct EPos<P>(PhantomData<P>);
pub struct ENeg<P>(PhantomData<P>);
pub trait SignedExp {
    const E: i64;
}
impl SignedExp for EZero {
    const E: i64 = 0;
}
impl<P: Pos> SignedExp for EPos<P> {
    const E: i64 = P::VAL;
}
impl<P: Pos> SignedExp for ENeg<P> {
    const E: i64 = -P::VAL;
}

// -- the exponent form: ranged (float) or fixed (fixed point) ---------------
pub trait ExponentForm {}
pub struct Ranged<Lo, Hi>(PhantomData<(Lo, Hi)>);
pub struct Fixed<E>(PhantomData<E>);
impl<Lo: SignedExp, Hi: SignedExp> ExponentForm for Ranged<Lo, Hi> {}
impl<E: SignedExp> ExponentForm for Fixed<E> {}

// -- the numeral contract (model of 78:618-623) -----------------------------
pub trait Numeral {
    const RADIX: i64;
    const PRECISION: i64;
    type Exponent: ExponentForm;
}

// A decimal float: radix 10, p = 3, exponents -2..=1.
pub struct Dec3;
impl Numeral for Dec3 {
    const RADIX: i64 = 10;
    const PRECISION: i64 = 3;
    type Exponent = Ranged<ENeg<O<H>>, EPos<H>>; // -2 ..= 1
}

// At<M, Q>: M's identity members, exponent pinned at Q. No new vocabulary:
// Fixed<Q> is the constant exponent function that fixed point already is.
pub struct At<M, Q>(PhantomData<(M, Q)>);
impl<M: Numeral, Q: SignedExp> Numeral for At<M, Q> {
    const RADIX: i64 = M::RADIX;
    const PRECISION: i64 = M::PRECISION;
    type Exponent = Fixed<Q>;
}

// The typed quantise signature: generic over the operand numeral and the
// type-level quantum, producing a datum of the target. The body is elided
// (probe 2 runs the arithmetic; this probe is the typecheck).
pub struct Datum<N: Numeral>(pub i64, PhantomData<N>);
pub fn quantise<M: Numeral, Q: SignedExp>(x: Datum<M>) -> Datum<At<M, Q>> {
    Datum(x.0, PhantomData)
}

// The witnesses: At<Dec3, EZero> is a Numeral through the ordinary bound, its
// identity members read through, and its pinned exponent is a literal member
// of the sealed vocabulary.
const fn is_numeral<N: Numeral>() -> i64 {
    N::PRECISION
}
const _: () = assert!(is_numeral::<At<Dec3, EZero>>() == 3);
const _: () = assert!(<At<Dec3, EZero> as Numeral>::RADIX == 10);
const _: () = assert!(<EZero as SignedExp>::E == 0);
const _: () = assert!(<ENeg<O<H>> as SignedExp>::E == -2);

// And the negative-quantum instance, since a quantum is any member of the
// vocabulary, not zero specially: At<Dec3, ENeg<H>> pins at 10^-1.
const _: () = assert!(is_numeral::<At<Dec3, ENeg<H>>>() == 3);
