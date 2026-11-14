# Review of `work/arvo-format-slot`, pass 1

**Date:** 2026-11-14
**Branch:** `work/arvo-format-slot` at `1ed363f9`, against `work/arvo-format-base` at `22f0c963`.
**Reviewer:** `pr-reviewer-senior` on opus, fresh, one pass. It was stopped partway so the base's second
review could run first, and resumed on the moved tip with the range widened to the merge and round
`202611140345`.

The report as it came back, verbatim.

## Review of `work/arvo-format-slot`, range `22f0c963..1ed363f9` (rounds `202611140239`, merge `b5f2e83b`, round `202611140345`)

Canon gate: passed. I checked the work against `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` (ratified, experts). It says every operation "is a function of the declared width and never of the machine carrier", so a refusal because the `i128` index cannot hold a product is the carrier-shaped condition that ruling dissolves. I also checked `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule` and `ruling::never_a_runtime_check_and_one_lowered_path`. `question::one_word_or_two_for_is_a_number_system` (open) is not filled, because `Slot` stays an `i128`. My read was not blind: I had read the doc changelist's citations first, so it confirms the two blind seats (PLAN, READ) rather than corroborating them.

Test gate. The full suite is green at `1ed363f9`: lib 324, compile_fail 31; ieee754 binary 14, decimal 5, underflow 7; matlab fi 15, fimath 268, ties alias 10; doctests 4; two further binaries, 31 and 5. `cargo mock --lint-only` passed. Findings 2 and 4 are the gate's problems.

### Verdict

MERGE AFTER FIXES. The arithmetic is correct. What needs fixing is a cost regression on the hottest path, a map law that was only tested over part of its range, and stale prose about the removed mechanism.

### Findings

1. blocker `src/apply.rs:220-221` [correctness/drift]. An in-range position now goes through `reduced(lo, span)`. That is `Wide::residue` to `floor_divided`, meaning four 128-bit divisions in `by_one_limb`, or Knuth D for a span of `2^64`, plus two `rem_euclid`. Before this range the in-range path was `Slot::at(down + step)`. Every `add`/`mul`/`div`/`convert` result inside its range now pays a long division. Scenario: any `adapt` call on an in-range position. It contradicts the comment at `:209-212`, which says the in-range case is "the identity... rather than one pass that always touches the value". Cost: unpriced; no bench in `mock/benches/` covers `arvo-format`. Fix: in range, `down` lies in `[lo-1, hi]`, so narrow `down` with `index()` and add the step, with `i128::MIN` for the one case `down = MIN-1, up`. Add a test for that case.
2. blocker `src/apply/tests/the_wide_oracle_sweep.rs:9-18,148-155` and `the_oracle.rs:47-55` [tests]. The map's law against the independent oracle is only fed floors up to `2^127 + 2^63`, which is as far as `Exact::between` reaches. The oracle's `Wide` (`hi: i128, lo`) cannot even represent floors past about `2^191`. What goes unchecked: `lands_within`, `reduced` and saturation at floors from `2^192` to `2^256 - 1` are checked by nothing independent. Only hand cases exist: the `UFixed<64,128>` corner and `the_far_quotient.rs`. Fix: widen the oracle to four limbs and feed `Exact::floored` at `±2^128`, `±2^192`, `±2^255` and `±(2^256-1)` into every range.
3. should-fix `src/addition.rs:95-97`, `DESIGN.md.tmpl:1700` [drift]. Addition keeps a carried refusal whose stated reason ("not one a slot index can hold") is the premise this round removed from the other three steps, under the same ruling. It is excluded as "left for its own round" (src changelist, NO CHANGE section), but carries no `FIXME(<ticket>)` and no ticket I could find. Fix: add the marker and file the ticket.
4. should-fix, expected values formed by the code under test [tests]. `src/apply/tests/the_edges.rs:173,176` asserts `floor() == Wide::of_sum(..)`, and `between` is `of_sum`. `the_step.rs:201-204` builds its expectations with `Wide::times`. `wide/tests.rs` has no direct test of `of_sum`, `residue`, `end` or `is_even`. Add literal-limb laws: `of_sum(MAX,MAX)=2^128-2`, `of_sum(MIN,MIN)=-2^128`, `residue(-(2^256-1), 2^64)=1`, `residue(_, 1)=0`, `is_even` on a negative odd value.
5. should-fix, stale prose about the removed `past` distance, pinning, or wrap-only admission [drift]. `src/apply/tests/the_oracle.rs:8-9,405`: "a pinned slot, a step and a distance". `src/apply/tests/the_cross_end_saturation.rs:39-43`: `past = -2` and the old three-part reduction; same answer, dead derivation. `src/multiplication.rs:172-174` and `src/conversion.rs:156-158`: "a floor that leaves the index is past it... the obligation has ruled both out". `src/conversion.rs:107-108`: "past the index" should read "not formed". `src/multiplication/tests/the_wrap_by_every_mode.rs:14`: "the region the wrapping admission covers".
6. nit `src/wide.rs` [rules]: went from 492 to 530 lines in this range. The seam is the long-division helpers (`by_one_limb`, `by_limbs`).
7. nit `b5f2e83b` [rules]: the merge commit keeps git's default "Merge branch ..." subject and the `# Conflicts:` block. Every other commit is clean: no bylines, no tool adverts, all under 72 characters.
8. nit `README.md.tmpl:36-37` [public-surface]: the line is left ragged, and "four 64-bit limbs" sits beside "no machine container anywhere in this crate".

### Verified

- Q1: the design follows from the cited rows, the topic's reading of design `:885-926` and the step section, and nothing ratified forbids it.
- Q2: the source matches both locked changelists. `is_multipliable_under`, `MultipliableUnder`, `wrapped`, `is_held`, `wraps` and the `*_is_carried` functions are all gone, with zero hits outside `design_rounds/`.
- Q3 arithmetic: `of_sum` (`wide.rs:66-72`): the same-sign carry reaches limb 2, the opposite-sign case cannot overflow, and `MIN+MIN` gives `2^128`. `residue` (`:87-90`): a floored remainder below the span, read from two limbs. `end` (`:77`). `lands_within` (`apply.rs:85-95`): exact on both sides, including `lo-1` with a step up, at `lo = i128::MIN`. `reduced` (`:108-111`): `(-span, span]`, cannot overflow, correct modulo the span at every floor traced; for example `-(2^256-1)` modulo `2^64` is 1. Saturation (`:233-238`). A formed step's floor is at most `2^255` when the denominator is at least 2, and the remainder is zero when it is 1, so floor plus step fits. The round-345 edges are correct: 128/129 and -255/-256 for 64 bits, 130/131 for 63 bits.
- Q4: the broken maps are real. The wide sweep asserts all 17 maps are reported and that the shipped map passes. `the_far_quotient.rs` catches the narrowed parity on its negative side.
- Merge and round 345: the base's crossed sweep, `tests/ieee754/mod.rs` and `ieee754_underflow_parity.rs` are byte-identical to `22f0c963`. The boundary test is ported to `is_multipliable` with each refusal as its control. Nothing of the fourth condition survives apart from findings 3 and 5.
- Public surface: the root README and `LICENSE` are untouched. The `Cargo.toml` description is unchanged and still true. `docs/FORMAT.md` is regenerated and agrees with the template where spot-checked. There are no em-dashes in added lines.

### Notes

`the-public-surface-is-checked-on-every-pr.md` is not in the worktree's `.claude/rules/`; the reviewer worked from the workspace card's own words. Its `cargo mock --lint-only` run created an ignored `target/mockspace/` inside the review worktree, the tool building outside the shared target, which wants a ticket.

## The author's reading

Findings 1 to 6 and 8 stand and are answered in round `202611140402`. Finding 1 needs a bench before its cost is a number, and the fix does not wait on one: the in-range answer is the floor plus the step, as the comment says, and the bench ticket `bench_arvo_format_adapt_before_and_after_the_four_limb_floor` already stands. Finding 3's ticket exists, `arvo_addition_s_sum_past_the_slot_index_could_ride_the_four`, filed with round `239`; the marker naming it was missing. Finding 7 is pushed history and the branch is not rewritten.
