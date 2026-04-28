//! xxHash3 variant for the arvo ContentHash bench.

use arvo_benches::Fnv1aVsXxHash3;
use arvo_hash::{HasherExt, XxHash3};
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant(Fnv1aVsXxHash3, "xxhash3", sizes = [64, 256, 1024, 4096])]
fn run_xxhash3<const N: usize>(input: &[u8; N], output: &mut u64) -> FfiBenchCall {
    timed! {
        run { *output = XxHash3::<64>::new().hash(input).to_raw(); }
    }
}
