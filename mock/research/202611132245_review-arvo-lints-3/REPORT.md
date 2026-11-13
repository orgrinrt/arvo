# Self-review 3 of `work/arvo-lints`

A third fresh `pr-reviewer-senior` agent on opus, run once over the branch at tip `f77366c2` against `origin/dev` (merge-base `faec24c6`), after the fixes to the second review. The brief named the branch, its base, the earlier reports and dispositions as claims to check, the governing material by path and the questions, and gave no verdict. The report is the agent's, as returned. The dispositions follow it.

## Report

VERDICT: merge after fixes

Canon gate passed. I checked against the in-force constraints in `arvo/.claude/CLAUDE.md` and the registry rows that touch markers (`ruling.toml:1605` is consistent). The canon says nothing about how a source-comment marker is written, as round 202611132059 also says. I did not query the registry with `cargo mock query`, because that would regenerate the pack, so that check rests on a grep.

Test gate passed. Lint pack at `f77366c2` (clone HEAD confirmed): 978 passed, 0 failed, 20 ignored, which is the 19 job-6 arms plus `an_alias_in_one_module_is_not_the_crate_in_another`. I ran that arm with `--ignored` and it is red (`[1, 2]` against `[1]`). `axis-undeclared-rows`: 14 passed. `cited-passages-check`: 23 passed (scratch copies). I read every new or changed test body and found no tautologies.

The probes are ad-hoc spikes and are not committed: the count matcher extracted verbatim into a rustc binary, and `crate_root_paths.rs` copied into a scratch crate against the same `mockspace-lint-rules` checkout. Each probe establishes one input and output pair, nothing more.

## Findings

1. Medium, correctness and fix-the-class. `mock/lints/no_registry_comment_counts_its_own_rows.rs:42`. `COUNTED` only accepts `row/rows/entry/entries`. The registry names its rows by namespace, so a count written with that noun passes.
   - Live instance: `mock/registry/ruling.toml:1798` says `# --- composition contracts above the numeral: four rulings out of ten proposals ---`, and exactly 4 `[[ruling]]` rows follow it before the next header. The lint reports nothing.
   - The module doc (lines 39-41) says "the noun has to name what the file itself holds", and in `ruling.toml` that is rulings.
   - Fix: add the declared namespace nouns, singular and plural, to `COUNTED`. Plant `# four rulings below` as a must-fail case and reword line 1798.

2. Medium, correctness. Same file, `:46` and `:130`. With the `PLACE` arm, any number directly followed by `below`, `above` or `follow(s)` is a hard error, including numbers that count nothing. Probe outputs:
   - `# Widths up to 64 follow the same rule.` gives `("64","follow")`
   - `# See row 3 below.` gives `("3","below")`
   - `# The error stays 3 above the bound.` gives `("3","above")`
   - Smaller ones from the number-word widening: `# The two entry points into this file.` gives `("two","entry")`, and `# The score rows below.` gives `("score","rows")`.
   - The real registry has no hit today. Every one of these would force a sound comment to be reworded.
   - Fix: take the place word only when the number opens the sentence or follows a determiner (`the`, `these`, `those`, `all`). Plant the first three inputs as silent controls.
   - Lesser misses in the same arm: `Six more follow.`, `The four listed below.` and `Six (new) rows below.` all give nothing, because `)` reads as a clause end.

3. Medium, scope and fix-the-class. `mock/lints/a_fixme_names_its_ticket.rs:47-49`. It is a crate lint over Rust comments only, so the bare `# FIXME: this variant does not build...` markers stay unrefused.
   - They are at line 1 of `mock/benches/variants/{structural-decomposition,xxhash3,spectral-bisection,fnv1a}/Cargo.toml`.
   - `mark-placeholders-fixme.md` names TOML explicitly, and the ticket is "nothing refuses them". The report says `arvo_fixmes_carry_no_ticket_id_and_nothing_refuses_them` "can close on this branch landing", which is not true yet.
   - Also at `:83-87`: the new boundary only accepts `(`, `:` or whitespace, so `// FIXME, carried default.`, `// FIXME.` and `// FIXME-x` are silently passed.
   - Fix: treat any non-alphanumeric, non-underscore character as the boundary, with those three as must-fire cases. Either give the four bench markers tickets plus a check that reaches TOML, or keep the ticket open and say so.

4. Low, round claims.
   - `mock/design_rounds/202611132059/202611132059_topic.the-marker-form-in-the-strategy-design.md` says "The generated `docs/STRATEGY.md` regenerates from the template." No `docs/STRATEGY.md` exists at `faec24c6` or at the tip.
   - `202611132036_topic.a-fixme-names-its-ticket.md` says "Twelve markers ... already name a ticket". `git grep` at `faec24c6` finds 11 (11 + 6 = 17 at the tip).
   - Every other claim checks out: lines 68 and 186 of the template, `lib.rs:28` at `c64a94ba`, the src changelist's grep, and "no code line changes".
   - Fix: correct both sentences in a superseding changelist. Do not edit the closed rounds.

5. Low, rules (`writing-style.md`, rustdoc renders).
   - The branch adds bold: `mock/lints/canon_citations.rs:165` `/// **Where this is wrong.**`.
   - The round-2 sweep fixed only the six files it was told about. Files this branch edits still carry bold doc lines: `no_registry_comment_counts_its_own_rows.rs:18,22`, `canon_citations.rs:8,23`, `a_standing_is_reachable_from_what_it_cites.rs` (4), `canon_lint_testkit.rs` (9).
   - Fix: remove at least the added one, and those in the files edited since round 2.

6. Low, report accuracy (`/home/user/report-arvo-lints.md`). It is stale against the tip:
   - Line 34 still credits job 8 with "a control that advisory and build-gate severities fail the blocking check", which `c32a3fab` deleted.
   - Line 51 says "17 tests pass"; the module runs 16.
   - Line 112 says "972 passed, 19 ignored" and "`cited-passages-check` 22"; it is 978/20 and 23.
   - Line 38 says 18 `FIXME` lint tests; it is 19.
   - There is no section for the round-2 dispositions.
   - Fix: update those lines.

7. Low, prose (`pr-text-is-published-writing-in-ops-voice.md`). The `c32a3fab` subject, "plant the ticket's own comments against the count lint", describes how, and covers one of its six changes. The body leaves out the deleted control, the skipped count, the bold removal and the catalogue row. Rewording is not possible after the push; noted only so the follow-up record names what that commit did.

8. Low, prose. `mock/registry/question.toml:1821`: "found two rows ... giving incompatible answers" became "found rows", which loses a historical fact (the number of conflicting rows seat 215 found), not a count of the file's own rows. It was forced by the lint's noun-only reading. Fix: name the two rows by slug.

9. Note, correctness. `crate_root_paths.rs:75-78`: `extern crate self as std; fn f() { std::x(); }` reports line 2, a false positive. It is contrived and low-value; a catalogue arm is enough.

## Checked and sound

- `crate_root_paths`: 23 further probes behave correctly: raw identifiers, alias fixpoint both orders, `$x::std` and `$crate::std` in macro bodies and raw tokens, turbofish, qualified self, attributes, `cfg_attr`, struct expressions, patterns, `pub(in std::x)`, nested and generic token trees, raw strings, doc strings. The file is 451 lines.
- The ignored scoping arm: it has its `mock/catalogue.md` row, its key resolves in `.shared/scripts/life/tickets-arvo.toml`, and it fails when run.
- The count lint over every real registry comment gives zero findings. Its tests plant the ticket's three comments; the first two fail on the `c64a94ba` code, and the third ("sixteen rows") would have passed there too.
- The `FIXME` lint: every ticket id at a marker in `mock/crates` resolves in the lane file. The `FIXMEs`/`FIXMEd` controls are paired with must-fire markers.
- The deleted testkit control asserted engine constants only. The overlay control at `tests.rs:266` still drives each gate down.
- `cited-passages-check`: the skipped total is summed and tested, and that test fails on the old format string.
- The registry rewordings in `probe.toml:1216`, `proposal-the-later-topics.toml:563`, `proposal-the-value-stance.toml:282` (I verified each row below supersedes one above) and `proposal.toml:7-8` are true.
- The strategy template and module doc now match the markers.
- No em or en dashes and no banned vocabulary in added lines. Commit subjects are under 72 characters, lowercase, with no parentheses and no bylines. The worktree is clean.
- Not checked: `cargo mock --lint-only`, which I did not run so as not to regenerate the pack or touch the clone.



## Dispositions

| finding | disposition | where |
| --- | --- | --- |
| 1 | fixed: the file's own `[[ns]]` nouns count (singular and plural), `# four rulings below` planted as a must-fail, `ruling.toml:1798` reworded without a number; the widened lint found `question.toml:1802` ("the two questions"), reworded | round 3 commit |
| 2 | fixed: a place word counts only when the number opens the comment or follows `the`, `these`, `those` or `all`; a singular noun only after the digit 1; `score` is no number word; `)` is no clause end; the three probes plus `The two entry points` and `The score rows below` are silent controls | round 3 commit |
| 3 | boundary fixed: any non-alphanumeric, non-underscore character after `FIXME` ends the word, with `FIXME,`, `FIXME.` and `FIXME-x` planted as must-fire and `FIXME_x` silent. The TOML half is ticket `85b00facadb9`; `arvo_fixmes_carry_no_ticket_id_and_nothing_refuses_them` stays open and the report says so | round 3 commit |
| 4 | recorded here as errata, no third round: round `202611132059` cites `docs/STRATEGY.md`, which does not exist, and round `202611132036` says twelve markers already named a ticket where eleven did. Closed rounds are frozen records and the claims carried no change | |
| 5 | fixed: bold removed from the doc comments of `canon_citations.rs`, `a_standing_is_reachable_from_what_it_cites.rs`, `canon_lint_testkit.rs` and `no_registry_comment_counts_its_own_rows.rs` | round 3 commit |
| 6 | fixed in `report-arvo-lints.md`, with the round 2 and round 3 dispositions | |
| 7 | not changed: the commit is pushed and a landing keeps each subject as a merge commit | |
| 8 | fixed: `question.toml:1821` now says "found a pair of rows in one topic" | round 3 commit |
| 9 | catalogued: an ignored arm `an_alias_of_self_is_not_the_crate` under the alias ticket, whose ledger row now says aliases are read by name over the whole file, not by what they point to or the module they sit in | round 3 commit |

Round 3 is the last: the branch lands with what is left recorded as follow-ups, the TOML marker check (`85b00facadb9`), the alias scoping (`crate_root_paths_reads_a_use_alias_in_one_module_as_the_crat`) and the severity overlay copy (`1f9f55fff7c3`).
