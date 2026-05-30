//! Bit-matrix adjacency structures.
//!
//! `BitMatrix<W, C: Capacity>` stores an adjacency matrix as the capacity's
//! backing array `C::Array<Mask<W>>`. Row `i` (a `Mask<W>`) has bit `j` set
//! when edge `i -> j` exists. `C` is a `Capacity` type carrying the row count;
//! the capacity is a type, so no `cap_size` expression sits in type position.
//! A body that needs the row count as a value reads `cap_size(C::CAP)`.
//!
//! Round 202605031748 (#313) collapsed the prior parallel
//! `BitMatrix64<N>` and `BitMatrix256<N>` structs onto this single
//! generic chassis. The substrate's `BitsContainerFor` projection
//! plus the `BitPrim` impls on `WideBits` (round 3 substrate side)
//! make the chassis work uniformly across W up to 256 bits.

use arvo::{Bool, USize};
pub use arvo_tensor::cap_size;
use arvo_tensor::Capacity;
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};

use crate::mask::Mask;
use crate::node::NodeId;

/// Generic adjacency matrix chassis.
///
/// Row `i` (a `Mask<W>`) has bit `j` set when edge `i -> j` exists. `C`'s
/// capacity is the row count; bit positions within a row cover up to
/// `<W as HasBitWidth>::WIDTH` column nodes. The per-row bit-width `W` is a
/// separate axis from the row-count capacity `C`.
pub struct BitMatrix<W, C: Capacity>
where
    W: BitSequence + BitAccess + Copy + Default,
{
    /// Row storage. `rows[i]` is the successor mask of node `i`.
    pub rows: C::Array<Mask<W>>,
}

impl<W, C: Capacity> BitMatrix<W, C>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
{
    /// Empty matrix (no edges).
    #[inline]
    pub fn empty() -> Self {
        Self {
            rows: C::filled(Mask::<W>::from_word(W::default())),
        }
    }

    /// `Bool::TRUE` when edge `i -> j` is set.
    #[inline(always)]
    pub fn edge(&self, i: NodeId, j: NodeId) -> Bool {
        let row_idx = (i.0).0;
        if row_idx >= cap_size(C::CAP) {
            return Bool::FALSE;
        }
        self.rows.as_ref()[row_idx].contains(j.0)
    }

    /// Set edge `i -> j`. Leaves self unchanged when `i` or `j` is
    /// out of range.
    #[inline(always)]
    pub fn set_edge(&mut self, i: NodeId, j: NodeId) {
        let row_idx = (i.0).0;
        if row_idx >= cap_size(C::CAP) {
            return;
        }
        self.rows.as_mut()[row_idx].insert(j.0);
    }

    /// Clear edge `i -> j`. Leaves self unchanged when `i` or `j` is
    /// out of range.
    #[inline(always)]
    pub fn clear_edge(&mut self, i: NodeId, j: NodeId) {
        let row_idx = (i.0).0;
        if row_idx >= cap_size(C::CAP) {
            return;
        }
        self.rows.as_mut()[row_idx].remove(j.0);
    }

    /// Successor mask of node `i` (all outgoing edges).
    #[inline(always)]
    pub fn successors(&self, i: NodeId) -> Mask<W> {
        let row_idx = (i.0).0;
        if row_idx >= cap_size(C::CAP) {
            return Mask::<W>::from_word(W::default());
        }
        self.rows.as_ref()[row_idx]
    }

    /// Predecessor mask of node `j` (all incoming edges).
    ///
    /// Scans each row and tests column `j`; sets bit `i` in the
    /// result whenever `rows[i].contains(j)`.
    #[inline]
    pub fn predecessors(&self, j: NodeId) -> Mask<W> {
        let mut out = Mask::<W>::from_word(W::default());
        let rows = self.rows.as_ref();
        for i in 0..cap_size(C::CAP) {
            if *rows[i].contains(j.0) {
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
        let n = cap_size(C::CAP);
        for k in 0..n {
            let row_k = self.rows.as_ref()[k];
            for i in 0..n {
                if *self.rows.as_ref()[i].contains(USize(k)) {
                    let unioned = self.rows.as_ref()[i].union(row_k);
                    self.rows.as_mut()[i] = unioned;
                }
            }
        }
    }
}

impl<W, C: Capacity> Copy for BitMatrix<W, C>
where
    W: BitSequence + BitAccess + Copy + Default,
    C::Array<Mask<W>>: Copy,
{
}

impl<W, C: Capacity> Clone for BitMatrix<W, C>
where
    W: BitSequence + BitAccess + Copy + Default,
    C::Array<Mask<W>>: Copy,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<W, C: Capacity> PartialEq for BitMatrix<W, C>
where
    W: BitSequence + BitAccess + Copy + Default,
    Mask<W>: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: std-trait method signature; core::cmp::PartialEq::eq is fixed to return bool by the trait (no-bare-primitives.md exception 5, std-trait method impls); tracked: #207
        self.rows.as_ref() == other.rows.as_ref()
    }
}

impl<W, C: Capacity> Eq for BitMatrix<W, C>
where
    W: BitSequence + BitAccess + Copy + Default,
    Mask<W>: Eq,
{
}

impl<W, C: Capacity> Default for BitMatrix<W, C>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
    }
}
