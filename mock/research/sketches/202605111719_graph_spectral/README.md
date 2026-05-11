# Sketches: arvo-sparse + arvo-spectral CSR-driven algorithms

**Date**: 2026-05-11T17:19Z
**Round**: 202605111719
**Status**: S1 compile-probed (PASSES). S2 through S4 documented for SRC-phase verification.
**Tracks**: task #431 (arvo cross-repo: ship graph + spectral algos hilavitkutin needs).

## What this directory exists to validate

The round 202605111719 topic file commits to four trait-shaped abstractions:

1. `SparseAdjacency<N>` sealed `pub const trait` with associated lifetime-generic iterator type for successors / predecessors.
2. Generic algorithm bodies over `T: SparseAdjacency<N>` (block-diagonal as the canonical exemplar).
3. `LinearOperator<F, N>` sealed trait with `SparseLaplacian<T, W>` impl driving Fiedler / power iteration without dense Matrix materialisation.
4. Wider return shape `(USize, [USize; cap_size(N)])` for Dulmage-Mendelsohn and spectral bipartition, replacing the 64-cap `Mask<...>` returns.

Per the workspace `cl-claim-sketch-discipline.md` rule, these get probed before the doc CL locks. Items with high rustc-novelty get live compile probes; items relying on patterns already in arvo are documented as design probes that the SRC phase verifies by construction.

## Sketches

| Sketch | Outcome | Findings |
|---|---|---|
| `01_sparse_adjacency_trait.rs` | **WORKS** | `pub const trait SparseAdjacency` with `type Successors<'a>: Iterator<Item = NodeId> where Self: 'a` compiles on HEAD nightly. Two concrete impls (`Csr<'data>` slice-iterator, `BitAdj<'data>` `trailing_zeros` iterator) drive a generic-over-`T` consumer (`total_edges`) and both produce the expected edge count. Compile invocation: `rustc +nightly --edition 2024 -Z next-solver=globally 01_sparse_adjacency_trait.rs`. Required additional feature: `#![feature(const_index)]` because the body uses slice indexing inside the const trait. |
| `02_generic_block_diagonal_body.md` | DESIGN-PROBE-DURING-SRC | Generic-over-`T: SparseAdjacency<N>` body for `block_diagonal`. Probed in SRC phase by replacing one existing algorithm body with the generic form and confirming `cargo check --workspace` green at every consumer site. Lower novelty: arvo-sparse already uses generic-over-numeric-trait bodies for CSR algorithms. |
| `03_linear_operator_sparse_laplacian.md` | DESIGN-PROBE-DURING-SRC | `LinearOperator<F, N>` sealed trait + `SparseLaplacian<T, W>` impl. Probed in SRC phase by replacing `power_iteration` and `fiedler_vector` parameter types and confirming the existing dense `Matrix<W, N>` impl of `LinearOperator` keeps the existing call sites green. Lower novelty: plain trait + impl, no const-trait machinery. |
| `04_wider_return_shape.md` | DESIGN-PROBE-DURING-SRC | Replace `Mask<Bits<64, ...>>` return types in DM and `spectral_bisection` with `(USize, [USize; cap_size(N)])`. Probed in SRC phase by changing the function signatures and migrating the test files in the same commit. Pattern already used by `block_diagonal`; risk is in the migration not the shape. |

## Decision: live probe S1, design probe S2 through S4

Standard discipline: every trait-solver / const-generic / lifetime-tied shape that has not appeared elsewhere in the codebase gets a live compile probe. Shapes that compose patterns already shipped in arvo are documented as design probes and verified during SRC. The S1 GAT-in-const-trait shape is the only genuinely novel pattern in this round; the rest extend established arvo patterns.

If S1 fails: fall back to non-GAT iterator (consumer-allocates-scratch shape) or split the trait into successors-only / predecessors-only halves. Resolution path captured in the topic file Subtopic A.

If any of S2 through S4 fail during SRC: the failure indicates a real design problem with the proposed shape (not a rustc gotcha caught by a sketch); pause and redesign the relevant subtopic. The round's SRC CL captures the deviation per `cl-claim-sketch-discipline.md`.

## Compilation command for S1

```bash
rustc --edition 2024 \
    -Z unstable-options \
    -Z next-solver=globally \
    01_sparse_adjacency_trait.rs \
    -o /tmp/sketch_s1 && /tmp/sketch_s1
```

## Cross-references

- `mock/design_rounds/202605111719_topic.graph-spectral-for-hilavitkutin.md` — round topic (subtopics A through F).
- `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md` — sketch discipline rule.
- `~/Dev/clause-dev/.claude/rules/arvo-toolbox-not-policer.md` — frames the trait-shaped surface as tools-not-policy.
