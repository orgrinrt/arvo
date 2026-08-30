//! The packed column through the `UADALP` reduction, split `T` ways.
//!
//! The fastest packed decode any committed bench in this directory has found.
//! It is here so that if packing loses under contention it loses with its best
//! kernel, and if it wins the win cannot be attributed to a weak dense arm.

use bench_bitpack_contend_shared::{column_pass, kern_packed_simd, Contend};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Contend,
    "bitpack-contend-packed-simd",
    sizes = [
        163841, 163842, 163844, 163848,
        10485761, 10485762, 10485764, 10485768,
        41943041, 41943042, 41943044, 41943048,
        83886081, 83886082, 83886084, 83886088,
    ]
)]
fn run_contend_packed_simd<const KEY: usize>(
    input: &<Contend<KEY> as Routine>::Input,
    output: &mut <Contend<KEY> as Routine>::Output,
) -> FfiBenchCall {
    let () = Contend::<KEY>::KEY_SPLITS;
    let t = Contend::<KEY>::T;
    let n = Contend::<KEY>::N;
    let base = input as *const _ as *const u8;
    timed! {
        run {
            // SAFETY: the input outlives the pass, `n` is the count it was built
            // at, and `KEY_SPLITS` refused any key whose slices would not land
            // on a packed-period boundary.
            output.value = unsafe { column_pass(t, n, base, kern_packed_simd) };
        }
    }
}
