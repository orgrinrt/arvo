# 243. Attacking 241: the resolution's failure arm is uninhabited and its success arm is not injective

Seat 242, attack phase, addressed to the seat that wrote
`241_kiselyov_admission_is_a_resolution_not_a_verdict.md`.

I refute the central thesis and I owe a replacement, so section 5 is the replacement in
enough detail to build from. Sections 6 and 7 are where 241 beat me and I say so plainly.

## 0. The dispatcher's claim, checked

**`Ambient` declares `RADIX: u32` and `SIGNED: bool` and no other item.** Verified on this
tree at `origin/dev`: the trait body carries exactly those two and nothing else. The
dispatcher is right and I have nothing to add to it. It is the one thing 241 and I reached
separately with different instruments, and it is as settled as anything in this sitting.

I also opened R2 rather than inheriting 241's quotation of it, because R2 carries the whole
thesis. **241 quotes it accurately.** Every word it attributes to
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule` is in the
row. My attack is on the reading, not the transcription, and I want that distinction on the
record before I start.

## 1. The thesis under attack, in 241's own words

> **Admission is a resolution.** Given a candidate, it returns either a total assignment of
> the ratified coordinate set, or the name of a coordinate the candidate failed to fix.

and:

> A **predicate** is that object composed with "did it succeed" [...] A **location** is that
> object composed with "which coordinate", defined only on the success branch.

So the claim is a sum type with two arms, and both of Q30's options are folds over it.
`242_probes/resolution_has_no_second_arm/` asks whether either arm is what 241 needs it to
be. Two arms, three stated failure cases, output committed.

## 2. The failure arm is uninhabited, and that is not a quibble

The second arm needs a candidate that resolved some coordinates and failed to fix one. At
the tier 241 names, that candidate **cannot be constructed**:

```
error[E0046]: not all trait items implemented, missing: `PHASE_DEN`
   --> src/main.rs:123:1
    |
123 | impl Format for Underdetermined {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `PHASE_DEN` in implementation
```

An `impl Format` that leaves a coordinate unfixed is not a candidate that fails admission.
It is not a candidate. It has no value, no type and no existence, so nothing can be handed
to a resolution and get its name back.

**A sum whose right arm is uninhabited is its left arm.** So at the candidate tier the
resolution is a total function from candidates to assignments, and "either ... or the name
of a coordinate" describes a shape the encoding cannot express. 241's own probe 3 gets
within one step of this and does not take it: it excludes `ADMITTED` from the ten on the
ground that "it carries no value, and it is what every use site forces rather than something
a candidate chooses", and calls that separation "the resolution shape section 2 derives,
sitting in the tree unnamed". It is not. `ADMITTED` is a **relation among coordinates
already fixed**, forced after resolution is total; it is not a coordinate that resolution
could fail to determine.

**What I am not claiming.** `Slots::ADMITTED` genuinely refuses, at codegen, as my round-one
probe measured. So admission *can* say no. What it cannot do is say no **by naming an unfixed
coordinate**, because unfixedness is a compile error one tier below the question. The refusal
carries a message about an inconsistency between three coordinates that were all fixed.

## 3. The success arm is not injective, so it is not a location either

Four ambient algebras over one grid, following 241's own probe 1, resolved through the ten:

```
  rationals (+,*)  : Resolved { radix: 2, signed: true, exp_at_0: -3, ... has_zero: true }
  tropical (min,+) : Resolved { radix: 2, signed: true, exp_at_0: -3, ... has_zero: true }
  boolean (and,or) : Resolved { radix: 2, signed: true, exp_at_0: -3, ... has_zero: true }
  interval algebra : Resolved { radix: 2, signed: true, exp_at_0: -3, ... has_zero: true }
  CONTROL radix 10 : Resolved { radix: 10, ... }
```

Case 1 holds, the four are identical. Case 2 holds, the control moves, so the instrument
sees a coordinate change and the identity above is not blindness.

**Every one of the four resolved. None returned the name of a missing coordinate.** The
operation family is not among the ten, so a resolution *over the ten* cannot report it
missing. That is the attack: **the diagnosis 241 cites to motivate the shape is not
reachable from the shape.** Section 4 of 241 uses R2's "the signature is missing a
coordinate" as a live diagnosis of exactly these four; measured, the object 241 builds to
carry that diagnosis returns success on all four and says nothing.

And 241 supplies the second instance itself, in its reconciliation: a Gray code "fixes none
of the ten", being a re-encoding of the same representable set over the same ambient, which
R1 ratifies as the same format. So the success arm collapses a Gray code onto its binary
sibling too, by a ratified sentence rather than by a measurement.

**Two collapses, two sources, one conclusion: the success arm is a quotient map, not a
location.** 241's own critique of Q30's location option, that it "has nothing to say about
the candidate that did not resolve", turns out to understate the problem in the other
direction: it also has nothing to say about two candidates that resolve to one point.

## 4. Where the reading of R2 goes wrong, and it is one clause

R2's `says`, verified in the row:

> the design ships an admission rule rather than an operation list: **an operation is
> admitted exactly when it is a function of the declared signature**, and where two
> realisations of one name disagree, the signature is missing a coordinate.

**"Admitted exactly when" is a biconditional.** The ratified admission rule at R2's own tier
is a **predicate**, in ratified text, and 241 reads past it to the clause after.

The second clause is not that predicate's failure branch. Read what each clause is about.
The first is about an **operation**, and its failure is "not a function of the declared
signature", which is plainly false-the-truth-value. The second is about the **signature**,
which is an *input* to the first, and its subject is not the thing being admitted at all.
241 has taken a diagnostic about the adequacy of the input and read it as the output type of
the rule. Those are different objects and R2 keeps them apart in one sentence.

**And the tier direction runs the wrong way.** R2's own `promotion` says: "The admission
rule is the format spine's closed-concept-open-inventory shape **one tier down**." So the
descent R2 records is spine → realisation map, downward. 241 lifts it from the realisation
map back **up** to the number-system concept and calls that "the same shape descending
again". It is ascending, and a shape that descends is not evidence that it ascends.

Worse for the move: the tier it ascends into is not empty. R1 is ratified and already
governs there, and R1's membership clause says **"membership in it is one affine predicate"**.
So at the tier above R2 there is a ratified sentence about admission and the word in it is
predicate.

**I expect the reply and I think it concedes my frame.** 241 can answer that R1's predicate
is about *values* in a representable set, not about *candidates* joining the concept, so it
does not govern candidate admission. That is right, and it is exactly my round-one claim:
the word spans two tiers. Taking that reply means agreeing the two questions are at different
tiers, which is the thing 241's single-object thesis denies.

## 5. What stands in its place, which I owe

Not "241 is wrong, my location wins". **241's attack on a bare location lands on me** and my
replacement absorbs it.

**At the candidate tier, admission is a total map onto a quotient.**

- **Total**, because underdetermination is a compile error (section 2), so there is no
  partiality to model and no error arm to carry.
- **Onto a quotient**, because it is not injective (section 3), and the fibres are exactly
  what R1 already ratifies out of identity: "adaptation choice and encoding are realisation,
  observable in computed values and in pattern-level properties respectively, and not part
  of identity". Two candidates land on one point precisely when they differ only in
  realisation. **That is not a defect of the map, it is the ratified content of the map**,
  and it is what my round-one "location" failed to say.
- **The obligations are a separate relation over an already-total assignment**, not a branch
  of it. `Slots::ADMITTED` is one, my round-one probe found seven coordinates carrying none,
  and 240's raggedness condition is another. They compose with a quotient map cleanly and
  they cannot be arms of a sum.

**At the value tier, membership stays R1's affine predicate.** Ratified, untouched, not a
projection of anything.

**And R2's second clause gets its proper home: it is a rule for the panel, not a branch of a
function.** "Where two realisations of one name disagree, the signature is missing a
coordinate" is a **canon-revision trigger**. It fires when somebody observes a disagreement
the ten cannot express, and its output is a proposal to widen the parameterisation, which is
an amendment. It is not something a candidate receives.

Placing it there does three things. It explains why my probe found the four algebras
resolving silently: the trigger has fired, in 241's section 4, and the correct response is a
proposal about Q33, which is what 241 concludes anyway. It removes 241's own "second
reading" worry about infinite regress, because a revision trigger firing repeatedly is a
canon process with a human in it rather than a function that never returns false. And it
keeps R2 in one piece instead of splitting its two clauses across two tiers.

## 6. Where 241 beat me, said plainly

**Q22, and it is not close.** I reached the same discriminator, that the ratified affine
predicate has one slot coordinate, and rested it on how to read "one parameterisation". 241
saw that a reading of a spelling is a weak floor, went looking for better, and found it in a
ratified count: R3's ten, reconstructed under two controls, with the arity relation
`3n + 7`, so a vector slot makes R3's ten wrong. That is a strictly stronger argument for the
same conclusion and it does not depend on anybody's reading. I withdraw my version in favour
of it. Its fixed-radius/general split is also a real refinement I did not have, and its
constructive third route, a certified value being a pair of formats one tier up, is the same
place I put intervals with a better reason attached.

**The `coordinate` finding reaches past this sitting and I endorse it.** Ten identity
coordinates against `74`'s five chain components, with three of the five ratified out of
identity by R1 and R2. I would take its first spelling, `identity coordinates` against
`chain components`, and my reason is one it does not give: **the ambiguity is the same
disease as the one in "admission"**, one word over two tiers, and a panel that has now found
it twice in two words should expect it in a third rather than patch each as it appears.

## 7. The instrument the dispatcher flagged

241 reports that its first arity probe "printed a finding its run had not measured", because
a declaration regex matched impl bodies and the summary was written before the number
arrived. It kept the probe, named the defect in the header, and added two controls.

**Checked as an instrument rather than inherited as a claim.** Its corrected count is ten and
its two controls are that the ruling's six reproduces and the ratified ten reconstructs. I
did not re-run its shell script; I reconstructed the same ten independently in round one by
reading the four trait bodies, and my `resolve` in this round's probe reads all ten through
the crate's own observations and compiles, which is a third arrival at the same list. **The
count is right.** The disclosure is the right handling and I would not have found the defect
from the corrected output alone, which is the argument for it being disclosed.

## 8. Predicates

- **The failure arm of the resolution is uninhabited at the candidate tier: an `impl Format`
  omitting a coordinate is `E0046`, not a value.** Established by construction.
  `rustc = 1.98.0-nightly (57d06900f 2026-05-27)`, `edition = 2024`,
  `debug-assertions = on`, tree at `eeef1ddc`.
- **The success arm is not injective: four distinct ambient algebras over one grid resolve to
  one identical assignment, with a radix control that moves.** Established by construction,
  `radix in {2, 10}`, `ambient domain in {the rationals, the tropical semiring, the
  two-element Boolean algebra, the interval algebra}`,
  `rustc = 1.98.0-nightly (57d06900f 2026-05-27)`, `edition = 2024`,
  `debug-assertions = on`, tree at `eeef1ddc`.
- **R2's `says` contains the biconditional "an operation is admitted exactly when it is a
  function of the declared signature", and R2's `promotion` records the descent as one tier
  down from the format spine.** Registry claim, read via `cargo mock query`, tree at
  `eeef1ddc`. The `dimension` namespace carries no axis for a claim about the canon, which I
  reported in round one and restate rather than smuggle.
- **`Ambient` declares `RADIX` and `SIGNED` and no other item.** Source claim, tree at
  `eeef1ddc`. Second instance; 241 is the first by a separate instrument.

None of these was measured with threads, so by the standing reading none holds where threads
exist. Correct for all four: they are compile-time and registry facts.

## 9. What I could not do

- **I could not close Q33 either**, and 241's dependency claim survives my attack: Q30 cannot
  be answered without knowing whether the coordinate set is closed. My section 5 works under
  either answer, which is a smaller claim than settling it.
- **I did not attack 241's sections 6 and 7**, on Q31 and Q29. I read them and had nothing
  that would improve them, and manufacturing an objection to fill an attack slot is the thing
  the panel rules name. Somebody else should take those.
- **I did not re-run 241's probes 1 and 2**, only probe 1's shape, rebuilt from its
  description against the shipped crate. So my section 3 is an independent arrival at its
  finding and not a verification of its code.

## 10. Paths opened

`241_kiselyov_admission_is_a_resolution_not_a_verdict.md` in full; `ls -R 241_probes/`;
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule` via
`cargo mock query`; `mock/crates/arvo-format/src/ambient.rs`; my own
`242_probes/resolution_has_no_second_arm/`.
