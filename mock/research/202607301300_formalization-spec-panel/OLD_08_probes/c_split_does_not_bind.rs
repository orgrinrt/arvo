//! Does 02 sec 5's parameter split make "a law may not condition on Lowering"
//! a TYPING fact, as 02 claims, or is it still review discipline?
//! 02's `d_fusion.rs` showed a law conditioned on Lowering compiles under the
//! FUSED parameter. This asks the same question under the SPLIT parameter.
use crate::*;

pub trait AddAssocIllegal {}

/// A law conditioned on a Lowering member, under split parameters.
impl<N: Numeral, P: Policy, L: Lowering> AddAssocIllegal for Number<N, P, L> where L::Layout: IsDense
{}
pub trait IsDense {}
impl IsDense for Dense {}

pub fn needs_illegal<T: AddAssocIllegal>() {}
pub fn ok() {
    needs_illegal::<Number<Fix<13, 3, Signed>, Warm, Warm>>()
}
