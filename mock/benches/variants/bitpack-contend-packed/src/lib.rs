//! The packed 13-bit column through the scalar windowed decode, split `T` ways.
//!
//! 1.625 bytes per element, the smallest footprint in the sweep and the largest
//! per-element cost. On one core that trade loses to every dense carrier below
//! about 5.8 bytes. Under contention the compute cost is unchanged and the byte
//! demand is the lowest of the six arms, which is the asymmetry this bench is
//! pointed at.

use bench_bitpack_contend_shared::{column_pass, kern_packed, Contend};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Contend,
    "bitpack-contend-packed",
    sizes = [
        163841, 163842, 163844, 163848,
        10485761, 10485762, 10485764, 10485768,
        41943041, 41943042, 41943044, 41943048,
        83886081, 83886082, 83886084, 83886088,
    ]
)]
fn run_contend_packed<const KEY: usize>(
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
            output.value = unsafe { column_pass(t, n, base, kern_packed) };
        }
    }
}
