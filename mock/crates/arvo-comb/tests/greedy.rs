//! greedy_group: sequential interval grouping correctness.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::{Bool, Cap, USize};
use arvo_comb::{Range, greedy_group};
use arvo_tensor::Array;

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

const C0: Cap = cap(0);
const C2: Cap = cap(2);
const C4: Cap = cap(4);
const C5: Cap = cap(5);
const C8: Cap = cap(8);

/// Sum-capped accumulator: the group holds items whose total is <= cap.
#[derive(Copy, Clone)]
struct SumCap {
    total: u32,
    cap: u32,
}

fn new_acc(cap: u32) -> SumCap {
    SumCap { total: 0, cap }
}

#[test]
fn empty_input_returns_zero_groups() {
    let items: Array<u32, C0> = Array::new([]);
    let (count, _groups) = greedy_group::<C0, C4, SumCap, u32>(
        &items,
        |acc, x| Bool(acc.total + x <= acc.cap),
        |acc, x| SumCap { total: acc.total + *x, cap: acc.cap },
        || new_acc(10),
    );
    assert_eq!(count, USize(0));
}

#[test]
fn all_items_fit_one_group() {
    // Four items summing to 10, cap 10. One group covering 0..4.
    let items: Array<u32, C4> = Array::new([2, 3, 4, 1]);
    let (count, groups) = greedy_group::<C4, C4, SumCap, u32>(
        &items,
        |acc, x| Bool(acc.total + x <= acc.cap),
        |acc, x| SumCap { total: acc.total + *x, cap: acc.cap },
        || new_acc(10),
    );
    assert_eq!(count, USize(1));
    assert_eq!(*groups.get(USize(0)), Range { start: USize(0), end: USize(4) });
}

#[test]
fn splits_on_overflow() {
    // cap=5, items=[3,2,4,1,3]. Trace:
    //   [3+2=5] fits -> close at i=2 when 4 rejected.
    //   [4+1=5] fits -> close at i=4 when 3 rejected.
    //   [3] trailing.
    // Groups: [0..2), [2..4), [4..5).
    let items: Array<u32, C5> = Array::new([3, 2, 4, 1, 3]);
    let (count, groups) = greedy_group::<C5, C8, SumCap, u32>(
        &items,
        |acc, x| Bool(acc.total + x <= acc.cap),
        |acc, x| SumCap { total: acc.total + *x, cap: acc.cap },
        || new_acc(5),
    );
    assert_eq!(count, USize(3));
    assert_eq!(*groups.get(USize(0)), Range { start: USize(0), end: USize(2) });
    assert_eq!(*groups.get(USize(1)), Range { start: USize(2), end: USize(4) });
    assert_eq!(*groups.get(USize(2)), Range { start: USize(4), end: USize(5) });
}

#[test]
fn every_item_its_own_group_when_cap_is_tight() {
    // cap = 1, all items = 1. Each item is its own group.
    let items: Array<u32, C4> = Array::new([1, 1, 1, 1]);
    let (count, groups) = greedy_group::<C4, C4, SumCap, u32>(
        &items,
        |acc, x| Bool(acc.total + x <= acc.cap),
        |acc, x| SumCap { total: acc.total + *x, cap: acc.cap },
        || new_acc(1),
    );
    assert_eq!(count, USize(4));
    for i in 0..4 {
        assert_eq!(*groups.get(USize(i)), Range { start: USize(i), end: USize(i + 1) });
    }
}

#[test]
fn caps_at_m_when_more_groups_would_be_produced() {
    // cap=1, 4 items of 1 each would make 4 groups. With M=2, stops
    // after the second group closes.
    let items: Array<u32, C4> = Array::new([1, 1, 1, 1]);
    let (count, groups) = greedy_group::<C4, C2, SumCap, u32>(
        &items,
        |acc, x| Bool(acc.total + x <= acc.cap),
        |acc, x| SumCap { total: acc.total + *x, cap: acc.cap },
        || new_acc(1),
    );
    assert_eq!(count, USize(2));
    assert_eq!(*groups.get(USize(0)), Range { start: USize(0), end: USize(1) });
    assert_eq!(*groups.get(USize(1)), Range { start: USize(1), end: USize(2) });
}
