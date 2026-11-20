# Sketch findings: where the integer-count remainder is exact, completed, or composable, and where the S-grid alternatives break

**Date:** 2026-11-19
**Outcome:** WORKS
**Unblocks:** the region statements in `mock/research/202611192115_the-remainder-panel/fabian-giesen.md`, each of which cites a section number below.

## Hypothesis

The remainder `R = x - n y`, `n = round_c(x / y)` an integer by a rounding convention `c` drawn from the six modes, has these properties over constant-quantum formats, and the alternatives filed as options b and c do not.

1. With the divisor in the dividend's own format `F`, `R` lies on `F`'s grid for every pair and every convention exactly where `F`'s phase is whole, at every radix and exponent. With the divisor in another format `G`, exactly where `q_G / q_F` is a whole multiple of the reduced denominator of `G`'s phase (the lattice condition). The exponent alone does not decide it.
2. At a zero phase, `R` lies in `F`'s range for every pair on a signed range under all six conventions, and on an unsigned range under `toward_zero` and `floor` only; `|R| < |y|` under all six, with the sign rules of the directed conventions and `|R| <= |y|/2` at the half conventions.
3. The count and the slot of `R` read no quantum: at a fixed phase and slot range they are the same at every radix and exponent.
4. At a zero phase the count lies in `[MIN, -MIN]` (signed, one past the top at `MIN / -1` only) or `[0, MAX]` (unsigned); with a fractional phase it passes the slot range.
5. Toward zero is the host's `%`, and the host's `rem_euclid` is the floor remainder by `|y|`.
6. On a binary float with subnormals, `R` is a member for every pair under `toward_zero`, `half_up`, `half_even`, and not under `floor`, `ceil`, `stochastic`; the count passes the significand by far.
7. Option b (`r = x - y div::<S>(x, y)`) and option c (the consumer composing arvo's shipped `div`, `mul`, `sub` and `convert`) miss the definition outside the zero-exponent, zero-phase, wrapping cell; a reimplementation in a wider integer format (c3) reproduces it at every zero-phase format.

## What was tried, in order

The binary is `src/main.rs`, with the definition in `src/reference.rs` (its own rationals in `src/rat.rs`, reading nothing of arvo) and the arvo arms in `src/arms.rs` (arvo-format by path: its shipped `div`, `mul`, `sub`, `convert`, `adapt`, `Divisor::of`, `Divisor::member`). Every section asserts its prediction and runs a planted variant that the same check has to report. Run with `cargo run --release` from this directory; the output is `outputs/run.txt`.

The first build failed:

```
error[E0277]: the trait bound `Q: Hash` is not satisfied
    --> src/main.rs:316:13
     |
 316 |         set.insert(Q::int(0));
     |             ^^^^^^ the trait `Hash` is not implemented for `Q`
```

Fixed by deriving `Hash` on the sketch's rational.

The first run failed on prediction 4 as written:

```
thread 'main' (29821) panicked at src/main.rs:244:17:
assertion `left == right` failed: signed 1
  left: (0, 1)
 right: (-1, 1)
```

The prediction `[MIN, -MIN]` is wrong at signed width one, whose range `{-1, 0}` has no positive divisor, so the lowest count is 0. The assertion was corrected for that case alone and the comment at the assertion records it. Every other prediction held as first written.

The first version of the named c1 example used `25 / 13`, whose product fits the range and so showed nothing; it was replaced by a search for the first pair where c1 misses, which is `-32 / -31` under `ceil` and saturation.

## The result

From `outputs/run.txt`.

1. Section 1a: 720 formats (radix 2, 3, 10; exponents -3, -1, 0, 1, 2; eight phases; both signednesses; widths 3 to 5), divisor in the same format: exact at every pair and convention exactly where the phase is whole, 270 exact and 450 not. The exponent-only rule, option b's region, disagrees at 306 of them. Section 1b: 960 (F, G) format pairs; the lattice condition predicts exactness at all 960 (438 exact). The quantum-ratio-only rule, which forgets `G`'s phase, is wrong at 202; the member door's same-grid rule is wrong at 440.
2. Section 2: 576 zero-phase cells (radix 2 and 10; exponents -3, 0, 2; widths 1 to 8; both signednesses; six conventions): in range everywhere except unsigned under `ceil`, `half_up`, `half_even`, `stochastic` from width 2; the bounds hold everywhere.
3. Section 3: 11,709,630 points; the definition's count and slot are identical across radix and exponent; option b's remainder moves with the quantum at 8,153,320 of them.
4. Section 4: signed 8 counts lie in `[-128, 128]`, unsigned 8 in `[0, 255]`; at phase one half over signed 6 the count reaches 63, past the range's 32.
5. Section 5: 86,870 signed pairs at widths 1 to 8: `toward_zero` is the host's `%`; `rem_euclid` equals `floor` by `|y|`, and `floor` for `y > 0` with `ceil` for `y < 0`. The planted Euclid-as-floor-by-`y` is reported at every width from 2.
6. Section 6: precisions 2, 3, 4, exponents -3 to 3 with subnormals: `toward_zero`, `half_up`, `half_even` exact at every pair; `floor` and `ceil` off at 132 of 930, 552 of 3906, 2256 of 16002 pairs; `stochastic` off too. The largest count is 192, 448, 960 against significands of 2, 3, 4 bits.
7. Section 7 (width 6, ten formats, six conventions, three policies; the table is in `outputs/run.txt`): at the zero-phase, zero-exponent signed integers option b misses the definition at one pair (`-32 / -1`), and c1 and c2 match it only under wrapping. At `e = -3`, `e = 2`, radix ten, a whole phase of one or of minus three, and the two fractional phases, b, c1 and c2 miss it at most pairs. c3 matches at every zero-phase format and every policy (4032 of 4032, 16128 of 16128 at stochastic). Named pairs: option b at `-32 / -1` under saturation gives `q = 31`, `r = -1`, so `|r| = |y|`; option b over the values 1 to 64, value 3 by value 5 under floor and saturation, gives `q` at value 1 and `r = -2`, where the definition gives 3; c1 at `-32 / -31` under ceil and saturation forms the product `-32` (exact `-62`) and answers 0, where the definition gives 30; c2 at `e = -3` under half to even rounds `-32/-22` onto eighths as 1.5 and then to 2, where the count is 1 (double rounding), and lands `-4 / -1` on eighths past the top, wrapping the count to -4 where it is 4.

## What is NOT established

- Widths past 8 at the zero phase, past 6 for the arvo arms, past 5 for the lattice sweep. The lattice theorem is proved in the seat's file; the sweep is one instance of it.
- The magnitude-indexed family beyond the toy binary format at precisions 2 to 4 over exponents -3 to 3: no decimal float, no overflow region, no binary64. The float claim rests on this sweep, on the standard exactness of IEEE 754 `remainder` and C `fmod`, and on Python's documented `-1e-100 % 1e100`.
- Clamp was run as shipped; no clamp bound other than the range's ends was exercised.
- Nothing here times anything, and nothing here is arvo code: the definition's arm is the sketch's rationals handed to arvo's `adapt`, a spike chosen to reach the check.
