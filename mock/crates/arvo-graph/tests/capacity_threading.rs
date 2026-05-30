//! Generic-threading proof for the `Capacity`-migrated arvo-graph algorithms.
//!
//! The migrated functions run when the node-count capacity is threaded through
//! a caller's OWN generic `C: Capacity` (not a concrete `Dim<N>` fixed at the
//! call site). That is the shape #652 (hilavitkutin threading `PlanDims`
//! capacities into the plan algorithms) needs, and the exact shape that
//! overflowed `generic_const_exprs` under the `const N: Cap` form. There is
//! deliberately no `#![feature(...)]` gate: its absence is the proof the
//! algorithms escaped the GCE surface.

use arvo::{FBits, ibits, USize};
use arvo::strategy::{Hot, Unsigned};
use arvo::Bits;
use arvo::ufixed::UFixed;
use arvo_bitmask::{BitMatrix, NodeId, cap_size};
use arvo_graph::{components, topo_sort, upward_rank};
use arvo_tensor::{Capacity, Dim};

type W = UFixed<{ ibits(8) }, { FBits::ZERO }, Hot>;

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

fn w(n: usize) -> W {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test helper; runtime usize→u8 cast for typed weight in concrete-W test scope; tracked: #256
    W::from_raw(n as u8)
}

// Build a linear-chain DAG `0 -> 1 -> ... -> (n-1)` over a caller-owned
// `C: Capacity`, topo-sort it, and check the sort is the full identity order.
// This is the threading shape that ICE'd under `const N: Cap`; here the
// capacity is a TYPE so no `cap_size` sits in any type position.
fn chain_sorts_in_order<C: Capacity>() -> usize
where
    C::Array<USize>: Copy,
    C::Array<NodeId>: Copy,
{
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, C> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    let n = cap_size(C::CAP);
    let mut i = 0usize;
    while i + 1 < n {
        dag.set_edge(nid(i), nid(i + 1));
        i += 1;
    }
    let (valid, order) = topo_sort(&dag);
    // Linear chain: the only valid extension is 0, 1, ..., n-1.
    let mut k = 0usize;
    while k < n {
        assert_eq!(order.as_ref()[k], nid(k));
        k += 1;
    }
    valid.0
}

#[test]
fn topo_sort_threads_generically_over_capacity() {
    assert_eq!(chain_sorts_in_order::<Dim<4>>(), 4);
    // Dim<7>, non-power-of-two node count.
    assert_eq!(chain_sorts_in_order::<Dim<7>>(), 7);
}

// upward_rank threaded through a caller-owned `C: Capacity`. A linear chain of
// unit weights has rank[v] = n - v (leaf grounded at its own weight). Exercises
// the `C::Array<W>` / `C::Array<USize>` / `C::Array<NodeId>` Copy-bound
// propagation generically.
fn chain_root_rank<C: Capacity>() -> u8
where
    C::Array<W>: Copy,
    C::Array<USize>: Copy,
    C::Array<NodeId>: Copy,
{
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, C> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    let n = cap_size(C::CAP);
    let mut i = 0usize;
    while i + 1 < n {
        dag.set_edge(nid(i), nid(i + 1));
        i += 1;
    }
    let weights: C::Array<W> = C::filled(w(1));
    let rank = upward_rank(&dag, &weights);
    // Root (node 0) rank is the full chain length.
    rank.as_ref()[0].to_raw()
}

#[test]
fn upward_rank_threads_generically_over_capacity() {
    assert_eq!(chain_root_rank::<Dim<4>>(), 4);
    assert_eq!(chain_root_rank::<Dim<5>>(), 5);
}

// components threaded through a caller-owned `C: Capacity`. A single linear
// chain is one connected component, so every node shares node 0's id.
fn chain_is_one_component<C: Capacity>() -> bool
where
    C::Array<USize>: Copy,
    C::Array<NodeId>: Copy,
{
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, C> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    let n = cap_size(C::CAP);
    let mut i = 0usize;
    while i + 1 < n {
        dag.set_edge(nid(i), nid(i + 1));
        i += 1;
    }
    let comp = components(&dag);
    let id0 = comp.as_ref()[0];
    let mut k = 0usize;
    while k < n {
        if comp.as_ref()[k] != id0 {
            return false;
        }
        k += 1;
    }
    true
}

#[test]
fn components_threads_generically_over_capacity() {
    assert!(chain_is_one_component::<Dim<4>>());
    assert!(chain_is_one_component::<Dim<6>>());
}
