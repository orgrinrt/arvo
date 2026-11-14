# Review of `work/arvo-format-member`, pass 1

**Date:** 2026-11-14
**Branch:** `work/arvo-format-member` at `306512ea`, range `6f80ce21..306512ea` (round `202611140438`).
**Reviewer:** `pr-reviewer-senior` on opus, fresh, one pass, on a detached worktree of the tip.

The report as it came back, verbatim.

Canon gate passed. I checked it against `ruling::never_a_runtime_check_and_one_lowered_path` (ratified, op), `ruling::the_ingest_door_waits_on_a_known_use` (stated) and `question::what_a_datum_stands_for` (answered in part). Nothing in the change classifies a value at run time, and nothing fills an open question: `Member<F>` is a slot refined by its range, not a datum. I am the first reader, so a second independent canon read is still owed.

The test gate failed.

## Verdict

DO NOT MERGE. The gate is what blocks it, not the code. I found no wrong verdict in the obligation. The fix is a bounded set of tests and text edits, so the next pass is a re-review, not a rethink.

## Findings

1. `mock/crates/arvo-format/src/member/tests.rs:52-54` and `:57-66` [tests]: these tests cannot fail.
   - `outside(&SIGNED) == 0` and its two siblings run over constants the author picked inside the range. `constant` stores `Slot::at(B)` whatever its check does, so the assertion holds even if the door's check is deleted.
   - `the_perimeter_check_reports_a_constant_door_with_no_range_check` only shows that the test's own helper (`is_within`) works on `assumed(0)` and `assumed(256)`. No shipped door ever reaches that helper with a value outside the range.
   - This is a fabricated planted-mutation test: it has the look of a mutation check and checks nothing. Delete both.
   - Replace them with a compile-fail pair one below `MIN`; only the top side is pinned today. Also check the constant door at `MIN` and `MAX` at every admitted width of both families, with a macro over literal widths; `DESIGN.md.tmpl:2250-2253` already claims this is tested.
2. `src/division/tests/the_oracle.rs:79` with `the_member_door.rs:122-137` [tests]: the property the door rests on is never tested, and the sweep covers only a sample.
   - `pair` builds the expected answer from `value::<S::Format>(b)`, never `value::<G>(b)`. So the sweep assumes the door's grid claim (`division.rs:135-137`, "a slot of `G` denotes the value the same slot of `F` does") instead of testing it.
   - Only two pairs are swept, both radix 2, exponent 0, phase 0.
   - Two pairs the table admits are never divided: `FromOne`/`FromOne`, the whole non-zero phase where the sign of `-n` matters, and `Biased`/`Quarters`, the reduced-phase equality case.
   - Fix, part one: in `ByMembers`, assert `value::<G>(b) == value::<F>(b)` and build the expected answer from `G`'s value.
   - Fix, part two: sweep every admitted row of the table, plus one negative-exponent pair and one decimal pair.
   - Fix, part three: for every format in the table, assert `no_member_names_the_zero::<G>()` equals `members::<G>().all(|b| value::<G>(b).num != 0)`. The independent check already exists in `division/tests/mod.rs:37-60`; the hand-written table is the author restating the code.
3. `src/apply/tests/totality.rs:84-92` [tests]: the in-range assertion is made on `adapt`'s output, not on the member. The member reaches it only through an equality with the function it wraps. Assert `is_within` on `Member::adapted(..).slot()` directly.
4. `the_member_door.rs:226-241` [tests]: redundant. Once `disagreements::<Shipped>() == 0` passes, the result is fixed by the table's literals. Make it a check over the literals or drop it.
5. `DESIGN.md.tmpl:1018-1020`, `:1050-1053`, `:2250-2253`, and `member.rs:16-17` [spec]: the text says "both doors" are fed the edges and the ends of the coordinate at every admitted width. The constant door is tested at three ranges and not past the bottom. Finding 1 closes this; otherwise the text has to say less.
6. `member.rs:62`, `:79` [drift]: neither door forces `<F as Format>::ADMITTED` (`format.rs:135`, the phase denotes) or requires a constant quantum.
   - "Serves a constant-exponent quantum only" (`member.rs:76`, design `:1021`) is stated but nothing enforces it.
   - So `Member<F>` builds for a phase that does not denote, and for a float format where the crate's own `contains` (`format.rs:160`) needs a magnitude for membership.
   - Refuse both at build, each with a compile-fail pair, or mark it `FIXME(<ticket>)`.
7. Registry `proposal::a_quotient_s_divisor_is_non_zero_by_type` [drift]: its `says` still lists two doors, and its `because` keeps the compromise "until a slot can be typed by membership". This branch removes that compromise from the design, so the registry's record of the stance is now false. Append a superseding proposal on topic `validation` in this branch.
8. `202611140438_changelist.doc.lock.md` "NO CHANGE" [naming]: "`Member` is a value type rather than a coordinate" contradicts the design's "`Member<F>` is a slot of `F`'s range". That label is the only reason given for leaving the coordinate table unchanged. Fix it forward in the design.
9. `member.rs:58-59`, `:70-71`, `division.rs:264-265` [naming]: these rustdoc summary sentences run 90 to 125 characters. Split each one.
10. Pre-existing, outside this diff: `arvo-numeral` `tests/a_consumer_crate.rs` is red.
    - The consumer's `Cargo.lock` pins notko as `git+ssh://` while `mock/Cargo.toml:64` declares https. The branch touches neither file.
    - A ticket is owed; I filed nothing because the review is read-only.
    - My `cargo mock check` run rewrote that lock. I restored it from `HEAD`, the tree is clean, and the parked copy is in `/home/user/arvo/.git/safe-restore/20261114T053510Z-pid17526`.

## Notes

What holds up:
- The five verdicts in `member_conditions` are correct. `-n` cannot overflow, and a negative denominator is normalised.
- Each compile-fail file differs from the admitted one by exactly one token.
- `docs/FORMAT.md` matches the template in both changed sections.
- `cargo mock check` reports the lints green (strict).
- The commits are well-formed with no bylines, and the diff adds no em-dashes and no bold.
- Suite: `arvo-format` 337 tests plus 33 compile-fail, all green. The rest of the workspace is green except finding 10.

The public-surface checklist file is not in this tree's `.claude/rules/`. I worked from the workspace card only, not the `pr-review` skill's full list.

Findings 1 to 3 are the ones the gate turns on.
