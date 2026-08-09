//! p5 negative control: the strategy selection that loses the law is refused.
//!
//! `Guarded` at the signed window selects saturating addition over a
//! sign-crossing range, which p4 measures at 952 associativity violations over
//! Q^3. The reassociating fold demands the law and is refused here, while the
//! same instantiation passes the sequential fold in the positive file.
//!
//! Expected: refused. Committed transcript in `p5_neg.stderr`.
//!   rustc --edition 2024 --crate-type lib p5_neg.rs

#![no_std]
#![allow(dead_code)]
#![allow(unused_attributes)]

#[path = "p5_strategy_selects_the_member.rs"]
mod sel;

use sel::*;

pub fn refused() -> &'static str {
    reassociating_fold::<SignedWindow, Guarded>()
}
