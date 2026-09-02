//! Index-driven packed decode feeding the heavier per-element kernel. See
//! `bench-bitpack-plan-shared`.

use bench_bitpack_plan_shared::{mac_naive, MacColumn};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(MacColumn, "bitpack-mac-naive", sizes = [16384, 65536, 98304, 262144])]
fn run_mac_naive<const N: usize>(
    input: &<MacColumn<N> as Routine>::Input,
    output: &mut <MacColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = mac_naive(&input.packed, N);
        }
    }
}
