// Tick 3, shape B, the negative control: make mul_full's growth actually
// DEPEND on which policy is live, by writing two implementations of the
// same growth trait for the same (MulFull, N1, N2) triple that disagree
// on Out. This is the only way growth could vary "by policy" without
// threading Policy into the trait's own parameter list (probe 2 already
// shows that route is inert). It is refused before rustc even reaches a
// question about correctness: coherence forbids two impls of one trait
// for one type at overlapping generic instantiations, full stop. This is
// expected to fail with E0119.

#![allow(dead_code)]

pub trait Numeral {
    const P: u32;
}
pub struct N8;
impl Numeral for N8 {
    const P: u32 = 8;
}
pub struct N16;
impl Numeral for N16 {
    const P: u32 = 16;
}

pub struct MulNum<N1, N2>(core::marker::PhantomData<(N1, N2)>);
impl<N1: Numeral, N2: Numeral> Numeral for MulNum<N1, N2> {
    const P: u32 = N1::P + N2::P;
}
pub struct HalfMulNum<N1, N2>(core::marker::PhantomData<(N1, N2)>);
impl<N1: Numeral, N2: Numeral> Numeral for HalfMulNum<N1, N2> {
    const P: u32 = (N1::P + N2::P) / 2;
}

pub struct MulFull;

pub trait MulFullGrowth<N1: Numeral, N2: Numeral> {
    type Out: Numeral;
}

// the canonical closure formula (49:269), un-conditioned on any policy.
impl<N1: Numeral, N2: Numeral> MulFullGrowth<N1, N2> for MulFull {
    type Out = MulNum<N1, N2>;
}

// an attempt at a second, policy-flavoured growth for the identical
// (MulFull, N1, N2) domain. There is no way to write this that rustc
// accepts: it is not gated behind a where-clause disambiguating the two
// (both are fully generic over the same N1, N2), so it is a bare
// coherence conflict.
impl<N1: Numeral, N2: Numeral> MulFullGrowth<N1, N2> for MulFull {
    type Out = HalfMulNum<N1, N2>;
}

fn main() {}
