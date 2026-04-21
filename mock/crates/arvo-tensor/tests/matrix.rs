//! Smoke tests for `Matrix<W, N>`.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::{Cap, USize};
use arvo_tensor::{Array, Matrix};

const C3: Cap = Cap(USize(3));

#[test]
fn from_fn_with_typed_indices() {
    let m: Matrix<u32, C3> = Matrix::from_fn(|i, j| (i.0 * 10 + j.0) as u32);
    assert_eq!(m.get(USize(0), USize(0)), 0);
    assert_eq!(m.get(USize(1), USize(2)), 12);
    assert_eq!(m.get(USize(2), USize(2)), 22);
}

#[test]
fn set_overwrites_cell() {
    let mut m: Matrix<u32, C3> = Matrix::filled(0);
    m.set(USize(1), USize(1), 99);
    assert_eq!(m.get(USize(1), USize(1)), 99);
    assert_eq!(m.get(USize(0), USize(1)), 0);
}

#[test]
fn diagonal_returns_array() {
    let m: Matrix<u32, C3> = Matrix::from_fn(|i, j| (i.0 * 10 + j.0) as u32);
    let diag: Array<u32, C3> = m.diagonal();
    assert_eq!(*diag.get(USize(0)), 0);
    assert_eq!(*diag.get(USize(1)), 11);
    assert_eq!(*diag.get(USize(2)), 22);
}
