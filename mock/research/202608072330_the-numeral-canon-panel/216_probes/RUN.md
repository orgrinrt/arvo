# How to run seat 216's probes

Every probe is a single-file Rust program with no dependencies, built with the
repository's pinned toolchain and run from this directory.

```sh
mkdir -p .build
rustc -O --edition 2021 -o .build/p1 p1_accumulator_width_independent_sweep.rs
./.build/p1 40000000 > p1.out 2>&1

rustc -O --edition 2021 -o .build/p2 p2_closing_the_unreached_cells.rs
./.build/p2 > p2.out 2>&1

rustc -O --edition 2021 -o .build/p3 p3_the_one_bit_constant_breaks_at_fold_length_nine.rs
./.build/p3 > p3.out 2>&1          # add --l10 for the 2^30 cell, which takes minutes
```

`p1`'s argument is the per-cell tuple budget. A cell above it is reported as
unreached rather than estimated.

Do not pipe a run into `head`. The first p1 run here was truncated at 79 lines
because `head` closed the pipe and the program took SIGPIPE, and the artifact
looked like a completed run that had stopped reporting. The exit status was 0
because it was the status of the last command in the pipeline.

`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, edition 2021, opt level 3.
