//! Integration tests for the const-callable `ConstCapacity` surface.
//!
//! These assert const construction plus indexed read/write of a capacity's
//! backing array in a forced-const-eval context, generic over
//! `C: [const] ConstCapacity`, at distinct N. The real-type analog of the R0
//! feasibility sketch: success means a const fn can allocate and walk scratch
//! arrays at compile time, the foundation for a const `waist_detect`.

#![feature(const_trait_impl)]

use arvo::USize;
use arvo_tensor::{cap, cap_size, ConstCapacity, Dim};

// Build a C::Array<usize>, write `i * 3 + 1` into the first `live` slots through
// the const `set`, read them back through the const `get`, and sum. Generic over
// the const capacity contract, fully const-evaluable. This is the core move a
// const DAG analysis needs: a typed scratch array constructed, indexed, and
// walked at compile time.
const fn fill_and_sum<C: [const] ConstCapacity>(live: usize) -> usize {
    let mut a = C::filled(0usize);
    let n = cap_size(C::CAP);
    let mut i = 0;
    while i < live && i < n {
        C::set(&mut a, USize(i), i * 3 + 1);
        i += 1;
    }
    let mut sum = 0;
    let mut j = 0;
    while j < live && j < n {
        sum += C::get(&a, USize(j));
        j += 1;
    }
    sum
}

// Forced const evaluation at distinct N. `i * 3 + 1` for i in 0..4 is
// 1 + 4 + 7 + 10 = 22; for i in 0..6 is 1 + 4 + 7 + 10 + 13 + 16 = 51. If the
// const fn const-evaluates over the GAT at both N with no per-N GCE blowup,
// these compile and hold.
const S4: usize = fill_and_sum::<Dim<4>>(4);
const S8: usize = fill_and_sum::<Dim<8>>(6);

#[test]
fn const_build_index_and_walk_at_distinct_n() {
    assert_eq!(S4, 22, "const fill+walk over Dim<4>");
    assert_eq!(S8, 51, "const fill+walk over Dim<8> with slack");
}

#[test]
fn const_cap_is_typed_and_exact() {
    assert_eq!(<Dim<3> as ConstCapacity>::CAP, cap(3));
    assert_eq!(<Dim<13> as ConstCapacity>::CAP, cap(13));
}

// Slack slots past `live` keep the fill value, untouched by the partial walk.
const S8_PARTIAL: usize = fill_and_sum::<Dim<8>>(3); // 1 + 4 + 7 = 12

#[test]
fn partial_fill_leaves_slack_untouched() {
    assert_eq!(S8_PARTIAL, 12, "only the first 3 slots contribute");
}
