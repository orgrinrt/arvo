//! Probe 7. A rank-2 shape whose two axes are declared in two different
//! vocabularies gets two different perimeters, and one of them is not checked
//! at all. Executed reproduction of the shape, plus the repair.
//!
//! Why-evidence only, per the method constraint. The shipped source is read to
//! establish one factual claim and not for meaning. That claim, checked at
//! source: in `arvo-bitmask/src/matrix.rs`, `BitMatrix<W, C: Capacity>` carries
//! `pub rows: C::Array<Mask<W>>` (line 34), `edge` guards the ROW index with
//! `if row_idx >= cap_size(C::CAP) { return Bool::FALSE }` (lines 52-55), and
//! `set_edge` guards the same row index (lines 63-66) and then calls
//! `self.rows.as_mut()[row_idx].insert(j.0)` (line 67) with the COLUMN index
//! `j` passed through unguarded. The column's only bound is `W`'s physical bit
//! width. This probe reproduces that shape standalone, so nothing here depends
//! on building the tree.
//!
//! The design reading is in the deliverable. What runs here is the behaviour.

// A four-node graph in a sixty-four-bit row. Rank 2, extents 4 and 4, over a
// container that holds 64 columns.
const DECLARED_ROWS: usize = 4;
const DECLARED_COLS: usize = 4;

#[derive(Copy, Clone, Default)]
struct Row(u64);

impl Row {
    fn insert(&mut self, j: usize) {
        self.0 |= 1u64 << j; // the column bound is the CONTAINER's, not the shape's
    }
    fn contains(&self, j: usize) -> bool {
        (self.0 >> j) & 1 == 1
    }
}

struct Mat {
    rows: [Row; DECLARED_ROWS],
}

impl Mat {
    fn empty() -> Self {
        Mat {
            rows: [Row::default(); DECLARED_ROWS],
        }
    }
    /// Row index guarded against the declared extent; column index not.
    fn set_edge(&mut self, i: usize, j: usize) {
        if i >= DECLARED_ROWS {
            return;
        }
        self.rows[i].insert(j);
    }
    fn edge(&self, i: usize, j: usize) -> bool {
        if i >= DECLARED_ROWS {
            return false;
        }
        self.rows[i].contains(j)
    }
    /// The raw byte image, which is what a datum-keyed column digest hashes
    /// under the free-shortcut theorem (`91:643-651`).
    fn bytes(&self) -> [u8; DECLARED_ROWS * 8] {
        let mut out = [0u8; DECLARED_ROWS * 8];
        for (r, row) in self.rows.iter().enumerate() {
            out[r * 8..r * 8 + 8].copy_from_slice(&row.0.to_le_bytes());
        }
        out
    }
}

fn main() {
    // CLAIM A. The guarded axis behaves. A write past the declared row extent
    // is refused, and the read agrees.
    let mut a = Mat::empty();
    a.set_edge(9, 0);
    assert!(!a.edge(9, 0), "row 9 is past the extent, and stays empty");
    assert_eq!(
        a.bytes(),
        Mat::empty().bytes(),
        "and the byte image is untouched"
    );
    println!("CLAIM A  guarded axis: write at row 9 of 4 is refused, image unchanged  OK");

    // CLAIM B. The unguarded axis does not. A write past the declared column
    // extent lands, reads back true, and changes the byte image. Safe code
    // throughout: no unsafe, no transmute, no niche.
    let mut b = Mat::empty();
    b.set_edge(0, 9);
    assert!(
        b.edge(0, 9),
        "column 9 is past the extent of 4, and it took"
    );
    assert_ne!(b.bytes(), Mat::empty().bytes(), "the byte image moved");
    println!("CLAIM B  unguarded axis: write at column 9 of 4 LANDS and is readable  OK");

    // CLAIM C. Two values that are equal at every declared index have different
    // byte images, so the free raw-buffer digest shortcut (`91:643-651`)
    // separates them. This is the mutation gap `91:598-627` quantified for one
    // dimension, reached at rank 2 through an ordinary safe call, with the
    // asymmetry between the axes as the only reason it is reachable.
    let mut c = Mat::empty();
    c.set_edge(0, 1);
    let mut d = Mat::empty();
    d.set_edge(0, 1);
    d.set_edge(0, 9); // outside the declared shape entirely

    let mut agree_on_shape = true;
    for i in 0..DECLARED_ROWS {
        for j in 0..DECLARED_COLS {
            if c.edge(i, j) != d.edge(i, j) {
                agree_on_shape = false;
            }
        }
    }
    assert!(agree_on_shape, "equal at every index the shape declares");
    assert_ne!(
        c.bytes(),
        d.bytes(),
        "and distinguishable by their raw bytes"
    );
    println!("CLAIM C  value-equal at every declared index, byte images differ      OK");
    println!("         c = {:?}", &c.bytes()[..8]);
    println!("         d = {:?}", &d.bytes()[..8]);

    // CLAIM D. The repair is the shape, not a second guard. When BOTH extents
    // come from one declaration, the same check covers both axes, and there is
    // no axis left to forget: the write is refused and the images agree.
    struct Shaped {
        rows: [Row; DECLARED_ROWS],
    }
    impl Shaped {
        fn extent(axis: usize) -> usize {
            [DECLARED_ROWS, DECLARED_COLS][axis]
        }
        fn set_edge(&mut self, i: usize, j: usize) -> bool {
            // one guard, generated from the shape, per axis, not per axis-kind
            let idx = [i, j];
            let mut k = 0;
            while k < 2 {
                if idx[k] >= Self::extent(k) {
                    return false;
                }
                k += 1;
            }
            self.rows[i].insert(j);
            true
        }
        fn bytes(&self) -> [u8; DECLARED_ROWS * 8] {
            let mut out = [0u8; DECLARED_ROWS * 8];
            for (r, row) in self.rows.iter().enumerate() {
                out[r * 8..r * 8 + 8].copy_from_slice(&row.0.to_le_bytes());
            }
            out
        }
    }
    let mut e = Shaped {
        rows: [Row::default(); DECLARED_ROWS],
    };
    let mut f = Shaped {
        rows: [Row::default(); DECLARED_ROWS],
    };
    assert!(e.set_edge(0, 1));
    assert!(f.set_edge(0, 1));
    assert!(
        !f.set_edge(0, 9),
        "the out-of-shape write is refused on both axes"
    );
    assert_eq!(e.bytes(), f.bytes(), "so the byte images cannot diverge");
    println!("CLAIM D  one guard from the shape covers both axes, images agree      OK");

    println!();
    println!("The asymmetry is not a bug in a bounds check. The row extent is a");
    println!("`Capacity` and the column extent is a `Bits<N>` width, so the two");
    println!("axes of one rank-2 shape are declared in two vocabularies, and only");
    println!("one of them has a shape to be checked against.");
}
