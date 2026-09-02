//! Dense read at a 64-bit carrier: 8 bytes per element, the widest
//! alternative a consumer plausibly reaches for when the logical field is 13
//! bits. This is the arm packing has the most to win against: 6.375 bytes
//! saved per element. Transform is `sum_d64` from the shared crate, the one
//! definition, called here and nowhere else re-derived.

use bench_bitpack_carrier_shared::{sum_d64, CarrierColumn};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    CarrierColumn,
    "bitpack-carrier-d64",
    sizes = [16384, 131072, 1048576, 2097152, 4194304, 8388608]
)]
fn run_carrier_d64<const N: usize>(
    input: &<CarrierColumn<N> as Routine>::Input,
    output: &mut <CarrierColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = sum_d64(&input.d64, N);
        }
    }
}
