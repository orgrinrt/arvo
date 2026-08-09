//! p1 negative control A: the telescope's dependency is enforced, not decorative.
//!
//! An encoding declared over one identity attached to an adaptation declared over
//! another. If this compiled, the five components would be an independent product
//! and the telescope claim would be false.
//!
//! Expected: refused. Committed transcript in `p1_neg_a.stderr`.
//!   rustc --edition 2024 --crate-type lib p1_neg_a.rs

#![no_std]
#![allow(dead_code)]
#![allow(unused_attributes)]

#[path = "p1_telescope.rs"]
mod tele;

use core::marker::PhantomData;
use tele::*;

type RingS4 = Id<RingZ, S4>;
type RingU4 = Id<RingZ, U4>;

// Wrap is declared over RingS4; the encoding is declared over RingU4.
pub type Mismatched = Num<Wrap<RingS4>, Byte<TwosComplement<RingU4>>>;

pub fn use_it() -> u8 {
    let x: Mismatched = Num(0, PhantomData);
    x.0
}
