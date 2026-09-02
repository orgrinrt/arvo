//! The library half for the `generic` price sweep kind: the exact trait
//! from probe_5_generic_biasmul.rs, extracted to its own file so the
//! sweep can `--extern` it once per point rather than recompiling probe
//! 5's own correctness witnesses at every point. Identical mechanism,
//! no behavioural difference from the committed probe.

#![allow(dead_code)]

#[path = "../vu_bias.rs"]
pub mod bias;

use bias::nat::{AsPos, ExactDivOdd, Gcd, Pos, Pz, Ratio, Strip2, H};
use bias::{BPos, Bias, PMul};

pub trait BiasMulGeneric<Rhs> {
    type Out: Bias;
}

impl<N1, D1, N2, D2, RawN, RawD, StripN, StripD, Divisor, QuoN, QuoD, FinalN, FinalD>
    BiasMulGeneric<BPos<N2, D2>> for BPos<N1, D1>
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
