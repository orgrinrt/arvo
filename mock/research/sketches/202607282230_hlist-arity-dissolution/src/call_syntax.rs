//! Shape F: literal `f(a, b)` on a typestate-carrying wrapper, no feature gates.
//!
//! `core::ops::function` shows why the earlier attempts kept hitting a wall.
//! `Fn` carries `#[lang = "fn"]` and `#[rustc_paren_sugar]`: the call syntax is a
//! lang-item attribute, not something a user trait can attach. Implementing `Fn`
//! directly needs `unboxed_closures`, which is vetted forbidden (#29625,
//! `S-tracking-design-concerns`, open since 2015).
//!
//! But **call position autoderefs**, and `Deref` is stable to implement. A
//! wrapper that derefs to the closure it holds is callable with ordinary syntax,
//! at any arity, because the arity lives in the closure and never touches the
//! wrapper.
//!
//! That splits the problem cleanly:
//!
//! - **Invocation** is one `Deref` impl with no arity in it.
//! - **Validation** is a marker trait relating the list to the callable. Those
//!   impls are per-arity, but they are empty markers: they never dispatch and
//!   never run. The arity survives only as a compile-time table.
//!
//! Without the validation bound the typestate is decoration and can lie; that
//! failure is recorded in FINDINGS.md because it compiled cleanly.

use super::{Bool, Cons, Empty};
use core::marker::PhantomData;
use core::ops::Deref;

/// Does this list describe this callable? Marker only: no methods, no bodies.
///
/// One impl per arity, macro-generated in a real crate. They exist solely so a
/// mismatched typestate fails to compile.
pub trait Describes<F> {}

impl<A, F: Fn(&A) -> Bool> Describes<F> for Cons<A, Empty> {}
impl<A, B, F: Fn(&A, &B) -> Bool> Describes<F> for Cons<A, Cons<B, Empty>> {}
impl<A, B, C, F: Fn(&A, &B, &C) -> Bool> Describes<F> for Cons<A, Cons<B, Cons<C, Empty>>> {}
impl<A, B, C, D, F: Fn(&A, &B, &C, &D) -> Bool> Describes<F>
    for Cons<A, Cons<B, Cons<C, Cons<D, Empty>>>>
{
}

/// A predicate carrying its argument list as typestate.
pub struct Pred<L, F>(F, PhantomData<L>);

impl<L: Describes<F>, F> Pred<L, F> {
    /// The bound is the whole point: a list that does not describe the closure
    /// cannot be constructed.
    #[inline(always)]
    pub fn new(f: F) -> Self {
        Pred(f, PhantomData)
    }
}

/// One impl, no arity. This is what makes `p(a, b)` legal.
impl<L, F> Deref for Pred<L, F> {
    type Target = F;
    #[inline(always)]
    fn deref(&self) -> &F {
        &self.0
    }
}

pub type L1<A> = Cons<A, Empty>;
pub type L2<A, B> = Cons<A, Cons<B, Empty>>;
pub type L3<A, B, C> = Cons<A, Cons<B, Cons<C, Empty>>>;
pub type L4<A, B, C, D> = Cons<A, Cons<B, Cons<C, Cons<D, Empty>>>>;

#[cfg(test)]
mod tests {
    use super::*;

    // Asymmetric throughout, so argument order is genuinely under test.

    #[test]
    fn literal_call_syntax_at_arity_one() {
        let is_even = Pred::<L1<u32>, _>::new(|a: &u32| Bool(a % 2 == 0));
        assert!(is_even(&4).0);
        assert!(!is_even(&5).0);
    }

    #[test]
    fn literal_call_syntax_respects_argument_order() {
        let lt = Pred::<L2<u32, u32>, _>::new(|a: &u32, b: &u32| Bool(a < b));
        assert!(lt(&1, &2).0);
        assert!(!lt(&2, &1).0, "order must survive the deref");
    }

    #[test]
    fn heterogeneous_and_positional() {
        let f = Pred::<L3<i32, char, i32>, _>::new(|lo: &i32, t: &char, hi: &i32| {
            Bool(lo < hi && *t == 'x')
        });
        assert!(f(&1, &'x', &9).0);
        assert!(!f(&9, &'x', &1).0, "lo and hi swapped must fail");
        assert!(!f(&1, &'y', &9).0);
    }

    #[test]
    fn arity_four_adds_nothing_to_the_wrapper() {
        let asc = Pred::<L4<u8, u8, u8, u8>, _>::new(|a: &u8, b: &u8, c: &u8, d: &u8| {
            Bool(a < b && b < c && c < d)
        });
        assert!(asc(&1, &2, &3, &4).0);
        assert!(!asc(&1, &2, &4, &3).0);
    }

    #[test]
    fn capturing_closures_survive_the_wrapper() {
        let threshold = 10u32;
        let over = Pred::<L1<u32>, _>::new(move |a: &u32| Bool(*a > threshold));
        assert!(over(&11).0);
        assert!(!over(&10).0);
    }

    #[test]
    fn plain_fn_items_work_too() {
        fn shorter(a: &&str, b: &&str) -> Bool {
            Bool(a.len() < b.len())
        }
        let p = Pred::<L2<&str, &str>, _>::new(shorter);
        assert!(p(&"ab", &"abc").0);
        assert!(!p(&"abc", &"ab").0);
    }

    #[test]
    fn the_typestate_is_readable_by_a_consumer_bound() {
        fn takes_binary<A, B, F>(p: &Pred<L2<A, B>, F>, a: A, b: B) -> Bool
        where
            F: Fn(&A, &B) -> Bool,
        {
            p(&a, &b)
        }
        let lt = Pred::<L2<u32, u32>, _>::new(|a: &u32, b: &u32| Bool(a < b));
        assert!(takes_binary(&lt, 1u32, 2u32).0);
        assert!(!takes_binary(&lt, 2u32, 1u32).0);
    }

    #[test]
    fn two_predicates_at_one_arity_stay_distinct() {
        let lt = Pred::<L2<u32, u32>, _>::new(|a: &u32, b: &u32| Bool(a < b));
        let gt = Pred::<L2<u32, u32>, _>::new(|a: &u32, b: &u32| Bool(a > b));
        assert!(lt(&1, &2).0);
        assert!(!gt(&1, &2).0);
        assert!(gt(&2, &1).0);
    }
}
