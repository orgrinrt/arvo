//! Rank-1 fixed-size tensor.

use arvo::newtype::{Cap, USize};

use crate::cap::cap_size;

/// Rank-1 fixed-size tensor.
///
/// `#[repr(transparent)]` over `[T; cap_size(N)]`. Typed `get(USize)` /
/// `set(USize, T)` hide the raw-`usize` array indexing. `from_fn` builds
/// by invoking `f(i: USize)` for each slot.
#[repr(transparent)]
pub struct Array<T, const N: Cap>
where
    [(); cap_size(N)]:,
{
    data: [T; cap_size(N)],
}

impl<T, const N: Cap> Array<T, N>
where
    [(); cap_size(N)]:,
{
    /// Construct from a pre-built inner array.
    #[inline(always)]
    pub const fn new(data: [T; cap_size(N)]) -> Self {
        Self { data }
    }

    /// Read the slot at `i`.
    #[inline(always)]
    pub fn get(&self, i: USize) -> &T {
        debug_assert!(i.0 < cap_size(N), "Array::get: index out of range");
        &self.data[i.0]
    }

    /// Write `v` to slot `i`.
    #[inline(always)]
    pub fn set(&mut self, i: USize, v: T) {
        debug_assert!(i.0 < cap_size(N), "Array::set: index out of range");
        self.data[i.0] = v;
    }

    /// Slot count. Matches the const generic capacity.
    #[inline(always)]
    pub const fn len() -> USize {
        USize(cap_size(N))
    }

    /// Inner slice view.
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Mutable inner slice view.
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T: Copy, const N: Cap> Array<T, N>
where
    [(); cap_size(N)]:,
{
    /// Construct by invoking `f(i)` for every slot.
    #[inline]
    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(USize) -> T,
    {
        let data = core::array::from_fn(|i| f(USize(i)));
        Self { data }
    }

    /// Build an Array populated with `v` in every slot.
    #[inline]
    pub fn filled(v: T) -> Self {
        Self { data: [v; cap_size(N)] }
    }
}

impl<T: Copy, const N: Cap> Copy for Array<T, N> where [(); cap_size(N)]: {}

impl<T: Copy, const N: Cap> Clone for Array<T, N>
where
    [(); cap_size(N)]:,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T, const N: Cap> IntoIterator for &'a Array<T, N>
where
    [(); cap_size(N)]:,
{
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T, const N: Cap> IntoIterator for &'a mut Array<T, N>
where
    [(); cap_size(N)]:,
{
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}
