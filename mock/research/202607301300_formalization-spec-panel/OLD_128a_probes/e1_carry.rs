//! E1: can a const be carried into a type by ONE blanket impl (no enumeration),
//! and is the resulting type canonical (identity determined by the value alone)?

#![no_std]
#![feature(const_trait_impl)]

mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

/// The carrier: a const carried, never transformed.
pub struct W<const N: u16>;
impl<const N: u16> Sealed for W<N> {}

pub const trait Nat: Sealed {
    const VAL: u64;
}

// ONE impl. Not a table. Covers every u16 width, no cap, no range chosen.
const impl<const N: u16> Nat for W<N> {
    const VAL: u64 = N as u64;
}

// --- canonicity probe 1: is W<16> one type however it is spelled? ---
const _: () = assert!(<W<16> as Nat>::VAL == 16);

fn takes16(_: W<16>) {}
fn spelled_two_ways() {
    takes16(W::<{ 8 + 8 }>);
    takes16(W::<{ 13 + 3 }>);
    takes16(W::<16>);
}

// --- canonicity probe 2: through a GENERIC parameter, which is the real case ---
// Does rustc identify W<A> with W<16> when A is a generic const equal to 16?
fn generic_site<const A: u16>(w: W<A>) -> W<A> {
    w
}
fn call_generic() {
    let _: W<16> = generic_site::<16>(W::<16>);
    // the two spellings meet at one type:
    let a: W<16> = generic_site::<{ 13 + 3 }>(W);
    let b: W<16> = generic_site::<{ 8 + 8 }>(W);
    takes16(a);
    takes16(b);
}
