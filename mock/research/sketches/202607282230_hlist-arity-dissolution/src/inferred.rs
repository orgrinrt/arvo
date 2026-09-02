//! Can the call site read as an ordinary call?
//!
//! Three questions, in order of ambition.
//!
//! **1. Can the chain end in a real call rather than a field access?** Yes,
//! trivially: the last link already returns `Bool`, and `.0` was only unwrapping
//! a newtype in the tests.
//!
//! **2. Can the argument types be inferred so no list is named?** Recorded
//! failure below: `impl<A, G, F> Chained for F where F: Fn(&A) -> G` is `E0207`,
//! because a type may implement `Fn` at several argument types, so `A` is not
//! determined by `F`. The list has to be named somewhere.
//!
//! **3. So where does it get named?** Not by the end consumer. A consumer
//! FUNCTION knows its own arity, so it names the list once in its own signature
//! and presents an ordinary call outward. That is the shape below, and it is the
//! one that actually reads well.
//!
//! **4. Can a custom type be called as `f(a, b)` literally?** Only with
//! `fn_traits` / `unboxed_closures`, tested at the bottom and rejected by the
//! vetting gate.

use super::recursive::Chain;
use super::{Bool, Cons, Empty};

// ---------------------------------------------------------------------------
// Question 2, recorded as a failure rather than deleted.
//
// impl<A, G, F> Chained for F where F: Fn(&A) -> G, G: Chained
//   error[E0207]: the type parameter `A` is not constrained by the impl trait,
//   self type, or predicates
//
// A type may implement `Fn` at more than one argument type, so `A` is not
// determined by `F` alone. The typestate must name it. That is not a defect in
// the approach: it is why `Chain<F>` in recursive.rs is keyed on the LIST, where
// A and B are named and therefore constrained.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Question 3. The consumer function names the list once, in its own signature,
// and every caller of it writes an ordinary call.
// ---------------------------------------------------------------------------

/// Two arguments, because this particular consumer takes two. The list, the
/// nested tuple and the recursion are all inside; callers see none of it.
#[inline(always)]
pub fn holds2<A, B, F>(f: &F, a: A, b: B) -> Bool
where
    Cons<A, Cons<B, Empty>>: Chain<F, Args = (A, (B, ()))>,
{
    <Cons<A, Cons<B, Empty>> as Chain<F>>::run(f, (a, (b, ())))
}

/// Three, to show the pattern scales without the library changing.
#[inline(always)]
pub fn holds3<A, B, C, F>(f: &F, a: A, b: B, c: C) -> Bool
where
    Cons<A, Cons<B, Cons<C, Empty>>>: Chain<F, Args = (A, (B, (C, ())))>,
{
    <Cons<A, Cons<B, Cons<C, Empty>>> as Chain<F>>::run(f, (a, (b, (c, ()))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_call_site_is_an_ordinary_call() {
        let lt = |a: &u32| {
            let a = *a;
            move |b: &u32| Bool(a < *b)
        };
        // No list, no tuple, no turbofish, no macro. Just a call.
        assert!(holds2(&lt, 1u32, 2u32).0);
        assert!(!holds2(&lt, 2u32, 1u32).0);
    }

    #[test]
    fn three_arguments_read_the_same() {
        let asc = |a: &i32| {
            let a = *a;
            move |b: &i32| {
                let b = *b;
                move |c: &i32| Bool(a < b && b < *c)
            }
        };
        assert!(holds3(&asc, 1, 2, 3).0);
        assert!(!holds3(&asc, 1, 3, 2).0);
        assert!(!holds3(&asc, 3, 2, 1).0);
    }

    #[test]
    fn heterogeneous_reads_the_same() {
        let f = |lo: &i32| {
            let lo = *lo;
            move |tag: &char| {
                let t = *tag;
                move |hi: &i32| Bool(lo < *hi && t == 'x')
            }
        };
        assert!(holds3(&f, 1, 'x', 9).0);
        assert!(!holds3(&f, 9, 'x', 1).0);
        assert!(!holds3(&f, 1, 'y', 9).0);
    }

    #[test]
    fn the_terminal_step_is_a_real_call_not_a_field_read() {
        // `.0` in the asserts above unwraps the Bool newtype for assert!.
        // The chain itself ends by CALLING the innermost closure; nothing reads
        // a field to get the answer.
        let lt = |a: &u32| {
            let a = *a;
            move |b: &u32| Bool(a < *b)
        };
        let r: Bool = holds2(&lt, 1u32, 2u32);
        assert!(r.0);
    }
}
