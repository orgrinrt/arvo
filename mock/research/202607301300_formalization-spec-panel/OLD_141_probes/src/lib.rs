//! One exported symbol per (arm, width, density) case, so the emitted code
//! for each bench arm can be read on its own rather than located inside the
//! 28-key dispatch of a variant dylib.
//!
//! `eager_*` is the projection written after every operation (the `headroom`,
//! `minimum` and `plusone` arms). `lazy_*` is the projection written once
//! (the `native` arm). The carrier named in each symbol is the container the
//! corresponding arm's table selects.

use bench_warm_container_shared::{run_sat, run_wrap};

macro_rules! case {
    ($eager_name:ident, $lazy_name:ident, $c:ty, $w:literal, $d:literal) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $eager_name(p: *const $c, n: usize, k: $c) -> u64 {
            let s = unsafe { core::slice::from_raw_parts(p, n) };
            run_wrap::<$c, $w, $d, true>(s, k)
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn $lazy_name(p: *const $c, n: usize, k: $c) -> u64 {
            let s = unsafe { core::slice::from_raw_parts(p, n) };
            run_wrap::<$c, $w, $d, false>(s, k)
        }
    };
}

// W = 13, sub-rung. minimum/plusone carrier u16, headroom carrier u32.
case!(eager_w13_min_d1, lazy_w13_min_d1, u16, 13, 1);
case!(eager_w13_head_d1, lazy_w13_head_d1, u32, 13, 1);
case!(eager_w13_min_d8, lazy_w13_min_d8, u16, 13, 8);
case!(eager_w13_head_d8, lazy_w13_head_d8, u32, 13, 8);

// W = 64, exactly filled. minimum carrier u64, headroom carrier u128.
case!(eager_w64_min_d1, lazy_w64_min_d1, u64, 64, 1);
case!(eager_w64_head_d1, lazy_w64_head_d1, u128, 64, 1);
case!(eager_w64_min_d8, lazy_w64_min_d8, u64, 64, 8);
case!(eager_w64_head_d8, lazy_w64_head_d8, u128, 64, 8);

// W = 32, exactly filled, the middle rung.
case!(eager_w32_min_d3, lazy_w32_min_d3, u32, 32, 3);
case!(eager_w32_head_d3, lazy_w32_head_d3, u64, 32, 3);

// Saturating. No lazy form exists, so one symbol each.
macro_rules! sat_case {
    ($name:ident, $c:ty, $w:literal, $d:literal) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name(p: *const $c, n: usize, k: $c) -> u64 {
            let s = unsafe { core::slice::from_raw_parts(p, n) };
            run_sat::<$c, $w, $d>(s, k)
        }
    };
}

sat_case!(sat_w13_min_d3, u16, 13, 3);
sat_case!(sat_w13_head_d3, u32, 13, 3);
sat_case!(sat_w64_min_d3, u64, 64, 3);
sat_case!(sat_w64_head_d3, u128, 64, 3);

// ---------------------------------------------------------------------------
// The elementwise regime, which the benches do not cover.
//
// Every bench row in file 141 measures a reduction, where the accumulator's
// own projection sits on the critical path. An elementwise transform with no
// cross-iteration dependence is a different regime and its magnitude is
// unpriced. These symbols exist so the qualitative half can be read off the
// emitted code rather than guessed: whether the eager projection blocks
// vectorisation when nothing carries between iterations.
//
// Written out here rather than reusing `run_wrap` because `run_wrap` ends in
// a reduction by construction; this is the same per-element step sequence
// with the accumulation removed and a store in its place.
// ---------------------------------------------------------------------------

macro_rules! elementwise {
    ($name:ident, $c:ty, $w:literal, $d:literal, $eager:literal) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name(src: *const $c, dst: *mut $c, n: usize, k: $c, x2: $c) -> () {
            let s = unsafe { core::slice::from_raw_parts(src, n) };
            let d = unsafe { core::slice::from_raw_parts_mut(dst, n) };
            let m: $c = if $w >= <$c>::BITS {
                <$c>::MAX
            } else {
                (1 as $c).wrapping_shl($w).wrapping_sub(1)
            };
            for i in 0..n {
                let mut v = s[i];
                let mut j = 0usize;
                while j < $d {
                    v = match j % 4 {
                        0 => v.wrapping_add(k),
                        1 => v.wrapping_mul(3),
                        2 => v.wrapping_sub(k),
                        _ => v ^ x2,
                    };
                    if $eager {
                        v &= m;
                    }
                    j += 1;
                }
                d[i] = v & m;
            }
        }
    };
}

elementwise!(ew_eager_w13_min_d4, u16, 13, 4, true);
elementwise!(ew_lazy_w13_min_d4, u16, 13, 4, false);
elementwise!(ew_eager_w13_head_d4, u32, 13, 4, true);
elementwise!(ew_eager_w64_min_d4, u64, 64, 4, true);
elementwise!(ew_eager_w64_head_d4, u128, 64, 4, true);

// ---------------------------------------------------------------------------
// Why 33..=64 falls off the vector path, isolated one operation at a time.
//
// The elementwise sweep shows W=60 at 3466 ns against W=13 at 267 ns with the
// same element count and the same operation count, and the emitted code for
// the 64-bit cases contains no vector register at all. These symbols separate
// the step kinds so the responsible one can be named rather than guessed.
// ---------------------------------------------------------------------------

macro_rules! ew_ops {
    ($name:ident, $c:ty, $w:literal, $body:expr) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name(src: *const $c, dst: *mut $c, n: usize, k: $c) {
            let s = unsafe { core::slice::from_raw_parts(src, n) };
            let d = unsafe { core::slice::from_raw_parts_mut(dst, n) };
            let f: fn($c, $c) -> $c = $body;
            for i in 0..n {
                d[i] = f(s[i], k);
            }
        }
    };
}

// add and exclusive or only: no multiply anywhere.
ew_ops!(op_addxor_u64, u64, 64, |v, k| v.wrapping_add(k) ^ k);
ew_ops!(op_addxor_u16, u16, 16, |v, k| v.wrapping_add(k) ^ k);
// multiply by a constant three, which is strength-reducible to shift plus add.
ew_ops!(op_mul3_u64, u64, 64, |v, _k| v.wrapping_mul(3));
ew_ops!(op_mul3_u16, u16, 16, |v, _k| v.wrapping_mul(3));
// multiply by a runtime value, which is not.
ew_ops!(op_mulk_u64, u64, 64, |v, k| v.wrapping_mul(k));
ew_ops!(op_mulk_u16, u16, 16, |v, k| v.wrapping_mul(k));
// the 32-by-32 decomposition of a 64-bit low-half multiply by a runtime
// value, which is what a microkernel for this cell would write by hand.
ew_ops!(op_mulk_u64_split, u64, 64, |v, k| {
    let a0 = v & 0xFFFF_FFFF;
    let a1 = v >> 32;
    let b0 = k & 0xFFFF_FFFF;
    let b1 = k >> 32;
    let lo = a0.wrapping_mul(b0);
    let mid = a0.wrapping_mul(b1).wrapping_add(a1.wrapping_mul(b0));
    lo.wrapping_add(mid << 32)
});

// ---------------------------------------------------------------------------
// The microkernel attempt for the one cell nothing above recovers.
//
// `op_mulk_u64_split` shows that writing the 32-by-32 decomposition in plain
// Rust does not get the loop back onto the vector path: LLVM keeps the pieces
// in 64-bit registers and never narrows them to `.2s` inputs for `umull`.
// This is the same decomposition written with the NEON intrinsics directly,
// which is the form a hand-written kernel for this cell would take.
// ---------------------------------------------------------------------------

#[cfg(target_arch = "aarch64")]
#[unsafe(no_mangle)]
pub extern "C" fn op_mulk_u64_neon(src: *const u64, dst: *mut u64, n: usize, k: u64) {
    use core::arch::aarch64::*;
    let s = unsafe { core::slice::from_raw_parts(src, n) };
    let d = unsafe { core::slice::from_raw_parts_mut(dst, n) };
    let pairs = n / 2;
    unsafe {
        let b = vdupq_n_u64(k);
        let b_lo = vmovn_u64(b);
        let b_hi = vshrn_n_u64::<32>(b);
        for i in 0..pairs {
            let a = vld1q_u64(s.as_ptr().add(i * 2));
            let a_lo = vmovn_u64(a);
            let a_hi = vshrn_n_u64::<32>(a);
            let lo = vmull_u32(a_lo, b_lo);
            let m1 = vmull_u32(a_lo, b_hi);
            let m2 = vmull_u32(a_hi, b_lo);
            let mid = vaddq_u64(m1, m2);
            let res = vaddq_u64(lo, vshlq_n_u64::<32>(mid));
            vst1q_u64(d.as_mut_ptr().add(i * 2), res);
        }
    }
    for i in pairs * 2..n {
        d[i] = s[i].wrapping_mul(k);
    }
}
