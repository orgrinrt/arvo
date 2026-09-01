# Would the suite have caught it

An ad-hoc quick spike, nothing timed, and it answers one question: does the suite
this round adds actually fail when the magnitude search is removed, or does it
only agree with the implementation it was written beside.

## The mutation

`has_additive_identity` in `mock/crates/arvo-format/src/format.rs`, one line, with
everything else left as it is:

```rust
// shipped
let bound = if magnitudes < MAGNITUDE_SEARCH_BOUND { magnitudes } else { MAGNITUDE_SEARCH_BOUND };

// mutant: one magnitude instead of the admitted range
let bound = if magnitudes < 1 { magnitudes } else { 1 };
```

That is the behaviour the round replaced, on the axis the round is about: the
predicate answers at magnitude zero and never looks higher. The mutant was written
into the working tree, run, and restored with `git checkout HEAD -- <path>`, so it
is not in any commit and its content is above rather than in a file.

## The run

`cargo test -p arvo-format --lib`

```
test result: FAILED. 77 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out

failures:
    tests::identity::a_fractional_phase_that_becomes_whole_higher_up_keeps_the_identity
    tests::identity::a_phase_no_power_of_the_radix_cancels_has_no_identity_in_any_family
    tests::identity::a_whole_phase_out_of_reach_low_down_is_found_at_a_higher_magnitude
    tests::identity::an_extreme_phase_still_answers_at_every_magnitude
    tests::identity::the_identity_answer_is_membership_asked_at_zero
    tests::identity::the_identity_is_decidable_at_const_time_including_the_search
    tests::identity::the_identity_law_holds_at_a_radix_other_than_two
    tests::identity::the_magnitude_the_identity_is_found_at_is_not_always_the_first
```

Against the shipped function the same command is `85 passed; 0 failed`.

## What it establishes, and the part that matters more

Eight arms fail, so the suite is measuring the search rather than agreeing with it.

**None of the three arms this suite already had is among them.**
`a_zero_phase_puts_the_additive_identity_on_the_grid`,
`a_fractional_phase_takes_the_additive_identity_off_the_grid` and
`a_whole_multiple_phase_keeps_the_identity_at_a_shifted_slot` all pass under the
mutant, and they were the whole of what the previous round left behind. So the
defect was not merely unfixed, it was **invisible**: every phase arm in the file
ran in the constant family, where there is one magnitude and the search has
nothing to search.

That is the same shape as the gap the previous round closed one axis over. That
round found every phase tried was fractional because the numerator was always odd;
this one finds every phase tried sat in the family where the magnitude cannot
matter. Both come from the same cause, which is that `Biased` is the only shipped
point with a phase and it pins three coordinates.

## What it does not establish

It cuts one axis. It says nothing about whether the suite would catch a wrong
cancelling slot, a wrong bound, or a wrong obligation, each of which has its own
arms and its own controls and none of which this run varied.
