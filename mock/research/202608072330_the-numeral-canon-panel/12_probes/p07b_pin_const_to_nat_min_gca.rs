//! p07b. Attacking p06's residue: the printable const must be PINNED to the nat
//! or it can lie.
//!
//! p06 puts the consts in printable position and the nats in defaulted position,
//! and the headline diagnostic recovers ("expected 13, found 26"). But nothing
//! ties the const to the nat: a caller could name a numeral whose printed width
//! is not its real width. Pinning needs the nat's value in const position.
//!
//! The nat already CARRIES its value as an associated const (`Nat::V`, computed
//! structurally, no table). The question is whether an associated const may
//! appear in a const-argument position. Three syntactic positions, three files.
//! This one uses `min_generic_const_args`, which is allowed.
//!
//! THIS FILE IS EXPECTED TO FAIL. The result is WHICH FEATURE rustc names.
//!
//! rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib \
//!       --emit=metadata -o out/p07b.meta p07b_pin_const_to_nat_min_gca.rs 2> out/p07b.log
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");

pub struct Hot;

// the pinning relation: "this nat has this value".
pub trait NatIs<const N: u32> {}

// ONE blanket impl, structural, no table. If this is admitted, the const
// coordinate can never disagree with the nat, and p06's residue closes.
impl<W: Nat> NatIs<{ <W as Nat>::V }> for W {}

pub struct Fixed<const I: u32, S, WI>(PhantomData<(S, WI)>)
where
    WI: NatIs<I>;
