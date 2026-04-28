//! Smoke tests for `Array<T, N>`.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_tensor::{Array, Enumerator, cap_size};

const C4: Cap = Cap(USize(4));

#[test]
fn new_and_get() {
    let a: Array<u32, C4> = Array::new([10, 20, 30, 40]);
    assert_eq!(*a.get(USize(0)), 10);
    assert_eq!(*a.get(USize(3)), 40);
}

#[test]
fn set_overwrites() {
    let mut a: Array<u32, C4> = Array::new([0; 4]);
    a.set(USize(2), 99);
    assert_eq!(*a.get(USize(2)), 99);
}

#[test]
fn from_fn_uses_usize_index() {
    let a: Array<usize, C4> = Array::from_fn(|i| i.0 * 10);
    assert_eq!(*a.get(USize(0)), 0);
    assert_eq!(*a.get(USize(1)), 10);
    assert_eq!(*a.get(USize(2)), 20);
    assert_eq!(*a.get(USize(3)), 30);
}

#[test]
fn len_matches_cap() {
    assert_eq!(Array::<u32, C4>::len().0, 4);
    assert_eq!(Array::<u32, C4>::len().0, cap_size(C4));
}

#[test]
fn into_iter_yields_refs_in_order() {
    let a: Array<u32, C4> = Array::new([1, 2, 3, 4]);
    let collected: [u32; 4] = core::array::from_fn(|_| 0);
    let mut collected = collected;
    for (idx, t) in (&a).into_iter().enumerate() {
        collected[idx] = *t;
    }
    assert_eq!(collected, [1, 2, 3, 4]);
}

#[test]
fn enumerated_yields_typed_usize() {
    let a: Array<u32, C4> = Array::new([100, 200, 300, 400]);
    let mut total: usize = 0;
    for (i, t) in (&a).enumerated() {
        total += i.0 * (*t as usize);
    }
    assert_eq!(total, 0 * 100 + 1 * 200 + 2 * 300 + 3 * 400);
}

#[test]
fn mutable_iter_allows_writes() {
    let mut a: Array<u32, C4> = Array::new([0; 4]);
    for t in (&mut a).into_iter() {
        *t = 7;
    }
    assert_eq!(*a.get(USize(2)), 7);
}

#[test]
fn copy_and_clone_preserve_data() {
    let a: Array<u32, C4> = Array::new([1, 2, 3, 4]);
    let b = a;
    let c = b.clone();
    assert_eq!(*a.get(USize(0)), *b.get(USize(0)));
    assert_eq!(*b.get(USize(1)), *c.get(USize(1)));
}

#[test]
fn filled_populates_every_slot() {
    let a: Array<u32, C4> = Array::filled(42);
    for t in &a {
        assert_eq!(*t, 42);
    }
}
