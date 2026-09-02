//! Hand-written NEON with eight accumulators instead of four, added after the
//! first run to attack the mechanism behind the four-accumulator arm's plateau
//! at the longest reduction length rather than to report it.
//!
//! See `bench-satfold-shared` for the kernel, the legality argument and the
//! oracle.

use bench_satfold_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "satfold-neon8",
    sizes = [
        1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000,
        9000, 10000, 11000, 12000, 1001, 3001, 7001, 10001,
        12001
    ]
)]
fn run_neon8<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms::neon8(KEY, input);
        }
    }
}
