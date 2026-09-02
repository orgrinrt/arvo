//! Shape G: recursive validation. Two impls, zero arity anywhere, real call
//! syntax, no feature gates.
//!
//! Shape F pushed the arity into marker impls that never run, which is good but
//! still a table. This removes the table entirely.
//!
//! The wall it gets around: `Fn(&A, &B) -> Bool` names both types **in one
//! bound**, and generating that bound from a recursive structure needs variadic
//! generics, which this toolchain does not have. Curry the closure and the
//! recursion has somewhere to go, one argument per step.
//!
//! `Deref` still supplies real call syntax, so the price is `f(a)(b)` rather
//! than `f(a, b)`. That is the entire trade against Shape F.
//!
//! Arguments are taken by value here rather than by reference, which is what
//! makes currying read cleanly. Fine for the `Copy` scalars a predicate over
//! arvo primitives actually sees; a large payload would want Shape F.

use super::{Bool, Cons, Empty};
use core::marker::PhantomData;
use core::ops::Deref;

/// Does this list describe this callable? **Two impls. No arity.**
pub trait Describes<F> {}

/// Base: every argument consumed, so what remains is the answer.
impl Describes<Bool> for Empty {}

/// Step: consume one argument, and whatever comes back describes the tail.
impl<H, T, F, G> Describes<F> for Cons<H, T>
where
    F: Fn(H) -> G,
    T: Describes<G>,
{
}

/// A predicate carrying its argument list as typestate.
pub struct Pred<L, F>(F, PhantomData<L>);

impl<L: Describes<F>, F> Pred<L, F> {
    #[inline(always)]
    pub fn new(f: F) -> Self {
        Pred(f, PhantomData)
    }
}

impl<L, F> Deref for Pred<L, F> {
    type Target = F;
    #[inline(always)]
    fn deref(&self) -> &F {
        &self.0
    }
}

pub type G1<A> = Cons<A, Empty>;
pub type G2<A, B> = Cons<A, Cons<B, Empty>>;
pub type G3<A, B, C> = Cons<A, Cons<B, Cons<C, Empty>>>;
pub type G5<A, B, C, D, E> = Cons<A, Cons<B, Cons<C, Cons<D, Cons<E, Empty>>>>>;

#[cfg(test)]
mod tests {
    use super::*;

    // Asymmetric throughout, so argument order is genuinely under test.

    #[test]
    fn unary() {
        let e = Pred::<G1<u32>, _>::new(|a: u32| Bool(a % 2 == 0));
        assert!(e(4).0);
        assert!(!e(5).0);
    }

    #[test]
    fn binary_respects_order() {
        let lt = Pred::<G2<u32, u32>, _>::new(|a: u32| move |b: u32| Bool(a < b));
        assert!(lt(1)(2).0);
        assert!(!lt(2)(1).0, "order must survive the recursion");
    }

    #[test]
    fn ternary_heterogeneous_and_positional() {
        let f = Pred::<G3<i32, char, i32>, _>::new(|lo: i32| {
            move |t: char| move |hi: i32| Bool(lo < hi && t == 'x')
        });
        assert!(f(1)('x')(9).0);
        assert!(!f(9)('x')(1).0, "lo and hi swapped must fail");
        assert!(!f(1)('y')(9).0);
    }

    #[test]
    fn arity_five_adds_nothing_anywhere() {
        let g = Pred::<G5<u8, u8, u8, u8, u8>, _>::new(|a: u8| {
            move |b: u8| {
                move |c: u8| move |d: u8| move |e: u8| Bool(a < b && b < c && c < d && d < e)
            }
        });
        assert!(g(1)(2)(3)(4)(5).0);
        assert!(!g(1)(2)(3)(5)(4).0);
    }

    #[test]
    fn capturing_closures_survive() {
        let threshold = 10u32;
        let over = Pred::<G1<u32>, _>::new(move |a: u32| Bool(a > threshold));
        assert!(over(11).0);
        assert!(!over(10).0);
    }

    #[test]
    fn two_predicates_at_one_arity_stay_distinct() {
        let lt = Pred::<G2<u32, u32>, _>::new(|a: u32| move |b: u32| Bool(a < b));
        let gt = Pred::<G2<u32, u32>, _>::new(|a: u32| move |b: u32| Bool(a > b));
        assert!(lt(1)(2).0);
        assert!(!gt(1)(2).0);
        assert!(gt(2)(1).0);
    }

    // The mismatches below are checked by compile-fail rather than assertion.
    // All three were verified rejected on 2026-07-28; see FINDINGS.md. They stay
    // here as prose because a compile-fail harness is not set up in this sketch,
    // and a commented-out test that nothing runs would be worse than a note.
    //
    //   too few:    Pred::<G2<u32,u32>,_>::new(|a: u32| Bool(a > 0))
    //   wrong type: Pred::<G2<u32,u32>,_>::new(|a: u8| move |b: u32| Bool(true))
    //   too many:   Pred::<G1<u32>,_>::new(|a: u32| move |b: u32| Bool(a < b))
}
