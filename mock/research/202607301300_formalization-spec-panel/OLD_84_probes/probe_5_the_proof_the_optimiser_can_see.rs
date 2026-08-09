//! probe 5: a proof the type system holds and the optimiser cannot see is not free.
//!
//! Probe 4 found that the declaration home (`Nz`, a `repr(transparent)` newtype with a
//! private field and a fallible door) removes the fallibility from the API and from
//! the layout, and does **not** remove it from the emitted code: `total_loop` still
//! carries a `cbz` and a `panic_const_div_by_zero` landing pad, because a
//! `repr(transparent)` newtype over `i64` carries no validity range and LLVM has no
//! way to know the divisor is nonzero.
//!
//! This probe asks what it would take for the proof to reach the optimiser, and
//! measures three shapes at `-O`:
//!
//!   A. `Nz`, probe 4's newtype: the proof is in the type system only.
//!   B. `core::num::NonZeroI64`: the proof is in the type system **and** in the
//!      layout, through the validity range `core` declares on it.
//!   C. a bare `i64` with the check written out: the honest baseline.
//!
//! And two layout questions, which decide whether the refusing carrier costs anything
//! at all when the numeral has a spare pattern:
//!
//!   `size_of::<Option<i64>>()` against `size_of::<Option<NonZeroI64>>()`.
//!
//! Nothing here is a timing claim. The artifacts are the emitted code and the sizes.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use core::num::NonZeroI64;

const SCALE: i64 = 1_000_000;

// ---------------------------------------------------------------------------
// A. the proof is in the type system only.
// ---------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Nz(i64);

impl Nz {
    #[inline]
    pub const fn new(v: i64) -> Option<Nz> {
        if v == 0 {
            None
        } else {
            Some(Nz(v))
        }
    }
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn newtype_loop(src: &[Nz; 64], dst: &mut [i64; 64]) {
    let mut i = 0;
    while i < 64 {
        dst[i] = SCALE / src[i].get();
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// B. the proof is in the layout too.
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn niche_loop(src: &[NonZeroI64; 64], dst: &mut [i64; 64]) {
    let mut i = 0;
    while i < 64 {
        dst[i] = SCALE / src[i].get();
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// C. the honest baseline: no proof, the check is written out.
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn checked_loop(src: &[i64; 64], dst: &mut [i64; 64]) {
    let mut i = 0;
    while i < 64 {
        let d = src[i];
        dst[i] = if d == 0 { 0 } else { SCALE / d };
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// The layout question: does a niche make the refusing carrier free?
// ---------------------------------------------------------------------------

pub const SIZE_OPT_PLAIN: usize = core::mem::size_of::<Option<i64>>();
pub const SIZE_OPT_NICHE: usize = core::mem::size_of::<Option<NonZeroI64>>();
pub const SIZE_OPT_NEWTYPE: usize = core::mem::size_of::<Option<Nz>>();
pub const SIZE_NICHE: usize = core::mem::size_of::<NonZeroI64>();

const _: () = assert!(SIZE_NICHE == 8);
const _: () = assert!(
    SIZE_OPT_NICHE == 8,
    "a declared validity range gives the refusing carrier back for free"
);
const _: () = assert!(
    SIZE_OPT_PLAIN == 16,
    "without a niche the refusing carrier doubles the column"
);
const _: () = assert!(
    SIZE_OPT_NEWTYPE == 16,
    "a repr(transparent) newtype over a full-range primitive has no niche to give"
);
