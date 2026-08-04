//! Plan-driven packed decode through a byte gather, so fields arrive in their
//! natural lane width rather than in the window's. Sequential column sum. See
//! `bench-bitpack-plan-shared`.

use bench_bitpack_plan_shared::{sum_simd, Pack, PlanColumn};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(PlanColumn, "bitpack-plan-simd", sizes = [16384, 65536, 98304, 262144])]
fn run_plan_simd<const N: usize>(
    input: &<PlanColumn<N> as Routine>::Input,
    output: &mut <PlanColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            // SAFETY: `packed` carries 16 bytes of headroom past the last
            // group and every swept N is a multiple of the width-13 period.
            output.value = unsafe { sum_simd::<Pack<13>>(&input.packed, N) };
        }
    }
}
