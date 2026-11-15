Source: kaski channel (coord/kaski of clause-dev), briefs/fixup/review-arvo-two.md, first posted 2026-11-13 08:01 UTC, read 2026-11-15.

# Collected review: arvo `work/numeral-integers` (pass three) and `work/oracle-fraction-wide` (pass two)

Heads match: numeral `77064c5e57e4ad32eefce995c57349a20b363ac5`, oracle `fd99be05536b6725cafc913e936cb94530d40163`, `origin/dev` at `989e02f9`.

**Limits.** I ran no suite and built nothing, as instructed, so every "passes" is the helpers' claim. I read the bodies of the tests the fixes added or changed, and compared committed stderr and mutant output. I am one expert on every canon reading below.

Canon gate: passes for both branches against `mock/registry/*.toml` (`mockspace.toml:31`), with one open point on the numeral ruling recorded as follow-up N2.

---

## 1. `work/numeral-integers`: MERGE

Condition on the landing: fix N1 in the same commit that regenerates `docs/FORMAT.md`, which the merge needs anyway (see Order).

### Pass-two blocking items
- **B1, fixed as instructed.**
  - The registry now says (`mock/registry/ruling.toml:1845-1856`, row `a_format_datum_is_one_point_and_a_saturating_end_is_one_point`, ratified): `says` holds the datum clause, the set-valued clause and the saturating end; the sole-occupancy sentence is gone.
  - `promotion` (`:1852`) says: "Its questions reached it already worded toward their answers, as its own note says" and that the second read "derived its answer before it opened the first read's proposal file, by its own account". The quoted wording matches `mock/research/202611122300_value-stance-second-read.md:59`.
  - `gap` lists the sole-occupancy clause "as a statement of its own, at one cold read".
  - The design does: `arvo-numeral/DESIGN.md.tmpl:12-16` cites `behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` (`ruling.toml:1459`, says near `:1470`) and `the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule` (`:1531`) as the licence and says the clause "is open, with one cold read so far". Both rows are `ratified`.
  - Seals: `DESIGN.md.tmpl:87-90` cites the `:1470` row for `Machine` and `Held`.
  - No live citer of the old row id is left; only two closed round changelists name it.
- **B2, partly.** `mock/PRINCIPLES.md.tmpl:98-114`, `:249-251`, `:281-288`, the agent rules and `WORKFLOW.md.tmpl` are corrected and the docs regenerated (each generated doc differs from its template by the header only). One instance of the class is left, N1.
- **B3, fixed.** `README.md:19` names `Abort` and `Quiet`; `:23` says one number exists.
- **B4, fixed.** `proposal-the-value-type.toml:64-78` supersedes the marker pair; `proposal-the-value-stance.toml:300-312` supersedes the `raw` door row. Op's words are recorded with their four options at `ruling.toml:1865-1877` (`rung = "stated"`); the quote is in `.data/op-responses/202611130539_...md`.
- **B5, fixed.** `arvo-numeral/src/admit.rs:75-80` asserts `carrier_agrees::<SlotsOf<S>>()` directly. The compile-fail pair differs in one token (`Carrier8` against `Carrier16`), and the committed stderr shows the new assertion firing at `Sole::ADMITTED`. `arvo-placement/src/sole.rs:13-16` no longer claims the forms cannot drift.
- **B6, fixed for the exponent.** `tests/band_1_to_8.rs:41-46` runs `UFixed<6, 3>` exhaustively over both policies and five modes; `outputs/mutant-8.txt:16` shows it red under the mutant. The radix half is N3.

### Non-blocking items asked for
- Release test: present, `tests/the_debug_check_in_release.rs:26-47`, three tests under default `Abort` asserting the completed slots. No release profile in `mock/Cargo.toml` turns debug assertions on. No output of a release run is committed.
- `readme` fields: unchanged, all five still `README.md.tmpl`. The reason is recorded at `mock/design_rounds/202611130720-wub5/..._changelist.doc.lock.md:110-111` (no rendered crate readme exists).

### Follow-ups, in order
- **N1 [public-surface].** `mock/crates/arvo-format/DESIGN.md.tmpl:1007-1010` and `docs/FORMAT.md:1016-1019` still say "No crate here holds a value of any format yet, and what one datum of a numeral stands for is an open question". False on this branch. One sentence.
- **N2 [canon, judgement, one expert].** The `promotion` quotes the first read as handed "a datum is a point" in the same list as the sole-occupancy wording, and its part (a) heading is "Is a datum of a format one point of the ambient domain". The row drops one clause for being handed and keeps the other without saying what differs. Either state the difference in `promotion` or put it to op as a question ticket.
- **N3 [tests].** `arvo-numeral/DESIGN.md.tmpl:66` says "scaled integers at any radix pass". No test instantiates a `DecimalRationals` format.
- **N4.** `admit.rs:74` still reads `Slots::ADMITTED` through the row's own provided constant. Harmless by reading: `whole_range` (`:55-65`) re-checks width 1 to 64 and both ends. Drop the line or say so.
- **N5.** `readme` fields point at a template; settle before any publish.
- **N6.** `the_debug_check_in_release.rs:13` is file-gated, so a debug `cargo test` runs zero of it silently.
- Carried from pass two: full-width rungs at 16, 32, 64 sampled; hand mutations of `ABORTS` uncommitted; the `sole!` table.

### What the fixes newly broke
Nothing found. The eight new commit subjects are in form with no bylines.

---

## 2. `work/oracle-fraction-wide`: MERGE AFTER FIXES (one line, no further pass needed)

### Pass-one findings
- **A1, fixed.**
  - Canon says (`ruling.toml:949-954`, ratified): "Never any runtime checks, ever".
  - Code does: `division.rs:103-126` admits `Divisor::of` only where the reduced phase denominator is past one (`no_slot_names_the_zero`). The zero-factor branch is gone from `placed` (`:281-313`). `reduced_phase` (`placing.rs:36-48`) normalises sign and divides by the gcd, so `n + b d` is never zero for `d > 1`. `constant` still refuses out-of-range slots, so the non-member zero slot is unreachable there too.
  - The whole-phase range without zero is refused with `FIXME(arvo_a_runtime_divisor_over_a_whole_phase_range_without_the)` at `:110`.
  - Tests: `division/tests/the_divisor.rs:63-101` checks the verdict against a brute-force search for a zero-valued slot over nine formats, including `6/4` reducing and `4/4` not. The compile-fail pair differs in the phase only.
- **A2, fixed.** `README.md.tmpl:38`.
- **A3, fixed.** `docs/FORMAT.md` differs from the template by the generated header only.
- **A4, fixed.** `Remainder` is public with `of`, `ZERO`, `numerator`, `denominator` (`apply/position.rs:158-203`); `Exact::at` is public (`:294`); `placing.rs:138` and the test oracle build through them; `held` and `within` have no occurrence left. The root-surface fixture builds `100 / 3` from outside and compares it to the step.
- Non-blocking, done: obligation and verdict read one `conditions` array (`division.rs:421-437`, `conversion.rs:~255`). The regenerated stderr shows each refusal still firing at its own assertion.
- Left as reported: `saturated` dropping the distance, law cells 64, 65, 128, rows at `one_expert`.

### Blocking
1. **`mock/crates/arvo-format/src/division.rs:388-389` [public-surface]:** the rustdoc of public `quotient_position` still says "the one non-member that names the zero saturates toward the positive end". No `Divisor` names that slot and the branch is deleted. Delete the clause.

### Non-blocking
- `division.rs:278-280` says every caller of `placed` passes a `Divisor`'s slot or an `extreme_divisors` member. The test-only `unforced_position` (`:442`) passes raw slots; its callers in `the_edges.rs:226-230` and `:291-296` skip the pole. Narrow the sentence to name the test door.
- `Remainder` equality is of the pair, so a consumer's `Exact::at(s, Remainder::of(2, 6))` differs from the step's reduced third. Documented at `position.rs:156-157`; now reachable from outside.
- `Wide`, `over` and `placed` stay crate-private, so an outside operation forms its own numerator. Pass one did not ask for more.

Six new commit subjects in form, no bylines.

---

## Conflicts and order
- Each branch merges into `origin/dev` cleanly (`git merge-tree --write-tree`). Numeral is one commit behind `dev`, oracle is on its tip.
- Against each other: one conflict, `docs/FORMAT.md`, a generated file. Regenerate, never pick a side. `arvo-format/DESIGN.md.tmpl`, `README.md.tmpl`, `mock/Cargo.toml` and `Cargo.lock` auto-merge; the registry edits are in different files.
- No semantic clash found by reading: oracle removes no public item of `arvo-format`, and numeral's committed stderr quotes only `adapt.rs` and `slots.rs`, which oracle does not touch.
- **Order: numeral first, then oracle.** Oracle owes a fix commit anyway; it merges `dev` in, regenerates `docs/FORMAT.md` and carries N1 if the numeral landing did not.
- Neither helper's green was measured on the merged tree. Run `cargo test --release -p arvo-format -p arvo-numeral` and `cargo mock --lint-only` on the merge before pushing the second.

## Files
- `/home/user/arvo/mock/registry/ruling.toml`, `/home/user/arvo/mock/crates/arvo-numeral/src/admit.rs`, `/home/user/arvo/mock/crates/arvo-format/DESIGN.md.tmpl`, `/home/user/arvo/mock/crates/arvo-format/src/division.rs`
- Scratch copies: `/tmp/claude-0/-home-user/043fb1ca-2567-5c5e-93b5-7a1c2a432e48/scratchpad/p3-ruling.toml`, `/tmp/claude-0/-home-user/043fb1ca-2567-5c5e-93b5-7a1c2a432e48/scratchpad/p2-division.rs`, `/tmp/claude-0/-home-user/043fb1ca-2567-5c5e-93b5-7a1c2a432e48/scratchpad/p2-the_divisor.rs`, `/tmp/claude-0/-home-user/043fb1ca-2567-5c5e-93b5-7a1c2a432e48/scratchpad/p2-conversion.diff`. No repository file changed; the trial merges wrote loose objects into `.git` only.
