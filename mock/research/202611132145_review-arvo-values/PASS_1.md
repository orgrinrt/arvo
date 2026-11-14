# Review of `work/arvo-values`, pass 1

Reviewer: `pr-reviewer-senior`, a fresh seat, on the branch at `9a3d5149` against `origin/dev`. The
text below is the reviewer's report as returned, unedited apart from line wrapping. It is a
reviewer's reading and not a ruling. What was done about each finding is in `RESPONSE.md` beside it.

## Verdict

DO NOT MERGE.

The test gate fails, and that alone stops the merge. Two blockers on the public-surface checklist
stop it separately (`/home/user/.claude/.rules-b/the-public-surface-is-checked-on-every-pr.md`; this
repo's `.claude/rules/` has no copy, so I used the workspace one).

Canon gate: aligned on the shape of the work, with one point handed back (finding 5). I checked these
rows:

- `ruling::wrap_or_clamp_stays_open_and_both_get_priced` (rung open). The branch correctly ships no
  default (`DESIGN.md.tmpl:526-530`, `:753`).
- `ruling::never_a_runtime_check_and_one_lowered_path`. The divisor is non-zero by type,
  `ratio.rs:41`.
- `ruling::the_overflow_panic_is_permitted_and_bounded`. The debug abort sites.
- `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`. The host door is
  stated over the slot index, not the carrier.

The value row stands on its provisional stamp and every round says so. I am the first expert on the
canon call. A second independent read is owed.

## Findings

1. `tests/the_quotient.rs:37,42-47`, `tests/the_conversion.rs:46,49-55`,
   `tests/common/mod.rs:177,180-186` (as used by `fixed_point_product.rs:29-54`) [tests]: these
   tautological sweeps carry the oracle claim for `div`, `convert`, the fractional `mul` and
   `fits_mul`.
   - `Num::div` is literally `at_index(div_slots::<S>(self.slot(), by, Dither::UNUSED).index())`
     (`src/ratio.rs:58`). The test compares it with `div::<S>(Slot::at(a), b, Dither::UNUSED)`, the
     same call.
   - `fits_div` (`ratio.rs:64-66`) is `!panic_on_overflow(quotient_position(..))`, and the test
     asserts exactly that expression.
   - The same holds for `convert` against `convert_slots` (`ratio.rs:100,106-109`), for `mul` at a
     non-zero exponent against `mul_slots` (`num.rs:401`), and for `fits_mul` at every width against
     `num.rs:408-411`.
   - If `arvo_format::div` were wrong, both sides of line 37 would be wrong identically and the sweep
     would stay green. Its only discriminating power is the `at_index`/`slot` round trip, which
     `check_value` already covers (`common/mod.rs:113-122`).
   - This is the "exhaustive against the oracle" the brief demanded, and for these three operations
     the oracle is the implementation. The real oracle lives in
     `arvo-format/src/division/tests/the_oracle.rs`. These sweeps add a count and nothing else.
   - `mutant-div` and `mutant-convert` were placed in the wrapper (`ratio.rs:50,92`), which is the one
     place a sweep that restates its callee can see. A mutant in the format's division, where the
     arithmetic actually happens, passes every one of these sweeps. "A mutant per operation, shown
     red" was arranged to be true rather than shown to be.
   - The "control" `fixed_point_product.rs:82-107` compares `arvo_format::mul` at two signatures. It
     tests the format and says nothing about `Num`.
   - The design prescribes this "proof" (`DESIGN.md.tmpl:636-647`), so the design is wrong too.
   - What settles it: delete the restating sweeps and the `fits_*` assertions that equal their
     bodies. Either prove these operations against an independent rational walk, the way
     `arvo-format/src/multiplication/tests/the_identity.rs:264` does with `Ratio`, or state in the
     design that they are proved in `arvo-format` and that the value tier proves only the delegation.
     The named cases (`the_named_quotients_sit_where_arithmetic_puts_them`,
     `a_tie_below_zero_goes_where_each_mode_sends_it`,
     `the_named_conversions_sit_where_arithmetic_puts_them`) are real and stay.
   - The ring operations at exponent zero are not affected: the rung is an independent
     implementation, and the mutants against it are genuine.

2. `README.md:19,23` [public-surface, blocker]: the README still describes the value before this
   branch.
   - Line 19 says `Num` "adds the way the format says it adds, compares". Line 23 says "with addition
     and comparison on it and nothing more yet".
   - The branch ships `sub`, `neg`, `mul`, `div`, `convert`, the host door, `Fix`/`UFix` and
     `Accumulator`.
   - Rewrite both passages.

3. `mock/DESIGN.md.tmpl:115-116` to `docs/DESIGN.md:124`, and `mock/agent/MAIN.md.tmpl:17`
   [public-surface, blocker]: the root design and the agent template are stale.
   - `DESIGN.md.tmpl` says "with their sum and comparison, and nothing yet of subtraction or the
     product". This branch edited that file (`+8 -6`) and left the sentence it falsified.
   - `MAIN.md.tmpl` says the same and renders into every agent's `CLAUDE.md`.
   - `cargo mock` regenerates cleanly, so the render is current. It is the source that is false.

4. `mock/design_rounds/202611132039-hccc/..._topic...md:72-75` [drift]: the door shape for T2 is not
   in the repo.
   - The brief ordered the door's signature and contract into the round topic as a proposal.
   - The topic gives one sentence ("a ratio of two machine integers adapted to a width, a mode and a
     policy") and points at a channel doc, `briefs/arvo-door-shape-t3.md`. That is not the
     `briefs/arvo-door-shape.md` the brief named, and I could not find it anywhere under `/home/user`.
   - T2 builds against a contract nobody can read here
     (`a-helpers-report-lands-in-the-repo-it-is-about.md`).
   - Put the signature, the parameters and the equality law into a topic or a proposal row.

5. `src/accumulator.rs:32` [canon, handed back]: whether this ships one arm of an open question.
   - `question::which_chain_carrier_ships` is open. Its first option is "The named accumulator, which
     carries the budget only". Its `bound` calls the fourth option, a composition of carriers, "the
     design rather than a compromise", and its `note` says "the choice is op's" while
     `decider = "panel"`.
   - Shipping `Accumulator` under that name either builds one arm of the endorsed composition or
     pre-empts the choice. `DESIGN.md.tmpl:504-505` asserts the first reading without citing the
     bound.
   - The canon does not decide between the two readings. The call is the dispatcher's, taken from the
     canon text, and it needs a second read.

6. `src/accumulator.rs:75-92,138-146` [correctness]: a zero capacity is admitted.
   - `CAP = 0` is admitted (`sums(0)` = `(0, 0)` holds). `take` then returns an empty accumulator and
     the whole slice, so a caller looping on `take` never advances.
   - Refuse `CAP == 0` in `ADMITTED` and add a compile-fail pair.

7. `tests/a_consumer_crate.rs:28-34,37-43` [tests]: the path fix is fragile and its test does not
   catch a regression.
   - `ancestors().nth(3)` is wrong under `--target <triple>`, where the build directory is
     `<target>/<triple>/<profile>/deps` but `tmp` sits at `<target>/tmp`.
   - `the_paths_are_the_ones_this_run_was_given` asserts that cargo sets the manifest directory equal
     to the working directory. Reverting to `env!` passes it in any single checkout, so it does not
     exercise the bug it is named for.

8. `arvo-format/src/multiplication/tests/the_identity.rs:317-374` [tests]: the identity law is
   sampled.
   - `multiplicative_identity` is a law over every phase `n/d`, radix and exponent. It is walked only
     at `d` of 1 and 2, and the decimal radix only at a zero phase.
   - Add phases with `d` of three or more, and decimal with a non-zero phase.

9. `Cargo.toml:39-51`, `src/*` `cfg!(feature = "mutant-*")` [drift]: nine more wrong-answer features
   in shipping source.
   - Each feature makes the published library compute wrong answers, and feature unification lets any
     crate in a dependency graph turn one on.
   - `proposal::the_debug_check_is_a_parameter_the_consumer_names` rejected a cargo feature as a
     vehicle for exactly that reason: a feature reaches "a crate that never named it".
   - The pattern predates this branch (`mutant-8` to `mutant-64`). The branch adds nine more. File a
     ticket on the mechanism.

10. `arvo-format/src/{adapt,lib,multiplication}.rs` [drift]: `arvo-format` got more than one minimal
    addition.
    - The brief allowed "a minimal addition you name". The branch adds four: `WithOverflow`,
      `WithRounding`, `points::Fixed`, and `multiplicative_identity` with a 178-line suite. Each sits
      in the right crate.
    - The coordinator must reconcile this with T2's overhaul before T2 starts.

11. `9a3d5149`, `ae4efecb` [rules]: two merge commits do not follow the message rule. Both use git's
    default "Merge remote-tracking branch ...", not `type: lowercase imperative`. The other 41 commits
    pass: no bylines, no adverts, no leakage beyond citing round identifiers.

12. [naming, minor]
    - `band_*` tests named `..._adds_and_orders_...` now also assert `sub` and `neg`.
    - `the_count_without_const.rs:24-27` repeats `count/mod.rs:108`.
    - The trybuild pass cases emit 14 unused-import warnings.

13. The author's report [claims]:
    - It says the release twin ran 6 tests in the dev profile. That binary runs 0 there. It runs 6
      under `--release`, which I checked.
    - It says "None is red on the tip" after running only `arvo-numeral`, although the branch changes
      `arvo-format` and moves the workspace's `notko` lock from `0a13d99` to `8a74dbc`.
      `arvo-format`'s suite is red (below).

## Checks run, and what they printed

- `cargo test -q -p arvo-numeral` (dev profile): all green. lib 12, `band_1_to_8` 11, the other bands
  2 each, `a_consumer_crate` 2, compile_fail 32, `fixed_point_product` 6, `the_accumulator` 10,
  `the_conversion` 6, `the_crossing` 6, `the_debug_check` 27, `the_host_door` 5, `the_lane` 4,
  `the_quotient` 5. 0 tests in `the_debug_check_in_release`, `the_count` and
  `the_count_without_const`.
- `--release --test the_debug_check_in_release`: 6 passed.
- `--features hlist-const --test the_count`: 4 passed. `--features hlist --test
  the_count_without_const`: 3 passed.
- `cargo test -q -p arvo-format`: 300 passed, 1 failed. The failure is
  `the_inverse_of_three_round_trips_at_every_unsigned_width_from_one_to_63`, panicking at
  `the_inverse_round_trip.rs:37`. The branch does not touch that file, and ticket
  `arvo_format_inverse_round_trip_underflows_at_width_1` already names it. I did not reproduce it on
  `dev`.
- `cargo mock`, run from the worktree with `env -C`: no lint errors, 46 warnings, `git status` clean
  afterwards. A first run went to `/home/user/arvo` by mistake because the working directory resets
  between calls; that clone also stayed clean.
- Mutants. Each committed red output was reproduced line for line:

  | Feature | Test target | Result |
  | --- | --- | --- |
  | `mutant-div` | `the_quotient` | 4 failed |
  | `mutant-convert` | `the_conversion` | 4 failed |
  | `mutant-index` | `the_host_door` | 3 failed |
  | `mutant-accumulator` | `the_accumulator` | 3 failed |
  | `mutant-count` | `the_count`, with `hlist-const` | 3 failed |
  | `mutant-mul` | `band_1_to_8` | 1 failed, 197950 of 1747600 |

- Sketch `202611130342_value-add-disassembly`: builds. The emitted `.s` matches
  `outputs/asm-release-x86_64.txt` instruction for instruction, aliases included. Its two `Abort`
  probes are vacuous: in release there is no debug check to remove, so they carry no case that could
  fail.

## Not checked

- `mutant-sub`, `mutant-neg` and `mutant-under`, and every mutant against the lib tests and the wider
  bands.
- `tests/consumer/src/main.rs` line by line, and the compile-fail `.stderr` contents.
- The changelists against the source, claim by claim.
- The channel docs (op-answers-35 and -36, the door brief), which are not in the repo.
- Whether the `arvo-format` red reproduces on `dev`.
