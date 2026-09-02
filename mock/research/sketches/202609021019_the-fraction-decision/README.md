# The fraction coordinate's constructor, against the ratio it was handed

**Hypothesis.** `Fraction::of` reads two pairs as whole numbers because their exact form is not
representable, and its rustdoc says so. The question is whether that reason is true, whether the
positive-denominator invariant it protects is load-bearing, and what a better rule would cost.

**Outcome: FAILS, in the direction the hypothesis did not anticipate.** The stated reason is false on
about two fifths of the pairs the branch fires on. `13835058055282163710` pairs have an exact form
inside the type and the shipped rule throws it away, because it never cancels the factor of two the
two operands share.

## The two files

`what_the_constructor_answers.rs` sweeps 9604 `i64` pairs over 98 values per axis, chosen to include
every edge and a spread that is neither a power of two nor near one. Four candidate rules against one
exact oracle.

`the_rule_over_every_width.rs` runs the same four rules over the **whole** matrix at widths three to
sixteen, which is what the `i64` sweep cannot do. Representability is decided by exhaustive search at
`W <= 8` and by a derived family above it, and the two are cross-checked at every width where both
run, with a deliberately broken family as the control.

## What was established

**The positive-denominator invariant is load-bearing, so the `Phase` repair does not transfer.**
`4477` of `4801` negative-denominator pairs store a remainder outside `[0, 1)` once `Exact::between`
splits them, and the `[0, 1)` reading is what lets the rounding modes be stated once rather than once
per sign. The control: every one of `4704` positive-denominator pairs stays inside. Holding the pair
as `Phase` does is lossless and breaks the invariant on `4900` of `9604` pairs.

**The shipped rule's stated reason is false on the larger part of its own family.** At width `W` it
loses an exact answer on `2^(W-1) + 2^(W-2) - 2` pairs. That closed form reproduces the exhaustive
counts at `W` in `{3, 4, 8, 16}` exactly, which is what licenses reading it at 64.

**It also flips the sign.** `2^W + 2^(W-1) - 2` pairs come back with the wrong sign, `of(3, i64::MIN)`
answering a positive three where the named ratio is a tiny negative.

**A rule exists that gives up nothing.** Cancel the shared power of two first, and fall back only
where the other operand is odd. Over the whole matrix at eight widths it holds four properties with
no exceptions: the denominator is positive, the sign is the named one, the answer is exact wherever
an exact one exists in the width, and where none exists the relative error is at most `1 / MAX`. The
bound is attained rather than slack, at `1 / i64::MAX`.

## What the first draft got wrong, and how

The first version of the rule sent every `i64::MIN` pair to the substitution. The exhaustive search
refused it at `W = 3`: `of(-4, -2)` is exactly two over one, because both operands are even. The
sampled `i64` sweep had scored that draft clean, and would have kept scoring it clean, because none
of its 98 values put an even operand against `i64::MIN`.

The first version of the oracle formed `|rn*d - n*rd| * i64::MAX` in `i128`, which reaches `2^189`
and wrapped. It reported a worst relative error of `0/-1` and fewer breaches for the shipped rule
than there are. Both are recorded in the module comments rather than edited away.

## Two facts about the reader, found while establishing the first one

`Exact::is_tie` doubles the stored remainder in `i64` where the neighbouring `round_slot` does the
same comparison in `i128`. `233` of the swept remainders leave `i64` when doubled.

`Exact::between` adds the euclidean carry to the slot index in `i64`. `4637` of the swept
combinations leave `i64`.

## Reproducing

```
rustc -O what_the_constructor_answers.rs -o /tmp/frac_probe && /tmp/frac_probe
rustc -O the_rule_over_every_width.rs -o /tmp/width_probe && /tmp/width_probe
```

Both carry their own controls and assert them, so a run that prints its tables has also shown that
the columns can move. Output as committed is in the two `*_output.txt` files beside the sources.
