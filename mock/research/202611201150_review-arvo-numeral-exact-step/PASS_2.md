# Review of work/arvo-numeral-exact-step-clean, pass 2

Reviewed sha `d0a3d0b5` against `origin/dev` `3c78ee81`. I confirmed that `git diff bf922293 d0a3d0b5` is empty. This was a read-only review: nothing was built or run, and nothing was posted.

Canon gate: passed. The work does not contradict `ruling::the_format_spine_is_canon` (ratified, both). `Member::product`, `quotient` and `converted` return an `Exact`, and `adapt` stays the only adaptation, which is the ruling's "exact operation composed with a named total adaptation". It also does not contradict `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`: the arm is chosen from the declared range, and each arm is held equal to the slot step. The arm choice is an `if const`, which fits `ruling::never_a_runtime_check_and_one_lowered_path`. On the stamp that puts the step in `arvo-format`, I found no row that contradicts it. My agreement is not blind, though: the brief handed me pass 1's gate before I wrote mine, so it does not count as the second independent read. That read is still owed.

I read the registry with grep, not `cargo mock query`. `cargo mock` regenerates files on every run, and this review was read-only.

Test gate: I did not run the suite. The brief forbids it, and `cargo mock test` writes the tracked `mock/test-history/`. The helper's figure of 1121/1122 passing, with an arvo-bits ui flake, is unverified. I read every test body in `member/step/tests.rs`, `the_quotient_by_a_value.rs`, `the_conversion_at_the_room_edge.rs` and `common/edges.rs`. Finding 1 is a sampled-law defect.

Commit messages: I read all 33, one at a time. None has a byline, a co-author line or a tool or model name. The four that pass 1 flagged are reworded (`ae06c0a9`, `d0c43424`, `09b750cc`, `0a2b21aa`). Every subject is under 72 characters, except the merge commit `46029f8d`, which keeps git's default text.

## Pass 1 findings

1. The round is open. Resolved: `202611200735/` holds both lock files.
2. The edge matrix was dropped. Resolved for exponents at or below zero: `tests.rs:303-359` covers widths 1, 8, 16, 32, 63 and 64, both ranges and three radixes, with edges in the tests' own arithmetic. The positive side is still not held (finding 1 below).
3. Commit messages. Resolved.
4. The cost of `landed` is unmeasured and the bench results describe deleted code. Not addressed; see finding 2 below.
5. The test name. Resolved: `tests.rs:433-441` now says what the test holds.
6. Decimal and ternary 64-bit corners. Resolved: `tests.rs:204-247`.
7. Doc widths. Resolved.

## Verdict

MERGE AFTER FIXES. Finding 1 blocks.

## Findings

1. `mock/crates/arvo-format/src/member/step.rs:201-206` [tests], blocks. The `u = r^max(e,0)` side of all three bounds is held at no edge, at any width.
   - The matrix starts at exponent 0 and only goes down. The hosted sweeps stop at exponent 2.
   - The quotient's guard, which requires `times(REACH, POWERS.0)` to be `Maybe::Is`, has no test that fails if it is deleted. Take `Fixed<64, 70>`, which `Divisible` admits. Without the guard, `holds(2^63, 2^70)` is true, and `b.unsigned_abs() * UP` at `step.rs:304` overflows `u128`: a panic in debug, a silent wrong quotient in release. No test reaches it.
   - The product's width-dependent edge, `widest^2 r^e < 2^127`, appears only as spot points at 64 bits plus `Fixed<8, 2>` inside it. `HOLDS` from a source at a positive exponent is never at an edge.
   - Fix: extend `at_the_edge!` with the positive side at the same widths, radixes and ranges. Check `PRODUCT` at its last `e` and one past, `QUOTIENT` where `widest * r^e` leaves `u128`, and `HOLDS` from `At<e>` onto `At<0>`. Show the guard's deletion going red in the `202611200745_member-step-mutants` sketch.

2. `mock/research/202611192235_numeral-ratio-bench/NOTES.md` [drift], does not block. This carries forward pass 1 finding 4.
   - The quoted run and its 1.12x and 1.58x factors timed `room.rs`, which no longer exists. The `-room` arms now time `Member::landed` plus the member step.
   - Those numbers are what justified deleting the three `FIXME(arvo_numeral_s_fixed_point_product_goes_through_the_128_bit)` markers. Rounding in `adapt` is still four-limb (`Remainder`/`Exact` hold `Wide`).
   - Fix: add an addendum file saying the run measured deleted code. Rerun `numeral-ratio` before `member_adapted_lands_an_on_grid_slot_through_the_four_limb_a` closes.

3. `mock/crates/arvo-format/src/member/step/tests.rs:11-12` [tests], does not block. The module doc says each sweep first shows its arm. `ring_at_the_corners` (`:172`) does not; labels such as "the quotient's edge" are strings, not assertions. Fix: pass the expected arm per operation and assert it.

4. `mock/research/202611201150_review-arvo-numeral-exact-step/PASS_1.md` [drift], does not block. It cites `7df1e65e`, `72c08840`, `a2426475`, `8fd8fae8` and others, which exist only on the replaced branch. Fix: add a file beside it mapping each old sha to its replay, or keep `work/arvo-numeral-exact-step` on the remote.

5. `mock/crates/arvo-numeral/tests/common/edges.rs:134,144`, `tests/the_conversion_at_the_room_edge.rs`, the `room` shape and the `numeral-ratio-*-room` arms [naming], do not block. "Room" names the deleted `room.rs`; the concept is now the format's host arm. Fix: rename to match the format's terms.

6. `step.rs:181-189` [drift], does not block. `Sides::POWERS` derives `r^max(e,0)` and `r^max(-e,0)` a second time; `Multipliable::STEP` already holds them as `scale` and, at a zero phase, `divisor`. This goes against the workspace rule `extract-the-shared-logic-never-mirror-it`. Fix: derive one from the other.

## Notes

`docs/FORMAT.md` and `docs/NUMERAL.md` carry the new template text. I added up no em-dashes or bold in the added prose, and no version or licence moved.

Commit `6a1f3f2d` also changes `support/numeral-ratio-shared/Cargo.lock`, and its message does not say so. That is cosmetic now that the history has been rewritten.

`complete_index`'s wrap formula and `Ratio::exact`'s negative floor are both right. The floor stays at or above `i128::MIN` because the quotient is below `2^127`.
