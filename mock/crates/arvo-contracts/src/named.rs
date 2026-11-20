//! Two contracts named ahead of the families that will implement them.

/// The reciprocal, over an operand at which zero cannot be met.
///
/// `Nonzero` is the instance's own type for a member other than zero, as division takes a divisor,
/// so no reciprocal meets zero and none checks for it at run time. No instance ships yet, and no
/// law is stated until one does.
pub const trait Reciprocal: Copy {
    /// A member other than zero.
    type Nonzero: Copy;

    /// The reciprocal of the operand.
    fn reciprocal(operand: Self::Nonzero) -> Self;
}

/// The square root. No instance ships yet, and no law is stated until one does.
pub const trait SquareRoot: Copy {
    /// The square root.
    fn square_root(self) -> Self;
}
