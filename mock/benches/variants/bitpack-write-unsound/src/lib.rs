//! Packed write, the naive parallel encoder a caller would produce without
//! knowing about the packed representation's own period, run against
//! `bitpack-write-contend-race`'s deliberately misaligned sizes.
//!
//! This is the same `kern_packed_plain` function `bitpack-write-aligned`
//! runs; only the `(N, T)` pair differs, which is the whole point: the
//! kernel is not the defect, the split is. Included as a demonstration arm,
//! not as a candidate. `bitpack-write-contend-shared`'s own stress tests
//! (`stress.rs`) already measured this exact kernel corrupting 16 to 19
//! percent of concurrent passes at a misaligned split on this host; this
//! variant exists to show what that costs (or does not cost) against the
//! guarded fix, not to compete for the fastest time. The harness's own
//! cross-arm validation may or may not catch an individual run's corruption
//! (a race is not guaranteed to fire on every pass), which is itself part of
//! the finding: a defect that is not always caught is worse than one that
//! fails loudly every time.

use bench_bitpack_write_contend_shared::{
    decode_packed_sum, kern_packed_plain, write_pass, WriteContend,
};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    WriteContend,
    "bitpack-write-unsound",
    sizes = [655341, 655342, 655344, 20971501, 20971502, 20971504]
)]
fn run_write_unsound<const KEY: usize>(
    input: &<WriteContend<KEY> as Routine>::Input,
    output: &mut <WriteContend<KEY> as Routine>::Output,
) -> FfiBenchCall {
    let t = WriteContend::<KEY>::T;
    let n = WriteContend::<KEY>::N;
    let vals_ptr = input.vals.as_ptr();
    let out_ptr = input.packed_out.as_ptr() as *mut u8;
    timed! {
        run {
            // SAFETY: liveness only. Correctness is NOT guaranteed here; that
            // is the point of this arm.
            unsafe { write_pass(t, n, vals_ptr, out_ptr, kern_packed_plain) };
            let region = unsafe { std::slice::from_raw_parts(out_ptr, input.packed_out.len()) };
            output.value = unsafe { decode_packed_sum(region, n) };
        }
    }
}
