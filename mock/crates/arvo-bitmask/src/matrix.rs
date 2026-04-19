//! Bit-matrix adjacency structures.
//!
//! `BitMatrix64<N>` stores an adjacency matrix for up to 64 nodes as
//! `[Mask64; N]`. `BitMatrix256<N>` is the 256-node variant with
//! `[Mask256; N]` rows. Row `i` has bit `j` set if edge `i -> j`
//! exists.
//!
//! Const-generic `N: usize` rather than `arvo::Cap` — Rust array
//! lengths require `usize` and `Cap` is a wrapper; the conversion
//! lives at the consumer boundary when needed.
//!
//! `transitive_closure` runs Warshall's algorithm over the bitmask
//! rows: for each pivot `k`, every row containing `k` unions in row
//! `k`. Bitmask union is one instruction per word, so the inner loop
//! is cheap.

use arvo::newtype::{Bool, USize};

use crate::mask::{Mask, Mask256, Mask64};
use crate::node::NodeId;

// --- BitMatrix64 ----------------------------------------------------------

/// Adjacency matrix over up to 64 nodes.
///
/// Row `i` (a `Mask64`) has bit `j` set when edge `i -> j` exists.
/// `N` is the row count; bit positions within a row cover up to 64
/// column nodes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitMatrix64<const N: usize> {
    /// Row storage. `rows[i]` is the successor mask of node `i`.
    pub rows: [Mask64; N],
}

impl<const N: usize> BitMatrix64<N> {
    /// Empty matrix (no edges).
    #[inline]
    pub fn empty() -> Self {
        Self { rows: [Mask::<_>::from_word(Default::default()); N] }
    }

    /// `Bool::TRUE` when edge `i -> j` is set.
    #[inline(always)]
    pub fn edge(&self, i: NodeId, j: NodeId) -> Bool {
        let row_idx = (i.0).0;
        if row_idx >= N {
            return Bool::FALSE;
        }
        self.rows[row_idx].contains(j.0)
    }

    /// Set edge `i -> j`. Leaves self unchanged when `i` or `j` is
    /// out of range.
    #[inline(always)]
    pub fn set_edge(&mut self, i: NodeId, j: NodeId) {
        let row_idx = (i.0).0;
        if row_idx >= N {
            return;
        }
        self.rows[row_idx].insert(j.0);
    }

    /// Clear edge `i -> j`. Leaves self unchanged when `i` or `j` is
    /// out of range.
    #[inline(always)]
    pub fn clear_edge(&mut self, i: NodeId, j: NodeId) {
        let row_idx = (i.0).0;
        if row_idx >= N {
            return;
        }
        self.rows[row_idx].remove(j.0);
    }

    /// Successor mask of node `i` (all outgoing edges).
    #[inline(always)]
    pub fn successors(&self, i: NodeId) -> Mask64 {
        let row_idx = (i.0).0;
        if row_idx >= N {
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
        let mut i = 0usize;
        while i < N {
            if *self.rows[i].contains(j.0) {
                out.insert(USize(i));
            }
            i += 1;
        }
        out
    }

    /// Transitive closure via Warshall's algorithm.
    ///
    /// For each pivot `k`, every row that contains `k` unions in
    /// row `k`. Runs in place.
    #[inline]
    pub fn transitive_closure(&mut self) {
        let mut k = 0usize;
        while k < N {
            let row_k = self.rows[k];
            let mut i = 0usize;
            while i < N {
                if *self.rows[i].contains(USize(k)) {
                    self.rows[i] = self.rows[i].union(row_k);
                }
                i += 1;
            }
            k += 1;
        }
    }
}

impl<const N: usize> Default for BitMatrix64<N> {
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
pub struct BitMatrix256<const N: usize> {
    /// Row storage. `rows[i]` is the successor mask of node `i`.
    pub rows: [Mask256; N],
}

impl<const N: usize> BitMatrix256<N> {
    /// Empty matrix (no edges).
    #[inline]
    pub fn empty() -> Self {
        Self { rows: [Mask256::empty(); N] }
    }

    /// `Bool::TRUE` when edge `i -> j` is set.
    #[inline(always)]
    pub fn edge(&self, i: NodeId, j: NodeId) -> Bool {
        let row_idx = (i.0).0;
        if row_idx >= N {
            return Bool::FALSE;
        }
        self.rows[row_idx].contains(j.0)
    }

    /// Set edge `i -> j`. Leaves self unchanged when `i` or `j` is
    /// out of range.
    #[inline(always)]
    pub fn set_edge(&mut self, i: NodeId, j: NodeId) {
        let row_idx = (i.0).0;
        if row_idx >= N {
            return;
        }
        self.rows[row_idx].insert(j.0);
    }

    /// Clear edge `i -> j`. Leaves self unchanged when `i` or `j` is
    /// out of range.
    #[inline(always)]
    pub fn clear_edge(&mut self, i: NodeId, j: NodeId) {
        let row_idx = (i.0).0;
        if row_idx >= N {
            return;
        }
        self.rows[row_idx].remove(j.0);
    }

    /// Successor mask of node `i` (all outgoing edges).
    #[inline(always)]
    pub fn successors(&self, i: NodeId) -> Mask256 {
        let row_idx = (i.0).0;
        if row_idx >= N {
            return Mask256::empty();
        }
        self.rows[row_idx]
    }

    /// Predecessor mask of node `j` (all incoming edges).
    #[inline]
    pub fn predecessors(&self, j: NodeId) -> Mask256 {
        let mut out = Mask256::empty();
        let mut i = 0usize;
        while i < N {
            if *self.rows[i].contains(j.0) {
                out.insert(USize(i));
            }
            i += 1;
        }
        out
    }

    /// Transitive closure via Warshall's algorithm.
    #[inline]
    pub fn transitive_closure(&mut self) {
        let mut k = 0usize;
        while k < N {
            let row_k = self.rows[k];
            let mut i = 0usize;
            while i < N {
                if *self.rows[i].contains(USize(k)) {
                    self.rows[i] = self.rows[i].union(row_k);
                }
                i += 1;
            }
            k += 1;
        }
    }
}

impl<const N: usize> Default for BitMatrix256<N> {
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
    }
}
