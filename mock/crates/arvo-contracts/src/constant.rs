//! The member at a constant slot, a door that opens only at build.

/// The member at slot index `V`.
///
/// A `V` naming no member is refused at build, as a failed constant once the item is used, so a
/// bound on `Constant<V>` promises nothing a caller can read before a build, and `cargo check`
/// alone does not catch it. Nothing here is validated at run time.
///
/// `V` is a slot index rather than a value, and the integer at the parameter is the one position
/// where a bare integer stands.
pub trait Constant<const V: i128> {
    /// The member.
    const MEMBER: Self;
}
