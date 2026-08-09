# 79. Attacking the two cold derivations on derived algebraic laws

**Author lens:** Fallin. Instruction selection, verified lowering, the trust boundary between what a
typestate proves and what a backend can act on.
**Position:** first attacker in the derived-algebraic-laws unit, after the two cold derivations (`76`,
`77`). I13 was ratified in the sitting between the panel's prior topic and this one, so this is also the
unit's first pass under the ratified predicate discipline.

## 0. Gates

**Canon gate: passes, situation two.** No canon exists. `mock/canon/` is absent, `mock/crates` is empty
by the declared mutation order, and this panel is writing the first canon. Only I13 is ratified, and
narrowly, on op's own word: "the entry means no more than he said." Eleven of the twelve older entries in
`INTENTS.md` are STATED, meaning direction rather than settlement. Nothing below settles anything that
I13 or op's other statements have not settled.

**Test gate: no suite exists to run.** The substitute is the probe discipline. I opened and re-ran every
probe I cite directly (section 1), and I built one new probe, `79_probes/p1_compositional_predicate_search.rs`,
committed with its output before this file, per the standing instruction.

## 1. Verification before argument

Per the panel's rule that a probe is cited for what it proved and presumed flawed until checked, I opened
and re-ran `76`'s and `77`'s probes rather than trusting either file's account of its own instruments.

`76_probes/probe1b_associativity_refined.rs` reproduces exactly: unsigned saturating `(a+b)-c == a+(b-c)`
fails 13,882,880 of 16,777,216 triples, 82.7484%, and unsigned saturating `+` alone is associative
universally (`76_probes/probe1_output.txt`, `76_probes/probe1b_output.txt`, re-run here,
`79_probes/p1_output.txt` section 1 matches to the digit). `77_probes/probe2_works_validate_erase.rs`
and `probe2_bad_case_refused.rs` compile clean and refuse `I = 0` as claimed, with the same shape as
`76`'s own probe 2. `77_probes/probe3_strategy_resolve_lattice.rs` and its negative control both run as
described. I did not re-run `77_probes/probe1_chain_error.py`; I checked its committed source against
its own quoted table in section 6 below instead of re-executing it, and note that as a coverage gap
rather than a verification.

I also opened `42_willsey_the_law_layer.md` directly, in full, rather than through either cold
derivation's citation of it. Both `76` and `77` paraphrase `42` accurately at every point I checked
against the source (sections 4.3, 5.2, 6.1, 6.4 specifically), which is worth stating because this panel
has caught citations resolving to the wrong text more than once, and here the citing files hold up.

## 2. A new probe: the composed law has its own predicate, and it is not derivable from the parts'

Both cold derivations argue, from different formal traditions, that a law belongs to a set of operations
composed under a strategy rather than to a single operation (`76`'s congruence closed under the
strategy's exposed operations; `77`'s judgment over a chain of arbitrary length). Neither builds an
instance where a composed law's own predicate is actually found and checked against a predicate a reader
might guess from the parts. I built one, because I13's own directive ("we collect and compound answers to
specific regions where a predicate holds") asks for exactly this shape of result and neither file
supplies a worked instance of it.

`76`'s probe1b already establishes the two facts that make the question sharp: saturating `+` alone
associates on every triple of `u8` (predicate: `any`), and the composed expression built from `+` and `-`,
`(a+b)-c == a+(b-c)`, fails on 82.7484% of the domain. So the per-operation predicate for `+` is the
widest one available and the composed expression's predicate is nearly the opposite of wide. If a
consumer (or a downstream rewrite pass) inferred the composed law's region from the fact that addition
itself is unconditionally associative, it would be wrong on five sixths of the domain.

**`79_probes/p1_compositional_predicate_search.rs` hunts for the composed law's actual predicate**, in
the same style `57b`'s H1/H2 evaluation used for signed saturating multiplicative associativity: propose
a candidate, check it for sufficiency and necessity violations over the whole domain, and report the
first candidate with zero of both. Four natural candidates, each informed by the panel's own
"clamp-then-pullback" mechanism from `62`/`57b`, each miss a direction:

| candidate | sufficiency violations | necessity violations |
|---|---|---|
| P0, no elementary clamp on either op | 0 | 65,280 |
| P1, exact unbounded result lands in `[0,255]` | 8,355,840 | 65,280 |
| P2, no ceiling-clamp-then-pullback on the add | 5,559,680 | 0 |
| P3, P2 plus no underflow on the subtract | 0 | 32,640 |
| P4, full four-way case split | **0** | **0** |

P4 is the four-way split on whether `a+b` clamps at the ceiling and whether `b-c` clamps at the floor:
neither clamps, always equal; ceiling clamps only, equal exactly when `c == 0`; floor clamps only, equal
exactly when `a == 0`; both clamp, never equal. This matches the measured holding set exactly, over the
full 16,777,216-triple domain, in both directions. A mutant of P4's floor-only arm (`a == 0` weakened to
`a <= 1`, a plausible-looking off-by-one) reintroduces 32,640 sufficiency violations
(`79_probes/p1_negative_output.txt`), so the zero-residue result is not an artifact of a check that
cannot fail.

**What this establishes, stated at the bar I13 sets.** The composed expression's predicate is not the
conjunction of the parts' predicates (add's predicate is `any`; the composed predicate is a four-way case
split naming both clamp events and, in two of the four cases, a specific operand's value). It had to be
derived and checked against the composed expression directly. This is a direct, mechanically checked
answer to one of the questions the dispatch names as live: **the composition of two gated arms is not
itself gated by conjoining the arms' own predicates; the composed arm needs its own predicate,
independently established.** It also extends `42` section 5.2's proposed general pattern ("a law holds on
the bounded numeral exactly on the region of computations whose trajectory does not reach the boundary
that would falsify it") to a new case that pattern was stated from only two data points and never tested
against: a **mixed** composition of two different operations, rather than a repeated fold of one. P4's
own shape, a case split naming clamp-reachability of each operation separately and then a residual
operand condition in the mixed cases, is itself evidence the pattern generalizes past same-operation
folds, at the cost of the residual condition (`c == 0`, `a == 0`) that a single-operation instance of the
pattern never needed to state.

**Predicate, per the required notation:** `N = 8, S = saturate (unsigned), F = 0, threads any, features
any`. Threads and target features are claimed `any` on a semantic argument rather than a measurement:
`u8::saturating_add`/`saturating_sub` are pure value functions with platform-independent semantics
specified by the language, not codegen facts, so no target feature or thread count can change what they
compute. Width is **not** claimed beyond `N = 8`; nothing here was checked at any other width, and I13
forbids listing what was not checked.

## 3. Neither cold derivation engaged the unit that already built and measured a predicated-arm mechanism

I13 asks what a law layer looks like when its purpose is to name gateable regions. The format-concept
unit, two topics ago in this same panel, already built and measured one, and its own author marked it
provisional and unattacked. Neither `76` nor `77` names it, because cold derivations by design read
nothing before writing, and neither picked it up in reconciliation either: `76`'s phase two reaches `56`,
`57`, `61` through `OPTIONS.md`'s account and discusses absorption-versus-coherence, but stops short of
the frame those files' own author, `57b`, withdrew and replaced its shared theorem with. `77`'s phase two
does not reach it at all.

**C6, `63:659-673`.** *"The laws of a format's operations are derived, not enumerated per policy. The
induced operation is associative when the ambient operation is associative on the reachable set (H1) and
the reduction's kernel is a congruence for it (H2). The congruence half is decided by the range's geometry
per operation and is repairable by choosing the representable set. The ambient half fails wherever a
fixed-width rescale is part of the operation itself, for every policy, and nothing downstream repairs
it."* Evaluated mechanically over 24 cells (three sign domains, two policies, two operations, two
scales): zero sufficiency violations, zero cells associative without both hypotheses, zero residue
(`63:389-392`, citing `57b:247-297`). This is I13's own shape, arrived at independently, before I13 was
ratified: a nameable predicate, decomposed into named sub-facts (H1, H2), each separately checkable, that
together carve the exact region a law holds in.

**Its rung is stronger than "unattacked" makes it sound.** `74` section 3.4 reports a deliberate attack
on it from the number-system unit, built to face it from a direction it was not built for (the admission
test rather than the law-region question), and the frame held (`74:273-275`, "a deliberate attack on the
law frame the format unit left provisionally marked, from a direction it was not built to face, and the
frame held"). A failed attack is not a proof, and `63` says so explicitly (`63:665-673`, "the verdict
firms when the frame survives an independent attack"). But it is a stronger evidential position than
either cold derivation credits it, since neither cites it at all.

**What this changes about the dispatch's open question, "is the region derivable from the format's own
data, or must it be measured per law."** C6 answers it directly, and the answer is neither pole: the
region is **derived from more primitive facts** (ambient associativity, kernel congruence), each of which
is itself measured, and the derivation composes them by conjunction. That is a genuine third answer, and
it is the one both cold derivations' own "derivation as generation" reading (`76`'s Reading two, `77`'s
Reading B) was reaching for without an instance to point at. I would adopt C6 as the sharpest available
model of what a predicated arm's internal structure looks like, and recommend the consolidation state it
alongside the entailment/generation distinction rather than let the two live in separate sections of the
canon.

**One caution C6 does not itself carry, which section 2's finding supplies.** C6 is stated for a single
reduction's induced operation, one law, one op. Section 2's P4 shows that even where H1 and H2 both hold
for a component operation (`+`'s H1 and H2 both hold unconditionally for unsigned saturating addition,
which is exactly why `76`'s probe found it associative universally), a *composition* of that operation
with another still needs its own H1/H2-style derivation, not an inheritance from the component's. C6's
own wording is scoped to "a reduction's induced operation," singular, so this is not a defect in C6; it
is a boundary of C6's stated scope that a canon adopting it should name rather than let a reader assume
away, because the assumption is exactly the one that P4 shows costs 82.7% of a domain when made silently.

## 4. Neither predicate is stated at I13's bar, and the panel already has a compiled instance of what that costs

I13 requires every finding to name its region as an explicit predicate over every dimension that could
move it, with absence read as the strongest negative claim rather than a hedge. Neither `76` nor `77`
writes any of its headline findings in this form. `76`'s "`wrapping_add` is associative universally" does
not state `N = 8` (its own domain, `u8`), does not state `threads`, does not state `features`. `77`'s
identity-representability claim similarly names `I, F` as the varying axes and leaves width, threads, and
features implicit. Under `every-finding-carries-its-predicate.md`'s own rule that an unstated dimension
means the finding is false everywhere that dimension is present, both files' findings are, read strictly,
claims that hold nowhere threads or hardware features exist at all. That reading is obviously not what
either author meant, and it is also exactly the trap the rule exists to name: **a predicate is not a
courtesy, it is load-bearing**, and both files were written before I13 existed to demand it, so the gap is
a timing fact rather than a defect in either author's reasoning. I flag it because the consolidation
should not carry either finding forward without restating it in the required form, and section 2 and
section 6 below do that restatement for the findings I am adding or endorsing.

**Why this is not pedantry, with a compiled instance already in the panel's own record.**
`DROPLIST.md:25-29`, cited independently by `42` section 4.3 and read here at the source: gating
`arvo-graph`/`arvo-comb`/`arvo-spectral` on a single `AddAssoc` flag by default was refused by
measurement, because it "admits the one preset (`Hot`, wrapping) whose recurrences return wrong answers
under these algorithms' own stated specifications, and refuses the two (`Warm`/`Cold`, saturating) that
compute correctly, because associativity and the distributivity these algorithms need are different,
complementary laws that invert across the same presets." This is what happens when a predicated arm's
predicate is under-dimensioned: a single flag, keyed on one law and silent about which law an algorithm
actually needs, admits the preset that is wrong and refuses the presets that are right, for **both**
algorithms at once, in opposite directions. It is a compiled counterexample to the idea that "the format
associates" is a sufficient predicate for gating an algorithm's admission; the predicate needs the
specific law an algorithm's own specification requires, named, not a generic soundness flag. `42` section
4.3 draws the same conclusion from the same evidence: "a consumer states its requirement per property,
not per a bundled notion of soundness." I support this reading independently, from my own reading of the
DROPLIST entry at the source rather than through `42`'s citation of it, and it directly answers a second
of the dispatch's open questions: **whether the composition of two gated arms is itself gated** has a
sibling failure mode, where a *single* gated arm built at the wrong grain silently serves two consumers
with opposite needs. The fix in both cases is the same: name the specific law and the specific consumer
requirement in the predicate, never a bundled or generic stand-in for it.

## 5. `(operation, strategy)` is necessary and not sufficient, on the panel's own measured cube

Both cold derivations converge on "a law belongs to a pair, operation and strategy's axis assignment,"
derived from I9. This is correct as far as it goes, and the format-concept unit's own cube (`63` section
4.3, nine rows over sign domain, operation, policy, fraction width, and range symmetry) shows it stops
one level too early to be usable as a predicate's actual dimension list. Reproduced here because it bears
directly on this unit's question:

| sign | op | policy | F = 0 | F > 0 |
|---|---|---|---|---|
| unsigned | mul | saturate | semiring half, by congruence | dead |
| signed | mul | saturate, 2c range | dead, 160 at w=4 | dead |
| signed | mul | saturate, symmetric range | associative, monoid | dead |

The middle and bottom rows share **sign, operation, and policy**, everything either cold derivation's
`(operation, strategy)` framing would resolve to a single answer for. They differ only in **range
symmetry**, which is a fact about the representable set (`Q` in `63`'s vocabulary), not about the
strategy. Under I1 (STATED, open), the strategy axis itself is not fixed at four named markers, so
"strategy" is not even a settled name for a predicate dimension yet; what is settled, by measurement, is
that at least sign, operation, overflow policy, fraction width, and representable-set symmetry all
independently move a law's verdict, and the last of these is not a strategy fact under any reading of
`63`'s own concept split (C2: representable set is identity, not realisation).

**This sharpens rather than contradicts I9.** I9 says the strategy is what makes an answer correct; it
does not say the strategy is the only thing that does, and `63`'s cube shows a representable-set choice
(symmetric versus two's-complement range) changes a law's verdict independently of anything a strategy
axis assignment states. A canon sentence built on `(operation, strategy)` alone, without naming the
representable set as a further coordinate a predicate must carry, would misdescribe the middle-versus-
bottom row distinction above as if it were unstateable, when it is exactly stateable and exactly measured.
I would push the consolidation to state the predicate's dimension list explicitly as **(operation, sign
domain, overflow policy, fraction width, representable-set shape, and whatever named axis a strategy
resolves to)**, rather than compress it to "strategy" the way both cold derivations do, because the
compression is what makes a table like `63`'s cube look like it needs a fifth undiscovered axis instead of
what it actually is, a representable-set fact the strategy framing has no slot for.

## 6. Chains: `77` goes further than `76`, and `63`'s schedule concept sharpens both, naming a predicate dimension neither uses

`76`'s own phase two credits `77`'s chain-error finding as going further than anything in `76`'s own
file, and I agree after reading `77_probes/probe1_chain_error.py`'s committed source directly: addition of
already-quantized fixed-point values needs no chain machinery because the sum stays exact, while
multiplication needs a widened accumulator because the exact product needs `2F` bits. Both cold
derivations, and the exchange between them, treat "does this operation need chain machinery" as a
property of the operation alone.

**`63`'s C9, read together with `60`'s derivation it rests on, names something neither cold derivation's
chain treatment states as a predicate dimension: the schedule.** *"A chain is a composition of exact
operations together with a schedule of adaptation points, and the schedule is part of the function's
meaning. The per-operation model extends to chains exactly when adaptation is unfused from the
operations"* (`63:692-699`, resting on `60`'s D-B derivation). This is not merely a restatement of "widen
the accumulator." It says the number and position of the points at which a chain's intermediate result is
rounded back into the format **is itself part of what the chain computes**, not an implementation detail
that a correctness argument can abstract over. Two chains computing "the same sequence of multiplies"
with different adaptation schedules (round after every step versus round once at the end) are, under this
reading, different functions, not two implementations of one function with different error bounds.

`63` section 4.5 measures exactly the consequence this has for a law's predicate: the multiplicative
fold's accumulator saving is not the constant `77` (and `58` before it) first reported, it is
**rounding-conditional**: 3 bits at fold length 3 and 4 under truncation, and 0 under round-to-nearest-even
at the same lengths, with the equality between the two rounding modes breaking at length 5 (`63:515-521`,
resting on `60_probes/p_d.out`). So even holding the operation (multiplication), the strategy, the sign
domain, the policy, and the fraction width all fixed, the **rounding rule** and the **adaptation
schedule** each independently move the answer to "how much does a chain-level law cost to hold." Neither
`76` nor `77` names either of these as a predicate dimension in its own vocabulary; `77`'s chain-error
probe fixes a rounding mode implicitly (its Python script's rounding behaviour is not named as a
parameter the way the accumulator width is), and its finding is correspondingly narrower than it reads,
in exactly the direction `63`'s later, more careful measurement caught.

**What I would add to the register on this point, offered as a candidate rather than a settlement.** A
chain-level law's predicate needs the schedule and the rounding rule as named, independent dimensions,
alongside operation, sign domain, policy, and fraction width. `76`'s own three-tier taxonomy (entailments,
convenience facts, explicitly refused laws) has no slot that distinguishes a schedule-independent law
(addition's chain exactness, which `77` showed holds for every schedule because there is nothing to
schedule) from a schedule-conditional one (multiplication's accumulator saving), and I would want that
distinction stated explicitly wherever the tier taxonomy is carried forward, because collapsing it is
exactly how `77`'s own reported constant, "the saving is exactly `F`," turned out to be one rounding
mode's answer wearing a universal's clothes.

## 7. A rung correction: self-agreement is not the corroboration `76` reads it as

`76`'s phase two states that finding the same conclusion in `42` (its own earlier, non-cold file, same
persona) and in phase one of this dispatch "is a form of corroboration this panel's rules do not have a
name for," and that "a persona that reliably re-derives its own settled positions from first principles
is itself a small piece of evidence that the position is not an accident of framing" (`76:282-297`). `76`
is careful to say this is not TWO EXPERTS, since it is one author, and that is the right call as far as it
goes. But the framing still claims evidentiary weight for the recurrence itself, and RULES.md's own
principle, "agreement among unratified artifacts is not corroboration, because agents copy each other's
framing," applies with more force to one persona's self-agreement than to two different personas': a
single author has every reason to reach the same conclusion twice, including reasons that have nothing to
do with the conclusion being right (a settled prior, a remembered probe shape, a habit of framing). The
honest tally across this unit, counting instances rather than files, is **two independent instances**
supporting "a law is a fact about the pair" (`76`'s cold phase one and `77`'s cold phase one, written
without either reading the other or `42`), plus `42` itself as a third, earlier, non-cold instance from
the same author as one of the two, which is corroborating evidence of a weaker kind than either of the
first two and should be weighted as such rather than folded into "reliably re-derives," a phrase that
reads as though the recurrence adds a fourth data point. It does not add a fourth; it restates one of the
two under a different session.

## 8. The tier taxonomy survives, sharpened by two findings neither cold derivation had

`76`'s three-tier split (entailments, convenience facts, explicitly refused laws) and `77`'s parallel
proof-versus-generation framing both hold up under the panel's own record, and both gain something from
material outside either file's cold phase one.

**On entailments versus convenience facts, `68`'s finding sharpens the boundary condition rather than
just restating it.** `68` section 2.2, opened directly: a mutant with a deliberately overstated declared
window compiles clean through an entire validation suite, because "the coverage check `covers(rep,
demand)` can only fail when a declaration understates; an overstated declaration passes vacuously... a
representation's declared properties are worth exactly nothing unless validation runs through the maps;
validation of declarations against declarations is paper checking paper" (`68:136-151`). This is the exact
failure a convenience fact is: a property that reads as checked because a declaration exists, without a
check that actually runs the maps. `76`'s tier already names the risk in words ("these are exactly the
trap: they read as laws"); `68` supplies the mechanical instance of how a declared-but-unvalidated
property survives a whole test suite while being false, which is the sharper, checkable version of the
same warning.

**On the risk that a predicate can be gamed rather than merely under-checked, `74`'s N11 attack is the
sharper cousin still.** A system exposing its own computed algebra as its ambient domain satisfies every
verdict, including a law inventory check, "while voiding every conclusion" (`74:543-550`, resting on
`73:119-183`'s compiled demonstration that a mutation set including a reduction adapting every value to
zero cannot make the collapsed verdict fail). This is a distinct failure mode from `68`'s: not an
unvalidated declaration, but a validated declaration against the wrong domain, chosen by the system itself
to make the check trivially pass. Neither cold derivation's tier taxonomy names this failure mode
separately from the convenience-fact tier, and I think it should be named separately, because the fix is
different: an unvalidated declaration is fixed by adding a check; a self-collapsed ambient domain is fixed
by requiring the ambient domain be named independently of the system being checked against it, which is a
requirement on the predicate's construction, not on whether a check runs at all.

## 9. Two questions still in flight, restated at the boundary this file actually reaches

Per the dispatch's instruction, I have not answered either. Stating where this file's own findings sit
relative to each.

**Which verb "validate" is.** Section 2's P4 result and section 3's C6 discussion are both stated as
compile-time facts, checked exhaustively at a model width, with no runtime component. `68` section 2.2's
own resolution for the representation case, that validation is two boundary-keyed acts rather than one
ambiguous verb, applies with the same force to a law: a law checked at derivation, compile-time, per type,
against the maps, is a different claim from a law re-checked at an ingest boundary, runtime, per datum,
and nothing in this unit's evidence, mine included, touches the second. Every predicate this file states
is a `Q-A`, compile-time reading claim, and none of it bears on whether an ingest-boundary check is also
wanted for a law the way `68` argues it is forced for a representation.

**Whether the long-standing constraints are op's intents.** Nothing in section 2's probe, or in any
citation above, depends on `no_std`, `alloc`, `dyn`, or `TypeId` either existing or not existing. The
probes are plain arithmetic over `u8`; the C6 and cube material cited from `63` and `74` is, by both
files' own accounting, independent of the ban list. This unit's evidence does not move under either
answer to Q-B.

## 10. Fits against the register

**Kills nothing.** Additions and fits:

- **Q11 and Q12** gain section 2's worked instance directly: a fold-layer accumulator question and a
  law-layer composition question turn out to be the same mechanism (a composed expression's predicate is
  independently derived, never inherited from its parts), demonstrated here at the mixed add/sub case
  where Q12's own table already carries the headline percentage.
- **Q25**, how the law inventory is named, gains C6's own shape as a candidate answer: marker contracts
  per named sub-fact (H1-style, H2-style), composed by conjunction into the predicate a specific law
  needs, rather than one flag per law and never a bundled soundness marker (section 4's DROPLIST evidence
  against the bundled form).
- **The unit's own tier taxonomy** (`76`'s three tiers, `77`'s proof/generation split) is carried forward
  with two sharpenings: the declarations-versus-maps distinction from `68`, and the self-collapsed-domain
  failure mode from `74`'s N11, named as a fourth risk distinct from an unvalidated convenience fact.
- **Candidate canon sentence, offered at the same confidence C6 itself carries (ONE EXPERT, provisional,
  attacked once and held):** *a predicated arm's region is derived by conjoining named, independently
  checkable sub-facts about the operation and the representable set, never asserted as a single flag per
  law or per strategy; a composed expression's region is one such derivation in its own right and is never
  inferred from the regions of the operations it composes.*

## 11. Findings, restated in the required predicate notation

- **P4 (section 2):** `N = 8, sign = unsigned, op = {saturating_add, saturating_sub composed as
  (a+b)-c vs a+(b-c)}, F = 0, threads any, features any`. Holds exactly on the four-way case split named
  in section 2. Does not extend to any other width, sign domain, or operation pair; none of those was
  checked.
- **The composed-predicate-is-not-inherited finding (section 2, section 3's caution on C6):** `N = 8,
  sign = unsigned, op pair = {+, -}, F = 0, threads any, features any`. Stated narrowly on purpose; I
  believe the underlying mechanism (`42`'s reachability pattern) is broader, and say so as belief rather
  than as an extension of the predicate, per I13's own instruction that a widening is a later expert's
  claim to make in its own file, never a retroactive edit to this one.
- **The DROPLIST associativity-gating instance (section 4):** carried forward from the prior panel's
  record; not re-measured here, cited as an existing compiled counterexample, at whatever predicate its
  own source states.

## 12. Coverage, bounded honestly

**Read end to end:** `INTENTS.md`, `RULES.md`, `76` (both phases), `77` (both phases), `OPTIONS.md` in
full, `DROPLIST.md` in full, `42_willsey_the_law_layer.md` in full, `68_leroy_what_the_pipeline_certifies.md`
in full, `63_spj_consolidation_the_format_concept.md` sections 0 through 7, `74_giesen_consolidation_the_number_system_concept.md`
sections 0 through 10. **Read at the source:** `76_probes/probe1_associativity_exhaustive.rs`,
`probe1b_associativity_refined.rs` and their outputs; `77_probes/probe2_works_validate_erase.rs`,
`probe2_bad_case_refused.rs`, `probe3_strategy_resolve_lattice.rs` and its negative control, and their
outputs; `77_probes/probe1_chain_error.py`'s source (not re-executed, see section 1).

**Not read:** `63` sections 8 through 13 and `74` sections 11 and 12 (both files' own coverage and
process-record sections, judged lowest value for this unit's question); `OPTIONS.md` Q18 through Q24,
Q33 through Q37, and any file numbered `01` through `41`, `43` through `61` except as reached through
`63`, `74`, `OPTIONS.md` or `DROPLIST.md`'s own citations, which I did not independently verify past the
specific line ranges quoted above. Every statement in this file about `18`, `35`, `40`, `55b`, `56`,
`57`, `57b`, `58`, `59`, `60`, `61`, `62`, `65`, `66`, `67`, `69`, `70` through `73`, `75` is sourced
through `63` or `74`'s own account and inherits their errors if any; I checked `42`'s citations of `35`
and `18` against `42`'s own text (which quotes them directly) but not against `35` or `18` themselves.

**Built:** `79_probes/p1_compositional_predicate_search.rs` (section 2, five candidates, exhaustive over
16,777,216 triples each) and `79_probes/p1_negative_control.rs` (the mutant check). Both committed with
their compile and run transcripts before this file.

**Not done, and what it leaves open:** no attack on C6 itself, beyond the scope-boundary note in section
3; no re-execution of `77`'s Python chain-error probe; no check of whether P4's four-way case-split shape
recurs at other widths or for other mixed operation pairs (subtract-then-multiply, for instance), which
would be the cheapest next instance for whoever attacks this file to build; no attempt to state the
"strategy resolves to" clause section 5 leaves as a placeholder, which is exactly the ambient-family
question `67:440-446` and `OPTIONS.md` Q21's amendment already flag as open from a different direction.

**Nothing here settles anything.** The mode is explore; this file goes to whoever attacks next in this
unit's cadence.
