# Probe outcomes, file 76 (the real consumer price)

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`,
resolved from the repository's `rust-toolchain.toml`. Every probe and every timing was
run from inside the repository tree so the pin resolves. This matters and it bit this
session: the first version of `time_arm.sh` wrote its generated source to `mktemp -t`,
which lands in `/var/folders/...`, outside the tree, where `rustc` resolves to stable
`1.94.0` and refuses every feature gate. The harness now writes into `76_probes/gen/`.

Nothing under `mock/crates/` was edited or created.

## Part A: can the ratified unification be expressed at all

`74b` adopts one sealed bottom carrier shared by the tower's naturals and the capacity
domain, `Capacity` kept as a semantic alias over it. A capacity's job is to name the
backing array for a count, so the alias owes a `type Array<T>` that is a real `[T; N]`.

| probe | question | outcome |
|---|---|---|
| `a1_naive_unification.rs` | `type Array<T> = [T; <N as Nat>::VAL]` | REFUSES. `error: generic parameters may not be used in const operations`, and rustc's own help names `generic_const_exprs`, which `unstable-features.md` forbids outright (op, 2026-07-28) |
| `a2_escapes_refused.rs` | the const-block form, and a `min_generic_const_args` `type const` | REFUSES, 2 sites. `error: use of const in the type system not defined as type const` |
| `a3_type_const_all_the_way.rs` | the whole chain as `type const`, following rustc's own suggestion | REFUSES. `error: complex const arguments must be placed inside of a const block`, on `type const VAL: usize = 2 * P::VAL` |
| `a3b_type_const_constblock.rs` | the same with the suggested const block | REFUSES identically. `min_generic_const_args` cannot express the inductive step, so the successor feature does not rescue it |

Expected error counts, reproduced fresh at the end of the session, each including
rustc's own "aborting due to N previous errors" summary line:

| probe | errors | probe | errors |
|---|---:|---|---:|
| `a1_naive_unification` | 2 | `b1_structural_array` | 0 |
| `a2_escapes_refused` | 3 | `b1b_layout_law_negative_control` | 5 |
| `a3_type_const_all_the_way` | 2 | `b1c_perimeter_control` | 2 |
| `a3b_type_const_constblock` | 2 | `b2_split_by_layer` | 0 |
| `c_ys_attack` | 2 | `b2b_disagreement_refused` | 2 |
| `c_ys_attack_1_only` (link) | 2 | `d_consumer` against `d_lib` | 2 |
| `c_ys_attack_1_only` (metadata) | **0** | | |

The last row is the finding of Part C, not a passing probe.

## Part B: two constructions that do express it

| probe | question | outcome |
|---|---|---|
| `b1_structural_array.rs` | derive the array structurally from the binary encoding, `repr(C)` | WORKS, zero feature gates. Layout law asserted over 8 capacities (0, 1, 5, 7, 13, 28, 64, 4096) at 3 element types (`u8`, `u32`, `u128`), 24 compile-time assertions. The cast's precondition is discharged inside `as_slice` by an inline const block, so it is evaluated per monomorphisation rather than per remembered list |
| `b1b_layout_law_negative_control.rs` | one byte of padding in the odd-arity node | REFUSES at exactly 4 of the 8 capacities, and exactly the 4 whose encodings contain `I`. The law discriminates per constructor rather than firing indiscriminately |
| `b1c_perimeter_control.rs` | the same corruption, reached through a capacity no assertion list names | REFUSES at the door, `evaluation of <Pz<I<I<O<H>>>> as Capacity>::as_slice::<u32>::{constant#0} failed here`. The discharge is not sampled |
| `b2_split_by_layer.rs` | count on the shared carrier, array grammar on the lowering side, agreement checked at the door | WORKS, zero feature gates |
| `b2b_disagreement_refused.rs` | a `Slot` whose declared length lies about its carrier's value | REFUSES, `capacity's declared length disagrees with its value` |

## Part C and D: whether route Y can express the predicate

| probe | question | outcome |
|---|---|---|
| `c_ys_attack.rs` | forge the staged one-witness, two ways | Attack 2 (a downstream marker offered as a witness) REFUSED by the seal, `MyOwnYes: Sealed is not satisfied`. Attack 1 needs isolating because attack 2 aborts the build |
| `c_ys_attack_1_only.rs` | `Num<0, 8, OneYes, Hot>`, the forged affirmative witness | REFUSED under `--emit=link`. **NOT refused under `--emit=metadata`**, which is what `cargo check` pays. The guarantee exists only in codegen |
| `d_lib.rs` + `d_consumer.rs` | the consumer emits the predicate impl for its own declared numerals | REFUSES, `E0117`, orphan rule: the impl has no local type before any uncovered type parameters. rustc's own note says "define and implement a trait or new type instead", which is route Z |

## Timings

All `--edition 2024 --crate-type=lib`, hyperfine with 2 warmups. `metadata` is what a
consumer's `cargo check` pays. `results.csv`, `split_results.csv`, `deep_results.csv`.

**Count sweep**, n distinct numerals and n distinct capacities, 5 runs, mean ms:

| n | base | ys | zs | z |
|---:|---:|---:|---:|---:|
| 14 | 54 | 58 | 71 | 70 |
| 50 | 56 | 65 | 94 | 96 |
| 100 | 69 | 82 | 147 | 144 |
| 200 | 89 | 124 | 237 | 238 |
| 400 | 125 | 181 | 443 | 438 |
| 800 | 208 | 311 | 890 | 873 |

Doubling ratios for arm z from n=50: 1.50, 1.65, 1.84, 1.99. Converging to exactly
linear, with the sub-linear behaviour at small n being fixed overhead.

**Consumer-only**, machinery precompiled as a dependency rlib and only the consumer
crate timed (`split_bench.sh`): 66, 107, 151, 248, 459 for arm z at the same n. Within
noise of the whole-file figures, so **the carrier's cost does not amortise into the
dependency**. It is paid per consumer crate, because a type-level natural is
instantiated at the naming site.

**Route Y's impl table**, 14 numerals, ceiling sweep. The ceiling is the widths the
table covers, and it is not free to choose: at 16 the build FAILS with 3 errors (widths
27, 28 and 64 absent), and at 32 and 48 with 1 error each (width 64 absent), so 64 is
the floor the census forces, and `arvo-toolbox-not-policer.md` forbids a cap below what the substrate
dispatches, which is past 128.

| ceiling | impls | mean ms |
|---:|---:|---:|
| 64 | 2080 | 267 |
| 96 | 4656 | 840 |
| 128 | 8256 | 2190 |
| 160 | 12880 | 4988 |
| 192 | 18528 | 9780 |
| 256 | 32896 | 30031 |

Exponent in the impl count climbs from 1.42 to 1.86 across the sweep. The 256 point was
measured, not extrapolated, after an extrapolation predicted 29s.

**Capacity magnitude sweep** (`gen_deep.py`, 32 distinct capacities each, values near
`2**mag`, each pinned to its own value by a const assertion so no fold can be skipped):

| mag | 4 | 8 | 12 | 16 | 20 | 24 | 28 | 32 | 40 | 48 | 56 | 62 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| ms | 69 | 67 | 63 | 63 | 65 | 62 | 70 | 67 | 71 | 74 | 75 | 72 |

Flat. Negative control (`gen/deep_nc.rs`, one pinned value corrupted) refuses, so the
62-deep chains are genuinely being evaluated.

**Combined stress**: 400 numerals and 400 capacities near 2^16 together, 462ms against
438ms for the same counts at census-sized capacities. The two axes do not interact.

**Flags.** Arm z at n=400, metadata: default solver 438ms, `-Znext-solver` 415ms,
`--emit=link` 872ms against base 489ms. Route Y at ceiling 128: default 2204ms,
`-Znext-solver` 2626ms, `-Zthreads=1` 2217ms. Every result survives every flag change
tried, and route Y is slightly worse under the next-generation solver rather than better.

**Denominator.** `cargo check --offline --workspace --all-targets` in `arvo/mock`, after
touching the facade crate root, 6.35s +- 0.09 over 3 runs.

## Positive controls

The arms are only measuring work if the work happens. Both `z` arms pin every computed
width to the value the generator knows independently
(`const _: () = assert!(W{k} == {i+f});`, 14 of them at the real profile). `gen/nc_z.rs`
is the negative control: corrupting one expectation refuses with
`evaluation panicked: assertion failed: W3 == 5`. The capacity sweep pins every value
the same way.

## Reproducing

From inside the repository tree:

```
rustc --edition 2024 --crate-type=lib --out-dir out <probe>.rs   # parts A, B, C, D
./bench.sh <base|ys|zs|z|y> <numerals> <capacities> [ceiling] [metadata|link] [runs]
./split_bench.sh <arm> <numerals> <capacities> [ceiling] [runs]
python3 gen_deep.py <count> <magnitude>
```

Generated sources above 120KB are not committed (the route-Y tables past ceiling 64, and
the n=400 and n=800 arms). Regenerate with `gen_consumer.py`. The ceiling-256 table is
1.6MB of source and takes about two minutes to time.
