# 236. The seven `the_format` proposals against the ratification gate

**Date:** 2026-09-01. **Position:** a read of every `proposal` row whose `topic` is `the_format`,
asking of each whether it is ratifiable today. Worktree `rat-b`, branch
`research/ratify-the-format-rat-b`, cut from `origin/dev` at `0cac9beb`.

**Gates.**

Canon gate: **passes, and it moves the question before it answers it.** The brief asks which
format proposals are ratifiable. Three of the seven already are: `ruling::the_format_spine_is_canon`
is `rung = "ratified"`, `ratified_by = "both"`, and its `ratifies` field names
`a_format_is_identified_by_its_ambient_domain_and_its_representable_set`,
`membership_of_the_representable_set_is_one_affine_predicate`,
`arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation` and
`the_concept_is_closed_and_the_inventory_is_open`, the last of which is on `the_number_system`.
So the live question is over four rows, not seven, and the first section below says so before
anything else. Nothing here proposes a mechanism, writes a registry row, or settles a question
reserved elsewhere.

Test gate: **run, and it is green on the surface this touches.** `cargo mock test` walks nine
trees. The lint tree is 603 passing tests and I read bodies rather than names in the four lints
this file's judgements rest on: `a_standing_is_reachable_from_what_it_cites`,
`a_proposal_rests_on_more_than_a_consolidation`, `canon_citations` and
`measured_claim_cites_no_probe`. None is tautological, none is sampled where a matrix was
available, each carries named `control_` arms including the empty-registry and the
ceiling-fires-above cases, and both ratchets carry their measurement and their reasoning in the
module doc rather than a number somebody picked. `cargo mock --lint-only` is green over the
committed registry, 700 rows across ten namespaces, schema check passed, five warnings all in
`arvo-format` and `arvo-placement` source and none of them mine.

**Two things the gate found that are not defects in the work and are reported so nobody re-derives
them.** Four of ninety-five bench variants fail `cargo metadata`, and each of the four carries a
`FIXME` at the top of its own manifest saying it has not built since the crates it names were
deleted, that it is kept because its committed results are cited, and that repairing it means
inventing arms that measure nothing. That is a marked state rather than a rotting one. And a second
invocation of `cargo mock test` failed the generated lint tree with `E0460` and four `E0308`s
naming two different checkouts of `mockspace_lint_rules`, `b4e0c7a` and `a9268f6`; `mockspace.toml`
pins `mockspace_branch = "dev"` and the launcher re-resolves the branch head, which moved between
the two runs. Infrastructure, not the tree.

---

## 1. The headline: three of the seven are canon already, and the spine's own citations are not checkable

`cargo mock query 'proposal.where(topic=the_format)'` returns seven rows. Three of them are named
under `ruling::the_format_spine_is_canon`'s `ratifies`, and `grep -c` over `ruling.toml` confirms
that each of the other four is named by no ruling at all. So the answer to "which are ratifiable
today" is over four rows.

The ruling says what they are, and it carries op's words:

> Four propositions become canon together and they are the format topic's spine. A format is
> identified by its ambient domain and its representable set, and that set is a constant of the
> type. Membership in it is one affine predicate over one parameterisation, of which integers,
> fixed point, scaled integers and floats are points. Arithmetic on a format is an exact operation
> in the ambient domain composed with a named total adaptation onto that set, and the adaptation is
> a first-class object with its own laws.

**The bar the other four have to clear is stated twice in the canon and both statements are
ratified.** `ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`,
`rung = "ratified"`, `ratified_by = "op"`:

> Where several experts converge heavily and can reason and spell it out, and the coordinator
> judges the reasoning, the evidence and the stated region sufficient, the proposition is promoted
> to canon. [...] The experts propose; the coordinator holds the gate; convergence is the bar
> rather than the trigger.

And `ruling::the_additive_and_absorption_verdicts_are_canon`, the only ruling that has since put
that bar to work, records in its `promotion` field what the gate actually asks:

> Neither prose reaches past its predicate, which is the thing this gate is for and which the third
> proposal read alongside them failed.

The third proposal is `no_multiplicative_structure_survives_a_nonzero_fraction_width`, and its
refusal is written on the row itself, in a `gate` field:

> The statement claims the result holds "for any policy, sign domain, range or rescale spelling",
> and the predicate carries `total_width: W in 3..=7`. A measured band of five widths is not any
> range [...] That sentence is what a later reader quotes, and nothing audits it.

**So the gate has three limbs and all three are cited above: convergence, evidence, and prose that
does not reach past its predicate.** There is also a fourth thing the canon does with a one-expert
row, and it is worth quoting because it is the disposition three of my four verdicts land on.
`ruling::the_fused_result_is_composable_except_at_signed_saturating` is `rung = "stated"` and its
note ends:

> **One expert.** Recorded at `stated` rather than promoted, and a second instance is owed before
> it becomes canon.

**On the three that are already canon, one thing is worth recording and it is not a proposal to
undo anything.** All three rest on a single file. Their `provenance` fields carry two entries each
and both entries in each are `63_spj_consolidation_the_format_concept`, at two different line
offsets. `canon_citations::files_cited` counts distinct files precisely because "two anchors into
one file are one author", so each of the three names one author while carrying
`standing = "two_experts"`. That is the state
`a_standing_is_reachable_from_what_it_cites` grandfathers at a ceiling of 29, and that lint's own
module doc says what it means here: **"Under the rule that two agreeing experts ratify, these are
exactly the rows eligible for promotion, and not one of them names a second source."** The
convergence behind the spine is real, and `64_ringer_entailment_check_on_the_format_consolidation`
records the rungs member file by member file, so it is recoverable. It is simply not recoverable
from the rows. The additive repair `a_proposal_rests_on_more_than_a_consolidation` describes,
keeping the consolidation citation and adding the establishing files beside it, is owed on all
three.

One narrower note on the same three, because it bears directly on section 5 below.
`membership_of_the_representable_set_is_one_affine_predicate` was ratified whole, and its own
`note` says of one of its clauses:

> The necessity of the phase term does not: it is one expert's measurement, plus a concession, plus
> a constructive repair, and the consolidation offers it as an argument for stating phase
> explicitly rather than as a two-expert standing.

**The ratified sentence itself is true and my probes agree with it at every geometry I ran**, which
is the important half: a nonzero phase does decide whether the identity adaptation occurs and does
take the additive identity off the grid. What is false is a wider clause that lives in the
*proposal* row beneath it and in the shipped crate, and never in the ruling. Section 5 has it.

---

## 2. The structural finding, which decides three of the four verdicts

**The format topic has never been second-read, and no file in the corpus has ever read these four
rows.**

Three second reads exist. `214_jhala_a_standing_nobody_can_check` is the twenty-one
`the_number_system` proposals. `215_kiselyov_second_read_the_algebraic_laws` is the eleven
`algebraic_laws` rows at one expert. `216_lamport_second_read_the_chain_and_the_realisation_map` is
those two topics. There is no fourth.

That is a claim about absence, so it carries the search that established it, with a positive
control, per the discipline that a zero from an instrument is a claim about the instrument.
Searched: `grep -rlI '<slug>' --include='*.md'` over the whole panel directory, 430 entries.

- **Positive control**, a `the_number_system` slug that is known to have been second-read:
  `the_concept_is_closed_and_the_inventory_is_open` returns six files including `214`. The
  instrument works.
- `the_adaptation_slot_is_derived`: four files, `187`, `188`, `221`, `222`. None is a second read;
  `188` is the entailment check on the port, the other three are edge-wiring and fundamentals
  passes that cite the slug rather than read the claim.
- `the_format_concept_carries_three_things`: four files, `188`, `189`, `195`, `222`. Same shape.
- `a_nonzero_phase_leaves`: five files, `182` (the port that wrote it), `189`, `221`, `232`, `233`.
  `232` and `233` are the rounding-vocabulary work and reach only its `rounding` entry.
- `raw_order_agreement_holds_for_monotone`: **zero files.** No panel file mentions this row at all.

**And a second read cannot supply the missing instance anyway, which is the panel's own standard
rather than mine.** `214` states it, having been sent to do exactly this job on a different topic:

> **I could not raise a single standing, and I want to be exact about why, because "second read" as
> a dispatch shape presumes something this topic cannot supply.** To evaluate a row I had to read
> the file that established it. Having read it, my agreement is a read. [...] **So a second read of
> a topic can raise standing only in two ways.** It can find a pre-existing independent arrival the
> consolidation missed, which is a fact about the corpus rather than about the reader [...] Or the
> second reader can be dispatched **before** reading the sources, derive cold from the premises,
> and then compare.

I read the sources. So for every row below where my reading agrees, that agreement is a read and
raises nothing. What I looked for instead is the first of `214`'s two routes, a pre-existing
arrival the consolidation missed, and where I found refutations rather than agreement those stand
on their own, because a counterexample needs no second instance.

---

## 3. `the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`

**Not ratifiable.** Two blockers, and the second is repairable without any new measurement.

**Blocker one: one arrival, and the second party conceded after reading.** The row's own note
disaggregates it: *"The slot-is-derived claim is one author's."* Tracing it: `63` section 3.4 rests
the slot half on `55b:38-44`, and `55b` is Smith replying to Knuth, whose opening line is
*"Reply to 56: two concessions, two withdrawals, one advance, one refutation"* and whose read-list
is *"`56` in full, its `q1` source and output, the `q2` and `q3` outputs."* `55`'s phase one had
argued the opposite, under the heading *"3a. Wrapping is not an adaptation, and pretending it is
one costs the algebra"*, and `55b` section 3 withdraws it: *"the expulsion argument is withdrawn,
and with it 'wrap as a change of ambient domain'."*

The registry already knows what that shape is worth. `a_format_is_identified_by_its_ambient_domain
_and_its_representable_set`'s note says of its own second half that it is **"converged by attack
and concession rather than independently derived, which the source marks explicitly as weaker than
two experts because the conceding file read rather than derived."** `64` says the same thing about
this exact question at its line 155: **"through argument, which is a real result and is not the
TWO EXPERTS rung, because `55b` conceded."**

So the disposition the canon already has for this is
`the_fused_result_is_composable_except_at_signed_saturating`'s: stated, and a second instance owed.

**Blocker two: the row is filed `normative` and one of its four sentences is measured.** The
`proposal` namespace's own header states the test:

> `sentence_kind` is what established the claim, not what the sentence sounds like. [...] A claim
> that could be measured false is not `normative` however definitional its grammar, and it carries
> the region it was established in or it is not here at all.

"All four combinations occur" could be measured false. It was measured true, by `56_probes/q1_two
_law_families.rs`, at four bits in both sign domains, and that measurement is already a registry
row: `law::coherence_of_a_reduction_onto_its_induced_operation` carries it with the region
`W = 4, F = 0, signed, saturate, operations {add, mul}`. The proposal restates it with no region at
all, under a label the checker exempts from carrying one. `182_orchard_porting_the_four
_consolidations`, which wrote this row, names the incentive itself in section 6:

> **The incentive runs one way and I want that said plainly: `normative` is the only label the
> checker exempts from carrying a region, so every borderline call I made had a thumb on the scale
> pushing it there. Audit these first.**

Its section 6.1 then lists six rows it flags as the class to attack. All six are R-numbered, from
the later consolidations. **No C-numbered row from `63` is on that list, so the format topic's
sentence-kind assignments carry the stated incentive and none of the stated audit.**

**What would make it ratifiable.** Drop the measured sentence from `says`, since the law row
already carries it with its region and the `law` field already points there, which costs nothing.
Recover `55b` and `56` into `provenance` beside the consolidation. Then it is a normative claim at
one arrival, and it needs the second: a cold derivation on whether the space of total reductions
onto a fixed representable set is chosen or derived, dispatched at the premises with the registry
withheld, which is `214`'s second route and has never been run for this topic.

**One thing that is not a blocker, recorded because it looks like one.** The row settles
`question::where_wrapping_lives`, which is open with `decider = "panel"` and two live options, and
its `says` takes the first. A panel-decidable question being closed by a promotion is the mechanism
working, not an overreach. And the third sentence, that the strategy selects the member, sits
exactly where `ruling::the_strategy_is_what_makes_an_answer_correct` (I9, stated) puts it, with
`ruling::which_half_of_the_pair_i9_attaches_to_is_not_his` handing the placement to the experts in
op's own words: *"optimal and converged to by experts (plural, iterative)."* **Plural is the word
that blocks it. One is not plural.**

---

## 4. `the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws`

**Not ratifiable.** One blocker and it is decisive, plus a shape finding worth more than the
verdict.

**Blocker: one arrival, and the row says so itself.** Its note: *"One expert, uncontested rather
than corroborated. Absence of contest is not a second instance, and the consolidation does not
claim it is."* `63` agrees at line 283, filing the compositions clause **"ONE EXPERT, uncontested
through seven subsequent files"**, and at line 577 filing the statability argument **"ONE EXPERT,
cold, reconciled against the unit without contradiction, and unattacked."** Nothing in the corpus
has read the row since; section 2's search returns four files and none of them is a reader.

**The shape finding: it is two propositions by two authors welded into one row, and promoting it
would promote both in one act.** `63` rests the two halves on different member files.

- The three-things list and the statability argument under it come from `60:206-214`, Stam's cold
  derivation of the chain. `64` at line 250 confirms the attribution: *"the three things named
  (width algebra, named adaptation, exactness predicate) are what `60`'s own statability argument
  shows is required for I7 to be expressible at all."*
- The compositions-are-not-format-instances clause comes from `55:176-193`, Smith's cold derivation
  of the format concept, `63:280`.

**Two authors at one instance each is not two instances of one claim.** It is the shape
`expert-dispatch-defends-the-canon` names as agreement about the intersection of what each varied,
which here is empty: neither author derived the other's half. The row's `standing = "one_expert"` is
the honest field value and reads as understating a two-author row when it is doing the opposite.

**What would make it ratifiable.** Split it, one row per clause, each carrying its own establishing
file recovered into `provenance`. Then each is a normative claim at one arrival, and each needs its
own second. The statability half is the more valuable of the two and `64` says so at line 252,
calling it *"the more defensible of the two chain sentences"* and one it would trust over `C9` if
only one could survive; it is also the half whose subject is `the_chain` rather than `the_format`,
which is a locus question the split would force somebody to answer.

---

## 5. `a_nonzero_phase_leaves_the_representable_set_without_an_additive_identity`

**Not ratifiable.** Three blockers. The first is already an open registry question, the second is
measured below and is the reason this file has probes, and the third is the standing.

### 5.1 Its predicate names a rounding mode the canon retired

The row's predicate carries `rounding: rounding = nearest, against a phase-zero mutant`.
`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names` is `rung = "ratified"`,
`ratified_by = "op"`, and says:

> The rounding mode vocabulary is `toward_zero`, `floor`, `ceil`, `half_up`, `half_even`,
> `stochastic`.

`nearest` is not one of the six; it names two of them and says nothing about which. This is not my
finding, it is tracked: `question::which_tie_direction_an_unqualified_nearest_names` is open with
`decider = "panel"` and its `unblocks` field reads *"Two predicates whose rounding axis names no
member of the six, and which no other row tracks"*, with the note naming this row by slug.
`233_kiselyov_the_nine_rounding_entries_from_their_instruments` section 6 has already derived the
answer from the instrument, entry 4:

> `56_probes/q2_affine_membership.rs:73-80` states the tie rule in a comment, `ties toward positive
> infinity` [...] The entry wants to read `rounding = half_up` with the control in the note.

**That repair is derived and unapplied.** `233`'s own section 13, the addendum recording what the
mechanical rewrite actually landed, confirms it: *"both `nearest`
entries [...] whose trailing clause is intact. Section 6's verdicts apply to them as written."* The
row today names a value outside the ratified vocabulary. Cheap to fix and not fixed.

### 5.2 One clause of its `says` is false, and the counterexample is a format the shipped crate ships

The row says, with no quantifier:

> A half-step-biased grid is not closed under exact addition. No exact sum of two grid points lands
> on the grid, every one sits exactly half a step away, and the grid contains neither zero nor one.

Its predicate is `W = 4, F = 0, unsigned, add, arity 2`, and its instrument,
`56_probes/q2_affine_membership.rs`, ran at exactly one geometry. That probe's own run record says
so: *"All counts are at 4 bits (q1, q3) or one grid geometry (q2, step 1/4, bias 1/8, scale 2^5);
width transfer is argued, not probed."*

**The quantum is not on the predicate, and the quantum is what the last clause turns on.**

`236_probes/p1_the_phase_clause_over_three_geometries.rs`, committed with its output, writes one
affine membership predicate and instantiates it at three geometries in exact integer arithmetic
over 1/24, each checked against an enumeration built without consulting the predicate, each with
the bias-dropped mutant detected.

| arm | step | phase | closed under addition | contains zero | contains one | every sum half a step away |
|---|---|---|---|---|---|---|
| A, the measured geometry | 1/4 | 1/8 | no, 0 of 256 | no | no | yes |
| B | 2 | 1 | no, 0 of 256 | no | **yes** | yes |
| C | 1 | 1/3 | no, 0 of 256 | no | no | **no**, a third of a step |

Arm A reproduces `q2` on every count it reports. The case that had to fail was stated before the
run and is asserted in the file: `contains one` must differ between A and B, or the instrument
cannot separate two geometries and reports nothing; a phase-zero grid at each arm's own step must
contain zero, or the negative results are facts about the predicate; and the half-step clause must
fail at C, or the distance arm is structurally green. All three came out as required.

`236_probes/p2_the_shipped_crate_admits_the_counterexample.rs` compiles against `arvo-format` as
committed and closes the obvious objection, that arm B is a geometry nobody would declare. Every
arm is an instantiation of `arvo_format::points::Biased`, which is in the shipped inventory, pins
`PHASE_DEN = 2` so it is the half-step family by construction, and leaves the quantum exponent
free.

| arm | format | quantum | phase | contains zero | contains one | `has_additive_identity()` |
|---|---|---|---|---|---|---|
| A | `Biased<4, -2, 1>` | 1/4 | 1/8 | no | no | false |
| B | `Biased<4, 1, 1>` | 2 | 1 | no | **yes** | false |
| C | `Integer<4>` | 1 | 0 | yes | yes | true |

**So the clause "the grid contains neither zero nor one" is false at `Biased<4, 1, 1>`.** The zero
half holds at every arm and is the half the ratified spine carries; the "nor one" half is a fact
about a quantum below one and was written as a fact about half-step-biased grids.

**holds for:** `W = 4`, `signedness = signed`, `radix = 2`, `ambient domain = the binary
rationals`, quantum `2^1`, phase one half of a step. Two of those six are not declared axes, so
this finding **cannot be written as a registry predicate today**: there is no axis for the quantum
exponent above zero (`fraction_width` is *"how many bits of the declared format sit below the
point"* and a count of bits does not go negative) and none for the phase. **That is the same gap
the row it refutes has**, and it is the more useful half of this finding: a row whose sentence turns
on a coordinate the notation cannot express will keep reaching past its predicate no matter how
carefully the predicate is written. The additive repair is two `dimension` rows, and
`dimension.toml`'s own header says the set moves only on two independent readings, so that is work
rather than an edit.

**This is the refusal shape the gate already used once.** Narrow the sentence to the geometry it
was measured at, or declare the axes and measure the wider region, which under the predicate
discipline is a new claim in a new deliverable rather than a widening in place. Both are real work.

### 5.3 The standing is one, and the row says the two instruments are not two arrivals

> Its evidence is that two independent instruments erred at this coordinate in opposite directions,
> **which is a reason to name the coordinate rather than a second derivation of the claim.**

Nothing has changed that. The claim is at one arrival, and 5.2 is a refutation of part of it rather
than an arrival at it.

---

## 6. `raw_order_agreement_holds_for_monotone_encodings_not_only_unsigned`

**Not ratifiable.** Three blockers, and the second is the interesting one because the corpus
already contains its refutation.

**Blocker one: one arrival, and the only other file on the subject read it first.** The row's
`provenance` is one entry, `55_smith_the_format_concept_derived_cold::#3-one-refinement-to-08-from
-probe-3`. The row's own note concedes the shape: *"One probe, one width; offered as a refinement to
a wider claim rather than as its own general theorem."* The neighbouring file, `56_knuth_the_four
_choice_model_attacked` section 6.1, opens **"`55` refined `08`'s raw-order finding [...] I accept
the refinement"**, and its coverage section ends *"What I read and did not. `55` in full with its
probes."* That is a read, per `214`'s standard. What `56` adds independently is a different claim,
the uniqueness of the monotone bijection and the exclusivity of raw order against raw adder
correctness, and that one is already a live registry row of its own,
`law::no_bijective_signed_encoding_has_both_raw_order_and_raw_adder_correctness`, at
`signedness = signed, W any`, citing two probes.

**Blocker two: the row's third clause is denied by the next file in the corpus, which is not in its
provenance.** The row says it *"corrects an earlier claim that plain unsigned is the only
integer-keyed encoding with this property to a claim about that file's own pool of encodings."*
`56` section 6.1:

> `08`'s sentence was pool-scoped by its own words, "plain unsigned is the only one **of the eight
> integer-keyed encodings** where it holds" (`08:263-266`), so no quantifier in `08` needs
> correcting.

**There was nothing to correct**, and the row carries the corrected framing anyway, because `56`
is not among its sources. That is a defect the row's author could not have known about and a later
reader will inherit.

**Blocker three: its subject has no axis, so its region cannot be written.** The claim quantifies
over encodings. `dimension.toml` declares twenty-four axes and none of them is `encoding`. Its
predicate carries `W = 4` and `signedness = signed` and nothing else, so under I13's absence rule
the measured claim holds in no situation where a fraction width, an operation, an overflow policy
or a rounding mode exists, which is every situation a numeral is in. And it cannot be repaired by
adding `operation`, because `dimension::operation` says **"`operation any` is not admissible,
because `any` quantifies over a set nobody has closed"**, and a comparison is not one of the
operations the corpus's operation values name.

**One thing that is not a blocker.** The row is filed under `the_format` while the ratified
`a_format_is_identified_by_its_ambient_domain_and_its_representable_set` says encoding is
realisation rather than identity. Those do not conflict: the ratified row places encoding outside
*identity*, and `63` section 3.5 is *"The encoding: a second, ordered, behaviourally observable
axis"* of the format topic. The filing follows the consolidation. Worth a second look if the topic
is ever split, and not a reason to refuse the row.

**What would make it ratifiable.** Recover `56` into `provenance` and drop or rewrite the third
clause on `56`'s own argument. Declare an `encoding` axis, which is a `dimension` addition needing
two readings. Then the general clause needs its own honest filing: raw-order agreement holding for
exactly the monotone encodings is the definition of a monotone encoding unfolded, which is a
`theorem` rather than a `measured` claim, and what the probe actually measured is the instance,
that a signed value set has such an encoding and it is excess-K. The two want different rows.

---

## 7. Findings outside the question, stated harshly because the brief asks for them

### 7.1 The shipped crate carries two false statements about the phase, one in prose and one in code

`mock/crates/arvo-format/src/format.rs`, on `has_additive_identity`:

> A zero phase puts zero on the grid at slot zero, provided the slot range admits it. A nonzero
> phase takes it off, **and takes one off with it: every exact sum then lands half a step away from
> every grid point**, which is why the canon carries the coordinate rather than treating the bias as
> a corner case.

**Both halves of the bolded clause are false and both are refuted above.** "Takes one off with it"
fails at `Biased<4, 1, 1>` (p2 arm B). "Every exact sum lands half a step away" is a property of a
half-step phase specifically, and `Format::PHASE_DEN` is a free `i64` on the trait, so a phase of a
third is expressible and lands its sums a third of a step away (p1 arm C). The doc comment states
the wider "a nonzero phase" where even the proposal it is restating says "half-step-biased".

**This is the failure `the-canon-design-code-chain` names.** Nothing may appear in code that the
design does not say, and here the code restates an unratified proposal's overreaching clause and
widens it further. **The ratified ruling says none of this**; it says a nonzero phase decides
whether the identity adaptation occurs and whether the set carries an additive identity, which is
true and which every arm of both probes confirms.

### 7.2 `has_additive_identity` answers a different question than the one it is named for, and it is wrong on a format the crate ships

```rust
pub const fn has_additive_identity<F: Format>() -> bool {
    F::PHASE_NUM == 0 && slot_in_range::<F::Slots>(0)
}
```

The property is whether zero is on the grid. The test is whether the phase numerator is zero. Those
differ whenever the phase is a whole multiple of the quantum, because such a phase shifts the
lattice onto itself.

`p2` arm D is `arvo_format::points::Biased<4, 0, 2>`: quantum `2^0 = 1`, `PHASE_NUM = 2` over
`PHASE_DEN = 2`, so a phase of one whole step. Zero is on the grid, at slot -1, which the probe
computes from the crate's own exported coordinates. `has_additive_identity` returns **false**. Both
halves of the control are asserted: the lattice must not have moved, and the crate must disagree,
or there is nothing to report.

**And `PHASE_DEN` is read by no function in the crate at all.** `grep -rn 'PHASE_DEN'
mock/crates/arvo-format/src/` returns the trait declaration, four impls in `points`, one test impl,
and no use. Its doc says *"One for an unbiased grid, two for the half-step bias. Never zero."*
Nothing enforces any of that, and a `PHASE_DEN` of zero, three or seventeen compiles. **That is the
declarations-nothing-constrains class from `the-test-gate`, in its purest form: ask what value would
make it fail, and the answer is none.** The correct predicate is `PHASE_NUM % PHASE_DEN == 0 &&
slot_in_range(0)`, and writing it is what would make `PHASE_DEN` load-bearing.

**Neither of these is a design change and both are in scope for whoever next opens a source round
on `arvo-format`.** The design is right; the code disagrees with it.

### 7.3 The format topic's sentence-kind assignments carry a stated incentive and no audit

Section 3's second blocker generalises. `182` wrote every `sentence_kind` in this file, said in
terms that the `normative` label was the one with a thumb on the scale, and audited six rows, all
from the later consolidations. **The seven `the_format` rows were never audited under that
warning**, and at least one of them, section 3's, is a compound whose measured half is filed
normative and carries no region while the identical claim carries one in the `law` namespace. Two
of the seven are `measured` (`a_nonzero_phase...`, `raw_order_agreement...`) and both name a probe,
so the `measured-claim-cites-no-probe` ratchet is satisfied; it is the `normative` ones nobody
checked.

---

## 8. Verdicts

| row | verdict |
|---|---|
| `arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation` | out of scope, already canon under `ruling::the_format_spine_is_canon` |
| `a_format_is_identified_by_its_ambient_domain_and_its_representable_set` | out of scope, already canon under the same ruling |
| `membership_of_the_representable_set_is_one_affine_predicate` | out of scope, already canon under the same ruling |
| `the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation` | **not ratifiable**: one arrival with the second party conceding after reading, and a measured sentence filed `normative` with no region while `law::coherence_of_a_reduction_onto_its_induced_operation` carries the same claim with one |
| `the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws` | **not ratifiable**: one arrival, uncontested rather than corroborated by its own note, and two clauses by two authors welded into one row |
| `a_nonzero_phase_leaves_the_representable_set_without_an_additive_identity` | **not ratifiable**: predicate names `nearest`, retired by `ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`; the "contains no one" clause is false at `Biased<4, 1, 1>`, measured in `236_probes`; one arrival |
| `raw_order_agreement_holds_for_monotone_encodings_not_only_unsigned` | **not ratifiable**: one arrival with `56` conceding after reading `55` in full; its third clause is denied by `56`, which is not in its provenance; its subject has no declared axis |

**Nothing on the format layer is ratifiable today**, and the reason is one thing rather than four:
the topic was consolidated, ported and then left. Its spine was ratified by op himself in the round
that settled the ratification model, and everything under the spine has sat at one arrival ever
since, while three other topics got second reads and this one did not.

**What unblocks it is one dispatch, and `214` already specified the shape.** A cold derivation on
the format concept, dispatched at `55` through `62` with the registry withheld, deriving from the
premises before comparing. That is the only route by which the four rows can gain the instance they
need, because anybody who reads the rows first is a reader.

---

## 9. Bounds

**What I did.** Read the seven rows in full, every `ruling` row at `rung = "ratified"`, the
`dimension` roster, the two `question` rows that name these proposals, the fifteen `retirement`
rows on this topic, `63`, `64`, `55` sections 3 and 3.x of phase two, `55b`, `56` sections 6 and
coverage, `182` section 6, `214` sections 7 and 8, and `233` sections 6 and 13. Read the bodies of
four lints and both ratchets' module docs. Built two probes and committed them with their outputs
and a run record before citing either.

**What I did not do.** I did not read `57`, `57b`, `58`, `59`, `60`, `61` or `62` end to end; where
I attribute a claim to `60` it is through `63:570-577` and `64:250-254`, and both citations are
opened rather than remembered. I ran no bench and nothing here is priced. I did not re-run `q2`,
`q1` or `p3`; every count I attribute to them is transcribed from the file that reported it, except
`q2`'s, which `236_probes/p1` arm A independently reproduces.

**What my agreement is worth.** Nothing, per section 2. Where I agree with a row I am a reader and
I raise no standing. The two refutations in sections 5.2 and 7 stand on their own instruments with
their controls stated before the runs, and a counterexample needs no second instance; that is the
whole of what this file adds beyond a filing.

**The one thing I could not settle.** Whether `the_format_concept_carries_three_things_upward`'s
statability half belongs on `the_format` at all, given that `64` reads it as a chain sentence and
`63` derives it from `60`, the chain's cold derivation. Splitting the row forces the question and I
have not answered it.
