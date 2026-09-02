//! Dense read at a 32-bit carrier: 4 bytes per element, 2.375 saved by
//! packing. The middle rung, and on the arithmetic the one whose answer is
//! closest to the edge on this host.

use bench_bitpack_carrier_shared::{sum_d32, CarrierColumn};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    CarrierColumn,
    "bitpack-carrier-d32",
    sizes = [16384, 131072, 1048576, 2097152, 4194304, 8388608]
)]
fn run_carrier_d32<const N: usize>(
    input: &<CarrierColumn<N> as Routine>::Input,
    output: &mut <CarrierColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = sum_d32(&input.d32, N);
        }
    }
}
