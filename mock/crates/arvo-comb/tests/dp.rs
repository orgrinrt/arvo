//! matrix_chain_dp: optimal split on known-shape cost functions.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Bool, Cap, FBits, IBits, USize};
use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo_comb::matrix_chain_dp;

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

const C1: Cap = cap(1);
const C3: Cap = cap(3);
const C4: Cap = cap(4);

type W = UFixed<{ IBits(16) }, { FBits::ZERO }, Hot>;

fn w(n: usize) -> W {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test helper; runtime usize→u16 cast for typed weight in concrete-W test scope; no runtime-FromConstant by design (round 202604271346); tracked: #256
    W::from_raw(n as u16)
}

#[test]
fn singleton_returns_leaf_cost() {
    // N=1, cost(0,0) = 7.
    let (cost, _splits) =
        matrix_chain_dp::<C1, W>(|_, _| w(7), |_, _| Bool::TRUE);
    assert_eq!(cost.to_raw(), 7);
}

#[test]
fn all_leaves_cheap_root_expensive_prefers_split() {
    // Intervals of length 1 cost 1; any wider interval taken as a
    // leaf costs 100. DP must split to achieve cost N.
    // N=4 -> optimal cost 4 (four singletons summed).
    let (cost, _splits) = matrix_chain_dp::<C4, W>(
        |i, j| if i.0 == j.0 { w(1) } else { w(100) },
        |_, _| Bool::TRUE,
    );
    assert_eq!(cost.to_raw(), 4);
}

#[test]
fn single_leaf_cheaper_than_splitting() {
    // cost(0..N-1) = 1 as a whole leaf; cost(i,i) = 10 per singleton.
    // DP should take the whole interval, cost = 1.
    let (cost, _splits) = matrix_chain_dp::<C4, W>(
        |i, j| {
            if i.0 == 0 && j.0 == 3 {
                w(1)
            } else if i.0 == j.0 {
                w(10)
            } else {
                w(100)
            }
        },
        |_, _| Bool::TRUE,
    );
    assert_eq!(cost.to_raw(), 1);
}

#[test]
fn infeasible_leaves_forces_split() {
    // Singletons feasible cost 1. Any multi-element interval is
    // infeasible as a leaf, so the DP must compose singletons.
    // N=3 -> optimal cost 3.
    let (cost, _splits) = matrix_chain_dp::<C3, W>(
        |i, j| if i.0 == j.0 { w(1) } else { w(200) },
        |i, j| Bool(i.0 == j.0),
    );
    assert_eq!(cost.to_raw(), 3);
}
