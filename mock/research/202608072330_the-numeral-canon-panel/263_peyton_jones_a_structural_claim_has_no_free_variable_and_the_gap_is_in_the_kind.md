# 263. Peyton Jones: a structural claim has no free variable, and the gap is in the kind rather than in the axes

**Minted as 262 and reassigned to 263 by the dispatcher.** Two seats ran this row
in parallel and both counted the directory rather than the ledger, so both took
262. The other is `262_orchard_the_structural_region_derived_blind.md`, which
committed first and keeps the number. Section 10 predicted this collision before
it happened. The probe artifacts under `263_probes/` were produced under the
older number and their own contents are left as they were run.

Answering `question::can_a_claim_about_the_canons_own_structure_carry_a_region`, blind. Sections 0
to 9 were written to disk before I opened anything in this directory other than seat 223 at the
anchor the row's provenance names; section 10 records what I read afterwards and what it changed,
and the one paragraph of section 0 that mentions seat 261 was rewritten after that reading. The
registry I read is arvo at `b544c82cf66536bfd19e3d3f7bdf995a4a813c52`, which is `origin/dev` at the
time of writing, with twenty-five `dimension` rows and six `sentence_kind` values.

## 0. The two gates

**Canon gate: passed, against the rows named here.** I checked the work against
`ruling::the_work_is_predicated_arms_composed`, `ruling::a_predicate_lists_only_what_holds`,
`ruling::a_thing_that_constrains_the_work_and_cannot_be_designed_away_is_canon`,
`ruling::the_option_set_is_not_a_boundary`,
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`, and the header of
`mock/registry/dimension.toml`. Nothing in them forbids answering this row, and the row's `decider`
is `panel`. One thing the gate turned up that the brief did not know: the question's own premise,
that a region-free filing is available only as `normative` or `definition`, is stated nowhere in
`ruling.toml`. The search was `grep -n "sentence_kind\|normative\|regionless\|no region\|carry no
region\|carries no region" mock/registry/ruling.toml`, which returns two hits, both inside the note
and promotion of the warrant-marker ruling and neither stating the coupling; the control,
`grep -c predicate` on the same file, returns 52. So the coupling of "carries no region" to those two
kinds lives in `mockspace.toml`'s schema text and in `mock/lints/a_region_agrees_with_the_sentence_kind.rs`,
which are the design and code tiers, and is not a ratified sentence. That matters for what kind of
change an answer is.

**Test gate: run, and the first run was blocked while the next two were green at the same pin.**
The first `cargo mock` in this fresh worktree exited 1 before running a single lint: six `E0308`
mismatches on `LintPack`, two copies of `mockspace-lint-rules` in one graph, and the engine's own
notice, `BLOCKED: this repo's custom lints could not be built, so no lint below them ran. Nothing was
checked.` That log is `263_probes/cargo_mock.log`. The next full run in the same worktree, with the
launcher's registry showing the same engine build `dc747ead03d81111` at
`a7dd822305629e54c6ed4ed2c5670840ecab2677`, exited 0 with `all lints passed`, and so did a third run
after deleting `mock/target/mockspace-lints` entirely; the second is
`263_probes/cargo_mock_at_the_resolved_pin.log` and the launcher's record is
`263_probes/launcher_registry.toml`. In the failing run I saw a generated manifest with no `[patch]`
table and a `cargo tree -d` naming the crate twice, once by `rev` and once by `branch`; I did not
preserve that manifest before the engine rewrote it, so that observation is mine and not evidence,
and `263_probes/generated_pack_Cargo.toml` is the rewritten one, which carries both patch tables.
What is committed establishes that one launcher key produced one blocked run and two green ones in
one worktree within twenty minutes, and the registry says the engine build at that key was compiled
during the first run. I could not establish the mechanism and I say so rather than guess; seat 261
section 8 records the earlier URL-spelling defect at `b57007c` as repaired at this pin, and nothing
here contradicts that, since the green runs are at this pin. What it does say is that a fresh
worktree's first gate can report blocked at a pin that then passes, and a reader seeing that should
run it twice before filing anything.

The suite itself. The three source crates pass under `cargo test` in `mock/`: `arvo-format` 115
unit, 11 compile-fail, 13 parity; `arvo-placement` 21; `arvo-strategy` 10; plus 9 in the tools. The
lint pack was run as a copy placed under `mock/target/probe-pack/`, 685 tests, 669 passed, 0 failed,
16 ignored, in `263_probes/pack_test.log`; the same copy run from a scratch directory outside the
repository reported seven failures that were all `no mockspace.toml above the working directory`,
which is the testkit's `repo_root` walk and not a lint, and that log is kept beside it under its own
name so nobody spends a dispatch on it. I read the body of every test in
`a_region_agrees_with_the_sentence_kind.rs`, the surface this row touches: thirteen tests, each
planting rows and asserting a finding kind or its absence, with a positive control that an empty
registry is silent and a control that the two correct shapes are both silent. None is tautological.
The lint is real and the gate proceeds.

## 1. What I read, in what order, and what I did not

The row itself in `mock/registry/question.toml`. Then seat 223 in full, whose `#its-standing` anchor
is three sentences saying the layering is one expert's derivation filed as
`proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`. Then the `dimension`
namespace end to end, its header included, and the `sentence_kind`, `predicate`, `standing` and
`evidence` field descriptions in `mockspace.toml`. Then the two lints that read a `predicate`, and
the `topic` rows for the four machinery topics. Then the proposal rows the question is about, and a
census of the machinery-topic rows by kind and predicate. Then `ruling.toml` for anything bearing on
the notation.

I did not read seat 224, seat 258, seat 261, `OPTIONS.md`, `DROPLIST.md` or any other panel file
until section 10. I saw their file names when I listed the directory, as the brief requires, and the
title of 261 says something about this subject; I say so rather than pretend the listing was blind.

## 2. The question as one concrete program

Take the smallest structural claim the registry actually carries, one edge of the layering:

> `the_format` cannot be stated without a decision belonging to `the_container_premise`.

Now try to give it a region. The notation's job, per `dimension.toml`'s header, is to say at which
values of which axes the claim is true, where an axis "indexes a situation the world can be in: a
format has a radix, a workload's values live in a domain, a build has a profile, and a claim is true
or false at each." So the question is: what, in the world arvo is designed for, can be set to a
value at which this sentence flips?

Nothing. It does not flip at `F = 0` or `F > 0`, at `wrap` or `saturate`, on one thread or eight,
under any target feature set. The only thing whose value it depends on is the text of two `topic`
rows, that is, which revision of the registry you read. That is the whole finding, and the rest of
this file is drawing out what follows from it.

## 3. What a dimension is, per the canon's own declaration

The header of `dimension.toml` gives one test and states that gateability is a corollary of it rather
than the test itself:

> **The test is what the value indexes.** An axis indexes a situation the world can be in ... A
> parameter indexes a run: how many arms it compared, which cost coordinates it collected. Nothing
> about the world is different at five arms rather than six.

and it says where a parameter goes instead:

> The second belongs on the `probe` row that ran it, whose `establishes` field already says a probe
> shows what it shows at the widths and shapes it checked.

So the canon already distinguishes two kinds of coordinate and gives each a home: an axis goes in the
predicate, a run parameter goes in the evidence. That is the distinction I need and I did not have to
invent it.

## 4. A region is over a claim's free world-parameters, and a structural claim has none

Let me be precise about what a predicate quantifies. A finding about arvo is a sentence with free
variables ranging over the design space: the width, the policy, the operation, the strategy. Its
predicate is the set of assignments to those variables at which it holds, and `absent` on an axis
means the empty set on that axis, which `ruling::a_predicate_lists_only_what_holds` fixes as the
reading: what is not written is not claimed. A predicate is, in other words, the domain of a
proposition-valued function over the design space.

A structural claim is not a function over the design space. "The format rests on the container
premise" has no free variable ranging over anything arvo can be; it is a closed sentence about a
document. Its truth depends on one thing, the corpus revision it was read from, and by the header's
own test that coordinate is a parameter of the derivation rather than a situation the world can be
in: nothing about arvo is different because the `topic` file gained a row. So under the canon's own
declaration, the honest predicate for a structural claim is the empty function: not "holds nowhere",
which is a claim about the design space, but "is not a claim about the design space at all".

That is exactly the reading the schema already gives a `definition`. The `predicate` field's own text
in `mockspace.toml` says: "a definition is not a claim about where anything holds, so a region on one
is a category error rather than a narrowing." A reasoned structural claim is in the same position for
the same reason, and the only difference between it and a definition is how it was earned: by
argument rather than by stipulation. Which brings us to where the gap actually is.

## 5. `sentence_kind` conflates two axes, and the structural claim is the cell that separates them

Here is the delightful thing, and it is the sort of thing a type tells you. The six values of
`sentence_kind` are declared as a warrant: `theorem` is proved, `measured` ran, `enumeration` was
walked, `argument` is reasoned, `definition` is stipulated, `normative` is imposed. That is one axis,
how the sentence was earned. But the region requirement is enforced along a different axis, whether
the sentence ranges over the design space, and the schema and the lint both implement that second
axis as a function of the first: `REGIONLESS = ["normative", "definition"]` at
`mock/lints/a_region_agrees_with_the_sentence_kind.rs`, and "Absent entirely only where
`sentence_kind` is `normative` or `definition`" in the field text.

That coupling rests on a premise: every sentence earned by proof, run, walk or argument was earned
somewhere in the design space. For claims about arvo the premise is true, and the coupling is a
good compression. A structural claim is the counterexample: earned by argument, ranging over
nothing. The product `warrant × domain` has a cell, `argument × not-the-design-space`, that the six
values cannot spell, and every filing the registry offers such a claim is a lie in one direction or
the other. The same schema text says of `argument` that it "is the mark that keeps getting dropped";
the region check is the mechanism that drops it.

So, to the question as asked, "is that the intended shape or a gap in the notation":

- **It is intended for an imposed structural claim.** "Every canon sentence names the prefix it
  quantifies over" imposes an order; `normative` is its right filing and the absence of a region is
  correct. Option two is right on that subset, and the row's note is right that option two is not a
  concession.
- **It is a gap for a reasoned structural claim**, and the gap is not where option one puts it. The
  dimension vocabulary is not short an axis; `sentence_kind` is short a coordinate, or rather it is
  two coordinates wearing one name, and the second one has no value for "not over this space".

The fork the row offers, axis or governance or exile, dissolves once the two coordinates are pulled
apart: both of the first two options are half right and the half each is wrong about is the other's
half.

## 6. The gap is live, it costs something today, and the checker cannot see the worst case

**A census.** The twelve `proposal` rows on the four machinery topics, read off `proposal*.toml` at
this revision, as kind against predicate:

| topic | kind | predicate | row |
|---|---|---|---|
| `canon_form` | `normative` | none | `every_canon_sentence_names_the_prefix_it_quantifies_over` |
| `canon_form` | `normative` | none | `the_topics_form_a_stack_a_frame_and_the_canons_own_machinery` |
| `naming` | `normative` | none | `naming_is_partial_and_injective_or_it_is_broken` |
| `the_predicate_notation` | `normative` | none | four rows, two `cross_topic`, two seat 224's |
| `panel_conduct` | `normative` | none | `an_instrument_is_mutated_and_the_battery_is_made_to_notice` |
| `panel_conduct` | `argument` | `threads = 1` | `the_derived_laws_units_re_runs_reproduced_and_its_defects_were_all_in_blind_spots` |
| `panel_conduct` | `argument` | six numeric axes | `one_measured_count_was_attributed_to_the_wrong_operation_in_two_topics_and_corrected_in_both` |
| `panel_conduct` | `enumeration` | six numeric axes | `the_four_consolidations_contradict_each_other_nowhere` |
| `panel_conduct` | `measured` | `threads = 1` | `most_committed_bench_regions_predate_the_harness_cross_variant_validation` |

Eight of twelve are `normative` with no region. The layering row's own note says it was "written
`argument`" and refiled because the check forced it, so at least one of the eight is a reasoned claim
wearing the imposed mark; I cannot tell from the registry how many others are, which is the row's
`unblocks` sentence demonstrated rather than argued. The other four carry numeric predicates on
claims about the panel, and two of them say in their own notes that the predicate is not the region:
"The predicate carries almost none of this claim's scope. Its real coordinates are one unit, eight
probe directories and five rerun events, none of which is a declared axis", and, on the bench-regions
row, "its coordinates are a named branch head, 254 files and 24 producing commits, none of which is a
declared axis." The other two borrow the numeric region of the measurement they are talking about,
which their notes say in as many words: "The predicate is the union of the two conflatable
measurements' regions", and "carried from the row it is about".

**That sentence is a class, not an instance.** `grep -n "carries almost none of this claim's scope"
mock/registry/proposal*.toml` finds six rows, four of them `measured` on `the_strategy_axis` and
`the_strategy_object` whose subject is the committed carrier table and the corpus rather than arvo,
and a seventh, `three_topics_independently_terminate_on_the_strategy_axis_as_their_shared_placeholder`,
says the same in other words: "the weakest kind of region in this file ... treat this row as a pointer
to the three sources rather than as a gate." Seven rows have hand-written, in prose, the field the
schema lacks. Say it once, in the right type: that recurring note is the missing coordinate, spelled
seven times where nothing can read it.

**The checker's four cells.** `263_probes/probe_262.rs` plants the one sentence from section 2 in
each filing the schema offers it and runs the two shipped predicate lints; `263_probes/probe_262.out`
is the run:

| cell | filing | `a-region-agrees-with-the-sentence-kind` | `every-predicate-names-a-declared-axis` |
|---|---|---|---|
| A | `argument`, no predicate | `an-established-claim-carries-no-region` | silent |
| B | `argument`, `topic: topic = the_format` | silent | `undeclared-axis` |
| C | `argument`, `threads: threads = 1` | silent | silent |
| D | `normative`, no predicate | silent | silent |
| E | control: an arvo claim, `argument`, `F = 0` | silent | silent |

The controls are A and B, which must fire, and E, which must not; the probe asserts all five and
fails if any lint changes its mind. The honest filing, A, is refused. The filing option one would
introduce, B, is refused. The two filings that pass are D, the refile that loses the warrant, and C,
a region the author does not believe, and C is the one that should worry the panel most: it is a
check quieted to pass, and the registry already holds it at least three times. Worse, the
`three_topics` row writes `operation: operation any`, which `dimension::operation` declares
inadmissible in its own `grammar`, and the checker passes it because it reads the slug side only.
The warrant-marker ruling's note already lists that violation; I add only that it was produced by
this exact pressure, a structural claim needing something, anything, on the values side.

## 7. Option three is refused by ratified canon

`ruling::a_thing_that_constrains_the_work_and_cannot_be_designed_away_is_canon`, in op's own two
sentences before the panel's inference: "A thing that constrains the work, that is needed in order
to do it, that is a law, or that cannot be avoided or designed away, is canon." The layering decides
dispatch order under his own standing order to work bottom-up; it constrains the work. The third
sentence of that row, that "predicated or not" does not decide membership, is marked as the panel's
inference rather than his words, and I lean on the first two only. Option three would move out of
the registry a thing the ratified test puts in it, and the row's own cost line concedes the registry
would then be unable to state what it rests on. Refused, on that citation.

## 8. What I would answer, and what kind of change it is

**The answer to the row: a gap, and it is in the kind, not the axes.** The intended shape holds for
imposed structural claims and `normative` is right for them. For a reasoned structural claim the
notation cannot say the true thing, which is "earned by argument, over no axis in this vocabulary,
because it is not about arvo", and the two available filings each say a false thing instead. None of
the three options names this: option one adds an axis that fails the canon's own test for an axis,
because a topic is a claim's subject rather than a situation the world can be in and a corpus
revision is a run parameter; option two is right for half the population and blind to the other
half; option three is refused by a ratified ruling. `ruling::the_option_set_is_not_a_boundary`, at
rung `stated` and carrying his words, "don't even restrict the panel to these three", is the licence
for saying so, and the `answered` field's own text asks for exactly this shape: "Where the answer
reshapes the question rather than picking an option, say so here: that is the most valuable kind."

**What the repair is, marked as a proposal for the design tier rather than as a ruling.** The type
is a product, so the honest spelling is two coordinates: the warrant `sentence_kind` already carries,
and a second saying what the sentence quantifies over, with the region demanded exactly when the
second names the design space and the first is not `definition` or `normative`. A structural claim
then files as `argument` over the canon, carries no predicate, keeps its `because` open to attack,
and the seven hand-written notes collapse into one field a lint can read. The scope it does have, the
corpus revision and the rows it read, is a run parameter and already has a home in `provenance`,
which names the seat, and the seat names what it read. I am not specifying the field's name or its
values here; that is the design's, and the canon's job is only to say that the two coordinates are
distinct, which section 5 does.

**Why this is not a canon change.** Section 0's search shows no ruling ratifies the coupling of
"regionless" to two kinds. It is schema text and a lint constant. Pulling the two coordinates apart
therefore nukes nothing above the design tier, and the append-only rule on predicates is untouched
because no existing predicate is restated: cells C become honest by dropping a region their authors
already disown, which is a correction of the kind those authors invited, and cells D become honest by
changing a kind back to what their notes say they were.

**What it costs, said here.** A second coordinate is a second field somebody fills, and the failure
it invites is an arvo claim filed as being about the canon to escape the region demand. The guard is
the one the imposition lint already uses in the other direction: a row whose `evidence` names an
instrument that swept a width is about arvo whatever its domain field says, and a lint can read that.

## 9. The predicate this answer carries, and what that tells you

The brief asks for an explicit predicate over every dimension that could move this answer, with no
vocabulary for doubt. So, honestly, over the twenty-five declared axes: none moves it. It does not
depend on width, signedness, policy, rounding, operation, arity, chain length, container, alignment,
access pattern, target features, threads, strategy, ambient domain, radix, accumulator width,
toolchain, build profile, operand window, occupancy, association, leaf aliasing or phase. What it
does depend on is three facts about the registry at `b544c82`: that every declared axis indexes a
design-space situation, that `sentence_kind` has the six values it has, and that the region demand
is keyed on two of them. It holds at that revision and at every later one that keeps those three
facts, and it is refuted the moment any of the three moves.

Under the notation as it stands that is an `argument` with an empty predicate, which is cell A, and
the shipped checker refuses it. To file this file as a row I would have to choose between cell C,
writing `threads = 1` on a sentence that has never met a thread, and cell D, calling my own reasoning
an imposition. That is the question demonstrating itself on its own answer, and it is why I say the
demand for a predicate is satisfiable here only vacuously, and the notation has no spelling for
"vacuously" that differs from "nowhere". That missing spelling is the gap, stated one more way.

## 10. What I read afterwards, and what it changed

Written after sections 1 to 9 were on disk, and the only edits to them since are two counts in
section 6 corrected against the census output, nine to eight and three to four. Read in this order:
seat 258 section 8 and its probe `258_probes/p5_the_open_row_nobody_in_this_sitting_cited.sh` with
its output; seat 261 end to end with its probe listing; seat 224 grepped for `structural` and
`normative`, which returns nothing, so 224 does not discuss this and its two rows were refiled by
somebody else; `OPTIONS.md` and `DROPLIST.md` grepped for the slug and for `structural`, where every
hit is about type-level structure in arvo and none is about this row.

**258's two claims, checked rather than inherited.** The first, that four files reported the gap
without citing the row: `p5` measured it at tree `2a2995d4`, with a control that a slug the sitting
does cite returns two files and a control that a slug existing nowhere returns zero, and found no
file in the panel citing the row while naming 240, 242, 244 at A10, 246 and 248 as reporting the gap
in prose. I re-ran the citing half at `b544c82`: `grep -rl
can_a_claim_about_the_canons_own_structure_carry_a_region` over the panel directory finds 258, its
two `p5` files, and this file. So the count stands and has grown by one: seat 261 is about this row,
is titled for it, answers it, and does not contain the slug. The second claim, that the row is
"currently blocking the filing of every process finding this panel produces", is overstated by
half. Process findings are being filed today, in cells C and D of section 6; what the row blocks is
the honest filing. Cheapest, plausibly. Blocking, only in that sense.

**261 is the second seat on this row and it was not running beside me.** It landed on `origin/dev` on
2026-09-03 as `5c69b710`, so the brief's parallel expert is at least a third read. Its own header
records that two seats took number 259 at once; the same can happen to 262, and the number carries no
meaning.

Where 261 and this file agree, each arrived at separately, in the order I had them on disk before
opening it: that the gap is in the `sentence_kind` and warrant tier rather than in the dimension
vocabulary, its sections 1 and 15 against my section 5; that option one fails, which it reaches
through the checker's own refusal sentence and the append-only ratchet and I reach through the
`dimension.toml` test of what a value indexes, two routes to one conclusion; that option two is right
for imposed rows and wrong for reasoned ones, its section 5 against mine; that option three is
forbidden, where its ground is that a workspace rule is op's words by construction so filing there
launders one expert into ratified authority, and mine is the ratified membership test, two different
citations so the refusal now has two independent instances; that the rows whose notes disown their
predicate are the residue, its F3 against my section 6; and that the checker cannot see a borrowed
region, its arms `d6` and `d8` against my cell C.

Where 261 has what I did not: it ran `an-imposition-rests-on-no-instrument`, and a structural claim
that walked something, filed `normative`, is refused the moment it cites its instrument. So option
two's cost is not "indistinguishable from an imposition" but "unciteable", which is the shape
`evidence-lives-in-the-repo-or-it-never-happened` forbids outright, and the layering row is in that
case: a walk over twenty `what` sentences with no `evidence`. I adopt that sharpening. It also
establishes that the `ruling` namespace declares no region field at all, its F1, so a claim promoted
out of `proposal` loses its region by construction whatever it is about, which bounds this whole
question to the proposal tier. And its instance 3, the additive verdict row listing no `rounding`
where its sibling lists `rounding any`, shows the inapplicable-axis class reaching a numeric claim,
which I had not seen.

**Where I disagree with 261, and this is the one finding of the reconciliation.** Its arm three,
"`argument`, and the region is the universal on every declared axis, tokenless", is not admissible
under the canon as declared. `dimension::operation`'s `grammar` says, in bold, "`operation any` is
not admissible, because `any` quantifies over a set nobody has" closed, and `dimension::strategy`'s
says "`S any` is not admissible, because it quantifies over a set op has stated is open." The
universal on all twenty-five axes is refused on two of them by the dimension rows themselves. 261's
`d3` and `d8` show it passing because the shipped checker reads the slug side and never the values
side, which 261's own section 4 and the warrant ruling's note both record, and 261's section 11 then
practises the arm on its own findings. An arm that is admissible only because the checker is blind
is cell C with more axes. 261's section 10 half-sees this, calling the universal "an unmeasured
`any`" and filing it as O5, a collision between a ratified expert row and a stated row of op's that
nothing ranks. It is narrower than that and already decided: two declared grammars refuse the
spelling outright, no ranking needed. And my section 4 says why the universal is the wrong spelling
even on the twenty-three axes that allow it: `any` on an axis asserts the claim holds regardless of
that axis, which is a statement about the design space, and a structural claim makes no statement
about the design space at all. "Vacuously" and "regardless" are different words and the notation has
only the second.

**And on the repair the two seats converge from opposite sides**, which is worth more than either
alone. 261's O2 asks "whether a row can declare that its subject instantiates no declared axis",
a one-bit field, and asks whether that bit is derivable from `topic`. My section 8 proposes a
coordinate saying what the sentence quantifies over. Same thing. On derivability I can answer from
the census: it is not derivable from `topic`, because `panel_conduct` carries a `measured` row about
the panel beside `argument` rows about it, and the four "carries almost none of this claim's scope"
rows about the corpus sit on `the_strategy_axis` and `the_strategy_object`, which are stack topics.
`topic` is subject matter; the domain is a separate fact. So it is a field, and two seats now say so
having reached it from a lint and from a type respectively.

**What did not change.** Sections 2 through 9 stand as written. 261's F4, that any single axis
pinned to an unobserved value also passes, is a stronger form of my cell C and I would have written
it had I built the wider probe; it changes nothing in the argument and everything in how urgent the
repair is.

## Standing

One expert on the derivation in sections 2 to 9, blind, and with 261 on disk the core answer, that
the gap is in the kind tier and options one and three are refused while two is half right, has two
independent arrivals. That mine predates the reading is checkable only by the shape of this file and
by my word, which is what the brief's ordering was for. The disagreement in section 10 over the
universal spelling is one seat's and wants a reply from 261 rather than a third opinion. What would
refute section 4: a structural claim carrying a genuine free variable ranging over something arvo
can be, in which case that claim has a region in the existing vocabulary and is not structural in
the sense used here. What would refute section 5: a ratified sentence keying the region demand on
the kind, which section 0's search did not find. What would refute section 7: a reading of the
canon test under which a dispatch-order derivation does not constrain the work. What would refute
the disagreement in section 10: a ratified row admitting `operation any` or `S any` over the two
dimension grammars, which I did not look for beyond `ruling.toml`.
