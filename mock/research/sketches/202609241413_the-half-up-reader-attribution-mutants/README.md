# The clause reader after the attribution round, broken arm by arm

Whether the lint's tests notice each arm of the reader being broken, from
`523cf149`, the first commit carrying round `202609241413`'s source, plus the
three walks this sketch added to the suite. The reader was mutated the same
way in `the-half-up-lint-reader-mutants/` at the round before; every arm it
asked about is asked again here against the split reader, and the arms this
round added are asked for the first time: which rule a reading's own segment
is about, a leading conjunction read past to the pronoun, a denied spelling
kept out of the carry, the conjunction rule on the denotation and the segment
that bounds it, a list neighbour found past an empty segment, `reading` kept
out of the mention words, and the history lint's phrases matched as words.

`WORKS`: all 37 mutants are caught, 36 of the reader and 1 of the history lint,
and every one of them by at least one test other than the corpus's aggregate
arm.

## How it was run

`run.sh` here, from the repository root. It replaces four files by the
`.mutant.rs.txt` copies beside it, each the shipped file with a `mutant()`
switch read from `HALF_UP_MUTANT` at each arm: `reading.rs`,
`reading/cutting.rs` and `reading/escapes.rs` under
`mock/lints/half_up_is_not_a_magnitude_rule/`, and
`mock/lints/no_project_history_in_published_prose.rs`. The two submodules reach
the switch through `super::mutant`. It runs each reader mutant as

```
HALF_UP_MUTANT=<name> cargo test --manifest-path mock/target/mockspace-lints/Cargo.toml half_up
```

and the history mutant under the `no_project_history` filter, then copies the
shipped files back and touches them so the next build cannot reuse the mutant
one. `none` is the control under each filter: 102 passed under `half_up`, 15
under `no_project_history`, both against the mutant copies with every switch
left at its shipped arm. The restore itself runs once, in `run.sh`'s exit
trap, after every mutant in the loop; nothing here re-runs the suite against
the restored files to check the swap-back took, so that is not a claim this
sketch makes. `out/runs.txt` is the raw output, filtered to the failing test
names and the result line.

Everything except the switches is the shipped file byte for byte, so a diff of
each pair is exactly the list of arms this asks about. One expression is
respelled rather than switched: the between rule in `negator_binds` reads
`r.min(n) .. r.max(n)` in the mutant copy so `betweenboth` can run it in either
order, which is the shipped slice whenever the shipped guard `r < n` holds.

## The mutants

Carried over from the round before, same names, same breakage:

The antecedent and its reach: `backtick` (the carry is refused after two more
backticks), `noother` (nothing named after the mode defeats the carry),
`nocarry` (nothing is carried), `nopass` (a clause opening with a pronoun and
naming nothing drops the carry rather than passing it on), `pronounonly` (a
clause naming nothing consults only the doc block's item, never the carry),
`pronounany` (every clause is read as opening with a pronoun), `codespan` (a
clause opening with a code span is read as naming nothing).

What counts as another name: `nocap` (a name opening a sentence is missed),
`nomask` (the denotation is not blanked, so `floor` inside `floor(x + q/2)`
reads as the floor mode).

The denotation escape: `nodenote` (never excused).

The negator's binding: `readafter`, `nameafter`, `nobetween`, `betweenboth`,
`noconj` (a conjunction does not end a negator's reach, in the segment rule or
the between rule).

The list escape: `twoseg`, `nobare`, `neighbour`.

What counts as a reading: `noqual`, `nomention`, `mentionspan`.

Added this round:

Attribution: `noattrib` (never attributed to another rule, clause or row),
`noattribclause` (only in a clause), `noattribrow` (only in a table cell),
`segmine` (the mode named ahead of the reading in its segment does not keep
it), `segnamed` (another rule named ahead of the reading in its segment does
not take it), `segback` (a segment pointing back never asks what stood last),
`lastany` (any other rule named before the segment takes the reading, even with
the mode named after it).

Pointing back: `relonly` (only a relative points back, a pronoun opening the
segment does not), `pronounsonly` (only a pronoun does, a relative does not),
`noconjskip` (a leading conjunction is not read past).

The carry: `negatedante` (a denied spelling is still an antecedent).

The denotation: `nodenoteconj` (no conjunction ends the excuse, which is finding
8), `denotewhole` (the conjunction rule reads the whole stretch from the
denotation to the reading, which is what this round shipped first and what the
pre-commit gate caught on the oracle's comment).

The list: `emptyneighbour` (the empty segment a bracket and a comma cut counts
as a neighbour item).

Mentions: `mentionreading` (`reading` and `readings` count as mention words,
which is finding 3).

The history lint: `unboundedphrases` (phrases matched as substrings, which is
finding 5).

## What this run found

The first run caught every mutant, and five of the carried-over ones, `nocap`,
`readafter`, `nobetween`, `betweenboth` and `nobare`, only through the corpus's
aggregate arm, `every_pair_fires_on_one_side_and_is_silent_on_the_other`. That
is the state the round before left them in: each had one pair, so the arm was
held, by one negator or one name, and a failure there names the arm only
through the pair's `differs` line. Three walks were added and the run repeated:

- `every_negator_is_bound_by_where_it_sits_and_not_by_being_between`, in
  `tests.rs`, walks every negator behind the reading in its own segment, one
  segment further on, opening a list the name is an item of, and in an aside
  between the name and the reading. Its first form put the negator in the
  name's own segment for the list case, which the name-segment rule answers
  before the between rule is asked, and `nobetween` passed it; the list shape
  is what isolates the between rule, and the second run shows it failing under
  `nobetween`.
- `every_other_name_opening_a_sentence_capitalised_stops_the_carry`, in
  `carry_tests.rs`, walks every plain name whose capitalised form differs.
- `every_reading_is_a_list_item_only_where_its_item_holds_nothing_else`, in
  `segment_tests.rs`, walks every reading as a bare item and as an item with a
  word in front of it.

After it no mutant is caught by the aggregate arm alone. The aggregate arm still
catches 34 of the 36 reader mutants; the two it misses, `nomask` and `twoseg`,
are caught by the denotation walk and the catalogue's twin arm, and by
`control_what_is_not_a_boundary_keeps_the_clause_whole`.

## Which test holds which arm alone

A stricter question is whether a mutant has a test that fails under it and
under no other. Ten do: `mentionspan`, `neighbour`, `nobare`, `nocarry`,
`nomention`, `noother`, `noqual`, `pronounany`, `twoseg` and
`unboundedphrases`, counted with `noattrib` left out because it is the union of
`noattribclause` and `noattribrow` by construction. The rest share their
targeted walk with a neighbouring mutant, because the walks cover whole lists
and adjacent arms of the reader decide overlapping parts of them. The sharpest
case is `a_denotation_conjoined_with_the_other_reading_does_not_excuse_it`,
which fails under `nodenoteconj`, `neighbour` and `emptyneighbour`: its comma
spelling puts the reading in a segment behind the closing bracket of
`floor(x + q/2)`, so the list escape is asked the question too, and a broken
list escape answers it.

## What it does not show

That the arms are right. A mutant being caught says a test depends on the arm,
not that the arm reads English correctly, and the lint's module doc and
`sentences/catalogue.rs` carry the eight sentences it is known to read wrong.
