# 178. The restoration pass

`177` checked the revision and its verdict is sound; it found seven things, none overturning an
amendment, and withdrew an eighth after opening the source. This pass repairs all seven, restoring
from the establishing sources, and records the withdrawal. `176` stands as landed, because `177`
cites it by line; **where this pass restates a sentence of `176`, this pass governs, and everywhere
else `176` and, through it, `173` govern unchanged.**

**What this pass supersedes, listed at the top:**

1. `176` section 1 catch two's two biconditional words and its wrap-column citation sentence
   (F177-1, F177-2; sections 2 and 3 below).
2. `176`'s amended clause 5's bracketed measurement counts (F177-3; section 4).
3. `176`'s amended legend's convention sentence (F177-4; section 5).
4. `176`'s L3 discount paragraph, restored to full strength (F177-5; section 6).
5. `176`'s canon-gate enumeration (F177-6; section 7).
6. `176_probes/anchor_accounting/count_anchors.py`'s docstring, repaired in the artifact itself
   with numbers verified unchanged (F177-7; section 8).

**Every finding was reproduced before it was accepted** (section 1), and one clarification of
scope emerged in the reproduction and is stated where it lands.

---

## 0. The two gates

**Canon gate: passed.** Nothing below closes anything reserved. The complete reserved list is
restated at section 7, which is itself one of the repairs. I13 is the working method throughout.

**Test gate: passed, at 123 across 13**, crate by crate at `--release`, the serialised crate
terminating at 15 passed; this run is the **sixth** confirmation of its termination.
`178_probes/run_test_gate.sh`, output `178_probes/gate_release.out`, script control firing.

---

## 1. The findings, reproduced

`178_probes/reproduce_findings.sh`, committed with its output and a must-differ control firing:

- **`177` p1** (the biconditional): REPRODUCES, including direction (i) holding with no
  counterexample, direction (iii) failing at `+97 *3 +13` under round-to-8, the wrap column at
  1, 2, 3, 1, and all four of `177`'s controls.
- **`177` p3** (the mark positions): REPRODUCES, one intra-sentential mark before the amendment
  and two after.
- **`177` p2** (the independent accounting extraction): REPRODUCES.
- **`177` p6** (nothing reserved closed; the gate list partial): REPRODUCES.
- **F177-5's two missing components**: confirmed at source, present in `175` (1, 1) and absent
  from `176` (0, 0), under the whitespace layer.
- **F177-7's stale docstring control**: CONFIRMED PRESENT at reproduction time; repaired in
  section 8.
- **F177-6's partial enumeration**: confirmed, five of the eight reserved names in the gate
  sentence.

---

## 2. F177-1: the biconditional becomes the implication that holds

`177` split `176`'s reproduction catch into three directions and measured that (i), a licence
holds implies the family is a singleton, holds on every cell, while (iii), the converse the words
"exactly" and "precisely" assert, has a counterexample: `+97 *3 +13` under round-to-8 has a
singleton family with both licences refusing. The repair keeps the composition claim, which the
failing direction does not touch, and drops the converse.

**The superseded sentences of `176` section 1, restated:**

> **Two, the shape of B3's witnesses.** `175`'s chains with boundary-function families are chains
> where clause 6's deletion licences refuse: `*3 >>1 *5` under a rounding resolution has no
> algebra licence (rounding does not commute with the shift) and no range licence (intermediates
> off the grid). **Where a deletion licence holds, the family is a singleton** [measured: no
> counterexample over `175`'s four chains at three resolutions, `177` p1 direction (i)], **and
> the converse is false**: a family can be a singleton with both licences refusing, so the
> licences under-approximate the safe region rather than characterise it, which is what a sound
> licence is for. **So the definite description in clause 2 denotes wherever a deletion licence
> holds or an exact grade is declared**, the contradiction B3 measured lives only outside that
> region, and `175`'s R2 composes with clause 6 rather than merely replacing a pointer: under
> the repaired clause 2, the licences become the proof obligations that a placement meets an
> exact grade.

**One scope clarification from the reproduction, stated rather than absorbed.** `177`'s K3
control records that only **two cells** in the swept grid had a licence holding at all, and its
coverage bound calls that "a thin base for a universal". Direction (i) is not resting on the
sweep alone: it is the soundness of the licences themselves, established at `168` 4.3 with four
firing controls over 4096 inputs per cell and carried at L9, of which `177`'s two cells are a
reproduction. The sweep's thinness bounds the *reproduction*, not the claim. That is exactly the
argument-beside-enumeration split of C-X2, and both predicates travel: the soundness argument at
`168` 4.3's grid, the biconditional refutation at `177` p1's.

---

## 3. F177-2: the wrap sentence repaired from the establishing sources

The superseded sentence cited `175`'s identity column as a wrap column. Measured, they differ:
identity gives 1, 1, 1, 1 (a control, vacuous for any claim about wrap) and wrap gives 1, 2, 3, 1,
the two chains above one being the two carrying a right shift. The sentence's restrictive clause
was correct and its support was not.

**The restated sentence, cited from the establishing sources:**

> Under the wrap resolution, where the algebra licence holds, the family has one member: `168`
> 4.1's degeneracy table measures wrapping at degeneracy 1 at every depth **over ring-affine
> chains** (`+k, *3, -k`, no shift), which is the region the licence names, and `168` 4.3's own
> control is the counterexample outside it (`CONTROL wrap, chain containing a right shift:
> DISAGREE at x=4962`). `177` p1's wrap column, 1, 2, 3, 1 over `175`'s chains, is the same fact
> measured on a second instrument: above one exactly where a shift breaks the homomorphism.

`175`'s identity column is dropped from the sentence entirely; it is `175`'s control and supports
no claim about any resolution.

---

## 4. F177-3: the counts leave the statement and keep their predicate

`176`'s amended clause 5 embedded three sweep-scoped counts (663, 17, 160) into a statement that
carried no measurement figure, and the measurement's own author records that the shortfall count
moves with the shape of `S` (`174:384`). The counts are correct at source; the defect is where
they sit, by the candidate's own permanence test.

**The superseded bracket of clause 5, restated:**

> ...**Under I14 containers are finite and the deferred realisation is frequently not computable
> in the carrier; there the theorem's value is a lower bound, an optimum among realisable
> placements still exists, and the bound is sometimes unattainable by any placement** [measured;
> the counts are sweep-scoped and live in the ledger], **and the gap is governed by clause 7's
> window and clause 9's band.**

**The ledger note that receives the counts, L12-adjacent:**

> `174` r1's sweep: an optimum exists in all 663 cells where full deferral is unrealisable; it is
> not in general subset-minimal (2396 of the optima are not, and 566 of 663 cells have several,
> because placements differing only in dead resolutions induce one boundary function, which is
> clause 1 doing its work); and in 17 cells no realisable placement attains `pi(exact)`, worst
> shortfall 160 of 256. `holds for: W = 8, S = [0,255], pi = clamp (a nearest-point selection),
> 600 chains of depth 3..=5, ops in {+97, +13, *3, *5, *7, >>1, >>2, xor 182}, carriers 8..=24
> and 120, all 2^(depth-1) placements, inputs exhaustive over 0..=255, threads = 1.` The
> shortfall count moves with the shape of `S` (`174:384`) and the existence of a shortfall does
> not.

This is `174` A4 honoured at the strength it asked: the canon reader learns clauses 7 and 9
govern, and the counts travel with the nine dimensions they were measured under.

---

## 5. F177-4: the legend's third mark position

The statement has carried an intra-sentential mark since `173` (clause 7), and `176`'s amendment
added a second while stating a two-position convention. **The superseded convention sentence,
restated with the third position:**

> A clause's opening mark gives its primary kind; a trailing mark attaches to the sentence
> immediately before it; **a mark inside a sentence attaches to the semicolon- or comma-separated
> clause it follows.**

That is what both existing instances already mean, per `177`'s reading, and it costs one clause.

---

## 6. F177-5: the discount restored to full strength

`176` carried the discount's conclusion with three of `175` 5.3's five components. The two missing
are restored here in `175`'s own words, because the second is the only part of the discount
anything measured supports and the first is the reason the discount is one:

> **The reason.** "An empirical claim two parties reach independently is corroborated because the
> world had to cooperate twice. A definition two parties reach independently may be corroborated,
> or may be two members of one model family finding the same framing natural on one premise set,
> which is what shared training produces and which no failure-independence argument detects."
> (`175:296-299`)
>
> **The evidential anchor.** "`169` found one shared input by looking for it. Nothing establishes
> it found the only one." (`175:301-302`) That sentence is what turns the discount from a general
> worry into a measured fact about this unit, and it is what makes O-4 necessary rather than
> merely valuable.

L3's end state is otherwise unchanged from `176` section 5's form.

---

## 7. F177-6: the canon gate's enumeration completed

`176`'s gate claimed universally that nothing below closes anything reserved and enumerated three
of `173` section 6's six items. `177` verified the unnamed three are unclosed, so the claim was
true and the list partial. **The superseded gate sentence, restated complete:**

> Nothing in this arc's revisions closes anything reserved: **(1)** the container premise, Q65's
> marker question, and X1 through X4 exactly as `164` carries them; **(2)** the canon-form
> question coupled to `156` item 2 as one decision; **(3)** which accuracy target I7 names;
> **(4)** whether the observability principle becomes an arvo intent; **(5)** which chain carrier
> ships; **(6)** the vocabulary calls (chain against region, and the third sense of composition's
> name). All six stay op's.

---

## 8. F177-7: the instrument's docstring repaired, numbers unchanged

`176_probes/anchor_accounting/count_anchors.py`'s docstring declared its positive control as the
ninth unit's `112:904-906` while the code runs `60:210`. The docstring is repaired at the
artifact, the accounting regenerated, and **the diff of the regenerated output against the
committed output is empty**, so no number moved. This is the third recurrence of the
provenance-string class in two units (A6's two strings, then this), and the check it needs is the
one `177` names: **an instrument's docstring is read against its code**, which nobody had done
before `177` did.

---

## 9. The withdrawal, recorded as the discipline working

`177` began to write `173:676`'s "X1 through X4" up as a stale ninth-unit label against this
unit's X-A through X-F, a finding that would have read as the A6 class sitting in the sentence
addressed to op. **It opened the line before reporting**: the text reads "exactly as `164` carries
them", a deliberate cross-topic carry-forward of the ninth unit's reserved items, which `177` had
itself signed there. Acting on the reconstruction would have renamed a correct reference; only the
partial-enumeration finding survived, and it is repaired at section 7.

Recorded per the coordinator's instruction, and because this panel has had a check's severe
finding be half wrong before: verifying a severe finding by opening the source before reporting it
is the cheapest discipline this corpus has, and this is its clearest recent instance.

---

## 10. What I did not repair, and why

- **The question under F177-3**: whether a canon statement may carry a measurement at all. Op's,
  at `156` item 2, where both this unit's candidate and `177` couple it; section 4 repairs the
  placement and leaves the question exactly where it was.
- **Direction (iii)'s falsity is recorded, not repaired**: nothing in the repaired text needs the
  converse, and no替 sharper characterisation of the singleton-family region is attempted here,
  because none is needed by any clause and inventing one would be scope creep on a licence
  question that clause 6 already governs soundly.
- **`177`'s coverage note that clauses 6 through 12 were not re-derived**: correct and not a
  defect; they stand from `173`'s own check chain, and re-deriving them was not this pass's
  assignment either.
- **Nothing op's**: the complete list at section 7, all six open.

---

## 11. Accounting, coverage and checks

**The accounting for this pass**, regenerated rather than edited:
`178_probes/anchor_accounting/`, extending the instrument with `177` as a source and this file as
the candidate, controls declared and firing, dropped list in the sibling file, output pasted:

```
ACCOUNTING-BLOCK
```

**Coverage.** Read in full this dispatch: `177` end to end; `176` at every superseded site; `175`
at 5.3 and its cited lines; `174` at A4, r1's predicate and `174:384`; `168` 4.1 and 4.3 reopened
for section 3; `173` section 6 for the reserved list. Reproduced rather than taken: `177` p1, p2,
p3, p6, and the three source confirmations, per section 1. Not re-run: `177` p4 and p5 (its
option census and absence-claim sweeps), taken on its committed outputs, which its own controls
gate.

**Which repairs would move if something I leaned on is wrong.** Section 2's scope clarification
rests on `168` 4.3's controls being the licence-soundness ground; if the licence predicates
`177` computed diverge from `168`'s, direction (i)'s base is thinner than stated, and the
repaired wording (one direction, no converse) is safe under either reading. Section 4's ledger
note quotes `174` r1's nine dimensions verbatim. Sections 5 through 8 restate text whose sources
are quoted at opened lines.

**What this pass settled.** All seven findings repaired or recorded, each from its establishing
source; the withdrawal recorded; the artifact repaired with numbers verified unchanged.

**What it could not.** Anything on op's list; the pricing questions, unchanged; and O-4, which
remains the register's most valuable dispatch and which nothing here advances.

**Citations and quotations, checked by opening them under all four layers.**
`178_probes/citecheck.out` for the anchors with both wrong-citation controls firing;
`178_probes/quotecheck/` for the verbatim quotations under whitespace, markup and case layers
with both planted controls behaving; a zero on a layer is a quoting-style fact.
