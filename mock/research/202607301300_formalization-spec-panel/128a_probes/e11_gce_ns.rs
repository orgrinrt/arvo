#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
pub struct W<const N: u16>;
pub type Twice<const N: u16> = W<{ N * 2 }>;
