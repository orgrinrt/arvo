# arvo

Numeric primitives + analysis algorithms with Hot/Warm/Cold/Precise strategy markers.

`#![no_std]`, no alloc, no platform deps. Monomorphisation is the dispatch. Designed to be consumed by pipeline engines (`hilavitkutin`), language toolchains (`clause`), and miscellaneous workspaces that need const-sized numeric substrate without pulling in `std`.

## Crates

| Crate | Layer | Purpose |
|-------|-------|---------|
| `arvo` | L0 | Numeric primitives, strategy markers, semantic aliases |
| `arvo-bits` | L1 | Bit-level contracts (BitWidth, BitAccess, BitSequence) |
| `arvo-bitmask` | L2 | Fixed-width masks on top of bit contracts |
| `arvo-sparse` | L2 | Sparse matrix (CSR, RCM, block diagonal) |
| `arvo-graph` | L2 | DAG algorithms (topo sort, rank, waist) |
| `arvo-comb` | L2 | Combinatorial optimisation (DP, greedy, bin-packing) |
| `arvo-spectral` | L3 | Spectral methods (Laplacian, Fiedler) |

## Status

**Design phase.** Crates scaffolded in `mock/` with authoritative DESIGN.md.tmpl files. No live implementations yet; next step is to implement L0 + L1 in the mockspace, validate, then graduate to root-level `crates/`.

Consumers that want to experiment against the shape should path-dep into `mock/crates/*` for now.

## Contributing

Design conversations happen in `mock/design_rounds/`. Implementation PRs land in `mock/crates/` against the mockspace validation gate (`cargo check` + `cargo mock`).
