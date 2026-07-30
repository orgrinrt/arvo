# Probes for panel file 05

Six probes written for `05_leijen_fallibility_without_poisoning.md`, all compiled and run under
`nightly-2026-05-28`, the workspace pin. Each is named for the section that cites it.

They were built in a scratch crate outside the repo (edition 2024) with a path dependency on
`notko` (`default-features = false, features = ["const"]`), and probe E additionally with path
dependencies on `arvo`, `arvo-graph`, `arvo-bitmask`, `arvo-tensor`, `arvo-bits-contracts`,
`arvo-numeric-contracts` and `arvo-storage`. Probes A, B and E are binaries; C is a binary that
prints a size table; D and F are library files.

| File | Question | Outcome |
|---|---|---|
| `a_handler.rs` | can one generic arithmetic body serve total and fallible compositions | WORKS. `A: total=1000 refusing_err=true refusing_ok=true` |
| `b_carrier_join.rs` | with two range positions that may differ, does the carrier join and the lift resolve | WORKS. `B: sat=100 precise_err=true mixed_hi_err=true mixed_lo=true` |
| `c_layout.rs` | what does a fallible return cost in bytes | Doubles every intermediate unless the validity range is known to rustc, in which case it is free. Table in the file header. |
| `d_delivery_codegen.rs` | what does each delivery cost in emitted instructions | Loop bodies transcribed in the file header, read from `--emit asm -C opt-level=3` on aarch64. No timing taken. |
| `e_refusing_through_graph.rs` | does a refusing policy reach `arvo_graph::upward_rank` when delivered as an absorbing bottom | WORKS, unmodified algorithm crate. Output in the file. |
| `f_const_falsification.rs` | does 03's proposed bounded const-eval check compile and bite | WORKS, six checks. Flipping one polarity gives `error[E0080]: evaluation panicked`. |

Two mechanism notes recorded here so they are not rediscovered. A `const fn` cannot call through a
`fn` pointer, so probe F's oracle has to be instantiated by macro rather than passed as a parameter.
And probe E needs `#![feature(const_trait_impl, adt_const_params)]` at its own crate root to
implement `TotalOrd` and `FromConstant`, both of which are allowed per `unstable-features.md`.
