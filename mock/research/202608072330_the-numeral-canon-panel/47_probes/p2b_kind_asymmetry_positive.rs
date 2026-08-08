// p2b: the positive half of p2, so p2's refusal is a statement about DIRECTION and not about
// the encoding being broken.
//
// p2 shows a scalar single output cannot yield a TYPE. This file shows the same scalar output
// yields every CONST fine, generically, in a const fn body, gate-free. And it shows the other
// direction: a TYPE yields consts fine too (size_of, align_of, associated consts).
//
// Together the two files pin the asymmetry that decides the topic:
//
//     type  -> const   total, free, gate-free, both concretely and generically
//     const -> type    refused, naming generic_const_exprs, which is forbidden
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p2b_kind_asymmetry_positive.rs -o bin/p2b && ./bin/p2b
//
// No #![feature] gate.

#![no_std]
extern crate std;
use std::println;

pub struct Warm;
pub struct Cold;

pub struct W13;
pub struct W16;

// ---- direction 1: a scalar output yields consts, generically ----
pub trait DeriveScalar<S> {
    const REPR: u32;
}
pub const fn pack(carrier_bits: u32, stride_bits: u32, access_bits: u32) -> u32 {
    carrier_bits | (stride_bits << 8) | (access_bits << 16)
}
pub const fn stride_bits(repr: u32) -> u32 {
    (repr >> 8) & 0xff
}
impl DeriveScalar<Warm> for W13 {
    const REPR: u32 = pack(16, 16, 16);
}
impl DeriveScalar<Cold> for W13 {
    const REPR: u32 = pack(16, 13, 32);
}
impl DeriveScalar<Cold> for W16 {
    const REPR: u32 = pack(16, 16, 32);
}

/// generic over BOTH the width marker and the strategy, and it compiles.
pub const fn stride_of<W: DeriveScalar<S>, S>() -> u32 {
    stride_bits(<W as DeriveScalar<S>>::REPR)
}

const _: () = assert!(stride_of::<W13, Cold>() == 13);
const _: () = assert!(stride_of::<W16, Cold>() == 16);
const _: () = assert!(stride_of::<W13, Warm>() == 16);

// ---- direction 2: a type output yields consts, generically ----
pub trait Representation {
    type Carrier: Copy;
    type Access: Copy;
    const STRIDE_BITS: u32;
}
pub struct PaddedU16;
impl Representation for PaddedU16 {
    type Carrier = u16;
    type Access = u16;
    const STRIDE_BITS: u32 = 16;
}
pub struct Packed13;
impl Representation for Packed13 {
    type Carrier = u16;
    type Access = u32;
    const STRIDE_BITS: u32 = 13;
}

pub trait DeriveType<S> {
    type Repr: Representation;
}
impl DeriveType<Warm> for W13 {
    type Repr = PaddedU16;
}
impl DeriveType<Cold> for W13 {
    type Repr = Packed13;
}

/// every const the scalar form carried, recovered from the TYPE form, generically.
pub const fn stride_of_t<W: DeriveType<S>, S>() -> u32 {
    <<W as DeriveType<S>>::Repr as Representation>::STRIDE_BITS
}
pub const fn carrier_bits_t<W: DeriveType<S>, S>() -> u32 {
    (core::mem::size_of::<<<W as DeriveType<S>>::Repr as Representation>::Carrier>() * 8) as u32
}
pub const fn access_bits_t<W: DeriveType<S>, S>() -> u32 {
    (core::mem::size_of::<<<W as DeriveType<S>>::Repr as Representation>::Access>() * 8) as u32
}
pub const fn carrier_align_t<W: DeriveType<S>, S>() -> u32 {
    core::mem::align_of::<<<W as DeriveType<S>>::Repr as Representation>::Carrier>() as u32
}

const _: () = assert!(stride_of_t::<W13, Cold>() == 13);
const _: () = assert!(carrier_bits_t::<W13, Cold>() == 16);
const _: () = assert!(access_bits_t::<W13, Cold>() == 32);
const _: () = assert!(carrier_align_t::<W13, Cold>() == 2);
const _: () = assert!(stride_of_t::<W13, Warm>() == 16);
const _: () = assert!(access_bits_t::<W13, Warm>() == 16);

// ---- and the type form yields the TYPE too, generically, which is what p2 could not do ----
pub fn widen_one<W: DeriveType<S>, S>(
    x: <<W as DeriveType<S>>::Repr as Representation>::Carrier,
) -> <<W as DeriveType<S>>::Repr as Representation>::Access
where
    <<W as DeriveType<S>>::Repr as Representation>::Access:
        From<<<W as DeriveType<S>>::Repr as Representation>::Carrier>,
{
    x.into()
}

fn main() {
    println!("direction: scalar output -> const, generically");
    println!("  stride_of::<W13, Cold>()  = {}", stride_of::<W13, Cold>());
    println!("  stride_of::<W16, Cold>()  = {}", stride_of::<W16, Cold>());
    println!("  stride_of::<W13, Warm>()  = {}", stride_of::<W13, Warm>());
    println!();
    println!("direction: type output -> const, generically");
    println!(
        "  W13/Cold  stride={} carrier={} access={} align={}",
        stride_of_t::<W13, Cold>(),
        carrier_bits_t::<W13, Cold>(),
        access_bits_t::<W13, Cold>(),
        carrier_align_t::<W13, Cold>()
    );
    println!();
    println!("direction: type output -> type, generically");
    let widened: u32 = widen_one::<W13, Cold>(8191u16);
    println!("  widen_one::<W13, Cold>(8191u16) -> u32 = {}", widened);
    println!("  the same call at Warm returns u16, from the same source line:");
    let widened_w: u16 = widen_one::<W13, Warm>(8191u16);
    println!("  widen_one::<W13, Warm>(8191u16) -> u16 = {}", widened_w);
    println!();
    println!("p2 could write none of the last block. its output was a value, not a type.");
}
