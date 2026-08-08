//! The 16-bit dense read with its widening chain attacked.
//!
//! Same region, same mask, same wrapping sum as `bitpack-contend-d16`, and the
//! harness holds the two to bit-identical output. The difference is the
//! accumulate: pairwise `UADALP` into four independent accumulators instead of
//! a chain of widening adds.
//!
//! This arm exists because attacking the packed decode and not the dense one
//! would make the comparison a strawman. If packing still wins after this, it
//! wins against the best dense kernel in the directory rather than against the
//! one that happened to be committed first.

use bench_bitpack_contend_shared::{column_pass, kern_d16_padal, Contend};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Contend,
    "bitpack-contend-d16-padal",
    sizes = [
        163841, 163844,
        41943041, 41943044,
        83886081, 83886084,
    ]
)]
fn run_contend_d16_padal<const KEY: usize>(
    input: &<Contend<KEY> as Routine>::Input,
    output: &mut <Contend<KEY> as Routine>::Output,
) -> FfiBenchCall {
    let () = Contend::<KEY>::KEY_SPLITS;
    let t = Contend::<KEY>::T;
    let n = Contend::<KEY>::N;
    let base = input as *const _;
    timed! {
        run {
            // SAFETY: the input outlives the pass and `KEY_SPLITS` refused any
            // key whose slices would not land on a packed-period boundary.
            output.value = unsafe { column_pass(t, n, base, kern_d16_padal) };
        }
    }
}
