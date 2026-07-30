# Probes for panel file 08, the union and what it costs

Nine artifacts, all compiled, run, or swept under `nightly-2026-05-28`, the workspace pin. The Rust
files are crate bodies; the two Python files are generators for the compile-cost sweeps. Nothing
here uses a forbidden feature: `a_union.rs` and its siblings carry `#![feature(const_trait_impl)]`
only, which is WATCH-allowed per `unstable-features.md`.

To reproduce the union: `cargo init --lib union`, edition 2024, pin the toolchain, drop `a_union.rs`
in as `src/lib.rs`, `b_spare_pattern_decides_delivery.rs` as `src/spare.rs`,
`c_split_does_not_bind.rs` as `src/fusion.rs`, and the `e`/`f`/`g` files under `src/bin/`. The
`d_fused_parameter_control.rs` file is a separate crate, mechanically derived from `a_union.rs` by
fusing `Number<N, P, L>` back to `Number<N, S>`; it exists only to isolate what the parameter split
costs in the error surface.

| File | Question | Outcome |
|---|---|---|
| `a_union.rs` | do all five standing proposals compile together | WORKS, with three composition defects recorded in the panel file. Includes `Refuse`, which 07's probe A could not express |
| `b_spare_pattern_decides_delivery.rs` | does the absorbing-bottom delivery's byte saving survive the graded aggregate | ONLY under a carrier keyed on the numeral's spare pattern. 2 bytes where one exists, 4 where it does not |
| `c_split_does_not_bind.rs` | does 02 sec 5's parameter split make law-independence-from-lowering a typing fact | NO. A law conditioned on `L::Layout` compiles clean under split parameters |
| `d_fused_parameter_control.rs` | what does the split cost in the rendered error type | 1.8x length; one modifier level of legibility |
| `e_codegen.rs` | what the four deliveries emit at `-C opt-level=3` on aarch64 | 8 / 9 / 8+2-exits / 87 / 10 instruction loop bodies. No timing claimed |
| `f_error_surface.rs` | four deliberately wrong consumer cases under the union | three of four truncate and spill to a long-type file |
| `g_classification_table.rs` | what the const checker computes for all five resolutions | reproduces 01's whole table mechanically, including the two `Refuse` rows |
| `h_gen_table_vs_projection.py` | macro-expanded table against associated-const projection, swept over size | table coherence is quadratic, projection flat |
| `i_gen_monomorphisation_sweep.py` | what a distinct composition costs | about 5.2ms of collector walk each, and zero symbols in the binary |

Mechanism notes, recorded so they are not rediscovered.

The recovery map must be partial (`fn phi(..) -> Rec` where `Rec` is `At(i32) | Refused`) rather than
total. With a total map, `Refuse` has no `Resolve` impl and the whole fallibility half of the design
is outside the witness mechanism. With a partial map, the same const check computes translation
stability under Kleene equality *and* the fallibility grade, from one definition.

`const fn` generic over a `[const]` trait bound works, and a `const { .. }` block inside a generic
trait method is evaluated at monomorphisation, so the door check fires per composition. It names the
composition, not the misclassified rule, which is why the eager per-constructor const is still
needed.

The witness at the composition's actual width is bounded by
`error: constant evaluation is taking a long time` under `#[deny(long_running_const_eval)]`, which
fires at 9 bits. Cost quadruples per bit: 0.53s at 3 bits, 2.26s at 6, 8.65s at 7, 28.45s at 8.
