//! Probe 1, crate 1 of 3. The zero-dependency foundation (notko's position).
//!
//! Branch B of the truth-contract fork: the foundation declares the CONTRACT a
//! truth type implements, the way D6/D5 have `Cardinal` be the contract a count
//! type implements. The question this crate answers is whether the contract can
//! be stated without the foundation naming the host's `bool` at all, and whether
//! the predicate concept (D15's Shape F) can live here while its argument list
//! and its output type are both someone else's.
#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]

use core::marker::PhantomData;
use core::ops::Deref;

/// The truth contract. A two-element Boolean algebra and nothing else.
///
/// No `bool` appears anywhere in this declaration. That is the point: the
/// foundation is below the crate that names the host primitive.
pub const trait Truth: Copy {
    const TRUE: Self;
    const FALSE: Self;
    fn not(self) -> Self;
    fn and(self, other: Self) -> Self;
    fn or(self, other: Self) -> Self;
}

/// Exit spelling 1: the contract yields the host primitive.
///
/// This is what `AsBool` already is in the shipped tree, lifted one crate down.
/// It re-imports `bool` into the foundation, which is the thing branch B was
/// supposed to avoid.
pub const trait TruthHolds: [const] Truth {
    fn holds(self) -> bool;
}

/// Exit spelling 2: the contract yields control flow, and never names `bool`.
///
/// A selector is the `if` of a Boolean algebra that has no host primitive to
/// hand back. Both arms are thunks so the unselected arm is not evaluated.
pub trait TruthSelect: Truth {
    fn select<R, T: FnOnce() -> R, F: FnOnce() -> R>(self, on_true: T, on_false: F) -> R;
}

// The hlist, D5/D9's `notko-hlist` contents, modelled here in one crate because
// the question under test is coherence against the *foundation*, and splitting
// the list into a fourth crate does not change which items are foreign to arvo.
pub struct Nil;
pub struct Cons<H, T>(PhantomData<(H, T)>);

#[macro_export]
macro_rules! hl {
    () => { $crate::Nil };
    ($h:ty $(, $r:ty)*) => { $crate::Cons<$h, hl!($($r),*)> };
}

/// D15's validation marker. The arity lives here and never dispatches.
pub trait Describes<F> {}

/// D15's Shape F wrapper: typestate in `L`, invocation by `Deref`, no gates.
pub struct Pred<L, F>(F, PhantomData<L>);

impl<L: Describes<F>, F> Pred<L, F> {
    pub const fn new(f: F) -> Self {
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

/// The arity table, generic over the truth contract.
///
/// The output type `B` is constrained by the `Fn` bound's associated `Output`,
/// so this is not an unconstrained impl parameter. One extra type parameter per
/// row; zero extra rows.
#[macro_export]
macro_rules! describes {
    () => {};
    ($h:ident $(, $r:ident)*) => {
        impl<$h, $($r,)* F, B> $crate::Describes<F> for hl!($h $(, $r)*)
        where
            F: Fn(&$h $(, &$r)*) -> B,
            B: $crate::Truth,
        {}
        describes!($($r),*);
    };
}

describes!(A1, A2, A3, A4, A5, A6, A7, A8);
