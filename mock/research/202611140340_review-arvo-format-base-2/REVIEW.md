# Review of `work/arvo-format-base`, pass 2

**Date:** 2026-11-14
**Branch:** `work/arvo-format-base` at `f384696f`, against `origin/dev` (`3865b39a`).
**Reviewer:** `pr-reviewer-senior` on opus, fresh, one pass, the second of the branch.

The report as it came back, verbatim. Its spike was a scratch copy outside the repo; what it showed is
committed as tests in round `202611140332`, and the scratch path is not named here.

Canon gate passed. I checked it against `ruling::never_a_runtime_check_and_one_lowered_path` (ratified, op), `ruling::the_overflow_panic_is_permitted_and_bounded` and the canon that `mock/crates/arvo-format/DESIGN.md.tmpl:3-22` declares. No ratified or in-force row forbids admitting the wrapping product or carrying the remainder in four limbs, and `wraps::<S>()` is a const branch. I read the canon myself before reading the first pass's verdict, so this is the owed second read, and it agrees. The doc changelists of rounds `202611140127`, `202611140156` and `202611140209` cite no canon row at all, so for those rounds question 1 can only be answered against the design's own declaration.

Test gate: I ran the full suite on the worktree with `cargo test -p arvo-format` and it exits 0 (lib 317, compile_fail 31, binary 14, decimal 5, underflow 6, fi 15, fimath 268, ties-away 10, plus 4 and 5). `cargo mock --lint-only` passes and the worktree is still clean. I found no tautological test. The wrapping sweep does sample its matrix, though (finding 2 below).

## First-pass findings

1. Wrong slot rounded: fixed. `src/placing.rs:180-214` reduces the floor modulo 2·count on its own side of zero. I checked the argument: `round_slot` (`src/apply.rs:132-195`) reads only the remainder, the dither, `is_negative` and `is_even`, and the representative keeps all four plus the class modulo the count.
2. `-lo` overflow: fixed. `placing.rs:185` no longer negates the range's start; it is tested at `the_wrap_by_every_mode.rs:218`.
3. Sweep sampled: partly fixed. See finding 2.
4. No verdict for the signature: fixed (`multiplication.rs:361`, tested at `the_wrap_past_the_index.rs:69-87`).
5. No `Clamp` twin: fixed (the `tests/ui` clamping case, registered in `compile_fail.rs`).
6. Tie at `c = i128::MIN`: fixed (`tests/ieee754/mod.rs:254`).
7. Draws mostly on the stand-in: partly fixed. See finding 3.
8. README sentence: fixed (`README.md.tmpl:47-49`).
9. `c920748f` body: not fixed, by decision. `53d41c81`'s body also still states the false sentence from first-pass finding 1, in pushed history.

## Findings

1. should-fix, `DESIGN.md.tmpl:2029` and `src/multiplication.rs:315-318`. Both claim `UFixed<64, k>` is admitted under wrapping "at every `k`" or "at an exponent at or above zero". It is false from k = 129, where the corner (2^64-1)^2·2^129 passes 2^256. Scenario: `is_multipliable_under::<Signature<UFixed<64,129>, Adapt<Floor,Wrap>>>()` returns false and k = 128 returns true. That is from an uncommitted spike, so it is one instance and not evidence; commit it as a test. Say "0 to 128". The same doc at `:315-316` still says the floor is "reduced modulo the range's count onto the range", the shape round 209 removed.
2. should-fix, `src/multiplication/tests/the_wrap_by_every_mode.rs:55-59`. Each dimension is present but the dimensions are not crossed. Among pairs past the index, these cells are empty: a negative floor with a remainder below one half, and a negative tie on an odd count, which is the cell the `[-2c, 0)` branch exists for. `AtTheBottom` has no inexact position at all, so its eight mode columns assert one thing eight times. Add `OddWide = [-(2^63-1), 2^63-1]` (width 64) at phase 1/2 with `Constant<2>`, and at phase 1/4 with `Constant<3>`. Also add a bottom range at `Constant<-1>` and `Constant<-2>`, zero phase. A phased range at `i128::MIN` is refused as unformed. The spike passes on all of these; the tests are owed in the repo.
3. should-fix, `tests/ieee754/mod.rs:434`. The round-209 src changelist (lines 64-66) says "no draw falls where the search hands its stand-in", and `DESIGN.md.tmpl:1424-1425` says the same. Both are false. The draw range starts at `below - 1`. For binary, that is `qmin - 128`, where 2^128 leaves u128. For decimal, both `qmin-39` and `qmin-40` are past u128. Measured by the spike: binary64 has 4 of 500 draws on the stand-in, decimal64 46 of 500. Fix: start binary at `below`, and decimal at `qmin-38`, or correct the claim. `ieee754_underflow_parity.rs:15-16` says the same thing.
4. should-fix, `src/apply/position.rs:193`. `Remainder::of(num: u128, den: u128)` is the same rule's instance as the two doors round 209 removed. Neither the topic nor task `5ce2bb7e67a4`'s list of thirteen names it; add it to the task.
5. nit, `tests/ieee754/mod.rs:262`. "`2|c|` is at most `2^128`" should now read "below".
6. nit, `the_wrap_past_the_index.rs:84`. It says "a phase whose step leaves the four limbs", but `UFixed<64,-300>` has zero phase; it is the exponent.
7. nit, `src/multiplication.rs:19-24`. The module doc names one obligation; `MultipliableUnder` and `is_multipliable_under` are missing.
8. nit, `mock/research/202611140220_review-arvo-format-base-1/REVIEW.md:80`. It names a scratch path; the evidence rule forbids that, even though the cases are now committed.

## Verified

- `wide.rs` `by_limbs` (Knuth D): the estimate stays under 2^64 after the loop, the add-back is exact, two limbs never borrows, and the remainder un-shifts correctly. The oracle `Big` is independent, and the add-back case is really reached.
- `Remainder` over four limbs: `against_half`, `is_above` and `reduced` are correct.
- Every commit subject is under 72 characters, lowercase, with no parentheses, no em-dashes and no bylines. The only leaked body is `c920748f`.
- Lint-only is green and no emphasis was added in the range.

## Verdict

Merge after fixes. The arithmetic is right as far as I could break it. Findings 1 to 3 are false design and test claims, plus a sweep that does not cover the matrix it claims to cover.

## The author's reading

Every finding stands and is answered in round `202611140332`, except finding 4, which is a ticket and not
this branch's work: task `arvo_format_s_coordinate_reads_return_host_integers` is bumped with
`Remainder::of`. Finding 3 is answered by moving the draws, so the sentence becomes true, rather than by
weakening it. First-pass finding 9 and the same sentence in `53d41c81`'s body stay in pushed history; the
branch lands by a merge and its history is not rewritten.
