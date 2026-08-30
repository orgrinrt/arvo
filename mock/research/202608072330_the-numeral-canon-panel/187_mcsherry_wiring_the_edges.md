# 187. Wiring the edges, and what the wiring measures

## The gates

**Canon gate: passed.** Checked against op at `181`, who says to "port all the current results and
agreements and convergences and settled things to the registry, shape the registry, its meta, so it
works", and against the schema in `mockspace.toml`, which declares `answers`, `obligation`,
`ratifies`, `declines`, `supersedes` and `corrects` as fields on exactly the rows I touched. Wiring
these is the "so it works" half: without them `refsto()` returns nothing for every row and the
registry's whole no-status-field design has no mechanism behind it. Nothing here adds a row, changes
a `says`, a `because` or a `predicate`, or touches a namespace the brief reserved.

**Test gate: passed, and the suite is real.** 42 passing, 1 ignored with a catalogue reason naming
the empty `probe` namespace as its blocker, which is accurate. I read the bodies rather than the
names, in `mock/checks/tests/what_one_field_obliges_another_to_carry.rs` and its four siblings, plus
the arms they call in `mock/checks/src/`. No tautologies, no setup that helps: every arm has both
directions planted, and `the_two_correct_shapes_are_both_silent` exists specifically to catch an arm
that reports the rule rather than a breach of it. `the_rulings_with_no_verbatim_are_the_four_the_corpus_has_no_words_for`
is the one that does not assert zero, and it is right not to: it pins a known hole by name so a fifth
fails. That is a suite somebody thought about. Re-run after every batch of edges, still green.

One correction to the brief, which cost nothing but should not be inherited: **`182`'s edge table is
section 13, not section 8.** Section 8 is where the schema fought it.

---

## 1. The headline

```
$ 187_probes/coverage.sh

######## question, by decider
  decider         total  answered  proposed   open
  coordinator         3         0         0      3
  measurement         3         0         0      3
  op                 27         4         0     23
  panel              45         0        13     32
  ALL                78         4        13     61

######## obligation, by consumer
  ALL                11         0         1     10
```

**Answered** means a `ruling` names it: op said it. **Proposed** means only a `proposal` names it: the
panel would settle it if op stamped the paper. **Open** means nothing points at it. The split is the
schema's own and not a presentation choice; `mockspace.toml` on the obligation namespace says a
proposal alone is "proposed rather than answered" and that "anything counting coverage draws that
line, because reporting a proposal as an answer closes a gap op has never seen."

So the answer to the question this dispatch was sent to measure: **of the forty-five questions
recorded as the panel's, the panel has settled none of them, and has paper against thirteen.** Thirty-two
have nothing at all. That is a different project from forty already settled, and it is the worse of
the two.

The four answered are all op's own, all from `104` and `105` and `28`, and all four are rulings that
pick one of the question's listed options in nearly its own words. Twenty-three of op's twenty-seven
remain his to answer.

Three of the eleven obligations gained nothing, seven were never reachable, and one has a proposal
against it. Section 4 is why, and it is the finding I would put in front of op before any of the
others.

Before trusting any of that: the counter has a control, and both planted edges move the right
columns.

```
=== CONTROL: one ruling edge and one proposal edge planted ===
  panel              45         1        14     30      (was 45  0  13  32)
  ALL                78         5        14     59      (was 78  4  13  61)
```

## 2. What I wired, and the rule I wired it by

Twenty-one edges: four `ruling.answers`, fifteen `proposal.answers`, two `proposal.obligation`.
**No `ratifies` and no `declines`, and zero is the right number there rather than a gap.** One
ruling sits at the ratified rung, `the_work_is_predicated_arms_composed`, and what it ratifies is two
paragraphs of op's own rather than anything the panel converged on. `179` established that and it is
still true.

The rule, applied to every candidate: **open both rows, and ask whether the claim addresses what the
question asks, over the same subject, closely enough that a reader could act on it.** Where the match
needed a step of my own reasoning to get from the claim to the question, I did not write it and it is
in section 5 instead. That test is why most of what the four seats tabulated did not survive.

**`ruling.answers`, four.** Each picks a listed option in the ruling's own words: `validate_means_all_three_readings`
takes all three readings of Q1 and Q1's three options are those readings; `notko_renames_and_strategy_is_arvos_name`
takes Q46's second option; `the_imitation_is_ergonomics_not_an_arithmetic_boundary` takes Q47's third,
which is op's own and was not among the two the panel built; `the_overflow_panic_is_permitted_and_bounded`
takes the first option of the split-off second half of Q47, near-verbatim on "the intent bends for it".

**`proposal.answers`, fifteen against thirteen questions.** The dense seam is the number-system topic,
which turns out to have nineteen proposals sitting on top of fifteen questions and had no edge between
any of them. `a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts` and
`an_exposure_test_over_reduction_verdicts_alone_is_satisfied_by_a_system_that_computes_nothing` both
land on `what_the_admission_contract_asks_a_candidate_to_expose`, one supplying its second option and
one refuting its first; the question's own option (a) text is the refutation's `says` in miniature,
which is somebody having made this connection in prose and nowhere a machine could see it. Two more,
`the_model_band_transfer_is_defeated_in_both_fragments` and
`inside_a_fragment_with_a_complete_test_set_the_verdict_is_computed_at_the_shipped_width`, land
together on `where_a_law_verdict_is_established`, the first killing the two options that rest on a
model band and the second replacing the third.

**`proposal.obligation`, two, both on `composition_contracts_above_the_numeral`.**
`a_chain_is_exact_operations_together_with_a_schedule_of_adaptation_points` supplies the concept the
obligation needs ("a concept that hides the adaptation inside each operation cannot state the
chain-accuracy intent at all", which is the obligation's `why` reached from the other side), and
`within_an_unbound_stretch_the_design_may_select_any_realisation` supplies four declared grades and
what each owes at a boundary, which is a contract for a unit bigger than a numeral in the plainest
sense available.

The full list is in the commit, `c050f260`, and reads more usefully as a diff than as a table here.

## 3. The seats' tables are "what was put to him", not "what he settled", and that is most of this pass

`179` section 6 tabulates fourteen rows against the questions each would answer. **Eight of the
fourteen point at rows that decline to answer.** Five are `kind = "deferral"`, four of those at
`rung = "open"`, and two more are `kind = "process"` rulings whose content is *keep exploring*.

Wiring them would have reported the questions as settled where op's own words say the opposite. Two
examples, and they are representative rather than the worst:

- **`wrap_or_clamp_stays_open_and_both_get_priced` → Q6.** The ruling's `says` is "Whether the intuitive-compromise strategy wraps or clamps **is not settled**. Both readings are explored and both are priced." Q6 asks "Does the `Warm` preset wrap or clamp?" Under the schema `answers` means "questions this settles", and a `refsto()` hit here would tell every later reader that op picked one.
- **`consider_all_options_and_do_not_get_married` → Q2, Q3, Q5.** One process ruling, three questions, and what it says is "take whatever is convenient, keep every option open, and explore, per the standing rules." Its own `note` says it was "given as the answer to" those three. Given as the answer to a question, and not an answer to it. Three edges that would have converted a standing instruction to keep exploring into three closures.

`179` was not wrong; it was writing before `question.toml` existed and its column header says "Would
answer", by Q-number against `OPTIONS.md`. What has happened since is that the registry rephrased
those questions, and the rephrasing is what breaks the match. **Q50 is the sharpest case**: the
register recorded it CLOSED, and what was closed was *whose call it is*, while the registry's
`is_i9_about_a_strategy_or_one_component` asks the substantive question, sets `decider = "panel"`, and
its own note says "It stays live as an implementation question."

**So the general lesson for the next port: a tabulation written against a prose register does not
survive the port of that register into rows, and it does not fail loudly when it stops surviving.**
Every one of those eight would have linted clean.

## 4. Nothing in the corpus reaches five of the eleven obligations, and that is the report to op

`187_probes/obligation_reach.sh` nets, for each obligation, every ruling and proposal whose `says`
mentions any of that obligation's own keywords. It is a net rather than a test and its hits are
mostly noise: its keywords drag in 52 hits, 46 of them on the bare word "set" and every one of those
about a representable set rather than a bit set. **The zeros are the finding.**

```
a_sparse_adjacency_a_plan_can_be_built_on          0 rows mention any of its own keywords
a_content_hash                                     0
ordering_a_directed_acyclic_graph                  1   ("ordering", in a sentence about strategies)
a_spectral_partition_of_a_dependency_graph         1   ("partition", in a sentence about levels)
a_cost_dynamic_program                            11   every one of them "cost coordinates"
set_operations_over_a_fixed_size_bit_set          52   every one of them a mathematical set
```

Read them together: **the five obligations that come from hilavitkutin's twelve-step plan chain, which
is the heaviest consumer's account of what it actually needs from arvo, are not what this panel has
been talking about.** Not underserved, not partially served. Untouched, across a hundred and eighty
files. The word "Fiedler" appears in no `says` in the registry. Neither does "topological", "CSR", or
"content hash".

The control plants one row carrying "a fiedler partition of the dependency graph, and a stable
content hash" and it surfaces under both zero obligations, so the net can catch something and the
zeros are about the corpus rather than about the grep.

I am not going to dress this up. Op's bar at `181` is that the canon be "exhaustive enough that a full
design and then a full impl of everything can be done based on it". On the demand side as `184`
enumerated it, the canon currently reaches one obligation of eleven, with paper rather than a ruling.
Whether that is a scoping decision nobody wrote down, or work nobody has started, is not something I
can settle from here, and it is the first thing I would put in the batch for op.

**One thing it is not**: evidence that `184`'s enumeration is wrong. `184` says plainly that it read
three consumer repositories and op's `I11` and did not sweep this panel's own corpus, and that "an
obligation absent from the registry today means nobody has enumerated it yet". The gap runs both ways
and the sweep it names is still owed.

## 5. Every asserted edge I could not confirm

Each of these was asserted by a seat that had the material in front of it. I opened both rows and
could not make the claim address the question. They are listed with what the two rows actually say,
because in several cases the seat found something real and filed it against the wrong target.

**From `179` section 6.**

1. **`consider_all_options_and_do_not_get_married` → Q2, Q3, Q5.** Section 3. Says keep exploring.
2. **`the_option_set_is_not_a_boundary` → Q4.** "An option set put to him does not bound the panel. A shape nobody has written down is admissible." He widened the space. Q4 asks what a datum stands for and lists four readings; the ruling declines to bound the list.
3. **`wrap_or_clamp_stays_open_and_both_get_priced` → Q6.** Section 3.
4. **`the_carrier_question_waits_on_the_contention_measurement` → Q7.** `says`: "explore and wait for the contention measurement." `instead`: "It goes back to the panel and waits on the contention run."
5. **`his_instinct_on_one_family_is_not_to_be_acted_on` → Q8.** `179` writes this one as answering "by declining it", which is honest and is still not an `answers` edge: the row's `instead` is "It goes back to the panel to explore and converge. His instinct is recorded and is explicitly not to be acted on."
6. **`the_family_question_wants_the_comparison_first` → Q8's ancestor.** Wants a written comparison first. Nothing to point at even if it did.
7. **`which_half_of_the_pair_i9_attaches_to_is_not_his` → Q50.** Section 3. Settles who decides, not what.
8. **`the_predicate_is_whatever_is_available_at_const_time` → Q39.** `179` says "bears on", which is exact, and the brief compressed it to an edge. Both the ruling's `gap` and Q39's `note` say the same thing in the same words: what is not settled is "what happens to a condition that is genuinely not const-available", and that residue is what Q39 is now about.
9. **`never_a_runtime_check_and_one_lowered_path` → Q-A**, **`the_operating_constraints_are_intents_and_rules` → Q-B**, **`the_predicate_is_whatever_is_available_at_const_time` → Q-C.** No row carries a `key` of `Q-A`, `Q-B` or `Q-C`. `grep 'key = "Q-' question.toml` returns nothing. Three questions op is recorded as having closed exist only in `85`.

**From `182` section 13.**

10. **`the_laws_of_a_format_are_derived_from_two_hypotheses_rather_than_enumerated_per_policy` → "how a law verdict's truth is established".** The registry's row for that is `where_a_law_verdict_is_established`, whose three options are all about *where the checking happens*: in the compiler, offline at a model width, offline as a closed form. The proposal never mentions compilers, model widths or offline computation; it is a theorem about when the induced operation is associative. Topical adjacency, and the two rows that do reach that question are wired instead. I looked for a better target and `which_route_a_law_verdict_takes_to_closed_form` is the near miss; its own note says "no file in this panel has asked it of any law it measured", and the derivation frame is not one of its three routes either.
11. **`an_additive_verdict_is_independent_of_the_fraction_width` → the mixed-scale addition question.** `182` marks this one itself: "**partially**: the row depends on it rather than answering it." Agreed, and not written.
12. **`a_format_is_identified_by_its_ambient_domain_and_its_representable_set` → "the wrapping filing question".** The registry's row is `where_wrapping_lives`, whose first option is "One slot, members classified along the two law roles". The format-identity proposal says adaptation and encoding are realisation rather than identity, which is a different claim. **The row that does answer it is `the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`**, whose `says` carries "Members classify along two independent law roles" against the option's "members classified along the two law roles". Wired to that one instead. A found edge filed one row over.
13. **`a_format_is_identified_by_its_ambient_domain_and_its_representable_set` → "format equality".** No question row asks about format equality.
14. **`a_strategy_is_a_declared_semantics_together_with_a_weighting_over_the_arms_that_realise_it` → "which object the word `strategy` names".** No question row, and the absence is deliberate: `180` section 3 excluded Q51 from the port as a finding wearing a question's number, on the ground that it "opens 'It survives as a two-component object', which is an answer". So four rows define what a strategy is and none of them can point anywhere, correctly.
15. **`the_licensed_category_is_const_available_and_four_constructions_bind_at_four_times` → "the binding-time question".** No question row. `179` records the four-construction question as one op refused with no register identifier.
16. **`no_total_join_exists_over_the_observable_axes_so_the_operation_reports` → "cross-strategy resolution; the conservatism-order reading".** No question row for either.
17. **`roles_derive_representations_and_a_realisation_variant_computes_nothing_new` → whether the role set is homogeneous.** `182` marks it "**by naming the condition rather than settling it**", and the proposal agrees in its own words: "whether every proposed role is of that kind is exactly the open question". The nearest row, `is_the_role_set_closed`, gets one real thing from it: "a chain's extent is not a role at all but a schedule" removes the candidate fourth role its second option names, and that is a partial with nowhere to go. Section 7.

**From `183` section 8.** This one is different in kind and should be said: **`183` tabulated seven
rows and answered "would stamping it settle the question" with "no" for six of them and "partly" for
the seventh.** It got there by the same route I did and it got there first. I confirmed all seven and
wrote none of them. Its summary sentence is the one to keep: "Not one of my rows settles a question
outright, and that is not an accident of the port. The questions these topics left are the ones their
own authors reserved."

**And one I talked myself into and a row talked me out of.** I had
`an_order_is_named_exactly_where_a_crossing_is_lossy` → `whose_reduction_governs_a_lossy_crossing`
written down and ready. Its `says` names the two candidate orders and says the typestate cannot break
the tie, and the question's third option is "A third named at the crossing site", so it read as a
match. Then I opened its `gap`: "Whose reduction governs a lossy crossing, the source's, the target's,
or one named at the crossing site, **is unresolved and is op's**." The row refutes the edge in the
field directly under the one I was reading. I kept the other edge from the same row, to
`when_is_an_order_owed_at_a_crossing`, which is verbatim.

Worth noting beside it: that question's `decider` is `panel` and the proposal says it is op's. Somebody
should reconcile those, and it is not my call.

## 6. Questions whose answer went into the register rather than into a row

`187_probes/note_says_answered.sh` finds question rows whose own `note` records that they were
answered, and splits them by whether anything now points at them. **Six say they were answered or
declined or dissolved and nothing points at them.** Two are false positives of the phrase list
(`the_width_surface_crossing` and `the_exchange_rate_a_preference_yields_at`, where the matched word
is describing something else). Four are real, and they are four different shapes:

- **`is_i9_about_a_strategy_or_one_component`.** "Recorded as put to op on 2026-08-14 and declined by him as not his." The declining ruling is a row. The relation has no expressible form. Section 7.
- **`which_carrier_the_packing_claim_is_about`.** "Op answered the regime half at `32`." The row for `32` is `arvo_takes_no_stance_on_how_many_cores_it_runs_on`, which exists and is good, and it answers **half** of Q7. Section 7 again.
- **`container_derivation_output_count`.** "`47` is recorded as dissolving the one-against-two fork: the value-valued spelling of a single output is compiled-refused six times across three syntactic positions." That is a measured finding with six compiled refusals behind it and **there is no proposal row for it.** It is not an edge I can wire; it is a row somebody has to write, and until they do the registry cannot see that the fork is dissolved.
- **`are_open_domain_dimensions_stated_as_open`.** "The coordinator records it as not derivable and folds it into the unpredicated-proposition question." A coupling between two questions, in prose, with no field for it.

**And all three `coordinator` questions are this shape.** Each says, in nearly identical words,
"answered in op's stead by the coordinator at `156` as derivable; **the answer is not written here**",
and records the reasoning offered. So the registry states three times that an answer exists and
carries none of them. `refsto()` reports all three open, and that reading is currently the honest one:
an overturnable call recorded only as prose in a question's note is not something a design can be
written from. Three rows would fix it, in whichever namespace the panel judges right, and adding rows
was not this dispatch.

## 7. What the schema cannot say, which is the same gap four times

Four separate relations in this corpus have no field, and every one of them ended up as prose in a
`note`:

1. **A ruling that closes *whose call it is* without closing the question.** Five deferrals do this. `instead` carries who it goes back to, which is the right field and is not an edge, so `refsto(question::x)` cannot see that op has been asked and has answered.
2. **A claim that settles half a question.** `arvo_takes_no_stance_on_how_many_cores_it_runs_on` against Q7's regime half; `a_law_is_read_off_the_algebra_and_never_declared` against Q38's who-may-assert half, which `183` marks "partly"; `roles_derive_representations...` removing one option's named candidate from `is_the_role_set_closed`.
3. **A claim that bears on a question without settling it.** The largest class by far, and the one every seat reached for a word for. `182` wrote "bears on", `183` wrote "bears on both and settles neither", `179` wrote "bears on Q39".
4. **Two questions that are one decision.** `180` found four such couplings and each row names the other in prose.

**I am not proposing a field.** The right response might be a fifth thing, or it might be that these
belong in `note` and the count is supposed to be conservative, and that is a canon-form call rather
than a schema one. What I can report is that the absence has a measurable cost right now: **of the
thirty-two panel questions with nothing pointing at them, at least six have something in the corpus
that reaches them and cannot be joined to them**, so the open count is real as a count of settled
questions and pessimistic as a count of unexamined ones.

## 8. Duplicates, splits, and questions asking something the corpus stopped arguing about

**`is_number_system_broad_enough_for_non_magnitude` (Q21) asks a question its own note retires.** The
note: "The entry's stated discriminator is measured not to cut... **What remains is whether the ambient
operation family is fixed or is a parameter, which is Q33, and the two entries should be read as one.**"
I wired an answer to it anyway, because `the_concepts_edge_is_not_an_order_and_wrapping_is_the_test`
does settle the row as written and says so in the same words the note uses. So the honest reading of
the counts is that Q21's half is done and Q33's is open, and a consolidation should merge them rather
than carry two rows one of which is answered.

**`are_the_level_hierarchies_the_same_cut` asks one thing and lists options for another.** The `asks`
is "Are the two proposed level hierarchies the same cut at different granularity, or different cuts?"
The `options` are "Three levels: ..." and "Five levels: ...", which answer a different question.
`the_concept_commits_to_its_choices_and_to_no_count_of_levels` answers the `asks` cleanly ("Both
partitions are real and neither refines the other") and refuses both options ("no count of levels").
I wired it to the `asks`.

**`what_goes_into_the_compiler_winner_or_cost_table` is the same shape and it matters more.** The
`asks` is "Which table goes into the compiler: the winner table, or the cost table?" Its third option
is the compose-all-three answer, and
`what_ships_is_four_arms_with_disjoint_predicates_over_one_generated_artifact_set` states that option
almost word for word. I wired it. But that proposal's own `gap` says: "the fork with consumer-visible
content is a different one nobody picked up, **whether the compiler is handed a winner table or a cost
table**." So the row answers the option list and leaves the `asks` open, by its own account. A reader
counting off `refsto()` will read this question as proposed-against when the thing it literally asks
is untouched.

**Three rows carry a duplicated `key` and that is correct rather than a defect.** `Q37` twice
(`does_the_canon_name_crossing_classes`, `when_is_an_order_owed_at_a_crossing`) and `Q47` twice
(`which_reading_of_i3_at_a_non_native_width`, `does_the_native_imitation_cover_the_debug_panic`), each
a deliberate split `180` recorded as its own call. The Q47 split is vindicated by this pass: op
answered the two halves differently on the same day, and they now carry edges to two different
rulings. A merged row could have carried only one.

**Two more couplings `180` found and I confirmed are still uncrossed.** Q61, Q62, `156` item 2 and the
tenth unit's normative-sentence question are four rows and, by `151`'s and `173`'s own readings, two
decisions. All four are `decider = "op"` and all four are open, so op is currently owed four questions
where two are wanted.

## 9. One question the registry has already answered on op's behalf, in the schema

`what_a_proof_marker_is_against_a_measurement` is `decider = "op"`, open, and its second option is:
"Op ruling that a proof carries a different marker from a measurement, so a width-free argument is
written as one rather than being dressed as a sweep that happened to stop at three widths."

**That option is implemented.** `proposal.sentence_kind` has values `theorem`, `measured`,
`enumeration`, `normative` and `argument`, the schema comment explains it as existing "because a canon
that states a theorem and a normative licence in one voice loses the result that distinguishing them
was", and `182` section 6 assigned a mark to all seventy-four of its rows. There is no ruling behind
any of it and op has not been asked.

I am not calling that wrong. It may be exactly what he would rule and it is a good mechanism. What it
is, mechanically, is a question of op's that the registry's own shape has taken a side on while the
question row still reads as open, and that is worth him knowing before he is asked the question as
though it were fresh.

## 10. Leads: questions nothing answers that I think the corpus answers somewhere

Marked as leads. None of them is an edge and none should be treated as one until somebody reads the
material properly.

- **`is_i9_about_a_strategy_or_one_component` (Q50).** Three rows say the answer-fixing half is the policy half: `no_cost_model_may_move_an_answer` ("Every difference in an answer traces to the declared policy and nothing else may move one"), `a_strategy_is_an_assignment_and_a_weighting` ("Component one... fixes the denoted answer. Component two... ranges over realisations of that denotation"), and the superseded `a_strategy_is_a_pair_of_an_observable_assignment_and_a_weighting`. Put beside `I9`, the intent that the strategy is what makes an answer correct, each yields Q50's second option. **I did not wire any of them, because getting from the claim to the question takes a syllogism and none of the three mentions `I9`.** Somebody should either write the syllogism as a proposal or say why it fails.
  - **And the third of those three records the load-bearing clause as false**, in its own `gap`: "Clause three is false and it is the one that mattered: component two was defined as ranging over the arms that produce the answer the first component fixed, and in that region a fidelity column would measure a constant." Its successor repairs it by making distance-from-the-declaration a cost coordinate, which makes the weighting answer-affecting after all. **So the three rows do not agree**, and whoever takes this lead is walking into a live contradiction rather than a convergence.
- **`what_a_platform_width_type_is` (Q26).** Its first option is written in `a_format_is_identified_by_its_ambient_domain_and_its_representable_set`'s words ("a value set that depends on other data is not a format but storage"). But `each_choice_in_the_sequence_has_an_owner_and_a_resolution_time` says "a platform-width numeral is a target-indexed family of formats **whose exclusion grounds apply only to dependence that survives to runtime**", which is a qualification on exactly that rule. Neither cites the other. The disputed step is the application, not the rule, so no edge; the reconciliation is worth a dispatch on its own.
- **`is_the_ambient_operation_family_fixed` (Q33).** `the_numeral_concept_is_a_dependent_sequence_of_choices` makes the ambient domain the first *choice* in the sequence, which arguably makes its operation family a parameter by construction. It never says so.
- **`container_derivation_output_count`.** `the_lens_degenerates_to_an_ordinary_value_at_sole_occupancy` describes the realisation as "a placement of the value's bits, a carrier, an offset and a width", which is the shape the question's third option calls "one richer output that is a type with named projections". It does not claim to be the derivation's output and says nothing about injectivity.
- **`arms_no_weighting_can_select` (Q45).** `the_corpus_cannot_exhibit_the_accuracy_intents_because_a_coordinate_is_absent` says the barrier is the absent coordinate rather than the arm sets, which is the reasoning of Q45's second option about a different subject.
- **`what_goes_into_the_compiler_winner_or_cost_table` (Q43), second row.** `generation_relocates_the_check_rather_than_removing_it` composes with the row I did wire into exactly Q43's third option, and says so, but it says so in `because`, not in `says`. Its claim proper is about defect-detection rates. If the composition is real it should be a row, and its own `because` already writes the sentence: "the committed winner table equals the argmin of the stated weighting over the committed cost table at every region, compiled as a const item".
- **`what_a_numeral_guarantees_to_a_fold`.** `a_coherent_reduction_needs_no_accumulator` and `an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit` between them supply the content the question's third option would need, which is when an accumulator is owed and how wide. Neither says the numeral should name it, which is what the option is about.
- **`which_axes_a_build_arm_may_move`.** `headroom_and_intermediate_precision_are_unobservable_inside_a_pure_ring_region` supplies a regional classification the second option depends on, and does not answer which axes an arm may move.

## 11. Control runs

Transcript at `187_probes/control_runs.txt`. Four instruments, and the controls found three defects,
two of them in my own tools and one of them serious.

**The checker fails on a bad slug, which is the one the brief asked for.** Planted
`answers = ["a_question_that_does_not_exist"]` on `ruling::validate_means_all_three_readings`:

```
ERROR [unknown-row-reference]: ruling::validate_means_all_three_readings: field `answers`
references `question::a_question_that_does_not_exist`, which no row declares.
registry check failed: 1 error(s)
```

Swapped the slug for the real one, `schema check passed`, `491 rows across 9 namespaces`. So every
edge in this pass is resolved by something that has been shown to be capable of refusing.

**`row.sh` printed nothing for a row that was there.** Its table-header rule ran after its
namespace-match rule and reset the buffer before it could flush, so every read came back empty and was
indistinguishable from a missing slug. Caught only because the first thing I ran it on was a row I
knew existed. That is the whole reason the positive case is in the transcript beside the negative one.

**`add_edge.sh` reported "row not found" for a row it had found**, whenever the row carried none of
the tail fields the insertion anchors on. Harmless here, since every row in these files carries `provenance`
and `keywords`, and it was still wrong and is fixed.

**`add_edge.sh` corrupted the file when the namespace matched nothing, and exited 0 saying it had
succeeded.** It buffered every table header regardless of namespace, so each header printed one row
late and the last one landed at the end of the file. This is the one worth naming: a tool that silently
reorders a canon file while reporting success is worse than one that crashes, and nothing but the
control would have caught it, because the row I asked about was untouched and the damage was elsewhere
in the file. It now buffers only the matching namespace and the control asserts that a row in another
namespace comes out byte for byte identical.

**`obligation_reach.sh` printed nothing at all on its first run**, because a keyword matching zero rows
exits 1 and `set -e` killed the script. An all-empty report from a script about obligations nothing
reaches is the single most believable wrong answer available. Fixed with `|| true` on the grep, and the
`--control` arm plants a row that must appear under the two zero obligations.

**`coverage.sh --control`** plants one ruling edge and one proposal edge against two questions that
have neither and both move to the right columns, shown in section 1.

`by_topic.sh` and `pair.sh` both have a negative arm: a topic naming no row prints empty sections
rather than falling over or silently matching everything.

## 12. What I would tell the next reader to check first

1. **Section 4, and put it to op.** Five of eleven obligations are untouched by the whole corpus and they are the heaviest consumer's actual list. Either the scope excludes them and nobody wrote that down, or the panel has been working somewhere else for a hundred and eighty files. Both are answers; not knowing which is not.
2. **The three coordinator answers that exist only in a note.** Section 6. Three rows, and then the queue is three shorter and `refsto()` agrees with the prose.
3. **Q21 and Q33, and the four canon-form rows that are two decisions.** Section 8. Merging is cheap now and expensive after anything is keyed on the slugs.
4. **The Q50 contradiction in section 10**, which is the largest live disagreement I found and is not recorded as one anywhere: `standing` on the three rows reads `one_expert`, `two_experts` and `one_expert`, and none of them says the other two exist.
5. **Section 9**, before op is asked a question the schema has already answered.
6. **The `bears on` gap in section 7**, which is a canon-form decision rather than a schema one, and which is currently costing six or more questions their visible link to material that reaches them.

## 13. What I did not do

Added no rows, in any namespace. Changed no `says`, `because`, `predicate`, `note` or `gap`. Did not
touch `topic.toml`, `dimension.toml`, `question.toml`, `retirement.toml`, `obligation.toml` or
`probe.toml`. Wrote no `ratifies` and no `declines`, and section 2 says why zero is the answer rather
than an omission. Did not sweep the panel's own corpus for obligations, which `184` names as owed and
which is where most of the enumeration still is. Did not reconcile the `decider` disagreements I
found, of which there are at least two: `whose_reduction_governs_a_lossy_crossing` is `panel` and a
proposal says it is op's, and `two_shapes_of_aggregate_composition` is `panel` where `183` calls it a
vocabulary call reserved to op.
