//! H3 probe: `BitPresentation::LOGICAL_WIDTH: USize` const-position
//! reachability on rustc 1.96.0-nightly.
//!
//! Validates whether the trait-projected `LOGICAL_WIDTH` is callable
//! in a `const` evaluation context. This determines what Round 2's
//! typed-Width lift can rely on:
//!
//! - **Const-projection works:** `const X: USize = <UFixed<...>
//!   as BitPresentation>::LOGICAL_WIDTH;` evaluates at const time.
//!   Consumers can compute on it through arvo's typed-arithmetic
//!   surface (`USize + USize`, etc.).
//!
//! - **Direct array-length use does NOT work:** `[u8; <UFixed<...>
//!   as BitPresentation>::LOGICAL_WIDTH]` is rejected because array
//!   length expects `usize`, not `USize`. This is a language
//!   constraint, not a substrate bug. Consumers reach for typed-
//!   storage primitives (`arvo_tensor::Array<T, N: Cap>`) which
//!   wrap the array-length unwrap exactly once at the boundary.
//!
//! Round 2 work tracked under #312 (Bits<const N: Width> lift) will
//! tighten LOGICAL_WIDTH to typed Width and provide const-trait
//! Width arithmetic so that Width-typed array lengths are reachable
//! once rustc adt_const_params accepts them at array-length position.

#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{BitPresentation, USize, UFixed, IFixed, ibits, fbits};
use arvo::strategy::Hot;

// Const projection: LOGICAL_WIDTH evaluates at const time. This is
// the load-bearing test: it confirms that the const-trait projection
// machinery actually delivers the typed const value at compile time.
// Domain construction uses ibits / fbits helpers (the documented
// boundary unwrap point per no-bare-primitives.md).
type U16 = UFixed<{ ibits(16) }, { fbits(0) }, Hot>;
type U8x8 = UFixed<{ ibits(8) }, { fbits(0) }, Hot>;
type I32 = IFixed<{ ibits(31) }, { fbits(0) }, Hot>;

const _W_U16: USize = <U16 as BitPresentation>::LOGICAL_WIDTH;
const _W_U8X8: USize = <U8x8 as BitPresentation>::LOGICAL_WIDTH;
const _W_I32: USize = <I32 as BitPresentation>::LOGICAL_WIDTH;

#[test]
fn logical_width_const_projection_resolves() {
    assert_eq!(_W_U16, USize(16));
    assert_eq!(_W_U8X8, USize(8));
    // IFixed: 1 (sign) + I + F = 32 for IFixed<31, 0, _>.
    assert_eq!(_W_I32, USize(32));
}

#[test]
fn logical_width_arithmetic_through_typed_surface() {
    // Const-trait arithmetic on the projected value works because
    // USize impls const Add (per the impl_unsigned_integer_newtype!
    // macro from round 202605021800).
    const SUM: USize = _W_U16 + _W_I32;
    assert_eq!(SUM, USize(48));
}
