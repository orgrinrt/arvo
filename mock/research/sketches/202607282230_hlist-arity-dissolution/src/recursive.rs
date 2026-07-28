//! Fully recursive application: ZERO per-arity impls.
//!
//! `Apply` in `lib.rs` dissolved the arity from the public trait but kept one
//! impl per arity on the carrier. The hlist is recursive by construction, so
//! the application should recurse over it too. That requires the function side
//! to peel one argument at a time, which means the predicate is curried.
//!
//! Two impls total, and they do not mention an arity: a base case for the empty
//! list and a step case for a cons cell.

use super::{Bool, Cons, Empty};

/// Recursive application over an hlist. Two impls, no arities.
pub trait Chain<F> {
    /// The value-level argument list. Itself recursive: `(A, (B, (C, ())))`.
    type Args;
    fn run(f: &F, args: Self::Args) -> Bool;
}

/// Base case. Every argument consumed, so `F` is already the answer.
impl Chain<Bool> for Empty {
    type Args = ();
    #[inline(always)]
    fn run(f: &Bool, _: ()) -> Bool {
        Bool(f.0)
    }
}

/// Step case. Consume one argument; whatever comes back handles the tail.
impl<H, T, F, G> Chain<F> for Cons<H, T>
where
    F: Fn(&H) -> G,
    T: Chain<G>,
{
    type Args = (H, <T as Chain<G>>::Args);
    #[inline(always)]
    fn run(f: &F, args: Self::Args) -> Bool {
        let g = f(&args.0);
        <T as Chain<G>>::run(&g, args.1)
    }
}

/// The public surface, unchanged in shape from `Pred`: one trait, one blanket
/// impl, no arity.
pub trait Curried<L> {
    type Args;
    fn call(&self, args: Self::Args) -> Bool;
}

impl<L: Chain<F>, F> Curried<L> for F {
    type Args = <L as Chain<F>>::Args;
    #[inline(always)]
    fn call(&self, args: Self::Args) -> Bool {
        <L as Chain<F>>::run(self, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{P1, P2, P3, P4};

    // Same asymmetric discipline as lib.rs: a symmetric predicate would pass
    // even if the recursion threaded arguments in the wrong order.

    #[test]
    fn unary_recurses() {
        let is_even = |a: &u32| Bool(a % 2 == 0);
        assert!(<_ as Curried<P1<u32>>>::call(&is_even, (4, ())).0);
        assert!(!<_ as Curried<P1<u32>>>::call(&is_even, (5, ())).0);
    }

    #[test]
    fn binary_recurses_and_respects_order() {
        let lt = |a: &u32| {
            let a = *a;
            move |b: &u32| Bool(a < *b)
        };
        assert!(<_ as Curried<P2<u32, u32>>>::call(&lt, (1, (2, ()))).0);
        assert!(
            !<_ as Curried<P2<u32, u32>>>::call(&lt, (2, (1, ()))).0,
            "if this passes the recursion threaded arguments reversed"
        );
    }

    #[test]
    fn ternary_recurses() {
        let ascending = |a: &i32| {
            let a = *a;
            move |b: &i32| {
                let b = *b;
                move |c: &i32| Bool(a < b && b < *c)
            }
        };
        assert!(<_ as Curried<P3<i32, i32, i32>>>::call(&ascending, (1, (2, (3, ())))).0);
        assert!(!<_ as Curried<P3<i32, i32, i32>>>::call(&ascending, (1, (3, (2, ())))).0);
        assert!(!<_ as Curried<P3<i32, i32, i32>>>::call(&ascending, (3, (2, (1, ())))).0);
    }

    #[test]
    fn arity_four_needs_no_new_impl_at_all() {
        // The point of this file: nothing was added anywhere to support four.
        let asc = |a: &u8| {
            let a = *a;
            move |b: &u8| {
                let b = *b;
                move |c: &u8| {
                    let c = *c;
                    move |d: &u8| Bool(a < b && b < c && c < *d)
                }
            }
        };
        assert!(<_ as Curried<P4<u8, u8, u8, u8>>>::call(&asc, (1, (2, (3, (4, ()))))).0);
        assert!(!<_ as Curried<P4<u8, u8, u8, u8>>>::call(&asc, (1, (2, (4, (3, ()))))).0);
    }

    #[test]
    fn arity_seven_also_needs_no_new_impl() {
        // Well past anything the hand-written family ever offered.
        type P7 = Cons<u8, Cons<u8, Cons<u8, Cons<u8, Cons<u8, Cons<u8, Cons<u8, Empty>>>>>>>;
        let all_ones = |a: &u8| {
            let a = *a;
            move |b: &u8| {
                let b = *b;
                move |c: &u8| {
                    let c = *c;
                    move |d: &u8| {
                        let d = *d;
                        move |e: &u8| {
                            let e = *e;
                            move |f: &u8| {
                                let f = *f;
                                move |g: &u8| {
                                    Bool(
                                        a == 1
                                            && b == 1
                                            && c == 1
                                            && d == 1
                                            && e == 1
                                            && f == 1
                                            && *g == 1,
                                    )
                                }
                            }
                        }
                    }
                }
            }
        };
        assert!(<_ as Curried<P7>>::call(&all_ones, (1, (1, (1, (1, (1, (1, (1, ())))))))).0);
        assert!(
            !<_ as Curried<P7>>::call(&all_ones, (1, (1, (1, (1, (1, (1, (2, ())))))))).0,
            "the seventh argument must reach the innermost closure"
        );
    }

    #[test]
    fn heterogeneous_payloads_recurse() {
        let f = |name: &&str| {
            let n = name.len();
            move |count: &usize| {
                let ok = n == *count;
                move |flag: &bool| Bool(ok && *flag)
            }
        };
        assert!(<_ as Curried<P3<&str, usize, bool>>>::call(&f, ("abcd", (4, (true, ())))).0);
        assert!(!<_ as Curried<P3<&str, usize, bool>>>::call(&f, ("abcd", (5, (true, ())))).0);
        assert!(!<_ as Curried<P3<&str, usize, bool>>>::call(&f, ("abcd", (4, (false, ())))).0);
    }
}
