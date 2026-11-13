# Self-review 2 of `work/arvo-lints`

A second fresh `pr-reviewer-senior` agent on opus, run once over the branch at tip `c64a94ba` against `origin/dev` (merge-base `faec24c6`), after the fixes to the first review. The brief named the branch, its base, the first review's report and dispositions as claims to check, the governing material by path and five questions, and gave no verdict. The report is the agent's, as returned. The dispositions follow it.

## Report

VERDICT: merge after fixes

Gates. Canon gate passed, against the in-force constraints in `arvo/.claude/CLAUDE.md` and, for the one registry row edit, against `ruling::a_composed_expressions_region_is_never_inherited_from_its_parts` (`ruling.toml:1815` already cites `215::442`). The reviewer is the second reader on that edit and agrees with the first. The canon is silent on source-comment markers, as the doc changelist says.

Test gate. Lint pack at `c64a94ba`: 972 passed, 0 failed, 19 ignored, all 19 job-6 `half_up` arms. `axis-undeclared-rows` 14 of 14 and `cited-passages-check` 22 of 22, run in scratch copies. Every new or changed test body was read. Probes ran in a scratch copy of the pack; nothing in the clone or the worktree was touched.

### Findings

1. Medium, tests and correctness. `mock/lints/no_registry_comment_counts_its_own_rows.rs:91-105` and `:155-165`, tests at `:242-279`. Ticket 11 is closed against planted cases that avoid the real shapes the ticket names. Two of the three comments `CHECK.md` 1.7 cites still pass the widened lint. Probe output on the original text (from `git show 326b52f5^`): "The six below were added after the first port..." gives 0 findings; "The four below were not in the sweep..." gives 0; "Fifteen of the sixteen rows below" gives 1. Other misses: "Twenty-one rows below." gives 0, because `one` is not in `NUMBER_WORDS` and every `-one` compound fails; "Seventeen new rows follow." and "17 proposal rows below." give 0, because any word between the number and the noun hides the count. The planted cases were picked to pass the new code. Fix: plant the ticket's own three comments verbatim as a test that must fail; accept `<number> below|above|follow` with the noun left out and one or two words between number and noun; let a compound's `one` part count while a bare `one` stays excluded.
2. Medium, round claims. `mock/design_rounds/202611132036/202611132036_changelist.doc.lock.md:15`. The doc changelist says a grep of `mock/**/*.md.tmpl` for "the carried preset defaults" finds no instance. That is false: `mock/crates/arvo-strategy/DESIGN.md.tmpl:68` ("the other three presets' choices are marked `// FIXME:` at the declaration") and `:186` ("marked `// FIXME:` at each impl"), and the same stale form in source prose at `mock/crates/arvo-strategy/src/lib.rs:28`. The design now prescribes, in so many words, the bare marker form that the round's own lint refuses at `HARD_ERROR`. Fix: deprecate the doc changelist and supersede it with one that amends both template lines to `FIXME(<ticket id>):`; the src changelist then also covers `lib.rs:28`.
3. Low, correctness. `mock/lints/crate_root_paths.rs:61-68`, `:88-93`, `:224`. Comparisons use the raw identifier text, so `r#` raw identifiers slip past: `extern crate r#std;` gives `[]`; `fn f() { r#std::process::exit(0); }` gives `[]`; `extern crate std as r#s;` followed by `s::y()` misses the use. Fix: strip a leading `r#` in `text()` before comparing or inserting an alias, with the three inputs as tests.
4. Low, correctness. Smaller misreads in `crate_root_paths.rs`. An alias of an alias: `extern crate std as s; use s as t; t::x();` gives `[1, 2]` and misses line 3 (`collect_aliases` compares against `krate` only); the file is still refused. Aliases are not scoped: `mod m { use std as s; } mod n { use other as s; fn f() { s::x(); } }` gives `[1, 2]`, a false positive. A macro metavariable root: `$x::std::y` gives `[1]`, because `:236` does not list `metavariable`. Fix: add `metavariable` to that list, collect aliases to a fixpoint over `names`, and catalogue the scoping case as an ignored test with its ticket.
5. Low, correctness. `mock/lints/a_fixme_names_its_ticket.rs:81`. `strip_prefix("FIXME")` takes any word that starts with `FIXME`: `// FIXMEs here are tracked in the lane.` gets a hard error. Fix: require the next character to be `(`, `:` or whitespace, and add the case as a silent control.
6. Low, tests. `mock/lints/canon_lint_testkit/tests.rs`, `control_a_lint_that_only_warns_does_not_pass_the_blocking_check`. It asserts mockspace's own `Severity` constants and touches no arvo code, so it is not a control for anything in this repository, and it is redundant under the test gate. The real control is the overlay test beside it. Fix: delete it.
7. Low, rules (`writing-style.md`, rustdoc renders). New bold in `//!` and `///` on new files: `a_fixme_names_its_ticket.rs:16`, `a_registry_row_link_names_a_row.rs:16,22,28`, `crate_root_paths.rs:12,19,24`, `a_lint_guards_a_crate_that_exists.rs:42`, `canon_lint_testkit/severity.rs:15`, `canon_lint_testkit/declared.rs:74`. Fix: remove it.
8. Low, prose (`pr-text-is-published-writing-in-ops-voice.md`). Commit subjects: `62c2212e` "fix: close the review findings on the lints and tools" says how the change was produced, not what changed; `edda54a8` and `167cd054` leave the tool names unbackticked. Fix: reword before landing.
9. Note, drift. `cited-passages-check/src/lib.rs:400-405`. A block with one unreadable evidence file and a match in the other is `OK` with `skipped=1`, and the totals line has no skipped count. Fix: add `skipped` to the totals.
10. Note, outside the repo. `/home/user/report-arvo-lints.md:59-60` still says "twelve phrasings" and "11 tests". The code now has 13 and 14.

### Checked and sound

- `crate_root_paths` survived 34 probes: nested and rooted brace lists, `self` aliases, globs in braces, `super` and `self` roots; `pub use`, generics, qualified self, patterns, attributes and derives; multi-line paths, `macro_rules` bodies, `crate::std` inside macros, `x . std::y`.
- The round-1 findings 1 and 7 are fixed with controls.
- `code_only_source::comment_lines` and the `FIXME` lint: block interior lines, doc comments, strings and prose mentions. The six marker ids resolve in `tickets-arvo.toml:661-681`, and the upstream ticket exists in `tickets-mockspace.toml:2663`.
- The moved tests in `canon_citations/tests.rs` are verbatim. All touched files are under 500 lines, apart from the untouched-in-substance `no_prose_citation_into_nothing.rs`, which was 1139 lines on `dev` too.
- `a_registry_row_link_names_a_row`: wrong namespace, unknown slug and file stem each planted, with a right-namespace control. The only remaining stem links are in the frozen catalogue.
- Job 4: commented-out gates, with a live-gate control. Job 5: `arrivals` logic. Job 7: the replacement slug exists and is superseded. Job 8: the overlay and its controls, whose textual limits are ticketed as `1f9f55fff7c3`.
- `cited-passages-check`: the verdict split, the skipped lines and the bad-margin branch. The `verify.pl` differences in the doc comment are accurate against the script.
- `axis-undeclared-rows`: the row argument check, the new cue and its planted test.
- Registry comment rewordings are forced by the widened lint and touch comments only.
- No em or en dashes in added lines. No banned vocabulary. No bylines.

## Dispositions

| finding | disposition | where |
| --- | --- | --- |
| 1 | fixed: the ticket's three comments planted verbatim; a place word without a noun, up to two words between number and noun, compounds ending in `one`, a count stopping at a clause end; two more counts found in the registry and reworded | `c32a3fab` |
| 2 | fixed: round `202611132059` amends both template lines and the module doc line, and corrects the first round's claim in its own changelist; the closed round is not edited | `4dbb4c02`, `08cb7afe` |
| 3 | fixed, three inputs as tests | `c32a3fab` |
| 4 | fixed: `metavariable` listed, aliases collected to a fixpoint; the scoping case is an ignored catalogue test under `crate_root_paths_reads_a_use_alias_in_one_module_as_the_crat` with a ledger row | `c32a3fab` |
| 5 | fixed with silent controls and the bare marker still firing | `c32a3fab` |
| 6 | fixed: deleted | `c32a3fab` |
| 7 | fixed in the six files, bold removed from their doc comments | `c32a3fab` |
| 8 | not changed: the commits are pushed and posted, and a landing here is a merge commit that keeps each subject, so a reword would move shas already named. The two unbackticked subjects name tools in plain words | |
| 9 | fixed: the totals line carries the skipped count, with a test | `c32a3fab` |
| 10 | fixed in the report | `report-arvo-lints.md` |
