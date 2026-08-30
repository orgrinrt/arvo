//! Dense write baseline, split `T` ways over one column.
//!
//! Every element is a whole, naturally aligned `u16`, so no split of `[0, n)`
//! at any `lo`/`hi` can put two threads' elements in the same word. This arm
//! never has a hazard regardless of `(N, T)`, which is what makes it the
//! reference every packed arm is priced against.

use bench_bitpack_write_contend_shared::{
    decode_dense_sum, kern_dense_d16, write_pass, WriteContend,
};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    WriteContend,
    "bitpack-write-dense",
    sizes = [
        655361, 655362, 655364,
        20971521, 20971522, 20971524,
        655341, 655342, 655344,
        20971501, 20971502, 20971504,
    ]
)]
fn run_write_dense<const KEY: usize>(
    input: &<WriteContend<KEY> as Routine>::Input,
    output: &mut <WriteContend<KEY> as Routine>::Output,
) -> FfiBenchCall {
    let t = WriteContend::<KEY>::T;
    let n = WriteContend::<KEY>::N;
    let vals_ptr = input.vals.as_ptr();
    let out_ptr = input.dense_out.as_ptr() as *mut u16;
    timed! {
        run {
            // SAFETY: `input` outlives the pass and `n` is the count it was
            // built at. The dense kernel never touches a byte another thread
            // touches, at any `(n, t)`, so no external synchronisation is
            // required beyond what the pool already provides for liveness.
            unsafe { write_pass(t, n, vals_ptr, out_ptr as *mut u8, kern_dense_d16) };
            output.value = unsafe { decode_dense_sum(out_ptr, n) };
        }
    }
}
