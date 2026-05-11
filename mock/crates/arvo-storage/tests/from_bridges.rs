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

#![feature(const_trait_impl)]
#![feature(const_convert)]
#![allow(incomplete_features)]

use arvo_storage::{Bool, Cap, USize};

// ---- Cap -> USize at const time ----

const CAP_ONE: Cap = Cap(USize(1));
const CAP_FORTY_TWO: Cap = Cap(USize(42));

const USIZE_FROM_CAP_ONE: USize = USize::from(CAP_ONE);
const USIZE_FROM_CAP_FORTY_TWO: USize = USize::from(CAP_FORTY_TWO);

const _: () = {
    assert!(USIZE_FROM_CAP_ONE.0 == 1);
    assert!(USIZE_FROM_CAP_FORTY_TWO.0 == 42);
};

// ---- USize -> Cap at const time ----

const USIZE_SEVEN: USize = USize(7);
const CAP_FROM_USIZE_SEVEN: Cap = Cap::from(USIZE_SEVEN);

const _: () = {
    assert!(CAP_FROM_USIZE_SEVEN.0.0 == 7);
};

// ---- Round trip Cap -> USize -> Cap at const time ----

const _: () = {
    let c = Cap(USize(123));
    let u: USize = USize::from(c);
    let c2: Cap = Cap::from(u);
    assert!(c2.0.0 == 123);
};

// ---- bool -> Bool at const time ----

const BOOL_TRUE: Bool = Bool::from(true);
const BOOL_FALSE: Bool = Bool::from(false);

const _: () = {
    assert!(BOOL_TRUE.0);
    assert!(!BOOL_FALSE.0);
};

// ---- Bool -> bool at const time ----

const RAW_TRUE: bool = bool::from(Bool(true));
const RAW_FALSE: bool = bool::from(Bool(false));

const _: () = {
    assert!(RAW_TRUE);
    assert!(!RAW_FALSE);
};

// ---- Runtime test exercising the same paths through .into() ----

#[test]
fn cap_to_usize_into() {
    let c = Cap(USize(13));
    let u: USize = c.into();
    assert_eq!(u.0, 13);
}

#[test]
fn usize_to_cap_into() {
    let u = USize(99);
    let c: Cap = u.into();
    assert_eq!(c.0.0, 99);
}

#[test]
fn bool_to_arvo_bool_into() {
    let b: Bool = true.into();
    assert!(b.0);
    let f: Bool = false.into();
    assert!(!f.0);
}

#[test]
fn arvo_bool_to_bool_into() {
    let t: bool = Bool(true).into();
    assert!(t);
    let f: bool = Bool(false).into();
    assert!(!f);
}
