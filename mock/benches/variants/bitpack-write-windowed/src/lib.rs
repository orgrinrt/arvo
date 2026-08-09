//! Packed write, attacked: the same period-aligned splits
//! `bitpack-write-aligned` runs, with the byte-at-a-time naive encoder
//! replaced by a group-at-a-time window merge (`write_packed_windowed`),
//! mirroring the read side's `sum_windowed` shape. Priced against
//! `bitpack-write-aligned` to measure what the attack bought and against
//! `bitpack-write-dense` to see whether it changes which side of the trade
//! packing sits on.

use bench_bitpack_write_contend_shared::{
    decode_packed_sum, kern_packed_windowed, write_pass, WriteContend,
};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    WriteContend,
    "bitpack-write-windowed",
    sizes = [655361, 655362, 655364, 20971521, 20971522, 20971524]
)]
fn run_write_windowed<const KEY: usize>(
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
            // at every thread count it declares, which is `kern_packed_windowed`'s
            // own safety contract.
            unsafe { write_pass(t, n, vals_ptr, out_ptr, kern_packed_windowed) };
            let region = unsafe { std::slice::from_raw_parts(out_ptr, input.packed_out.len()) };
            output.value = unsafe { decode_packed_sum(region, n) };
        }
    }
}
