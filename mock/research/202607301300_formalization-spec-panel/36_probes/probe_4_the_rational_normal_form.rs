//! Probe 4: the rational adjustment's normal form, and file 34's probe 5b
//! turned from a refusal into a compiled positive.
//!
//! The normal form: an adjustment is `Ratio<N, D>` with `N`, `D` positive in
//! the value-unique encoding and `gcd(N, D) = 1`. Two things make it a
//! guarantee rather than a convention.
//!
//! First, `N` and `D` are `Pos`, so each has exactly one spelling per value
//! (probe 2). Coprimality alone would not be enough on the old encoding:
//! `UInt<UTerm, B0, ...>`-style padding would still give one rational many
//! types, so the rational normal form rests on the natural one and cannot be
//! stated without it. That dependency is the reason probe 1 comes first.
//!
//! Second, the coprimality side is enforced where it is observed rather than
//! maintained by whoever writes the type: `impl Adjustment for Ratio<N, D>`
//! carries `N: Gcd<D, Out = H>`, so `Ratio<Six, Twelve>` is a well-formed
//! type that is not an `Adjustment` and cannot reach any position bounded by
//! one. Probe 4b is that refusal, committed refusing.
//!
//! Reduction itself needs a division, which the gcd does not supply. It is
//! the only genuinely new operation this obligation costs, and it is exact
//! division by an odd divisor, done LSB-first (`vu_nat.rs`), after the common
//! power of two has been stripped structurally.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_4_the_rational_normal_form.rs
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]

use core::marker::PhantomData;

#[path = "vu_nat.rs"]
mod nat;

use nat::{Adjustment, AsPos, ExactDivOdd, Gcd, Nat, Pos, Pz, Ratio, Reduce, Reduced};
use nat::{H, I, O, Z};

// --- values ---

pub type P1 = H;
pub type P2 = O<H>;
pub type P3 = I<H>;
pub type P4 = O<O<H>>;
pub type P5 = I<O<H>>;
pub type P6 = O<I<H>>;
pub type P7 = I<I<H>>;
pub type P8 = O<O<O<H>>>;
pub type P12 = O<O<I<H>>>;
pub type P15 = I<I<I<H>>>;
pub type P24 = O<O<O<I<H>>>>;
pub type P255 = I<I<I<I<I<I<I<H>>>>>>>;

// --- CLAIM A: exact division by an odd divisor is correct. ---

pub type Q<N, D> = <<Pz<N> as ExactDivOdd<D>>::Out as AsPos>::Out;

const _: () = assert!(<Q<P15, P5> as Pos>::VAL == 3);
const _: () = assert!(<Q<P15, P3> as Pos>::VAL == 5);
const _: () = assert!(<Q<P24, P3> as Pos>::VAL == 8);
const _: () = assert!(<Q<P12, P3> as Pos>::VAL == 4);
const _: () = assert!(<Q<P255, P15> as Pos>::VAL == 17);
const _: () = assert!(<Q<P255, P5> as Pos>::VAL == 51);
const _: () = assert!(<Q<P7, P7> as Pos>::VAL == 1);
const _: () = assert!(<Q<P8, P1> as Pos>::VAL == 8);

// --- CLAIM B: reduction is correct, including the case file 34 refused. ---

pub type RN<N, D> = <Ratio<N, D> as Reduce>::N;
pub type RD<N, D> = <Ratio<N, D> as Reduce>::D;

// 6/12 reduces to 1/2. This is probe 5b's exact witness (`34:320`).
const _: () = assert!(<RN<P6, P12> as Pos>::VAL == 1);
const _: () = assert!(<RD<P6, P12> as Pos>::VAL == 2);
// 12/8 -> 3/2, an even/even pair whose common factor is not the whole gcd.
const _: () = assert!(<RN<P12, P8> as Pos>::VAL == 3);
const _: () = assert!(<RD<P12, P8> as Pos>::VAL == 2);
// 15/255 -> 1/17, the UNORM-shaped case (a `FullRange<F>` denominator).
const _: () = assert!(<RN<P15, P255> as Pos>::VAL == 1);
const _: () = assert!(<RD<P15, P255> as Pos>::VAL == 17);
// already reduced, and the identity 1/1.
const _: () = assert!(<RN<P3, P4> as Pos>::VAL == 3);
const _: () = assert!(<RD<P3, P4> as Pos>::VAL == 4);
const _: () = assert!(<RN<P1, P1> as Pos>::VAL == 1);
const _: () = assert!(<RD<P1, P1> as Pos>::VAL == 1);
// an integer-valued ratio.
const _: () = assert!(<RN<P24, P8> as Pos>::VAL == 3);
const _: () = assert!(<RD<P24, P8> as Pos>::VAL == 1);

// --- CLAIM C: reduction is idempotent, as a type identity. This is the ---
// --- normal-form property itself: reducing a reduced ratio is the same ---
// --- type, not merely the same value. ---

pub fn same_type<T>(_: PhantomData<T>, _: PhantomData<T>) {}

pub fn reduce_is_idempotent_as_types() {
    same_type(
        PhantomData::<Reduced<P6, P12>>,
        PhantomData::<Reduced<RN<P6, P12>, RD<P6, P12>>>,
    );
    same_type(
        PhantomData::<Reduced<P12, P8>>,
        PhantomData::<Reduced<RN<P12, P8>, RD<P12, P8>>>,
    );
    same_type(
        PhantomData::<Reduced<P15, P255>>,
        PhantomData::<Reduced<RN<P15, P255>, RD<P15, P255>>>,
    );
}

// --- CLAIM D: file 34's probe 5b, now compiling. The product of the ---
// --- reduced numerals 2/3 and 3/4 inhabits the type of the numeral 1/2 a ---
// --- consumer writes directly. ---
//
// The product formula is componentwise on the pair, then reduced. The
// componentwise part needs multiplication of positives, which this probe
// avoids needing to build: 2*3 = 6 and 3*4 = 12 are written directly, since
// what is under test is the normal form, not the multiplier. (A type-level
// multiply is the `AddWidth` machinery's sibling and is orthogonal to the
// obligation; it is named in the file as the one piece this probe assumes
// rather than builds.)

/// The unreduced product spelling: 6/12, exactly file 34's `Adj { 6, 12 }`.
pub type ProductSpelling = Ratio<P6, P12>;
/// The numeral a consumer writes for the same quantum: 1/2.
pub type WrittenHalf = Ratio<P1, P2>;

pub fn the_product_inhabits_its_own_numeral() {
    same_type(PhantomData::<Reduced<P6, P12>>, PhantomData::<WrittenHalf>);
}

/// And through the alias a consumer would actually write, which normalises
/// at every naming site, so the two spellings are one type before anything
/// asks whether they unify.
pub fn both_spellings_name_one_type() {
    same_type(
        PhantomData::<Reduced<P6, P12>>,
        PhantomData::<Reduced<P1, P2>>,
    );
}

// --- CLAIM E: the perimeter. A reduced ratio is an `Adjustment`; the ---
// --- unreduced spelling is not, and probe 4b is that refusal. ---

pub fn takes_an_adjustment<A: Adjustment>() -> (u64, u64) {
    (A::NUM, A::DEN)
}

pub fn reduced_ratios_are_adjustments() {
    let _ = takes_an_adjustment::<Reduced<P6, P12>>();
    let _ = takes_an_adjustment::<Reduced<P12, P8>>();
    let _ = takes_an_adjustment::<Reduced<P15, P255>>();
    let _ = takes_an_adjustment::<WrittenHalf>();
    let _ = takes_an_adjustment::<Ratio<P3, P4>>();
}

const _: () = assert!(<Reduced<P6, P12> as Adjustment>::NUM == 1);
const _: () = assert!(<Reduced<P6, P12> as Adjustment>::DEN == 2);
const _: () = assert!(<Reduced<P15, P255> as Adjustment>::NUM == 1);
const _: () = assert!(<Reduced<P15, P255> as Adjustment>::DEN == 17);

// --- CLAIM F: the dyadic case, which is what every fixed-point and IEEE ---
// --- composition actually uses, never reaches the divider at all. A ---
// --- power-of-two denominator with numerator one strips to 1/1 and the ---
// --- gcd is `H` on the first impl. The reduction machinery is present ---
// --- for MATLAB's slope-and-bias and for UNORM, and is inert for the ---
// --- compositions arvo ships today. ---

pub type P16 = O<O<O<O<H>>>>;
pub type P32 = O<O<O<O<O<H>>>>>;
pub type P256 = O<O<O<O<O<O<O<O<H>>>>>>>>;

const _: () = assert!(<RN<P1, P256> as Pos>::VAL == 1);
const _: () = assert!(<RD<P1, P256> as Pos>::VAL == 256);
const _: () = assert!(<RN<P16, P32> as Pos>::VAL == 1);
const _: () = assert!(<RD<P16, P32> as Pos>::VAL == 2);

pub fn dyadic_adjustments_are_already_normal() {
    same_type(
        PhantomData::<Reduced<P1, P256>>,
        PhantomData::<Ratio<P1, P256>>,
    );
    same_type(
        PhantomData::<Reduced<P1, P16>>,
        PhantomData::<Ratio<P1, P16>>,
    );
}

// Keep the unused imports honest.
const _ADJ_GCD_IS_REACHABLE: fn() = || {
    fn f<N: Pos + Gcd<D>, D: Pos>() {}
    f::<P6, P12>();
};
const _NAT_IS_REACHABLE: u64 = <Z as Nat>::VAL;
