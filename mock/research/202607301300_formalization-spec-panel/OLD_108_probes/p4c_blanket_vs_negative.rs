// p4c: with the absence declared, does a later blanket impl (route R2) still
// compile, or is it a coherence error?
#![no_std]
#![crate_type = "lib"]
#![feature(negative_impls)]
#![feature(with_negative_coherence)]
pub trait TruthAlgebra: Copy + PartialEq {
    const TRUE: Self;
}
pub trait Branch: TruthAlgebra {
    fn is_true(self) -> bool;
}
#[derive(Clone, Copy, PartialEq)]
pub struct Mask2([bool; 2]);
impl TruthAlgebra for Mask2 {
    const TRUE: Self = Mask2([true, true]);
}
impl !Branch for Mask2 {}
// route R2, written later by someone who did not read the rule:
impl<T: TruthAlgebra> Branch for T {
    fn is_true(self) -> bool {
        self == T::TRUE
    }
}
