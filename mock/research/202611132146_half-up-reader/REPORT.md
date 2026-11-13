# The half_up reader and its eighteen catalogued sentences

This records the job on two tickets, `arvo_s_half_up_reader_gets_eighteen_catalogued_sentence_pair` and `arvo_s_catalogue_ledger_holds_18_half_up_clause_shapes_under`, done on branch `work/arvo-lints-half-up` from `126d0790` and merged into `work/arvo-lints`. The work was done by an opus sub-agent and the text below is its report, kept as returned apart from the layout. The sub-agent could not write it to the repository, so the session landing the branch wrote it here. The counts under "Checked on the merged tree" were run again on the merge.

The reader in `mock/lints/half_up_is_not_a_magnitude_rule/` got eighteen catalogued pairs wrong on one named side, and the arm that reads the ratified rulings was red on the governing ruling's `promotion` field. Seventeen of the eighteen are closed, and so is the ruling arm. One is left, because a regression arm asserts the opposite reading of the same construction.

The governing row is `ruling::half_up_denotes_ties_toward_positive_infinity`, read from `mock/registry/ruling.toml`. It says `half_up` is `floor(x + q/2)`, a tie going toward positive infinity at every sign, and that ties away from zero is the alias. No catalogue entry states anything that row contradicts on the side it names. The canon says nothing about how a pronoun resolves, which is the one place below where a call would rest on a single seat, and it was not made.

## Before

- `CARGO_INCREMENTAL=0 cargo test --release half_up` in the pack: 103 passed, 0 failed, 19 ignored.
- `half_up -- --ignored`: 0 passed, 19 failed. Each entry's own `catalogue_case!` test asserts the side the reader gets wrong, so each was red before the change that closed it.
- The ratified arm, run with a temporary print of each hit's context, failed on one clause only, in the `promotion` field: "270 read blind, barred from 228, ... and first answered ties away from zero".
- `cargo mock --lint-only`: all lints passed, 46 warnings, none from this lint.

## Each shape and its outcome

The number is the entry's place in the old `CATALOGUE`. The closed pairs are in `sentences/closed.rs`, in the words they were catalogued in, and each has a test asserting both sides. None was deleted, and `every_twin_in_the_catalogue_reads_right` ran after every change.

| # | Shape | Outcome and test | What the reader does now |
|---|---|---|---|
| 0 | `Round` opening a clause as an imperative | closed, `round_opening_a_clause_on_an_object_is_an_imperative` | A capitalised method name opening the text is not a name where the next word is in `IMPERATIVE_NEXT`: determiners, pronouns, prepositions, `aside`. |
| 1 | `Fix` opening a clause as an imperative | closed, `fix_opening_a_clause_on_an_object_is_an_imperative` | Same rule. |
| 2 | `Ceiling aside` | closed, `ceiling_opening_an_aside_is_not_the_clauses_subject` | Same rule, on `aside`. |
| 3 | `Round to it` | closed, `round_as_a_verb_taking_the_mode_is_not_the_method` | Same rule, on `to`. |
| 4 | A code span holding a number opens a clause | closed, `a_code_span_value_opening_a_clause_is_not_a_subject` | A span whose body is a digit, behind an optional sign or point, is a value, so the clause carries the mode. |
| 5 | The alias named under a reading word, `roundTiesToAway` | closed, `the_alias_named_under_a_reading_word_is_a_name` | A reading spelled as an `OTHER_NAMES` entry is a name, not a reading, where its segment does not name the mode and it either heads the segment or has a rule noun behind it. |
| 6 | A relative after "is not Java's rule" | closed, `a_relative_binds_to_a_rule_the_mode_is_denied_to_be` | A relative binds to the thing denied where a negator stands between the mode's last spelling and the segment. |
| 7 | A clause opening on "Seat 270" | closed, `a_clause_opening_on_a_seat_has_a_subject_of_its_own` | A clause has a subject of its own where it opens on a bare whole number, or on a `SUBJECTS` noun with or without an article. |
| 8 | The reading before `rather than` and the denotation | closed, `a_negator_denying_the_denotation_behind_the_reading_leaves_it` | A negator behind the reading, with the denotation behind it, belongs to the denotation. |
| 9 | The denotation and the reading joined by a bare comma | closed, `a_denotation_excuses_only_a_reading_set_against_it` | Across segments, the denotation excuses the reading only where `where` or `wherever` opens the reading's segment. |
| 10 | A one-word aside in brackets, read as a list neighbour | closed, `a_parenthetical_beside_the_reading_is_not_a_list_item` | A segment that is the whole of one pair of round brackets is skipped when looking for a neighbour. |
| 11 | `toward zero` in ordinary words, read as the rule | closed, `toward_zero_behind_a_verbs_object_is_a_direction` | The lower-case phrase is the rule only behind a `NAME_LEADS` word, punctuation, or nothing. |
| 12 | A pronoun in one clause after "differs from `floor`" | closed, `a_pronoun_in_one_clause_binds_past_a_denied_rule` | A pronoun opening a segment skips a rule the clause denies the mode is. A relative still binds to the nearest. |
| 13 | `value` used as a predicate | closed, `value_behind_a_verb_of_giving_is_what_the_mode_gives` | `value` and `values` quote a span only where no `RESULTS` verb stands ahead of it. |
| 14 | A direction before `in magnitude` that names an error's growth | closed, `in_magnitude_nearest_an_error_is_the_errors` | The qualified reading counts only where a tie noun stands nearer than an error noun, or there is no error noun. |
| 15 | The ties-away rule named in words | closed, `the_ties_away_rule_named_in_words_is_a_name` | Rule 5, through the rule noun behind it. |
| 16 | "The crate" as a subject | closed, `the_crate_named_as_subject_has_the_clause` | Rule 7. |
| 17 | `It` after a negated identity across a full stop | still ignored, `#arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi` | Unchanged. |
| arm | No ratified ruling field is refused | closed, `no_ratified_ruling_field_is_refused`, now asserted | Rule 7 reads "270 read blind" as a seat. |

## The one left

Entry 17 wants "`half_up` is not Java's `HALF_UP`. It sends a tie away from zero." to fire, reading `It` as the sentence's subject.

`carry_tests::every_other_name_defeats_the_carry` asserts the same construction silent for every other name, "`half_up` is not `HALF_UP`. It sends a tie away from zero." among them. The silent twins of closed 11 and 12 rest on the same model: across a full stop, the rule standing last takes the pronoun. The only word that differs between the catalogued sentence and the asserted one is "Java's".

A spike tried the other model: in `antecedent`, a name the clause denies (a negator ahead of it in its segment) does not take the carry. With it, `cargo test --release half_up` gave 112 passed and 5 failed: `every_other_name_defeats_the_carry`, `control_a_pronoun_points_at_whatever_was_named_last`, `a_pronoun_opening_a_segment_binds_past_a_rule_the_clause_denies_the_mode_is`, `every_lead_makes_the_lower_case_toward_zero_the_rule`, and `every_twin_in_the_catalogue_reads_right`, because the entry's own twin, "That one goes away from zero", also opens on a pronoun. Telling `It` apart from `that one` would mend the twin and still break the other four. The spike is out.

Which model the gate reads by is not something the canon settles, so the call is filed as a question for two agreements, ticket `arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi`, holder coord. Its options are to keep last-name-takes-the-pronoun and delete the entry, or to adopt subject-takes-`It` after a negated identity and rewrite `every_other_name_defeats_the_carry` and the twins of closed 11 and 12.

## Two entries added

Closing shapes 0 to 3, 7 and 16 left the catalogue with no refused-true entry, and the lint's module doc rests its warning severity on having one. Each fix is a closed list, so each class keeps a residue the lists do not reach. Both residues are now catalogued, each under its own ticket.

| Entry | Pair | Ticket |
|---|---|---|
| `an_imperative_on_an_open_class_object_is_read_as_the_method` | "Round halfway ties away from zero" is let through; "Round sends halfway ties away from zero" is silent | `#arvo_s_half_up_reader_reads_a_capitalised_method_name_as_the` |
| `a_subject_named_in_words_outside_the_vocabulary_is_refused` | "Python's decimal module sends a tie away from zero" is refused; "Its decimal module sends a tie away from zero" fires | `#arvo_s_half_up_reader_refuses_a_clause_whose_subject_is_name` |

Both were red under `--ignored`, and both twins read right, before they were committed.

## The ledger

`mock/catalogue.md` loses the eighteen row, which no ignore cites any more, and gains one row per remaining id. Every `#[ignore]` reads `catalogue: <gap>; tracked #<own id>`.

## After, as reported

- `CARGO_INCREMENTAL=0 cargo test --release half_up`: 136 passed, 0 failed, 3 ignored.
- `half_up -- --ignored`: 0 passed, 3 failed, the three entries above.
- The whole pack: 1014 passed, 0 failed, 5 ignored. The other two ignored are the `crate_root_paths` catalogue entries.
- `cargo mock --lint-only`: all lints passed with 46 warnings, the same 46 lines as before by a sorted diff and none from this lint. The real registry and doc corpus read exactly as before. No touched file is over 500 lines: `catalogue.rs` went from 564 to 282.

The new tests are the walks in `shape_tests.rs` over every word of `IMPERATIVE_NEXT`, `NAME_LEADS`, `SUBJECTS` and `ERRORS`, `DENOTES` against `NEGATORS`, and the alias spellings that are also readings, each beside the same sentence without the word; the pronoun test in `carry_tests.rs`; the subject walk for a code span; the `RESULTS` walk in `the_words_that_give_a_reading_are_not_mention_words`; the unit tests in `reading/words.rs`; and the parenthetical walk in `an_empty_segment_between_two_delimiters_is_not_a_list_item`, which replaces an assertion that had pinned shape 10 silent.

## Checked on the merged tree

Run on `097a663e`, the merge of the branch into `work/arvo-lints`: `cargo mock --lint-only` reports all lints passed; the pack gives 1014 passed, 0 failed, 5 ignored; `-- --ignored` gives 5 failed, the three `half_up` entries and the two `crate_root_paths` entries. These match the figures above.

## What was not done

- Entry 17 stays open, for the reason above.
- The two residue entries are open by construction: closing either needs the reader to tell a verb from a noun beyond its closed lists.
- The severity, the lint configuration and every existing catalogue sentence are as they were.
