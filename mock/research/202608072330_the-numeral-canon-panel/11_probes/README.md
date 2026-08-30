# `11_probes`

Sources and emitted output for `11_chlipala_prior_art_on_typed_widths.md`. Every claim in that file
that names a probe names one of these, and every compile below is reproducible with the command in its
own header comment.

Toolchains: `rustc +nightly-2026-05-28` reporting `1.98.0-nightly (57d06900f 2026-05-27)`, edition
2024, aarch64-apple-darwin. Homebrew clang 22.1.8. Zig 0.16.0.

## The comparators, where the survey is compiled rather than recalled

| File | Question | Result |
|---|---|---|
| `a01_cpp_nttp_derivation.cpp` | does C++ need a bridge | no. Arithmetic on a non-type template parameter in type position. Six layout asserts pass, erasure holds |
| `a02_zig_comptime_derivation.zig` | does Zig need a bridge | no. Types are comptime values. `arvo16` and `native16` are the same symbol |

## The bridge as a ceiling on the algebra

Each is `10_probes/p12_improved_full.rs` copied unmodified, with the addition marked in the file.
`ladder.rs` is copied from `10_probes/` because `p12` includes it.

| File | Question | Result |
|---|---|---|
| `b01_table_caps_the_algebra.rs` | does the table cap `mul` | yes. `arvo does not ship this width: Idx<48>`, from two inputs that are both in the table |
| `b02_the_table_chases_its_tail.rs` | does adding the row fix it | no. The row closes 48 and the next octave fails at 96 and 32 |
| `b03_the_ceiling_is_the_const_surface.rs` | whose ceiling is it | the const surface's. Same ladder keyed on nat types: three octaves and a 1636-bit numeral compile with no bridge |

## Coherence, across a real crate boundary

In `c_orphan/`. Two crates, so the orphan rule is actually in play.

| File | Question | Result |
|---|---|---|
| `arvo_min.rs` | the library side | compiles to `libarvo_min.rlib` |
| `consumer_ok.rs` | can a downstream crate add a width | yes, against a local marker |
| `consumer_bad.rs` | against arvo's marker | `E0117`, structurally impossible |
| `consumer_partition.rs` | do two markers compose | no. `Fixed<13,0,LibA>` does not flow into `Fixed<13,0,Arvo>` |
| `consumer_partition_nored.rs` | does a marker inherit arvo's rows | no. Each marker starts an empty table |

## Route 15, the bare-parameter carrier

| File | Question | Result |
|---|---|---|
| `d01_bare_parameter_carrier.rs` | can a numeral be built with no bridge at all | yes, at arity three. Erases to a symbol alias and vectorises identically |
| `d02_postmono_check_fires.rs` | does the post-mono validation fire, and how does it read | `E0080` with a custom message naming the instantiation |

## Where the wall actually is

| File | Question | Result |
|---|---|---|
| `e01_enumeration_free_bridge.rs` | is a table-free bridge writable | **yes, in one blanket impl.** Its codomain overshoots by exactly eight |
| `e02_closing_the_overshoot.rs` | close it, no features | refused, names `generic_const_exprs` |
| `e03_overshoot_under_min_gca.rs` | under `min_generic_const_args` | refused, wants a const block |
| `e04_overshoot_const_block.rs` | following that suggestion | refused, names `generic_const_args` |
| `f01_const_param_default_from_siblings.rs` | can a const default use its siblings | refused, names `generic_const_exprs` |

`e02` through `f01` are four independent syntactic positions beyond `10`'s three, all terminating on a
forbidden feature.

## Reproducing all of it in one command

`./verify.sh` from this directory recompiles every source and prints one line each. Its output is
committed as `out/verify.txt`, so a later reader compares against a record rather than rerunning to
find out whether they agree. Seventeen rows: seven compile, ten refuse, and every refusal is a result
rather than a failure.

## `out/`

Emitted metadata, assembly, object files and captured diagnostics. `*.log` is the captured stderr of
the compile named in the corresponding source header. `d01.s` is the assembly the codegen claims in
section 10.2 are read from, which is an ad-hoc quick spike and not a bench.
