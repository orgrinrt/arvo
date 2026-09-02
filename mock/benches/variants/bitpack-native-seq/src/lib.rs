//! Native-typed Dense extraction, sequential access: the reference ceiling
//! this whole bench measures the two `Layout::Bitpacked` readings against.
//! `input.logical` is already the natively-typed `[u16; _]` carrier array
//! (see `bench-bitpack-shared`'s `extract_native`); this is what
//! `Layout::Dense` at a native register width actually ships as, not the
//! byte-buffer stand-in `bitpack-aligned-seq` uses to keep both readings on
//! comparable byte-addressed infrastructure.

use bench_bitpack_shared::{extract_native, Column};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(Column, "bitpack-native-seq", sizes = [256, 4096, 16384])]
fn run_native_seq<const N: usize>(
    input: &<Column<N> as Routine>::Input,
    output: &mut <Column<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            let mut s: u64 = 0;
            for i in 0..N {
                s = s.wrapping_add(extract_native(&input.logical, i) as u64);
            }
            output.value = s;
        }
    }
}
