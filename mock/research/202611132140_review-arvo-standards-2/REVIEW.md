# Review 2 of `work/arvo-standards` at `9e270619`

Base `faec24c6` (`origin/dev`). Round one sat at `f0eeb2e3`; this reads the five commits after it (design round `202611132100`) and checks the round-one findings against the tip.

Canon gate passed. Before opening round one's reasoning I read `ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` (ratified, op: "Let's start with the two named, and reserve the rest named for future, so it can't close them out"): the branch builds MATLAB `fi`/`fimath` and IEEE 754 and reserves SystemC, Vitis and Algorithmic C. I also read `ruling::the_standard_is_parity_in_output_not_in_the_internals` (stated, op's words). The suites assert output parity and do not copy internals, which fits it. This is the second independent read on canon fit, and it agrees with round one. One clause of that second row is not met, and it is finding 3.

Test gate. `cargo test -q --workspace --no-fail-fast` fails in one place only, the known lib red at `multiplication/tests/the_inverse_round_trip.rs:37` (297 of 298). Every integration target is green, and that includes `arvo-numeral`'s `tests/compile_fail.rs`, 18 of 18. `cargo test -p arvo-format --doc` and `cargo mock --lint-only` are green. I read every test body under `tests/ieee754*`, `tests/ieee754/mod.rs`, `tests/ties_away/mod.rs`, `tests/matlab_fimath_parity.rs` and the two new trybuild cases. None is tautological. Finding 2 is a gap in the planted arms.

## Round-one findings at the tip

1. `RoundTowardZero` taking over the printed path: fixed. The names are now type aliases (`src/standards/ieee754.rs:157-170`), and `arvo-numeral` passes 18 of 18.
2. The binary `gap` overstated: fixed. Binary16 runs four directions against the rule beside its encodings, and binary64 runs four against the host. The row now says exactly that.
3. No decimal `roundTiesToAway`: fixed. `TiesToAway<F, O>` (`tests/ieee754_decimal_parity.rs:35-47`) runs under all five directions.
4. The "reads no arvo code" claim: fixed. `DECIMAL32`/`DECIMAL64`/`BINARY16` are hard-coded (`tests/ieee754/mod.rs:190-209`), and the range pin is a separate arm.
5. Unsigned ambient and `P = 0`: partly fixed. Every reader of the range now refuses both, and two compile-fail cases pin it. The fix breaks a contract of its own, though: see finding 1.
6. Root README: fixed (`README.md:21`).
7. Fimath at one shape: fixed in substance. There are 13 shapes, signed and unsigned, with planted arms at every shape. The wording that describes them is finding 5.
8. Declarations checked against declarations: fixed. `the_shipped_floating_point_is_binary32_but_one_member` asserts exactly one miss.
9. Casts and bare locals: answered with a reason in the topic ("the overhaul lane's"). Accepted as non-blocking.
10. Summaries over 80 characters: mostly fixed. See finding 6.
11. Stamp without an id: fixed. `where_arvo_s_standard_declarations_live` exists in `.shared/scripts/life/tickets-arvo.toml`.

## Findings

1. `mock/crates/arvo-format/src/standards/ieee754.rs:73-99` [correctness, blocking]. The new obligation sits inside the values of `MAX` and `MIN`. That makes `slots::is_admissible` force it. The design says that function must not force: `DESIGN.md.tmpl:856` reads "Each obligation has a verdict beside it that asks the same question without forcing the const", and `:370-372` lists `slots::is_admissible` and `is_admissible_format` as those verdicts. Evidence: `probes/`, feature `slots_verdict`. `is_admissible::<Significand<UnsignedBinaryRationals, 24>>()` fails to compile with the `SIGN_AND_MAGNITUDE` panic instead of returning `false`. Meanwhile `is_admissible_format::<Interchange<UnsignedBinaryRationals, 24, 127>>()` returns `true` (default probe). So one verdict refuses the build and the other admits a format the crate refuses. `DESIGN.md.tmpl:1323-1327`, added this round, says "every reader of the range refuse[s]". That sentence contradicts `:856`, and the code follows the wrong one of the two. Fix: give the obligation a verdict form that returns `false`, so that the verdict reports and the obligation refuses, as every other obligation in the crate does. Then put a negative test beside the two trybuild cases asserting that the verdict is `false`. Correct the `:1323` sentence too.
2. `mock/crates/arvo-format/tests/ieee754_decimal_parity.rs:103-111` [tests, blocking]. The brief says "Each parity arm also carries a planted wrong mode, and you show that arm failing." The decimal suite runs ten arms, five directions on each of two formats, and plants only six of them. Not planted: decimal32 `TowardPositive` and `TowardZero`, decimal64 `TowardPositive` and `TowardNegative`. `roundTowardPositive` has no planted arm on either decimal format. Every binary format plants all four of its directions. All four missing arms can be written and each one misses heavily: probe output `d32 +inf as zero 2820, d32 zero as -inf 2995, d64 +inf as zero 2912, d64 -inf as +inf 5913`. Add them, plus the short-precision plant on decimal64 (5652 misses), which is planted on decimal32 only.
3. `mock/crates/arvo-format/src/standards/ieee754.rs:15-19` [spec, blocking]. Op, in `ruling::the_standard_is_parity_in_output_not_in_the_internals`: "we can leave each standard a few holes with a FIXME to fill later". Four holes are documented: no infinity, no signed zero, no binary128/decimal128, and no operation parity (the obligation's `gap`). None of them carries a `FIXME(<ticket>)` (grep of `src/standards` and the IEEE tests: zero). The module doc also says each hole "is a test rather than a sentence". For the signed zero that is false: the only mention is a sentence in the test module doc (`tests/ieee754_binary_parity.rs:19-20`), and no arm asserts it. Fix: add one marker per hole naming its ticket, and add an arm that lands `-0` (`0x8000_0000`) and asserts that it meets the one zero.
4. `mock/crates/arvo-format/tests/ieee754/mod.rs:159-165` [tests, non-blocking]. `agrees` accepts a landing with `finite: false` wherever its saturated value equals the standard's finite answer. So under round to nearest, an overflow verdict raised at the top magnitude, where the standard says finite, passes. Since `panic_on_overflow` does not depend on magnitude, I could not construct a real defect that this hides. Require `finite` except where the exact value lies past the largest member.
5. `mock/registry/obligation.toml:201`, `DESIGN.md.tmpl:1389-1391` [drift, non-blocking]. Both say "word lengths from one to sixty-four". The suite runs 13 hand-picked shapes (`tests/matlab_fimath_parity.rs:260-272`). Either write "at thirteen shapes from one to sixty-four bits", or generate all 128 widths with the macro, which costs little. As worded, the row claims a range that was only sampled.
6. `mock/crates/arvo-format/src/standards/ieee754.rs:6-7,142-143` [naming, non-blocking]. Both summary sentences still run past 80 characters.
7. `mock/crates/arvo-format/README.md.tmpl` [public-surface, non-blocking]. The crate readme never mentions `standards`, MATLAB or IEEE 754. That has been stale since MATLAB landed on `dev`. It is not new here, but this branch adds the second standard.

## Verified

- The binary32 and binary64 host oracles are exact: the products are exact `f64`s, wide `i64` inputs stay normal, and directed stepping, overflow and the `MAX` to infinity step are right.
- The digit rule's overflow is right for round to nearest (unbounded-exponent round, then infinity) and for directed modes (largest finite member on the saturating side).
- The ties-away read-off (`floor` at a negative tie, `half_up` otherwise) is right, and `overflows` shares its tie test.
- The fimath integer rules for all six methods and both actions are right, including unsigned 64.
- `EMAX = 0` is refused by the quantum's own obligation (probe `emax0`).
- Generated `docs/FORMAT.md` carries this round's design text. Commit subjects are well formed with no bylines. No em- or en-dashes and no bold were added.

## Notes

Findings 1 to 3 are small, and they are what blocks. Finding 1 needs a decision about the design: the sentence the branch added at `:1323` contradicts the verdict rule at `:856`, and the verdict rule is the one to keep. Probes and raw output are in `probes/` and `probes/README.md`.

Verdict: do not merge.
