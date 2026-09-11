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
