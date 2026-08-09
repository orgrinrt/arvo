#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
#![no_std]
#![allow(dead_code)]
include!("cap_core.rs");
// Can "precision 16" be stated as a where clause, so it is checked at the
// signature rather than at instantiation?
pub fn wants_precision_16<A: Format<PRECISION = 16>>(_: A) {}
