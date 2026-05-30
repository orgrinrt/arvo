# Sketch findings: capacity-trait migration (#651)

**Date:** 2026-05-30
**Toolchain:** `nightly-2026-05-28` (rustc 1.98.0-nightly, 57d06900f)
**Task:** #651 (migrate arvo's `const N: Cap` container/trait surface to `C: Capacity`)
**Verdict:** GREEN. The migration of the cross-cutting trait family is mechanical and sound on the pinned nightly. Proceed.

## Why this sketch exists

#650 shipped `Capacity` + `Dim` and proved the standalone container shape (a GAT `type Array<T>`, consumed from generic code, GCE-free). It did not exercise the part #651 actually trips over: the trait families in arvo-sparse and arvo-spectral that are parameterised by `const N: Cap` and implemented for several concrete types. Those traits are the coupling points (one trait, many impls), so they migrate in lockstep, and they carry shapes #650's sketch never touched: a sealed `pub const trait` with its own GAT, sub-traits, default-method problem-traits with blanket impls returning a fixed-capacity array, and a second cross-cutting trait (`LinearOperator`) impl'd for both a dense and a sparse type.

The risk was a trait-solver or const-trait interaction that would force a larger redesign than mechanical substitution. The sketch removes that risk before any firing is spent on the big atomic round.

## What was proven

The sketch reproduces the real arvo shapes with toy stand-ins for the #650 foundation (shipped `Capacity`: `type Array<T>: AsRef+AsMut`, `const CAP: Cap`, `filled`, plus the `from_fn` the migration adds):

1. `Adjacency<C: Capacity>` (mirror of `SparseAdjacency`) with a GAT `type Successors<'a>: Iterator where Self: 'a`, implemented for both `BitMatrixLike<C>` and `CsrLike<R, NNZ, W>`. The same trait, two impls, the lockstep coupling.
2. `BiAdjacency<C: Capacity>: Adjacency<C>` (mirror of `BidirectionalSparseAdjacency`) with a second GAT.
3. `Reducer<C: Capacity>: BiAdjacency<C>` (mirror of `BandwidthReducer`) with a DEFAULT method `reduce_bandwidth(&self) -> C::Array<NodeId>` and a blanket impl `impl<C: Capacity, T: BiAdjacency<C>> Reducer<C> for T`.
4. `rcm_reorder_via<C: Capacity, A: Adjacency<C>>(adj: &A) -> C::Array<NodeId>` (mirror of the free `_via` algorithm): the fully-generic-threaded shape that ICE'd under `cap_size(cap(N))` and that #652 needs.
5. `LinearOperator<F, C: Capacity>` impl'd for both `MatrixLike<F, C>` (whose field is the nested `C::Array<C::Array<F>>` 2-D shape) and `SparseLapLike` wrapping a `CsrLike`. The second cross-cutting trait.
6. The `Capacity::from_fn` extension the containers need to keep their `from_fn` API, used generically.

All compile and run. Outputs are recorded at the bottom of `src/main.rs`.

## The four questions

- **Q1 trait + GAT + two impls + generic consume.** Proven. The GAT (owned iterator for BitMatrix, borrowed for Csr) resolves under a `C: Capacity` type parameter.
- **Q2 default-method problem-trait + blanket impl.** Proven. `reduce_bandwidth() -> C::Array<NodeId>` type-checks as a default method and through the blanket impl; threaded through a generic `analyse::<C, _>` wrapper.
- **Q3 GCE-free.** Proven. The migration machinery compiles with no `generic_const_exprs`. No `cap_size` appears in any type position; scratch arrays come from `C::filled` / `C::from_fn`, value-position counts from `cap_size(C::CAP)`. The one feature gate present (`const_trait_impl`) is for the Q4 probe module alone.
- **Q4 keep `const` on the trait declaration.** Proven. A `const trait Foo<C: Capacity>` admits a `const` impl when the method body is const-clean (reads `C::CAP`, calls the `cap_size` const fn). A non-const-clean body (e.g. `Ord::min`, or `.into()` needing const_convert) is rejected. This does not bind the migration, because arvo's real `SparseAdjacency` / `BidirectionalSparseAdjacency` impls are PLAIN (non-const) impls carrying ordinary runtime bodies. The `pub const trait` DECLARATION survives the parameter-kind change untouched; impls stay plain.

## Decisions this settles for the round

- The migration is **mechanical parameter substitution**: `const N: Cap` -> `C: Capacity` on every declaration; `[T; cap_size(N)]` -> `C::Array<T>` in fields and returns; scratch `[v; cap_size(N)]` -> `C::filled(v)` and indexed builds -> `C::from_fn(..)`; value-position `cap_size(N)` -> `cap_size(C::CAP)`.
- `Capacity` gains a `from_fn<T, F: FnMut(USize) -> T>(f) -> Self::Array<T>` method (foundation extension, consistent with `filled`; impl for `Dim<N>` is `core::array::from_fn`).
- The `pub const trait` qualifier stays on `SparseAdjacency` / `BidirectionalSparseAdjacency`; impls remain plain. No const-ness change needed.
- After the migration, the algorithm crates likely no longer need `#![feature(generic_const_exprs)]` at all (no `cap_size` in type position, no `[(); cap_size(N)]:` bounds). Drop the gate per crate where genuinely unused; keep it only where some other GCE use remains. This is the bonus that obviates #628 for these crates.
- The migration is one atomic unit (the cross-cutting `SparseAdjacency` and `LinearOperator` traits couple BitMatrix, Csr, Matrix, graph, comb, and spectral into one connected component; no-legacy-shims forbids a transitional parallel definition). One mockspace round, one PR, green at lock.

## Out of scope (confirmed)

The `BitMatrix<W, const N: Cap>` `W` axis (per-row bit width, `Bits<64>` today, the firing-12 "64-node column wall") is a separate widening concern, not part of this Capacity migration. Only the row-count `N` migrates here.
