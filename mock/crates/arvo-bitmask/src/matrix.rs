//! Bit-matrix adjacency structures.
//!
//! `BitMatrix64<N>` stores an adjacency matrix for up to 64 nodes as
//! `[Mask64; cap_size(N)]`. `BitMatrix256<N>` is the 256-node variant
//! with `[Mask256; cap_size(N)]` rows. Row `i` has bit `j` set if
//! edge `i -> j` exists.
//!
//! `N: Cap` carries arvo's const-generic capacity newtype on the
//! public surface; `cap_size(c: Cap) -> usize` unwraps for array
//! sizing. Canonical `cap_size` lives in `arvo-tensor` as of round
//! 202604280000; this module re-exports it so existing
//! `arvo_bitmask::cap_size` / `crate::matrix::cap_size` import paths
//! keep working.
//!
//! `transitive_closure` runs Warshall's algorithm over the bitmask
//! rows: for each pivot `k`, every row containing `k` unions in row
//! `k`. Bitmask union is one instruction per word, so the inner loop
//! is cheap.

use arvo::{Bool, Cap, USize};
pub use arvo_tensor::cap_size;

use crate::mask::{Mask, Mask256, Mask64};
use crate::node::NodeId;
// --- BitMatrix64 ----------------------------------------------------------

/// Adjacency matrix over up to 64 nodes.
///
/// Row `i` (a `Mask64`) has bit `j` set when edge `i -> j` exists.
/// `N` is the row count; bit positions within a row cover up to 64
/// column nodes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitMatrix64<const N: Cap>
where
    [(); cap_size(N)]:,
{
    /// Row storage. `rows[i]` is the successor mask of node `i`.
    pub rows: [Mask64; cap_size(N)],
}

impl<const N: Cap> BitMatrix64<N>
where
    [(); cap_size(N)]:,
{
    /// Empty matrix (no edges).
    #[inline]
    pub fn empty() -> Self {
        Self {
            rows: [Mask::<_>::from_word(Default::default()); cap_size(N)],
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
    pub fn successors(&self, i: NodeId) -> Mask64 {
        let row_idx = (i.0).0;
        if row_idx >= cap_size(N) {
            return Mask64::empty();
        }
        self.rows[row_idx]
    }

    /// Predecessor mask of node `j` (all incoming edges).
    ///
    /// Scans each row and tests column `j`; sets bit `i` in the
    /// result whenever `rows[i].contains(j)`.
    #[inline]
    pub fn predecessors(&self, j: NodeId) -> Mask64 {
        let mut out = Mask64::empty();
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

impl<const N: Cap> Default for BitMatrix64<N>
where
    [(); cap_size(N)]:,
{
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
    }
}

// --- BitMatrix256 ---------------------------------------------------------

/// Adjacency matrix over up to 256 nodes.
///
/// Row `i` (a `Mask256`) has bit `j` set when edge `i -> j` exists.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitMatrix256<const N: Cap>
where
    [(); cap_size(N)]:,
{
    /// Row storage. `rows[i]` is the successor mask of node `i`.
    pub rows: [Mask256; cap_size(N)],
}

impl<const N: Cap> BitMatrix256<N>
where
    [(); cap_size(N)]:,
{
    /// Empty matrix (no edges).
    #[inline]
    pub fn empty() -> Self {
        Self {
            rows: [Mask256::empty(); cap_size(N)],
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
    pub fn successors(&self, i: NodeId) -> Mask256 {
        let row_idx = (i.0).0;
        if row_idx >= cap_size(N) {
            return Mask256::empty();
        }
        self.rows[row_idx]
    }

    /// Predecessor mask of node `j` (all incoming edges).
    #[inline]
    pub fn predecessors(&self, j: NodeId) -> Mask256 {
        let mut out = Mask256::empty();
        for i in 0..cap_size(N) {
            if *self.rows[i].contains(j.0) {
                out.insert(USize(i));
            }
        }
        out
    }

    /// Transitive closure via Warshall's algorithm.
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

impl<const N: Cap> Default for BitMatrix256<N>
where
    [(); cap_size(N)]:,
{
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
    }
}
