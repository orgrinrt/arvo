# Second reading of `work/arvo-values`, at `a303d07b`

A fresh `pr-reviewer-senior` seat on opus, in a detached worktree at `a303d07b`, with a neutral brief
that named `PASS_1.md` and `RESPONSE.md` as claims to test. The report is below as it was returned.

## Verdict

DO NOT MERGE.

Two things block on their own. First, `Accumulator` builds one arm of a canon question that is still open, and that call is waiting on op. Second, the suite gate fails: the oracle sweeps cover only some widths, and one test asserts nothing.

On the gates: the canon gate passes for the ring operations, `div`, `convert`, `under` and the debug check. It does not pass for two specific surfaces, findings 1 and 2. The suite gate fails on findings 3 and 4.

I am the second expert to read finding 1. `PASS_1.md` called it ambiguous. My own reading, taken from the canon text before I opened PASS_1, is that it is misaligned. The two reads disagree, so the call is op's, and the ticket the author filed (`does_the_name_accumulator_in_arvo_numeral_pre_empt_which_cha`) is already the right route. I did not verify that the ticket exists.

## Findings

1. `src/accumulator.rs:32` and `DESIGN.md.tmpl:507-511` [canon, blocks]
   - `question::which_chain_carrier_ships` has no `answered` value, and its decider is `panel`.
   - Its first option is, word for word, "The named accumulator, which carries the budget only". That is what this type is: `CAP` is the budget and it holds an exact sum.
   - The design says "nothing here answers it" and "it is not offered as the carrier". But shipping the type puts option 1 into the shipped set. That rules out options 2 and 3 on their own, so it answers part of the question.
   - A helper's design round is not the panel deciding.
   - Fix: take `Accumulator`, its tests, ui pairs and mutant off this branch until the ticket is answered. The rest of the branch does not depend on it.

2. `src/host.rs:131` (`holds_index`) and round topic `202611132039-hccc_topic…md:28-30,89` [canon, blocks pending the call]
   - The topic rests on `never_a_runtime_check_and_one_lowered_path` ("no door validates").
   - It never cites `ruling::the_ingest_door_waits_on_a_known_use` (stated), which is the row on point: "a runtime membership door at ingest is not blessed now", and it waits on a known use.
   - `holds_index` is exactly a runtime membership verdict on an outside host integer.
   - `fits_add` and the other `fits_` functions are about results of typed arithmetic, so this question does not touch them.
   - The design has to address that row. Either it shows a known use, or `holds_index` goes. `from_index`, which completes rather than validates, can stay.
   - This is the first read on this point, so a second read is owed.

3. The oracle laws are checked over a sample of widths [tests, blocks]
   - `div`: widths 1 to 8 and one fractional grid (`the_quotient.rs:82-117`).
   - `convert`: no source wider than 16 bits, and nothing narrowing from 33 to 64 bits (`the_conversion.rs:88-113`).
   - Fractional `mul`: only 6-bit grids (`fixed_point_product.rs:31-57`).
   - The bands already have the lattice and seeded machinery for widths up to 64 (`common/mod.rs:200-213, 270-333`). Use it for all three.
   - Divisors at widths 5 to 8: the unsigned sets are `[1, 3, 31|63|127|255]`, all odd, so no unsigned quotient at those widths ever lands on a tie. `HalfUp` and `HalfEven` are never told apart there.
   - I checked this with a mutant of the format's `div` at the case -7/2. The widths 5 to 8 sweep did not catch it; only the widths 1 to 4 sweep did.

4. `tests/a_consumer_crate.rs:46-49` [tests, blocks]
   - `the_crate_directory_as_resolved` is a `#[test]` that asserts nothing. It is a helper that `the_crate_directory_is_read_when_the_test_runs` re-runs and reads back.
   - It passes on every run and adds to the count.
   - Make it a non-test entry (an env-gated early return inside the real test, or `#[ignore]` run with `--ignored`).

5. `Cargo.toml` mutant features and `cfg!(feature = "mutant-*")` in `num.rs:194`, `ratio.rs:50,92`, `host.rs:116`, `accumulator.rs:149` [drift, follow-up]
   - Nine new features that make the published library compute wrong answers, and feature unification lets any crate in a dependency graph turn one on.
   - `proposal::the_debug_check_is_a_parameter_the_consumer_names` turned down a cargo feature as the debug check's vehicle for exactly this reach.
   - Ticketed as `arvo_numeral_carries_its_test_mutants_as_cargo_features_in_s`. A `--cfg arvo_mutant=…` set through rustflags would avoid unification.

6. `src/alias.rs` `USize::from_host`/`host` and `ISize::from_host`/`host` [drift, follow-up]
   - These sit beside `from_index::<usize>` and `index::<usize>`: two public doors for one operation.
   - The design keeps them "as they are". Pre-1.0, the older door goes.

7. Commits [rules, follow-up]
   - `9a3d5149` and `ae4efecb` carry git's default merge subject.
   - `252d1ce4` is typed `test:` but changes `src/accumulator.rs` (the `take` refusal).

8. arvo-format red [not this branch's, does not block]
   - `the_inverse_of_three_round_trips_at_every_unsigned_width_from_one_to_63` panics on an overflow inside the test itself (`the_inverse_round_trip.rs:37`, `top - 2` at width 1).
   - The file is unchanged on the branch, and the ticket is `arvo_format_inverse_round_trip_underflows_at_width_1`.

## RESPONSE.md, judged against the code

- Finding 1, adequate. `tests/common/oracle.rs` is an independent implementation, and I read its rounding and scaling arithmetic.
  - I mutated `arvo_format::div`, `convert` and `mul` in a throwaway copy. All three sweeps went red: 10, 30 to 70 and 10 disagreements.
  - That copy was an ad-hoc check outside the repo, so it carries no weight as evidence. The tests' power is visible from their bodies anyway.
- Findings 2, 3, 6, 7, 8 and 12, done.
- Finding 4, done. `THE_CALLS_ACROSS_THE_FORMAT_BORDER.md` is in the repo.
- Finding 5, not settled. See finding 1 above.
- Finding 9, deferred with a ticket.

## What I ran

- `cargo test -q -p arvo-numeral`: everything green, compile_fail 33.
- `--features hlist-const --test the_count`: 4 passed. `--features hlist --test the_count_without_const`: 3 passed. `--release --test the_debug_check_in_release`: 6 passed.
- `cargo test -q -p arvo-format`: 301 passed, 1 failed (finding 8).
- `--features mutant-div --test the_quotient`: red, as committed.
- `cargo mock --lint-only --strict`: all lints passed, 46 warnings, none in `arvo-numeral`, and the worktree stayed clean.
- No em-dashes in any added line. Bold only on round-metadata lines. All 50 commit messages read one at a time: no bylines or adverts.
- Every `pub` item in `arvo-numeral` is named in `DESIGN.md.tmpl`.
- `const_trait_impl` is on the workspace "watch" list, so it is allowed.

## What I did not check

- Whether `docs/` matches a fresh regeneration: the run in a scratch copy was refused by permissions.
- The committed runs of `mutant-sub`, `-neg`, `-under`, `-convert`, `-index`, `-accumulator` and `-count`.
- The `.stderr` contents of the ui tests.
- Whether the tickets named in RESPONSE exist.
- The open branch `work/numeral-integers`, 59 commits and 78 `arvo-numeral` files, which looks stale against `dev`. The dispatcher should confirm it is dead.

The repo's `.claude/rules/` has no `the-public-surface-is-checked-on-every-pr.md`. I used the workspace copy instead.
