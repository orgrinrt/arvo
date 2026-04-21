//! Named predicate bounds for consumer callbacks.
//!
//! `Pred<A>`, `Pred2<A, B>`, `Pred3<A, B, C>` name the shape
//! "callable returning Bool" at 1, 2, and 3 arities. They are
//! trait-alias shims over `Fn(&A) -> Bool` / `Fn(&A, &B) -> Bool` /
//! `Fn(&A, &B, &C) -> Bool`: the bound `impl Pred2<A, B>` is
//! equivalent to `impl Fn(&A, &B) -> Bool`, but reads as a named
//! concept at consumer sites.
//!
//! Callers invoke via either the `test` method or direct `Fn` call
//! syntax; both work because `Fn` is a supertrait.

use crate::newtype::Bool;

/// Unary predicate. Equivalent to `impl Fn(&A) -> Bool`.
pub trait Pred<A>: Fn(&A) -> Bool {
    /// Apply the predicate. Default forwards to the `Fn` call.
    #[inline(always)]
    fn test(&self, a: &A) -> Bool {
        self(a)
    }
}
impl<A, F: Fn(&A) -> Bool> Pred<A> for F {}

/// Binary predicate. Equivalent to `impl Fn(&A, &B) -> Bool`.
pub trait Pred2<A, B>: Fn(&A, &B) -> Bool {
    /// Apply the predicate. Default forwards to the `Fn` call.
    #[inline(always)]
    fn test(&self, a: &A, b: &B) -> Bool {
        self(a, b)
    }
}
impl<A, B, F: Fn(&A, &B) -> Bool> Pred2<A, B> for F {}

/// Ternary predicate. Equivalent to `impl Fn(&A, &B, &C) -> Bool`.
pub trait Pred3<A, B, C>: Fn(&A, &B, &C) -> Bool {
    /// Apply the predicate. Default forwards to the `Fn` call.
    #[inline(always)]
    fn test(&self, a: &A, b: &B, c: &C) -> Bool {
        self(a, b, c)
    }
}
impl<A, B, C, F: Fn(&A, &B, &C) -> Bool> Pred3<A, B, C> for F {}
