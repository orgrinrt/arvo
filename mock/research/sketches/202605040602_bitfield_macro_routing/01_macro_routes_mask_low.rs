//! Probe: bitfield macro routes mask construction through
//! `BitPrim::mask_low` at the dispatched container type.
//!
//! Mirrors the bitfield macro's emission shape with stub `BitPrim`
//! impls on bare u{8,16,32,64}. Demonstrates that the routed call
//! evaluates at compile time and composes with `<<` shift and `&`
//! bitand on the same container type, inside a const fn body.
//!
//! Run:
//!   rustc +nightly --edition 2024 --crate-type rlib \
//!       01_macro_routes_mask_low.rs -o /tmp/bitfield_routing.rlib
//!
//! Outcome: compiles cleanly. The macro can route through mask_low.

#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]

#[derive(Copy, Clone)]
pub struct USize(pub usize);

pub const trait BitPrim: Copy + 'static {
    const WIDTH: USize;
    const ZERO: Self;
    const ONE: Self;
    fn mask_low(n: USize) -> Self;
}

macro_rules! impl_bitprim {
    ($ty:ty, $width:expr) => {
        impl const BitPrim for $ty {
            const WIDTH: USize = USize($width);
            const ZERO: Self = 0;
            const ONE: Self = 1;
            fn mask_low(n: USize) -> Self {
                if n.0 >= $width {
                    <$ty>::MAX
                } else if n.0 == 0 {
                    0
                } else {
                    (1 as $ty).wrapping_shl(n.0 as u32) - 1
                }
            }
        }
    };
}

impl_bitprim!(u8, 8);
impl_bitprim!(u16, 16);
impl_bitprim!(u32, 32);
impl_bitprim!(u64, 64);

// --- the macro's mask-construction shape, parametric on container ---

const fn build_slot_mask_u8(field_bits: usize, lo: usize, n: usize) -> u8 {
    let mask = <u8 as BitPrim>::mask_low(USize(field_bits));
    let parent_mask = <u8 as BitPrim>::mask_low(USize(n));
    (mask << lo) & parent_mask
}

const fn build_slot_mask_u16(field_bits: usize, lo: usize, n: usize) -> u16 {
    let mask = <u16 as BitPrim>::mask_low(USize(field_bits));
    let parent_mask = <u16 as BitPrim>::mask_low(USize(n));
    (mask << lo) & parent_mask
}

const fn build_slot_mask_u32(field_bits: usize, lo: usize, n: usize) -> u32 {
    let mask = <u32 as BitPrim>::mask_low(USize(field_bits));
    let parent_mask = <u32 as BitPrim>::mask_low(USize(n));
    (mask << lo) & parent_mask
}

const fn build_slot_mask_u64(field_bits: usize, lo: usize, n: usize) -> u64 {
    let mask = <u64 as BitPrim>::mask_low(USize(field_bits));
    let parent_mask = <u64 as BitPrim>::mask_low(USize(n));
    (mask << lo) & parent_mask
}

// --- const canaries: each evaluates at compile time ----------------

const _CANARY_U8: () = {
    // 3-bit slot at lo=4 inside an 8-bit parent. Mask = 0b0111_0000.
    let m = build_slot_mask_u8(3, 4, 8);
    assert!(m == 0b0111_0000);
};

const _CANARY_U16: () = {
    // 5-bit slot at lo=2 inside a 12-bit parent.
    // Slot mask = 0b0001_1111 << 2 = 0b0_0111_1100.
    // Parent mask = (1 << 12) - 1 = 0x0FFF.
    // Result = 0x07C.
    let m = build_slot_mask_u16(5, 2, 12);
    assert!(m == 0x007C);
};

const _CANARY_U32: () = {
    // 17-bit slot at lo=8 inside a 28-bit parent.
    let m = build_slot_mask_u32(17, 8, 28);
    let expected = ((1u32 << 17) - 1) << 8 & ((1u32 << 28) - 1);
    assert!(m == expected);
};

const _CANARY_U64: () = {
    // 33-bit slot at lo=16 inside a 64-bit parent (parent uses
    // saturating-at-WIDTH path).
    let m = build_slot_mask_u64(33, 16, 64);
    let expected = ((1u64 << 33) - 1) << 16; // & u64::MAX = no-op
    assert!(m == expected);
};

const _CANARY_U64_FULL_WIDTH: () = {
    // Field = full container width. mask_low saturates to MAX.
    let m = build_slot_mask_u64(64, 0, 64);
    assert!(m == u64::MAX);
};
