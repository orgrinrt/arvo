//! Rank-1 fixed-size tensor.

use arvo::USize;

use crate::cap::cap_size;
use crate::capacity::Capacity;

/// Rank-1 fixed-size tensor.
///
/// `#[repr(transparent)]` over the capacity's backing array `C::Array<T>`
/// (`[T; N]` for `Dim<N>`). Typed `get(USize)` / `set(USize, T)` hide the
/// raw-`usize` slice indexing. `from_fn` builds by invoking `f(i: USize)` for
/// each slot through `Capacity::from_fn`. The capacity is a TYPE, so no
/// `cap_size` expression sits in type position.
#[repr(transparent)]
pub struct Array<T, C: Capacity> {
    data: C::Array<T>,
}

impl<T, C: Capacity> Array<T, C> {
    /// Construct from a pre-built backing array.
    #[inline(always)]
    pub const fn new(data: C::Array<T>) -> Self {
        Self { data }
    }

    /// Construct by invoking `f(i)` for every slot.
    #[inline]
    pub fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(USize) -> T,
    {
        Self { data: C::from_fn(f) }
    }

    /// Read the slot at `i`.
    #[inline(always)]
    pub fn get(&self, i: USize) -> &T {
        debug_assert!(i.0 < cap_size(C::CAP), "Array::get: index out of range");
        &self.data.as_ref()[i.0]
    }

    /// Write `v` to slot `i`.
    #[inline(always)]
    pub fn set(&mut self, i: USize, v: T) {
        debug_assert!(i.0 < cap_size(C::CAP), "Array::set: index out of range");
        self.data.as_mut()[i.0] = v;
    }

    /// Slot count. Matches the capacity.
    #[inline(always)]
    pub const fn len() -> USize {
        USize(cap_size(C::CAP))
    }

    /// Inner slice view.
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        self.data.as_ref()
    }

    /// Mutable inner slice view.
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data.as_mut()
    }
}

impl<T: Copy, C: Capacity> Array<T, C> {
    /// Build an Array populated with `v` in every slot.
    #[inline]
    pub fn filled(v: T) -> Self {
        Self { data: C::filled(v) }
    }
}

impl<T, C: Capacity> Copy for Array<T, C> where C::Array<T>: Copy {}

impl<T, C: Capacity> Clone for Array<T, C>
where
    C::Array<T>: Copy,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T, C: Capacity> IntoIterator for &'a Array<T, C> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.data.as_ref().iter()
    }
}

impl<'a, T, C: Capacity> IntoIterator for &'a mut Array<T, C> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.data.as_mut().iter_mut()
    }
}
