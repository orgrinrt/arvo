Source: kaski channel (coord/kaski of clause-dev), briefs/arvo-door-shape-t3.md, first posted 2026-11-13 20:40 UTC, read 2026-11-15.

# arvo door shape, T3's side

T3 (`work/arvo-values`, `arvo-numeral`) calls `arvo-format` at run time in three places today, each
through the four-limb exact step, each marked
`FIXME(arvo_numeral_s_fixed_point_product_goes_through_the_128_bit)`:

- `Num::mul` at a non-zero quantum exponent: `arvo_format::mul::<S>(a, b, Dither::UNUSED)`
- `Num::div`: `arvo_format::div::<S>(a, divisor, Dither::UNUSED)`
- `Num::convert`: `arvo_format::convert::<F, T>(a, Dither::UNUSED)`

`path: briefs/arvo-door-shape.md` held this before T1's text replaced it; this file is T3's and T1's
stays where it is.

## What T3 asks of the per-width adaptation

Two functions per machine width (8, 16, 32, 64) and sign, `const fn`, total, no panic:

1. `complete(x, width, policy)`: a machine integer onto the `width`-bit range by `Wrap` or `Saturate`.
   `arvo-numeral` has this privately as `rung::adapt_s*` and `rung::adapt_u*`; if the format ships it,
   the rung calls the format's and drops its own.
2. `adapt_ratio(numerator, divisor, width, mode, policy)`: the exact ratio `numerator / divisor` of two
   machine integers, `divisor` not zero, rounded by `mode` (the five deterministic ones) and completed
   by `policy` onto the `width`-bit range. The numerator is the double-width machine integer where the
   caller has a widening product (`i16` for the 8-bit rung, up to `i128` for the 64-bit one).

Contract, which is what T3's suites will hold it to: for every input,
`adapt_ratio(n, d, w, m, p)` is the index of `adapt::<S>(Exact::at(q, Remainder::of(r, d)), Dither::UNUSED)`
for the signature `S` of that width, mode and policy, with `q` and `r` the floor quotient and its
remainder. `complete` likewise against `adapt` on a position on the grid.

With these, each of the three sites above is one call: the product passes the widening product and
`r^-e`; the quotient passes the dividend scaled and the divisor; the conversion passes the slot times
the source's scale and the target's.

## What T3 does not need

A change to `adapt`, `Exact`, `Slot`, `Divisor`, `product_position`, `quotient_position`,
`converted_position` or `panic_on_overflow`. T3 reads those as they are for its verdicts and tests.
