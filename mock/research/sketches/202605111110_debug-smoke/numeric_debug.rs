//! Smoke test for `core::fmt::Debug` on arvo numerics.
//!
//! Round 202605111110. Confirms representative widths and strategies
//! produce non-empty Debug output via `core::fmt::write` (no alloc
//! dep). End-to-end check on the substrate-level Debug surface.

#![no_std]

use core::fmt::Write;

use arvo::strategy::{Additive, Identity, Warm};
use arvo::{FastFloat, Int, Uint};
use arvo_storage::{Bits, Bool, Cap, USize};

/// Fixed-size buffer that implements `fmt::Write`.
struct Buf<const N: usize> {
    data: [u8; N],
    used: usize,
}

impl<const N: usize> Buf<N> {
    fn new() -> Self {
        Self {
            data: [0; N],
            used: 0,
        }
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.data[..self.used]).expect("ascii Debug output")
    }
}

impl<const N: usize> Write for Buf<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        if self.used + bytes.len() > N {
            return Err(core::fmt::Error);
        }
        self.data[self.used..self.used + bytes.len()].copy_from_slice(bytes);
        self.used += bytes.len();
        Ok(())
    }
}

fn dbg<T: core::fmt::Debug>(v: &T) -> bool {
    let mut buf: Buf<128> = Buf::new();
    write!(&mut buf, "{:?}", v).is_ok() && !buf.as_str().is_empty()
}

#[test]
fn usize_debug_works() {
    let u = USize(42);
    assert!(dbg(&u));
}

#[test]
fn bool_debug_works() {
    let b = Bool(true);
    assert!(dbg(&b));
}

#[test]
fn cap_debug_works() {
    let c = Cap(USize(7));
    assert!(dbg(&c));
}

#[test]
fn bits_debug_works() {
    let b: Bits<5, Warm> = <Bits<5, Warm> as Identity<Additive>>::IDENTITY;
    assert!(dbg(&b));
}

#[test]
fn uint_debug_works() {
    let u: Uint<8> = <Uint<8> as Identity<Additive>>::IDENTITY;
    assert!(dbg(&u));
}

#[test]
fn int_debug_works() {
    let i: Int<8> = <Int<8> as Identity<Additive>>::IDENTITY;
    assert!(dbg(&i));
}

#[test]
fn fastfloat_debug_works() {
    let f: FastFloat<f32> = FastFloat(1.5_f32);
    assert!(dbg(&f));
}
