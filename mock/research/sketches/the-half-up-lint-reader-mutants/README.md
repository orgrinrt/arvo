# The rewritten clause reader, broken arm by arm

Whether the lint's corpus notices each arm of the rewritten reader being broken.
The reader that stood before this one was mutated the same way in
`the-half-up-lint-mutants/`, at `b8a7ce4b`, and every arm it had then is covered
there. This is the arms `df9d286f` added: the antecedent carried across clauses,
the three ways a negator binds, the two extra conditions on the list escape, and
the two filters on what counts as a reading. None of them existed when the first
sketch ran, so none of them was mutated by it.

`WORKS`: every one of the 14 mutants is caught, after four that were not.

## How it was run

`reading.rs` was replaced for the run by `reading.mutant.rs.txt` here, which is
that module with a `mutant()` switch read from `HALF_UP_MUTANT` at each arm. The
shipped module was written back afterwards, and the mutant copy is not in the
tree the lint runs from. Each mutant was run as

```
HALF_UP_MUTANT=<name> cargo test --manifest-path mock/target/mockspace-lints/Cargo.toml half_up
```

with `none` as the control, which is the mutant build with every switch off and
passes. `out/runs.txt` is the raw output, filtered to the failing test names and
the result line.

## The mutants

Anaphora: `nocarry` (nothing is carried from one clause to the next, so a
pronoun never finds a name), `antecedent` (a spelling is carried even where
something else is named after it), `pronounany` (every clause is read as opening
with a pronoun).

The negator's binding: `readafter` (a negator behind the reading in the
reading's own segment does not count), `nameafter` (one behind the name does
count), `nobetween` (a negator between the reading and a later name does not
count), `betweenboth` (one between them counts in either order), `noconj` (a
conjunction does not end a negator's reach).

The list escape: `twoseg` (two segments are a list), `nobare` (the item holding
the reading may hold anything else too), `neighbour` (no neighbouring item has
to be a bare name).

What counts as a reading: `noqual` (`in magnitude` counts with no direction word
in front of it), `nomention` (a reading quoted as a name is read as a reading),
`mentionspan` (any reading inside a code span is read as a mention).

## What the first run found

Five survivors, and each is a separate thing the corpus was not asking.

`readafter`, `nobetween`, `betweenboth` and `nobare` survived because the
sentences that drove those four arms into the reader were never written down as
sentences. Three of the four are prose this repository actually carries, and the
arms were added by watching the lint fire over the tree rather than by putting
the sentence in the corpus, so the tree was the only thing holding them. Four
pairs were added to `sentences.rs`: the alias clause from
`symmetry/tests/the_classification.rs`, the reflection partition from
`arvo-format/DESIGN.md.tmpl`, a negator in an aside between the name and the
reading, and a list item carrying more than the reading it holds. The corpus
runs to 25 pairs now and kills all four.

`termcase` survived for a different reason and is not in the list above, because
the arm it broke is gone. `term_at` looked a term up among the spellings and the
readings, folding case to do it, and it is reached from one place, on a reading's
own offset, with every reading written in lower case already. The spellings half
and the folding were both unreachable. It is `reading_at` now, over the readings
alone, and what made the folding unnecessary is asserted rather than assumed:
`every_reading_is_written_lower_case_so_a_lowered_clause_can_be_searched_for_it`,
with the same filter run over the spellings as its control, since those do carry
upper case and the filter has to find them.

## What it does not show

That the arms are right about English. It shows each arm is load-bearing for
some test, not that the word lists are complete: a negator, a marker or a
direction word missing from a list is a clause this lets through or refuses
wrongly, and no mutant of the code finds that. The corpus is where that is
argued, one pair at a time.
