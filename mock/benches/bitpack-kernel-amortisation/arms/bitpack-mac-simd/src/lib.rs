//! The same byte-gather decode feeding the heavier per-element kernel. See
//! `bench-bitpack-plan-shared`.

use bench_bitpack_plan_shared::{MacColumn, Pack, mac_simd};
use mockspace_bench_core::{FfiBenchCall, Routine, timed};
use mockspace_bench_macro::bench_variant;

#[bench_variant(MacColumn, "bitpack-mac-simd", sizes = [16384, 65536, 98304, 262144])]
fn run_mac_simd<const N: usize>(
    input: &<MacColumn<N> as Routine>::Input,
    output: &mut <MacColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            // SAFETY: as `bitpack-plan-simd`.
            output.value = unsafe { mac_simd::<Pack<13>>(&input.packed, N) };
        }
    }
}
