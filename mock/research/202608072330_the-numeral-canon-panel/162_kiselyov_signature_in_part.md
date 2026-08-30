# 162. Signature in part on the candidate for the primitive

Kiselyov, author of `154` and `159`, which `161` compresses across both sittings. A signature is
not approval: it is the author of compressed material saying whether the compression represents it
at the strength its own file supports. Clause by clause below, and the value is in the four places
I do not sign.

**Summary.** I **sign** eleven clauses outright. I **sign with an amendment** on four. I **refuse**
one, and it is a rung on my own row that flatters me.

---

## 0. The gates

**Test gate: passed.** Thirteen `-shared` crates, crate by crate, `--release`,
`bitpack-write-contend-shared` serialised and untouched:

```
9+12+6+5+3+6+1+3+11+7+15+30 = 108   (twelve crates)
bitpack-write-contend-shared         = 15
                               total = 123, all passing
```

This is the twelfth count and it agrees with `161`'s eleventh.

**Canon gate: passed.** Nothing below touches the RATIFIED rung. The three questions reserved for
op are left open: I do not decide the container premise, I do not answer Q65's marker question, and
I do not adjudicate the contested items. My amendment 2 below is about **where** the container
premise's conditionality is marked, which is a statement defect, not an answer to the premise.

---

## 1. Refused: L15's rung on the entailment's ground

**L15 reads: "Rung: CONVERGED on the split; TWO+ INSTANCES on the entailment's ground (`154`
blind, `109`'s criterion blind, different routes)."**

**I refuse the second half.** It is my own row and it is inflated in my favour, which is the
direction I was asked to look.

TWO+ INSTANCES requires two derivations **of the same claim**. There are two claims here and each
has one deriver:

- **`109` section 11's claim** is a membership *criterion*: "A property belongs in the primitive iff
  it must be const-available in order to decide whether a program is valid, or to select a
  lowering." Opened at source: it grounds "const-available" in **I13**, in its own words, "op's own
  widening at I13 ... The criterion inherits that scope and I did not choose it" (`109:534-537`). It
  grounds the *scope* of the modal. It does not derive the modal.
- **My `154` section 2's claim** is the *entailment*: that the "must" is compelled rather than
  chosen, because a parameter left runtime forces the `cmp` I15 forbids
  (`154_probes/p1_saturation/sat.s:31-39`).

`109` mentions I15 five times and never as the ground of its criterion's necessity
(`grep -n "I15" 109_bellard_the_primitive_derived_cold.md | cut -d: -f1` returns 310, 320, 452, 454,
656; the first two are about naming as the validator, 452 and 454 are its I18 section, and 656 is
alternative C).
**So `109` did not arrive at the entailment by a different route. It did not arrive at it.**

**Amendment I would sign instead:** *Rung: CONVERGED on the split (both phase twos, independently
stated); **ONE EXPERT** on the entailment (`154`, blind), with `109`'s criterion at the rung its own
file supports.* The claim is not weakened by this, and `159` section 6's own warning applies to my
refusal as much as to anyone's: a corrected rung that reads as a refutation is how a true finding
gets retired. The entailment stands. It stands at one expert and it is asking for the second read
the rung exists to request.

---

## 2. Signed with amendment: the conditionality is not localised to one clause

`161` section 4's preamble promises "Clauses conditional on op's premise say so inline", and its
closing note says "Clause 4 is conditional on op." Section 9 states as settled that the statement
has "its conditionality localised to one clause". **It reaches at least three.**

I built my own instrument rather than reuse `157`'s p4 grid, because a claim I am making against a
candidate that cites `157` should not rest on `157`'s instrument.
`162_probes/p1_how_far_does_the_premise_reach.py` fingerprints each configuration's realisation map
over a probe range covering both sides of both bounds, with four controls declared before the run
and all four passing:

```
C1 container rule is inert without the observation  : True
C1b and the rules do differ in container bits       : True
C2 the observation separates at least one pair      : True
C3 a value-set difference separates on both branches: True

distinct primitives, container internal   : 32
distinct primitives, container observable : 64
pairs with identical (V,R) split by the observation: 32 of 32
```

It reproduces `157` F157-4's 32-against-64 from a different construction, which is corroboration of
that finding as a by-product and is offered as such.

**Clause 2 is conditional and unmarked.** "Its identity is that structure up to
denotation-preserving isomorphism, relative to the declared operation set." If the operation set
contains a container observation, the isomorphism must preserve the container, and the clause's
extension moves from 32 primitives to 64. `161`'s own L25 says exactly this ("only identity's
granularity moves with it"), so the candidate knows it; the statement does not carry it.

**Clause 6 is conditional and unmarked, and its truth value moves rather than its extension.** "The
realisation is **not part of identity** and is emphatically part of the surface." Clause 5 defines
the realisation as a lens: "a carrier, an offset and a width". Under footprint-observable the
carrier is identity-bearing, so clause 6's first sentence is **false on that branch**. That is a
stronger defect than clause 2's, because a reader on the observable branch reads a false sentence
with no marker on it.

**Amendment.** Mark clauses 2 and 6 inline as clause 4 is marked, and correct section 9's "localised
to one clause" to three. I am not proposing new text for the branches; that is the design tier's and
op's premise decides it. The defect is that the statement claims a localisation it does not have,
and a canon candidate that understates its own conditionality is worse than one that overstates it,
because the unmarked clause is the one a later reader builds on.

---

## 3. Signed with amendment: L20's rung was right for a reason that was not yet true, and now is

L20 records **TWO+ INSTANCES (one compiled instrument, one independent argument)**, the argument
being my F159-2.

When `161` was written that rung was **conditional on a premise I had not verified**. F159-2 argues
that I15 cannot see a `cfg`-varying realisation map because every single build satisfies it; the
argument needs a `const fn` to be able to read `cfg` at all, and I took that from `157` and said so
in `159` ("read rather than re-run; I did not rebuild it and I say so"). An argument resting on
another instance's premise is not fully independent of it.

**I closed it rather than flagging it.** `162_probes/p2_cfg_in_const_fn/`, built independently of
`157`'s probe, three controls declared before the run:

```
=== build base ===                     === build alt ===
HAZARD          R(MAX+1) = 8191        HAZARD          R(MAX+1) = 0
CONTROL_STABLE  R(MAX+1) = 8191        CONTROL_STABLE  R(MAX+1) = 8191
lowered(): cmp x0,x8; csel             lowered(): and x0,x0,#0x1fff
branch on a build value: none          branch on a build value: none
```

The hazard differs, the unreachable control does not, and neither build branches on the build. **So
the rung stands and now stands on its own evidence.** Sign, with the amendment that L20's
parenthetical should read *one compiled instrument, one independent argument since verified against
its own instrument at `162_probes/p2_cfg_in_const_fn/`*.

**And one sharpening of my own wording, which `161` inherited verbatim.** `159` F159-2 says every
build emits "one lowered path and no runtime check". The base build's body is `cmp; csel`, which is
check-shaped. It is not a validity check: it is the declared saturating semantics, which is
arithmetic and is exactly what I15 permits. **The claim F159-2 needs is narrower**: neither build
branches on *the build*, and each emits one path. Anywhere the candidate carries "no runtime check"
from my file, that is the phrase to tighten. The conclusion is untouched.

---

## 4. Signed with amendment: three of my anchors were dropped, and two are cheap to restore

`161_probes/anchor_accounting/dropped_anchors.txt` lists 188 dropped source anchors. Three are mine
(`grep -nE '154|159' dropped_anchors.txt`):

```
80: 154_probes/p1_saturation/sat.s:31-39
89: 159:176-185
90: 159:225-230
```

**No live option was lost**, which is the failure this panel has paid for twice, and I checked
specifically: O-A, O-B, O-C and O-E are all carried in section 7 with costs and discriminators, and
O-D's closure at R14 is correct on both branches per my F159-3. That check passed.

The three dropped anchors are a **degradation rather than a loss**: each one's content survives.
`159:176-185` is the O-C narrowing, carried at X2 and in section 7. `159:225-230` is F6's widening,
carried at L22. `154_probes/p1_saturation/sat.s:31-39` is the emitted check, and the file is still named at L15 without the
lines.

**Amendment.** Restore the line ranges on those three. `a-compression-is-checked-by-someone-else.md`
excludes citations into the nuked code tier from a canon candidate's obligation, and correctly, but
these are **panel-internal and probe anchors**, which that rule says count and must survive. The
cost of the drop is small and specific: a reader who wants to check that the `cmp` is really there
must now re-derive which lines of a 53-line assembly file show it, and a reader who wants the exact
wording of the O-C narrowing must find it in a 321-line file by search. Neither is fatal. Both were
free to keep.

---

## 5. Signed with amendment: R16's reason is narrower than the defect

R16 retires `111:555-556`'s adequacy-checkability sentence as "unpayable as stated: nobody in either
sitting has made a transfer argument".

**I sign the retirement and amend the reason.** The reason as written says the sentence is
undischarged. The stronger and truer statement, which `161`'s own L17 already contains, is that the
sentence asks for the **wrong kind of evidence**: soundness is structural and needs no enumeration
at any width, so no transfer argument could ever be relevant to it; and completeness is a
conjunction of inequalities, each discharged by one witness at the real width, so there is nothing to
transfer there either. A sentence that is undischarged might be discharged tomorrow. A sentence that
asks for a transfer argument where no quantity needs transferring is superseded permanently, which
is what R16's own "Superseded by L17's decomposition" says and its stated reason undersells.

My F159-2 does not bear on R16, and I want that said rather than left to look like support: it is
about soundness's **enforceability**, not about adequacy's checkability. Different half, different
claim.

---

## 6. Signed outright, with what I checked

Each of these I read against my own file and found faithful. Where a check of mine could have caught
a defect and did not, I say which.

**L3** (value set and realisation vary independently; my F5 as a second blind instance on direction
A). Faithful, including the TWO EXPERTS claim scoped to direction A only and direction B left at one
expert, which is exactly what `154` P2.2 claimed and no more. **My check that could have failed:** I
looked for whether my F159-1 discount should apply here, since the crate I read is in the
plan-shared family. It should not: the crate is the *object* of my instance, not a second instance,
and `109` built its own. No double-count.

**L22** (no standalone `Sized` form at the packed end). Faithful, and it is the clause I most wanted
checked. It carries the count at **three**, names the family discount, names the `Copy + 'static`
bound as a proof rather than a sample, and carries the widening to `W any where W mod 8 != 0` as a
proof carrying no sweep. This is my correction applied against a finding of my own that the
correction weakens, which is the discount being applied where it was inconvenient rather than where
it was convenient.

**L23** (types as degenerate lenses). Faithful, and it records my concession **as a concession**:
"`159` adopted it outright ... withdrawing O-B's cost clause". The repaired degeneracy condition is
correctly marked ONE EXPERT rather than folded into the CONVERGED half. I sign the repair as
`160`'s, not mine.

**L24** (arity is a property of the target's addressing). Faithful.

**X1** (O-A against O-B). Faithful to F159-3, including "declined to argue either branch, which is
the correct posture", which is the part I would have expected a compression to lose.

**X2** (O-C's residue). Faithful, **and this is the one the compression was most likely to flatten**.
A compression that recorded my concession and lost the narrowing would have lost the information;
`161` carries the narrowing in my own terms ("enough about the wall and not enough about the shape
at the wall") and carries the storability/aliasing/lifetime discriminator with it. Signed without
amendment.

**R2, R3, R4, R8, R13, R14.** All six concern my work, all six are accurate, and R4 correctly
attributes the withdrawal of F11, F12 and F13 to me rather than to `157` or `110`.

**Section 7's option pass.** Faithful. O-A and O-B carry their branch conditions, O-C and O-E carry
none, which is F159-3's mapping exactly.

**C3** ("N bench crates agree" is worth much less than N). Faithful, and correctly generalised past
the finding it came from.

**The discount's symmetry, which the brief asked me to test.** `grep -n 'crates\b'` over the
candidate returns four hits outside the test-gate block: the ledger preamble applying the discount,
and C3 carrying it. No other L-entry counts bench crates as instances, so there was nowhere else the
discount was owed and skipped. The claim that it was applied "throughout" is true because
"throughout" is two places.

---

## 7. What it should carry and does not

**One thing, and it is small.** `154` section 1's three-senses finding is carried at O-E in the
options pass and nowhere in the ledger. It is not an option: it is a finding about the vocabulary,
un-refuted through both sittings, and `161`'s own O-E entry says so ("stands un-refuted through both
sittings"). An un-refuted finding living only inside a live option is the shape this panel loses
things in, because when the option closes the finding goes with it. **It wants a ledger entry at
ONE EXPERT**, separate from the option that currently houses it.

## 8. What it carries that should go

**Nothing.** I looked for material of mine that survived past its evidence and found none; the four
retirements touching my work all fire in the right direction, and R4 retires three of my own
findings that I had already withdrawn.

---

## 9. Coverage, bounded

**Read in full:** `161` sections 0, 1.1 (L3), 1.3, 1.4, 2, 3, 4, 7, 8, 9; `109` section 11 and its
I15 occurrences; `159` and `154` (mine); `161_probes/anchor_accounting/dropped_anchors.txt` in full.

**Read by grep, not in full:** `161` sections 1.2, 1.5, 1.6, 5, 6; `160`; `157` and `158` beyond
what I read for `159`.

**Not opened:** `161_probes/citecheck.out`, `161_probes/anchor_accounting/count_anchors.py`, `110`,
`111`, `112`, `114` this dispatch. So I have **not** verified `161`'s anchor accounting by re-running
its instrument; I verified the dropped list against my own three anchors only, and my section 4 rests
on that list being complete, which I did not check. If `count_anchors.py` under-reports drops, more
of mine may be missing than three.

**What would move if I am wrong.** Section 1's refusal rests on `109` section 11 grounding its modal
in I13 and not in I15; I opened it and grepped every I15 occurrence in that file, and if `109` makes
the entailment argument somewhere I did not look, the rung stands as written and my refusal is the
error. Section 2's clause-6 reading rests on the carrier being part of "the realisation" as clause 5
defines it; if the candidate means by "realisation" only the map `R` and not the placement, clause 6
is unconditional and only clause 2 is unmarked, which would make the count two rather than three and
leave the defect real but smaller.

**What I settled:** that the conditionality reaches three clauses, and that L15's rung on my own row
is one expert.

**What I moved:** L20's rung from resting on another file's premise to resting on its own instrument.

**What I could not:** I could not verify the anchor accounting itself, and I say so rather than
signing section 8 on trust. That is the check the next reader of this candidate should run, and it is
the one my signature does not cover.
