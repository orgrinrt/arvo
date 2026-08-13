//! The fold as a consumer writes it: one loop-carried accumulator, strictly
//! left-associated, no law invoked. `80`'s `sat_sum_seq`, which is the arm every
//! ratio in `80` section 5.3 is quoted against.
//!
//! See `bench-satfold-shared` for the workload, the oracle, the law the
//! reassociated arms rest on, and why every arm here computes the same value.

use bench_satfold_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "satfold-seq",
    sizes = [
        1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000,
        9000, 10000, 11000, 12000, 1001, 3001, 7001, 10001,
        12001, 3010, 7010, 10010, 12010, 3100, 7100, 12100,
        3101, 7101, 12101
    ]
)]
fn run_seq<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms::seq(KEY, input);
        }
    }
}
