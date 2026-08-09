#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features, dead_code)]
pub struct W<const N: u32>;
pub struct N<const P: u32, const Q: u32, const R: u32 = { P + Q }>;
