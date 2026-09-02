# 47_probes

Eleven files, six that must compile and run, five that must fail to compile. `./verify.sh`
rebuilds and reruns the lot on `nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f
2026-05-27)`) and reports each refusal's error count.

Zero `#![feature]` gates in any file. `grep -c '^#!\[feature' ./*.rs` returns 0 for all eleven,
and `verify.sh` prints that at the end.

| File | Expected | What it checks |
|---|---|---|
| `p1_single_type_output.rs` | compiles | the one-richer-output form built: one associated TYPE, every layout fact a projection; the eight `Cold` widths 9..=16 that share a carrier get eight distinct single outputs |
| `p1b_negctl_distinctness.rs` | REFUSED, 3 | that p1's `SameType` bridge is not vacuous: three false type equalities, all refused |
| `p2_scalar_single_output_refused.rs` | REFUSED, 6 | the one-richer-output taken as a SCALAR: a lossless const encoding cannot yield the carrier type, in three syntactic positions, naming the forbidden `generic_const_exprs` |
| `p2b_kind_asymmetry_positive.rs` | compiles | the other direction: a type-valued output yields every const generically, and yields types too, all gate-free |
| `p3_access_type_from_const_refused.rs` | REFUSED, 3 | 16's closed form for the packed access width cannot reach a TYPE from a const-carried width, in three positions |
| `p3b_access_type_two_routes_that_work.rs` | compiles | the two routes that do reach it, and which of the two is closed by the compiler and which by the design's no-enumeration ruling |
| `p4_stating_injectivity_needs_one_subject.rs` | compiles | stating the injectivity property needs one subject; a componentwise assertion is satisfied by the collapse it is meant to catch |
| `p4b_negctl_joint_distinctness.rs` | REFUSED, 2 | the joint assertion refuses the collapse; the carrier-only assertion on the same pair does not, and its absence from the error list is half the result |
| `p5_one_output_against_all_three_forcings.rs` | compiles | the single output against `Cold` packing, alignment divergence, and `Precise` widening at once |
| `p5b_negctl_forcings.rs` | REFUSED, 2 | two must-refuse claims and one must-not-refuse claim, the last confirming the flat pair collapses `Warm` and `Precise` |
| `p6_two_ladders_not_one.rs` | compiles | the native rung partition and the access rung partition of widths 1..=128 share no jump point, so one ladder cannot key both |

Cross-check worth recording: `p6`'s `access_bytes` reproduces `16`'s reported "28 of 64" figure
exactly over widths 1..64, from a formula transcribed from `16:187` and independently rounded to
a power of two. That is one instance of agreement between two codings of the same closed form,
not two independent derivations of it.
