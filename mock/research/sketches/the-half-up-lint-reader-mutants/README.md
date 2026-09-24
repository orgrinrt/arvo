# The clause reader, broken arm by arm

Whether the lint's corpus notices each arm of the reader being broken. The
reader that stood before the rewrite was mutated the same way in
`the-half-up-lint-mutants/`, at `b8a7ce4b`, and every arm it had then is covered
there. This one covers the arms the rewrite added at `df9d286f` and the arms the
anaphora round added on top of them: the antecedent bound to a name rather than
to a backtick, the reach the carry has across clauses, the clause with a subject
of its own, the denotation masked out of what counts as another name, the
sentence-initial spelling of one, and the escape for a clause that states the
denotation before contrasting with it.

`WORKS`: all 21 mutants are caught. One survived the first run and the corpus
gained the pair that kills it.

## How it was run

`reading.rs` was replaced for the run by `reading.mutant.rs.txt` here, which is
that module with a `mutant()` switch read from `HALF_UP_MUTANT` at each arm. The
shipped module was written back afterwards, and the mutant copy is not in the
tree the lint runs from. Each mutant was run as

```
HALF_UP_MUTANT=<name> cargo test --manifest-path mock/target/mockspace-lints/Cargo.toml half_up
```

with `none` as the control, which is the mutant build with every switch off and
passes at 81 tests. `out/runs.txt` is the raw output, filtered to the failing
test names and the result line.

Everything except `mutant()` itself is the shipped module byte for byte, so a
diff of the two files is exactly the list of arms this asks about.

## The mutants

The antecedent and its reach: `backtick` (the carry is refused after two
backticks, which is what the arm this round replaced did), `noother` (nothing
in the other-name list defeats the carry), `nocarry` (nothing is carried from
one clause to the next), `nopass` (a clause naming nothing drops the carry
rather than passing it on), `pronounonly` (only a clause opening with a pronoun
consults the carry), `pronounany` (every clause is read as opening with one),
`codespan` (a clause opening with a code span is read as naming nothing).

What counts as another name: `nocap` (a name is matched only as written in the
list, so one opening a sentence is missed), `nomask` (the denotation is not
blanked, so `floor` inside `floor(x + q/2)` reads as the floor mode).

The denotation escape: `nodenote` (a clause stating the denotation before the
reading is not excused).

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

## What this run found

`nocap` survived. The arm it breaks is the one matching an other-name at the
start of a sentence, and it was put into the reader by watching the lint fire
over the tree rather than by writing the sentence down, which is the same way
four arms escaped the previous run. `Toward zero reads the sign of the slot`, in
`arvo-format/DESIGN.md.tmpl`, was the only thing holding it, and a test suite
that depends on one sentence of one document staying as it is holds nothing.

The pair that kills it is `half_up` adding half a step and flooring, followed by
a clause that either says a tie goes away from zero, which the carry reaches and
the reader has to refuse, or opens on `Toward zero`, which names another rule and
stops the carry. The corpus runs to 38 pairs.

One mutant from the previous run is gone rather than renamed. `antecedent`
carried a spelling even where something else was named after it, and the arm it
broke counted backticks. That arm does not exist now, and `backtick` restores it
as a mutant so the corpus is asked about the thing that replaced it.

## What it does not show

That the arms are right about English. It shows each arm is load-bearing for
some test, not that the word lists are complete: a negator, a marker, a
direction word or an other-name missing from a list is a clause this lets
through or refuses wrongly, and no mutant of the code finds that. The corpus is
where that is argued, one pair at a time.
