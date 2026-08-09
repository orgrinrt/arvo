//! Probe 5: file 41's section 3 states, and this dispatch's brief quotes
//! verbatim, that "the design cannot have a generic `BiasMul` trait" and
//! that `Reduce` "composes safely only as a bare, top-level type alias".
//! That conclusion is too strong. Probe 4 isolated the actual trigger
//! (naming `Reduce` itself as a bound, not the underlying machinery);
//! this file is the generic trait that mechanism predicts should exist,
//! built by spelling `Strip2`/`Gcd`/`ExactDivOdd`/`AsPos` directly rather
//! than through `Reduce`, mirroring file 41's own failing attempt
//! (section 3, `BiasMul<N1, D1, N2, D2>`) one construction later.
//!
//! One repair needed beyond the mechanical unbundling: file 41's own
//! failing draft, and `Reduce`'s own declaration, never state that a
//! reduced pair is coprime as a type-level fact (`Reduce`'s own
//! associated types are only bound `: Pos`, not `: Pos + Gcd<_, Out =
//! H>`); the algorithm's correctness is informal, established by the
//! math, not machine-checked. `BPos<N, D>`'s own bound to `Bias`
//! (`N: Pos + Gcd<D, Out = H>`) DOES demand this, so a generic `BiasMul`
//! producing a `BPos` output has to supply it, as an axiom already
//! implicit everywhere else in the design that treats a `Reduce` output
//! as coprime without proof.
//!
//! Correctness is checked against file 41's own two witnesses (probe 3,
//! `41_probes/`): 1/2 * 5/2 = 5/4, and the unreduced-magnitude case
//! 2/3 * 3/4 = 6/12 correctly renormalising to 1/2.
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_5_generic_biasmul.rs --out-dir <dir>
//! Outcome: WORKS, including both correctness witnesses (the const
//! assertions are load-bearing: a wrong reduction path would fail to
//! compile here, not merely produce a wrong runtime value, because
//! nothing here ever runs).
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

#[path = "vu_bias.rs"]
pub mod bias;

use bias::nat::{AsPos, ExactDivOdd, Gcd, Pos, Pz, Ratio, Strip2, H};
use bias::{BPos, Bias, PMul};

/// The generic trait file 41's own header wished for ("A trait would be
/// the nicer surface, `b1.mul(b2)`, uniform across sign combinations")
/// and its own section 3 concluded does not exist. It does.
pub trait BiasMulGeneric<Rhs> {
    type Out: Bias;
}

impl<N1, D1, N2, D2, RawN, RawD, StripN, StripD, Divisor, QuoN, QuoD, FinalN, FinalD>
    BiasMulGeneric<BPos<N2, D2>> for BPos<N1, D1>
where
    // the operands: genuine Bias magnitudes, already reduced (matching
    // BPos<N,D>'s own construction discipline; an unreduced pair is not
    // a well-typed BPos to begin with).
    N1: Pos + Gcd<D1, Out = H> + PMul<N2, Out = RawN>,
    D1: Pos + PMul<D2, Out = RawD>,
    N2: Pos + Gcd<D2, Out = H>,
    D2: Pos,
    RawN: Pos,
    RawD: Pos,
    // the reduction chain, spelled directly rather than through Reduce
    // (probe 4's finding: this composes generically; naming Reduce does
    // not).
    Ratio<RawN, RawD>: Strip2<N = StripN, D = StripD>,
    StripN: Pos + Gcd<StripD, Out = Divisor>,
    StripD: Pos,
    Divisor: Pos,
    Pz<StripN>: ExactDivOdd<Divisor, Out = QuoN>,
    Pz<StripD>: ExactDivOdd<Divisor, Out = QuoD>,
    QuoN: AsPos<Out = FinalN>,
    QuoD: AsPos<Out = FinalD>,
    // the axiom Reduce's own declaration leaves informal: the reduced
    // pair is coprime. Needed because BPos<N,D>'s own bound to Bias
    // demands it; Reduce's declared associated-type bound (`: Pos`
    // alone) does not carry it, so a generic caller has to assert it.
    FinalN: Pos + Gcd<FinalD, Out = H>,
    FinalD: Pos,
{
    type Out = BPos<FinalN, FinalD>;
}

pub type P1 = bias::nat::H;
pub type P2 = bias::nat::O<bias::nat::H>;
pub type P5 = bias::nat::I<bias::nat::O<bias::nat::H>>;

/// 1/2.
type Half = BPos<P1, P2>;
/// 5/2.
type FiveHalves = BPos<P5, P2>;
/// 1/2 * 5/2 = 5/4, file 41's own probe 3 witness.
type Product = <Half as BiasMulGeneric<FiveHalves>>::Out;
const _: () = assert!(<Product as Bias>::NUM == 5 && <Product as Bias>::DEN == 4);

/// 2/3.
type TwoThirds = BPos<P2, bias::nat::I<bias::nat::H>>;
/// 3/4.
type ThreeQuarters = BPos<bias::nat::I<bias::nat::H>, bias::nat::O<P2>>;
/// 2/3 * 3/4, raw componentwise product 6/12, must renormalise to 1/2.
type Product2 = <TwoThirds as BiasMulGeneric<ThreeQuarters>>::Out;
const _: () = assert!(<Product2 as Bias>::NUM == 1 && <Product2 as Bias>::DEN == 2);
