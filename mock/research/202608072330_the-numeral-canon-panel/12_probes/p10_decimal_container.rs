//! p10. The decimal ladder's blocker: can it derive the CONTAINER?
//!
//! p09 gives base-ten addition, canonical, checked. That is half a ladder. The
//! binary ladder in `ladder.rs` also derives the container, by two operations
//! that are trivial in base two and not obviously available in base ten:
//!   the native rung, which is the digit count of W-1, that is floor(log2)+1
//!   the word count, which is ceil(W / 64)
//!
//! Both reduce to halving. Halving a little-endian decimal tower is structural:
//! recurse to the tail, halve from the top, and pass the odd bit down one place
//! as a plus-five. Twenty rows, ten digits by two incoming states, and no width.
//!
//! Then floor(log2) is the number of halvings before the tower is empty, and
//! ceil(W/64) is six halvings with the round-up folded in the way the binary
//! ladder folds it: ceil(W/64) == floor((W-1)/64) + 1.
//!
//! rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
//!       --emit=metadata -o out/p10.meta p10_decimal_container.rs
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("p09_core.rs");

// --- halving, little endian ---------------------------------------------------
// Recurse to the tail first, halve there, and take back whether the tail was
// odd. An odd tail contributes five to this digit before it is halved.
pub trait Halve {
    /// the halved tower
    type O;
    /// whether the ORIGINAL tower was odd
    type Odd;
}
impl Halve for E {
    type O = E;
    type Odd = C0;
}

// digit halving with an incoming five: twenty rows, the whole table.
pub trait HalfDigit<Carry> {
    type Q;
    type R;
}
macro_rules! hd {
    ($($d:ident => $q0:ident, $r0:ty, $q5:ident, $r5:ty);* $(;)?) => { $(
        impl HalfDigit<C0> for $d { type Q = $q0; type R = $r0; }
        impl HalfDigit<C1> for $d { type Q = $q5; type R = $r5; }
    )* };
}
// C1 means the digit ABOVE this one was odd, so ten is carried down into this
// place before it is halved: (d + 10c) / 2 and (d + 10c) % 2. Adding ten does
// not change the parity, so the remainder column is the same in both halves and
// only the quotient shifts by five.
hd! {
    N0 => N0, C0, N5, C0;
    N1 => N0, C1, N5, C1;
    N2 => N1, C0, N6, C0;
    N3 => N1, C1, N6, C1;
    N4 => N2, C0, N7, C0;
    N5 => N2, C1, N7, C1;
    N6 => N3, C0, N8, C0;
    N7 => N3, C1, N8, C1;
    N8 => N4, C0, N9, C0;
    N9 => N4, C1, N9, C1;
}

// dropping a leading zero keeps the representation canonical
pub trait Cons<R> {
    type O;
}
impl Cons<E> for N0 {
    type O = E;
}
macro_rules! cons_nonzero { ($($d:ident),*) => { $( impl Cons<E> for $d { type O = T<$d, E>; } )* } }
cons_nonzero!(N1, N2, N3, N4, N5, N6, N7, N8, N9);
impl<D, DR, RR> Cons<T<DR, RR>> for D {
    type O = T<D, T<DR, RR>>;
}

impl<D, R> Halve for T<D, R>
where
    R: Halve,
    D: HalfDigit<<R as Halve>::Odd>,
    <D as HalfDigit<<R as Halve>::Odd>>::Q: Cons<<R as Halve>::O>,
{
    type O = <<D as HalfDigit<<R as Halve>::Odd>>::Q as Cons<<R as Halve>::O>>::O;
    type Odd = <D as HalfDigit<<R as Halve>::Odd>>::R;
}

// --- floor(log2) + 1, as a unary tally of halvings ----------------------------
pub trait Bits {
    type N;
}
impl Bits for E {
    type N = Z;
}
impl<D, R> Bits for T<D, R>
where
    T<D, R>: Halve,
    <T<D, R> as Halve>::O: Bits,
{
    type N = S<<<T<D, R> as Halve>::O as Bits>::N>;
}

// --- decrement, so the rung is keyed on W - 1 as in the binary ladder ---------
pub trait Dec {
    type O;
}
macro_rules! dec_nonzero { ($($d:ident => $p:ident),* $(,)?) => { $(
    impl<R> Dec for T<$d, R> where $p: Cons<R> { type O = <$p as Cons<R>>::O; } )* } }
dec_nonzero! { N1 => N0, N2 => N1, N3 => N2, N4 => N3, N5 => N4, N6 => N5, N7 => N6, N8 => N7, N9 => N8 }
impl<R: Dec> Dec for T<N0, R>
where
    N9: Cons<<R as Dec>::O>,
{
    type O = <N9 as Cons<<R as Dec>::O>>::O;
}

// --- the native rung, by halving count of W-1 --------------------------------
pub trait Rung {
    type C;
}
impl Rung for Z {
    type C = u8;
}
impl Rung for S<Z> {
    type C = u8;
}
impl Rung for S<S<Z>> {
    type C = u8;
}
impl Rung for S<S<S<Z>>> {
    type C = u8;
}
impl Rung for S<S<S<S<Z>>>> {
    type C = u16;
}
impl Rung for S<S<S<S<S<Z>>>>> {
    type C = u32;
}
impl Rung for S<S<S<S<S<S<Z>>>>>> {
    type C = u64;
}
impl Rung for S<S<S<S<S<S<S<Z>>>>>>> {
    type C = u128;
}
impl<X> Rung for S<S<S<S<S<S<S<S<X>>>>>>>> {
    type C = Wide<X>;
}
pub struct Wide<X>(PhantomData<X>);

pub trait Container {
    type C;
}
impl<W> Container for W
where
    W: Dec,
    <W as Dec>::O: Bits,
    <<W as Dec>::O as Bits>::N: Rung,
{
    type C = <<<W as Dec>::O as Bits>::N as Rung>::C;
}

// --- checks, against arithmetic ----------------------------------------------
pub type W8 = T<N8, E>;
pub type W13 = T<N3, T<N1, E>>;
pub type W16 = T<N6, T<N1, E>>;
pub type W17 = T<N7, T<N1, E>>;
pub type W32 = T<N2, T<N3, E>>;
pub type W33 = T<N3, T<N3, E>>;
pub type W64 = T<N4, T<N6, E>>;
pub type W65 = T<N5, T<N6, E>>;
pub type W128 = T<N8, T<N2, T<N1, E>>>;
pub type W129 = T<N9, T<N2, T<N1, E>>>;

const _: () = {
    assert!(<W13 as Nat>::V == 13);
    assert!(<W129 as Nat>::V == 129);
    // halving is right
    assert!(<<W129 as Halve>::O as Nat>::V == 64);
    assert!(<<W65 as Halve>::O as Nat>::V == 32);
    assert!(<<W13 as Halve>::O as Nat>::V == 6);
    assert!(<<W8 as Halve>::O as Nat>::V == 4);
    // decrement is right, including across a borrow
    assert!(<<W16 as Dec>::O as Nat>::V == 15);
    assert!(<<T<N0, T<N1, E>> as Dec>::O as Nat>::V == 9); // 10 - 1, tower shortens
    assert!(<<T<N0, T<N0, T<N1, E>>> as Dec>::O as Nat>::V == 99); // 100 - 1
};

// the container, at every rung boundary and one past it
const _: () = {
    assert!(core::mem::size_of::<<W8 as Container>::C>() == 1);
    assert!(core::mem::size_of::<<W13 as Container>::C>() == 2);
    assert!(core::mem::size_of::<<W16 as Container>::C>() == 2);
    assert!(core::mem::size_of::<<W17 as Container>::C>() == 4);
    assert!(core::mem::size_of::<<W32 as Container>::C>() == 4);
    assert!(core::mem::size_of::<<W33 as Container>::C>() == 8);
    assert!(core::mem::size_of::<<W64 as Container>::C>() == 8);
    assert!(core::mem::size_of::<<W65 as Container>::C>() == 16);
    assert!(core::mem::size_of::<<W128 as Container>::C>() == 16);
};

// --- the wide arm: ceil(W / 64) words, above 128 bits ------------------------
// Same identity the binary ladder uses: ceil(W/64) == floor((W-1)/64) + 1, so
// the round-up needs no classifier. Six halvings is division by sixty-four.
pub type H1<X> = <X as Halve>::O;
pub trait Div64 {
    type O;
}
impl<W> Div64 for W
where
    W: Halve,
    H1<W>: Halve,
    H1<H1<W>>: Halve,
    H1<H1<H1<W>>>: Halve,
    H1<H1<H1<H1<W>>>>: Halve,
    H1<H1<H1<H1<H1<W>>>>>: Halve,
{
    type O = H1<H1<H1<H1<H1<H1<W>>>>>>;
}

// tower to unary tally, so the word payload can be built by structural cons.
pub trait ToTally {
    type T;
}
impl ToTally for E {
    type T = Z;
}
impl<D, R> ToTally for T<D, R>
where
    T<D, R>: Dec,
    <T<D, R> as Dec>::O: ToTally,
{
    type T = S<<<T<D, R> as Dec>::O as ToTally>::T>;
}

#[repr(C)]
pub struct WNil;
#[repr(C)]
pub struct WCons<X> {
    pub w: u64,
    pub rest: X,
}
pub trait Build {
    type P;
}
impl Build for Z {
    type P = WNil;
}
impl<X: Build> Build for S<X> {
    type P = WCons<<X as Build>::P>;
}

pub trait WordCount {
    type W;
}
impl<W> WordCount for W
where
    W: Dec,
    <W as Dec>::O: Div64,
    <<W as Dec>::O as Div64>::O: ToTally,
{
    type W = S<<<<W as Dec>::O as Div64>::O as ToTally>::T>;
}

pub type W129b = T<N9, T<N2, T<N1, E>>>;
pub type W200 = T<N0, T<N0, T<N2, E>>>;
pub type W256 = T<N6, T<N5, T<N2, E>>>;
pub type W1636 = T<N6, T<N3, T<N6, T<N1, E>>>>;

const _: () = {
    // division by sixty-four, checked against arithmetic
    assert!(<<<W129b as Dec>::O as Div64>::O as Nat>::V == 2); // (129-1)/64
    assert!(<<<W256 as Dec>::O as Div64>::O as Nat>::V == 3); // (256-1)/64
    assert!(<<<W1636 as Dec>::O as Div64>::O as Nat>::V == 25); // (1636-1)/64
                                                                // and the payload the word count builds
    assert!(core::mem::size_of::<<<W129b as WordCount>::W as Build>::P>() == 24);
    assert!(core::mem::size_of::<<<W200 as WordCount>::W as Build>::P>() == 32);
    assert!(core::mem::size_of::<<<W256 as WordCount>::W as Build>::P>() == 32);
    assert!(core::mem::size_of::<<<W1636 as WordCount>::W as Build>::P>() == 208);
};
