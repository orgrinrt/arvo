//! `digest-crossover`: `Xxh3::<64>::of`, xxhash-rust's runtime path.
//!
//! The slice goes through `black_box` for the reason the FNV-1a arm gives.

use core::hint::black_box;

use arvo_bits::Bytes;
use arvo_digest::Xxh3;
use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    "digest-xxh3-64",
    sizes = [1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256]
)]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let bytes: &[u8] = black_box(&input[..]);
            *output = Xxh3::<64>::of(Bytes::of(bytes)).raw().raw().to_le_bytes();
        }
    }
}
