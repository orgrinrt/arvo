//! The sum, the difference and the product, each with the predicate that asks whether the exact
//! result is a member.
//!
//! Each method is total under the instance's own completion. An instance whose debug check aborts a
//! debug build when a result leaves the range does that through these methods as it does through
//! its own, so the law is stated per debug check, and the `fits` predicate is what a caller asks
//! ahead of it.

use arvo_format::Bool;

/// The sum under the instance's own completion.
pub const trait Sum: Copy {
    /// The sum of the two members.
    fn sum(self, rhs: Self) -> Self;

    /// Whether the exact sum is a member, before the completion is consulted.
    fn fits_sum(self, rhs: Self) -> Bool;
}

/// The difference under the instance's own completion.
pub const trait Difference: Copy {
    /// The difference of the two members, `self` less `rhs`.
    fn difference(self, rhs: Self) -> Self;

    /// Whether the exact difference is a member, before the completion is consulted.
    fn fits_difference(self, rhs: Self) -> Bool;
}

/// The product under the instance's own completion.
pub const trait Product: Copy {
    /// The product of the two members.
    fn product(self, rhs: Self) -> Self;

    /// Whether the exact product, rounded by the instance's mode, is a member, before the
    /// completion is consulted.
    fn fits_product(self, rhs: Self) -> Bool;
}
