//! Every test the crate has, in one place because they share one fixture.

#![cfg(test)]

use bench_bitpack_carrier_shared::{sum_d16, CarrierColumn, Sum, TOTAL_INPUT_BYTES};
use mockspace_bench_core::Routine;

use crate::input::{build_bytes, slice_bounds, Layout};
#[cfg(target_arch = "aarch64")]
use crate::kernels::kern_packed_simd;
use crate::kernels::{kern_d16, kern_d32, kern_d64, kern_packed};
use crate::pool::column_pass;
use crate::routine::Contend;
use crate::SliceKernel;

#[cfg(test)]
mod suite {
    use super::*;
    use bench_bitpack_carrier_shared::MAX_N;

    /// Every `(n, t)` the bench declares. Named once so no test can sample a
    /// subset of the matrix without that being visible here.
    const NS: [usize; 4] = [16384, 1048576, 4194304, 8388608];
    const TS: [usize; 4] = [1, 2, 4, 8];

    /// The layout is independent of the const parameter, which is what makes
    /// `CarrierColumn<0>` a legitimate name for it and what the harness's cast
    /// of the raw input buffer relies on.
    #[test]
    fn layout_is_independent_of_the_const_parameter() {
        assert_eq!(core::mem::size_of::<Layout>(), TOTAL_INPUT_BYTES);
        assert_eq!(
            core::mem::size_of::<Layout>(),
            core::mem::size_of::<CarrierColumn<{ MAX_N }>>()
        );
        assert_eq!(
            core::mem::size_of::<Layout>(),
            core::mem::size_of::<CarrierColumn<16384>>()
        );
    }

    /// The runtime-`n` builder agrees byte for byte with the carrier crate's
    /// const-`N` one. Without this the two benches could drift into measuring
    /// different value streams while both looked fine, and every claim that this
    /// file's numbers compose with the carrier sweep's would be void.
    #[test]
    fn build_bytes_equals_the_carrier_crates_builder() {
        for seed in 0u64..4 {
            let mine = build_bytes(16384, seed);
            let theirs = <CarrierColumn<16384> as Routine>::build_input_bytes(seed);
            assert_eq!(mine.len(), theirs.len(), "length differs at seed {seed}");
            assert!(mine == theirs, "byte streams differ at seed {seed}");
        }
        let mine = build_bytes(131072, 9);
        let theirs = <CarrierColumn<131072> as Routine>::build_input_bytes(9);
        assert!(mine == theirs, "byte streams differ at n = 131072");
    }

    /// Key decoding over every key the bench declares, not a sample of them.
    #[test]
    fn every_declared_key_decodes_and_splits() {
        for n in NS {
            for t in TS {
                let key = n * 10 + t;
                assert_eq!(key / 10, n, "key {key} decodes to the wrong n");
                assert_eq!(key % 10, t, "key {key} decodes to the wrong t");
                assert_eq!(
                    n % (t * 8),
                    0,
                    "n = {n} does not split {t} ways on a packed-period boundary"
                );
            }
        }
    }

    /// The slice bounds tile the column exactly, at every declared thread count.
    /// A gap or an overlap would change the answer, and a gap in the middle of a
    /// large column is precisely the defect a sum check at one thread count
    /// would not see.
    #[test]
    fn slices_tile_the_column_at_every_thread_count() {
        for n in NS {
            for t in TS {
                let mut expect_lo = 0usize;
                for i in 0..t {
                    let (lo, hi) = slice_bounds(i, n, t);
                    assert_eq!(lo, expect_lo, "gap or overlap at n={n} t={t} slice {i}");
                    assert!(hi > lo, "empty slice at n={n} t={t} slice {i}");
                    assert_eq!(lo % 8, 0, "slice {i} at n={n} t={t} is off the period");
                    expect_lo = hi;
                }
                assert_eq!(
                    expect_lo, n,
                    "slices do not cover the column at n={n} t={t}"
                );
            }
        }
    }

    /// A split pass equals the whole pass, for every kernel and every declared
    /// thread count. Driven through `slice_bounds` rather than through
    /// `column_pass`, because the pool is sized once per process by design and a
    /// test cannot exercise four thread counts through it. This is the half that
    /// could be wrong: the slicing arithmetic and the packed byte offset. The
    /// pool itself is covered by the test below.
    #[test]
    fn a_split_pass_equals_the_whole_pass_for_every_kernel() {
        const N: usize = 16384;
        let buf = build_bytes(N, 5);
        let col: &Layout = unsafe { &*(buf.as_ptr() as *const Layout) };
        let base = buf.as_ptr();

        let mut truth = 0u64;
        for &v in col.d16[..N].iter() {
            truth = truth.wrapping_add(v as u64);
        }

        let mut kernels: Vec<(&str, SliceKernel)> = vec![
            ("d16", kern_d16 as SliceKernel),
            ("d32", kern_d32 as SliceKernel),
            ("d64", kern_d64 as SliceKernel),
            ("packed", kern_packed as SliceKernel),
        ];
        #[cfg(target_arch = "aarch64")]
        kernels.push(("packed-simd", kern_packed_simd as SliceKernel));

        for (name, k) in kernels {
            for t in TS {
                let mut got = 0u64;
                for i in 0..t {
                    let (lo, hi) = slice_bounds(i, N, t);
                    got = got.wrapping_add(unsafe { k(base, lo, hi) });
                }
                assert_eq!(got, truth, "kernel {name} split {t} ways disagrees");
            }
        }
    }

    /// The pool itself: workers pick up the job, compute their slice, and the
    /// coordinator's total matches the ground truth. One thread count only,
    /// because the pool is deliberately sized once per process and a test that
    /// resized it would be testing a contract the bench does not have.
    #[test]
    fn the_pool_computes_the_same_total_as_a_serial_pass() {
        const N: usize = 16384;
        let buf = build_bytes(N, 13);
        let col: &Layout = unsafe { &*(buf.as_ptr() as *const Layout) };
        let base = buf.as_ptr();

        let mut truth = 0u64;
        for &v in col.d16[..N].iter() {
            truth = truth.wrapping_add(v as u64);
        }
        let got = unsafe { column_pass(4, N, base, kern_d16 as SliceKernel) };
        assert_eq!(got, truth, "the four-thread pool disagrees with the column");

        // and a second pass over the same pool, because a generation counter
        // that only works once is a defect a single call cannot see
        let got2 = unsafe { column_pass(4, N, base, kern_packed as SliceKernel) };
        assert_eq!(got2, truth, "the second pass through the pool disagrees");
    }

    /// `validate_output` refuses a wrong answer. A validation pass that cannot
    /// fail is not a validation pass, and this bench's fidelity argument rests
    /// on the harness calling it.
    #[test]
    fn validate_output_rejects_a_wrong_sum() {
        const KEY: usize = 163841; // n = 16384, t = 1
        let buf = build_bytes(16384, 3);
        let col: &Layout = unsafe { &*(buf.as_ptr() as *const Layout) };
        let good = Sum {
            value: sum_d16(&col.d16[..16384], 16384),
        };
        assert!(<Contend<KEY> as Routine>::validate_output(col, &good).is_ok());
        let bad = Sum {
            value: good.value.wrapping_add(1),
        };
        assert!(
            <Contend<KEY> as Routine>::validate_output(col, &bad).is_err(),
            "validate_output accepted a sum off by one, so it would accept a broken arm"
        );
    }

    /// The refusal's arithmetic. A key whose slices do not land on a
    /// packed-period boundary would mis-address the packed arm, and
    /// `KEY_SPLITS` is what stops it at monomorphisation; the condition it
    /// tests is asserted here because a compile-fail test needs a trybuild
    /// harness this directory does not have.
    #[test]
    fn a_key_that_does_not_split_would_be_refused() {
        // 8200 elements do not divide four ways onto a period-8 boundary
        assert_ne!(8200 % (4 * 8), 0);
        // every key the bench declares does
        for n in NS {
            for t in TS {
                assert_eq!(n % (t * 8), 0);
            }
        }
    }
}
