# p1a, first run, kept because it was wrong

`p1a_output_BROKEN_FIRST_RUN.txt` is the first run of `p1a_declared_law_lies.rs`
and it is wrong in two ways that are worth keeping on disk rather than deleting,
per this panel's habit of keeping a failed instrument beside its repair.

1. **The wrapping map was not wrapping addition.** It computed
   `(a - LO) + (b - LO) mod n + LO`, which adds the two offset-binary
   *representations*, giving `a + b - LO` rather than `a + b`. Zero was not the
   identity under it. The corrected map is `(a + b - LO).rem_euclid(n) + LO`.
2. **The fold was seeded with a literal `0`**, so the measurement was partly a
   measurement of whether `0` was the map's identity rather than of
   associativity. The corrected fold starts from `xs[0]` and never mentions an
   identity at all.

The visible symptom was `Wrap` disagreeing at 65536 of 65536, a number that
looks like a strong result and is an artifact. It is the same failure shape this
panel has recorded elsewhere: an instrument that cannot fail, or in this case
cannot pass, reporting a headline.
