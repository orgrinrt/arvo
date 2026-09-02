//! Plan-driven packed decode that narrows each group to natural-width lanes
//! before the kernel runs. See `bench-bitpack-plan-shared`.

use bench_bitpack_plan_shared::{mac_windowed_narrow, MacColumn, Pack};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(MacColumn, "bitpack-mac-narrow", sizes = [16384, 65536, 98304, 262144])]
fn run_mac_narrow<const N: usize>(
    input: &<MacColumn<N> as Routine>::Input,
    output: &mut <MacColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            // SAFETY: as `bitpack-plan-windowed`.
            output.value = unsafe { mac_windowed_narrow::<Pack<13>>(&input.packed, N) };
        }
    }
}
