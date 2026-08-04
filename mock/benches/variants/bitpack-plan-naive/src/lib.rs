//! Zero-inter-value-padding buffer, read by file 75's own decoder shape:
//! byte offset and bit shift both derived from the running index at runtime.
//! See `bench-bitpack-plan-shared`.

use bench_bitpack_plan_shared::{sum_naive, PlanColumn};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(PlanColumn, "bitpack-plan-naive", sizes = [16384, 65536, 98304, 262144])]
fn run_plan_naive<const N: usize>(
    input: &<PlanColumn<N> as Routine>::Input,
    output: &mut <PlanColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = sum_naive(&input.packed, N);
        }
    }
}
