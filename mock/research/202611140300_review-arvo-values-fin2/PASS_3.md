# work/arvo-values, third reading after round 202611140447

I read the branch at `9d96b08a` against `dev` at `3865b39a` in `/home/user/.worktrees/arvo-values-review3`, read-only. The worktree was clean when I finished. One scratch clone was made in the session scratchpad and then deleted.

## Canon gate

It passes provisionally, on the same condition as in both earlier passes. The rows I checked, queried with `cargo mock query` and `cargo mock links`:

- `ruling::the_panel_finishes_the_canon_without_him` (ratified by op). The panel decides, "put through two independent agreements".
- `question::whose_reduction_governs_a_lossy_crossing`. Its decider is `panel` and it has no `answered`, so it is still open.
- `proposal::a_conversion_is_one_placement_onto_the_target_grid`. It has `answers` Q36, `stamped_by = "experts"` and `standing = "one_expert"`.
- `proposal::an_order_is_named_exactly_where_a_crossing_is_lossy`.
- The schema for a proposal's stamp, at `mockspace.toml:1455`: "A stamped proposal is still a proposal and not canon; the stamp says the work depending on it may go on until he answers."

The schema line is what licenses building `convert` on the stamp. The design does not fill Q36 itself; it says it takes the target's option on the stamp (`DESIGN.md.tmpl:30-33`). Nothing in the registry has moved since PASS_2, so the gate rests on ticket `6a490295809d` exactly as before. That call is not mine.

I read PASS_1 and PASS_2 before I formed my own reading of the rows. My agreement with them is therefore confirmation, not an independent second reading.

Two observations for that ticket that PASS_2 did not list:

- The stamp names its question ticket as `does_the_target_s_reduction_govern_a_lossy_conversion_in_arv`. That is a truncated slug, and `mockspace.toml:1462` asks for "the id in backticks".
- The stamp sends the formalisation to op ("asks him to make it formal"). `mock/agent/MAIN.md.tmpl:89` says "a canon question does not go to him".

## Suite audit

It passes.

- I ran `cargo test -p arvo-numeral --no-fail-fast` over all 44 targets: 286 passed, 0 failed, 0 ignored.
- I ran the feature-gated tests by hand:
  - `--features hlist-const --test the_count`: 4 passed, including both `ui_count` compile-fail pairs.
  - `--features hlist --test the_count_without_const`: 3 passed.
- I ran `cargo test -p arvo-format --lib multiplication::tests`. Everything passes except `the_inverse_of_three_round_trips_at_every_unsigned_width_from_one_to_63`, which is still red. That red was already on dev (N4).
- Bodies I read:
  - `the_host_door.rs` and `common/hosts.rs`. The door is held against `adapted` for every host integer. The named doors check the sign-crossing cases with the slots written out.
  - `the_count.rs` and `count/mod.rs`.
  - `the_quotient.rs`: every divisor at widths 1 to 4, checked against the oracle and against `fits_div`.
  - `the_debug_check.rs`: a `should_panic` test per operation per policy, plus the Quiet twins and the fits twins.
  - The ring sweep in `src/tests.rs`: every rung against the format's `sub`, `neg` and `mul`, exhaustive at widths 1 to 12.
  - Both `the_conversion_under_every_source_signature*` binaries.
  - arvo-format's `the_identity.rs`. Its control rule is shown wrong on phased formats (`control_wrong == with_one`).
- I found no tautologies. Every sweep asserts its own size, and the source-signature sweeps assert whether some member left the range.
- No test body changed since PASS_2; only the two `.stderr` snapshots did.

## Verdict

Merge after fixes.

## Blockers

1. Commit text [rules]
   - These commits are still on the branch:
     - `ff8aeffa` and `8e46316d`: "by a fresh seat".
     - `2b413b50` body: "The third fresh seat refuses the branch".
     - `1f27fce0`: "each point of the first reading".
     - `e33c0c13`, `9a3d5149` and `ae4efecb` keep git's default merge subject.
   - These become permanent in a public repository once the branch lands, and `the-public-surface-is-checked-on-every-pr` blocks on one offending commit.
   - Fix: reword them before landing. Ticket `ffa4ce633701`. If the branch lands as it is, say in that ticket that the leakage is now permanent rather than pending.

## Non-blocking

1. `mock/test-history/default/arvo-numeral.json:216-217` [tests]
   - The record lists `the_count` and `the_count_without_const` as passing binaries in the default flavour. Built without features they contain zero tests (`#![cfg(feature = ...)]` at `tests/the_count.rs:12` and `tests/the_count_without_const.rs:12`).
   - No committed record or gate ever runs the `hlist` suites, so the feature's 7 tests and its compile-fail pairs are checked only when someone passes a feature by hand. The record reads as coverage it does not have.
   - Fix:
     - Give both binaries `required-features` in `Cargo.toml` so that a default run skips them visibly.
     - Commit a test record for an `hlist` run and an `hlist-const` run.
   - No ticket is named for this; one is owed.

2. `tests/the_conversion_under_every_source_signature_past_the_range.rs:8` [naming]
   - "Every member past the target's range on either end" is the same false wording PASS_2 non-blocking 3 fixed in the design. Over an unsigned range only the top end is left.
   - Fix it the way `DEEPDIVE_PROOF.md.tmpl:169-171` now words it.

3. `DEEPDIVE_PROOF.md.tmpl:174` [naming]
   - The paragraph on the radix sweep ("Between every pair of radixes") is fused onto the end of the paragraph on the source-signature sweep, on one 188-column line. The five sweeps read as one block.
   - Fix: break the line and the paragraph there.

4. `tests/common/exponents.rs:37` [rules]
   - `pub use super::shapes::{minus_one, one, zero};` is unused. Every conversion binary prints the warning, about 30 per run.
   - Fix: remove the re-export or use it.

5. `DESIGN.md.tmpl` is 549 lines; it was 365 on dev [rules]. Moving the proof out took it from 785 to 549, which is still past the 500-line tripwire in `file-size-limit`. The next seam is the doors and the host-integer surface (`:175` onward).

## PASS_2 findings

- B1 (generated documents stale): resolved.
  - `c33374b5` and `a706508c` regenerated `docs/`, and no template or registry row changed after them.
  - I ran `cargo mock` in a scratch clone at `9d96b08a` and it left `git status` empty.
  - `docs/NUMERAL.md:39` carries the Q36 disclosure, `:72` states the mutants as flags, and the five-sweep text is on `docs/NUMERAL_PROOF.md`.
- B2 (commit text): left. It is blocker 1 above, ticket `ffa4ce633701`.
- Non-blocking 1 (policy mutant run too narrow): resolved.
  - `run.sh` now runs `arvo_mutant_convert_policy` over every `the_conversion*` binary.
  - The committed output shows only `no_source_policy_moves_*` red and every other conversion binary passing, which matches `DEEPDIVE_PROOF.md.tmpl:172-174`.
- Non-blocking 2 (no mode mutant): resolved.
  - `arvo_mutant_convert_mode` is at `src/ratio.rs:105-111` and is declared to `check-cfg`.
  - Its committed run turns all three `no_source_mode_moves_*` tests red.
- Non-blocking 3 ("on either end" in the design): resolved in the design at `DEEPDIVE_PROOF.md.tmpl:169-171`, but left in the test comment (non-blocking 2 above).
- Non-blocking 4 (bare `bool` on `Machine::SIGNED`): resolved.
  - `src/machine.rs:26` is now `const SIGNED: Bool`, and the design's named surface (`DESIGN.md.tmpl:183`) says so.
  - The other primitives in `src/host.rs` are inside the surface the design declares at `:175-193`.
- Non-blocking 5 (design file size): partly resolved, at 549 lines. Non-blocking 5 above.
- Non-blocking 6 (stale `says` on the order proposal): left. `proposal::an_order_is_named_exactly_where_a_crossing_is_lossy` still says "An order must be named". Ticket `d13906e4673f`.
- Canon gate (stamp independence): left, ticket `6a490295809d`, with the two added observations above.
- N4 (arvo-format red dropped from the record): left. It is still red and the record still drops it. Ticket `7c3655630b91`.

## Verified

- `cargo mock --lint-only --strict`: all lints pass. None of the 46 warnings falls on a line this branch added; I checked the hunk ranges of the warned templates.
- Added lines carry no em-dashes or en-dashes; the grep was checked against a positive control. No rustdoc first line in the changed source is over 80 characters.
- The rung arithmetic for `sub`, `neg` and `mul` is right below and at the carrier width:
  - Unsigned saturating difference is pinned at zero.
  - Signed saturating product takes its side from the operands' signs.
  - `mul` takes the slot product only at exponent zero, which `Sole` makes sound by refusing phases and indexed quanta.
- In `from_index`, the sign-crossing cases are answered before any cast, and `holds_index` is right at width 64.
- The root `README.md`, the crate's `README.md.tmpl`, `mock/DESIGN.md.tmpl`, `PRINCIPLES.md.tmpl` and `MAIN.md.tmpl` match the surface:
  - no accumulator, no remainder, no unsigned 64-bit product;
  - the opt-in `const_trait_impl`, which `unstable-features` lists as allowed.
- No version moved and the toolchain pin did not move. `hlist` and `hlist-const` both forward to the dependency. The licence is unchanged.
- No open pull requests on `orgrinrt/arvo`.
- Commit messages carry no `Co-Authored-By` lines, no generated-by lines and no tool adverts.

