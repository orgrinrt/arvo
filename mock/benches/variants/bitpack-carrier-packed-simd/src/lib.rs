//! The packed arm with the reduction attacked.
//!
//! Same 13-bit packed region and same group decode as `bitpack-carrier-packed`,
//! differing only in how the eight decoded lanes are accumulated:
//! `sum_simd_padal` folds them with one `UADALP` into 32-bit lanes and drains
//! to 64 bits once per 262,160 groups, against the six-instruction widening
//! chain the committed `sum_simd` pays every group. See the kernel's own doc
//! comment in `bench-bitpack-carrier-shared` for the disassembly that located
//! the cost.
//!
//! This arm exists so the packed side of the sweep is not represented by a
//! decoder that a reader could reasonably suspect of being the bottleneck.
//! If packing loses even with its best kernel, the loss is about packing.

use bench_bitpack_carrier_shared::{sum_simd_padal, CarrierColumn, Plan13, LOGICAL_BITS};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    CarrierColumn,
    "bitpack-carrier-packed-simd",
    sizes = [16384, 131072, 1048576, 2097152, 4194304, 8388608]
)]
fn run_carrier_packed_simd<const N: usize>(
    input: &<CarrierColumn<N> as Routine>::Input,
    output: &mut <CarrierColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    let bytes = (N * LOGICAL_BITS) / 8 + 16;
    timed! {
        run {
            // SAFETY: the packed region carries 16 bytes of read headroom and
            // every declared size is a multiple of the width-13 period.
            output.value = unsafe { sum_simd_padal::<Plan13>(&input.packed[..bytes], N) };
        }
    }
}
