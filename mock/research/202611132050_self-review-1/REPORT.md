# Self-review 1 of `work/arvo-lints`

A fresh `pr-reviewer-senior` agent on opus, run once over the branch at tip `edda54a8` against `origin/dev` (merge-base `faec24c6`). The brief named the branch, its base, the governing material by path and five questions, and gave no verdict. The report below is the agent's, as returned. The dispositions follow it.

## Report

VERDICT: merge after fixes

Gates. Canon gate passed. Checked against the in-force constraints in `arvo/.claude/CLAUDE.md` and, for the one registry content edit, against `ruling::a_composed_expressions_region_is_never_inherited_from_its_parts`. The reviewer is the first expert on this, so a second read is owed. Test gate: the lint pack at tip `edda54a8` gives 949 passed, 0 failed, 19 ignored. All 19 ignores are the job-6 `half_up` catalogue arms. `axis-undeclared-rows` passes 11 and `cited-passages-check` passes 17. `cargo mock --lint-only` on the tip reports 46 warnings and all lints pass, which matches the report. Every new or changed test body was read, with no tautologies found.

### Findings

1. Medium, correctness. `mock/lints/crate_root_paths.rs:92-109`. A path nested in a `use_list` under a `scoped_use_list` is read as rooted at `std` or `alloc`, contradicting the module doc and the `no_std_enforcer` doc. Reproduced: `use facade::{std, core};` gives `[1]`; `use facade::{std::fmt};` gives `[1]`; `use my_crate::{alloc::Pool, x};` gives `[1]` for alloc. Fix: work out the root by walking up through `use_list` and `scoped_use_list` to the outermost path, for the `identifier` arm and the scoped arm. Keep these three inputs as silent controls.
2. Medium, scope. Job 9 is parked as `mock/research/202611131930_a-fixme-names-its-ticket/a_fixme_names_its_ticket.rs.txt`. A `.rs.txt` in research is not a home for a check (`a-check-lives-in-lints-or-tools.md`), and its 18 tests cannot be run from the tree. The reason given for not opening a round misreads `a-round-is-a-feature-across-crates.md`: "One narrow exception: a defect found and fixed stands alone." Six markers with no ticket are such a defect against `mark-placeholders-fixme.md`. The check is workspace-general, but no upstream ticket for `mockspace-extra-lints` is named, which the brief requires. Fix: open the defect round, land the lint with the six marker edits, and file the upstream ticket.
3. Medium, scope. Job 6 was not attempted. The brief's clause is "If it resists, stop and post `waiting`"; the report says it was read as resisting before starting. Nothing was tried, so nothing resisted. Report it as declined, not as compliance.
4. Low-medium, tests. `mock/tools/axis-undeclared-rows` has no positive control against the known answer. The ticket names 14 hand-found instances (`CHECK.md` 1.10), two split over a TOML line continuation, and the report says it did not reconcile against them. Fix: a committed reconciliation, or a test, showing each of the 14 appears in the tool's output.
5. Low, correctness. `axis-undeclared-rows/src/lib.rs:109-114`. A row argument naming no row, for example `proposal::typo`, prints "no sentence in a `gap` or `note` says an axis is undeclared", a claim about the whole registry. Fix: say that row has no candidates, or that it does not exist.
6. Low, correctness. Three problems in `cited-passages-check/src/lib.rs`. Lines 350-354: a block with evidence but no `passage:` is labelled `NOEVID`, so "81 with no evidence" mixes two cases; give it its own verdict. Lines 357-360: one unparseable `evidence:` line voids the other, checkable lines in the block; check the parseable ones and note the unparseable one. Lines 119-123: an unparseable margin silently becomes 0; return inconclusive. Separately, the tool was never compared with `verify.pl` on the same input, so "the same check" (line 14) is not shown.
7. Low, correctness. `crate_root_paths.rs:57-73`. The aliases in `use std::{self as s};` and `use ::std as s;` are not collected, so `s::process::exit(0)` is not reported. The declaration line is still reported, so the crate is still refused. Fix: add both to alias collection, with tests.
8. Low, rules (`file-size-limit.md`). Three lint files crossed 500 lines on this branch: `canon_citations.rs` 303 to 581, `a_standing_is_reachable_from_what_it_cites.rs` 431 to 605, `canon_lint_testkit.rs` 498 to 552. Fix: move the tests into a sibling file, as `canon_lint_testkit/tests.rs` already does.
9. Low, drift (`extract-the-shared-logic-never-mirror-it.md`). `canon_lint_testkit.rs:416-466`. `severity_after_config` is a hand-written copy of the engine's TOML config overlay. It misses `[lints."catalogue-ignore"]`, dotted keys under `[lints]`, inline tables and single-quoted values, so with any of them the arm stays green while the gate is off. Fix: file a ticket to get the overlay from the engine.
10. Low, drift. The write hook is still a line scan that mirrors `no-bare-numeric` (ticket `3d101a63b02a`). `a_registry_row_link_names_a_row.rs:130-159` adds a second backticked-link scanner beside `no_prose_citation_into_nothing.rs` `tokens`, where ticket 10 asked for that lint to be widened. Fix: extract one shared scanner or file a ticket.
11. Low, prose. Two registry comment rewrites changed what the comment says. `mock/registry/proposal.toml:7-8`, "rulings name rows between them ... alone ratifies several", is garbled; name the `ratifies` relation without a count. `mock/registry/proposal-the-value-stance.toml:282`, "the rows above, superseded", now reads as every row above.
12. Low, report accuracy. The report gives job 4's commit as `fd497a5d`, which is on no branch; the commit on the branch is `42c8efd7`.
13. Note, scope. `mock/registry/proposal.toml:693` adds the `215::442` citation, a registry edit outside the `mock/lints` and `mock/tools` lane. The content is right: the ratified ruling's own `provenance` already cites `215::442` as an independent instance.

### Checked and sound

- `code_only_source.rs`: blanking keeps byte length and line breaks, lifetimes are left alone, multi-byte text is safe; a char literal is blanked and a lifetime on the same line is kept.
- The four enforcers now read the parse; every arm that must stay silent has a control that still fires. Tickets 2 (`Vec::`, `Box::`) and 3 (a fully qualified `std::` path) are closed with controls.
- Write hook regex checked by hand: hex, binary and octal take integer suffixes only; a range in front of a suffixed literal is denied; field access and identifiers ending in digits are allowed; char literals are stripped before strings.
- The two hook arms now run for real through `repo_root` instead of returning early.
- `a-lint-guards-a-crate-that-exists`: line, block and multi-line block comments, each with a live-gate control.
- `members_named` and `arrivals`: both real pairs (`91`/`106`, `215`/`244`) are arms; the real `90` preamble yields the members the doc describes.
- Job 7: the corpus string exists at `234_dolan...:114` and `220_orchard...:999`.
- Job 8: arms over the whole pack and the config overlay, each gate turned down in turn as a control.
- Job 11: number words, the shape `one` excluded, with controls.
- `a-registry-row-link-names-a-row`: wrong namespace, unknown slug and file stem each planted and refused; only two file-stem links remain under `research/`, both in the frozen catalogue.
- Commit subjects under 72 characters, lowercase imperative, no parentheses or bylines; no em or en dashes and none of the banned vocabulary in added lines or commit messages.
- The worktree and the clone were clean.
- Not checked: the `pr-review` skill's public-surface sweep. No README or root file is touched, and both new `Cargo.toml` files declare MPL-2.0 with `publish = false`.

## Dispositions

| finding | disposition | where |
| --- | --- | --- |
| 1 | fixed, three inputs as silent controls | `62c2212e` |
| 2 | fixed: defect round `202611132036`, lint landed with the six edits, upstream ticket `d0ff4306c13c` | `72e9e907`, `17eb11c6` |
| 3 | accepted: the report says declined, not attempted | `report-arvo-lints.md` |
| 4 | fixed: reconciled against the hand search; one row missed, a cue added with a planted test | `62c2212e`, `d76f998c` |
| 5 | fixed, inconclusive with a test | `62c2212e` |
| 6 | fixed: `NOPASS` verdict, unusable evidence lines skipped and counted, bad margin inconclusive; the doc comment states the differences from `verify.pl` instead of claiming sameness, because no reader output was committed to run both on | `62c2212e` |
| 7 | fixed with finding 1 | `62c2212e` |
| 8 | fixed: tests and two helpers moved to sibling files, all three under 500 lines | `62c2212e` |
| 9 | ticket `1f9f55fff7c3` | |
| 10 | ticket `1ecf2a9cb485`; the write hook stays on `3d101a63b02a` | |
| 11 | fixed | `d76f998c` |
| 12 | fixed in the report | `report-arvo-lints.md` |
| 13 | kept, noted in the report as out of lane | `report-arvo-lints.md` |
