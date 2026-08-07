// Negative control for p6 arm B.  Accumulator grid COARSER than the element's,
// which p5.out Q4 measures as the unsound region.  The bound must refuse, and it
// must refuse at type-check rather than at monomorphisation.
#![no_std]
#![allow(dead_code)]
include!("p6_core.rs");

type AccCoarse = U<3, N1>;
type ElemFine = U<1, N3>;

#[inline(never)]
pub fn must_not_compile(seed: u64, xs: &[u64]) -> u64 {
    fold_arm_b::<AccCoarse, ElemFine>(seed, xs)
}
