//! `Matrix` construction, get/set, and diagonal round-trip.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_spectral::Matrix;

mod common;

const C4: Cap = Cap(USize(4));
const C3: Cap = Cap(USize(3));

#[test]
fn from_fn_builds_expected_cells() {
    // f(i, j) = 10 * i + j.
    let m: Matrix<u32, C4> = Matrix::from_fn(|i, j| (10 * i.0 + j.0) as u32);
    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(m.get(USize(i), USize(j)), (10 * i + j) as u32);
        }
    }
}

#[test]
fn set_then_get_round_trips() {
    let mut m: Matrix<u32, C3> = Matrix::from_fn(|_, _| 0);
    m.set(USize(1), USize(2), 42);
    assert_eq!(m.get(USize(1), USize(2)), 42);
    // Other cells untouched.
    assert_eq!(m.get(USize(0), USize(0)), 0);
    assert_eq!(m.get(USize(2), USize(1)), 0);
}

#[test]
fn diagonal_extracts_ii_cells() {
    // Cell value = 100 * i + j. Diagonal is [0, 101, 202, 303].
    let m: Matrix<u32, C4> = Matrix::from_fn(|i, j| (100 * i.0 + j.0) as u32);
    let d = m.diagonal();
    // d is `arvo_tensor::Array<u32, C4>`; compare via slice view.
    assert_eq!(d.as_slice(), &[0, 101, 202, 303]);
}

#[test]
fn new_wraps_given_array() {
    let data: [[u32; 3]; 3] =
        [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let m: Matrix<u32, C3> = Matrix::new(data);
    assert_eq!(m.get(USize(0), USize(0)), 1);
    assert_eq!(m.get(USize(1), USize(1)), 5);
    assert_eq!(m.get(USize(2), USize(2)), 9);
    assert_eq!(m.get(USize(0), USize(2)), 3);
    assert_eq!(m.get(USize(2), USize(0)), 7);
}
