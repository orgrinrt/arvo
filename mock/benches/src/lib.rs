//! Shared bench Routine for the arvo ContentHash bench.
//!
//! Single source of truth for `Fnv1aVsXxHash3<const N: usize>: Routine`
//! consumed by the orchestrator (`main.rs`) and both variant cdylibs
//! (`variants/{fnv1a,xxhash3}/src/lib.rs`). The variants need this
//! type to satisfy the `bench_variant` macro's `Algo<N>: Routine`
//! bound; the orchestrator needs it to build per-N `RoutineBridge`s.

use mockspace_bench_core::Routine;

/// Routine: hash an N-byte input. Variants pick the algorithm.
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
