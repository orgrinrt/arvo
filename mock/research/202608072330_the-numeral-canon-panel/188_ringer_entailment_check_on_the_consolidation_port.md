# 188. Entailment check on the consolidation port

**Position:** the independent check on `182`, and on the 74 `proposal` rows and 14 `law` rows it landed.
**Author:** the `ringer` persona. **Probes:** `188_probes/`, ten of them, committed as they ran, each with the
case that had to fail.

**Canon gate: passed.** Checked against `181` (op's own words on the registry becoming the canon), `INTENTS.md`
for I13, and `mockspace.toml`'s declaration of `mock/registry/*.toml` as `canon_paths`. Running an entailment
check on a compression is what `RULES.md` and `87` require and what `182`'s own closing paragraph asks for. I
wrote no row and edited none.

**Test gate: passed.** 50 checks green, one ignored with a catalogue reason naming `probe.toml` as the unbuilt
mechanism. `cargo mock --lint-only` clean, 495 rows across 9 namespaces. I read the bodies of the six test files
under `mock/checks/tests/` rather than their names, and `src/shape.rs` and `src/predicate.rs`, because two of my
instruments turn on what those functions select on. **One of them is decorative and section 7 is why**, but it
is decorative in a way nobody could have prevented and it is not grounds to refuse the assigned work.

---

## 1. Verdict

**Sound, with repairs named.** Not one claim that `64`, `75`, `91` or `107` refuted survives into a row. I went
looking for that specifically, because the brief called it the worst thing available, and the reason it is not
there is structural rather than lucky: **all four consolidations were repaired after their checks landed, and
the port read the repaired files.** `182`'s stated method held up under every test I could put on it, its
self-flagged weaknesses are real and are the right ones, and the one severe finding I drafted against it I had
to withdraw when I checked it.

Against that, four things:

- **`97`'s F-F, the criterion that makes an infinite family of laws decidable from a finite table, is in no
  row.** Its own author calls it the one he would most like carried. It is writable today. Section 4.
- **`106`'s repair pass claims to account for what its check left unfixed and accounts for three of six.**
  `182` inherited that undercount and says so honestly, having only `106` to go on. Section 4.
- **Two law rows state a conjunction over a width-and-sign product with one cell no cited instrument reached.**
  Section 6.
- **The panel reference root is declared frozen and all four consolidations were edited.** Section 8. The rows
  are unaffected; the two entailment checks are not.

**And two claims in my own brief are false.** Section 2, before anything else, because I was sent partly on the
strength of them.

---

## 2. Breaking the brief first

**Both errors are cheap to check and I checked them before doing anything else.**

**The brief says: "It found four axes nothing declared. Two of them, and two more, have since been declared
(`ambient_domain`, `radix`, `accumulator_width`, `toolchain`). Check whether any of its unportable claims are
now writable."**

**None of `182`'s four is declared.** Section 5.1 names `declared_operand_window`,
`representable_range_geometry`, `encoding` and `constant_embedding_convention`; the axis pass at `c765e8c1`
landed a disjoint four. `188_probes/axes_182_needed.out`, whose control reads `yes` on the four that were
declared, so the four `no`s are about the file and not about the grep.

**So the answer to the question the brief asked is: no section 5.1 claim is now writable, and the axis blocking
the most is still `declared_operand_window`**, which is what R7 and F-H both turn on and which `106` section 16
restored precisely because it is the unit's only positive law result. A different claim did become writable and
section 5 is that one.

**The brief says: "`54` and `75` [are the entailment checks] on `63` and `74`."** `54` is the check on `53`, the
container-derivation consolidation, which is a fifth consolidation and not among the four ported. The check on
`63` is `64`. I read `64` and did not read `54`.

---

## 3. Instrument 2, the anchor set difference, printed

`188_probes/anchor_set_difference.out`. Normalised to the target rather than the spelling, so `93` and
`` `93:611` `` and `93_probes/p7` compare as `93` and `93_probes`. Tier per
`a-compression-is-checked-by-someone-else.md`: these rows are canon candidates, so panel-internal and probe
anchors count and citations into the deleted crate tree do not and are not to be restored.

**The rows carry six distinct targets between them:** `63`, `74`, `90`, `106`, `108`, `AGREEMENTS`.

```
63_spj_consolidation_the_format_concept.md        41 distinct anchors,  0 survive
  00 04 08 18 20 22 23 25 28 35 35_probes 40 42 42_probes 43 50 51 53 54 55 55_probes 56
  56_probes 57 57_probes 58 58_probes 59 60 60_probes 61 61_probes 62 62_probes 63_probes
  64 7 DROPLIST.md INTENTS.md OPTIONS.md RULES.md

74_giesen_consolidation_the_number_system_concept.md  23 distinct,  1 survives
  00 01 16 35 51 55 62 64 65 66 67 68 69 70 71 72 73 75 DROPLIST.md INTENTS.md OPTIONS.md RULES.md

90_giesen_consolidation_derived_algebraic_laws.md  38 distinct,  2 survive
  135 2 28 35 42 55 57 60_probes 68 73 76 76_probes 77 77_probes 79 79_probes 80 80_probes
  81 82 82_probes 83 84 84_probes 85 86 86_probes 87 88 89 89_probes 91 DROPLIST.md
  INTENTS.md OPTIONS.md RULES.md

106_giesen_consolidation_the_strategy_axis.md      41 distinct,  2 survive
  0 1 100 100_probes 101 101_probes 102 102_probes 103 104 105 106_probes 107 2 22 25 35 40
  6 82 83 87 88 9 93 93_probes 94 95 96 97 97_probes 98 98_probes 99 DROPLIST.md INTENTS.md
  OPTIONS.md PRIOR_CALLS.md RULES.md

AGREEMENTS.md                                      77 distinct,  5 survive
  00 01 04 08 103 104 105 107 109 111 112 113 114 117 119 122 123 124 125 132 136 137 138
  139 146 151 152 153 156 161 164 165 166 173 176 177 178 28 32 34 35_probes 36 37 38 39 42
  43 55 56 57 57_probes 60 61 62 65 66 67 68 76 77 80 83 84 85 86 87 88 95 DROPLIST.md
  INTENTS.md OPTIONS.md RULES.md
```

Both controls fired: every source self-diffs to zero, so the extractor is stable, and a target no source
contains is absent from the kept set.

**How much of this is a defect and how much is design.** Every probe directory in that list is where `evidence`
was meant to point, `probe.toml` does not exist, and the brief allocated it elsewhere. That is the catalogue-red
and it is honest. What is not covered by it is the member files: **`93_probes`, `94_probes` and `103_probes`
were already at zero anchors carried in `106`, `107` reported that as unrepairable by editing `106`, `106`
section 17 agreed, and the port has now inherited it one level further down.** A reader of a row about the
strategy object has `106` and nothing beneath it.

**The depth arm is weaker than it first reads and its control is why.** `188_probes/citation_depth.out`: of 141
distinct line citations, 53 land on a window carrying no onward panel citation. A control reading 400 lines past
every citation gives 51. **So that rate is a fact about how densely these documents cite, not about which lines
the rows chose.** What it does establish is that the rows do not select for evidence-bearing lines at all, which
is a smaller claim and the one the measurement supports.

---

## 4. Instrument 1, entailment from the original forward, and what it found

**Working from `106` backward you cannot find this. There is no heading where it would go.**

### 4.1 `97`'s criterion reached no row, and three links lost it

`188_probes/the_criterion_reached_no_row.out`. At `97:707-709`:

> a law holds in the representable set **iff** it is an identity of exact arithmetic **and** `pi` respects
> every ordered nesting of operations the law contains.

552 cells, zero soundness mismatches, zero conservative mismatches, both directions counted separately on
purpose. `97`'s own account of what it buys: the retraction table has `|ops|^2` entries and the space of
identities is infinite over every arity, **so deciding the finite table decides the infinite family**, and it is
the general form of the route `OPTIONS.md` records as never asked of any law this panel measured.

The three links, each measured:

1. **`106` carries the score and not the content.** `552 cells` appears once; the criterion's content appears
   zero times. That is `107` section 4.4 exactly.
2. **`106`'s repair pass did not fix it and did not record it as unfixed.** Section 17 is headed *"What the
   check found that is not repaired here, and why"* and lists three items. `107` recorded six. The criterion,
   the `bitpack-shared` doc-comment misattribution and the over-strong section 0.3 heading are in neither
   section 16 nor section 17. **A section claiming to enumerate what was left unfixed, that enumerates half of
   it, is the shape `a-claim-of-totality-names-what-enforces-it.md` names**, and it is what `182` then quoted:
   its section 3 says "three further defects that stand unrepaired", taking `106`'s count at face value, which
   is the honest thing to do with the only source you have.
3. **No row carries it.** Zero content hits across `proposal.toml` and `law.toml`. The control, a different
   `97` claim that did reach a row, returns non-zero, so the search finds `97`'s claims where they are present.

**It is not the `coherence_of_a_reduction_onto_its_induced_operation` law row in different words.** That row
states one instance of the property at `W = 4` unsigned. F-F is the decision procedure built on it.

**It is writable today.** Nine of the ten axes its region names are declared; the tenth is
`declared_operand_window`, and its entry names both of that axis's values, which makes leaving it out the blind
spot `182` section 5.1 already accepted for R12's container half rather than a negative claim.

**This is the single thing I would restore first.**

### 4.2 What each of the four checks found, and where it stands now

`188_probes/did_the_port_carry_the_repairs.out`, with every hit and miss then read by hand, because a keyword
hit is not a claim.

| check | finding | in the consolidation now | in a row |
|---|---|---|---|
| `64` | D-C, the chain as a first-class typed object, dropped entirely | restored, `63:585-592`, and `63:922` records the drop | yes, the three-way direction question is carried |
| `64` | the knob/visible material and `55`'s Alternative B/C labels | partly | no, and both are bookkeeping |
| `75` 2a | `65`'s change-test misfiles its own chain | restored per `df98f2e1` | **no** |
| `75` 2b | `66` uncredited on N9 against `74`'s own merge policy | restored | **no** |
| `75` 2c | the typestate cannot break the tie | restored | yes, in `an_order_is_named_exactly_where_a_crossing_is_lossy` |
| `91` 1 | R8 misattributes which instances share an author | repaired, `90:346-357` | yes, and the row records the repair accurately |
| `91` 2 | `76`'s Precise-refines-Hot ordering candidate dropped | restored | vocabulary present; I did not confirm the claim is the same one |
| `107` 4.1 | the law bullet unpredicated and false at signed saturating | repaired, and `106` records it as the third loss | **yes, and correctly**: section 5 |
| `107` 4.2 | four one-file results missing from the droplist | all four restored in `106` section 16 | three of four reach a row; the exchange-rate reading does not |
| `107` 4.3 | the `87` half-quotation | recorded unrepaired in section 17 | n/a |
| `107` 4.4 | the criterion cited by score, never stated | **not repaired, not recorded** | **no**: section 4.1 |
| `107` 4.5 | `bitpack-shared` doc comment attributed to the module | **not repaired, not recorded**, still at `106:126` | no, nothing ports it |
| `107` 4.6 | section 0.3's heading stronger than its body | **not repaired, not recorded**, still at `106:111` | no |

**Nothing refuted survives.** The two that could have hurt are `107` 4.1 and `91` 1, and both are carried in
their repaired form. Section 5 is the first of those in detail, because it is the one that would have been a
licence to emit a wrong rewrite.

---

## 5. Instrument 4, predicate dimensions, and the one that went right

**The distributivity row is the best thing in either file and it should be said before the defects.**

`107` section 4.1 is the severest finding in the corpus: `106` section 3.1 stated *"Multiplicative associativity
and distributivity hold at `F = 0` and fail at `F > 0`"* with no predicate, dropping `signedness = unsigned`
from `93`'s F1, while citing as support the work that refutes it. The consolidation was repaired. **The row
carries the repair whole:**

- `holds` names `signedness: signedness = unsigned`, `fails` names `signedness: signedness = signed`.
- `witness` carries both models: `93_probes/p7` at 47.72% and `W = 7`, `97_probes/p2` at 34.52% and `W = 6`,
  with the mechanism, that a one-sided clamp is a congruence and a two-sided one is not.
- `note` states *"`F = 0` is necessary and it is not sufficient, and the unqualified form of this sentence is
  false"* and records that this is the third loss of the qualifier in the panel's own history, naming all three
  sites including the live workspace rule.
- `gap` names the F-H cell as established, on neither list, and why the notation cannot hold it.

**A dimension dropped three times in one panel's history is carried on the fourth, with its own loss record
attached.** That is the discipline working, and it is worth more than any defect below.

I diffed the dimension sets of every law row against the source region I could locate. **I found no other
widening of the drop-a-dimension kind.** The two problems I did find are a different shape.

---

## 6. A conjunction claims a cell nobody swept

`188_probes/a_conjunction_claims_a_cell_nobody_swept.out`. `predicate.rs` reads a predicate as a conjunction,
one entry per axis, so a list naming a set on two axes claims the whole product.

**`additive_associativity_under_wrapping`** holds at `W in {4, 8}` and `signedness in {unsigned, signed}`, which
is four width-and-sign cells. The cited instruments are `63`'s cube (signed add wrap, at four-bit widths) and
`76`'s exhaustive sweep, which `90:122` describes as *"wrapping addition associates universally over `u8`"*, so
unsigned. `93`'s F10 covers signed wrap at `W in 3..7`. **Signed at `W = 8` is in none of them.**

**`multiplicative_associativity_under_wrapping`** holds at `W in 3..=8` and `signedness in {unsigned, signed}`.
`93`'s F1 is `signedness = unsigned` at `W in 3..8`; F1a extends it to `W any` and is still unsigned; F10 covers
signed at `W in 3..7`. **Signed at `W = 8` again.** And **the row's own note says the F1 sweep is "for the
unsigned case"**, so the gap is disclosed in prose and asserted in the field a gate reads.

The control matters here: applied to unsigned at `W = 8` under wrapping addition, the same argument finds the
cell covered by `76`'s exhaustive sweep, so it discriminates rather than rejecting every cell.

**This is not a claim that the laws fail there.** They very likely hold. It is that `182` section 8.1 found the
case where the *verdict* inverts across a cell and solved it by splitting the law, and this is the case where
the *evidence* does not cover the product. Nothing splits, nothing fires, and the predicate reads as exact.

**And one narrowing, recorded so the directions are not confused.** `93`'s F1a proves the unsigned `F = 0`
multiplicative half at `W any` by a congruence argument rather than a sweep. The row writes `W in 3..=8`, which
claims less than the source. That is honest under the notation. The saturating sibling's note discloses the
same trade explicitly; the wrapping row's note does not mention F1a at all.

---

## 7. The 43 unpredicated rows

`188_probes/normative_rows_that_measure.out` and `normative_rows_with_no_law_anchor.out`. The mechanical tell is
that a stipulation's justification does not report a measurement; the control fires on 15 of the 18 rows already
marked `measured`, so it detects measurement. The refined tell, after section 9's withdrawal, adds: **and no law
row holds the region.**

**The finding that governs all 43 is not about any of them individually.** `shape.rs` carries two regionless
kinds, `normative` and `definition`, and the corpus uses `normative` 43 times and `definition` zero times.
**The `definition` kind postdates the rows by twenty-three minutes** (`c765e8c1` at 01:59:31 against `6142ce47`
at 01:36:48), so `182` section 6.2's account of the choice, that it lay between labelling a stipulation
normative and dropping the row, was true when written and is not true now. **Roughly two thirds of the 43 are
stipulations and now have a correct label available.** That is bookkeeping, and section 7.1 is what it costs.

Rulings, one line each, in file order. **I is imposition, D is stipulation now mislabelled, C is a claim wearing
the exempt mark.**

1. `arithmetic_on_a_format_factors...` **D.** "Arithmetic on a format is X" is a stipulation.
2. `a_format_is_identified_by...` **D.** An identity criterion, stipulated.
3. `membership_of_the_representable_set_is_one_affine_predicate` **D.** A parameterisation, stipulated.
4. `the_adaptation_slot_is_derived_and_a_strategy_selects...` **D**, with a derivability claim inside it.
5. `a_chain_is_exact_operations_together_with_a_schedule...` **D.**
6. `the_format_concept_carries_three_things_upward...` **I.** What the concept owes the layers above.
7. `the_numeral_concept_is_a_dependent_sequence_of_choices` **D.** The topic's central stipulation.
8. `every_canon_sentence_names_the_prefix_it_quantifies_over` **I.** A rule about canon prose.
9. `derivation_is_completion_of_the_sequence_by_the_typestate` **D.**
10. `a_crossing_carries_two_relations_and_a_verdict...` **D.**
11. `a_crossing_preserves_an_operation_exactly_when_it_moves_no_coordinate...` **C.** A biconditional, refutable
    by one counterexample. Not an imposition on any reading.
12. `meaning_is_decided_by_the_first_three_coordinates...` **C.** "those two can never change what it computes"
    is a universal negative.
13. `a_crossings_preservations_are_the_two_law_families` **C.** Two more biconditionals, "exactly when monotone"
    and "exactly when coherent".
14. `an_order_is_named_exactly_where_a_crossing_is_lossy` **C+I.** The first two sentences are theorems, the
    last is the rule, and the row is labelled for the last. Same shape as 32.
15. `conversion_and_resolution_are_one_obligation_at_two_arities` **D.**
16. `membership_and_hosting_are_two_questions` **D.**
17. `a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts` **I.** An admission obligation.
18. `a_closed_ambient_operation_makes_the_reduction_the_identity` **C.** Provable from the definitions, and the
    row states it as a consequence.
19. `the_concepts_edge_is_not_an_order_and_wrapping_is_the_test` **C.** "varies within a single family" and
    "groups a wrapped integer with a bit vector" are structural facts.
20. `admission_returns_a_coordinate_rather_than_a_verdict` **I.** A rule about how to ask.
21. `the_concept_commits_to_its_choices_and_to_no_count_of_levels` **C+I.** "Both partitions are real and
    neither refines the other" is mathematics; the commitment is the rule. Flagged by the measurement tell.
22. `each_choice_in_the_sequence_has_an_owner_and_a_resolution_time` **D.**
23. `validation_is_two_acts_keyed_by_boundary_and_it_validates_maps` **I**, with the measurement as its evidence.
24. `erasures_guarantee_is_a_conditional_with_a_stated_base` **I.** What the canon must list.
25. `one_container_hosts_many_systems_so_the_canon_types_the_system` **I**, on a stipulated premise.
26. `the_concept_is_closed_and_the_inventory_is_open` **I.** An admission policy.
27. `roles_derive_representations_and_a_realisation_variant_computes_nothing_new` **C.** "cannot change what
    anything computes" is a theorem, and it follows from 12.
28. `the_two_law_families_have_two_consumer_classes` **C.** "all four combinations occur" is an existence claim.
29. `every_dispute_in_the_number_system_topic_was_a_dispute_about_an_address` **C**, defensibly. A universal over
    a corpus; its `because` defends it as a composition of four earlier sentences rather than a new claim, which
    is the best available argument and I would not fight it.
30. `a_law_is_a_fact_about_an_operation_composed_under_a_fixed_arithmetic_semantics` **D.** `182` says so itself.
31. `arvo_owes_laws_as_checkable_facts_and_not_as_a_rewriting_engine` **I.** A locus refusal, correct.
32. `a_composed_expressions_region_is_never_inherited_from_its_parts` **I**, and section 9 is my withdrawal of
    the opposite reading. The measured half is the law row it wires to and that row carries the full region.
33. `a_const_eval_frontier_is_a_fact_about_the_procedure...` **C+I.** The first clause is factual and the second
    is the rule; defensible as written.
34. `a_law_layer_answers_whether_a_law_reaches_a_lowering_the_backend_cannot_prove` **C. The strongest of the
    43.** `182` says it is the one it is least comfortable with and I agree, and it is now repairable: the
    instrument's own header names `aarch64-apple-darwin`, `rustc nightly-2026-05-28` and `-O`, and
    `target_features`, `toolchain` and `build_profile` are all declared. The counts stay out, correctly, because
    the instrument forbids them; the qualitative claim can carry a region. `188_probes/the_seventh_claim_is_now_writable.out`.
35. `an_instrument_is_mutated_and_the_battery_is_made_to_notice` **I.** Genuinely the discipline.
36. `a_strategy_is_a_pair_of_an_observable_assignment_and_a_weighting` **D**, and superseded.
37. `a_strategy_is_a_declared_semantics_together_with_a_weighting...` **D.** Section 7.1.
38. `a_region_whose_arms_may_differ_is_validated_arm_by_arm...` **I.** The measurement is the evidence for the
    rule, not the rule. Accept.
39. `what_ships_is_four_arms_with_disjoint_predicates...` **I.** A shipping instruction.
40. `a_predicate_carries_every_dimension_it_depends_on...` **I**, and it wires to a law row.
41. `arms_with_const_predicates_are_the_organising_shape_all_four_topics_reached` **C.** The first sentence is
    I13, which is op's and is an imposition. **The second is an empirical claim about four documents**, at
    `cross_topic`, the strongest standing in the enumeration, on a row exempt from carrying a region.
42. `a_strategy_is_a_preference_over_measurements_resolved_as_a_compile_time_argmin` **D.** Section 7.1.
43. `the_named_strategies_are_points_in_a_product_and_the_flat_set_is_a_slice` **D.**

**Count: 14 impositions, 16 stipulations now mislabelled, 13 rows carrying a claim** (11, 12, 13, 14, 18,
19, 21, 27, 28, 29, 33, 34, 41; three of those are mixed and are counted here). **Of the 13, one is repairable
today with a real region (34), and the rest want either `definition`, a split into the measured half and the
rule, or a region I could not locate.**

**None of them is a fabrication and none reads as label-shopping.** `182` said the incentive ran one way and
named the six it felt it on; four of those six are on my list and two are not, and I found six it did not name.
That is what an independent read is for.

### 7.1 What the label cost, mechanically

`188_probes/the_definition_check_sees_nothing.out`.

**`the_committed_canon_defines_no_term_twice` is green over an empty selection.** It reads `defines` on rows
marked `definition`; there are zero of each in 74 rows.

**And the corpus does define one term twice.** Three rows state what a strategy is:
`a_strategy_is_a_declared_semantics...` (supersedes the pair row), `a_strategy_is_a_preference...`, and
`the_named_strategies_are_points_in_a_product...`. The first two are live rivals and neither supersedes the
other.

**My case that had to fail did not fire, and that is the second half of the finding.** I relabelled the two
rivals on a copy so the selection was non-empty. No finding appeared, because at `shape.rs:214` the check skips
**any** row carrying a `supersedes`, and the declared-semantics row supersedes the *pair* row while rivalling
the *preference* row. The skip discards it on the strength of the first fact while the second is the thing the
check exists for.

**So the check has two independent reasons it cannot see this, and fixing the labels alone would not surface
it.** The `supersedes` skip wants to be per-term rather than per-row.

---

## 8. The frozen root is not frozen

`188_probes/frozen_root_is_not_frozen.out`. `mockspace.toml:311` gives the reason for `frozen = true`:

> a numbered panel file is written once and never edited, and that is what makes a line citation into it honest
> rather than a hazard

**Every one of the four files every registry citation lands in was edited after landing:**

| file | commits | first line touched | net shift below it |
|---|---|---|---|
| `63` | 2 | 165 | +54 |
| `74` | 3 | 28, then 218 | +17, then +33 |
| `90` | 2 | 343 | +42 |
| `106` | 3 | 320, then 1400 | +86, then +45 |

**The rows are unaffected**, because every repair predates the port by weeks and `182` read the current files.

**The two entailment checks are not.** `91:85` cites `90:346-348` and quotes the sentence it is reporting as
false. Those lines today carry the paragraph announcing that the sentence was repaired. **The citation resolves,
the linter passes it, and it points at the opposite of what it was quoting.** That is the whole hazard the
freeze declaration exists to deny, demonstrated on the file that declares it.

**A second, smaller one: `106` has two sections numbered 16**, at lines 1294 and 1343. A heading anchor into
section 16 of `106` is ambiguous, and heading anchors are what the config recommends over line numbers.

---

## 9. My own severe finding, withdrawn

`188_probes/r2_had_its_region_all_along.sh`, kept as a withdrawal rather than deleted.

I drafted this as the sharpest finding in the report. `a_composed_expressions_region_is_never_inherited_from_its_parts`
is `normative`, carries no predicate, and its source states a full region two lines below the sentence its
`because` quotes, at `90:136-137`, on seven axes every one of which is declared. `182` section 6.1's defence for
the label is *"the honest alternative is `theorem` with no region, which the checker refuses, so the row would
not exist"*, and that defence would then be false.

**Every fact in that is true and the conclusion is wrong.** The row wires
`law = ["associativity_of_a_composed_saturating_add_and_subtract"]`, and that law row's `fails` list carries all
seven axes in the same order with the same values, and cites `90:137` in its provenance. The row's own `note`
says exactly that: the measured claim is the law row, this row is the rule drawn off it. The landing commit's
message is *"splitting each measurement from the rule drawn off it"*, and it did.

**Caught by the verification step, not by review.** The brief warned that one of nine findings on a prior check
was half wrong in this exact way, and acting on my draft would have produced a repair pass duplicating a region
that is already stored once, correctly.

**What survives is small and is about `182` rather than about the rows:** section 6.1's stated defence is weaker
than the one the row itself carries, and a reader working from `182` alone concludes the region was unavailable
when it is one field away.

**And one instrument defect fell out of the same investigation, honestly reported as costing nothing.** `182`'s
headline census pattern is `[Pp]redicate:` with the colon adjacent, and `90:136` spells it "Predicate as `79`
stated it:". A wider pattern finds 24 against the census's 11.
`188_probes/the_census_pattern_missed_regions.out` reproduces the census's published table exactly before it
diffs, so it is comparing against what `182` ran. **Of the 13 the narrow pattern misses, eleven are prose using
the word, one is R7's axis-blocked region already known, and one is `90:136`** whose content was carried
anyway. So the census's 11 is not off by 13; it is off by one that cost nothing, and saying which is the whole
of the finding.

---

## 10. Absence claims, re-run last and separately

`188_probes/absence_claims_rerun.out`. Searches executed again rather than outputs re-read, each with a positive
control, because a structural claim degrades while an absence claim inverts with its own text unchanged.

**All three hold.**

- **`the_multiplicative_guard...` gap: "No bench harness ran on any of it and every member says so."** `62:381`
  says *"no bench harness ran"* in terms. `60:210` says *"That is a statability argument, not a benchmark"*.
  Control: both files return 45 and 40 hits for "probe", so the grep reaches them.
- **`a_chain_is_exact_operations...`: "nobody in the unit attacked it."** Three files in the format unit carry
  the vocabulary. I opened all three: `55:242` states the same thing, `58:310` and `58:320` name the precondition
  as consistent with it. **None is an attack.**
- **`an_additive_verdict...`: "nobody in the first five files of the unit had named it."** Exact. Zero hits for
  the scale premise across `55`, `55b`, `56`, `57`, `57b`; four, three, three and nine across `60`, `61`, `62`,
  `63`. **The cleanest absence claim in either file.**

---

## 11. Confirmations rather than discoveries

Marked separately because the two are worth different amounts.

- **The 43 normative rows are the port's largest weakness.** `182` section 8 says so and section 14 item 2 tells
  the next reader to start there. It was right and section 7 is the audit it asked for.
- **`a_law_layer_answers_whether_a_law_reaches_a_lowering...` is the least defensible label.** `182` section 6.1
  names it in those words. **What is new is that it is repairable now** and was not when `182` ran.
- **The 18 rows red on missing evidence are a catalogue-red and the fix is `probe.toml`.** `182` section 9 is
  right, the table there is buildable from, and the check has since been marked ignored with a reason naming the
  unbuilt namespace, which is the correct handling.
- **`182`'s section 5.2 self-correction is the most useful thing in the file** and it generalises further than
  it claims. Its own words: the panel knows the regions, they are in the instruments, and the compression from
  instrument to consolidation is where they were lost. **The measurement of how much: 82 `holds for:` lines
  across the 34 member files, every one of them in the strategy topic**, against the 11 the four consolidations
  carry. `188_probes/predicated_findings_census.out`, with controls both ways. The other three topics' members
  write none, because the notation postdates them, which is why their consolidations have none to lose.
- **The `AGREEMENTS.md` near-miss is real.** `182` section 10 flags that six `cross_topic` rows rest on a ledger
  whose author opened no member file. My anchor diff shows the same ledger citing 77 distinct targets of which
  five reach a row. Both readings point the same way.

---

## 12. Coverage, bounded

**Read end to end:** `182`; `64`, `75`, `91`, `107`; `106` sections 16, 17 and 18; `181`;
`mock/registry/proposal.toml`, `law.toml` and `dimension.toml`; `mock/checks/src/predicate.rs` and
`src/shape.rs`; the six test files under `mock/checks/tests/`.

**Read in the part a finding of mine turns on:** `63` sections around the cube and the D-C restoration; `74`
around N9; `90` R2 and R8; `93` F1, F1a, F10; `97` sections 6 and its findings block; `76`, `55`, `58`, `60`,
`62` at the passages the absence re-runs named; `AGREEMENTS.md` only through the anchor extractor.

**Not read at all:** `54` (which is the check on `53`, not on `63`); `53`; every member file of the format,
number-system and derived-laws topics except the passages above; `OPTIONS.md`, `DROPLIST.md`, `RULES.md`,
`PRIOR_CALLS.md`, `INTENTS.md` beyond I13; `probe.toml`, `ruling.toml`, `question.toml`, `retirement.toml` and
the two `*-the-later-topics*` files, all per the brief.

**Verified by running rather than by reading:** the suite and the linter; the twenty declared axes and which of
`182`'s four are among them; the anchor set difference with both controls; the citation depth with a shifted
control; the 82 member `holds for:` lines with both controls; the census pattern comparison, reproducing `182`'s
published table first; the three absence searches with positive controls; the `definition` selection being empty
and the relabel not firing; every git history claim in section 8.

**Verified by opening the target:** every `file:line` I quote in this file. The instrument generated a five-line
window for all 159 citation instances across both registry files, with the owning row and field, at
`188_probes/cited_context_*.out`, and its control arm reading 400 lines past every citation produces different
windows, so the fetch moves. **I read a few dozen of those windows rather than all 159 and I did not count
which**, so treat the file as an instrument I built and sampled rather than as a completed citation audit.

**Not verified, and named.** I ran no probe of the panel's and reproduced no measurement. Every percentage,
count and width in a row's `witness` or `because` is carried from a consolidation which is itself carrying it
from a member, and I checked the *dimensions* of those claims rather than their *values*. Where a row says
47.72% or 13,882,880 or 82.7484%, I have confirmed only that the source says the same number in the same region.
**I re-derived no standing**, adjudicated no located disagreement, and read no probe source beyond the four
output headers section 4 and 5 quote.

**What no instrument of mine could check**, and it is most of sections 4, 6 and 7: whether a cited passage
supports the argument put on it, and whether a sentence is an imposition or a claim. That is judgement, and it
is why this should not be the last read of these rows either.

---

## 13. What I would repair, in order

1. **Write `97`'s F-F as a row.** Section 4.1. The material is complete, the region is writable, and its author
   calls it the one he most wants carried. Nothing else on this list buys as much.
2. **Relabel the stipulations `definition` and give them a `defines` field**, and fix `shape.rs:214` so the
   `supersedes` skip is per-term. Section 7.1. Without the second, the first surfaces nothing.
3. **Give row 34 its region** and move it off `normative`. Section 7, item 34. Three axes, all declared.
4. **Split the two conjunctions**, or narrow them to the widths their instruments ran. Section 6.
5. **Record `107`'s 4.4, 4.5 and 4.6 in `106` section 17**, so the next reader of that section gets six rather
   than three. Not a repair to the rows.
6. **Restore `75`'s 2a and 2b to rows**, or state why they belong in `question.toml`.
7. **Say in `mockspace.toml` that the panel root is frozen by convention and was breached four times**, or drop
   the reason clause. Section 8. And renumber one of `106`'s two section 16s.

**And one thing not to repair.** Do not put a predicate on
`a_composed_expressions_region_is_never_inherited_from_its_parts`. Section 9. The region is on the law row it
points at, and duplicating it would store one region twice with nothing keeping the two in step.
