# Probes for panel file 66, the transfer argument

Five artifacts. Everything was compiled and run on the workspace pin,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`,
resolved from `rust-toolchain.toml`, invoked from inside the repo tree. No file
under `mock/crates/` was touched. No timing claim about runtime is made anywhere
here; the only wall-clock numbers are compile times, and they are labelled as
machine-shaped where they are quoted.

`model.rs` is a fresh implementation of the `Ranged` numeral exactly as
`50_fog_the_float_model.md` section 1 states it, written for this file so its
results do not inherit that file's code. No float appears in it. Every value is
an exact pair `(m, q)` denoting `m * r^q` over `i128`, every rescale is checked
and panics rather than wrapping, and every comparison and sum is exact.

| File | Question | Outcome |
|---|---|---|
| `gen_exhaustion.sh` + `heavy9.rs` (`OUT_probe_0.txt`) | is the const-eval refusal a fact about the WIDTH or about the STEP BUDGET | step budget. A cheap single sweep reaches ten bits; the same nine-bit quantification with a heavier body refuses. The quadrupling per bit reproduces and is structural |
| `probe_1_exponent_shift_symmetry.rs` (`OUT_probe_1.txt`) | does the quantiser commute with a shift of the exponent window | YES. 509,660,160 checks, zero failures, across radices 2 and 10, precisions 2 and 3, spans 1 to 4, both underflow policies, shifts -3 to +5. Two negative controls both disagree, so the check is not vacuous |
| `probe_2_span_saturation_flip.rs` (`OUT_probe_2.txt`) | is there a property of a `Ranged` numeral whose truth value moves along the exponent-span axis alone | YES. Absorption-freedom is TRUE at span p and FALSE at span p+1 under `Abrupt`, and FALSE from span 2 under `Gradual`. Exhaustive over the whole value set at every cell |
| `probe_3_radix_is_not_uniform.rs` (`OUT_probe_3.txt`) | re-derive `63:220-223` from the quantiser rather than from the formula | CONFIRMED. Every even radix in 2..=13 reaches an exact tie, every odd one reaches none, with rounding occurring in both cases so the odd rows are not vacuous |
| `probe_4_abrupt_under_unnormalised.rs` (`OUT_probe_4.txt`) | does `decode` land inside the value set under each (normalisation, `Underflow`) pair | Exactly one cell leaks: `Abrupt` with an unnormalised significand. At r=10, p=3 that is 108 of 2997 data decoding into the hole. All three other cells are clean |

## Reproduction

```
rustc --edition 2024 -O -o /tmp/p1 probe_1_exponent_shift_symmetry.rs && /tmp/p1
rustc --edition 2024 -O -o /tmp/p2 probe_2_span_saturation_flip.rs && /tmp/p2
rustc --edition 2024 -O -o /tmp/p3 probe_3_radix_is_not_uniform.rs && /tmp/p3
rustc --edition 2024 -O -o /tmp/p4 probe_4_abrupt_under_unnormalised.rs && /tmp/p4

./gen_exhaustion.sh 9 > /tmp/sw_9.rs
rustc --edition 2024 --crate-type=lib --out-dir /tmp/swout /tmp/sw_9.rs   # accepted
rustc --edition 2024 --crate-type=lib --out-dir /tmp/swout heavy9.rs      # refused
```

Probes 1 through 4 each end in assertions rather than in printed tables, so a
result that stops holding fails loudly rather than being read off a table by a
later reader. Each carries at least one negative control that must disagree; a
probe whose control agreed would be reporting that its own check discriminates
nothing.
