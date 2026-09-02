//! Bundle 1 (proper harness form): structural-decomposition Routine
//! + bench_variant cdylib for `rcm_reorder`.
//!
//! The capacity-as-type migration replaced `rcm_reorder`'s `const N:
//! Cap` parameter with `C: Capacity`. The backing array is now the GAT
//! `C::Array<NodeId>` (`[NodeId; N]` for `Dim<N>`), so no `cap_size`
//! expression sits in type position and the prior named-`Cap`-constant
//! ICE workaround is gone: each `run` call site instantiates the
//! algorithm at `Dim<N>` directly.
//!
//! The Routine's Input/Output stay usize-parameterised so the
//! mockspace `bench_variant` macro (which emits `usize` literals)
//! works unchanged. Input carries raw `[u64; N]` bit storage; the
//! variant reconstructs `BitMatrix<W, Dim<N>>` per-N at call time.

#![no_std]

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_sparse::rcm_reorder;
use arvo_tensor::Dim;
use mockspace_bench_core::{FfiBenchCall, Routine, timed};
use mockspace_bench_macro::bench_variant;

pub type W = Bits<64, Hot, Unsigned>;

/// FFI-safe input: raw row-bit storage. The variant converts to
/// `BitMatrix<W, Dim<N>>` per-N at call time.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct RcmInput<const N: usize> {
    pub rows: [u64; N],
}

impl<const N: usize> Default for RcmInput<N> {
    fn default() -> Self {
        Self { rows: [0u64; N] }
    }
}

pub struct Rcm<const N: usize>;

impl<const N: usize> Routine for Rcm<N> {
    type Input = RcmInput<N>;
    type Output = [u32; N];

    fn build_input(seed: u64) -> Self::Input {
        let mut rows = [0u64; N];
        let mut state = seed.wrapping_add(0x1234_5678_9ABC_DEF0);
        for i in 0..N {
            for j in 0..N {
                if i == j {
                    continue;
                }
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                if (state >> 32) & 1 == 1 {
                    rows[i] |= 1u64 << j;
                }
            }
        }
        RcmInput { rows }
    }

    fn validate_output(
        _input: &Self::Input,
        output: &Self::Output,
    ) -> Result<(), &'static str> {
        let mut seen = [false; N];
        for v in output.iter() {
            let idx = *v as usize;
            if idx >= N {
                return Err("output element out of range 0..N");
            }
            if seen[idx] {
                return Err("duplicate element in output permutation");
            }
            seen[idx] = true;
        }
        Ok(())
    }
}

/// Per-N safe dispatch trait. Each supported size implements this with
/// a body that instantiates the arvo algorithm at the corresponding
/// named `Cap` constant. The variant fn then dispatches via
/// `<Rcm<N> as RcmDispatch>::run(input, output)`, propagating the
/// `Rcm<N>: RcmDispatch` bound through the `bench_variant` macro
/// expansion. The compiler resolves the impl per literal `N` the
/// macro emits, so no runtime match and no unsafe pointer casts.
pub trait RcmDispatch: Routine {
    fn run(input: &Self::Input, output: &mut Self::Output);
}

impl RcmDispatch for Rcm<16> {
    #[inline(never)]
    fn run(input: &RcmInput<16>, output: &mut [u32; 16]) {
        let mut adj: BitMatrix<W, Dim<16>> = BitMatrix::<W, _>::empty();
        for i in 0..16 {
            for j in 0..16 {
                if (input.rows[i] >> j) & 1 == 1 {
                    adj.set_edge(NodeId::new(USize(i)), NodeId::new(USize(j)));
                }
            }
        }
        let perm = rcm_reorder(&adj);
        for i in 0..16 {
            output[i] = perm[i].0.0 as u32;
        }
    }
}

impl RcmDispatch for Rcm<32> {
    #[inline(never)]
    fn run(input: &RcmInput<32>, output: &mut [u32; 32]) {
        let mut adj: BitMatrix<W, Dim<32>> = BitMatrix::<W, _>::empty();
        for i in 0..32 {
            for j in 0..32 {
                if (input.rows[i] >> j) & 1 == 1 {
                    adj.set_edge(NodeId::new(USize(i)), NodeId::new(USize(j)));
                }
            }
        }
        let perm = rcm_reorder(&adj);
        for i in 0..32 {
            output[i] = perm[i].0.0 as u32;
        }
    }
}

impl RcmDispatch for Rcm<64> {
    #[inline(never)]
    fn run(input: &RcmInput<64>, output: &mut [u32; 64]) {
        let mut adj: BitMatrix<W, Dim<64>> = BitMatrix::<W, _>::empty();
        for i in 0..64 {
            for j in 0..64 {
                if (input.rows[i] >> j) & 1 == 1 {
                    adj.set_edge(NodeId::new(USize(i)), NodeId::new(USize(j)));
                }
            }
        }
        let perm = rcm_reorder(&adj);
        for i in 0..64 {
            output[i] = perm[i].0.0 as u32;
        }
    }
}

#[bench_variant(Rcm, "rcm-bits64", sizes = [16, 32, 64])]
fn rcm_variant<const N: usize>(
    input: &<Rcm<N> as Routine>::Input,
    output: &mut <Rcm<N> as Routine>::Output,
) -> FfiBenchCall
where
    Rcm<N>: RcmDispatch,
{
    timed! {
        run {
            <Rcm<N> as RcmDispatch>::run(input, output);
        }
    }
}
