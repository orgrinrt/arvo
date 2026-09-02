//! The packed arm: 13 bits per element, 1.625 bytes, decoded through the
//! width's own plan.
//!
//! The decoder is `bench_bitpack_plan_shared::sum_windowed` at `Plan13`,
//! imported unmodified. It is the fastest packed decode any committed bench
//! in this directory has found: at n = 262144 the decoder-shape run puts it
//! at 43388 ns against the naive index-driven shape's 133317 ns and, notably,
//! against the SIMD arm's 55824 ns
//! (`bitpack-decoder-shape_n262144_findings.md`). Using anything slower here
//! would be benching packing against a strawman of itself.
//!
//! # Safety
//! `sum_windowed`'s contract is that the buffer holds `n * W` bits plus 8
//! bytes of headroom and that `n` is a multiple of `Pack<13>::P`, which is 8.
//! The shared crate reserves 16 bytes of headroom
//! (`bitpack-carrier-shared`'s `PACKED_BYTES`) and every size this arm
//! declares is a multiple of eight, both declaration-time facts about the
//! column rather than runtime conditions.

use bench_bitpack_carrier_shared::{CarrierColumn, Plan13, LOGICAL_BITS};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    CarrierColumn,
    "bitpack-carrier-packed",
    sizes = [16384, 131072, 1048576, 2097152, 4194304, 8388608]
)]
fn run_carrier_packed<const N: usize>(
    input: &<CarrierColumn<N> as Routine>::Input,
    output: &mut <CarrierColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    let bytes = (N * LOGICAL_BITS) / 8 + 16;
    timed! {
        run {
            output.value = unsafe {
                bench_bitpack_plan_shared::sum_windowed::<Plan13>(&input.packed[..bytes], N)
            };
        }
    }
}
