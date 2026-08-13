//! The same gate over a law that is false, so it selects the sequential fallback.
//! The control: if this timed like `satfold-lanes16` the gate would not be
//! selecting and the agreement of the other two would prove nothing.
//!
//! See `bench-satfold-shared` for the verdicts, the kernels and the oracle.

use bench_satfold_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "satfold-gate-false",
    sizes = [
        7000, 10000, 12000
    ]
)]
fn run_gate_false<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms::gate_false(KEY, input);
        }
    }
}
