# Review of `work/arvo-format-member`, pass 2

**Date:** 2026-11-14
**Branch:** `work/arvo-format-member` at `68f70d7e`, range `origin/work/arvo-format-slot..68f70d7e` (rounds `202611140438` and `202611140537`).
**Reviewer:** `pr-reviewer-senior` on opus, fresh, one pass, on a detached worktree of the tip.

The report as it came back, verbatim.

Canon gate passed, but my read is not independent and does not count as the second agreement (details under "Canon gate"). The test gate failed: the compile-fail target is red at the tip.

## Verdict

MERGE AFTER FIXES. The branch cannot land as it stands, because `cargo test -p arvo-format --test compile_fail` fails: 34 passed, 1 failed. Round `202611140537` locked its source changelist and closed on a suite that does not pass. A locked changelist is a claim about the source, and this one does not match it. The fixes are bounded. None of them needs a rethink.

## Findings

1. `tests/ui/a_member_over_a_magnitude_indexed_quantum_is_refused.stderr` [tests, blocker]. The expected output says `:33:13` for `MEMBER.slot()`. The `.rs` file has that call on line 32, so trybuild reports a mismatch.
   - Both files arrived in `d4026518`, so the test has been red since it was added. The compile-fail target was not run before the lock.
   - Fix: re-bless with `TRYBUILD=overwrite`, check that the diff touches only the line numbers, then re-run the whole `compile_fail` target.

2. `src/division.rs:181` and `:145-147`, `DESIGN.md.tmpl:2171-2172`, `proposal-the-exact-operations.toml:45` [correctness, spec]. "On the grid" is defined as same radix, same exponent and same reduced phase. That is enough for slot `b` to denote the same value in both formats, but more than is needed.
   - At exponent 0 the quantum is 1 whatever the radix, so the two grids are the same. Radix 4 at exponent 1 and radix 2 at exponent 2 likewise share the step 4.
   - `tests/ui/a_member_divisor_from_another_radix_is_refused.rs` pins exactly this case: `Member::<G>::constant::<7>()` with `G` decimal at `Constant<0>`, as a divisor of `UFixed<8, 0>`. Slot 7 is 7 in both formats. The refusal message then says "its slot names a value off the divisor format's grid", which is false here.
   - The table row `Byte, Decimal` at `the_member_door.rs:247` makes the same mistake.
   - `ruling::validate_means_all_three_readings` (stated) asks for a readable diagnostic on usage, and a diagnostic that states a falsehood is not one.
   - Fix, either way: compare the quantum `r^e` as a rational together with the phase, or keep the radix condition and say it is refused by intent. In both cases, move the radix pair and table row to a shape whose grids really differ, such as exponent -1. Correct the design sentence and the proposal's `says` to match.

3. `DESIGN.md.tmpl:2183-2185`, the proposal `note` at `:48-51`, and the round 202611140438 topic, line 26 [drift]. These records say the compromise is removed. It is not.
   - A slot range is one contiguous interval. A zero-free range at a whole phase therefore lies entirely on one side of `-n`.
   - So `Integer<8>` still cannot divide by a non-zero signed divisor read at run time when that divisor may be either sign, which is the ordinary `a / b`. Only constants reach that case, which is the old compromise in a narrower form.
   - Fix: state the remaining gap in the design and in the row, and track it with a ticket and a `FIXME(<ticket>)` where the gap sits. The door that would close it is the design's to choose, not mine.

4. `src/division/tests/the_member_door.rs:149-177` and types `:63-88` [tests, sampled law]. Every swept dividend format is unsigned or a phase-offset unsigned range, and every divisor range is positive.
   - The case the design names first, "a plain integer range", is never swept: a signed `F`, a negative dividend, or a negative member divisor.
   - That means the `n + b d < 0` negation is never reached through this door. The zero-column search never sees a negative zero-free range either.
   - Fix: add `Integer<8>` divided by members of 1..127 and of -128..-1. Add both formats to the table and to `the_zero_column_agrees_with_a_search_over_every_member`.

5. Scope [process]. `git diff origin/dev...68f70d7e` is 102 commits and 4954 lines, not 16 commits.
   - The member branch contains `origin/work/arvo-format-slot` at `6f80ce21`, and that branch is not on `dev`.
   - The subject of `aa6ed505`, "merge the slot branch, now on dev", is false.
   - Landing this branch lands the slot branch. I reviewed only `origin/work/arvo-format-slot..68f70d7e`. Before landing, either land the slot branch on its own closed review, or confirm that its review closed.

6. Commit subjects `c0200378` ("answer the member review…") and `02f2525a` ("…, the member review") [public surface]. Both describe how the change was produced, which is workflow leakage, and arvo is public. Reword them before landing if the flow still allows it. Otherwise keep it out of the merge commit.

## Verified

- Lib suite: 341 passed. The other unit targets pass. Clippy with `-D warnings` is clean. `cargo mock --lint-only` passes all lints. The worktree is clean after the runs.
- Pass-one findings are fixed in source: the constant door at both ends at every width 1 to 64 in both families, with the literal list checked against `ADMITTED_WIDTHS`; the compile-fail pairs on both sides of the range; the sweep taking the divisor's value from `G` and asserting it equal to `F`'s; the zero column checked against an independent search; `MemberOf` forcing the format's `ADMITTED` and a constant quantum; the superseding proposal row; the three rustdoc summaries split.
- The refusal at each end of the range delegates to `slot_in_range`. `tests/the_predicate.rs:89-112` tests that function at every width on both sides of both ends.
- The guarantee holds over the surface a caller can observe: the field is private, `Copy` is a plain copy, there is no `Default`, and the policy `match` in `complete_slot` is exhaustive.
- No em-dashes, no bold and no bylines in the member diff.

## Canon gate

- `ruling::never_a_runtime_check_and_one_lowered_path` (ratified by op): no door classifies a value at run time. `adapted` is the total map, and the other door is checked at build.
- `ruling::the_ingest_door_waits_on_a_known_use` (stated): the member is the typestate answer, and it adds no ingest door.
- `ruling::validate_means_all_three_readings` (stated): this is what finding 2 rests on.
- `question::what_a_datum_stands_for` (answered in part): `Member` is a slot refined by its range, not a datum's denotation, so it fills none of the open part.
- `ruling::wrap_or_clamp_stays_open_and_both_get_priced` (open): this branch does not touch it.

Result: aligned. However, I read pass one's `REVIEW.md` before forming my own canon reading. Under the two-expert rule that makes my read confirmation, not corroboration, so one independent canon read is still owed.

## Notes

- Outside this diff: `Divisor::of`'s rustdoc (`division.rs:251-253`) says a slot outside the range is positioned like any other. The 202611140438 topic and `DESIGN.md.tmpl:2173` say the quotient is not exact there.
- Also outside this diff: `Member::constant` and the existing `Divisor::constant` both take a bare `const B: i128` in a public signature. That is the crate's existing pattern, and it goes against the arvo rule on primitives in public positions.
