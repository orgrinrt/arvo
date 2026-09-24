# The `half-up-is-not-a-magnitude-rule` lint, broken arm by arm

Whether the lint's own suite notices each of its arms being broken. A lint whose
tests pass with an arm deleted is not testing that arm, and a lint reporting
nothing over the real tree says as much about a dead arm as about clean prose.

`WORKS`: every one of the 28 mutants is caught.

The run is of the reader as it stood at `b8a7ce4b`, which is the revision the
mutant copies here were taken from. It was rewritten at `df9d286f`, and the arms
that rewrite added are mutated in `the-half-up-lint-reader-mutants/` rather than
here: this file is not re-run against a module it no longer matches.

## How it was run

The two reader modules were replaced for the run by the copies here,
`reading.mutant.rs.txt` and `comments.mutant.rs.txt`, which carry every arm of
the shipped modules behind a switch read from `HALF_UP_MUTANT`. The shipped
modules were written back afterwards; neither mutant copy is in the tree the
lint runs from. Each mutant was run as

```
HALF_UP_MUTANT=<name> cargo test --manifest-path mock/target/mockspace-lints/Cargo.toml half_up_is_not
```

and `none` is the control, the mutant build with no arm broken, which passes.
`out/runs.txt` is the raw output, filtered to the failing test names and the
result line.

## The mutants

Reader of a passage, `reading.rs`: `rowblank` (a table row is not blanked out
of the clause text), `offset` (a hit's offset is taken from its clause rather
than the passage), `row` (tables not read), `subject` (a doc block's item not
applied), `between` (the negator window ends at the earlier of the two rather
than the later), `list` (no list escape), `onecomma` (one comma counts as a
list), `clause` (the whole passage is one clause), `dot` (a `.` inside a token
cuts a clause), `newline` (every line break cuts a clause), `neg` (no negator
escape), `nt` (no `n't` escape), `else` (no marker escape), `bound` (matches
inside identifiers count).

Reader of a Rust file, `comments.rs`: `lineof` (every finding on the passage's
first line), `fourslash` (`////` read as a doc comment), `trailing` (a comment
after code joins the run above it), `nojoin` (consecutive lines not joined),
`kinds` (doc and plain comments joined), `gap` (comments across a blank line
joined), `nest` (block comments do not nest), `string`, `escape`, `raw` and
`char` (each kind of literal not skipped, or its escapes not honoured),
`nosubject` (no doc block bound to its item), `noskip` (attributes between the
doc block and its item not skipped), `itemcomment` (a comment on the item's
line counted as naming it).

## What the first run found

Three mutants survived the first run: `offset`, `raw` and `escape`. Every test
of an offset had its hit in the passage's first clause, where the clause offset
and the passage offset are both zero, and the literal test held no raw string
or escaped quote that would expose a comment if read wrongly. A test was added
for each, `an_offset_counts_from_the_passage_in_a_later_clause_and_a_later_row`
and three more lines in `nothing_inside_a_literal_is_prose`, and the three were
run again, which is the tail of `out/runs.txt`. The first run's `offset`
survivor is not in the file, which holds the second run of the reading mutants
onward.

## What it does not show

That the arms are right about English. It shows each arm is load-bearing for
some test, not that the word lists are complete: a negator or a marker missing
from the list is a clause this lets through or refuses wrongly, and no mutant
of the code finds that.
