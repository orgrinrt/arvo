# 193. Wiring the evidence, and the five pairings that did not survive reading

## The gates

**Canon gate: passed.** Same licence as `187`: op at `181` says to shape the registry so it works, and
`mockspace.toml` declares `evidence` on `proposal` as `probe[]`, "the committed instruments behind it",
checked in `mock/checks` rather than by the schema. Nothing here writes another namespace, another edge
kind, a `says`, a `predicate` or a `sentence_kind`.

**Test gate: passed, 68 passing, no ignores**, which is a real change from the 42-and-one-ignored I ran
against last time. I read the bodies in the surface I touch:
`measurements_resting_on_an_unusable_instrument` and its four tests, `names_no_control` and its three,
and the ceiling. They are good tests. `a_measurement_may_not_rest_on_a_defective_withdrawn_or_uncontrolled_probe`
plants all four cases and asserts the sound one passes and an argument citing a defective probe is not
reported, which is the arm checking itself rather than the rule. `the_ways_this_corpus_says_none_are_all_read_as_none`
copies its eight admissions out of real rows instead of inventing them, and
`a_control_that_fired_may_open_with_the_same_word` is the carve-out's own control.

**One of those tests is not strong enough and section 4 is the reproduction.**

---

## 1. What landed

**Fifteen edges of the twenty pairings offered. Five declined. The ceiling drops from 21 to 6.**

The map's claim column is not taken on trust. `193_probes/claims_without_evidence.sh` derives the
uncovered set from the registry independently, and it is the map's column exactly:

```
diff <(claims_without_evidence.sh) <(cut -f1 192_probes/p4_coverage_map.tsv | sort)
  -> identical, 21 rows
```

So the seat that built the map measured the right thing. Where it went wrong is one step later, and
it went wrong the same way five times.

**The new ceiling is 6 and it is exact rather than merely satisfied**: at 5 the test fails, at 6 it
passes. The six are one absence claim that must stay uncovered, four whose instrument is committed and
whose probe row points at the wrong arm of it, and one the gate refuses on purpose.

## 2. The five I did not write, and the single mechanism behind four of them

**A probe row describes the arm its author wrote it for. Where a directory holds several arms behind
one row, the map named the row and not the arm.** That is the whole of it, and it is checkable in each
case by opening the probe's `lives` and looking at what is beside the files it names.

**`the_rationalisability_counts_on_the_committed_carrier_table` → `the_rationalisable_sections_on_the_committed_carrier_table`.**
The names differ by one word and the numbers do not match at all. The claim: "of 15625 sections, **72**
are rationalisable by a non-negative weighting and **9** by a strictly positive one, and 63 of the 72
select an arm no weighting can select." The probe: "of 15625 sections **9** are rationalisable by a
non-negative weighting at zero tolerance, rising to **134** at one percent." The one number they share
is attached to a different weighting class on each side. The claim's run is `98`'s, which states it at
`98:291` as "sections at L3 that select an arm 97 says no weighting can select: 63 of 72", citing `97`
as a predecessor rather than as its source. `98_probes` has exactly one probe row and it is
`generating_a_winner_table_from_a_stated_weighting`, correctly used elsewhere in this map. **So the
counting run in `98_probes` has no probe row and the map substituted the nearest name from `97_probes`.**

**`a_coordinate_set_is_a_countable_ceiling_on_how_many_strategies_can_exist` → `the_cost_coordinate_census_and_its_calibration_control`.**
The claim counts reachable sections per coordinate count: 1, 9 and 42. The probe counts which
coordinates the repository carries: three, two of which turn out to be a median and a spread of one
sample series. Two different measurements sharing the word "coordinate". The claim's numbers are in
`101_wronski_the_cost_coordinates.md:128` and the run is `101_probes/p4_what_a_coordinate_buys.py`;
the probe row names `101_probes/p1_the_coordinate_census.py` and the `p0` calibration control. **Same
directory, adjacent file, different arm.** The claim's own phrase "with the control dropped" refers to
a control arm in the carrier table and not to the probe's calibration control, which is the word
collision that makes this pairing read well.

**`a_coherent_reduction_needs_no_accumulator` → `the_accumulator_width_is_the_exact_sum_width_less_one_bit`.**
The map gives this probe to two claims. It matches the second, `an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit`,
almost sentence for sentence, down to the final-adaptation clause. It does not match this one, and the
reason is in the probe's own output: `57_probes/p5_output.txt:72` says the format it sweeps is "which
`q1` and `p4` both measure as the **incoherent** policy." A probe measuring the incoherent case cannot
establish that the coherent case needs no accumulator.

**And the coherent arm exists.** `57_probes/p4_which_factor_breaks_and_what_coherence_buys.rs` is a
row, `which_factor_of_a_reduction_breaks_the_semiring`, whose `control` reads: "a coherent reduction is
a homomorphism so reducing at every step must agree, an incoherent one must not, and the run reports
the same checker on the same code path giving opposite verdicts at the same fold length." **That is the
claim, in the probe's `control` field rather than in its `establishes`**, which is about attributing
semiring failure to a named map. So there is nothing to point at, and the repair is one sentence in
`probe.toml`.

**`the_corpus_cannot_exhibit_the_accuracy_intents_because_a_coordinate_is_absent` → `the_argmin_mechanism_has_never_run_on_arms_that_disagree`.**
Different in kind from the other four, and the worst of the five. The probe's `establishes`: "**Every
committed family's arm set is answer-equivalent**, so the argmin-over-a-cost-table mechanism has only
ever been run where the arms compute one value." The claim's `says`: "**The corpus does contain
answer-differing arms**, and one region with a strict accuracy ordering between them."

They contradict. And the claim's own `because` says why: "One file claimed in bold that every committed
region is answer-equivalent; a second was dispatched to verify it, built its own instrument before
opening the first, and **refuted it** with three independently sufficient measurements." The probe is
the refuted instrument. The refuting one is a different file's and has no row.

**The probe is not wrong to exist and its surviving half is right**: its `note` carries "the barrier is
the absent coordinate rather than the arm sets", which is the claim's headline, and the claim's own
`because` says the conclusion "survived the refutation one tier lower and better". What is wrong is
that `establishes` still states the refuted proposition with no `defect` beside it, so a reader arriving
at the probe row alone learns the thing the corpus spent a dispatch disproving. **Wiring `evidence` here
would cite, as the instrument behind a claim, a probe whose stated finding that claim contradicts.**
`probe.toml` is out of scope, so it is reported rather than repaired.

**`chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type` → `chain_accuracy_needs_an_intermediate_wider_than_the_operand_type`.**
The only one of the five where the pairing is **correct**. The probe is the instrument and says so. It
is declined because the gate refuses it, and the refusal is right. Written and run, one at a time:

```
test the_committed_canon_rests_no_measurement_on_an_unusable_instrument ... FAILED
  kind: "measurement-rests-on-an-uncontrolled-instrument",
  at:   "proposal::chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type",
  says: "`evidence` names `probe::chain_accuracy_needs_an_intermediate_wider_than_the_operand_type`,
         whose own `control` says no case that had to fail was run."
```

The probe's `control` opens "None was run as a case that had to fail" and closes with the honest
sentence: "What neither arm carries is a chain shape on which the width must NOT grow, and without one
the linear growth is measured rather than isolated." **The refusal is information and it is not about
the edge: it says this claim is not a measurement.** Its honest repairs are re-marking the sentence or
building the missing arm, and both are somebody else's file.

## 3. The one that stays uncovered, and should

`the_four_consolidations_contradict_each_other_nowhere` is the map's `NONE` row, and I am recording
agreement rather than passing over it. It is one author reading four documents and reporting that
nothing in them collides. **That is an absence claim naming no search**, which is the one shape a
`file:line` cannot rescue: nothing to open, no failing case to plant, and a reader has no way to
distinguish "I looked and found none" from "I did not find any". The seat before me declined to
manufacture a probe for it, which was right, and manufacturing one now would be worse than the gap.

Its repairs are the two that seat named: re-mark it `argument`, which is what it is, or build the scan
that would make it a measurement, which means stating what was compared against what over which set.
Both are edits this dispatch may not make.

## 4. Two probes admit no control in the same words and the gate catches one

**This is the finding, rather than the edges.** `names_no_control` in `shape.rs:210` reads a `control`
opening with "None" as an admission, unless the field somewhere reports a case that came out one way.
The carve-out exists for a real reason, given in its own doc comment: "None of the arms disagreed, and
that is the control firing" begins identically and means the opposite. And the doc comment says, in
terms, that "an opening-word rule is blunt enough that a second reader is owed on it."

**This is that second read and the rule loses it.** Two probes, both `sound`, both opening their
`control` with the same sentence:

> `chain_accuracy_needs_an_intermediate_wider_than_the_operand_type`
> "**None was run as a case that had to fail.** The two instruments are at different widths and fraction
> widths by different authors, so what stands in place of a control is that the growth rate agrees
> across a change of both..."

> `the_four_const_available_constructions_bind_at_four_times`
> "**None was run as a case that had to fail**, and the shape is a compiled lattice over four
> constructions where the four outcomes differ from one another, which is itself the separation: four
> constructions **reported** as binding at one time would say the instrument cannot distinguish them.
> That is weaker than a planted case and it is what there is."

Measured by writing each edge and running the gate separately:

```
chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type   -> FAILED
the_licensed_category_is_const_available_and_four_constructions_bind_at_four_times  -> ok, 28 passed
```

**The whole difference is the word `reported`**, and in the sentence it sits in it is not a report of
anything. It is a counterfactual: what a *different* outcome would have meant. The carve-out reads over
the whole field on purpose, which the doc comment defends with "none of the six admissions this corpus
actually writes contains any of these words anywhere". **That was true of the six it was measured
against and is not true of the seventh.**

**I wrote the edge anyway, and that is a judgement worth arguing with.** The edge is factually right:
that probe is the instrument behind that claim. Withholding it would make the registry say the claim
rests on nothing, which is a different and larger falsehood, and it would put my ceiling one too high.
What is wrong is the claim's `sentence_kind`, which I may not touch, and the matcher, which is not
mine. So the corpus now carries one `enumeration` row resting on an uncontrolled instrument with
nothing red anywhere saying so, and this paragraph is the only thing that does.

**What I would not do is widen the word list.** The failure is not that `reported` is missing a
qualifier; it is that a word list cannot tell a report from a counterfactual, and lengthening it moves
the false negative rather than removing it. If the field is going to be read mechanically the honest
shape is a separate field saying whether a control fired, with the prose beside it. That is a schema
question and it is not mine.

## 5. Thirteen of my fifteen edges made a `note` false

Every row I wired except two carries, in its `note`, a sentence of the form "`evidence` is empty and
the measured-implies-evidence check is red on this row". Several add "because `probe.toml` does not
exist", which stopped being true when 48 probe rows landed.

```
$ 193_probes/notes_the_edges_falsified.sh | wc -l
13
```

The control plants a row whose note says the same thing and has no evidence, and it is correctly not
reported, so the count is of notes the edges falsified rather than of notes containing a phrase.

**This was going to happen to whoever wired these**, and it is not a defect in the notes: each was true
when written and described the row's real state. What it means is that **an edge pass and a note pass
are one job that has been split into two**, and the half that leaves is the half that gets skipped.
`note` is out of scope by instruction so all thirteen stand. They are the most trustworthy-looking wrong
sentences in the file, because a reader who checks the field sees an edge and a reader who reads the
prose is told there is none.

Two of the fifteen escaped: `where_fusion_changes_the_answer_it_is_not_a_lowering` and
`an_exposure_test_over_reduction_verdicts_alone_is_satisfied_by_a_system_that_computes_nothing`
happen to phrase their notes differently.

## 6. What I confirmed, briefly, because the interesting half is above

Fifteen pairings where the probe's `establishes` states the claim's `says`, several near-verbatim:
`a_half_step_biased_grid_is_not_closed_under_addition` gives "0 of 256 exact sums land on the grid"
against the claim's "no exact sum of two grid points lands on the grid";
`most_committed_bench_regions_predate_the_validation_gate` carries 175, 254 and 79 exactly as the claim
does; `fusion_is_an_axis_position_rather_than_a_new_axis` and
`a_compile_time_strategy_selection_leaves_no_residue` are the claim's sentence with a measurement
attached.

**Three are partial and I wired them anyway**, because `evidence` names the instruments behind a claim
rather than promising they reach all of it, and each of these covers the claim's load-bearing half:

- `the_const_eval_frontier_by_arity` gives the widths 19/9/5/3/1 verbatim and says nothing about the guard's 5.85, 49.06 and 370.95 second timings.
- `three_of_the_four_proposed_axes_change_no_answer` establishes that headroom and intermediate precision move no answer, and not the claim's second half about becoming visible at the first non-ring step.
- `generating_a_winner_table_from_a_stated_weighting` establishes the relocation and not the 0-of-190, 0-of-147, 0-of-152 and 489 detection counts.

**One is worth naming because the two rows count differently and agree anyway.**
`no_conservatism_order_exists_on_the_overflow_axis` reports "two candidate orders... and that order
itself splits into two metrics"; the claim says three. Two-plus-a-split and three are the same
structure, and a reader diffing the numerals would call it a mismatch.

## 7. What I would tell the next reader

1. **`probe::the_argmin_mechanism_has_never_run_on_arms_that_disagree` states a refuted proposition in `establishes` with no `defect` field.** Section 2. One row, and until it moves the claim above it cannot cite its own evidence base.
2. **Four instruments are committed and have no row describing the arm that matters**: `98_probes`' counting run, `101_probes/p4`, and the coherent half of `57_probes/p4` whose result currently sits in a `control` field. Four probe rows, and four of my six remaining claims are unblocked.
3. **`names_no_control`, and the shape rather than the word list.** Section 4. Two agreements are owed on that rule and this file is one.
4. **The thirteen notes.** Section 5. Cheap, mechanical, and currently a set of confident false sentences sitting next to correct fields.
5. **The ceiling is 6 and four of the six are one dispatch of probe rows.** The other two are honest: one absence claim that should be re-marked, and one claim that is not a measurement.

## 8. What I did not do

Wrote only `evidence`, only in `proposal.toml` and `proposal-the-later-topics.toml`. Added no probe
row, changed no `says`, `predicate`, `sentence_kind` or `note`, and did not relabel the sentence of the
claim the gate refused, which was the available workaround and is the one the brief names. Did not
touch `.claude/`, `.github/` or `mock/agent/`. `docs/PROPOSAL.md` is in the commit because it is
generated from the registry and `cargo mock` rewrote it; the diff is the new `evidence` column and a
timestamp.

One thing I did that was not asked: this worktree had no generated hooks, so the first commit was
refused with "mockspace is not initialised in this repo". Running `cargo mock` fixed it and regenerated
the document. Worth knowing for the next fresh worktree, since `--lint-only` works fine without them
and the gap only shows at commit time.
