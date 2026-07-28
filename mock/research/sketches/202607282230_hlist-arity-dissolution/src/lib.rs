//! Sketch: dissolve a hand-capped arity family into one trait over an hlist.
//!
//! The problem: `Pred` / `Pred2` / `Pred3` in the arvo facade are three
//! hand-written arities, capped at three because three is what someone typed.
//! That is the shape D4 already rejected for capacities ("needs impls generated
//! per arity and caps rank at whatever is written").
//!
//! The failed attempts and why, in order, are in FINDINGS.md. The move that
//! works is the `Capacity` move: stop dispatching on the wildcard `F`, and put
//! the dispatch on a named carrier that answers for itself. The hlist already
//! knows its own arity.
//!
//! NO feature gates. `min_specialization` was tried and does not help; the
//! reason is structural and is recorded in FINDINGS.md.

#![no_std]

#[cfg(test)]
extern crate std;

pub mod call_syntax;
pub mod ergonomics;
pub mod inferred;
pub mod recursive;
pub mod recursive_describes;

pub struct Bool(pub bool);

pub struct Empty;
pub struct Cons<H, T>(core::marker::PhantomData<(H, T)>);

// ---------------------------------------------------------------------------
// The carrier. Impls are keyed on the LIST type, which is structurally distinct
// per arity, so nothing overlaps and no blanket covers the wildcard.
// ---------------------------------------------------------------------------

pub trait Apply<F> {
    type Args;
    fn apply(f: &F, args: Self::Args) -> Bool;
}

impl<A, F: Fn(&A) -> Bool> Apply<F> for Cons<A, Empty> {
    type Args = (A,);
    fn apply(f: &F, a: (A,)) -> Bool {
        f(&a.0)
    }
}

impl<A, B, F: Fn(&A, &B) -> Bool> Apply<F> for Cons<A, Cons<B, Empty>> {
    type Args = (A, B);
    fn apply(f: &F, a: (A, B)) -> Bool {
        f(&a.0, &a.1)
    }
}

impl<A, B, C, F: Fn(&A, &B, &C) -> Bool> Apply<F> for Cons<A, Cons<B, Cons<C, Empty>>> {
    type Args = (A, B, C);
    fn apply(f: &F, a: (A, B, C)) -> Bool {
        f(&a.0, &a.1, &a.2)
    }
}

impl<A, B, C, D, F: Fn(&A, &B, &C, &D) -> Bool> Apply<F>
    for Cons<A, Cons<B, Cons<C, Cons<D, Empty>>>>
{
    type Args = (A, B, C, D);
    fn apply(f: &F, a: (A, B, C, D)) -> Bool {
        f(&a.0, &a.1, &a.2, &a.3)
    }
}

// ---------------------------------------------------------------------------
// The public surface. ONE trait, ONE blanket impl, no arity in either.
//
// `Args` must live on `Pred` rather than as a where-clause on the trait's
// generics. With `pub trait Pred<L: Apply<Self>>` every call site has to restate
// the `Fn` bound, which defeats the whole thing. Recorded because it is the
// non-obvious part.
// ---------------------------------------------------------------------------

pub trait Pred<L> {
    type Args;
    fn test(&self, args: Self::Args) -> Bool;
}

impl<L: Apply<F>, F> Pred<L> for F {
    type Args = <L as Apply<F>>::Args;
    #[inline(always)]
    fn test(&self, args: Self::Args) -> Bool {
        L::apply(self, args)
    }
}

/// Domain aliases, the way D7 says each domain aliases the cell and the leaf.
pub type P1<A> = Cons<A, Empty>;
pub type P2<A, B> = Cons<A, Cons<B, Empty>>;
pub type P3<A, B, C> = Cons<A, Cons<B, Cons<C, Empty>>>;
pub type P4<A, B, C, D> = Cons<A, Cons<B, Cons<C, Cons<D, Empty>>>>;

// ---------------------------------------------------------------------------
// Consumer-shaped functions, the shape `arvo-comb` actually uses.
// ---------------------------------------------------------------------------

pub fn feasible_binary<F: Pred<P2<u8, u16>, Args = (u8, u16)>>(f: F, a: u8, b: u16) -> Bool {
    f.test((a, b))
}

pub fn feasible_generic<A, B, F>(f: F, a: A, b: B) -> Bool
where
    F: Pred<P2<A, B>, Args = (A, B)>,
{
    f.test((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every predicate below is ASYMMETRIC on purpose. A symmetric one would
    // pass even if the tuple mapped arguments in the wrong order, which is the
    // bug this whole sketch is most likely to have.

    #[test]
    fn unary_computes_and_discriminates() {
        let is_even = |a: &u32| Bool(a % 2 == 0);
        assert!(<_ as Pred<P1<u32>>>::test(&is_even, (4,)).0);
        assert!(!<_ as Pred<P1<u32>>>::test(&is_even, (5,)).0);
    }

    #[test]
    fn binary_respects_argument_order() {
        let lt = |a: &u32, b: &u32| Bool(a < b);
        assert!(<_ as Pred<P2<u32, u32>>>::test(&lt, (1, 2)).0, "1 < 2");
        assert!(
            !<_ as Pred<P2<u32, u32>>>::test(&lt, (2, 1)).0,
            "2 < 1 must be false; if this passes the tuple mapped args reversed"
        );
    }

    #[test]
    fn ternary_respects_full_order() {
        let ascending = |a: &i32, b: &i32, c: &i32| Bool(a < b && b < c);
        assert!(<_ as Pred<P3<i32, i32, i32>>>::test(&ascending, (1, 2, 3)).0);
        assert!(!<_ as Pred<P3<i32, i32, i32>>>::test(&ascending, (1, 3, 2)).0);
        assert!(!<_ as Pred<P3<i32, i32, i32>>>::test(&ascending, (3, 2, 1)).0);
    }

    #[test]
    fn quaternary_extends_without_touching_the_public_trait() {
        let ascending = |a: &u8, b: &u8, c: &u8, d: &u8| Bool(a < b && b < c && c < d);
        assert!(<_ as Pred<P4<u8, u8, u8, u8>>>::test(&ascending, (1, 2, 3, 4)).0);
        assert!(!<_ as Pred<P4<u8, u8, u8, u8>>>::test(&ascending, (1, 2, 4, 3)).0);
    }

    #[test]
    fn heterogeneous_payloads_keep_their_types() {
        // Distinct types per position, and the predicate reads each as itself.
        let f = |name: &&str, count: &usize, flag: &bool| Bool(name.len() == *count && *flag);
        assert!(<_ as Pred<P3<&str, usize, bool>>>::test(&f, ("abcd", 4, true)).0);
        assert!(!<_ as Pred<P3<&str, usize, bool>>>::test(&f, ("abcd", 5, true)).0);
        assert!(!<_ as Pred<P3<&str, usize, bool>>>::test(&f, ("abcd", 4, false)).0);
    }

    #[test]
    fn asymmetric_heterogeneous_catches_positional_swaps() {
        // If positions 0 and 1 were swapped this would not even typecheck, so
        // use two same-typed positions around a distinct one to make a swap
        // expressible and therefore catchable.
        let f = |lo: &i32, tag: &char, hi: &i32| Bool(lo < hi && *tag == 'x');
        assert!(<_ as Pred<P3<i32, char, i32>>>::test(&f, (1, 'x', 9)).0);
        assert!(
            !<_ as Pred<P3<i32, char, i32>>>::test(&f, (9, 'x', 1)).0,
            "lo and hi swapped must fail"
        );
        assert!(!<_ as Pred<P3<i32, char, i32>>>::test(&f, (1, 'y', 9)).0);
    }

    #[test]
    fn capturing_closures_work() {
        let threshold = 10u32;
        let over = |a: &u32| Bool(*a > threshold);
        assert!(<_ as Pred<P1<u32>>>::test(&over, (11,)).0);
        assert!(!<_ as Pred<P1<u32>>>::test(&over, (10,)).0);
    }

    #[test]
    fn plain_fn_items_work_not_only_closures() {
        fn shorter(a: &&str, b: &&str) -> Bool {
            Bool(a.len() < b.len())
        }
        assert!(<_ as Pred<P2<&str, &str>>>::test(&shorter, ("ab", "abc")).0);
        assert!(!<_ as Pred<P2<&str, &str>>>::test(&shorter, ("abc", "ab")).0);
    }

    #[test]
    fn consumer_shaped_wrappers_resolve() {
        let lt = |a: &u8, b: &u16| Bool(u16::from(*a) < *b);
        assert!(feasible_binary(lt, 1, 2).0);
        assert!(!feasible_binary(lt, 2, 1).0);

        // and the fully generic wrapper, which is the arvo-comb shape
        let ne = |a: &char, b: &char| Bool(a != b);
        assert!(feasible_generic(ne, 'a', 'b').0);
        assert!(!feasible_generic(ne, 'a', 'a').0);
    }

    #[test]
    fn several_distinct_predicates_coexist_at_one_arity() {
        // Two different closures at the same arity and payload, to confirm the
        // blanket impl is not somehow collapsing them.
        let lt = |a: &u32, b: &u32| Bool(a < b);
        let gt = |a: &u32, b: &u32| Bool(a > b);
        assert!(<_ as Pred<P2<u32, u32>>>::test(&lt, (1, 2)).0);
        assert!(!<_ as Pred<P2<u32, u32>>>::test(&gt, (1, 2)).0);
        assert!(<_ as Pred<P2<u32, u32>>>::test(&gt, (2, 1)).0);
    }
}
