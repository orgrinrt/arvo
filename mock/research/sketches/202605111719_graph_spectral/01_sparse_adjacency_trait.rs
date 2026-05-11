//! Sketch 01: SparseAdjacency `pub const trait` with GAT iterator.
//!
//! Question: does rustc accept a `pub const trait` whose associated type
//! is a lifetime-generic iterator (`type Successors<'a>: Iterator<Item = NodeId> where Self: 'a`)?
//! Two concrete impls (a CSR-shaped one and a bitmask-shaped one) drive
//! a generic-over-`T: SparseAdjacency` function body that walks every
//! node's successors. The aim is to mirror the proposed shape of the
//! arvo-sparse `SparseAdjacency<N>` trait closely enough that a green
//! compile demonstrates rustc + -Znext-solver-globally accept the
//! pattern.
//!
//! Run:
//! ```
//! rustc --edition 2024 -Z unstable-options -Z next-solver=globally \
//!     01_sparse_adjacency_trait.rs -o /tmp/sketch_s1 && /tmp/sketch_s1
//! ```
//!
//! Outcome: **WORKS** on HEAD nightly with `-Znext-solver=globally`. Required
//! addition: `#![feature(const_index)]` (slice indexing inside the const trait
//! body needs `Index` as a const trait). Without that feature the build fails
//! with `error: Index is not yet stable as a const trait`. With it, both
//! impls compile and `total_edges` produces the expected count for both
//! representations.

#![feature(const_trait_impl)]
#![feature(const_index)]
#![allow(incomplete_features)]
#![allow(dead_code)]

// Minimal stand-in for arvo's `NodeId` newtype. The real one is
// repr(transparent) over Uint<N> over Bits<N, Hot, Unsigned>; the
// sketch only cares about it being a `Copy` ZST-of-newtype shape.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NodeId(pub usize);

/// Probe: a sealed `pub const trait` with an associated GAT iterator type.
///
/// The trait is `const` because the algorithm bodies that use it should be
/// callable in const contexts where the consumer types are const-known. The
/// GAT carries a lifetime that ties the iterator to the implementor's borrow.
///
/// Sealed via a private module + supertrait; the seal is omitted from this
/// sketch since it adds no rustc novelty (the seal is the same `mod sealed`
/// pattern used elsewhere in arvo). The probe is the trait shape itself.
pub const trait SparseAdjacency {
    type Successors<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    fn successors<'a>(&'a self, node: NodeId) -> Self::Successors<'a>;
    fn node_count(&self) -> usize;
}

// Impl 1: CSR-shaped sparse adjacency. Carries row offsets and column
// indices in plain slices. Successors of node `i` are a slice iterator
// over `col_indices[row_offsets[i] .. row_offsets[i + 1]]` (or
// `col_indices[row_offsets[i] ..]` for the last node).
pub struct Csr<'data> {
    pub row_offsets: &'data [usize],
    pub col_indices: &'data [NodeId],
}

impl<'data> const SparseAdjacency for Csr<'data> {
    type Successors<'a>
        = CsrSuccessors<'a>
    where
        Self: 'a;

    fn successors<'a>(&'a self, node: NodeId) -> CsrSuccessors<'a> {
        let i = node.0;
        let start = self.row_offsets[i];
        let end = if i + 1 < self.row_offsets.len() {
            self.row_offsets[i + 1]
        } else {
            self.col_indices.len()
        };
        CsrSuccessors {
            slice: &self.col_indices[start..end],
            pos: 0,
        }
    }

    fn node_count(&self) -> usize {
        self.row_offsets.len()
    }
}

pub struct CsrSuccessors<'a> {
    slice: &'a [NodeId],
    pos: usize,
}

impl<'a> Iterator for CsrSuccessors<'a> {
    type Item = NodeId;
    fn next(&mut self) -> Option<NodeId> {
        if self.pos < self.slice.len() {
            let n = self.slice[self.pos];
            self.pos += 1;
            Some(n)
        } else {
            None
        }
    }
}

// Impl 2: bitmask-shaped sparse adjacency. Each row is a 64-bit mask of
// set neighbours. Successors of node `i` are an iterator over the set
// bits of `rows[i]`.
pub struct BitAdj<'data> {
    pub rows: &'data [u64],
}

impl<'data> const SparseAdjacency for BitAdj<'data> {
    type Successors<'a>
        = BitAdjSuccessors
    where
        Self: 'a;

    fn successors<'a>(&'a self, node: NodeId) -> BitAdjSuccessors {
        BitAdjSuccessors {
            word: self.rows[node.0],
        }
    }

    fn node_count(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Copy, Clone)]
pub struct BitAdjSuccessors {
    word: u64,
}

impl Iterator for BitAdjSuccessors {
    type Item = NodeId;
    fn next(&mut self) -> Option<NodeId> {
        if self.word == 0 {
            return None;
        }
        let bit = self.word.trailing_zeros() as usize;
        self.word &= self.word - 1;
        Some(NodeId(bit))
    }
}

// Generic-over-`T: SparseAdjacency` consumer: counts total edges by
// walking every node's successor iterator. Demonstrates the trait drives
// algorithm code without runtime indirection.
fn total_edges<T: SparseAdjacency>(adj: &T) -> usize {
    let n = adj.node_count();
    let mut total = 0;
    let mut i = 0;
    while i < n {
        for _ in adj.successors(NodeId(i)) {
            total += 1;
        }
        i += 1;
    }
    total
}

fn main() {
    // 4-node graph: 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3.
    // 4 edges total.

    let csr = Csr {
        row_offsets: &[0, 2, 3, 4],
        col_indices: &[NodeId(1), NodeId(2), NodeId(3), NodeId(3)],
    };

    // Same graph as a bitmask: row[i] has bits set for each successor of i.
    // row[0] = (1 << 1) | (1 << 2) = 0b110 = 6
    // row[1] = (1 << 3) = 8
    // row[2] = (1 << 3) = 8
    // row[3] = 0
    let bitmask = BitAdj {
        rows: &[0b0110, 0b1000, 0b1000, 0b0000],
    };

    let csr_edges = total_edges(&csr);
    let bit_edges = total_edges(&bitmask);

    assert_eq!(csr_edges, 4, "CSR edge count must match");
    assert_eq!(bit_edges, 4, "BitAdj edge count must match");

    // Walk explicitly to confirm successor order semantics.
    let csr_succ_0: Vec<NodeId> = csr.successors(NodeId(0)).collect();
    assert_eq!(csr_succ_0, vec![NodeId(1), NodeId(2)]);

    let bit_succ_0: Vec<NodeId> = bitmask.successors(NodeId(0)).collect();
    assert_eq!(bit_succ_0, vec![NodeId(1), NodeId(2)]);

    println!("Sketch 01 OK: SparseAdjacency pub const trait with GAT iterator compiles and runs.");
    println!("  CSR total edges: {}", csr_edges);
    println!("  BitAdj total edges: {}", bit_edges);
}
