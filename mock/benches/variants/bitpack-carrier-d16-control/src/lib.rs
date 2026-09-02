//! Noise-floor control. Calls the identical `sum_d16` on the identical
//! region with the identical arguments as `bitpack-carrier-d16`, so the two
//! arms differ only in the exported symbol name and must compile to the same
//! machine code. Any measured gap between them is the harness's own
//! resolution on this workload, and every other delta in the run is read
//! against it. The byte-identity is not assumed: `26_probes/control_identity.sh`
//! extracts both function bodies from the two built dylibs and diffs them.

use bench_bitpack_carrier_shared::{sum_d16, CarrierColumn};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    CarrierColumn,
    "bitpack-carrier-d16-control",
    sizes = [16384, 131072, 1048576, 2097152, 4194304, 8388608]
)]
fn run_carrier_d16_control<const N: usize>(
    input: &<CarrierColumn<N> as Routine>::Input,
    output: &mut <CarrierColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = sum_d16(&input.d16, N);
        }
    }
}
