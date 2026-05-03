//! Dirty propagation over bit-matrix adjacency.
//!
//! `propagate_dirty` fix-points a dirty mask through a DAG adjacency
//! matrix. For every set bit `i` in `dirty`, the successor mask of
//! `i` is OR'd in. The pass repeats until `dirty` stops growing.
//!
//! Termination: `dirty` only grows, and it is bounded by the mask's
//! width. Each pass either adds at least one bit (loop continues) or
//! adds nothing (loop exits). Upper bound is `width` iterations.
//!
//! Round 202605031748 (#313) collapsed the parallel
//! `propagate_dirty_64` / `propagate_dirty_256` shipping shapes onto
//! this single generic chassis. Free function, not a method, so
//! consumers can call it from any matrix instantiation without
//! importing a trait.

use arvo::Cap;
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};

use crate::mask::Mask;
use crate::matrix::{BitMatrix, cap_size};
use crate::node::NodeId;

/// Propagate dirty bits through a `BitMatrix<W, N>` adjacency matrix.
///
/// For each set bit `i` in `dirty`, union in `matrix.successors(i)`.
/// Repeat until no change.
#[inline]
pub fn propagate_dirty<W, const N: Cap>(matrix: &BitMatrix<W, N>, dirty: &mut Mask<W>)
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default + PartialEq,
    [(); cap_size(N)]:,
{
    loop {
        let before = *dirty;
        let snapshot = before;
        for i in snapshot.iter_set_bits() {
            let row = if i.0 < cap_size(N) {
                matrix.successors(NodeId(i))
            } else {
                Mask::<W>::from_word(W::default())
            };
            *dirty = dirty.union(row);
        }
        if *dirty == before {
            return;
        }
    }
}
