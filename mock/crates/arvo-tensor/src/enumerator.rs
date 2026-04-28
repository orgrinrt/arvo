//! `Enumerator` trait: typed-index parallel of `core::iter::Iterator::enumerate`.

use arvo::USize;

/// Parallel of `core::iter::Iterator::enumerate` that yields `USize`
/// instead of raw `usize`.
///
/// Implemented as a blanket `impl<T: IntoIterator>`: any iterable
/// (including `&Array<T, N>`, `&Matrix<W, N>`, plain slices, and types
/// outside arvo-tensor) picks up `.enumerated()` automatically.
pub trait Enumerator {
    type Item;
    /// Yield `(index: USize, item)` pairs.
    fn enumerated(self) -> impl Iterator<Item = (USize, Self::Item)>;
}

impl<T: IntoIterator> Enumerator for T {
    type Item = <T as IntoIterator>::Item;

    #[inline]
    fn enumerated(self) -> impl Iterator<Item = (USize, Self::Item)> {
        self.into_iter()
            .enumerate()
            .map(|(i, item)| (USize(i), item))
    }
}
