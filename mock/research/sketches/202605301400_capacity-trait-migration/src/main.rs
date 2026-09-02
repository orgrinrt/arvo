//! Sketch (#651): migrate the `const N: Cap`-parameterised trait family in
//! arvo-sparse / arvo-spectral onto a `C: Capacity` type parameter.
//!
//! #650 proved the standalone container shape (`Capacity` + `Dim` + a GAT
//! `type Array<T>`, consumed from generic code, GCE-free). It did NOT prove
//! the part #651 actually trips over: the cross-cutting *trait* families that
//! today are parameterised by `const N: Cap` and impl'd for several concrete
//! types. Those are the real coupling points (one trait, many impls), so they
//! must migrate in lockstep, and they carry features #650's sketch never
//! exercised:
//!
//!   1. `pub const trait SparseAdjacency<const N: Cap>` with its own GAT
//!      (`type Successors<'a>: Iterator where Self: 'a`), sealed, impl'd for
//!      BitMatrix AND Csr.
//!   2. A sub-trait `BidirectionalSparseAdjacency<const N: Cap>: SparseAdjacency<N>`.
//!   3. Problem-shaped traits with DEFAULT methods returning a fixed-capacity
//!      array (`fn reduce_bandwidth(&self) -> [NodeId; cap_size(N)]`), plus
//!      blanket impls `impl<T: BidirectionalSparseAdjacency<N>> Reducer<N> for T`.
//!   4. `LinearOperator<F, const N: Cap>` impl'd for Matrix AND SparseLaplacian.
//!   5. A generic consumer that THREADS the capacity (`fn run<C: Capacity, A:
//!      Adjacency<C>>(..) -> C::Array<usize>`), the exact shape that ICE'd
//!      under `cap_size(cap(N))` and that #652 needs.
//!
//! The questions this answers:
//!   Q1. Does `trait Adjacency<C: Capacity>` + a GAT compile and resolve when
//!       impl'd for two distinct types and consumed generically?
//!   Q2. Do the DEFAULT-method problem-traits + blanket impls still type-check
//!       when the array return becomes `C::Array<_>` instead of `[_; cap_size(N)]`?
//!   Q3. Does the migration stay GCE-free (no `generic_const_exprs`, no
//!       `cap_size` in any type position)?
//!   Q4. Can the trait family keep the `const` qualifier given a non-const
//!       `C: Capacity` bound, or must `const` be dropped? (Probed in the
//!       `const_variant` module under the const_trait_impl feature.)
//!
//! Outcome recorded at the bottom of this file and in FINDINGS.md.

#![allow(dead_code)]
// Enabled only to probe Q4 (can the migrated trait family keep `const`?). The
// plain-trait migration above this point needs no features (Q3: GCE-free).
#![feature(const_trait_impl)]

// ---------------------------------------------------------------------
// Toy stand-ins for the shipped #650 foundation. Only the shapes matter;
// the real ones are arvo::Cap / arvo_tensor::{Capacity, Dim, cap, cap_size}.
// This mirrors the SHIPPED Capacity (filled + CAP + Array), NOT the larger
// #650-sketch variant (which still had const N / as_slice).
// ---------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cap(usize);
const fn cap(n: usize) -> Cap {
    Cap(n)
}
const fn cap_size(c: Cap) -> usize {
    c.0
}

trait Capacity {
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
    const CAP: Cap;
    fn filled<T: Copy>(v: T) -> Self::Array<T>;
    // The migration also needs from_fn (Array/Matrix expose it; the diagonal
    // extractor and several algorithms build by index). Proven here too.
    fn from_fn<T, F: FnMut(usize) -> T>(f: F) -> Self::Array<T>;
}

struct Dim<const N: usize>;
impl<const N: usize> Capacity for Dim<N> {
    type Array<T> = [T; N];
    const CAP: Cap = cap(N);
    fn filled<T: Copy>(v: T) -> [T; N] {
        [v; N]
    }
    fn from_fn<T, F: FnMut(usize) -> T>(f: F) -> [T; N] {
        core::array::from_fn(f)
    }
}

// NodeId stand-in (arvo_bitmask::NodeId is a newtype over USize).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NodeId(usize);

// ---------------------------------------------------------------------
// Plain-trait variant (the leading candidate: drop `const`, since these
// methods are runtime graph queries, not const-eval).
// ---------------------------------------------------------------------

// Mirror of SparseAdjacency<const N: Cap> -> SparseAdjacency<C: Capacity>.
trait Adjacency<C: Capacity> {
    type Successors<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;
    fn successors<'a>(&'a self, node: NodeId) -> Self::Successors<'a>;
    fn node_count(&self) -> usize;
}

// Mirror of BidirectionalSparseAdjacency<N>: SparseAdjacency<N>.
trait BiAdjacency<C: Capacity>: Adjacency<C> {
    type Predecessors<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;
    fn predecessors<'a>(&'a self, node: NodeId) -> Self::Predecessors<'a>;
}

// Mirror of BitMatrix<W, N>: a row-per-node bitset. Toy: a Dim-sized array of
// u64 row masks. The capacity C drives row count.
struct BitMatrixLike<C: Capacity> {
    rows: C::Array<u64>,
}

// A successors iterator over set bits of one u64 row (toy SetBitsIter).
struct BitRowIter {
    bits: u64,
}
impl Iterator for BitRowIter {
    type Item = NodeId;
    fn next(&mut self) -> Option<NodeId> {
        if self.bits == 0 {
            None
        } else {
            let i = self.bits.trailing_zeros() as usize;
            self.bits &= self.bits - 1;
            Some(NodeId(i))
        }
    }
}

impl<C: Capacity> Adjacency<C> for BitMatrixLike<C> {
    type Successors<'a>
        = BitRowIter
    where
        Self: 'a;
    fn successors<'a>(&'a self, node: NodeId) -> BitRowIter {
        BitRowIter {
            bits: self.rows.as_ref()[node.0],
        }
    }
    fn node_count(&self) -> usize {
        cap_size(<C as Capacity>::CAP)
    }
}
impl<C: Capacity> BiAdjacency<C> for BitMatrixLike<C> {
    type Predecessors<'a>
        = BitRowIter
    where
        Self: 'a;
    fn predecessors<'a>(&'a self, _node: NodeId) -> BitRowIter {
        // toy: transpose not modelled; return empty.
        BitRowIter { bits: 0 }
    }
}

// Mirror of Csr<const ROWS: Cap, const NNZ: Cap, W> impl SparseAdjacency<ROWS>.
// Two independent capacities, the second naming the nnz storage.
struct CsrLike<R: Capacity, NNZ: Capacity, W: Copy> {
    row_ptr: R::Array<usize>,
    col_idx: NNZ::Array<NodeId>,
    values: NNZ::Array<W>,
    live_rows: usize,
    live_nnz: usize,
}

struct CsrRowIter<'a> {
    col_idx: &'a [NodeId],
    pos: usize,
    end: usize,
}
impl<'a> Iterator for CsrRowIter<'a> {
    type Item = NodeId;
    fn next(&mut self) -> Option<NodeId> {
        if self.pos >= self.end {
            None
        } else {
            let n = self.col_idx[self.pos];
            self.pos += 1;
            Some(n)
        }
    }
}

// Csr impls Adjacency over its ROW capacity R (the node count). This is the
// key coupling: the SAME trait `Adjacency<C>` is impl'd for both BitMatrixLike
// and CsrLike, so migrating the trait forces both impls in lockstep.
impl<R: Capacity, NNZ: Capacity, W: Copy> Adjacency<R> for CsrLike<R, NNZ, W> {
    type Successors<'a>
        = CsrRowIter<'a>
    where
        Self: 'a;
    fn successors<'a>(&'a self, node: NodeId) -> CsrRowIter<'a> {
        let rp = self.row_ptr.as_ref();
        let start = rp[node.0];
        let end = if node.0 + 1 < rp.len() {
            rp[node.0 + 1]
        } else {
            self.live_nnz
        };
        CsrRowIter {
            col_idx: self.col_idx.as_ref(),
            pos: start,
            end,
        }
    }
    fn node_count(&self) -> usize {
        self.live_rows
    }
}

// ---------------------------------------------------------------------
// Problem-shaped traits with DEFAULT methods returning a fixed-capacity array.
// Mirror of BandwidthReducer<N>: the default runs an algorithm and returns
// `[NodeId; cap_size(N)]` -> now `C::Array<NodeId>`. Blanket-impl'd for every
// BiAdjacency. This is Q2.
// ---------------------------------------------------------------------
trait Reducer<C: Capacity>: BiAdjacency<C> {
    fn reduce_bandwidth(&self) -> C::Array<NodeId>
    where
        Self: Sized,
    {
        // toy "algorithm": identity permutation built generically, GCE-free.
        rcm_reorder_via::<C, Self>(self)
    }
}
impl<C: Capacity, T: BiAdjacency<C>> Reducer<C> for T {}

// Mirror of the free `rcm_reorder_via<T, const N: Cap>` -> `<C: Capacity, A>`.
// Builds a `C::Array<NodeId>` from generic adjacency. The exact generic-thread
// shape that ICE'd under `cap_size(cap(N))`.
fn rcm_reorder_via<C: Capacity, A: Adjacency<C>>(adj: &A) -> C::Array<NodeId> {
    let n = adj.node_count();
    // scratch built via from_fn over the capacity, no cap_size in type position.
    let mut perm: C::Array<NodeId> = C::from_fn(|i| NodeId(i));
    // walk: just touch successors to exercise the GAT iterator generically.
    let slots = perm.as_mut();
    let mut visited = 0usize;
    let mut node = 0usize;
    while node < n && visited < slots.len() {
        let mut succ = adj.successors(NodeId(node));
        if let Some(first) = succ.next() {
            // toy reorder: record first successor; real RCM does BFS.
            slots[visited] = first;
        }
        visited += 1;
        node += 1;
    }
    perm
}

// ---------------------------------------------------------------------
// LinearOperator<F, const N: Cap> -> LinearOperator<F, C: Capacity>, impl'd
// for both a dense Matrix-like and a sparse-Laplacian-like, mirroring the
// arvo-spectral coupling (operator.rs:47 and :169). Q1/Q2 for the second
// cross-cutting trait.
// ---------------------------------------------------------------------
trait LinearOperator<F: Copy, C: Capacity> {
    // apply: y = A x, returning a fresh C::Array<F>.
    fn apply(&self, x: &C::Array<F>) -> C::Array<F>;
    fn dim(&self) -> usize;
}

struct MatrixLike<F: Copy, C: Capacity> {
    data: C::Array<C::Array<F>>, // 2-D = composition, the #650 nested shape.
}
impl<F: Copy + core::ops::Add<Output = F> + core::ops::Mul<Output = F>, C: Capacity>
    LinearOperator<F, C> for MatrixLike<F, C>
where
    C::Array<F>: Copy,
{
    fn apply(&self, x: &C::Array<F>) -> C::Array<F> {
        let xs = x.as_ref();
        let rows = self.data.as_ref();
        C::from_fn(|i| {
            let row = rows[i].as_ref();
            // dot product row . x; seed with row[0]*x[0] to avoid needing Zero.
            let mut acc = row[0] * xs[0];
            let mut j = 1usize;
            while j < row.len() && j < xs.len() {
                acc = acc + row[j] * xs[j];
                j += 1;
            }
            acc
        })
    }
    fn dim(&self) -> usize {
        self.data.as_ref().len()
    }
}

// A sparse-Laplacian-like operator wrapping a CsrLike, impl'ing the SAME
// LinearOperator<F, C> trait over the row capacity. The second impl that the
// trait migration forces in lockstep with MatrixLike.
struct SparseLapLike<'d, R: Capacity, NNZ: Capacity, F: Copy> {
    csr: &'d CsrLike<R, NNZ, F>,
}
impl<'d, R: Capacity, NNZ: Capacity, F: Copy + core::ops::Add<Output = F>>
    LinearOperator<F, R> for SparseLapLike<'d, R, NNZ, F>
where
    R::Array<F>: Copy,
{
    fn apply(&self, x: &R::Array<F>) -> R::Array<F> {
        let xs = x.as_ref();
        let rp = self.csr.row_ptr.as_ref();
        let cols = self.csr.col_idx.as_ref();
        let vals = self.csr.values.as_ref();
        R::from_fn(|i| {
            // toy spmv: sum over the row's nnz of value (ignoring x weight to
            // avoid needing a multiplicative identity); shape is what matters.
            let start = rp[i];
            let end = if i + 1 < rp.len() { rp[i + 1] } else { self.csr.live_nnz };
            let mut acc = xs[i];
            let mut k = start;
            while k < end && k < vals.len() {
                let _c = cols[k];
                acc = acc + vals[k];
                k += 1;
            }
            acc
        })
    }
    fn dim(&self) -> usize {
        self.csr.live_rows
    }
}

// ---------------------------------------------------------------------
// Q4: can the family keep `const`? Probe a const variant of the adjacency
// trait under the feature, with a non-const `Capacity` bound.
// ---------------------------------------------------------------------
mod const_variant {
    #![allow(dead_code)]
    use super::{Cap, cap, cap_size, Capacity, Dim, NodeId};

    // The real arvo traits are `pub const trait`. Probe whether the const
    // qualifier survives a non-const `C: Capacity` bound, when the method body
    // only reads an associated const + calls a const fn (no non-const-stable
    // ops like Ord::min). If even this works, const can be kept where bodies
    // stay const-clean; where a body needs runtime-only ops, const is dropped.
    const trait ConstAdjacency<C: Capacity> {
        fn node_count(&self) -> usize;
    }

    struct Toy<C: Capacity> {
        _c: core::marker::PhantomData<C>,
        live: usize,
    }
    impl<C: Capacity> const ConstAdjacency<C> for Toy<C> {
        fn node_count(&self) -> usize {
            // reads a non-const Capacity assoc const in a const-trait method,
            // const-clean (no Ord::min).
            cap_size(<C as Capacity>::CAP)
        }
    }

    pub fn probe() -> usize {
        let t = Toy::<Dim<4>> {
            _c: core::marker::PhantomData,
            live: 3,
        };
        ConstAdjacency::<Dim<4>>::node_count(&t)
    }
}

// ---------------------------------------------------------------------
// Consumers: non-generic default-dims call site + a deeply-generic wrapper.
// ---------------------------------------------------------------------
fn build_bitmatrix<C: Capacity>(rows: C::Array<u64>) -> BitMatrixLike<C> {
    BitMatrixLike { rows }
}

// Fully-generic wrapper: threads C through the whole trait family, returns
// C::Array<NodeId>. The shape #652 needs (PlanDims member as the capacity).
fn analyse<C: Capacity, A: BiAdjacency<C>>(adj: &A) -> C::Array<NodeId> {
    adj.reduce_bandwidth_generic()
}
trait ReducerExt<C: Capacity>: Reducer<C> {
    fn reduce_bandwidth_generic(&self) -> C::Array<NodeId>
    where
        Self: Sized,
    {
        self.reduce_bandwidth()
    }
}
impl<C: Capacity, T: Reducer<C>> ReducerExt<C> for T {}

fn main() {
    // BitMatrix path with Dim<4>: 4 rows of u64.
    let bm = build_bitmatrix::<Dim<4>>([0b0010, 0b0100, 0b1000, 0b0001]);
    let perm = rcm_reorder_via::<Dim<4>, _>(&bm);
    println!("bitmatrix perm: {:?}", perm.as_ref());

    // Blanket problem-trait default method, threaded generically.
    let perm2 = analyse::<Dim<4>, _>(&bm);
    println!("bitmatrix analyse: {:?}", perm2.as_ref());

    // Csr path with two capacities Dim<4> rows, Dim<6> nnz.
    let csr = CsrLike::<Dim<4>, Dim<6>, i32> {
        row_ptr: [0, 2, 3, 5],               // R::Array<usize> = [usize; 4] for Dim<4>
        col_idx: [NodeId(1), NodeId(2), NodeId(3), NodeId(0), NodeId(2), NodeId(1)],
        values: [10, 20, 30, 40, 50, 60],
        live_rows: 4,
        live_nnz: 6,
    };
    let csr_perm = rcm_reorder_via::<Dim<4>, _>(&csr);
    println!("csr perm: {:?}", csr_perm.as_ref());

    // LinearOperator: dense Matrix path. 4x4 over i64.
    let m = MatrixLike::<i64, Dim<4>> {
        data: [
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
        ],
    };
    let x: [i64; 4] = [5, 6, 7, 8];
    let y = m.apply(&x);
    println!("matrix apply (identity): {:?} dim={}", y.as_ref(), m.dim());

    // LinearOperator: sparse path over the same csr.
    let lap = SparseLapLike::<Dim<4>, Dim<6>, i32> { csr: &csr };
    let xs: [i32; 4] = [1, 1, 1, 1];
    let ys = lap.apply(&xs);
    println!("sparselap apply: {:?} dim={}", ys.as_ref(), lap.dim());

    // const-trait probe.
    println!("const_variant probe: {}", const_variant::probe());
}

// =====================================================================
// OUTCOME (cargo +nightly-2026-05-28 run, rustc 1.98.0-nightly 57d06900f):
//
// COMPILES AND RUNS CLEAN (one unused-import warning in const_variant only).
//   bitmatrix perm:    [NodeId(1), NodeId(2), NodeId(3), NodeId(0)]
//   bitmatrix analyse: [NodeId(1), NodeId(2), NodeId(3), NodeId(0)]
//   csr perm:          [NodeId(1), NodeId(3), NodeId(0), NodeId(1)]
//   matrix apply:      [5, 6, 7, 8] dim=4
//   sparselap apply:   [31, 31, 91, 61] dim=4
//   const_variant:     4
//
// Q1 trait + GAT, two impls (BitMatrix + Csr), generic consume: PROVEN.
// Q2 default-method problem-trait + blanket impl returning C::Array:  PROVEN.
// Q3 GCE-free (no generic_const_exprs in the migration machinery):     PROVEN.
//     The only feature gate (const_trait_impl) is for the Q4 probe alone.
// Q4 keep `const` on the trait declaration: PROVEN. A `const trait
//     Foo<C: Capacity>` admits a const impl when the body is const-clean
//     (reads C::CAP + const fn); a NON-const-clean body (Ord::min, .into())
//     is rejected. Because arvo's real SparseAdjacency impls are PLAIN
//     (non-const) impls, the runtime bodies are unaffected and the
//     `pub const trait` declaration survives the parameter-kind change.
//
// Conclusion: the const N: Cap -> C: Capacity migration of the cross-cutting
// trait family is mechanical and sound on the pinned nightly. Proceed (#651).
// =====================================================================
