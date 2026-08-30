# 177. Independent check on the chain revision

Checking `176` against the members it compresses, worked from the sources forward. I took no part in
the tenth unit; my last work here was `163`, the ninth unit's second signature.

---

## Verdict, first

**The revision is sound.** Every dissent it claims to have reproduced reproduces on my own rerun,
with its control firing. The B3 repair resolves the contradiction rather than relocating it, and the
direction of the structural claim that the repair actually needs is verified by measurement. Every
number it quotes from a source is correct at that source. Every negative claim about evidence it
makes is true when turned into a search. The A6 artifact repair moved no number. The anchor
accounting reproduces exactly and survives an independent extraction with its own accounting section
cut out. Nothing reserved is closed.

**Seven findings, none of which overturns an amendment.** In descending order of what they cost:

1. **The reproduction catch is stated as a biconditional and only one direction holds.** The
   direction the repair needs, licence holds implies the family is a singleton, is verified on
   `175`'s own four chains. The converse, which the words "exactly" and "precisely" assert, has a
   measured counterexample.
2. **Clause 5 now carries three sweep-dependent counts in canon text**, into a statement that
   carried no measurement figure before, and `174`'s own coverage note says one of the three moves
   with a parameter the clause does not name.
3. **A resolution is substituted for another between source and revision.** `175`'s column is the
   identity; `176` reports it as the wrap resolution. Measured, they are different columns.
4. **The marking convention names two mark positions and the statement has three**, and the
   amendment that states the convention adds an instance of the third.
5. **The rung discount is carried, and not at full strength.** Two of its five components are gone,
   including the one that anchors it to a measured fact about this unit.
6. **`176`'s canon gate enumerates three of the six reserved items** while claiming universally that
   nothing below closes anything reserved.
7. **`176`'s own accounting instrument carries a stale docstring control** naming a ninth-unit
   anchor, which is the A6/B7 class in the file that repairs A6/B7.

**One finding I began to write as severe and withdrew after opening the source**, recorded in section
8 because the near miss is worth more than the finding would have been.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` in full, including its normative "How to read an entry", and `RULES.md`
in full. An independent check on a revision is what `RULES.md` requires of a compression, and the
brief's own framing, that the author of a compression is the person who believes it entails, is that
rule. Nothing in this file closes anything reserved, and section 7 verifies the revision closes
nothing either. I13 is the working method and is argued with nowhere.

### 0.2 Test gate: passed, at 123 across 13

Twelve crates crate by crate at `--release`, `bitpack-write-contend-shared` serialised and otherwise
untouched per the standing instruction. `177_probes/run_test_gate.sh`, output
`177_probes/gate_release.out`:

```
12 crates: 9+12+6+5+3+6+1+3+11+7+15+30 = 108   [--release]
bitpack-write-contend-shared, --test-threads=1  = 15 passed, finished in 1.99s
total                                           = 123, all passing
```

The script's negative control (a crate producing no parseable pass count prints `MISSING OR ZERO`)
was armed and did not fire, which is the correct outcome. **This is a fifth confirmation that the
serialised crate terminates in about two seconds**, after the four `176` names; my figure is 1.99s
at `--release`, and the profile is stated because a bare wall-clock figure for this crate has cost
this panel a true finding once.

---

## 1. The refusal repair: it resolves the contradiction, and the resolution is not a relocation

**What B3 said.** `175:180-188`: clause 2 licenses realisations inducing *the* stretch's boundary
function; clause 3 makes the schedule free on an unbound edge and says two schedules compute
different functions; so on those edges the stretch has a family and the definite description has no
referent. Measured at `175_probes/clause23/`: three of four chains carry more than one boundary
function, the witness four, two placements differing at the boundary on 30 of 256 inputs.

**The repair, adopted from `175`'s R2.** Clause 2's invariant becomes the **declared grade** rather
than the boundary function.

**Does it relocate the contradiction?** No, and the reason is structural rather than measured. The
definite description that failed was "the stretch's boundary function", whose referent a free
schedule multiplies. Its replacement is "the stretch's **declared** grade", whose referent is
supplied by the consumer's declaration rather than derived from the realisation. One declaration per
stretch, so the description denotes by construction, and nothing the schedule does can multiply it.
The family is then admitted or refused wholesale according to whether every member meets the grade,
which is `175`'s own stated reason for preferring R2 and is correct.

**Do exact grades recover the original definite description?** Yes, and I checked it against clause
4's grade list rather than against `176`'s summary of it. Clause 4 names four grades
(`173:556-566`): composite correct rounding, stepwise correct rounding, bounded drift, and
structural exactness. `176`'s amended clause 2 maps three of them to unique functions: composite
correct rounding to `pi` of the exact value, structural exactness to the exact value, stepwise to
the fully-eager function. Each of those names a function of the input, so on a stretch declaring one
of them the family has at most one member meeting it and the earlier form is recovered. Bounded
drift is correctly excluded and stated as admitting a family within its bound. **The mapping is
right and complete over clause 4's list.**

**One thing the repair changes that neither signature flagged, and it is in the revision's favour.**
Under the original clause 2, the schedule choice was routed to a licence whose invariant it broke.
Under the repaired clause 2 the schedule choice is routed to clause 4's grades, and clause 4 is
`[enumeration]` while clause 2 is `[normative]`. So the repair moves the schedule question from a
normative licence to an enumerated vocabulary, which is the direction that makes it checkable. That
is what `175` meant by "it explains why clause 4 sits where it does", and it survives inspection.

---

## 2. The reproduction catch: one direction verified, the other refuted

This is the load-bearing claim, the reason `176` says the repair composes rather than patches, and
it is the one the brief sent me to test. `176:68-75`:

> `175`'s chains with boundary-function families are exactly chains where clause 6's licences refuse
> deletion ... So the definite description in clause 2 denotes precisely where a deletion licence
> holds or an exact grade is declared, and the contradiction lives only where the licences already
> say no.

"Exactly" and "precisely" are biconditionals. Decomposed:

- **(i)** a licence holds implies the family is a singleton. **This is what the repair needs.**
- **(ii)** the family exceeds one implies both licences refuse. Same statement.
- **(iii)** the family is a singleton implies a licence holds. **Asserted by the words, needed by
  nothing.**

`177_probes/p1_licence_family_biconditional.py`, which first reproduces `175`'s table so the
extension is anchored to it, then computes both licences of clause 6 (`173:575-580`) structurally
over the reachable set. Four controls declared before the run, all passing.

```
                     chain  identity  round-to-8   wrap
                +97 *3 +13         1           1      1
                 *3 >>1 *5         1           4      2
            +97 *5 >>1 +13         1           3      3
                 *3 *3 +97         1           3      1

                     chain  resolution  algebra  range  family
                +97 *3 +13  round-to-8    False  False       1
                +97 *3 +13        wrap     True  False       1
                 *3 >>1 *5  round-to-8    False  False       4
                 *3 >>1 *5        wrap    False  False       2
            +97 *5 >>1 +13        wrap    False  False       3
                 *3 *3 +97        wrap     True  False       1

  (i)   licence holds => family = 1 : HOLDS  (no counterexample)
  (ii)  family > 1 => licence refuses: HOLDS  (no counterexample)
  (iii) family = 1 => licence holds  : FAILS
        COUNTEREXAMPLE  chain='+97 *3 +13'  resolution=round-to-8  algebra=False range=False family=1

  K1 the family counter can report > 1        : PASS
  K2 identity resolution gives 1 everywhere   : PASS
  K3 licence holds => family 1 on every such  : PASS   (2 cells where a licence holds)
  K4 the licence predicate is not constant    : PASS
```

The round-to-8 column reproduces `175`'s exactly: 1, 4, 3, 3.

**So the repair composes.** Direction (i) is what "the contradiction lives only where the licences
already say no" needs, and it holds on every cell measured. That half of `176`'s catch is right and
is worth more than either signature noticed, because it is the reason the repair is not a patch.

**And the word is wrong.** `+97 *3 +13` under a rounding resolution has a singleton family while
both licences refuse. A reader taking "precisely" literally concludes that wherever no licence holds
and no exact grade is declared, clause 2's earlier form fails; on that chain it does not. The cost
is bounded: the over-claim is about the repair's **scope characterisation**, not about the repair.

**F177-1. The reproduction catch's needed direction holds and its stated biconditional does not.**
Over `175`'s four chains at three resolutions, no cell has a licence holding with a family above
one, and one cell has a singleton family with both licences refusing. `W = 8, F = 0, signedness =
unsigned, resolution in {identity, round-to-8-clamped-at-248, wrap mod 256}, ops in {+97, +13, *3,
*5, >>1}, chains of depth 3 and 4, all interior placements, inputs exhaustive over 0..=255, threads
= 1, target features any`. Evidence: `177_probes/p1_output.txt`.

**Amendment.** Replace "are exactly chains where" with "are chains where", and "denotes precisely
where" with "denotes wherever". One word each, and the composition survives intact.

---

## 3. A resolution substituted for another between the source and the revision

The same sentence continues:

> Under the wrap resolution, where the algebra licence holds, `175`'s own identity column and `168`
> 4.1's degeneracy table agree the family has one member at every depth.

`175`'s column is not a wrap column. `175_probes/clause23/clause23.py` defines it as
`def ident(v): return v`, with the probe's own comment recording that an earlier version used a
clamp there and the control failed. It is the identity, so no placement can do anything, and the
singleton is a property of the control rather than of any resolution.

Measured, the wrap column is **not** all ones: 1, 2, 3, 1 over the same four chains, the two chains
carrying a right shift being the two above one. `168` carries the same fact in its own text, at
4.3's control table: `CONTROL wrap, chain containing a right shift  DISAGREE at x=4962`. And `168`
4.1's degeneracy table, which the sentence cites, sweeps `+k, *3, -k` and contains no shift at all,
which is why its "wrapping's degeneracy is 1 at every depth" is true there.

**The sentence survives on its restrictive clause and its citations do not support it.** "Where the
algebra licence holds" excludes the shift chains, and my (i) verifies the restricted claim. What is
wrong is the support: the identity column is vacuous for any claim about wrap, and `168` 4.1's table
establishes the wrap case only for homomorphic chains while `168` 4.3's own control is the
counterexample sitting in the same file.

**F177-2. The revision reports `175`'s identity column as a wrap column, and the two differ.**
Identity gives 1, 1, 1, 1; wrap gives 1, 2, 3, 1 on the same chains. Same predicate as F177-1.
Evidence: `177_probes/p1_output.txt`, and `175_probes/clause23/clause23.py`'s `ident` at source.

**Amendment.** Cite `168` 4.3's algebra licence for the restriction and `168` 4.1 for the
homomorphic case, and drop the identity column from the sentence: it is `175`'s control, not
evidence about wrap.

---

## 4. Clause 5 now carries three sweep-dependent counts in canon text

`176`'s amended clause 5 embeds `[measured: an optimum exists in all 663 unrealisable-deferral cells
swept, is not in general subset-minimal, and falls short of pi(exact) in 17 of them, worst shortfall
160 of 256]`.

All three numbers are correct at source: `174_probes/r1_finiteness_and_the_deferral_theorem.out`
lines 7, 21 and 22 give 663, 17 and 160. That is not the finding.

**The finding is that they are in the statement.** Two facts decide it.

`173`'s twelve clauses carry no measurement figure in numerals. Every numeral in them is a clause
number or a cross-reference (`sed -n '/^## 4. The statement/,/^\*\*Permanence/p' 173_...md | grep -oE
'\b[0-9]+\b' | sort -u` returns `1 10 108 11 12 122 156 161 164 2 3 4 5 6 7 8 9`). The one
quantitative bound in the statement is clause 7's "up to two bits loose", written in words and
scope-free. So the amendment introduces the first sweep-scoped figures the statement has carried.

And the measurement's own author says one of them is not stable. `174:384`: "A4 rests on r1, and its
`pi` is the clamp; a grid-shaped `S` would change the shortfall count". `176` quotes that sentence
itself, in its section 8's "Not verified" paragraph, while the count it qualifies sits in the
statement.

r1's predicate has nine dimensions (`174:192-194`: `W = 8, S = [0,255], pi = clamp, 600 chains of
depth 3..=5, ops in {...}, carriers 8..=24 and 120, all placements, inputs exhaustive over 0..=255,
threads = 1`). The clause carries the word "swept" and the range 256, and none of the nine.

**This is not the predicate-widening defect**, and I checked before writing it as one: statement
clauses in this candidate are not written in the predicate notation, so applying that notation to
one would be a category error. It is the permanence test. The **existence** claim, that a finite
carrier sometimes cannot attain the optimum by any placement, survives a rewrite in another decade;
663, 17 and 160 are properties of one sweep and do not.

**F177-3. The amendment places three sweep-dependent counts into a statement that carried no
measurement figure, and the measurement's own author records that one of them moves with a parameter
the clause does not name.** `corpus = 173 section 4 and 176 section 4 at this branch; source = 174 r1
at its stated predicate`. Evidence: the grep above, `174:192-194`, `174:384`,
`174_probes/r1_finiteness_and_the_deferral_theorem.out`.

**Amendment.** Keep the existence clause and the governance pointer to clauses 7 and 9, both of
which are permanent and both of which answer `174`'s A4 exactly. Move the three counts to L-entry
territory, where an anchor and a predicate can travel with them. `174`'s A4 asked for a canon reader
to be able to know clauses 7 and 9 govern; it did not ask for the counts.

---

## 5. The marking convention names two positions and the statement has three

`176`'s amended legend: "A clause's opening mark gives its primary kind; a trailing mark attaches to
the sentence immediately before it."

`175` proved the trailing reading forced from clauses 4 and 5, each ending in a mark with no
successor. That is right and I reproduced it. What neither signature nor the revision asks is
whether those are the only positions.

`177_probes/p3_marks_after_the_amendment.py`, built on `175`'s own mark pattern and blockquote
extraction so a disagreement with `175`'s table would be my defect:

```
=== UNAMENDED (173) ===
  clauses 12   clauses opening with a mark 12   terminal marks 3   intra-sentential marks 1
    clause  7  [measured]   followed by: "; and a nested composition's derivation consumes the"

=== AS AMENDED (173 + 176) ===
    clause  5  [measured: an optimum exists in all 663 ...]  followed by: ", **and the gap is governed by"
    clause  7  [measured]                                    followed by: "; and a nested composition's"

  M1 every clause opens with a mark         : PASS  (12 of 12)
  M2 clauses 4 and 5 have terminal marks    : PASS  [4, 5]
  M3 the intra-sentential class exists before 176 : PASS  (1)
  excluding clause 4: intra-sentential marks before 1, after 2.
```

A mark between two semicolon- or comma-joined clauses of one sentence is neither opening nor
trailing, and "the sentence immediately before it" does not pick out a referent for it. Clause 7 has
carried one such mark since `173`. `176`'s amended clause 5 adds a second, in the amendment that
states the convention.

**Clause 4 is excluded from the count and I say why.** `176` does not quote clause 4's amended text
anywhere, only describes the change, so where the `60` qualifier sits relative to the mark is the
reader's choice. My synthesis put the qualifier after the mark, which made it read intra-sentential.
**That is my artifact**, and reporting it would be the scope-not-mechanism class this unit named.

**F177-4. The stated marking convention classifies two mark positions and the statement as amended
contains three.** One intra-sentential mark before the amendment (clause 7), two after (clause 7 and
clause 5's new bracket), counted over text quoted rather than synthesised. `corpus = 173 section 4
and 176's quoted clause blocks at this branch; instrument = 175's own mark pattern`. Evidence:
`177_probes/p3_output.txt`.

**Amendment.** One clause added to the legend: a mark inside a sentence attaches to the
semicolon- or comma-separated clause it follows. That is what both instances already mean, and it
costs nothing.

---

## 6. The rung discount is carried, and it is not at full strength

The brief asked me to verify this specifically, because it is the most droppable sentence in the
file. It is present, in `176`'s L3 end state, and it is shorter than its source in two ways that
matter.

`175` 5.3 has five components. Three survive: both instances are definitions and that is the weakest
kind of two; two members of one model family on one premise set; no failure-independence argument
detects a shared framing, and the stated failure modes are the ones the authors thought to name.

**Two are gone.**

**The reason is gone.** `175:295-299`: "An empirical claim two parties reach independently is
corroborated because the world had to cooperate twice. A definition two parties reach independently
may be corroborated, or may be two members of one model family finding the same framing natural."
That contrast is why a definitional two is weak. `176` keeps the conclusion and drops the argument,
so the sentence asserts a discount without saying what makes it one.

**The evidential anchor is gone.** `175:301-303`: "`169` found one shared input by looking for it.
Nothing establishes it found the only one." I verified `169`'s own statement of it at `169:84-108`,
which bounds itself explicitly ("this is easy to overstate and I am not going to") and claims only
that the rule was in every context and declared by nobody. That sentence is what turns the discount
from a general worry into a measured fact about this unit, and it is the one that makes O-4
necessary rather than merely valuable.

**F177-5. The discount is carried with its conclusion and without its reason or its evidential
anchor.** Three of `175` 5.3's five components survive into `176`'s L3; the empirical-versus-
definitional contrast and the `169`-found-one-by-looking point do not. `corpus = 175 section 5.3 and
176 section 5 at this branch`. Evidence: the two files opened at the cited lines.

**Amendment.** Restore the second one at least, in `175`'s own words. It is one sentence and it is
the only part of the discount that anything measured supports.

---

## 7. What I checked and found nothing wrong with

Reported because the brief asks for it and because a check that reports only defects has not said
what it covered.

**All six dissent reproductions, rerun independently.** `sh 176_probes/reproduce_dissents.sh` on my
own host: `174` r1, `174` r2, `175` marks, `175` clause23, `175` partial3 and `175` options all
REPRODUCE, and the script's must-differ control fires. The only line differing from the committed
output is the A6 one, which now reads `NOT FOUND (already repaired?)` because the repair landed;
`176` states exactly that ("CONFIRMED PRESENT at reproduction time; repaired in section 6") and the
script's own fallback text anticipates it. **Correctly labelled.**

**The harness self-catch.** The corrected script emits the runner's `##########` headers and the
trailing `echo`, and partial3 now reproduces. I checked what rests on the bad run: it produced a
**verdict** (DIFFERS), not a number, and every partial3 figure `176` quotes comes from
`175_probes/partial/partial3.out`, which I opened: "constructed splits: 1", inputs 0..4096, value
disagreements 0, at both profiles. **No number rests on the bad run.**

**The A6 repair.** `173_probes/anchor_accounting/count_anchors.py` and `dropped_anchors.txt` now say
"nine source files" and "into 173". I regenerated the accounting and diffed against the committed
output: **identical**. `176`'s "no number moved" is exact.

**Every number quoted from a source.** 663, 17 and 160 at `174` r1's output lines 7, 21, 22; 256/256
at off, 53/256 at on, 203 diverging at `174` r2's output lines 2, 11, 12; three of four chains, the
witness four, 30 of 256 at `175` clause23's output; the 1-in-4096 split at partial3; the disclaimer
at 1 occurrence in `60` and 0 in `173`. **All correct.**

**`60:210`.** Opened. It holds "That is a statability argument, not a benchmark". The anchor is
right and the `[argument]` kind now attached to the statability sentence matches `60`'s own
description of it, which is what `174` A3 asked for.

**The anchor accounting, independently.** `177_probes/p2_anchor_diff_both_ways.py` reproduces
`176`'s numbers exactly from a separate extractor: 63 source anchors, 1 candidate anchor, 0 carried,
1 novel (`60:210`), 63 dropped, 2 of 90 probe paths carried from sources. A deliberately looser
extractor needing no backticks finds nothing extra. With section 7 cut out entirely the anchor set
is unchanged, so **no carried anchor exists only inside the accounting block** and the diff is not
self-disabled. The dropped list is a sibling file. All three of my controls pass.

**The option set, by label, over `173`, `176` and the pair.**
`177_probes/p4_option_census_after_the_revision.sh`, control returning zero on two nonexistent
labels. O-171-1 goes from 0 in `173` to 5 in `176`, with a dedicated R-o entry in the
register-repairs pass, which is where B6 asked for it. **Three other labels are also at zero in
both** (Q-C3, Q-C6, Q-C7), and I checked each rather than reporting a count: Q-C6 is retired at
`173`'s R-k (`173:500`), Q-C3's cost half is carried at `172:519` and in `173`'s unpriced list, and
Q-C7's substance is clause 12 itself. **All four accounted for. No live option lost.**

**Every absence claim, turned into a search.** `177_probes/p5_absence_claims.sh`, with a
present-pattern and an absent-pattern control.

- "Neither signature stated this composition": neither `174` nor `175` has a line joining a
  boundary-function family to a licence refusing. **True.**
- "the signatures quoted the sentence and neither spelled its line": `174` quotes the disclaimer
  three times and `175` once; neither spells `60:210`; `174` spells `60:69-84`, a different anchor.
  **True**, and it is what makes `60:210` correctly novel.
- "partiality was in neither `171`'s tested six nor its named four": the string `partial` occurs
  **zero** times in `171`. **True in the strongest available form.**
- "recorded only inside L3's rung history": O-171-1 occurs zero times in `173`, and `173:76` inside
  L3 is the only mention of `172` section 10. **True.**

**Nothing reserved is closed.** `177_probes/p6_reserved_still_open.sh`. The reserved items appear in
`176` only in its canon-gate sentence, which asserts they stay op's. No closure verb is applied to
any of them anywhere in the file.

---

## 8. The finding I began to write as severe and withdrew

I had written up `173:676`'s "**X1 through X4**" as a stale ninth-unit label, on the ground that this
candidate's contested set is **X-A through X-F** (`173` section 2, six items, and `173:656` names
them that way). It propagates to `174:28` and into `176`'s canon gate, so it looked like the A6/B7
class sitting uncaught in the sentence addressed to op.

**It is not a defect.** `173:676` reads "**X1 through X4**, exactly as `164` carries them", which is
a deliberate carry-forward of the **ninth unit's** reserved items from the primitive candidate, not
a mislabelling of this unit's. I know those four; I signed the candidate that opened them. Opening
the line rather than trusting my reconstruction of it is what caught this, and it is recorded because
the brief asked for a severe finding to be verified before it is reported as one, and because acting
on mine as written would have renamed a correct cross-topic reference.

**What survives of it is small and real.** `176`'s canon gate lists "the container premise, Q65, X1
through X4, the canon-form question coupled to `156` item 2, and whether the observability principle
becomes an arvo intent". Those are items 1, 2 and 4 of `173` section 6. **Items 3, 5 and 6 are not
named**: which accuracy target I7 names, which chain carrier ships, and the vocabulary calls. The
gate's claim is universal ("Nothing below closes anything reserved") and its enumeration is half the
set. I verified none of the three unnamed items is closed by any amendment, so the gate's claim is
true; its list is partial.

**F177-6. The canon gate's enumeration of reserved items covers three of the six in `173` section
6.** `corpus = 173 section 6 and 176 section 0 at this branch`. Evidence:
`177_probes/p6_output.txt`. The unnamed three are verified unclosed, so this is a precision defect in
a scope sentence and not a closure.

---

## 9. One defect in the revision's own instrument

`176_probes/anchor_accounting/count_anchors.py`'s docstring declares its positive control as
"`112:904-906` must be found in the candidate (it is cited there)". `112:904-906` is the **ninth**
unit's anchor, from the primitive candidate. The **code** runs the right control, `60:210`, and it
passes. So the instrument is correct and its documentation describes a control it does not run.

This is the A6/B7 class exactly: a generated instrument carrying its predecessor's provenance
strings. `176` repairs that class in `173_probes/` in the same file. It is worth one line rather
than a repair round, and it is worth naming because the class has now recurred three times in two
units and nobody has checked an instrument's docstring against its code before.

**F177-7. The revision's accounting instrument documents a control it does not run.** `corpus =
176_probes/anchor_accounting/count_anchors.py at this branch`. Evidence: the docstring at lines
15-21 against the control at the `-- controls --` block, both opened.

---

## 10. Entailment: does the revision claim only what the members established

Checked clause by clause against the sources rather than against `176`'s own account of them.

**Clause 1's amendment** adds "read at the build profile in force" and "whether two realisations
share a definedness domain is itself a function of the profile". `174` r2 establishes exactly that:
256/256 agreeing at `off`, 53/256 at `on` with 203 inputs where one realisation panics. The
amendment **adds** a dimension to the hypothesis rather than dropping one, which is the safe
direction, and `176`'s reading of r2 ("its probe built to show the clause false at `on` showed it
vacuous for its pair instead") is what `174` 3.2 says of itself. **Entailed.**

**Clause 2's amendment** names two normative premises where `173` named one. `175` B2 is the source
and it says the lower bound is unnamed; the declaration is the natural filler and `175` proposes it
in R2. The traceability sentence ("`167`'s original sentence carried three claims, `171` quoted it
whole and named two") is `175` B7's, carried accurately. **Entailed.**

**Clause 3's replaced sentence** routes the unbound-edge placement to clause 4 and clause 5 and adds
"where the declared grade is exact, clause 6's licences are the proof that a placement meets it".
That last conjunct is `176`'s own, from the reproduction catch, and it is entailed by direction (i),
which I verified. **Entailed, and stronger than either signature offered.**

**Clause 4's amendment** changes one mark to `[argument]` and quotes `60`'s disclaimer. `174` A3 and
`175` B4 both ask for it; `175` B4 asks unconditionally and `176` adopts the unconditional form,
which is what `175` established when it proved the trailing convention forced. **Entailed.**

**Clause 5's amendment** is `174` A4's, and A4 signs the theorem. The counts are correct at source;
section 4 above is about where they sit, not whether they are true. **Entailed.**

**L3's end state** is `175` B5's three-numbers-three-claims form. I checked the three: (P) as a
definition at two instances by failure-independence, (P) as rule-free-derivable at one, (L) at zero.
`175` 5.1 and 5.2 establish the first two and `172` 1.2 establishes that (L) is normative and
underivable, so zero is right. **Entailed, with the discount short by two components (section 6).**

**No rung is flattened upward anywhere I checked.** The one place a flattening would have been easy
is L3's "two instances", and `176` carries the discount that argues against it rather than dropping
it, which is the opposite of the failure this check exists to find.

---

## 11. Coverage, bounded honestly

**Read in full:** `176`, `175`, `174`, `173` sections 2, 4, 5, 6, 7 and 8, `166`, and my own
probes' sources.

**Read at the cited sections, opened rather than remembered:** `167` 9 (the Q-C options) and 12;
`168` 4.1, 4.3 and its predicate blocks; `169` 1.2 in full; `171` sections 3 and 11 by grep and at
the two lines quoted; `172` 1.2, 4.1 and 10 by grep; `60:206-212`; `173:76`, `:500`, `:543-552`,
`:600-607`, `:656`, `:672-692`; `174:28`, `:142`, `:147-200`, `:192-194`, `:266`, `:384`; `175:29`,
`:97`, `:164`, `:173`, `:178-240`, `:290-339`, `:466`; the committed outputs of `174` r1 and r2,
`175` marks, clause23, partial3 and options, and `175_probes/clause23/clause23.py` in full.

**Grepped, not read:** the bodies of `167`, `168`, `170`, `171` and `172` outside the sections
above; `43_rompf_what_a_composition_is.md`; `AGREEMENTS.md`; `OPTIONS.md`; `DROPLIST.md`.

**Not opened at all:** `164` and `165` (the ninth unit's revision and check), the ninth-unit
material beyond what I wrote myself, and every panel file below `43`.

**Which of my sections would move if something I leaned on were wrong.**

- **Section 2 is the load-bearing one and it rests on a model, not on arvo.** Its licence predicates
  are my reading of clause 6's two sentences, computed structurally over the reachable set. If
  "commutes with or is absorbed by" is meant more loosely than commutation at every step, more cells
  would count as licensed and direction (i) would have more chances to fail; it did not fail on any
  cell I could construct, and K3 records that only two cells had a licence at all, which is a thin
  base for a universal and I say so rather than round it up.
- **Section 3's finding is about support rather than truth.** If "under the wrap resolution, where
  the algebra licence holds" is read as one restrictive phrase, the sentence is true and only its
  two citations are wrong. That is how I have reported it.
- **Section 4 rests on the permanence test rather than on a measurement.** If op rules at `156` item
  2 that a canon may carry sweep-scoped figures, the finding narrows to "the figures need their
  predicate" and does not disappear.
- **Section 6's grading of the discount as short by two components is a reading of two texts.**
  Someone could hold that "the weakest kind of two available" carries the empirical contrast
  implicitly. It does not carry the `169` point under any reading, and that is the half I would
  insist on.
- **Section 10's entailment pass covered the amendments and not the seven clauses that stand.** I
  checked that `176` says they stand and did not re-derive them from the members; `173`'s own check
  is not something this dispatch was asked to redo, and if clauses 6 through 12 were wrong in `173`
  they are wrong here.

**Citations checked by opening them.** `177_probes/citecheck.out`, with two deliberately wrong
citations as controls.

**What I settled.** That the repair composes, by verifying the direction it needs rather than the
direction it claims.

**What I moved.** The reproduction catch from a biconditional to an implication; the wrap sentence
from supported to true-but-unsupported; the counts in clause 5 from correct to correct-and-misplaced.

**What I could not.** Price anything: no harness ran in this dispatch and no timing here decides
anything. Verify clauses 6 through 12, which stand from `173` and were checked there. Or reach the
question all seven findings sit under, which is whether a canon statement may carry a measurement at
all, and which is op's at `156` item 2.
