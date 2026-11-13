# Review 3 of `work/arvo-standards` at `366d2e15`

Base `origin/dev`. Round two sat at `9e270619`. This review reads the five commits after it (round
`202611132200`) and judges the whole branch as it stands. It is the third and last review round.

Canon gate: passed. I wrote my own reading before opening the earlier reviews.
`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` (ratified, op) is met: MATLAB and
IEEE 754 are built, and SystemC, Vitis and Algorithmic C are reserved in `src/standards.rs:20-23`.
`ruling::half_up_denotes_ties_toward_positive_infinity` (ratified, experts) reaches `roundTiesToAway`
as an alias and not as a name, and `rounding_direction` follows that. Op's words on the ticket ask
for typestate and generic composition and not a hard-coded choice. `Interchange<A, P, EMAX>` is
exactly that. `ruling::the_standard_is_parity_in_output_not_in_the_internals` (stated, op) allows
holes "with a FIXME". Finding 1 is a hole that carries none. This is the third independent read on
canon fit, and it agrees with the first two.

Test gate: I read every test body in `tests/ieee754*`, `tests/ieee754/mod.rs`, `tests/ties_away/mod.rs`,
`tests/matlab_fi_parity.rs`, `tests/matlab_fimath_parity.rs` and the four new trybuild cases. No test
is tautological. Every binary and decimal direction now has a planted arm, and each planted arm misses.
The gate fails in one place: a sampled law, finding 2.

## Verdict

MERGE AFTER FIXES. As the branch stands today, do not merge. Findings 1 to 3 block. Each is small.

## Findings

1. `src/standards/ieee754.rs:14-24`, `DESIGN.md.tmpl:1353-1361,1366-1382` [spec, blocking]. No crate
   path rounds a value onto an IEEE format. `is_convertible` is false for `Fi<32, 0>` to `Binary32`,
   for `Binary32` to `Fi<32, 0>` and for `Binary16` to `Binary32`. A fixed-point control converts.
   The probe is `probes/src/lib.rs`. So the "adaptation of a value" that the parity is said to be of
   exists only as `land` in `tests/ieee754/mod.rs:111`, and the design disowns it ("not a mechanism of
   this crate", `:1359`). `:1186-1190` reads "A convention that needed this crate to compute something
   to make it work ... would be a gap in the primitives". By the branch's own sentence, then, this is
   a hole. It is not among the four holes, and the FIXME at `:22-24` names only sums, products and
   quotients. Failure scenario: a consumer declares `Binary32` from the README and finds that nothing
   in arvo puts a value on it. The regression test op wanted "for free" guards `adapt` at one fixed
   magnitude and a loop in a test file, not a path that ships. Also, "each a test" (`:14-15`,
   `DESIGN:1366`) is false for the operation hole: no test names an IEEE format. The only refusal
   test is trunk's `tests/ui/addition_over_a_float_is_refused.rs`, over `Floating<4, -3, 4>`. Fix:
   name conversion in the operation hole, cite a ticket in a FIXME, and add a test asserting
   `is_addable`, `is_multipliable`, `is_divisible` and `is_convertible` false on `Binary32` and
   `Decimal32`, so the test fails the day either is admitted.
2. `tests/matlab_fimath_parity.rs:260-272` [tests, blocking]. The law is stated over every word length
   and signedness, but it runs at 13 of the 128 shapes. Round two offered two fixes. One was to run
   the whole matrix, "which costs little". The other was to reword the claim. The round reworded
   (`obligation.toml` gap, `DESIGN:1397`), which keeps the sample and makes the sentence accurate. In
   `probes/tests/fimath_every_width.rs` the file's own macro runs at all 128 shapes: 256 tests, all
   green, built and run in about five seconds. Someone chose which widths to include. Replace the 13
   invocations with the full matrix and keep the hand-picked fraction lengths beside it.
3. `tests/ieee754_binary_parity.rs:462`, `DESIGN.md.tmpl:1330-1331` [correctness, blocking]. The test
   asserts that `slots::is_admissible::<Significand<UnsignedBinaryRationals, 24>>()` is true, and the
   probe shows `is_admissible_format` is true as well. `Significand`'s `Slots::ADMITTED` refuses that
   range. `DESIGN:370-372` names `slots::is_admissible` as the verdict form of the `Slots` contract's
   obligation, and `:856` says the verdict "asks the same question". The override at
   `ieee754.rs:107-118` makes that false for this implementor. The new sentence at `:1330` ("answer
   their own obligations and nothing further") redefines the contract so that it fits the code. That
   is an undeclared design change, and the test pins the disagreement as expected. Code generic over
   `S: Slots` that asks `is_admissible` gets true, and then the build is refused. Fix: delete `:1330-1331`
   and the assertion at `:462`. State the gap instead: an override of `ADMITTED` is invisible to the
   free verdict. Ticket it on the `Slots` contract, for example a verdict const that `ADMITTED`
   asserts, and put a FIXME at `ieee754.rs:107`.
4. `tests/ieee754_binary_parity.rs:269-275,339-345` [tests, non-blocking]. Binary32 and binary64 plant
   wrong modes but not a wrong format. Decimal plants a short precision (`ieee754_decimal_parity.rs:127`).
   Add `binary32_misses::<Binary<23, 127>, RoundTiesToEven>(Mode::HalfEven) > 0` and the same with a
   wrong `EMAX`.
5. `src/standards/ieee754.rs:59-60` [naming, non-blocking]. The new `SignificandBounds` summary runs to
   about 105 characters.

## Round-two findings at the tip

R2-1 is partly fixed: the forcing is gone, and what remains is finding 3. R2-2 is fixed: all ten
decimal arms are planted, and the short precision is planted on both formats. R2-3 is fixed, except
for the operation-hole test in finding 1. The three tickets exist in `tickets-arvo.toml`, and
`the_standards_negative_zero_lands_on_the_one_zero` asserts the zero. R2-4 is accepted with its
reason. R2-5 was reworded and not run, which is finding 2. R2-6 is fixed, but finding 5 is new. R2-7
is fixed: the crate readme names both standards.

## Checks run

- `cargo test -q --workspace --no-fail-fast` from `mock/`: one red only, the known
  `the_inverse_round_trip.rs:37` (297 of 298 lib). Every integration target is green, including
  `arvo-numeral` compile-fail at 18 of 18 and the five standards targets (12, 5, 15, 26, 10).
- `cargo test -q -p arvo-format --doc`: 5 of 5.
- `cargo mock --lint-only`: all lints pass, with 46 warnings that the branch did not add.
- `docs/FORMAT.md`: the IEEE section is byte-identical to the template.
- No em-dashes, en-dashes or bold were added. The 20 commit subjects are well formed and under 72
  characters, with no bylines. No stale `standards::Fi` paths remain anywhere.

## Verified

- The exponent ranges, significand bounds and widths are right for all five aliases. The 128-bit and
  unsigned refusals fire where the range is forced.
- The host oracles are single-rounding. The directed stepping handles the zero, `MAX` and infinity
  cases.
- The ties-away `overflows` and `read_off` share one predicate, so the two cannot disagree.

## Notes

Under the three-round cap the coordinator decides whether 1 to 3 are fixed before landing or land as
follow-ups. All three cost less than this review did. Probes are in `probes/` and the raw output is in
`probes/README.md`.
