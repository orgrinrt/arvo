//! Probes for the stack audit, exported under plain names so the emitted
//! assembly can be read, and reused by the timing binary beside this file.
//!
//! An ad-hoc spike. Nothing here is a bench and no number it leads to is a
//! measurement in the harness's sense.
//!
//! Three questions. What a value costs to bring in from a host integer known
//! only at run time, since the public doors are `constant`, `adapted` and
//! `slot`. What a fold of values costs beside the same fold over the host
//! integer. And what each rounding mode costs on a fixed-point product written
//! by hand, which is the yardstick the value's product will be read against
//! once it exists.

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Exact, Slot};
use arvo_numeral::{Int, Quiet, Uint};

pub type U13S = Uint<13, Adapt<TowardZero, Saturate>, Quiet>;
pub type U13W = Uint<13, Adapt<TowardZero, Wrap>, Quiet>;
pub type I13S = Int<13, Adapt<TowardZero, Saturate>, Quiet>;
pub type U16W = Uint<16, Adapt<TowardZero, Wrap>, Quiet>;

// The doors, from and to a host integer known at run time.

#[unsafe(no_mangle)]
pub fn ingest_u13_saturating(x: u16) -> U13S {
    U13S::adapted(Exact::on_grid(Slot::at(x as i128)))
}

#[unsafe(no_mangle)]
pub fn ingest_u13_wrapping(x: u16) -> U13W {
    U13W::adapted(Exact::on_grid(Slot::at(x as i128)))
}

#[unsafe(no_mangle)]
pub fn ingest_i13_saturating(x: i16) -> I13S {
    I13S::adapted(Exact::on_grid(Slot::at(x as i128)))
}

#[unsafe(no_mangle)]
pub fn egress_u13(v: U13S) -> u16 {
    v.slot().index() as u16
}

#[unsafe(no_mangle)]
pub fn host_ingest_u13_saturating(x: u16) -> u16 {
    if x > 8191 { 8191 } else { x }
}

#[unsafe(no_mangle)]
pub fn host_ingest_u13_wrapping(x: u16) -> u16 {
    x & 8191
}

// The fold, through the value and through the host integer.

#[unsafe(no_mangle)]
pub fn fold_u13_saturating(xs: &[U13S]) -> U13S {
    let mut acc = U13S::constant::<0>();
    for &x in xs {
        acc = acc.add(x);
    }
    acc
}

#[unsafe(no_mangle)]
pub fn fold_u13_wrapping(xs: &[U13W]) -> U13W {
    let mut acc = U13W::constant::<0>();
    for &x in xs {
        acc = acc.add(x);
    }
    acc
}

#[unsafe(no_mangle)]
pub fn fold_u16_wrapping(xs: &[U16W]) -> U16W {
    let mut acc = U16W::constant::<0>();
    for &x in xs {
        acc = acc.add(x);
    }
    acc
}

#[unsafe(no_mangle)]
pub fn host_fold_u13_saturating(xs: &[u16]) -> u16 {
    let mut acc = 0u16;
    for &x in xs {
        let s = acc + x;
        acc = if s > 8191 { 8191 } else { s };
    }
    acc
}

#[unsafe(no_mangle)]
pub fn host_fold_u13_wrapping(xs: &[u16]) -> u16 {
    let mut acc = 0u16;
    for &x in xs {
        acc = acc.wrapping_add(x) & 8191;
    }
    acc
}

#[unsafe(no_mangle)]
pub fn host_fold_u16_wrapping(xs: &[u16]) -> u16 {
    let mut acc = 0u16;
    for &x in xs {
        acc = acc.wrapping_add(x);
    }
    acc
}

// The elementwise sum of two columns into a third.

#[unsafe(no_mangle)]
pub fn map_u13_saturating(a: &[U13S], b: &[U13S], out: &mut [U13S]) {
    for ((o, &x), &y) in out.iter_mut().zip(a).zip(b) {
        *o = x.add(y);
    }
}

#[unsafe(no_mangle)]
pub fn host_map_u13_saturating(a: &[u16], b: &[u16], out: &mut [u16]) {
    for ((o, &x), &y) in out.iter_mut().zip(a).zip(b) {
        let s = x + y;
        *o = if s > 8191 { 8191 } else { s };
    }
}

// A Q16.16 product in a 32-bit signed carrier, by hand, one per rounding mode.
// Each forms the exact 64-bit product and puts it back on the grid.

const FRAC: u32 = 16;
const HALF: i64 = 1 << (FRAC - 1);
const MASK: i64 = (1 << FRAC) - 1;

#[inline(always)]
fn wrap(v: i64) -> i32 {
    v as i32
}

#[inline(always)]
fn saturate(v: i64) -> i32 {
    if v > i32::MAX as i64 {
        i32::MAX
    } else if v < i32::MIN as i64 {
        i32::MIN
    } else {
        v as i32
    }
}

#[inline(always)]
pub fn q_floor(a: i32, b: i32) -> i64 {
    (a as i64 * b as i64) >> FRAC
}

#[inline(always)]
pub fn q_toward_zero(a: i32, b: i32) -> i64 {
    let p = a as i64 * b as i64;
    // Floor, then one up where the product is negative and off the grid.
    (p >> FRAC) + ((p < 0) & (p & MASK != 0)) as i64
}

#[inline(always)]
pub fn q_half_up(a: i32, b: i32) -> i64 {
    (a as i64 * b as i64 + HALF) >> FRAC
}

#[inline(always)]
pub fn q_half_even(a: i32, b: i32) -> i64 {
    let p = a as i64 * b as i64;
    let up = (p + HALF) >> FRAC;
    // A tie whose half-up answer is odd steps down to the even neighbour.
    up - ((p & MASK == HALF) & (up & 1 != 0)) as i64
}

macro_rules! column {
    ($name:ident, $q:ident, $fit:ident) => {
        #[unsafe(no_mangle)]
        pub fn $name(a: &[i32], b: &[i32], out: &mut [i32]) {
            for ((o, &x), &y) in out.iter_mut().zip(a).zip(b) {
                *o = $fit($q(x, y));
            }
        }
    };
}

column!(qmul_floor_wrap, q_floor, wrap);
column!(qmul_toward_zero_wrap, q_toward_zero, wrap);
column!(qmul_half_up_wrap, q_half_up, wrap);
column!(qmul_half_even_wrap, q_half_even, wrap);
column!(qmul_floor_saturate, q_floor, saturate);
column!(qmul_half_even_saturate, q_half_even, saturate);

#[unsafe(no_mangle)]
pub fn fmul_f32(a: &[f32], b: &[f32], out: &mut [f32]) {
    for ((o, &x), &y) in out.iter_mut().zip(a).zip(b) {
        *o = x * y;
    }
}

/// A decay applied `steps` times to one value, which is a loop-carried chain
/// and so reads the latency of each mode and not its throughput.
macro_rules! chain {
    ($name:ident, $q:ident, $fit:ident) => {
        #[unsafe(no_mangle)]
        pub fn $name(start: i32, factor: i32, steps: u32) -> i32 {
            let mut v = start;
            let mut i = 0;
            while i < steps {
                v = $fit($q(v, factor));
                i += 1;
            }
            v
        }
    };
}

chain!(decay_floor, q_floor, wrap);
chain!(decay_toward_zero, q_toward_zero, wrap);
chain!(decay_half_up, q_half_up, wrap);
chain!(decay_half_even, q_half_even, wrap);
chain!(decay_half_even_saturate, q_half_even, saturate);
