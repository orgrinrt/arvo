//! Bit scanning on `Mask<Bits<64, Hot, Unsigned>>`: `lowest_set`, `highest_set`,
//! `iter_set_bits` ordering.

#![allow(incomplete_features)]

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_bitmask::Mask;

fn mk(bits: &[usize]) -> Mask<Bits<64, Hot, Unsigned>> {
    let mut m = Mask::<Bits<64, Hot, Unsigned>>::empty();
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
    let m = Mask::<Bits<64, Hot, Unsigned>>::empty();
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
    let m = Mask::<Bits<64, Hot, Unsigned>>::empty();
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
    let m = Mask::<Bits<64, Hot, Unsigned>>::empty();
    let mut it = m.iter_set_bits();
    assert!(it.next().is_none());
}

#[test]
fn iter_set_bits_dense() {
    let mut m = Mask::<Bits<64, Hot, Unsigned>>::empty();
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
