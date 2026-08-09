//! PROBE 5: file 17's dissolution was measured on `f64` reductions only. arvo's
//! representations are mostly not `f64`. This checks the two families it did
//! not touch: the wide multi-limb integers, and the saturating integers that
//! `Precise` selects.
//!
//! The question in each case is the same one: is there a liberty here whose
//! effect a source-level body can reach, and if not, is `core::arch` the door.
//!
//! Build: see 05_run.sh

#![crate_type = "lib"]
#![no_std]

// ---------------------------------------------------------- multi-limb width
// A 256-bit numeral, four limbs, which is the shape any `UFixed<I, F>` with
// I + F > 128 has to have.

#[no_mangle]
#[inline(never)]
pub fn add_u256(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let mut out = [0u64; 4];
    let mut carry = false;
    let mut i = 0;
    while i < 4 {
        let (s, c) = a[i].carrying_add(b[i], carry);
        out[i] = s;
        carry = c;
        i += 1;
    }
    out
}

/// A reduction over 256-bit numerals: the multi-limb analogue of the float
/// reduction file 17 measured.
#[no_mangle]
#[inline(never)]
pub fn reduce_u256(xs: &[[u64; 4]]) -> [u64; 4] {
    let mut acc = [0u64; 4];
    let mut i = 0;
    while i < xs.len() {
        acc = add_u256(acc, xs[i]);
        i += 1;
    }
    acc
}

// ------------------------------------------------------ saturating: the one
// `Precise` selects, and the one file 13 measured as non-associative.

#[no_mangle]
#[inline(never)]
pub fn reduce_saturating(xs: &[u64]) -> u64 {
    let mut a = 0u64;
    let mut i = 0;
    while i < xs.len() {
        a = a.saturating_add(xs[i]);
        i += 1;
    }
    a
}

/// The source-level regrouping that IS legal for wrapping and is NOT legal for
/// saturating. Present so the emitted code can be compared, not because the
/// design may select it.
#[no_mangle]
#[inline(never)]
pub fn reduce_saturating_regrouped(xs: &[u64]) -> u64 {
    let (mut a, mut b) = (0u64, 0u64);
    let mut i = 0;
    while i + 2 <= xs.len() {
        a = a.saturating_add(xs[i]);
        b = b.saturating_add(xs[i + 1]);
        i += 2;
    }
    let mut t = a.saturating_add(b);
    while i < xs.len() {
        t = t.saturating_add(xs[i]);
        i += 1;
    }
    t
}

#[no_mangle]
#[inline(never)]
pub fn reduce_wrapping(xs: &[u64]) -> u64 {
    let mut a = 0u64;
    let mut i = 0;
    while i < xs.len() {
        a = a.wrapping_add(xs[i]);
        i += 1;
    }
    a
}
