//! One slice kernel per arm, plus the two attacks on the packed decode.
//!
//! Each kernel is a thin wrapper over a sum the carrier crate already exports,
//! so the inner loop is exactly the code the single-core sweep measured. They
//! share one function-pointer shape so the pool can carry any of them and the
//! tests can drive all of them through one table.

use bench_bitpack_carrier_shared::{sum_d16, sum_d32, sum_d64, Plan13, LOGICAL_BITS, MASK13};

use crate::input::Layout;

// ── the kernels ────────────────────────────────────────────────────────────
//
// One per arm, each a thin wrapper over the carrier crate's own inlined sum, so
// the inner loop is exactly the code the single-core sweep measured. They live
// here rather than in each variant crate so the pool's function-pointer type is
// satisfied by identically-shaped functions and the tests can drive all five
// through one table.

/// # Safety
/// See [`SliceKernel`].
pub unsafe fn kern_d16(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    sum_d16(&col.d16[lo..hi], hi - lo)
}

/// # Safety
/// See [`SliceKernel`].
pub unsafe fn kern_d32(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    sum_d32(&col.d32[lo..hi], hi - lo)
}

/// # Safety
/// See [`SliceKernel`].
pub unsafe fn kern_d64(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    sum_d64(&col.d64[lo..hi], hi - lo)
}

/// # Safety
/// See [`SliceKernel`]. `lo` must be a multiple of the packed period so the
/// slice starts on a byte boundary.
pub unsafe fn kern_packed(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    let byte_lo = (lo * LOGICAL_BITS) / 8;
    unsafe { bench_bitpack_plan_shared::sum_windowed::<Plan13>(&col.packed[byte_lo..], hi - lo) }
}

/// # Safety
/// See [`SliceKernel`]. `lo` must be a multiple of the packed period so the
/// slice starts on a byte boundary.
#[cfg(target_arch = "aarch64")]
pub unsafe fn kern_packed_simd(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    let byte_lo = (lo * LOGICAL_BITS) / 8;
    unsafe {
        bench_bitpack_carrier_shared::sum_simd_padal::<Plan13>(&col.packed[byte_lo..], hi - lo)
    }
}

// ── attacking the decode ────────────────────────────────────────────────────
//
// Under contention the dense arms stop scaling and the packed arm does not, so
// the packed arm's cost becomes its decode and nothing else. Every picosecond
// taken off that decode converts directly into margin against the tightest
// dense carrier, which is the one packing has never beaten. So the decode is
// the mechanism to attack, and these are the attacks.
//
// `sum_simd_padal` in the carrier crate already replaced a six-instruction
// widening chain with one `UADALP`, worth 5 to 7 per cent, and that file
// recorded why it recovered only a third of the instructions it removed: the
// decode chain (`ld`, `tbl`, `tbl`, `ushl`, `and`, `movn`) is a serial
// dependency through the vector unit, and the reduction had been hiding in its
// shadow. Removing the reduction exposed the decode's latency rather than its
// throughput.
//
// The fix for an exposed latency chain is more independent chains in flight.
// The accumulate is loop-carried on one register, so consecutive groups cannot
// overlap even though nothing about them is actually dependent. Splitting the
// accumulator lets the machine work on several groups at once.
//
// Two unroll factors are shipped rather than one, because which wins is a
// question about register pressure and issue width on this specific core and is
// not answerable from the source. Guessing one and reporting it would be a
// bench with no competitor.

/// The pairwise-accumulate decode with two independent accumulators.
///
/// # Safety
/// Identical to `sum_simd_padal`: `buf` holds `n * W` bits plus sixteen bytes of
/// read headroom and `n` is a multiple of `K::P`.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn sum_padal_pipe2<K: bench_bitpack_plan_shared::Packing>(buf: &[u8], n: usize) -> u64 {
    use bench_bitpack_plan_shared::neon::decode_group;
    use core::arch::aarch64::*;

    let drain: usize = (u32::MAX as u64 / (2 * K::MASK)) as usize;
    let groups = n / K::P;
    let mut total: u64 = 0;
    let mut base = 0usize;
    let mut done = 0usize;
    unsafe {
        while done < groups {
            let chunk = core::cmp::min(groups - done, drain);
            let (mut a0, mut a1) = (vdupq_n_u32(0), vdupq_n_u32(0));
            let pairs = chunk / 2;
            for _ in 0..pairs {
                let l0 = decode_group::<K>(buf, base);
                let l1 = decode_group::<K>(buf, base + K::G);
                a0 = vpadalq_u16(a0, l0);
                a1 = vpadalq_u16(a1, l1);
                base += 2 * K::G;
            }
            for _ in 0..(chunk - pairs * 2) {
                a0 = vpadalq_u16(a0, decode_group::<K>(buf, base));
                base += K::G;
            }
            // widen before summing: two accumulators each hold up to
            // `2 * MASK * chunk / 2`, so adding them as u32 could overflow
            let w = vaddq_u64(vpaddlq_u32(a0), vpaddlq_u32(a1));
            total = total
                .wrapping_add(vgetq_lane_u64(w, 0))
                .wrapping_add(vgetq_lane_u64(w, 1));
            done += chunk;
        }
    }
    total
}

/// The pairwise-accumulate decode with four independent accumulators.
///
/// # Safety
/// Identical to `sum_simd_padal`.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn sum_padal_pipe4<K: bench_bitpack_plan_shared::Packing>(buf: &[u8], n: usize) -> u64 {
    use bench_bitpack_plan_shared::neon::decode_group;
    use core::arch::aarch64::*;

    let drain: usize = (u32::MAX as u64 / (2 * K::MASK)) as usize;
    let groups = n / K::P;
    let mut total: u64 = 0;
    let mut base = 0usize;
    let mut done = 0usize;
    unsafe {
        while done < groups {
            let chunk = core::cmp::min(groups - done, drain);
            let mut a = [vdupq_n_u32(0); 4];
            let quads = chunk / 4;
            for _ in 0..quads {
                let l0 = decode_group::<K>(buf, base);
                let l1 = decode_group::<K>(buf, base + K::G);
                let l2 = decode_group::<K>(buf, base + 2 * K::G);
                let l3 = decode_group::<K>(buf, base + 3 * K::G);
                a[0] = vpadalq_u16(a[0], l0);
                a[1] = vpadalq_u16(a[1], l1);
                a[2] = vpadalq_u16(a[2], l2);
                a[3] = vpadalq_u16(a[3], l3);
                base += 4 * K::G;
            }
            for _ in 0..(chunk - quads * 4) {
                a[0] = vpadalq_u16(a[0], decode_group::<K>(buf, base));
                base += K::G;
            }
            let w = vaddq_u64(
                vaddq_u64(vpaddlq_u32(a[0]), vpaddlq_u32(a[1])),
                vaddq_u64(vpaddlq_u32(a[2]), vpaddlq_u32(a[3])),
            );
            total = total
                .wrapping_add(vgetq_lane_u64(w, 0))
                .wrapping_add(vgetq_lane_u64(w, 1));
            done += chunk;
        }
    }
    total
}

/// # Safety
/// See [`SliceKernel`]. `lo` must be a multiple of the packed period.
#[cfg(target_arch = "aarch64")]
pub unsafe fn kern_packed_pipe2(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    let byte_lo = (lo * LOGICAL_BITS) / 8;
    unsafe { sum_padal_pipe2::<Plan13>(&col.packed[byte_lo..], hi - lo) }
}

/// # Safety
/// See [`SliceKernel`]. `lo` must be a multiple of the packed period.
#[cfg(target_arch = "aarch64")]
pub unsafe fn kern_packed_pipe4(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    let byte_lo = (lo * LOGICAL_BITS) / 8;
    unsafe { sum_padal_pipe4::<Plan13>(&col.packed[byte_lo..], hi - lo) }
}

#[cfg(test)]
mod pipe_tests {
    use bench_bitpack_carrier_shared::OFF_D16;

    use super::*;
    use crate::input::build_bytes;

    /// Both unrolled kernels agree with the scalar decode and with the ground
    /// truth, at sizes on both sides of a drain boundary and at sizes that are
    /// not multiples of the unroll factor, which is where a mishandled tail
    /// would hide.
    #[cfg(target_arch = "aarch64")]
    fn agrees_at(n: usize) {
        let buf = build_bytes(n, 21);
        let bytes = (n * LOGICAL_BITS) / 8 + 16;
        let region = &buf[bench_bitpack_carrier_shared::OFF_PACKED
            ..bench_bitpack_carrier_shared::OFF_PACKED + bytes];
        let mut truth = 0u64;
        for i in 0..n {
            truth = truth
                .wrapping_add(
                    u16::from_le_bytes([buf[OFF_D16 + i * 2], buf[OFF_D16 + i * 2 + 1]]) as u64,
                );
        }
        let p2 = unsafe { sum_padal_pipe2::<Plan13>(region, n) };
        let p4 = unsafe { sum_padal_pipe4::<Plan13>(region, n) };
        assert_eq!(p2, truth, "sum_padal_pipe2 disagrees at n={n}");
        assert_eq!(p4, truth, "sum_padal_pipe4 disagrees at n={n}");
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn pipelined_kernels_agree_below_one_drain() {
        agrees_at(16384);
        agrees_at(131072);
        // 8 groups short of a multiple of four groups, so both tails run
        agrees_at(131072 - 8 * 3);
    }

    /// Past the drain boundary, which at width 13 is 262,160 groups, or
    /// 2,097,280 elements. The drain fold is the part most likely to be wrong
    /// and it is invisible below the period, so a test that only ran small
    /// sizes would prove nothing about it.
    #[cfg(target_arch = "aarch64")]
    #[test]
    fn pipelined_kernels_agree_across_a_drain_boundary() {
        agrees_at(4194304);
        agrees_at(4194304 - 8);
    }
}

// ── attacking the dense side too ────────────────────────────────────────────
//
// The decode attack above is worth about 40 per cent, which raises a question
// the bench has to answer before any of it can be believed: is the dense arm
// it is being compared against the best dense arm, or is it the one that
// happened to be committed?
//
// It is not the best one. `sum_d16`'s disassembly is a `ldp q, q` pair, a
// `bic.8h` mask and then four `uaddw` widening adds per sixteen elements, about
// one instruction per element, and it runs at 3.5 elements per cycle on an
// eight-wide core. The widening is the same cost `sum_simd_padal` removed on the
// packed side and it is removable the same way: `UADALP` folds eight 16-bit
// lanes into four 32-bit lanes in one instruction, which drops the loop to a
// load, a mask and an accumulate for every eight elements.
//
// Comparing an attacked packed kernel against an unattacked dense one is the
// strawman failure, so both sides get the same attack and the comparison is
// between two kernels written with the same care.

/// The `u16` dense read with pairwise accumulation and four accumulators.
///
/// Identical semantics to `sum_d16`: every element is masked to the logical
/// width and wrapping-added, so the two agree bit for bit and the harness's
/// cross-variant comparison holds them to it.
///
/// # Safety
/// `vals` holds at least `n` elements.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn sum_d16_padal(vals: &[u16], n: usize) -> u64 {
    unsafe {
        use core::arch::aarch64::*;

        let mask = vdupq_n_u16(MASK13 as u16);
        // a 32-bit lane takes two 16-bit lanes per group of eight, so it holds at
        // most `2 * MASK * groups` and must drain on the same period the packed
        // kernel uses
        let drain: usize = (u32::MAX as u64 / (2 * MASK13)) as usize;
        let groups = n / 8;
        let mut total: u64 = 0;
        let mut done = 0usize;
        let p = vals.as_ptr();
        unsafe {
            while done < groups {
                let chunk = core::cmp::min(groups - done, drain);
                let mut a = [vdupq_n_u32(0); 4];
                let quads = chunk / 4;
                let mut g = done;
                for _ in 0..quads {
                    let v0 = vandq_u16(vld1q_u16(p.add(g * 8)), mask);
                    let v1 = vandq_u16(vld1q_u16(p.add(g * 8 + 8)), mask);
                    let v2 = vandq_u16(vld1q_u16(p.add(g * 8 + 16)), mask);
                    let v3 = vandq_u16(vld1q_u16(p.add(g * 8 + 24)), mask);
                    a[0] = vpadalq_u16(a[0], v0);
                    a[1] = vpadalq_u16(a[1], v1);
                    a[2] = vpadalq_u16(a[2], v2);
                    a[3] = vpadalq_u16(a[3], v3);
                    g += 4;
                }
                for _ in 0..(chunk - quads * 4) {
                    a[0] = vpadalq_u16(a[0], vandq_u16(vld1q_u16(p.add(g * 8)), mask));
                    g += 1;
                }
                let w = vaddq_u64(
                    vaddq_u64(vpaddlq_u32(a[0]), vpaddlq_u32(a[1])),
                    vaddq_u64(vpaddlq_u32(a[2]), vpaddlq_u32(a[3])),
                );
                total = total
                    .wrapping_add(vgetq_lane_u64(w, 0))
                    .wrapping_add(vgetq_lane_u64(w, 1));
                done += chunk;
            }
            for i in (groups * 8)..n {
                total = total.wrapping_add((*p.add(i) as u64) & MASK13);
            }
        }
        total
    }
}

/// The `u32` dense read with pairwise accumulation and four accumulators.
///
/// `vpadalq_u32` folds four 32-bit lanes into two 64-bit lanes, which cannot
/// overflow for any column this bench can hold, so unlike the 16-bit case there
/// is no drain.
///
/// # Safety
/// `vals` holds at least `n` elements.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn sum_d32_padal(vals: &[u32], n: usize) -> u64 {
    unsafe {
        use core::arch::aarch64::*;

        let mask = vdupq_n_u32(MASK13 as u32);
        let groups = n / 4;
        let mut a = [vdupq_n_u64(0); 4];
        let quads = groups / 4;
        let p = vals.as_ptr();
        unsafe {
            let mut g = 0usize;
            for _ in 0..quads {
                let v0 = vandq_u32(vld1q_u32(p.add(g * 4)), mask);
                let v1 = vandq_u32(vld1q_u32(p.add(g * 4 + 4)), mask);
                let v2 = vandq_u32(vld1q_u32(p.add(g * 4 + 8)), mask);
                let v3 = vandq_u32(vld1q_u32(p.add(g * 4 + 12)), mask);
                a[0] = vpadalq_u32(a[0], v0);
                a[1] = vpadalq_u32(a[1], v1);
                a[2] = vpadalq_u32(a[2], v2);
                a[3] = vpadalq_u32(a[3], v3);
                g += 4;
            }
            for _ in 0..(groups - quads * 4) {
                a[0] = vpadalq_u32(a[0], vandq_u32(vld1q_u32(p.add(g * 4)), mask));
                g += 1;
            }
            let w = vaddq_u64(vaddq_u64(a[0], a[1]), vaddq_u64(a[2], a[3]));
            let mut total = vgetq_lane_u64(w, 0).wrapping_add(vgetq_lane_u64(w, 1));
            for i in (groups * 4)..n {
                total = total.wrapping_add((*p.add(i) as u64) & MASK13);
            }
            total
        }
    }
}

/// # Safety
/// See [`crate::SliceKernel`].
#[cfg(target_arch = "aarch64")]
pub unsafe fn kern_d16_padal(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    unsafe { sum_d16_padal(&col.d16[lo..hi], hi - lo) }
}

/// # Safety
/// See [`crate::SliceKernel`].
#[cfg(target_arch = "aarch64")]
pub unsafe fn kern_d32_padal(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    unsafe { sum_d32_padal(&col.d32[lo..hi], hi - lo) }
}

#[cfg(test)]
mod dense_padal_tests {
    use bench_bitpack_carrier_shared::{sum_d16, sum_d32};

    use super::*;
    use crate::input::build_bytes;

    /// The attacked dense kernels agree with the reference ones they replace,
    /// at sizes on both sides of the 16-bit drain boundary and at sizes that
    /// are not multiples of the unroll factor, which is where a mishandled
    /// tail would hide.
    #[cfg(target_arch = "aarch64")]
    fn agrees_at(n: usize) {
        let buf = build_bytes(n, 31);
        let col: &Layout = unsafe { &*(buf.as_ptr() as *const Layout) };
        let r16 = sum_d16(&col.d16[..n], n);
        let r32 = sum_d32(&col.d32[..n], n);
        assert_eq!(
            unsafe { sum_d16_padal(&col.d16[..n], n) },
            r16,
            "sum_d16_padal disagrees at n={n}"
        );
        assert_eq!(
            unsafe { sum_d32_padal(&col.d32[..n], n) },
            r32,
            "sum_d32_padal disagrees at n={n}"
        );
        assert_eq!(
            r16, r32,
            "the two carriers disagree, so the fixture is wrong"
        );
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn attacked_dense_kernels_agree_below_one_drain() {
        agrees_at(16384);
        agrees_at(131072);
        agrees_at(131072 - 24);
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn attacked_dense_kernels_agree_across_a_drain_boundary() {
        agrees_at(4194304);
        agrees_at(4194304 - 8);
    }
}
