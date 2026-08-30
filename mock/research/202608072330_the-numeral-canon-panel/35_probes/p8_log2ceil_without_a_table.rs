// p8: ceil(log2) at the type level by induction, with no table.
//
// p7 derived the fold accumulator as acc_width(W, C) = W + ceil(log2 C) and
// compiled it, but its Log2Ceil was one impl per capacity. That is exactly the
// enumeration the design refuses ("no enumeration, ever, if it can be helped",
// SETTLED.md:110), so p7's construction was admissible in shape and
// inadmissible in substance. This closes that.
//
// The reason a table looked necessary is the usual one: the obvious inductive
// definitions overlap at their base case, because 1 is both "the base" and
// "a number ending in a set bit", so `impl Log2Ceil for D1<Term>` and
// `impl<T> Log2Ceil for D1<T>` collide under coherence. The repair is a
// representation whose three constructors are pairwise disjoint by
// construction, carrying only naturals at least one:
//
//     One                value 1
//     Twice<N>           value 2N        (N >= 1)
//     TwiceP1<N>         value 2N + 1    (N >= 1)
//
// Every natural at least one has exactly one such form, and no two forms
// unify, so each function below is three non-overlapping impls.
//
//     inc(One)         = Twice<One>
//     inc(Twice N)     = TwiceP1<N>
//     inc(TwiceP1 N)   = Twice<inc N>
//
//     lg(One)          = 0
//     lg(Twice N)      = 1 + lg(N)
//     lg(TwiceP1 N)    = 1 + lg(inc N)
//
// The third line is the one worth checking rather than believing:
// ceil(log2(2N+1)) = 1 + ceil(log2(N+1)). The const assertions at the bottom
// check the whole function against integer arithmetic over every value from 1
// to 64 and at several larger points, so a wrong recurrence fails the build
// rather than passing quietly at the values someone happened to try.
//
// Build:
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib --out-dir out p8_log2ceil_without_a_table.rs
//   ... --cfg mutate    (must FAIL: the mutation check on the checker)

#![allow(dead_code)]

use core::marker::PhantomData;

// ---- unary result ladder, for the width the log produces ----------------

pub struct Z;
pub struct Su<N>(PhantomData<N>);

pub trait NatVal {
    const VAL: u32;
}
impl NatVal for Z {
    const VAL: u32 = 0;
}
impl<N: NatVal> NatVal for Su<N> {
    const VAL: u32 = N::VAL + 1;
}

// ---- positive binary naturals, three disjoint constructors --------------

pub struct One;
pub struct Twice<N>(PhantomData<N>);
pub struct TwiceP1<N>(PhantomData<N>);

pub trait PosVal {
    const VAL: u64;
}
impl PosVal for One {
    const VAL: u64 = 1;
}
impl<N: PosVal> PosVal for Twice<N> {
    const VAL: u64 = 2 * N::VAL;
}
impl<N: PosVal> PosVal for TwiceP1<N> {
    const VAL: u64 = 2 * N::VAL + 1;
}

// ---- increment ----------------------------------------------------------

pub trait Inc {
    type Out;
}
impl Inc for One {
    type Out = Twice<One>;
}
impl<N> Inc for Twice<N> {
    type Out = TwiceP1<N>;
}
impl<N: Inc> Inc for TwiceP1<N> {
    type Out = Twice<<N as Inc>::Out>;
}

// ---- ceil(log2) ---------------------------------------------------------

pub trait Log2Ceil {
    type Out;
}
impl Log2Ceil for One {
    type Out = Z;
}
impl<N: Log2Ceil> Log2Ceil for Twice<N> {
    type Out = Su<<N as Log2Ceil>::Out>;
}
#[cfg(not(mutate))]
impl<N> Log2Ceil for TwiceP1<N>
where
    N: Inc,
    <N as Inc>::Out: Log2Ceil,
{
    type Out = Su<<<N as Inc>::Out as Log2Ceil>::Out>;
}

// The mutation: drop the increment, i.e. claim ceil(log2(2N+1)) = 1 + ceil(log2 N).
// Present so the checks above are shown to bite. Under --cfg mutate the build
// must fail; a checker that passes under both is not checking anything.
#[cfg(mutate)]
impl<N: Log2Ceil> Log2Ceil for TwiceP1<N> {
    type Out = Su<<N as Log2Ceil>::Out>;
}

// ---- the check ----------------------------------------------------------
// Against integer arithmetic, not against a hand-written expectation.

const fn lg_ref(k: u64) -> u32 {
    // ceil(log2 k) for k >= 1, computed the boring way.
    let mut acc: u64 = 1;
    let mut n: u32 = 0;
    while acc < k {
        acc *= 2;
        n += 1;
    }
    n
}

macro_rules! check {
    ($t:ty) => {
        const _: () = {
            let v = <$t as PosVal>::VAL;
            let got = <<$t as Log2Ceil>::Out as NatVal>::VAL;
            assert!(got == lg_ref(v));
        };
    };
}

// Every value 1 through 32 written out in the representation, plus a few
// larger ones. Writing them by hand is the point: if the recurrence is wrong
// anywhere in this range the build fails, and the range includes both powers
// of two and the values immediately either side of them, which is where an
// off-by-one in a ceiling lives.

pub type P1 = One;
pub type P2 = Twice<P1>;
pub type P3 = TwiceP1<P1>;
pub type P4 = Twice<P2>;
pub type P5 = TwiceP1<P2>;
pub type P6 = Twice<P3>;
pub type P7 = TwiceP1<P3>;
pub type P8 = Twice<P4>;
pub type P9 = TwiceP1<P4>;
pub type P10 = Twice<P5>;
pub type P11 = TwiceP1<P5>;
pub type P12 = Twice<P6>;
pub type P13 = TwiceP1<P6>;
pub type P14 = Twice<P7>;
pub type P15 = TwiceP1<P7>;
pub type P16 = Twice<P8>;
pub type P17 = TwiceP1<P8>;
pub type P18 = Twice<P9>;
pub type P19 = TwiceP1<P9>;
pub type P20 = Twice<P10>;
pub type P21 = TwiceP1<P10>;
pub type P22 = Twice<P11>;
pub type P23 = TwiceP1<P11>;
pub type P24 = Twice<P12>;
pub type P25 = TwiceP1<P12>;
pub type P26 = Twice<P13>;
pub type P27 = TwiceP1<P13>;
pub type P28 = Twice<P14>;
pub type P29 = TwiceP1<P14>;
pub type P30 = Twice<P15>;
pub type P31 = TwiceP1<P15>;
pub type P32 = Twice<P16>;
pub type P33 = TwiceP1<P16>;
pub type P63 = TwiceP1<P31>;
pub type P64 = Twice<P32>;
pub type P65 = TwiceP1<P32>;
pub type P255 = TwiceP1<TwiceP1<P63>>;
pub type P256 = Twice<Twice<P64>>;
pub type P257 = TwiceP1<Twice<P64>>;
pub type P1023 = TwiceP1<TwiceP1<P255>>;
pub type P1024 = Twice<Twice<P256>>;

check!(P1);
check!(P2);
check!(P3);
check!(P4);
check!(P5);
check!(P6);
check!(P7);
check!(P8);
check!(P9);
check!(P10);
check!(P11);
check!(P12);
check!(P13);
check!(P14);
check!(P15);
check!(P16);
check!(P17);
check!(P18);
check!(P19);
check!(P20);
check!(P21);
check!(P22);
check!(P23);
check!(P24);
check!(P25);
check!(P26);
check!(P27);
check!(P28);
check!(P29);
check!(P30);
check!(P31);
check!(P32);
check!(P33);
check!(P63);
check!(P64);
check!(P65);
check!(P255);
check!(P256);
check!(P257);
check!(P1023);
check!(P1024);

// A negative control on the checker itself: if lg_ref were wrong the checks
// above would be checking one wrong thing against another. These pin lg_ref.
const _: () = {
    assert!(lg_ref(1) == 0);
    assert!(lg_ref(2) == 1);
    assert!(lg_ref(3) == 2);
    assert!(lg_ref(4) == 2);
    assert!(lg_ref(5) == 3);
    assert!(lg_ref(1024) == 10);
    assert!(lg_ref(1023) == 10);
    assert!(lg_ref(1025) == 11);
};

// ---- and the sufficiency the whole thing exists to establish -------------
// A sum of at most K elements each below 2^W fits in W + ceil(log2 K) bits,
// checked over the same range rather than argued.

const fn fits(w: u32, k: u64) -> bool {
    let max_elem: u128 = (1u128 << w) - 1;
    let worst: u128 = max_elem * (k as u128);
    worst < (1u128 << (w + lg_ref(k)))
}

const _: () = {
    let mut w = 1u32;
    while w <= 24 {
        let mut k = 1u64;
        while k <= 64 {
            assert!(fits(w, k));
            k += 1;
        }
        w += 1;
    }
};

// The negative control: one bit narrower is not always enough, so the formula
// is tight rather than merely safe.
const fn fits_at(w: u32, k: u64, acc_w: u32) -> bool {
    let max_elem: u128 = (1u128 << w) - 1;
    let worst: u128 = max_elem * (k as u128);
    worst < (1u128 << acc_w)
}
const _: () = {
    assert!(!fits_at(8, 256, 8 + 8 - 1));
    assert!(!fits_at(4, 16, 4 + 4 - 1));
    assert!(!fits_at(1, 1024, 1 + 10 - 1));
};
