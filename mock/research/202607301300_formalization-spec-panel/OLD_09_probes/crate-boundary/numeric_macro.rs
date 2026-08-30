//! uses algebra_macro's exported macro to generate the ONE forwarding impl,
//! then tries to ALSO hand-write a second impl of the same trait for the
//! same type, conditioned on L, to see what stops it (if anything).
#![crate_type = "rlib"]
#![crate_name = "numeric_macro"]

extern crate algebra_macro;
extern crate lowering;
extern crate numeral;
extern crate policy;

use algebra_macro::AddAssoc;
use lowering::Lowering;
use numeral::Numeral;
use policy::Policy;

pub struct Number<N: Numeral, P: Policy, L: Lowering>(core::marker::PhantomData<(N, P, L)>);

algebra_macro::derive_add_assoc!(Number<N, P, L>);

pub fn fold<T: AddAssoc>() {}
pub fn ok() {
    fold::<Number<numeral::Fix13_3Signed, policy::Warm, lowering::MinWidth>>();
}
