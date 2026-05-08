//! Const-context smoke tests for `Bool`'s `ConstTry` /
//! `ConstFromResidual` impls.
//!
//! Round 5 (#315) lifted Bool's existing `Try` / `FromResidual`
//! routing into notko's const-callable bridges. Const-context
//! consumers (mockspace lints, narrow_from helpers) reach the same
//! routing through the const family. These tests exercise
//! `branch` / `from_output` / `from_residual` inside `const _: () =
//! { ... };` blocks.

#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use core::convert::Infallible;
use core::ops::ControlFlow;
use arvo_storage::Bool;
use notko::{ConstFromResidual, ConstTry};

const _BOOL_TRUE_BRANCH_CONTINUES: () = {
    let b = Bool(true);
    match <Bool as ConstTry>::branch(b) {
        ControlFlow::Continue(v) => assert!(v),
        ControlFlow::Break(_) => panic!("Bool(true) should Continue"),
    }
};

const _BOOL_FALSE_BRANCH_CONTINUES_WITH_FALSE: () = {
    // Bool's residual is Infallible: branch always Continues, with
    // the inner bool. That is by design (Bool is not a fallibility
    // marker; it carries a value).
    let b = Bool(false);
    match <Bool as ConstTry>::branch(b) {
        ControlFlow::Continue(v) => assert!(!v),
        ControlFlow::Break(_) => panic!("Bool(false) should still Continue"),
    }
};

const _BOOL_FROM_OUTPUT_RECONSTRUCTS: () = {
    let b = <Bool as ConstTry>::from_output(true);
    match <Bool as ConstTry>::branch(b) {
        ControlFlow::Continue(v) => assert!(v),
        ControlFlow::Break(_) => panic!("from_output(true) should Continue with true"),
    }
};

const _BOOL_FROM_RESIDUAL_TYPECHECKS: () = {
    // The Infallible residual is uninhabited; we can name the
    // function path without invoking it. This is a typecheck-time
    // probe that the impl exists with the expected shape.
    let _f: fn(Infallible) -> Bool = <Bool as ConstFromResidual<Infallible>>::from_residual;
};

#[test]
fn bool_branch_runtime() {
    let b = Bool(true);
    match <Bool as ConstTry>::branch(b) {
        ControlFlow::Continue(v) => assert!(v),
        ControlFlow::Break(_) => panic!("Bool(true) should Continue"),
    }
}
