//! Sparse adjacency contracts.
//!
//! `SparseAdjacency<N>` and `BidirectionalSparseAdjacency<N>` are sealed
//! const traits that name the interface every structural-decomposition
//! algorithm in this crate consumes: successor / predecessor iteration
//! and node count. Algorithms become generic over `T: SparseAdjacency<N>`;
//! consumers pick the concrete adjacency representation (`BitMatrix<W, N>`,
//! `Csr<ROWS, NNZ, W>`, `CsrBidirectional<ROWS, NNZ, W>`, or any future
//! representation that implements the trait).
//!
//! Round 202605111719 introduces these traits to lift the prior
//! hardcoded `BitMatrix<Bits<64, Hot, Unsigned>, N>` adjacency
//! signature to a W-generic + trait-driven surface.

use arvo::{Cap, USize};
use arvo_bitmask::{BitMatrix, NodeId, SetBitsIter, cap_size};
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};

/// Sealed const trait naming the interface for sparse adjacency.
///
/// Every structural-decomposition algorithm in arvo-sparse takes
/// `T: SparseAdjacency<N>`. Consumers implement once per representation
/// and reuse every algorithm.
///
/// The associated `Successors<'a>` iterator yields the set of nodes
/// `node` has outgoing edges to. `node_count` is the abstract count of
/// nodes in the graph (used to bound algorithms that walk every node).
/// `successor_mask` returns the same set as a bitmask for impls that can
/// answer the bitmap question more cheaply than walking an iterator
/// (e.g. `BitMatrix` returns the row mask directly); impls without a
/// cheap bitmap path fall back to the iterator collection.
pub const trait SparseAdjacency<const N: Cap> {
    /// Iterator over the successors of a given node.
    ///
    /// The lifetime ties the iterator to the borrow of `self`; the
    /// iterator may borrow internal state without copying. Each item
    /// is a `NodeId` in `0..node_count()`.
    type Successors<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    /// Walk the successor set of `node` as an iterator.
    fn successors<'a>(&'a self, node: NodeId) -> Self::Successors<'a>;

    /// Number of nodes in the graph.
    fn node_count(&self) -> USize;
}

/// Sealed const trait extending `SparseAdjacency<N>` with cheap
/// predecessor access.
///
/// Algorithms that need to walk back-edges (RCM step, Dulmage-Mendelsohn)
/// bound `T: BidirectionalSparseAdjacency<N>`. Implementors that cannot
/// answer predecessor queries cheaply (forward-only CSR) do not implement
/// this sub-trait; consumers needing predecessors construct a
/// bidirectional wrapper (e.g. `CsrBidirectional` builds a transpose at
/// construction time).
pub const trait BidirectionalSparseAdjacency<const N: Cap>:
    SparseAdjacency<N>
{
    /// Iterator over the predecessors of a given node.
    type Predecessors<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    /// Walk the predecessor set of `node` as an iterator.
    fn predecessors<'a>(&'a self, node: NodeId) -> Self::Predecessors<'a>;
}

// --- BitMatrix impls --------------------------------------------------

/// Iterator yielding `NodeId` for set bits of a `Mask<W>`.
///
/// Wraps `Mask::iter_set_bits()` (which yields `USize`) and rewraps
/// each as `NodeId(USize)` so the iterator's `Item` matches the trait
/// contract. The owning `Mask<W>` is moved into the iterator; this is
/// safe because `successors`/`predecessors` on `BitMatrix` return a
/// fresh `Mask<W>` per call.
pub struct BitMatrixSuccessors<W>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
{
    inner: SetBitsIter<W>,
}

impl<W> Iterator for BitMatrixSuccessors<W>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
{
    type Item = NodeId;

    #[inline(always)]
    fn next(&mut self) -> Option<NodeId> { // lint:allow(no-bare-option) reason: core::iter::Iterator::next trait-method signature returns Option<Self::Item>; tracked: #115
        self.inner.next().map(NodeId)
    }
}

impl<W, const N: Cap> SparseAdjacency<N> for BitMatrix<W, N>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
    [(); arvo_bitmask::cap_size(N)]:,
{
    type Successors<'a>
        = BitMatrixSuccessors<W>
    where
        Self: 'a;

    #[inline]
    fn successors<'a>(&'a self, node: NodeId) -> BitMatrixSuccessors<W> {
        BitMatrixSuccessors {
            inner: BitMatrix::<W, N>::successors(self, node).iter_set_bits(),
        }
    }

    #[inline]
    fn node_count(&self) -> USize {
        USize(cap_size(N))
    }
}

impl<W, const N: Cap> BidirectionalSparseAdjacency<N> for BitMatrix<W, N>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
    [(); arvo_bitmask::cap_size(N)]:,
{
    type Predecessors<'a>
        = BitMatrixSuccessors<W>
    where
        Self: 'a;

    #[inline]
    fn predecessors<'a>(&'a self, node: NodeId) -> BitMatrixSuccessors<W> {
        BitMatrixSuccessors {
            inner: BitMatrix::<W, N>::predecessors(self, node).iter_set_bits(),
        }
    }
}
