//! The noise floor: byte-identical to `bitpack-wide-d16`.
//!
//! Same kernel, same region, same arguments, same thread count. Its gap against
//! `bitpack-wide-d16` is measurement rather than code, and it is what makes the
//! deltas in this section readable given that the threaded floor is two to three
//! per cent rather than the single-threaded sweep's under one.

use bench_bitpack_contend_shared::column_pass;
use bench_bitpack_wide_shared::{kern_d16, Wide};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Wide,
    "bitpack-wide-d16-control",
    sizes = [
        83886081, 83886084,
        167772161, 167772164,
        335544321, 335544324,
    ]
)]
fn run_wide_d16_control<const KEY: usize>(
    input: &<Wide<KEY> as Routine>::Input,
    output: &mut <Wide<KEY> as Routine>::Output,
) -> FfiBenchCall {
    let () = Wide::<KEY>::KEY_SPLITS;
    let t = Wide::<KEY>::T;
    let n = Wide::<KEY>::N;
    let base = input as *const _ as *const u8;
    timed! {
        run {
            // SAFETY: the input outlives the pass and `KEY_SPLITS` refused any
            // key whose slices would not land on a packed-period boundary.
            output.value = unsafe { column_pass(t, n, base, kern_d16) };
        }
    }
}
