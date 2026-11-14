# Review of `work/arvo-format-base`, pass 1

**Date:** 2026-11-14
**Branch:** `work/arvo-format-base` at `5948a757`, against `origin/dev`.
**Reviewer:** `pr-reviewer-senior` on opus, fresh, one pass.

The report as it came back, verbatim.

Canon gate: passed, with nothing misaligned. I checked it against `ruling::never_a_runtime_check_and_one_lowered_path`, `ruling::the_overflow_panic_is_permitted_and_bounded` and `ruling::a_format_datum_is_one_point_and_a_saturating_end_is_one_point` (`mock/registry/ruling.toml`). No ratified row forbids admitting the wrapping product, and `wraps::<S>()` is a const branch. I am the first read, so a second independent read is owed.

## Verdict

**DO NOT MERGE.** The wrapping round `202611140127` admits formats that every policy refused before this branch, and on some of them it returns the wrong member or panics. Its tests cannot see either problem, because every input they feed has a zero remainder. The IEEE round `202611140156` is sound apart from minor points. I did not re-review the six carried rounds beyond reading them in passing.

Test gate: the full suite is green (`cargo test -p arvo-format --no-fail-fast`, exit 0: lib 311, compile_fail 31, underflow 7, and the rest). `cargo mock --lint-only` reports "all lints passed". That green says nothing about the wrapping round.

## Findings

1. **`src/placing.rs:177-195` with `src/apply.rs:155-183` [correctness]: the wrapping product rounds the wrong slot.**
   - **What goes wrong.** `wrapped` hands `adapt` the reduced slot `lo + residue`. `round_slot` then reads that slot's sign (`TowardZero`) and parity (`HalfEven`), not the true floor's.
   - **The tree already warned about this.** `apply.rs:121-124` says these two modes "read the sign and parity of the slot the position names, which past the index is not the slot it pins at", and `DESIGN.md.tmpl:923` says the same. The topic (lines 28-30), the design text added at about line 2066 and the body of commit `53d41c81` all assert "wrapping reads only the floor modulo the range's count". That is false, and it contradicts both of those passages.
   - **`TowardZero` reproduction.** Format: `Grid<BinaryRationals, Constant<3>, Signed<64>, 1, 2>` under `Adapt<TowardZero, Wrap>`, with a = 2^63-1 and b = -3·2^61.
     - The true position is negative and sits on a tie: P - 1/2, with P ≡ 2^63-2 mod 2^64.
     - The expected member is wrap(P) = 9223372036854775806. `mul` returns 9223372036854775805.
     - Control: the held pair (1, -1) gives the correct -6.
   - **`HalfEven` reproduction, on an odd count.** The `Slots` trait is open. The range is `[i128::MAX-198, i128::MAX]`, the format `Constant<-1>` with zero phase, and the expected values were computed with `bc`:
     - (MAX, MAX-12): `mul` gives ...105556, expected ...105555.
     - (MAX, MAX-10): `mul` gives ...105532, expected ...105533.
   - **Fix, in a new round.** Reduce the floor modulo `2·count` (at most 2^65, so it fits the index). Take the representative in `[0, 2count)` when the floor is non-negative and in `[-2count, 0)` when it is negative. That keeps the sign, the parity and the class modulo `count`, and `complete_slot` wraps the rest. It also removes the `-lo` in finding 2. Correct the false sentence in the design and the topic.

2. **`src/placing.rs:184` [correctness]: `-lo` overflows when a range starts at `i128::MIN`.**
   - With `BottomOf200` (MIN = `i128::MIN`, a shape the crate's own tests admit) under `Adapt<Floor, Wrap>`, `mul(MIN, MIN)` panics in a debug build with "attempt to negate with overflow".
   - In release `-lo` wraps, so the shift is off by 2^128. From my arithmetic the answer is then off by 56 modulo 200; I did not run a release build.
   - This makes the doc at `multiplication.rs:311-312` false: "Total over every pair of slots, with no check and no panic".

3. **`src/multiplication/tests/the_wrap_past_the_index.rs:191-239` [tests]: the law is sampled over one corner of what was admitted.**
   - **What is tested.** One mode (`TowardZero`), one range (`UFixed<64, K>`, zero phase, K = 0, 3, 63). With zero phase the remainder is always zero, so the rounding half of the topic's claim (line 28) is never exercised.
   - **What was admitted.** `MultipliableUnder` (`multiplication.rs:121-136`) admits every formed format under wrapping: signed ranges at positive exponents, phased grids, and foreign ranges at either end of the index.
   - **What was asked.** The ticket asked for "the minimal admission, nothing beyond", and that is the `UFixed<64, k>` case.
   - **Result.** Both defects above sit in the region nobody chose to test.
   - **Fix.** Add a sweep over all six modes, wrapping only, with a nonzero phase, signed and unsigned ranges, both ends of the index, and odd and even counts. Check it against a 256-bit reference that reads no step code.

4. **`multiplication.rs:367` [drift]: `MultipliableUnder<S>` has no question that can be asked without forcing it.**
   - `is_multipliable::<UFixed<64,0>>()` says false while `mul` under `Wrap` builds.
   - Every other obligation in this crate has such a mirror. Add `is_multipliable_under::<S>()`, or a design line saying why there is none.

5. **`tests/ui/multiplication_saturating_past_the_index_is_refused.rs` [tests]: only `Saturate` is shown refused.** `Clamp` is the other policy that does not wrap. Add a `Clamp` twin.

6. **`tests/ieee754/mod.rs:116-118` [correctness, test oracle]: the comment's bound is not enough.** "`2|c|` at most `2^128` and the unit past that" leaves the tie at `c = i128::MIN`, radix 2, drop 128, where `TiesToAway` answers 0 instead of -1. No current input reaches it. Assert `c != i128::MIN` in `round_off`, or handle the tie.

7. **IEEE draws [tests, redundant]: most random draws take the stand-in branch.**
   - For binary64, about 70% of the 500 draws (q below qmin-128) hand the identical stand-in remainder: one class asserted roughly 350 times.
   - At radix 10 the exact `u128`-only branch of `position` is unreachable at `den = 1`, because 10^38 < 2^127 < 2^128 < 10^39. So the design's "where the scaled denominator fits the unsigned 128-bit integer the remainder is exact" is empty for decimal.
   - Decimal64's underflow inputs also run twice, through `inputs` and through the underflow suite.
   - Fix: narrow the draws to qmin-128..qmin-21 and say in the design that the exact `u128` region is binary only.

8. **`README.md.tmpl` [public-surface]: the summary sentence is wrong and badly wrapped.**
   - "Each has a parity suite ... from far below the smallest subnormal" grammatically covers MATLAB `fi` too, and `fi` has no subnormal.
   - The edited line is left unwrapped. This file is the crates.io readme (`Cargo.toml` `readme`). Rewrite it so the claim applies to IEEE 754 only.

9. **Commit `c920748f` [rules]: the body leaks workflow.** It reads "Carried from d0efc772 on work/arvo-format, whose type was perf", which names another branch and sha in published history. Every other commit is clean: no bylines, no adverts, subjects under 72 characters.

## Verified

- **Q1, rounds against their changelists:**
  - Round `127`'s source matches its locked src changelist. Its failing control is real: `wrapped` answering the saturated end turns both tests red.
  - Round `156`'s source matches its changelist, and both claimed controls reproduce:
    - The search answering nothing below the index: 6 of 7 tests red.
    - The old unit in `round_off`: the binary32 and binary64 sweeps red, `[4,0,0,0]` each.
  - `docs/FORMAT.md` was regenerated after the last template edit.
  - The design describes what the code does, except the false sentence in finding 1.
- **Q3, the IEEE stand-in:** the remainder `position` hands back is sound for every mode the suites run (Floor, Ceil, TowardZero, HalfEven, and the ties-away alias through HalfUp/Floor with `is_tie`). The floor stays 0 or -1, so sign and parity are kept, and no position past `u128` is a tie. It would not hold for `Stochastic`, which no suite runs. The digit rule is right at the first unit past the index: `rest` against `unit - rest`.
- **No toolchain or version drift:** no `Cargo.toml` or toolchain pin moved.
- **The compile-fail pair** for `Saturate` against `Wrap` differs only in the policy.

## Notes

The IEEE round and the carried rounds could land on their own. Round `127` needs a new round: the fix in finding 1, the sweep in finding 3, a corrected design sentence and commit text, and then a re-review of that round only.

My spike tests (the `TowardZero`, `HalfEven` and `BottomOf200` cases) are in a scratch copy of the tree, not in the repo: `/tmp/claude-0/-home-user/7c35e2ad-fb65-5898-963d-96f2bad980fd/scratchpad/rb/mock/crates/arvo-format/src/multiplication/tests/spike_wrap.rs`. By the evidence rule they count only once the fixer commits them as the failing tests that come first. The inputs and expected values above are enough to rewrite them.

What I could not check: release-mode behaviour (finding 2 is arithmetic, not a run), and anything in the carried rounds or the new lints beyond a passing read.

## The author's reading

Round `202611140209` answers the findings, at `a051a587` and closed at `600bfac6`.

Findings 1 and 2: `placing::wrapped` reduces the floor modulo twice the count on its own side of zero and forms the count without negating `S::MIN`. The reviewer's three spike cases are committed as tests in `src/multiplication/tests/the_wrap_by_every_mode.rs`, each red on the tree before the fix with the values the reviewer gave.

Finding 3: the same file sweeps all six modes and three dithers over five formats (a phase of a half, a third and a quarter, signed and unsigned, ranges at both ends of the index, odd and even counts) against a reference in `Big` that reads no step code, with a control showing the reference separates the modes.

Finding 4: `is_multipliable_under`, tested on both sides of the policy boundary and on a step that leaves the limbs.

Finding 5: the clamping compile-fail case beside the saturating one.

Finding 6: `round_off` refuses a significand of `-2^127`. Finding 7: the underflow draws start one past the first unit past the index, and the duplicate decimal64 sweep is dropped with its pinned point and control kept. Finding 8: the README sentence is said of the IEEE 754 suites only.

Finding 9: `c920748f`'s body names another branch and sha. It is pushed and is not rewritten; the coordinator is told.

The coordinator's call on `388c088fee6c` is in the same round: the remainder's two `Maybe<u128>` doors are removed. The thirteen other bare coordinate reads are task `5ce2bb7e67a4`.
