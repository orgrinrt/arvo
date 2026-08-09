//! Probe 6: the adjustment half of the biased-product closure formula
//! (`31:397-400`, `adjustment = gcd(A1*A2, A1*B2, A2*B1)`), generalised
//! over rational A1, A2 (Adjustment, already rational before this
//! dispatch) and B1, B2 (Bias, rational as of file 41), which file 41
//! named as untouched and this dispatch was sent to close.
//!
//! "gcd of three rationals" is read per file 41's own section 6 framing:
//! a generator of the additive subgroup the three terms jointly
//! generate. Concretely: reduce each of the three products to its own
//! lowest terms (`PMul` + the reduction chain, reused unchanged from
//! probe 5), place all three over their common denominator (the lcm of
//! the three individual reduced denominators), take the ordinary
//! integer gcd of the resulting three numerators, and reduce the
//! (gcd, common-denominator) pair once more, because that pair is not
//! guaranteed already lowest terms. No new arithmetic primitive:
//! `Lcm<A, B> = A * (B / gcd(A, B))`, and `B / gcd(A, B)` is exactly
//! `GenReduce<A, B>::D` (probe 5's own reduction chain, applied to the
//! pair rather than to a single Bias magnitude).
//!
//! One clean, small finding beyond the mechanism itself: `Adjustment`'s
//! own type is unsigned (`const NUM: u64`, no sign field at all), so the
//! closure formula's OUTPUT never needs a sign combination the way
//! `Bias`'s own multiplication does (file 41's four-alias
//! `BiasMulPP`/`PN`/`NN` dispatch). Only the MAGNITUDES of B1, B2 ever
//! enter this half's computation; sign is absent from the type, not
//! merely unused by convention, matching ordinary gcd's own
//! sign-indifference.
//!
//! Checked against two independent witnesses, cross-validated against
//! Python's `fractions.Fraction` before being spelled at the type level
//! (both in OUTCOMES.md): A1=3/4, A2=1/2, B1=1/2, B2=1/3 -> 1/8; and the
//! harder cross-denominator case A1=2/3, A2=3/5, B1=1/4, B2=5/6 -> 1/180
//! (lcm(5,9,20)=180). Every intermediate value is asserted, not only the
//! final answer, so a wrong step fails to compile at that step, not
//! silently downstream.
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_6_adjustment_half.rs --out-dir <dir>
//! Outcome: WORKS, every intermediate and final assertion for both
//! witnesses.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

#[path = "vu_bias.rs"]
pub mod bias;

use bias::nat::{AsPos, ExactDivOdd, Gcd, Pos, Pz, Ratio, Strip2};
use bias::nat::{H, I, O};
use bias::PMul;

// --- the generic reducer, unbundled (probe 4's finding: this composes
// generically; naming Reduce as a bound does not), restated here for
// this file's self-containment and applied to arbitrary Pos pairs, not
// only Bias magnitudes. ---
pub trait GenReduce<N, D> {
    type N: Pos;
    type D: Pos;
}
pub struct Reducer;
impl<N, D, StripN, StripD, Divisor, QuoN, QuoD, FinalN, FinalD> GenReduce<N, D> for Reducer
where
    N: Pos,
    D: Pos,
    Ratio<N, D>: Strip2<N = StripN, D = StripD>,
    StripN: Pos + Gcd<StripD, Out = Divisor>,
    StripD: Pos,
    Divisor: Pos,
    Pz<StripN>: ExactDivOdd<Divisor, Out = QuoN>,
    Pz<StripD>: ExactDivOdd<Divisor, Out = QuoD>,
    QuoN: AsPos<Out = FinalN>,
    QuoD: AsPos<Out = FinalD>,
    FinalN: Pos,
    FinalD: Pos,
{
    type N = FinalN;
    type D = FinalD;
}
pub type GRN<N, D> = <Reducer as GenReduce<N, D>>::N;
pub type GRD<N, D> = <Reducer as GenReduce<N, D>>::D;

/// Lcm(A, B) = A * (B / gcd(A, B)) = A * GRD<A, B>.
pub type Lcm<A, B> = <A as PMul<GRD<A, B>>>::Out;
/// Lcm of three, associative.
pub type Lcm3<A, B, C> = Lcm<Lcm<A, B>, C>;
/// Gcd of three, associative, no new mechanism (Gcd already composes).
pub type Gcd3<A, B, C> = <<A as Gcd<B>>::Out as Gcd<C>>::Out;
/// D / Di, where Di divides D exactly: read off GenReduce<D, Di>'s
/// numerator side (gcd(D, Di) = Di when Di | D, so this is an exact
/// quotient, not merely a reduction).
pub type PlaceOver<D, Di> = GRN<D, Di>;

/// The magnitude of A1 * A2, reduced: reuses PMul + the reducer, no new
/// arithmetic.
pub type RatProdN<N1, D1, N2, D2> = GRN<<N1 as PMul<N2>>::Out, <D1 as PMul<D2>>::Out>;
pub type RatProdD<N1, D1, N2, D2> = GRD<<N1 as PMul<N2>>::Out, <D1 as PMul<D2>>::Out>;

// --- the staged pipeline: reduce each term, place all three over their
// common denominator, gcd the placed numerators, reduce once more.
// Wrapped in a macro so both witnesses reuse the identical pipeline
// definition rather than two hand-copied variants that could decorrelate. ---

macro_rules! adjustment_half {
    ($name:ident, $a1n:ty, $a1d:ty, $a2n:ty, $a2d:ty, $b1n:ty, $b1d:ty, $b2n:ty, $b2d:ty,
     $t1n:literal, $t1d:literal, $t2n:literal, $t2d:literal, $t3n:literal, $t3d:literal,
     $common_d:literal, $expect_n:literal, $expect_d:literal) => {
        mod $name {
            use super::*;

            type T1N = RatProdN<$a1n, $a1d, $a2n, $a2d>;
            type T1D = RatProdD<$a1n, $a1d, $a2n, $a2d>;
            type T2N = RatProdN<$a1n, $a1d, $b2n, $b2d>;
            type T2D = RatProdD<$a1n, $a1d, $b2n, $b2d>;
            type T3N = RatProdN<$a2n, $a2d, $b1n, $b1d>;
            type T3D = RatProdD<$a2n, $a2d, $b1n, $b1d>;

            const _: () = {
                assert!(<T1N as Pos>::VAL == $t1n && <T1D as Pos>::VAL == $t1d);
                assert!(<T2N as Pos>::VAL == $t2n && <T2D as Pos>::VAL == $t2d);
                assert!(<T3N as Pos>::VAL == $t3n && <T3D as Pos>::VAL == $t3d);
            };

            type CommonD = Lcm3<T1D, T2D, T3D>;
            const _: () = assert!(<CommonD as Pos>::VAL == $common_d);

            type M1 = <T1N as PMul<PlaceOver<CommonD, T1D>>>::Out;
            type M2 = <T2N as PMul<PlaceOver<CommonD, T2D>>>::Out;
            type M3 = <T3N as PMul<PlaceOver<CommonD, T3D>>>::Out;

            type G = Gcd3<M1, M2, M3>;

            pub type ResultN = GRN<G, CommonD>;
            pub type ResultD = GRD<G, CommonD>;

            const _: () =
                assert!(<ResultN as Pos>::VAL == $expect_n && <ResultD as Pos>::VAL == $expect_d);
        }
    };
}

// H=1, O<H>=2, I<H>=3, O<O<H>>=4, I<O<H>>=5, O<I<H>>=6, I<I<H>>=7,
// O<O<O<H>>>=8, I<O<O<H>>>=9, O<I<O<H>>>=10, O<O<I<O<H>>>>=20.

// witness 1: A1=3/4, A2=1/2, B1=1/2, B2=1/3.
//   term1 = A1*A2 = 3/8, term2 = A1*B2 = 1/4, term3 = A2*B1 = 1/4.
//   lcm(8,4,4) = 8. gcd(3,2,2) = 1. result = 1/8.
adjustment_half!(
    witness1,
    I<H>,
    O<O<H>>,
    H,
    O<H>,
    H,
    O<H>,
    H,
    I<H>,
    3,
    8,
    1,
    4,
    1,
    4,
    8,
    1,
    8
);

// witness 2: A1=2/3, A2=3/5, B1=1/4, B2=5/6.
//   term1 = A1*A2 = 2/5, term2 = A1*B2 = 5/9, term3 = A2*B1 = 3/20.
//   lcm(5,9,20) = 180. gcd(72,100,27) = 1. result = 1/180.
//   Cross-checked independently against Python's Fraction before being
//   spelled at the type level (OUTCOMES.md).
adjustment_half!(
    witness2,
    O<H>,
    I<H>,
    I<H>,
    I<O<H>>,
    H,
    O<O<H>>,
    I<O<H>>,
    O<I<H>>,
    2,
    5,
    5,
    9,
    3,
    20,
    180,
    1,
    180
);
