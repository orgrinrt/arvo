//! Const-context smoke tests for the round-202605112200 From bridges.
//!
//! Four `impl const From` impls land in this round:
//! - `Cap` -> `USize` (typed capacity projection)
//! - `USize` -> `Cap` (symmetric companion)
//! - `bool` -> `Bool` (lifted from non-const)
//! - `Bool` -> `bool` (lifted from non-const)
//!
//! These tests exercise each bridge at const time inside
//! `const _: ... = { ... };` blocks. If any impl regresses to
//! non-const, the const-context evaluation will fail to compile.
//!
//! Const-context unwraps route through `<T as Transparent>::raw(...)`
//! rather than `.0` field access, per the workspace
//! `.0`-is-a-smell discipline (PRINCIPLES.md.tmpl).

#![feature(const_trait_impl)]
#![feature(const_convert)]
#![allow(incomplete_features)]

use arvo_storage::{Bool, Cap, USize};
use arvo_transparent::Transparent;

// ---- Cap -> USize at const time ----

const CAP_ONE: Cap = Cap(USize(1));
const CAP_FORTY_TWO: Cap = Cap(USize(42));

const USIZE_FROM_CAP_ONE: USize = USize::from(CAP_ONE);
const USIZE_FROM_CAP_FORTY_TWO: USize = USize::from(CAP_FORTY_TWO);

const _: () = {
    assert!(<USize as Transparent>::raw(USIZE_FROM_CAP_ONE) == 1);
    assert!(<USize as Transparent>::raw(USIZE_FROM_CAP_FORTY_TWO) == 42);
};

// ---- USize -> Cap at const time ----

const USIZE_SEVEN: USize = USize(7);
const CAP_FROM_USIZE_SEVEN: Cap = Cap::from(USIZE_SEVEN);

const _: () = {
    let inner: USize = <Cap as Transparent>::raw(CAP_FROM_USIZE_SEVEN);
    assert!(<USize as Transparent>::raw(inner) == 7);
};

// ---- Round trip Cap -> USize -> Cap at const time ----

const _: () = {
    let c = Cap(USize(123));
    let u: USize = USize::from(c);
    let c2: Cap = Cap::from(u);
    let inner: USize = <Cap as Transparent>::raw(c2);
    assert!(<USize as Transparent>::raw(inner) == 123);
};

// ---- bool -> Bool at const time ----

const BOOL_TRUE: Bool = Bool::from(true);
const BOOL_FALSE: Bool = Bool::from(false);

const _: () = {
    assert!(<Bool as Transparent>::raw(BOOL_TRUE));
    assert!(!<Bool as Transparent>::raw(BOOL_FALSE));
};

// ---- Bool -> bool at const time ----

const RAW_TRUE: bool = bool::from(Bool(true));
const RAW_FALSE: bool = bool::from(Bool(false));

const _: () = {
    assert!(RAW_TRUE);
    assert!(!RAW_FALSE);
};

// ---- Runtime tests exercising the same paths through .into() ----

#[test]
fn cap_to_usize_into() {
    let c = Cap(USize(13));
    let u: USize = c.into();
    assert_eq!(<USize as Transparent>::raw(u), 13);
}

#[test]
fn usize_to_cap_into() {
    let u = USize(99);
    let c: Cap = u.into();
    let inner: USize = <Cap as Transparent>::raw(c);
    assert_eq!(<USize as Transparent>::raw(inner), 99);
}

#[test]
fn bool_to_arvo_bool_into() {
    let b: Bool = true.into();
    assert!(<Bool as Transparent>::raw(b));
    let f: Bool = false.into();
    assert!(!<Bool as Transparent>::raw(f));
}

#[test]
fn arvo_bool_to_bool_into() {
    let t: bool = Bool(true).into();
    assert!(t);
    let f: bool = Bool(false).into();
    assert!(!f);
}
