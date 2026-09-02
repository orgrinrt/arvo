// t2. The erasure check the acceptance criterion's fourth clause actually needs.
//
// The panel's instrument (15's q12) compares ONE typed operation on ONE value against
// ONE native instruction. That is a real result and it is quantified over a scalar. The
// clause says the design erases "to be exactly what you describe", and what a Cold
// consumer describes is a packed run, so the instrument has to compare a typed walk over
// an aggregate against the hand-written walk it claims to erase to.
//
// This probe builds both and emits assembly for each. The question is whether the typed
// form's body is instruction-identical to the hand-written form's body.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O --emit asm --crate-type lib \
//     t2_aggregate_erasure.rs --out-dir asm
//
// Features: none. Checked by grep in verify.sh.
//
// Nothing here is timed. Instruction counts below are counts of lines in emitted
// assembly, which is a count and not a measurement of speed. No bench harness has run in
// this panel, so what any of this COSTS is unpriced.
//
// Spike. Presume it flawed. The typestate here is a two-impl stand-in, not the panel's
// ladder; what it checks is whether a derivation's second output survives lowering, not
// how the derivation should be keyed.

#![no_std]
#![allow(dead_code)]

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// A two-output derivation, in the smallest form that carries both outputs.
// The ladder is not the subject here; the pair is.
// ---------------------------------------------------------------------------
pub trait Derived {
    /// output one: what an operation lowers to
    type Carrier: Copy;
    /// the declared width, which the consumer wrote
    const W: usize;
    /// output two: what one element occupies in a run
    const STRIDE: usize;
    /// the byte span a field can occupy at unknown phase
    const ACCESS: usize;
}

pub struct Cold13;
impl Derived for Cold13 {
    type Carrier = u16;
    const W: usize = 13;
    const STRIDE: usize = 13;
    const ACCESS: usize = 3;
}

pub struct Warm13;
impl Derived for Warm13 {
    type Carrier = u16;
    const W: usize = 13;
    const STRIDE: usize = 16;
    const ACCESS: usize = 2;
}

/// Isolates the stride. Byte-aligned like Warm13, but with Cold13's 3-byte access
/// window, so a difference between this and Warm13 is the access width and a difference
/// between this and Cold13 is the stride.
pub struct Aligned13Access3;
impl Derived for Aligned13Access3 {
    type Carrier = u16;
    const W: usize = 13;
    const STRIDE: usize = 16;
    const ACCESS: usize = 3;
}

// N is fixed at the probe's instantiation so no expression sits in type position.
// 1000 elements at 13 bits is 1625 bytes.
pub const N: usize = 1000;
pub const BYTES_COLD: usize = 1625;

#[repr(transparent)]
pub struct Col<D: Derived> {
    bytes: [u8; BYTES_COLD],
    _p: PhantomData<D>,
}

impl<D: Derived> Col<D> {
    #[inline(always)]
    fn get(&self, k: usize) -> u32 {
        let base = k * D::STRIDE;
        let byte = base / 8;
        let phase = base % 8;
        let mut acc: u64 = 0;
        let mut i = 0;
        while i < D::ACCESS {
            acc |= (self.bytes[byte + i] as u64) << (8 * i);
            i += 1;
        }
        ((acc >> phase) as u32) & ((1u32 << D::W) - 1)
    }
}

// The typed walk. Everything about the layout comes from the derivation.
#[unsafe(no_mangle)]
pub extern "C" fn t2_typed_sum(c: &Col<Cold13>) -> u64 {
    let mut s: u64 = 0;
    let mut k = 0;
    while k < 999 {
        s += c.get(k) as u64;
        k += 1;
    }
    s
}

// The hand-written walk a consumer would write if they did the packing themselves.
// This is the "exactly what you describe" side of the clause.
#[unsafe(no_mangle)]
pub extern "C" fn t2_handwritten_sum(b: &[u8; BYTES_COLD]) -> u64 {
    let mut s: u64 = 0;
    let mut k = 0usize;
    while k < 999 {
        let base = k * 13;
        let byte = base / 8;
        let phase = base % 8;
        let acc = (b[byte] as u64) | ((b[byte + 1] as u64) << 8) | ((b[byte + 2] as u64) << 16);
        s += (((acc >> phase) as u32) & 0x1FFF) as u64;
        k += 1;
    }
    s
}

// The control: the same typed walk over a derivation whose second output was LOST, so
// the stride fell back to the carrier's width. If this body is identical to the Cold one
// then the second output did not survive lowering.
/// Iteration count is matched to t2_typed_sum at 400 so the only difference between
/// the three walks below is the derivation. 400 elements at stride 16 is 800 bytes,
/// well inside BYTES_COLD, so no walk reads out of range.
#[unsafe(no_mangle)]
pub extern "C" fn t2_typed_sum_warm(c: &Col<Warm13>) -> u64 {
    let mut s: u64 = 0;
    let mut k = 0;
    while k < 400 {
        s += c.get(k) as u64;
        k += 1;
    }
    s
}

#[unsafe(no_mangle)]
pub extern "C" fn t2_typed_sum_aligned_access3(c: &Col<Aligned13Access3>) -> u64 {
    let mut s: u64 = 0;
    let mut k = 0;
    while k < 400 {
        s += c.get(k) as u64;
        k += 1;
    }
    s
}

#[unsafe(no_mangle)]
pub extern "C" fn t2_typed_sum_cold_400(c: &Col<Cold13>) -> u64 {
    let mut s: u64 = 0;
    let mut k = 0;
    while k < 400 {
        s += c.get(k) as u64;
        k += 1;
    }
    s
}

// The scalar pair, which is what the panel's existing instrument compares. Included so
// the two arities sit in one file and the difference is visible in one assembly dump.
#[unsafe(no_mangle)]
pub extern "C" fn t2_scalar_typed(a: u16, b: u16, c: u16) -> u16 {
    a.wrapping_mul(b).wrapping_add(c)
}
#[unsafe(no_mangle)]
pub extern "C" fn t2_scalar_native(a: u16, b: u16, c: u16) -> u16 {
    a.wrapping_mul(b).wrapping_add(c)
}
