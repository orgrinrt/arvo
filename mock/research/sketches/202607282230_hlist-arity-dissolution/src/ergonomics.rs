//! Can the currying and the nested argument list be made invisible?
//!
//! Shape B recurses with two impls and no arity, but it shows: predicates are
//! written curried, and the argument list is a nested tuple. Both are shapes a
//! `macro_rules!` repetition handles natively, with no cap, because macro
//! repetition IS the variadic mechanism the type system lacks.
//!
//! Nothing here adds a per-arity impl. Two macros, each defined once.

use super::recursive::Curried;
use super::Bool;

/// Build the value-level argument list. `args!(a, b, c)` is `(a, (b, (c, ())))`.
#[macro_export]
macro_rules! args {
    () => { () };
    ($head:expr $(, $rest:expr)* $(,)?) => {
        ($head, $crate::args!($($rest),*))
    };
}

/// Build the type-level list. `argl!(A, B, C)` is `Cons<A, Cons<B, Cons<C, Empty>>>`.
#[macro_export]
macro_rules! argl {
    () => { $crate::Empty };
    ($head:ty $(, $rest:ty)* $(,)?) => {
        $crate::Cons<$head, $crate::argl!($($rest),*)>
    };
}

/// Write a predicate as if it were n-ary; get the curried chain.
///
/// `pred!(a: u32, b: u32 => a < b)` expands to
/// `move |a: &u32| { let a = *a; move |b: &u32| Bool(a < *b) }`
/// except the recursion is in the macro, so there is no arity anywhere.
#[macro_export]
macro_rules! pred {
    // Base: last parameter. Its body produces the Bool.
    ($a:ident : $ta:ty => $body:expr) => {
        move |$a: &$ta| -> $crate::Bool { let $a = *$a; $crate::Bool($body) }
    };
    // Step: bind one parameter by value, recurse for the rest.
    ($a:ident : $ta:ty, $($rest:tt)+) => {
        move |$a: &$ta| { let $a = *$a; $crate::pred!($($rest)+) }
    };
}

// ---------------------------------------------------------------------------
// What a consumer actually writes, with the machinery hidden.
// ---------------------------------------------------------------------------

/// The `arvo-comb` shape, arity-free in its bound.
pub fn feasible<L, F>(f: &F, args: <F as Curried<L>>::Args) -> Bool
where
    F: Curried<L>,
{
    f.call(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_macro_builds_the_nested_list() {
        let a: (u8, (u16, (u32, ()))) = args!(1u8, 2u16, 3u32);
        assert_eq!(a.0, 1);
        assert_eq!((a.1).0, 2);
        assert_eq!(((a.1).1).0, 3);
        let empty: () = args!();
        let _ = empty;
    }

    #[test]
    fn pred_macro_reads_n_ary_and_respects_order() {
        let lt = pred!(a: u32, b: u32 => a < b);
        assert!(<_ as Curried<argl!(u32, u32)>>::call(&lt, args!(1u32, 2u32)).0);
        assert!(
            !<_ as Curried<argl!(u32, u32)>>::call(&lt, args!(2u32, 1u32)).0,
            "argument order must survive the macro"
        );
    }

    #[test]
    fn unary_through_the_macros() {
        let is_even = pred!(a: u32 => a % 2 == 0);
        assert!(<_ as Curried<argl!(u32)>>::call(&is_even, args!(4u32)).0);
        assert!(!<_ as Curried<argl!(u32)>>::call(&is_even, args!(5u32)).0);
    }

    #[test]
    fn ternary_through_the_macros() {
        let asc = pred!(a: i32, b: i32, c: i32 => a < b && b < c);
        assert!(<_ as Curried<argl!(i32, i32, i32)>>::call(&asc, args!(1, 2, 3)).0);
        assert!(!<_ as Curried<argl!(i32, i32, i32)>>::call(&asc, args!(1, 3, 2)).0);
        assert!(!<_ as Curried<argl!(i32, i32, i32)>>::call(&asc, args!(3, 2, 1)).0);
    }

    #[test]
    fn arity_six_needs_nothing_new_in_macro_or_impl() {
        let f = pred!(a: u8, b: u8, c: u8, d: u8, e: u8, g: u8 =>
            a < b && b < c && c < d && d < e && e < g);
        type L = argl!(u8, u8, u8, u8, u8, u8);
        assert!(<_ as Curried<L>>::call(&f, args!(1u8, 2, 3, 4, 5, 6)).0);
        assert!(
            !<_ as Curried<L>>::call(&f, args!(1u8, 2, 3, 4, 6, 5)).0,
            "the sixth argument must reach the innermost closure"
        );
    }

    #[test]
    fn heterogeneous_through_the_macros() {
        let f = pred!(lo: i32, tag: char, hi: i32 => lo < hi && tag == 'x');
        type L = argl!(i32, char, i32);
        assert!(<_ as Curried<L>>::call(&f, args!(1, 'x', 9)).0);
        assert!(!<_ as Curried<L>>::call(&f, args!(9, 'x', 1)).0);
        assert!(!<_ as Curried<L>>::call(&f, args!(1, 'y', 9)).0);
    }

    #[test]
    fn the_consumer_facing_wrapper_reads_cleanly() {
        let lt = pred!(a: u32, b: u32 => a < b);
        assert!(feasible::<argl!(u32, u32), _>(&lt, args!(1u32, 2u32)).0);
        assert!(!feasible::<argl!(u32, u32), _>(&lt, args!(2u32, 1u32)).0);
    }
}
