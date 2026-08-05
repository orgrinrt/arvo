//! Second packed variant, existing only so `bitpack-footprint-packed` has a
//! real cross-validation partner (see `bitpack-footprint-dense-alt` for why a
//! second variant is necessary rather than a nicety: the harness never runs
//! `validation::validate` on a single-variant section at all). This one is
//! also a genuine second data point: `sum_naive`, file 75's index-driven
//! decode (offset and shift recomputed from the running index every element,
//! no plan carried in associated consts), against `sum_windowed`'s
//! plan-driven decode, at the sizes this bench actually sweeps rather than
//! the cache-resident sizes file 81 measured it at. See
//! `bench-bitpack-footprint-shared`.

use bench_bitpack_footprint_shared::{sum_naive, FootprintColumn, LOGICAL_BITS};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    FootprintColumn,
    "bitpack-footprint-packed-naive",
    sizes = [16384, 65536, 1048576, 4194304, 7000000, 33554432]
)]
fn run_footprint_packed_naive<const N: usize>(
    input: &<FootprintColumn<N> as Routine>::Input,
    output: &mut <FootprintColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            let packed_bytes = (N * LOGICAL_BITS) / 8 + 16;
            output.value = sum_naive(&input.packed[.. packed_bytes], N);
        }
    }
}
