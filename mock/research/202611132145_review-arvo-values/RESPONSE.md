# What was done about the first reading

`PASS_1.md` is a fresh seat's reading of `work/arvo-values` at `9a3d5149`, and its verdict was that
the branch should not merge. This file says what happened to each of its thirteen findings, in its
numbering, so that the next reading can check the answers and need not derive them again.

The source changes went through design round 202611132143-c8zp. The documents that need no round
were changed beside it.

## By finding

1. The sweeps of `div`, `convert`, the product at a non-zero exponent and the three `fits_` verdicts
   compared the value with the function of `arvo-format` that the value calls. They now compare it
   with `tests/common/oracle.rs`, a rounded quotient and a completion written in the test crate that
   calls no computing function of the format. Every sweep is green against it, and the red runs of
   `mutant-div`, `mutant-convert` and `mutant-mul` against the rewritten tests are in
   `mock/research/sketches/202611132143-c8zp_value-mutants/`. That note also says what is not shown:
   no mutant reaches the scaled product, whose control is computed in the test, and the oracle is
   written for the binary radix alone.
2. The readme said the value had no product, quotient or conversion. It now says what the value has.
3. The root design and the agent instructions said the same and were changed the same way.
4. The calls across the format border lived only in a channel document. They are now in
   `THE_CALLS_ACROSS_THE_FORMAT_BORDER.md` beside this file.
5. `Accumulator` against `question::which_chain_carrier_ships`. The crate design now cites the
   question as the bound the type stays inside. That is one seat's reading, so the question went to
   op as ticket `does_the_name_accumulator_in_arvo_numeral_pre_empt_which_cha`, with keeping the
   name, renaming it and removing it as the options.
6. `take` at a capacity of zero returned the slice it was given. It is refused at build now, with a
   compile-fail pair. The finding asked for the refusal on the type, and that was tried first and
   taken back: an accumulator of no terms is what `of` gives for an empty array and what the consumer
   test builds, and `empty`, `plus` and `finish` are all right at zero. The one function that goes
   wrong there carries the refusal.
7. The consumer test found the build directory by counting three levels up from the test binary, and
   its second test passed with paths fixed at build. The directory is now the nearest ancestor
   holding cargo's tag file, and a test runs the binary again under another crate directory and reads
   back what it resolved.
8. `multiplicative_identity` was sampled at phases of denominator one and two. A test walks it at
   thirds, quarters and sevenths in the binary radix and at five phases in the decimal one.
9. The mutants are cargo features in shipping source. Not changed on this branch, because the
   mechanism is shared by every mutant run already committed. Ticket
   `arvo_numeral_carries_its_test_mutants_as_cargo_features_in_s`.
10. The branch added `WithOverflow`, `WithRounding`, the point `Fixed` and `multiplicative_identity`
    to `arvo-format`. That stands, and the coordinator is told in the message that hands the branch
    over, since other branches hold that crate.
11. Two merge commits carry git's default message. They are pushed and are left as they are. Merges
    from here on carry a subject in the repository's form.
12. Test names that asserted more than they said, a duplicated assertion and unused imports in seven
    passing compile-fail halves are fixed.
13. The report claimed two things that did not hold. The release twin of the debug check runs no test
    in the dev profile and six under `--release`, and the report now says so. The `arvo-format` suite
    has one red, `the_inverse_of_three_round_trips_at_every_unsigned_width_from_one_to_63`, which
    `red-check` finds red on `dev` at `4d6aa22cb271` as well, in a file this branch does not touch.
    It is ticket `arvo_format_inverse_round_trip_underflows_at_width_1`.

## Beside the findings

The disassembly sketch `mock/research/sketches/202611130342_value-add-disassembly/` repeats two of
its probes under `Abort` and reports the same instructions as under `Quiet`. The reading pointed out
that this is expected and shows nothing, because the sketch builds in release and the check is a
debug assertion. The two probes show that `Abort` costs nothing in release, and they say nothing
about what a debug build emits.
