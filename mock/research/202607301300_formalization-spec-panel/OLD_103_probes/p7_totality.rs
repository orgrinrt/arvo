//! Probe 7. Whether D15's "properties as associated consts" reaches file 07's
//! rung 2, which requires the compiler to force every constructor to answer.
//!
//! Two encodings of the same property. With a default, silence is an answer and
//! the site is not load-bearing (07's own `a6` finding). Without one, silence
//! does not compile.
#![no_std]
#![allow(dead_code)]

pub trait Defaulted {
    const FRESH_ALWAYS_ACCEPTS: bool = true; // silence promises
}
pub struct Careless;
impl Defaulted for Careless {} // compiles, and has now promised

pub trait Total {
    const FRESH_ALWAYS_ACCEPTS: bool; // no default
}
pub struct Answering;
impl Total for Answering {
    const FRESH_ALWAYS_ACCEPTS: bool = false;
}
pub struct Silent;
impl Total for Silent {} // must not compile
