//! `BitLogic` coverage: `bitor` / `bitand` / `bitnot` / `bitxor` /
//! `clear_lowest_set_bit` on Hot containers.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::strategy::Hot;
use arvo_bits::{BitLogic, BitSequence, Byte, QWord};

#[test]
fn byte_bitor() {
    let a = Byte::<Hot>::from_raw(0b1010_0000);
    let b = Byte::<Hot>::from_raw(0b0000_1010);
    let c = <Byte<Hot> as BitLogic>::bitor(a, b);
    assert_eq!(c.to_raw(), 0b1010_1010);
}

#[test]
fn byte_bitand() {
    let a = Byte::<Hot>::from_raw(0b1111_1100);
    let b = Byte::<Hot>::from_raw(0b0011_1111);
    let c = <Byte<Hot> as BitLogic>::bitand(a, b);
    assert_eq!(c.to_raw(), 0b0011_1100);
}

#[test]
fn byte_bitnot() {
    let a = Byte::<Hot>::from_raw(0b1010_0000);
    let c = <Byte<Hot> as BitLogic>::bitnot(a);
    assert_eq!(c.to_raw(), 0b0101_1111);
}

#[test]
fn byte_bitxor() {
    let a = Byte::<Hot>::from_raw(0b1111_0000);
    let b = Byte::<Hot>::from_raw(0b1010_1010);
    let c = <Byte<Hot> as BitLogic>::bitxor(a, b);
    assert_eq!(c.to_raw(), 0b0101_1010);
}

#[test]
fn qword_bitor_bitand() {
    let a = QWord::<Hot>::from_raw(0xFFFF_0000_0000_FFFF);
    let b = QWord::<Hot>::from_raw(0x0000_FFFF_FFFF_0000);
    assert_eq!(<QWord<Hot> as BitLogic>::bitor(a, b).to_raw(), 0xFFFF_FFFF_FFFF_FFFF);
    assert_eq!(<QWord<Hot> as BitLogic>::bitand(a, b).to_raw(), 0);
}

#[test]
fn qword_bitxor_self_is_zero() {
    let a = QWord::<Hot>::from_raw(0xDEAD_BEEF_CAFE_BABE);
    let c = <QWord<Hot> as BitLogic>::bitxor(a, a);
    assert_eq!(c.to_raw(), 0);
}

#[test]
fn clear_lowest_set_bit_zero_is_noop() {
    let a = QWord::<Hot>::from_raw(0);
    let c = <QWord<Hot> as BitLogic>::clear_lowest_set_bit(a);
    assert_eq!(c.to_raw(), 0);
}

#[test]
fn clear_lowest_set_bit_single_bit() {
    let a = QWord::<Hot>::from_raw(0b1000);
    let c = <QWord<Hot> as BitLogic>::clear_lowest_set_bit(a);
    assert_eq!(c.to_raw(), 0);
}

#[test]
fn clear_lowest_set_bit_multi_bit() {
    let a = QWord::<Hot>::from_raw(0b1010);
    let c = <QWord<Hot> as BitLogic>::clear_lowest_set_bit(a);
    assert_eq!(c.to_raw(), 0b1000);
}

#[test]
fn clear_lowest_set_bit_iterates_to_empty() {
    let mut q = QWord::<Hot>::from_raw(0xAAAA_AAAA_AAAA_AAAA);
    let mut remaining_ones: usize = 32;
    while !<QWord<Hot> as BitSequence>::is_zero(q).0 {
        q = <QWord<Hot> as BitLogic>::clear_lowest_set_bit(q);
        remaining_ones -= 1;
    }
    assert_eq!(remaining_ones, 0);
}
