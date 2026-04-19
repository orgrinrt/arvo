//! Csr construction and query.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::USize;
use arvo_bitmask::NodeId;
use arvo_sparse::Csr;

fn nid(i: usize) -> NodeId {
    NodeId::new(USize(i))
}

#[test]
fn new_nnz_zero_is_empty() {
    // With NNZ = 0 the default row_ptr = [0; ROWS] truly gives an
    // empty matrix: every row range is `0..0`.
    let csr: Csr<4, 0, u32> = Csr::new();
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
    let mut csr: Csr<3, 4, u32> = Csr::new();
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
    let mut csr: Csr<3, 6, u32> = Csr::new();
    csr.row_ptr = [USize(0), USize(2), USize(3)];
    csr.col_idx = [nid(0), nid(2), nid(1), nid(0), nid(1), nid(2)];
    csr.values = [10, 30, 20, 40, 50, 60];

    // Row 0.
    assert_eq!(csr.nnz(USize(0)).0, 2);
    assert_eq!(csr.get(USize(0), nid(0)), Some(10));
    assert_eq!(csr.get(USize(0), nid(2)), Some(30));
    assert_eq!(csr.get(USize(0), nid(1)), None);

    // Row 1.
    assert_eq!(csr.nnz(USize(1)).0, 1);
    assert_eq!(csr.get(USize(1), nid(1)), Some(20));
    assert_eq!(csr.get(USize(1), nid(0)), None);

    // Row 2.
    assert_eq!(csr.nnz(USize(2)).0, 3);
    assert_eq!(csr.get(USize(2), nid(0)), Some(40));
    assert_eq!(csr.get(USize(2), nid(1)), Some(50));
    assert_eq!(csr.get(USize(2), nid(2)), Some(60));
}

#[test]
fn row_slice_views() {
    let mut csr: Csr<2, 4, u16> = Csr::new();
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
    let csr: Csr<2, 4, u16> = Csr::new();
    assert_eq!(csr.nnz(USize(5)).0, 0);
    assert_eq!(csr.row_values(USize(5)).len(), 0);
    assert_eq!(csr.row_col_indices(USize(5)).len(), 0);
    assert_eq!(csr.get(USize(5), nid(0)), None);
}
