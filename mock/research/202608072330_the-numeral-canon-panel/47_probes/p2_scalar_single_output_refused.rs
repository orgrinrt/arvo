// p2: the "one richer output" taken literally as a SCALAR, and refused by the compiler.
//
// The alternative this file's parent dispatch was sent to test is: maybe the carrier was
// under-specified and one richer output suffices. There are two ways to make an output richer.
// Make it a richer TYPE (p1, which works). Or make it a richer VALUE: one const encoding
// everything the pair encoded, which is lossless as INFORMATION and is the reading a reader
// naturally reaches for when told "a bit count is insufficient".
//
// This file builds the second and is EXPECTED TO FAIL TO COMPILE. The committed .err is the
// result. Three arms, three syntactic positions, so the refusal is not an artifact of one:
//
//   arm A: recover the carrier type from the packed const, via arithmetic  (type alias)
//   arm B: recover it without any arithmetic, the const used bare          (type alias)
//   arm C: the same recovery in a function return position                 (fn signature)
//
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib p2_scalar_single_output_refused.rs
//
// No #![feature] gate is enabled. generic_const_exprs and generic_const_args are both on the
// forbidden list, so a refusal naming either is a closed route rather than a shopping list.

#![no_std]

pub struct Hot;
pub struct Warm;
pub struct Cold;

pub trait Width {
    const BITS: u32;
}
pub struct W13;
impl Width for W13 {
    const BITS: u32 = 13;
}
pub struct W16;
impl Width for W16 {
    const BITS: u32 = 16;
}

// ---- the width-keyed type ladder a recovery would have to land on ----
pub struct Nat<const N: u32>;
pub trait NativeFor {
    type T: Copy;
}
impl NativeFor for Nat<8> {
    type T = u8;
}
impl NativeFor for Nat<16> {
    type T = u16;
}
impl NativeFor for Nat<32> {
    type T = u32;
}
impl NativeFor for Nat<64> {
    type T = u64;
}

// ---- the derivation, with ONE output, and that output a VALUE ----
//
// The encoding is lossless: carrier width in the low byte, stride in the next, access width in
// the next. Nothing the pair carried is missing. This is the strongest form of the
// "one richer output" proposal that is not a type.
pub trait DeriveScalar<S> {
    const REPR: u32;
}

pub const fn pack(carrier_bits: u32, stride_bits: u32, access_bits: u32) -> u32 {
    carrier_bits | (stride_bits << 8) | (access_bits << 16)
}
pub const fn carrier_bits(repr: u32) -> u32 {
    repr & 0xff
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

// ---- the stride comes back fine, because a const from a const is the free direction ----
pub const STRIDE_13_COLD: u32 = stride_bits(<W13 as DeriveScalar<Cold>>::REPR);
pub const STRIDE_16_COLD: u32 = stride_bits(<W16 as DeriveScalar<Cold>>::REPR);
const _: () = assert!(STRIDE_13_COLD == 13);
const _: () = assert!(STRIDE_16_COLD == 16);

// ---- and the carrier does not, in three positions ----

// arm A: with arithmetic, in a type alias.
pub type CarrierA<W, S> = <Nat<{ carrier_bits(<W as DeriveScalar<S>>::REPR) }> as NativeFor>::T;

// arm B: no arithmetic at all, the associated const used bare in const-argument position.
pub type CarrierB<W, S> = <Nat<{ <W as DeriveScalar<S>>::REPR }> as NativeFor>::T;

// arm C: the same recovery in a function's return position.
pub fn read_one<W: Width, S>(
    _bits: u64,
) -> <Nat<{ carrier_bits(<W as DeriveScalar<S>>::REPR) }> as NativeFor>::T
where
    W: DeriveScalar<S>,
{
    unimplemented!()
}
