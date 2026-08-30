# 63_probes: the consolidation's own verification record

No new instrument was built for `63`. What this directory holds is the consolidation's re-run of
**every committed instrument in unit two**, performed before any of their counts were consolidated,
per the discipline `53` section 1 set and `RULES.md`'s evidence rules.

Toolchain `nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`), passed explicitly,
because a bare `rustc` outside the repository tree resolves to stable.

```
rustc +nightly-2026-05-28 -O --edition 2021 -o <tag> <source>.rs && ./<tag> > <tag>.rerun.txt 2>&1
diff <committed output> <tag>.rerun.txt
```

## rerun/

Thirty-two instruments, one rerun output each, every diff against the committed output **empty**:

- `55_probes/`: p1, p2, p3, p4, p5 (five)
- `56_probes/`: q1, q2, q3 (three)
- `57_probes/`: p1, p2, p2b, p3, p4, p5, p6, p7, p8, p9 (ten)
- `58_probes/`: p1, p2 (two)
- `60_probes/`: p_a, p_b, p_c1, p_d (four), plus p_c2 rebuilt as the compile-fail control:
  the committed `p_c2.stderr` and the rerun both carry exactly 4 instances of
  "generic parameters may not be used" (`60_pc2.stderr.rerun`)
- `61_probes/`: q1, q2 (two)
- `62_probes/`: p1, p2, p2b, p3, p4 (five)

The two `.build.err` files kept are dead-code warnings from `57_probes/p1` and `p4`, retained as the
honest build record; every other build was warning-free and its empty error file was dropped.

Binaries are not committed. The `.rerun.txt` files are byte-identical to the committed outputs they
sit beside, which is checkable with `diff` in one command per pair.
