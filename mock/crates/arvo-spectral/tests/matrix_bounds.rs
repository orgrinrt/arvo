//! Pins the post-review fix (F5): `Matrix::get` / `Matrix::set` must
//! reject out-of-range indices under `cfg(debug_assertions)` with a
//! `debug_assert!` panic (zero release cost, catches bugs in dev).

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_spectral::Matrix;

mod common;
use common::TF;

const C4: Cap = Cap(USize(4));

#[test]
fn matrix_get_in_bounds_succeeds() {
    let m: Matrix<TF, C4> = Matrix::from_fn(|i, j| TF((i.0 * 4 + j.0) as f32));
    assert_eq!(m.get(USize(0), USize(0)), TF(0.0));
    assert_eq!(m.get(USize(3), USize(3)), TF(15.0));
}

#[test]
fn matrix_set_in_bounds_succeeds() {
    let mut m: Matrix<TF, C4> = Matrix::from_fn(|_, _| TF(0.0));
    m.set(USize(2), USize(1), TF(42.0));
    assert_eq!(m.get(USize(2), USize(1)), TF(42.0));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "Matrix::get: row index out of range")]
fn matrix_get_row_out_of_range_debug_panics() {
    let m: Matrix<TF, C4> = Matrix::from_fn(|_, _| TF(0.0));
    let _ = m.get(USize(4), USize(0));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "Matrix::get: column index out of range")]
fn matrix_get_col_out_of_range_debug_panics() {
    let m: Matrix<TF, C4> = Matrix::from_fn(|_, _| TF(0.0));
    let _ = m.get(USize(0), USize(4));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "Matrix::set: row index out of range")]
fn matrix_set_row_out_of_range_debug_panics() {
    let mut m: Matrix<TF, C4> = Matrix::from_fn(|_, _| TF(0.0));
    m.set(USize(4), USize(0), TF(0.0));
}
