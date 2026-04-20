//! Bit scanning on `Mask64`: `lowest_set`, `highest_set`,
//! `iter_set_bits` ordering.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::USize;
use arvo_bitmask::Mask64;

fn mk(bits: &[usize]) -> Mask64 {
    let mut m = Mask64::empty();
    for b in bits {
        m.insert(USize(*b));
    }
    m
}

#[test]
fn lowest_set_single_bit() {
    let m = mk(&[5]);
    assert_eq!(m.lowest_set(), USize(5));
}

#[test]
fn lowest_set_picks_minimum() {
    let m = mk(&[3, 10, 40, 63]);
    assert_eq!(m.lowest_set(), USize(3));
}

#[test]
fn lowest_set_empty_is_width() {
    let m = Mask64::empty();
    // trailing_zeros of zero word returns container width (64).
    assert_eq!(m.lowest_set(), USize(64));
}

#[test]
fn highest_set_picks_maximum() {
    let m = mk(&[0, 7, 32, 55]);
    assert_eq!(m.highest_set(), USize(55));
}

#[test]
fn highest_set_topmost() {
    let m = mk(&[63]);
    assert_eq!(m.highest_set(), USize(63));
}

#[test]
fn highest_set_empty_is_width() {
    let m = Mask64::empty();
    assert_eq!(m.highest_set(), USize(64));
}

#[test]
fn iter_set_bits_lowest_first() {
    let m = mk(&[0, 5, 10, 63]);
    let mut collected = [0usize; 64];
    let mut n = 0usize;
    for u in m.iter_set_bits() {
        collected[n] = u.0;
        n += 1;
    }
    assert_eq!(&collected[..n], &[0, 5, 10, 63]);
}

#[test]
fn iter_set_bits_empty_yields_nothing() {
    let m = Mask64::empty();
    let mut it = m.iter_set_bits();
    assert!(it.next().is_none());
}

#[test]
fn iter_set_bits_dense() {
    let mut m = Mask64::empty();
    for i in 0..64 {
        m.insert(USize(i));
    }
    let mut collected = [0usize; 64];
    let mut n = 0usize;
    for u in m.iter_set_bits() {
        collected[n] = u.0;
        n += 1;
    }
    assert_eq!(n, 64);
    for (idx, bit) in collected[..n].iter().enumerate() {
        assert_eq!(idx, *bit);
    }
}
