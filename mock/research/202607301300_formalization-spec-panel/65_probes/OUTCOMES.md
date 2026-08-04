# Probe outcomes, file 65 (pricing the L0 migration)

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`,
resolved from the repository's `rust-toolchain.toml`. Every probe here was compiled from
inside the repository tree so the pin resolves; a bare `rustc` outside it picks up stable
and reports `E0554` on every feature gate rather than the real answer.

Nothing under `mock/crates/` was edited. The whole-crate work in `migration/` was done on
a copy of `mock/` outside the tree, at
`/private/tmp/.../scratchpad/arvo-copy/mock`, and the diff is committed here as an
artifact rather than applied.

| probe | question | outcome |
|---|---|---|
| `probe_1_mgca_refusal.rs` | does `min_generic_const_args` admit the shipped `UFixed` shape | REFUSES, `complex const arguments must be placed inside of a const block`, 2 sites |
| `probe_1b_mgca_constblock.rs` | does the const-block escape it names work | REFUSES, escalates to needing `generic_const_args`, 4 sites |
| `probe_2_const_to_type_escape.rs` | can the width become a type while `I`/`F` stay consts, via a const-keyed projection | REFUSES, 6 sites. The addition is what sits in const position, so wrapping the result in a type does not move it |
| `probe_2b_standalone_and_peel.rs` | the recursive-peel route | REFUSES twice: the const operation, and `E0119` coherence, which only full `specialization` could break and that is forbidden |
| `probe_3_target_shape.rs` | does the tower shape (`I`, `F` as `Nat`s, width a type-level sum) compile | WORKS, zero feature gates |
| `probe_4_bucket_machinery.rs` | the container bucket derived structurally rather than by const fn, checked against the shipped const fn | WORKS, 13 boundary points, zero gates. Decomposes into predecessor, bit length and a clamped subtract: three type-level functions the const fn got for free |
| `probe_5_struct_512.rs` | the same law over EVERY width 1..=512 rather than a sample | WORKS, 512/512, and a negative control (one boundary moved by one) fires at `W9`, so the law is not vacuous |
| `probe_6_reparameterise_on_total_width.rs` | third route: `UFixed<const W, const F>` so nothing is ever computed in type position | WORKS, only `adt_const_params`, which is already ALLOWED |
| `probe_7_strategy_only_dropin.rs` | can `arvo-strategy`'s gate come off with the public signature unchanged | WORKS as a mechanism (the residual refusals are widths absent from the 10-row demo table, and they print rustc's own "the following other types implement" listing) |
| `probe_8_width_table_*.rs` | what a per-width impl table costs as it grows | Quadratic. See `timings.csv` |
| `probe_9_predicates_under_each_route.rs` | the four const-fn predicates under the two candidate parameterisations | WORKS. Under the tower shape each is one impl, and the negative control (`Z: OneRepresentable` absent) refuses with a clean diagnostic |

## Timings

`timings.csv`: the per-width impl table (the 2026-07-28 sketch's shape). Two runs each, wall
clock, `rustc --edition 2024 --crate-type=lib`. Ratio per doubling is 2.7, 3.4, 3.8, 4.5,
4.9, so the growth is quadratic or slightly worse, and the cost is paid for the table's
ceiling whether or not a consumer instantiates those widths.

`timings_structural.csv`: the structural derivation, 30 impls total, timed by the number of
distinct widths INSTANTIATED rather than tabulated. Linear, roughly 0.4ms per distinct
width, and zero for widths nobody uses.

The two columns do not measure the same variable, and that is the point.

## `migration/`

`container_migrated.rs` is `arvo-strategy/src/container.rs` rewritten to the bucket-as-type
shape with a table to 256. With it in place and `#![feature(generic_const_exprs)]` removed
from `arvo-strategy/src/lib.rs`, `cargo check --offline --workspace --all-targets` is clean
and `cargo test --offline --workspace` reports 658 passed, 0 failed, 9 ignored, matching the
consolidation's own baseline exactly.

`instrumented_migration.diff` is the further, incomplete facade work. It is INSTRUMENTED, not
a proposal: it relaxes the `OneRepresentable` guard (which reinstates the `UFixed<0, F>::ONE`
defect on purpose, to isolate what else was blocking) and introduces a stand-in fraction
carrier in arvo-strategy. Read it for the shape and the counts, never as a migration step.

## Reproducing, and the expected error counts

Run each from inside the repository tree so the pin resolves:

```
mkdir -p out && rustc --edition 2024 --crate-type=lib --out-dir out <probe>.rs
```

Expected, including rustc's own "aborting due to N previous errors" summary line:

| probe | errors | why |
|---|---:|---|
| `probe_1_mgca_refusal` | 3 | 2 refusals plus the summary. The refusal IS the result |
| `probe_1b_mgca_constblock` | 5 | 4 refusals plus the summary |
| `probe_2_const_to_type_escape` | 7 | 6 refusals plus the summary |
| `probe_2b_standalone_and_peel` | 3 | 1 const-op refusal, 1 `E0119`, plus the summary |
| `probe_3_target_shape` | 0 | |
| `probe_4_bucket_machinery` | 0 | |
| `probe_5_struct_512` | 0 | the law holds over all 512 widths |
| `probe_6_reparameterise_on_total_width` | 0 | |
| `probe_7_strategy_only_dropin` | 5 | the demo table has 10 rows and the const checks name widths outside it. The mechanism is what this probe establishes; `probe_8_width_table_256` is the same shape with a table that covers its own checks and is clean |
| `probe_8_width_table_128` | 2 | its own wide-bucket check names `Width(129)`, past a 128 ceiling. This is the ceiling behaving as designed, not a defect |
| `probe_8_width_table_256` | 0 | |
| `probe_9_predicates_under_each_route` | 0 | the negative control lives in the file's closing comment and is described there, not asserted, because asserting it would not compile |

The generated probes (`probe_5_struct_*`, `probe_8_width_table_*`) at ceilings above 256 are not
committed; regenerate with `gen_exhaustive.py <N>` and `gen_width_table.py <N>`. `time_curve.sh`
reproduces `timings.csv` and takes over half an hour past the 4096 point.
