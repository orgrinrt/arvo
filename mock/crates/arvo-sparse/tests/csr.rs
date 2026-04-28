//! Csr construction and query.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_bitmask::NodeId;
use arvo_sparse::Csr;
use notko::Maybe;

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

const R2: Cap = cap(2);
const R3: Cap = cap(3);
const R4: Cap = cap(4);
const NNZ0: Cap = cap(0);
const NNZ4: Cap = cap(4);
const NNZ6: Cap = cap(6);

fn nid(i: usize) -> NodeId {
    NodeId::new(USize(i))
}

#[test]
fn new_nnz_zero_is_empty() {
    // With NNZ = 0 the default row_ptr = [0; ROWS] truly gives an
    // empty matrix: every row range is `0..0`.
    let csr: Csr<R4, NNZ0, u32> = Csr::new();
    for r in 0..4 {
        assert_eq!(csr.nnz(USize(r)).0, 0);
        assert_eq!(csr.row_values(USize(r)).len(), 0);
        assert_eq!(csr.row_col_indices(USize(r)).len(), 0);
    }
}

#[test]
fn empty_rows_via_explicit_offsets() {
    // With NNZ > 0, an empty matrix requires `row_ptr` set to `NNZ`
    // on every row (every row's range is `NNZ..NNZ`).
    let mut csr: Csr<R3, NNZ4, u32> = Csr::new();
    csr.row_ptr = [USize(4), USize(4), USize(4)];
    for r in 0..3 {
        assert_eq!(csr.nnz(USize(r)).0, 0);
        assert_eq!(csr.row_values(USize(r)).len(), 0);
    }
}

#[test]
fn round_trip_values() {
    // Build a 3x3 matrix with non-zeros:
    // row 0: (col 0, 10), (col 2, 30)
    // row 1: (col 1, 20)
    // row 2: (col 0, 40), (col 1, 50), (col 2, 60)
    let mut csr: Csr<R3, NNZ6, u32> = Csr::new();
    csr.row_ptr = [USize(0), USize(2), USize(3)];
    csr.col_idx = [nid(0), nid(2), nid(1), nid(0), nid(1), nid(2)];
    csr.values = [10, 30, 20, 40, 50, 60];

    // Row 0.
    assert_eq!(csr.nnz(USize(0)).0, 2);
    assert_eq!(csr.get(USize(0), nid(0)), Maybe::Is(10));
    assert_eq!(csr.get(USize(0), nid(2)), Maybe::Is(30));
    assert_eq!(csr.get(USize(0), nid(1)), Maybe::Isnt);

    // Row 1.
    assert_eq!(csr.nnz(USize(1)).0, 1);
    assert_eq!(csr.get(USize(1), nid(1)), Maybe::Is(20));
    assert_eq!(csr.get(USize(1), nid(0)), Maybe::Isnt);

    // Row 2.
    assert_eq!(csr.nnz(USize(2)).0, 3);
    assert_eq!(csr.get(USize(2), nid(0)), Maybe::Is(40));
    assert_eq!(csr.get(USize(2), nid(1)), Maybe::Is(50));
    assert_eq!(csr.get(USize(2), nid(2)), Maybe::Is(60));
}

#[test]
fn row_slice_views() {
    let mut csr: Csr<R2, NNZ4, u16> = Csr::new();
    csr.row_ptr = [USize(0), USize(2)];
    csr.col_idx = [nid(0), nid(3), nid(1), nid(2)];
    csr.values = [7, 11, 13, 17];

    let r0_vals = csr.row_values(USize(0));
    let r0_cols = csr.row_col_indices(USize(0));
    assert_eq!(r0_vals, &[7, 11]);
    assert_eq!(r0_cols.len(), 2);
    assert_eq!((r0_cols[0].0).0, 0);
    assert_eq!((r0_cols[1].0).0, 3);

    let r1_vals = csr.row_values(USize(1));
    let r1_cols = csr.row_col_indices(USize(1));
    assert_eq!(r1_vals, &[13, 17]);
    assert_eq!((r1_cols[0].0).0, 1);
    assert_eq!((r1_cols[1].0).0, 2);
}

#[test]
fn out_of_range_row_is_empty() {
    let csr: Csr<R2, NNZ4, u16> = Csr::new();
    assert_eq!(csr.nnz(USize(5)).0, 0);
    assert_eq!(csr.row_values(USize(5)).len(), 0);
    assert_eq!(csr.row_col_indices(USize(5)).len(), 0);
    assert_eq!(csr.get(USize(5), nid(0)), Maybe::Isnt);
}
