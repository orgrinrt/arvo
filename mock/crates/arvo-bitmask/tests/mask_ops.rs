//! Set operations on `Mask64`: union, intersection, difference,
//! complement, and the predicate surface.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::USize;
use arvo_bitmask::Mask64;

fn mk(bits: &[usize]) -> Mask64 {
    let mut m = Mask64::empty();
    for b in bits {
        m.insert(USize(*b));
    }
    m
}

#[test]
fn empty_is_empty() {
    let m = Mask64::empty();
    assert!(*m.is_empty());
    assert_eq!(m.count(), USize(0));
}

#[test]
fn insert_and_contains() {
    let mut m = Mask64::empty();
    m.insert(USize(0));
    m.insert(USize(5));
    m.insert(USize(63));
    assert!(*m.contains(USize(0)));
    assert!(*m.contains(USize(5)));
    assert!(*m.contains(USize(63)));
    assert!(!*m.contains(USize(1)));
    assert_eq!(m.count(), USize(3));
}

#[test]
fn remove_clears_bit() {
    let mut m = mk(&[1, 2, 3]);
    m.remove(USize(2));
    assert!(!*m.contains(USize(2)));
    assert!(*m.contains(USize(1)));
    assert!(*m.contains(USize(3)));
    assert_eq!(m.count(), USize(2));
}

#[test]
fn union_ors_bits() {
    let a = mk(&[0, 5, 10]);
    let b = mk(&[5, 15, 20]);
    let u = a.union(b);
    for b in &[0, 5, 10, 15, 20] {
        assert!(*u.contains(USize(*b)));
    }
    assert_eq!(u.count(), USize(5));
}

#[test]
fn intersection_ands_bits() {
    let a = mk(&[0, 5, 10]);
    let b = mk(&[5, 10, 15]);
    let i = a.intersection(b);
    assert_eq!(i.count(), USize(2));
    assert!(*i.contains(USize(5)));
    assert!(*i.contains(USize(10)));
    assert!(!*i.contains(USize(0)));
    assert!(!*i.contains(USize(15)));
}

#[test]
fn difference_is_self_and_not_other() {
    let a = mk(&[0, 5, 10]);
    let b = mk(&[5, 10, 15]);
    let d = a.difference(b);
    assert!(*d.contains(USize(0)));
    assert!(!*d.contains(USize(5)));
    assert!(!*d.contains(USize(10)));
    assert_eq!(d.count(), USize(1));
}

#[test]
fn complement_flips_all_bits() {
    let m = mk(&[0, 63]);
    let c = m.complement();
    assert!(!*c.contains(USize(0)));
    assert!(!*c.contains(USize(63)));
    assert!(*c.contains(USize(1)));
    assert!(*c.contains(USize(62)));
    assert_eq!(c.count(), USize(62));
}

#[test]
fn intersects_reports_any_overlap() {
    let a = mk(&[0, 5]);
    let b = mk(&[5, 10]);
    let c = mk(&[10, 20]);
    assert!(*a.intersects(b));
    assert!(!*a.intersects(c));
}

#[test]
fn empty_union_is_self() {
    let a = mk(&[3, 7]);
    let z = Mask64::empty();
    assert!(a.union(z) == a);
}

#[test]
fn full_difference_is_empty() {
    let a = mk(&[1, 2, 3]);
    let b = mk(&[1, 2, 3, 4, 5]);
    let d = a.difference(b);
    assert!(*d.is_empty());
}
