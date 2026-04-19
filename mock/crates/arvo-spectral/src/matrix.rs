//! Dense `N x N` matrix wrapper.
//!
//! `Matrix<W, const N: Cap>` stores a dense matrix as a stack-allocated
//! `[[W; cap_size(N)]; cap_size(N)]`. The const-generic parameter carries arvo's
//! `Cap` newtype on the public surface; the array-sizing path extracts
//! to `usize` via `.0.0` in const contexts. Mirrors the L0 pattern for
//! `UFixed<const I: IBits, const F: FBits, _>` where the const-generic
//! type is a newtype and the storage path unwraps.
//!
//! Ships local to arvo-spectral this round. Promotion to arvo-sparse
//! (as the shared dense-matrix format) is tracked in BACKLOG.

use arvo::newtype::{Cap, USize};

/// Extract the raw `usize` from a `Cap`.
///
/// Const bridge for array sizing. Nightly `generic_const_exprs` rejects
/// `N.0.0` inline in a const-expression position but accepts a named
/// const fn that returns the same value. Matches the L0 pattern of
/// `ufixed_bits(I, F)` projecting two bit-count newtypes into a raw
/// `usize` inside `[T; ufixed_bits(I, F) as usize]`.
#[inline(always)]
pub const fn cap_size(c: Cap) -> usize {
    c.0.0
}

/// Dense `N x N` matrix over element type `W`.
///
/// Stored as a `[[W; N]; N]` stack-allocated array. `repr(transparent)`
/// over the inner array-of-arrays; no additional state.
#[repr(transparent)]
pub struct Matrix<W: Copy, const N: Cap>
where
    [(); cap_size(N)]:,
{
    /// Private row-major storage. Row index first, column index second.
    data: [[W; cap_size(N)]; cap_size(N)],
}

impl<W: Copy, const N: Cap> Matrix<W, N>
where
    [(); cap_size(N)]:,
{
    /// Construct from a pre-built inner array.
    #[inline(always)]
    pub const fn new(data: [[W; cap_size(N)]; cap_size(N)]) -> Self {
        Self { data }
    }

    /// Construct by invoking `f(row, col)` for every cell.
    ///
    /// Row index and column index are passed as `USize`; the closure
    /// returns the cell value.
    #[inline]
    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(USize, USize) -> W,
    {
        let data = core::array::from_fn(|i| {
            core::array::from_fn(|j| f(USize(i), USize(j)))
        });
        Self { data }
    }

    /// Read the value at `(i, j)`.
    #[inline(always)]
    pub fn get(&self, i: USize, j: USize) -> W {
        self.data[i.0][j.0]
    }

    /// Write `v` to cell `(i, j)`.
    #[inline(always)]
    pub fn set(&mut self, i: USize, j: USize, v: W) {
        self.data[i.0][j.0] = v;
    }

    /// Extract the diagonal as a `[W; N]` array.
    #[inline]
    pub fn diagonal(&self) -> [W; cap_size(N)] {
        core::array::from_fn(|i| self.data[i][i])
    }
}

impl<W: Copy, const N: Cap> Copy for Matrix<W, N> where [(); cap_size(N)]: {}

impl<W: Copy, const N: Cap> Clone for Matrix<W, N>
where
    [(); cap_size(N)]:,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
