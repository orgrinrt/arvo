//! FNV1a variant for the arvo ContentHash bench.

use arvo_hash::{Fnv1a, HasherExt};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

/// Local copy of the Routine type. Each variant cdylib carries its own
/// because the `bench_variant` macro requires `Algo<N>: Routine` at the
/// const-generic dispatch point. Identical impl across variants.
pub struct Fnv1aVsXxHash3<const N: usize>;

impl<const N: usize> Routine for Fnv1aVsXxHash3<N> {
    type Input = [u8; N];
    type Output = u64;

    fn build_input(seed: u64) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut x = seed.wrapping_mul(0x9E3779B97F4A7C15);
        for chunk in buf.chunks_mut(8) {
            x ^= x >> 30;
            x = x.wrapping_mul(0xBF58476D1CE4E5B9);
            let bytes = x.to_le_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len()]);
        }
        buf
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        N as u64
    }

    fn outputs_may_differ() -> bool {
        true
    }
}

#[bench_variant(Fnv1aVsXxHash3, "fnv1a", sizes = [64, 256, 1024, 4096])]
fn run_fnv1a<const N: usize>(input: &[u8; N], output: &mut u64) -> FfiBenchCall {
    timed! {
        run { *output = Fnv1a::<64>::new().hash(input).to_raw(); }
    }
}
