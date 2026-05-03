//! FNV1a variant for the arvo ContentHash bench.

use arvo::Hot;
use arvo::strategy::Unsigned;
use arvo_hash::{ConstHash, Fnv1a};
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("fnv1a", sizes = [64, 256, 1024, 4096])]
fn run_fnv1a<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            *output = <Fnv1a<64> as ConstHash<64, Hot, Unsigned>>::hash_const(input)
                .to_raw()
                .to_le_bytes();
        }
    }
}
