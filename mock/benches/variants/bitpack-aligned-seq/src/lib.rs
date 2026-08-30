//! Byte-aligned-slot extraction, sequential access. See `bench-bitpack-shared`
//! for the `Column<N>` Routine type and the shared `extract_aligned` transform.

use bench_bitpack_shared::{extract_aligned, Column};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(Column, "bitpack-aligned-seq", sizes = [256, 4096, 16384])]
fn run_aligned_seq<const N: usize>(
    input: &<Column<N> as Routine>::Input,
    output: &mut <Column<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            let mut s: u64 = 0;
            for i in 0..N {
                s = s.wrapping_add(extract_aligned(&input.aligned, i) as u64);
            }
            output.value = s;
        }
    }
}
