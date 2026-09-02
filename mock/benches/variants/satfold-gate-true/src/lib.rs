//! `Lanes16` reached through a const gate whose verdict is computed by an
//! exhaustive sweep inside a `const fn`. `82` F11 and `80` section 5.1 both
//! report that such a gate erases; this arm and `satfold-lanes16` are the same
//! computation with and without it, so the claim becomes refutable in time.
//!
//! See `bench-satfold-shared` for the verdicts, the kernels and the oracle.

use bench_satfold_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "satfold-gate-true",
    sizes = [
        7000, 10000, 12000
    ]
)]
fn run_gate_true<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms::gate_true(KEY, input);
        }
    }
}
