//! The packed decode with four independent accumulators, on a column several
//! times past L2.
//!
//! The best packed kernel in the directory. Both regions here are four to five
//! times the last-level cache, so neither arm can be resident in either harness
//! mode and the comparison is bytes against bytes with nothing else in it.

use bench_bitpack_contend_shared::column_pass;
use bench_bitpack_wide_shared::{kern_pipe4, Wide};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Wide,
    "bitpack-wide-pipe4",
    sizes = [
        83886081, 83886084,
        167772161, 167772164,
        335544321, 335544324,
    ]
)]
fn run_wide_pipe4<const KEY: usize>(
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
            output.value = unsafe { column_pass(t, n, base, kern_pipe4) };
        }
    }
}
