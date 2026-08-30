//! Packed reading for the footprint bench: the plan-driven decode file 81
//! found optimal for a decode-dominated plain sum (section 4.1, `windowed`,
//! 1.50x against dense at cache-resident sizes), swept across the identical
//! sizes `bitpack-footprint-dense` uses so a footprint saving that only shows
//! up once both layouts leave cache has somewhere to appear. See
//! `bench-bitpack-footprint-shared`.

use bench_bitpack_footprint_shared::{sum_windowed, FootprintColumn, Plan13};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    FootprintColumn,
    "bitpack-footprint-packed",
    sizes = [16384, 65536, 1048576, 4194304, 7000000, 33554432]
)]
fn run_footprint_packed<const N: usize>(
    input: &<FootprintColumn<N> as Routine>::Input,
    output: &mut <FootprintColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            // SAFETY: `packed` carries 16 bytes of headroom past the last
            // group (`MAX_PACKED_BYTES`), and every N this bench declares is
            // a multiple of the width-13 period, 8 (checked in the shared
            // crate's own tests over the full period/group matrix at
            // `bitpack-plan-shared`, reused unmodified here).
            output.value = unsafe { sum_windowed::<Plan13>(&input.packed, N) };
        }
    }
}
