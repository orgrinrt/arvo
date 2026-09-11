# 266. What stands between each of the ten composition rows and canon

Blind derivation. Nothing under `202609030724_*`, nothing at `226_*` or `259_*`, and no
`git log -p` on `mock/research/` was read before this commit. The claims below rest on the
registry rows, their own cited provenance, and the probes committed at `266_probes/`.

## 0. The gate

`obligation::composition_contracts_above_the_numeral` is arvo's canon (`mockspace.toml`
`canon_paths` names `mock/registry/*.toml`), the obligation exists in that registry, and the
question asked, whether each of ten proposals answering it is promotable, is a question the
canon gate is built to answer rather than one that presupposes something forbidden. Proceeding.

## 1. What the obligation asks, and whether it is one demand

`need = "Contracts for units bigger than a single numeral, so that what a chain of operations
guarantees is expressible at all."` (obligation::composition_contracts_above_the_numeral's own `need` field). One sentence, and it
is not one demand: the ten rows answering it split cleanly across five things a reader would
have to be told separately before any single contract could be stated, and no one of the ten
discharges the obligation alone.

- **What counts as a composition at all**, before a contract can be written over it: rows 6
  (`configuration_is_not_composition_and_a_composite_is_a_primitive`) and 1
  (`a_chain_is_exact_operations_together_with_a_schedule_of_adaptation_points`), and half of row 9
  (the "three things" list, which `64`'s own reading files as chain content rather than format
  content, section 3 below).
- **A meta-law that composition itself changes the correctness argument**, so no per-part
  contract may be inherited: row 2 (`a_composed_expressions_region_is_never_inherited_from_its_parts`).
- **Per-construction contracts**, what a specific composed operation needs to be sound: rows 3
  (fold), 4 (min-plus fold), 5 (multiplicative chain), 8 (split reduction).
- **A binding-time/handoff contract**, what a composition is handed from the numeral below it and
  what it may hold itself: row 7 (the grid) and the other half of row 9 (what the format hands a
  composition).
- **A freedom bound** on how much latitude a realisation has inside a chain: row 10.

Five sub-demands, five topics (`the_chain`, `algebraic_laws`, `execution_environment`,
`the_primitive`, `the_format`). The obligation is satisfied jointly or not at all; nothing here
argues for closing it with a single ruling.

## 2. Per-row verdicts

### 2.1 `a_chain_is_exact_operations_together_with_a_schedule_of_adaptation_points`

**Not promotable.** `standing = "one_expert"` (proposal::a_chain_is_exact_operations_together_with_a_schedule_of_adaptation_points's own `standing` field), Stam alone (`60`), and the
row's own `note` says it "assumes one of three live directions for where a chain lives" (D-A,
D-B, D-C in `63`'s naming) and that adopting it "must not be read as closing that question."
`63_spj_consolidation_the_format_concept.md:697-703` confirms: C9 (this row, restated as a format
consolidation clause) is "ONE EXPERT cold plus reconciliation, unattacked," explicitly under D-B,
with D-A and D-C "remain live." I checked whether the direction fork was ever closed later: `173`
(the chain's own canon candidate, well after `60`/`63`) still lists "which chain carrier ships"
as O-1, discriminator "the obligations table... then op," i.e. still open
(`173_leroy_the_canon_candidate_for_the_chain.md:696-698`). None of the later `_op_` files
(`181`, `206`, `211`, `212`, `213`, `217`, `227`) mentions "carrier composition" or the D-A/B/C
directions (`grep`, empty). So the fork is open through the end of the panel's own record.
Blocked on: a second independent instance, and a closed direction fork this row presupposes.

`holds for: nothing measured; a normative sentence resting on an unresolved fork.`

### 2.2 `a_composed_expressions_region_is_never_inherited_from_its_parts`

**Promotable as written**, sentence unchanged, with the law it cites as its evidence.
`standing = "two_experts"` and the corpus backs it past that bar: `90_giesen_consolidation`
states the finding; `79_fallin_attacking_the_two_cold_derivations_on_derived_laws.md` reproduces
it independently ("section 1: reproduced independently. holds=2894336 fails=13882880 (82.7484%)
total=16777216", `79_probes/p1_output.txt:1`) and adds the four-way case split (P0-P4) with a
mutant control that reintroduces exactly 32,640 violations
(`79_probes/p1_negative_output.txt:1`, "p4_mutant (a<=1 instead of a==0): sufficiency_violations=32640"),
matching law::associativity_of_a_composed_saturating_add_and_subtract's own witness text verbatim
("a mutant control reintroducing 32,640 violations"). `215_kiselyov` recomputes
the figure a third time. That law row carries `standing = "sound"` with a stated `fails` predicate. Nothing
blocks this; the coordinator has evidence at three arrivals for a two-arrival bar.

`holds for: total_width W = 8, fraction_width F = 0, signedness = unsigned, overflow_policy =
saturate, operation = add-sub pair composed, threads any, target_features any` (the counterexample
law's own region, which is what the normative sentence rests its "needs its own derivation" claim
on).

### 2.3 `a_fold_needs_a_closed_operation_and_a_separately_determined_accumulator`

**Not promotable.** `standing = "one_expert"`, McSherry (`35`), sound probe evidence I opened
directly: four widening formulations refused with one diagnosis
(`35_probes/p1.out:1`, `E0308` on `Num<S<<W as Max<W>>::Out>>`), four positive arms compiling
clean in the same run. `216_lamport_second_read...md:963-976` reads and agrees, but says so in
words that disqualify it as the second independent instance this cluster needs: "I could not
arrive at that separately. I had the type-level shape and not the loop-carried argument, and the
loop-carried argument is the whole of why it is true." This panel's own convergence discipline
distinguishes a blind arrival from an attack that agrees after reading (`161_leroy`'s CONVERGED
vs ONE EXPERT rungs, e.g. L28 "blind" vs L25 "argument, ONE EXPERT"), and an admitted inability to
re-derive is squarely the second kind. Blocked on: a blind second derivation. The row's own `gap`
(capacity is a second input, no consumer-facing derivation given) stands too.

`holds for: toolchain rustc 1.98.0-nightly (57d06900f 2026-05-27) edition 2021, build_profile no
feature gates opt level 3, threads any (argued, not swept, per the row's own note).`

### 2.4 `a_min_plus_fold_needs_an_absorbing_top_and_wrapping_supplies_none`

**Not promotable as one row; the absorption half is promotable once split out, and the row's own
`gap` already says so.** I confirm the split independently rather than merely citing it.
`215_kiselyov_second_read_the_algebraic_laws.md:525-547` reproduces the absorption sweep over the
identical rectangle (`W in 2..=10`, `F in 0..=W`, unsigned, 63 cells) and goes further: it proves
the wrapping half as a theorem (`T + x = T` forces `x = 0` by group cancellation, so no element
absorbs but the identity, "at every width and both signednesses") and checks every semiring axiom
for saturation, not just the one it names ("every semiring axiom holds: product associativity,
product commutativity, product identity 0, sum identity TOP, TOP annihilating, and
distributivity"), which is strictly stronger than the row's own claim. That is two independent
arrivals (`35` and `215`) on the absorption half, one of them a proof. The end-to-end DAG half
(`p5`, wrapping wrong on 5.4M-of-11.9M and 407M-of-832M in-range instances) has one arrival and
nobody has re-run it, which the row's own `gap` states plainly. Splitting is not new work; it is
what the row already asks for.

`holds for` (absorption half): `total_width W in 2..=10, fraction_width F in 0..=W, signedness =
unsigned, overflow_policy in {wrap, saturate}, operation = add, arity = 2, threads = 1`.
`holds for` (end-to-end half, stays one_expert): `total_width W in {3, 4}, fraction_width F = 0,
signedness = unsigned, overflow_policy in {wrap, saturate}, threads = 1`.

### 2.5 `a_multiplicative_chain_is_writable_without_an_ever_growing_intermediate_by_windowing`

**Not promotable, for a reason that is not about standing.** The row's own `note` says the
width-arithmetic half (`ceil(log2 k)` vs `kF` bits) "is exact for any k and any F, established by
elementary counting rather than by the probe," i.e. it is a proof, and the ratified ruling
ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked created a marker for exactly this shape and explicitly "leaves the
marker's spelling to the panel," unsettled. There is no legal predicate form to write that half
into yet, independent of whether the reasoning is sound. The measured half (windowed dot product
vs staged narrowing) is `standing = "one_expert"` with sound, committed probe evidence
(`60_probes/p_a.out:1`: 46656/46656 correctly rounded for the window, 42892 and 15628 drifting
for the two staged schedules, mutant control flags 22476/46656). `167_rompf_the_chain_derived_cold.md:1238-1250`
built a rival mechanism (carried-limb residual) and, on its own instrument (probe G), conceded the
window dominates it, "no region where the carried form beats a window of the same width." That is
real corroboration of the cost comparison but not a blind re-derivation of the theorem, so it does
not clearly cross this panel's own convergence bar. Blocked on: (a) the proof-marker spelling,
which is panel work rather than evidence, (b) a blind second instance for the measured half.

`holds for` (measured half only): `operation = mul, arity = 3, fraction_width F = 8`.

### 2.6 `configuration_is_not_composition_and_a_composite_is_a_primitive`

**Promotable as written.** `standing = "two_experts"`, confirmed at
`161_leroy_the_canon_candidate_for_the_primitive.md:328-347`: L28 (the distinction) and L29 (the
closure, that a construction carries a predicate-on-base and a base-refinement transformer, with
equality transporting free where a predicate never does) are both "Rung: TWO+ INSTANCES," from
`110` F10 (blind) and `154` section 7 (a second blind arrival, "from a different instrument set,"
classified by `157` as genuine convergence with the shared bench corpus named). The row's own
`note` already correctly excludes any particular categorical frame beyond the distinction from its
own standing, which the source confirms L29 does too ("the fibration frame `154` offered beyond it
is `154`'s own proposal and stays at one expert"). Nothing found narrows this.

`holds for`: normative, no measured region (definitional, consistent with this panel's own
practice for concept-defining sentences, stated in the header comment above the number-system proposal block in that registry file).

### 2.7 `no_derivation_reads_the_grid_so_a_composition_may_hold_it_at_run_time`

**Not promotable, and the row's stated `gap` is not the whole gap.** `standing = "one_expert"`,
Rompf (`43`), sound probe evidence with three negative controls I opened
(`43_probes/RUN.md:1`, `--cfg negcontrol` fails at `E0080`). A second reader,
`216_lamport_second_read...md:1136-1153`, withdrew an initial objection as a misreading but
explicitly did not reproduce the type-equality result ("holds for: nothing. I did not reproduce
the type-equality result and have no instrument here"), so this is not the second instance. That
same reader located a real incompleteness the registry row's own `gap` does not name: the row's
source (`43`) also measures, in the same file, that the **operations** do not agree with the
**derivations** it establishes, specifically that multiplication reads the canonical exponent, and
that half is "measured in the same file and not filed" (the registry row's `note` says this too,
but its `gap` field talks only about "what holds it," not about the derivation/operation split).
`216`'s conclusion: "a composition holding its grid at run time has derivations that are unaffected
and at least one operation that is not," and a reader taking the row alone "will conclude more
than it says." Blocked on: a second independent instance, and filing the paired
operations-vs-derivations fact so the two are read together.

`holds for: toolchain rustc 1.98.0-nightly (57d06900f 2026-05-27) edition 2021, build_profile no
feature gates anywhere main arm exit 0, threads any (equalities decided at compile time).`

### 2.8 `splitting_a_reduction_is_sound_in_three_of_the_four_sign_and_policy_cells`

**Not promotable.** `standing = "one_expert"`, McSherry (`35`), sound probe evidence
(`35_probes/p3.out`, three cells at zero disagreement, signed-saturating at 28,336/65,536 and
11,760,675/16,777,216). Search for a second instance (`266_probes/second_instance_searches.out`)
found only `197_mcsherry_filing_the_algorithm_surface.md` and
`235_kiselyov_which_obligations_the_ratified_canon_supports.md`, both filing/discussion contexts
citing the finding rather than re-deriving it; neither states a `holds for:` of its own for this
claim. Blocked on: a second blind instance. Gap already stated: addition only, `F = 0`, one
thread; multiplication, nonzero fraction, genuinely multi-lane execution untested.

`holds for: total_width W in {3, 4}, fraction_width F = 0, signedness in {unsigned, signed},
overflow_policy in {wrap, saturate}, operation = add, arity = 2, chain_length in {4, 8}, threads = 1
(splits computed rather than executed on lanes).`

### 2.9 `the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws`

**Not promotable as one row; a real split exists, sharper than the row's own note admits.**
`237_the_format_proposals_against_the_ratification_gate.md:226-241` names this row explicitly as
"two propositions by two authors welded into one row, and promoting it would promote both in one
act": the "three things" list is Stam's, from `60`, and the statability argument
("compositions... owe their own laws") is SPJ's, from `63`. `240_the_format_layer_derived_from_its_denotation.md:1115-1123`
then independently seconds **only** the statability half, by a different route ("One line, no
probe, and independent of `60`, whose route is the schedule"): a concept whose operations have
signature `V x V -> V` has no name for the intermediate a composite claim quantifies over, so the
chain clause has no expressible form against it. `240`'s own words: "my arrival is evidence for
splitting the row rather than for ratifying it whole." So: the statability clause now has two
independent arrivals (`63`, `240`) and is close to ready once split out, though `64` and `240` both
read it as chain content rather than format content, which bears on which `topic` a ruling would
carry it under. The "three things" list stays `one_expert` (Stam alone), and is itself suspected
of misfiling: `64_ringer_entailment_check_on_the_format_consolidation.md` reads it as a chain
sentence per `237`'s account (`237:527-530`), which the registry `topic = "the_format"` field does
not reflect. Blocked on: undoing the weld, and (for the list half) a second instance plus settling
which topic it belongs to.

`holds for`: normative, no measured region for either half (definitional/statability content).

### 2.10 `within_an_unbound_stretch_the_design_may_select_any_realisation`

**Not promotable, and by a wide margin.** `standing = "one_expert"` in the registry field, but the
row's own `note` is more honest than the field: "the honest number is zero... A reader counting
instances from this field will count one too many." Three members tried to derive the licence
from arvo's own stated intents and all three failed for named reasons; what actually grounds it is
a workspace rule, not an arvo intent, and "a workspace rule is not a ratification" (the row's own
words). The row's `gap` names two further op-only questions: whether the observability principle
ever becomes an arvo intent at all, and whether a canon may carry a sentence of this shape (a
normative licence resting on an external premise) beside established ones. I found nothing in
`183`, `187`, or `189` (the files mentioning this slug besides its own provenance) that changes
this; all three cite it as a filed answer to a discriminator question rather than re-deriving it.
Blocked on: op, on two named questions the row itself states, neither answerable from evidence.

`holds for`: normative, no measured region; the build-bound clause (`debug-assertions = off`) is
stated in prose rather than as a predicate, per the row's own `gap`.

## 3. Summary table

| row | verdict | region |
|---|---|---|
| `a_chain_is_exact_operations...` | not promotable: 1 expert, presupposes an open fork | none |
| `a_composed_expressions_region...` | **promotable as written** | W=8,F=0,unsigned,saturate,add-sub,threads any |
| `a_fold_needs_a_closed_operation...` | not promotable: 1 expert (agreement, not re-derivation) | toolchain-pinned, threads any (argued) |
| `a_min_plus_fold_needs_an_absorbing_top...` | split: absorption half **promotable**, end-to-end half stays 1 expert | W in 2..=10, F in 0..=W (absorption) |
| `a_multiplicative_chain_is_writable...` | not promotable: proof marker unspecified + measured half at 1 expert | mul, arity=3, F=8 (measured half) |
| `configuration_is_not_composition...` | **promotable as written** | normative, no region |
| `no_derivation_reads_the_grid...` | not promotable: 1 expert, unfiled paired fact | toolchain-pinned, threads any |
| `splitting_a_reduction_is_sound...` | not promotable: 1 expert | W in {3,4}, F=0, add, threads=1 |
| `the_format_concept_carries...` | split: statability half close once split, list half stays 1 expert and may be misfiled by topic | normative, no region |
| `within_an_unbound_stretch...` | not promotable: honest zero instances, op-only questions open | normative, no region |

Two rows promotable as written. Two more promotable after an explicit split the corpus already
argues for. Six rows (counting the non-promotable remainder of the two split rows) blocked on a
named, specific thing: a second blind instance in four cases, an unresolved fork in one, an
unsettled notation marker in one, and two questions reserved for op in the last.

## 4. Reconciliation against the prior seat

Read after the commit above. The prior seat's files:
`mock/research/202609030724_the_composition_cluster_is_four_obligations_and_two_rows_are_gateable.md`
(main file, referred to below as `main`) and
`mock/research/202609030724_section_twelve_reconciliation_against_195.md` (`s12`). I also opened
`226_lattner_the_derivation_outputs.md` and `259_fallin_whether_the_fused_result_composes.md`,
required by the brief's blindness list; a slug grep against all ten of mine and against the
obligation itself returns nothing in either file, so neither bears on this cluster and there is
nothing to reconcile there.

One process observation before substance. Both `main` (`main:5-9`) and `s12` (`s12:5-8`) state
twice that the reconciliation section is a separate commit from the main file, specifically so the
ordering is checkable from git rather than asserted in prose. The committed history does not bear
that out: `git log --oneline -- <both files>` returns one commit, `2a2995d4`, for both. Whatever
happened between writing and committing, the claim as it stands in the text is not verifiable from
the artifact it points at.

### Agreement

The obligation bundles several demands rather than one. I reached five groups
(ontology/definitional, a general meta-law, per-construction contracts, binding-time/handoff, a
freedom bound), `main` reaches four (concept, region, schedule, binding-time), folding my
per-construction and freedom-bound groups differently. Same underlying finding, independently
reached, different cut (`main:130-170`, section 1 above).

`a_composed_expressions_region_is_never_inherited_from_its_parts` is the strongest row and is
promotable as written. Both of us reran the underlying instrument and got 82.7484 per cent and the
32,640-violation mutant figure exactly (`main:184-221`, section 2.2 above).

`configuration_is_not_composition_and_a_composite_is_a_primitive` is real at two-or-more instances
via `161`'s L28/L29, both of us reading the same two blind arrivals, `110` and `154` (`main:223-284`,
section 2.6 above).

`a_min_plus_fold_needs_an_absorbing_top_and_wrapping_supplies_none` should be split into an
absorption half (near-promotable) and an end-to-end half (stays low), which is what the row's own
`gap` already asks for and what both of us independently confirm by rerunning the absorption sweep
(`main:292-299`, section 2.4 above).

`a_fold_needs_a_closed_operation_and_a_separately_determined_accumulator`,
`no_derivation_reads_the_grid_so_a_composition_may_hold_it_at_run_time` and
`a_chain_is_exact_operations_together_with_a_schedule_of_adaptation_points` are one_expert, genuinely
blocked on a second blind derivation, no shortcut available (`main:329-356`, sections 2.1, 2.3,
2.7 above).

### Where I found something `main` did not

`the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws`. `main:347-356`
treats this row as flatly one_expert, citing only the row's own `note`, and recommends "a second
seat" with no further path. I opened `237_the_format_proposals_against_the_ratification_gate.md`
and `240_the_format_layer_derived_from_its_denotation.md`, which `main` never opens (`grep -n
"237\|240" main` returns nothing): the row welds two authors' clauses, and seat 240 independently
seconds, by a stated-different route, the statability half alone (`240:1115-1123`). That is a real
second arrival on half the row, findable only by going one level past the row's own note, and it
changes the row from "wants a second seat" to "half-way there once split," which section 2.9 above
states.

### Where `main` found something I did not

`a_multiplicative_chain_is_writable_without_an_ever_growing_intermediate_by_windowing`. I marked
this blocked on the proof-marker spelling that `ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked`
left to the panel, current as of the sources I read. `main:333-345` found that the panel has since
settled it: `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`, ratified by experts,
gives three tokens including `construction`, priced at an `evidence` edge naming an instrument
that varied the axis and found no movement. I opened that ruling directly and it says what `main`
claims. My verdict in section 2.5 above is stale against the current registry; the corrected
verdict is `main`'s: the marker blocker is gone, and what is left is a named, priced choice
(write the differential probe or take the weaker warrant) plus a second instance on the measured
half.

`within_an_unbound_stretch_the_design_may_select_any_realisation`. I took the row's own `gap` at
face value, two questions reserved for op. `main:44-53` checked whether those two named questions
were since answered and found both are:
`ruling::the_observability_licence_is_an_intent_and_he_put_it_to_the_panel` (`rung = "stated"`,
his own words "It becomes your intent," answering the first) and
`ruling::the_panel_finishes_the_canon_without_him` (`rung = "ratified"`, "every remaining canon
question is the panel's," answering the second). I opened both and confirm they say this. The row
is still not promotable, honest zero instances per its own note, but the two blockers I repeated
from its `gap` field are stale, and the field should be corrected rather than left for the next
reader to rediscover, which is `main`'s own conclusion too.

`splitting_a_reduction_is_sound_in_three_of_the_four_sign_and_policy_cells`. I read this row in
isolation. `main:309-319` finds a cross-row conflict I missed entirely: this row and
`within_an_unbound_stretch_the_design_may_select_any_realisation` both bear on
`question::reduction_order_or_associativity`, and they answer it in two incompatible shapes. The
splitting row already gives arms over regions, three cells sound and one not, which is the shape
`ruling::arms_over_regions_are_the_fundamental_heart` calls the fundamental heart, the same
anti-pattern this workspace's own `never-ask-which-single-rule-governs.md` names: a single policy
over a whole category is the wrong shape, and op has refused that shape three times per that
ruling's own history. `within_an_unbound_stretch` answers the same question with a licence, which
is exactly a single rule over the category. Neither row says so, and `main` is right that this is
worth flagging in both rows rather than in neither.

The min-plus end-to-end half, quantitatively. This is the largest single addition in `main` and I
did not come close to it: rather than rerunning `35_probes/p5_algorithm_end_to_end.rs`, `main`
rebuilt the DAG instrument from the probe's own specification, reproduced the headline, and then
found the row's own `because` clause does not hold up: the max-plus "control" everyone reads as
clean is an artifact of how that source file encodes unreachability (`Option<u128>` skip against a
top-standing sentinel), not a genuine cross-routine control. Rebuilt honestly, max-plus fails under
both policies too, so the control the earlier probe's own `standing = "sound"` leans on is
uncontrolled by the schema's own vocabulary, not defective, uncontrolled (`main:379-398`). The
further decomposition, that absorption accounts for only 8.9 per cent and 4.0 per cent of the
wrapping failures at the two widths and the rest is a mechanism the row never names, overflow
mapping in the direction the reduction discards, is new content nobody else in the cluster has,
including me (`main:400-422`). If this row is ever promoted, it should be promoted as `main`
restates it, not as it is currently worded.

### What this changes about section 2 above

Section 2.5's verdict is corrected: the proof-marker blocker is resolved, and what remains is a
priced choice plus a second measured instance, not an unsettled notation gap.

Section 2.10 stays "not promotable" but its stated blockers are now marked stale rather than
current, matching `main`.

Section 2.8 gains the cross-row note about `question::reduction_order_or_associativity` that
neither of us had in isolation.

Section 2.4's verdict is right as far as it goes and understates the end-to-end half's distance
from the row as written: it is not merely "one expert, unreproduced." It is one expert whose own
stated mechanism does not survive a from-spec rebuild, which is a stronger finding than "needs a
second instance."

Section 2.9 is where I have something `main` lacks rather than the reverse: the statability half
of the format-concept row is closer to promotable than `main`'s file reports.
