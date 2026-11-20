//! Where a member sits against zero, and its absolute value.

use arvo_format::Bool;

/// Where a member sits against the additive identity.
///
/// An unsigned instance answers `is_negative` false for every member. A format without a zero
/// member has no sign, and the instance refuses at build.
pub const trait Sign: Copy {
    /// Whether the member is below zero.
    fn is_negative(self) -> Bool;

    /// Whether the member is zero.
    fn is_zero(self) -> Bool;

    /// Whether the member is above zero.
    fn is_positive(self) -> Bool;
}

/// The absolute value under the instance's own completion.
///
/// Under wrap the least member of a signed format has no positive counterpart, and its magnitude
/// is itself, which is negative. Every other member's magnitude is not negative.
pub const trait Magnitude: Copy {
    /// The absolute value.
    fn magnitude(self) -> Self;
}
