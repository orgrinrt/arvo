# Review of work/arvo-format, pass 2

**Date:** 2026-11-14
**Reviewer:** a fresh `pr-reviewer-senior` on opus, worktree detached at `ef0053dd`.
**Range:** `3865b39a..ef0053dd`, the merge base with `origin/dev`.
**Verbatim:** the reviewer's report as returned, kept whole below.

## Report

Canon gate passed for the branch's direction, checked against the registry. One mechanism inside `land` contradicts a ratified row; it is new finding 2. Suite gate: the landing oracle is written to skip the exact region where `land` returns wrong answers (new finding 1). Overall verdict: **MERGE AFTER FIXES**. The ibrf fixes block the merge.

### 1. Canon gate

| Row | Rung | Outcome |
|---|---|---|
| `ruling::the_format_spine_is_canon` | ratified | Aligned on membership and on the exact step composed with an adaptation. One open question: `land` returns `Isnt` at run time on admitted signatures (new finding 3). The spine says the adaptation is "total". I am the first read on whether `land` is part of that adaptation; a second read is owed. |
| `ruling::never_a_runtime_check_and_one_lowered_path` | ratified | Violated by `land` on a format whose phase has a zero denominator (new finding 2). The design at `landing.rs:140` ("Nothing where the phase names no member") states the runtime refusal, so the design is the defect. |
| `ruling::the_operating_constraints_are_intents_and_rules` | in_force | Second read of pass-1 finding 3. My read differs in part from pass 1 (see section 3), so two reads have not settled it. |
| `ruling::wrap_or_clamp_stays_open_and_both_get_priced` | open | Not filled. `land` is generic over the policy. |
| `proposal::wrapping_is_admitted_exactly_where_the_representable_set_is_one_progression` | proposal, stamped by two experts | Usable under the provisional-stamp rule, and `DESIGN.md.tmpl:2288` cites it as provisional. The row's own `gap` says an exhaustive instrument (ticket `990f0e11c61c`) is owed before promotion. The 9-law test is not that instrument. |

### 2. Verdict per closed round

| Round | Verdict |
|---|---|
| li8r, ouou, c27b | merge |
| aun1, jb7q | merge after fixes: two commits still use `perf:` (pass-1 finding 7) |
| sknb | superseded by ibrf; its defects 1, 2 and 4 are fixed there |
| c5qc | merge (the pass-1 test finding is fixed; the `Exact` growth is still unpriced) |
| wqfi | merge |
| dp7c | merge (new finding 8 is a note, not a blocker) |
| ibrf | **merge after fixes**: new findings 1, 2, 4 and 6 block |

### 3. Pass-1 findings

1. **Phase in the wrong units: addressed.** `position_at` (`landing.rs:265-328`) uses `b = BASE - s`. The test `the_phase_is_read_in_units_of_the_magnitude_zero_quantum` pins `21/32` at magnitude 1, slot 5. The oracle `placed` is derived independently, but its "past" classification copies the code's three cases (`tests.rs:181-194` mirrors `landing.rs:313-326`).
2. **Walk start for a negative shift: addressed.** `landing.rs:214-224` uses the ceiling of `log2 radix` for `t < 0`. I checked the closed form `least`, `room` and `i` by hand. `the_walk_start.rs` pins `2^126` on the decimal law at magnitude 119.
3. **Bare primitives: partly addressed, and the two reads disagree.** `den` is now `NonZeroU128`. `num: i128` stays, and `Remainder::numerator`/`denominator` still return `Maybe<u128>` (`position.rs:217,229`). My read: `Scaled::of`'s parameters fall under definition-site exception 4 of `no-bare-primitives` (helper parameters that build a type from literals), as `Slot::at` does. The `Remainder` accessors are unwrap doors that are not declared as such, which is not an exception. Since this disagrees with pass 1, it goes to a question ticket rather than a third expert.
4. **Runtime zero-denominator check in `Scaled::of`: addressed** for `Scaled`. `Remainder::of` and `Fraction::of` still read zero as zero, under their ticket.
5. **Stale `docs/FORMAT.md`: addressed.** It has the landing section (`FORMAT.md:2228`), and the public-surface sweep is clean.
6. **The "canon question" FIXME: addressed.** It is gone. The two new FIXMEs name `task` tickets in `tickets-arvo.toml`.
7. **`perf:` commit type: not addressed.** `d0efc772` and `65ae8c57` still use it.
8. **Silent skip in `the_wide_divisor`: addressed** (`assert_eq!(checked, 3)`). The 192 case is still in the list only so that it gets skipped. The `Exact` growth from 56 to 112 bytes is still unpriced.

### 4. New findings

1. **`land` silently lands on a coarser magnitude** [correctness, tests]. In `landing.rs:151-161`, a magnitude whose position is unformed is skipped and the walk moves on to a coarser one. Where both terms pass `2^256` but their difference is small, the finest magnitude in range is skipped. This happens on falling laws with a nonzero phase.
   - **Repro** (an uncommitted spike, public API only, at `/tmp/claude-0/-home-user/7c35e2ad-fb65-5898-963d-96f2bad980fd/scratchpad/repro/src/main.rs`):
     - Quantum `BASE 0, MAGNITUDES 300, SLOPE -1`, `Signed<8>`, `Phase::halves(1)`, `TowardZero`, saturate.
     - Point `Scaled::of((1<<126)+1, (1<<127)+1, 0)`, which is `1/2 + 1/(2^128+2)`.
     - Expected: magnitude 135, slot 127 (the position there is `128 - ε`).
     - Got: `magnitude 128, slot 0`, so `adapt` gives the value `1/2`. That is 127 finest steps off, and the design (`DESIGN.md.tmpl:2223`) promises the finest magnitude in range.
   - **The oracle cannot see it.** `walked` returns `Isnt` whenever an unplaceable magnitude is finer than the best one (`tests.rs:239-243`), and `misses` drops those points (`tests.rs:328-330`). The guard `placed * 2 >= all` only rules out total blindness. This is setup that helps.
   - **Fix.** Take the difference `x - p/q·r^BASE` once, at `min(power, BASE)`, before scaling by `r^-s`. That removes the cancellation, which grows with `s`. At minimum, return `Isnt` on an unformed magnitude in the walk. Commit this point as a failing test first, and make the oracle assert `land`'s answer in its unknown region instead of skipping it.
2. **`land` never forces `Format::ADMITTED`** [canon, correctness]. With `Phase::of(1, 0)` on `Constant<0>`, a call to `land` compiles and returns `Isnt` at run time (repro, same file, `nophase`). Multiplication refuses this phase at compile time (`multiplication.rs:69`). Fix: force `<S::Format as Format>::ADMITTED` in `land`, delete the `reduced_phase` `Isnt` branch, add a compile-fail pair, and correct the doc at `landing.rs:140` and the design sentence.
3. **`land` is partial on admitted signatures, decided at run time** [canon question, drift]. On `Integer<8>`, `2^-255` lands but `2^-256` and `2^-300` return `Isnt`, because `D` passes 2^256 (repro, `tiny`). The design declares this limit (`DESIGN.md.tmpl:2262`). The commit body of `c35874ba` claims `land` "refuses at compile time what it cannot land", which is false. No test asserts `land`'s `Isnt` on any input. This is the spine question from the canon gate, and it needs a second read.
4. **Missing failure-path tests, and properties tested on samples** [tests].
   - No test hits `land`'s `Isnt` on any input.
   - Clamping is tested only on a rising law. `DESIGN.md.tmpl:2325-2326` promises a falling one too.
   - `the_members.rs` checks a property that holds in every mode, but only on two of six modes per format, and never on `Stochastic`.
   - `is_one_progression` is tested on 9 hand-picked laws, all radix 2 with a positive slope: no falling law, no radix 3 or 10, no range on one side of zero.
5. **`is_one_progression` only picks the error message** [drift]. `landing.rs:113` is implied by `:121`, because `one_grid` implies a single progression (`progression.rs:36`). `land` admits wrapping exactly where the slope test did. Calling ticket `bb82327ee374` "settled" overstates it, and the claimed mutant only reddens the stderr snapshot.
6. **Workflow leakage in published docs** [public-surface, blocker]. `DESIGN.md.tmpl:2324` and `docs/FORMAT.md:2333` say "The two points the first review reproduced". Name the points without saying how they were found.
7. **Wrong number in a test comment** [naming]. `tests.rs:311-313` says a phased decimal law "of 50 magnitudes", and so does the author's report. `PhasedDecimal` is `Indexed<-20, 30>`, which is 30 magnitudes. The sweep was cut to whatever the oracle could read, not to the format's range.
8. **Note: two landings remain in the IEEE harness** [drift]. `tests/ieee754/mod.rs:82-110` keeps a second landing search, for `TiesToAway`, with the same silent `continue` on an unformed position.
9. **Minor** [public-surface, naming]. The crate README.md.tmpl summary of `land` leaves out its other two refusals. `Scaled` derives `PartialEq` on its representation, so `1/2` and `2/4` compare unequal.

### 5. What I verified and the checks I ran

- `cargo test -p arvo-format --no-fail-fast`: everything green. lib 329, compile_fail 32, ieee754_binary 14, ieee754_decimal 5, matlab_fi 15, matlab_fimath 268, ties_away 10, and two more suites at 4 and 5.
- `cargo clippy -p arvo-format --all-targets -- -D warnings`: clean.
- `cargo mock --lint-only`: all lints passed.
- The `pr-review` surface sweep: clean. The worktree is clean afterwards.
- No em-dashes in the diff. Commit format is fine apart from the two `perf:` commits.
- Checked by reasoning:
  - `is_one_progression` is correct for every case I worked by hand.
  - The provably-past logic in `position_at` is sound, because `Wide` is sign and magnitude below `2^256`.
  - The operations still refuse a magnitude-indexed quantum through `is_constant_family`.
  - `Landable::ADMITTED` has three asserts.
- I did not rerun the author's mutants.

## The author's reading

The three reproductions were run as given and print what the report says: magnitude 128 slot 0 where
135 slot 127 is right, `Isnt` at `2^-256` and `2^-300` on `Integer<8>`, and `Isnt` at run time for a
zero-denominator phase. `Indexed<MIN_EXP, COUNT>` takes a count, so finding 7 holds: `PhasedDecimal`
has 30 magnitudes and the wide phased law it replaced had 200, not 300.

Finding 1 is the cancellation the reviewer names. Every magnitude's numerator is the one difference
`num q r^(power - e) - p den r^(BASE - e)`, with `e = min(power, BASE)`, times a nonnegative power of the
radix, so the difference is formed once and only its scale changes with the magnitude; and a magnitude
whose position is still not formed ends the walk with nothing rather than passing to a coarser one.
Finding 2 is taken as written. Finding 3 is the declared limit of the four-limb remainder, under
`arvo_s_exact_position_carries_no_remainder_denominator_past`; the commit body's sentence cannot be
corrected without rewriting pushed history, so this record says it is wrong instead, and a test now
asserts the `Isnt`. Finding 5 holds: the set test names the refusal, the slope test still decides
admission until wrapping on a magnitude-indexed progression is built. Pass-1 finding 7 is left for the
same reason as the commit body. Pass-1 finding 3, where the two reads differ, goes to a question ticket.
