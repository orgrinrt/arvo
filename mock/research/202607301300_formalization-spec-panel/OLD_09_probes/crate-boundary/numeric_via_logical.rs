//! a crate WITH Lowering in scope, instantiating LogicalNumber with a real
//! Lowering type for L, confirming the law's truth is now invariant under
//! L for both Dense and Bitpacked layouts alike, by construction.
#![crate_type = "rlib"]
#![crate_name = "numeric_via_logical"]
extern crate algebra_logical;
extern crate lowering;
extern crate numeral;
extern crate policy;

use algebra_logical::{AddAssoc, LogicalNumber};

pub fn fold<T: AddAssoc>() {}
pub fn dense_ok() {
    fold::<LogicalNumber<numeral::Fix13_3Signed, policy::Warm, lowering::MinWidth>>();
}
pub fn bitpacked_ok_too() {
    fold::<LogicalNumber<numeral::Fix13_3Signed, policy::Warm, lowering::DoubleWidth>>();
}
