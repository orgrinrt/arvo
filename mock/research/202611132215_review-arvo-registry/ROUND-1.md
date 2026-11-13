# Review of `work/arvo-registry`, round 1

A fresh reviewer read the branch at `276dab95` against its base `faec24c6` on `origin/dev`, on a worktree at the tip, and returned do not merge. Its gates were green: `cargo mock --lint-only --commit` passed and the lint crate passed 881 tests with 35 ignored. The findings below are its own, in its order, each followed by what the branch did with it in the commit that carries this file.

## The findings and what was done

1. Two ratified rows had their edges and notes rewritten: `ruling::what_the_written_history_shows_to_be_his_is_ratified_by_default` from `corrects` to `amends` with an `amended` field, and `ruling::the_panel_finishes_the_canon_without_him` from `supersedes` to `corrects`. He answered "The eighteen, not the notes" when asked whether agents correct ratified rows' notes, and the branch's own schema says a ratified row is edited only by him. Done: both rows are back to `dev`'s edges and notes. The retyping is a question for him, ticket `5b3097e2a695`, with three options, and the `amends` edge stays in the schema with no user until he answers. The stamp sentence of the first row's note, which the branch had rewritten, is now a `corrects_note` proposal, `proposal::the_written_history_notes_schema_now_has_a_stamp_field`.

2. `ruling::there_is_no_universal_answer_take_the_win_and_gate_it` was raised on two of three readings, the third brought in after the first two disagreed. The second reader's objection is on substance and the record bears it out: at `88_op_...:101` he names "one universal solution" as the anti-pattern, not the asking of the question, and the row's first sentence says the asking is. Done: the row is back at `stated` as `dev` has it.

3. Fifteen ratified rows gained or extended a `ratification` citing his record. The reviewer opened the citations and found they hold, and could not settle from the canon whether `ratification` is content under his answer. Kept, as evidence added to a record field and no claim changed; the coordinator is told so with the report.

4. `a-ratification-names-a-record-of-his-words` took any backticked span starting with a digit and ending `.md` as a capture, so a research path or `1.md` passed. Done: a capture is now a bare file name of the store's stamp shape (eight to twelve digits, optionally a four-digit clock, then a lowercase slug), and three tests show the research path and `1.md` refused and both stamp shapes passing. Whether the file is in the store is not checked, since the store is outside the tree.

5. The same lint's message told the author to cite a `mock::` file, which it refuses. Done: the message names his round file or `INTENTS` and a capture, with a test.

6. `an-amendment-quotes-the-sentence-it-replaces` matched a substring where its schema promises a sentence, so `old: The` passed. Done: a quotation must end on a sentence's closing mark and start and end on sentence boundaries in the target, bold markers taken out on both sides. The platform-width correction now quotes its two stale sentences whole. Two tests refuse a fragment and a clause, one passes a sentence set in bold.

7. The same lint skipped a target row with no `note` or `says` silently. Done: only an absent row is left to the engine, and a quotation of a field the row does not have is `not-in-target`, with a test.

8. `no-proposal-supersedes-a-ratified-one` refused a superseding proposal that is itself ratified, the repair its own message names. Done: such a proposal is skipped, with a test.

9. The stamp fields' description said a pack lint checks them, and no lint of that name is in arvo's pinned pack. Done: the description says it checks once arvo takes the pack at a revision that has it, and each `stamped_by` field carries `FIXME(bc400834f3a7)`, the ticket for the pin.

10. Commit messages `276dab95`, `7d9183a0`, `0030dc4a` and `ec21a18d` and the comment at `mockspace.toml:46` carry workflow words in a public repository. The commits are pushed and are not rewritten; the comment is now a `FIXME` naming its ticket and nothing of who decides. Later messages say what changed.

11. The `EDGES` constant had been put between `rank`'s doc comment and `rank`. Done: it sits above the comment.

12. Two rewritten notes kept their bold. Done: the min-plus proposal's note has none, and the written-history ruling's note is `dev`'s again.

13. `PROVISIONAL.md` still listed the raised row as provisional. Resolved by 2: the row is provisional again.

## Checks

The lint crate passes 890 tests with 35 ignored, and `cargo mock --lint-only --commit` is green. A clause planted in place of a whole sentence in the platform-width correction was refused on the real registry, then taken out.
