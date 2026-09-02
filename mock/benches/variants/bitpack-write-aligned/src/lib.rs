//! Packed write, safe by construction: `bitpack-write-contend-safe` only
//! declares sizes where every internal thread boundary lands on the packed
//! period (8 elements at 13 bits), so no byte this kernel writes is ever
//! written by another thread. The kernel itself never checks this; the
//! `(N, T)` pair the caller chose is the whole of the correctness argument,
//! which is exactly the property `bitpack-write-contend-shared`'s own test
//! suite pins for both this kernel and the misaligned one.

use bench_bitpack_write_contend_shared::{
    decode_packed_sum, kern_packed_plain, write_pass, WriteContend,
};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    WriteContend,
    "bitpack-write-aligned",
    sizes = [655361, 655362, 655364, 20971521, 20971522, 20971524]
)]
fn run_write_aligned<const KEY: usize>(
    input: &<WriteContend<KEY> as Routine>::Input,
    output: &mut <WriteContend<KEY> as Routine>::Output,
) -> FfiBenchCall {
    let t = WriteContend::<KEY>::T;
    let n = WriteContend::<KEY>::N;
    let vals_ptr = input.vals.as_ptr();
    let out_ptr = input.packed_out.as_ptr() as *mut u8;
    timed! {
        run {
            // SAFETY: `bitpack-write-contend-safe`'s sizes are period-aligned
            // at every thread count it declares, pinned by
            // `chosen_sizes_land_where_the_bench_needs_them_to`, so no byte
            // this kernel writes is ever written by another thread.
            unsafe { write_pass(t, n, vals_ptr, out_ptr, kern_packed_plain) };
            let region = unsafe { std::slice::from_raw_parts(out_ptr, input.packed_out.len()) };
            output.value = unsafe { decode_packed_sum(region, n) };
        }
    }
}
