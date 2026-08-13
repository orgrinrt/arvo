# 90. Consolidation: derived algebraic laws

**Author lens:** Giesen. Cost models, what the machine actually does, and the discipline that a claimed
win is real only where a named predicate holds. I wrote `74`, the number-system consolidation; the
independent entailment check on this file must be run by someone who wrote neither.

**Position:** the consolidation of the derived-algebraic-laws unit, ninth file of the 4-4-1. Members
compressed: `76`, `77` (cold derivations, parallel and blind), `79`, `80` (attackers), `81` (the
checkpoint), `82`, `84`, `86`, `89`. Op wrote into the unit four times while it ran, at `83`, `85`, `87`
and `88`, and those four files govern everything below.

**What this document is, and is not.** Per op's `87` section 1, a consolidation is **input, not canon in
miniature**: `mock/canon/` stays empty until every topic is done, the canon is then written once from all
the consolidations read alongside the members they compressed, and op ratifies that single act. So this
file is the topic's best available compression and has no standing beyond that. It is standalone and
versioned, never a delta; where the reader needs the full predicate of a finding, the member's own
F-numbered statement is the authority and is cited by file and line. Nothing here settles anything.

**Gates.** Canon gate: passes, situation two. No canon exists; `mock/canon/` is absent and `mock/crates/`
is empty by the declared mutation order; this panel is writing the first canon. Checked against
`INTENTS.md` in full: I13 is the one RATIFIED entry, narrow on op's own instruction that it means no more
than he said (`INTENTS.md:200-252`); I14 is IN FORCE (`INTENTS.md:254-283`); I15, I16 and I17 are STATED
and were read at the source (`INTENTS.md:285-340`); and against `RULES.md` in full, and op's `83`, `85`,
`87` and `88` at their sources. Test gate: no suite exists and the mock workspace has no members; the
substitute is the probe discipline, and this file applies it by marking, at every claim it carries,
whether the instrument behind the claim was shown able to fail. Four instruments in this unit were found
structurally unable to fail on the thing they validated, one of them in the last member's own first run,
and R13 carries that as a result rather than a footnote.

**Nothing in this unit is priced.** No bench has run, and op's `87` section 3 says so plainly
(`87:83-87`). Every instruction count, every wall-clock second and every compile time in the members is
an ad-hoc quick spike with no substance for any how-much question. Where a magnitude appears below it is
carried for its qualitative content only: which instruction appears in which arm, which build accepts and
which refuses.

---

## 1. The question, and the shape the answer took

The unit was dispatched on: what does op's three-verb vocabulary for a typestate, derive, validate,
erase, mean when the object is not a container width but a claimed algebraic fact; what is a law a fact
about; and what does I13's "arms with const predicates" require of a law layer.

The unit's provenance work settled the vocabulary's own status first. The sentence "have the typestate
derive the matching container and numeral representations, then validate, and erase" is op's, verbatim,
traced by `80` to `seed/OLD_SETTLED_container.md:33-36`, quoting him at `135b:12-16` in the closed
formalization panel, which is a demoted body (`RULES.md:525-548`, `INTENTS.md:27-33`). Op re-entered the
vocabulary into this panel himself by answering a question about "then validate" at `28:67-95`. The
status both attackers converged on and the checkpoint adopted (`81:110-117` context, stated at
`81:49-72`): derive, validate and erase are **op's live names for three things a typestate does, and not
a ratified acceptance test**. `77` had refused to use the sentence untraced (`77:4-11`); `76` used it
unflagged (`76:5-9`); the trace closed the question two files later.

The answer's final shape, compressed to its shortest honest form:

**A law is a quantified proposition, not a value, so the container pipeline does not transfer to it
unchanged.** For a container, derive computes a value, validate checks it, erase leaves it, all at
monomorphisation. For a law, the truth is established somewhere (and where turned out to be the unit's
hardest question), the typestate **routes** verdicts rather than re-establishing them, a fourth verb,
**select**, is where a law does its only work (`80`'s staging reading, marked as `80`'s own synthesis at
`80:797-799`), and under op's I15 the selection must be const-time branching that monomorphisation and
const solving erase to one lowered path (`85:20-25`, `INTENTS.md:285-302`).

**Where a law verdict's truth is established moved three times, and ended somewhere nobody proposed at
the start.** The cold derivations built per-instantiation exhaustive checking and reported it working
(`76:60-72`, `77_probes/probe2_works_validate_erase.rs`). `80` measured that it works at model widths
only and fails in the direction that matters, and proposed the model-band cross-check as the escape
(`80_probes/p2c_closed_form_checked_on_a_model.rs`). `84` defeated the band's transfer by construction
(`84_probes/p2_defeat_the_cross_check.rs`, `84_probes/p2b_no_band_can_catch_it.rs`) and replaced it with
verdicts **computed at the shipped width** by a complete test set for the ring fragment
(`84_probes/p4_difference_certificate.rs`). `86` proved that criterion at every arity and degree,
simplified it to evaluation on the degree box, and extended the defeat to saturating laws
(`86_probes/p3_saturating_threshold_family.rs`, `86_probes/p4_sat_defeat_gate.rs`). `89` showed the
saturating fragment is decided by **the same degree-box test set through the same triangularity
argument**, built it as a `no_std` const gate with controls that show the hypotheses inverting the
verdict when violated, and retired an entire piecewise apparatus `86` had built
(`89_probes/p2_const_gate.rs`, `89_probes/p1_min_form_and_the_degree_criterion.rs`).

The one-sentence form of the destination, which is `89`'s and is the most compressible thing the unit
produced (`89` section 11 item 1): **a law verdict at a shipped width is computable at that width, on a
test set determined by the law's degree, for fragments where the failure condition is preserved along the
coordinatewise order; the verdict transfers nothing and needs no band.** Divisibility by `2^W` plays that
role in the wrapping fragment and clamping at `MAX` plays it in the monotone saturating one, and the
falling-factorial triangularity is the shared argument. That sentence passes the canon's permanence and
equivalence tests as intent; everything below it is evidence, region and cost.

---

## 2. What the unit established, with predicates

Each result names its establishing instrument, its region in the ratified notation (absence of a
dimension is the strongest negative claim and is meant), its rung, and the correction that reached it
where one did. Predicates are never widened in place: where a later member widened or corrected an
earlier one's, both are named and the later claim carries its own attribution.

### R1. A law is a fact about an operation composed under a fixed arithmetic semantics, never about a type or an operation alone

The one claim in the unit at the TWO EXPERTS rung: `76` and `77` derived it cold, in parallel, blind to
each other, from I9, in two different formal traditions (`76`'s congruence closed under a strategy's
exposed operations, `76:120-131`; `77`'s judgment between expression forms of arbitrary finite length,
`77:372-378`). `79` section 7 audited the tally and corrected `76`'s attempt to count its own earlier
file `42` as further corroboration: the honest count is two independent instances plus one earlier
same-author instance at lesser weight (`76:282-297`).

The dimension list a law's region needs grew all unit and is not closed. `79` section 5, from `63`'s
cube: operation, sign domain, overflow policy, fraction width, representable-set shape, and a
placeholder, "whatever named axis a strategy resolves to" (`79:200-219`). `82` added the **declared
operand window**, which separates verdicts with all six of `79`'s coordinates fixed (`82` F6 against the
grading entry at `OPTIONS.md:1550-1556`). `80` section 4.5 added the **schedule** and, resting on `63`'s
C9 via `79:230-239` and `79:241-251`, the **rounding rule**, for chain laws. `86` F4 added the
**constant-embedding convention**, which alone manufactures or destroys a threshold family
(`86:396-402` neighbourhood, established at `86_probes/p3_saturating_threshold_family.rs`). `84` section
3 item 2 named the width-indexing of a law family as an author's convention nothing checks
(`84:190-194`). This list is a floor, not an enumeration; treating any snapshot of it as closed is the
shape op rejected three times in one sitting (`88:118-123`).

The existence-and-locus half of the dispatch also closed inside this result, in `76`'s phase two
adopting its own prior file: arvo owes algebraic laws **as a vocabulary of checkable facts, never as a
rewriting engine that acts on them**; the facts belong in arvo because only arvo knows its own axis
values, and the decision to act on a fact belongs to whoever performs the rewrite (`76:299-309`, with
op's direct refusal of the engine relocation carried at `DROPLIST.md:19-22`). The wrapping-associativity
base fact behind much of the unit is `76`'s exhaustive probe
(`76_probes/probe1_associativity_exhaustive.rs`, `76_probes/probe1_output.txt`): wrapping addition
associates universally over `u8`, signed `i8` saturating addition fails on 4,177,792 of 16,777,216
triples with first counterexample `a = -128, b = -128, c = 1`.

### R2. A composed expression's region is never inherited from its parts

`79`'s P4, verified at source and re-run by both later attackers (`82_probes/p0_rerun_of_79_p1.rs`,
`82_probes/p0_rerun_output.txt`): unsigned saturating addition is associative on every triple (predicate
`any` on the operand dimension), the composed `(a+b)-c == a+(b-c)` fails on 13,882,880 of 16,777,216
triples, 82.7484%, and the exact holding region is a four-way case split on the two clamp events plus a
residual operand condition, zero sufficiency and zero necessity violations, with a mutant control
reintroducing 32,640 violations so the zero residue is not an artifact
(`79_probes/p1_compositional_predicate_search.rs`, `79_probes/p1_output.txt`,
`79_probes/p1_negative_control.rs`, `79_probes/p1_negative_output.txt`). Predicate as `79` stated it:
`N = 8, sign = unsigned, policy = saturate, op pair = {+, -} composed, F = 0, threads any, features any`
(`79:95-100`), threads and features claimed on a semantic argument about pure value functions.

The general statement, which `79` put as a caution on `63`'s C6 and `84`/`86`/`89` never disturbed: even
where a component operation's own hypotheses hold unconditionally, the composition needs its own
derivation (`79:112-121`; C6's own text and scope are at `63:659-673`, its zero-residue evaluation at
`63:389-392` resting on `57b:247-297`, and the attack it survived at `74:273-275`). `76`'s probe1b independently established the same
headline percentage (`76_probes/probe1b_associativity_refined.rs`, `76_probes/probe1b_output.txt`,
re-run to the digit per `79_probes/p1_output.txt` section 1).

### R3. A law stated as an author-written marker is a declaration checked by nothing

`80` section 3.1, which is `68:145-148`'s paper-checking-paper hole one coordinate up
(`80_probes/p1a_declared_law_lies.rs`): two overflow policies declare one `AssocAdd` marker over a
four-bit signed window, one declaration is false, the compiler raises nothing, and the licensed
reassociating consumer returns a different answer on 16,268 of 65,536 vectors, 24.8%, with a zero
arity-2 control. Predicate: `N = 4, sign = signed, policy = {wrap, saturate}, op = add, F = 0,
arity = 4, threads = 1, features any`. The repair, computing the permission from the policy's own map so
the false instantiation is `E0080` (`80_probes/p1b_computed_law_refuses.rs`), works, and R4 is what it
costs; `77` built the same mechanism independently, including the refused inline-const spelling that
rustc answers by naming the forbidden `generic_const_exprs`
(`77_probes/probe2_works_validate_erase.rs`, `77_probes/probe2_works_output.txt`,
`77_probes/probe2_bad_case_refused.rs`, `77_probes/probe2_bad_output.txt`,
`77_probes/probe2_fail_inline_const_expr.rs`, `77_probes/probe2_fail_output.txt`). The unit's prior compiled instance of the same class is the droplist's refusal of a single
associativity flag as an algorithm gate, which admits the wrong preset and refuses the right ones in
opposite directions for two algorithms at once (`DROPLIST.md:25-29`, read at source by `79` section 4).

### R4. Exhaustive const-eval verdicts do not reach shipped widths, and every frontier number is a fact about the procedure, not the law

The unit's deepest correction chain, four instruments long, and the chain **is** the finding.

`80` measured the frontier as a curve in (width, arity), collapsing along arity: widest evaluable width
19 at arity 1, 9 at 2, 5 at 3, 3 at 5, 1 at 8, every refusal `long_running_const_eval`
(`80_probes/p2_frontier.py`, `80_probes/p2_frontier_output.txt`, `80_probes/p2_frontier.json`),
extending `68`'s single measured point (`68:196-211`) and the droplist record that the wall is a
total-step budget rather than a width ceiling (`DROPLIST.md:234-235`). `76`'s own narrow-domain choice,
"16 values, 4,096 triples, small enough for const-eval to finish", was the frontier being observed and
reported as a nuisance (`76:74-83`, relocated by `80`).
Asymmetry at a shipped width: the false verdict exits at its first counterexample, `E0080` in 0.50s; the
true verdict, the one that licenses an arm, is refused after 4.48s
(`80_probes/p2b_swept_verdict_at_shipped_width.rs`). Allowing the guard, which `68:206-208` had noted is permitted and left unpriced, buys three bits and
no more:
widths 6, 7, 8 accept at 5.85s, 49.06s, 370.95s, a per-bit ratio of about 8x, which is the enumeration's
own `2^3`; width 9 did not finish (`80_probes/p5_allow_the_guard.py`,
`80_probes/p5_allow_the_guard_output.txt`). `80` also corrected the live workspace rule that had quoted
a 4x-per-bit rate without its arity (`unstable-features.md:41-45`, inherited by `68:209`): the rate is
`2^k` for arity k, and there is no such thing as "the" rate (`81` corrections section).

`82` then showed the frontier reads the **encoding**: three spellings of the identical check, agreeing
on 952 violations at width 4, reach widths 6, 5 and 5 by default, so `80`'s table rows are one bit low
for a lighter per-tuple spelling (`82_probes/p11_the_frontier_reads_the_encoding.py`, F17). `82` also
bought exactly one bit from a sign-uniform operand restriction, which crosses `i8`: the positive verdict
accepted at width 9 in 173.52s guard-allowed where the full set does not finish
(`82_probes/p9_positive_verdict_at_a_shipped_width.py`, F16).

`84` and `86` then showed the frontier reads the **procedure**, which is the extreme member of the same
class: inside a fragment with a complete test set, the positive verdict at width 64 costs a degree box
rather than `2^(W·k)`, so the wall is the price of semantic ignorance about the law, not the price of a
positive verdict (`84` section 5, `86` F3). The box has its own frontier: about `2^14` grid points by
default and `2^20` guard-allowed at width 64 on that host (`86_probes/p2_box_frontier.py`), and `89`'s
box gate landed on the same default boundary on a different fragment
(`89_probes/p7_arity_frontier.py`), with the off-guard behaviour diverging and the mechanism behind
the coincidence explicitly unestablished (`89` F6, section 4). `89`'s own gate frontiers, the
width-independent per-law verdict cost and the per-library checker cost, are at
`89_probes/p3_frontier.py` and `89_probes/p3b_checker_frontier.py`.

The consequence, owed to whoever quotes any of these numbers: **a const-eval frontier is a function of
domain size times per-tuple cost times procedure**, and a frontier cited without all three is the same
defect at three depths that `74:942-943` recorded for a count cited without its operation.

### R5. The model-band transfer is defeated, in both fragments, and the polarity theorem is what survives of it

`80`'s escape from R4 was the closed-form verdict cross-checked against the sweep over a model band,
with "the transfer of the agreement from widths 2 through 5 to width 64" claimed as the single named
residue (`80:297-299`, restated at `80:688-689`, register form at `OPTIONS.md:1896-1901`, elevated by
the checkpoint at `81:110-117`). `80` itself named this the piece it most wanted broken (`80:807-809`).

`84` broke it by construction. The threshold family `L_k`, every product of k consecutive integers
vanishes mod `2^W`, is true at every width up to `k - s2(k)` and false above, by Legendre's formula,
verified exhaustively for nine family members with the instrument shown able to fail
(`84_probes/p1_threshold_family.rs`). `L_16` run through `80`'s own construction: band agreement green,
perturbation control still refuses, arm licensed at width 64, **law false at width 64**, refuted in
sixteen const multiplies under `--cfg audit` (`84_probes/p2_defeat_the_cross_check.rs`). `L_64` is the
stronger member: true at every width 1 through 63, false exactly at 64, so **no model band below the
shipped width, at any guard setting, on any host, could have disagreed**
(`84_probes/p2b_no_band_can_catch_it.rs`). The family is not exotic: magic constants have exactly this
downward-correct, upward-false shape, and the INV3 sibling, `(x * 3) * INV3 == x` with the 64-bit
inverse constant, holds through width 65 and fails at 128 with witness 1, computed
(`84` F6, re-derived by hand at `86_probes/p0_rerun_of_84_probes.txt`). `68`'s transfer proviso, no
law-relevant path dispatches on width (`68:213-219`), is satisfied by `L_64` and is therefore not
sufficient either, and the workspace rule that a model-width check needs its own named transfer
argument (`unstable-features.md:54-81`) gains its strongest instance yet, past the droplist's earlier
const-tag-dispatch counterexample which at least needed a width-keyed path (`DROPLIST.md:230-232`).

`86` extended the defeat outside the ring fragment: `E_d`, `x^d == x^(d+1)` under unsigned saturating
multiplication, constant-free, clamps live, truth set exactly widths 1 through d for all eleven swept
members, so `E_63` is true below the shipped width and false at it, and the band mechanism licenses it
green end to end (`86_probes/p3_saturating_threshold_family.rs`, `86_probes/p4_sat_defeat_gate.rs`, F4
and F5).

What survives of band transfer is exact and narrow: `84`'s polarity theorem for the ring fragment
(`84_probes/p3_transfer_polarity.rs`, F3, measured at arity 1 by its own admission at `84:520-521`). Equation laws have initial-segment truth sets, so a band
FALSE embeds upward with its witness and a band TRUE exports nothing; disequation laws are the dual.
`86` proved the polarity at every arity as a corollary of R6, discharging that limit, and measured that **saturating laws have
neither direction**: gapped and interior-run truth sets exist among constant-free depth-2 terms, 16 and
24 of 21,945 pairs, including a law true at width 2 alone and a genuine ring identity false at width 2
alone (`86` F6). `89`'s p8 supplied the mechanism: the gapped sets occur exactly where the min-form
property fails, a ceiling clamp discarding magnitude a later negative operand would have restored, and
zero of the 8 non-monotone pairs in its own term space have both sides min-form (`89` F9,
`89_probes/p8_why_signed_is_gapped.rs`).

Composed with I15 the defeat's weight doubles, and `84`'s addendum says it plainly: with the runtime
column gone, the compile-time verdict mechanism is the **only** validation path there is, so a false
soundness story there has no catch anywhere (`84` section 12).

### R6. Inside a fragment with a complete test set, the verdict is computed at the shipped width, and two fragments now have one

The constructive replacement, three members deep.

For the **wrapping ring fragment** (add, sub, mul, neg, integer constants, `F = 0`): a law vanishes
identically on `(Z/2^W)^k` exactly when `2^W` divides every mixed forward difference at the origin,
which by `89`-independent classical results (Kempner 1921, Singmaster 1974) and `86`'s complete tensor
proof (`86` section 2) reduces to **evaluation on the degree grid**: agreement on
`prod_i {0..d_i}` decides the whole domain, a false law has a witness inside its own degree box, the
exact truth threshold is the minimum 2-adic valuation over the grid's integer values, and equation truth
sets are initial segments at every arity (`86` F2, measured at genuine degree where `84`'s battery had
none: 1,975 pairs zero mismatches, 25 tensor members matching the Legendre prediction exactly,
`86_probes/p1_multivariate_at_real_degree.rs`). Width-64 instances at genuine degree: `(x)_33 (y)_33`
false in 1,156 evaluations; `(x)_34 (y)_34` **true at width 64 and false at 65**, the sharpest reminder
that a verdict is per width (`86` F2).

For the **monotone unsigned saturating fragment** (sat add, sat mul, nonnegative clamp-embedded
constants), whose right framing `86` had already stated, saturating arithmetic is exact integer
arithmetic composed with clamps rather than modular arithmetic (`86:269-270`): `89`'s Theorem A, saturating evaluation equals the exact integer polynomial clamped once,
23,950,484 evaluations zero disagreements (F1, `89_probes/p1_min_form_and_the_degree_criterion.rs`);
Theorems B and C, agreement on the degree box decides any width at any arity, with the same
falling-factorial triangularity carrying the proof, 9,503 univariate and 3,006 multivariate verdicts
zero mismatches, direct falsification searches of 770,006 and 117,780 pairs finding no counterexample,
and 507 cases attaining the bound exactly so it is not slack (F2, F3,
`89_probes/p4_multivariate_box.rs`, `89_probes/p10_wider_falsification.rs`).

Both are **const gates on the pinned toolchain**, `no_std`, no feature gates, no `dyn`, no `TypeId`, no
allocation: `84`'s certificate gate replacing the band gate (`84_probes/p4b_certificate_gate.rs`),
`89`'s saturating gate with four controls (`89_probes/p2_const_gate.rs`, F4), and `89`'s signed window
gate deciding `82`'s fold law at width 64 in 0.50s (`89_probes/p6_signed_window_gate.rs`, F8). The
controls are the load-bearing part: `--cfg perturb` refuses at the rung-0 implementation check, `--cfg
nonfragment` refuses at the fragment check, and the two `unchecked_*` builds **accept while the same
crate proves the licensed law false at width 64**, with witnesses 101 and `(MAX, MAX, MIN)`. Violating a
hypothesis does not degrade the verdict, it inverts it (`89` F4, F8).

The trusted base after the gate runs, itemised at `89` section 8 extending `84` section 3 and
`68:221-253`: fragment membership (a const fn over the term's node array, not a comment), the window
hypothesis for signed, the degree bound (exact for the saturating fragment; over-approximation safe,
under-statement unsafe and shown live), the encoding of the law into the term array (the one item with
no mechanical check), rustc's const evaluator plus pin plus host plus guard budget, and the checker's
implementation validated at rung 0 against sweeps, **paid once per library rather than per law** (`89`
F5 splits the frontier into a width-independent per-law verdict cost and a per-library checker cost).
**The transfer is not on the list. There is none.** The wrap fragment additionally carries the
register-noted collapse that its separately measured properties are one group theorem rather than four
facts (`OPTIONS.md:1144-1147`). And what a consumer is entitled to conclude is bounded the way `77`
stated and `80` sharpened: exactly as much as the obligation their own instantiation discharged, with
erasure carrying nothing across instantiations, so after erasure the guarantee is a property of the
generator that emitted the code rather than of any value (`77:169-186`, `80` section 9).

Boundaries, refuted rather than cautioned: `F = 0` is a hard boundary of the criterion, two-point
witness `(x >> 2) * 4 == (x >> 1) * 2` true on its box and false at `x = 2`, 82,002 of 417,384
shift-carrying pairs true-on-box-false-in-domain (`89` F10, `89_probes/p9_two_routes_closed.rs`);
general signed with both clamps reachable has no procedure, with the cheap saturation-radius route
closed by a non-constant tail (`89` F11) and the expensive route named and unbuilt (section 8).

### R7. Sign uniformity of a declared operand window is exact for signed saturating fold reassociation, and it is the criterion's own hypothesis

`82` established the predicate by measurement (`82:281-285`): a declared window `[LO, HI]` with `LO >= 0 || HI <= 0`
matches associativity on the window's generated closure exactly, both directions, every interval at
widths 2 through 6, four weakened predicates each breaking it (`82_probes/p2_sign_uniform_lifting.rs`,
F6), a hypothesis taken from the bracketing evidence the register carried, `35`'s 70.1% reassociation
divergence for the signed straddling case against zero for the other three combinations and `55b`'s
decomposition in which zero of 952 divergent triples are sign-uniform (`OPTIONS.md:1093-1096`,
`OPTIONS.md:1117-1135`, both register-routed); and the interval is not the limit, the same predicate is necessary and sufficient over **every
operand set of any shape**, exhaustively at widths 2 through 4 over all subsets
(`82_probes/p10_is_the_interval_the_limit.rs`, F15). At the shipped width, sampled: straddling window
63.62% divergent over four million length-8 vectors, both sign-uniform halves zero (`82` F7, stated as
sampled). The arity-3 count 952 at width 4 reproduces `80`'s p6 and the count `74:939-943` had
corrected onto the right operation, a three-instrument agreement.

`89` then made the predicate proof-shaped: sign uniformity is exactly the hypothesis under which
Theorem C applies to the signed case, so the sufficiency direction is now a **decision computed at width
64 by a const gate** rather than an argument (`89` F7, F8, instrument at
`89_probes/p5_signed_windows.rs`: 25,120 admissible-window checks zero mismatches, 3,808 of 62,210
straddling checks wrong, which is what makes the hypothesis load-bearing). Before that gate existed,
`82` had pushed its own min/max identity as wide as sweeps go, width 16 over pairs and width 12 over
triples, and named the last step as prose (`82_probes/p12_shrinking_the_transfer_residue.rs`,
`82:930-938`), which is the argument `84` identified as the construction's real load-bearing item.
This discharged the item `84` section 6 had named "the only load-bearing unmechanised thing in the
construction" (`84:344-347`) and `86` had left on the named-argument route (`86:292-293`): the first
law in the unit to move from argument to procedure. `82`'s necessity direction remains `82`'s
measurement, corroborated but not re-proved (`89` section 6 is exact about this).

Two subsumptions worth keeping: `76`'s surprise that unsigned saturating addition is associative
(`76:51-53`) is this theorem with the sign domain moved from container to declaration (`82` section 7);
and the wrapping-grading register entry is right about the representable set and incomplete, because
the grade is not a property of the policy alone, the declared window is a further coordinate
(`82` F6 against `OPTIONS.md:1550-1556`).

### R8. What a lifting is, and the closure criterion that decides it

`82`'s assigned question was `80`'s cheapest-next-instance: can a measured trajectory region become a
declaration. The answer is per region, and the discriminator is **closure**: a trajectory condition
lifts into a declaration over the operand set exactly when the condition survives the set's closure
under the operations the law is about (`82` section 12, stated as one-directional sufficiency, with the
member's own honesty note that its three instances, `77:250`, `76:370-372` and its own box
characterisation, share an author and a framing). The distinction itself predates the unit: the register's reachability entry describes `42`'s condition
over a fold's **declared** operand range, and "declared" is the word that moves a trajectory fact across
the binding-time boundary (`OPTIONS.md:1113-1115`, read by `80` section 6). P4's condition does
not survive closure, so its declared-range lifting reaches only the degenerate sub-cases: the maximal
box is 21.98% of the holding set and every non-degenerate holding box is clamp-free, zero residue at
four widths against a live control (`82_probes/p1_box_lifting_of_p4.rs`,
`82_probes/p1b_is_every_lifted_box_degenerate.rs`, `82_probes/p1b_output.txt`, F1 through F3). Sign uniformity survives closure,
and R7 is what that buys. A bounded length survives closure up to the bound, so a length-aware
predicate licenses strictly more and needs the length at const time, connecting to the capacity-static,
length-dynamic staging boundary `80` section 7 derived from the register's fold entries
(`OPTIONS.md:1055-1086`, `OPTIONS.md:1063-1065`, `OPTIONS.md:1236-1238`); the length-aware construction
is unbuilt (`82` F13, section 10).

Two negative results guard the whole lifting question. The declaration a consumer would naturally
write, "my exact result stays in range", is **unsound**: true while the law is false on 49.80% of the
domain, wrong in 87.5% of the boxes satisfying it, with a hand-checkable witness
(`82_probes/p1_output.txt` section 4, F4). And const-available operands decide P4 only at the
degenerate values, 4.52% of the holding set at best, though the `{a, c}` row's 32,640 **refused**
configurations are a genuinely new verdict kind, a const proof that the arm is wrong for every runtime
operand (`82_probes/p6_const_availability_lattice.rs`, F5).

### R9. The licensed category is const-available, the binding times inside it differ, and op refused to rank them

One decomposition question was retired before op's files arrived: `80` section 1.2 found that op's
own answer on "then validate", usage, admissibility, self-validation, all that makes sense
(`28:82-95`, carried at `OPTIONS.md:57-88`), names three **things validated**, all compile-time acts,
while the binding-time axis was `68`'s and not among them (`68:126-129`); the runtime column of that
grid was then deleted outright by I15, so the grid survives only as the observation that the unit's
evidence sits at usage-at-compile-time and that admissibility and self-validation of a law declaration
remain uninstrumented. Op's `83`, answering the checkpoint's Q-C, dissolved the unit's
typestate-against-trajectory axis: the
axis is **const-available against not**, a predicate is a const expression that may call const functions
and read const data from outside the typestate, with the typestate one source among several. What his
words do not reach, and this unit did not extend them to: whether a genuinely non-const condition may
gate anything (`83`, in full, and `82` section 5 for what it changed mid-flight).

`82` measured that const-available is not one moment: four constructions, four binding times. A
crate-level const and a structural trait bound refuse on dead code under `--emit=metadata`; an inline
const in a non-generic fn refuses at codegen; a const assert in a generic's associated const refuses
only where the instantiation is **reached**, so a wrong declaration in an unreached `pub fn` compiles
clean at every setting (`82_probes/p3a_the_construction.rs`, `82_probes/p8_binding_time_ladder.sh`, F8;
the bound-position spelling is refused by the compiler naming the forbidden `generic_const_exprs`,
`82_probes/p3e_bound_position_refused.rs`, F9; the structural decomposition that does refuse at type
check is `82_probes/p3d_structural_permission.rs`; the outside-the-typestate const form is
`82_probes/p7_window_from_outside_the_typestate.rs`).

Op's `85` section 2 then refused the ranking question as policing (`85:49-59`, I16,
`INTENTS.md:303-318`): the requirement on a law's expression is **functional**, it must actually work,
meaning reach one lowered path through monomorphisation and const solving, and which construction gets
it there is case by case. `82`'s measurement is re-read under that, not discarded: the
reachability-dependent rung is not a banned category, it is a construction that fails the functional
test **when used for a library-wide claim**, and `84` section 7 supplied the specification that makes
the split safe in one sentence: a rung-3 permission quantifies over reached instantiations, and a
library claim must sit at rung 0. Both shipped gate constructions in the unit conform (`84` p4b, `89`
p2 and p6).

### R10. Where a law pays is strictly narrower than where it is true, and the win is the microkernelling shape

`80` section 5.2, the probe that refuted its own thesis: at `F = 0` the emitted assembly aliases the
fused and general forms to one symbol, the backend performed the distributive rewrite unaided, and the
arm bought nothing in exactly the region where the law holds
(`80_probes/p3_select_and_erase.rs`, `80_probes/p3_lib.s`, `80_probes/p3_asm_report.txt`; the
symbol-aliasing instrument is `68`'s, from `68:179-195`, turned to a less comfortable use). So the
question a law layer answers is not whether the law holds but **whether it reaches a lowering the
backend could not prove for itself**.

Where it does, both attackers measured the same shape on the case a backend structurally cannot
reassociate, a saturating reduction. `80` unsigned: 6.000 instructions per element for the fold as
written, **8.500 for the first licensed attempt** (the law true, the arm legal, the bounds unprovable,
vectorisation abandoned), 0.250 with the bounds proof via `chunks_exact`, 0.141 unrolled, against a
wrapping control at 0.125, with the vector saturating add appearing only in licensed arms
(`80_probes/p4_what_the_law_unlocks.rs`, `80_probes/p4_asm_report.txt`). `82` signed, with the control
that decides attribution: the bounds proof **without** the law gets a 16x unroll with the serial clamp
chain intact and no vector instruction, 6.188 against the licensed 0.250, so the law is load-bearing
and not the bounds proof (`82_probes/p4_what_the_lifted_arm_unlocks.rs`, F10). Three distinct licensed
declarations assemble to one symbol, so the declaration erases (F11), and the licensed arms agree with
the fold as written on 200,000 vectors per window while the refused windows disagree at scale, so the
agreement rows prove something (F12, `82_probes/p5_agreement_and_the_length_axis.rs`). Every number
here is instructions per element read off emitted assembly: an ad-hoc quick spike, unpriced, carried
for which instruction appears where and nothing else.

The value-gated alternative was measured and then closed on principle: materialise both arms and select
with `csel`, 13 instructions against 6 and 3, worse than either static arm (`80` section 5.1), and
under I15 "never any runtime checks, ever" it is out as a design option regardless of its cost
(`85:20-25`, which names `80` section 5.1 explicitly).

### R11. Chain laws split by whether a lifting theorem exists, and the schedule kind is the one op's accuracy intent is stated over

`80` section 4.5, correcting its own section 4.1: grouping questions lift from arity 3 by the
generalized associative law, so the frontier's arity axis never touches them, measured as wrap zero
divergence at n = 2 through 5 and saturate nonzero from 3 (`80_probes/p6_which_chain_laws_reduce_to_arity_three.rs`);
schedule questions, stepwise rounding against round-once, are vacuous at n = 2 and a fresh statement at
every higher n, with no lower-arity statement implying them. The schedule kind carries rounding, sits
outside both R6 fragments, and is the kind I7 is stated over (`INTENTS.md:119-121`). `86` F3 priced
what the box procedure does for multilinear chains: a chain law must reach length 15 before the default
guard notices, so the residual exposure is high-arity non-liftable ring laws, of which the unit
exhibited none.

The cold derivations' chain findings survive inside this frame: addition needs no chain machinery
because sums of quantized values are exact, multiplication does because the exact product needs `2F`
bits, the naive error is magnitude-driven and non-monotone rather than length-driven
(`77_probes/probe1_chain_error.py`, adopted by `76:370-372` as the closure framing), and the
accumulator saving is rounding-conditional per `63:515-521`, resting on `60_probes/p_d.out`, which
is why the rounding rule is a predicate dimension (R1).

### R12. A law verdict is invariant under change of encoding and container

Carried by `80` section 8 from the number-system consolidation, read at source: a law contract is
decided at the pair of identity and selected adaptation, reads neither encoding nor container, and the
same identity with only the adaptation moved flips a bound from accepted to refused (`74:144-147`,
`74:507-511`). Consequence nobody had stated before `80`: the law layer's compile-time computation is
keyed on strictly less than the container derivation's, so a verdict can be computed once per
identity-and-adaptation pair and reused across every container realising it. Whether that matters is
unpriced. This is also why every finding in the unit is predicated on `policy = wrap` or
`policy = saturate` as overflow-policy dimensions of the law rather than on any marker name, which is
what lets the whole unit survive the strategy set being reshaped (`86` addendum, `87` section 3).

### R13. The unit's own instruments failed in one repeated way, and the record of it is a result

Four instances, four authors' intentions, one failure: nobody asked what the instrument would say if
the thing it validates were broken. `84`'s p4 battery pinned its thresholds at zero on its first run
and caught itself (`84:41-48`). `84`'s multivariate battery contained no law of true per-variable
degree above one, so the interesting branch of the criterion had zero instances; `86` found it and
corrected F4's predicate (`86:73-92`, against `84:464-466` and `84:516-519`, source at
`84_probes/p4_difference_certificate.rs:331-336`). `86`'s p5 battery (`86_probes/p5_sat_piecewise_procedure.rs`), 3,708 verdicts, could not
distinguish the piecewise procedure from one with the entire breakpoint apparatus deleted; `89`'s
mutant M1 changes not one verdict, so the number 3,708 was doing work it cannot do (`89` section 1,
`89_probes/p0_mutate_86_p5.rs`). `89`'s own p3 checker table measured nothing on its first run because
every swept law was false and both sides exited early; the corrected instrument asserts inside the
generated crate that the laws are true, and both runs are on disk
(`89_probes/NOTE_p3_checker_half_first_run.md`). `80` had already committed two first-run failures with
notes in the same spirit (`80_probes/NOTE_p1a_first_run.md`, `80_probes/NOTE_p2_first_run.md`), one of
them the exact setup-that-helps shape.

Beside this sits the reproduction chain, which never broke: `79` re-ran `76`'s and `77`'s probes, `82`
re-ran `79`'s, `84` re-ran `80`'s p2c (`84_probes/p0_rerun_80_p2c.txt`), `86` re-ran six of `84`'s
(`86_probes/p0_rerun_of_84_probes.txt`), `89` re-ran `86`'s p5, and everything reproduced to the digit.
**Every defect found in this unit was in what an instrument could not see, never in whether it
reproduced.** The cheap disciplines that caught the class: mutate the validated thing and demand the
battery notice; assert the battery's own non-degeneracy inside the instrument; commit first runs with
notes rather than replacing them. One further hazard worth carrying verbatim: rustup resolves the
toolchain from the working directory, so a probe run in a scratch directory silently uses the wrong
toolchain, and nothing in the transcript says so (`89` section 0).

---

## 3. What the unit refuted, and what killed each

The band-transfer mechanism as a verdict carrier, killed by construction in both fragments: threshold
families place the flip exactly at the shipped width where no band can look (`84` F2,
`84_probes/p2b_no_band_can_catch_it.rs`; `86` F5, `86_probes/p4_sat_defeat_gate.rs`). Its two shipped
instances survive because something else carried their verdicts: a group-structure argument for wrap,
`82`'s min/max identity for the sign-uniform law, both now recomputable by the R6 procedures. The
band's one licensed job is validating a checker's implementation at rung 0 (`84` section 4, `89` F5).

The register sentence "at a shipped width the compiler produces only NEGATIVE verdicts"
(`OPTIONS.md:1888-1889`), killed as a universal by being procedure-relative: positive verdicts at width
64 cost a degree box inside a fragment (`84` section 5), and even the sweep produces one at width 9
inside a sign-uniform region (`82` F16).

Value-gated arms, killed twice: measured worse than either static arm (`80` section 5.1), then closed
on principle by I15 (`85:20-25`). Q39's option (b) is dead, and not by the panel.

The natural in-range declaration as a licence, killed by measurement: sound-looking, wrong on half the
domain (`82` F4).

The saturation-radius shortcut for general signed laws, killed by a non-constant tail (`89` F11); and
any reading of the degree-box criterion at `F > 0`, killed by a two-point witness (`89` F10).

`86`'s piecewise breakpoint machinery, retired as unnecessary rather than wrong: the min-form lemma
makes the degree box the test set, the monotonicity induction `86` flagged as its own weakest point
(`86:449-452`) is not weakened but unneeded, and the verdicts agree at roughly one sixtieth the
evaluation steps (`89` section 2 against `86:276-278`).

The typestate-against-trajectory framing as the licensing axis, dissolved by op at `83`: the axis is
const-availability. And the three category-ruling questions the coordinator built during this unit,
each rejected by op as the anti-pattern I13 names: rank the four constructions (`85:49-59`), pick
typestate or values (`83`), decide whether consumers would write window declarations (`88`, where his
words are "Take the win where it applies, gate it out from where it does not"). Per `89` section 10,
`86`'s three-kinds row enumeration (O-J') is the same shape arriving from a member rather than the
coordinator, and this consolidation carries the kinds as **instances observed so far**, not as a closed
taxonomy: the durable sentence is that a verdict names its evidence and where it was computed.

The panel-wide hedge that the operating constraints were unratified ground, killed by op at `85`
section 3 (`85:78-94`): they are I14, in force, not to be questioned, and **nothing built on them needs
redoing**, because every member of this unit had correctly made its findings independent of the bans in
either direction (`79` section 9, `80` section 13, `82` section 14, `84` section 12). The hedge itself
appeared in `76`, `77`, `79`, `80` and `81` and is wrong in each as a matter of record.

---

## 4. The corrections chain, as a result

The unit corrected itself seven times, and each correction is a measured fact about how this kind of
reasoning fails, which is worth more to the canon writer than the corrected claims alone.

`79` corrected `76`'s rung arithmetic: self-agreement across sessions is one instance wearing two hats
(`79` section 7). `80` relocated `79`'s headline: P4 is a predicate on values, established by a probe
containing no const item, so `79:322-325`'s claim that all its predicates were compile-time claims was
wrong about its own best result, and the relocation, not the demotion, was the general finding
(`79:72-74`, `80` section 6, confirmed at source by the checkpoint). `82` corrected `80`'s frontier by
one bit, mechanism identified and confirmed with a third arm: the frontier reads the per-tuple encoding
(F17). `84` defeated the mechanism `80` had proposed and `82` had built three constructions on,
exactly at the sentence `80` had flagged as the piece it most wanted broken; `82`'s constructions
survived because their soundness never actually lived in the band (`84` section 6). `86` corrected
`84`'s multivariate predicate: established at the degree fed to the instrument, not the degree of any
law in it (`86` section 1). `89` showed `86`'s battery structurally unable to fail on the machinery it
validated, and retired that machinery (`89` section 1). And op corrected the coordinator twice in one
file: quoting verbatim and naming the intent inside the quotation are two acts, and a fork asking one
rule to govern a category is the rejected universal wearing new clothes (`88`, sections 2 and 4).

Three structural lessons sit in that chain. **Flags work**: the two defeats landed exactly where the
defeated files had marked their own least-certain items, so the least-certain sections are not
paperwork, they are the attack surface the next expert actually uses. **Survival must be re-attributed
when a mechanism dies**: `82`'s gates outlived the band because a different argument carried them, and
a consolidation that had recorded "checked on a model band, transfer as residue" would have carried a
false soundness story onto the design's only validation path. **The predicate discipline caught real
errors**, twice: `79` section 4 on the cold derivations' unstated dimensions, `86` section 1 on `84`'s
degree dimension, both exactly the class `every-finding-carries-its-predicate.md` exists to prevent.

---

## 5. Live options, written out so the compressor cannot drop them

Per `RULES.md:189-210`, the options no member resolved are the ones a compression structurally loses,
so this pass is separate from the results and lists each with its costs and discriminators. The
register's unit-four entries are at `OPTIONS.md:1870-1953`.

**Q38, where a law verdict's truth is established** (`OPTIONS.md:1880-1906`). Route (a), exhaustive in
the compiler, stands with its frontier restated per R4: reaches model widths, produces the licensing
verdict at shipped widths only inside restricted regions, and its cost lines are facts about a
procedure and an encoding. Route (b), offline declaration cited in the compiler, stands with
`68:145-148`'s hole in full. Route (c), the band cross-check, is **corrected**: its buy line is
band-local, its residue is unbounded in general (`84` section 3), and it is strictly dominated inside
any fragment with a test-set theorem. The route added mid-unit, `84`'s O-J (`84:362-370`) as extended by `86` and
`89`: a verdict row carries a witness, or a per-fragment complete test set evaluated at the gated
width, or a named structural argument, with band agreement licensed only as an implementation check on
a checker. Two fragments have test-set theorems; the named-argument class shrank by one law during the
unit and cannot be emptied while `79`'s P4 exists. What would distinguish the routes for a given law:
whether its operations sit inside a fragment with a theorem, decidable from the law's spelling. The
same architecture is the unit's candidate for Q25, how the law inventory is named
(`OPTIONS.md:1637-1641`), with `89`'s addendum that a law can move between row kinds when a theorem is
found, so the rows are provisional.

**Q39, whether an arm's predicate may read data** (`OPTIONS.md:1907-1932`). Resolved in substance by
op rather than by the panel, and the register entry should be read through three of his statements:
the licensed category is whatever is const-available (`83`); option (b) is out on principle, never any
runtime checks (`85:20-25`); and an arvo-owned ingest door is not arvo's to open, ingest is the
consumer's, arvo may ship casting and conversion helpers and may not use them on the consumer's behalf
(`88` section 3). What survives of option (c) is the consumer-side constructor shape
(`82_probes/p3d_structural_permission.rs`'s window constructor), available exactly where closure holds
(R8), which is a per-region fact rather than a policy. This reading is the consolidator's synthesis of
the three quotes and is open to attack on that basis.

**Q40, which route a verdict takes to its closed form** (`OPTIONS.md:1933-1953`). Route (a), lifting
through a proof, subsumed for ring laws by the box procedure at any affordable arity (`86` F3 prices
the box); route (b), structural argument, shrank by the sign-uniform law and remains load-bearing for
signed, multivariate and mixed laws; route (c), stays swept, is the schedule-conditional class, outside
every fragment with a theorem, and is the class op's I7 is stated over.

**O-I, where a fold's operand window comes from** (`82` section 14, in full there). Four sources:
structural shape (unspellable straddling bounds, refuses at type check), a const outside the typestate
(refuses at codegen, author-asserted), checked once at a consumer-side boundary (available only where
closure holds), derived from the container's own sign domain (free, the unsigned case). Nobody attacked
this entry; it is the unit's most concrete unresolved design-shaped option and the one most at risk of
silent loss.

**The defect-and-benefit pair for I5-licensed refusals** (`77`, adopted and sharpened by `76` phase
two). A law Hot gives up is not an absence but a paired claim, fails by at most d, buys at least g,
and whether the pair lives in the type as a queryable fact or only in canon prose and bench evidence
is genuinely open. No member after `77` touched it. It connects directly to the unpriced state of the
whole unit: the numbers such a pair would carry do not exist yet.

**Cross-strategy resolution laws** (`77`'s probe 3, described at `77:129-140`:
`77_probes/probe3_strategy_resolve_lattice.rs`, `77_probes/probe3_output.txt`, with the negative
control at `77_probes/probe3_negative_check.rs` and `77_probes/probe3_negative_output.txt`). Commutativity, associativity and idempotence of the resolve operator over
a two-axis product order, checked at const time, the first mechanically-checked instance on a question
the register flags as open from the number-systems side. Untouched since. It lands in the strategy-axis
unit's lap.

**The length-aware predicate** (`82` section 10, F13). Licenses strictly more than the closure
predicate, needs the length at const time, unbuilt. Decides how much a statically-sized fold buys over
a runtime-length one.

**The expensive general-signed route** (`89` section 7). Exact coefficient tracking with root
isolation into monotone intervals, const-implementable with fixed-size arrays, needs multi-limb
coefficients past degree 3. Named with its cost; nobody has built it; nothing yet says a design needs
it.

**The per-point-cost experiment** (`89` section 4). One instrument varying per-point cost at fixed box
size would settle whether the guard boundary is a point budget or a work budget, which the coincidence
between `86` F3 and `89` F6 raises and does not answer.

**`76`'s tier taxonomy with `79`'s two additions.** Entailments, convenience facts, explicitly refused
laws, plus the two failure modes `79` section 8 insisted be named separately because their fixes
differ: a declaration validated against nothing (`68:136-151`) and a declaration validated against a
self-collapsed ambient domain (`74:543-550`, resting on `73:119-183`). Carried as a member proposal
about vocabulary, not as a closed classification, per section 3's enumeration discipline.

---

## 6. Where this topic touches the others

**The strategy axis, the shared placeholder.** This unit's predicate dimension list terminates in
"whatever named axis a strategy resolves to" (`79:200-219`), the same placeholder the format and
number-system topics stopped at, and `87` section 3 chose the strategy axis as the next unit partly on
this unit's own record. Everything here survives the set being reshaped because every finding is
predicated on overflow policy as a law dimension rather than on a marker name (R12). What this unit
hands the strategy unit: the resolve-operator laws (`77` probe 3), I13's arms as the mechanism a
strategy's weighting would select among, and the observation that `(operation, strategy)` was already
too coarse before the axis was even settled (R1).

**The format-concept topic.** This unit instantiated and bounded that topic's law frame: C6's H1
became a const predicate over a declaration (`82` section 7 on `79:112-121`), C6's scope boundary was
confirmed by P4 (R2), the cube's representable-set coordinate was extended by the declared window
(R7), C9's schedule became the non-liftable chain class (R11; C9's text at `63:692-699`), and `68`'s declaration hole and
validation-through-maps line recur at the law layer as R3 and as the band's demotion. The
constant-embedding convention (`86` F4) is a new obligation on that topic's width-family concept:
a law family across widths owes the embedding convention a predicate dimension.

**The number-system topic.** R12 is its invariance carried into this unit, and the caching consequence
is new and unpriced. `74`'s N11 self-collapsed-domain attack is carried in the tier-taxonomy option
above.

**The fold and reduction entries, Q11 and Q12.** The staging boundary, capacity at stage zero and
length at stage one, explains from this unit's side why the capacity-keyed accumulator relation
compiles gate-free and the length-keyed one is refused (`80` section 7, `OPTIONS.md:1063-1065`,
`OPTIONS.md:1236-1238`), and `82` F6/F7 add a third door to Q12's signed-fold row: a declaration-gated
arm with no soundness trade (`OPTIONS.md:1097-1099`), plus the length axis of `82` section 10.

**Op's mid-unit files reach every later topic.** I15 (never a runtime check), I16 (no policing of law
shapes), I17 (Cold's intent survives the strategy set), the const-available axis (`83`), the
intent-versus-clause discipline and the no-universal correction (`88`) all bind whatever unit comes
next, and three of them were provoked by this unit's own coordinator building category-ruling forks.

---

## 7. What I would tell the canon writer that the members would not

**The unit's durable content is one intent sentence, one invariance, one prohibition and one
discipline.** The intent sentence is R6's: a law verdict at a shipped width is computable at that
width, on a test set determined by the law's degree, for fragments whose failure condition is preserved
along the coordinatewise order; where no such fragment applies, a verdict is a witness or a named
argument, and band agreement is never a verdict. The invariance is R12's: a law verdict reads neither
encoding nor container. The prohibition is I15 composed with R5: there is exactly one validation path,
it is compile-time, and a mechanism whose soundness story is false there has no catch, which is why
the band defeat is the unit's most consequential single result. The discipline is R13's: an instrument
earns trust by being shown able to fail, and this unit's four counterexamples are the concrete case
for writing that into how the canon says doability is established.

**Resist every enumeration this unit tempts you with.** The dimension list is a floor (R1). The
verdict-row kinds are instances so far (`89` section 10 against `86:421-423`, and op's own words at
`88:118-123`). The binding-time rungs are measurements, not a ranking (I16). The tier taxonomy is a
member's vocabulary proposal. Op rejected the closed-category shape three times in this unit's own
span; a canon sentence that enumerates will be the fourth.

**Watch the two silent conventions.** The constant-embedding convention and the width-indexing of a
law family are author's choices nothing checks, and `86` showed one of them manufacturing a threshold
family by itself. Any canon statement about laws across widths owes both a place.

**The frontier numbers must never travel bare.** Three levels of correction established that a
frontier is a fact about domain size, per-tuple encoding and procedure. If the canon carries any cost
statement about verdicts, it carries what the number was counted over, or it carries nothing.

**Two member files carry claims their own instruments could not support, and the corrections live in
later files.** `84`'s F4 multivariate predicate is corrected at `86` section 1; `86`'s F7 evidence
weight (`86:404-410`) is corrected at `89` section 1, with F7's verdicts independently re-established
by `89`'s own theorems. A canon writer reading members in isolation would inherit both originals; the panel's rule
that an original file stands unedited (`RULES.md:509-518`) makes this consolidation the only place
short of the members' full text where the pairs are joined.

**And the whole unit is unpriced.** Every performance-flavoured claim, including the vectorisation
results that motivate the entire arms mechanism, is an instruction-count spike. The first bench run on
the harness will be the first time any of this touches a how-much answer, and `87` section 3 places
that as a dispatch inside a unit, on the coordinator's call.

---

## 8. Genuinely open, split honestly

**Attacked and unresolved.** General signed saturating laws with both clamps reachable: two routes
closed with diagnostics, one expensive route named and unbuilt (`89` section 7). Verdicts at `F > 0`:
refuted for the degree-box criterion with a witness; whether another theorem exists for
clamp-and-shift terms was not attempted (`89` F10). The guard-boundary mechanism: raised by a measured
coincidence, one cheap experiment named, not run (`89` section 4, F6). `79`'s P4's non-degenerate
region: attacked from every side the unit had, does not lift (R8), is not const-available (`82` F5),
and under I15 cannot be value-gated, so it stands as a characterisation and as the strongest evidence
that the named-argument class cannot be emptied.

**Never attacked.** The length-aware predicate construction (`82` section 10). O-I's four window
sources (`82` section 14). The defect-and-benefit pair's location (`77`). Cross-strategy resolution
beyond `77`'s first instance. Theorem C at arity 4 and above, and per-variable degree above 6, by
measurement (`89` item 1 of section 12; the proof stands, the falsification search is bounded).
`86`'s gapped signed members above width 11, and the depth-3 signed catalogue (`86` items 3 and 4).
Mechanising the two remaining prose arguments, `84`'s k = 64 valuation stretch and `89`'s
non-positive-window negation argument (`84` section 11 item 1, `89` item 3). Whether any of this
reaches the numeral tower's real expressions: `mock/crates/` is empty, so the question has no object,
and the only new machinery a design needs is the degree extractor and the fragment checker, both
syntactic (`89` section 11).

**Owed to the whole panel rather than this unit.** The strategy axis. Pricing. The final canon
writing, which reads this file beside the eight members it compresses.

---

## 9. Coverage, and where this compression is weakest

**What was read.** All nine unit files end to end, including both phases of both cold derivations;
op's `83`, `85`, `87`, `88` at their sources; `INTENTS.md` and `RULES.md` in full; the register's
unit-four section (`OPTIONS.md:1870-1953`) and every register or droplist range a member's fit cited,
at the cited lines. Probe transcripts were read where a carried number depended on them; probe
**sources** were not independently re-executed by this file, and the reproduction chain of R13 is the
members' work, not mine.

**What was deliberately dropped, by class.** The members' internal narrative of hypothesis formation
(kept only where the correction is itself a result); most per-probe negative-control counts (kept
where the control is what makes a headline credible); `76`'s and `77`'s phase-two reconciliation
detail against `42` and the format-unit files, which the format consolidation already carries from its
own side; the full text of the members' F-numbered predicates, compressed here to their load-bearing
dimensions with the member cited as authority; and every wall-clock second except where an accept
against a refuse is the result. Anchors into members' own prose were kept wherever a claim they
support is carried, with one deliberate exception: members written before op's `83` and `85` were
folded into `INTENTS.md` cite that file at line numbers that no longer hold, and this file cites the
current ones instead of restoring stale anchors. The set difference against the members is reported
beside this file's commit, not inside it.

**Where I am least certain, as a floor for the independent check.** The Q39 synthesis in section 5 is
the one place this file joins three op statements into a reading none of them states alone; if the
check finds one place I have overreached, look there first. Section 6's cross-topic paragraphs
compress other units' concepts (C6, C9, N11, the cube) through this unit's citations of them rather
than from their sources, and inherit any error in those accounts. R7's split between what `89` proved
and what remains `82`'s measurement is delicate and I may have drawn the necessity/sufficiency line a
shade differently than either member would. The corrections chain in section 4 compresses seven
corrections into one narrative and could have flattened the order or the attribution somewhere. And
the option pass in section 5 is exactly where the two prior consolidations lost material; I wrote it
from a fresh sweep of the members rather than from the register, but the class of loss is structural
and a previous consolidator's own risk list was not where its worst defects were.

**Nothing here settles anything.** This file is input to the final canon writing, per `87`. The
independent entailment check that follows must be run by someone who wrote neither this file nor `74`,
must work from the members forward, and must diff the option sets and the anchor sets, not only the
claims.
