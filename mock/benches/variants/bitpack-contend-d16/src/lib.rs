//! Dense read at a 16-bit carrier, split `T` ways over one column.
//!
//! The tightest native carrier a 13-bit value can have, and the one every prior
//! bitpack bench in this directory measures against. On one core it never
//! becomes bound by bytes delivered at any size this host can hold, which is why
//! packing has lost to it everywhere. Four cores demand four times its bytes at
//! the same per-element work, so whether it stays compute-bound is the question
//! this whole bench exists to answer.

use bench_bitpack_contend_shared::{column_pass, kern_d16, Contend};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Contend,
    "bitpack-contend-d16",
    sizes = [
        163841, 163842, 163844, 163848,
        10485761, 10485762, 10485764, 10485768,
        41943041, 41943042, 41943044, 41943048,
        83886081, 83886082, 83886084, 83886088,
    ]
)]
fn run_contend_d16<const KEY: usize>(
    input: &<Contend<KEY> as Routine>::Input,
    output: &mut <Contend<KEY> as Routine>::Output,
) -> FfiBenchCall {
    let () = Contend::<KEY>::KEY_SPLITS;
    let t = Contend::<KEY>::T;
    let n = Contend::<KEY>::N;
    let base = input as *const _;
    timed! {
        run {
            // SAFETY: the input outlives the pass, `n` is the count it was built
            // at, and `KEY_SPLITS` refused any key whose slices would not land
            // on a packed-period boundary.
            output.value = unsafe { column_pass(t, n, base, kern_d16) };
        }
    }
}
