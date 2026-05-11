//! Bundle 1 (proper harness form): structural-decomposition Routine
//! + bench_variant cdylib for `rcm_reorder`.
//!
//! Sidesteps a rustc ICE during const-evaluation of `rcm_reorder`'s
//! internal `cap_size(N)` bound when N comes from a const-fn
//! application (`cap_of(N_usize)`). The workaround uses **named
//! Cap constants** per supported N. Each `run_rcm_at_*` call site
//! passes a literal `Cap` constant, not a const-fn expression, so
//! rustc evaluates `cap_size(C16)` etc. directly without trigger.
//!
//! The Routine's Input/Output stay usize-parameterised so the
//! mockspace `bench_variant` macro (which emits `usize` literals)
//! works unchanged. Input carries raw `[u64; N]` bit storage; the
//! variant reconstructs `BitMatrix<W, C>` per-N at call time.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Bits, Cap, Hot, USize, Unsigned};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_sparse::rcm_reorder;
use mockspace_bench_core::{FfiBenchCall, Routine, timed};
use mockspace_bench_macro::bench_variant;

pub type W = Bits<64, Hot, Unsigned>;

const C16: Cap = Cap(USize(16));
const C32: Cap = Cap(USize(32));
const C64: Cap = Cap(USize(64));

/// FFI-safe input: raw row-bit storage. The variant converts to
/// `BitMatrix<W, C>` per-N at call time.
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

#[inline(never)]
fn run_at_c16(input: &RcmInput<16>, output: &mut [u32; 16]) {
    let mut adj: BitMatrix<W, C16> = BitMatrix::<W, _>::empty();
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

#[inline(never)]
fn run_at_c32(input: &RcmInput<32>, output: &mut [u32; 32]) {
    let mut adj: BitMatrix<W, C32> = BitMatrix::<W, _>::empty();
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

#[inline(never)]
fn run_at_c64(input: &RcmInput<64>, output: &mut [u32; 64]) {
    let mut adj: BitMatrix<W, C64> = BitMatrix::<W, _>::empty();
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

#[bench_variant(Rcm, "rcm-bits64", sizes = [16, 32, 64])]
fn rcm_variant<const N: usize>(
    input: &<Rcm<N> as Routine>::Input,
    output: &mut <Rcm<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            match N {
                16 => {
                    let input16: &RcmInput<16> = unsafe { &*(input as *const _ as *const RcmInput<16>) };
                    let output16: &mut [u32; 16] = unsafe { &mut *(output as *mut _ as *mut [u32; 16]) };
                    run_at_c16(input16, output16);
                }
                32 => {
                    let input32: &RcmInput<32> = unsafe { &*(input as *const _ as *const RcmInput<32>) };
                    let output32: &mut [u32; 32] = unsafe { &mut *(output as *mut _ as *mut [u32; 32]) };
                    run_at_c32(input32, output32);
                }
                64 => {
                    let input64: &RcmInput<64> = unsafe { &*(input as *const _ as *const RcmInput<64>) };
                    let output64: &mut [u32; 64] = unsafe { &mut *(output as *mut _ as *mut [u32; 64]) };
                    run_at_c64(input64, output64);
                }
                _ => unreachable!(),
            }
        }
    }
}
