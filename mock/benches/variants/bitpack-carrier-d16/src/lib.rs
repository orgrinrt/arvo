//! Dense read at a 16-bit carrier: 2 bytes per element, the tightest
//! native carrier a 13-bit value can have and the only one every prior
//! bitpack bench in this directory measures against. Kept in the sweep so it
//! contains the already-measured point and can be checked against them.

use bench_bitpack_carrier_shared::{sum_d16, CarrierColumn};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    CarrierColumn,
    "bitpack-carrier-d16",
    sizes = [16384, 131072, 1048576, 2097152, 4194304, 8388608]
)]
fn run_carrier_d16<const N: usize>(
    input: &<CarrierColumn<N> as Routine>::Input,
    output: &mut <CarrierColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = sum_d16(&input.d16, N);
        }
    }
}
