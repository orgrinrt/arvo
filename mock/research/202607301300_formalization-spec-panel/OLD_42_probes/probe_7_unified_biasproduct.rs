//! Probe 7: closes file 41's own section 6 open item ("whether the
//! sign-and-magnitude split this file adopts ... is the shape the
//! eventual shipped crate should carry") on the evidence probe 5 and the
//! price sweep establish: it is not. `BiasProduct` (file 41's own
//! zero-absorbing dispatch trait, three impls: `BZero * anything`,
//! `BPos * BZero`, `BNeg * BZero`) extends cleanly to all nine sign
//! combinations, using probe 5's chain as the magnitude computation for
//! the four non-zero combinations. One trait, `b1.product(b2)`-shaped
//! (via `<B1 as BiasProduct<B2>>::Out`), no bare aliases exposed to the
//! consumer, no asymmetry between the zero and non-zero cases.
//!
//! One further, previously unrecorded finding, discovered while
//! building this file: the magnitude computation cannot be factored
//! into a SHARED helper trait either. A first attempt wrapped it in a
//! `Magnitude<T>` trait, one blanket impl over a tuple `(N1, D1, N2,
//! D2)`, and named it as a bound with its associated type additionally
//! projected and re-bounded (`MagN<..>: Pos`), the shape needed to name
//! the computed output type at all. That diverges, `Ratio<O<_>, O<_>>:
//! Strip2` overflow, isolated precisely in probe 7b: the bare, unused
//! `Mag: Magnitude<(N1, D1, N2, D2)>` bound alone, with nothing
//! projected from it, does NOT diverge, unlike probe 4b's bare `Ratio<N,
//! D>: Reduce`. The trigger for a freshly declared trait is narrower
//! than for `Reduce` itself; both converge on the same wall the moment
//! the associated type is actually used, which is what any real caller
//! needs. The chain has to be inlined directly into each of the four
//! sign-combination impls below, exactly as probe 5 inlines it into
//! `BiasMulGeneric`'s single impl; a bare TYPE ALIAS (never a trait) is
//! the only shape that stays shareable, matching probe 6's own
//! `RatProdN`/`RatProdD` pattern.
//!
//! The four sign-combination impls are pairwise disjoint by construction
//! (each names a distinct concrete `(Self, Rhs)` pair among `{BPos,
//! BNeg} x {BPos, BNeg}`; Rust's coherence checker confirms this
//! compiles with no overlap error against file 41's own three
//! zero-handling impls).
//!
//! Correctness reuses probe 5's own two witnesses, now reached through
//! the unified trait, plus one sign-mixing witness (`BPos * BNeg =
//! BNeg`) neither file 41 nor probe 5 individually needed to check
//! because file 41's own aliases kept sign and magnitude apart, and one
//! zero-absorption witness confirming the old and new impls coexist
//! without a coherence conflict.
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_7_unified_biasproduct.rs --out-dir <dir>
//! Outcome: WORKS, no overlap error, all four correctness witnesses.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

#[path = "vu_bias.rs"]
pub mod bias;

use bias::nat::{AsPos, ExactDivOdd, Gcd, Pos, Pz, Ratio, Strip2, H, I, O};
use bias::{BNeg, BPos, BZero, Bias, PMul};

/// file 41's own dispatch trait, unchanged in name and in its three
/// zero-handling impls (BiasProduct already existed for these; nothing
/// here duplicates or replaces them).
pub trait BiasProduct<Rhs> {
    type Out: Bias;
}
impl<R: Bias> BiasProduct<R> for BZero {
    type Out = BZero;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> BiasProduct<BZero> for BPos<N, D> {
    type Out = BZero;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> BiasProduct<BZero> for BNeg<N, D> {
    type Out = BZero;
}

/// positive times positive: positive. The chain is inlined directly,
/// per the header note: a shared helper TRAIT hits the same wall probe
/// 4 already named, one level up.
impl<N1, D1, N2, D2, RawN, RawD, StripN, StripD, Divisor, QuoN, QuoD, FinalN, FinalD>
    BiasProduct<BPos<N2, D2>> for BPos<N1, D1>
where
    N1: Pos + Gcd<D1, Out = H> + PMul<N2, Out = RawN>,
    D1: Pos + PMul<D2, Out = RawD>,
    N2: Pos + Gcd<D2, Out = H>,
    D2: Pos,
    RawN: Pos,
    RawD: Pos,
    Ratio<RawN, RawD>: Strip2<N = StripN, D = StripD>,
    StripN: Pos + Gcd<StripD, Out = Divisor>,
    StripD: Pos,
    Divisor: Pos,
    Pz<StripN>: ExactDivOdd<Divisor, Out = QuoN>,
    Pz<StripD>: ExactDivOdd<Divisor, Out = QuoD>,
    QuoN: AsPos<Out = FinalN>,
    QuoD: AsPos<Out = FinalD>,
    FinalN: Pos + Gcd<FinalD, Out = H>,
    FinalD: Pos,
{
    type Out = BPos<FinalN, FinalD>;
}

/// positive times negative: negative. Identical magnitude chain, sign
/// flipped in the output constructor only.
impl<N1, D1, N2, D2, RawN, RawD, StripN, StripD, Divisor, QuoN, QuoD, FinalN, FinalD>
    BiasProduct<BNeg<N2, D2>> for BPos<N1, D1>
where
    N1: Pos + Gcd<D1, Out = H> + PMul<N2, Out = RawN>,
    D1: Pos + PMul<D2, Out = RawD>,
    N2: Pos + Gcd<D2, Out = H>,
    D2: Pos,
    RawN: Pos,
    RawD: Pos,
    Ratio<RawN, RawD>: Strip2<N = StripN, D = StripD>,
    StripN: Pos + Gcd<StripD, Out = Divisor>,
    StripD: Pos,
    Divisor: Pos,
    Pz<StripN>: ExactDivOdd<Divisor, Out = QuoN>,
    Pz<StripD>: ExactDivOdd<Divisor, Out = QuoD>,
    QuoN: AsPos<Out = FinalN>,
    QuoD: AsPos<Out = FinalD>,
    FinalN: Pos + Gcd<FinalD, Out = H>,
    FinalD: Pos,
{
    type Out = BNeg<FinalN, FinalD>;
}

/// negative times positive: negative.
impl<N1, D1, N2, D2, RawN, RawD, StripN, StripD, Divisor, QuoN, QuoD, FinalN, FinalD>
    BiasProduct<BPos<N2, D2>> for BNeg<N1, D1>
where
    N1: Pos + Gcd<D1, Out = H> + PMul<N2, Out = RawN>,
    D1: Pos + PMul<D2, Out = RawD>,
    N2: Pos + Gcd<D2, Out = H>,
    D2: Pos,
    RawN: Pos,
    RawD: Pos,
    Ratio<RawN, RawD>: Strip2<N = StripN, D = StripD>,
    StripN: Pos + Gcd<StripD, Out = Divisor>,
    StripD: Pos,
    Divisor: Pos,
    Pz<StripN>: ExactDivOdd<Divisor, Out = QuoN>,
    Pz<StripD>: ExactDivOdd<Divisor, Out = QuoD>,
    QuoN: AsPos<Out = FinalN>,
    QuoD: AsPos<Out = FinalD>,
    FinalN: Pos + Gcd<FinalD, Out = H>,
    FinalD: Pos,
{
    type Out = BNeg<FinalN, FinalD>;
}

/// negative times negative: positive.
impl<N1, D1, N2, D2, RawN, RawD, StripN, StripD, Divisor, QuoN, QuoD, FinalN, FinalD>
    BiasProduct<BNeg<N2, D2>> for BNeg<N1, D1>
where
    N1: Pos + Gcd<D1, Out = H> + PMul<N2, Out = RawN>,
    D1: Pos + PMul<D2, Out = RawD>,
    N2: Pos + Gcd<D2, Out = H>,
    D2: Pos,
    RawN: Pos,
    RawD: Pos,
    Ratio<RawN, RawD>: Strip2<N = StripN, D = StripD>,
    StripN: Pos + Gcd<StripD, Out = Divisor>,
    StripD: Pos,
    Divisor: Pos,
    Pz<StripN>: ExactDivOdd<Divisor, Out = QuoN>,
    Pz<StripD>: ExactDivOdd<Divisor, Out = QuoD>,
    QuoN: AsPos<Out = FinalN>,
    QuoD: AsPos<Out = FinalD>,
    FinalN: Pos + Gcd<FinalD, Out = H>,
    FinalD: Pos,
{
    type Out = BPos<FinalN, FinalD>;
}

type Half = BPos<H, O<H>>;
type FiveHalves = BPos<I<O<H>>, O<H>>;
type NegFiveHalves = BNeg<I<O<H>>, O<H>>;

/// 1/2 * 5/2 = 5/4, file 41's own probe 3 witness, reached through the
/// unified trait.
type Product = <Half as BiasProduct<FiveHalves>>::Out;
const _: () = assert!(<Product as Bias>::NUM == 5 && <Product as Bias>::DEN == 4);

/// 1/2 * -5/2 = -5/4, a sign-mixing case neither file 41's aliases nor
/// probe 5's standalone trait individually needed to check.
type ProductNeg = <Half as BiasProduct<NegFiveHalves>>::Out;
const _: () = assert!(<ProductNeg as Bias>::NUM == -5 && <ProductNeg as Bias>::DEN == 4);

/// 1/2 * 0 = 0, confirming the new non-zero impls and file 41's own
/// zero-absorbing impls coexist without a coherence conflict.
type ProductZero = <Half as BiasProduct<BZero>>::Out;
const _: () = assert!(<ProductZero as Bias>::NUM == 0 && <ProductZero as Bias>::DEN == 1);
