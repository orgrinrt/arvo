# Build and run record, file 61

Toolchain pinned explicitly throughout: `nightly-2026-05-28`
(`rustc 1.98.0-nightly (57d06900f 2026-05-27)`).

## Re-runs, before anything was argued with

```
rustc +nightly-2026-05-28 -O --edition 2021 -o rerun/q1 ../56_probes/q1_two_law_families.rs
./rerun/q1 > rerun/q1_output.txt
diff rerun/q1_output.txt ../56_probes/q1_output.txt   # empty, byte-identical

rustc +nightly-2026-05-28 -O --edition 2021 -o rerun/p2 ../57_probes/p2_absorption_necessity_sweep.rs
./rerun/p2 > rerun/p2_output.txt
diff rerun/p2_output.txt ../57_probes/p2_output.txt   # empty, byte-identical

rustc +nightly-2026-05-28 -O --edition 2021 -o rerun/p4_55b ../55_probes/p4_induced_algebra_grades.rs
./rerun/p4_55b > rerun/p4_55b_output.txt
diff rerun/p4_55b_output.txt ../55_probes/p4_output.txt   # empty, byte-identical
```

All three reproduce byte for byte. Every count taken from `56_probes/q1`, `57_probes/p2` or
`55_probes/p4` in `61_absorption_against_coherence.md` is a count regenerated on this machine, on
this pin, before it was cited.

## New probes

```
rustc +nightly-2026-05-28 -O --edition 2021 -o q1 q1_absorption_versus_coherence.rs && ./q1
# exit 0, "Q1 WORKS"; output committed as q1_output.txt

rustc +nightly-2026-05-28 -O --edition 2021 -o q2 q2_wrap_ring_at_nonzero_fraction.rs && ./q2
# exit 0, "Q2 WORKS"; output committed as q2_output.txt
```

`q1_absorption_versus_coherence.rs` diffs `57_probes/p2`'s absorption predicate against two
faithful readings of `56_probes/q1`'s coherence law, over the same 4248-configuration sweep `p2`
uses and a second, deliberately widened sweep (7744 configurations) where the operand box extends
far past the clamp bounds. It reports, per configuration, whether the box sits entirely inside `Q`,
since that is the condition the dispatch names as the crux.

`q2_wrap_ring_at_nonzero_fraction.rs` extends `57_probes/p3`'s own `Policy::Wrap` arm, which
already implements a genuine rescaling multiply (`(a*b)/scale` then `rem_euclid`) but was driven
only at `F = 0` in that file's own instrument-validation section, to `F = 0, 1, 2, 3` across
`M = 15, 31, 63`, checking the full ring axiom set (associativity, commutativity, both identities,
distributivity, additive inverses).

## Nothing here is a bench

Every number in either probe's output is a count of counterexamples or a boolean verdict from an
exhaustive sweep over a stated small domain. No timing was taken anywhere, and no magnitude in
`61_absorption_against_coherence.md` is a price.
