# Independent read 1 of `c4d665d0` (half_up reader), as returned

VERDICT: do not merge

The fixes close the seventeen catalogued sentences, but seven of them do it by widening a closed list, and that widening breaks neighbouring sentences the old reader already read correctly, in both directions. The new tests walk the reader's own word lists against the reader, so none of them can catch this. The report says this branch is already merged into `work/arvo-lints` (`097a663e`), so that merge carries everything below.

Canon gate: passed. Checked against `ruling::half_up_denotes_ties_toward_positive_infinity` (ratified by experts): `half_up` is `floor(x + q/2)`, ties away from zero is the alias (Java `HALF_UP`, Python, IEEE `roundTiesToAway`, MATLAB `fi` `Round`). Nothing in the diff contradicts it. No registry row names the lint; it exists under the workspace rule `a-gotcha-becomes-a-lint`. A second independent read is owed.

Test gate: failed.
- The pack gives 136 passed, 3 ignored; `--ignored` gives 3 failed. On the parent, `--ignored` gives 19 failed, so every closed pair and the ruling arm was red before the change.
- Lists walked against themselves: every walk in `shape_tests.rs:71-181` iterates `IMPERATIVE_NEXT`, `NAME_LEADS`, `SUBJECTS`, `ERRORS`, `DENOTES` x `NEGATORS`, and the walk on `RESULTS` in `tests.rs:193-196` does the same. Each feeds a list's own words back to the reader that consults that list, so it cannot fail for a word the list got wrong. Each has one hand-picked control, and none walks the direction that regressed.
- Not tautological, but weak: the count pins (`closed.rs:163`, `catalogue.rs:113`) and the computed list (`shape_tests.rs:57`).
- Unit tests in `words.rs:158-219`: the inputs are real, but there is no case where a preposition follows the method name (the shape behind finding 1).

## Findings

The regression probe compares the old and new reader on one sentence list, in scratch only: `/tmp/halfup-review-pack/probe/sents.rs`, with old and new output in `old.txt`, `new.txt` and `out.txt` beside it. It is not committed, so it is reproduction steps and not evidence. Old is `126d0790`, new is `c4d665d0`.

True sentences the old reader passed and the new one refuses

1. High. `reading/vocabulary.rs:418-422` and `words.rs:41-48`: `to` and `toward` in `IMPERATIVE_NEXT` strip the MATLAB method name.
   - Input: "`half_up` sends a tie toward positive infinity. Round to nearest sends a tie away from zero."
   - New fires, old was silent. Same with "Round to nearest goes away from zero in `fi`" and "Round toward nearest is MATLAB's, and it sends...".
   - Fix: a preposition entry counts as an imperative only where a determiner or pronoun follows it ("Round to it"). Add "Round to nearest sends..." as a silent twin.
2. High. `words.rs:94-95`: a number in a code span no longer counts as a subject, even when the clause hands the reading to a rule.
   - Input: "`half_up` maps `-2.5` to `-2`. `-2.5` under Java's rule goes away from zero."
   - New fires, old was silent.
   - Fix: catalogue it as a refused-true pair at least. Better, keep the clause as its own subject when it names a rule in words ("under X's rule").
3. High. `escapes.rs:256,264-269`: `denies_the_denotation` also fires on a copula.
   - Input: "For `half_up`, away from zero is not toward positive infinity."
   - New fires, old was silent. The sentence is true.
   - Fix: apply it only where the reading is a verb's complement. Here the reading is the subject of "is not".
4. Medium. `vocabulary.rs:476`: `CONTRASTS` holds only `where` and `wherever`.
   - Input: "`half_up` is floor(x + q/2), as opposed to away from zero."
   - New fires, old was silent.
   - Fix: add `as opposed to` and the other contrast connectives, walked both ways.
5. High. `words.rs:77-82` and `vocabulary.rs:432-436`: a gerund ahead of `toward zero` names the rule, and `NAME_LEADS` misses it.
   - Input: "`half_up` is floor(x + q/2). Adding sign(x) q/2 and truncating toward zero sends a tie away from zero."
   - New fires, old was silent. The sentence describes the alias. Same with "Rounding toward zero after adding half..." and "..., and rounding toward zero sends...".
   - Fix: an `-ing` verb directly ahead counts as a lead.

False sentences the old reader refused and the new one lets through

6. High. `words.rs:132-151` and `reading.rs:201-210`: an alias spelling at the head of a segment after a conjunction is read as a name even when the segment points back to the mode.
   - Input: "`half_up` is nearest, and roundTiesToAway is what it does."
   - New silent, old fired. Same with "..., and ties-away is its rule." and "..., where ties-away is what it does."
   - Fix: skip `a_reading_stands_as_a_name` wherever `points_back` holds in that segment.
7. High. `escapes.rs:278-280` and `reading.rs:91-93`: `denied_since` treats any negator as a denied identity.
   - Input: "`half_up` is nearest and not biased, which means a tie goes away from zero."
   - New silent, old fired. Same with "`half_up` never sends a tie down, which is why it goes away from zero."
   - Fix: require a copula, then a negator, then a noun phrase ("is not X").
8. High. `words.rs:92-117`: `opens_on_a_subject` takes the whole clause even when a pronoun later in it points back to the mode, and treats any bare integer as a seat.
   - Inputs: "`half_up` is the ruled mode. Every test shows it sends a tie away from zero." and "`half_up` sends ties up. 5 of them go away from zero."
   - New silent, old fired, for both.
   - Fix: a pronoun pointing back keeps the clause with the mode. A bare number counts only when a verb follows it, not "of".

The pronoun call and the documentation

9. Medium. `reading.rs:95,111-114` against `carry_tests.rs:404-410`: the change makes the pronoun-resolution call the report says it declined to make.
   - The report says the canon is silent on how a pronoun resolves, so it was not made. But fix 12 (`past_denied`) makes exactly that call inside a clause: "differs from `{n}`, and it..." fires. Across a full stop the suite asserts the opposite model: "differs from `{n}`. It..." is silent.
   - That ships one seat's call in one place and refuses it in the other.
   - Fix: put fix 12 under the same question ticket and the same two agreements.
10. Medium. `half_up_is_not_a_magnitude_rule.rs:35-37` and `:94-96`: the module doc and the `SEVERITY` doc now say the gate rests on a single refused-true entry.
    - Findings 1-8 are not catalogued, so the "incomplete in both directions" list at `:39-71` is false as shipped.
    - Fix: catalogue each of findings 1-8 as a pair (`catalogue-edge-cases-as-tests`), then rewrite both docs.

## Questions

1. Closed shapes against the ruling: each catalogued sentence reads correctly. Neighbours do not: findings 1-8. Shapes 0-3, 4, 5/15, 6, 7/16, 8, 9 and 11 each regress at least one sentence the old reader had right. Shapes 10, 12, 13 and 14 held in the neighbours tried.
2. Test gate: every closed pair has a planted case that was red first. The walks do not, in the direction that regressed.
3. The ignored pronoun entry and the two residue entries: the conflict is real, `carry_tests.rs:121-126` asserts "`half_up` is not `HALF_UP`. It sends..." silent and the only differing word is "Java's". Leaving the entry open under a question ticket is correct, apart from finding 9. Both residue entries are real: red under `--ignored`, twins read correctly.
4. Ledger, ids, sizes, style: `mock/catalogue.md` has one row per remaining id and the ignore reasons agree. The three ids are in the live `/home/user/.shared/scripts/life/tickets-arvo.toml` (about line 981); `/home/user/tickets-arvo.toml` is a stale copy. Largest file `reading.rs` at 467 lines. No em-dashes, bold, curly quotes or FIXME/TODO in added lines. The commit subject is 62 characters and in form.
5. Severity, configuration, existing sentences: `SEVERITY` and `mockspace.toml` unchanged. All 36 old catalogue sentences survive verbatim across `catalogue.rs` and `closed.rs`, and 4 residue sentences are added. `corpus.rs` is unchanged.

## Checked and sound

- The 19 entries were red before and 3 are red after.
- The ruling arm `no_ratified_ruling_field_is_refused` passes and is asserted.
- `no_sentence_the_corpus_asserts_is_one_the_catalogue_says_is_wrong` now spans `CLOSED`.
- The `segment_tests.rs:94-104` change deletes an assertion that had pinned shape 10 in its wrong reading. Deleting it is correct.

## Dispositions

The work was resumed and answered in `63095320`. Findings 1 to 8: each sentence is a neighbour pair in `sentences/neighbours.rs` with a twin on the other side of the line, and each reader change has a pair or a walk that goes red when it is undone (checked one change at a time in a scratch copy). Finding 9: the in-clause pronoun reading was removed and its entry sits under the pronoun question ticket `arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi`. Finding 10: the module doc and the `SEVERITY` doc were rewritten to the known cases. The second read of `63095320` found three of these answers still incomplete or regressing, recorded in `202611132350_half-up-read-2`.
