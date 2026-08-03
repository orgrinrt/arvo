# 44 probes: outcomes

One probe. Built fresh against the workspace pin (`rustc 1.98.0-nightly (57d06900f
2026-05-27)`, matching files 41/42/43) while writing `44_ringer_what_the_overturn_
left_behind.md`.

## probe_1_the_overflow_band_for_mixed_format_addition.rs

Build:

```
rustc --edition 2021 --crate-type lib probe_1_the_overflow_band_for_mixed_format_addition.rs --out-dir out
```

Outcome: **WORKS**. Compiles clean; every `const _: () = assert!(...)` evaluates to
true at compile time, which is the whole result (a `const` assertion that fails to
evaluate is a compile error, so a clean build is the positive claim, exactly the
shape every prior file in this review uses for exhaustive const-eval checks).

### Claims established

- **CLAIM A**: mixed-format addition CAN inhabit the overflow band. Two independently
  shaped witnesses (`_WITNESS_1`, `_WITNESS_2`), each cross-checked against a second,
  hand-derived rational-arithmetic assertion (`_WITNESS_1_LO`/`_WITNESS_1_HI`) rather
  than trusted from the boolean search alone.
- **Negative control**: same-format addition (`d1 == d2 == dr`) never inhabits the
  band, re-run at three parameter choices, confirming the search machinery agrees
  with `28:229-231` / `26:169-171`'s own claim for the case those files did check.
- **CLAIM B**: the sweep of 40 genuinely mixed (`d1 != d2`) triples splits 36
  inhabited / 4 empty (pinned exactly: `_SWEEP_TRIED == 40`, `_SWEEP_INHABITED ==
  36`, `_SWEEP_EMPTY == 4`). Mixed-format addition is not unconditionally in the
  band.
- **The structural read on the four empty cases**: all four share `d2 = 2 * d1`
  (the pair (3, 6) tried both ways), i.e. one operand quantum divides the other, so
  the "mixed" pair collapses to single-quantum arithmetic in disguise. Confirmed by a
  direct contrast: swapping in a non-dividing pair at the same relative magnitude
  (4 and 5) and the same two destination choices that emptied for (3, 6) makes the
  band inhabit both times (`_NONDIVIDING_PAIR_INHABITED_1`, `_2`).

### Scope, stated honestly

Small swept parameter windows (denominators up to 6, operand ranges up to 20,
`m` chosen as an odd numerator per destination). This is enough to establish
existence (the band CAN be inhabited) and non-universality (it is not ALWAYS
inhabited) with a compiled witness for each, which is what the deliverable's
question needed. It is not a claim that the 36/4 ratio, or the specific
dividing-quantum structural read, is the complete characterisation; a member
building the real mixed-format addition law would want the same closed-form
treatment file 43 gave division (a membership predicate proved algebraically, not
merely swept), which this probe does not attempt.

The "every float operation" member of `40:178-180`'s claim is untouched by this
probe. Nothing here builds a `Specials`-carrying model numeral or checks a float
addition specifically; that is named as still open in the deliverable this probe
supports.
