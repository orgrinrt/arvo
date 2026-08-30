//! p14. Attacking p13's finding: can the alias-site silence be closed?
//!
//! p13 shows an undeclared width written at the alias-definition site produces
//! nothing there, and surfaces later at the first use, naming an internal type
//! the consumer has never seen. That is a defect of the const door and it is
//! shared by the design as it stands, since C0 has the same door.
//!
//! Rust has one lever on it: `lazy_type_alias`, which makes a type alias
//! well-formedness-checked at its own declaration. This file is minimal on
//! purpose, because turning the feature on over the whole ladder makes every
//! internal projection alias want bounds (out/p14_full.log records that, four
//! E0277s against `Sum`, `Cont`, `Q6` and `R5`), and that is a library-side cost
//! to be stated rather than a blocker.
//!
//! `lazy_type_alias` is NOT on the workspace's vetted feature list and this file
//! does not claim it is admissible. It establishes only what it would buy.
//!
//! THIS FILE IS EXPECTED TO FAIL TO COMPILE, AT LINE 45. That line number is the
//! result.
//!
//! rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib \
//!       --emit=metadata -o out/p14.meta p14_lazy_type_alias.rs 2> out/p14.log
#![feature(lazy_type_alias)]
#![allow(incomplete_features)]
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

pub struct Arvo;
pub struct Warm;
pub struct Idx<const N: u32>;
#[diagnostic::on_unimplemented(
    message = "no width literal `{Self}` is declared in this program",
    label = "this literal width is not declared",
    note = "a literal width is declared once, in one line"
)]
pub trait ToNat<M> {
    type N;
}
pub struct W5;
pub struct W13;
impl ToNat<Arvo> for Idx<5> {
    type N = W5;
}
impl ToNat<Arvo> for Idx<13> {
    type N = W13;
}

pub struct Fixed<WI, S>(PhantomData<(WI, S)>);
pub type NatOf<const N: u32>
    = <Idx<N> as ToNat<Arvo>>::N
where
    Idx<N>: ToNat<Arvo>;
pub type UInt<const N: u32>
    = Fixed<NatOf<N>, Warm>
where
    Idx<N>: ToNat<Arvo>;

// declared, fine
pub type StrHandle = UInt<13>;

// line 57: undeclared. Under lazy_type_alias the error should be HERE, not at
// the first use.
pub type PacketTag = UInt<7>;

pub struct Header {
    pub _pad: u8,
}
pub fn unrelated(h: Header) -> Header {
    h
}
