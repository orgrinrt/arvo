//! Smoke tests for `Matrix<W, C>`.
//!
//! No `#![feature(...)]` gates: the `Capacity`-migrated `Matrix` escapes the
//! `generic_const_exprs` surface (the capacity is a type on both axes, not a
//! `Cap` const generic with `cap_size(N)` in type position).

use arvo::USize;
use arvo_tensor::{Array, Dim, Matrix};

#[test]
fn from_fn_with_typed_indices() {
    let m: Matrix<u32, Dim<3>> = Matrix::from_fn(|i, j| (i.0 * 10 + j.0) as u32);
    assert_eq!(m.get(USize(0), USize(0)), 0);
    assert_eq!(m.get(USize(1), USize(2)), 12);
    assert_eq!(m.get(USize(2), USize(2)), 22);
}

#[test]
fn set_overwrites_cell() {
    let mut m: Matrix<u32, Dim<3>> = Matrix::filled(0);
    m.set(USize(1), USize(1), 99);
    assert_eq!(m.get(USize(1), USize(1)), 99);
    assert_eq!(m.get(USize(0), USize(1)), 0);
}

#[test]
fn diagonal_returns_array() {
    let m: Matrix<u32, Dim<3>> = Matrix::from_fn(|i, j| (i.0 * 10 + j.0) as u32);
    let diag: Array<u32, Dim<3>> = m.diagonal();
    assert_eq!(*diag.get(USize(0)), 0);
    assert_eq!(*diag.get(USize(1)), 11);
    assert_eq!(*diag.get(USize(2)), 22);
}
