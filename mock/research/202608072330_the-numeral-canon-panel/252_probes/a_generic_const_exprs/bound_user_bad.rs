// A3 control, negative half: 6 + 6 = 12, which is not implemented. MUST fail,
// in a consumer carrying no feature attribute. If it fails, the consumer really
// evaluates the const expression and checks the bound, so A3's clean build is a
// result about containment rather than about the bound going unchecked.
#![no_std]
use bound_lib2::joined;
pub const TOTAL: usize = joined::<6, 6>();
