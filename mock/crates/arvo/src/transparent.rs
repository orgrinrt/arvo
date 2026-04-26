//! Universal `repr(transparent)` access surface.
//!
//! Every arvo wrapper newtype is `#[repr(transparent)]` over an inner
//! type. By Rust spec, `repr(transparent)` makes the wrapper layout-
//! equivalent to the inner — same size, alignment, ABI. This module
//! exposes that relationship as a typed surface so consumers don't
//! reach for `.0.0` chains or hard-code container widths.
//!
//! Two traits cover the surface:
//!
//! - [`Transparent`] (`unsafe`): the wrapper declares its immediate
//!   inner type. Implementors guarantee `repr(transparent)` over the
//!   declared `Inner`. Reading `inner()` is a layout-equivalent
//!   transmute, not a field projection.
//! - [`As<T>`]: typed conversion to a layout-equivalent native
//!   primitive. Blanket-implemented across the `Transparent` chain
//!   so `IBits → u8` works through `IBits: Transparent<Inner = u8>`
//!   in one step (or through chained `Transparent`s if a wrapper
//!   wraps another wrapper).
//!
//! The `NumericPrimitive` marker pins the set of primitives
//! eligible for the `As<T>` conversion: `u8` / `u16` / `u32` / `u64`
//! / `usize` and their signed counterparts, plus `bool`. This is the
//! native-type universe; arvo wrappers terminate at one of these.

use core::mem::transmute_copy;

/// Marker for native primitives that arvo wrappers can transmute
/// to via `repr(transparent)` layout-equivalence.
pub trait NumericPrimitive: Copy + Sized + 'static {}

impl NumericPrimitive for u8 {}
impl NumericPrimitive for u16 {}
impl NumericPrimitive for u32 {}
impl NumericPrimitive for u64 {}
impl NumericPrimitive for usize {}
impl NumericPrimitive for i8 {}
impl NumericPrimitive for i16 {}
impl NumericPrimitive for i32 {}
impl NumericPrimitive for i64 {}
impl NumericPrimitive for isize {}
impl NumericPrimitive for bool {}

/// Soundness contract for `repr(transparent)` arvo wrappers.
///
/// # Safety
///
/// Implementors must be `#[repr(transparent)]` over the declared
/// `Inner` type. The compiler treats a `repr(transparent)` wrapper as
/// having identical layout (size, alignment, ABI) to its single non-
/// zero-sized field. Implementing `Transparent` for a non-transparent
/// type is undefined behaviour because the `inner()` transmute relies
/// on this layout invariant.
pub unsafe trait Transparent: Copy + Sized {
    /// The single non-ZST field's type. Reading this returns a
    /// layout-equivalent value.
    type Inner: Copy;

    /// Read the inner value via layout-equivalent transmute.
    ///
    /// Bypasses `.0` field access so call sites don't depend on the
    /// wrapper's field name or type-projection chain. Consumers that
    /// want to chain through multiple wrappers (e.g. `IBits → Bits →
    /// u8`) use [`As<T>`] instead.
    #[inline(always)]
    fn inner(self) -> Self::Inner {
        // SAFETY: implementor's `Transparent` contract guarantees
        // `Self` is `repr(transparent)` over `Inner`, so the layouts
        // are byte-identical and `transmute_copy` is sound.
        unsafe { transmute_copy::<Self, Self::Inner>(&self) }
    }
}

/// Typed conversion to a layout-equivalent native primitive.
///
/// Blanket-implemented for any `Transparent` whose `Inner` either IS
/// `T` (one-step terminal) or is itself `As<T>` (recursive chain).
/// Consumers write `let n: u8 = ibits.as_native();` (or with turbofish
/// `ibits.as_native::<u8>()`) without naming the intermediate
/// wrapper.
///
/// At runtime this collapses to a single transmute by the optimiser
/// (every step is a no-op via `repr(transparent)`).
pub trait As<T: NumericPrimitive>: Sized + Copy {
    fn as_native(self) -> T;
}

// Terminal case: T == Inner. The wrapper directly transmutes to T.
impl<W, T> As<T> for W
where
    W: Transparent<Inner = T>,
    T: NumericPrimitive,
{
    #[inline(always)]
    fn as_native(self) -> T {
        self.inner()
    }
}

// Note: a recursive blanket
//   impl<W, T> As<T> for W where W: Transparent, W::Inner: As<T>
// would conflict with the terminal impl in coherence (both could
// match when Inner happens to be T). The terminal impl above is
// sufficient for arvo's current wrapper depth (one level: meta-
// newtypes wrap u8 directly). When deeper chains land (e.g. IBits
// over Bits over u8), the chain is realised via per-pair
// implementations declared at the wrapper's definition site, where
// the implementor knows its full chain unambiguously.
