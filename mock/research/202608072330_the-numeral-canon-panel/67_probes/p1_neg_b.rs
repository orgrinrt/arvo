//! p1 negative control B: a law contract is not decided by the identity alone.
//!
//! The accepted case lives in `p1_telescope.rs::t4_law_is_not_decided_by_the_identity_alone`:
//! `Wrap<Id<RingZ, S4>>` passes `reassociating_fold`. This file offers the same
//! ambient domain, the same representable set, the same encoding and the same
//! container, changing only the adaptation, and it is refused.
//!
//! So the two terms agree on the telescope's first two components and disagree
//! on the third, and a sentence quantified over the first two has no truth value
//! while one quantified over the first three does. That is the whole content of
//! the identity-versus-realisation dispute, made mechanical.
//!
//! Expected: refused. Committed transcript in `p1_neg_b.stderr`.
//!   rustc --edition 2024 --crate-type lib p1_neg_b.rs

#![no_std]
#![allow(dead_code)]
#![allow(unused_attributes)]

#[path = "p1_telescope.rs"]
mod tele;

use core::marker::PhantomData;
use tele::*;

type RingS4 = Id<RingZ, S4>;
type SatRingS4Twos = Num<Saturate<RingS4>, Byte<TwosComplement<RingS4>>>;

pub fn refused() -> &'static str {
    let x: SatRingS4Twos = Num(0, PhantomData);
    reassociating_fold(x)
}
