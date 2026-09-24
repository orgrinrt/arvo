# Where a consumer can write the ties-away alias against `arvo-format`

The ruling reaches ties away from zero as an alias, `toward_zero(x + sign(x) q/2)`.
The closed round `202609192215` wrote into the design, into `standards.rs` and into
`tests/matlab_fi_parity.rs` that the alias can be written only where the doubled
denominator fits, so not past `i64::MAX / 2`. That arm built the shift over `2d`,
and the limit it pinned was a limit of its own helper. This sketch asks where the
alias actually stops.

## Hypothesis

Two spellings, both from the public surface alone:

- the shift, from the slot and the ratio a caller built the position from: with an
  even `d`, add or subtract `d/2` over `d` and move the slot by one where the sum
  would leave the ratio's integer; with an odd `d` no tie is representable, and
  the alias is `half_up`;
- a reading off the position alone: `floor` at a tie whose slot is below zero,
  `half_up` everywhere else, reading `Exact::is_tie` and `Exact::slot`.

Each is swept against an integer statement of ties away from zero,
`sign(x) floor(|x| + 1/2)`, computed as a step of zero or one above the slot and
checked against a brute-force reading of the whole position wherever that fits,
then completed over the format's range the way the map completes.

## Outcome

`WORKS`, with one corner named. `out/run.txt` is the raw output of
`cargo run --release` in this directory.

The domain: slots `i128::MIN`, `MIN + 1`, `MIN + 2`, `-3` to `3`, `MAX - 2`,
`MAX - 1`, `MAX`; denominators 1 to 8, `i64::MAX / 2 - 1` to `i64::MAX / 2 + 2`,
`i64::MAX - 2`, `i64::MAX - 1`, `i64::MAX`, both parities at each end; numerators
`0`, `1`, `2`, `d/2 - 1`, `d/2`, `d/2 + 1`, `d - 2`, `d - 1`; four signatures,
`Integer<8>` and `Integer<64>` under `Wrap` and `Saturate`. 1196 cells a
signature, 91 of them ties.

- The read-off spelling matches the oracle at every cell, and at every cell of a
  second domain whose numerators carry out of `[0, 1)`, so the position lies past
  either end of the index: 1768 cells a signature, 429 ties, none wrong.
- The shift matches at every cell it can form, and cannot form two:
  `slot = i128::MAX`, `d = i64::MAX - 1`, `n` at `d - 2` and `d - 1`. The region
  is `slot = i128::MAX`, `d` even, `n + d/2 > i64::MAX`. There the shifted
  position is `i128::MAX + 1 + (n - d/2)/d`, and `Exact::between` names a
  position past the top only through a carry in the ratio, whose numerator would
  have to be `d + n - d/2`, past `i64::MAX`. No tie lies in that region, since
  `n > d/2`, so the alias there is `half_up`, which the read-off spelling returns.
- The doubled shift the closed round's arm used is unformable at 548 cells of
  1196, and not only past `i64::MAX / 2`: at `d = i64::MAX / 2` itself `2n + d`
  leaves the integer for large `n`.

Five planted wrong spellings, each reported at every signature: `half_up` alone
(42 wrong under `Wrap`), `toward_zero` in place of `floor` at a tie below zero (42),
zero read as negative (7), the shift always upward (249), and the shift composed
with `floor` (222). Under `Saturate` fewer cells differ, because positions past the
range saturate to the same end whichever slot a spelling picks, and every one is
still reported.

## What it unblocks

The design's sentence that the alias is expressible over part of the domain and by
some callers only is refuted, and so are the FIXME in `standards.rs` and the arm
pinning `None` past `i64::MAX / 2`. The replacement arm is a test in
`arvo-format/tests/`, written in the round that follows this sketch.

## What it does not show

That the read-off spelling is what a consumer should write. It shows the alias has
a spelling at every position the public surface constructs, which is every
position `Exact::between` names, since `Exact::on_grid` and `sum_position` both
reduce to it. It is one spike by one author; the test the round lands is the
second instance, and the sweep's two oracles are checked against each other.
