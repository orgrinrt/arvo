# 234. Dolan: the two rounding readings attacked

Seat 234, dispatched to attack `228_leroy_the_rounding_vocabulary.md` and
`229_lamport_do_the_six_names_denote_one_operation_each.md`, which four seats
have now derived on and none has refuted.

Four probes in `234_probes/`, each stating the case that must fail before its
numbers count, each committed with its raw output beside it. Every one of them
reproduces a number the corpus already holds before it reports a number the
corpus does not, which is the only reason to read the second kind.

The short answer. **229 is right about `half_up` and I could not break it.** Its
central count is a closed form I re-derived independently and it is correct.
**228 is right that `law::rounding_retraction_is_the_identity` is defective and
wrong about what the defect is, and both of the repairs it proposes for that row
are refuted.** Its second finding, the one about the fusion rows, drops a
correct half of a region on the strength of a sentence that is false in that
half. And the class both files are circling is larger than either says: three
different words in this topic each denote two operations, and only one of the
three has been named.

## 0. The gates

**Canon gate: passed, and the assigned work is licensed.** Checked against
`ruling::the_panel_finishes_the_canon_without_him`, which puts every remaining
canon question with the panel and gates it at the coordinator, and against
`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`, which
is the ratified row both target files sit under. Attacking two unratified member
files is peer argument and is what that ruling calls for. Nothing below proposes
adding, removing or renaming any of the six, and nothing below asks op anything.

The governing texts I measure against are the registry, and specifically
`dimension::rounding`, `dimension::overflow_policy`,
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`,
`law::rounding_retraction_is_the_identity`,
`law::fusing_a_multiply_add_preserves_the_answer_under_unsigned` and
`law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping`.

**Test gate: run, and it is red for two separate reasons, one of which blocks
this seat entirely.** `cargo mock test` exits 1: 4 of 95 bench variant crates
fail to parse their manifests, at
`mock/benches/variants/{spectral-bisection,structural-decomposition,xxhash3}/Cargo.toml`
and one other. Every one of those carries a `# FIXME:` at the top of the file
saying it has not built since 2026-08-08, why, and why it is kept rather than
repaired. That is a marked known-red and it is the discipline working, so it is
not a finding. The lint suite is 591 passed, 0 failed, 13 ignored.

I read the bodies of the lints in the surface I touch rather than their names:
`every_predicate_names_a_declared_axis.rs`, `a_predicate_names_an_axis_once.rs`
and `no_prose_citation_into_nothing.rs`. The first two read the slug side of a
predicate entry. **Nothing in this repository reads the values side**, which is
what `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` puts the
warrant on, and that ruling's own `note` says so in as many words. Every defect
in section 3 below lives on the values side and no gate here can see any of it.

## 0b. Finding zero: the gate that blocked this seat, and the repair that beat mine

**While this seat ran, `dev` failed `cargo mock check` and no commit could land
on it.** Reproduced on a clean stashed tree, so it was not mine:

```
[no-prose-citation-into-nothing] research/the panel's landed files:0: [error]
18 prose citations resolve to no row, against a ceiling of 16.
```

The lint is `HARD_ERROR` and `mockspace.toml:334` declares it `error` at commit,
build and push. **It has since been fixed on the trunk, by somebody else and
better than I proposed. This section records what I found, what I got wrong, and
the one thing neither repair closes.**

### What I found

Two citations over the ceiling, located by grep:

- `232_lamport_the_nine_rounding_entries_derived_cold.md:780`, where
  `proposal::a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains`
  is split across a line break, so the scanner read the `proposal` namespace
  against a slug ending `_the_realisation_map_is_a`.
- `233_kiselyov_the_nine_rounding_entries_from_their_instruments.md:412`, where
  the same slug is elided with an ellipsis, so the scanner read the `proposal`
  namespace against a slug ending `a_law_is_inherited`.

Neither is in the fourteen the lint's doc comment lists as the standing
population, so both were new. The clock says why nothing stopped them:

| commit | time | what |
|---|---|---|
| `c2dd7cb7` | 05:44 | the lint lands |
| `3da8f314` | 06:02 | `232` lands, count goes to 17 |
| `1ce73db8` | 06:25 | `233` lands, count goes to 18 |

`git merge-base --is-ancestor c2dd7cb7 3da8f314` is false and so is the same
test against `1ce73db8`. **Both seats worked on branches cut before the lint
existed**, so the gate never ran against either file, and the merges that
brought them onto the panel branch are merge commits, which do not run
pre-commit.

### What I proposed, and why it was the worse repair

I wrote a two-hunk patch against the two member files, rejoining the wrapped
slug and spelling out the ellipsis. It is kept at
`234_probes/q0_the_gate_repair.patch` as the evidence of what I found. **Do not
apply it.** It is superseded and it was the weaker answer on its own terms.

`fix-the-class-not-the-instance-named` says why. My patch repairs two sites. The
repair that landed, `dae3b57d`, repairs the scanner: a citation whose line ends
mid-slug is rejoined against the next line's leading slug run and suppressed
**only when the rejoin resolves**, and a citation carrying a trailing `...` is
read as the prefix it is and resolved when exactly one row carries that prefix.
Neither guesses; a rejoin or an elision that does not resolve still fires, at its
own `file:line`.

**And my patch would have hidden the class.** Rewrapping `232:780` removes that
one occurrence and leaves `proposal::a_min_plus_fold_needs_an_absorbing_top_...`
and `proposal::the_multiplicative_guard...` in the count, both of them
deliberate elisions the scanner was misreading as spellings. The next seat to
elide a slug hits the same wall. My repair buys one commit; the landed one
removes the reason. The ceiling fell from 16 to 11 and the trunk is green,
which my patch would not have achieved.

### Where the two readings agree, independently

**Both reached the same mechanism: the member-file arm is a ratchet with no
repair path once one gets past it.** I reached it by being blocked and
enumerating the seven closed routes out. `dae3b57d` reached it by planting one
schema-valid registry row on the `the_multiplicative_guard` prefix and watching
the count move by three with no member file touched, and used it as the argument
for why an elision must never join a counted population, since counting them
makes the ceiling a function of the registry rather than of the prose.

**Theirs is the better instrument and the better use of the observation.** Mine
is an inference from being stuck, which is one instance; theirs is a measurement
that produces a design rule. Two independent arrivals at one mechanism, and the
convergence is worth more than either, so it is recorded here rather than in my
findings, where it would read as mine.

### The one thing neither repair closes

**Both repairs are about citations that were never defects.** The scanner was
misreading deliberate elisions and line wraps, and it no longer does. Nothing in
either addresses the mechanism that let two land unseen, which is structural: a
seat works on a branch cut before a gate exists, the gate never runs against its
file, and the merge that lands it does not run pre-commit either. A genuinely
broken new citation, written by a seat on such a branch, still arrives unseen,
still puts a counted population over its ceiling, and still has no repair
available, because the member file is the record and the number may not be
raised.

`dae3b57d`'s own reasoning names that dead end, in its words, "with no repair
available, since the member file is the record and the ceiling may not be
raised", and uses it to keep elisions out of the counted population. It does not
close it for the population that is counted. **That residue is the finding, and
it belongs to whoever decides what the member-file arm should do**, which is not
this seat and is not a rounding question.

**One observation on the landed arm, in the safe direction.** The rejoin is
gated on `line.ends_with(&cited)`, so a wrapped citation with trailing
whitespace after the slug is not rejoined and fires instead. That over-reports
rather than under-reports, which is the right way for it to be wrong.

## 1. What I could not break

I went at `229` first and hardest, because it is the file that settles something
by measurement and settling by measurement is the easiest thing to get wrong.

**Its central count is right and I re-derived it without reading its probe.**
The two readings of `half_up` differ exactly on the negative ties, which are the
`k` in `[-2^(W-1), -1)` with `k mod 2^F == 2^(F-1)`, and there are
`2^(W-1) / 2^F = 2^(W-1-F)` of them. That is `229`'s closed form, it matches
every figure it prints, it is zero at `F = 0` because no tie exists, and it is
zero unsigned because no negative tie exists. The form is not a fit to the
numbers; it falls out of the definition, and the numbers agree.

**Its equivariance claims are right.** `floor(x + t + 1/2) = floor(x + 1/2) + t`
for integer `t`, so the corpus's reading is translation equivariant; ties-away
branches on the sign of its argument and is not. `q3` measures the consequence
independently: under wrap the free set for the multiply-add is
`{floor, ceil, toward_zero, away_from_zero, half_up(+inf)}` unsigned and
`{floor, ceil, half_up(+inf)}` signed, which is the two law rows' `holds` fields
exactly, and `half_up(away)` is outside the signed set. So `229`'s finding 4,
that the canon rows reproduce under one reading and not the other, holds against
an instrument that was not built to check it.

**Its structural argument is right too**, and it is the part I expected to
break. `half_up` is the one name in the six whose directional half is a word the
set never defines, `floor` and `ceil` and `toward_zero` each name their direction
outright, and there is no `up` row to resolve it against. `half_even` has no seam
because parity is sign-invariant. I looked for a second reading of `half_even`
and found the same nothing `229` reports finding.

**One imprecision, and it is in the framing rather than in a finding.** `229`
writes "Every name in the set behaves identically on the non-negative half of any
format". Read literally that is false: `floor` and `ceil` differ at every
non-integer non-negative value. What it means is that each name's two readings
agree there. Nothing downstream depends on the literal reading, so it is worth a
correction and not a retraction.

**And one thing it declined that it should have declined.** It refuses to pick a
reading for `half_up`, on the ground that naming is not one expert's call. That
is right, and it is the same division of labour the retirement ruling ran on.

## 2. `228` section 3.3: the statement is two clauses and only one was checked

`228`'s largest claim about `law::rounding_retraction_is_the_identity` is that
the row's `statement` "is not what its instrument measured". The conclusion that
the row is defective is correct. The diagnosis is not, and the difference decides
the repair.

The statement is:

> Rounding a value already on the representable grid returns it unchanged, so
> the reduction retracts.

That is two clauses joined by "so". `228` section 3.3 says of the whole sentence
that it "is **true of every mode at every fraction width**", and its control C2
checks it over six modes at three widths and every `F`, finding zero
counterexamples.

**C2 checks the first clause.** `228`'s own finding F5 says so in its predicate,
which carries `operation: quantise` and `arity: 1`. The sentence's second clause
is about `arity: 3` and `chain_length: 2` and is the property the instrument
measures, because the instrument defines the word in its own header:

```
94_probes/c_retraction.rs:12
  retracts(q, op1, op2)  :=  forall a b c.
      q(q(a op1 b) op2 c)  ==  q(a op1 b op2 c)
```

`q1` runs both clauses side by side over seven deterministic rules, both
signednesses, `W in {4, 6}` and every `F`:

```
PART A  clause 1, on-grid idempotence: 0 counterexamples everywhere
        C1 must-fail: a planted map that adds one grid step is caught,
                      1792 counterexamples, so the zeros are earned
PART B  clause 2, the instrument's retraction:
        holds at every F = 0 cell: true
        fails at every F >= 1 cell, every listed mode: true
        C3 must-fail: the planted map differs on 3840 triples at F = 0
```

`q1` reproduces `94_probes/c_retraction.out.txt` digit for digit at `W = 4`:
truncate 800, 1128, 910, 543 and nearest 864, 1248, 880, 550 at `F = 1..4`, so
it is the same comparison and not a lookalike.

**So the sentence is not a theorem. Its antecedent is.** The row welds a true and
irrelevant theorem to the front of its measured claim with a "so" that does not
follow: idempotence on grid points says nothing about whether staging a
quantisation agrees with deferring it. The row's `note` uses "retraction" in the
instrument's sense throughout and is consistent with its own fields; the
statement's first clause is the only intruder.

**The repair is therefore smaller than `228`'s and different in kind.** `228`
reads the row as being about the wrong property and would restate it as being
about relocation. The minimal correct repair is to delete the first clause and
the "so", which leaves a statement that already matches the fields.

**And the real defect is one level up.** The word `retraction` denotes two
things here: the standard one, a map restricted to the image of an embedding
being the identity, which is clause 1 and is what the row's own id
`rounding_retraction_is_the_identity` reads as; and the instrument's one, a
quantisation commuting with a chain, which is clause 2. `228` did not name that,
and it is the same shape as the defect `229` names in `half_up` one file
earlier: one word, two operations, and the corpus using both.

## 3. `228` finding F4: `fails: rounding any` is false, three ways

`228` proposes, for the same row:

```
holds: rounding: rounding any: construction, at F = 0 the representable grid is
       the whole value set so every mode is the identity and the axis cannot enter
fails: rounding: rounding any: swept, five ratified deterministic modes and
       away-from-zero at all 108 cells, stochastic by construction since the eager
       step is then not a function of the triple
```

**The `holds` side is right** and I could not break it. `q1` part B's `F = 0`
column is zero for every rule and `q2` control C2 adds the stochastic case: at
`F = 0` no residue exists for a draw to act on, so both couplings report zero at
every cell tested. The construction covers the one ratified name a sweep cannot
reach, which is exactly what a `construction` warrant is for.

**The `fails` side is false, and each of the three reasons is independent.**

### 3.1 `rounding any` includes `rounding = exact`, and the law holds there

`dimension::rounding`'s own grammar declares the value:

> `rounding = exact` names the case where nothing is discarded, which is a value
> of the axis rather than its absence.

Nothing discarded means the staged form and the deferred form are the same
expression, so the law holds at that value at every fraction width and both
signednesses. `q1` part D reports zero at all 24 cells it runs, and control C2
replaces the arm with one that discards exactly one bit and gets 1,767,192
differing triples out of the same harness, so part D's zeros come from the arm
rather than from a dead loop.

A `fails` region spanning `rounding any` claims a failure at every value of the
axis. This is one where it does not fail. **No file among `228`, `229`, `232` or
`233` considers `exact` on a `fails` side**; `232` and `233` both reach it, and
both reach it only on the `holds` side at `F = 0`.

### 3.2 The clause naming `stochastic` does not discharge a `construction`

`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` defines the
token as being for "an axis that cannot enter the argument at all, with the
clause saying what makes it unable to", and obliges the row's `evidence` to name
"an instrument that varied that axis and found no movement".

`228`'s clause is "stochastic by construction since the eager step is then not a
function of the triple". The premise is true and the conclusion does not follow.
A randomised rule turns the comparison into a question about a joint
distribution over the draws at the three quantisation points, and the answer
depends on how those draws are coupled. `q2` runs two couplings, both of which
an implementation might actually have:

```
  W   F      sign    A independent      rate    B shared bits      rate   same?
  4   1  unsigned            17920   27.3438%             4224   25.7812%   NO
  4   2  unsigned           432432   41.2399%            23392   35.6934%   NO
  4   3  unsigned          6124532   36.5051%            71024   27.0935%   NO
  6   2  unsigned         36520704   54.4201%          2225152   53.0518%   NO
  6   3  unsigned        659090112   61.3826%          9731264   58.0029%   NO
```

Coupling A draws independently at each point; coupling B builds the deferred
`2F`-bit draw by concatenating the two `F`-bit eager draws, which is what one
shared entropy source produces. Three controls make the gap mean something. C1
collapses the draw space to a point, which makes the rule deterministic, and the
two couplings then agree at all twelve cells, so the disagreement is about the
draw and not the harness. C2 confirms every `F = 0` cell is zero. C3 confirms the
collapsed rule reproduces `q1`'s floor and ceil counts exactly, 800, 1128, 910
and 800, 1128, 925, so this is the same comparison.

**So an instrument that varied the axis found movement**, which is the opposite
of what the `construction` token obliges. `232` reached this independently and
declined to write `stochastic` into `fails` for the same reason, at two other
figures, 28,704 against 27,312 under its own two couplings. That is two
instruments and two authors against `228`'s clause.

### 3.3 The `swept` clause on the neighbouring finding names a six that did not run

`228`'s finding F5 carries `rounding: rounding any: exhaustive, all six named
modes`. `q4` parses the ratified six out of the ruling's `says` field and the
swept six out of `228_probes/p3_retraction_over_the_whole_vocabulary.rs:40`, and
diffs them:

```
ratified  (6) ceil, floor, half_even, half_up, stochastic, toward_zero
swept     (6) away_from_zero, ceil, floor, half_even, half_up, toward_zero

  ratified but never swept: ["stochastic"]
  swept but not ratified:   ["away_from_zero"]
```

Neither list is typed into the probe; both are parsed, and C1 refuses to compare
them unless each parse returns exactly six, because two empty sets compare equal
and that is the shape that would have made `q4` agree with `228` for the worst
possible reason. C2 plants an identical list and a one-name-off list and confirms
the differ reports nothing and exactly one name respectively.

**The one ratified name the sweep never reached is the one that cannot be swept,
and the one it did reach is not a value of the axis.** Under the warrant ruling
`exhaustive` obliges the clause to name the domain the span is the whole of, and
this clause names a domain the run was not the whole of. F4 gets the same run
right, in a clause that says what ran; F5 compresses it into a phrase that names
something else.

**And there is an internal inconsistency underneath it.** `228` section 2.1
argues at length that `away_from_zero` should leave three predicates because on a
non-negative domain it is `ceil`. The retraction row is unsigned. `q1` confirms
the collapse there: `away_from_zero` and `ceil` return 800, 1128, 925, 495 at
`W = 4` and `toward_zero` and `floor` return 800, 1128, 910, 543, identical pairs
at every unsigned cell. So by `228`'s own argument the `away_from_zero` column in
its clause is a second run of `ceil` and carries nothing, and `228` writes it in
anyway.

### 3.4 What the entry should read

Neither file has this right and each has half of it.

```
holds: rounding: rounding any: construction, at F = 0 nothing is discarded so
       every value of the axis is the identity and it cannot enter the argument
fails: rounding: rounding in {floor, ceil, toward_zero, half_up, half_even}:
       swept, the five ratified deterministic names at all 108 (W, F) cells
```

`exact` is out because the law holds there. `stochastic` is out because it has no
verdict until a coupling is named, and naming one is a design question nobody has
put. `away_from_zero` is out because it is not a value of the axis and, on this
row's unsigned domain, is `ceil` re-run.

**`232` and `233` both wrote an explicit list on this side rather than `any`, so
both avoid the `exact` trap without arguing about it.** `233`'s list carries
`ceil` and `away_from_zero`, which on an unsigned domain is `ceil` twice.

## 4. `228` finding F2 drops a correct half of a region, on a third ambiguous word

This is the finding I did not expect and it is the largest thing in this file.

`228`'s finding F2 covers the fusion prediction and carries
`signedness: in {unsigned, signed}`. It narrows the overflow axis, and gives its
reason:

> **`overflow_policy: wrap` and not `in {wrap, saturating}`.** The prediction was
> run under wrap only, and under saturating every mode fails at every fraction
> width including zero, so the prediction is known not to extend and I do not
> claim it does.

`law::fusing_a_multiply_add_preserves_the_answer_under_unsigned` carries
`overflow_policy: in {wrap, saturating}` on its `holds` field for five modes.
Those two statements are about the same property and cannot both be right.

`q3` runs the multiply-add comparison at `W = 6`, both signednesses, both
policies, seven deterministic rules, `F in 0..=5`, and it is anchored to the
corpus by four positive controls before it says anything new:

```
C3  the unsigned wrapping half_even rates, against the row's own `note`:
      measured ["0.00","12.50","12.50","9.38","6.25","3.91"]   six of six
C4  the unsigned saturating half_even rates, against `228` section 2.3 and the
    row's stated range 0.93 to 2.18:
      measured ["0.93","1.61","2.02","2.18","2.08"]            five of five
C4  `228` section 0's own v1 control figure:
      signed saturating, F = 0, reading 1: 110476 of 262144
C1  the two readings must be identical at every wrapping cell               ok
C2  every mode free at F = 0 under wrap                                     ok
```

**The 110,476 is `228`'s own number and it falls out of one specific cell of my
table: signed, saturating, and one particular reading of what fusing removes.**

The two readings, both of which the row's `statement` licenses, since it says
only that "the intermediate product is not resolved before the addition":

- **Reading 1, resolve means round and place in the container.** The stepwise
  form reduces twice, once after the product and once after the sum; the fused
  form reduces once.
- **Reading 2, resolve means round only.** The reduction is a property of the
  final result. One reduction on each side.

Under wrap these are one measurement, because reduction modulo `2^W` is a ring
homomorphism, and `q3` control C1 confirms they are identical at every wrapping
cell. Under saturation they are two.

```
                             free at every F in 0..=5
  unsigned  wrap      reading 2   floor ceil toward_zero away_from_zero half_up(+inf) half_up(away)
  unsigned  wrap      reading 1   the same six
  unsigned  saturate  reading 2   the same six
  unsigned  saturate  reading 1   the same six
  signed    wrap      reading 2   floor ceil half_up(+inf)
  signed    wrap      reading 1   the same three
  signed    saturate  reading 2   floor ceil half_up(+inf)
  signed    saturate  reading 1   []
```

**Three consequences, in the order they matter.**

**The canon row is right and `228`'s objection to it is wrong.** On the unsigned
domain the two readings coincide under saturation as well, because clamping at
the top is idempotent and a non-negative addend cannot come back down:
`clamp(clamp(x) + c)` is `clamp(x + c)` for every `c >= 0`. Every mode is free
under unsigned saturating at every `F` including zero, under both readings, at
all 42 cells measured. So `overflow_policy: in {wrap, saturating}` on that row's
`holds` is supported twice over and needs no defending.

**`228`'s sentence is true only on the signed half and only under reading 1.**
That is precisely the cell its own v1 control fired in, at 42.14 percent of
262,144, which is 110,476. The control was right, the diagnosis of it was right,
and the sentence generalised it from one cell of a four-cell grid to the whole
axis, then used the generalisation to narrow a predicate that spans both
signednesses. **A correct half of a region was discarded because a control fired
in the other half under a reading the file never names.**

**The signed row's `overflow_policy: wrap` is right for a reason nobody has
written down.** Under reading 2 the signed free set extends to saturating
unchanged; under reading 1 it is empty. So the saturating extension of that row
has no truth value until the reading is named, and `dimension::overflow_policy`
cannot name it, because what varies is not which policy applies but **how many
times it is applied**. The row was right to stay silent and its silence is
currently unexplained.

## 5. The class: three words, three pairs of operations, one named

Put the three together.

| word | reading A | reading B | where they differ | named by |
|---|---|---|---|---|
| `half_up` | `floor(x + 1/2)` | ties away from zero | negative ties, `2^(W-1-F)` values | `229` |
| `retraction` | idempotent on grid points | quantisation commutes with a chain | everywhere at `F >= 1` | nobody |
| `resolve` the intermediate | round and reduce | round only | signed saturating, up to 42.14 percent | nobody |

Each is one word denoting two operations. Each pair agrees on a large region and
differs on a small one, which is what keeps them alive: `half_up` agrees unsigned
and at `F = 0`, `resolve` agrees under wrap everywhere and under saturation
unsigned, and `retraction`'s two readings agree at `F = 0`. Each was found by
somebody chasing something else.

**So `228`'s answer to `question::is_the_rounding_vocabulary_complete_at_six` is
right about what it asked and asks the wrong thing.** The vocabulary of rounding
mode names is complete with respect to `away_from_zero`; `228` establishes that
and I did not break it. But the corpus's rounding topic is not short a mode
name. It is short a definition per word, and two of the three ambiguous words are
not mode names at all, so no widening of the mode vocabulary reaches them and
`mock/tools/rounding-vocabulary` cannot see either.

`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names` is the
precedent and its `because` is the test: a word that "named two different
operations" where "a reader coming from the hardware and a reader coming from C
would have understood the same word as two operations that genuinely differ".
All three pass that test. One has been retired.

## 6. `232` and `233`: what they conceded, and whether it was earned

Both files derived on the same nine entries and both largely conceded to `228`
and `229`. Reading them for what they gave up:

**`233` says outright that it took `228`'s section 3.3 without checking the
sentence.** That concession was not earned, and section 2 above is why. The
sentence is not a theorem; its antecedent is. Neither `232` nor `233` opened it
either.

**On entry 8, the `holds` side at `F = 0`, `233` rejects `228` and is wrong.**
It proposes `rounding = exact` and calls it "strictly better" than the options,
on the ground that at `F = 0` nothing is discarded. The observation is right and
the entry is a category error: `exact` is the axis value a design *declares* when
it wants no rounding, and a design that declares `rounding = half_even` and sets
`F = 0` has `half_even` on that axis, not `exact`. Writing `rounding = exact`
excludes the six modes from a cell where the law demonstrably holds for all of
them, which `q1` part B's `F = 0` column and `q2` control C2 both show. **A const
predicate gating on `rounding == exact` would not fire for that design.** `228`'s
`rounding any: construction` fires for every one of them and is correct.

**On entry 9, the `fails` side, `232` and `233` are both closer than `228` and
neither is right.** Both write an explicit mode list rather than `any`, which
avoids the `exact` trap without arguing about it, and both include
`away_from_zero`, which on this row's unsigned domain is `ceil` measured twice.
`232` additionally declines to write `stochastic` into `fails` and gives the
coupling reason, which is correct and is the finding `228` got wrong. So the
right entry is assembled from three files and appears whole in none.

**`232`'s reading of `228` on entry 6 is worth keeping.** It notices that
`rounding = nearest, against a phase-zero mutant` puts control metadata in a
predicate value, and that under the warrant mechanism the trailing clause is
close to what a warrant clause is for. That is the general shape of every defect
in section 3: an author reaching for somewhere to record how a region was earned
before there was anywhere.

## 7. `233`'s closed-set argument refutes the remedy it was offered to support

The brief asks whether `233`'s claim that `rounding any` is writable only because
the rounding set is closed appears in the other three files. It does not, and I
checked: `228` uses the word "closed" once and not about this, `229` three times
and not about this, `232` four times and not about this.

The argument is sound. `dimension::operation` forbids `operation any` in bold,
"because `any` quantifies over a set nobody has closed", and
`dimension::signedness`'s neighbour `dimension::strategy` forbids `S any` on the
same ground. The rounding set is closed, so `rounding any` is writable.

**Two corrections, and the second is the point.**

**The set is closed at seven, not six.** The ratified ruling closes the mode
vocabulary at six names. `dimension::rounding`'s grammar declares an eighth thing
that is not one of them: `rounding = exact`, "a value of the axis rather than its
absence". So the closure that makes `any` writable comes from two rows and the
set it quantifies over has seven members. `233` attributes it to one row and
counts six.

**And that is exactly why `228`'s `fails: rounding any` is false.** `233` offers
the closed-set argument as support for `228`'s remedy. Run it to its end and it
does the opposite: `any` is meaningful precisely because it ranges over a known
set, the known set contains `exact`, and the law holds at `exact`. **The
strongest available argument for writing `any` on that row is the argument that
`any` is wrong on one of its two fields.** Neither file noticed, and the reason
is visible: `233` reasoned about whether `any` is *writable* and never asked
whether it is *true*.

## 8. The findings, with their predicates

Per `every-finding-carries-its-predicate`: a dimension listed with a value or
`any` is claimed there and nowhere else, and a dimension absent is not claimed at
all. Nothing here widens any existing predicate; a widening is a new claim and
these are mine.

**The common region.** Every finding below carries it and states only what
differs.

```
radix: 2
ambient_domain: the dyadic rationals with denominator 2^F
container: i128, every intermediate strictly inside it at the widths swept
threads: 1
target_features: host aarch64-apple-darwin
toolchain: rustc 1.98.0-nightly 57d06900f 2026-05-27, edition 2024
build_profile: opt level = 2, debug-assertions off
```

**G1. The `statement` of `law::rounding_retraction_is_the_identity` is two
clauses with opposite truth values, and only the first is a theorem.**

```
rounding: rounding in {floor, ceil, toward_zero, away_from_zero,
          half_up as floor(x+1/2), half_up as ties-away, half_even}: exhaustive,
          the seven deterministic rules this file implements
total_width: in {4, 6}: exhaustive
fraction_width: in 0..=W: exhaustive
signedness: in {unsigned, signed}
operation: for clause 1, quantise; for clause 2, mul
arity: for clause 1, 1; for clause 2, 3
chain_length: for clause 2, 2
```

Clause 1 has zero counterexamples at every cell. Clause 2 holds at every `F = 0`
cell and fails at every `F >= 1` cell for every listed rule. Evidence
`234_probes/q1_the_statement_is_two_clauses.rs`, parts A and B, with C1 and C3
showing each checker can fail.

**G2. The law holds at `rounding = exact`, so `rounding any` is false on the
`fails` field.**

```
rounding: rounding = exact: construction, nothing is discarded so the staged and
          deferred forms are the same expression
total_width: in {4, 6}: exhaustive
fraction_width: in 0..=W: exhaustive
signedness: in {unsigned, signed}
operation: mul
arity: 3
chain_length: 2
```

This is a construction and not a sweep, and it is worth saying which: the zeros
are structural, and what `q1` part D's control C2 establishes is only that the
harness reports 1,767,192 differing triples for an arm that does discard, so the
zeros come from the arm. That `exact` is a value of the axis at all comes from
`dimension::rounding`'s grammar and not from any measurement.

**G3. A stochastic rule has no retraction verdict until a draw coupling is
named, and two defensible couplings give different rates.**

```
rounding: rounding = stochastic proportional, in the (p + u) >> s realisation:
          swept, the whole draw space enumerated rather than sampled
coupling: in {independent, shared bits}: exhaustive, the two this file implements
total_width: in {4, 6}
fraction_width: in 1..=3: exhaustive
signedness: in {unsigned, signed}
operation: mul
arity: 3
chain_length: 2
```

Eleven of twelve cells disagree between the couplings, the twelfth being `W = 4,
F = 1` signed where both give exactly 25 percent. Evidence
`234_probes/q2_stochastic_has_no_verdict_without_a_coupling.rs`, with C1 showing
the couplings are indistinguishable once the randomness is removed and C3
showing the collapsed rule reproduces `q1`'s floor and ceil counts.

`coupling` is not a declared axis. I am not proposing one; I am saying the
finding cannot be written without it, which is the same shape `229` reports for
`strategy` and `overflow_policy` and is a third instance of it.

**G4. `228_probes/p3`'s mode set is not the ratified six.**

```
rounding: rounding in {the six of ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names}
          vs {the six of 228_probes/p3_retraction_over_the_whole_vocabulary.rs:40}:
          exhaustive, both sets read from the files rather than typed in
```

Symmetric difference `{stochastic}` against `{away_from_zero}`. Evidence
`234_probes/q4_which_six_the_warrant_names.rs`, with C1 refusing the comparison
unless each parse returns six and C2 exercising the differ on a planted
identical and a planted one-off list.

**G5. Under saturation the two readings of "resolve the intermediate" coincide
on the unsigned domain and diverge on the signed one.**

```
rounding: rounding in {floor, ceil, toward_zero, away_from_zero,
          half_up as floor(x+1/2), half_up as ties-away, half_even}: exhaustive
total_width: 6
fraction_width: in 0..=5: exhaustive
signedness: in {unsigned, signed}
overflow_policy: in {wrap, saturate}: exhaustive
reduction_count: in {one, two}: exhaustive, the two readings of `resolve`
operation: multiply-add
arity: 3
chain_length: 2
container: declared width
```

Under wrap the readings are identical at all 84 cells. Under saturation unsigned
they are identical at all 42 cells and the free set is unchanged from wrap. Under
saturation signed the free set is `{floor, ceil, half_up(+inf)}` under one
reading and empty under the other, with the gap at `F = 0` being 42.14 percent of
262,144 triples for every mode. Evidence
`234_probes/q3_resolving_the_intermediate_names_two_operations.rs`, with C1, C2,
C3 and C4 above.

`reduction_count` is not a declared axis either, and is the third one this file
needed and could not write.

**G6. `law::fusing_a_multiply_add_preserves_the_answer_under_unsigned`'s
`overflow_policy: in {wrap, saturating}` on `holds` is supported, and `228`
finding F2's justification for narrowing it is false in half its own region.**

Predicate as G5, restricted to `signedness: unsigned`. Every one of the six
deterministic rules other than `half_even` is free at every `F` in `0..=5` under
saturation, under both readings. `half_even` fails at 0.93, 1.61, 2.02, 2.18 and
2.08 percent, which is the row's own stated range and `228`'s own five figures.

## 9. What I could not reach

**I did not settle which reading of "resolve" the instrument behind the two
fusion rows actually ran.** `q3` reproduces every number both the row and `228`
publish, under wrap where the readings coincide and under unsigned saturation
where they also coincide, so none of those figures discriminates. The only
discriminating cell is signed saturating, and neither the row nor `228` publishes
a signed saturating figure. Somebody who can open `142_probes` can close it by
reading which form it wrote; I could not close it from the registry and the two
files alone. **Until it is closed, `q3`'s two readings are both live and the
signed row's silence about saturating is correct rather than merely cautious.**

**I did not find a second reading of `half_even`, `floor`, `ceil` or
`toward_zero`**, and I spent real effort on `half_even` because a family that
broke once might break twice. `229` reports the same negative and gives the
reason, that parity is sign-invariant, and I have nothing to add to it. Recorded
so the next seat does not spend it again.

**I did not break `229`'s finding 5**, the emptiness result over all 65,536
nearest modes. I checked the argument rather than the enumeration: integer
translation acts transitively on the ties, so an equivariant tie rule is
constant, and a constant tie rule cannot balance. That is a proof and the
enumeration is a check on it, which is the right way round.

**I did not price anything.** Nothing in this file is a measurement of time or
of instructions, and none of it went near `mock/benches/`. Every number here is a
count over an enumerated domain.

**And for most of this seat I could not commit.** Section 0b. The gate has since
been repaired on the trunk at `dae3b57d` and everything above is committed, so
the figures are citable. What that cost was the ordering: the probes were built
and re-run against an index that could not be written, so each was verified by
re-running it rather than by a commit at the moment it passed, which is weaker
than the discipline wants and is worth saying.

## 10. What the next seat gets

**The one thing I would ask the panel to act on**, and it is not any of the
findings: decide what the member-file arm of `no_prose_citation_into_nothing`
should do when a genuinely broken citation gets past it. `dae3b57d` removed the
reason the last two got past, and it did not change the arm: a counted
population over its ceiling still has no repair available, because the member
file is the record and the number may not be raised. The miss is structural
rather than careless, since seats work on branches cut before a gate exists and
the merge that lands them runs no pre-commit hook, so it will recur on the next
ratchet rather than on this one.

**Three routes I opened and did not follow.**

`law::rounding_retraction_is_the_identity` and
`law::double_rounding_is_innocuous_at_an_intermediate_width_between_f_and_2f`
are the same family and neither cites the other. The second says no intermediate
width strictly between `F` and `2F` makes staged narrowing innocuous, and its
`note` names "staged-versus-direct narrowing" as the open mechanism. The first
measures the endpoint case, intermediate width exactly `F`, and calls it
retraction. Together they close `[F, 2F)`. **Somebody should check whether the
first row is the endpoint instance of the second's family**, in which case the
repair to its statement is not a rewording but a merge, and the topic gains a
closed result it currently has in two pieces under two names.

**The three ambiguous words want one instrument, not three questions.** The
retirement ruling's `because` is a usable test and it is mechanical: for a word
in the corpus, are there two shipped or standard operations it names, and do
they agree on a large region and differ on a small one. `mock/tools/rounding-vocabulary`
checks mode names against the six. Nothing checks a word that is not a mode name,
and two of the three found so far are not.

**And the axes.** This file needed `coupling` and `reduction_count` and `229`
needed a value for `strategy` and for `overflow_policy` meaning "no reduction
applied". Four in three files, all of the same shape: a predicate that cannot be
written because the notation has no name for the thing that varies. That is worth
one pass over the whole corpus rather than four separate rows, and I did not open
one because opening rows is not what this seat was for.
