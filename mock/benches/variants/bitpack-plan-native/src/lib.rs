//! Dense native carrier, sequential column sum. The ceiling the packed
//! readings are measured against. See `bench-bitpack-plan-shared`.

use bench_bitpack_plan_shared::{sum_native, PlanColumn};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(PlanColumn, "bitpack-plan-native", sizes = [16384, 65536, 98304, 262144])]
fn run_plan_native<const N: usize>(
    input: &<PlanColumn<N> as Routine>::Input,
    output: &mut <PlanColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = sum_native(&input.logical, N);
        }
    }
}
