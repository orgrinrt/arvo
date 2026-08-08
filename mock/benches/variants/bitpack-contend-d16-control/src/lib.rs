//! The noise floor: byte-identical to `bitpack-contend-d16`.
//!
//! Same kernel, same region, same arguments, same thread count. Any difference
//! between this arm and `bitpack-contend-d16` at a given row is measurement
//! rather than code, which is what makes every other delta in the row readable.
//! Under contention the floor is worth more than it is on one core, because a
//! parallel arm has a scheduler and a barrier in it and a reader is entitled to
//! ask how much of a small delta is either of those.

use bench_bitpack_contend_shared::{column_pass, kern_d16, Contend};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Contend,
    "bitpack-contend-d16-control",
    sizes = [
        163841, 163842, 163844, 163848,
        10485761, 10485762, 10485764, 10485768,
        41943041, 41943042, 41943044, 41943048,
        83886081, 83886082, 83886084, 83886088,
    ]
)]
fn run_contend_d16_control<const KEY: usize>(
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
            output.value = unsafe { column_pass(t, n, base, kern_d16) };
        }
    }
}
