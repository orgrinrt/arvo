//! An out-of-range bit position leaves the word unchanged.
//!
//! `BitsBitPrim` states this and `BitMatrix::set_edge` relies on it for the
//! column axis: the row bound is checked explicitly, the column bound is
//! not, because the word's own insert is contractually a no-op past its
//! width. Nothing pinned that, and the plausible regression is silent
//! rather than loud.
//!
//! Setting a bit is a shift by the position, and a native shift by 70 on a
//! 64-bit word masks the shift amount to 6 in release builds. Were that to
//! reach the container, `set_edge(nid(0), nid(70))` would set column **6**:
//! an edge nobody asked for, in a graph, feeding every algorithm
//! downstream. Every existing test would still pass, because none of them
//! addresses a column that does not exist.
//!
//! A compile-time refusal is not available here without changing the API,
//! since `set_edge` takes runtime `NodeId` values, so a runtime pin is the
//! strongest form the case has.

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_tensor::Dim;

type M = BitMatrix<Bits<64, Hot, Unsigned>, Dim<8>>;

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

/// The row word is 64 bits wide, so columns 64 and above do not exist.
/// `Dim<8>` sizes the row count; the column count comes from the word.
const OUT_OF_RANGE: [usize; 5] = [64, 65, 70, 127, 128];

#[test]
fn out_of_range_column_sets_nothing() {
    for col in OUT_OF_RANGE {
        let mut m = M::empty();
        m.set_edge(nid(0), nid(col));

        for probe in 0..64 {
            assert!(
                !m.edge(nid(0), nid(probe)).0,
                "set_edge(0, {col}) set column {probe}, which was never asked for",
            );
        }
    }
}

/// The specific aliasing a masked shift would produce, named on its own so
/// a failure reads as the regression rather than as one cell of a loop.
#[test]
fn out_of_range_column_does_not_alias_to_its_masked_position() {
    let mut m = M::empty();
    m.set_edge(nid(0), nid(70));
    assert!(
        !m.edge(nid(0), nid(6)).0,
        "column 70 aliased to column 6: the shift amount was masked to 70 % 64",
    );
}

/// The out-of-range write must not disturb edges that were already set,
/// which is a different failure from setting a phantom one.
#[test]
fn out_of_range_column_leaves_existing_edges_intact() {
    let mut m = M::empty();
    m.set_edge(nid(0), nid(3));
    m.set_edge(nid(0), nid(63));

    for col in OUT_OF_RANGE {
        m.set_edge(nid(0), nid(col));
    }

    assert!(m.edge(nid(0), nid(3)).0, "edge 0 -> 3 was disturbed");
    assert!(m.edge(nid(0), nid(63)).0, "edge 0 -> 63 was disturbed");
}

/// Clearing is the same shift and carries the same hazard, so it gets the
/// same pin rather than being assumed symmetric.
#[test]
fn out_of_range_column_clear_removes_nothing() {
    let mut m = M::empty();
    m.set_edge(nid(0), nid(6));

    for col in OUT_OF_RANGE {
        m.clear_edge(nid(0), nid(col));
    }

    assert!(
        m.edge(nid(0), nid(6)).0,
        "an out-of-range clear removed column 6, so the position aliased",
    );
}

/// The row axis is bounds-checked explicitly rather than by contract, so it
/// is pinned separately: `Dim<8>` means rows 8 and above do not exist.
#[test]
fn out_of_range_row_sets_nothing() {
    for row in [8, 9, 64, 70] {
        let mut m = M::empty();
        m.set_edge(nid(row), nid(1));

        for probe in 0..8 {
            assert!(
                !m.edge(nid(probe), nid(1)).0,
                "set_edge({row}, 1) set row {probe}, which was never asked for",
            );
        }
    }
}
