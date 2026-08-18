//! P2b. The case that must fail: writing a packed 13-bit element as an instance
//! of an element-level numeral signature.
//!
//! This file is EXPECTED NOT TO COMPILE. Its compiler output is the finding and
//! is committed as `fibre_refuted.err`. Per
//! `a-test-that-cannot-compile-is-the-finding.md`, it is not to be restructured
//! until it builds; the refusal is the result.
#![no_std]
#![allow(dead_code)]

pub const W: u32 = 13;

pub trait Numeral: Copy {
    const WIDTH: u32;
    fn to_u64(self) -> u64;
}

// Attempt 1. The honest declaration of what a packed element is: 13 bits.
// Rust has no such type. The nearest expressible thing is a bitfield-shaped
// newtype, which is 16 bits, i.e. the dense instance wearing a packed name.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Packed13Elem([bool; 13]);

impl Numeral for Packed13Elem {
    const WIDTH: u32 = W;
    fn to_u64(self) -> u64 {
        let mut v = 0u64;
        let mut i = 0;
        while i < 13 {
            v |= (self.0[i] as u64) << i;
            i += 1;
        }
        v
    }
}

// The assertion that makes the failure explicit rather than incidental: if a
// packed element were an element at all, its footprint would be its width.
const _: () = assert!(
    core::mem::size_of::<Packed13Elem>() * 8 == W as usize,
    "a packed 13-bit element does not occupy 13 bits as a standalone value"
);

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
