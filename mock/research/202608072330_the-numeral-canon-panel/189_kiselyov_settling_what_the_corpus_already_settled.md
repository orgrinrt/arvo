# 189. Settling what the corpus already settled

**Persona:** Oleg Kiselyov. **Subject:** the forty-five questions whose `decider` is `panel`, and the
thirty-two of them that nothing pointed at. **Position:** after `187` wired the cross-namespace edges
and after `188`'s entailment check landed, both of which this file depends on and neither of which it
repeats.

The dispatch asked whether the corpus already settles what the register records as open. It does, for
half of them, and the half it does not divides further into three shapes that want three different
remedies. This file is the mapping, question by question, with the citation where one exists and the
reason where none does.

Nothing here argues a question. Where I found no answer in the corpus I wrote that I found none, which
is a complete result and is the one thing this dispatch could produce that a later reader must be able
to trust. The temptation in a dispatch shaped like this is to reason a row into existence so the count
moves, and a row that reads as an answer while restating its question is worse than the empty cell it
replaced, because the cell announces itself and the row does not.

## 0. The gates

**Canon gate: passed.** Measured against op's own words rather than against a summary of them.
`ruling::port_what_exists_into_the_registry_then_comb_the_rest` reads: "The first work is to port
every current result, agreement, convergence and settled thing into the registry and shape the
registry and its metadata so it works, and then to comb all the rulings, proposals and statements."
Combing the statements for what settles a recorded question is the second clause of that sentence.
Nothing here writes canon: a `proposal` is a claim op has not seen, and
`ruling::the_canon_is_written_once_at_the_end` keeps it that way.

**Test gate: passed, and it found one thing.** I ran the whole suite before starting, read the bodies
of the arms I depend on in `mock/checks/src/shape.rs` rather than their names, and then made each of
them fail on purpose. `60 passed, 1 ignored` before, the same after. The bodies are real: every arm
has both directions planted and `the_two_correct_shapes_are_both_silent` exists specifically to catch
an arm that reports the rule rather than a breach of it.

**What it found: the one ignored test is ignored for a reason that is now false.**
`the_committed_canon_has_an_instrument_behind_every_measurement` carries
`#[ignore = "catalogue: the probe namespace is empty and is being written, so no row can name an
instrument yet ... Remove this attribute once probe.toml has rows and the citing proposals point at
them."]`. `probe.toml` carries forty-eight rows. Run with `--ignored` the test fails, naming
twenty-one existing `measured` and `enumeration` rows that cite no probe at all. The stated blocker is
gone and the reason misdescribes the state; the test file is not this dispatch's to edit. The
twenty-one are listed in section 6, because they are the reason two of the three rows I wrote say less
than the corpus knows.

---

## 1. The headline

Re-run of `187_probes/coverage.sh`, unmodified, from the same worktree, after everything below landed.
Committed as `189_probes/coverage_after.out`.

```
$ 187_probes/coverage.sh

######## question, by decider
  decider         total  answered  proposed   open
  coordinator         3         0         0      3
  measurement         3         0         0      3
  op                 27         4         0     23
  panel              45         0        29     16
  ALL                78         4        29     45
```

**Sixteen of the thirty-two moved.** The panel row goes from `13 proposed / 32 open` to
`29 proposed / 16 open`, and the `ALL` open count from 61 to 45. Nothing moved out of `open` into
`answered`, correctly: `answered` means a ruling names it, and this dispatch may not write rulings.

**Three of the sixteen moved on rows written here. Thirteen moved on rows that already existed and
had no edge.** That ratio is the finding underneath the count. The panel's answers were mostly already
in the registry, sitting one field away from the questions they settle, and what was missing was not
work but a link.

`189_probes/open_panel_questions.sh` prints the population by name, so the cell above and the list in
section 4 cannot drift apart. Before and after outputs are committed beside it.

---

## 2. The rule I applied, stated before the results, because it decides half of them

An `answers` edge means, per `mockspace.toml` on the field, "questions this would settle if it were
stamped". That leaves a real judgement on every partial, and a dispatch that decides each one by feel
produces a table nobody can check. So I took the rule from the registry rather than from taste, by
reading what the edges already there do:

- `the_model_band_transfer_is_defeated_in_both_fragments` answers `where_a_law_verdict_is_established`
  by killing one of its three options and picking none.
- `the_corpus_cannot_exhibit_the_accuracy_intents_because_a_coordinate_is_absent` answers
  `what_the_cost_coordinates_are` by settling the premise its options are argued from.

**So: wire where stamping the row would remove an option from the question's own list, or would answer
the discriminator the question's own `note` names as what would distinguish them. Otherwise report the
partial and write no edge.**

That rule is why Q23 and Q44 below stay open with real corpus material against them, and why Q11 and
Q41 move on rows that pick no winner. It is applied identically to all thirty-two and I have said, per
question, which limb of it fired.

---

## 3. The sixteen that moved

Each line: the question, the row now pointing at it, what the row decides, and what the question still
asks.

### 3.1 Written here, because the answer existed and no row carried it

**`container_derivation_output_count` <- `an_output_of_a_derivation_is_a_fact_a_downstream_site_cannot_recover`.**
`47` separates three questions the panel had been running as one and shows the one-against-two fork is
not falsifiable as posed, since any product is one thing and anything with two projections is a pair
(`47:62`). With `16`'s own criterion for what counts as an output, which quantifies over the
observation surface rather than the codomain, the two candidate answers stop being rivals: a single
richer output with named projections is the pair wearing one name (`47:180`). `187` section 6 named
this as a row somebody had to write. **Still asks:** how many facts must be types, which `47` bounds
below at two and does not close.

**`does_narrowing_compose` <- `staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even`.**
The one instance whose instrument is registered: at `W = 9`, `F = 4`, staged from four fraction bits
through two to zero, half-even's direct and staged results disagree, first at `-247/16` where direct
gives `-15` and staged gives `-16`. So wanting composable narrowing costs the default rounding mode.
**Still asks:** whether the design wants it, which is a preference and is untouched. See section 6 for
the general rule, which is in the corpus and is not writable.

**`which_route_a_law_verdict_takes_to_closed_form` and `how_the_law_inventory_is_named` <-
`a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains`.**
`97:705` derives it: where the realisation map respects every ordered nesting a law contains, the
representable set under the induced operations is a quotient of exact arithmetic, and a quotient
inherits every identity its source satisfies. `97` states the consequence in the question's own words:
`OPTIONS.md`'s entry lists "(b) A structural argument about the representation" and says no file has
asked it of any law it measured, and "this is that route in general form". The retraction table is
finite at one entry per ordered operation pair while the space of identities is infinite at every
arity, so the finite table decides the infinite family. This is `188` section 4.1's first restoration
and I confirmed it independently before reading that section's conclusion. **Still asks, for Q40:**
nothing that I can see; the three routes are named and the criterion says which laws take which.
**For Q25:** which form the canon should describe, on which see section 5.

### 3.2 Moved on rows that already existed

| question | row now pointing at it | which limb fired |
|---|---|---|
| Q2 `which_width_coordinates_a_consumer_writes` | `membership_of_the_representable_set_is_one_affine_predicate` | option 4's premise, that the numeral is one object under one parameterisation, is established: a slot function, a quantum per magnitude and a phase, of which integers, fixed point, scaled integers and floats are points |
| Q11 `what_a_numeral_guarantees_to_a_fold` | `the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion` | removes option 3 as stated: the row's own last sentence is that any accumulator statement derived from a capacity is an additive-only mechanism, so a capacity-keyed relation cannot serve the multiplicative fold |
| Q12 `reduction_order_or_associativity` | `within_an_unbound_stretch_the_design_may_select_any_realisation`, `no_cost_model_may_move_an_answer` | the first removes option 2 by making the boundary function rather than the tree the thing fixed; the second removes option 4, since letting the answer follow the core count is a target fact moving an answer |
| Q13 `which_axes_a_build_arm_may_move` | `headroom_and_intermediate_precision_are_unobservable_inside_a_pure_ring_region` | refutes the premise every option shares, that observability is a property of the axis: the same two axes are unobservable at 0 of 640 inside ring chains and observable at 500 and 570 of 640 past a non-ring step |
| Q18 `adaptation_in_identity_or_realisation` | `a_format_is_identified_by_its_ambient_domain_and_its_representable_set` | option 2 verbatim: "Adaptation choice and encoding are realisation ... and not part of identity" |
| Q24 `does_the_canon_speak_about_cost` | `meaning_is_decided_by_the_first_three_coordinates_and_cost_by_the_last_two` | removes option 2: a canon carrying that sentence is not silent on cost |
| Q25 `how_the_law_inventory_is_named` | `a_law_stated_as_an_author_written_marker_is_checked_by_nothing`, `a_law_is_read_off_the_algebra_and_never_declared` | removes option 1 with a measurement: two policies declare one associativity marker, one declaration is false, the compiler raises nothing, and the licensed consumer returns a different answer on 16,268 of 65,536 vectors |
| Q33 `is_the_ambient_operation_family_fixed` | `the_numeral_concept_is_a_dependent_sequence_of_choices`, `a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts` | option 2: the ambient domain is the sequence's first choice, and a system exposes that domain together with that domain's own law inventory, so the family is per-system |
| Q37 `does_the_canon_name_crossing_classes` | `a_crossing_carries_two_relations_and_a_verdict_per_law_family`, `meaning_is_decided_by_the_first_three_coordinates_and_cost_by_the_last_two` | option 1: one class per coordinate, five falling out of the sequence, and the question's own distinguisher, whether any canon sentence quantifies over one class and not another, is met by both rows |
| Q39 `may_an_arms_predicate_read_data` | `a_trajectory_condition_lifts_into_a_declaration_exactly_when_it_survives_closure` | answers the discriminator the note names, whether any measured trajectory predicate has a lifting a consumer would write, and answers it against: the maximal lifted box is 21.98% of the holding set, and the declaration a consumer would naturally write is unsound, true while the law is false on 49.80% of the domain |
| Q41 `are_strategies_partially_ordered_by_chain_laws` | `no_total_join_exists_over_the_observable_axes_so_the_operation_reports` | removes option 1: by honoured laws wrapping and saturating are incomparable in three of four configurations, so the honoured sets do not nest. The row already carries `Q41` in its own `keywords` and had no edge |
| Q50 `is_i9_about_a_strategy_or_one_component` | `no_cost_model_may_move_an_answer` | option 2: every difference in an answer traces to the declared policy and nothing else may move one. Read section 5.4 before quoting this one |
| Q53 `what_transfers_from_a_model_width` | `the_model_band_transfer_is_defeated_in_both_fragments` | option 2 with the mechanism: two independently constructed families place a law's truth flip exactly at the shipped width, where no model band at any guard setting could have disagreed, and the row's own `instead` says that outside a fragment with a complete test set there is no established route |

Two of these deserve a word about what they do not do. **Q41's row measures three candidate orders and
finds two of them are not orders at all**, so the question's third option, that there is no order,
survives beside its second; the row removes only the refinement reading. **Q13's row is unpriced and
unbuilt by its own `gap`**, so what it settles is the classification question and not whether the
licence is worth taking.

---

## 4. The sixteen that did not move, and why each did not

Three shapes, and they want three different things done to them.

### 4.1 Settled in the corpus, unwritable here (four)

These have an answer, the answer ran an instrument, and the instrument has no `probe` row. Section 6
carries the detail. Naming the sentence and marking it `argument` to get past the evidence gate is the
dodge that gate exists to catch, so none of them is written.

- **Q15 `are_the_axes_independently_resolvable`.** Measured at `40_probes/p7`: the set of containers in
  contention differs between wrapping and saturating at five of six widths, on two matched committed
  families nobody had cited, so the axes are independently stateable and not independently resolvable.
  Option 1 is refuted and nothing carries it.
- **`is_the_derived_numeral_required_to_be_tightest`.** Three measured facts price all three options
  exactly: the sum-of-widths product form wastes one bit on a characterised minority, 476 pairs in one
  box reconciled against an earlier 461 as two conventions; a tight form exists whose predicate reduces
  to a one-line condition, compiled with a negative control at `06` sections 7.1 and 7.3; and the two
  admissions the tight form needs are disjoint regions of one formula's codomain, needed by 1 input and
  5,487 inputs with 0 needing both, at `07` section 3.1.
- **`is_the_cross_kind_join_closed_or_priced`.** The closure's price is measured at `07` section 4.3,
  roughly sixteen to thirty-four percent more shapes than the two named families contain, and `08`
  sharpens it by showing the closure specifically does not reach tapered formats.
- **`container_derivation_output_count`'s measured half**, which is why that row says less than `47`
  does. Listed here as well as in section 3.1 because the question moved on the half that was writable.

### 4.2 Genuinely open (six)

Nothing in the corpus reaches these. I looked and I am saying so.

- **Q4 `what_a_datum_stands_for`.** Op refused to bound the option set. The later topics use the word
  `interval` throughout for a range analysis over terms and not for a set-valued datum, so the
  apparent hits are a different subject. Nothing addresses the absorbing-top denotation.
- **Q6 `does_warm_wrap_or_clamp`.** `ruling::wrap_or_clamp_stays_open_and_both_get_priced` says in op's
  own words that it is not settled and that both readings get priced. Correctly open, and see 5.3.
- **Q22 `are_set_valued_carriers_admitted`.** The format topic scopes compositions out of the format
  concept and the number-system topic's admission rows make admission a contract rather than an
  enumeration, but neither reaches the question's own discriminator, whether certified accuracy should
  be expressible inside the system concept or beside it.
- **Q28 `arbitrary_width_demands_in_the_canon`.** `65` deliberately did not probe the general
  width-to-container projection and rests its pipeline claim on width-as-marker. The question's own
  note says it needs its own evidence, and no later file supplied any.
- **Q54 `are_consumer_terms_trees_or_dags`.** `119` C4: "Two files now depend on it and neither has
  looked." `114` section 5.4 names it as the assumption it is least comfortable with and `115:409`
  names it as what would decide its own section 5 against itself. Both name it and neither measures it.
- **Q34 `two_shapes_of_aggregate_composition`.** Two topics reach it from incompatible framings and
  neither addresses the other. See 5.5; I am filing it as open rather than as contested because the
  two may be a naming collision rather than a disagreement, and which it is I could not determine.

### 4.3 Partly settled, and the corpus answers a narrower question than the row asks (six)

For each: the narrower claim already exists as a row, and stamping it would not settle the question, so
under section 2's rule no edge is written. The value here is knowing what remains.

- **Q23 `is_the_role_set_closed`.** `roles_derive_representations_and_a_realisation_variant_computes_nothing_new`
  removes the one candidate fourth role the option names: "a chain's extent is not a role at all but a
  schedule". The option itself survives, because it anticipates roles nobody has named, and the row's
  own `gap` puts the homogeneity question back on the table. `187` called this a partial with nowhere
  to go and I agree with the diagnosis and with the conclusion.
- **Q26 `what_a_platform_width_type_is`.** Two rows reach it and give different answers.
  `a_format_is_identified_by_its_ambient_domain_and_its_representable_set`: "a value set that depends
  on other data is not a format but storage", which is option 1.
  `each_choice_in_the_sequence_has_an_owner_and_a_resolution_time`: "a platform-width numeral is a
  target-indexed family of formats whose exclusion grounds apply only to dependence that survives to
  runtime", which is none of the four options. **The option set is incomplete**, and I may not extend
  it. Whether the two rows conflict turns on whether target-indexing counts as depending on other data,
  which neither says.
- **Q44 `must_a_weighting_be_strictly_positive`.**
  `the_rationalisability_counts_on_the_committed_carrier_table` settles the empirical premise all four
  options argue from, that the whole 72-against-9 gap is one tie between two arms declaring the same
  size, reproduced by three independent implementations. It picks no option, and the fourth option, a
  unique-argmin rule, is what the note argues the tie result points at without anybody proposing it.
- **Q45 `arms_no_weighting_can_select`.** Option 2's rescue was built, reported and then withdrawn, and
  the instrument carrying the withdrawal is `probe::the_pareto_arm_that_rests_on_one_size_point`, whose
  `standing` is `withdrawn`. So the refutation of option 2 rests on an instrument no measured row may
  cite. Separately `probe::the_tail_coordinate_is_the_wrong_statistic`, which is sound, establishes
  that three negative findings about that coordinate share one estimator and are therefore one finding.
  **Net: option 2 is neither established nor refuted by usable evidence**, which is a worse state than
  open and is worth saying in those words.
- **Q36 `whose_reduction_governs_a_lossy_crossing`.** Refused by the row that looks like its answer.
  `an_order_is_named_exactly_where_a_crossing_is_lossy` states both candidate orders and that the
  typestate cannot break the tie, and its `gap` says the question "is unresolved and is op's". `187`
  hit this trap, read the `gap`, and did not wire it. I read the `gap` first because `187` said to, and
  confirm the refusal.
- **`does_the_canon_carry_a_numeric_threshold`.** Both files stating the second reading mark it as a
  reading. What the corpus does have is a structurally identical finding in a different subject, at
  `a_const_eval_frontier_is_a_fact_about_the_procedure_rather_than_about_the_law`: a frontier is a
  function of domain size, per-tuple cost and procedure, and one cited without all three is a defect,
  with the neighbouring row's `gap` demonstrating it ("no host, no toolchain pin and no optimisation
  level, so every timing here is a fact about one unnamed machine"). That is the inequality-over-figure
  reading established rather than mused, about a different threshold. Nobody has connected them and I
  am not connecting them by edge, because the subjects differ.

### 4.4 Not the panel's, on the panel's own account (one)

- **`should_phase_collide_across_two_vocabularies`.** See 5.2.

---

## 5. What else this turned up

### 5.1 Questions op has already answered, recorded as the panel's

The brief asked for these specifically. Three, and none of them can be fixed by an edge, because an
`answers` edge on a ruling means editing `ruling.toml`, which this dispatch may not do.

**`how_the_law_inventory_is_named` (Q25) is the shape op has refused.** He asks the canon not to
police it: `ruling::the_canon_does_not_police_what_shape_a_law_takes` reads "If a law is a law it
should be expressed so that it actually works, be it typestate or const expressions or whatever. What
kind of laws there are and what shapes they take is not to be policed; a law is defined as makes sense
and is applicable case by case." Q25 asks which single form the canon names, over the whole category
of laws, which is the pattern `ruling::there_is_no_universal_answer_take_the_win_and_gate_it` names
and which `never-ask-which-single-rule-governs.md` says is the wrong question rather than an open one.
**The proposals I wired to it stay right and their target may be wrong.** They establish that
author-written markers are unsound and that verdicts are inherited from a finite table, which are
findings about the design rather than instructions to the canon, and both survive op's refusal intact.

**`reduction_order_or_associativity` (Q12) has its fourth option removed by two of op's rulings.**
Option 4 is "Say nothing and let the answer depend on the core count, which under the soundness
condition is a sacrifice for every strategy except the one whose purpose is to make it."
`ruling::adaptation_is_conditional_on_proof_and_on_soundness`: "arvo is multi-threadable wherever that
is proven to improve performance without sacrificing soundness. Two conditions, not one." And
`ruling::hot_may_sacrifice_soundness_for_a_proven_meaningful_gain`: "The soundness side condition
holds for every other strategy." The option's own parenthetical is calibrated to exactly those two, so
the register knew and the registry cannot see it.

**`may_an_arms_predicate_read_data` (Q39) has two of its three options closed by op.** Option (b), a
value-gated arm, is closed on principle by `ruling::never_a_runtime_check_and_one_lowered_path`, which
`AGREEMENTS.md` section 4.4 records as one of two kills ("killed twice: measured worse than either
static arm, then closed on principle by I15"). Option (c), data permitted at a declared ingest
boundary, sits against `ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up`: "Data
entering from outside the program is the consumer's boundary, not arvo's." And option (a)'s
description of itself as the predicate intent read literally is stale under
`ruling::the_predicate_is_whatever_is_available_at_const_time`, which the question's own note records
and which nobody amended the option for. `187` declined to wire that last one on the ground that `179`
says "bears on", which is right about that ruling alone and understates what the three together do.

### 5.2 Deciders I believe are wrong

**`should_phase_collide_across_two_vocabularies` carries `decider = "panel"` and the panel files
naming collisions as op's.** `AGREEMENTS.md` section 13, on what topic ten did not settle: "and all of
it is op's: ... two vocabulary calls". And `the_numeral_concept_is_a_dependent_sequence_of_choices`'s
own note, on the structurally identical case: "The word `format` collides here: two units use it for
different prefixes of this same sequence, and the collision is unresolved and is **op's naming call**."
A vocabulary collision between two of the panel's own senses of one word is the same object in both
places. The row also carries no `topic`, correctly, and that absence is a second signal: it is not
about anything the panel is deciding.

**`whose_reduction_governs_a_lossy_crossing` (Q36) carries `decider = "panel"` and the row that
reaches it says op's.** Quoted in 4.3. `187` noted the same and said reconciling it was not its call.
It is not mine either, and two files now saying so is enough to act on.

**`does_warm_wrap_or_clamp` (Q6) carries `decider = "panel"` and op has ruled on it.** He did not pick
a side, and what he did is still a call: both readings are explored and both are priced. A question
whose disposition op has fixed is not one the panel settles by argument.

**Three canon-form questions may be one question of op's.** `does_the_canon_speak_about_cost` (Q24),
`arbitrary_width_demands_in_the_canon` (Q28) and `does_the_canon_carry_a_numeric_threshold` all ask
what kind of sentence the canon may carry. `where_a_law_verdict_is_established`'s own note says its
distinguisher "is itself a canon-form question and is coupled to the coordinator's item 2, which asks
what kind of sentence the canon may carry", and `AGREEMENTS.md` section 13 lists "the canon-form
question, coupled to `156` item 2 as one decision" among what is op's. I wired Q24 anyway, on the
narrow ground that a row which is itself a canon sentence about cost removes the silence option
whatever the wider decision turns out to be. If the coupling is real, all three go to op together and
my Q24 edge is answering the smaller half of a question that has a larger half.

### 5.3 Two questions whose `asks` no row can reach

The brief predicted these and I met two.

**`does_narrowing_compose` asks whether the design *wants* narrowing to compose.** Its options are
about what wanting it costs. So a row establishing the mathematics matches an option's content exactly
and leaves the `asks` untouched, which is what the row I wrote does and what its `gap` says.

**`does_the_canon_speak_about_cost` (Q24) asks a decision about the canon.** Its options are a state of
the canon. A row that is itself a canon sentence about cost demonstrates option 1 rather than choosing
it. Same shape.

Neither is a defect in the rows and I am not proposing the `asks` be changed. It is worth knowing that
for these two, a `refsto()` hit means the option is available and not that the question is decided.

### 5.4 Three rows define what fixes an answer and they disagree, with nothing recording it

This is the sharpest thing I found and it is not fixable inside this dispatch's write permissions.

**One.** `a_strategy_is_a_pair_of_an_observable_assignment_and_a_weighting` says component two
"selects among the arms that produce the answer the first component fixed". Its own `gap` says: "Clause
three is false and it is the one that mattered: component two was defined as ranging over the arms that
produce the answer the first component fixed, and in that region a fidelity column would measure a
constant, so op's accuracy intent is expressible in neither component while the same file claims the
mechanism serves it."

**Two.** `a_strategy_is_an_assignment_and_a_weighting`, from a later topic, restates that clause
unqualified: "Component two is a weighting over cost coordinates and it ranges over realisations of
that denotation." It carries `standing = "two_experts"`, it has no `gap`, and nothing in it records
that the clause it restates is one another row calls false.

**Three.** `a_strategy_is_a_declared_semantics_together_with_a_weighting_over_the_arms_that_realise_it`
repairs it in the opposite direction: "An arm realises the declared semantics exactly or approximately,
and the distance from the declaration is a cost coordinate like any other, which is what lets a
strategy weigh accuracy at all." If the distance is a coordinate the weighting weighs, the weighting
reaches accuracy, which is part of what the correct answer is.

**And a fourth row forbids exactly that.** `no_cost_model_may_move_an_answer`: "Every difference in an
answer traces to the declared policy and nothing else may move one." Its `instead` knows about the
approximate-realisation case and resolves it by constraining the arm set rather than by letting the
weighting reach the answer, which is a different mechanism from row three's and is incompatible with it.

**What I did about it.** I wired `no_cost_model_may_move_an_answer` to Q50 and **withdrew the edge I had
already written from `a_strategy_is_an_assignment_and_a_weighting`**, because that row's load-bearing
clause is one another row records as false, and reporting Q50 as proposed-against on the strength of it
would launder the disagreement into a settlement. The withdrawal is its own commit so the sequence is
visible.

**What is owed and is not mine.** A `standing = "contested"` on the pair, or a `supersedes` from row
three to row one, or both. Every one of those edits a `standing` or a `says`, which this dispatch may
not touch. Until it happens, a reader taking Q50 as proposed-against gets one row's answer and no sign
that two others disagree.

### 5.5 A cross-topic tension the AGREEMENTS ledger's scope does not cover

`the_four_consolidations_contradict_each_other_nowhere` is true and is about the four consolidations.
The composite question sits outside them.

`the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws`, from topic one:
"Compositions over formats, stored pairs, intervals, error-carrying values, are not format instances;
they consume these three things and owe their own laws."

`configuration_is_not_composition_and_a_composite_is_a_primitive`, from topic five reopened as the
ninth: "a composite is a primitive under the same definition, so one concept serves and every contract
written for a primitive applies to a composite unchanged."

Those are opposite answers if `format` and `primitive` name one object, and compatible if they name
different prefixes of the choice sequence. The registry already records that the naming is unresolved
and is op's. **Until it is resolved, Q34 cannot be answered**, because its two options map onto the two
concepts, and that is why I filed Q34 open rather than contested: I could not determine whether there
is a disagreement to record.

---

## 6. Where a claim was settled and could not be written

Eight places. In every one the blocker is the same and it is not an axis: **the instrument is committed
in the panel and no `probe` row names it**, so a `measured` or `enumeration` row is refused by
`measurements_resting_on_an_unusable_instrument` or by `measured_without_evidence`.

| claim | instrument, committed | question it would move |
|---|---|---|
| the axes are not independently resolvable | `40_probes/p7` | Q15 |
| the product form is not tight, and the tight form's two admissions are disjoint | `06_probes` sections 7.1 and 7.3, `07_probes` section 3.1 | `is_the_derived_numeral_required_to_be_tightest` |
| the cross-kind closure costs sixteen to thirty-four percent more shapes and does not reach tapered formats | `07_probes` section 4.3, `08` | `is_the_cross_kind_join_closed_or_priced` |
| a single richer output suffices if and only if it is a type; the value spelling is refused six times in three syntactic positions | `47_probes/p1`, `p1b`, `p2`, `p2b`, eleven files, five negative controls, `verify.sh` | `container_derivation_output_count`, the half not written |
| narrowing composes exactly when the mode's direction switches only at coarser-grid points, at zero failures on-grid and seven off-grid | `07_probes/p4_composition_and_forced_adjoint.py` | `does_narrowing_compose`, the general form |
| the criterion is necessary as well as sufficient, at zero conservative mismatches over 552 cells | `97_probes/p2_congruence_predicts_the_laws.py` | Q40, the other direction |
| option 2's rescue for a dominated arm | `144_probes/p10b`, `p10c`, and the probe row is `withdrawn` | Q45 |
| the corpus's own 21 existing `measured` rows | various, all committed | not a question, listed below |

**One axis blocker, and it is the one `190` warns about.** `a_law_is_inherited_where_...` names
`operand window in {full, declared non-negative}` in its source region and no `dimension` row declares
that axis. The sweep covered both of its values, so the written predicate is narrower than what was
established, and under the absence rule it says the claim holds nowhere a declared operand window
exists. That is stated in the row's own `note`. `190` says `declared_operand_window` is still not
declared and that the four axes which landed came from a different corpus, so this stays blocked and I
have not treated it as newly writable.

**The twenty-one rows the ignored test names**, which is the population section 0 refers to, and which
is why the two rows I wrote carrying measured content say less than their sources:
`a_coherent_reduction_needs_no_accumulator`,
`a_compile_time_strategy_selection_leaves_no_residue_in_the_emitted_body`,
`a_coordinate_set_is_a_countable_ceiling_on_how_many_strategies_can_exist`,
`a_law_stated_as_an_author_written_marker_is_checked_by_nothing`,
`a_nonzero_phase_leaves_the_representable_set_without_an_additive_identity`,
`a_trajectory_condition_lifts_into_a_declaration_exactly_when_it_survives_closure`,
`absorption_decides_associativity_of_a_clamped_reduction`,
`an_exposure_test_over_reduction_verdicts_alone_is_satisfied_by_a_system_that_computes_nothing`,
`an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit`,
`chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type`,
`generation_relocates_the_check_rather_than_removing_it`,
`headroom_and_intermediate_precision_are_unobservable_inside_a_pure_ring_region`,
`most_committed_bench_regions_predate_the_harness_cross_variant_validation`,
`no_total_join_exists_over_the_observable_axes_so_the_operation_reports`,
`the_const_eval_frontier_collapses_along_arity_and_buys_three_bits_from_the_guard`,
`the_corpus_cannot_exhibit_the_accuracy_intents_because_a_coordinate_is_absent`,
`the_four_consolidations_contradict_each_other_nowhere`,
`the_licensed_category_is_const_available_and_four_constructions_bind_at_four_times`,
`the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion`,
`the_rationalisability_counts_on_the_committed_carrier_table`,
`where_fusion_changes_the_answer_it_is_not_a_lowering`.

**Four of the sixteen edges I wrote land on rows in that list.** Q11, Q13, Q25 and Q41 are now
proposed-against by rows whose own evidence gate is red. That is not a reason to withhold the edge, and
it is a reason to say so here: the questions moved on claims whose instruments the registry cannot
resolve, and filling `probe.toml` for the earlier topics is what would close that.

---

## 7. The control runs

Full transcript in `189_probes/control_runs.txt`, committed before any row was written. Three planted
cases and one that planted itself.

**A `measured` row citing a defective probe.** Planted `control_row_planted_by_189_delete_me` citing
`probe::the_width_invariance_control_was_toothless`, whose `standing` is `defective`. Reported:

```
test the_committed_canon_rests_no_measurement_on_an_unusable_instrument ... FAILED
  at:   "proposal::control_row_planted_by_189_delete_me"
  kind: "measurement-rests-on-an-unusable-instrument"
```

**A predicate naming an undeclared axis.** The same planted row carried `moon_phase: moon phase any`.
Reported:

```
test the_committed_canon_names_only_declared_axes ... FAILED
  kind: "predicate-names-an-undeclared-dimension"
  says: "`predicate` names the axis `moon_phase`, which no `dimension` row declares."
```

Both put back, `git diff --stat` empty, suite green again.

**And the thing the controls found that nobody planted, which is worth more than either.**
`cargo mock --lint-only` **passed the planted row.** It printed `schema check passed` and
`all lints passed` with a defective-probe citation and an undeclared axis sitting in the file. The
engine's schema check enforces field presence, types and reference resolution and enforces none of the
three rules this dispatch was told to respect. **So `cargo mock --lint-only` is not a gate on a
proposal row's substance and must not be reported as one.** The whole of that enforcement is
`cargo test -p arvo-checks`, and a later dispatch running only the first will believe it validated
something it did not look at.

**The worklist probe's own control.** `189_probes/open_panel_questions.sh --control` plants an edge at
`which_width_coordinates_a_consumer_writes` and that row must vanish from the printed list. It did.
Without that, a list of thirty-two is indistinguishable from an awk rule matching the wrong field.

**The mapping probe's control.** `189_probes/who_answers_what.sh --control` asserts one edged question
reads non-empty and one unedged question reads empty, so a script that cannot tell them apart fails
rather than printing a plausible table. Both fired the right way.

---

## 8. The coordinator's three items

Sent mid-dispatch, additive. Taken in order.

**1. `97`'s F-F.** Taken, and it is section 3.1's third row. I had reached `97` section 6.1
independently while working Q40, because Q40's own note says the lifting route has never been asked of
any law the panel measured and `97` says in its own words that this is that route in general form. The
coordinator's account of the three lost links is accurate and I did not re-verify links one and two,
which are `188`'s measurement rather than mine.

**One correction to how it was described.** The row is written as a `theorem` over the sufficient
direction, not as the biconditional. `97` derives the criterion from the quotient construction and the
instrument states it before running, so the sweep is a control on a prediction; but only the direction
"criterion holds, therefore the law holds" is proved, and the converse rests on zero conservative
mismatches over the 552 cells with no `probe` row to cite. Writing the biconditional as `measured`
would have needed an instrument the registry cannot resolve, and writing it as a theorem would have
claimed a proof of a direction nobody gave. The `gap` says which half is which.

**2. The two `law.toml` rows claiming a cell nobody swept.** **Not taken, and it is not a collision, it
is a permission.** This dispatch may write `proposal.toml`, `proposal-the-later-topics.toml` and
`answers` edges, and `law.toml` is outside that. The finding reads correct to me on its face and I did
not verify it, because verifying a repair I may not make spends budget on nothing. It is yours.

**3. `a_law_layer_answers_whether_a_law_reaches_a_lowering_the_backend_cannot_prove` has no region.**
**Correct, and it must not get one while it stands as it is.** Its `sentence_kind` is `normative`, and
`predicate_disagrees_with_the_sentence_kind` reports an imposed proposition carrying a region, on the
stated ground that a region on one says the design may violate it everywhere the region does not reach.
So adding the three axes needs the `sentence_kind` changed first, which is a `says`-tier edit and
outside this dispatch. If the row really is a measured claim wearing a normative mark, that is a
finding about the row rather than about its predicate, and it should be raised as one.

**4. The instruction not to predicate `a_composed_expressions_region_is_never_inherited_from_its_parts`.**
Followed. I did not touch it.

**5. `190`'s correction about the four blocked axes.** Absorbed, and it changed one thing: I treated
`declared_operand_window` as still undeclared and wrote the F-F row's region without it rather than
looking for it in the new four. Section 6 records the consequence.

---

## 9. Coverage, stated honestly

**Read end to end:** `question.toml` for all forty-five `panel` rows, `proposal.toml`,
`proposal-the-later-topics.toml`, `ruling.toml`, `probe.toml` at the level of id, standing and control,
`dimension.toml` including its header, `topic.toml`, `obligation.toml`, `mockspace.toml`'s registry
schema, `mock/checks/src/shape.rs` and `citation.rs`, `187` in the sections it bears on, `188` sections
4.1 and 4.2, and `AGREEMENTS.md` sections 2.4, 3.4, 4.4, 5.5, 6, 7 and 13.

**Read at the passages I cite, by opening them:** `07` sections 2.4 and 8 item four, `47` sections 1,
2.1 through 2.4, 11 and 12, `97` section 6.1 and 6.2 and its F-F statement, `104` sections 1 through 4,
`119` section 1.2 item C4, `136_probes/x1` header and output, `OPTIONS.md` at Q15, Q22, Q24, Q25, Q28
and the two entries on narrowing and tightness. Every `file:line` in this document was opened and
checked against the claim it carries, and one of them moved: my first draft cited `07:374` and `07:383`
for the narrowing rule, and opening them found 374 blank, so the row cites `07:370` and `07:377`.

**Not read:** `02` through `06`, `08` through `46`, `48` through `96`, `98` through `103`, `105`
through `118`, `120` through `186` except where cited above, `DROPLIST.md`, `PRIOR_CALLS.md`,
`PERSONA_CALLS.md`, `SEED_*`, `archive/`, `seed/`, `mock/benches/`, `retirement.toml`, `law.toml` and
`law-the-later-topics.toml`.

**What that last omission costs, and it is real.** I did not read `retirement.toml`, which is the
must-not-cite list. **A claim I wired could be a retired one**, and nothing in my process would have
caught it. The three rows I wrote cite `07`, `47` and `97` at specific passages I opened, so those are
checkable; the thirteen edges rest on existing rows whose own authors did read the retirements. Someone
should run the retired-claim keywords against the sixteen questions, and it is one grep.

**Not verified:** every figure I quote from an existing row, which I took from the row rather than from
its instrument. `16,268 of 65,536`, `21.98%`, `49.80%`, `0 of 640` and `500 of 640`, `72` and `9`,
`476` and `461`, `1` and `5,487`, `552`: all of them are the rows' own or `OPTIONS.md`'s own, quoted to
say which claim is doing the work rather than to assert the number. The three I opened at source are
`97`'s 552 with both mismatch directions, `07`'s 0 on-grid and 7 off-grid, and `136_probes/x1`'s
`-247/16` witness.

**No bench harness ran and nothing here is priced.**

---

## 10. What the next dispatch should take, in order

1. **Fill `probe.toml` for the earlier topics.** Eight settled claims and twenty-one existing rows are
   blocked on it, four of my sixteen edges land on rows inside that population, and it is the single
   highest-value thing left in this area. `47_probes`, `40_probes`, `07_probes`, `06_probes` and
   `97_probes` are all committed with outputs.
2. **Resolve the three-row disagreement about what fixes an answer**, section 5.4. It needs a
   `standing` or a `supersedes`, both of which are `says`-tier edits.
3. **Put the four decider questions to whoever owns that call**, section 5.2, together rather than one
   at a time, since three of them are one canon-form decision.
4. **Run the retirement list against the sixteen**, section 9.
5. **Repair the ignored test's reason**, section 0, which is one attribute and one sentence and would
   turn twenty-one silent gaps into a red the suite reports.
