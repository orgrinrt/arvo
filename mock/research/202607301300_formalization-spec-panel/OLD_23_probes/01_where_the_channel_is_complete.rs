//! PROBE 1: the symbol channel is incomplete in the artifact that ships, and
//! complete in an artifact that costs one flag.
//!
//! File 20 measured that a v0 symbol carries every const argument and marker
//! type of a monomorphised composition, and that `#[inline]` closes the channel
//! entirely. It drew from that the constraint "the intent is legible exactly at
//! the granularity where the operation survives as a function".
//!
//! That constraint is about ONE artifact: the optimised object file. This probe
//! asks what happens with no inline attribute at all (the case arvo will
//! actually be in, since neither `#[inline]` nor `#[inline(never)]` is the
//! default for a generic function), and then asks the same question of an
//! artifact rustc will emit for free: the pre-optimisation IR.
//!
//! Twelve compositions are instantiated. Count how many are nameable in each.
//!
//! Nothing here has an inline attribute anywhere. That is the point.

#![crate_type = "lib"]
#![allow(dead_code)]

pub struct Strict;
pub struct Relaxed;
pub struct Dense;
pub struct Bitpacked;

pub struct Number<const I: u16, const F: u16, P, L>(u64, [P; 0], [L; 0]);

/// A small operation. Nothing stops the inliner from swallowing this.
pub fn add<const I: u16, const F: u16, P, L>(a: u64, b: u64) -> u64 {
    a.wrapping_add(b) ^ (I as u64) ^ ((F as u64) << 8)
}

/// A larger operation, to check whether size alone keeps a composition visible.
pub fn reduce<const I: u16, const F: u16, P, L>(xs: &[u64]) -> u64 {
    let mut acc = 0u64;
    let mut i = 0;
    while i + 4 <= xs.len() {
        acc = acc
            .wrapping_add(xs[i])
            .wrapping_mul(3)
            .wrapping_add(xs[i + 1])
            .wrapping_mul(5)
            .wrapping_add(xs[i + 2])
            .wrapping_mul(7)
            .wrapping_add(xs[i + 3]);
        i += 4;
    }
    while i < xs.len() {
        acc = acc.wrapping_add(xs[i]).rotate_left((I % 63) as u32 + 1);
        i += 1;
    }
    acc ^ ((F as u64) << 16)
}

macro_rules! call_both {
    ($ca:ident, $cr:ident, $i:expr, $f:expr, $p:ty, $l:ty) => {
        #[no_mangle]
        pub extern "C" fn $ca(a: u64, b: u64) -> u64 {
            add::<$i, $f, $p, $l>(a, b)
        }
        #[no_mangle]
        pub extern "C" fn $cr(xs: &[u64]) -> u64 {
            reduce::<$i, $f, $p, $l>(xs)
        }
    };
}

call_both!(c_a0, c_r0, 3, 5, Strict, Dense);
call_both!(c_a1, c_r1, 3, 5, Strict, Bitpacked);
call_both!(c_a2, c_r2, 3, 5, Relaxed, Dense);
call_both!(c_a3, c_r3, 3, 5, Relaxed, Bitpacked);
call_both!(c_a4, c_r4, 7, 9, Strict, Dense);
call_both!(c_a5, c_r5, 7, 9, Strict, Bitpacked);
call_both!(c_a6, c_r6, 7, 9, Relaxed, Dense);
call_both!(c_a7, c_r7, 7, 9, Relaxed, Bitpacked);
call_both!(c_a8, c_r8, 23, 41, Strict, Dense);
call_both!(c_a9, c_r9, 23, 41, Strict, Bitpacked);
call_both!(c_a10, c_r10, 23, 41, Relaxed, Dense);
call_both!(c_a11, c_r11, 23, 41, Relaxed, Bitpacked);
