//! The packed decode with 4 independent accumulators.
//!
//! Same group decode as `bitpack-contend-packed-simd`, same `UADALP` pairwise
//! accumulate, differing only in how many accumulators the loop carries. The
//! single accumulator makes consecutive groups dependent when nothing about
//! them is, which is what leaves the decode's own latency chain exposed once
//! the reduction stops hiding it.
//!
//! Two unroll factors ship because which one wins is a question about register
//! pressure and issue width on this core, and picking one and reporting it
//! would be a bench with no competitor.

use bench_bitpack_contend_shared::{column_pass, kern_packed_pipe4, Contend};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Contend,
    "bitpack-contend-pipe4",
    sizes = [
        163841, 163844,
        41943041, 41943044,
        83886081, 83886084,
    ]
)]
fn run_contend_pipe4<const KEY: usize>(
    input: &<Contend<KEY> as Routine>::Input,
    output: &mut <Contend<KEY> as Routine>::Output,
) -> FfiBenchCall {
    let () = Contend::<KEY>::KEY_SPLITS;
    let t = Contend::<KEY>::T;
    let n = Contend::<KEY>::N;
    let base = input as *const _ as *const u8;
    timed! {
        run {
            // SAFETY: the input outlives the pass and `KEY_SPLITS` refused any
            // key whose slices would not land on a packed-period boundary.
            output.value = unsafe { column_pass(t, n, base, kern_packed_pipe4) };
        }
    }
}
