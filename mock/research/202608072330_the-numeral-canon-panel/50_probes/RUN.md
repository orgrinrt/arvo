# 50_probes

Seven probes for file `50`, plus one committed compiler refusal. `./verify.sh` rebuilds and reruns
everything from source and diffs each result against the committed output.

Toolchain `nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, Python 3.14.6.
Zero `#![feature]` gates in any `.rs` file, which `verify.sh` prints per file.

| file | kind | what it establishes |
|---|---|---|
| `p1_criterion_fixpoints.py` | exhaustive enumeration | `16:100-101` is a fixpoint equation with a non-monotone operator. Over 16 cells of three parameters it never names, it admits output sets of size 0, 2, 3 and 4, and the two sets of size two are different sets. |
| `p2_reading_a_loses_the_width.rs` | exhaustive count, compiled | Under the reading where a site holds only the derivation's outputs, the declared width is unrecoverable for 389 of 512 declarations, so a fact the machine needs is excluded by clause one and unreachable. |
| `p3_site_recomputes_the_stride.rs` | compiled, const-asserted | A one-output derivation with the site recomputing the stride compiles gate-free, so the collapse `48` reports is real. Adding one plausible fifth strategy makes the site's answer silently wrong, and the repair relocates the fact onto the strategy rather than removing it. A sixth strategy breaks the repair. |
| `p4_access_width_is_keyed_on_the_stride.py` | exhaustive, brute-forced | `16:187`'s closed form is exactly the worst case over all eight bit phases (0 mismatches, W = 1..1024) and over-estimates the phases a packed run actually reaches at 48 of 128 widths. The exact access width is a function of `(W, stride)`. Recomputed, `16`'s 28-of-64 becomes 16-of-64 and `47_probes/p6`'s zero shared ladder jump points becomes all four. |
| `p5_three_facts_two_slots.rs` | compiled, with a must-not-refuse control | `48:227-229`'s modelling-independent restatement of `47` section 4 holds, and its stated reason does not: under one assignment the pair does separate `Warm` from `Precise`, and what is missing is a type rather than information. |
| `p5b_negctl_three_facts.rs` | **expected refusal**, `.err` committed | The non-vacuity control (`u16: SameType<u32>` refused) plus three `generic_const_exprs` refusals reaching a type from a stride const, from a third starting point after `16_probes/p5b` and `47_probes/p2`, `p3`. |
| `p6_precise_fork_is_not_a_fork.py` | exhaustive, two instruments cross-checked | Per-step refusal on inexact and end-of-chain refusal admit exactly the same chains once zero operands are excluded, with a proof of why. Per-step refusal admits 1.18% of single multiplies at F=8 and 0.000018% of triples. |
| `p7_bench_readout.py` | readout of committed harness output, runs nothing | The magnitude `47:536-541` and `48:504-521` call unpriced is priced in `mock/benches/bitpack-decoder-shape`: a runtime-derived access plan costs 3.04x to 3.12x a compile-time one, across four sizes, with the dense carrier as a competitor arm. |

## Reading them honestly

`p1`, `p3`, `p4` and `p6` are models. Their inputs are stated in each file's header and a reader who
rejects an input should reject the conclusion. `p1` in particular is a model of one sentence, not of
arvo, and it is built to show how far that sentence's answer moves under parameters it never names.

`p2`, `p3`, `p5` and `p5b` are compiled. Every layout and equality claim in them is a
`const _: () = assert!(...)` or a trait-bound refusal, so the compiler checked it and the binaries
print only what was already proved.

`p6` carries two independently coded instruments (a brute force over every chain, and a dynamic
program over 2-adic valuations) that agree on every cell the brute force can afford, plus a proof of
the equivalence the DP reports. `p4` checks its closed form against a brute force over every phase.
Everything else in this directory is one instance.
