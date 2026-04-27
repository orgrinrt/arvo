#![no_std]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

//! arvo-transparent. Universal `repr(transparent)` access surface.
//!
//! Every arvo wrapper newtype is `#[repr(transparent)]` over an inner
//! type. By Rust spec, `repr(transparent)` makes the wrapper layout-
//! equivalent to the inner. Same size, alignment, ABI. This crate
//! exposes that relationship as a typed surface so consumers don't
//! reach for `.0.0` chains or memorise per-wrapper accessor names.
//!
//! # The surface
//!
//! Three forms reach the same value, pick whichever reads cleanest:
//!
//! - **Inherent method** `wrapper.raw()`. No trait import needed.
//!   The macro that defines arvo wrappers emits this on every type;
//!   it's `const fn` so it works in const-generic-position bodies.
//! - **Trait method** `Transparent::raw(wrapper)`. Typed bound for
//!   generic code that wants to abstract over "any arvo wrapper".
//! - **Free function** `arvo::raw::<T>(wrapper)`. Prefix style for
//!   consumers who'd rather write `let n: u8 = raw(ibits);` than
//!   `let n = ibits.raw();`. Works in const-fn context.
//!
//! All three collapse to the same `transmute_copy` at codegen.
//!
//! # Soundness
//!
//! [`Transparent`] is `unsafe`: implementors guarantee
//! `#[repr(transparent)]` over the declared `Inner`. The compiler
//! treats a `repr(transparent)` wrapper as having identical layout
//! (size, alignment, ABI) to its single non-ZST field, so
//! `transmute_copy::<W, W::Inner>` is sound by definition.
//! Implementing `Transparent` for a non-transparent type is undefined
//! behaviour.

use core::mem::transmute_copy;

/// Marker for native primitives that arvo wrappers can transmute to.
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
/// `Inner` type. The default `raw()` method does a layout-equivalent
/// transmute and relies on this invariant; implementing `Transparent`
/// for a non-transparent type is undefined behaviour.
pub unsafe trait Transparent: Copy + Sized {
    /// The single non-ZST field's type.
    type Inner: Copy;

    /// Read the inner value via layout-equivalent transmute.
    #[inline(always)]
    fn raw(self) -> Self::Inner {
        // SAFETY: Transparent contract guarantees Self is
        // repr(transparent) over Inner, so layouts are byte-identical.
        unsafe { transmute_copy::<Self, Self::Inner>(&self) }
    }
}

/// Free-fn form of [`Transparent::raw`] for prefix-style call sites.
///
/// `let n: u8 = arvo::raw(ibits);` is equivalent to
/// `let n: u8 = ibits.raw();` and to
/// `let n: u8 = Transparent::raw(ibits);`. Pick whichever reads best
/// at the call site.
///
/// `const fn` so it works in const-generic-position bodies.
#[inline(always)]
pub const fn raw<W, T>(w: W) -> T
where
    W: Transparent<Inner = T> + Copy,
    T: NumericPrimitive,
{
    // SAFETY: Transparent contract: W is repr(transparent) over T.
    unsafe { transmute_copy::<W, T>(&w) }
}
