//! Compressed sparse row storage.
//!
//! `Csr<R, NNZ, W>` holds a compressed sparse row matrix in three
//! fixed-size capacity arrays:
//!
//! - `row_ptr[r]` is the start offset of row `r` into `col_idx` /
//!   `values`. The last live row's end is `live_nnz` (see `row_end`
//!   accessor).
//! - `col_idx[k]` is the column `NodeId` of the `k`-th non-zero.
//! - `values[k]` is the value of the `k`-th non-zero.
//!
//! `R` and `NNZ` are `Capacity` types (the row capacity and the nnz
//! capacity). Storage is the associated array `R::Array<USize>` /
//! `NNZ::Array<NodeId>` / `NNZ::Array<W>`, so no `cap_size` expression
//! sits in type position. A body that needs a count as a value reads
//! `cap_size(R::CAP)` / `cap_size(NNZ::CAP)`.
//!
//! This round ships read-only storage. The constructor fills every
//! slot with a default value. Population happens via direct field
//! assignment in a later round; mutation accessors (`insert`,
//! `remove`) are deferred. The current surface exposes query-side
//! accessors only: `get`, `row_values`, `row_col_indices`, `nnz`.
//!
//! Generic over the value type `W: Copy + Default`. No numeric trait
//! bound: the CSR structure itself does not compute on values.
//! Algorithms that do (SpMV in a later round) will tighten the bound
//! at their own impl sites.

use arvo::USize;
use arvo_bitmask::{NodeId, cap_size};
use arvo_tensor::Capacity;
use notko::Maybe;

/// Compressed sparse row matrix.
///
/// Storage: `row_ptr` of capacity `R`, `col_idx` and `values` of
/// capacity `NNZ`. Row `r` occupies `col_idx[row_ptr[r] .. row_end(r)]`
/// with corresponding entries in `values`. `row_end(r)` is
/// `row_ptr[r + 1]` for non-last rows and `live_nnz` for the last row.
pub struct Csr<R: Capacity, NNZ: Capacity, W: Copy> {
    /// Row start offsets. `row_ptr[r]` is the index of row `r`'s
    /// first non-zero within `col_idx` / `values`.
    pub row_ptr: R::Array<USize>,
    /// Column index of each non-zero, flattened row-major.
    pub col_idx: NNZ::Array<NodeId>,
    /// Value of each non-zero, flattened row-major.
    pub values: NNZ::Array<W>,
    /// Live row count. Rows `0..live_rows` carry data; rows at or
    /// beyond it are empty. `new` defaults this to `cap_size(R::CAP)`
    /// (fully packed). A capacity-with-slack consumer sets a smaller
    /// count so the unused tail is never iterated.
    pub live_rows: USize,
    /// Live non-zero count. The last live row ends at `live_nnz`
    /// rather than `cap_size(NNZ::CAP)`, so the slack tail of `col_idx` /
    /// `values` past it is never read. `new` defaults it to
    /// `cap_size(NNZ::CAP)` (fully packed).
    pub live_nnz: USize,
}

impl<R: Capacity, NNZ: Capacity, W: Copy> Copy for Csr<R, NNZ, W>
where
    R::Array<USize>: Copy,
    NNZ::Array<NodeId>: Copy,
    NNZ::Array<W>: Copy,
{
}

impl<R: Capacity, NNZ: Capacity, W: Copy> Clone for Csr<R, NNZ, W>
where
    R::Array<USize>: Copy,
    NNZ::Array<NodeId>: Copy,
    NNZ::Array<W>: Copy,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<R: Capacity, NNZ: Capacity, W: Copy + Default> Csr<R, NNZ, W> {
    /// Empty matrix: all offsets zero, all columns `NodeId(USize(0))`,
    /// all values `W::default()`.
    ///
    /// Callers populate by writing into the public fields directly;
    /// this round does not expose a mutation API.
    #[inline]
    pub fn new() -> Self {
        Self {
            row_ptr: R::filled(USize(0)),
            col_idx: NNZ::filled(NodeId::new(USize(0))),
            values: NNZ::filled(W::default()),
            // packed default: every row and every nnz slot is live.
            live_rows: USize(cap_size(R::CAP)),
            live_nnz: USize(cap_size(NNZ::CAP)),
        }
    }

    /// Empty matrix with explicit live counts.
    ///
    /// Like `new`, but sets `live_rows` / `live_nnz` to the given
    /// counts instead of the full caps. A capacity-with-slack consumer
    /// (a fixed-capacity buffer with a runtime live count) uses this to
    /// declare how much of the storage carries data, so the unused tail
    /// is never iterated by the algorithms or the transpose builder.
    /// Callers then populate `row_ptr` / `col_idx` / `values` for the
    /// live range by direct field assignment.
    #[inline]
    pub fn with_live_counts(live_rows: USize, live_nnz: USize) -> Self {
        let mut csr = Self::new();
        csr.live_rows = live_rows;
        csr.live_nnz = live_nnz;
        csr
    }
}

impl<R: Capacity, NNZ: Capacity, W: Copy> Csr<R, NNZ, W> {
    /// End offset of row `r`.
    ///
    /// Returns `row_ptr[r + 1]` for non-last rows and `live_nnz` for the
    /// last row. `r >= live_rows` yields `USize(0)` (empty range).
    #[inline(always)]
    fn row_end(&self, r: USize) -> USize {
        // Live counts clamped to the caps so a bogus live count can
        // never index past the storage arrays (defensive, matching the
        // query-side `end > cap_size(NNZ::CAP)` guards below). For a
        // packed matrix (live == cap) this reduces exactly to the prior
        // last-row-to-`cap_size(NNZ::CAP)` behaviour.
        let live_rows = self.live_rows.0.min(cap_size(R::CAP));
        let live_nnz = self.live_nnz.0.min(cap_size(NNZ::CAP));
        if r.0 + 1 < live_rows {
            self.row_ptr.as_ref()[r.0 + 1]
        } else if r.0 + 1 == live_rows {
            USize(live_nnz)
        } else {
            USize(0)
        }
    }

    /// Value at `(row, col)` if present, otherwise `Maybe::Isnt`.
    ///
    /// Scans `col_idx[row_ptr[row] .. row_end(row)]` for the target
    /// column. Linear in the row's non-zero count.
    #[inline]
    pub fn get(&self, row: USize, col: NodeId) -> Maybe<W> {
        if row.0 >= cap_size(R::CAP) {
            return Maybe::Isnt;
        }
        let start = self.row_ptr.as_ref()[row.0].0;
        let end = self.row_end(row).0;
        if start > end || end > cap_size(NNZ::CAP) {
            return Maybe::Isnt;
        }
        let cols = self.col_idx.as_ref();
        let vals = self.values.as_ref();
        let mut k = start;
        while k < end {
            if cols[k] == col {
                return Maybe::Is(vals[k]);
            }
            k += 1;
        }
        Maybe::Isnt
    }

    /// Slice of value entries for `row`.
    ///
    /// Returns an empty slice when `row` is out of range or the row
    /// offsets are inconsistent.
    #[inline]
    pub fn row_values(&self, row: USize) -> &[W] {
        if row.0 >= cap_size(R::CAP) {
            return &[];
        }
        let start = self.row_ptr.as_ref()[row.0].0;
        let end = self.row_end(row).0;
        if start > end || end > cap_size(NNZ::CAP) {
            return &[];
        }
        &self.values.as_ref()[start..end]
    }

    /// Slice of column indices for `row`.
    ///
    /// Returns an empty slice when `row` is out of range or the row
    /// offsets are inconsistent.
    #[inline]
    pub fn row_col_indices(&self, row: USize) -> &[NodeId] {
        if row.0 >= cap_size(R::CAP) {
            return &[];
        }
        let start = self.row_ptr.as_ref()[row.0].0;
        let end = self.row_end(row).0;
        if start > end || end > cap_size(NNZ::CAP) {
            return &[];
        }
        &self.col_idx.as_ref()[start..end]
    }

    /// Number of non-zeros in `row`.
    ///
    /// Returns `USize(0)` when `row` is out of range or the row
    /// offsets are inconsistent.
    #[inline]
    pub fn nnz(&self, row: USize) -> USize {
        if row.0 >= cap_size(R::CAP) {
            return USize(0);
        }
        let start = self.row_ptr.as_ref()[row.0].0;
        let end = self.row_end(row).0;
        if start > end || end > cap_size(NNZ::CAP) {
            return USize(0);
        }
        USize(end - start)
    }
}

impl<R: Capacity, NNZ: Capacity, W: Copy + Default> Default for Csr<R, NNZ, W> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

// ---- SparseAdjacency impl on Csr (round 202605111719) -----------------
//
// Walks `col_idx[row_ptr[i] .. row_end(i)]` as a slice iterator. The
// associated `Successors<'a>` is `Copied<Iter<'a, NodeId>>` because
// `NodeId` is `Copy` and the trait's iterator must yield `NodeId` by
// value (not `&NodeId`).

impl<R: Capacity, NNZ: Capacity, W: Copy> crate::adjacency::SparseAdjacency<R>
    for Csr<R, NNZ, W>
{
    type Successors<'a>
        = core::iter::Copied<core::slice::Iter<'a, NodeId>>
    where
        Self: 'a;

    #[inline]
    fn successors<'a>(&'a self, node: NodeId) -> Self::Successors<'a> {
        // NodeId derefs to USize; row_col_indices takes USize.
        self.row_col_indices(node.0).iter().copied()
    }

    #[inline]
    fn node_count(&self) -> USize {
        // Live node count, clamped to the cap. A packed Csr reports the
        // cap (so existing packed consumers are unchanged); a loose Csr
        // reports its smaller live row count, bounding the algorithms.
        USize(self.live_rows.0.min(cap_size(R::CAP)))
    }
}

// ---- CsrBidirectional + Csr::with_transpose (round 202605111719) ------
//
// Carries the original CSR plus a transposed CSR (transpose_row_ptr +
// transpose_col_idx, no values needed because predecessor queries only
// ask "which rows have an edge to me"). The transpose is built once at
// construction in O(NNZ + ROWS) via the canonical count-prefix-scatter
// algorithm.
//
// Predecessor lookup costs O(in_degree). The trade vs forward-only `Csr`
// is 2x memory for the transpose indices (no values duplicated) plus
// one construction sweep. Algorithms needing predecessors (RCM step,
// Dulmage-Mendelsohn) bound on `BidirectionalSparseAdjacency<R>`.

/// Bidirectional compressed sparse row matrix.
///
/// Carries the original CSR (`forward`) plus a transpose row/col index
/// pair (`transpose_row_ptr`, `transpose_col_idx`) for cheap predecessor
/// lookup. Predecessor of node `j` is found by walking
/// `transpose_col_idx[transpose_row_ptr[j] .. transpose_row_end(j)]`,
/// which lists every row `i` whose forward edge `i -> j` exists.
///
/// The transpose has the same `NNZ` capacity and is built once via
/// `Csr::with_transpose`. Use `Csr` directly when only successor
/// queries are needed.
pub struct CsrBidirectional<R: Capacity, NNZ: Capacity, W: Copy> {
    /// Forward CSR (carries values).
    pub forward: Csr<R, NNZ, W>,
    /// Transpose row start offsets. `transpose_row_ptr[j]` indexes
    /// `transpose_col_idx` for the predecessors of node `j`.
    pub transpose_row_ptr: R::Array<USize>,
    /// Transpose column indices: the predecessor rows themselves.
    pub transpose_col_idx: NNZ::Array<NodeId>,
    /// Live row count, carried from the forward CSR. Predecessor
    /// queries and `node_count` honour it; rows at or beyond it are
    /// empty.
    pub live_rows: USize,
    /// Live non-zero count, carried from the forward CSR. The last
    /// live transpose row ends at `live_nnz`.
    pub live_nnz: USize,
}

impl<R: Capacity, NNZ: Capacity, W: Copy> Copy for CsrBidirectional<R, NNZ, W>
where
    Csr<R, NNZ, W>: Copy,
    R::Array<USize>: Copy,
    NNZ::Array<NodeId>: Copy,
{
}

impl<R: Capacity, NNZ: Capacity, W: Copy> Clone for CsrBidirectional<R, NNZ, W>
where
    Csr<R, NNZ, W>: Copy,
    R::Array<USize>: Copy,
    NNZ::Array<NodeId>: Copy,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<R: Capacity, NNZ: Capacity, W: Copy + Default> Csr<R, NNZ, W> {
    /// Consume this CSR and build a bidirectional view with the
    /// transpose pre-computed.
    ///
    /// Cost: O(NNZ + ROWS) construction, 2x memory for the transpose
    /// indices (no values duplicated). The forward CSR is copied
    /// in; the transpose row pointers and column indices are built
    /// via count-prefix-scatter.
    ///
    /// Preconditions: the input CSR is well-formed. Specifically,
    /// `row_ptr` is monotone non-decreasing with `row_ptr[live_rows - 1]
    /// <= live_nnz`, and every `col_idx[k].0.0 < live_rows`. Out-of-range
    /// column indices are silently skipped (treated as having no
    /// transpose contribution); a malformed `row_ptr` may truncate
    /// the scatter at the per-column cursor bound. The type system
    /// is the contract: substrate algorithms trust their input
    /// shapes per `arvo-toolbox-not-policer`. Construct the input
    /// CSR through the documented constructors and the
    /// preconditions hold by construction.
    pub fn with_transpose(self) -> CsrBidirectional<R, NNZ, W>
    where
        R::Array<USize>: Copy,
        NNZ::Array<NodeId>: Copy,
    {
        // Capture the live counts before `self` is moved into the
        // result, and clamp the iteration bounds to the caps so the
        // loops never index past the storage arrays. The slack tail
        // (rows beyond `live_rows`, nnz slots beyond `live_nnz`) is
        // never read, so no phantom edge into a default-`NodeId(0)`
        // tail slot is ever counted. For a packed matrix (live == cap)
        // the bounds equal the caps and behaviour is unchanged.
        let live_rows = self.live_rows;
        let live_nnz = self.live_nnz;
        let n_rows = live_rows.0.min(cap_size(R::CAP));
        let n_nnz = live_nnz.0.min(cap_size(NNZ::CAP));

        let fwd_col_idx = self.col_idx;
        let fwd_row_ptr = self.row_ptr;

        // Count incoming edges per column.
        let mut counts: R::Array<USize> = R::filled(USize(0));
        {
            let cols = fwd_col_idx.as_ref();
            let counts_mut = counts.as_mut();
            let mut k = 0;
            while k < n_nnz {
                let col = cols[k].0;
                if col.0 < n_rows {
                    counts_mut[col.0] = USize(counts_mut[col.0].0 + 1);
                }
                k += 1;
            }
        }

        // Prefix-sum into transpose_row_ptr.
        let mut transpose_row_ptr: R::Array<USize> = R::filled(USize(0));
        {
            let counts_ref = counts.as_ref();
            let trp = transpose_row_ptr.as_mut();
            let mut acc = 0;
            let mut r = 0;
            while r < n_rows {
                trp[r] = USize(acc);
                acc += counts_ref[r].0;
                r += 1;
            }
        }

        // Scatter: for each forward edge (i, col_idx[k]), record
        // i at transpose_col_idx[cursor[col]++].
        let mut cursor: R::Array<USize> = transpose_row_ptr;
        let mut transpose_col_idx: NNZ::Array<NodeId> =
            NNZ::filled(NodeId::new(USize(0)));
        {
            let cols = fwd_col_idx.as_ref();
            let row_ptr = fwd_row_ptr.as_ref();
            let cursor_mut = cursor.as_mut();
            let tci = transpose_col_idx.as_mut();
            let mut i = 0;
            while i < n_rows {
                let start = row_ptr[i].0;
                let end = if i + 1 < n_rows {
                    row_ptr[i + 1].0
                } else {
                    n_nnz
                };
                let mut k = start;
                while k < end {
                    let col = cols[k].0;
                    if col.0 < n_rows {
                        let slot = cursor_mut[col.0].0;
                        if slot < n_nnz {
                            tci[slot] = NodeId::new(USize(i));
                            cursor_mut[col.0] = USize(slot + 1);
                        }
                    }
                    k += 1;
                }
                i += 1;
            }
        }

        CsrBidirectional {
            forward: Csr {
                row_ptr: fwd_row_ptr,
                col_idx: fwd_col_idx,
                values: self.values,
                live_rows,
                live_nnz,
            },
            transpose_row_ptr,
            transpose_col_idx,
            live_rows,
            live_nnz,
        }
    }
}

impl<R: Capacity, NNZ: Capacity, W: Copy> CsrBidirectional<R, NNZ, W> {
    /// End offset of transpose row `r`.
    ///
    /// Symmetric to `Csr::row_end`. Returns `transpose_row_ptr[r + 1]`
    /// for non-last rows and `live_nnz` for the last row.
    #[inline(always)]
    fn transpose_row_end(&self, r: USize) -> USize {
        // Symmetric to `Csr::row_end`: the last live transpose row ends
        // at `live_nnz`, rows at or beyond `live_rows` are empty. Live
        // counts clamped to the caps; packed (live == cap) reduces to
        // the prior last-row-to-`cap_size(NNZ::CAP)` behaviour.
        let live_rows = self.live_rows.0.min(cap_size(R::CAP));
        let live_nnz = self.live_nnz.0.min(cap_size(NNZ::CAP));
        if r.0 + 1 < live_rows {
            self.transpose_row_ptr.as_ref()[r.0 + 1]
        } else if r.0 + 1 == live_rows {
            USize(live_nnz)
        } else {
            USize(0)
        }
    }

    /// Slice of predecessor row indices for node `node`.
    #[inline]
    fn predecessors_slice(&self, node: NodeId) -> &[NodeId] {
        let r = node.0;
        if r.0 >= cap_size(R::CAP) {
            return &[];
        }
        let start = self.transpose_row_ptr.as_ref()[r.0].0;
        let end = self.transpose_row_end(r).0;
        if start > end || end > cap_size(NNZ::CAP) {
            return &[];
        }
        &self.transpose_col_idx.as_ref()[start..end]
    }
}

impl<R: Capacity, NNZ: Capacity, W: Copy> crate::adjacency::SparseAdjacency<R>
    for CsrBidirectional<R, NNZ, W>
{
    type Successors<'a>
        = core::iter::Copied<core::slice::Iter<'a, NodeId>>
    where
        Self: 'a;

    #[inline]
    fn successors<'a>(&'a self, node: NodeId) -> Self::Successors<'a> {
        self.forward.row_col_indices(node.0).iter().copied()
    }

    #[inline]
    fn node_count(&self) -> USize {
        // Live node count, clamped to the cap. A packed Csr reports the
        // cap (so existing packed consumers are unchanged); a loose Csr
        // reports its smaller live row count, bounding the algorithms.
        USize(self.live_rows.0.min(cap_size(R::CAP)))
    }
}

impl<R: Capacity, NNZ: Capacity, W: Copy>
    crate::adjacency::BidirectionalSparseAdjacency<R> for CsrBidirectional<R, NNZ, W>
{
    type Predecessors<'a>
        = core::iter::Copied<core::slice::Iter<'a, NodeId>>
    where
        Self: 'a;

    #[inline]
    fn predecessors<'a>(&'a self, node: NodeId) -> Self::Predecessors<'a> {
        self.predecessors_slice(node).iter().copied()
    }
}
