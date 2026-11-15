//! Dense native carrier feeding the heavier per-element kernel. See
//! `bench-bitpack-plan-shared`.

use bench_bitpack_plan_shared::{MacColumn, PlanColumn, mac_native};
use mockspace_bench_core::{FfiBenchCall, Routine, timed};
use mockspace_bench_macro::bench_variant;

#[bench_variant(MacColumn, "bitpack-mac-native", sizes = [16384, 65536, 98304, 262144])]
fn run_mac_native<const N: usize>(
    input: &<MacColumn<N> as Routine>::Input,
    output: &mut <MacColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    let _ = core::marker::PhantomData::<PlanColumn<N>>;
    timed! {
        run {
            output.value = mac_native(&input.logical, N);
        }
    }
}
