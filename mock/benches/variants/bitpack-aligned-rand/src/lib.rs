//! Byte-aligned-slot extraction, random access. See `bench-bitpack-shared`
//! for the `Column<N>` Routine type, the shared `extract_aligned` transform,
//! and `build_input`'s permutation (deterministic per seed, built once
//! outside the timed region).

use bench_bitpack_shared::{extract_aligned, Column};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(Column, "bitpack-aligned-rand", sizes = [256, 4096, 16384])]
fn run_aligned_rand<const N: usize>(
    input: &<Column<N> as Routine>::Input,
    output: &mut <Column<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            let mut s: u64 = 0;
            for i in 0..N {
                let idx = input.perm[i] as usize;
                s = s.wrapping_add(extract_aligned(&input.aligned, idx) as u64);
            }
            output.value = s;
        }
    }
}
