//! `Mask256` behaviour across multi-word boundaries.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::USize;
use arvo_bitmask::Mask256;

fn mk(bits: &[usize]) -> Mask256 {
    let mut m = Mask256::empty();
    for b in bits {
        m.insert(USize(*b));
    }
    m
}

#[test]
fn empty_is_empty() {
    let m = Mask256::empty();
    assert!(*m.is_empty());
    assert_eq!(m.count(), USize(0));
    assert_eq!(m.lowest_set(), USize(256));
    assert_eq!(m.highest_set(), USize(256));
}

#[test]
fn insert_in_each_word() {
    let m = mk(&[0, 64, 128, 192, 255]);
    assert_eq!(m.count(), USize(5));
    assert!(*m.contains(USize(0)));
    assert!(*m.contains(USize(64)));
    assert!(*m.contains(USize(128)));
    assert!(*m.contains(USize(192)));
    assert!(*m.contains(USize(255)));
    assert!(!*m.contains(USize(63)));
    assert!(!*m.contains(USize(127)));
}

#[test]
fn out_of_range_contains_is_false() {
    let m = mk(&[5]);
    assert!(!*m.contains(USize(256)));
    assert!(!*m.contains(USize(999)));
}

#[test]
fn out_of_range_insert_and_remove_noop() {
    let mut m = mk(&[5]);
    m.insert(USize(256));
    m.insert(USize(1000));
    assert_eq!(m.count(), USize(1));
    m.remove(USize(256));
    assert_eq!(m.count(), USize(1));
}

#[test]
fn union_across_words() {
    let a = mk(&[0, 64, 128]);
    let b = mk(&[1, 192, 255]);
    let u = a.union(b);
    assert_eq!(u.count(), USize(6));
    for b in &[0, 1, 64, 128, 192, 255] {
        assert!(*u.contains(USize(*b)));
    }
}

#[test]
fn intersection_across_words() {
    let a = mk(&[0, 64, 128, 192]);
    let b = mk(&[64, 128, 255]);
    let i = a.intersection(b);
    assert_eq!(i.count(), USize(2));
    assert!(*i.contains(USize(64)));
    assert!(*i.contains(USize(128)));
    assert!(!*i.contains(USize(192)));
}

#[test]
fn difference_across_words() {
    let a = mk(&[0, 64, 128, 192]);
    let b = mk(&[64, 128]);
    let d = a.difference(b);
    assert_eq!(d.count(), USize(2));
    assert!(*d.contains(USize(0)));
    assert!(*d.contains(USize(192)));
}

#[test]
fn complement_covers_all_bits() {
    let m = mk(&[0, 100, 255]);
    let c = m.complement();
    assert_eq!(c.count(), USize(256 - 3));
    assert!(!*c.contains(USize(0)));
    assert!(!*c.contains(USize(100)));
    assert!(!*c.contains(USize(255)));
    assert!(*c.contains(USize(64)));
    assert!(*c.contains(USize(200)));
}

#[test]
fn intersects_across_words() {
    let a = mk(&[0, 200]);
    let b = mk(&[200, 201]);
    let c = mk(&[5, 250]);
    assert!(*a.intersects(b));
    assert!(!*a.intersects(c));
}

#[test]
fn lowest_set_respects_word_order() {
    let m = mk(&[192, 200, 3]);
    assert_eq!(m.lowest_set(), USize(3));
    let m = mk(&[100, 200]);
    assert_eq!(m.lowest_set(), USize(100));
    let m = mk(&[250]);
    assert_eq!(m.lowest_set(), USize(250));
}

#[test]
fn highest_set_respects_word_order() {
    let m = mk(&[3, 100, 200]);
    assert_eq!(m.highest_set(), USize(200));
    let m = mk(&[5]);
    assert_eq!(m.highest_set(), USize(5));
    let m = mk(&[255]);
    assert_eq!(m.highest_set(), USize(255));
}

#[test]
fn iter_set_bits_crosses_words_in_order() {
    let m = mk(&[0, 63, 64, 100, 127, 128, 192, 255]);
    let collected: Vec<_> = m.iter_set_bits().map(|u| u.0).collect();
    assert_eq!(collected, vec![0, 63, 64, 100, 127, 128, 192, 255]);
}

#[test]
fn iter_set_bits_empty_yields_nothing() {
    let m = Mask256::empty();
    let mut it = m.iter_set_bits();
    assert!(it.next().is_none());
}
