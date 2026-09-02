# 249. Jhala, seat 249: `standing` counts arrivals, and an argument for a handed conclusion is not one

Blind derivation. Everything from here to section 11 was written and committed
before I opened any file numbered 244 through 249 or read seat 248's parallel
answer. The blind commit's hash is recorded in section 11, which is the only
section written afterwards.

Base of this worktree: `eac588fd2dfd157faaf088ae69c6342227ac2c98`.

## 0. The two gates

**Canon gate: passed.** The question is answerable from the canon and the canon
does not forbid asking it. What I checked it against, in tier order: the ratified
rulings `two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`,
`an_ack_is_not_a_ratification`, `the_panel_finishes_the_canon_without_him` and
`behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`; the stated
rulings `the_canon_is_written_once_at_the_end` and
`the_canon_candidate_is_formalised_and_established_together`; the declared field
vocabulary in `mockspace.toml`; and the 127 committed `proposal` rows.

I do have one locus correction to the brief, in section 2. It does not rise to a
refusal, because the question I was asked is a real question with a real answer
and the correction is about what the answer is then used for.

**Test gate: run.** Two instruments, in `249_probes/`, each with the case that
had to fail run first and shown failing. `p2`'s third control caught a defect in
my own control fixture on its first run: a planted file meant to have no
reconciliation heading had the word in its title, so the splitter matched the
title and reported a split where none should exist. The fixture was reworded and
a fourth arm added that asks the same question of the real tree, where the answer
is zero files. That arm is the one that makes the ten measurements in section 8
mean anything, and it did not exist until a control failed.

## 1. The answer, stated once before the argument for it

`standing` counts neither instances of the claim nor arguments for it. It counts
**arrivals**: distinct authors who reached the content by a route that was not
supplied to them. That is not my coinage. It is what the field's own declared
vocabulary says, at `mockspace.toml`, `proposal` namespace, field `standing`:

> How many independent instances reached it, over the region they share.
> `one_expert`; `two_experts`, each deriving before reading the other;
> `three_or_more`; `cross_topic` where separate topics arrived at it without
> citing each other, which is the strongest thing this panel produces;
> `contested` where it is stated because somebody stated it and somebody else
> disagrees.

Three clauses in that do the whole job. "reached it" rather than "states it" or
"argues for it". "each deriving before reading the other". "without citing each
other".

So the seat in the question does not count. It read the conclusion in the row it
was dispatched on, and then derived. That is deriving after reading the other,
which is the exact negation of the condition `two_experts` names. Its file may be
excellent, its route may be new, and none of that is what `standing` measures.

What the file does count toward is a different and independently necessary gate,
which the canon also declares and which is where a new route genuinely belongs.
Sections 5 and 6 are that half, and it is the half that makes this answer a
composition rather than a refusal.

## 2. Breaking the brief first, which is the first task

Three things in the brief are wrong or off-locus. None of them changes the
answer; the third changes what the answer is for.

**The value list flattens three disjoint vocabularies into one.** The brief says
"the values in use across `mock/registry/*.toml` are `one_expert`, `two_experts`,
`three_or_more`, `sound`, `cross_topic`, `prior_attempt`, `uncontrolled`,
`defective` and `withdrawn`". True as a union over files and false as a
description of a field. `standing` is declared three times in `mockspace.toml`,
once per namespace, with three disjoint value sets:

| namespace | declared values | rows carrying each |
|---|---|---|
| `proposal` | `one_expert`, `two_experts`, `three_or_more`, `cross_topic`, `contested` | 74 / 23 / 6 / 6 / 0 |
| `probe` | `sound`, `uncontrolled`, `defective`, `withdrawn` | 95 / 8 / 2 / 1 |
| `strategy` | `prior_attempt`, `op_stated`, `proposed` | 4 / 0 / 0 |

A probe's `sound` is a statement about whether a control fired. A strategy's
`prior_attempt` is a statement about where a name came from. Neither is a count
of anything and neither is in the question's scope. Reading them as one
vocabulary is how somebody ends up asking whether `sound` is count-shaped.

**`cross_topic` is count-shaped, and the brief says it is not.** The brief lists
the count-shaped values as `one_expert`, `two_experts`, `three_or_more`. The
enforcing lint disagrees by name, at
`mock/lints/a_standing_is_reachable_from_what_it_cites.rs`:

```rust
/// The standings that assert more than one independent arrival.
const MULTI_ARRIVAL: [&str; 3] = ["two_experts", "three_or_more", "cross_topic"];
```

And the declared vocabulary agrees: `cross_topic` is where "separate topics
arrived at it without citing each other". That is a count of arrivals with an
extra independence condition on top, which makes it the strongest count rather
than a non-count. One committed row says so in its own words: "`cross_topic`
rather than `two_experts` because three separately dispatched topics computed it
independently and none cites the prior two, which is the strongest standing this
panel produces."

This matters for the question rather than being pedantry. `cross_topic` is the
value whose definition states the disqualifying condition outright, so it is the
value the answer is easiest to read off, and the brief's list omits it.

**The locus correction, and this is the one worth arguing about.** The brief says
what the answer changes is "whether certain proposal rows are promotable". That
reads promotability off `standing`, and the canon does not work that way. Two
declared fields sit between `standing` and promotion, and both say so:

- `gate`, on `proposal`: "The bar and the gate are two different things:
  convergence says several experts reached it, and the gate asks whether the
  reasoning, the evidence and the stated region are enough for canon."
- `promotion`, on `ruling`: "Convergence between two experts is the bar, not the
  trigger ... this field carries that judgement and can be argued with, which a
  bare count of agreeing experts cannot."

And the ratified ruling behind both, `two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`,
in its note: "It is not automatic promotion on a count of agreeing experts ... A
promotion carries the reasoning that justified it, so it can be argued with
rather than merely counted."

I measured what that means in the committed corpus and the answer is stronger
than I expected. Eleven proposals are named by a `ratifies` edge. **Four of them
carry `standing = "one_expert"`** and are ratified canon anyway
(`observability_is_relative_to_a_declared_signature`,
`every_operation_arvo_declares_is_a_function_of_the_declared_width`,
`the_carrier_is_observable_through_the_ambient_layout_observation_alone`,
`at_shared_occupancy_no_per_element_footprint_observation_exists`, all ratified
by `behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`).

So `standing` is not necessary for promotion in practice, and it is not
sufficient either. Inflating or deflating it therefore does not directly inflate
or deflate promotability, which is the brief's stated stake. What it does instead
is worse and quieter, and section 9 is about that.

## 3. Why "instances of the claim" is the wrong reading, with the corpus's own counterexamples

If `standing` counted instances of the claim, a second author restating a claim
they read would increment it. The corpus refuses that in four places, each of
which is a committed row's own note rather than my reading of one.

- The conceding half of `a_format_is_identified_by_its_ambient_domain_and_its_representable_set`:
  "converged by attack and concession rather than independently derived, which
  the source marks explicitly as weaker than two experts **because the conceding
  file read rather than derived**."
- "On top of those sit one formalisation and one second read, which are not
  further independent derivations of the same principle."
- "Their regions do not intersect in a way either could carry alone, so the tier
  here is the sweep's and the second arrival is recorded rather than counted."
- "The two experts behind the two axes did not independently reach one claim, so
  this is not a convergence and is not written as one."

Four separate rows, written for four separate topics, all applying the same rule:
agreement reached by reading is not an instance. The last of them adds the axis
condition, which is the "over the region they share" clause of the declared
vocabulary doing work: two authors who agree about different things have not
converged about anything.

## 4. Why "arguments for it" is also the wrong reading

If `standing` counted independent arguments, then one author's two instruments
would be two, and two authors sharing one argument would be one. The corpus
refuses both directions, again in its own words.

- One author, two instruments, counted as one: "One expert with two instruments,
  which is **one author's two arrivals rather than two authors'**." The row's
  `standing` is `one_expert`. Route multiplicity does not increment.
- Two authors, one shared argument, counted as two: "The standing carries a
  discount its own authors stated: both derivations draw on one
  numerical-analysis literature, so this is two independent instruments over one
  shared premise rather than two arrivals from nothing." The row's `standing` is
  `two_experts`, with the discount written into the note rather than into the
  field. Argument-sharing does not decrement.

So argument-independence is neither necessary nor sufficient. Author-independence
of arrival is what moves the field, and premise-sharing is a discount recorded in
prose because, in that row's own words, "there is no tier between the two that
the schema can express".

That sentence is the schema admitting its own resolution limit, and it is
directly the shape our question needs. A seat that argues to a handed conclusion
is worth more than a bare read and less than an arrival, and there is no value
for that either.

## 5. What the seat's file does count toward, because the answer is not "nothing"

The ratified promotion bar has two limbs, and the second is the one a new route
discharges. From `two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`,
op's own words in the `quote`: "if two separate experts agree on it heavily **and
can reason and spell it out**, that is a ratification for canon".

Agreement is the first limb. Reasoning spelled out is the second. A seat handed a
conclusion cannot supply the first and is often the best available supplier of the
second, because it is not spending its budget on rediscovering the destination.

So the composition is:

- The seat's file does not touch `standing`.
- It goes into the row's `because`, which is where the reasoning lives, and into
  the row's `provenance`, which is where a reader is sent to check it.
- If it plugs holes or bounds the region, it is doing the job
  `the_canon_candidate_is_formalised_and_established_together` names, which that
  ruling separates from attacking in terms: "the formalising half is a separable
  instruction and is a different job from attacking".
- If the coordinator promotes on the strength of it, the reasoning goes in the
  ruling's `promotion` field, which is the field declared to carry exactly that
  and to be arguable.

That last one is not a proposal. It is what the canon already did, once, in the
one worked case it holds.

## 6. The arms, with their predicates

The question asked whether a handed conclusion disqualifies. It is not one
question, because contamination is per content rather than per file. The canon's
own worked case proves that, and it is where these arms come from.

`behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` is
ratified by experts, and its `promotion` field records a contamination of exactly
the shape in the question, declared rather than hidden:

> One contamination, declared by seat 225 and recorded here rather than filed
> away. Its brief withheld the four proposals by naming their slugs, and those
> slugs are descriptive sentences, so withholding them by name signposted the
> destination. Its route and its instruments are its own and were committed
> first. What that cost is unmeasurable and what it did not reach is the
> finding: no signpost suggests dissolving the binary, since all four slugs read
> as claims on one branch, and the dissolution is what this ruling turns on.

That is a seat handed its destination, counted as the second independent
instance, on a stated ground: the part of its result the signpost could not have
supplied. So the rule is not "handed conclusion, therefore nothing". It is a
partition, and here are the pieces.

**Arm A. The handed conclusion itself. Does not count.**
Predicate: the row, brief or ledger stated the conclusion C; the seat read it
before its route to C was fixed; the seat reached C. Then the seat is not an
arrival at C and `standing` on the row stating C does not move. The file is
recorded in `note` and cited in `provenance`, never counted. Grounded in the
declared vocabulary's "each deriving before reading the other".

**Arm B. Content the handover could not have signposted. Counts.**
Predicate: the seat reached C', C' is not derivable from the handed text, the
seat states why not, and the seat's own route to C' is committed. Then it is an
arrival at C'. Grounded in the ratified promotion text above: the dissolution was
counted precisely because no slug signposted it. The burden is on the seat to say
what its exposure was and what that exposure could not have produced, and it is a
burden a seat can discharge, because seat 225 discharged it.

**Arm C. Disagreement with the handed conclusion. Counts, and is worth more than
agreement.**
Predicate: the seat's route contradicted part of what it was handed. Then it is
an arrival, on stronger evidence than an agreeing arrival, because a route
steered by its destination does not produce disagreement with it. Grounded in a
committed row's own note: "Two experts and the second was a blind prediction that
refuted the first's constant, which is a stronger shape than agreement."

This is the arm I would press hardest. Disagreement is self-authenticating and
agreement is not. A seat that argues to the handed conclusion and disagrees with
it nowhere has produced the one output indistinguishable from having been told.

**Arm D. A blind commit preceding the read. Counts for everything inside it.**
Predicate: the seat's phase-one commit precedes, in git, the commit in which it
read the stating row, and a later reader can check the ordering. Then everything
in the blind commit is an arrival regardless of what the later reading contained.
Grounded in the same ratified `promotion` field: "its phase-one commit precedes
the commit in which it read the prior work, so the ordering is checkable rather
than asserted."

The word doing the work there is **checkable**. Not asserted, not described in a
note, not attested by the seat. Two commits in an order anybody can read.

**Arm E. The same author twice. Never counts, at any number of routes.**
Predicate: the second route is by the same author or persona as the first. Then
`standing` does not move however many instruments were built. Grounded in the
committed note "one author's two arrivals rather than two authors'", and enforced
in practice once: the same ratified `promotion` records that "seat 221, the same
persona, declined to raise them and said in terms that a non-Dolan second read
was owed". A seat refusing to second itself is the rule working.

**The composition.** A single file is usually several of these at once. Seat 225
is Arm A on the four slugs it was handed, Arm B on the dissolution, and Arm D on
everything in its phase-one commit. The honest treatment is per claim, not per
file, and the canon's one worked case does exactly that.

## 7. The test that follows, stated so it can be gated on

Reading the arms as a predicate a checker could hold:

> A file F is an independent arrival at claim C exactly when, before F's
> phase-one commit, F cited no artifact whose content includes C.

Artifacts whose content includes C: the other establishing file; a consolidation
compressing it; a `proposal` row stating it; a `question` row whose `note`,
`options` or `answered` states it; and the brief that dispatched F.

Two consequences, and the second is the hole.

The first is that reading a **premise** row in the blind half is fine and is the
point of having a registry at all. Seat 225's own blind half opens on "What the
canon already forces", and that is the panel working rather than failing. The
line is between rows that are upstream of C and rows that are C, and the declared
vocabulary draws it in the phrase "reading **the other**", where the other is the
other instance of this claim and not any other row.

The second is that a brief is not committed. It is the widest-bandwidth channel
into a seat and it leaves no artifact in the repository, so the predicate above is
uncheckable on its most dangerous term. `evidence-lives-in-the-repo-or-it-never-happened`
governs a seat's outputs; nothing governs its inputs. The only repair available
without new machinery is the one seat 225 already used: the file states its own
exposure, under a heading, and the standing rests on that statement. Its heading
is `### Standing, stated for the coordinator's gate`. That is a precedent rather
than an invention, and I would make it the requirement.

## 8. What I measured

Instruments and outputs are in `249_probes/`, committed with this file. Predicates
below name every axis I varied. An axis not listed is one the finding does not
hold across at all.

**M1. `standing` against the number of distinct files its provenance names.**
Predicate: `namespace = proposal`, `registry = mock/registry/proposal.toml` and
`proposal-the-later-topics.toml`, `commit = eac588fd`, `panel = the numeral canon panel`.

127 rows. 29 carry a multi-arrival standing while citing exactly one distinct
file. That is a reimplementation from scratch landing on the number the lint
carries as its measured ceiling, `const CEILING: usize = 29`, by a different
route: the lint walks a `RegistryView` through the engine, mine parses the TOML
with awk. Two instruments, one number.

**M2. The proposals a `ratifies` edge names, and their standing.**
Same predicate. Eleven rows. Ten of the eleven rest on a single cited file. Four
carry `one_expert`, four `two_experts`, one `three_or_more`, one `cross_topic`,
and one `two_experts` with three files. So five expert-ratified canon rows assert
several arrivals from a single cited file, and four more were ratified while
asserting one arrival.

Extraction control: I built the ratified set twice by different methods, once by
scanning `ratifies` blocks and once by intersecting every long quoted token in
`ruling.toml` with the set of real proposal ids. The first over-ran into prose on
a multi-line block and I discarded it; the second is in the probe. They agree on
the same eleven, which is why I trust the list rather than the first reader.

**M3. Registry-slug citations in panel member files, split at file 189.**
Predicate: `panel = the numeral canon panel`, `commit = eac588fd`, files 244
through 249 excluded because I had not read them. Files numbered below 189: 191
files, 3 cite the registry, 5 citation lines. Files numbered 189 and above: 55
files, 42 cite the registry, 835 citation lines.

The registry became the panel's input. That is the structural fact underneath the
question: before it existed, a seat could only read peers, and reading a peer is
visible in the peer's own numbering. Now a seat can read a compression of a peer
and cite a slug, and the compression carries the conclusion without the route.

**M4. Registry citations inside blind halves.**
Same predicate, restricted to files numbered 189 and above carrying a
reconciliation heading. Ten such files. All ten cite the registry before their
reconciliation heading, 161 citations in total. And 45 of the 55 post-189 files
carry no reconciliation heading at all, so the blind protocol is the minority
mode in the later panel.

Bound on M4, stated because the number overstates what I want it to: some of
those citations sit in a file's opening gate or brief-correction sections rather
than inside its derivation, and I did not separate them. So 161 is an upper bound
on derivation-time exposure. The instance that needs no such bound is
`232_lamport_the_nine_rounding_entries_derived_cold.md`, whose reconciliation
begins at line 598 and which carries, at line 441, inside the blind half, the
heading `## 9. The two question rows, answered`, answering
`question::is_the_rounding_vocabulary_complete_at_six` and
`question::what_region_does_a_predicate_naming_no_mode_state` by slug. A file
whose title says derived cold, answering registry rows in its cold half.

**M5. `question.toml`'s header against its own rows.**
Predicate: `file = mock/registry/question.toml`, `commit = eac588fd`. The header
states, at the line the probe prints, "No answer is recorded here, including for
the rows whose source records one." Fourteen of the 106 rows carry an `answered`
field. Twenty-three carry a `bound`.

## 9. Findings outside the question, reported as the brief requires

**F1. `question.toml`'s header is false, and it is false in the direction that
hands conclusions to seats.** Fourteen rows carry `answered`. The header also
says `note` "says that it was [answered] and where, and never which way", and at
least one row breaks that too: `which_width_coordinates_a_consumer_writes` states
the whole dissolution in `answered`, restates its shape in `options`, and
attributes it in `note`. A seat dispatched on that row reads the answer three
times before it starts.

This is the same defect `proposal.toml`'s own header warns about, in the file next
door: "A header describing a state the file has passed is worse than no header,
because it is read as current by everyone who does not think to test it, and
nothing about it changes when the state does." That header had the discipline
written down and the file beside it did not get it. Repair: strike the sentence,
or say which rows carry `answered` and why, and stop claiming a policy the rows
do not keep. The claim costs a seat its independence, which is not a small thing
to be wrong about in a file seats are dispatched on.

**F2. `standing` is stale by construction and nothing says so.** The four
`one_expert` rows ratified by `behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
still read `one_expert`, while the ruling that ratified them records "Promoted on
two independent instances of each of the four". Under the append-only discipline
that is correct: the row claimed what its evidence supported when it was written,
and a later arrival is a new claim that lands at the promotion rather than by
editing the old row. But the consequence is that `standing` answers "what did the
establishing file have" and not "how much backing does this claim now have", and
nothing on the row says which question it is answering. A reader summing
`standing` across the registry to find promotable rows is reading a field the
promotion mechanism never updates.

This bears directly on the question, because it means an inflated `standing`
does its damage somewhere other than where the brief expects. Nobody promotes off
it. What they do is quote it, in a consolidation, as the tier a claim holds, and
that sentence is the one nothing audits.

**F3. Nothing checks a `standing` against the `promotion` that ratified it.**
Five rulings carry a `promotion` field. Each argues a count. No lint compares that
count to the `standing` on the rows the same ruling `ratifies`. That is a cheap
ratchet and the disagreement is already in the tree, at four rows. I am not
proposing the lint here, because a lint is a state somebody refuses and deciding
which of the two fields is wrong is the coordinator's call rather than mine. I am
naming it as a gap with a known non-empty population.

**F4. The lint that guards this says in its own doc comment that it cannot.**
`a_standing_is_reachable_from_what_it_cites` is explicit: "A necessary condition
rather than a sufficient one. Nothing here can tell whether two authors really
arrived separately, and it does not pretend to." The question I was sent on is
precisely the gap it declares. So the answer cannot be enforced mechanically by
what exists, and treating a green gate as evidence of independence is reading a
check for something it says it does not check. Worth stating plainly because the
ceiling of 29 is measured and falling, which makes the lint look like it is
converging on a guarantee it never offered.

**F5. The brief's own framing, restated as a finding.** Reading promotability off
`standing` is the error F2 makes structural. Both declared fields that sit between
them say so in their own descriptions. If a later reader takes one thing from this
file it should be that `standing`, `gate` and `promotion` are three different
questions and only the third decides anything.

## 10. What I could not establish, and what would close it

- **Whether the specific seat in the question falls in Arm A alone or also in Arm
  B.** That is per claim and needs its file, which I have not opened. The test is
  in section 7 and the seat's own file is where the exposure statement would be.
  I decline to guess, and section 11 is where I check.
- **How much of the 161 blind-half registry citations is derivation-time exposure
  rather than gate preamble.** The instrument that closes it splits a member file
  at its first substantive section rather than at its reconciliation, and the
  section numbering is not uniform enough across the corpus for a regex. A tool
  reading the heading tree and classifying the leading sections would do it. I did
  not build it, because the bound I could state honestly was enough for the
  argument and the unbounded number would have been the more persuasive one, which
  is the reason to distrust wanting it.
- **Whether any live proposal row's multi-arrival standing actually rests on a
  seat that read the claim first.** Establishing it needs content matching between
  a row's `says` and the text a cited file read before its blind commit, which is
  a semantic comparison rather than a grep. The cheap approximation available now
  is: for every proposal row citing two files, check whether either file postdates
  a registry row stating the same claim, by git. I did not run it because the
  same-claim test is the hard half and the date test alone would produce a list
  nobody could act on.
- **What a brief contained.** Structurally unavailable. Section 7 says why and
  gives the only repair I can see that needs no new machinery.

## 11. Reconciliation

Written after the blind commit, whose hash is filled in here.

