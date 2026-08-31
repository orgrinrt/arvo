# Sketch: does the value path give the same answer under both settings of overflow-checks

**Hypothesis.** The adaptation's exact step was performed in the carrier its result has to fit, so a
large exact position produced a wrong answer without `overflow-checks` and a runtime panic with it.
Moving the exact step to a carrier wide enough to hold it should make the answer correct and identical
in both settings, with no check anywhere.

Cited by `mock/design_rounds/202609010115_changelist.src.md`.

## What it was, reproduced before the change

Through the then-public `complete_slot`, with two controls that both passed:

```
complete_slot(Wrap, 3, -1, 5)         -> 3    control, in range
complete_slot(Wrap, 8, -1, 5)         -> 1    control, ordinary out of range
complete_slot(Wrap, i64::MAX, -1, 5)  -> 5    at overflow-checks = off, exit 0
                                      -> panic at apply.rs:194, exit 101, at on
```

**The correct answer is 0.** The span is 7 and `i64::MAX - (-1)` is 9223372036854775808, whose
remainder modulo 7 is 1, so the result is `-1 + 1`. The code returned 5, which is **inside the
representable range** and indistinguishable from a right answer.

## What it is now

`output.txt`, both settings. The surface is `adapt`, because the two regions are no longer public.
`Integer<3>` is the slot range `[-4, 3]`, span 8.

```
in range   2
ordinary   1
i64::MAX   -1
i64::MIN   0
```

Identical in both settings. `i64::MAX` wrapping into `[-4, 3]` is `(i64::MAX + 4) mod 8 = 3`, so
`-4 + 3 = -1`, computed by hand and matched. `i64::MIN` gives `4` for the same remainder and `0` for
the answer.

## What must fail

The two settings must agree. If they ever differ, the exact step has moved back into a carrier that
cannot hold it. And the two control lines must keep their old answers, or the change altered ordinary
behaviour rather than only the edge.

## What this does not establish

That every arithmetic site in the crate is safe. It establishes it for the completion and rounding
path reachable through `adapt`, at the edges of `i64`, for the wrapping policy. The saturating and
clamping policies compare rather than compute and were never at risk.

## Toolchain

The repository's pinned nightly, copied in so the sketch resolves the way the workspace does.
