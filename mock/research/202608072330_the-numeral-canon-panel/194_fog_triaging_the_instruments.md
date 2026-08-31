# 194. Triaging the instruments, and the word list that had to go

`185` filled the namespace, `192` doubled it and reported that the prose matcher caught one admission
in five. This pass makes the admission data. **Nine of the seventy-nine rows now carry
`standing = "uncontrolled"`, and four new rows exist because a directory holds several arms and a row
describes one.**

**The matcher misses three of the nine, not one.** The second reader found the counterfactual; reading
all seventy-five control fields in full found two more, and they are a different failure from the
counterfactual:

```
a_compile_time_strategy_selection_leaves_no_residue
  "...no single one of them carries a case that had to fail."
the_argmin_mechanism_has_never_run_on_arms_that_disagree
  "Neither arm carries a planted case that had to fail..."
```

Neither opens with the word. Neither contains the phrase `no case that had to fail` as a substring,
because the negation sits on a different noun: *no single one of them carries a case*, *neither arm
carries a planted case*. **A word list cannot see a negation it is not adjacent to**, which is the same
lesson as the counterfactual arriving from the other side. Six of nine, and the two directions of
failure are now both on record.

## 0. The gates

**Test gate: 69 tests across nine files, all passing, zero ignored, run before the work.** I read the
bodies of the four that govern this pass. `the_ways_this_corpus_says_none_are_all_read_as_none` and
`a_control_that_fired_may_open_with_the_same_word` are a genuine pair, eight admissions against three
counterfactuals, both directions planted. `names_no_control`'s own docstring already says it is a
guess right about half the time, kept only while the triage runs, and marked for deletion. That is
accurate and this pass is the deletion's precondition.

**Canon gate: passed.** `standing` carries four values at `mockspace.toml:776`, the description says
plainly that `uncontrolled` is not a lesser `defective`, and nothing else in the schema moved.

**One test is red when I finish and I did not silence it.** Section 3.

## 1. The rule, written before it was applied

Every borderline in this pass turns on one sentence, so it goes first:

> **`uncontrolled` where the `control` field states that no case whose outcome was required in
> advance exists.** Not where I judge the control weak.

Three things are not controls and do not by themselves make a row `sound`: **corroboration by a second
instrument, a prerequisite arm, and a comparison whose either outcome would have been informative.**

But a required outcome may be **implicit in what the claim needs**. Where a claim only stands if a
comparison comes out one way, that way is the required outcome even where the field never uses the
word control. **Those rows stay `sound`** and are section 4.

**This is what keeps the distinction `192` section 5 was protecting.** "Unrefutable by design" and "no
control stated" are different, and the value is for the second. **All five of the thin `sound` rows
`192` named moved**, and that is not the collapse the brief warned against: on reading their fields in
full, every one of the five *states* that no case that had to fail was run. The distinction held; the
five simply turned out to be on the stated side of it.

## 2. The nine, and what each field says

```
before   2 defective   72 sound                     1 withdrawn
after    2 defective   67 sound   9 uncontrolled    1 withdrawn
```

| row | the words in its own field |
|---|---|
| `chain_the_third_definition_is_not_observation_bounded` | "None stated in the material read." |
| `no_dependent_survives_the_rounding_units_defects` | "None stated as a must-fail arm." |
| `the_bench_tree_was_built_at_the_undocumented_profile` | "None run, and the shape does not admit an obvious one" |
| `the_debug_release_gap_that_retired_a_true_finding` | "None, and none is needed for what it establishes" |
| `an_equivalence_checker_that_skips_panics_...` | "None. It is a structural argument" |
| `chain_accuracy_needs_an_intermediate_wider_than_the_operand_type` | "None was run as a case that had to fail." |
| `the_four_const_available_constructions_bind_at_four_times` | same sentence, and the matcher misses it on `reported` inside a counterfactual |
| `a_compile_time_strategy_selection_leaves_no_residue` | "no single one of them carries a case that had to fail" |
| `the_argmin_mechanism_has_never_run_on_arms_that_disagree` | "Neither arm carries a planted case that had to fail" |

**The ratio, read from the field rather than from prose** (`194_probes/p4_the_ratio_from_data.out`):

```
instruments that ran with no case that had to fail:  9 of 79  (11%)
instruments carrying one:                           67 of 79  (85%)
plus two `defective` and one `withdrawn`, which are neither.
```

**`185`'s ratio instrument is now wrong and should not be rerun for this number.** It counts by
looking at whether a field opens with the word, reports 7 of 79, and the two it cannot see are the
two above. **Prose says seven, the data says nine**, and the prose set is a strict subset of the data
set, which is the check that says the triage missed nothing rather than merely disagreeing.

## 3. What the triage turned red, named

`the_committed_canon_rests_no_measurement_on_an_unusable_instrument` **fails, and is left failing.**
Two live claims cite a row that is now `uncontrolled` (`194_probes/p2_what_the_triage_turned_red.out`,
which confirms each against the registry rather than reading the failure message back):

| claim | probe it cites |
|---|---|
| `a_compile_time_strategy_selection_leaves_no_residue_in_the_emitted_body` | `a_compile_time_strategy_selection_leaves_no_residue` |
| `the_licensed_category_is_const_available_and_four_constructions_bind_at_four_times` | `the_four_const_available_constructions_bind_at_four_times` |

**Both edges were wired last pass from my own map, and both are rows `192` section 5 named as thin.**
So this is not a surprise arriving from outside: the register said these two were weak in prose, the
edges were written anyway because prose is not a gate, and the value makes the same statement in a
form the gate reads. **That is the whole argument for the value in one worked example.**

**The repair is on the claims and not on the rows.** Either they cite a controlled instrument or they
stop being marked as something that ran. Exactly one other test in that file could have moved and none
did.

## 4. The bucket, which is what I was asked for and which empties

Rows where I could not tell whether a control was stated. Found mechanically as the residue of two
classifications rather than as my list of rows I felt uneasy about, so it cannot be curated:
`admission` where the field says none was run, `required` where it states an outcome demanded in
advance, `BUCKET` where neither.

**The first run put 31 of 79 in the bucket and was wrong**, because the pattern carried only the demand
words. This corpus states a fired control in other vocabulary: *the negative arm*, *it fired*, *a
mutant the construction must reject*, *are each other's control*. Widened, the bucket is **13**, and
the first run is kept as `p3_run1_requirement_vocabulary_too_narrow.out`. `disagree` is deliberately
excluded from the widened list, because "the two disagree or they do not" is precisely the shape with
no required outcome and admitting the word would have classified the clearest candidate as `required`.

**All thirteen were read and the verdicts are committed** in the probe output rather than only here.
Five state a requirement in vocabulary the pattern does not carry. Six have a requirement implicit in
what the claim needs. One is already `defective` and correctly so. **And one could not be settled by
reading the field, so I opened the instrument.**

**That one is the finding.** `the_collapsed_declaration_cannot_be_made_to_fail` is a mutation probe,
and a mutation probe reporting that nothing noticed is indistinguishable from a broken checker unless
something is shown to be noticeable. Its output carries exactly that arm, and my row's `control` field
did not name it:

```
ok  the mutation set breaks the HONEST verdicts in 3 of 4 cases
```

One of the four reductions is labelled `the control` in the source. **So the row understated its own
instrument**, which is the same defect as overstating one pointed the other way, and it would have
sent a reader looking for a control that was already there. The field is amended to name the arm.

**So the bucket empties, and that is a result about the corpus rather than about the reading**: none of
the thirteen turned out to be a row where a control was neither stated nor implied. **The six implicit
ones are the real residue**, and they are where a second reader should push, because an implicit
requirement is one nobody wrote down and the next reader has to reconstruct it exactly as I just did:

`staged_narrowing_depends_on_its_staging`, `operation_erasure_holds_at_every_optimisation_level`,
`one_congruence_property_predicts_every_law_verdict`, `no_conservatism_order_exists_on_the_overflow_axis`,
`the_const_eval_frontier_by_arity`, `generating_a_winner_table_from_a_stated_weighting`.

## 5. The four arms, and the mistake they all were

**One directory, several arms, one row describing one of them.** In all four the directory was right
and the row was about something else in it. The rows are written; **no edge is written.**

**`a_coherent_reduction_diverges_nowhere_at_any_fold_length`**, from `57_probes/p4` section 2. Coherent
policies at zero divergence for every fold length two through six; the incoherent one at zero for two
and 476 for three. The neighbouring row in that directory measures the incoherent policy and says so in
its own source: `57_probes/p5_output.txt:72` names its format as the one both `q1` and `p4` measure as
incoherent. **An edge pointed at the other row would have cited a measurement of the opposite case.**

**`the_rationalisability_counts_reproduced_and_rungged`**, from `98_probes/p6`. 72 of 15625 at
non-negative and 9 at strictly positive, on a six-region five-arm two-coordinate model, with 63 of the
72 selecting an arm the predecessor's own section 10 says no weighting can select. **Not the same
measurement** as `the_rationalisable_sections_on_the_committed_carrier_table`, which is a different
model reporting 9 at zero tolerance and 134 at one percent. Both are real; the map reached for the
nearest name because the right directory's row was already spoken for.

**`how_many_strategies_a_coordinate_set_can_distinguish`**, from `101_probes/p4`. Two coordinates reach
9 sections and three reach 42, and adding a sixth arm to the two-coordinate model moves neither count.
Its calibration section reproduces the published 72 and 9 before measuring anything new, and **it
fired**: the first version returned 9 where 72 was published, kept as its own file. The census row in
the same directory counts which coordinates exist; this one counts what a set of them can distinguish.

**`the_committed_corpus_does_contain_answer_differing_arms`**, from `103_probes` p2, p3 and p6, and this
is the one the brief called worst. Three independently sufficient measurements: two committed arms
denoting different values on 1989 of 2048 lanes; the same arms given the identical reals answering
identically on 92,556 of 200,000 trials; and a census over the right unit finding 234 of 254 regions
pinned by an exact-value oracle, 10 to a property only, 4 with no oracle, 6 with no bridge.

**And the row that was offered instead now carries the `defect` field whose absence let it be offered.**
`the_argmin_mechanism_has_never_run_on_arms_that_disagree` opened with "Every committed family's arm
set is answer-equivalent", **which is the proposition the citing claim's own `because` records as
refuted by three measurements.** Two changes:

- **`defect` filled**, naming the refutation, the row that carries it, and the mechanical cause: a census whose unit is the shared crate directory cannot see a region whose bridge is declared inline in the driver's own table. It says plainly that the row may not be cited for the answer-equivalence premise.
- **`establishes` narrowed**, because that field's contract is what the instrument shows stated as narrowly as it shows it, and half of what it said is known false. It now attributes the refuted half to the first arm and states what survives: the mechanism has still never been run on arms that disagree, and the refutation **strengthens** that rather than weakening it, since arms that disagree existing is what makes the region real.

**A row whose `establishes` asserts a refuted proposition with no `defect` beside it will be offered
innocently by anybody reading the field**, and that is what happened. The field pair exists for exactly
this and one half of it was empty.

## 6. My own defects this pass

Three, all caught, none by reading the code.

- **A replacement that deleted the line it was supposed to sit beside.** Inserting the `defect` field, my edit consumed the `standing = "uncontrolled"` line above it and the schema check reported the row as missing a required property. Caught immediately by `--lint-only`; the argument for running it after every edit rather than after every batch.
- **A classifier whose vocabulary was the wrong half of the corpus's.** Section 4. It inflated the bucket from 13 to 31 and buried the rows worth reading inside rows that plainly state a requirement.
- **I overwrote a previous round's committed artifact, again, having documented the hazard in `192` section 10.** Rerunning `185_probes/p8` for a number wrote over `185`'s output a second time. Restored from a named stash both times. **Writing a hazard down did not stop me walking into it**, which is the argument for the fix rather than the note: the output path wants to be an argument. `194_probes/p4` reads the registry directly for this reason and does not rerun anything.

## 7. What the next dispatch should take

**One. Delete `names_no_control`.** Its own docstring marks it for deletion once the rows carry the
value. They do. It is now six of nine, the two directions of its failure are both recorded, and every
row it flags is `uncontrolled`, so removing it loses nothing and keeps a wrong number from being
quoted. **The rows not yet triaged are none: all seventy-nine carry a standing I set by reading.**

**Two. The two red edges**, section 3. Not mine and not silenceable.

**Three. The four arms want their edges**, section 5, and the two-claims-one-row case from `192`
section 3 is still the only one in the map needing a decision rather than a transcription.

**Four. The six implicit requirements**, section 4. Each is a row where the case that had to fail is
reconstructable but unwritten, and the cheap repair is one sentence per row rather than a new value.

**Five. `185`'s ratio instrument should be retired or repointed**, section 2. It answers a question
the data now answers better and it reports a number two short.
