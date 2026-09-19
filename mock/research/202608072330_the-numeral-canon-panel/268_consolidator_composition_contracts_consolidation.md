# 268. Consolidation: `obligation::composition_contracts_above_the_numeral`

Consolidator, not a panel voice. My job is to state what the two reads established, what the
registry now shows, and what is still open, not to argue a third reading. Where I checked a
claim myself I say so and name what I ran; where I only cross-read a citation, I say that too.

## 0. The gate, and the fact that changes everything else in this file

`obligation::composition_contracts_above_the_numeral` is arvo's canon (`mockspace.toml`'s
`canon_paths` names `mock/registry/*.toml`), the obligation exists there, and neither input read
contradicts a ratified row. Proceeding.

The one fact that governs the rest of this consolidation: **the promotion has already happened.**
Commit `6ae6cade` ("canon: promote four composition rows on two reads each"), dated 2026-09-18,
already on `dev` and an ancestor of this branch, ratified exactly the two rows both reads call
promotable as written and split out exactly the two half-rows both reads call promotable once
split, in the same words the reads use, with the same mechanical repairs (a `probe` row and an
`evidence` edge for the associativity law, a second `reproduced` field on the absorption probe,
the equality-clause disambiguation, the statability-half split). Arvo `#117` was committed
2026-09-03, arvo `#142` (seat 266) 2026-09-11, and the promoting commit 2026-09-18: both reads
predate it and neither is the promoting commit's own record of itself, so I checked authorship
and content rather than assuming the timeline implies causation. `git show 6ae6cade` is small (98
lines across `law.toml`, `obligation.toml`, `probe.toml`, `ruling.toml`) and its commit message
names the same four rows, the same split rationale, and the same six-rows-stay-unpromoted count
both reads reach. I did not find a fifth research file or a third read that would explain the gap
between "two reads recommend it" and "a ruling lands it"; the mechanism connecting them is outside
what either input file or the registry states, and I am not asserting one. What I can state is
that the four ratified rulings' text matches what the two reads independently establish, checked
clause by clause below, and that `ratified_by = "experts"` is exactly the route
`mockspace.toml`'s own schema names for a proposal that converged and was promoted without
needing op's individual stamp on each one, a route he set up himself (`ruling::the_panel_finishes_the_canon_without_him`,
`ratified_by = "op"`, ratified 2026-08-xx, quoted below). So this is not agent output presumed
wrong; it is a ratified row citing the mechanism that licenses it, and I read it as settled rather
than as a claim I owe a second opinion on.

This changes what "promotable" means for the rest of this file. Four of the ten rows are past the
gate already. My ledger states that as the current fact rather than as a recommendation, and the
verdict column for the other six is what remains live.

## 1. The ledger: every claim, which read made it, and independence

| claim | `#117` | `#142` (266) | independent? |
|---|---|---|---|
| The obligation is one slug over four (or five) unrelated demands | yes, four groups (concept, region, schedule, binding-time) | yes, five groups (ontology, meta-law, per-construction, binding-time, freedom bound) | **yes.** Different cuts of the same ten rows, reached before either read the other (`#142` states it opened neither `202609030724_*` nor `226_*`/`259_*` before its own commit; `#117` states its own ordering and that it left `195` closed until after its own commit). Registered on the obligation's own `gap` field, both readings cited by name. |
| `a_composed_expressions_region_is_never_inherited_from_its_parts`: promotable as written | yes | yes | **yes**, and both reran the underlying instrument on their own machines and got 82.7484% and the 32,640-violation mutant figure to the digit. Now ratified, `ruling::a_composed_expressions_region_is_never_inherited_from_its_parts`. |
| `configuration_is_not_composition_and_a_composite_is_a_primitive`: promotable as written, distinction and closure only | yes | yes | **yes**, both reading `161`'s L28/L29 as two blind arrivals (`110`, `154`). Now ratified, `ruling::configuration_is_not_composition_and_a_predicate_never_transports_for_free`, with the equality clause split into a congruence reading the evidence supports and an adequacy reading it refutes. |
| `a_min_plus_fold_needs_an_absorbing_top_and_wrapping_supplies_none`: not one row, split into an absorption half (near two instances) and an end-to-end half (stays one) | yes, and `#117` reran the absorption sweep itself, 63-and-63 | yes, and `#142` cites `215`'s independent rerun plus its theorem proof of the wrapping half | **yes** on the split call. The absorption half's second instance is `215` in both reads' account, so it is one arrival cited twice rather than two independent findings of a second arrival; both reads say this plainly. Now ratified, absorption half only, `ruling::a_min_plus_top_absorbs_addition_under_saturation_and_never_under_wrapping`. |
| `the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws`: welded row, statability half has a second arrival (`240`), list half stays one expert | **no**, `#117` treats the whole row as one_expert and recommends "a second seat" | **yes**, `#142` opened `237` and `240`, which `#117` never opens (checked: `grep -n "237\|240"` against `#117`'s file returns nothing) | **`#142` alone.** `#117`'s section twelve (the reconciliation file) accepts this as something `#142`'s counterpart found that its own file did not, before either file could have read the other's final form, since `#117`'s section twelve was written and committed before `#142` existed. Recorded in `#117`'s own text as agreement reached across the sitting, not as something either read owes retroactively to the other. Now ratified, statability half only, `ruling::a_format_concept_closed_over_v_cross_v_to_v_cannot_state_a_composite_claim`. |
| `question::reduction_order_or_associativity` is answered in two incompatible shapes by two rows in this cluster (`splitting_a_reduction...` gives arms over regions, `within_an_unbound_stretch...` gives a single licence over the category) | **yes**, `#117` finds it | **no** in isolation; `#142` states it read `splitting_a_reduction...` alone and missed the cross-row conflict, then credits `#117` for it in its own reconciliation section | **not independent.** One read found it, the other did not reach it separately and says so in its own words. Carried forward as `#117`'s finding, corroborated by `#142` reading and agreeing after the fact, which is support rather than a second arrival. **Still open, in exactly the state both reads describe**: the promoting commit's own message says this conflict "is left for the row that touches it next," and neither proposal row's text has been edited to name it. |
| The min-plus end-to-end half's own stated control (max-plus as a clean comparison column) is not a control at all, being an artifact of how the source file encodes unreachable predecessors, and the true mechanism is overflow mapping in the direction the reduction discards, which absorption accounts for at only 8.9% (W=3) and 4.0% (W=4) | **yes**, `#117` rebuilt the DAG instrument from the probe's own specification rather than rerunning the committed one | **no**, `#142` read this row in isolation and states plainly it "did not come close to it" | **`#117` alone**, the strongest single result either file produced, and a from-scratch reproduction rather than a rerun. This is now folded into the ratified absorption ruling's own `note`, with the same 8.9%/4.0% figures. The end-to-end half itself stays unpromoted; the correction is recorded so nobody promotes it on the row's old, now-refuted, stated mechanism. |
| The proof-marker blocker on `a_multiplicative_chain_is_writable_without_an_ever_growing_intermediate_by_windowing` is resolved (`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` settled the marker's spelling) | **yes**, `#117` found this | **no**, `#142`'s own section 2.5 is stale on this point and its reconciliation section explicitly corrects itself against `#117` | **`#117` alone**, and `#142` says so about itself rather than my saying it about `#142`. |
| Both blockers named in `within_an_unbound_stretch_the_design_may_select_any_realisation`'s `gap` are stale (`the_observability_licence_is_an_intent_and_he_put_it_to_the_panel` and `the_panel_finishes_the_canon_without_him` both since landed) | **yes** | **yes, independently**, `#142` checked the same two rulings itself before reading `#117`'s claim about them | **yes.** Both reads checked the cited rulings directly. Neither read edited the row's `gap` field, so it still names the two questions as open; that repair is unmade and is not mine to make. |

## 2. Per-row verdict, evidence, and predicate

Ten proposal rows named by either read. Four are now ratified; I state the ratified text's
predicate rather than re-deriving it, and name where the ratifying ruling narrowed the proposal's
own sentence.

### 2.1 `a_composed_expressions_region_is_never_inherited_from_its_parts` — **ratified**

`ruling::a_composed_expressions_region_is_never_inherited_from_its_parts`, `rung = "ratified"`,
`ratified_by = "experts"`. Sentence carried unchanged from the proposal. Evidence: `law::associativity_of_a_composed_saturating_add_and_subtract`,
now carrying `evidence = ["a_composed_saturating_add_then_subtract_fails_associativity_on_82_percent_of_triples"]`,
a committed `probe` row citing `79_probes` and `215_probes` sources, drivers and stdout, with a
mutant control (32,640 violations reintroduced) and a negative control (the same composition
under wrapping reports zero).

Obligation this carries for code: **a composition's region may not be derived by intersecting or
otherwise combining its parts' regions, even where every part's own hypotheses hold
unconditionally on the whole domain.** A type or trait implementation composing two numeral
operations must derive the composite's holding region as its own claim, backed by its own
evidence, and may not default to "both parts hold everywhere, so the composite does". A lint or a
design that infers a composite's soundness from its parts' soundness is exactly the shape this
rules out.

`holds for: total_width: W = 8, fraction_width: F = 0, signedness: unsigned, overflow_policy:
saturate (vs. wrap as the negative control), operation: add-sub pair composed, threads: any,
target_features: any` — the counterexample law's own region, which is what the normative "needs
its own derivation" claim rests on. The claim itself (the "never" clause) is normative and holds
over derivation rules rather than over values, so it is not region-bound the way a measured claim
is; the region above is the counterexample's, not a scope limit on the rule.

### 2.2 `configuration_is_not_composition_and_a_composite_is_a_primitive` — **ratified, narrowed**

`ruling::configuration_is_not_composition_and_a_predicate_never_transports_for_free`, `rung =
"ratified"`, `ratified_by = "experts"`. The distinction and the closure carried as written
(two-plus instances, `110` and `154`, both blind, `157` classifying it a genuine convergence).
The equality clause is narrowed: the ruling promotes the **congruence** reading (two values
related by the base's lifted equality denote the same thing) and explicitly does not promote the
**adequacy** reading (the lifted equality is the construction's own equality), which both reads'
probes refute on a stored-pair rational at base width four (688 of 57,600 adequacy violations,
first witness `(0/1)` against `(0/2)`, congruence holding at zero violations of the same 57,600).

Obligation this carries for code: **a composite type may derive `PartialEq` from its base's
lifted equality only as a congruence, never as a claim that the derived equality is the
composite's canonical equality.** A construction that normalises (a rational reducing to lowest
terms is the named example) has a real distinct notion of "same value" that the lifted relation
does not capture, and a design deriving `PartialEq` mechanically from the base has taken the
adequacy reading this ruling refutes. Any future composite type's equality implementation is
obliged to state which reading it claims, and a bare `#[derive(PartialEq)]` composed
componentwise is not licensed by this row to mean "the construction's own equality" without a
separate argument.

`holds for`: normative for the distinction and the closure, no measured region (definitional).
The equality-congruence clause holds at `total_width: W = 4` on the rational and pair
constructions both reads' probes checked directly, and at whatever coordinates `110`'s own four
constructions (product2, complex, dual, interval) ran at, which `110` itself does not state as a
predicate.

### 2.3 `a_min_plus_fold_needs_an_absorbing_top_and_wrapping_supplies_none` — **split; absorption half ratified, end-to-end half not promotable**

`ruling::a_min_plus_top_absorbs_addition_under_saturation_and_never_under_wrapping`, `rung =
"ratified"`, `ratified_by = "experts"`, absorption half only: unsigned, over every declared
width-and-fraction cell swept (63 cells), the saturating top absorbs at all 63 and no wrapping
element but the additive identity absorbs anything, proved as a theorem from group cancellation
by seat 215 in addition to the swept instance.

Obligation this carries for code: **a min-plus (shortest-path-style) reduction over a bounded
numeral may rely on the saturating top as an absorbing element for its sentinel, and may not rely
on any wrapping policy to do the same job**, unsigned, at any width and fraction width the
container supports. This is absorption only; see below for what it does not license.

The end-to-end half stays unpromoted, and its own stated mechanism is now refuted rather than
merely unreproduced: `#117`'s from-specification rebuild found the row's own control (a max-plus
"clean" comparison column) is an artifact of how the source encodes unreachable predecessors
(`Option<u128>` skip against a top-standing sentinel in one routine, add-to-the-top in the other),
not a genuine cross-routine control, and that the true mechanism is overflow mapping in the
direction the reduction discards, of which absorption is the special case for one value.
Absorption accounts for only 8.9% (W=3) and 4.0% (W=4) of the wrapping failures the end-to-end
probe measures; the remaining 91-96% is unfiled. **This finding is already folded into the
ratified ruling's `note` field, verbatim to the percentages.** A superseding proposal exists,
`proposal::a_min_plus_computation_needs_monotonicity_as_well_as_an_absorbing_top` (`standing =
"one_expert"`, McSherry, `supersedes = ["a_min_plus_fold_needs_an_absorbing_top_and_wrapping_supplies_none"]`),
stating that absorption and monotonicity are separable and a numeral can hold either alone,
measured over three overflow policies: wrapping fails both (wrong on 48.08% of 621,961,320
in-range instances), saturation holds both (wrong on none), and wrapping-below-a-reserved-top
holds absorption but fails monotonicity (wrong on 12.57%). **This row does not carry an
`obligation` edge to `composition_contracts_above_the_numeral`**, checked directly (`grep` over
its block finds no `obligation` field), which is exactly what `#117` reports and flags as
"additive work for whoever next touches this cluster." I am not wiring that edge; it is a
registry edit and outside what I was asked to do, but a later pass on this cluster should treat
its absence as a known gap rather than as the monotonicity half being unfiled entirely.

Obligation the end-to-end half does not yet license: **"a numeral with an absorbing top is
sufficient for a min-plus reduction to be correct" is false and is not what the ratified ruling
says.** Absorption is necessary but covers a small minority of the failure mode; monotonicity is
the larger, separately-required property and is not yet a ratified obligation on any numeral.

`holds for` (absorption, ratified): `total_width: W in 2..=10, fraction_width: F in 0..=W,
signedness: unsigned, overflow_policy: {wrap, saturate}, operation: add, arity: 2, threads: 1`.
`holds for` (end-to-end, unpromoted, McSherry's superseding measurement): `total_width: W in {3,
4}, fraction_width: F = 0, signedness: unsigned, overflow_policy: {wrap, saturate, wrap-below-a-
reserved-top}, operation: add, arity: 2, threads: 1`.

### 2.4 `the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws` — **split; statability half ratified, list half not promotable**

`ruling::a_format_concept_closed_over_v_cross_v_to_v_cannot_state_a_composite_claim`, `rung =
"ratified"`, `ratified_by = "experts"`, statability half only: a format concept whose operations
have signature `V x V -> V` has no name for the intermediate a composite claim quantifies over,
so a composite claim (that `rho(f(g(x, y), z))` equals something) has no expressible form against
a concept closed that way. Two independent routes: SPJ (`63`) from the chain-accuracy intent,
Lamport (`240`) from the signature alone, dispatched deliberately without reading `60`, `63`,
`236`, `237` or the `proposal` namespace on this topic.

Obligation this carries for code: **a format type's trait surface, closed under `V x V -> V`
operations, cannot itself state or check a claim about a composite expression's error
(`rho(f(g(x,y),z))`-shaped).** Any such claim has to be stated at the composition layer, over a
type that names the intermediate, not retrofitted onto the format's own operation signatures.
This is a negative obligation: it forbids a design from trying to express composite-claim
checking inside a format trait whose operations are all binary-in-binary-out.

The list half (the format concept supplies width algebra, named adaptation, exactness predicate
upward) stays at one expert (Stam, `60`), unpromoted, and carries a live filing tension neither
ratifying ruling nor either read resolves: both `64` and `240` read the statability clause as
chain content rather than format content, which `topic = "the_format"` on the underlying proposal
does not reflect. The ratifying ruling keeps `topic = "the_format"` "for continuity" and records
the tension rather than resolving it, exactly as both reads recommend doing (neither recommends
forcing a retitle now).

`holds for`: normative, no measured region on either half (definitional/statability content, by
this panel's own stated practice for concept-defining sentences).

### 2.5 `a_chain_is_exact_operations_together_with_a_schedule_of_adaptation_points` — not promotable

Both reads agree: one expert (Stam, `60`), and the row's own `note` says it assumes one of three
live directions for where a chain lives (D-A/D-B/D-C) and that adoption must not be read as
closing that fork. `#142` checked whether the fork closed later (`173`, and every `_op_` file
through `227`) and found it still open through the end of the panel's own record, via `grep`
returning empty for "carrier composition" and the D-A/B/C vocabulary. Blocked on: a second
independent instance, and the carrier-direction fork closing first (or the row being written to
not presuppose one, which neither read proposes as a fix). Nothing in the registry has moved this
since either read; still `standing = "one_expert"`.

`holds for: nothing measured; normative, resting on an unresolved fork.`

### 2.6 `a_fold_needs_a_closed_operation_and_a_separately_determined_accumulator` — not promotable

Both reads agree: one expert (McSherry, `35`), sound probe evidence (four widening formulations
refused with one diagnosis, four positive arms compiling in the same run). `#142` checked the one
candidate second reading, `216_lamport_second_read`, and found it explicitly disqualifies itself
as a second instance: "I could not arrive at that separately. I had the type-level shape and not
the loop-carried argument, and the loop-carried argument is the whole of why it is true," an
admitted inability to re-derive rather than a blind arrival, which this panel's own convergence
discipline (`161`'s CONVERGED vs. ONE EXPERT rungs) treats as the second kind, not the first.
Blocked on: a blind second derivation, briefed without the slug (both reads agree on this
mechanism, citing `behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`'s own
record of contamination from naming a slug in a brief). The row's own `gap` (capacity is a second
input; no consumer-facing derivation of it is given) stands too.

`holds for: toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), edition 2021, build_profile:
no feature gates, opt level 3, threads: any (argued rather than swept, per the row's own note).`

### 2.7 `a_multiplicative_chain_is_writable_without_an_ever_growing_intermediate_by_windowing` — not promotable

`#117` found what `#142` missed and `#142`'s own reconciliation corrects itself to match: the
proof-marker blocker `#142`'s first pass names (`ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked`
leaving the marker's spelling to the panel) is stale, because `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`,
ratified by experts, since gave three tokens including `construction`, priced at an `evidence`
edge naming an instrument that varied the axis and found no movement. What remains, per both
reads once reconciled: a priced choice (write the differential probe the `construction` token
obliges, or take the weaker warrant and say so) for the additive width-arithmetic half, which is
a proof rather than a sweep and does not admit a predicate at all under I13 until one of those two
is chosen; and a second, blind instance for the measured half (windowed dot product vs. staged
narrowing, `standing = "one_expert"`, sound probe evidence, 46656/46656 correctly rounded for the
window against 42892 and 15628 drifting for the two staged schedules). `167_rompf`'s rival
mechanism conceding the window dominates it is real corroboration of the cost comparison but not
a blind re-derivation of the theorem, and both reads agree it does not cross the convergence bar.

`holds for` (measured half only): `operation: mul, arity: 3, fraction_width: F = 8`.

### 2.8 `no_derivation_reads_the_grid_so_a_composition_may_hold_it_at_run_time` — not promotable

Both reads agree: one expert (Rompf, `43`), sound probe evidence with three negative controls.
`#142` opened `216_lamport_second_read` and found it explicitly withdraws as a second instance
("holds for: nothing. I did not reproduce the type-equality result and have no instrument here")
while separately locating a real incompleteness the row's own `gap` does not name: the same
source (`43`) also measures that the operations do not all agree with the derivations
(multiplication reads the canonical exponent, addition and the bias do not), which is filed in
the row's `note` but not in its `gap`, so a reader taking the `gap` field alone concludes more
than the row states. Blocked on: a second independent instance, and filing the paired
operations-vs-derivations fact where the `gap` field, not only the `note`, carries it.

`holds for: toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), edition 2021, build_profile:
no feature gates anywhere, main arm exit 0, threads: any (equalities decided at compile time).`

### 2.9 `splitting_a_reduction_is_sound_in_three_of_the_four_sign_and_policy_cells` — not promotable, and carries the open Q12 conflict

Both reads agree on standing: one expert (McSherry, `35`), sound probe evidence (three
sign-and-policy cells at zero disagreement, signed-saturating at 28,336/65,536 at width 4/arity 4
and 11,760,675/16,777,216 at width 3/arity 8). `#142` searched for a second instance directly
(`266_probes/second_instance_searches.out`) and found only citing/discussion contexts
(`197_mcsherry_filing_the_algorithm_surface`, `235_kiselyov_which_obligations_the_ratified_canon_supports`),
neither re-deriving the finding nor stating its own `holds for:`. Blocked on: a second blind
instance, and the per-obligation argument the row's own `note` explicitly declines to make
(the reduction-split precondition plainly generalises past its one filed `precondition_for` edge
to any obligation reducing over numerals on more than one core, and nobody has made that argument
per obligation, so it is not filed).

**The open conflict, found by `#117` alone and not independently corroborated (see the ledger):**
this row and `within_an_unbound_stretch_the_design_may_select_any_realisation` both bear on
`question::reduction_order_or_associativity` and answer it in two incompatible shapes. This row
gives arms over regions (three cells sound, one not), which `ruling::arms_over_regions_are_the_fundamental_heart`
(ratified, both) names the organising shape the whole panel converged on. The other row licenses
"the design may select any realisation" within an unbound stretch, which is a single rule over
the category, exactly the shape `never-ask-which-single-rule-governs.md` and the ruling that
records op refusing it three times both reject. **Neither proposal row's text names this conflict
as of this consolidation.** The promoting commit's own message defers it explicitly ("left for
the row that touches it next"), and no row has touched it since. This consolidation is that
notice, for whoever does.

`holds for: total_width: W in {3, 4}, fraction_width: F = 0, signedness: {unsigned, signed},
overflow_policy: {wrap, saturate}, operation: add, arity: 2, chain_length: {4, 8}, threads: 1
(splits computed rather than executed on lanes).`

### 2.10 `within_an_unbound_stretch_the_design_may_select_any_realisation` — not promotable, by a wide margin, blockers stale but unrepaired

Both reads agree, and the row's own `note` is unusually blunt about it: standing is recorded as
`one_expert` but the honest number is zero, the panel's own ledger putting it at zero independent
instances, three members having tried to derive the licence from arvo's stated intents and failed
for stated reasons. Both reads independently checked the row's two named blockers
(`does_the_observability_principle_become_an_intent`'s answer, and whether a canon may carry a
sentence of this normative shape) and found both stale: `ruling::the_observability_licence_is_an_intent_and_he_put_it_to_the_panel`
(op, `rung = "stated"`, "It becomes your intent") answers the first, and
`ruling::the_panel_finishes_the_canon_without_him` (op, `rung = "ratified"`) answers the second by
moving every remaining canon question to the panel. **Neither read edited the row, and the row's
`gap` field still names both as open questions reserved for op.** That repair (striking the two
stale blockers from `gap`) is a registry edit both reads recommend and neither made; I have not
made it either, per my brief. Blocked on: a first instance. A second seat cannot help; what this
needs is a first one, from any member willing to try deriving it from the intents rather than
from the workspace rule it currently rests on.

Also carries the Q12 conflict named under 2.9: this row's licence is the single-rule-over-the-
category shape the splitting row's arms-over-regions shape already dissolves for at least the
reassociation question. Same open item, same disposition.

`holds for`: normative, no measured region; the build-bound clause (`debug-assertions = off`) is
stated in prose rather than as a predicate, per the row's own `gap`.

## 3. Summary table

| row | verdict | registry state | region / predicate |
|---|---|---|---|
| `a_composed_expressions_region_is_never_inherited_from_its_parts` | **ratified** | `ruling`, `ratified_by = experts` | W=8, F=0, unsigned, saturate, add-sub, threads any |
| `configuration_is_not_composition_and_a_composite_is_a_primitive` | **ratified, narrowed** | `ruling`, `ratified_by = experts` | normative; equality clause at W=4 (congruence only) |
| `a_min_plus_fold_needs_an_absorbing_top...` | **split: absorption ratified**, end-to-end not promotable, mechanism refuted | `ruling` (absorption) + `proposal` (superseding, unwired to this obligation) | absorption: W in 2..=10, F in 0..=W; end-to-end: W in {3,4}, F=0 |
| `the_format_concept_carries_three_things...` | **split: statability ratified**, list half one expert, topic tension open | `ruling` (statability) + `proposal` (list, unchanged) | normative, no region either half |
| `a_chain_is_exact_operations_together...` | not promotable: one expert, presupposes an open carrier fork | `proposal`, `standing = one_expert` | none |
| `a_fold_needs_a_closed_operation...` | not promotable: one expert, an agreement is not a re-derivation | `proposal`, `standing = one_expert` | toolchain-pinned, threads any (argued) |
| `a_multiplicative_chain_is_writable...` | not promotable: marker resolved, priced choice + second instance owed | `proposal`, `standing = one_expert` | mul, arity=3, F=8 (measured half) |
| `no_derivation_reads_the_grid...` | not promotable: one expert, paired fact unfiled in `gap` | `proposal`, `standing = one_expert` | toolchain-pinned, threads any |
| `splitting_a_reduction_is_sound...` | not promotable: one expert, carries open Q12 conflict | `proposal`, `standing = one_expert` | W in {3,4}, F=0, add, threads=1 |
| `within_an_unbound_stretch...` | not promotable: honest zero instances, stale-but-unstruck blockers, carries open Q12 conflict | `proposal`, `standing = one_expert` | normative, no region |

Four ratified, two of those narrowed at ratification from what the underlying proposal states.
Six unpromoted, all six blocked on something named and specific rather than on a vague "needs
more work": a second blind instance in four cases, an unresolved carrier fork in one, a priced
instrument choice plus a second instance in one. None of the six is close by virtue of anything
this consolidation found; all six are exactly as far as both reads, independently, say they are.

## 4. Conflicts

### 4.1 Between the two reads

None substantive. Every place one read reached something the other did not is recorded in `#142`'s
own reconciliation section as agreement once compared, not as a live disagreement. `#142` found
the format-row split that `#117` missed, and says so about itself. `#142` in turn credits `#117`
for the multiplicative-chain marker correction, the min-plus end-to-end mechanism finding, and
the Q12 cross-row conflict, correcting its own earlier sections against them. The one place the
two reads name an actual disagreement is not
between each other: `#117`'s section twelve disputes a claim in a third file, `195`, that two of
the ratified rows (`the_format_concept_carries_three_things_upward...` and
`a_composed_expressions_region_is_never_inherited_from_its_parts`) say the same thing from two
sides and should be read as a convergence. `#117` argues they are adjacent, not identical (one is
about laws only a composition has, the other about a law the parts and the composite share; one
carries an instrument, the other cannot), and that treating them as one convergence would
wrongly lift a one-expert row to two by pairing it with an arrival that is not an arrival at it.
Neither read's cluster and `195` overlap enough for `#142` to have an independent view; `#142`
does not mention `195` at all. So this is a real, stated disagreement, but it is between `#117`
and a third file, not between the two inputs this consolidation was asked to reconcile, and the
ratifying rulings do not treat the two rows as one convergence, consistent with `#117`'s position.

### 4.2 With standing rows

**`question::reduction_order_or_associativity` (Q12).** Open, `decider = "panel"`, not reserved.
Two proposal rows reaching this obligation answer it in incompatible shapes, per 2.9 and 2.10
above. This is a live, unresolved conflict, found by one read, corroborated by the other only
after reading the first, explicitly deferred by the promoting commit, and untouched since. It
sits exactly where `ruling::arms_over_regions_are_the_fundamental_heart` (the organising shape
the whole panel converged on) meets a row proposing the opposite shape for one question. Whoever
next touches either row owes this obligation a resolution: either the licence row is narrowed to
not claim a single rule over reassociation, or an argument is made for why reassociation is an
exception to the arms-over-regions convergence, stated as such rather than left implicit.

**The obligation's own four-way split.** Both reads independently establish it; the obligation
row's `gap` field states both readings and their divergent groupings (four groups vs. five) and
explicitly declines to file the split, on the ground that "where an obligation's edges sit is a
design call this ruling round does not make." That stands as written and is not mine to resolve
either; a design call, named as such, is outside a consolidation's remit under `do-not-question-
the-tier-above.md`.

**No conflict found against any other ratified row.** I checked `question::reduction_order_or_associativity`
by name per the brief and found the Q12 conflict above; I did not find a second standing row in
tension with anything either read or the four now-ratified rulings state.

## 5. What I could not establish

I did not rerun any instrument myself; both reads and the promoting commit's own `probe` and
`law` edits already carry committed, reproduced evidence for everything the ratified rulings
state, and rerunning a fourth time would be a run rather than a new instance, per this panel's own
standing on what counts as one. I did not find the commit, agent, or session that read these two
files and authored `6ae6cade`; the commit is signed by op's own git identity and its message
reads as a coordinator's summary of exactly these two files, but I have no artifact naming who or
what wrote it beyond `git log`'s author field, and I am not asserting a mechanism I cannot cite.
I did not check whether `proposal::a_min_plus_computation_needs_monotonicity_as_well_as_an_absorbing_top`
should carry an `obligation` edge to this cluster; both `#117` and I agree it is missing one, and
wiring it is a registry edit I was told not to make.
