//! Rank-2 fixed-size N×N tensor. Promoted from arvo-spectral.

use arvo::USize;

use crate::array::Array;
use crate::cap::cap_size;
use crate::capacity::Capacity;

/// Dense `N × N` matrix over element type `W`.
///
/// `#[repr(transparent)]` over the 2-D composition `C::Array<C::Array<W>>`,
/// row-major (row index first, column index second). Typed
/// `get(USize, USize)` / `set(USize, USize, W)` hide the raw-`usize` slice
/// indexing behind the method calls. The capacity is a TYPE on both axes, so
/// no `cap_size` expression sits in type position.
#[repr(transparent)]
pub struct Matrix<W: Copy, C: Capacity> {
    /// Private row-major storage. Row index first, column index second.
    data: C::Array<C::Array<W>>,
}

impl<W: Copy, C: Capacity> Matrix<W, C> {
    /// Construct from a pre-built backing array-of-arrays.
    #[inline(always)]
    pub const fn new(data: C::Array<C::Array<W>>) -> Self {
        Self { data }
    }

    /// Construct by invoking `f(row, col)` for every cell.
    #[inline]
    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(USize, USize) -> W,
    {
        let data = C::from_fn(|i| C::from_fn(|j| f(i, j)));
        Self { data }
    }

    /// Read the value at `(i, j)`.
    #[inline(always)]
    pub fn get(&self, i: USize, j: USize) -> W {
        debug_assert!(i.0 < cap_size(C::CAP), "Matrix::get: row index out of range");
        debug_assert!(j.0 < cap_size(C::CAP), "Matrix::get: column index out of range");
        self.data.as_ref()[i.0].as_ref()[j.0]
    }

    /// Write `v` to cell `(i, j)`.
    #[inline(always)]
    pub fn set(&mut self, i: USize, j: USize, v: W) {
        debug_assert!(i.0 < cap_size(C::CAP), "Matrix::set: row index out of range");
        debug_assert!(j.0 < cap_size(C::CAP), "Matrix::set: column index out of range");
        self.data.as_mut()[i.0].as_mut()[j.0] = v;
    }

    /// Extract the diagonal as an `Array<W, C>`.
    #[inline]
    pub fn diagonal(&self) -> Array<W, C> {
        Array::from_fn(|i| self.get(i, i))
    }
}

impl<W: Copy, C: Capacity> Matrix<W, C>
where
    C::Array<W>: Copy,
{
    /// Build a Matrix populated with `v` in every cell.
    #[inline]
    pub fn filled(v: W) -> Self {
        Self { data: C::filled(C::filled(v)) }
    }
}

impl<W: Copy, C: Capacity> Copy for Matrix<W, C> where C::Array<C::Array<W>>: Copy {}

impl<W: Copy, C: Capacity> Clone for Matrix<W, C>
where
    C::Array<C::Array<W>>: Copy,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
