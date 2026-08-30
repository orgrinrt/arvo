# 146. The canon candidate for the strategy object

**A draft for signature, not a finished statement.** `139`, `140`, `141` and `144` are resumed after
this to sign, dissent, or sign in part; an agent outside the topic then checks the compression.
Nothing moves to `mock/canon/`. Op ratifies.

**Member:** Leroy, who formalised this topic at `145` and took no part in `139` through `144`.
**Read for this file:** the seven topic files in full, `INTENTS.md` in full, `OPTIONS.md:2425-2461`,
and the commit ordering. **Built:** one probe, `146_probes/w2`, committed with its output.

---

## 0. Gates

**Canon gate: passed.** Nothing below proposes a design decision or presumes the strategy set closed
at four; I1 is demoted to open at `INTENTS.md:51-61`. Every predicate lists only what was established
and none carries a hedge token, per op's instruction at `INTENTS.md:241-246`. Section 5 states regions
and what holds in them rather than constructions, because I16 at `INTENTS.md:317-331` says the canon
does not police what shape a law takes and a candidate naming an arm's construction is that failure.

**Test gate: passed, at 123 across 13, inherited from `137_probes/g0_test_gate.out`**, which I opened
and which ends `123 passed across 13 crates, 0 failed`. The `-- --test-threads=1` requirement has four
independent reproductions (`139` at 7.97s debug, `141` at 46.65s, `143` at 2.57s, `144` at 3.03s
release) against `139`'s livelock diagnosis in `pool.rs`.

---

## 1. The ledger

### 1.1 The blind pair, and exactly how much the ordering carries

`139` and `140` are two cold derivations of the same brief, each declaring its blindness in its own
opening paragraph. `139:3-6` names what it read and says the file "is committed before any of that is
opened". `140:3-7` is stronger on the one point that matters here: "I checked that `139_probes/`
exists and did not open it."

**The commit ordering carries less here than in the topic before this one, and the ledger should not
lean on it.** `139`'s six probes landed between 09:51 and 10:11 and its phase one at `861f89bd` 10:17;
`140`'s phase one at `a60f1a47` 10:19, **two minutes later and after `139`'s file was already in the
tree**. So the ordering establishes the within-file half for each (predictions and probes committed
before the reconciliation) and establishes **nothing** about the between-file half, and for `140` it
runs the wrong way. Blindness here rests on the two coverage statements and on `140:6`'s specific
disclaimer, and on nothing else.

**And one shared input contaminates part of it, which `139` declared and `140` did not.** `139`'s
opening declares that this repository's `.claude/` rules load automatically and that
`arvo-always-optimal-internals.md` contains the sentence that a one-sided clamp of a monotone
operation is a congruence. `139:177-179` says of its own result that it "is exactly the sentence
sitting in the workspace rule I declared above, so it is corroboration and not discovery". `140`
records reading "the workspace rules" in the same breath, so it had the same input.

> **Wherever the one-sided-clamp congruence is the mechanism, the two cold derivations are one
> instance wearing two hats.** That reaches the unsigned half of the fusion result, the unsigned
> accumulator cells, and `140`'s own refuted P3. It does not reach anything measured rather than
> mechanised: the class-count tables, the container sweeps and the weight-cell geometry are
> independent.

**What is genuinely blind and convergent between the pair:**

| # | claim | `139` | `140` | independence |
|---|---|---|---|---|
| B1 | the two components are a specification and a selector, told apart by whether an answer moves | section 1 | section 2 | both blind, both derived from op's intents, different routes (bytes-level in `139`, I8 against I9 in `140`) |
| B2 | the storage-minimising concern is a weighting at the column | `139_probes/p6`, signed bitstream at arbitrary offsets | `140_probes/p3`, unsigned partition over rungs | both blind, disjoint models, and the two span the signedness dimension only between them |
| B3 | the count is not a shape-free number | `139_probes/p1`'s class table by shape | `140` F2 by witness set | both blind, and `141` proved they are one claim |
| B4 | named presets are vocabulary and the axes are the design's obligation | section 7 | section 2.1 | both blind, both citing I2 against I1 |

**B3's status is the interesting one and it is not a convergence in the form either file wrote it.**
`141` section 5.1 proved both are quotients of one assignment set by observational equality over an
observation set, so fixing a shape and fixing an operation set restrict the same set. It is a theorem,
`141` says so itself rather than presenting it as a discovery, and `143` added the anti-monotone
control that converts its own zero from a caveat into a checked property. So B3 is **two blind
instances of a phenomenon plus one proof that they are the same phenomenon**, which is a stronger and
differently shaped record than three agreements.

### 1.2 The convergence rescoped rather than refuted, and the instrument built without reading

B2 is the entry this topic will be read for, because of what happened to it.

**`141` reproduced it and then narrowed it.** `141_probes/p4` is a third instrument on the column result,
Rust over raw integers sharing no code with either, with the contested limit-reading pinned at the
declared width. T1 confirmed at every shape, lossy control firing at every shape.

**Then it asked what neither file had asked.** Both measured where a value is put; neither measured
how wide the arithmetic is while the value is computed. Adding an accumulator dimension gives
`141` F2: the accumulator width is answer-visible exactly at `signedness = signed, overflow =
saturating`. `141` half-predicted this and records the half that was wrong: it expected unsigned
saturating to be visible too, and it is not, by the same congruence.

> **This is not a refutation and the ledger must not record it as one.** `141:209-223` says so
> directly: both files are true of what they measured, and what is refuted is the **scope** each
> stated, because `139:244` generalises from packing to "the storage-minimising concern" and
> `140:577-579` says the concern "composes with every assignment". Neither predicate carried a
> dimension that would have narrowed it, because neither model had one.

**Two further instruments then confirmed the rescoping, and one of them was built without reading the
first.** `142`'s `q3` is a fold over a sequence, which is the shape a column kernel has, with sequence
length as a dimension `141`'s model did not have; `143`'s `p2` is a fold with three kernels, and
`143:533` records that it deliberately did not read `141_probes/p4`, which is what makes it a second
instance rather than a re-run.

**And `142` found a dimension none of the three had.** F142-6: same cell, same widths, same domain,
the accumulator is visible at 672 cases with the largest multiplier first and at 0 with it last. So
the cell is real and whether a kernel enters it is a property of the accumulation schedule. `142`
offers a conservative reading and a precise one and takes the conservative; I record both in section
5.5 rather than choosing.

### 1.3 Reproductions, which are not the rung a blind convergence earns

The topic is unusually careful about this and the ledger should preserve the care.

**`142` re-derived translation equivariance after reading `141`'s prose claim and says so**
(`142:122-126`): "a re-derivation and a re-measurement, not a blind instance, and it does not earn the
two-expert rung. Saying otherwise would be exactly the inflation the coordinator just corrected
upstream of me." Its contribution is not the mechanism but the **generalisation**: F142-2 partitions
six rounding modes three against three, where `141` had tested two.

**`142`'s `q1` reached `141`'s section 3.7 conclusion from inside the file `141` attacked**, by
reading two of `139`'s own committed probes side by side. `142:109-112` credits `141` with getting
there first and by a harder route. Two instruments on one claim, and `142` names which is which.

**`143` reproduced F2's refutation on its own axis set**, deliberately expecting a different
counterexample count and getting 134 against `141`'s 714, and says matching would have been a reason
to distrust its instrument rather than to trust it.

**`144` reproduced `139`'s weight-cell geometry exactly** (F144-1) and then decomposed it, which is
reproduction followed by correction rather than agreement.

**And `145`'s `z1` A1 is a third instance of a proof rather than a third measurement**, since it is
`141`'s monotonicity theorem restricted to one axis. Recorded at that rung and not higher.

### 1.4 The withdrawal

`144` surveyed the committed bench corpus for the arm `139`'s O-139-C asks for, using
`144_probes/p10`, found exactly one,
verified it by a second decision procedure, and then found the verdict rests on a single coordinate
where the deciding gap is 3.8 nanoseconds against a 79.2 nanosecond interquartile range.

**It withdrew its own headline and left the wrong headline committed**, on the ground that the
sequence is the finding. The mechanism it extracted is worth more than the incident: its first noise
control compared arm-to-arm spread against within-arm spread **per family**, and dominance and
selectability are **pairwise** questions, so the gate has to be pairwise. `144_probes/p10c` rebuilds it pairwise and finds zero established instances corpus-wide
(`144` F144-18), after `144_probes/p10b` had verified the withdrawn one by a second decision
procedure.

**And the zero depends on something F144-18 does not name.** `145_probes/z2` sweeps the gate and finds
the direction of conservatism unstated: under the reading where the strict half is gated and the
no-worse half is read on medians, 29 of 35 families still carry two or more established non-dominated
arms at `k = 0.5`; under the reading where both halves are gated, 20 do, and the count moves
non-monotonically in gate strength. Both readings are defensible and they give different corpora.

### 1.5 One-expert claims, listed as such

None of these has a second instrument and each is named where a later reader would otherwise assume a
rung.

- **The closure asymmetry** (`140` section 2): the assignment space is closed and enumerable because
  an axis position with no lowering cannot be supplied from outside; the weighting space is open
  because a weighting re-ranks arms that already exist. `141:780-782` carries it forward and says its
  own count function is consistent with it rather than evidence for it.
- **The declared-width companion rule** (`140` section C): the range policy's limit is read at the
  declared width and a container is never allowed to move it. Every later instrument is built on that
  reading, which makes it load-bearing and still one expert.
- **`141` F9's width-sensitive-axis half.** `143` reproduced the invariance half on its own committed
  output and explicitly did not reproduce this half.
- **The Chebyshev construction and its costs** (`144` F144-6 through F144-9). One author, several
  probes, no second reader.
- **`145`'s five results**: the visibility quantifier, the gate-direction finding, the enforceability
  condition, the equivariance placement, and the units reading of the baseline.

### 1.6 Every convergence carries the intersection of what its instances ranged over

This is the standard the topic established the hard way, stated at `143:424-431`, and it is why B2
looked like corroboration while excluding the case that mattered. `145_probes/z6` computes it
mechanically over 50 predicate blocks, 49 of which name `threads`, which is the control.

| convergence | instances | intersection | in the union only |
|---|---|---|---|
| the container is answer-invisible at the column | `139_probes/p6`, `140_probes/p3`, `141_probes/p4` | `F`, `W`, operations, overflow, rounding, threads | 10, including `signedness` |
| the accumulator is visible exactly at signed saturating | `141` F2, `142` F142-5 on `142_probes/q3`, `143` F1 on `143_probes/p2` | `F`, `W`, accumulator, overflow, rounding, signedness, target features, threads | 14, including the schedule |
| the class count is monotone and not strict | `141` F8, `143` F2' | `F`, `W`, assignments, signedness, target features, threads, witness sets | 4, including accumulator and container |
| fusion is an axis position already | `141` F7, `142` F142-1 | `F`, `W`, arity, chain length, operation, overflow, rounding, signedness, target features, threads | 4 |
| the firewall proposition | `139`, `141`, `142` | **none: it carries no predicate** | not applicable |

**The intersection is an upper bound rather than the region.** `z6` matches dimension names, so it
catches a dimension one instance never varied, which is `143`'s failure exactly. It cannot check that
two files mean the same thing by a name, nor that a listed dimension was swept rather than pinned.

### 1.7 Contested, each with what would decide it

**C1. Which selector the design ships** (`144` O-144-A). Linear reaches hull vertices; augmented
Chebyshev reaches
every non-dominated arm at the same measured compile-time and portability cost. *Decided by* whether
an arm set arvo ships ever contains an established unsupported efficient arm. Zero today across 29
askable families, so the honest default is linear and the question reopens on the first instance.

**C2. Which units a weighting is expressed in.** Section 6.2. Both readings are coherent and they
answer different questions. *Decided by* a declaration, not by evidence.

**C3. Whether the accumulator cell is stated conservatively or with a schedule dimension.** `142`
O-142-B. *Decided by* whether any consumer kernel has a fixed declarable accumulation order;
`satfold-*` and `warm-clamp-*` under `mock/benches/variants/` already carry accumulator arms, so it is
a computation over committed artifacts.

**C4. Whether a policy pins one answer or declares a set.** `139` O-139-B opened it and `141` and
`142` between them closed it against the set formulation in every cell measured; `139` conceded. I
record it as **retired rather than contested** in 1.8 and note here that `139` is the signature that
decides whether that is fair.

**C5. Whether the gate under F144-18 is symmetric or conservative.** `145` section 3.2. *Decided by*
stating which the design means; the two give 20 and 29 askable families.

### 1.8 Retired

**`139`'s declared-slack repair, in every cell.** Conceded by its own author at `142` section 1, on
`142_probes/q1`, from `139`'s own committed probes rather than from `141`'s instrument. The cell-by-cell
accounting is `141` F4's slack table, `141` F5's characterisation of the residual unit as a rounding
relocation rather than a reduction one, and `141` F6's measurement that the declaration admitting fusion
at signed saturating permits a mean of 41.74 answers of 64 for a single input. The residue the mechanism served is
empty: where fusion is answer-preserving it is a free lowering, and where it is not it is an axis
position the design already has.

**`139`'s "the count is not a property of the design at all".** Too strong; `shape -> count` is a
well-defined function. Conceded at `142` section 6, and `139`'s own table is the evidence against it.

**`140`'s F2 as worded.** "Strictly increasing" is false, with 714 counterexamples on `141`'s axis set
and 134 on `143`'s own. The content survives as F2' and the original stands unedited.

**`141`'s replacement B**, that the fractional shift be spelled as an arithmetic shift right. Contested
by `142` section 3 on `142_probes/q2` and replaced by B-prime: `142` F142-4 measures the swap changing up
to 44.53% of multiply answers on signed shapes, so it is a policy change and not a spelling. What survives inside B is a naming obligation,
which is `131` F131-3 arriving from another topic.

**`144`'s own headline instance**, per 1.4. **And `144`'s own reading of its seven-arm table**: `144`
F144-3 decomposes the six-against-five gap as `139`'s exact-duplicate control arm losing a tie-break, so
the table is not a second instance of the Pareto claim and the witness carries it alone.

**And two of `139`'s predictions and two of `140`'s**, each refuted by its own probe before either read
anything, which is the protocol working rather than a defect.

---

## 2. What this candidate rests on

Q51's two-component object, at `OPTIONS.md:2425-2461`, which this topic tested rather than assumed and
which nothing in it unseats. Twenty-one probe entries across `139` and `144` produced no candidate for
a third component and no reason to merge the two.

I13, ratified at `INTENTS.md:214-235`, whose scope limit at `:263-267` says the dimension list and the
exactness bar are elaboration rather than ratified. I have not cited either as authority.

The workspace rule `arvo-always-optimal-internals.md`, which is a shared input to both cold
derivations and is declared in 1.1 rather than counted as evidence.

---

## 3. Doability, established rather than assumed

**A consumer-defined strategy resolves at compile time to one unconditional branch.**
`139_probes/p5_open_set.rs`, scanned by `139_probes/p5_scan.sh`, compiles a strategy point defined outside the library against an unchanged library half, and its
scanned assembly shows each monomorphisation as one `mov` and an unconditional tail branch to exactly
one arm, with a runtime-selected control retaining both arms and a `tbz`. `139` records that its first
scan reported zero arms for all three functions because it was reading `extern "C"` thunks one hop
above the monomorphised symbol, and the corrected scan is committed with the note.

**A non-linear selector is no harder.** `144` F144-7, on `144_probes/p8`, const-evaluates a weighted
Chebyshev selector to the same shape, and removes the thunk hazard at the source by making the dispatch `inline(always)`, so
each entry point's own body is the dispatch.

**So the openness in section 5.1 and the selector question in section 5.6 are both doable under I15**,
and neither waits on evidence.

---

## 4. The kinds of argument, since they bound the clauses differently

Sorted because a later reader needs to know which clauses transfer and which stop at their sweep.

**Theorem, no sweep required.** The monotonicity of the class count in the observation set (`141`
section 5.1). The absorption of a prior reduction modulo a power of two (`141` F3). The
argmin-preservation of a positive scalar rescale (`145` `z5` E1). The mutual exclusion of negation
symmetry and translation equivariance (`145` `z4` D2). Each is proved and then checked, and a check
that came out otherwise would have meant the instrument was broken.

**Exact certificate over a continuum, which is the one shape this topic has and the others did not.**
`144` F144-2 decides that `139`'s compromise arm loses by at least one unit **at every point of the
simplex**, by exact vertex enumeration over rationals rather than at 2001 sampled points, and its
control arm inside the hull comes out strictly selectable at `-1`. F144-6's Chebyshev reachability is
the same shape from the other direction: a certificate weight per arm, so each arm is decided rather
than searched, verified at 3667 of 3667.

**Closure.** The one-sided-clamp congruence, which is a shared input rather than a finding of this
topic (1.1). The enforceability condition at 5.4, checked by exhaustive extensional comparison with a
firing control.

**Equivariance.** The licensing condition for relocating a rounding across an integer addition
(`142` F142-2 and F142-3), six modes of six agreeing with the partition.

**Exhaustive enumeration over a small domain.** The class counts, the container sweeps, the
accumulator cells. These stop at their widths and none of them carries a transfer argument to 64 bits.

**Existence.** The fusion difference rates and the slack tables: a witness at one width establishes
the failure at every width and its rate at none.

**Bounded to the sweep.** `142` F142-6's schedule dimension, on two schedules. `144` F144-19's six
reordering families of twenty-four, on committed CSVs at the sizes the harness ran.

**Arithmetic on committed output, taking no measurement.** `144`'s `p10` through `p10c` and `145`'s
`z2`. These read harness artifacts and do arithmetic; they price nothing.

---

## 5. The statement

### 5.1 The object

> A strategy is a pair. **Component one is an assignment on observable policy axes and it fixes the
> denoted answer.** **Component two is a weighting over cost coordinates and it ranges over
> realisations of that denotation.** The two are told apart by one mechanical test and not by taste.
>
> The components have different closure properties. The assignment space is **closed and enumerable**,
> because an axis position with no lowering cannot be supplied from outside. The weighting space is
> **open and consumer-supplied**, because a weighting re-ranks arms that already exist. So "is the
> strategy set closed" has two answers and the question conflates them: the axes are closed, the
> named points are open.

*The closure asymmetry holds for: the assignment set of rounding, overflow and intermediate positions
as swept in this topic; threads = 1. **Argument kind: derivation from I8 and I9**, one expert
(`140` section 2), carried forward unchallenged.*

### 5.2 The membership test

> Take the candidate axis, hold everything else fixed, sweep the domain, and look for an input where
> the answer changes. If one exists it is policy. If none exists over the whole domain it is a
> weighting. If it is neither it is not an axis.
>
> **The test is stated over chains rather than per axis.** An axis with no effect at chain length one
> and a large effect at chain length two is not a property of the axis, and observability is a
> property of the chain.

*holds for: W = 4; F in {0, 1, 2}; signedness in {unsigned, signed}; assignment set = rounding
{floor, toward zero} x overflow {wrap, saturate both, saturate high only} x intermediate {stepwise,
exact}; observation sets = every subset of {add, subtract, multiply, multiply-add, multiply-subtract};
inputs exhaustive over the declared range; container width = declared width; overflow limit read at
the declared width; threads = 1; target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly
57d06900f).*

**Argument kind: exhaustive enumeration**, with the chain clause conceded by `139` against itself at
`139:769-777` and supported by `141`'s T5 that `add` alone cannot see the accumulator at any shape.

### 5.3 The count

> The class count is the cardinality of an assignment set quotiented by observational equality over an
> observation set. An observation is a shape, an operation and an input, so fixing a shape and fixing
> an operation set restrict the same set. The count is **monotone non-decreasing** in the observation
> set and **not strictly increasing**; strictness is a property of the operation added.
>
> **Cutting an axis out of the assignment set changes the count if and only if that axis is
> answer-visible under the observation set**, and visibility is **monotone in that set and therefore
> saturates**. So the axis-only property is visibility under the maximal observation set the design
> admits: an axis visible there is component one and cutting it is forbidden; an axis invisible there
> is dead and cutting it is a no-op everywhere.
>
> The count therefore has one argument that moves, the observation set, plus the axis set the design
> ships. **What the canon states is the axes and their positions**, which are finite, arvo's and
> permanent.

*The monotonicity holds for: assignment sets and observation sets as in 5.2, plus `141`'s and `143`'s
own axis sets; W in {3, 4}; F in {0, 1, 2}; signedness in {unsigned, signed}; threads = 1; target
features = host. **Argument kind: theorem**, checked over 2532 subset pairs with zero violations by
`141` and over 540 by `143`, the latter with an anti-monotone control firing at 28.*

*The biconditional holds for: W = 4; F in {0, 1, 2}; signedness in {unsigned, signed}; assignments =
rounding {toward zero, floor} x overflow {wrap, saturate both, saturate high only} x intermediate
{stepwise, exact} x a dead axis with two identical positions; observation sets = {add, sub, mul},
{madd}, {mul, madd}, {add, sub, mul, madd, msub}; inputs exhaustive; container width = declared width;
overflow limit read at the declared width; accumulator width = unbounded; threads = 1; target features
= host. **Argument kind: exhaustive enumeration**, 96 cells with no exceptions (`144` F144-16), with
the dead axis as the control that must never move a count.*

*The saturation holds for: the same assignment set at W = 4; F in {0, 1, 2}; signedness in {unsigned,
signed}; observation sets = six nested subsets of the five operations; threads = 1; target features =
host. **Argument kind: theorem plus enumeration**, 432 ordered triples with zero violations and a
control firing at 55 instances where enlarging the observation set makes an axis visible
(`145` `z1`).*

### 5.4 The firewall, and the condition that makes it enforceable

> **No cost model may move an answer.** Every difference in an answer traces to the declared policy
> and nothing else may move one.
>
> That is enforceable with no mechanism beyond declaration exactly when **every lowering arm the
> design admits realises the denotation of some assignment in the assignment set.** Where it does, a
> conforming pair is two arms of one assignment and the weighting picks between them freely; a
> non-conforming pair is two assignments and the type names which one. Where an arm realises no
> assignment, the firewall has no repair available in the type.
>
> With the firewall, a target change moves which arm runs and moves no answer. Without it, a target
> change moves answers at the arms' full disagreement rate and no predicate names the change, because
> the cost table is not in the type.

*The consequence holds for: numeral fixed-point signed; W = 6; F in {0, 3}; overflow = saturating;
rounding = truncate toward zero; conforming pair = two routes to one multiply; non-conforming pair =
fused and stepwise multiply-add; cost coordinates = 3 with two committed synthetic tables; selector =
linear; weight grid resolution = 1/24; inputs exhaustive over the declared range; container width =
declared width; threads = 1; target features = host. **Argument kind: exhaustive enumeration with both
controls firing** (`144` F144-15): zero at weights where the selection agrees, zero on identical
targets.*

*The enforceability condition holds for: W = 4; F in {0, 1, 2}; signedness in {unsigned, signed};
assignment set = rounding {floor, toward zero} x overflow {wrap, saturating} x intermediate {stepwise,
exact}; arm set = {fused by widening, fused by partial products, stepwise by shift, stepwise by
partial products} and a control arm rounding off the axis; operation = multiply-add; arity = 3; inputs
exhaustive; container width = declared width; threads = 1; target features = host. **Argument kind:
closure with a firing control** (`145_probes/z3`).*

**The proposition itself is unpredicated and section 6.1 says what that costs.**

### 5.5 The axes, and where each is free

> **Fusing a multiply-add is a free lowering where the axes make it answer-preserving.** Under
> unsigned range policies that is every rounding position, by the one-sided-clamp congruence. Under
> wrapping it is exactly the **translation-equivariant** rounding positions.

*holds for: W = 6; F in {0, 1, 2, 3, 4, 5}; signedness = unsigned; overflow in {wrap, saturating};
rounding in {floor, ceiling, toward zero, away from zero, nearest-half-up, nearest-half-even};
operation = multiply-add; arity = 3; chain length = 2; container width = declared width; threads = 1;
target features = host.*

*and holds for: W = 6; F in {0, 1, 2, 3, 4, 5}; signedness = signed; overflow = wrap; rounding in
{floor, ceiling, nearest-half-up}; operation = multiply-add; arity = 3; chain length = 2; container
width = declared width; threads = 1; target features = host. **Argument kind: closure for the unsigned
half and equivariance for the signed wrapping half** (`142` F142-3 on `141` F3's absorption).*

> **Where fusion changes the answer it is not a lowering, and the axis already names it.** The fused
> and stepwise realisations at signed saturating are two denotations. A consumer wanting the fused
> answer declares the exact-intermediate position, keeps full determinism, and gets the fast arm.

*holds for: W in {4, 6}; F in {0 .. W-1}; signedness in {unsigned, signed}; overflow in {wrap,
saturating}; rounding = truncate toward zero; operation = multiply-add; arity = 3; chain length = 2;
container width = declared width; threads = 1; target features = host. **Argument kind: extensional
identity**, established twice, `141` F7 by building both arms and `142` F142-1 from `139`'s own two
committed probes, over 6,356,992 triples with a cross-pairing control at 757,954 differences.*

> **A lossless storage container contributes no distinguishable answer functions**, so the
> storage-minimising concern has zero policy content where a value is put.

*holds for: W = 4; F in {0, 1, 2}; operations in {add, subtract, multiply}; overflow in {wrap,
saturating}; rounding in {toward zero, floor}; overflow limit read at the declared width; threads = 1.
**Argument kind: exhaustive enumeration, three instruments**, and this predicate is the intersection
of what the three ranged over rather than the union, per 1.6.*

> **Narrowing an accumulator is a cost choice everywhere except at signed saturating**, where it is a
> policy choice and changes answers. Whether a kernel reaches that cell is a property of its
> accumulation schedule rather than of the assignment.

*holds for: W = 4; F in {0, 1, 2}; signedness in {unsigned, signed}; overflow in {wrap, saturating};
rounding in {toward zero, floor}; accumulator width varied above the declared width; threads = 1;
target features = host. **Argument kind: exhaustive enumeration, three instruments**, intersection per
1.6.*

*The schedule dimension holds for: signedness = signed; W = 4; F = 1; overflow = saturating;
accumulator width in {W, W+1, W+2, 2W}; fold length = 3; operation = multiply-accumulate fold;
multiplier schedules {[1,-1,2], [2,-1,1]}; rounding = floor; domain = every sequence over the declared
range; threads = 1; target features = host. **Argument kind: bounded to the sweep**, two schedules
(`142` F142-6).*

### 5.6 The selector

> A linear weighting selects an arm exactly when that arm is a vertex of the lower convex hull of the
> arm set. **So "a weighting over cost coordinates" must not be read as "any arm can be asked for",
> because that is false at roughly one non-dominated arm in nine** (`144` F144-5, 6.6% to 12.7%). Where
> the wider selector is taken it is the **augmented** form, because `144` F144-8 measures the plain one
> selecting a dominated arm at 11 of 41 weights and the augmented one at 0. Either the component is defined
> with a selector reaching every non-dominated arm, or the design records as part of what a weighting
> *is* that it names hull vertices.
>
> A cost coordinate exists only if every arm carries a value on it **and the arms differ on it**
> (`139` section 5's obligation with `144` F144-4's second half). A coordinate they do not differ on
> makes every arm weakly selectable and hands its region of weight
> space to the tie-break rule.

*The limit holds for: cost tables drawn uniformly from integers 1..20; arms in {4, 6, 8}; cost
coordinates in {2, 3, 4}; 120 tables per cell; no exact duplicate arms; decision procedure = exact
vertex enumeration over rationals; threads = 1; target features = host. **Argument kind: exact
certificate over a continuum**, decided rather than swept.*

*The reachability of every non-dominated arm holds for: the same population; reference point =
componentwise minimum minus one; certificate weight proportional to the reciprocal deviation;
augmentation coefficient = 1/1000; threads = 1; target features = host. **Argument kind: construction
with a per-arm certificate**, 3667 of 3667, with zero dominated arms certified.*

*The dead-coordinate result holds for: arms = 4; cost coordinates in {2, 3} with the third constant
across arms; selector = linear; decision procedure = exact vertex enumeration; threads = 1; target
features = host. **Argument kind: exact certificate.***

**And no arm set arvo ships today contains an established instance of the limit**, over 29 askable
families under the conservative gate reading and 20 under the symmetric one, so the honest default is
linear and the question reopens on the first instance.

### 5.7 The baseline, and what travels

> **A weighting travels and an arm does not.** A weighting carried to a new target re-resolves there
> and lands on that target's optimum by construction. An arm carried to a new target is frozen. A
> predicate that names an arm is bound to the target it was measured on; a predicate that names a
> weighting is not.
>
> **Every strategy's cost claim is stated against the same named arm** (`140` F4, sharpened by `144`
> F144-14, which measures the per-arm alternative reporting a winner that is not the winner at 33.5%
> once cost is a vector), and the design declares
> **which units a weighting is expressed in**, because one named baseline names two operations.
> Applied once to the weighted scalar it is a reporting normalisation and preserves the argmin.
> Applied per coordinate before the weighting it is a change of the weighting's units, identical to
> the substitution `w -> w/b`, and it is what makes a weighting travel across a per-coordinate rescale
> of the target.

*The arm regret holds for: cost tables drawn uniformly from integers 1..60; arms = 7; cost coordinates
= 3; 200 independent target pairs; selector = linear; weight grid resolution = 1/40 sampled every
seventh point; threads = 1; target features = host. **Argument kind: enumeration over a drawn
population**, with the identical-target control at exactly zero (`144` F144-11).*

*The scalar placement holds for: cost coordinates any; arms any; weights any positive; baseline any
arm with positive weighted cost; threads = 1. **Argument kind: order-preservation theorem**, which is
why this predicate lists no width, no population and no resolution.*

*The per-coordinate identity holds for: cost coordinates = 3; arms = 7; cost tables drawn uniformly
from integers 1..60; weight grid resolution = 1/12 on the 2-simplex; baseline any arm with positive
coordinates; threads = 1; target features = host. **Argument kind: algebraic identity**, checked at
5460 pairs with zero disagreements (`145` `z5` E2).*

*The no-op condition holds for: baseline uniform across coordinates; cost coordinates = 3; arms = 7;
threads = 1. **Argument kind: sufficiency by algebra and necessity by witness**, every non-uniform
baseline tried moving a selection on the first arm set drawn.*

### 5.8 What the canon owes a consumer picking one

> **The policy is chosen from the consumer's semantics** and the substrate cannot know it. **The
> weighting is chosen from the consumer's measurements** and nobody else can take them. **Pick the
> policy first**, because a wrong policy is a wrong answer and a wrong weighting is a slow answer.
>
> Two presets sharing a policy assignment compute the same answers on every input and no test
> distinguishes them. That is not a defect and a reviewer expecting four presets to be four semantics
> will keep looking for a difference that is not there.

**Argument kind: derivation from the object**, one expert (`139` section 7), unchallenged.

---

## 6. The three things that must stay visible

### 6.1 The central surviving claim is unpredicated, and this candidate does not fix that

**The firewall carries no predicate in any of the three files that endorse it.** `139` proposes it,
`141` endorses the proposition while attacking the repair, `142` keeps it. `145_probes/z6` resolves
none of the three to a predicate block. Under I13 a claim with no dimensions listed holds in no region
where any of them is present, which is the strongest negative the notation has.

**I am not supplying one, and the reason is that the claim is not the kind of thing a sweep
establishes.** It is a design proposition about what a cost model may do, and a predicate over widths
and operations would misdescribe it. What can be predicated is what it **buys**, and that is `144`
F144-15, whose predicate is in 5.4 and which is the region the proposition has evidence in.

**So the candidate states it in two pieces and says which is which**: an unpredicated design
proposition, plus a predicated measurement of its consequence and a predicated condition for its
enforceability. **What that costs is real and I state it rather than smoothing it.** A canon sentence
with no predicate cannot be gated on, cannot be composed with an arm, and cannot be narrowed by a
later measurement without someone deciding what it was quantified over. If op wants the firewall
gateable it has to become a statement about a region, and nothing in this topic has done that work.

### 6.2 `144` sections 6 and 4.3 conflict, and the reading that dissolves it

Section 4.3 **requires** the per-coordinate normalisation, because `144` F144-12 measures it driving
the cross-target switch rate to exactly 0.0% on a pure per-coordinate rescale where the un-normalised
rate is a mean of 18.0%. Section 6 **forbids** it, because `144` F144-13 measures it changing which arm a
fixed weighting picks at 24.6%. Both are
correct about the same operation. `145_probes/z5` E4 reproduces the 24.6% independently, at **894 of
3640 cases**, on a differently written instrument.

**The reading that dissolves it: the per-coordinate division is a declaration of the weighting's units
rather than a transformation of the cost table.** A weighting declared in baseline-relative units
selects by `argmin` over `sum_k w_k c_ik / b_k` **by definition**, so there is no absolute-units
weighting it was supposed to agree with. Measured: across a pure per-coordinate rescale it moves at 0
of 3640, and across an arbitrary target change at 2892 of 3640, which is the control saying it is a
change of units rather than a claim that targets do not matter.

**This is one expert's and it is an argument rather than a measurement.** The measurement in it only
confirms that a units reading behaves as a units reading should. `140` and `143`, whose obligation it
repairs, and `144`, whose sections it reconciles, are the signatures that decide whether it holds.

### 6.3 The count's second argument closes, and the table waits on a decision

`143` F4 said the count has two moving arguments and a shape-to-count table "is not currently
writable".
`144` answered with the biconditional. `145` supplied the quantifier the closure needs, on `145_probes/z1`: visibility is monotone in the
observation set and therefore saturates, so the axis-only property is visibility under
the **maximal** observation set.

**The second argument is closed. The table still waits, and it waits on a decision rather than on
evidence.** The maximal observation set is the set of operations the design ships, which is a design
act. So `143`'s conclusion is right for a reason `143` does not give: not because the denotation line
is open, but because the operation set is not named. Those are different questions and the second one
closes when op or the design names the operation set, at which point the table is writable
exhaustively at model widths.

---

## 7. What this does to the rounding topic, which is closed

The rounding topic has its own candidate at `132`, revised at `136`, signed and checked, and three
members of this topic cite it by line. **Nothing here edits it and nothing here asks for it to be
reopened.**

`145_probes/z4` places translation equivariance against that candidate's own law enumeration over
eleven rounding modes:

- **The order bound implies equivariance.** Zero modes carry the bound without it.
- **Negation symmetry excludes equivariance**, by a two-line argument: symmetry gives
  `rnd(-1/2) = -rnd(1/2)`, equivariance gives `rnd(-1/2) = rnd(1/2) - 1`, so `rnd(1/2) = 1/2`. **That
  is a second exclusivity of exactly the shape `132` section 5.3 already records** for the order bound
  against negation symmetry.
- **Among modes carrying neither it is undetermined**, witnessed by construction on the third attempt
  and marked as constructed.
- **On the ten natural modes the coincidence is perfect**: equivariance holds exactly where negation
  symmetry fails.

> **So the closed candidate is not wrong and is not complete for this use.** It enumerates which
> member carries which law. An arm relocating a rounding across an integer addition reads a property
> that the enumeration determines in two cases of four and leaves open in the rest. A design reading
> the law table alone gets the right answer on every mode it would plausibly ship and has no argument
> that it will keep getting it, which is the precise content of `142:264-270`'s "naming the six modes
> is necessary and not sufficient".

**My reading is that this does not require reopening the rounding topic**, because the gap is in what
that candidate covers rather than in what it says, and because the property is read by an arm's
predicate, which is this topic's surface. If op reads it the other way, that is op's call and this
file stops here rather than making it.

---

## 8. What only op decides

**Which selector the design ships** (C1). Measured today as linear-is-sufficient and reopening on the
first established instance; which of the two the canon says it is doing is not derivable.

**Which units a weighting is expressed in** (C2). A declaration, not evidence.

**The operation set the design ships**, which is what the shape-to-count table waits on (6.3).

**Whether the accumulator cell is stated conservatively or with a schedule dimension** (C3).

**Whether the default rounding position is chosen for familiarity or for what it licenses.** Sharper
after section 7: the IEEE default is on the non-equivariant side, so a design defaulting to it has
every relocation unavailable at the default.

**How many named presets there are and what they are called.** Not derivable from anything in this
topic; both cold derivations reached that independently and I1 is already demoted.

**Whether a canon may carry an unpredicated proposition at all** (6.1), which is the question this
candidate cannot answer from inside itself.

---

## 9. The four signatures owed

**`139`**, on whether 1.8 is fair to the repair it conceded, on whether the contamination in 1.1 is
stated at the right severity, and on the firewall's unpredicated status in 6.1, which is its
proposition.

**`140`**, on whether the closure asymmetry and the declared-width companion rule are represented at
one expert correctly, and on 6.2, which repairs its own obligation.

**`141`**, on whether 1.2 records the rescoping as a narrowing rather than a refutation at the
strength it intended, and on whether 4's sorting of its theorems is right.

**`144`**, on 6.2 and on 1.4's reading of its gate, and on whether 5.6 states the selector question at
the strength its own measurements support.

---

## 10. Anchor accounting

Counted on `146_probes/w2`, which reuses `119_probes/r1`'s stripper and extends `132_probes/w1`'s
classes with option ids and file-qualified findings, with this section excluded from the computation.
Neither earlier instrument is edited.

**The extractor strips markdown before matching.** `145_probes/z7` had two citations fail that were
verbatim correct, defeated by the source's own backticks and emphasis inside the quoted phrase, and
that is the fifth instance of the class in this panel. `w2`'s controls confirm the stripping recovers
an id wrapped in markup and cannot manufacture an absent one.

```
  class                    in the union   in 146   not carried
  finding ids                        35       33             2
  file-qualified findings            21       33             0
  option ids                         18        4            14
  theorem labels                      4        2             2
  probe stems                        29       22             8
  line anchors (panel)               55       14            51
```

**What the `not carried` sets are, exactly.** **Two findings**: `144` F144-10, the median 90.2%
cross-target switch rate, and F144-17, the four-to-six arms a linear selector can distinguish at three
coordinates. Both support clauses this candidate does not state, and both are worth a later reader's
attention if op takes C1 toward the wider selector. **Zero file-qualified findings**, so every finding
this candidate carries carries its file. **Two theorem labels** from the rounding topic, quoted inside
`141`'s own finding list rather than claimed here. **Eight probe stems**, all instruments for findings
the candidate cites by id rather than by probe. **Fourteen option ids**,
which is the largest cell and the deliberate one: this candidate carries the four that became contested
items in 1.7 and leaves the option registers of four files where they are. A candidate carrying all
eighteen would be a register rather than a statement.

**Fifty-one line anchors are not carried and that number is honest rather than alarming.** The union's
line-anchor class is dominated by each file citing its own sections and its neighbours' by line, which
is what a reply does. A candidate cites a clause, not a paragraph, so it carries the fourteen that pin
a quotation or a disputed sentence and none of the rest.

**The `not carried` set is not audited to zero on purpose**, and that is the difference between this
file and a compression of a single predecessor. Where a clause rests on an anchor, the anchor is at
the clause. Where a finding is one this candidate does not state, its id lives in its own file and is
reachable through the ledger's tables.

**The stripper fired, on the paragraph above.** Naming `F144-10` and `F144-17` in order to account for
them makes both present in this file's text, so the unstripped count reads 35 findings of 35: a clean
sheet produced entirely by the sentence admitting it is not clean. `w2` reports
`finding +2, finding_qualified +1, probe_stem +2`, which is that gap exactly. This is the guard's
designed case and it has now fired on three consecutive candidates, which is worth stating rather than
burying: an author accounting honestly for what it dropped disables the instrument that would have found
it, and the more careful the accounting the more complete the disabling.

**And one control the earlier instruments did not have earned its place.** `w2`'s finding pattern first
read `141 F9's width-sensitive half` as an id called `F9'`, leaving `F9` counted as not carried. A
trailing apostrophe is a real part of an id here (`F2'`, `F3'`) and a possessive is not, so the pattern
now refuses one before an `s` and a control confirms it.

**No line anchor into shipped source is carried and none is owed**, because the code tier is the one
that gets rewritten and a canon anchored to it cannot be its oracle.

---

## 11. Coverage, bounded

**Read in full:** `139`, `140`, `141`, `142`, `143`, `144`, `145`, `INTENTS.md`. **Read in part:**
`RULES.md` at its predicate section; `OPTIONS.md:2425-2461` only. **Opened at source rather than
recalled:** `137_probes/g0_test_gate.out`; `139:3-6`, `139:177-179`, `139:244`, `139:769-777`;
`140:3-7`, `140:577-579`; `141:209-223`, `141:780-782`; `142:109-112`, `142:122-126`, `142:264-270`;
`143:424-431`, `143:533`; the commit log for the phase-one ordering.

**Not read:** `40`, `93`, `102`, `106`, `107`, `108`, so where Q51 compresses them I rely on Q51's
account exactly as `141`, `143` and `144` do. `132` and `136` beyond the sections `142` cites and the
law enumeration `145_probes/z4` tests. The probe sources of `139` through `144`, except
`144_probes/p10c`'s gate shape.

**Built:** one probe, six predictions, all confirmed, with four controls, and the citation instrument
`145_probes/z7` reused as the model for `w2`'s markup handling. **Not done:** no
measurement, so nothing here prices anything; no re-derivation of any theorem in sections 4 or 5; no
second read of my own five one-expert results in 1.5, which is what the signatures are for.

**Everything in section 5 is at model widths**, `W` in {4, 6}, with no transfer argument to 64 bits,
and **everything is `threads = 1`**, so under the notation none of it holds anywhere threads exist.
**Container width equals declared width** in every fixed-point instrument this topic contains, the
narrowing `139` reported against itself and every later file inherited.
