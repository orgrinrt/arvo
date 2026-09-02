//! The `u16`-against-packed question at a column far past last-level cache.
//!
//! The contention sweep's largest row holds a 16 MiB `u16` column and a 13.0 MiB
//! packed one against this host's 12 MiB L2. Both are past it, and both are only
//! just past it, so partial residency is still in the numbers and the two arms
//! land within 2.4 per cent of each other, which is inside the threaded noise
//! floor. A tie is the one answer that cannot be reported as a result.
//!
//! This crate carries only the two regions that question needs, which buys four
//! times the record count inside the same allocation: at 33,554,432 records the
//! `u16` region is 64 MiB and the packed region 52 MiB, five and four times the
//! L2. Nothing is resident, in either harness mode, and the comparison is bytes
//! against bytes.
//!
//! Everything else is the contention crate's: the same pool, the same slicing,
//! the same kernels, the same key encoding. The pool takes an erased `*const u8`
//! base so it can carry either layout without knowing about either.
//!
//! Bench infrastructure, not shipping arvo source: no `#![no_std]`, `std` used
//! freely.

use bench_bitpack_carrier_shared::{sum_d16, Plan13, Sum, LOGICAL_BITS, MASK13};
use bench_bitpack_plan_shared::{pack, sum_naive};
use mockspace_bench_core::Routine;

/// 33,554,432 records: a 64 MiB `u16` region and a 52 MiB packed region, both
/// several times this host's 12 MiB L2.
pub const MAX_N: usize = 33_554_432;
pub const D16_BYTES: usize = MAX_N * 2;
pub const PACKED_BYTES: usize = (MAX_N * LOGICAL_BITS) / 8 + 16;
pub const OFF_D16: usize = 0;
pub const OFF_PACKED: usize = D16_BYTES;
pub const TOTAL_INPUT_BYTES: usize = OFF_PACKED + PACKED_BYTES;

/// The two-region layout. Field sizes do not depend on the const parameter, so
/// every instantiation has the same layout and `WideColumn<0>` names it.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WideColumn<const N: usize> {
    pub d16: [u16; MAX_N],
    pub packed: [u8; PACKED_BYTES],
}

/// The layout without a const parameter, for the same reason the contention
/// crate has one: the routine's `Input` cannot carry an expression of `KEY`.
pub type Layout = WideColumn<0>;

struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// Build the two-region input for a runtime element count.
///
/// The same value stream the contention crate builds, from the same seed
/// derivation, which is what makes the two benches' rows comparable. Pinned by a
/// test rather than assumed.
pub fn build_bytes(n: usize, seed: u64) -> Vec<u8> {
    let mut rng = SplitMix64(seed ^ 0xB179_ACC0_0001_5EED);
    let vals: Vec<u16> = (0..n).map(|_| (rng.next() & MASK13) as u16).collect();
    let mut buf = vec![0u8; TOTAL_INPUT_BYTES];
    for (i, &v) in vals.iter().enumerate() {
        buf[OFF_D16 + i * 2..OFF_D16 + i * 2 + 2].copy_from_slice(&v.to_le_bytes());
    }
    let packed_bytes = (n * LOGICAL_BITS) / 8 + 16;
    pack(&vals, &mut buf[OFF_PACKED..OFF_PACKED + packed_bytes]);
    buf
}

/// # Safety
/// `base` points at a live `Layout` and `lo <= hi <= N`.
pub unsafe fn kern_d16(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    sum_d16(&col.d16[lo..hi], hi - lo)
}

/// # Safety
/// As [`kern_d16`].
#[cfg(target_arch = "aarch64")]
pub unsafe fn kern_d16_padal(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    unsafe { bench_bitpack_contend_shared::sum_d16_padal(&col.d16[lo..hi], hi - lo) }
}

/// # Safety
/// As [`kern_d16`], and `lo` is a multiple of the packed period.
#[cfg(target_arch = "aarch64")]
pub unsafe fn kern_pipe4(base: *const u8, lo: usize, hi: usize) -> u64 {
    let col = unsafe { &*(base as *const Layout) };
    let byte_lo = (lo * LOGICAL_BITS) / 8;
    unsafe {
        bench_bitpack_contend_shared::sum_padal_pipe4::<Plan13>(&col.packed[byte_lo..], hi - lo)
    }
}

/// One row: `KEY = N * 10 + T`, the contention crate's encoding unchanged.
pub struct Wide<const KEY: usize>;

impl<const KEY: usize> Wide<KEY> {
    pub const N: usize = KEY / 10;
    pub const T: usize = KEY % 10;
    /// Refuses at monomorphisation when a key cannot be split onto a
    /// packed-period boundary, or when it exceeds the layout.
    pub const KEY_SPLITS: () = {
        assert!(Self::T >= 1 && Self::T <= 8);
        assert!(Self::N <= MAX_N);
        assert!(Self::N % (Self::T * 8) == 0);
    };
}

impl<const KEY: usize> Routine for Wide<KEY> {
    type Input = Layout;
    type Output = Sum;

    fn build_input(_seed: u64) -> Self::Input {
        unreachable!(
            "Wide::build_input is never called by the real bench path and is not \
             safe at any KEY: Self::Input is MAX_N-sized for every \
             monomorphisation. Use build_input_bytes."
        )
    }

    fn build_input_bytes(seed: u64) -> Vec<u8> {
        let () = Self::KEY_SPLITS;
        build_bytes(Self::N, seed)
    }

    /// Ground truth from the `u16` region, then the packed region through
    /// `sum_naive`, an index-driven decoder no timed arm here runs, so a defect
    /// shared between `pack` and the windowed decoders is not invisible.
    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let n = Self::N;
        let mut expect: u64 = 0;
        for &v in input.d16[..n].iter() {
            expect = expect.wrapping_add(v as u64);
        }
        if output.value != expect {
            return Err("column sum mismatch against the u16 ground truth");
        }
        let packed_bytes = (n * LOGICAL_BITS) / 8 + 16;
        if sum_naive(&input.packed[..packed_bytes], n) != expect {
            return Err("packed region mismatch: sum_naive disagrees with the u16 truth");
        }
        Ok(())
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        Self::N as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bench_bitpack_contend_shared::slice_bounds;

    const NS: [usize; 3] = [8_388_608, 16_777_216, 33_554_432];
    const TS: [usize; 2] = [1, 4];

    #[test]
    fn layout_is_independent_of_the_const_parameter() {
        assert_eq!(core::mem::size_of::<Layout>(), TOTAL_INPUT_BYTES);
        assert_eq!(
            core::mem::size_of::<Layout>(),
            core::mem::size_of::<WideColumn<{ MAX_N }>>()
        );
    }

    /// The point of the crate, asserted rather than described: at every declared
    /// size both regions are several times this host's L2, so neither can be
    /// resident and the comparison is bytes against bytes.
    #[test]
    fn both_regions_are_far_past_this_hosts_l2() {
        const L2: usize = 12_582_912;
        for n in NS {
            let d16 = n * 2;
            let packed = (n * LOGICAL_BITS) / 8;
            assert!(d16 > L2, "the u16 region fits L2 at n={n}");
            assert!(packed > L2, "the packed region fits L2 at n={n}");
        }
        // and at the largest size, by a wide margin on both sides
        assert!(NS[2] * 2 > 4 * L2);
        assert!((NS[2] * LOGICAL_BITS) / 8 > 4 * L2);
    }

    #[test]
    fn every_declared_key_decodes_and_splits() {
        for n in NS {
            for t in TS {
                let key = n * 10 + t;
                assert_eq!(key / 10, n);
                assert_eq!(key % 10, t);
                assert_eq!(
                    n % (t * 8),
                    0,
                    "n={n} does not split {t} ways on the period"
                );
                assert!(n <= MAX_N);
            }
        }
    }

    /// The value stream matches the contention crate's at the same seed and
    /// count, which is what makes rows from the two benches comparable. Checked
    /// on the `u16` region element by element rather than on a sum, because a
    /// permutation passes a sum check.
    #[test]
    fn the_value_stream_matches_the_contention_crates() {
        const N: usize = 16384;
        for seed in 0u64..3 {
            let mine = build_bytes(N, seed);
            let theirs = bench_bitpack_contend_shared::build_bytes(N, seed);
            for i in 0..N {
                let a = u16::from_le_bytes([mine[OFF_D16 + i * 2], mine[OFF_D16 + i * 2 + 1]]);
                let off = bench_bitpack_carrier_shared::OFF_D16;
                let b = u16::from_le_bytes([theirs[off + i * 2], theirs[off + i * 2 + 1]]);
                assert_eq!(a, b, "value {i} differs at seed {seed}");
            }
        }
    }

    /// Every kernel, split every declared way, equals the whole pass.
    #[test]
    fn a_split_pass_equals_the_whole_pass_for_every_kernel() {
        const N: usize = 16384;
        let buf = build_bytes(N, 5);
        let base = buf.as_ptr();
        let col: &Layout = unsafe { &*(base as *const Layout) };
        let mut truth = 0u64;
        for &v in col.d16[..N].iter() {
            truth = truth.wrapping_add(v as u64);
        }
        let mut kernels: Vec<(&str, unsafe fn(*const u8, usize, usize) -> u64)> =
            vec![("d16", kern_d16)];
        #[cfg(target_arch = "aarch64")]
        {
            kernels.push(("d16-padal", kern_d16_padal));
            kernels.push(("pipe4", kern_pipe4));
        }
        for (name, k) in kernels {
            for t in [1usize, 2, 4, 8] {
                let mut got = 0u64;
                for i in 0..t {
                    let (lo, hi) = slice_bounds(i, N, t);
                    got = got.wrapping_add(unsafe { k(base, lo, hi) });
                }
                assert_eq!(got, truth, "kernel {name} split {t} ways disagrees");
            }
        }
    }

    #[test]
    fn validate_output_rejects_a_wrong_sum() {
        const KEY: usize = 16384 * 10 + 1;
        let buf = build_bytes(16384, 3);
        let col: &Layout = unsafe { &*(buf.as_ptr() as *const Layout) };
        let good = Sum {
            value: sum_d16(&col.d16[..16384], 16384),
        };
        // validate_output reads Self::N elements, so it is exercised at the key's
        // own size elsewhere; here the refusal itself is what is pinned
        let bad = Sum {
            value: good.value.wrapping_add(1),
        };
        assert!(<Wide<KEY> as Routine>::validate_output(col, &bad).is_err());
    }
}
