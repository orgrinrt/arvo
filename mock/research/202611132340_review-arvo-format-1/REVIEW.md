# Review of work/arvo-format, pass 1

**Date:** 2026-11-13
**Reviewer:** a fresh `pr-reviewer-senior` on opus, worktree detached at `fefdab22`.
**Range:** `f0eeb2e3..fefdab22`, the merge base with `origin/work/arvo-standards`.
**Verbatim:** the reviewer's report as returned, kept whole below.

## Report

Canon gate: passed, against the registry (`ruling::never_a_runtime_check_and_one_lowered_path`, ratified;
`ruling::the_operating_constraints_are_intents_and_rules`, in_force) and the crate design. The branch
builds what the design asks for. Two of the findings below are places where the design or code conflicts
with those rows. I am the first read on both canon calls, so a second independent read is owed.

### Verdict

DO NOT MERGE. `land` is the branch's new public door, and it gives wrong answers in two separate ways.
Both are reproduced below. Its tests could not catch either one: the oracle reuses the code's wrong
formula, and the sweep never reaches the inputs where the start bound fails. The landing design section
has to be fixed first, then the code under it.

| Group | Verdict |
|---|---|
| li8r, inverse sweep at W=1 | merge |
| ouou, clippy findings | merge |
| aun1, `#[inline]` and its lint | merge after fixes (finding 7) |
| sknb, `land` / `Scaled` / `Landing` | do not merge (1, 2, 3, 4, 6) |
| jb7q, Knuth D at two limbs | merge after fixes (7) |
| c27b, walks kept out of line, and its lint | merge |
| c5qc, four-limb remainder | merge after fixes (3, 5, 8) |
| wqfi, IEEE suites call `land` | merge once sknb is fixed. The parity inputs (zero phase, small significands) reach neither bug. |

### Findings

1. `src/apply/landing.rs:199-207`, `DESIGN.md.tmpl:2211-2221`, `landing/tests.rs:109-133` [correctness]:
   `land` uses the wrong phase. `Format` and the design (`DESIGN.md.tmpl:70`, `format.rs:47,59`) define a
   member as `phase * r^BASE + slot * r^step(m)`, so the phase is in units of the magnitude-zero quantum.
   `position_at` subtracts `p/q` in units of the step at the current magnitude. So at every magnitude whose
   step is not `r^BASE`, the position is off by `(p/q)(r^(BASE-s) - 1)` slots. Repro: `PhasedFour`
   (`Indexed<-4,6>`, `Signed<4>`, `Phase::halves(1)`), `Scaled::of(21, 32, 0)`, `TowardZero`.
   `21/32 = 1/32 + 5/8` is exactly a member: slot 5 at magnitude 1. `land` returns slot 4 at `2^-3` with
   `on_grid == false`, which is the value `17/32`. The test oracle `placed` copies the same formula. It
   agrees with the code and checks nothing about membership. Rewrite it from the membership predicate,
   then fix the design and the code.
2. `src/apply/landing.rs:134-169` (`first_candidate`), `DESIGN.md.tmpl:2223` [correctness]: the walk can
   start past the right magnitude. The bound relies on `radix^t >= 2^(t floor(log2 radix))`. That is false
   for `t < 0` when the radix is not a power of two: `10^-1 < 2^-3`. Repro on `WideDecimal` with zero
   phase: `Scaled::of(1<<126, 1, 0)` lands at step `10^20` (slot 850705917302346158), losing a digit. The
   same point written as `Scaled::of(8507059173023461586, 1, 19)` lands correctly at `10^19`, slot
   8507059173023461586. For `t < 0`, use `ceil(log2 r)`. Add points with numerators of 100 bits or more on
   the decimal formats to `Far`, which samples only `(1<<100)+7` on a signed range, where the bug sits just
   below the edge.
3. `src/apply/landing.rs:44`, `src/apply/position.rs:217,229` [rules]: the new public positions are bare
   primitives: `Scaled::of(num: i128, den: u128, ...)`, and `Remainder::numerator`/`denominator` returning
   `Maybe<u128>`. This is against `the_operating_constraints_are_intents_and_rules` ("public API positions
   using the stack's own primitives"). `Fraction`/`Phase`/`Remainder::of` already did this, but that does
   not license new instances.
4. `src/apply/landing.rs:45-51`, `DESIGN.md.tmpl:2197` [canon]: `Scaled::of` checks `den == 0` at run
   time and silently turns it into zero. `never_a_runtime_check_and_one_lowered_path` says "invalids are
   caught at compile time", unless the lead designer blesses an exception. The ticket
   `a_zero_denominator_in_arvo_format_s_ratio_constructors_reads` records "left as it is ... by coord's
   word", and a coordinator is not the lead designer. The design is wrong here, and this is a brand-new
   type with no consumers yet. Make the denominator non-zero by type, as `Divisor` is.
5. `docs/FORMAT.md` [public-surface, blocker]: not regenerated anywhere in the range, though the base
   regenerated it with design changes (41c00767). It has no landing section, and it still says "passes
   `2^128 - 1`" twice where the template no longer does. Run `cargo mock` and commit the result.
6. `src/apply/landing.rs:116`, `DESIGN.md.tmpl:2236` [drift]: the FIXME and the design call this "a canon
   question". No `question` row exists (`question.where(asks~...)` finds nothing on the completion or
   union), and the ticket is a `task`. `mark-placeholders-fixme` calls for a `question` where the canon is
   silent.
7. d0efc772, 65ae8c57 [rules]: the commit type `perf:` is not in the repo's list
   (`.claude/rules/writing-style.md`: feat, fix, refactor, docs, test, chore). `check-message` accepts it
   anyway, which is a gap in the check.
8. `src/wide/tests/the_wide_divisor.rs:97-107` [tests]: when the product leaves the four limbs,
   `let ... else { continue }` silently skips the case. That is likely at `top = 192`, which is also four
   limbs, not the "three" the test name says. Assert a count. Separately, `Exact` grew from 56 to 112 bytes
   (`Remainder` 80) on every `adapt` path, and no measurement prices that.

### Verified

- Knuth D (`wide.rs` `by_limbs`): normalisation, the qhat test (`||` short-circuits before
  `estimate*second` could overflow), multiply-subtract, add-back and unshift are correct, and the three-
  and four-limb tests check against the independent `Big` oracle.
- The corner argument for "the step is formed" (each intermediate is bilinear or affine) holds.
- The two lints and `rust_text` read through the view, and the inline lint's 24-call claim cites a
  committed sketch.
- The li8r fix and its planted arm through zero are correct. The 64/65/128 cells of
  `arvo_law_the_multiplicative_inverse_round_trip_is_unreachabl` are still unreached and stay on that
  ticket.

### Checks

`cargo test -p arvo-format`: lib 319/0, compile_fail 30/0, ieee754_binary_parity 8/0,
ieee754_decimal_parity 4/0, matlab_fi_parity 15/0, matlab_fimath_parity 4/0, the_ties_away_alias 10/0,
plus two more suites at 4/0 and 5/0. `cargo clippy -p arvo-format --all-targets -- -D warnings`: clean.
`cargo mock --lint-only`: all lints passed, 46 warnings. The worktree is clean, with no `Cargo.lock`
change.

### Notes

Both repros come from an uncommitted spike crate in the dispatcher's scratchpad, which uses only the
public API. Under the evidence rule they should be copied into `landing/tests.rs` as failing tests before
the fix, and the inputs above are enough to rebuild them.

## The author's reading

Findings 1 and 2 were checked against the source before acting: `format.rs` gives the phase in units of
the quantum at magnitude zero, and `position_at` forms its offset from the phase at the current step; and
`10^-1 < 2^-3`. Findings 3 and 4 are canon calls on one read and wait on a second independent one before
they change the shape; 4 also stands against the coordinator's word on the zero-denominator ticket.
