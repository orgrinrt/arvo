//! `digest-crossover`: `Fnv1a::<64>::of`.
//!
//! The input reaches the digest as a slice whose length the optimiser cannot
//! see, through `black_box`, because the switched digest is called on slices
//! of a length known only at run time. A fixed `[u8; N]` would let the loop
//! unroll to the exact length and time a shape no caller has.

use core::hint::black_box;

use arvo_bits::Bytes;
// FIXME(arvo_format_admits_the_wrapping_product_over_the_unsigned_64): this variant
// builds once `Fnv1a<64>` joins; the `digest-crossover` bench waits on it.
use arvo_digest::Fnv1a;
use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    "digest-fnv1a64",
    sizes = [1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256]
)]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let bytes: &[u8] = black_box(&input[..]);
            *output = Fnv1a::<64>::of(Bytes::of(bytes)).raw().raw().to_le_bytes();
        }
    }
}
