//! `lanes16`, built declaring exactly the three sizes the const-gate section
//! runs, so it can be compared against the gated arm without the comparison
//! being about how many monomorphisations each dylib carries.
//!
//! `satfold-lanes16` declares twenty-seven sizes because it is an arm in five
//! other sections. Its `bench_entry` therefore holds twenty-seven
//! monomorphisations and a dispatch over them, and hashing it against the gated
//! arm's three compares dispatch tables rather than the thing under test. This
//! crate is that control: same kernel, same declared sizes as
//! `satfold-gate-true`, no gate.

use bench_satfold_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "satfold-lanes16-3",
    sizes = [
        7000, 10000, 12000
    ]
)]
fn run_lanes16_3<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms::lanes16(KEY, input);
        }
    }
}
