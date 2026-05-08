//! Const-context smoke tests for `NUSize`.
//!
//! Round 5 (#315) added `NUSize` as a niche-filled `Maybe<USize>`
//! alternative with auto +1 / -1 shift. These tests exercise the
//! const surface (`NONE`, `some`, `into_maybe`, `unwrap_or`,
//! `is_some`, `is_none`) inside `const _: () = { ... };` blocks so
//! a regression to non-const callability surfaces as a compile
//! error.

#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use arvo_storage::{NUSize, USize};
use notko::Maybe;

const _NUSIZE_NONE_PROBES: () = {
    let n: NUSize = NUSize::NONE;
    assert!(matches!(n.into_maybe(), Maybe::Isnt));
};

const _NUSIZE_SOME_ZERO_ROUND_TRIPS: () = {
    // Logical 0 maps to NonZeroUsize::new(1) internally; into_maybe
    // shifts back. The presence of a value distinct from NONE is the
    // load-bearing claim.
    let n = NUSize::some(USize(0));
    assert!(matches!(n.into_maybe(), Maybe::Is(USize(0))));
};

const _NUSIZE_SOME_NONZERO_ROUND_TRIPS: () = {
    let n = NUSize::some(USize(42));
    assert!(matches!(n.into_maybe(), Maybe::Is(USize(42))));
};

const _NUSIZE_UNWRAP_OR_PRESENT: () = {
    let n = NUSize::some(USize(7));
    let v = n.unwrap_or(USize(99));
    assert!(matches!(v, USize(7)));
};

const _NUSIZE_UNWRAP_OR_ABSENT: () = {
    let n = NUSize::NONE;
    let v = n.unwrap_or(USize(99));
    assert!(matches!(v, USize(99)));
};

const _NUSIZE_IS_SOME_PRESENT: () = {
    let n = NUSize::some(USize(0));
    assert!(n.is_some().0);
    assert!(!n.is_none().0);
};

const _NUSIZE_IS_NONE_ABSENT: () = {
    let n: NUSize = NUSize::NONE;
    assert!(n.is_none().0);
    assert!(!n.is_some().0);
};

#[test]
fn nusize_runtime_round_trip() {
    let cases = [USize(0), USize(1), USize(usize::MAX / 2)];
    for &v in &cases {
        let n = NUSize::some(v);
        assert!(matches!(n.into_maybe(), Maybe::Is(_)));
    }
}

#[test]
fn nusize_default_is_none() {
    let n = NUSize::default();
    assert!(matches!(n.into_maybe(), Maybe::Isnt));
}

#[test]
fn nusize_some_at_usize_max_returns_none() {
    // Documented contract: `NUSize::some(USize(usize::MAX))` cannot
    // shift +1 without overflow, so construction folds into
    // `NUSize::NONE` rather than panicking. Practical workloads
    // never hit the cap because `Cap<N>` is bounded well below
    // `usize::MAX`, but the contract is load-bearing for the
    // surface's "cannot fail" story (the only failure mode is the
    // documented overflow, which folds into NONE).
    let n = NUSize::some(USize(usize::MAX));
    assert!(matches!(n.into_maybe(), Maybe::Isnt));
    assert!(n.is_none().0);
    assert!(!n.is_some().0);
}
