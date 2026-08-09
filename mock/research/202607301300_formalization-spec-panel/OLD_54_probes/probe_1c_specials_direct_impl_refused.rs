//! Probe 1c (negative control): route one, a downstream direct impl of a sealed carrier.
//!
//! rustc --edition 2021 --crate-type lib --extern vu54=libvu54.rlib probe_1c_specials_direct_impl_refused.rs
//!
//! The attack a `Specials` seal exists to refuse: a downstream declares its own instance
//! claiming NaN is present while the encoding has no NaN datum, or claiming both absent
//! while the numeral's own value set has infinities. Either forges the value set the
//! crossing contract quantifies over (probe 2), so `Specials` is a carrier and owes a seal.
//!
//! Expected: E0277 on the private supertrait, for each of the four carriers minted here.

#![allow(dead_code)]

use vu54::numeral::{Radix, SignDomain, Specials, Underflow};

pub struct ForgedSpecials;
impl Specials for ForgedSpecials {
    const INF: bool = true;
    const NAN: bool = true;
    const NAN_DATA_MIN: u32 = 0;
}

pub struct ForgedUnderflow;
impl Underflow for ForgedUnderflow {
    const GRADUAL: bool = true;
}

pub struct ForgedDomain;
impl SignDomain for ForgedDomain {
    const SIGNED: bool = true;
}

pub struct ForgedRadix;
impl Radix for ForgedRadix {
    type Digits = vu54::numeral::P2;
    const R: u64 = 1;
}
