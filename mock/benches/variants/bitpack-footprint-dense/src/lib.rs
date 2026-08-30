//! Dense floor for the footprint bench: sequential sum over the native `u16`
//! carrier, swept across sizes that clear this host's L2 rather than sitting
//! inside its L1 the way every prior bitpack sweep in this directory did. See
//! `bench-bitpack-footprint-shared`.

use bench_bitpack_footprint_shared::{sum_native, FootprintColumn};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    FootprintColumn,
    "bitpack-footprint-dense",
    sizes = [16384, 65536, 1048576, 4194304, 7000000, 33554432]
)]
fn run_footprint_dense<const N: usize>(
    input: &<FootprintColumn<N> as Routine>::Input,
    output: &mut <FootprintColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = sum_native(&input.logical, N);
        }
    }
}
