//! Rank-2 fixed-size N×N tensor. Promoted from arvo-spectral.

use arvo::newtype::{Cap, USize};

use crate::array::Array;
use crate::cap::cap_size;

/// Dense `N × N` matrix over element type `W`.
///
/// `#[repr(transparent)]` over `[[W; cap_size(N)]; cap_size(N)]`. Row-major
/// storage. Typed `get(USize, USize)` / `set(USize, USize, W)` hide the
/// raw-`usize` array indexing behind the method calls.
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
    /// Construct from a pre-built inner array-of-arrays.
    #[inline(always)]
    pub const fn new(data: [[W; cap_size(N)]; cap_size(N)]) -> Self {
        Self { data }
    }

    /// Construct by invoking `f(row, col)` for every cell.
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

    /// Build a Matrix populated with `v` in every cell.
    #[inline]
    pub fn filled(v: W) -> Self {
        Self { data: [[v; cap_size(N)]; cap_size(N)] }
    }

    /// Read the value at `(i, j)`.
    #[inline(always)]
    pub fn get(&self, i: USize, j: USize) -> W {
        debug_assert!(i.0 < cap_size(N), "Matrix::get: row out of range");
        debug_assert!(j.0 < cap_size(N), "Matrix::get: col out of range");
        self.data[i.0][j.0]
    }

    /// Write `v` to cell `(i, j)`.
    #[inline(always)]
    pub fn set(&mut self, i: USize, j: USize, v: W) {
        debug_assert!(i.0 < cap_size(N), "Matrix::set: row out of range");
        debug_assert!(j.0 < cap_size(N), "Matrix::set: col out of range");
        self.data[i.0][j.0] = v;
    }

    /// Extract the diagonal as an `Array<W, N>`.
    #[inline]
    pub fn diagonal(&self) -> Array<W, N> {
        Array::from_fn(|i| self.data[i.0][i.0])
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
