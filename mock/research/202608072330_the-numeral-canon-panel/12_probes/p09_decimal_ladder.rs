//! p09. Attacking p05's finding, route 2: make the tower LEGIBLE.
//!
//! p05 shows a nat-keyed numeral prints `D1<D0<D1<D1<Term>>>>` for 13, and p06's
//! recovery only fixes the headline label; the notes still carry towers. There is
//! no diagnostic attribute for E0308 (checked: `diagnostic::on_type_error` is an
//! unknown attribute on this toolchain, see out/p09_attr.log), so the message
//! cannot be overridden. The remaining lever is the SPELLING rustc prints.
//!
//! A reader decoding `D1<D0<D1<D1<Term>>>>` does two things: read little-endian,
//! and convert base two. This file removes the second by moving the ladder to
//! base ten. 13 becomes `T<N3, T<N1, E>>`, which is the digits of the number,
//! reversed, and nothing else.
//!
//! The arithmetic has to survive the move or the idea is worthless, so this file
//! carries structural base-ten addition with carry, complete, and checks it
//! against `Nat::V` at eleven points including two carry chains that lengthen the
//! tower. No width appears anywhere. The only table is a table OF DIGITS, closed
//! at ten, which is what the binary ladder's sixteen Add impls already are at
//! base two.
//!
//! rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
//!       --emit=metadata -o out/p09.meta p09_decimal_ladder.rs
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

// --- the tower: a cell of one digit and a tail, little endian ----------------
pub struct E;
pub struct T<D, R>(PhantomData<(D, R)>);

pub struct N0;
pub struct N1;
pub struct N2;
pub struct N3;
pub struct N4;
pub struct N5;
pub struct N6;
pub struct N7;
pub struct N8;
pub struct N9;

// --- the value, so every claim below is checked against arithmetic -----------
pub trait Digit {
    const D: u32;
}
macro_rules! dv { ($($d:ident => $v:expr),* $(,)?) => { $( impl Digit for $d { const D: u32 = $v; } )* } }
dv! { N0 => 0, N1 => 1, N2 => 2, N3 => 3, N4 => 4, N5 => 5, N6 => 6, N7 => 7, N8 => 8, N9 => 9 }

pub trait Nat {
    const V: u32;
}
impl Nat for E {
    const V: u32 = 0;
}
impl<D: Digit, R: Nat> Nat for T<D, R> {
    const V: u32 = D::D + 10 * R::V;
}

// --- digit magnitude as a unary tally, so digit addition is tally addition ---
pub struct Z;
pub struct S<X>(PhantomData<X>);
pub type U0 = Z;
pub type U1 = S<U0>;
pub type U2 = S<U1>;
pub type U3 = S<U2>;
pub type U4 = S<U3>;
pub type U5 = S<U4>;
pub type U6 = S<U5>;
pub type U7 = S<U6>;
pub type U8 = S<U7>;
pub type U9 = S<U8>;

pub trait Mag {
    type M;
}
macro_rules! mg { ($($d:ident => $m:ty),* $(,)?) => { $( impl Mag for $d { type M = $m; } )* } }
mg! { N0 => U0, N1 => U1, N2 => U2, N3 => U3, N4 => U4, N5 => U5, N6 => U6, N7 => U7, N8 => U8, N9 => U9 }

pub trait AddU<R> {
    type O;
}
impl<R> AddU<R> for Z {
    type O = R;
}
impl<L: AddU<R>, R> AddU<R> for S<L> {
    type O = S<<L as AddU<R>>::O>;
}

// --- the normaliser: a tally in 0..=19 becomes a digit plus a carry ----------
// Twenty rows. This is the whole digit table, and it names no width.
pub struct C0;
pub struct C1;
pub trait Norm {
    type Put;
    type Carry;
}
macro_rules! nm { ($($u:ty => $d:ty, $c:ty);* $(;)?) => { $(
    impl Norm for $u { type Put = $d; type Carry = $c; } )* } }
pub type U10 = S<U9>;
pub type U11 = S<U10>;
pub type U12 = S<U11>;
pub type U13 = S<U12>;
pub type U14 = S<U13>;
pub type U15 = S<U14>;
pub type U16 = S<U15>;
pub type U17 = S<U16>;
pub type U18 = S<U17>;
pub type U19 = S<U18>;
nm! {
    U0 => N0, C0; U1 => N1, C0; U2 => N2, C0; U3 => N3, C0; U4 => N4, C0;
    U5 => N5, C0; U6 => N6, C0; U7 => N7, C0; U8 => N8, C0; U9 => N9, C0;
    U10 => N0, C1; U11 => N1, C1; U12 => N2, C1; U13 => N3, C1; U14 => N4, C1;
    U15 => N5, C1; U16 => N6, C1; U17 => N7, C1; U18 => N8, C1; U19 => N9, C1;
}

// --- tower addition, one impl per (carry-in, shape) --------------------------
pub trait Add<B> {
    type O;
}
pub trait AddC<B> {
    type O;
} // self + B + 1

impl<B> Add<B> for E {
    type O = B;
}
impl<D, R> Add<E> for T<D, R> {
    type O = T<D, R>;
}
impl<D1, R1, D2, R2> Add<T<D2, R2>> for T<D1, R1>
where
    D1: Mag,
    D2: Mag,
    <D1 as Mag>::M: AddU<<D2 as Mag>::M>,
    <<D1 as Mag>::M as AddU<<D2 as Mag>::M>>::O: Norm,
    R1: Chain<<<<D1 as Mag>::M as AddU<<D2 as Mag>::M>>::O as Norm>::Carry, R2>,
{
    type O = T<
        <<<D1 as Mag>::M as AddU<<D2 as Mag>::M>>::O as Norm>::Put,
        <R1 as Chain<<<<D1 as Mag>::M as AddU<<D2 as Mag>::M>>::O as Norm>::Carry, R2>>::O,
    >;
}

// carry-in one: shift the digit sum by one tally step, same normaliser.
impl AddC<E> for E {
    type O = T<N1, E>;
}
impl<D2, R2> AddC<T<D2, R2>> for E
where
    T<D2, R2>: Add<T<N1, E>>,
{
    type O = <T<D2, R2> as Add<T<N1, E>>>::O;
}
impl<D1, R1> AddC<E> for T<D1, R1>
where
    T<D1, R1>: Add<T<N1, E>>,
{
    type O = <T<D1, R1> as Add<T<N1, E>>>::O;
}
impl<D1, R1, D2, R2> AddC<T<D2, R2>> for T<D1, R1>
where
    D1: Mag,
    D2: Mag,
    <D1 as Mag>::M: AddU<<D2 as Mag>::M>,
    S<<<D1 as Mag>::M as AddU<<D2 as Mag>::M>>::O>: Norm,
    R1: Chain<<S<<<D1 as Mag>::M as AddU<<D2 as Mag>::M>>::O> as Norm>::Carry, R2>,
{
    type O = T<
        <S<<<D1 as Mag>::M as AddU<<D2 as Mag>::M>>::O> as Norm>::Put,
        <R1 as Chain<<S<<<D1 as Mag>::M as AddU<<D2 as Mag>::M>>::O> as Norm>::Carry, R2>>::O,
    >;
}

// the carry selector: two impls, which is what makes the digit case one impl.
pub trait Chain<C, B> {
    type O;
}
impl<A: Add<B>, B> Chain<C0, B> for A {
    type O = <A as Add<B>>::O;
}
impl<A: AddC<B>, B> Chain<C1, B> for A {
    type O = <A as AddC<B>>::O;
}

// --- legibility, which is the reason for all of the above --------------------
pub type Thirteen = T<N3, T<N1, E>>;
pub type Three = T<N3, E>;
pub type TwentySix = T<N6, T<N2, E>>;
pub type Six = T<N6, E>;
pub type TwentyFour = T<N4, T<N2, E>>;
pub type Eight = T<N8, E>;
pub type SevenSevenSeven = T<N7, T<N7, T<N7, E>>>;

const _: () = {
    assert!(<Thirteen as Nat>::V == 13);
    assert!(<TwentySix as Nat>::V == 26);
    assert!(<SevenSevenSeven as Nat>::V == 777);
};

// --- the arithmetic, checked, including two carry chains ---------------------
pub type Sum<A, B> = <A as Add<B>>::O;
const _: () = {
    assert!(<Sum<Thirteen, Thirteen> as Nat>::V == 26);
    assert!(<Sum<Three, Three> as Nat>::V == 6);
    assert!(<Sum<TwentyFour, TwentyFour> as Nat>::V == 48);
    assert!(<Sum<Eight, Eight> as Nat>::V == 16); // digit carry, tower lengthens
    assert!(<Sum<Thirteen, Three> as Nat>::V == 16);
    assert!(<Sum<SevenSevenSeven, SevenSevenSeven> as Nat>::V == 1554);
    assert!(<Sum<T<N9, T<N9, E>>, T<N1, E>> as Nat>::V == 100); // carry chain
    assert!(<Sum<Sum<TwentyFour, TwentyFour>, Sum<TwentyFour, TwentyFour>> as Nat>::V == 96);
    assert!(<Sum<E, Thirteen> as Nat>::V == 13);
    assert!(<Sum<Thirteen, E> as Nat>::V == 13);
};

// --- canonicity: a computed tower IS the written tower, as in p01 ------------
pub fn canonical_26(x: PhantomData<Sum<Thirteen, Thirteen>>) -> PhantomData<TwentySix> {
    x
}
pub fn canonical_48(x: PhantomData<Sum<TwentyFour, TwentyFour>>) -> PhantomData<T<N8, T<N4, E>>> {
    x
}
pub fn canonical_16(x: PhantomData<Sum<Eight, Eight>>) -> PhantomData<T<N6, T<N1, E>>> {
    x
}
pub fn canonical_1554(
    x: PhantomData<Sum<SevenSevenSeven, SevenSevenSeven>>,
) -> PhantomData<T<N4, T<N5, T<N5, T<N1, E>>>>> {
    x
}
