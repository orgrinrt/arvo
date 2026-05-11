//! Bundle 1 bench: structural-decomposition.
//!
//! Times RCM, block_diagonal, and dulmage_mendelsohn over BitMatrix
//! adjacency at several N and graph shapes. Output is a Markdown
//! table consumed by findings_graph_spectral_202605111719.md.
//!
//! Limitations vs the full sweep named in the topic file:
//! - W axis pinned to Bits<64, Hot, Unsigned>. The Bits<256> and
//!   CSR-driven crossovers belong to follow-up bench rounds once
//!   wider BitMatrix containers (or the consumer's chosen W) are
//!   the bench target.
//! - N capped at 64 (the Bits<64> column width). N in {128, 512,
//!   2048} requires Bits<256> / WideBits stagings that are out of
//!   scope for this round's lock criteria.
//! - Out-degree axis collapses to "linear" + "random ~50% density"
//!   shapes. The denser fan-out / layered shapes are deferred.
//!
//! These limitations are the trade for landing real numbers under
//! the round's lock window. The follow-up audit catalogue
//! (round 202605111741) and arvo-graph W-generic sweep cover the
//! axis expansion.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::time::Instant;

use arvo::{Bits, Cap, Hot, USize, Unsigned};
use arvo_bitmask::{BitMatrix, NodeId, cap_size};
use arvo_sparse::{block_diagonal, dulmage_mendelsohn, rcm_reorder};

type W = Bits<64, Hot, Unsigned>;

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

fn nid(i: usize) -> NodeId {
    NodeId::new(USize(i))
}

/// Linear chain: 0 -> 1 -> 2 -> ... -> N-1, undirected.
fn linear_chain<const N: Cap>() -> BitMatrix<W, N>
where
    [(); cap_size(N)]:,
{
    let mut adj: BitMatrix<W, N> = BitMatrix::<W, _>::empty();
    let n = N.0.0;
    for i in 0..(n - 1) {
        adj.set_edge(nid(i), nid(i + 1));
        adj.set_edge(nid(i + 1), nid(i));
    }
    adj
}

/// Deterministic pseudo-random graph (~50% density). LCG over the
/// node-pair index decides each edge; identical across runs.
fn pseudo_random<const N: Cap>() -> BitMatrix<W, N>
where
    [(); cap_size(N)]:,
{
    let mut adj: BitMatrix<W, N> = BitMatrix::<W, _>::empty();
    let n = N.0.0;
    let mut state: u64 = 0x1234_5678_9ABC_DEF0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            if (state >> 32) & 1 == 1 {
                adj.set_edge(nid(i), nid(j));
            }
        }
    }
    adj
}

fn time_micros<F: FnMut()>(mut f: F, iters: u32) -> f64 {
    f();
    let start = Instant::now();
    for _ in 0..iters {
        f();
    }
    let elapsed = start.elapsed();
    elapsed.as_nanos() as f64 / iters as f64 / 1000.0
}

fn run_n<const N: Cap>(label: &str)
where
    [(); cap_size(N)]:,
{
    let n = N.0.0;
    let lin = linear_chain::<N>();
    let rnd = pseudo_random::<N>();
    let iters = 1000;

    let rcm_lin = time_micros(|| {
        let r = rcm_reorder(&lin);
        std::hint::black_box(&r);
    }, iters);
    let rcm_rnd = time_micros(|| {
        let r = rcm_reorder(&rnd);
        std::hint::black_box(&r);
    }, iters);

    let blk_lin = time_micros(|| {
        let r = block_diagonal(&lin);
        std::hint::black_box(&r);
    }, iters);
    let blk_rnd = time_micros(|| {
        let r = block_diagonal(&rnd);
        std::hint::black_box(&r);
    }, iters);

    let dm_lin = time_micros(|| {
        let r = dulmage_mendelsohn(&lin);
        std::hint::black_box(&r);
    }, iters);
    let dm_rnd = time_micros(|| {
        let r = dulmage_mendelsohn(&rnd);
        std::hint::black_box(&r);
    }, iters);

    println!(
        "| {label} (N={n}) | {rcm_lin:>8.3} | {rcm_rnd:>8.3} | {blk_lin:>8.3} | {blk_rnd:>8.3} | {dm_lin:>8.3} | {dm_rnd:>8.3} |"
    );
}

fn main() {
    println!("# Bundle 1: structural-decomposition\n");
    println!("Microseconds per call, mean of 1000 iterations.");
    println!("W = Bits<64, Hot, Unsigned>. Shapes: linear chain vs ~50% random.\n");
    println!("| Variant | RCM (lin) | RCM (rnd) | Blk (lin) | Blk (rnd) | DM (lin) | DM (rnd) |");
    println!("|---|---:|---:|---:|---:|---:|---:|");

    run_n::<{ cap(16) }>("structural-decomposition");
    run_n::<{ cap(32) }>("structural-decomposition");
    run_n::<{ cap(64) }>("structural-decomposition");
}
