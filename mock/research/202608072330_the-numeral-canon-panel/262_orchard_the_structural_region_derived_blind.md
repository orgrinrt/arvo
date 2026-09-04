# 262. Orchard, seat 262: the second read seat 261 asked for, and the row it was already answered by

Base of this worktree: `b544c82c`, branch `canon/the-structural-region-q223a`.

**I was dispatched fresh on `question::can_a_claim_about_the_canons_own_structure_carry_a_region`
and told a second expert was running beside me on the same row at the same time. Neither of us was
told that the row was already answered.** Seat 261,
`261_jhala_a_structural_claim_has_a_region_and_the_checker_cannot_read_it.md`, is on `origin/dev`,
merged as pull request 125 off `research/whether-a-structural-claim-carries-a-region`, and it
answers this question at length with two instruments and eleven arms. Section 11 below is what that
cost and why nothing in the loop reported it.

I take the number 262 because 261 is the highest allocated. **The seat dispatched beside me will
almost certainly take it too.** 261's own header records the same collision happening three ways at
259 and resolves it the same way: the number is an identifier, it carries no meaning, and the later
arrival is reallocated. Every bare `261` below names the file spelled out above.

## 0. The two gates

**Canon gate: passed.** The row is open, its `decider` is `panel`, and
`ruling::the_panel_finishes_the_canon_without_him` puts every remaining canon question with the
panel rather than with op. What I checked the work against, in tier order: `mockspace.toml`'s
`canon_paths` declaration and the `proposal`, `dimension`, `topic` and `question` namespace
declarations in it; the header of `mock/registry/proposal.toml`; the header of
`mock/registry/dimension.toml` and its 25 rows; the shipped lints
`a-region-agrees-with-the-sentence-kind`, `every-predicate-names-a-declared-axis` and
`an-imposition-rests-on-no-instrument`; and the workspace rules `every-rule-is-ops-words`,
`writing-for-agents` and `never-ask-which-single-rule-governs`.

**One challenge to the dispatch rather than a refusal**, in section 11: the row's existence is
sound and its locus is right, and what is wrong is that it is still open. Answering it a third time
is not what it needs.

**Test gate: run, and it caught a machine-level failure before it caught anything about the row.**
`cargo mock --lint-only` first came back `BLOCKED: this repo's custom lints could not be built, so
no lint below them ran. Nothing was checked.` The cause was not this repository: `~/.cargo/registry`
had been emptied of both `src/` and `cache/` while the build was running, leaving only `index/`, so
`tree-sitter-0.24.7` was compiling against a source tree that vanished underneath it. A second run
refetched and the gate came back `all lints passed`. **The reason it is worth a paragraph is that
the first symptom was a C compile error naming a missing `./stack.c`**, which reads as a broken
vendored crate and sends the reader upstream, and the actual state was that the whole registry was
gone. `262_probes/p3_output.txt` carries the green baseline that every arm below is measured
against.

**And the run before that one returned exit 0 while doing nothing at all.** I wrote
`timeout 900 cargo mock --lint-only`, `timeout` is not on macOS, the shell reported `command not
found`, and `echo "EXIT: $?"` printed `0`. That is `never-discard-stderr-on-a-check` firing on its
own example inside the first two minutes of the dispatch. Had I piped that into a `grep -c` I would
have measured a clean gate over a command that never ran.

I read the bodies of the three lints named above end to end rather than their names. They are real:
`a_region_agrees_with_the_sentence_kind` tests both halves of its contract against each other,
sweeps four blank spellings of an empty predicate in one arm, carries an arm asserting that a
`ruling` is not read whatever it carries, and carries an empty-registry control. Nothing in the
three is tautological, nothing asserts a value against itself, and each reports every offending row
rather than the first. **The one thing I would say against them is in section 7**, and it is not a
defect in the tests.

## 1. The answer, before the argument for it

**None of the three options, because the question asks which single filing governs a class that is
not homogeneous.** That is the shape `never-ask-which-single-rule-governs` names, and the answer is
a partition with a filing per part rather than a winner:

- **A structural claim that imposes.** `normative`, no region, no evidence. Correct today. **The
  question's option two is right here and nothing needs to change.**
- **A structural claim that stipulates a term.** `definition`, no region, `defines` naming the
  term. Correct today for the reason a definition about numerals is.
- **A structural claim that reasons or walks something.** This is the whole of the gap, and it is
  the only part any option is about.

**And for that third part, the canon has already decided more than the question knows.** The header
of `mock/registry/proposal.toml`, which is the header of the namespace every one of these rows sits
in, says:

> `sentence_kind` is what established the claim, not what the sentence sounds like. [...] A claim
> that could be measured false is not `normative` however definitional its grammar, and it carries
> the region it was established in or it is not here at all.

That is a two-horned disposition in governing text. **It forecloses option two for any structural
claim that could be measured false**, which is every reasoned one, and it leaves exactly the two
horns the question's options one and three grope at: carry a region, or be somewhere else.

**Between the two horns I reach option one's diagnosis and refuse its vehicle.** The gap is real.
The axis it proposes, "an axis whose values are topics or namespaces", is not the axis the class
needs and fits one of the three structural coordinate systems the corpus already uses. Section 5.

**Option three is forbidden rather than costly, and I derived that before reading 261, which is the
one thing this dispatch was actually needed for.** Section 6.

## 2. What predates reading 261, and what came after

The brief required this split and it is the load-bearing part of the file, so it comes before the
argument rather than in a coverage note. My derivation was frozen to a scratch file before I opened
the panel directory for anything except an `ls`, and it is committed beside this file as
`262_probes/a_derivation_frozen_before_the_panel.md` so the split can be checked rather than
believed.

**Derived before reading any panel file except 223**, from the row, from `mockspace.toml`, from
`dimension.toml`, `proposal.toml`, `topic.toml`, and from the three lint sources:

1. The row's premise about the axis vocabulary holds, and its count of twenty-one is stale at
   twenty-five.
2. The refusal is two `HARD_ERROR` lints rather than a convention, and it fires both ways: no
   region is refused, and an undeclared axis is refused.
3. `proposal.toml`'s header already forecloses option two for anything that could be measured
   false.
4. `proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery` breaches that header by
   its own admission, being filed `normative` while its note calls it "Checkable rather than
   arguable" and gives the refutation procedure.
5. Option one's proposed axis under-fits the class, because the corpus uses at least three
   different structural coordinate systems.
6. The variable common to all of them is corpus state, and the corpus already writes that region in
   prose because no axis exists to write it on.
7. A corpus axis is not a coordinate of a numeric claim the way `threads` is, and the
   `dimension.toml` `moves` test is where the difference is stated. **With the counter to my own
   point, which I found in `proposal.toml`'s header rather than being handed:** the corpus already
   accepts exactly this severity for `threads` and `target_features` and calls the resulting
   narrowing faithful.
8. **Option three is forbidden on `every-rule-is-ops-words` and the ratification model, not merely
   costly.**
9. The corpus has already invented a workaround, visible in three rows' notes: split the imposed
   sentence from the observed one, file the imposition, exile the observation to a panel file.
10. Therefore none of the three, and the answer is a partition.
11. My own answer is an instance of the class it is about and cannot be filed as what it is.

**Reached only after reading 261**, and I say so plainly because each is 261's result and not mine:

- That four machinery rows carry a predicate today, so the question's premise is false as a matter
  of fact rather than merely stale. I had assumed the premise.
- That `an-imposition-rests-on-no-instrument` strips `evidence` from a structural row filed
  `normative`, which is a larger cost than the one option two states.
- That the `ruling` namespace declares no `predicate` at all, so the canon proper states no region
  for anything and this is a proposal-tier question throughout.
- That narrowing any single axis on a structural claim is equally acceptable to the checker, so
  there are twenty-five ways to be wrong and the checker sees none.

**On the overlap: items 1 through 11 above and 261's sections 1 through 6 agree wherever they
meet, and the agreement is worth what two blind derivations are worth and no more.** We are both
agents, we both read the same declarations, and `expert-dispatch-defends-the-canon` says two
unratified artifacts agreeing can be shared drift. What is not shared drift is item 8, because 261
named it as the one call it could not close alone and stated the exact procedure that closes it.

## 3. Breaking the row and the brief first

**The row's note says twenty-one declared dimensions and there are twenty-five.** `occupancy`,
`association`, `leaf_aliasing` and `phase` landed after it was written. Nothing in the question
turns on the count: I read all four and they are numeric or machine axes like the other
twenty-one, so the premise survives its own staleness. **What it shows is that the row's premise was
never re-checked**, and the row's list is quoted downstream. 261 records the same correction.

**The row's `unblocks` asks the right question and I can answer it with a number.** "Whether
`normative` is currently doing duty for a third thing nobody named." Across the registry: 70
`normative` against 12 `argument`, 33 `measured`, 8 `theorem`, 3 `enumeration` and 1 `definition`.
On the four machinery topics specifically, twelve rows: eight `normative` with no region, four
carrying one. **Three of the eight say in their own notes that they were filed `normative` because
nothing else was available**, and one of the three says where the observation went instead:
`an_instrument_is_mutated_and_the_battery_is_made_to_notice` records that "the four instances are
enumerated in `182` as a finding with no region over any declared axis." So the answer to the
`unblocks` is yes, measurably, and the corpus has been paying for it by exiling the checkable half
of each row into prose.

**Seat 258's two claims about this row, checked rather than inherited.**

*"Four files of this panel have now reported it without citing it."* Under the pattern I used, which
is the phrases those rows actually write, I find three panel files reporting the gap without citing
the row: `182`, `229` and `261`. `261` postdates `258` and cannot have been in its count, so two
predate it. **I cannot reproduce four and `258` does not state its pattern**, so this is a
disagreement about instruments rather than a finding that `258` was wrong. The control is that
`258` itself does cite the row, and the same search finds it.

*"The cheapest unblocking act available on this whole subject."* **True when written and false now**,
because the act was performed by 261 and the row was not updated to say so. That is section 11.

**One thing in the brief I could not use.** It named seat 258's section 8 as reporting that
answering this row blocks the filing of every process finding the panel produces. That is 258's
reading and it is a reasonable one, but the blocking is not mechanical: nothing in the gate stops a
process finding being filed as `normative`. What is blocked is filing one *honestly*, which is a
different and smaller claim than the brief carries.

## 4. The canon clause nobody has applied here

`262_probes/p1_the_header_clause_nobody_cites.sh`, two controls, both passing on the second run.

**The clause is in `mock/registry/proposal.toml`'s header and exactly one panel file quotes it**,
`237_the_format_proposals_against_the_ratification_gate.md`. Seat 261 quotes it zero times, measured
directly.

**And 237 uses it as a live test to reclassify a row**, on a numeric proposal about law families:
"'All four combinations occur' could be measured false. It was measured true, by
`56_probes/q1_two_law_families.rs`". So the move is not one I am inventing. It is an established
move in this panel, applied to a numeric row by a different seat, and **nobody has applied it to the
structural class.**

Applied there it does the following, and this is the argument of the file in one step:

- `proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery` is filed `normative`.
- Its own note says "Checkable rather than arguable: take an edge, read the depending topic's
  unanswered questions, and see whether any turns on the other's subject."
- Checkable is could-be-measured-false.
- The header says a claim that could be measured false is not `normative`.

**So the row raising this question is in breach of the header of the namespace it sits in, by its
own words, and no lint can see it.** That is not my judgement about how it should have been filed.
It is the row and the header disagreeing in the same repository, and the probe prints both.

The same reading catches `every_dispute_in_the_number_system_topic_was_a_dispute_about_an_address`,
whose `because` says it "is offered as a composition of four earlier sentences [...] rather than as
a new claim" and whose note says "if any of the four sentences it composes falls, it falls with
them". A row that falls when its premises fall is a row that could be measured false. It is
`normative`.

## 5. The three options, weighed

### Option one: the diagnosis is right and the axis is wrong

**Right that it is a gap.** The header quoted above says a reasoned structural claim "carries the
region it was established in", and `every-predicate-names-a-declared-axis` refuses every spelling of
that region. Two pieces of canon in the same repository, one demanding a thing and one refusing it.

**Wrong in the axis it proposes, and I refuse it on a different ground than 261 does.** 261 refuses
option one on permanence: `the-axis-set-is-append-only` gates at every commit, so a wrong axis
cannot be taken back, and declaring one makes every committed numeric predicate assert that it holds
in no situation involving a registry. That argument is sound and I do not repeat it.

**My ground is that the option's axis does not fit the class it is for.** The option says "an axis
whose values are topics or namespaces". The corpus's structural claims quantify over at least three
different things:

- `the_topics_form_a_stack...`: **topics**, and the edges between them.
- `every_canon_sentence_names_the_prefix_it_quantifies_over`: **depth in a construction sequence**,
  the identity, the identity with its reduction, the encoding. Its note says so: "The structural
  coordinates it quantifies over are not `dimension` rows, so no predicate in this registry can
  express a depth."
- `an_instrument_is_mutated_and_the_battery_is_made_to_notice`: **panel units and the instruments
  in them**, four failures in one unit.

An axis over topics expresses the first and neither of the others. **So option one as worded does
not close the gap it correctly identifies**, and an axis that did would have to range over corpus
state generally, which is where 261's permanence objection bites hardest.

**Where option one is right: nowhere I found, and I looked for the region rather than assuming
there was none**, which `every-finding-carries-its-predicate` requires before writing a prohibition.

### Option two: right for the imposed part, foreclosed for the rest

**Right, and no change wanted**, for the rows that impose:
`naming_is_partial_and_injective_or_it_is_broken`,
`a_transfer_argument_is_a_construction_warrant_and_needs_no_new_rule`,
`const_availability_is_the_axis_and_a_trajectory_condition_is_not_an_arm`. These establish nothing.
`normative` is what they are and the absence of a region is correct rather than missing, exactly as
the option says. The row's note is right that this is not a concession.

**Foreclosed for the rest by the header clause**, which is the contribution of section 4, and
compounded by 261's finding that `an-imposition-rests-on-no-instrument` then refuses the row's
`evidence`. Taken together: the filing is refused by the header for being wrong about provenance,
and it strips the instrument on the way. **Two independent refusals of the same manoeuvre, one
textual and one mechanical, and neither seat had both.**

### Option three: forbidden, and this is the second read 261 asked for

Section 6, because it is the reason the dispatch was worth running.

## 6. The second read seat 261 asked for, and it closes

261's section 12 lists what it opened, and O1 is this:

> **O1. Whether option three of the question is forbidden or merely costly.** My reading is
> forbidden, on `every-rule-is-ops-words` and `writing-for-agents`. **Closed by a second expert
> forming the same reading from those two files before reading this one.** It does not go to a third
> if we disagree.

**That is the procedure I happened to follow, and the freeze file is the evidence I followed it.**
Item 8 of my frozen derivation, written before I opened the panel directory:

> Option 3 refused on provenance. Workspace rules are op's words by `every-rule-is-ops-words`;
> relocating an unratified one-expert panel derivation there promotes it to op's voice by filing.
> And op is out of the track by `ruling::the_panel_finishes_the_canon_without_him`, so nothing could
> ratify it there.

**Same reading, same first ground, reached blind.** `every-rule-is-ops-words.md` opens "All rules
here are op's words, straight from the lead designer", and `writing-for-agents.md` supplies the
mechanism: "nothing in a rule is attributed, quoted or sourced [...] The file existing is the
ratification, and a rule is read as his words." A structural claim carrying `standing =
"one_expert"` moved into `.claude/rules/` acquires ratified authority in the act of being filed, and
the filing is the only thing that changed. **That is provenance forgery, and it is worse than the
filing problem option three sets out to fix.**

**I reached one ground 261 did not state in that section.** Option three does not merely risk
laundering the claim, it leaves nothing able to ratify it: the panel finishes the canon without op
by a ratified ruling, so a claim moved into a surface whose whole ratification mechanism is his
authorship is a claim that can never be stamped by anybody. The destination has no route to
legitimacy for this class of claim at all.

**261 also states a second ground I agree with and did not derive**, that the registry already holds
structural claims at the ruling tier and several are ratified, so option three would move
`the_panel_finishes_the_canon_without_him` itself. I record that as inherited rather than
corroborated.

**So O1 closes: option three is forbidden.** Two experts, each grounding it in quoted canon, the
second having formed the reading before seeing the first, which is what
`expert-dispatch-defends-the-canon` requires and what it says agreement reached the other way round
is not. The row's third option should be recorded as refused rather than as an option carrying a
cost.

## 7. The residue 261 left, closed in the negative

261's section 12 also opens this:

> **O2. Whether a row can declare that its subject instantiates no declared axis.** A one-bit field
> would make arms three and four checkable and is not a new dimension. **Closed by somebody deriving
> whether that bit is derivable from `topic` instead, which would make it free.**

`262_probes/p2_the_bit_is_not_derivable_from_topic.sh`, two controls, both passing.

**Derivable would mean: every row on a machinery topic has a structural subject, and no row on a
stack or frame topic does.** One counterexample in the second direction refutes it.

**There are two, and both are on `the_number_system`, a stack topic:**

- `every_dispute_in_the_number_system_topic_was_a_dispute_about_an_address`, whose `says` opens
  "Every question this unit disputed was a dispute about an address". That quantifies over the
  panel's own disputes. Its subject is the corpus.
- `the_concept_is_closed_and_the_inventory_is_open`, whose `says` is "The canon defines once what a
  number system is and what admission requires [...] rather than by amending the canon". That is a
  claim about how the canon is organised.

**So the bit is not derivable from `topic` and O2's cheap route is closed.** If the bit is wanted it
has to be declared, which is a schema change with a ratification behind it, and that is a different
and more expensive proposition than 261 could price.

**One refinement to 261's F2 and F3 while the instrument was pointed there.** All four
region-bearing machinery rows are on one topic, `panel_conduct`, and none is on `canon_form`,
`the_predicate_notation` or `naming`. 261 reports the count of four and not their concentration.
That matters for anybody attacking the borrowed-region finding, because it says the four are one
seat's habit on one subject rather than a pattern spread across the machinery.

## 8. Independent reproduction of the decisive split

`262_probes/p3_the_gate_on_a_structural_argument.sh`, six arms, three of them controls, run through
`cargo mock --lint-only` over the real registry with the tree asserted clean afterwards. Third
instance of what 261 measured with two instruments, by a different author from a different base.

| arm | filing | verdict |
|---|---|---|
| `a_control_baseline` | nothing planted | `all lints passed` |
| `b_argument_no_region` | `argument`, no predicate | refused, `an-established-claim-carries-no-region` |
| `d_normative_no_region` | `normative`, no predicate | accepted, silent |
| `c_control_must_fire` | `normative` carrying a predicate | refused, `an-imposed-proposition-carries-a-region` |
| `e_argument_corpus_axis` | `argument`, `corpus_state: ...` | refused, `undeclared-axis` |
| `f_control_argument_declared_axis` | `argument`, `fraction_width: F = 0` | accepted, silent |

**Agrees with 261's table on every arm they share.** The controls are what make it worth anything:
`a` establishes the registry is green before each arm, `c` establishes the instrument reads planted
rows at all, and `f` establishes that `argument` as a kind is accepted, so `b` and `e` are refusals
of the structural claim rather than of the kind.

**One honest defect in the probe.** Every planted arm also drew `a-row-carries-keywords`, because my
minimal row omits `keywords`. It fires identically on all five planted arms including both controls,
so it separates nothing and no verdict above turns on it, but a cleaner probe would have carried the
field and the reader should not have to work that out.

## 9. My own predicate, and what writing it demonstrates

The brief asked whether my answer can carry a region under the notation as it stands. **It cannot,
and the attempt is the demonstration rather than a flourish.**

My answer is a reasoned claim about the canon's own contents. It is not proved, no instrument
produced its central claim, it stipulates nothing and it imposes nothing. Under the declared
vocabulary it is `argument`. `argument` owes a region. Every one of the twenty-five declared axes is
absent from it, because none of them moves its truth. `a-region-agrees-with-the-sentence-kind`
therefore refuses it, and arm `b` above is that refusal executed on the nearest available
substitute.

**The predicate I can honestly write, and it is a structural one in 261's arm-three sense:**

*Holds for: this repository at `b544c82c`; the 25 `dimension` rows, the six `sentence_kind` values
and the 127 `proposal` rows committed at that revision; the three lints at the revision
`mock/target/mockspace-lints/Cargo.toml` pins; the workspace rules `every-rule-is-ops-words` and
`writing-for-agents` as they stand in this worktree. No declared axis bears on any of it. threads
absent, target features absent, toolchain absent.*

**What the last sentence costs under the notation is exactly the finding.** An absent axis says the
claim holds in no situation where that axis exists. Threads exist wherever anything runs, so my
answer holds nowhere anything runs. That reading is technically correct and completely
uninformative, because **the notation cannot distinguish a claim that is false everywhere from a
claim that is about something else.** Both come out as absent on every axis. The three states per
axis are only distinguishable within one space of situations, and this claim is not in that space.

**That is the sharpest statement of the gap I can make and it is not an argument, it is what
happened when I tried to comply.**

## 10. What I could not establish

**Whether the third part of the partition should be a new `sentence_kind` or a warrant clause on
the existing one.** My derivation reached "keyed to the kind rather than to the axis set" and I
could not get further, because both routes need a schema change and I have no way to price which is
cheaper against the checks that read `sentence_kind`. 261's arm four answers it differently, as
`measured` or `enumeration` plus a universal region plus prose coordinates, and **that answer works
today and mine does not**, which is a real point against mine and I am not going to dress it up.
What mine has that 261's does not is the header clause forcing the issue rather than leaving the
current filing tolerable.

**Whether the universal-on-all-25-axes spelling is honest.** 261 leaves this open as O5, `230`
opened it before that, and I add nothing. I formed no view worth recording, having spent the budget
elsewhere.

**Whether the eight `normative` machinery rows should be corrected now.** I have named the three
that are correctly filed and the specimen that is not. `every-finding-carries-its-predicate` says a
predicate is never widened in place and the correction goes in the later expert's deliverable, which
this is. Whether it reaches those rows is the coordinator's call and not mine.

**Whether my disagreement with 258's count of four is a disagreement at all.** I could not
reconstruct its pattern and it does not state one.

## 11. The finding that cost the most, and it was not about the row

**This row was answered by seat 261, on `origin/dev`, before I was dispatched, and the row does not
say so.**

The `question` namespace declares an `answered` field, and its own declaration says exactly what its
absence causes:

> Written when it is answered and absent otherwise, which is what lets a roster tell a live question
> from a settled one sitting in the queue: six were doing exactly that, four saying so only in prose
> and two saying nothing, and two of those would have gone back to op in the next batch.

**This is a seventh, and it did not go back to op, it went back to two fresh experts at once.** The
row carries no `answered`. Nothing in the registry points at it except one `probe` row's note. So a
coordinator reading the open list sees a live question with a `decider` of `panel`, which is exactly
what it looks like, and dispatches it.

**The brief I was given anticipated this failure and did not prevent it.** It told me a dispatch here
had been issued five times over a deliverable that already existed because nobody ran `ls`, and told
me to run `ls` before briefing myself into the panel directory. I ran it and found 261 immediately.
**The `ls` catches the collision at the cost of a whole dispatch's context, which is after the money
is spent.** What would have caught it beforehand is the row saying it was answered, and the field
for that exists and is empty.

**So the cheapest unblocking act available on this subject is now writing `answered` on the row**,
not answering it. That takes one edit and it is not mine to make: 261 is the answer, its author is
not here, and what goes in the field is "what settled the question, in the answerer's own words".

**What was not wasted.** 261 asked for a second expert to close O1 by exactly the procedure I
followed by accident, so the second dispatch was owed even though the question was not. **The right
brief was "close 261's O1 blind", which is a fraction of this one and would have cost a fraction of
it.** I would rather say that than pretend the whole dispatch was necessary.

**And the seat running beside me is doing all of this again right now.** By the time it returns
there will be three answers to a question that had one, and two of the three will have been paid for
because a field was left blank.

## 12. Findings, with what carries each

Every finding here is structural. Its coordinates are this repository at `b544c82c`, the files
named, and the lints at the revision the launcher pins. **No declared dimension bears on any of
them**, and I have written the true coordinates rather than the empty universal, which is section 9
practised rather than described.

**F1.** `mock/registry/proposal.toml`'s header states that a claim which could be measured false is
not `normative` and carries the region it was established in or is not in the registry at all. One
panel file quotes it, `237`, on a numeric row. `261` quotes it zero times. *holds for: this
repository at `b544c82c`; `262_probes/p1_output.txt`, two controls passing.*

**F2.** `proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery` is filed
`normative` while its own note calls it checkable, which the header of its own namespace says
disqualifies it from that filing. No lint can see the contradiction. *Same coordinates as F1.*

**F3.** The bit "this row's subject instantiates no declared axis" is not derivable from `topic`.
Two rows on `the_number_system`, a stack topic, have structural subjects. *holds for: the 127
`proposal` rows at `b544c82c`; `262_probes/p2_output.txt`, two controls passing.*

**F4.** All four region-bearing machinery rows are on `panel_conduct` and none is on the other three
machinery topics. *Same coordinates as F3.*

**F5.** On the real gate, a structural claim filed `argument` with no region is refused, with an
undeclared corpus axis is refused, and filed `normative` is accepted silently, while an `argument`
over a declared numeric axis is accepted. *holds for: the three lints at the pinned revision; the
registry at `b544c82c`; `262_probes/p3_output.txt`, three controls passing.*

**F6.** The question row carries no `answered` field and nothing in the registry answers it, while
`261` on `origin/dev` does. *holds for: `mock/registry/question.toml` at `b544c82c`; the grep in
section 11 with `the_panel_finishes_the_canon_without_him` as its positive control.*

**F7.** Under the phrases the rows themselves write, three panel files report this gap without
citing the row and two of them predate `258`. `258` says four and states no pattern. *holds for: the
panel directory at `b544c82c`; the search in section 3, control being that `258` itself is found.*

## 13. Options this opens, and what closes each

**O1 of `261` is closed by section 6.** Option three is forbidden. Two experts, quoted canon,
second derived blind. It does not go to a third.

**O2 of `261` is closed in the negative by section 7.** The bit is not derivable from `topic`. What
remains open is whether it is worth declaring, which is a schema question with a cost I did not
price.

**N1. Whether the header clause obliges a correction to the eight machinery `normative` rows now.**
My reading is that it obliges it for the ones that could be measured false, which is at least the
specimen and `every_dispute_in_the_number_system_topic_was_a_dispute_about_an_address`. **Closed by
a second expert reading the header clause against those rows.** I am the first read on applying it
to this class.

**N2. Whether the row should be marked `answered` naming `261`, or answered again.** My reading is
the first. **Closed by the coordinator, since it is a filing act rather than a question.**

**N3. Whether the four `panel_conduct` rows' borrowed regions should be corrected or retired.**
Untouched by me; 261 states the correction and leaves the reach to the coordinator, and I agree.

## 14. Coverage, bounded

**Read in full:** the question row raw; `223_checkpoint_the_topic_layering.md`; `261` sections 0
through 6 and 10 through 13; the three lint sources including every test body;
`mock/registry/dimension.toml`'s header and the rows for `strategy`, `ambient_domain`, `radix`,
`accumulator_width`, `toolchain`, `build_profile`, `operand_window`, `occupancy`, `association`,
`leaf_aliasing` and `phase`; `mock/registry/proposal.toml`'s header; the `proposal`, `dimension`,
`topic` and `question` namespace declarations in `mockspace.toml`; eight `proposal` rows raw.

**Read partially:** `258` section 8 and its coverage note; `237` around its use of the header
clause, twenty lines, and nothing else of it.

**Not read at all, and it bounds everything above:** `261` sections 7, 8, 9 and 14; every other file
in this panel directory, which is 482 entries; `229` and `230`, which both seats before me cite as
having reached parts of this first, so where I claim novelty against `261` I am not claiming it
against those two; the shipped source outside the lints; every `law` row.

**What I did not verify:** that `261`'s eleven-arm instrument says what its file reports, beyond the
four arms my own probe reproduces. I opened its probe directory listing and not its sources.

## 15. The one sentence

**The question asks which single filing governs a class that splits three ways, the canon's own
namespace header already forecloses the option it was leaning toward, the option to move the class
out is forbidden and that is now settled by two blind reads, and the row was answered a dispatch ago
by somebody who wrote it all down and could not mark the field that would have told anybody.**
