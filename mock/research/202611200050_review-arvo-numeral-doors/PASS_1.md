# Review pass 1, arvo-numeral-doors

**Date:** 2026-11-20
**Reviewed:** `work/arvo-numeral-doors` at `51ef3a67`, by `pr-reviewer-senior`. Its findings are taken up by round `202611200019` (topic `202611200013_topic.the-room-at-its-edge.md`).

## Verdict

MERGE AFTER FIXES. Most of the branch is sound: the arithmetic in `room.rs` is correct, the suite is green, and the mutant turns the suites red. What blocks it is one stale public surface, design text that claims more than the code does, a test sweep that skips half its own domain and never takes the path its doc describes, and a locked round file that cites a bench run that was never committed.

Gates. The canon gate passed. I checked against `ruling::the_format_spine_is_canon` (exact step plus a named adaptation, which `room` keeps) and `ruling::never_a_runtime_check_and_one_lowered_path` (each arm is chosen by `if const`). The canon does not say whether the fast path lives in the format or the value. I am the first read on that, so a second is owed.

Tests. `CARGO_INCREMENTAL=0 cargo mock test` at 51ef3a67 ended "10 tree(s) green" with no FAIL lines. I checked the grep is not silent by counting its 1062 PASS lines. No tautological tests.

## Findings

1. `docs/NUMERAL.md:323` and `:341` [public-surface, blocker]: the generated docs were not regenerated. They still say "The cure is the format's to ship" and "a value cannot yet be divided by a value". `docs/NUMERAL_PROOF.md` is stale the same way. Dev regenerates `docs/` in the same commit as the template (d0161800, 38d78396). Fix: run `cargo mock` and commit `docs/`.

2. `mock/crates/arvo-numeral/DESIGN.md.tmpl:318` [correctness/spec]: "every product at a negative exponent, at every width ... is one 128-bit division" is false.
   - At radix two, `power(2, 128)` overflows, so `POWERS.1` is `Isnt` and `PRODUCT` is false for e ≤ -128 (e ≤ -39 at radix ten).
   - The format admits products down to -255: `tests/common/exponents.rs:19`, and `deep(64, true, 2) == -255` is asserted.
   - The same overclaim is in the doc CL (`202611192051_changelist.doc.lock.md:11-12`), in the topic (lines 69-71) and in the test name at `room.rs:272`. That test asserts only -1 and -64.
   - The `bound` module says it shows each bound "true one exponent inside its edge and false one past it", but the product's real negative edge, -127 and -128, is never tested.
   - Fix: state the bound as `r^-e < 2^128` and add `Sides::<Fixed<64,-127>>::PRODUCT` true and `<Fixed<64,-128>>` false.

3. `DESIGN.md.tmpl:342-344` [spec/drift]:
   - "the canon has no row on one" stopped being true at 51ef3a67. That commit adds `question::what_rounds_a_remainder_s_integer_count` and `question::where_a_remainder_is_admitted_past_one_format_at_a_whole_phase`.
   - The name the design cites, `what_a_remainder_over_a_fractional_arvo_format_is_beside_the`, is a workspace ticket (`.shared/scripts/life/tickets-arvo.toml:1973`). It is not a registry row, and the template renders into a public doc.
   - Fix: cite the two question rows with `{{ reg::question::... }}` so the reference lint checks them. Fix the doc CL line 19 the same way.

4. `mock/design_rounds/202611192051/202611192047_topic...md:128-134` [drift/evidence]:
   - The topic quotes the first run's factors ("four to ten times", "seven to twenty") and says "The first run stays committed beside the second."
   - No such run exists: `results/numeral-ratio/*/` holds only `20261119T224053Z-1dd38711`, and `NOTES.md:33-34` says that run was dirty-tree and not kept.
   - By the evidence rule those numbers are void, so dropping the gcd and the u128 division was never priced against the version it replaced.
   - Fix: add an addendum file naming the claim as void. If the comparison matters, add a gcd/u128 arm to the bench.

5. `tests/the_quotient_by_a_value.rs:32-39, 113` [tests, sampled law]: `UFixed<8, E>` divides only by `Positive` (1 to 127).
   - The unsigned divisors 128 to 255 never go through `Divisor::member`.
   - Lines 80-84 also filter each value to G's range before landing it, so the `Member::adapted` "totality" that the module doc (lines 9-12) and the design rely on is never exercised. That is setup that helps.
   - The test can still fail on a wrong implementation: `oracle::quotient` is independent, and `outputs/arvo_mutant_room.txt` shows it red.
   - Fix: add a 1-to-255 range for the unsigned format. Feed values outside G and expect the oracle's quotient by the landed slot.

6. Wider suites [tests]: no oracle test sits at the room bound's last inside exponent with the extreme operands.
   - The shape suites sample exponents top, 0, -1, -ceil(w/2) and -255.
   - So the u128 arm at its edge is never compared to the oracle: quotient at `UFixed<64,-63>` and `Fixed<64,-63>` with a = MIN or MAX and b = ±1, conversion `UFixed<64,0>` to `UFixed<64,-63>`, product at -127.
   - The `bound` module checks only the constants.
   - Roadmap A-new-1a asks for the widest carrier to be included.

7. `mock/benches/numeral-ratio/bench.toml` [drift/bench]: only the 16-bit point is timed, which takes the 64-bit division.
   - The u128 arm (`__udivti3` plus `%`, then a third modulo inside `Remainder::of`, at `arvo-format/src/apply/position.rs:192-199`) is unpriced. `NOTES.md:64-67` says so.
   - The quotient is 9.0x the hand version and the cause is unidentified (`NOTES.md:55-60`): the arm's own cost is never separated from `Member::adapted`.
   - Fix: add a 64-bit sweep point with an i128 hand arm, plus a quotient arm on a pre-landed or constant divisor, plus `cargo mock bench disasm`.

8. Registry rows [spec, minor]:
   - Proposal `a_remainder_is_the_dividend_less...`: its `says` is backed by both seats (fabian-giesen.md:52-84, donald-knuth.md:130-213). Two exceptions:
     - "every divisor door fixes that sign at build" is only in the knuth seat (dk:186).
     - Its `because` calls "306 of 720" a planted twin. It is fabian's comparison (`run.txt:1`); the planted twins are knuth's.
   - `two_experts` is what the vocabulary (`mockspace.toml:1357`) admits: both seats state they were written blind (fg:3, dk:3-4). Two caveats on that standing:
     - Both seats were dispatched on a ticket whose text says "Recommended a" (`tickets-arvo.toml:1978`), so their convergence on "a, corrected" is partly seeded.
     - The knuth seat says a reconciliation "that starts from neither" is owed before any proposal is filed (dk:456). No reconciliation file exists.
   - `question.toml:2162`: "each marking its own side as the call it made without a row" holds for fabian (fg:74). The knuth seat argues the mode reading from the admission rule (dk:196-207) and marks a different call as its own, the landing of the pair's quotient (dk:64-69, 449).
   - `question.toml:2170` option 2 leaves out knuth's dependency: where `q` leaves `T`, the answer waits on `question::whose_reduction_governs_a_lossy_crossing` (dk:449-451).

9. `room.rs` [drift, low]: the value now carries its own exact position step beside the format's. The topic rejected that same shape itself at line 43, then took it at line 107. The argument for it holds only because the format has no position functions over `Member<F>` operands (`member.rs`: only `constant` and `adapted` build a `Member`). The workspace rule `a-crate-holds-only-its-own-thing` points that work at the format; it should be filed as a format ticket rather than left implicit.

## Verified

- `Ratio::exact`: the floor, the residue for negative ratios, and that `-whole - 1` and the u64 narrowing stay in range given the bounds.
- The three ratio formulas against slot algebra. Quantum is `r^e` only (`quantum.rs:352-367`).
- Every `Divisor<F>` under PLAIN is in F's range. `constant` and `member` check this at build, and `of` is refused at a zero phase (`division.rs:37-74`, `member_door.rs:154-180`).
- The `bound` tests can fail: each asserts a constant against a hand-derived literal, and swapping `u` and `v` flips them.
- The ui stderr: the refusal and message are unchanged, and one note goes because `arvo_format::mul` is no longer instantiated. The forced `product_position` const stays at `num.rs`.
- The mutant sketch has Outcome WORKS with the red run committed. `run.sh` change is sound.
- No em- or en-dashes in added lines (checked with a positive control), and no bold in templates or rustdoc. Commit subjects are all under 72 characters with no bylines or trailers.
- The src CL matches the source, apart from finding 2.
- README and `Cargo.toml` are not stale.

## Notes

Every scratch file I wrote is outside the repository and is not evidence.
