# 52. The fixpoint claim, second read

**Date:** 2026-08-09. **Persona:** Adam Chlipala. **Mode:** explore, do not settle (`00_brief.md`,
`04`, `28`). **Position in the unit:** eighth and last file on this topic, after `44` through `51`.
The next file consolidates. Nothing here settles anything, and this file exists to hand the
consolidation a checked verdict rather than a single author's unread claim.

**Status: COMPLETE.** Written to disk before the heaviest verification and extended in place, per
`RULES.md:328-329`.

My question is whether `50`'s refutation of `16:100-101` is correct. It is. Every computational
claim I checked reproduced exactly, the underlying logic is a known and correctly-applied shape from
nonmonotonic semantics, and the replacement principle `50` proposes is independently recovered by two
other authors (`47`, `49`) from two different routes. I found two small defects in `50`, neither
load-bearing: its headline sentence conflates two distinct kinds of underdetermination, and one of its
own counts is off by one. Both are named below with the arithmetic that finds them. The topic's
six-file convergence on carrier-and-stride survives. What does not survive is the belief that
`16:100-101` is what established it.

## 0. Gates

### 0.1 Canon gate

There is no ratified canon to defend or diverge from, and `mock/canon/` does not exist on this
branch. The fixed material is `01`, `04`, `28`, `INTENTS.md`, the workspace discipline, the
forbidden-feature list, and the acceptance criterion in `00_brief.md`. My question sits entirely
inside it: whether a stated criterion decides the cases the panel has been adjudicating against it.
Nothing below proposes a mechanism the forbidden-feature list excludes; I ran three of `50`'s and
`47`'s probes myself to check the refusal counts, and every refusal names `generic_const_exprs`
directly, never adopts it. **Gate: passes.**

### 0.2 Test gate

There is no suite; `mock/crates` is empty by construction. My evidence is `50`'s own committed
probes, rerun on this machine, plus three probes I compiled myself from `16_probes/` and `47_probes/`
to check a specific count, plus direct reads of `mock/benches/` sources `50` cites. Nothing new lives
in `52_probes/`: everything I needed to check already existed, committed, and I verified it rather
than writing a parallel instance. I applied the gate's own checks (tautology, sampled law, missing
fundamental) to `50`'s probes as I read them; none failed.

### 0.3 Independence

My dispatch names an inverted order for one specific act: read `16` in full and form my own reading
of `16:100-101` **before** opening `50`. I did. Section 1 is that reading, written after `16` and
before `48` or `50`. Where I then agree with `50`, that is an independent instance on the reading of
the sentence and a read on everything downstream of it, since `50`'s formalisation and probes were
built before I saw them. Where I attack, independence is not needed.

I did not run `git log` in this repository before writing section 1.

## 1. My own reading of `16:100-101`, formed before opening `50` or `48`

`16:100-101`:

> A component is an output of the derivation when the consumer did not write it, the machine needs
> it, and a downstream site that holds the other components cannot recover it.

Reading this in place, inside `16`, two things were visible without any formal apparatus.

**First, "the other components" is circular on its face.** It names the members of the very set the
sentence is defining. An ordinary engineer's test reads naturally this way and is usually harmless,
because the author has a candidate set already in mind and is checking membership one at a time
against it. But the harmlessness depends on the candidate set being fixed by something outside the
sentence, and the sentence supplies nothing that fixes it.

**Second, and this was visible directly, side by side, without reading ahead:** `16` itself does not
hold the candidate set fixed. Section 4 (`16:126-141`, `16:187-189`) runs the criterion against a site
that holds only the carrier and the extent, nothing else, and gets the injectivity failure that
establishes the second output. Section 10.1 (`16:572-577`) runs it again, against a site that holds
the numeral's declared width, and uses that to demote its own third candidate. I noticed the switch
before reading `48`'s account of it, because it is a five-hundred-line file and the two passages are
close enough in shape to compare directly: one excludes the declaration from what a site holds, the
other includes it, and the sentence never says which is meant. That is `48`'s Reading A and Reading B,
independently arrived at, though I had no name for it until later.

What I did **not** have, on a plain reading, is `50`'s formal apparatus: the observation that this is
a self-referential set-builder condition of the exact shape used in logic programming with default
negation (`p in O` unless `p` is derivable from `O` minus itself), that such conditions are studied
under stable-model and well-founded semantics precisely because they need not have a unique solution,
and that a monotone forward-chaining closure composed with a negation is antitone rather than merely
"not monotone" in some vague sense. I want that stated plainly because it changes how much weight `50`
carries: the underdetermination is not an artifact of a formalisation choice `50` made. It is visible
on a direct reading of the sentence, and `50` supplied the standard theory for why a sentence of this
shape is untrustworthy as a definition and how to make its behaviour precise instead of asserted.

## 2. Verdict on `50`'s central claim: correct, checked at source

I reran every probe in `50_probes/` rather than trusting its printed output.

```
$ cd 50_probes && ./verify.sh
=== p1 ... reproduces committed .out
=== p2 ... reproduces committed .out
=== p3 ... reproduces committed .out
=== p4 ... reproduces committed .out
=== p5 ... reproduces committed .out
=== p5b ... refused, as expected. generic_const_exprs: 3, SameType<u32> control: 2, reproduces committed .err
=== p6 ... reproduces committed .out
=== p7 ... reproduces committed .out
feature gates: 0 in every file
```

All eight reproduce, byte for byte, on this machine, on the pinned toolchain, with zero feature
gates. That is the strongest form of verification a probe can offer and `50` cleared it in full.

### 2.1 The self-reference is real, and the operator's behaviour is the standard one for its shape

`50`'s formalisation, `p1_criterion_fixpoints.py:1-17`, writes the criterion as

```
O  =  { f in NEEDED \ DECL  :  not derivable(f, (O \ {f}) union HELD union PRIM) }
```

I read the `closure()` implementation directly (`p1_criterion_fixpoints.py:66-76`): it is an ordinary
monotone forward-chaining fixpoint over Horn-shaped implications, the textbook Datalog chase. That
half is unremarkable and correct. What sits on top of it is a **negation**, testing "not derivable",
applied per-candidate against the rest of the same set being computed. That composition, a monotone
closure wrapped in a negation and applied circularly to the set it is defining, is exactly the shape
studied as normal logic programs with default negation, and the textbook fact about that shape is that
it need not have a unique stable model: it can have zero, one, or several, depending on the rules.
`50` calls this "non-monotone", which is loose (the precise word is antitone, since negating a
monotone map reverses its order); the looseness costs nothing, because the demonstrated consequence,
that the answer is not forced by the sentence alone, is the same either way.

**One correction to `50`'s own framing, which I want on the record before endorsing the rest.** Section
2.6 reads "there is one sentence with several solutions." I traced the `solve()` function
(`p1_criterion_fixpoints.py:94-117`) by hand against four of the sixteen cells and confirmed it returns
**at most one** self-consistent output set per fully-specified cell in this rule set; the printed table
has exactly one row per (reading, stratset, kind) combination, never two rows sharing a label. So the
seven-answers-across-sixteen-cells result is not evidence of the equation having several simultaneous
solutions for one fixed parameterisation. It is evidence of something narrower and, I think, more
useful to the consolidation: **the sentence leaves three background parameters unstated, and each way
of filling them in deterministically produces a different unique answer.** `50`'s own separate
non-monotonicity demonstration (the `{CARRIER}` versus `{CARRIER, STRIDE}` comparison,
`p1_criterion_fixpoints.py:156-167`) is real and matters for a different reason: it is why nobody
should trust a naive least-fixpoint iteration to find the answer even for a single fixed cell, which is
a warning about method rather than a second source of multiplicity. Both things are true. They are not
the same thing, and `50`'s section 2.6 states them as though they were.

This does not weaken `50`'s conclusion. If anything it sharpens it: a criterion that is silent on three
background facts is a *worse* defect for a canon to carry than a criterion whose fixpoint is merely
hard to compute, because the first means the sentence is not doing the work at all and the second means
it needs a smarter solver. `50`'s own section 3 replacement (locate the parameters, state them) is the
correct response to the defect it actually found, whichever of the two framings names it.

### 2.2 The panel's converged cell rests on two assumptions the panel elsewhere rejects

I checked `50`'s central table entry by hand against the rule provenance, since this is the claim the
consolidation would inherit if wrong. `{CARRIER, STRIDE}` is produced by the cell `A/S4/blind`
(`p1_criterion_fixpoints.out:5`). Tracing the rules active in that cell
(`p1_criterion_fixpoints.py:79-87`): `S4` supplies `RULE_S4_ONLY`, which lets a site conclude the
declared width equals the stride whenever it sees a stride below the carrier's width, a rule that is
only sound when `Cold`'s stride-equals-width packing is the *only* packing discipline in existence.
`blind` leaves every consequent, including the carrier as a type, reachable from bare consts. Both are
concrete, checkable design commitments, and both are contradicted elsewhere in this panel by material
I read directly rather than through `50`'s account of it:

- `INTENTS.md` I1, quoted in full at `INTENTS.md:44-54`, is op's own word that the strategy set is not
  closed at four, which is precisely what `S4` assumes to make the `RULE_S4_ONLY` rule sound.
- The kind boundary is not a modelling choice; I compiled it myself in section 3 below, and it refuses
  in exactly the shape `blind` assumes away.

So `50`'s finding here is not "the criterion is ambiguous in the abstract." It is "the specific cell
that reproduces the unit's answer is the cell built from a closed strategy set and an unbounded
const-to-type rule, and this panel has independently established both of those false." That is a
sharper and more damaging claim than mere underdetermination, and I confirmed it by tracing the code
rather than accepting the prose summary.

## 3. I recompiled the kind-asymmetry refusals myself, and `50`'s count is off by one

`50:200` and `p1_criterion_fixpoints.py:31-32` both say "twelve compiled refusals" for the rule that a
type cannot be produced from a const, citing `16_probes/p5b` (4), `47_probes/p2` (6) and
`47_probes/p3` (3). I compiled all three myself rather than trusting the sum:

```
$ rustc +nightly-2026-05-28 --edition 2021 --crate-type lib 16_probes/p5b_const_to_type.rs \
    2>&1 | grep -c 'generic parameters may not be used'
4
$ rustc +nightly-2026-05-28 --edition 2021 --crate-type lib 47_probes/p2_scalar_single_output_refused.rs \
    2>&1 | grep -c 'generic parameters may not be used'
6
$ rustc +nightly-2026-05-28 --edition 2021 --crate-type lib 47_probes/p3_access_type_from_const_refused.rs \
    2>&1 | grep -c 'generic parameters may not be used'
3
```

4 + 6 + 3 = 13, not 12. `50`'s own probe header states the three counts correctly
(`p1_criterion_fixpoints.py:31-32`: "4 refusals", "6", "3") and never sums them in the file; the
"twelve" is arithmetic done in `50`'s prose, at `50:177`, `50:200` and `50:390`, and it is wrong by
one. `RULES.md:124` asks that every count carry the command that produced it, and this is the one
count in `50` that did not, which is exactly where the slip is. It changes nothing about the
conclusion: thirteen compiled refusals across two authors and three starting points is if anything a
stronger instance count than twelve, and `47_probes/p2`'s six errors are real, independently compiled,
and correctly attributed to the same wall. I would put "thirteen" in front of the consolidation and
name this correction beside it, because it is exactly the kind of small, checkable slip this panel's
own rules ask every member to catch in a predecessor before building on it.

## 4. The number-theory underneath the access-width correction is standard and correctly applied

`50` section 4 corrects `16`'s access-width closed form on the grounds that the phase set a packed run
actually reaches is the multiples of `gcd(stride, 8)`, not all eight residues, unless the stride is
odd. This is not a novel construction; it is the statement that the subgroup of `Z/8Z` generated by
`stride mod 8` has order `8 / gcd(stride, 8)` and consists exactly of the multiples of
`gcd(stride, 8)`, a standard fact about cyclic groups. `50_probes/p4`'s `phases_for()` function
computes exactly that set, and I traced the code against the group-theory statement rather than
against `50`'s prose gloss. It matches. The brute-force check against every phase for widths one to
1024 (`p4_access_width_is_keyed_on_the_stride.py:65-68`), which I reran and which reproduced, confirms
`16`'s closed form is the correct *worst case*, which is the same thing `16` claimed; what `50` adds is
that the worst case is not what most packing disciplines reach, and I confirm the 16-of-64 and 28-of-64
figures both appear, correctly labelled, in the rerun output.

## 5. The `Precise`-fork argument is a clean, checkable inequality, and I checked it algebraically

`50` section 6.1 claims per-step refusal and end-of-chain refusal admit exactly the same chains once
zero operands are excluded, whenever the total width equals the fraction width. I worked the algebra
independently, not from `50`'s proof sketch: write `T_i` for the cumulative 2-adic valuation through
step `i`. A nonzero raw value below `2^F` has valuation at most `F - 1`. Suppose per-step refusal fails
at some step `i <= k`, so `T_i <= i F - 1`. Then `T_k <= T_i + (k - i)(F - 1) <= i F - 1 + (k - i)(F - 1)
= k F - 1 - (k - i) < k F`, so end-of-chain refusal also fails. That is a direct contrapositive and it
holds exactly under the stated hypothesis (`W == F`), which `50` states as the probe's domain rather
than as a general result. The bound `50` reports, `0.000018%` of three-multiply chains admitted at
`F = 8`, comes from `50_probes/p6`, which `verify.sh` reproduced in section 2 above with **42 cells
checked, 0 differing** between two independently coded instruments. The proof and the enumeration
agree, and I did not need `50`'s narrative to see why: the inequality is short enough to redo from
scratch in one paragraph, which is the property a canon-bound argument wants.

## 6. Three authors converged on the same replacement principle from three different routes

This is the part of my verdict that I would put in front of the consolidation with the most weight,
because it is not `50` alone.

`50` section 3.2 proposes: *a fact belongs in the derivation's result when obtaining it requires
applying a rule the strategy owns.* Two other members reached the same idea, independently, before
reading `50`, in different vocabulary and by different methods:

- **`49`, cold**, having read only `INTENTS.md`, `00_brief.md` and the workspace rules, before `16`'s
  criterion existed in its context at all: "a quantity earns a place... when getting its value
  requires consulting the strategy as an actual decision (not a formula), and when an entity other
  than the numeral itself needs the answer and cannot safely re-derive it without risking disagreement
  with what the numeral's own definition intended" (`49:97-100`). `50` cites this at `50:366-369` and I
  checked it against `49` directly: the "cannot safely re-derive it without risking disagreement" clause
  is a single-source-of-truth argument, not an information-recoverability one, which is the same shift
  `50` and `47` both make away from `16`'s original criterion.
- **`47`**, from the kind boundary rather than from ownership: "the derivation's result must make
  available, as types, every fact a lowering site cannot recompute from a const" (`47:377-379`). This is
  narrower than `50`'s and `49`'s (it fixes the *form*, not the *fact set*, and `50` section 3.3 says so
  explicitly: it is one clause of three, not the whole criterion), but it is reaching for the identical
  target from the kind side rather than the ownership side.

Read as a working engineer rather than as a logician, all three are the same discipline stated three
ways: a fact that a strategy computes must be exposed by that strategy's implementation and consumed by
reference, never re-derived by a downstream site guessing at the formula. That is exactly the argument
against duplicated proof scripts and duplicated tactics that this persona's own practice is built on,
applied here to a design specification instead of a proof: a fact re-derived at every use site is a
fact that can drift from the derivation that was supposed to own it, and `50_probes/p3` (section 8,
below) is the compiled demonstration of that drift actually happening the moment a fifth strategy
exists. Three authors landing on the identical shape by three routes, one of them cold, is close to the
strongest form of corroboration this panel's own rules recognise, and it is stronger evidence for the
replacement principle than any single probe in `50` is for the fixpoint diagnosis.

## 7. What a criterion would have to say to pin one answer

Combining `50`'s own three-clause split (section 3) with `49`'s and `47`'s independent arrivals, and
stated as a specification rather than as a narrative:

**A premise, stated rather than inferred.** A lowering site holds the full numeral type: the
declaration (width, sign, strategy) and the language's primitives, always. This is not a finding to
argue for; it is a design commitment already made everywhere in the panel's own probes (`45:314-333`,
`46:244-251`) and stating it removes `16`'s Reading A/B ambiguity by fiat rather than by adjudication.

**An ownership predicate**, decidable per fact against the specification rather than against an
as-yet-unknown output set: a fact belongs in the derivation's result exactly when producing it
requires applying a rule that some registered strategy owns, and does not belong when it is a pure
function of facts every site already has (the language's own primitives on a type, or the literal
consts the consumer wrote). This is what breaks the self-reference: the predicate is checked against
the *ruleset*, a fixed and known thing at design time, not against the candidate output set itself.

**A kind clause**, fixing the form rather than the fact set: whatever the ownership predicate selects
is exposed as a type when a downstream generic body would otherwise need to reach a type from a const
(which `generic_const_exprs` forbids and thirteen compiled refusals in this panel confirm), and as a
const or const-fn otherwise.

**The count is then a consequence, not a clause.** As many facts as there are rules a strategy owns,
in the form each needs, and the number moves when the strategy set moves, which `INTENTS.md` I1 already
says it will. A canon sentence built this way answers `RULES.md:79-83`'s permanence test (it survives a
fifth strategy without editing) and its equivalence test (three independent implementations of "expose
what the strategy owns, in the form each site needs" converge on the same behaviour, which is not true
of "the derivation has two outputs").

## 8. Checking the boundary `50` itself flagged as unresolved

`50` section 12 names, honestly, that it could not determine whether the ownership clause is decidable
in general, and points at `50_probes/p3` arm four (a strategy introducing a genuinely new per-element
fact rather than a new answer to an existing question) as where it expects the strain to show. I opened
`p3` rather than taking the flag on faith.

`p3_site_recomputes_the_stride.rs` arms two through four are a clean escalation: arm two adds a
grid-packing strategy and shows a site that re-implements the stride formula gets a silently wrong
answer at width thirteen while agreeing by coincidence at width twelve, which `50` correctly calls the
shape that makes a small sampled test report green over a broken rule. Arm three repairs it by moving
the fact onto the strategy marker (`const GRID_BITS`), which is the ownership clause working as
intended. Arm four then adds a strategy that pads for an inline validity flag, and the repaired formula
is wrong again, because the new strategy is not answering the stride question differently, it is
introducing a question the schema never had a slot for.

I agree with `50` that this is genuinely open rather than decided by anything in the unit. It is also,
read against section 6 above, exactly the boundary a trait-based schema is supposed to make cheap to
extend: a new strategy that answers an existing question adds an impl; a new strategy that asks a new
question needs a new associated item on the trait, which is a schema change rather than a value change,
and nothing in this unit has needed one yet. Whether that boundary stays cheap as the strategy set
actually grows is not answerable from six declared strategies (four ratified-against, plus `50`'s and
`49`'s hypothetical fifth and sixth); it is a question for whenever `INTENTS.md` I1 is actually
exercised by a real strategy proposal, not for this topic.

## 9. Bearing on the live options

Per `RULES.md:264-266`. I cite `OPTIONS.md` by section and quoted phrase, never by line, per my brief.

**The derivation's outputs section.** *Confirmed as `50` filed it: kills the criterion's status as the
thing that decided the topic, keeps the finding, and the finding is now independently triangulated by
`49`'s cold derivation and `47`'s kind-boundary argument on top of `50`'s own.* I would add: the
section should carry all three replacement statements (`50`'s, `47`'s, `49`'s), named as the same
principle in three vocabularies, rather than adopting one and citing the others as support, because a
reader checking the equivalence test in `RULES.md:79-83` benefits from seeing three independent
implementations of the idea agree.

**`50`'s own diagnosis of itself (section 2.6).** *Fits badly as written, fits well once split.* The
sentence "there is one sentence with several solutions" should read as two sentences: the equation has
at most one solution per fixed parameterisation in the rule set checked, and the sentence leaves three
parameters unfixed, seven answers across sixteen fixings of them. Both are true; only the second is
what the probe demonstrates.

**The count of compiled refusals for the kind boundary.** *A number, not an option, and it should read
thirteen.* Checked at source in section 3.

**Everything else `50` touched** (the `Precise` fork, the access-width phase correction, the two-ladder
reversal, the bench readout): I checked each independently in sections 4, 5, 8 and found no defect
beyond the ones stated above. They stand.

## 10. What the consolidation should build on

Not `16:100-101`. It should state plainly that the sentence was tried, formalised, found to admit its
answer only under two assumptions the panel elsewhere rejects, and retired, with `50_probes/p1` as the
citation for why. In its place, the three-clause criterion of section 7, with `50`, `47` and `49` all
cited as independent arrivals at the ownership clause specifically, since that convergence is the
strongest single piece of evidence this topic has produced and it should not be flattened into one
author's finding.

The two-output answer itself does not move. Eight files, four different methods (injectivity, kind
boundary, cold schema derivation, ownership-predicate formalisation), and a fifth that measured the
codegen consequence of getting the packaging wrong (`51`), all land on carrier and stride as the
minimum, with a contingent third (compute carrier) open exactly where `INTENTS.md` leaves `Precise`
open. That is as settled as an "explore, do not settle" topic is meant to get, and it survived losing
the one argument it had been resting its authority on.

## 11. What I could not determine

**Whether the ownership predicate is decidable in every case a future strategy might introduce.**
Section 8 traces the one boundary case built in this unit and confirms `50`'s own doubt rather than
resolving it.

**Whether `50`'s rule set (`RULES_BASE` in `p1_criterion_fixpoints.py`) is the complete set a fully
faithful formalisation would use.** I checked its provenance citations against the primary sources I
had already read (`16` in full) and found them accurate. I did not independently re-derive whether any
rule is missing; `50` names this same limitation about itself at `50:785-789` and I have not extended
that check.

**Whether the "twelve versus thirteen" correction changes anything a reader would act on differently.**
I do not think it does, and I did not find a place in the unit where the exact count, rather than "more
than one instance from more than one author", carries weight.

**Whether a fourth or fifth independent route to the ownership clause exists that I have not read.**
I did not read `02` through `43`, so if an earlier file in the closed predecessor's carried material
states the same principle, I would not know.

## 12. Coverage, bounded honestly

**Read end to end, directly:** `INTENTS.md`, `00_brief.md`, `RULES.md`, `16` in full (before opening
`50` or `48`, per section 0.3), `48` in full, `50` in full, `47` in full, `49` in full including its
phase two, `51` in full.

**Compiled myself, independently of the authors' reported output:** `50_probes/verify.sh` (all eight
probes), `16_probes/p5b_const_to_type.rs`, `47_probes/p2_scalar_single_output_refused.rs`,
`47_probes/p3_access_type_from_const_refused.rs`, each against the pinned toolchain, to check the
refusal count in section 3.

**Opened at source, not from a compression:** `p1_criterion_fixpoints.py` in full,
`p4_access_width_is_keyed_on_the_stride.py` in full, `p3_site_recomputes_the_stride.rs` in full,
`mock/benches/variants/bitpack-plan-windowed/src/lib.rs`,
`mock/benches/variants/bitpack-plan-naive/src/lib.rs`, and `p7_bench_readout.py` in full, all cross-
checked against `50`'s claims about them.

**Grepped rather than read:** `OPTIONS.md`'s "The derivation's outputs" section (lines 703-976), to
confirm what the register currently carries before naming what it should gain. `DROPLIST.md`'s section
headings, to confirm `50`'s own check of it.

**Not read:** `02` through `15`, `17` through `43`, `44`, `45`, `46`, `PERSONA_CALLS.md`, `SETTLED.md`,
`archive/`, `seed/` beyond what `16` and `INTENTS.md` cite directly, the closed predecessor panel.
Where my sections rely on what `44`, `45` or `46` established, I take `48`'s and `47`'s accounts of
them, which I read in full, as the source, and I have not independently reopened the original files.

**The specific risk in what remains.** My verdict rests most heavily on section 2.2's trace of the
`A/S4/blind` cell against `INTENTS.md` I1 and the kind-boundary refusals I compiled myself, both of
which I checked directly against primary sources rather than against any file's account of them. The
weaker link is section 6's claim that `49`'s and `47`'s formulations are the same principle as `50`'s:
that is a reading of three sentences in three vocabularies, and a reader who thinks the three are
genuinely different ideas rather than one idea stated three ways should treat section 6, and the
"consolidation should carry all three" recommendation in section 9, as the part most likely to move.

**No bench harness ran in this dispatch.** Section 5's algebra and section 4's group theory are proofs,
not measurements, and I did not add a magnitude anywhere. Every timing question this file touches
remains **unpriced**, exactly as `50` and `51` both state about their own work.
