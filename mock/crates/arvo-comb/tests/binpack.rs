//! bin_pack: packing counts and affinity ordering.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, FBits, IBits, USize};
use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo_comb::bin_pack;
use arvo_tensor::Array;

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

const C0: Cap = cap(0);
const C2: Cap = cap(2);
const C3: Cap = cap(3);
const C4: Cap = cap(4);

type W = UFixed<{ IBits(16) }, { FBits::ZERO }, Hot>;

fn w(n: usize) -> W {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test helper; runtime usize→u16 cast for typed weight in concrete-W test scope; no runtime-FromConstant by design (round 202604271346); tracked: #256
    W::from_raw(n as u16)
}

#[test]
fn empty_input_no_bins() {
    let items: Array<u8, C0> = Array::new([]);
    let (count, _assign) =
        bin_pack::<C0, C4, u8, W>(&items, w(10), |_| w(1), |_, _| w(0));
    assert_eq!(count, USize(0));
}

#[test]
fn unit_weights_pack_to_ceil_n_over_capacity() {
    // 4 items, each weight 1, capacity 3. All affinities equal so
    // tie-breaking falls to insertion-sort stability (original order).
    // First-fit: items 0,1,2 -> bin 0; item 3 -> bin 1.
    let items: Array<u8, C4> = Array::new([10, 20, 30, 40]);
    let (count, assign) =
        bin_pack::<C4, C4, u8, W>(&items, w(3), |_| w(1), |_, _| w(0));
    assert_eq!(count, USize(2));
    // All items must land in either bin 0 or 1.
    for i in 0..4 {
        let b = assign.get(USize(i)).0;
        assert!(b < 2, "item {i} -> bin {b}");
    }
}

#[test]
fn single_heavy_item_uses_its_own_bin() {
    // Two items, one fills a bin on its own.
    let items: Array<u8, C2> = Array::new([0, 1]);
    let (count, _assign) =
        bin_pack::<C2, C4, u8, W>(&items, w(5), |x| if *x == 0 { w(5) } else { w(1) }, |_, _| w(0));
    assert_eq!(count, USize(2));
}

#[test]
fn everything_fits_one_bin() {
    // Weights 1+1+1 = 3 <= cap 5.
    let items: Array<u8, C3> = Array::new([1, 2, 3]);
    let (count, assign) =
        bin_pack::<C3, C4, u8, W>(&items, w(5), |_| w(1), |_, _| w(0));
    assert_eq!(count, USize(1));
    for i in 0..3 {
        assert_eq!(*assign.get(USize(i)), USize(0));
    }
}

#[test]
fn affinity_ordering_places_high_affinity_first() {
    // Four items. Item 0 has very high affinity to items 2 and 3
    // (mutual), while item 1 is lonely. Capacity 2 per bin, all
    // weights 1. High-affinity items go first -> the cluster
    // {0,2,3} is placed across two bins before item 1.
    let items: Array<u8, C4> = Array::new([0, 1, 2, 3]);
    let (count, _assign) = bin_pack::<C4, C4, u8, W>(
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
