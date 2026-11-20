//! The total order of the members.

use core::cmp::Ordering;

use arvo_format::Bool;

/// The total order of the members, which is the order of their representatives on the ambient
/// line.
///
/// Under a wrapping completion that order exists and is total, and it is not invariant under
/// translation: no finite cyclic group admits one that is. Whether a sum keeps the order is
/// `MonotoneSum`'s to say, not this contract's.
pub const trait Order: Copy {
    /// The order of `self` against `rhs`.
    fn order(self, rhs: Self) -> Ordering;

    /// Whether `self` is below `rhs`.
    fn lt(self, rhs: Self) -> Bool {
        Bool::of(matches!(self.order(rhs), Ordering::Less))
    }

    /// Whether `self` is below or at `rhs`.
    fn le(self, rhs: Self) -> Bool {
        Bool::of(!matches!(self.order(rhs), Ordering::Greater))
    }

    /// Whether `self` is above `rhs`.
    fn gt(self, rhs: Self) -> Bool {
        Bool::of(matches!(self.order(rhs), Ordering::Greater))
    }

    /// Whether `self` is above or at `rhs`.
    fn ge(self, rhs: Self) -> Bool {
        Bool::of(!matches!(self.order(rhs), Ordering::Less))
    }

    /// The lesser of the two, `self` where they are equal.
    fn min(self, rhs: Self) -> Self {
        if matches!(self.order(rhs), Ordering::Greater) { rhs } else { self }
    }

    /// The greater of the two, `self` where they are equal.
    fn max(self, rhs: Self) -> Self {
        if matches!(self.order(rhs), Ordering::Less) { rhs } else { self }
    }
}
