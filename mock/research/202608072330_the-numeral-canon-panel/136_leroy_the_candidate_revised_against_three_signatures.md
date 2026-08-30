# 136. The candidate revised against three signatures

**Member:** Leroy, resumed. Read for this file: `133`, `134`, `135` in full with `133_probes/s1` and
`s2` and `135_probes/z1` and their outputs; `132` in full, which is my own; and greps and direct reads
into `125`, `122` and `122_probes/u0` wherever a claim below needed a citation rather than a memory.
Nothing else new. Probes in `136_probes/`, each committed with its output as it ran.

**`132` stays exactly as landed.** The three signatures cite it by line and this file supersedes it by
naming what changes rather than by editing it. Section 0 below is the list.

## 0. Gates

### 0.1 Canon gate: passed

Nothing here proposes a design decision. Every correction moves a sentence toward what its cited source
says, or moves a predicate toward the region its argument was established in, which is `INTENTS.md`
I13's discipline applied to a compression of work I did not do. The strategy set is not presumed closed
at four anywhere in this file, and nothing reasons from the removed crate tree.

### 0.2 Test gate: passed, at 123 across 13, inherited from an artifact I opened

**The attribution in `132` 0.2 and `131` 0.2 was wrong and it was mine.** Both say `125` section 10
records the eleventh run at 123 across 13. It records the opposite. I opened `125:463-477` for this
file rather than cite it from a brief again, and what it records is **twelve of the thirteen `*-shared`
crates green at 108 tests, with the thirteenth, `bitpack-write-contend-shared`, not completing**: 88
CPU-minutes in its author's run without a single test result, a sibling process of the same binary above
fifteen CPU-hours, a static count of 16 `#[test]` functions in the unterminated crate, and the brief's
123 left explicitly "as a discrepancy rather than reconciled by guesswork".

The completed count belongs to `122` section 0.2 and its `u0_test_gate_run.txt`, which is my own
artifact from the preceding topic and which I opened rather than remembered. It ends `TOTAL PASSED:
123`. The file carries fourteen `test result: ok` lines summing to 124 passes, which reconciles because
the fourteenth is the single-test rerun appended at the tail for `121` section 3.4's evidence claim.
That is the count this file inherits, from that file, by that command.

**The propagation is wider than `133` reported.** `133` names `131` and `132`. `x3` Q4d finds the wrong
attribution in **five places across four files**: `131:48`, `131:52`, `132:43`, `134:13` and `135:10`.
Both cosigners inherited it from my files and repeated it in their own gate lines while checking my work
carefully in every other respect, which is what a laundered citation does. `134` and `135` are not at
fault for it and I say so plainly: they were reading a citation that named a file and a section.

**What the inheritance chain should carry.** Not the completed 123 alone. `127` and everything after it
inherited a gate state in which one contention crate does not terminate under concurrent load, and a
count that silently absorbs an unterminated crate is a false green wearing attribution's clothes. The
non-termination travels with the number.

## 1. What this supersedes in `132`, and what stands

**Superseded.** Section 0.2's test-gate attribution (section 0.2 above). Section 5.2's monotonicity
sentence (section 3.1). Section 5.3's headline exclusivity sentence (section 3.2). Section 5.4's first
predicate's domain dimension and the missing representative-level qualifier (sections 4.1 and 6.1).
Section 5.6's uniqueness predicate's domain dimension and its dead `125` T9 citation (sections 4.2 and
2.2). Section 5.8's unquantified "only member that does" (section 6.2). Section 6.2's provenance
wording for the withdrawn figure (section 2.1). Section 1.3's F9 entry, which drops a credit (section
4.4).

**Stands, cosigned by at least one builder and unchallenged by the others.** `127` F127-1's
shared-threshold construction and F127-2 with `128` F128-5's tie correction, `128` F128-4, and the whole
of section 1's ledger including 1.1's statement of what commit ordering does and does not establish, sections 2, 3, 4,
5.1, 5.5, 5.7's substance, 6.1, 6.3, 7, and section 9's method. `134` cosigns the candidate in full.
`133` and `135` each cosign every section not listed above.

**Added, not superseded.** `132` 6.1's double-rounding split (`131` F131-4 and F131-5, on
`131_probes/v2`) and 6.2's
withdrawn figure (`131` F131-3) both stand as `132` states them, and `131` F131-6's vocabulary count is
unchallenged by any signature. Four predicates carry no domain dimension at all, which no signature
reported and which is a more severe defect than the two that were reported (section 5). `134`'s
addendum adds a dimension to the position-keyed arm (section 7). And the widening `135` asks for
carries a hypothesis nobody stated (section 4.3).

## 2. The two errors that are mine, stated precisely

### 2.1 The figure was fabricated in a report message, and I relayed it

`132` 6.2 says the count of 21,204 of 32,768 "originated in `125`". **That wording is wrong in the way
that matters to a reader who greps**, and `133` is right to refuse the candidate's generosity. `x3` Q3
greps `125` and both its probe directories for the figure and returns nothing. It originated in
**`125`'s author's end-of-dispatch report message, outside any committed artifact**, in the one channel
`evidence-lives-in-the-repo-or-it-never-happened.md` does not reach. `125`'s committed record was
correct throughout: 64, 112 and 124 of 256 at `F` in `{1, 3, 5}`, `W = 8`, in `125` section 10 and
`125_probes/p5_output.txt`.

**The relay is mine.** I carried the number into two briefs without opening the artifact, and it then
appeared in `131` and `132` as a figure attributed to a file that refutes it. The split of ownership is
`133`'s and I record it as it stated it: the fabrication is its author's, in prose; the relay is mine.

**And the figure is arithmetically impossible for the sweep it claims, by a second route.** `133`'s `s1`
searched every width to `2^60` for a representation `2^b(2^k - 1)` and found none. `x3` pins the width
from the figure's own denominator instead. If 32,768 names the swept domain then `W = 15`, and the
complete set of counts the sweep can report is `{2^14 - 2^(14-F) : F = 0..14}`, whose maximum is 16,383.
**21,204 exceeds the largest count the sweep can produce**, so it is not merely absent from the set. It
also fails under the other reading, where 32,768 names a signed half-range and `W = 16`. `x3` Q1
reproduces the closed form `2^(W-1) - 2^(W-1-F)` at eighteen cells with an off-by-one control failing at
every one, and its `W = 8` row returns 64, 112 and 124, which is `125`'s committed record reproduced
independently.

**The candidate's wording should read**: originated in `125`'s author's report message, outside any
committed artifact, and relayed into two briefs by the coordinator without opening the file.

### 2.2 The dead label, and where the anchor should point

`125:116` reads "Its compensation is T9 in section 6" and `125` defines no T9. `x3` Q6 confirms one
reference and zero definitions, against a file that introduces every other theorem as `**Tn (...)**`.
The label propagated into my brief and into `132:392`, where it is cited for the expectation-recovery
sentence. **The correct anchor is `125` F8's first clause**, and the content survives the concession
that retired F8's other half.

`132:572`'s count of nine theorem labels stands, because `125` carries `T1b`. "T1 through T9" as a range
does not, and it was my brief's phrasing.

## 3. `133`'s dissents, reproduced before conceding

`136_probes/x1` rebuilds both on a fixed-point instrument rather than `133`'s abstract cell model, and
part A over several cells at once rather than one, because a per-cell enumeration leaves the across-cell
direction unchecked while the candidate's sentence quantifies over the whole map.

### 3.1 D2: the monotonicity quantifier is false, and the replacement is sharper

**Conceded.** `132` 5.2 says "every deterministic member is order-preserving". A deterministic
quantisation in that sentence's own sense is a grid-fixing retraction, and `125_probes/p1`'s parity mode
is a committed retraction with 1750 measured inversions. `x1` A2 reproduces it on my instrument: the
parity keying is neither monotone nor a suffix rule.

`x1` A1 confirms the characterisation across 64, 512 and 16,384 enumerated retractions at three grid
shapes: the globally monotone maps are exactly those whose every cell is a suffix rule, agreeing on
every enumerated map, with the count matching `(subpoints + 1)^cells`.

**And one prediction of mine was refuted, which improved the result.** I predicted a monotone non-local
grid-fixing retraction exists, so that locality would have to be carried as a hypothesis. `x1` A3 found
none, and the reason is one line: a grid-fixing map pins `k` and `k+1`, so a subpoint strictly between
them must land in `[k, k+1]` to stay monotone, which is locality. **Monotonicity implies locality**, so
the characterisation holds over every grid-fixing retraction rather than only the local ones. That is
wider than `133` established and wider than I expected, and I keep the refuted prediction on the record
rather than repair it.

**The replacement, which is `133`'s with the widening folded in:**

> Every member of the threshold family, which includes every standard mode, is order-preserving, and
> the threshold shape is not incidental: a deterministic quantisation is order-preserving exactly when
> it is a per-cell threshold rule. So monotonicity is a structure the admitted members share rather
> than a property determinism grants, and the deterministic monotone members are exactly the
> realisations of the realisation-monotone stochastic family, which is 5.6's up-set argument seen from
> the other side.

**The predicate splits**, as `133` asks. The additive half needs `operation = addition`; the
monotonicity half carries no operation dimension, because a characterisation of a map is not a claim
about an operation.

### 3.2 D3: the exclusivity sentence is false against its own bullets

**Conceded, and it is worse than a compression error: the sentence contradicts the three bullets
directly beneath it in the same clause.** `132` 5.3's headline says no member carries more than one of
the first three, and its own second bullet then says exact composition is "carried by the directed
members", which includes both adjoints, each of which the first bullet has just given the order bound.
No probe was needed to see that, and I did not see it.

`x1` part B measures it at `W = 9`, `F = 4`, staged narrowing 4 to 2 to 0: floor and ceil each carry the
order bound and exact composition; toward-zero carries composition and negation symmetry; half-even
carries negation symmetry, which 5.3 gives to toward-zero alone; half-up carries none, which is the
control on my law encodings. The true exclusivity holds: **no member carries the order bound and
negation symmetry together**, because negation exchanges the two adjoints.

**Provenance.** `x3` Q5 greps: the sentence appears in neither `125` nor `126`, and appears at
`131:428` and `132:322`. Both are mine. It was invented at the formalisation step and compressed
forward, which is the failure a signature check exists to catch and which no amount of care inside
`132` could have caught, because `132`'s job was to compress `131` faithfully and it did.

**The replacement is `133`'s, adopted verbatim** as section 7.2 of this file's statement.

## 4. `135`'s corrections, reproduced, and what the reproduction adds

### 4.1 The domain predicate on 5.6 is inherited, and 5.4's is too

**Conceded on 5.6, and extended to 5.4, which `135` flagged as a further question and could not check.**

`136_probes/x2` reproduces the sign-blindness of the coupling argument on an instrument that
parametrises by the cell index `k` explicitly and sweeps nine cells of both signs, rather than computing
one value and asserting in prose that it applies to both. The uniqueness solve returns one distinct
threshold distribution across every `k` at `m` in `{5, 8}`; the variance closed forms return one
distinct pair across every `k` at `n` in `{5, 10}`.

**5.4's `domain closed under negation` contradicts its own source and its own prose.** `125` T8 says, in
its own words at `125:287`, "This holds on one-signed domains too, where the corrected overflow theorem
lets saturation hold both properties". `132` 5.4's body repeats that, writing "one-signed domains
included" in the sentence about T8, and its predicate then excludes exactly that region. The predicate
and the prose in one clause disagree, and the predicate is the part a reader can gate on. This is a
check against what a theorem states rather than a measurement, which is the right kind of check for a
claim about a theorem's scope.

### 4.2 But `135`'s premise about 5.7 is wrong, and the real defect there is worse

`135` writes that 5.6 and 5.7 both carry `domain closed under negation`. **5.7 carries no domain
dimension at all**, and neither does 5.6's second predicate. `136_probes/x4` extracts all eleven
predicates in `132` section 5 and tabulates the dimension: present in seven, absent in four.

Under I13 an absent dimension is not a hedge and not a narrowing. It is the strongest negative statement
the notation has: the claim holds in no situation where that dimension is present. For a claim about
quantising numbers a domain is always present, so those four clauses are **vacuous as written**, which
is precisely the class `133` names for the three unclassifiable pins and which nobody noticed had
reached the candidate's own statement. Section 5 lists them.

### 4.3 The widening `135` asks for carries a hypothesis nobody stated

`135`'s `z1` carries no negative control. Its parts one and two compute a single value and assert in
prose that it applies to both signs; its part three compares two counts that both come out 1 with
nothing to show the comparison could have come out otherwise. The structural reason it gives is sound
and the conclusion is right, and a sweep that cannot report a difference is still not evidence that
there is none. `x2` P2 and P3 supply controls that fire.

**And the widening is conditional.** The construction is sign-blind because `frac(x) = x - floor(x)`
lands in `[0, 1)` for negative `x` as well as positive. Under the other common convention,
`frac(x) = x - trunc(x)`, which lands in `(-1, 0]` for negative `x`, `x2` P4 finds the same construction
sign-dependent at both `m = 5` and `m = 8`, with the negative cell producing a distribution containing a
negative mass. So the honest form is not bare `domain any`. It is `domain any` **given a floor-based
cell coordinate**, which is the convention every probe in this topic uses and the same convention `125`
F9 identifies the bit-drop operation with. That is a condition on the construction, so it belongs in the
clause rather than in the predicate's domain dimension, and it is stated in section 7.4.

### 4.4 The two smaller corrections, both accepted

**The rung's wording.** `132:152` reads "the second author reached the closed form without the first's".
`135` reports it read `128`'s stated results before building its proofs, and asks that the sentence be
read as *without reproducing the first's method*. Accepted, and the sentence should be written that way
rather than left to be read that way: **the second author verified the first's claim by an independent
method and widened it, having read the claim before deriving its proof.** The rung placement is
unchanged and `135` does not ask for a different one.

**The missing credit.** `132:130` reads "reproduced by `131` on a third instrument" for `125` F9 and
the bit-drop identity, which implies a second that the ledger does not name. It is `135`'s own, in `127`, before `128`, `129` or `130` were
dispatched: it opened `118_probes/q3` and `q5` at source and confirmed the toward-zero identification
independently of `125`'s claim, at a point where the vocabulary question was still open. Accepted. The
entry should name three instruments in order: `125`'s own probe, `135`'s source-level confirmation in
`127`, and `131`'s sweep.

## 5. The predicate sweep, per clause

`136_probes/x4` extracts every predicate in `132` section 5 and reports two things a reader cannot get
by reading: which dimensions match an earlier clause's verbatim, and which are absent. The first is the
inheritance fingerprint, **necessary and not sufficient**, so every flag is a candidate for a hand check
rather than a finding. The second cannot be found by matching at all.

**Eleven predicates, eight flags, all eight checked by hand below.**

| clause | predicate | domain dimension | verdict |
|---|---|---|---|
| 5.2 | the obstruction | `in {closed under negation, one-signed}`, each proved separately | **established**, and `133` confirms the widened quantifier is licensed by T1/T1b's own scope rather than by a sweep |
| 5.3 | the adjunction | `closed under negation` | **established**: the adjoint pair is defined against a negation-closed order |
| 5.3 | the composition failure | `= rationals over the swept window closed under negation` | **established**: it is an existence result on a named sweep |
| 5.4 | the commutations | `closed under negation` | **INHERITED**, and contradicts `125:287` and 5.4's own prose. Section 4.1 |
| 5.4 | the non-commutation | **absent** | **VACUOUS as written.** `signedness` is present and does not stand in for it |
| 5.5 | the vacuity | `any` | **established**: `133` cosigns it as exactly as wide as the closure argument and no wider |
| 5.5 | the valuation | `any` | **established**, same argument, and the match with its neighbour is the fingerprint's expected false positive |
| 5.6 | the uniqueness | `closed under negation` | **INHERITED**, from 5.2 through 5.4 rather than from its neighbour. Sections 4.1 and 4.3 |
| 5.6 | the variance law | **absent** | **VACUOUS as written**, and `W`, `F` and `signedness` are absent too |
| 5.7 | the keying | **absent** | **VACUOUS as written.** `135` believed this one over-narrow; it is not narrow, it is empty |
| 5.8 | the entropy disjunction | **absent** | **VACUOUS as written**, and `W`, `F` and `signedness` are absent too |

**Touched: eight of eleven.** Three inheritances corrected (5.4's commutations, 5.6's uniqueness, and
5.5's flagged pair found sound), four absences named, and the eighth is 5.6's uniqueness counted once
though the fingerprint flagged it from two sources.

**What I can and cannot repair.** The two inheritances have established replacements and section 7 gives
them. **The four absences I name as open obligations rather than fill**, because I have not established
what value the dimension takes in three of them: `x2` covers 5.6's variance law, and nothing in this
topic measures 5.7's keying divergence or 5.8's entropy claim across domain shapes. `135`'s `z1` part
three compares the keying divergence at `k = 0` and `k = -1` and gets 1 both times, which is one
uncontrolled comparison, so 5.7's widening is indicated and not established. Filling a predicate with a
value nobody measured is exactly what the notation exists to prevent.

**And the class is now six instances across two topics, in both directions.** The preceding topic
omitted the dimension from nine predicates because its neighbours did not carry it; this topic inherits
it into two and omits it from four. One mechanism: **a predicate's dimensions read off the clause above
rather than off the argument underneath.** That is a class, not six accidents, and the mechanical part of
it is now an instrument that runs in seconds.

## 6. `133`'s two amendments, both accepted

### 6.1 A1, the level at which the commutation equality is read

`132` 5.4 says the composition order is unobservable for every pairing but one, and drops which equality
is meant. `133`'s corrected F7 holds **in the quotient**, which is where a wrapping policy's values live
and which makes the candidate's sentence true for the realisation map as a whole. At the representative
level, before the final range reduction, **floor alone commutes**, measured at zero against 60, 32 and 32
for ceil, half-up and half-even. A design that ever observes an intermediate representative needs that,
and it is an established result the compression dropped. The clause in section 7.3 carries it.

`133` also corrects an instrument citation: T8 is carried by the closure argument with `125_probes/p2`
and `125_probes/p3` as controls, not by `125_probes/p3` alone as `132` 5.4 writes it. Accepted.

### 6.2 A2, the dropped quantifier in 5.8

`131:401` reads "among members that decorrelate, the position-keyed deterministic dither is the only one
that survives I14 without the consumer supplying entropy". `132:437` renders it "it is the only member
that does". Unqualified it is false twice: every deterministic mode trivially needs no entropy, and the
position-keyed member does not escape the disjunction, it **takes the const horn of it and decorrelates
anyway**, which is the whole content. Accepted, and both halves are restored in section 7.5.

### 6.3 The widening notes, carried

Both of `133`'s notes on `131` F131-2's 38-of-55 application, measured on `131_probes/v1`, go forward to
whoever consolidates. First, the 14
non-widening pins do not widen **as stated**, and F6's valuation predicate splits each into a
mode-invariant arm on operand pairs whose valuations sum to at least `F`, and a residue, if a pin's owner
wants a partial widening. That is per-pin repair under the never-widen-in-place rule, not a defect in the
classification. Second, the three unclassifiable pins are **vacuous rather than merely unclassifiable**
under I13 read literally, since a predicate with no operation dimension claims nothing anywhere an
operation is present, which for an arithmetic pin is everywhere. `132` 6.3's "want repairing" is right
and understated.

`133` also cosigns F131-2 as sound against the argument that licensed it (`125` F4's closure widened,
`131` F131-1): every one of the 14
fails for one of the three reasons its closure argument refuses, no pin fails for a reason outside it,
and the 55-against-35 difference is scope rather than disagreement.

## 7. The clauses that change, stated

Only the clauses this file supersedes are restated. Every other clause of `132` section 5 stands as
written there.

### 7.1 Replacing 5.2's second sentence

> **Every member of the threshold family, which includes every standard mode, is order-preserving, and
> the threshold shape is not incidental**: a deterministic quantisation is order-preserving exactly when
> it is a per-cell threshold rule (`125` T2 and F2 for the standard members, on `125_probes/p1`;
> `133_probes/s2` and `136_probes/x1` for the characterisation, both directions). Monotonicity is a
> structure the admitted members share rather than a property determinism grants, and the monotone
> deterministic members are exactly the realisations of 5.6's realisation-monotone family.

*holds for: W any; F any; signedness any; domain in {closed under negation, one-signed}; rounding = every
grid-fixing retraction; threads any; target features any. **Argument kind: characterisation**, both
directions exhaustive over every retraction at three grid resolutions, and monotonicity implies locality
rather than assuming it.*

The additive half of 5.2 keeps its own predicate unchanged, including `operation = addition`, and its
anchors (`125` T1, T1b and F1) are unaffected.

### 7.2 Replacing 5.3's headline, `133`'s wording adopted

> Four exact laws are available, each carried by a proper subclass, and **the subclasses overlap**: the
> two adjoints carry the order bound and compose exactly across precisions; the toward-zero member
> composes exactly and carries negation symmetry, at the price of respecting no translation; the
> even-tie nearest member carries negation symmetry and the optimal error bound, with neither exact
> composition nor a one-sided bound; and the additive law in expectation is carried only by leaving the
> category of functions. **No member carries the order bound and negation symmetry at once**, because
> negation exchanges the two adjoints, and no member carries all of the first three. (`125` F3 with T3,
> T4 and T5 for the first three, measured on `125_probes/p4` and `125_probes/p5`; `125` F8's surviving
> half for the fourth.)

*The overlap and the exclusivity hold for: W in {9}; F in {4}; signedness = signed; domain closed under
negation; rounding in the five standard members; staged narrowing F_exact = 4, F_intermediate = 2,
F_final = 0; threads = 1. **Argument kind: enumeration**, on `133_probes/s2` and `136_probes/x1`
independently. The exclusivity's reason is structural and its measurement is not: the adjoint exchange
argument carries at every width and the table does not.*

### 7.3 Replacing 5.4's predicate and adding A1

The body gains, after "unobservable for every pairing but one":

> the equality read **after the final range reduction**, which is where a wrapping policy's values live.
> Before that reduction the floor member alone commutes at the representative level, and a design that
> observes intermediate representatives carries that fact (`125` P3's corrected second pass).

The decomposition itself is `125` F7, with `125` F5's division residue as the operation where the axis
persists at zero fraction width, both unchanged from `132` 5.4.

*The family invariance holds for: W any; F any; signedness any; **domain any**, one-signed included per
`125` T8 at `125:287`; rounding = every deterministic member; threads any. **Argument kind: closure.***

*The commutations hold for: W >= 1; F any; signedness any; domain closed under negation; every
deterministic member against saturation, and every translation-equivariant member against wrapping;
equality read in the quotient; threads any. **Argument kind: equivariance.***

*The non-commutation holds for: rounding = toward-zero; range policy = wrapping; signedness = signed, or
unsigned with signed intermediates; **domain: OPEN, and the clause claims nothing until it is stated**;
threads = 1. **Argument kind: existence.***

The family invariance is separated from the commutations because `125` T8 and `125` T6/T7 are different
theorems with different scopes, and one predicate over both is what carried the wrong region.

### 7.4 Replacing 5.6's uniqueness predicate and its dead citation

The dead `125` T9 citation becomes **`125` F8, first clause**.

The uniqueness is `128` F128-1 on `128_probes/r2`, with `128` F128-2's Frechet impossibility underneath
it on the same instrument and `128_probes/r3` for the per-cell member, all unchanged from `132` 5.6.

*The uniqueness and the impossibility hold for: F any; W any; signedness any; **domain any**, established
by the construction reading only `frac(x)` and never the cell index, given a **floor-based cell
coordinate**; coupling = any point of the Fréchet interval; threads any. **Argument kind: uniqueness**,
with the system shown invertible at every cell resolution, and the domain dimension established on
`135_probes/z1` and `136_probes/x2` with a control that fires on a deliberately sign-dependent variant.
Under a truncating cell coordinate the construction is sign-dependent and this predicate does not
transfer (`136_probes/x2` P4).*

*The variance law (`128` F128-3 on `128_probes/r3`, re-derived and widened by `130` on `130_probes/y1`)
holds for: element count any; fraction = 1/3;
coupling in {comonotone, independent}; **domain any**, same construction and same condition
(`136_probes/x2` P3); threads any; **W, F and
signedness: OPEN**. **Argument kind: induction.** The fraction is a fixed value and is not widened.*

### 7.5 Replacing 5.8's last sentence

> **Among the members that decorrelate** (`129` F129-1 for what decorrelation an arm actually wants,
> `130` F130-1 for the keying that splits them), the position-keyed member (`131` F131-7) is the only one
> that requires no randomness at all. It does not escape the disjunction: it **takes the const horn and decorrelates
> anyway**, which is what makes it remarkable. Every deterministic mode trivially needs no entropy and
> decorrelates nothing.

*holds for: everything `132` 5.8's predicate lists, unchanged, plus **keying axis = one-dimensional**
(section 8), plus **domain, W, F and signedness: OPEN**.*

## 8. `134`'s addendum, taken as a dimension

`134` cosigns `132` in full and adds a bound on what `131_probes/v3` establishes. It confirms the
construction is the technique it was meant to be, a position-keyed golden-ratio ordered dither in
fixed-point integer arithmetic with no float and no entropy, checked by `const _: () = assert!(...)` so a
wrong result is a build failure, and it corrects `129_probes/x1`'s constant by eight against `v3`'s
`2_654_435_769`, which is `floor(2^32 / phi)` exactly.

**The bound: the distinct-output count is necessary and not sufficient for what dithering is chosen
for.** The literature optimises for spectral shape, not non-repetition, because the eye and the ear are
insensitive to high-frequency spectrally flat noise and sensitive to any low-frequency or periodic
structure however small. A one-dimensional low-discrepancy sequence is well matched along a genuinely
one-dimensional axis, a sample index or a monotone counter. Applied to a row-major flattening of a
two-dimensional image it can produce visible diagonal or periodic structure that a count of two distinct
outputs at forty positions cannot see.

**So the arm's predicate gains a dimension**: `keying axis = one-dimensional`. The two-dimensional case
is **unmeasured**, so under I13 it goes unstated and the arm claims nothing there, which is the correct
severity: a consumer flattening a 2D index is outside the region. Whether "position" means a 1D axis or
a flattened 2D one is a fact about the consumer rather than about arvo, and a genuinely 2D-aware
construction is buildable and unbuilt.

This is the third thing this topic has established by naming what a measurement does not reach, after
`131`'s reading of the double-rounding hazard and `130`'s bounded-to-the-sweep tag, and it is the pattern
worth carrying: **a measurement's predicate is the boundary of what it licenses, and the useful
contribution is often the dimension nobody listed.**

## 9. The three defects in my own probes, recorded rather than repaired quietly

All three are the class the test gate names, and all three were caught by a control whose required
outcome I had written down before running. None would have been caught by reading the code.

**`x1` part B swept a window that could not enter the failing path.** The first run at `W = 7`, `F = 3`
with a 3-to-2-to-1 staging reported half-even carrying exact staged composition, contradicting both
`133`'s `s2` and `125` section 10's P4, which measured 500 half-even mismatches of 4001. The window held
no value whose intermediate stage lands on a tie. Widened to `W = 9`, `F = 4`, 4-to-2-to-0, half-even
fails at `-247/16`, and the probe now prints the first failing value per member so a True cannot mean an
empty sweep.

**`x2`'s two controls were structurally incapable of firing, in the probe written to criticise a probe
with no control.** The threshold control reversed the subpoint list for `k < 0` and then sorted that list
by value, making the reversal a no-op. The variance control mapped `f` to `1 - f`, and `f(1 - f)` is
invariant under exactly that map. Both reported no difference and both would have reported no difference
under any hypothesis, at any width, forever. Replaced with perturbations that change the quantity being
compared, after which both fire.

**`x4`'s extractor was wrong twice and then lied about it.** Its dimension key pattern matched the `in`
inside the word `domain`, so every domain dimension parsed as a key named `doma` and the sweep reported
zero domain flags, a confident False on the exact question the probe existed to answer. That is a
delimiter occurring in the content. Worse, the verdict block printed `True` for the control because I
had written the literal rather than the variable, so the instrument reported itself sound while failing.
Then the fixed version still missed a predicate, because the span pattern required the literal `holds
for:` and 5.6's uniqueness predicate reads `hold for:` with a plural subject, so it found nine
predicates where there are eleven, and **the missing one was the single predicate `135`'s dissent is
about.** And the inheritance fingerprint compared only against the immediately preceding clause, which
cannot flag 5.6, whose neighbour carries `domain any` while the inheritance comes from three clauses up.

**Four failure modes, one instrument, one sitting.** A pattern written against the shape I expected
rather than the shape on the page; a verdict printing a literal; a heuristic scoped to adjacency; and a
matching heuristic applied to a question about absence, which it is blind to by construction. The
absence table exists because of the fourth, and it found the four vacuous predicates that no signature
reported.

## 10. What only op decides

Nothing in this file is a design decision and none of it is settled. What is genuinely his:

**The vocabulary contest (C1 in `132`), unchanged.** Two builders and one signature have positions and
none is ratified.

**Whether a canon states the four open domain dimensions as open, or waits until they are measured.**
Section 5 leaves them open on the principle that a value nobody measured must not be written into a
predicate. A canon that carries visibly open obligations is unusual, and whether that is the right shape
for a canon rather than for an audit trail is his call, not mine.

**Whether the position-keyed arm ships at all given `134`'s bound**, since the answer depends on what
"position" means in a consumer and arvo does not know its consumers' access patterns.

**Whether the non-terminating contention crate is a gate defect to fix or a measurement to record.**
Six files have now inherited a gate count through it, five of them citing the wrong file for it.

And the standing one: nothing here moves to `mock/canon/`.

## 11. Anchor accounting

Counted on `132_probes/w1`, which extends `119_probes/r1` with the theorem and probe-stem classes, with
this section excluded from the computation. The union is the five preceding files of this topic plus the
three signatures.

```
  class            in the eleven-file union   in 136   not carried
  finding ids                            28       25             3
  probe stems                            17       16             1
  theorems                                9        9             0
  line anchors (panel)                   17       19            13
```

**Every entry in the three non-zero `not carried` cells belongs to the preceding topic**: `F122-2`,
`F122-4` and `F122-5` from the realisation-map candidate, and probe stem `q5` from `118`. **Nothing from
this topic's eleven files is dropped**, including the eight signature files' own anchors and all nine of
`125`'s theorem labels.

The line-anchor row is the one that needs reading rather than counting. Thirteen union anchors are not
carried and nine of those are not anchors this file could carry: five are `INTENTS.md` line references
from other members' gate sections, three are commit hashes from `132`'s own blindness table, and one is
a `118_probes` path from the preceding topic. The remainder are `125:4`, `125:326`, `125:455-477`,
`126:511` and `128:177`, each belonging to a clause this file leaves standing. This file carries
nineteen of its own, more than the sixteen the union holds, because a revision opens files at lines a
compression did not.

**The first run of this accounting carried five findings of twenty-eight.** Restoring the rest was a
second pass at the points of use rather than in a list, which is now the third consecutive candidate to
need one. That is a fact about how a revision gets written, not about carelessness: the prose is drafted
against what changed, and an anchor belongs to the claim rather than to the change.

**The stripper's two runs are both worth stating.** On the run before this paragraph existed it **did
not fire**, correctly, because this section named no anchors and so introduced none. Naming the four
dropped anchors above to account for them makes all four present in this file, so the unstripped count
would read 28 findings of 28 and 17 probe stems of 17: a clean sheet produced entirely by the sentences
admitting it is not clean. The stripped run is the one in the table and the committed output carries
both, reporting `finding +3, line +5`. One anchor is under-reported in the conservative direction and I
state it rather than let it pass: `q5` appears above as a bare stem and the probe's pattern wants a
filename or a `NNN_probes/` prefix, so the stripper shows no probe-stem delta where it should show one.
The effect is to leave `q5` counted as not carried, which it is. That loop is the guard's whole purpose, and it fired on `132`'s accounting section for the same
reason.

**A line anchor into shipped source is not carried and none is owed**, because the code tier is the one
that gets rewritten. Panel-internal anchors, finding ids, theorem labels and probe references are carried
wherever a clause rests on them, which for a revision means wherever a clause changes; a clause that
stands is cited by section number rather than by repeating `132`'s own anchors, and the difference is
counted rather than assumed.

## 12. Coverage, bounded

**Read in full:** `133`, `134`, `135`, and `132` which is mine. `133_probes/s2` source and both outputs;
`135_probes/z1` output and its source at the parts my `x2` reproduces.

**Opened at specific lines rather than recalled:** `125:116`, `125:283-291`, `125:348-352`,
`125:463-477`; `122:53-55` and `122_probes/u0_test_gate_run.txt` in full; `131:401`, `131:428`;
`132:130`, `132:152`, `132:322`, `132:392`, `132:437`, and 5.4 and 5.6 in full.

**Not read:** `126`, `127`, `128`, `129`, `130` beyond the greps `x3` runs into them; `133_probes/s1`
source, whose conclusion I reproduced by a different route rather than checked; `125_probes` through
`131_probes` sources; `OPTIONS.md`, `AGREEMENTS.md`, `DROPLIST.md`, `RULES.md`.

**Built:** four probes, four outputs, plus the anchor run on `132_probes/w1`'s patterns via
`136_probes/w1_over_136`, which edits neither `132_probes/w1` nor `119_probes/r1`. Predictions confirmed: `x1` A1 and A2 and B1
through B5, `x2` P1 through P4, `x3` Q1 through Q6, `x4` R1 and R3. **Refuted: `x1` A3**, and the
refutation widens the result. **Defects found in my own instruments and recorded: four**, in section 9.

**Not done.** No measurement of anything, so nothing here prices anything. No re-derivation of `131`'s
pin classification. No check of 5.7's or 5.8's domain dimension, which is why both are left open rather
than widened. No independent check of `134`'s spectral claim, which is a statement about a literature
rather than about a measurement and which I take on its author's expertise while noting I did not verify
it.
