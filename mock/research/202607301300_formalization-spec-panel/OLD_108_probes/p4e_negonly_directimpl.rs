#![no_std]
#![crate_type = "lib"]
#![feature(negative_impls)]
pub trait TruthAlgebra: Copy + PartialEq {
    const TRUE: Self;
}
pub trait Branch: TruthAlgebra {
    #[inline(always)]
    fn is_true(self) -> bool {
        self == Self::TRUE
    } // route R1 default body
}
#[derive(Clone, Copy, PartialEq)]
pub struct Mask2([bool; 2]);
impl TruthAlgebra for Mask2 {
    const TRUE: Self = Mask2([true, true]);
}
impl !Branch for Mask2 {}
// route R1 taken by a later editor:
impl Branch for Mask2 {}
