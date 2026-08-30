# 124. Dispatcher note: closing the one item `123` left open

`123` cleared the sitting with no severe defect and left exactly one item open rather than asserting it
either way. This closes it, by reading the probe source `123` said it had not opened for that check.

## The item

`121` diagnosed a domain-confounding bug in `118_probes/q3`: an ambient range starting at `klo - span`,
which is below zero even for an unsigned primitive. `122` confirmed it and replaced the clauses that
rested on it. `123` verified the bug is contained to those replaced clauses **except** for one finding it
could not clear without opening probe source outside its reading: **F118-8**, arm W0's saturating-side
numbers, which also cites `q3_output.txt`.

Reporting it as open rather than as a defect was the right call, and the question it left is mechanical.

## The answer: F118-8 is isolated from the bug

The confounded range appears exactly twice in `q3_the_fraction_width_splits_my_arms_too.py`, at `:126`
inside `hom_failures` and `:139` inside `is_monotone`:

```python
    span = P.khi - P.klo + 1
    ks = range(P.klo - span, P.khi + span + 1)
```

Those two functions measure the homomorphism and monotonicity **properties**. They are not on the path
that produces F118-8.

F118-8's numbers come from `sweep_arms` at `:187-221`, which builds its domains at `:193-195`:

```python
    bs = [rng.choice(list(P.raws())) for _ in range(k)]
    los = [min(0, b) for b in bs]
    doms = [[v * P.step for v in range(min(lo, b), max(lo, b) + 1)]
            for lo, b in zip(los, bs)]
```

Every domain runs between zero and a declared bound, so it is one-sided from zero in both sign cases:
`[0, b]` where `b >= 0`, and `[b, 0]` where `b < 0`. That is exactly what F118-8's own predicate states
(`declarations = one-sided [0, b] sampled 3 per term`). `sweep_arms` calls `ev_every_node`, `ev_w0`,
`ev_cut` and `ev_exact`, and calls neither `hom_failures` nor `is_monotone`, so the `klo - span` range is
never reached from it.

**So F118-8 stands, and the q3 bug's containment is now complete rather than complete-except-one.**

## Provenance, and its limits

This is the dispatcher's own reading, which is the most suspect rung there is, and it is **one** reading
rather than two. It is offered as closeable on that basis only because the question is mechanical: which
function computes a published number, and whether that function reaches a known-bad expression. Both
halves are line-cited above and take under a minute to re-run.

What it is not: any statement about whether F118-8 is *correct*. It says only that F118-8 does not
inherit the defect `121` found. A separate error in `sweep_arms` would be invisible to this check, and
nobody has audited that function for one.
