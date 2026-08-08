# Run log, `42_probes/`

Both probes on `nightly-2026-05-28`, `rustc 1.98.0-nightly (57d06900f 2026-05-27)`. Zero anchored
feature gates in either file (`grep -c '^#!\[feature' 42_probes/*.rs` returns 0 on both).

## p1: commutativity is universal

```
$ rustc +nightly-2026-05-28 -O --edition 2021 -o p1 p1_commutativity_is_universal.rs && ./p1
```

Exit 0. Output in `p1.out`. Zero commute-failures across all four (sign, policy) combinations,
626,224 pairs total (167,480 unsigned x2 policies x2 ops, 145,632 signed x2 policies x2 ops).

## p2: composed accumulator and law bound

```
$ rustc +nightly-2026-05-28 --edition 2021 p2_composed_accumulator_and_law_bound.rs --crate-type lib -o /dev/null
```

Four builds, output in `p2.out`:

| arm | cfg | exit | what it shows |
|---|---|---|---|
| base | (none) | 0 | the composed bound (width-sufficiency + law-satisfaction) compiles; the unobservable-axis swap (`observable_swap_unobservable_axis`, headroom Minimum -> Doubled) compiles as part of the default build |
| bad_width | `--cfg=bad_width` | 1 | `Cap<7>: Log2Ceil` unsatisfied; the width half refuses independent of the law half |
| bad_law | `--cfg=bad_law` | 1 | `Wrap: AbsorbingTop` and `Wrap: MonotoneAdd` both unsatisfied; the law half refuses independent of the width half |
| observable_axis | `--cfg=observable_axis` | 1 | the same generic `generic_consumer` function, instantiated at an accumulator strategy differing only on the observable overflow axis, refuses with the identical diagnostic as `bad_law`, confirming 40's observable/unobservable split still separates cleanly at this composition point |

First version of `p2` mismatched the derived accumulator width in three return-type annotations
(`Num<N5, ...>` where the correct derivation is `N4 + ceil(log2 3) = N4 + 2 = N6`). rustc's own
E0308 caught it before any of the four intended-refusal arms were run, which is the mechanism
working rather than a probe defect worth hiding: the derivation is checked by the compiler at
every call site, not merely declared.
