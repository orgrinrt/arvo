# 248. `standing` counts arrivals, not arguments, and the field it was supposed to work with has been shut for 126 of 127 rows

Seat 248, blind derivation, dispatched on one question: does the file of a seat that was
handed a conclusion by the question row it was dispatched on count toward `standing`.

**Everything up to and including section 10 was written blind**, against the canon and the
registry alone, and committed before I opened `244` through `247`. The blind commit's hash
is recorded at the head of section 11, which is the only section written afterwards.

## 0. What I read, in what order, and one leak I have to declare

Before writing a word of this I read, in this order: `mockspace.toml`;
`mock/registry/proposal.toml` and `proposal-the-later-topics.toml` in full for the header and
in extract for the rows; `mock/registry/question.toml`'s header and a sample of its rows;
`mock/registry/ruling.toml`'s identifier list and five rows in full;
`mock/registry/retirement.toml`'s header and eleven rows; `mock/registry/probe.toml`'s header
and one row; `mock/registry/strategy.toml` entire;
`mock/lints/a_standing_is_reachable_from_what_it_cites.rs` entire;
`mock/lints/canon_citations.rs` entire; `mock/lints/a_proposal_rests_on_more_than_a_consolidation.rs`
in its doc comment; `docs/QUESTION.md` and `docs/PROPOSAL.md` in their headers;
`243_seat242_the_resolution_has_no_second_arm.md`'s first forty lines, for the corpus's
prose convention and nothing else.

**The leak, stated because it is mine and because a contaminated blind read that does not say
so is worse than none.** While looking for the probe-directory convention I ran
`find mock/research/.../24*_probes -type f`, and the glob reached `244_probes`. Seven
filenames entered my context: `output_ratified_rows.txt`,
`the_standing_answers_nobody_cited.sh`, `anchor_diff.sh`, `output_standing_answers.txt`,
`no_refusal_names_an_unfixed_coordinate.sh`, `the_bound_field_and_the_blind_cut.sh`,
`output_bound_and_blind.txt`. I opened none of them and I opened no numbered file in the 244
to 247 range. Two of those stems carry information: that `244` measured something about
standing answers nobody cited, and that it looked at the `bound` field and at a blind cut. I
did not use either. What I can say is that neither told me a conclusion and neither is the
route I took, since my route runs through the schema in `mockspace.toml`, which no filename
there names. **Discount this file's blindness by exactly that much and no more, and if a
later reader thinks the discount is bigger, the reader is right and I am the wrong person to
judge it.**

**Every blockquote in this file is verbatim from the source it names. Where a
blockquote carries bold, the emphasis is mine and the words are not.** I say it once
here rather than after each quotation.

## 1. The brief's factual claim, checked, and it is wrong in a way that matters

The brief says:

> Proposal rows in the registry carry a `standing` field. The values in use across
> `mock/registry/*.toml` are `one_expert`, `two_experts`, `three_or_more`, `sound`,
> `cross_topic`, `prior_attempt`, `uncontrolled`, `defective` and `withdrawn`.

That is an accurate census of the string literals appearing after `standing = ` in that glob.
It is not a description of a field. **There are three fields called `standing`, in three
namespaces, with three disjoint value sets and three unrelated meanings**, and
`mockspace.toml` declares all three separately:

| namespace | declared values | what it means |
|---|---|---|
| `proposal` | `one_expert`, `two_experts`, `three_or_more`, `cross_topic`, `contested` | how many independent instances reached the claim |
| `probe` | `sound`, `uncontrolled`, `defective`, `withdrawn` | whether the instrument's control fired |
| `strategy` | `prior_attempt`, `op_stated`, `proposed` | where the name came from |

The brief's list is the union of the three, minus the three values nobody has used
(`contested`, `op_stated`, `proposed`). **The three missing ones are precisely the three that
would have shown it is three fields**, because each is the one value in its namespace that
nothing in the corpus instantiates. So the merge is not a slip in the brief's wording; it is
what the corpus looks like when you read the field by its name instead of by its namespace,
and that is the first thing wrong here.

I say this at the top because the brief's framing carries the conflation into the question:
"is `standing` counting independent instances of the claim, or independent arguments for it"
presupposes one field whose semantics are open. **The `proposal` field's semantics are
written down, they are not open, and they answer the question outright.** Section 3.

## 2. Where `standing` is defined, and the fact that it is not canon

`mockspace.toml` declares `canon_paths = ["mock/registry/*.toml"]`. The comment above that
line is unusually explicit about what it buys: "The registry IS the canon. Nothing under
`mock/canon/` and no prose file: the canon is rows."

**`standing` is defined in `mockspace.toml`, which is not under `canon_paths`.** I grepped
`ruling.toml` for the word: eight hits, every one of them the ordinary English "standing
rules" or "standing instruction" or "left standing", and not one of them the field. No
`ruling` row defines it, bounds it, or ranks its values. Nothing ratified says what
`two_experts` means.

So the situation is:

- The **definition** lives in configuration, at `mockspace.toml`, tier: agent-authored config,
  no recorded human decision.
- The **necessary condition** lives in a lint doc comment, `mock/lints/a_standing_is_reachable_from_what_it_cites.rs`,
  tier: code, agent output.
- The **values** live in 127 rows, tier: `proposal`, which the namespace's own description
  calls "A claim the panel established and op has not seen."
- The **ratified** material that bears on it is one row,
  `ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`,
  which never uses the word.

**This is a locus finding and I am raising it as one under the brief's own licence.** The
panel has spent a sitting arguing about the tier a field confers, and the field's meaning has
never been ratified and, at its current address, cannot be: a canon edit is a `mock/registry/`
edit, and `mockspace.toml` is not there. The consequence is not academic. Section 8 shows what
has been happening in the gap.

## 3. What `standing` counts, in the schema's own words

`mockspace.toml`, the `proposal` namespace, the `standing` field:

> How many independent instances reached it, over the region they share. `one_expert`;
> `two_experts`, **each deriving before reading the other**; `three_or_more`; `cross_topic`
> where separate topics arrived at it **without citing each other**, which is the strongest
> thing this panel produces; `contested` where it is stated because somebody stated it and
> somebody else disagrees.

And the paragraph immediately above the namespace block:

> `standing` is how many independent instances reached it, and it is the field most likely to
> be inflated. Two derivations from one model family on one premise set are the weakest kind
> of two. [...] So the count is over the region the instances share, intersected over values
> rather than over dimension names, and where one instance spans an axis alone the honest row
> cites that instance for that axis rather than the convergence.

Three things are settled by that text and none of them is a matter of taste.

**First, it counts instances that reached the claim.** "How many independent instances
reached it." Not arguments, not routes, not supports. The unit is an arrival at a
proposition.

**Second, independence has a temporal definition and it is stated: `each deriving before
reading the other`.** That is not a description of intellectual independence, which is
unmeasurable. It is an ordering condition on two events, and it is the only operational
content the field has.

**Third, `cross_topic` adds a citation condition: `without citing each other`.** So the
strongest tier is defined partly by the absence of a citation edge.

The brief asks whether `standing` counts instances of the claim or independent arguments for
it. **It counts instances of the claim.** The alternative reading is not merely unsupported;
it is the reading the field was written to exclude, which is why the definition spends its
words on ordering and citation rather than on quality of reasoning.

## 4. Where the arguments went, since they did not vanish

They went into a different field, and the design already separates the two things the
question conflates. `mockspace.toml`, `proposal.gate`:

> Why the coordinator refused to promote it, where one did. [...] **The bar and the gate are
> two different things: convergence says several experts reached it, and the gate asks whether
> the reasoning, the evidence and the stated region are enough for canon.** A proposal at
> `two_experts` with no `gate` has not been looked at yet.

And the ratified row says the same thing in op's own structure.
`ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`, rung
`ratified`, ratified by op, with his verbatim quotation attached:

> **The experts propose; the coordinator holds the gate; convergence is the bar rather than
> the trigger.**

and in its `note`:

> It is not automatic promotion on a count of agreeing experts [...] **A promotion carries
> the reasoning that justified it, so it can be argued with rather than merely counted.**

**So the decomposition is corroborated across two tiers that were written by different hands
for different purposes**, a ratified process ruling and a schema field description, and they
agree on the split: `standing` is the counted thing, `gate` is the reasoned thing. The
question "is `standing` counting claims or arguments" has an answer because the design
already answered it by giving arguments their own field.

That is the whole of the answer to the question as asked. The rest of this file is what
follows from it, what the corpus actually looks like, and where I think the question is
pointed at the wrong field.

## 5. Therefore: the seat contributes zero, by definition and not by discount

Take the case the brief describes. A seat is dispatched on a question row. That row's `note`
or `options` state a conclusion. The seat argues to that conclusion by a route nobody had
written down, and reaches it.

Against `each deriving before reading the other`: **the seat read the conclusion before it
derived anything.** The conclusion was in its brief. So the ordering condition is not
narrowly missed, not weakly satisfied, not satisfied-but-discounted. It is inverted. There is
no reading of "deriving before reading" on which a seat handed the conclusion in its dispatch
satisfies it.

Against `cross_topic`'s `without citing each other`: the brief reports that the seat's file
**cites the row's own note as its source**. That is the citation edge the strongest tier is
defined by the absence of.

**This matters as a distinction because "zero by definition" and "zero after a discount" have
different consequences.** A discount can be argued back up; several rows in this corpus do
exactly that, stating a discount in a `note` and keeping the tier. The corpus's own precedents
for that shape:

- `proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`, at
  `two_experts`, whose note says "both derivations draw on one numerical-analysis literature,
  so this is two independent instruments over one shared premise rather than two arrivals from
  nothing", and keeps the tier because "there is no tier between the two that the schema can
  express".
- `retirement::r178_the_discount_carried_at_three_of_five_components`, which restores the
  reason a discount is a discount: "an empirical claim two parties reach independently is
  corroborated because the world had to cooperate twice, while a definition two parties reach
  independently may be two members of one model family finding the same framing natural on one
  premise set".

**Read `r178` forward and it decides my case.** A *suspected* shared premise set warrants a
discount. Here the shared premise is not suspected: it is the row, in the seat's brief, in
writing, cited in the seat's own file. What `r178` treats as grounds for a discount is present
here in its established rather than its hypothesised form, and an established common cause does
not reduce corroboration, it eliminates it. Nothing about the seat's agreement is explained by
the world; all of it is explained by the handover.

And the corpus has already ruled the adjacent case.
`retirement::r173_rb_the_three_way_definitional_convergence` retires the claim "Three instances
converge on one definition, so the convergence is three-way", replacement: "**Two instances
plus a composing third definition.**" A third party that *composed* what was there did not
increment the count. A seat that was handed the conclusion has done less than compose: it has
agreed.

`retirement::r176_ro_the_closure_recorded_only_in_a_rung_history` is the same verdict on a
closure rather than an arrival: "the closer shares the premise set whose influence the option
exists to test, **so the closure is a second read rather than independent evidence**".

**A second read.** That is the corpus's own name for what this seat produced, and it already
distinguishes it from evidence.

## 6. Where the seat's route does go, because it is real work and losing it would be the other error

The route was not handed over. If it is genuinely the seat's, it established something, and
the registry has three places for it. Which one depends on `sentence_kind`, and this is the
composition rather than a single policy: I am not saying the class contributes nothing, I am
saying it contributes to a different row, gated on which kind of sentence the target is.

**Case A. The target is `normative` or `definition`.** Contributes nothing anywhere.
`mockspace.toml` says these two "carry no region", and a stipulation has nothing for a second
route to corroborate: an argument that a definition is a good definition is not evidence that
it is the true one, because there is no fact for it to be true of. `r178`'s model-family
warning is the general case; a handed-over stipulation is the degenerate one. The seat's file
is worth reading and is worth citing in a `gate` note. It is not worth a row.

**Case B. The target is `measured`, `theorem` or `enumeration`, and the seat's route is an
entailment from premises the row did not supply.** Then the seat established the sentence
`those premises entail the claim`, which is **a different proposition from the claim** and
gets its own `proposal` row at `standing = "one_expert"`, with the claim's row untouched.

The corpus has done exactly this, deliberately, and recorded why.
`proposal::membership_in_the_type_and_identity_are_two_criteria` carries in its note:

> **Split from the const-availability row so the stronger standing is visible.** [...] the
> entailment about compulsion in the neighbouring row is the part that is one instance, and
> **folding the two together would have made this inherit the weaker rung**.

and its neighbour `proposal::the_type_carries_whatever_must_be_const_available` carries the
weaker half openly, at `one_expert`, with a `gap` saying "The second read the rung requests
has not happened. A reader treating the criterion as settled is treating a one-instance
entailment as two."

**Splitting the row is the corpus's established move when one file carries two things of
different standing.** It is what should happen here, in the other direction: not splitting one
file's contribution across two rows, but keeping the seat's contribution off the claim's row
and on its own.

**Case C. The target is `measured` or `enumeration`, and the seat ran an instrument.** This is
the one case where the handover genuinely does not matter, and it is worth being precise about
why. A measurement's agreement with a hypothesis is explained by the world, not by who
suggested the hypothesis. Being told what to look for does not make a result false.

But it does make one explanation of the result available that would not otherwise be, namely
that the instrument was built to produce it, and **the canon already has the mechanism that
excludes exactly that explanation**: `probe.standing`, whose `sound` value means "it ran and
its control fired", against `uncontrolled`, "it ran and no case that had to fail was stated,
so it produced a number and cannot be refuted", with the schema's own gloss that
"`uncontrolled` is not a lesser `defective`: the numbers may well be right, and what is
missing is any way for them to have been wrong."

So: **a seat handed a conclusion, which then ran an instrument with a stated control that
fired, has produced corroboration of the proposition.** The corroboration is the probe's. It
is filed as a `probe` row at `sound`, the claim's row points at it through `evidence` or `law`,
and **the claim's `standing` still does not move**, because `standing` counts people who
reached it and this seat did not reach it, it checked it. That is not a technicality: a claim
backed by one arrival and two controlled instruments is a different and often better thing
than a claim backed by two arrivals, and flattening them into one integer is what loses the
difference.

**And if the instrument is uncontrolled, it contributes nothing at all**, because the
handover is then an unexcluded explanation of the agreement and the probe namespace says so.

**Case D. The target is `argument`.** Nothing. An argument for a conclusion you were given is
the definition of the thing that does not count, and `sentence_kind = "argument"` is described
in the schema as "the mark that keeps getting dropped", which is its own warning.

## 7. Summarised as a gate

For a proposal row P and a seat file S, where S was dispatched on a question row Q whose
`note` or `options` state P's conclusion:

```
P.standing increments  ==  false,  for every P, every S, every sentence_kind.
```

and separately

```
a new probe row at standing="sound"   if  P.sentence_kind in {measured, enumeration}
                                      and S ran an instrument
                                      and S stated a case that had to fail
                                      and that case fired
a new proposal row at standing="one_expert"
                                      if  P.sentence_kind in {measured, theorem, enumeration}
                                      and S's premises are disjoint from Q.note
                                      and S.says is the entailment, not P.says
a gate note on P                      otherwise
nothing                               if  P.sentence_kind in {normative, definition, argument}
```

**The first line is the answer and it has no exceptions.** The block under it is where the
work goes, and it has four arms because the four cases genuinely differ.

## 8. What the corpus actually looks like, which is worse than the question assumes

Four instruments, in `248_probes/`, each stating the cases that had to fail before the run and
each exiting non-zero if any of them did not. All four exited zero. Sources and outputs
committed beside this file.

### 8.1 `p1`: 33 of 35 multi-arrival rows cannot be reached through two independent authors

`a_standing_is_reachable_from_what_it_cites` asks whether a row claiming several arrivals
cites two distinct **files**, and its own doc comment says why: "Independence is between
authors, and a numbered member file has one author." But `files_cited` counts any file, and
three of the things a `provenance` entry can name are not authors: a living ledger, a probe
artifact, and the topic consolidation, which
`ruling::the_canon_is_written_once_at_the_end` says "has no standing beyond that".

`p1` re-runs the lint's predicate and then re-runs it counting only what can be an author.

- The lint arm reports **29**, which is the exact value of `CEILING` in that lint, measured
  over this same corpus. That was control C1 and it is what says my transcription of
  `file_named` is the lint's rather than my own.
- The author arm reports **33 of 35**.

The two rows that survive are `the_concept_is_closed_and_the_inventory_is_open` (files 65 and
66, the two cold number-system derivations) and
`whether_an_operation_needs_chain_machinery_is_whether_its_rounding_step_is_closed_under_its_own_algebra`
(files 76 and 77, the two cold derived-law derivations). Both are blind cold pairs, which is
the shape the field was written for.

The four rows that clear the lint and fail the author test are worth naming, because each shows
a different way the necessary condition is met without the sufficient one:

- `a_composed_expressions_region_is_never_inherited_from_its_parts`: consolidation 90 plus one
  member file. One author outside a compression.
- `chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type`, at
  `cross_topic`: consolidation 106 plus `AGREEMENTS`, a **living ledger**. Zero authors.
- `the_law_frame_was_attacked_from_another_topic_and_held`, at `cross_topic`: consolidation 74
  plus `AGREEMENTS`. Zero authors.
- `an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit`: consolidation 63
  plus **two probe artifacts**. Zero authors.

**Both `cross_topic` rows that clear the lint clear it on a ledger citation.** `cross_topic` is
the tier the schema calls "the strongest thing this panel produces", and it is defined by
separate topics arriving "without citing each other". A citation into `AGREEMENTS` is a
citation into the panel's own convergence ledger, which is the record of topics citing each
other.

### 8.2 `p2`: zero of 35 have mechanical independence evidence

`proposal::a_strategy_is_an_assignment_and_a_weighting` states the panel's own instrument for
independence and its limit in one sentence: "The commit ordering establishes the within-file
half for each and nothing about the between-file half, and for one member it runs the wrong
way: its phase one landed two minutes after the other member's file was already in the tree."

`p2` runs that instrument over the two survivors and over five other cold pairs.

| pair | gap between the two add-commits |
|---|---|
| 65 / 66, the number-system cold pair | 76 seconds |
| 76 / 77, the derived-law cold pair | 33 seconds |
| 109 / 110, the primitive cold pair | 1393 seconds |
| 125 / 126, the rounding cold pair | 503 seconds |
| 09 / 10 | 1569 seconds |

Control C1 required consolidation 74 to land after member 65, which it does by 9389 seconds;
had the comparison been inverted this arm would have said so. Control C3 required the gaps not
to be constant; seven distinct values were observed.

**In every pair, at the moment the second file landed, the first was already in the tree and
readable.** So no pair in this corpus has mechanical evidence of "each deriving before reading
the other". Blindness rests entirely on the members' own coverage statements, which is what
`a_strategy_is_an_assignment_and_a_weighting` already says about itself and what nothing else
says about anything.

**This is not an accusation that the cold pairs were not blind.** It is the observation that
the field's operational condition is unverifiable across the entire corpus, which bears
directly on the question I was asked: if we cannot tell a blind arrival from a read one for the
two rows that pass every other test, the marginal case of a seat that admits it read the
conclusion is not a hard call.

### 8.3 `p3`: there is no such thing as a bare question row to dispatch a seat on

`question.toml`'s own header states the discipline:

> No answer is recorded here, including for the rows whose source records one. Where a question
> was answered, `note` says that it was and where, **and never which way**.

Measured against the file:

- **106 of 106** question rows carry a `note`.
- **106 of 106** carry an `options` list.
- **14** carry a populated `answered` field, which is an answer recorded there.
- **23** carry a `bound`.
- **33 of 106** `note` fields match a verdict vocabulary (`refut`, `undercut`, `measured
  false`, `closed by`, `dissolve`, `settled`, `retired`, `superseded`, `ruled out`,
  `excluded`, `withdrawn`, and others), against **3 of 106** for the `asks` field, which is
  interrogative by construction and is the false-positive floor. That ratio was control C1 and
  it is what says the vocabulary discriminates rather than matching ordinary technical prose.

So roughly a third of the question rows in the namespace state which way they go, in a
namespace whose header says none of them do, and **every one of the 106 carries the panel's own
compressed prior positions in its `options` list**. The header also says outright that ports
came from `OPTIONS.md`, the live option register, and from `156`, the standing queue: "Both are
living ledgers".

**A question row is not a prompt. It is a compression of prior member work with a question mark
attached.** So the case the brief describes is not a seat being unlucky in its dispatch; it is
the ordinary consequence of dispatching a seat on a question row at all.

### 8.4 `p4`: the shape occurs six times, not once

The brief says the rows in the concrete case "were introduced by the same commit as the file
whose argument the seat was later credited with seconding". `p4` counts that shape over the
whole history: **6 of the 34 commits that touch `question.toml` also land a numbered member
file in the same act.**

Control C2 required that a commit touching `question.toml` with no member file classify as not
co-landing; 28 do, which is what says the pattern is not matching the registry path itself.
Control C3 asserted the member-file pattern against five strings whose answer is known,
including a probe artifact and `OPTIONS.md`, both of which it correctly rejects.

Where a row is written in the same act as the file that argued for it, **the schema's ordering
condition cannot hold for anything dispatched on that row afterwards, by construction**: the
row, carrying the earlier file's conclusion, was in the tree before the later seat opened
anything.

## 9. The locus challenge, and it is bigger than the question

The brief says what the answer changes: "how much of a completed sitting counts as independent
backing, and therefore whether certain proposal rows are promotable."

**Promotability does not run through `standing`.** It runs through `gate`, and `gate` is
populated on **1 of 127** proposal rows.

The schema's own reading of an empty `gate`: "A proposal at `two_experts` with no `gate` has
not been looked at yet; one carrying a `gate` has, and was refused with a reason somebody can
argue with." By that reading **126 of 127 proposals have not been looked at**, and the
ratified ruling that makes promotion possible at all,
`two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`, puts the
decisive act precisely there: "convergence is the bar rather than the trigger", "the
coordinator remains the gatekeeper", "somebody has to judge that a proposition is justified,
reasoned and demonstrated enough to deserve canon".

**So the panel has been refining the bar while leaving the gate shut**, and the sitting I was
dispatched into is a further refinement of the bar. Getting my answer right changes the value
in one field on some rows. It does not make one row promotable, because promotion needs a
judgement nobody has recorded 126 times.

Three more things in the same place, each unlicensed by anything ratified and each reported
under the standing instruction to name them whether or not they are in scope.

**`contested` is declared and used zero times.** Across 127 proposal rows, in a repository
carrying a 2217-line `retirement` namespace whose entire subject is claims that turned out
wrong, the one `standing` value that records disagreement has never been written. Every one of
the 127 rows carries a value from the count-shaped end of the range. A field whose only unused
value is the negative one is a field that only goes up, and that is a structural bias toward
inflation independent of any individual row's honesty.

**`question.toml`'s header asserts a property of the file that the file does not have**, and
this is the exact class that `proposal.toml`'s own header names and repairs about itself: "A
header describing a state the file has passed is worse than no header, because it is read as
current by everyone who does not think to test it, and nothing about it changes when the state
does." `proposal.toml` learned this and rewrote its header. `question.toml` still says "No
answer is recorded here" over fourteen rows carrying `answered`.

**The panel has no axis vocabulary for its own process claims.** `dimension.toml` declares
twenty-five axes and every one of them is arithmetic or machine: widths, signedness, rounding,
container, threads, toolchain, occupancy, phase. `mock/lints/every_predicate_names_a_declared_axis.rs`
enforces that a predicate names a declared axis. **There is no declared axis over which a claim
about the corpus can be predicated** at all: not namespace, not tier, not authorship, not
citation depth. So `every-finding-carries-its-predicate` is unsatisfiable for exactly the class
of claim this whole sitting produces, including this file, and every process finding in the
registry is therefore either unpredicated or predicated in a vocabulary the lint cannot
recognise. I do not know whether that is a gap somebody should close or a sign that process
findings do not belong in a namespace shaped for arithmetic ones, and I am not licensed to
decide it. It is named because nothing else names it.

## 10. What I could not settle, and what would settle it

**Whether the concrete seat's route was in fact independent of the row.** I did not open the
seat's file. My answer does not depend on it: the ordering condition fails whatever the route
is, and Case B and Case C in section 6 say where a genuinely independent route goes without
needing to know whether this one was. But a reader wanting to apply the gate to that specific
seat has to read it, and I have not.

**Whether `standing` should exist at all.** I can see the argument that a single integer over a
sentence is the wrong object, that the corpus already carries the richer thing in `provenance`
plus `probe.standing` plus `gate`, and that collapsing them into a count is what makes an
inflation possible in the first place. I did not pursue it, because retiring a field on 127
rows is a change of a different order from answering the question I was sent, and because
`conceding-is-an-answer` says not to manufacture a proposal to fill a slot. **It is named as an
opened question and not as a recommendation**, and what would close it is somebody deriving,
independently of me, what `standing` buys that `provenance` and `gate` together do not.

**Whether a controlled instrument from a handed-over seat should also lift the claim's own
tier**, rather than only landing as a `probe` row. I say no in section 6 Case C, on the ground
that `standing` counts arrivals and a check is not an arrival. I hold that with less confidence
than the rest of this file, because there is a real argument the other way: three controlled
instruments agreeing is exactly what `evidence-lives-in-the-repo-or-it-never-happened` calls
for and `standing` is where a reader looks for how well-backed a row is. **What would settle
it: whether anyone can name a case where `probe` rows carry the backing and a reader was
misled by the claim's `standing` reading low.** If nobody can, my answer is safe and merely
pedantic; if somebody can, the field is doing two jobs and section 10's second item becomes the
live question rather than this one.

**Whether my leak in section 0 changed anything.** I cannot check this from inside my own head
and I am the wrong instrument for it. The route is checkable by anybody: it runs through the
`standing` and `gate` field descriptions in `mockspace.toml` and through the ratified ruling,
and if `244` reached the answer another way then the two are independent in the sense the
schema means, and if it reached it through the same schema text then they are one instance and
should say so.

## 11. Predicates

Per I13 and `every-finding-carries-its-predicate`, every finding here carries the region it
holds in, exactly, with an unlisted dimension meaning the finding does not hold where that
dimension exists at all. **None of the axes below is declared in `dimension.toml`**, for the
reason given in section 9, so these are written in the honest ad-hoc vocabulary and the fact
that they cannot be written in the sanctioned one is itself section 9's finding.

**F1. `standing` on a `proposal` counts instances that reached the claim, with independence
defined as each deriving before reading the other.**
Holds for: the arvo repository at `eac588fd2dfd157faaf088ae69c6342227ac2c98`;
`mockspace.toml` as committed at that ref; the `proposal` namespace only; `threads = 1`;
`toolchain = nightly-2026-05-28`. Derived from text, not measured, so no instrument bound
applies. Sentence kind: this is a reading of a definition, not a measurement.

**F2. A seat handed the conclusion by the row it was dispatched on contributes zero to that
claim's `standing`.**
Holds for: the same ref; the `proposal` namespace only; every `sentence_kind` in
`{theorem, measured, enumeration, definition, normative, argument}`; every `standing` value in
`{one_expert, two_experts, three_or_more, cross_topic, contested}`; `threads = 1`. Follows from
F1 by the ordering condition and does not depend on any measurement in section 8.

**F3. 33 of 35 multi-arrival proposal rows cannot be reached through two independent author
files; 2 can.**
Holds for: the arvo repository at `eac588fd2dfd157faaf088ae69c6342227ac2c98`; the namespaces
`proposal` and `proposal-the-later-topics`; the standings `two_experts`, `three_or_more`,
`cross_topic`; the author classification stated in `p1`'s source, which reads a living ledger,
a probe artifact and a topic consolidation as non-authors; `threads = 1`;
`toolchain = nightly-2026-05-28`. Instrument: `248_probes/p1_who_can_a_standing_be_reached_through.sh`,
four controls stated before the run, all four fired.

**F4. No multi-arrival proposal row in this corpus has mechanical independence evidence.**
Holds for: the same ref; the seven pairs `p2` names and no others; commit author-date ordering
as the instrument; a one-hour separation threshold; `threads = 1`. Instrument:
`248_probes/p2_what_ordering_evidence_exists.sh`, three controls, all three fired. **The
quantifier is over the seven pairs measured**, not over every pair in the corpus, and a reader
wanting the universal has to run the remaining pairs.

**F5. 106 of 106 question rows carry both a `note` and an `options` list; 14 carry `answered`;
33 of 106 notes carry verdict language against a 3-of-106 false-positive floor on `asks`.**
Holds for: the same ref; the `question` namespace only; the verdict vocabulary written into
`p3`'s source and no other; `threads = 1`. Instrument:
`248_probes/p3_does_a_question_row_hand_over_its_answer.sh`, three controls, all three fired.
**The 33 is an upper bound on rows stating a verdict and a lower bound on nothing**: a note may
state which way without using any of those words, and `p3` cannot see that.

**F6. 6 of 34 commits touching `question.toml` also land a numbered member file in the same
act.**
Holds for: the same ref; the reachable history of `mock/registry/question.toml` on this branch;
the member-file pattern written into `p4`'s source; `threads = 1`. Instrument:
`248_probes/p4_a_row_and_its_seconder_in_one_act.sh`, three controls, all three fired.

**F7. `gate` is populated on 1 of 127 proposal rows; `contested` on 0 of 127.**
Holds for: the same ref; the namespaces `proposal` and `proposal-the-later-topics`;
`threads = 1`. Instrument: a `grep -c` over the two files, reproducible in one command, with
the count of `[[proposal]]` headers as its denominator.

**F8. No `ruling` row defines, bounds or ranks `standing`, and `standing`'s definition lives
outside `canon_paths`.**
Holds for: the same ref; `mock/registry/ruling.toml` as committed; `canon_paths` as declared in
`mockspace.toml` at that ref; `threads = 1`. This is an absence claim, so it carries its
search: `grep -in "standing" mock/registry/ruling.toml`, eight hits, every one of them the
English phrase rather than the field; and `canon_paths = ["mock/registry/*.toml"]`, which does
not glob `mockspace.toml`. **A reader who believes this is wrong should re-run the grep rather
than re-read this sentence**, because an absence claim inverts silently the moment somebody
writes the row.
