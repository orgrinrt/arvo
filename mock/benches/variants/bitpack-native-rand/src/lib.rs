//! Native-typed Dense extraction, random access. See `bitpack-native-seq`
//! for the framing; this is the same reference ceiling under a permuted
//! access order.

use bench_bitpack_shared::{extract_native, Column};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(Column, "bitpack-native-rand", sizes = [256, 4096, 16384])]
fn run_native_rand<const N: usize>(
    input: &<Column<N> as Routine>::Input,
    output: &mut <Column<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            let mut s: u64 = 0;
            for i in 0..N {
                let idx = input.perm[i] as usize;
                s = s.wrapping_add(extract_native(&input.logical, idx) as u64);
            }
            output.value = s;
        }
    }
}
