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
//! Two overloads exist: one for `(BitMatrix64<N>, Mask64)` and one
//! for `(BitMatrix256<N>, Mask256)`. Free functions, not methods,
//! so consumers can call them directly from either matrix type
//! without importing a trait.

use arvo::{Cap, USize};

use crate::mask::{Mask256, Mask64};
use crate::matrix::{BitMatrix256, BitMatrix64, cap_size};
use crate::node::NodeId;

/// Propagate dirty bits through a 64-wide adjacency matrix.
///
/// For each set bit `i` in `dirty`, union in `matrix.successors(i)`.
/// Repeat until no change.
#[inline]
pub fn propagate_dirty_64<const N: Cap>(matrix: &BitMatrix64<N>, dirty: &mut Mask64)
where
    [(); cap_size(N)]:,
{
    loop {
        let before = *dirty;
        let snapshot = before;
        for i in snapshot.iter_set_bits() {
            let row = if i.0 < cap_size(N) {
                matrix.successors(NodeId(USize(i.0)))
            } else {
                Mask64::empty()
            };
            *dirty = dirty.union(row);
        }
        if *dirty == before {
            return;
        }
    }
}

/// Propagate dirty bits through a 256-wide adjacency matrix.
///
/// Same algorithm as `propagate_dirty_64`, iterating across the four
/// 64-bit words of `Mask256`.
#[inline]
pub fn propagate_dirty_256<const N: Cap>(matrix: &BitMatrix256<N>, dirty: &mut Mask256)
where
    [(); cap_size(N)]:,
{
    loop {
        let before = *dirty;
        let snapshot = before;
        for i in snapshot.iter_set_bits() {
            let row = if i.0 < cap_size(N) {
                matrix.successors(NodeId(USize(i.0)))
            } else {
                Mask256::empty()
            };
            *dirty = dirty.union(row);
        }
        if *dirty == before {
            return;
        }
    }
}
