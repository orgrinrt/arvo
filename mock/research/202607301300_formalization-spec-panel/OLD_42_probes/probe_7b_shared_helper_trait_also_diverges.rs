//! Probe 7b: the finding recorded in probe 7's own header, preserved as
//! its own committed, refusing artifact, corrected once from an initial
//! overgeneralisation caught by testing before this file settled. The
//! FIRST attempt at this file named `Mag: Magnitude<(N1, D1, N2, D2)>`
//! alone, unused, on a bare function, expecting it to diverge the way
//! probe 4b's bare `Ratio<N, D>: Reduce` does. It did not: that bound
//! alone, with nothing projected from it, compiles clean. The actual
//! trigger, isolated by testing rather than assumed, needs one more
//! ingredient: a further bound projecting and re-checking the trait's
//! own associated type (`MagN<N1, D1, N2, D2>: Pos`, i.e.
//! `<Mag as Magnitude<...>>::OutN: Pos`), which is exactly what a real
//! caller has to write to name the computed type at all (probe 7's own
//! `BiasProduct` impls, before they were rewritten to inline the chain,
//! needed precisely this to name the output type). So the corrected
//! statement is narrower than probe 4/4b's own finding for `Reduce`:
//! `Reduce` diverges as a bare, unused, unprojected bound; a freshly
//! declared trait of the same one-blanket-impl shape does not
//! necessarily diverge until its associated type is actually projected
//! and re-bounded, which is what any real use of it requires. Both
//! converge on the same practical conclusion (spell the chain directly;
//! a shared trait, even a fresh one nobody has used before, is not a
//! safe place to hide it), by two different routes to the same wall,
//! and the difference between the two routes is recorded here rather
//! than smoothed over.
//!
//! Committed as a FAILS that should stay a FAILS. Do not "fix" this
//! file by making it compile differently; the finding is that a
//! reasonable-looking factoring attempt hits the wall once actually
//! used, not that this specific formulation was wrong.
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_7b_shared_helper_trait_also_diverges.rs --out-dir <dir>
//! Outcome: FAILS WITH E0275, verbatim in OUTCOMES.md.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

#[path = "vu_bias.rs"]
pub mod bias;

use bias::nat::{AsPos, ExactDivOdd, Gcd, Pos, Pz, Ratio, Strip2, H};
use bias::{BPos, Bias, PMul};

/// An ordinary helper trait, one blanket impl, exactly the shape a
/// reasonable factoring attempt reaches for.
trait Magnitude<T> {
    type OutN: Pos;
    type OutD: Pos;
}
struct Mag;
impl<N1, D1, N2, D2, RawN, RawD, StripN, StripD, Divisor, QuoN, QuoD, FinalN, FinalD>
    Magnitude<(N1, D1, N2, D2)> for Mag
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
    type OutN = FinalN;
    type OutD = FinalD;
}
type MagN<N1, D1, N2, D2> = <Mag as Magnitude<(N1, D1, N2, D2)>>::OutN;
type MagD<N1, D1, N2, D2> = <Mag as Magnitude<(N1, D1, N2, D2)>>::OutD;

pub trait BiasProductLike<Rhs> {
    type Out: Bias;
}

/// Naming `Magnitude` as a bound AND projecting its own associated type
/// into a further bound, the shape any real consumer of `Magnitude`
/// needs in order to name the computed output type. This is the
/// version that diverges; a bare, unused `Mag: Magnitude<(N1, D1, N2,
/// D2)>` with nothing projected from it, tried first while building
/// this file, does not (recorded in OUTCOMES.md as the corrected
/// negative control, not committed as a separate file).
impl<N1, D1, N2, D2> BiasProductLike<BPos<N2, D2>> for BPos<N1, D1>
where
    Mag: Magnitude<(N1, D1, N2, D2)>,
    MagN<N1, D1, N2, D2>: Pos,
    MagD<N1, D1, N2, D2>: Pos,
{
    type Out = BPos<MagN<N1, D1, N2, D2>, MagD<N1, D1, N2, D2>>;
}
