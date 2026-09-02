# 220. The marker spelling, and what a checker can honestly enforce

Dominic Orchard, seat 220. Blind first pass, committed before reading any other
seat's file.

The question is the residue of `ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked`,
whose own `note` hands the panel two things: the marker's spelling, and where a
checker enforces the distinction. I take them in that order, because the second
is decided by the first and not the other way round.

## The gates

**Canon gate: aligned.** Checked against `ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked`
(`rung = "ratified"`, `ratified_by = "op"`), `mock/registry/dimension.toml` in
full, and `.claude/rules/every-finding-carries-its-predicate.md`. The ruling
asks for exactly this work, names it as the panel's, and states four constraints
I treat as binding: predicates are append-only and never widened in place, the
three omission states have no fourth, there is no vocabulary for doubt, and the
test for a proof is to name the construction that makes the axis unable to
enter. Nothing in the assignment conflicts with any of them. I did not find the
canon ambiguous on the question, so I have not handed the call back.

**Test gate: the suite is real and I would not refuse work on it.** `cargo test
-p arvo-checks` from `mock/`: 144 tests across nineteen binaries, all green,
zero ignored. I read the bodies of the surface I touch rather than the names:

- `checks/tests/every_predicate_names_a_declared_axis.rs`, all seven. Two run
  over the committed canon and five plant an input. Each planted one carries the
  case that must fail, and two of them exist specifically to stop the arm
  becoming vacuous: `a_keywords_list_is_not_read_as_a_predicate` pins that the
  walk is not over every `string[]`, and
  `a_laws_failing_region_is_read_as_well_as_its_holding_one` asserts a count of
  two so that a walk wired to one field reports one and fails. That is a test
  written by somebody who expected to be wrong.
- `checks/tests/the_axis_vocabulary_is_append_only.rs`, all three, and the third
  is the reason the first two are not a list checked against itself. It asserts
  the invented slug is absent from the canon and present in neither list, so
  both directions can fail.
- The region arms in `checks/tests/what_one_field_obliges_another_to_carry.rs`,
  and `no_new_measurement_lands_without_an_instrument` alongside its control
  `the_measured_without_evidence_finder_still_finds_the_stuck_rows`, which
  asserts a finder **non**-empty. That inversion is the right instinct and I
  reuse its shape below.

**One thing the gate found that is not in my question**, reported here under the
standing instruction and again at the end: **eight of the arms I exercised return
an empty list against an empty registry, and five of those return an empty list
against the real canon too, so for those five an empty registry is
indistinguishable from a clean one.** `arvo_checks::load` returns `Ok(empty)` for
a path that is not a directory (`checks/src/lib.rs:165-167`), so
`canon()`'s `.expect("mock/registry is readable")` at `checks/src/lib.rs:139`
cannot fire and the message is a claim nothing checks. I found it by pointing my
own spike at the wrong directory and getting a clean run. Measured in
`220_probes/p5_warrant_spike.out` section 5. Three arms would notice, all three
because somebody wrote a non-empty assertion beside them.

## 1. The thing the notation cannot say is not a region

Start where the ruling starts, with the three cases it names, and ask what
actually differs between them.

Take four rows, all legal today.

| | what the row writes on the width axis | what it means |
|---|---|---|
| A | `total_width: W any` | addition at a common scale performs no rescale, so no width appears in the argument |
| B | `total_width: W any` | nobody varied the width and `any` was the widest thing available |
| C | `total_width: W in 1..=64` | every width a `u64` admits was enumerated and none was skipped |
| D | `total_width: W in 1..=64` | a loop ran to 64 and the claim is about what it saw |

A and B write the same string. C and D write the same string. Under I13 all four
regions are exact and all four are correct: A and B both hold at every width, C
and D both hold over one to sixty-four. **Nothing about the region is wrong in
any of the four rows.** So the notation is not short of a region state, and any
proposal that adds one is answering a different question.

What differs is *how the region was earned*: what makes it true, and therefore
what a reader may conclude about it. Call that the **warrant**. Region says
where; warrant says why you may believe where.

The word is not mine. `dimension.toml` already uses it, in the `access_pattern`
row, and uses it for exactly this: "A correctness claim untouched by it writes
`access pattern: any`, **with the structural argument as the warrant**, exactly
as a compile-time result writes `threads any`." That sentence names the thing,
requires it, and has nowhere to put it. This deliverable is mostly the argument
that giving it a place is all that is being asked for.

Three warrants, matching the ruling's three cases one for one:

- **swept.** The axis was varied over the span and the claim checked at each. The
  span is a sample of the axis unless it happens to be the whole of it.
- **proof.** An argument establishes the claim over the span without the axis
  entering the argument.
- **exhaustive.** The axis was varied over the span, the claim checked at each,
  **and the span is the whole of the axis's domain for a named container.** A
  sweep's evidence with a universal's coverage, bounded. That is precisely why
  the ruling can say it is "neither a sample nor a universal": it has the first's
  method and the second's completeness, over a finite domain.

**Getting the direction right is the whole of the design.** A marker that
modifies the region is a fourth region state, and I13 has three and no fourth.
A marker that states a warrant adds no region state at all, which is what makes
the ruling's own claim in its `note` true rather than merely asserted: appending
this "invalidates nothing beneath it" because it is orthogonal to the thing every
existing predicate is a statement in.

## 2. The unmarked state has to stay unmarked, and it is not a hedge

Append-only forces the default. Every committed entry must keep meaning exactly
what it means, so the unmarked state must remain **warrant not stated**.

It must specifically **not** be defined as `swept`. Two entries in the committed
corpus settle that on their own:

```
"threads: threads any, the equalities being decided at compile time"
"threads: threads any, the refusal being a type-check outcome that precedes execution"
```

Nobody swept thread counts in either. Defining the unmarked default as `swept`
would make both rows assert a measurement that was never taken, which is
restating existing files, which the ruling declined by name.

So the honest reading of an unmarked entry is that the region stands exactly as
written and the warrant is not asserted. **The objection to that is obvious and
has to be met**: I13 says there is no vocabulary for doubt, because a hedge and
a proof look alike once written down. Is an unstated warrant a hedge?

No, and the reason is structural rather than a matter of taste.

I13's three-states rule quantifies over **the world**. An axis listed says where
the claim holds; absence says it holds nowhere that axis exists. That reading is
available because a claim is a total function from world-points to truth values,
so silence about a point can be given a definite value.

A warrant is not a function of world-points. It is a property of the argument
that produced the row. Absence of it makes no statement about the world, so it
has no negative to fire and needs none. `dimension.toml`'s own header draws
exactly this line one notch over, when it separates an axis from an instrument
parameter: "An axis indexes a situation the world can be in... A parameter
indexes a run." A warrant indexes **the argument**, which is a third kind of
thing, is not on the axis list, and could not be written for precisely that
reason.

And declining to make a stronger claim is not hedging a claim you did make. The
region is total and exact either way. What the marker adds is a separate,
stronger assertion that a row may simply not make.

The check that this stays honest is that the resulting notation admits no middle
ground: the warrant is one of two declared tokens, each carrying an obligation,
or it is absent. There is no `probably proved` and no `mostly exhaustive`, and
below I make sure a checker refuses one if somebody writes it. Two positive
states and an absence, which is I13's own shape one level up.

## 3. A bare token is the failure, and the schema already ran that experiment

The ruling's test is "name the construction that makes the width unable to
enter, or it was a sweep." Read as a design constraint that is severe, and it
kills the first thing anybody will propose.

A marker that is a bare word is a relabelling. Anybody can type it, nothing
checks it, and a reader cannot tell an earned one from a typed one.

**This is not a hypothesis. The schema already carries such a marker and it has
already failed.** `sentence_kind` takes six values across the registry, of which
`theorem` (8 rows) and `measured` (32) are a proof/measurement distinction at
row level. Here is every `theorem` row against the width span its own predicate
writes, from `220_probes/p2_warrant_vs_region.out`:

| row | width span |
|---|---|
| `the_laws_of_a_format_are_derived_from_two_hypotheses_rather_than_enumerated_per_policy` | `W = 4` |
| `no_multiplicative_structure_survives_a_nonzero_fraction_width` | `W in 3..=7` |
| `a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains` | `W in {4, 5, 6}` |
| `fusing_a_multiply_add_is_free_exactly_at_translation_equivariance` | `6` |
| `inside_a_fragment_with_a_complete_test_set_the_verdict_is_computed_at_the_shipped_width` | `W in 1..=65` |
| `the_model_band_transfer_is_defeated_in_both_fragments` | `W any` |
| `the_join_over_demands_is_union_and_it_is_free` | absent |
| `a_multiplicative_chain_is_writable_without_an_ever_growing_intermediate_by_windowing` | absent |

**One of eight writes `W any`.** Five write a sweep-shaped span and two write
nothing, which under I13 read literally says those two theorems hold at no width
in the library. `no_multiplicative_structure_survives_a_nonzero_fraction_width`
is a universal sentence in its own `id`, is marked `theorem`, and holds at widths
three through seven and nowhere else.

I am not saying those eight rows are wrong. I am saying the word `theorem` on a
row is doing no work that anybody can check, and it is doing it in the exact
shape the ruling warns about. **That is the experiment, it has run, and the
result is in the committed canon.** Anything the panel ships must not be a second
copy of it.

Two consequences follow immediately, and they are the whole of the design:

1. **The marker must be per axis.** A row that proves over width and sweeps over
   signedness is the normal case, not a corner. `an_additive_verdict_is_independent_of_the_fraction_width`
   is width-free by construction and its overflow policy is a two-value sweep. A
   row-level word cannot say that, so it says the strongest thing about the row
   and gets read as saying it about every axis, which is what happened above.
2. **The token must be unwriteable without its construction.** Not a token plus
   an optional note. One act, and a checker that refuses the half of it.

## 4. The candidate spellings

Seven, with what each buys and where it dies. I have kept the dead ones because
the next reader attacking this from another angle starts from the list rather
than from nothing.

**A. Prose after a comma, which is what the corpus does today.**

```
"threads: threads any, the equalities being decided at compile time"
```

*Buys:* nothing to build; already in use; reads well; costs no grammar change.

*Dies on:* the corpus writes warrants and sweep-notes in the same slot with the
same syntax, so nothing separates them. From `220_probes/p3_delimiter_collision.out`,
three committed spans are warrants in disguise (the two above plus `"threads:
threads = 1, the splits being computed rather than executed on lanes"`), sitting
beside `"total_width: W in 1..=19, the widest evaluable width per arity"` and
`"rounding: rounding = nearest, against a phase-zero mutant"`, which are notes
about a sweep. **Same shape, opposite kind, and no reader or checker can tell
them apart.** That is the empirical refutation of prose as the marker, and it is
also the evidence that the content the ruling asks for already exists and only
wants a slot. The ruling asked for a marker the notation lacked; prose is what
the notation already had.

**B. A row-level field, or reusing `sentence_kind`.**

*Buys:* trivially parseable; the field exists.

*Dies on:* granularity, per section 3. It also cannot survive the two-coordinate
problem `question::what_a_proof_marker_is_against_a_measurement` records, that
"one file writes a width universal as two separately quantified widths" — a
universal spelled `I any` plus `F any` rather than `W any`. A row-level word
cannot attach to one of those and not the other. **This candidate is not
hypothetical: it is what shipped, and the eight rows are its report card.**

**C. A parallel keyed list, `warrant = ["total_width: proof, ..."]`.**

*Buys:* parseable and per axis.

*Dies on:* two keyed lists that must agree with nothing pairing them. A warrant
naming an axis the predicate does not list warrants a claim nobody made; a
predicate entry with no warrant row is silence that looks like absence. It is one
keyed list written twice, and the divergence is silent.

**D. A dimension slug per warranted axis, `total_width_proof:`.**

*Dies hardest.* It doubles the axis set with non-axes, so absence of
`total_width` stops being readable: a reader cannot tell whether the axis is
absent or merely spelled the other way. Absence is the strongest negative
statement in the notation and this makes every one of them ambiguous. It also
fails `dimension.toml`'s own test for what an axis is, since a warrant indexes an
argument rather than a situation the world can be in.

**E. A leading token before the span.**

```
"total_width: proof(...) any"
```

*Dies on:* the existing splitter takes the first colon as the axis boundary, and
any spelling that puts structure before the span either moves that boundary or
buries the token inside a span grammar the checker deliberately does not read.

**F. A trailing warrant clause after a second colon.** `<axis>: <span>: <warrant>`.

```
"total_width: W any: proof, addition at a common scale performs no rescale so no width enters the argument"
"total_width: W in 1..=64: exhaustive, every width a u64 container admits"
```

*Buys:* per axis; parseable by one extra `split_once`; leaves the span grammar
untouched, which preserves the stated reason `predicate.rs` gives for not
reading the value side ("the grammars differ per axis... nothing in common a
regex would capture without also accepting everything") — a closed token after a
fixed delimiter is exactly what that objection does not cover; and reads left to
right as axis, where, why.

*Dies on:* nothing I could make it die on. The collision risk is the only real
question and it is measured, below.

**G. F with a rarer delimiter** (`|`, `;`, `@`, `~`, `::`, `--`, `=>`).

*Buys:* the same, with a delimiter that could never collide.

*Dies on:* register. Every one of them reads as machine syntax in a field the
corpus writes as English, and they buy protection against a collision that does
not exist. Measured: **all eight candidate delimiters, the colon included, occur
zero times across all 527 committed spans** (`220_probes/p3_delimiter_collision.out`).
The choice is therefore free of collision and is settled on readability, which
the colon wins because the entry already reads `axis: span` and this extends the
same rhythm.

## 5. The spelling

```
<axis>: <span>                                     the warrant is not stated
<axis>: <span>: proof, <the construction that makes the axis unable to enter>
<axis>: <span>: exhaustive, <the domain the span is the whole of>
```

Two tokens, closed, the way the axis set is closed. The third state is the
absence of a token, which is why the ruling asks for two markers and not three.

**`proof`** is the ruling's own noun. The clause is not decoration: `proof` with
nothing after it is the bare relabel, and a checker refuses it, exactly as the
shipped arm already refuses an axis listed with no values
(`predicate-entry-has-no-values`, `checks/src/predicate.rs:82-94`). The same
shape, one level down.

**`exhaustive`** rather than `whole` or `total`, because the distinguishing
content is that the sweep left nothing out. The clause names the domain, and the
row is separately obliged to carry a `container` entry, because `W in 1..=64` is
the whole of a `u64` and a sample of a `u128`, and the span alone does not say
which. **That obligation is on the axis vocabulary that already exists** rather
than on a second place to name a container, which is the whole reason to prefer
it over folding the container into the token.

The append-only property is compiled and run rather than argued:
`220_probes/p5_warrant_spike.out` section 1, **527 committed entries, zero read
differently under the new grammar, zero unreadable.** That is the append-only
rule discharged as a measurement.

Worked, on the row the canon already reasoned about this way. The `promotion` field of `ruling::the_additive_and_absorption_verdicts_are_canon`
promotes `an_additive_verdict_is_independent_of_the_fraction_width` with the
sentence "the mechanism it names, that addition at a common scale performs no
rescale, is why the width cannot enter." That is the ruling's test, applied,
recorded, and living in a `promotion` field on a different row. Under this
spelling it lives where it is checkable:

```
"total_width: W any: proof, addition at a common scale performs no rescale so no width enters the argument"
"fraction_width: F any: proof, the same"
"overflow_policy: overflow policy in {wrap, saturate}"
```

Width and fraction carry the warrant; overflow policy does not, because it was
swept. **No row-level word can express that shape**, which is the argument for
the locus in one line.

## 6. What a checker enforces

Six arms. Each is implemented and run in `220_probes/warrant_spike/`, each has a
planted case that must fail, and a reachability control asserts every arm fires
at least once, because an arm no planted case reaches is a line of code rather
than a check.

| arm | refuses |
|---|---|
| `warrant-is-not-a-known-token` | a second colon whose tail is not `proof` or `exhaustive`. Keeps the set closed the way the axis set is closed, and is also the arm that catches a colon written inside a span. |
| `warrant-has-no-clause` | a token with nothing after the comma. **This is the arm that stops `proof` becoming what `theorem` became.** |
| `warrant-clause-is-a-bare-relabel` | a clause from a closed blocklist that asserts the warrant instead of naming a mechanism. |
| `exhaustive-over-an-unbounded-span` | `exhaustive` on a span containing `any`. The ruling's own words: neither a sample nor a universal, and `any` is the universal. |
| `exhaustive-names-no-container` | `exhaustive` on a row whose predicate names no `container`. |
| `a-proof-asserted-only-at-row-level` | a `theorem` row none of whose entries carries a per-axis `proof`. |

I drafted a seventh, `a-span-carrying-a-colon`, and **the reachability control
deleted it**: `read` splits at the second colon, so a span never contains one,
and the case it was drafted for is refused by the token arm instead. One arm, not
two. I record that because the control catching my own dead arm is the reason to
trust the other six, and because a spike whose every arm passed on the first run
would be telling me about my expectations rather than about the design.

**What the checker cannot enforce, stated rather than glossed.** It enforces the
*presence and shape* of a construction. It cannot grade the *content*: no arm can
tell a real mechanism from a well-phrased restatement, and the blocklist arm is
the weakest thing here by a distance. It carries the identical defect
`checks/src/shape.rs:225-237` documents at length about its own retired word
list, that "a word list cannot tell a report from a counterfactual", and I have
no better instrument. So: **the mechanical half makes the absence of a
construction impossible to hide; the content test stays a human gate, and that
gate already exists and already works.** It is the `promotion` field on the
ratifying ruling, which in `ruling::the_additive_and_absorption_verdicts_are_canon` did this test correctly and in
public, and which in the same field records a third proposal failing it for
the same reason. I would rather say that plainly than claim the arms close the class.

**Where the arms run.** `mock/checks/`, as an extension of `predicate.rs`,
because that file already walks exactly these three fields
(`proposal.predicate`, `law.holds`, `law.fails`) and already splits on the first
colon. The extension is a second `split_once` and the arms above. Nothing else
in the crate moves. I have not written it into `checks/` and will not: the round
is at `TOPIC` (two flat topic files in `mock/design_rounds/`, no changelist), and
the crate is code tier, so a spike is the correct locus for it today.

## 7. The ratchet, because no existing file is restated

`a-proof-asserted-only-at-row-level` fires on eight rows today. The ruling says
the markers apply going forward and no existing file is restated, so this arm
cannot be a gate or it demands exactly the restatement pass op declined.

It is a **ratchet**: the count does not rise. The crate already runs one,
`no_new_measurement_lands_without_an_instrument`
(`checks/tests/what_one_field_obliges_another_to_carry.rs:42-65`), with a ceiling
of six and a doc comment naming why each of the six is stuck. Same shape here,
ceiling of eight, and the eight named in the comment with their width spans as
tabulated in section 3. The ninth turns it red.

**And it carries that ratchet's control**, which is the part that matters: the
finder is separately asserted **non**-empty, because a ceiling passes two ways
and the second is the finder having stopped working. That control is what would
have caught my own empty-registry problem, and it is why I am not proposing a
ratchet without one.

## 8. Corollaries

**An open question in the canon closes.** The `note` on `ruling::the_additive_and_absorption_verdicts_are_canon` records an
asymmetry it explicitly leaves to "whoever next touches the axis": the additive
row lists no `rounding` where its multiplicative sibling lists `rounding any`,
addition at a common scale does not round, "but the notation has no inapplicable
state and absence means the finding holds nowhere that dimension exists." That is
the same gap one axis over. There is no inapplicable state and there must not be
one, because it is the fourth omission state op refused and `dimension.toml`
already corrected itself for inventing per axis. The row wants `rounding:
rounding any: proof, addition at a common scale performs no rescale so no
rounding step exists`. **The marker is what makes writing that honest instead of
inflationary**, and I would say the question is answered by this spelling rather
than merely compatible with it.

**`sentence_kind = "theorem"` becomes derivable and should stop being asserted.**
A row is a theorem exactly when every entry of its predicate carries a `proof`
warrant. Once the per-axis form exists, the row-level word is a second,
unfalsifiable copy of a fact stated per axis, and the count of independent
warrant claims should go from two to one. **I am not proposing that here**,
because it would restate the eight existing files and the ruling declined that.
It is the natural follow-on once the ratchet has run down, and I flag it so
whoever gets there does not re-derive it.

**The two-coordinate problem survives.** A width universal spelled `I any` plus
`F any` carries the warrant on each entry independently. Nothing special is
needed, and that is a consequence of per-axis granularity rather than a feature
anybody has to remember.

## 9. Challenging the thing itself, as instructed

**Should the distinction exist?** Yes, and the corpus is what says so rather than
the ruling. Three committed spans are already warrants written as prose and
indistinguishable from sweep-notes; `dimension.toml` requires "the structural
argument as the warrant" and gives it nowhere to go; a ratified ruling applies the
test in a free-text field on a different row; and eight rows assert proofhood in
a place with no axis and no construction. **Five separate places in the canon are
carrying this one fact in five hand-maintained shapes, and none of them can be
checked.** That is the situation a single graded slot exists to collapse.

**Is the predicate notation the right place?** This is the challenge worth
taking seriously, and the argument against is decent: the row already carries
`evidence`, `provenance`, `standing`, `sentence_kind`, and its ratifying ruling
carries `promotion`. Adding a sixth warrant-shaped field could fairly be called
one more copy of the thing I just complained about.

It is not, and the reason is forced rather than chosen. **Every one of those five
is row-scoped, and the thing being warranted is a span on one axis.** The
predicate is the only structure in the schema keyed by axis, so it is the only
place a per-axis warrant can live at all. The choice is between putting it there
and not having it, and the eight rows are what "not having it" looks like.

**Could the three-way split fail to be spellable?** I looked for that, since the
brief says a failure to spell it is a legitimate return. The place it would have
failed is the doubt prohibition: if the unmarked state had to be given a positive
reading, every existing file would be restated and the ruling's own "invalidates
nothing" would be false. Section 2 is the argument that it does not, and it turns
on a distinction `dimension.toml` already draws between what indexes the world
and what indexes a run. **If a later reader wants to break this deliverable, that
section is where to aim**: if absence of a warrant is a fourth omission state
after all, the spelling has to go somewhere other than the predicate and I do not
know where.

## 10. Findings outside the question

Reported harshly, as instructed, and none of them softened.

**`arvo_checks::load` fails silently, and five of eight arms cannot tell an empty
registry from a clean one.** `checks/src/lib.rs:165-167` returns `Ok(())` for a
path that is not a directory, so `load` returns an empty registry and
`canon()`'s `.expect("mock/registry is readable")` at `checks/src/lib.rs:139`
never fires. That `expect` message is a claim that nothing checks. Measured:
`p5_warrant_spike.out` section 5, eight arms exercised, all eight silent on an
empty registry, three of them noisy on the real one only because somebody
wrote a non-empty assertion. **Any future arm asserted empty is vacuous under
this failure and nothing will say so.** The fix is one line, a `NotFound` error
in `walk` for a top-level path that is not a directory, and it is code tier, so
it needs a round rather than an edit here.

**`fusing_a_multiply_add_is_free_exactly_at_translation_equivariance` writes its
predicate in a different dialect from every other row.** `total_width: 6`,
`signedness: in {unsigned, signed}`, `arity: 3`, `threads: 1`, against the
corpus's `W = 6`, `signedness in {...}`, `arity = 3`, `threads = 1`. The shipped
arm cannot see it, because it checks the slug side only. It parses, it means what
it looks like, and it is the row that would break any future instrument keying on
the span's leading token. Not a defect in the claim; a defect in the entry, and
cheap to fix while somebody is in there.

**`W in 1..=64` appears zero times in the corpus and `W in 1..=65` twice.** The
`total_width` row of `dimension.toml` uses `W in 1..=64` as its example of the
whole container domain, so the canonical illustration of the case the ruling
names has no instance. The nearest instances are `1..=65`, `1..=19`, `1..=d` and
`1..=k - s2(k)`. Worth knowing before anybody writes an arm keyed to the literal
sixty-four: the exhaustive case in this corpus is more often a **parameterised**
whole domain (`fraction_width: in 0..=W-1`, five instances) than a numeric one,
and a parameterised whole domain is spelled identically to a sample.

## 11. Blindness

I did not open `mock/research/202608072330_the-numeral-canon-panel/` beyond
`mkdir`-ing my own probe directory and counting the entries with `ls | grep -c .`,
which printed a number and no names. I ran no `git log` at all, on any branch.

**I read beyond the four sources the brief enumerated, and I say so rather than
claim I did not.** Beyond the ruling row, `dimension.toml`, the workspace rule and
the predicate strings, I read: `mock/checks/src/*.rs` and the test bodies, which
the test gate requires and which necessarily precedes the assigned work;
`ruling.toml` around the governing row, including `1462-1470`, which turned out
to carry the applied test and an open question this answers;
`registry/question.toml`'s `what_a_proof_marker_is_against_a_measurement`, being
the question the ruling names in its own `answers` field; and row context in
`proposal.toml`, `proposal-the-later-topics.toml` and `law*.toml`, which the
census needed to attribute an entry to a row.

All of it is committed registry and crate source, which is shared input every
seat reads. None of it is another seat's file. **My read of the blindness
requirement is that it is independence from the parallel derivation, not
ignorance of the canon**, and on that reading it holds. If the dispatcher meant
the stricter thing, this paragraph is the record and the deliverable should be
discounted accordingly rather than trusted on my say-so.

## 12. What I did not settle

- **The blocklist arm is weak and I have no better instrument.** Section 6 says
  so. Somebody with a better idea for mechanically separating a named mechanism
  from a restatement would improve this materially, and I could not find one.
- **Whether `exhaustive` should also be admissible on a span that is a set**
  (`W in {1, 2, 3}` where three is the whole domain). I left it legal because a
  bounded whole domain can be enumerated either way, but I did not look for a
  case that makes it wrong.
- **The ceiling of eight is a count I measured, not a triage.** The ratchet's
  sibling names why each of its six is stuck, one at a time. I have not read the
  eight closely enough to do that, and a ratchet whose comment says only "eight"
  is weaker than one whose comment says why.

## Predicates

Corpus findings, in the notation, over the committed tree at `14d0bbab`.

```
the 527-entry census, the delimiter measurement, the zero-reparse result:
  holds for: mock/registry/*.toml at 14d0bbab, all 12 files, all 3 predicate-bearing
             fields (proposal.predicate, law.holds, law.fails), 527 entries, threads = 1
the eight theorem rows and their width spans:
  holds for: mock/registry/*.toml at 14d0bbab, sentence_kind = theorem, namespace = proposal,
             8 of 8 rows, threads = 1
the empty-registry vacuity:
  holds for: mock/checks at 14d0bbab, 8 of the crate's 30 finding-returning arms, rustc
             1.98.0-nightly (57d06900f), edition 2024, threads = 1
```

The arms not exercised are not claimed about. Eight of thirty is what I
ran, and the other twenty-two are a question rather than a result.

## Probes

All in `220_probes/`, sources and outputs both committed.

- `p1_predicate_census.sh` / `.out` — 527 entries, per-axis shape census, controls
  on the classifier and on a known-absent pattern.
- `p2_warrant_vs_region.sh` / `.out` — row-aware, `sentence_kind` against the
  width span it writes, controls on the reader and a planted classification.
- `p3_delimiter_collision.sh` / `.out` — eight candidate delimiters against 527
  spans, plus the prose clauses that are already warrants, with a positive
  control on the comma.
- `p4_load_on_a_missing_dir.sh` / `.out` — the `walk` guard and the `expect` that
  cannot fire.
- `warrant_spike/` and `p5_warrant_spike.out` — the grammar, the six arms, nine
  planted cases, the reachability control that deleted a seventh arm, the
  append-only run over the committed canon, and the empty-registry section.
  Build with `cargo run` from `warrant_spike/`; it exits non-zero if any case
  misbehaves.

---

# Reconciliation, after the blind commit

Everything above is `060ddccb`, committed before I opened the panel directory.
What follows was written after reading `217`, `OPTIONS` at
`#q65-whether-this-panel-s-own-findings-satisfy-the-notation-it-ratified`,
`159_kiselyov_reply.md`, and the ledgers. **No parallel seat's file existed to
read**: `ls` of the panel root shows `217` and then me, so my blindness was
intact by construction rather than by discipline, and I claim no credit for it.

## The derivation survives, and the origin file already had the three-way split

`159_kiselyov_reply.md` section 5 states the distinction I derived, in one
sentence and before the ruling existed: "the notation currently cannot
distinguish 'sampled three widths' from 'swept the entire domain of the thing'
from 'proved for all widths', and all three land as a non-`any` predicate that
reads as narrow."

That is the same three-way split and I take no independence credit for the
observation. What I did not find already made, and what I think this deliverable
adds, is the argument in section 1 that the split is **not a split among
regions**. Both `159` and `OPTIONS` reach for a region reading and get stuck on
it, which is visible in the question `OPTIONS` leaves open: whether `W in 1..=64`
"reads as `any` bounded by the container, or as a fixed set that claims nothing
outside itself."

**That question has no answer because it is two questions.** The region is the
span exactly as written, a bounded range claiming nothing outside itself, which
is the second horn. The completeness that makes it *feel* like `any` is the
warrant, which is the first horn. Nobody could choose between them because each
is right about a different thing, and the notation had one slot. **I would call
that sub-question closed rather than answered**, and I would rather say which of
the two mechanisms it decomposes into than pick a horn.

## `159` is also the proof that the warrant slot is already being used

Two of its findings widen on an argument, and it states both **in prose in its
own file**, correctly, because the append-only rule forbids editing the original
and the notation gave it nowhere else:

- F6, recorded at `W = 13`, widened to `W any where W mod 8 != 0`, "because
  `size_of` is denominated in bytes, so every Rust type's bit size is a multiple
  of eight".
- F1, recorded at `N in {13, 47}`, widened to `N any`, "because a width the
  compiler cannot know forces a range check, for any width whatever".

Both name the construction. Both pass the ruling's test. Both live in a paragraph
of a member file, reachable by whoever reads that file and nobody else. Under the
spelling above, the first is:

```
"total_width: W any where W mod 8 != 0: proof, size_of is denominated in bytes so no Rust type has exactly W bits where W is not a multiple of eight"
```

**That is now five places in the corpus carrying warrants with no slot**, up from
the four I counted blind: three prose spans in committed predicates,
`dimension.toml`'s `access_pattern` note, a ratified ruling's `promotion` field,
eight `sentence_kind = "theorem"` assertions, and these member-file widenings.
I did not have to look far for any of them, which is the argument for the slot in
one line.

## A drift in the ratified row, and it changes what `exhaustive` is about

**The third state is about widths at its origin and about values by the time it
is ratified.** Three renderings, in order:

- `159_kiselyov_reply.md` section 5: `W in 1..=64` is "exhaustive over every
  **width** a `u64` container can hold".
- `OPTIONS`, at the Q65 heading: "it is the whole domain of a `u64` container,
  exhaustively", introduced by `W in 1..=64`. Widths.
- `217`, in the coordinator's rendering of the option op selected: "every
  **value** of a container, exhaustively, at that container's width." And then
  `ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked`'s `says`:
  "exhaustively over every **value** a container holds."

Widths and values are different objects and the corpus has instances of both. The
absorption law is quantified over values ("a hundred interval numerals containing
zero"); `159`'s three findings are quantified over widths. **The drift is one
word, in the ratified text, in the direction that changes which axis the marker
qualifies**, and whoever writes the first `exhaustive` entry will read the ruling
rather than the origin.

**It does not change the spelling, which is the one piece of luck here.** The
warrant qualifies whatever axis its entry names, so the width reading is
`"total_width: W in 1..=64: exhaustive, ..."` and the value reading is
`"operand_window: operand window = full range: exhaustive, every value a u64
holds at W = 4"`, on the axis `dimension.toml` already declares for which subset
of a representable set the operands are drawn from. **A marker that had been
designed around the width axis specifically would have had to be redesigned
here**, which is the argument for having derived it as a warrant on an arbitrary
axis rather than as a width spelling.

I report the drift rather than resolve it. It is a ratified row and the reading
is not mine to pick; what I would say to whoever does is that both readings are
real cases in the corpus, so the honest answer is probably that the marker covers
both and the ruling's sentence is narrower than the thing it named.

## `W in 1..=64` appears nowhere in the registry because those findings were never ported

My blind census reported zero instances and I flagged it as odd. `159` explains
it: the three instances are its own findings, in `154_probes/p2_fibre/FINDINGS.md`
and in its own file, and no `proposal` row carries them. So the canonical example
of the case the ruling names has **no registry instance at all**, and the nearest
things in the registry are `W in 1..=65`, `1..=19`, `1..=d`, `1..=k - s2(k)` and
the five parameterised whole-domain fraction spans (`F in 0..=W-1` and kin).

Two consequences, both practical:

- **An arm keyed to the literal sixty-four would fire on nothing.** The
  exhaustive case in this corpus is more often a parameterised whole domain than
  a numeric one, and a parameterised whole domain is spelled identically to a
  sample. That is the strongest argument I have found for the warrant being
  written rather than inferred: **no instrument can derive `exhaustive` from
  `F in 0..=W-1`**, because the same string is a sample when `W` is a bound
  somebody chose.
- **The first three rows that would carry the marker are not in the registry.**
  If somebody wants the marker exercised rather than merely defined, porting
  `159`'s three is where the instances are.

## What I checked and did not change

- **The ruling carries no `quote` field, and that is correct.** `217` records the
  answer as a two-of-four multi-select from an options list, with no words of
  op's on this question. The row's `ratification` field says exactly that. The
  shipped test `the_rulings_with_no_verbatim_are_the_ones_the_corpus_has_no_words_for`
  is what pins it, and it is green.
- **Nothing in `DROPLIST` retires anything I have proposed.** Its marker entries
  are about strategy marker traits and a width-ceiling marker, a different sense
  of the word.
- **`AGREEMENTS` records Q65's marker question as op's** and carries no prior
  agreement on the spelling, so this is not a rediscovery of a settled position.

## One thing I would now say more strongly

Section 3 argues that `sentence_kind = "theorem"` is the bare-token experiment
already run. Having read `159`, I would put it harder: **the corpus contains
proofs that pass the ruling's test and are not marked `theorem`, and rows marked
`theorem` that carry a sweep's region.** The row-level word is uncorrelated with
the property in both directions, which is worse than the census alone showed. It
should be derived from the per-axis warrants once the ratchet has run down, and
until then it should be read as a genre label rather than as a claim.

---

# The two things I conceded, attacked

Section 12 listed three residues. Two of them turned out to be work rather than
walls, so I did the work. The third, whether `exhaustive` should be admissible on
a set-shaped span, I still have not found a case that decides, and it stays a
residue.

## The blocklist arm, replaced by a test pointed the other way round

The concession was that `warrant-clause-is-a-bare-relabel` is a phrase list and
therefore carries the defect `checks/src/shape.rs` documents about its own
retired one: **a blocklist has to anticipate every bad phrase**, and the set of
bad phrases is not finishable.

**Invert it.** Rather than forbidding phrases, require that the clause names
something **outside the vocabulary of warranting**. The observation that makes
this work is that a relabel is built *entirely* out of the notation's own words:
proof, construction, structural, argument, width, any, holds. A mechanism is not,
because a mechanism is about the thing rather than about the warrant. And **the
vocabulary to strip is closed and finishable**, being the words this one notation
uses to talk about predicates, plus the axis names, plus stop words. That is a
list somebody can finish writing. A blocklist is not.

`220_probes/clause_test/`, eighteen clauses, **six of the eight mechanisms taken
verbatim from the committed corpus** rather than invented, and the relabels
including one written adversarially to beat the test.

```
quietest mechanism: 2 content tokens
loudest relabel:    0 content tokens
misclassified:      0 of 18
```

**Every relabel scores exactly zero**, the adversarial one included: "the width
cannot enter the argument by construction of the proof" is nine words and not one
of them is about anything. The real warrants score two to nine. The populations
separate at zero against two, so any floor in `(0, 2]` works and one is the
safest point in it.

**I guessed three before measuring and it misclassified the shortest real
clause.** That is the reason to run the thing rather than reason about it, and
the floor in the spike is now one, set from the measurement.

**What it still cannot do**, said plainly: somebody can pad a relabel with
content words that are not about anything ("addition banana rescale" scores
three). No test catches that and I am not going to pretend otherwise. What
changes is the failure mode. A blocklist fails **open** on every phrase nobody
anticipated, silently, forever. This fails **closed** on the lazy case and can
only be beaten by deliberately typing a lie, which is the case the human gate at
promotion exists for and the case no checker was ever going to reach.

So the arm becomes `warrant-clause-names-no-mechanism`, the blocklist is deleted
rather than kept alongside, and the honest claim is one notch stronger than the
one I made blind.

## The ceiling of eight, triaged

The concession was that a ratchet whose comment says only "eight" is weaker than
one that says why each is stuck. Here is why each is stuck. **Three shapes, and
the distribution is itself the argument for per-axis granularity.**

**A genuine proof recorded as a sweep. Three of the eight, and the first says so
in its own `because`.**

- `a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains`
  writes `W in {4, 5, 6}` and argues: "A quotient satisfies every identity its
  source satisfies, because an identity is an equation between terms and the
  quotient map sends a proof of the equation to a proof of its image. **So no
  sweep is needed.**" The row states that it is not a sweep and its predicate
  says three widths. **This is the clearest instance in the corpus of the exact
  thing the ruling was ratified about**, and it wants
  `total_width: W any: proof, the realisation map is a congruence so the induced
  structure is a quotient and inherits every identity of its source`.
- `a_multiplicative_chain_is_writable_without_an_ever_growing_intermediate_by_windowing`
  carries no width axis at all, and its argument is a width-parametric identity:
  "a sum of `k` values each of fraction width `F` needs `kF` fraction bits". A
  proof in `k` and `F`, at any width, reading under I13 as holding at no width.
- `the_join_over_demands_is_union_and_it_is_free` carries no width axis, and
  correctly so: **the object has no width.** It is a free join semilattice on `d`
  generators. Under I13 that still reads as holding at no width, and this is the
  row where the absent-versus-inapplicable pressure is sharpest. **The marker is
  what lets it say the honest thing**, `total_width: W any: proof, the object is
  a semilattice on demand generators and no width appears in it`, without
  inventing the inapplicable state op refused.

**Mixed: a proof on one axis and a sweep on another. Three of the eight, and no
row-level word can describe any of them.**

- `no_multiplicative_structure_survives_a_nonzero_fraction_width` proves over
  policy and reduction ("the failure is in the ambient operation and no choice of
  reduction reaches it") and sweeps `W in 3..=7`.
- `the_laws_of_a_format_are_derived_from_two_hypotheses_rather_than_enumerated_per_policy`
  derives the two hypotheses and then evaluates "mechanically over the unit's
  cube" at `W = 4`.
- `inside_a_fragment_with_a_complete_test_set_the_verdict_is_computed_at_the_shipped_width`
  proves ("a complete tensor proof reduces to evaluation on the degree grid") and
  measures at 1,975 pairs and 23,950,484 evaluations, over `W in 1..=65`. **This
  is the corpus's best candidate for `exhaustive`**, and it is the row to write
  the first one on.

**A measurement wearing the word. One of the eight, and its own note concedes
it.**

- `fusing_a_multiply_add_is_free_exactly_at_translation_equivariance` is
  `sentence_kind = "theorem"` over a twelve-cell table at `total_width: 6`, and
  its `note` says "a clean sweep does not settle a property the way a
  counterexample settles its negation." The row is honest and the label is not.

**Already consistent. One of the eight.**

- `the_model_band_transfer_is_defeated_in_both_fragments` writes `W any` and its
  argument is Legendre's formula, so the region and the word agree. **Even here
  the row does not say why `W any` is earned**, and a reader has to open
  `because` and recognise the construction. That is the whole case for the
  marker on a row that has nothing wrong with it.

**So: six of eight are mixed or are proofs, and every one of the six needs a
per-axis warrant to be stated correctly.** That is a measurement rather than an
argument, and it is the strongest thing I can put behind the locus.

The ratchet's ceiling is eight and its comment is the four paragraphs above. **It
falls to seven the day somebody writes the congruence row's warrant**, which is
one line and no new evidence, and that is the right first move for whoever picks
this up.

---

# Reply to 219, and what it supersedes above

**Read this before acting on section 5.** Two things in my answer are superseded
by seat 219's file and I concede both: **the token for a width-free argument is
`construction` and not `proof`**, and **the marker's position is not settled by
my census, because it is downstream of a question neither of us framed as the
fork it is.** Section 6's arms stand, section 4's candidate list stands and gains
two entries it was missing, and section 1's derivation stands and was never two
instruments to begin with.

Its file is `219_kiselyov_the_marker_spelling.md`; its probes are `219_probes/`.
My verification instrument for this reply is `220_probes/reply_check/`, which
reads the registry through `arvo_checks`'s parser rather than through a
hand-rolled scanner, so where it and 219's scanner agree that is two instruments
and not one method run twice.

## 1. Where it is right and I am not

### The token is `construction`, and its reason is my own argument turned around

I wrote `proof`. 219 writes `construction`, and gives three reasons. The third is
decisive and it is **the argument I spent section 3 making**: a
construction-warranted axis can sit on a row whose overall standing is one
expert, so `proof` is a claim about the sentence made in a field that is about an
axis. That is the row-level-versus-axis-level confusion I diagnosed in
`sentence_kind`, and I then reintroduced it in my own token. Conceded without
reservation.

**I went looking for a collision to defend `proof` with and found the opposite.**
`reply_check` Q5: `construction` appears in **80 fields across the registry** and
in **7 row ids**, and the uses are this exact sense.
`law::double_rounding_is_innocuous_at_an_intermediate_width_between_f_and_2f`
carries "The construction is named in the governing f..." in its `gap` and "the
construction is parametric in F" in its `note`. It is the corpus's own word for
the thing, already, in prose, on rows that would carry the marker. **My attack
refuted my token rather than 219's**, and that is a better result than the one I
was hunting for.

### `swept` as an optional third token

I shipped two tokens and treated the unmarked case as carrying the whole of the
sweep population. 219 adds `swept`, admissible and never required. Conceded, and
the reason it is right is one I missed: with only two tokens, an unmarked entry
is **permanently ambiguous** between "swept" and "written before the marker
existed", and nothing can ever separate them. With `swept` available a new entry
is unambiguous and the ambiguity is confined to the entries that predate the
marker, which is a closed and shrinking set. Requiring it would restate every
committed entry, which is why optional is the right strength.

### The ratchet's population is better chosen than mine

My ratchet keys on `sentence_kind = "theorem"` rows carrying no per-axis warrant,
ceiling eight. 219's keys on unmarked universals, ceiling 38. **Its population is
the one where the risk lives**: a universal is the entry that claims the whole
axis, so an unwarranted universal is the expensive mistake, and eight
theorem-labelled rows is a symptom of that class rather than the class. Conceded.

They catch different things and both are worth having. Mine catches a row-level
relabel over a sweep-shaped region, which is invisible to a ratchet over
universals because five of my eight write no universal at all. **Two ratchets,
not one**, and I would keep both.

### Two live canon violations I did not find

219's `values_side_admissibility.rs` reports two classes I read past. Both are
real and I confirm both.

`proposal::three_topics_independently_terminate_on_the_strategy_axis_as_their_shared_placeholder`
writes `operation: operation any`, and `dimension::operation` says in bold in its
own `grammar` that `operation any` is not admissible. **My own census printed
`1 operation	operation any` in its section 4 and I did not look at it.** The
`dimension::operation` `note` predicted this failure in terms and it happened
anyway, in a different file, which is the whole of the finding.

And five values sides bind more than one thing, two of them packing an undeclared
axis inside a declared slug (`emission`, `feature gates`). I read those exact
strings in my delimiter census, in the "spans containing a comma followed by an
explanatory clause" list, and classified them as prose rather than as a defect
class. **The strings were in front of me and I read them for the wrong property.**

## 2. Where it is wrong, and both errors have one cause

219's headline counts are 38 unmarked universals and 25 mixed rows. My instrument
reads **41** and **28** over the same tree, and the gap is exactly three entries
on three rows. This is decidable and it is decided.

### The instrument is anchored to the end of the values side

`219_probes/warrant_census.rs:159`:

```rust
fn is_any(values: &str) -> bool {
    values == "any" || values.ends_with(" any")
}
```

Its doc comment says so plainly ("how many **end in** the bare token `any`"), so
this is a definition rather than a bug. **The definition is what is wrong for the
purpose it is put to.** A universal followed by anything is not counted as a
universal, so the count depends on whether the author wrote a trailing clause.

The three it misses, with their rows, from `reply_check` Q1:

| entry | row | `sentence_kind` |
|---|---|---|
| `threads: threads any, the equalities being decided at compile time` | `proposal::no_derivation_reads_the_grid_so_a_composition_may_hold_it_at_run_time` | `measured` |
| `threads: threads any, the refusal being a type-check outcome that precedes execution` | `proposal::a_fold_needs_a_closed_operation_and_a_separately_determined_accumulator` | `measured` |
| `threads: threads = 1 for the timed instance and threads any for the compile-time artifacts` | `proposal::a_compile_time_strategy_selection_leaves_no_residue_in_the_emitted_body` | `measured` |

All three are `measured`, which accounts for the second discrepancy exactly:
219's mixed-row kinds are `{"(none)": 7, "argument": 7, "measured": 7,
"theorem": 4}` and mine are `{"(none)": 7, "argument": 7, "measured": 10,
"theorem": 4}`. **One instrument defect, two wrong headline numbers, and the
diagnosis is checkable from its source in one line.**

### Why this is not a rounding error

**The three entries the anchor misses are the corpus's own existing warrants.**
Two of them are the proofs I identified blind, in section 4 of this file, as the
evidence that the content already exists and only wants a slot: "the equalities
being decided at compile time" and "the refusal being a type-check outcome that
precedes execution". They are missed **because** they carry a warrant. An author
who writes the warrant in prose falls out of the count of entries that need one.

So 219's Arm 6, the ratchet at 38, has a hole exactly the width of the thing it
guards, and the hole grows every time somebody does the right thing informally.
**The ceiling is 41 and the three above are named in its comment.**

It is also internally inconsistent, which is what makes it a defect rather than a
policy: 219's own Q1 list counts `emission in {metadata only, full codegen},
debug-assertions any` as a universal. That is a universal inside a longer values
side too. It is counted only because it happens to end in `any`.

### The registry does hold bounded whole-domain ranges

219 challenges the standing of the third state: "**The registry contains no
whole-container range.** The census looked for `1..=64` and `0..=63` across all
527 entries and found zero." The literal search is right and **I reproduce the
zero**: `reply_check` Q4, 35 range entries, zero matching either literal.

The conclusion drawn from it is too strong. `reply_check` Q4 also reports **seven
range entries whose upper bound is an axis or a formula rather than a number**:

```
proposal::where_fusion_changes_the_answer_it_is_not_a_lowering    fraction_width => in 0..=W-1
law::the_fused_and_the_stepwise_multiply_add_denote_one_function  fraction_width => in 0..=W-1   (holds and fails)
law::rounding_retraction_is_the_identity                          fraction_width => F in 1..=W
proposal::a_min_plus_fold_needs_an_absorbing_top_...              fraction_width => F in 0..=W
law::the_saturating_exponent_absorbing_identity                   total_width    => W in 1..=d
law::the_falling_factorial_family_vanishes_modulo_two_to_the_width total_width   => W in 1..=k - s2(k)
```

`F in 0..=W-1` is **every legal fraction width at a given total width**. That is
a bounded whole domain, walked in full, spelled parameterically. It is the third
state, in the registry, three times, and a literal search for `1..=64` cannot see
it.

**This is the same fact I reached blind from the other side.** My section on the
missing sixty-four says "no instrument can derive `exhaustive` from
`F in 0..=W-1`, because the same string is a sample when `W` is a bound somebody
chose". 219 concluded absence from the same registry; I concluded
present-but-unspellable. The rows decide it, and they decide it my way.

So: `exhaustive` is **not** derived-without-validation, there **are** live rows to
try it against, and 219's "the first row that needs one should be written with
the marker rather than retrofitted" is good advice pointed at a case it thought
was in the future and is not.

### One thing I could not attack and tried to

219's Arm 1 requires the construction as a **citation** to a row or probe rather
than as a sentence, on the ground that "a citation is a thing another seat can
open and attack, where a sentence is a thing another seat can only agree with."
I went looking for the cost of that and measured it rather than arguing it.
`reply_check` Q6: of the **31 distinct rows carrying an unmarked universal, 20
carry at least one outbound reference and 11 carry none**, so Arm 1 would make
eleven authors create a citable target before they may write a warrant.

**Eleven of thirty-one is a real cost and it is not a refutation**, and I am not
going to pretend it is one. See the fork below, where it does bear.

## 3. Where we genuinely differ, and what decides it

### Fork A: the position, which is not the fork either of us thought it was

I put the warrant on the values side after a second colon; 219 puts it on the
slug side after a `/`. We each measured our own position clean over the same 527
entries and both measurements are right: zero entries carry a second colon
(`p5_warrant_spike.out` section 1), and zero slug sides are anything but a bare
slug (`reply_check` Q3, reproducing 219's Q6). **The collision question does not
decide it, because neither collides.**

219's argument for its side is that the values side "is unparsed by design and
will stay unparsed", citing `checks/src/predicate.rs:17`, and that a marker in an
unparsed field will drift.

**That argument does not survive 219's own Arm 5**, which reserves the three
tokens and says "no `dimension` grammar and **no values side** may begin with
one". Enforcing that is a values-side parse. It is a minimal one, a leading token
against a closed set, which is exactly the amount of values-side reading my
position needs and exactly the amount `predicate.rs:17`'s stated objection does
not cover: that comment refuses a regex over the per-axis span grammars, not a
fixed token at a fixed position. **Both schemes parse the values side by the same
amount, so the unparsed-field argument distinguishes nothing.**

What actually decides the position is a different question, and neither of us
put it first:

**Does the warrant carry its construction inline, or point at it?**

- If inline, the marker needs room for a sentence, and the slug side has room for
  one word. 219 concedes this in terms: "A tag on a slug has room for one word."
  The values side is then forced.
- If a pointer, the marker is one token and the slug side is the better position,
  because a leading token survives a quotation that truncates and a trailing one
  does not. That is 219's own argument against the bracketed suffix and it
  applies to my trailing clause too. **Conceded: my position is the more
  fragile of the two under quotation.**

So Fork A is **downstream of Fork B** and should not be argued on its own. Stating
that is the most useful thing I can do with it.

### Fork B: citation or clause, and this is the real disagreement

**219: the construction is a citation, and an entry marked `construction` with no
resolvable target is refused.** Its reason is the strongest single argument in
either file, and it is the workspace's own evidence discipline: a citation is
attackable and a sentence is not.

**Me: the construction is a clause, guarded by a content test** which strips the
notation's own vocabulary and requires what is left to be non-empty
(`p6_clause_test.out`: 0 misclassified over 18, every relabel scoring exactly
zero, the adversarial one included).

Neither is wrong and each loses something the other keeps.

- **A citation cannot be padded.** My content test can: "addition banana rescale"
  scores three. I said so before reading 219 and it remains the honest limit.
- **A clause cannot be unfindable.** 219's Arm 1 sends the reader to a row; my
  clause is at the entry, where a reader already is.
- **A citation has a floor cost, measured at 11 of 31 rows** with no outbound
  reference today. And not every construction deserves a row:
  `the_join_over_demands_is_union_and_it_is_free`'s warrant is "the object is a
  semilattice on demand generators and carries no width", which is an observation
  rather than a proposition anybody would ratify separately. Forcing a row for it
  populates the registry with rows whose only purpose is to be cited.
- **The friction is the mechanism the ruling blames for the gap**, in its own
  `because`: "a width-free argument had no honest spelling, so its authors wrote
  nothing". A scheme whose cheapest honest path costs a new row risks buying that
  outcome back.

**What decides it: nothing either of us has, and I do not think more measurement
gets there.** The question is whether an uncited clause is worth more than
nothing, and that is a call about what the canon is for rather than a fact about
the corpus. I can measure the cost of requiring a citation and I did; I cannot
measure the value of the clauses that would not be written.

**So I offer a composition rather than a winner, and I would rather it be
attacked than adopted.** The warrant carries a clause always, and the row carries
the citation where a target exists; Arm 1 becomes a **ratchet on
construction-without-citation** rather than a gate, so a cited construction is
the norm, an uncited one is a bounded and visible population, and the cheap
honest path stays open. Both of us independently reached for a ratchet where a
gate would restate committed work, so the shape is not new to either file.

**Under that composition Fork A resolves to 219's side**: the token goes on the
slug as `total_width/construction`, the citation goes in the keyed `construction`
field, and the clause goes where the clause already goes, which is the row's
`because`. My second colon then buys nothing and I withdraw it.

**If the panel takes 219's Arm 1 as a gate instead, Fork A still resolves to
219's side** and my position is simply beaten. **The only branch where my
position survives is the one where the clause is mandatory and inline**, and I
am not confident enough in that branch to argue it is the right one.

### Fork C: not a fork, and worth saying so

219 asks, as the thing it would attack first in my position, whether
`construction` should be one token or two, splitting "the operation does not read
this axis" from "the axis was varied and nothing moved". It says it treated the
differential run as corroboration rather than as a warrant of its own, believes
that is right, and is not certain.

**It is right, and the reason is I13 rather than taste.** A run over a range is a
sweep over that range whatever it found, so "varied and nothing moved" is
`swept` with a wide region, and giving it its own warrant token would let a wide
sweep read as a construction. That is the ruling's named failure with extra
steps. One token.

## 4. The agreement inventory, intersected over instruments rather than names

219 flagged that we both read `every-finding-carries-its-predicate.md` and the
`dimension.toml` header unvaried, and that convergence downstream of those is
shared premise. It is right and I am applying it to my own agreements, including
the ones I would rather count.

| what we agree on | two instruments, or one premise read twice | the region the agreement actually covers |
|---|---|---|
| Warrant is orthogonal to region; the three things are not three regions | **One premise.** `every-finding-carries-its-predicate.md` states three-states-and-no-fourth and `dimension.toml`'s `total_width` note names the unnamed third state outright. Neither of us varied either. **Not corroboration.** | nothing measured |
| Two independent elaborations of it | 219: a construction can be bounded, so warrant and extent cross. Me: two rows with identical strings and different warrants. **Two arguments, still one premise.** | nothing measured |
| `sentence_kind` cannot carry the marker | **Two instruments.** 219 counted mixed rows; I triaged the eight `theorem` rows' spans. | committed registry at `14d0bbab`, `proposal` and `law`, all 527 entries |
| Per-axis granularity is required | **Two instruments**, over overlapping populations: its four mixed `theorem` rows are inside my eight | `namespace = proposal`, `sentence_kind = theorem`, 4 rows in the intersection |
| Unmarked claims no warrant, and it is not a fourth state | **One premise, two arguments.** 219 refutes four candidates against its count; I separate what indexes the world from what indexes the argument. 219's has a measurement behind it and mine does not, so **its instance is the stronger one** | 219's half: 38 (really 41) universal entries |
| The shipped checker reads only the slug side | **One document read twice**, `checks/src/predicate.rs`. Not corroboration | nothing measured |
| The committed `docs/` were stale, and we both regenerated | **One command run twice.** Not corroboration | nothing measured |
| `1..=64` and `0..=63` appear zero times | **Two instruments**, its literal search and my shape census | committed registry, all 527 entries, those two literals only |
| The registry holds no bounded whole-domain range | **Disagreement, not agreement.** See above; seven parameterised-bound ranges, three of them `F in 0..=W-1` | resolved against 219 |
| An `exhaustive` entry must name its container | **Two instruments and a third arrival.** I derived it from the ruling; 219 derived it and then found `157`'s F157-6 writing the container beside the range unprompted | the arm as stated; the corroborating instance is one member file |

**Three genuine two-instrument agreements, one resolved disagreement, and four
items that are one premise or one document wearing two hats.** A consolidation
that reports us as converging on the derivation is reporting the second group.

## 5. What this changes in my file, restated so nobody has to reconstruct it

- **Section 5's token `proof` is withdrawn. It is `construction`.**
- **Section 5's `<axis>: <span>: <warrant>` is withdrawn under the composition in
  Fork B**, which is where I expect this to land. It stands only on the branch
  where the clause is mandatory and inline, which I do not argue for.
- **A third token, `swept`, is adopted**, optional and never required.
- **My ratchet at eight stands and is joined by 219's**, whose ceiling is **41**
  rather than 38, with the three entries above named in its comment.
- **Sections 1, 2 and 3 stand**, and section 1 was never two instruments: it is
  one premise elaborated twice, and I have marked it so.
- **Section 6's arms stand**, with `warrant-clause-names-no-mechanism` demoted
  from the guarantee to the fallback that guards the uncited path.
- **Section 4's candidate list gains two entries it missed**: the slug-side
  suffix, which is 219's answer and which I did not consider, and the bracketed
  prose tag already in interim use across the member files, which 219 found and I
  did not.

## 6. The probe

`220_probes/reply_check/` and `p7_reply_check.out`. It reads the registry through
`arvo_checks` rather than through a hand-rolled scanner, so its agreements with
219 are two instruments. Six questions: three reproduce 219's headline numbers,
three attack. Its control plants a tree where every arm returns a value different
from the committed one, including the dirty-slug arm whose committed answer is
zero and which would otherwise be a check that has never fired.

## 7. What attacking 219's count turned up, which neither of us had

Its matcher is `values == "any" || values.ends_with(" any")` and its doc comment
justifies it: "Every declared grammar spells it as a trailing bare `any`, which
is what Q1 checks rather than assumes." **That sentence is true of the grammars
and false of the corpus**, and chasing why produced a bigger finding than the
three-entry gap that started it.

### The `threads` axis conforms to its own declared grammar 19 times in 36

`dimension::threads` declares exactly `threads = 1`, `threads = <n>`, or
`threads any`. `220_probes/p8_threads_grammar.out` walks all 36 threads entries
against that:

```
[conforms]  16  threads = 1
[DEPARTS ]  13  1
[conforms]   3  threads any
[DEPARTS ]   1  threads any, the refusal being a type-check outcome that precedes execution
[DEPARTS ]   1  threads any, the equalities being decided at compile time
[DEPARTS ]   1  threads = 1, the splits being computed rather than executed on lanes
[DEPARTS ]   1  threads = 1 for the timed instance and threads any for the compile-time artifacts
```

**Seventeen of thirty-six depart.** 219's Arm 7 refuses a values side writing a
spelling its axis's grammar declares **inadmissible**, and found one. The wider
arm, refusing a values side that does not write a spelling its axis's grammar
**declares at all**, finds seventeen on one axis. That is the same arm with the
quantifier the other way round and it is worth far more.

### And thirteen of those seventeen are a second dialect, which is two whole files

The bare `1` is not a slip. `p8_threads_grammar.out`, second section:

```
law-the-later-topics.toml         71 of  71 entries write a span naming nothing from their slug
proposal-the-later-topics.toml    68 of  68 entries write a span naming nothing from their slug
law.toml                          68 of 162
proposal.toml                     69 of 226
```

The two later-topics files are **total**, and I checked them by eye rather than
trusting the heuristic. Side by side, the same axes, same tree:

```
proposal.toml                    proposal-the-later-topics.toml
"total_width: W in 3..=7"        "total_width: 6"
"signedness: signedness in {…}"  "signedness: in {unsigned, signed}"
"arity: arity = 3"               "arity: 3"
"threads: threads = 1"           "threads: 1"
```

**139 of 527 entries, 26 percent of the corpus, in a second dialect that drops
the axis word from the values side entirely.**

The counts for the two main files are a heuristic and carry false positives
(`container: interval numerals containing zero` names no slug word and is fine).
**The two hundred-percent figures are not a heuristic**, and they are the finding.

### This corrects my own blind file

Section 10 of this file reports
`fusing_a_multiply_add_is_free_exactly_at_translation_equivariance` as writing
"its predicate in a different dialect from every other row". **That is wrong. It
is not one row, it is two entire files.** I named the instance the census handed
me and did not grep for the class, which is precisely the failure
`fix-the-class-not-the-instance-named.md` describes, committed in a file that
spends a section complaining about instruments that stop at the first hit. The
class is 139 entries and the correction is here rather than in section 10,
because that section is the blind record.

### What it does to the two schemes, and it is not symmetric

**It hands 219's position a much stronger argument than the one 219 gave**, and I
would rather supply it than win the fork.

219 argues the values side is untrustworthy from five prose entries. The real
number is that **the values side is written in two dialects, one of which drops
the axis word in 139 entries**, while the slug side is uniform across both:
`reply_check` Q3 reports **zero** non-bare slugs in all 527, spanning both
dialects. So the slug side is the only half of the entry that means the same
thing everywhere, and a one-word tag placed there is the only marker position
that does not have to know which dialect it is sitting in.

My trailing-colon position survives this on its own terms, because it splits on
the colon rather than on the span, and a second colon is absent from both
dialects. **But 219's Arm 5, the reserved leading token, does not**: a token in
front of a bare `6` has no dialect-independent reading, and Arm 5 is the arm I
used above to argue that both schemes parse the values side equally. They do, and
now both of those parses are standing on a field that is not uniform.

**So the conclusion moves further toward 219 than my Fork A analysis had it**, and
for a reason neither file contained: the slug side is uniform and the values side
is not, measured, at 527 entries and two dialects.

### The arm I would land before either warrant arm

**A values side writes a spelling its axis's declared grammar admits.** Not
"is not declared inadmissible", which is 219's Arm 7 and finds one. The positive
form, which finds seventeen on the one axis I walked and would be worth walking
across all twenty-one. **The dialect is the thing to settle before a marker is
placed anywhere**, because both candidate positions are parses of a field whose
shape nobody has pinned, and a marker added on top of an unpinned field inherits
whatever the next dialect does.

I did not walk the other twenty axes. That is the obvious next measurement and it
is cheap; `p8_threads_grammar.sh` is one axis wide and generalises by changing a
string. **`threads = 1` at 16 of 36 is a fact about the threads axis and I claim
nothing about the other twenty from it**, beyond the two-dialect count, which is
whole-corpus and is measured.

## Predicates for the reply

```
the 41 universals, 28 mixed rows, 0 dirty slugs, 35 range entries, 7 parameterised bounds,
80 `construction` fields, 31 universal-bearing rows of which 11 have no outbound edge:
  holds for: mock/registry/*.toml at 14d0bbab, all 12 files, all 3 predicate-bearing
             fields, 527 entries, read through arvo_checks, threads = 1
the threads-axis grammar conformance, 19 of 36:
  holds for: mock/registry/*.toml at 14d0bbab, axis = threads only, 36 entries, threads = 1
the two-dialect count, 139 of 527:
  holds for: mock/registry/*.toml at 14d0bbab, all 12 files, 527 entries, threads = 1
```

The other twenty axes are not claimed about. One axis walked is one axis walked.
