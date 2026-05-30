//! bin_pack: packing counts and affinity ordering.
//!
//! No `#![feature(...)]` gates: the migrated `bin_pack<N: Capacity, B:
//! Capacity, ..>` threads both capacities as types, escaping the
//! `generic_const_exprs` surface.

use arvo::{FBits, Maybe, USize, ibits};
use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo_comb::bin_pack;
use arvo_tensor::{Array, Dim};

type W = UFixed<{ ibits(16) }, { FBits::ZERO }, Hot>;

fn w(n: usize) -> W {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test helper; runtime usize→u16 cast for typed weight in concrete-W test scope; no runtime-FromConstant by design (round 202604271346); tracked: #256
    W::from_raw(n as u16)
}

#[test]
fn empty_input_no_bins() {
    let items: Array<u8, Dim<0>> = Array::new([]);
    let (count, _assign) =
        bin_pack::<Dim<0>, Dim<4>, u8, W>(&items, w(10), |_| w(1), |_, _| w(0));
    assert_eq!(count, USize(0));
}

#[test]
fn unit_weights_pack_to_ceil_n_over_capacity() {
    // 4 items, each weight 1, capacity 3. All affinities equal so
    // tie-breaking falls to insertion-sort stability (original order).
    // First-fit: items 0,1,2 -> bin 0; item 3 -> bin 1.
    let items: Array<u8, Dim<4>> = Array::new([10, 20, 30, 40]);
    let (count, assign) =
        bin_pack::<Dim<4>, Dim<4>, u8, W>(&items, w(3), |_| w(1), |_, _| w(0));
    assert_eq!(count, USize(2));
    // All items must land in either bin 0 or 1, and every item must
    // have placed (NUSize::some). The previous overloaded USize(0)
    // sentinel is replaced; an unplaced item would now read NUSize::NONE.
    for i in 0..4 {
        match assign.get(USize(i)).into_maybe() {
            Maybe::Is(b) => assert!(b.0 < 2, "item {i} -> bin {}", b.0),
            Maybe::Isnt => panic!("item {i} unplaced; expected bin 0 or 1"),
        }
    }
}

#[test]
fn single_heavy_item_uses_its_own_bin() {
    // Two items, one fills a bin on its own.
    let items: Array<u8, Dim<2>> = Array::new([0, 1]);
    let (count, _assign) =
        bin_pack::<Dim<2>, Dim<4>, u8, W>(&items, w(5), |x| if *x == 0 { w(5) } else { w(1) }, |_, _| w(0));
    assert_eq!(count, USize(2));
}

#[test]
fn everything_fits_one_bin() {
    // Weights 1+1+1 = 3 <= cap 5.
    let items: Array<u8, Dim<3>> = Array::new([1, 2, 3]);
    let (count, assign) =
        bin_pack::<Dim<3>, Dim<4>, u8, W>(&items, w(5), |_| w(1), |_, _| w(0));
    assert_eq!(count, USize(1));
    for i in 0..3 {
        // Single bin: every item lands at NUSize::some(USize(0)).
        // Compare via into_maybe so the test reads the sentinel-distinct
        // representation correctly.
        assert!(matches!(assign.get(USize(i)).into_maybe(), Maybe::Is(USize(0))));
    }
}

#[test]
fn affinity_ordering_places_high_affinity_first() {
    // Four items. Item 0 has very high affinity to items 2 and 3
    // (mutual), while item 1 is lonely. Capacity 2 per bin, all
    // weights 1. High-affinity items go first -> the cluster
    // {0,2,3} is placed across two bins before item 1.
    let items: Array<u8, Dim<4>> = Array::new([0, 1, 2, 3]);
    let (count, _assign) = bin_pack::<Dim<4>, Dim<4>, u8, W>(
        &items,
        w(2),
        |_| w(1),
        |a, b| {
            // Ring affinity: items 0,2,3 love each other, item 1 is alone.
            let is_cluster = |x: &u8| *x == 0 || *x == 2 || *x == 3;
            if is_cluster(a) && is_cluster(b) && a != b {
                w(10)
            } else {
                w(0)
            }
        },
    );
    // 4 items, cap=2, w=1 each -> 2 bins regardless of ordering.
    assert_eq!(count, USize(2));
}
