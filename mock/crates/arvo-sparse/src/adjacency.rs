//! Sparse adjacency contracts.
//!
//! `SparseAdjacency<C>` and `BidirectionalSparseAdjacency<C>` are sealed
//! const traits that name the interface every structural-decomposition
//! algorithm in this crate consumes: successor / predecessor iteration
//! and node count. Algorithms become generic over `T: SparseAdjacency<C>`;
//! consumers pick the concrete adjacency representation (`BitMatrix<W, C>`,
//! `Csr<R, NNZ, W>`, `CsrBidirectional<R, NNZ, W>`, or any future
//! representation that implements the trait).
//!
//! `C` is a `Capacity` type (arvo-tensor's `Capacity`) carrying the node
//! count; the capacity is a type, so no `cap_size` expression sits in type
//! position. Node arrays returned by the problem-shaped algorithms are the
//! associated type `C::Array<NodeId>`.
//!
//! Round 202605111719 introduces these traits to lift the prior
//! hardcoded `BitMatrix<Bits<64, Hot, Unsigned>, N>` adjacency
//! signature to a W-generic + trait-driven surface.

use arvo::USize;
use arvo_bitmask::{BitMatrix, NodeId, SetBitsIter};
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};
use arvo_tensor::Capacity;

/// Sealed const trait naming the interface for sparse adjacency.
///
/// Every structural-decomposition algorithm in arvo-sparse takes
/// `T: SparseAdjacency<C>`. Consumers implement once per representation
/// and reuse every algorithm.
///
/// The associated `Successors<'a>` iterator yields the set of nodes
/// `node` has outgoing edges to. `node_count` is the abstract count of
/// nodes in the graph (used to bound algorithms that walk every node).
/// `successor_mask` returns the same set as a bitmask for impls that can
/// answer the bitmap question more cheaply than walking an iterator
/// (e.g. `BitMatrix` returns the row mask directly); impls without a
/// cheap bitmap path fall back to the iterator collection.
pub const trait SparseAdjacency<C: Capacity> {
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

/// Sealed const trait extending `SparseAdjacency<C>` with cheap
/// predecessor access.
///
/// Algorithms that need to walk back-edges (RCM step, Dulmage-Mendelsohn)
/// bound `T: BidirectionalSparseAdjacency<C>`. Implementors that cannot
/// answer predecessor queries cheaply (forward-only CSR) do not implement
/// this sub-trait; consumers needing predecessors construct a
/// bidirectional wrapper (e.g. `CsrBidirectional` builds a transpose at
/// construction time).
pub const trait BidirectionalSparseAdjacency<C: Capacity>: SparseAdjacency<C> {
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
    // `rustfmt::skip` keeps the allow on its line: the lint reads the line the
    // violation is on, and the formatter otherwise moves the comment below it.
    #[rustfmt::skip]
    fn next(&mut self) -> Option<NodeId> { // lint:allow(no-bare-option) reason: core::iter::Iterator::next trait-method signature returns Option<Self::Item>; tracked: #115
        self.inner.next().map(NodeId)
    }
}

impl<W, C: Capacity> SparseAdjacency<C> for BitMatrix<W, C>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
{
    type Successors<'a>
        = BitMatrixSuccessors<W>
    where
        Self: 'a;

    #[inline]
    fn successors(&self, node: NodeId) -> BitMatrixSuccessors<W> {
        BitMatrixSuccessors {
            inner: BitMatrix::<W, C>::successors(self, node).iter_set_bits(),
        }
    }

    #[inline]
    fn node_count(&self) -> USize {
        C::CAP.into()
    }
}

impl<W, C: Capacity> BidirectionalSparseAdjacency<C> for BitMatrix<W, C>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
{
    type Predecessors<'a>
        = BitMatrixSuccessors<W>
    where
        Self: 'a;

    #[inline]
    fn predecessors(&self, node: NodeId) -> BitMatrixSuccessors<W> {
        BitMatrixSuccessors {
            inner: BitMatrix::<W, C>::predecessors(self, node).iter_set_bits(),
        }
    }
}

// --- Problem-shaped traits -------------------------------------------
//
// Each names what the algorithm produces, with a default impl that
// runs the canonical algorithm through the iterator-based `_via`
// path. Consumers with a `BitMatrix<W, C>` adjacency can call the
// free-function variants (`rcm_reorder`, `block_diagonal`,
// `dulmage_mendelsohn`) directly for the mask-based fast path; the
// trait defaults exist so any `SparseAdjacency` /
// `BidirectionalSparseAdjacency` implementor gets the result without
// re-implementing the algorithm.

/// Bandwidth-reduction reorder. Default impl runs RCM.
pub trait BandwidthReducer<C: Capacity>: BidirectionalSparseAdjacency<C> {
    /// Return a permutation `C::Array<NodeId>` where
    /// `result[new_pos] = old_NodeId`.
    #[inline]
    fn reduce_bandwidth(&self) -> C::Array<NodeId>
    where
        Self: Sized,
    {
        crate::rcm::rcm_reorder_via::<Self, C>(self)
    }
}

/// Block-diagonal partitioner. Default impl runs connected-components.
pub trait BlockPartitioner<C: Capacity>: BidirectionalSparseAdjacency<C> {
    /// Return `(block_count, per_node_block_ids)`.
    #[inline]
    fn partition_blocks(&self) -> (USize, C::Array<USize>)
    where
        Self: Sized,
    {
        crate::block::block_diagonal_via::<Self, C>(self)
    }
}

/// Bipartite structural analyser. Default impl runs Dulmage-Mendelsohn.
pub trait BipartiteStructuralAnalysis<C: Capacity>: BidirectionalSparseAdjacency<C> {
    /// Return a `DulmageMendelsohn<C>` classification.
    #[inline]
    fn analyse_structure(&self) -> crate::dm::DulmageMendelsohn<C>
    where
        Self: Sized,
    {
        crate::dm::dulmage_mendelsohn_via::<Self, C>(self)
    }
}

// Blanket impls: every `BidirectionalSparseAdjacency` consumer gets
// the three problem-shaped methods for free.
impl<T, C: Capacity> BandwidthReducer<C> for T where T: BidirectionalSparseAdjacency<C> {}

impl<T, C: Capacity> BlockPartitioner<C> for T where T: BidirectionalSparseAdjacency<C> {}

impl<T, C: Capacity> BipartiteStructuralAnalysis<C> for T where T: BidirectionalSparseAdjacency<C> {}
