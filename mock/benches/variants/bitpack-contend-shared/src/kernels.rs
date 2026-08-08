//! One slice kernel per arm, plus the two attacks on the packed decode.
//!
//! Each kernel is a thin wrapper over a sum the carrier crate already exports,
//! so the inner loop is exactly the code the single-core sweep measured. They
//! share one function-pointer shape so the pool can carry any of them and the
//! tests can drive all of them through one table.

use bench_bitpack_carrier_shared::{sum_d16, sum_d32, sum_d64, Plan13, LOGICAL_BITS};

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
pub unsafe fn kern_d16(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
    sum_d16(&col.d16[lo..hi], hi - lo)
}

/// # Safety
/// See [`SliceKernel`].
pub unsafe fn kern_d32(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
    sum_d32(&col.d32[lo..hi], hi - lo)
}

/// # Safety
/// See [`SliceKernel`].
pub unsafe fn kern_d64(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
    sum_d64(&col.d64[lo..hi], hi - lo)
}

/// # Safety
/// See [`SliceKernel`]. `lo` must be a multiple of the packed period so the
/// slice starts on a byte boundary.
pub unsafe fn kern_packed(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
    let byte_lo = (lo * LOGICAL_BITS) / 8;
    unsafe { bench_bitpack_plan_shared::sum_windowed::<Plan13>(&col.packed[byte_lo..], hi - lo) }
}

/// # Safety
/// See [`SliceKernel`]. `lo` must be a multiple of the packed period so the
/// slice starts on a byte boundary.
#[cfg(target_arch = "aarch64")]
pub unsafe fn kern_packed_simd(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
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
pub unsafe fn kern_packed_pipe2(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
    let byte_lo = (lo * LOGICAL_BITS) / 8;
    unsafe { sum_padal_pipe2::<Plan13>(&col.packed[byte_lo..], hi - lo) }
}

/// # Safety
/// See [`SliceKernel`]. `lo` must be a multiple of the packed period.
#[cfg(target_arch = "aarch64")]
pub unsafe fn kern_packed_pipe4(base: *const Layout, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*base };
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
