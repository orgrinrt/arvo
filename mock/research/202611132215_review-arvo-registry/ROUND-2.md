# Review round 2 of work/arvo-registry

The second round read the branch at `817a71c3` against the first round's thirteen findings and the three commits since. Its verdict was merge after fixes: the canon gate passed, `cargo mock --lint-only --commit` was green and the lint crate passed 890 tests with 35 ignored. One blocker was open, the workflow wording in six commit messages, and four smaller findings beside it. The dispositions follow, with the commit each fix landed in.

## Round-1 findings, as round 2 found them

Findings 1, 2, 5, 7, 8, 9, 11, 12 and 13 were resolved. Finding 4 was mostly resolved and finding 6 partly; both are closed below. Finding 3, the fifteen `ratification` citations kept on ratified rulings, stays as it is, by choice: his refusal at `OPS-WORDS.md:194` ("The eighteen, not the notes") is read as covering what a row says and its notes, and a citation of where his words are recorded is neither. Only one expert has read that call, so a second read is ticket `6f6b73df95a1`. Finding 10 is the blocker below.

## Round-2 findings and what was done

1. Workflow wording in the messages of `276dab95`, `7d9183a0`, `0030dc4a`, `ec21a18d`, `0812ad42`, and on a sweep `f21f1f48` as well. Six clean messages are written; rewording the commits needs a history rewrite and a force push of the branch, and the permission check refused the rewrite, so it went to the coordinator as blocked, with the messages attached.
2. `proposal::validation_is_two_acts_keyed_by_boundary_and_it_validates_maps` claimed the ingest door in `says` while its `gap` said it was not its claim. The ingest sentence is out of `says`. `81abf846`.
3. `ruling::a_value_alias_is_keyed_on_its_strategy_and_defaults_to_warm` quoted a word he did not write. The quote is his label alone, and `says` names the row direction from a plan choice. A second read is in `6f6b73df95a1`. `81abf846`.
4. `ruling::the_august_goal_and_the_panel_finish_each_bind_in_their_own_scope`: its note now says it states the ratified panel row's scope in that row's place, since that row's note is not edited. `81abf846`.
5. `a-ratification-names-a-record-of-his-words` checks a capture's name and not that it exists: `FIXME(f3053e2cccec)` on `is_a_capture`, and the ticket holds the check. `ea81f74f`.
6. Round-1 finding 6, a quotation starting after a colon: the colon is no longer a sentence start in `holds_sentence`, and the case is a test, seen failing on the old check. `81abf846`.

After the fixes `cargo mock --lint-only --commit` is green and the lint crate passes 891 with 35 ignored.
