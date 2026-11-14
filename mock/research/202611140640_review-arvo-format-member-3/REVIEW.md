# Review of `work/arvo-format-member`, pass 3

**Date:** 2026-11-14
**Branch:** `work/arvo-format-member` at `cc3a272c`, range `origin/dev...cc3a272c` (rounds `202611140438`, `202611140537`, `202611140603`).
**Reviewer:** `pr-reviewer-senior` on opus, fresh, one pass, on a detached worktree of the tip, its canon reading formed before reading the earlier reviews.

The report as it came back, verbatim.

The canon gate passes. The test gate fails on one sampled law. The fix for it is bounded, so the verdict below is merge after fixes, not a rethink.

## 1. Canon reading

I formed this from the registry before opening `mock/research/` or any round topic file. Rows are cited by slug and were queried with `cargo mock query`.

- `ruling::never_a_runtime_check_and_one_lowered_path` (ratified, op). No door classifies a value at run time: `Member::constant` and every `Divisor::member` condition are const assertions over types; `Member::adapted` is the total map. Aligned.
- `ruling::the_ingest_door_waits_on_a_known_use` (stated): "the typestate is the default answer". `Member<F>` is that typestate and adds no ingest door. The signed-divisor gap is correctly held back and not filled. It is carried as `FIXME(may_arvo_divide_by_a_signed_divisor_read_at_run_time_of_eith)` at `division.rs:116`, with an op-owned ticket. That ownership is right: the never-a-runtime-check row makes the escape his alone.
- `ruling::the_format_spine_is_canon` and `ruling::a_format_datum_is_one_point_and_a_saturating_end_is_one_point` (both ratified). Membership is one predicate, and floats are points; a datum is a member of its representable set. So a `Member` that refuses floats is legitimate only as a marked not-yet, never as a principle (finding 2). The datum row explicitly leaves "how many value types there are or in which crate" undecided. `Member` is a slot refined by its range: it has no arithmetic, strategy or numeral family. So it fills neither `ruling::his_instinct_on_one_family_is_not_to_be_acted_on` nor `ruling::the_family_question_wants_the_comparison_first` (both open).
- `ruling::the_operating_constraints_are_intents_and_rules` (in force): no heap, no `dyn`, const sizes. Held.
- The proposal rows are not canon. Both new rows are `one_expert`, and neither contradicts a ratified row.

Result: aligned. On the two-expert rule: pass 1's read was independent, and pass 2's was a confirmation. Mine was formed before reading either, so it is the second independent agreement. Pass 1 read an earlier tip, but the changes since (the one-quantum grid and the signed gap) do not touch the runtime-check or typestate question.

## 2. Verdict

MERGE AFTER FIXES.

## 3. Findings

1. `src/division/tests/the_member_door.rs:282-301`, the table, against `src/division.rs:197-216` [tests, sampled law, blocker]. The verdict table, the sweeps and the compile-fail pairs together cannot tell `one_quantum` and `same_phase` from two plausible wrong versions.
   - Exponent sample. Every row uses exponent 0, -1 or -2, so `U = r^max(e,0)` is 1 on both sides of every pair. A `one_quantum` that compares only `(f.down, g.down)` gives the same answer on every row, all nine sweeps, and `a_member_divisor_from_another_exponent_is_refused.rs` (-1 against 0).
   - Phase sample. The only phase-false row (`Byte, Biased`) and the compile-fail pair (`1/2` against `0`) differ by a non-integer. A `same_phase` that compares phases modulo one slot (`f.d == g.d && (f.n - g.n) % f.d == 0`) passes too. That is the natural "same grid as a set of points" mistake.
   - Concrete failure under either mutant: `Divisor::<Byte>::member(Member<Grid<.., Constant<1>, OneTo255, 0, 1>>)` is admitted, where slot 3 is 6 there and 3 in `Byte`; `Divisor::<Byte>::member(Member<FromOne>)` is admitted, where slot `b` is `b + 1` there and `b` in `Byte`. Either way the quotient is silently wrong, and nothing in the suite turns red.
   - Fix: add a type like `Doubled = Grid<BinaryRationals, Constant<1>, OneTo255, 0, 1>` and the rows `Byte, Doubled => [false, true, true, true]` and `Byte, FromOne => [true, false, true, true]`. Add a compile-fail pair for each, beside the admitted twin. Add both mutants to `the_table_reports_each_planted_verdict`, next to `NoRadix`. The shipped verdicts already answer these rows correctly. The mutants each produce two disagreements against them.

2. `src/member.rs:55-63` [rules, spec]. The refusal over a magnitude-indexed quantum is worded as a principle: "a member holds the slot alone. Declare the format over a constant quantum". The design (`DESIGN.md.tmpl`, member section) calls it a not-yet: "a floating format has no member yet". The canon rows above make a float datum a member. Failure scenario: a reader takes the message as a design law, and the type stays slot-only for good. Fix: mark it `FIXME(<ticket>)`, as `division.rs:323` does. I found no ticket for "a member has no magnitude coordinate" in `tickets-arvo.toml`, so file one. Reword the message to "not yet admitted", matching the quotient's own.

3. The commits that would land [public surface, blocker if merged rather than squashed]. `origin/dev..cc3a272c` is 110 commits, 97 of them on the first-parent line. Leaking workflow: `c0200378` "answer the member review…" and `02f2525a` "…, the member review", which pass 2 already named; from the slot branch, `07eabfbc` "fix: answer the slot review…", "docs: correct the wrapping range and the sweeps the review named", and "docs: record the author's reading of the base review". The slot branch's commits duplicate what `dev` already holds as linear squash commits (`280e3a2a` and below). Fix: land it the way `dev` is shaped, as one squash commit with a clean `feat:` title. Otherwise reword every subject listed.

4. `src/division.rs` [drift]. The file went from 426 to 559 lines on this branch. The lint passes because it counts code lines (356), but the workspace tripwire is 500 authored lines. The seam is the member door: `MemberDivisor`, `member_conditions` and its four verdicts, about 110 lines, can move to their own submodule. Low severity.

## Verified

- Tests: lib 344 passed; `compile_fail` 35 passed; every other target green; clippy with `-D warnings` clean; `cargo mock --lint-only` all lints pass, and of the 46 warnings none falls on a changed line; the worktree clean afterwards.
- Pass-2 findings: the stderr is re-blessed; the grid is one quantum; the signed gap is stated, with a FIXME and a ticket; the signed sweeps for both sides are present.
- The member's perimeter: the field is private; `Copy` is a plain copy; there is no `Default` or `From`; `constant` forces `Slots::ADMITTED` through `slot_in_range`; `adapted` forces it through `adapt`.
- Divisor math: `U_F V_G = U_G V_F` is the right quantum equality; the zero check reads `G`; `-n` is bounded by `Reduced` (at most `2^63`).
- Compile-fail pairs: each differs from its twin by one token, and each stderr names its own assertion.
- Generated docs: `docs/FORMAT.md` carries both new sections. No em-dash or bold was added. The root README does not describe the divisor doors, so it is not stale.

## Notes

- The public-surface checklist file is not in this worktree's `.claude/rules/`. I worked from the workspace card and the `pr-review` skill's `reference/public-surface.md`.
- The brief named `mock/mockspace.toml`; the file is at the root, `mockspace.toml:31`.
- Possibly ambiguous canon call, handed back rather than decided: `Member::constant` takes a bare `const B: i128`. The same pattern is already in `Divisor::constant`. Whether a const generic parameter position counts as a public API position under the in-force primitives rule is not settled by any row I found.
- The registry gains two proposal rows, and the first is born superseded by the second. That is acceptable as audit trail.
- The sketch `research/sketches/the-member-divisor` still refuses radix ten at exponent zero, which the shipped door now admits. It proved only that the door can be built, so nothing depends on its stale radix rule.
