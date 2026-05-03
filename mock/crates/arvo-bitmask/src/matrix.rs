//! Bit-matrix adjacency structures.
//!
//! `BitMatrix<W, const N: Cap>` stores an adjacency matrix as
//! `[Mask<W>; cap_size(N)]`. Row `i` (a `Mask<W>`) has bit `j` set
//! when edge `i -> j` exists. `N: Cap` carries arvo's const-generic
//! capacity newtype on the public surface; `cap_size(c: Cap) ->
//! usize` unwraps for array sizing.
//!
//! Round 202605031748 (#313) collapsed the prior parallel
//! `BitMatrix64<N>` and `BitMatrix256<N>` structs onto this single
//! generic chassis. The substrate's `BitsContainerFor` projection
//! plus the `BitPrim` impls on `WideBits` (round 3 substrate side)
//! make the chassis work uniformly across W up to 256 bits.

use arvo::{Bool, Cap, USize};
pub use arvo_tensor::cap_size;
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};

use crate::mask::Mask;
use crate::node::NodeId;

/// Generic adjacency matrix chassis.
///
/// Row `i` (a `Mask<W>`) has bit `j` set when edge `i -> j` exists.
/// `N` is the row count; bit positions within a row cover up to
/// `<W as HasBitWidth>::WIDTH` column nodes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitMatrix<W, const N: Cap>
where
    W: BitSequence + BitAccess + Copy + Default,
    [(); cap_size(N)]:,
{
    /// Row storage. `rows[i]` is the successor mask of node `i`.
    pub rows: [Mask<W>; cap_size(N)],
}

impl<W, const N: Cap> BitMatrix<W, N>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
    [(); cap_size(N)]:,
{
    /// Empty matrix (no edges).
    #[inline]
    pub fn empty() -> Self {
        Self {
            rows: [Mask::<W>::from_word(W::default()); cap_size(N)],
        }
    }

    /// `Bool::TRUE` when edge `i -> j` is set.
    #[inline(always)]
    pub fn edge(&self, i: NodeId, j: NodeId) -> Bool {
        let row_idx = (i.0).0;
        if row_idx >= cap_size(N) {
            return Bool::FALSE;
        }
        self.rows[row_idx].contains(j.0)
    }

    /// Set edge `i -> j`. Leaves self unchanged when `i` or `j` is
    /// out of range.
    #[inline(always)]
    pub fn set_edge(&mut self, i: NodeId, j: NodeId) {
        let row_idx = (i.0).0;
        if row_idx >= cap_size(N) {
            return;
        }
        self.rows[row_idx].insert(j.0);
    }

    /// Clear edge `i -> j`. Leaves self unchanged when `i` or `j` is
    /// out of range.
    #[inline(always)]
    pub fn clear_edge(&mut self, i: NodeId, j: NodeId) {
        let row_idx = (i.0).0;
        if row_idx >= cap_size(N) {
            return;
        }
        self.rows[row_idx].remove(j.0);
    }

    /// Successor mask of node `i` (all outgoing edges).
    #[inline(always)]
    pub fn successors(&self, i: NodeId) -> Mask<W> {
        let row_idx = (i.0).0;
        if row_idx >= cap_size(N) {
            return Mask::<W>::from_word(W::default());
        }
        self.rows[row_idx]
    }

    /// Predecessor mask of node `j` (all incoming edges).
    ///
    /// Scans each row and tests column `j`; sets bit `i` in the
    /// result whenever `rows[i].contains(j)`.
    #[inline]
    pub fn predecessors(&self, j: NodeId) -> Mask<W> {
        let mut out = Mask::<W>::from_word(W::default());
        for i in 0..cap_size(N) {
            if *self.rows[i].contains(j.0) {
                out.insert(USize(i));
            }
        }
        out
    }

    /// Transitive closure via Warshall's algorithm.
    ///
    /// For each pivot `k`, every row that contains `k` unions in
    /// row `k`. Runs in place.
    #[inline]
    pub fn transitive_closure(&mut self) {
        for k in 0..cap_size(N) {
            let row_k = self.rows[k];
            for i in 0..cap_size(N) {
                if *self.rows[i].contains(USize(k)) {
                    self.rows[i] = self.rows[i].union(row_k);
                }
            }
        }
    }
}

impl<W, const N: Cap> Default for BitMatrix<W, N>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
    [(); cap_size(N)]:,
{
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
    }
}
