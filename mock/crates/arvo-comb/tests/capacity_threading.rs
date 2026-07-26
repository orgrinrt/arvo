//! Generic-threading proof for the `Capacity`-migrated arvo-comb algorithms.
//!
//! The migrated functions run when the capacity is threaded through a caller's
//! OWN generic `C: Capacity` (not a concrete `Dim<N>` fixed at the call site).
//! That is the shape #652 (hilavitkutin threading `PlanDims` capacities) needs,
//! and the exact shape that overflowed `generic_const_exprs` under the
//! `const N: Cap` form. No `#![feature(...)]` gate: its absence is the proof
//! the algorithms escaped the GCE surface.

use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo::{ibits, Bool, FBits, USize};
use arvo_comb::{greedy_group, matrix_chain_dp, Range};
use arvo_tensor::{Array, Capacity, Dim};

type W = UFixed<{ ibits(16) }, { FBits::ZERO }, Hot>;

fn w(n: usize) -> W {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test helper; runtime usize→u16 cast for typed weight in concrete-W test scope; tracked: #256
    W::from_raw(n as u16)
}

// matrix_chain_dp threaded through a caller's own `C: Capacity`. Exercises the
// `Matrix` Copy-bound propagation (`C::Array<W>` / `Bool` / `USize`) generically.
fn dp_cost<C: Capacity>() -> u16
where
    C::Array<W>: Copy,
    C::Array<Bool>: Copy,
    C::Array<USize>: Copy,
{
    let (cost, _splits) = matrix_chain_dp::<C, W>(
        |i, j| if i.0 == j.0 { w(1) } else { w(100) },
        |_, _| Bool::TRUE,
    );
    cost.to_raw()
}

#[test]
fn matrix_chain_dp_threads_generically() {
    // All-singletons optimal cost equals N (each split costs 1).
    assert_eq!(dp_cost::<Dim<4>>(), 4);
    assert_eq!(dp_cost::<Dim<3>>(), 3);
}

// greedy_group threaded through caller-owned input and group capacities.
// Array-only path: no Matrix Copy bound, just the two `Capacity` params.
fn group_count<N: Capacity, M: Capacity>(items: &Array<u32, N>, cap: u32) -> usize {
    let (count, _groups) = greedy_group::<N, M, u32, u32>(
        items,
        |acc, x| Bool(*acc + *x <= cap),
        |acc, x| acc + *x,
        || 0u32,
    );
    count.0
}

#[test]
fn greedy_group_threads_generically() {
    // cap=5, items=[3,2,4,1,3] -> groups [0..2),[2..4),[4..5) = 3.
    let items: Array<u32, Dim<5>> = Array::new([3, 2, 4, 1, 3]);
    assert_eq!(group_count::<Dim<5>, Dim<8>>(&items, 5), 3);
    let _ = Range::default(); // Range stays the group element type post-migration.
}
