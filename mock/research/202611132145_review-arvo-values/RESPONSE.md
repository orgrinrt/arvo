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

## The second reading

`PASS_2.md` read the branch at `a303d07b` and refused it again, on two points about the canon and two
about the suite. Round 202611132216-pzi4 took all four. Points 5 and 6 above describe the
accumulator as it stood after the first reading, and the first item here replaces them.

1. `Accumulator` is the first option of `question::which_chain_carrier_ships`, which is open with
   the panel as its decider. The first reading called that ambiguous and the second misaligned, and
   neither found it aligned, so the type is off the branch: its module, tests, compile-fail pairs,
   mutant feature and design section. The source as it stood is at `a303d07b`, and its closed rounds
   and mutant run stay in the tree. The ticket to op is bumped with this.
2. `holds_index` was a public verdict on whether a host integer names a member, against
   `ruling::the_ingest_door_waits_on_a_known_use`. It is private now, and the debug check in
   `from_index` still asks it. Where that check aborts is shown by a test over every 8-bit host
   value. This is the first reading of that row against the function, so the question went to op as
   ticket `is_a_verdict_on_whether_a_host_integer_names_a_member_wanted`.
3. The oracle sweeps stopped at narrow widths. The quotient is now swept at integer widths 13, 32
   and 64 and at fixed-point formats of 16, 32 and 64 bits, the conversion from sources of 32 and 64
   bits over thirteen pairs, and the fractional product at fixed-point formats of 16 and 32 bits, on
   the edge lattice and seeded members. The named divisors at widths 5 to 8 gained an even one. The
   fractional product at 64 bits is left out, since the tests' arithmetic is done in 128 bits and has
   no room to scale that product. Red runs of `mutant-div` and `mutant-convert` against the wide
   sweeps are in `mock/research/sketches/202611132216-pzi4_value-mutants/`.
4. The test that asserted nothing is a branch of the test that reruns the binary.
5. The mutant features: ticket as before.
6. `from_host` and `host` on `USize` and `ISize` beside `from_index` and `index`: ticket
   `usize_and_isize_carry_from_host_and_host_beside_from_index_a`. Not taken here.
7. `252d1ce4` is typed `test:` and changes `src/accumulator.rs`. It is pushed and stays. The file it
   changed is gone.

The reading asked whether the branch `work/numeral-integers` is dead. It is not this track's, and
the question is passed to the coordinator with the branch.

## Beside the findings

The disassembly sketch `mock/research/sketches/202611130342_value-add-disassembly/` repeats two of
its probes under `Abort` and reports the same instructions as under `Quiet`. The reading pointed out
that this is expected and shows nothing, because the sketch builds in release and the check is a
debug assertion. The two probes show that `Abort` costs nothing in release, and they say nothing
about what a debug build emits.
