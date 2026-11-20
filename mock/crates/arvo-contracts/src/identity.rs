//! The identity of an operation, as one associated constant.

/// The member that leaves every member unmoved under the operation `M` names.
///
/// An instance whose format has no such member refuses at build, as a failed constant naming the
/// missing identity, rather than by having no impl.
pub trait Identity<M> {
    /// The identity.
    const IDENTITY: Self;
}

/// The operation of the sum, whose identity is zero.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Additive;

/// The operation of the product, whose identity is one.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Multiplicative;
