//! Packed write, correct at any split: the byte at each end of a thread's own
//! range that a neighbour could also be writing goes through an atomic
//! fetch-or; every other byte is written plain. Priced against
//! `bitpack-write-unsound` (same misaligned sizes, no guard) and against
//! `bitpack-write-aligned` (different, period-aligned sizes, safe by
//! construction instead of by atomics), which is the composition this bench
//! exists to produce: whether alignment-by-construction or atomics-at-the-
//! boundary is the cheaper way to buy correctness, and what either costs
//! against not buying it at all.

use bench_bitpack_write_contend_shared::{
    decode_packed_sum, kern_packed_guarded, write_pass, WriteContend,
};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    WriteContend,
    "bitpack-write-guarded",
    sizes = [655341, 655342, 655344, 20971501, 20971502, 20971504]
)]
fn run_write_guarded<const KEY: usize>(
    input: &<WriteContend<KEY> as Routine>::Input,
    output: &mut <WriteContend<KEY> as Routine>::Output,
) -> FfiBenchCall {
    let t = WriteContend::<KEY>::T;
    let n = WriteContend::<KEY>::N;
    let vals_ptr = input.vals.as_ptr();
    let out_ptr = input.packed_out.as_ptr() as *mut u8;
    timed! {
        run {
            // SAFETY: `kern_packed_guarded` computes its own guard bytes from
            // `(lo, hi, n)` and takes them through an atomic fetch-or, so no
            // external synchronisation beyond the pool's liveness guarantee
            // is required at any split.
            unsafe { write_pass(t, n, vals_ptr, out_ptr, kern_packed_guarded) };
            let region = unsafe { std::slice::from_raw_parts(out_ptr, input.packed_out.len()) };
            output.value = unsafe { decode_packed_sum(region, n) };
        }
    }
}
