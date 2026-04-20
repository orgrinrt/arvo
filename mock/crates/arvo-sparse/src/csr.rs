//! Compressed sparse row storage.
//!
//! `Csr<ROWS, NNZ, W>` holds a compressed sparse row matrix in three
//! fixed-size arrays:
//!
//! - `row_ptr[r]` is the start offset of row `r` into `col_idx` /
//!   `values`. `row_ptr[ROWS - 1]` acts as the end of the last row
//!   (see `row_end` accessor; the single-exclusive-end cell is
//!   implicit from `NNZ`).
//! - `col_idx[k]` is the column `NodeId` of the `k`-th non-zero.
//! - `values[k]` is the value of the `k`-th non-zero.
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

use arvo::newtype::{Cap, USize};
use arvo_bitmask::{NodeId, cap_size};
use notko::Maybe;

/// Compressed sparse row matrix.
///
/// Storage: `row_ptr` of length `ROWS`, `col_idx` and `values` of
/// length `NNZ`. Row `r` occupies `col_idx[row_ptr[r] .. row_end(r)]`
/// with corresponding entries in `values`. `row_end(r)` is
/// `row_ptr[r + 1]` for `r < ROWS - 1` and `NNZ` for the last row.
#[derive(Copy, Clone)]
pub struct Csr<const ROWS: Cap, const NNZ: Cap, W: Copy>
where
    [(); cap_size(ROWS)]:,
    [(); cap_size(NNZ)]:,
{
    /// Row start offsets. `row_ptr[r]` is the index of row `r`'s
    /// first non-zero within `col_idx` / `values`.
    pub row_ptr: [USize; cap_size(ROWS)],
    /// Column index of each non-zero, flattened row-major.
    pub col_idx: [NodeId; cap_size(NNZ)],
    /// Value of each non-zero, flattened row-major.
    pub values: [W; cap_size(NNZ)],
}

impl<const ROWS: Cap, const NNZ: Cap, W: Copy + Default> Csr<ROWS, NNZ, W>
where
    [(); cap_size(ROWS)]:,
    [(); cap_size(NNZ)]:,
{
    /// Empty matrix: all offsets zero, all columns `NodeId(USize(0))`,
    /// all values `W::default()`.
    ///
    /// Callers populate by writing into the public fields directly;
    /// this round does not expose a mutation API.
    #[inline]
    pub fn new() -> Self {
        Self {
            row_ptr: [USize(0); cap_size(ROWS)],
            col_idx: [NodeId::new(USize(0)); cap_size(NNZ)],
            values: [W::default(); cap_size(NNZ)],
        }
    }
}

impl<const ROWS: Cap, const NNZ: Cap, W: Copy> Csr<ROWS, NNZ, W>
where
    [(); cap_size(ROWS)]:,
    [(); cap_size(NNZ)]:,
{
    /// End offset of row `r`.
    ///
    /// Returns `row_ptr[r + 1]` for non-last rows and `NNZ` for the
    /// last row. `r >= ROWS` yields `USize(0)` (empty range).
    #[inline(always)]
    fn row_end(&self, r: usize) -> USize {
        if r + 1 < cap_size(ROWS) {
            self.row_ptr[r + 1]
        } else if r < cap_size(ROWS) {
            USize(cap_size(NNZ))
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
        let r = row.0;
        if r >= cap_size(ROWS) {
            return Maybe::Isnt;
        }
        let start = self.row_ptr[r].0;
        let end = self.row_end(r).0;
        if start > end || end > cap_size(NNZ) {
            return Maybe::Isnt;
        }
        let mut k = start;
        while k < end {
            if self.col_idx[k] == col {
                return Maybe::Is(self.values[k]);
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
        let r = row.0;
        if r >= cap_size(ROWS) {
            return &[];
        }
        let start = self.row_ptr[r].0;
        let end = self.row_end(r).0;
        if start > end || end > cap_size(NNZ) {
            return &[];
        }
        &self.values[start..end]
    }

    /// Slice of column indices for `row`.
    ///
    /// Returns an empty slice when `row` is out of range or the row
    /// offsets are inconsistent.
    #[inline]
    pub fn row_col_indices(&self, row: USize) -> &[NodeId] {
        let r = row.0;
        if r >= cap_size(ROWS) {
            return &[];
        }
        let start = self.row_ptr[r].0;
        let end = self.row_end(r).0;
        if start > end || end > cap_size(NNZ) {
            return &[];
        }
        &self.col_idx[start..end]
    }

    /// Number of non-zeros in `row`.
    ///
    /// Returns `USize(0)` when `row` is out of range or the row
    /// offsets are inconsistent.
    #[inline]
    pub fn nnz(&self, row: USize) -> USize {
        let r = row.0;
        if r >= cap_size(ROWS) {
            return USize(0);
        }
        let start = self.row_ptr[r].0;
        let end = self.row_end(r).0;
        if start > end || end > cap_size(NNZ) {
            return USize(0);
        }
        USize(end - start)
    }
}

impl<const ROWS: Cap, const NNZ: Cap, W: Copy + Default> Default for Csr<ROWS, NNZ, W>
where
    [(); cap_size(ROWS)]:,
    [(); cap_size(NNZ)]:,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
