// Negative control for p6 arm A.  Same offending pair, the post-monomorphisation
// assert instead of the bound.  Recorded so the two arms' failure MOMENTS can be
// compared rather than asserted.
#![no_std]
#![allow(dead_code)]
include!("p6_core.rs");

type AccCoarse = U<3, N1>;
type ElemFine = U<1, N3>;

#[inline(never)]
pub fn must_not_compile(seed: u64, xs: &[u64]) -> u64 {
    fold_arm_a::<AccCoarse, ElemFine>(seed, xs)
}
