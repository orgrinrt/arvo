# 145. Formalising the strategy object

**Member:** Leroy, dispatched to formalise rather than to attack. I took no part in `139` through
`144`; I formalised the two topics before this one, so the conventions are the ones I have been
using and not new here.

**Read:** `139`, `140`, `141`, `142`, `143`, `144` in full; `INTENTS.md` in full; `RULES.md`;
`OPTIONS.md` entry Q51 at `OPTIONS.md:2425-2461`. Probe outputs of the six where a claim below rests
on one. Nothing else new.

**Built:** seven probes in `145_probes/`, each committed with its output as it ran. Twenty-two
predictions confirmed, **three refuted**, and three defects found in my own instruments and recorded
rather than repaired quietly.

**Nothing here is settled and nothing moves to `mock/canon/`.** Op ratifies. Where I disagree with a
file I say so with a measurement, in section 7, kept out of the formalisation.

---

## 0. Gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` entry by entry. Nothing below proposes a design decision, presumes the
strategy set closed at four (I1 is demoted to open at `INTENTS.md:51-61`), or reasons from the removed
crate tree. The arms in section 4 are I13's shape and I13 is ratified at `INTENTS.md:214-235`; its
scope limit at `INTENTS.md:263-267` says the dimension list and the exactness bar are elaboration
rather than ratified, and I have not leaned on either as authority. No predicate below carries a hedge
token, per op's instruction at `INTENTS.md:241-246`.

One thing I record because section 6 turns on it. I16 (`INTENTS.md:317-331`) says the canon does not
police what shape a law takes. `141` and `142` both flag their own replacements against it. The same
caution binds me harder, because a formalisation naming an arm's shape is exactly the failure I16
describes, so section 4 states each arm's **region and what it buys** and leaves the construction to
whoever builds it.

### 0.2 Test gate: passed, at 123 across 13, inherited from an artifact I opened

I did not re-run the suite. Disk is at 7.2 GiB and thirteen release builds would take most of it. I
opened `137_probes/g0_test_gate.out`, which is the run `137` made itself rather than inherited, and
it ends `123 passed across 13 crates, 0 failed`, with the per-crate lines above it and
`bitpack-write-contend-shared` at 6.61s. That is the file and that is the count.

The `-- --test-threads=1` requirement now has **four independent reproductions** (`139` at 7.97s in a
debug build, `141` at 46.65s, `143` at 2.57s, `144` at 3.03s under release), and the mechanism is
`139`'s livelock diagnosis in `pool.rs`. Nobody has re-derived it and nobody needs to.

**Test quality.** Five members have now read bodies in five different crates and all five report the
same verdict. I read none, because I built no Rust this dispatch and my probes are exact integer
arithmetic in Python. I inherit that verdict with attribution rather than making it a sixth.

---

## 1. The discipline this topic established, applied to my own output

`143` section 1.3 states it: **two agreeing instruments agree about the intersection of their
dimensions, not the union**, so two probes that both lack a dimension agree vacuously about it and
the agreement reads as corroboration. That is what happened to `139` and `140` on the accumulator,
which `141` found by adding the dimension and `142` and `143` then reproduced.

The dispatch asks that every convergence I record name what each instance varied. I have computed
that rather than recalled it, in `145_probes/z6`, which extracts every predicate block from the six
files and intersects the dimension sets per convergence. **50 predicate blocks across the six files,
49 of them naming `threads`**, which is the control that says the parse is not dropping dimensions.

Two results from it before anything else, because they bound everything in section 5:

**The accumulator dimension is absent from every predicate in `139` and in `140`, and present in
`141`, `142` and `143`.** The incident is not that one file missed a dimension. It is that both
instances of an agreement lacked it, so the agreement's region excluded the case that mattered and
nothing in either predicate said so.

**The firewall proposition carries no predicate in any of the three files that endorse it.** `139`
proposes it, `141` endorses it while attacking the repair, `142` keeps it. Under I13 a claim with no
dimensions listed holds in no region where any of them is present, which is the strongest negative
the notation has. That is not a defect in those files: a design proposition is a legitimate thing to
state without a predicate. It does mean the two-expert count on the firewall is a count of agreements
about a **sentence**, and `144`'s F144-15 is the first predicated measurement of what the sentence
buys.

**What `z6` can and cannot do**, stated because an intersection it prints is an upper bound. It
matches dimension names, so it catches a dimension one instance never varied. It cannot check that
two files mean the same thing by a name, and it cannot check that a listed dimension was swept rather
than pinned.

---

## 2. The object, as the topic leaves it

Q51's two-component statement survives, and the topic tested it rather than assuming it.

**Component one is an assignment on observable policy axes and it fixes the denoted answer.**
Q51's repair, which `140` reproduced blind and then conceded to (`140` section B), and which nothing
in `141` through `144` unseats. Fifteen probe entries in `144` and six in `139` produced no candidate
for a third component and no reason to merge the two.

**Component two is a weighting over cost coordinates and it ranges over realisations of that
denotation.** `144` section 3.7's correction is inside this component rather than to the pair: a
weighting read as a linear objective names the vertices of the lower convex hull and not every arm.

Three structural facts about the pair are established and are worth stating as such, because a reader
meeting "a pair" will assume none of them:

**The two components have different closure properties** (`140` section 2, one expert, carried
forward by `141` and unchallenged since). The assignment space is closed and enumerable because an
axis position with no lowering cannot be supplied from outside. The weighting space is open and
consumer-supplied because a weighting re-ranks arms that already exist.

**The components are not independent.** `145_probes/z3` C4: at `overflow = wrap` all four lowering
arms in the topic's own arm set conform to every intermediate position, because the absorption
theorem (`141` F3) makes fused and stepwise the same denotation there; at `overflow = sat` the four
split two and two. So **the number of arms component two ranges over is itself a function of
component one**, and at some assignments a weighting has strictly fewer things to choose among.

**A weighting travels and an arm does not** (`144` section 4.2, F144-11). Carrying a weighting to a
new target costs exactly zero by construction; carrying an arm costs a median 128.2% relative regret.
The consequence for this panel's own notation is `144`'s and it is sharp: a finding whose region names
a specific arm has silently acquired a target dimension it did not list.

---

## 3. The five holes

### 3.1 The count's second argument: closed, with a quantifier the closure needs

`143` section 3 adds an argument to `141`'s joint statement: the count `|A / ~_O|` has two arguments
that move, because the assignment set `A` is parameterised by where the denotation and realisation
levels are cut, and cutting the intermediate axis moves `140`'s count from 24 to 12 or 14. It concludes
a shape-to-count table "is not currently writable".

`144` section 8 answers with F144-16, a biconditional over 96 cells: cutting an axis out of `A`
changes the count **iff** that axis is answer-visible under `O`. Where invisible the cut is a no-op;
where visible, cutting it puts two denotations under one type, which the firewall forbids. So the
second argument is a no-op or forbidden with no live case between.

**The arithmetic closes it and the statement needs one thing `144` does not supply.** `144`'s own
section 8 reports that the same cut is a no-op under one observation set and not under another, so
"answer-visible" is a property of an **(axis, observation set) pair**. The design-level claim needs an
axis-only property, so it needs a quantifier over `O`, and which one it gets decides whether the
closure holds.

`145_probes/z1` supplies it. **Visibility is monotone in the observation set**: 432 ordered triples,
zero violations, with a control firing at 55 instances where enlarging `O` makes an axis visible, so
the monotonicity is not vacuous. Monotone therefore **saturates**, and visibility under the maximal
observation set coincides with visibility under any observation set, checked at every cell. A dead
axis whose positions are the same function is invisible at every `O` including the maximum, which is
the second control.

And the two definitions of "visible" coincide: 144 cells compared between "some two assignments
differing only in this axis are distinguished under `O`" and "cutting this axis changes the count
under `O`", zero disagreements. So `144`'s biconditional and my monotonicity argument are about the
same property.

> **The closure holds, quantified over the maximal observation set the design admits.** An axis
> visible there is component one and cutting it is forbidden; an axis invisible there is dead and
> cutting it is a no-op at every observation set. The count has one argument that moves, the
> observation set, plus the axis set the design ships.

**What does not close, and it is a different question from the one `143` asked.** The maximal
observation set is the set of operations the design ships, which is a design act. So the shape-to-count
table waits on the operation set rather than on a denotation line, and `143`'s conclusion that the
table is not writable today is right for a reason `143` does not give: not because the denotation line
is open, but because the operation set is not named. Those are not the same open question and the
second one closes by a decision rather than by an argument.

### 3.2 The withdrawn instance: what rests on it, and what the emptiness is a fact about

`144` withdrew its own best real-world result. `p10` found exactly one non-dominated linearly
unreachable arm in the committed corpus, `p10b` verified it by a second procedure, and then its last
control found the verdict rests on a single coordinate where the deciding gap is 3.8 ns against a
79.2 ns interquartile range. `p10c` rebuilt the gate pairwise, because dominance and selectability are
pairwise questions, and found zero established instances corpus-wide (F144-18).

**Stated plainly, what rests on there being an instance:**

- **O-139-C closes**, in the direction `139` named as the alternative. The linear selector is
  sufficient for every arm set the library has today.
- **`144`'s O-144-A becomes the option it says it is** rather than a repair: ship the augmented
  Chebyshev selector, or keep the linear one and record the limit as a named predicate. `144` says
  the honest default is linear and reopens the moment an instance appears, and I agree.

**What survives without an instance, all of it:**

- **F144-6**, that an augmented weighted Chebyshev selector reaches every non-dominated arm. A
  construction with a certificate weight, verified at 3667 of 3667, so it does not need an instance.
- **F144-5**, the 6.6% to 12.7% rate in random cost tables. A fact about random tables.
- **F144-9**, the median 17.6% the limit costs in the unreachable arm's own criterion. Conditional on
  an instance existing, and stated that way.
- **F144-7**, that the selector const-evaluates to one unconditional branch. A doability result under
  I15, and doability does not wait on a use case.

**And the emptiness is partly a fact about the corpus and partly about the gate**, which `144` does
not separate and which changes what may be concluded. `145_probes/z2` asks the prior question: at gate
multiplier `k`, how many arms in a family are still **established** non-dominated? Unreachability is a
question about the non-dominated set, so where that set collapses to one arm the question is unaskable
whatever the geometry is.

**Under the conservative gate reading, 29 of 35 families still carry two or more established
non-dominated arms at `144`'s `k = 0.5`.** So `144`'s zero is a real statement about 29 families and
not about six of them. That is the better half of the answer and it strengthens `144`.

**And my own prediction was refuted, in a way that is a finding about the gate.** I predicted the
count falls monotonically in `k`. It does not, and the mechanism is that a gated dominance test has
two comparisons pulling opposite ways: raising `k` makes "a is established worse here" harder, which
disqualifies fewer dominators, and makes "a is established better somewhere" harder, which qualifies
fewer. Under the symmetric reading where both halves are gated the count falls from 3.03 to 1.77 and
then rises to 2.23, and the askable corpus is 20 families rather than 29. Under the conservative
reading, where the strict half is gated and the no-worse half is read on medians, it is monotone
non-decreasing, which is what a gate should do.

> **F144-18's zero depends on a direction of conservatism its statement does not name**, and the two
> readings give 29 askable families and 20. Both controls fire: a well-separated synthetic family
> keeps its three non-dominated arms at every `k`, and a family of exact duplicates keeps all four.

### 3.3 The join, and the condition that makes it enforceable

`144` F144-15 is the two components meeting, and it arrived in the last file of the topic. With arms
conforming to one policy a target change moves the selection and moves no answer, 0 of 192,512 cases.
With non-conforming arms it moves answers at the arms' entire disagreement rate, 42.14% of inputs at
`F = 0`, "and nothing in the type says so".

**That last clause is not a property of the firewall.** `142` F142-1 already established that the
non-conforming pair in question **is** the two positions of the intermediate axis, over 6,356,992
triples with a cross-pairing control differing at 757,954. So the type can say so, if the assignment
set carries the axis.

`145_probes/z3` turns that into a condition and checks it:

> **Every lowering arm the design admits realises the denotation of some assignment in the assignment
> set.**

Under that condition the firewall is enforceable by declaration alone, with no mechanism. A conforming
pair is two arms of one assignment and the weighting picks between them freely, moving no answer. A
non-conforming pair is two assignments and the type names which one, so the 42.14% is a **declared**
difference rather than a silent one.

Checked over four lowering arms and eight assignments at five shapes: every non-control arm realises
at least one assignment. The control fires: an arm rounding with a mode the axis set does not carry
realises **no** assignment at every shape where rounding fires at all, and for that arm the firewall
has no repair available in the type. The pairing control holds, at zero differences for the conforming
pair and 20.02% to 28.47% for the non-conforming one at `W = 4`.

**So the join is: the firewall is a proposition about cost models, and it is enforceable exactly when
the assignment set is rich enough to name every arm the design ships.** Enriching the assignment set
is the price of enforcement, and it is the same price `141` replacement C and `142` section 7 already
identified from the other side.

### 3.4 The equivariance predicate, and what it does to a closed topic

`142` F142-2 partitions the rounding axis by translation equivariance, three modes against three, and
`142` section 4 says naming the modes is "necessary and not sufficient" because the property an arm's
predicate reads is not recoverable from a mode's name. That reaches the rounding topic, which is
closed with its own candidate at `132`, revised at `136`.

The question that decides what may be said about a closed topic without editing it: **is translation
equivariance a new property, or a Boolean function of the laws that candidate already enumerates?**

`145_probes/z4`, over eleven rounding modes including two constructed ones:

- **The order bound implies equivariance.** Zero modes carry the bound without it. Both adjoints of
  the grid inclusion commute with integer translation.
- **Negation symmetry excludes equivariance**, and the argument is two lines rather than a sweep: if
  both held then `rnd(-1/2) = -rnd(1/2)` by symmetry and `rnd(-1/2) = rnd(1/2) - 1` by equivariance,
  so `rnd(1/2) = 1/2`, which is not an integer. **This is a second exclusivity of exactly the shape
  `132` section 5.3 already records** for the order bound against negation symmetry.
- **Among modes carrying neither, it is undetermined.** Witnessed by construction only, on the third
  attempt: `half_up`, `half_down` and a perturbed `half_up` share the law signature
  `(no order bound, no staged composition, no negation symmetry)` and the first two are equivariant
  while the third is not.
- **On the ten natural modes alone the coincidence is perfect**: equivariance holds exactly where
  negation symmetry fails.

> So a design reading the rounding candidate's law table gets the right answer on every mode it would
> plausibly ship, and has no argument that it will keep getting it. That is the precise content of
> `142`'s "necessary and not sufficient", and it is a gap in what the closed candidate happens to
> cover rather than an error in it.

**And it is this topic's finding rather than the rounding topic's**, which is why it belongs here and
why nothing in `132` or `136` is edited. The rounding topic asked what the axis selects. This topic is
where an arm's predicate is written, and an arm relocating a rounding across an integer addition reads
a property that the enumeration determines in two cases of four.

### 3.5 The shared baseline: one word, two operations, and a conflict inside the repair

`144` section 6 gives the obligation its second read after three files: `140`'s "every strategy's cost
claim is stated against the same named arm" is right and under-specified, because a cost is a vector
and where the division happens decides what the division does. Per coordinate before the weighting it
changes which arm a fixed weighting picks at 24.6%; once on the weighted scalar, at 0.0%.

Three things `145_probes/z5` adds, and one of them is a conflict inside `144` itself.

**The 0.0% is a theorem and needs no evidence.** Dividing every arm's weighted sum by one positive
constant preserves the argmin, so nothing could have come out otherwise. 5460 cases, zero changes.
`144`'s number is the theorem showing rather than a measurement.

**The per-coordinate form is exactly the substitution `w -> w/b`.** 5460 pairs checked, zero
disagreements between "normalise each coordinate then weight by `w`" and "do not normalise and weight
by `w/b`". So it is a reweighting **by identity** rather than by measurement, and the 24.6% is how
often that substitution crosses a cell boundary. And a uniform baseline is not merely sufficient for
it to be a no-op, it is necessary: every non-uniform baseline tried moved a selection on the first arm
set drawn.

**Sections 6 and 4.3 of `144` conflict as stated, and I reproduced its number doing it.** Section 4.3
requires the per-coordinate form, because it is what drives the cross-target switch rate to exactly
zero on a pure per-coordinate rescale. Section 6 forbids it, because it changes which arm a fixed
weighting picks. My own sweep puts that at **894 of 3640 cases, 24.6%**, matching `144`'s figure on an
independently written instrument. Both sections are correct about the same operation and nothing says
how a design does both.

**The conflict dissolves if the per-coordinate division is read as a declaration of the weighting's
units rather than as a transformation of the cost table.** A weighting declared in baseline-relative
units selects by `argmin_i sum_k w_k c_ik / b_k` **by definition**, so there is no absolute-units
weighting it was supposed to agree with. Measured: across a pure per-coordinate rescale it moves at 0
of 3640 cases, and across an arbitrary target change at 2892 of 3640, which is the control saying it
is a change of units rather than a claim that targets do not matter.

> **The design owes a declaration of which units a weighting is expressed in.** Once it declares that,
> both of `144`'s requirements hold at once and neither is violated. The 24.6% is the disagreement
> between two questions rather than one question answered wrongly.

---

## 4. The arms, with predicates and argument kinds

Each arm is a region and what holds there. Per I16 none of them names a construction; the shape is
whoever builds it. Every predicate lists only what was established, and a dimension not listed claims
nothing where that dimension is present.

### A1. Fusing a multiply-add is a free lowering where the axes make it answer-preserving

> Where the fused and stepwise realisations of a multiply-add compute the same function, fusing is a
> lowering choice and the weighting may take it. That region is not a single mode: under unsigned
> range policies it holds for every rounding position by the one-sided-clamp congruence, and under
> wrapping it holds exactly for the **translation-equivariant** rounding positions.

*holds for: W = 6; F in {0, 1, 2, 3, 4, 5}; signedness = unsigned; overflow in {wrap, saturating};
rounding in {floor, ceiling, toward zero, away from zero, nearest-half-up, nearest-half-even};
operation = multiply-add; arity = 3; chain length = 2; container width = declared width; threads = 1;
target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f).*

*and holds for: W = 6; F in {0, 1, 2, 3, 4, 5}; signedness = signed; overflow = wrap; rounding in
{floor, ceiling, nearest-half-up}; operation = multiply-add; arity = 3; chain length = 2; container
width = declared width; threads = 1; target features = host.*

**Argument kind: closure for the unsigned half** (a one-sided clamp of a monotone operation is a
congruence, so reducing early and late land in the same place), **and equivariance for the signed
wrapping half** (`142` F142-3, six of six modes agreeing with the partition, resting on `141` F3's
absorption theorem for the reduction). The equivariance property itself is `142` F142-2.

**What the predicate deliberately does not say.** It names three rounding positions rather than the
property, because the property is const-checkable and the enumeration is not the property. A design
carrying more than six modes reads section 3.4 rather than this list.

### A2. Where fusion changes the answer it is not a lowering, and the axis already names it

> The fused and stepwise realisations at signed saturating are two denotations, not two ways of
> computing one. A consumer wanting the fused answer selects the exact-intermediate position, keeps
> full determinism, and gets the fast arm. No slack mechanism is required and the residue the
> mechanism was built for is empty.

*holds for: W in {4, 6}; F in {0 .. W-1}; signedness in {unsigned, signed}; overflow in {wrap,
saturating}; rounding = truncate toward zero; operation = multiply-add; arity = 3; chain length = 2;
container width = declared width; threads = 1; target features = host.*

**Argument kind: extensional identity**, established twice. `141` F7 built both arms and identified
them; `142` F142-1 found the same identity between two of `139`'s own committed probes, over 6,356,992
triples, with a cross-pairing control at 757,954 differences.

### A3. The storage container is free at the column

> A lossless container contributes no distinguishable answer functions, so the storage-minimising
> concern has zero policy content where a value is put. It composes with every assignment there.

*holds for: W = 4; F in {0, 1, 2}; operations in {add, subtract, multiply}; overflow in {wrap,
saturating}; rounding in {toward zero, floor}; overflow limit read at the declared width; threads = 1.*

**Argument kind: exhaustive enumeration, three independent instruments.** The predicate is the
**intersection** of what the three ranged over, computed in `145_probes/z6` and not the union: `139`'s
signed bitstream round trip at arbitrary offsets, `140`'s unsigned partition over rungs, and `141`'s
partition with the accumulator pinned. Ten dimensions appear in the union and not in the intersection,
including `signedness`, which only two of the three varied.

### A4. The accumulator width is free except at one cell

> Narrowing an accumulator is a cost choice everywhere except at signed saturating, where it is a
> policy choice and changes answers.

*holds for: W = 4; F in {0, 1, 2}; signedness in {unsigned, signed}; overflow in {wrap, saturating};
rounding in {toward zero, floor}; accumulator width varied above the declared width; threads = 1;
target features = host.*

**Argument kind: exhaustive enumeration, three independent instruments**, and the predicate is again
the intersection: `141` F2 added an accumulator dimension to an operation set, `142` F142-5 built a
fold over a sequence with length as a dimension, `143` F1 built a fold with three kernels. Fourteen
dimensions are in the union and not the intersection.

**And the cell's reachability carries a dimension none of the three had until `142` found it.**
`142` F142-6: same cell, same widths, same domain, the accumulator is visible at 672 cases with the
largest multiplier first and at 0 with it last. So the cell is real and whether a kernel enters it is
a property of the accumulation schedule. `142`'s conservative reading is the one a substrate takes
under `arvo-toolbox-not-policer.md`, and I record its precise reading beside it rather than choosing.

### A5. The selector reaches hull vertices, and the wider selector is available and unneeded today

> A linear weighting selects an arm exactly when that arm is a vertex of the lower convex hull of the
> arm set. An augmented weighted Chebyshev selector reaches every non-dominated arm instead, at the
> same compile-time cost and the same portability.

*The linear limit holds for: cost tables drawn uniformly from integers 1..20; arms in {4, 6, 8}; cost
coordinates in {2, 3, 4}; 120 tables per cell; no exact duplicate arms; decision procedure = exact
vertex enumeration; threads = 1; target features = host.*

*The Chebyshev construction holds for: the same table population; reference point = componentwise
minimum minus one; certificate weight proportional to the reciprocal deviation; augmentation
coefficient = 1/1000; threads = 1; target features = host.*

*The lowering holds for: selector in {linear, weighted Chebyshev}; arms = 3; cost coordinates = 2;
integer weights and integer reference point; selection by associated const; dispatch inline(always);
arms inline(never); opt-level = 3; no feature gates beyond the stable language; threads = 1;
target = aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f.*

**Argument kinds, and they differ per clause, which is why they are separated.** The limit is
**exact linear programming over rationals**, decided rather than swept, which is the one result in
this topic that is an exact certificate over a continuum: `144` F144-2 puts `139`'s compromise arm at
`+1` over the whole simplex rather than at 2001 sampled points. The reachability of every non-dominated
arm is a **construction with a certificate**, so it is decided per arm rather than searched. The
lowering is **emitted-assembly inspection with a runtime-selected control** retaining all three arms
and three conditional branches.

**And the arm is not currently needed**, per section 3.2: zero established instances in the committed
corpus, on 29 askable families under the conservative gate reading.

### A6. The shared baseline is two operations and the design declares which

> Applied once to the weighted scalar it is a reporting normalisation: it preserves the argmin, so it
> changes no selection, and it is what makes two strategies' cost claims comparable. Applied per
> coordinate before the weighting it is a change of the weighting's units, identical to the
> substitution `w -> w/b`, and it is what makes a weighting travel across a per-coordinate rescale of
> the target.

*The scalar placement holds for: cost coordinates any; arms any; weights any positive; baseline any
arm with positive weighted cost; threads = 1.*

*The per-coordinate identity holds for: cost coordinates = 3; arms = 7; cost tables drawn uniformly
from integers 1..60; weight grid resolution = 1/12 on the 2-simplex; baseline any arm with positive
coordinates; threads = 1; target features = host.*

*The no-op condition holds for: baseline uniform across coordinates, and fails for every non-uniform
baseline tried; cost coordinates = 3; arms = 7; threads = 1.*

**Argument kinds.** The scalar placement is an **order-preservation theorem**: scaling by a positive
constant preserves the argmin, so its predicate is as wide as the algebra and lists no width, no
signedness and no table population. The per-coordinate identity is an **algebraic identity**, checked
rather than discovered. The no-op condition is **exhaustive over the swept population in one direction
and existential in the other**, so it is stated as necessity by witness rather than as a theorem.

### A7. The firewall is enforceable by declaration under a checkable condition

> No cost model may move an answer. That is enforceable with no mechanism beyond declaration exactly
> when every lowering arm the design admits realises the denotation of some assignment in the
> assignment set. Where it does, a target change moves the selection and no answer. Where an arm
> realises no assignment, the firewall has no repair available in the type.

*The condition holds for: W = 4; F in {0, 1, 2}; signedness in {unsigned, signed}; assignment set =
rounding {floor, toward zero} x overflow {wrap, saturating} x intermediate {stepwise, exact}; arm set
= {fused by widening, fused by partial products, stepwise by shift, stepwise by partial products} and
a control arm rounding off the axis; operation = multiply-add; arity = 3; inputs exhaustive over the
declared range; container width = declared width; threads = 1; target features = host.*

*The consequence holds for: W = 6; F in {0, 3}; signedness = signed; overflow = saturating; rounding =
truncate toward zero; cost coordinates = 3 with two committed synthetic tables; selector = linear;
weight grid resolution = 1/24; inputs exhaustive over the declared range; container width = declared
width; threads = 1; target features = host.* (`144` F144-15.)

**Argument kind: closure with a firing control.** The condition is checked by exhaustive extensional
comparison; the control is an arm built off the axis set, which realises no assignment wherever
rounding fires and which is the case the design must forbid.

---

## 5. The convergences, with what each instance actually ranged over

Per section 1, each names the intersection rather than the union, computed in `145_probes/z6`.

| convergence | instances | intersection of dimensions | in the union only |
|---|---|---|---|
| the container is answer-invisible at the column | `139` p6, `140` p3, `141` p4 | `F`, `W`, operations, overflow, rounding, threads | 10, including `signedness` |
| the accumulator is visible exactly at signed saturating | `141` F2, `142` F142-5, `143` F1 | `F`, `W`, accumulator, overflow, rounding, signedness, target features, threads | 14, including the schedule |
| the class count is monotone non-decreasing, not strict | `141` F8, `143` F2' | `F`, `W`, assignments, signedness, target features, threads, witness sets | 4, including accumulator and container |
| fusion is an axis position the design already has | `141` F7, `142` F142-1 | `F`, `W`, arity, chain length, operation, overflow, rounding, signedness, target features, threads | 4 |
| the firewall proposition | `139`, `141`, `142` | **none: the proposition carries no predicate** | not applicable |

**Two of these are three instances and two are two**, and the rungs are the files' own. What I add is
that the third row's two instances differ on four dimensions including the accumulator, so the
monotonicity convergence is about a region in which the accumulator is pinned at `140`'s value in one
instance and unstated in the other; and that the last row is not a convergence of instruments at all.

**And one of the three-instance rows is a theorem rather than a measurement.** `141` section 5.1
proves the monotonicity: if `O1` is a subset of `O2` then equality on `O2` implies equality on `O1`,
so the `O2` partition refines the `O1` partition. `141` says so itself rather than presenting a theorem
as a discovery, and `143` adds the anti-monotone control that converts its own zero from a caveat into
a checked property. My `z1` A1 is the same theorem restricted to one axis, so it is a third instance of
a proof rather than a third measurement, and I record it that way.

---

## 6. What the canon should state

Intents rather than mechanisms, per `the-canon-is-intent-not-implementation.md`, and each of these
survives a rewrite of every implementation.

**A strategy is a pair whose components answer to different tests.** One fixes what is computed and
is checked by whether an input exists on which the answer moves. The other selects among ways of
computing it and is checked by measurement. The membership procedure is mechanical and belongs in the
canon: vary the candidate over the whole domain with everything else held, and see whether an answer
moves. It is stated over **chains** rather than per axis, which `139` conceded against itself and
`141` supported from its own T5.

**The count is not a number the canon states.** It is the cardinality of an assignment set quotiented
by observational equality over an observation set, monotone non-decreasing in that set and not
strictly increasing, well defined and neither a single number nor an absence. What the canon owes is
**the axes and their positions**, which are finite, arvo's and permanent. `140`'s permanence test is
the argument: "there are four strategies" fails the moment anyone adds a fifth; "the observable policy
axes are these, with these positions" survives every rewrite.

**No cost model may move an answer.** The firewall, at two experts as a proposition, with `144`
F144-15 as its first predicated measurement and section 3.3's condition as what makes it enforceable
without a mechanism.

**A weighting travels and an arm does not.** Two measured sentences, and the second is what makes a
predicate naming an arm target-bound.

**A weighting is expressed in declared units, and the design says which.** Section 3.5. Without it the
same named baseline is two operations and the design will use one where it means the other.

**A cost coordinate exists only if every arm carries a value on it and the arms differ on it.**
`139`'s measurement obligation plus `144`'s F144-4: a coordinate the arms do not differ on makes every
arm weakly selectable and hands its region of weight space to the tie-break rule.

**Whether the storage-minimising concern reaches an accumulator is a policy question at exactly one
cell.** I17 is untouched by this: the concern is not deprioritised by being principally a weighting,
and `139` says so directly. What changes is that its scope has a named boundary rather than being
"storage".

---

## 7. Where I disagree, kept separate from the formalisation

Three, each with a measurement, none of them reopening a converged result.

**`144` sections 6 and 4.3 conflict and the file does not say so.** Section 3.5 and `z5` E4, at 894
of 3640 cases. The dissolution I offer is mine and is one expert's; it is an argument about what a
weighting's units are rather than a measurement, and the measurement in it only confirms that the
units reading behaves as a units reading should.

**`144` F144-18's zero rests on a gate whose direction of conservatism is unstated**, and the two
readings give 29 askable families and 20. Section 3.2 and `z2`. This does not refute F144-18; it
narrows what may be concluded from it, and under the reading I take as correct it strengthens rather
than weakens the finding.

**My own prediction that the established non-dominated count falls with gate strength was refuted**,
and the mechanism is that a gated dominance test has two comparisons pulling opposite ways. That is a
finding about how such a gate has to be written, and it applies to `144`'s as much as to mine.

---

## 8. What only op decides

**Whether the design ships a selector that names arms or one that names hull vertices.** `144`
O-144-A. The measurement says the honest default is linear today and that the question reopens the
moment an established instance appears. Which of the two the canon says it is doing is not derivable
from anything measured.

**Which units a weighting is declared in.** Section 3.5. Both readings are coherent and they answer
different questions; the design has to pick one and say so.

**Whether the accumulator cell is stated conservatively or with a schedule dimension.** `142` O-142-B.
Conservative means a type says visible and a consumer who knows their schedule cannot claim the
cheaper reading; precise means a predicate a kernel gates on and a type cannot.

**Whether the default rounding position is chosen for familiarity or for what it licenses.** `142`
O-142-C, and it is sharper after section 3.4: the IEEE default is on the non-equivariant side, so a
design defaulting to it has every relocation unavailable at the default.

**How many named presets there are and what they are called.** Not derivable from anything in this
topic, and both cold derivations reached that independently. I1 is already demoted to open.

---

## 9. Coverage, bounds, and what I did not do

**Read in full:** `139`, `140`, `141`, `142`, `143`, `144`, `INTENTS.md`. **Read in part:** `RULES.md`
at its predicate section, `OPTIONS.md:2425-2461` only. **Opened at source rather than recalled:**
`137_probes/g0_test_gate.out`; the committed CSVs under `mock/benches/`, 254 of them; the predicate
blocks of all six files, via `z6`.

**Not read:** `40`, `93`, `102`, `106`, `107`, `108`, so where Q51 compresses them I am relying on
Q51's account exactly as `141`, `143` and `144` do. `132` and `136` beyond the sections `142` cites and
the law enumeration `z4` tests. The probe sources of `139` through `144`, except `144_probes/p10c`'s
gate shape, which I read to make `z2` comparable.

**Every measurement here is at model widths**, `W` in {4, 6}, and I have no transfer argument to 64
bits and am not offering one. **Everything is `threads = 1`**, so under the notation none of my
findings holds anywhere threads exist. **Container width equals declared width** in every fixed-point
instrument of mine, the narrowing `139` reported against itself and every later file inherited.

**I priced nothing.** No claim here is a bench result and none is called one. `z2` reads committed
harness output and does arithmetic on it; it takes no measurement. Whether the Chebyshev selector or
a richer assignment set costs measurable compile time is **unpriced**.

**I did not attack the two cold derivations' probe sources**, and I did not re-derive `141`'s
absorption theorem, `144`'s Pareto LP, or the fusion tables. Where I use them I cite them.

**Predictions that fell: three.** `z2` B2, that the established non-dominated count falls
monotonically with gate strength, refuted with a mechanism that turned into a finding. `z4` D3's first
constructed witness, which came out negation-symmetric and settled nothing, and its second, which
broke both properties but landed alone in its law signature and so had nobody to disagree with; the
third attempt worked and is marked as constructed.

**Every citation above was opened and its content tested**, not merely resolved, by
`145_probes/z7`: 34 citations, 34 ok, and the checker is mutation-tested three ways with all three
mutants caught (a phrase nobody wrote, a real phrase in the wrong file, a real phrase at the wrong
span). **Two failed on the first run and both were verbatim correct**, defeated by the source's own
markdown inside the quoted phrase, which is the third instrument in three of my files defeated by
markup in the span it was reading; the normaliser now strips ticks and emphasis on both sides and the
mutants confirm that cannot manufacture a hit.

**Two defects in my own instruments, recorded rather than repaired quietly.** `z4`'s verdict block
asserted D3's predicted outcome while the run reported it not established, which is the same defect I
named in `136` section 9 and which I had to be caught by my own output to see; it now prints the
measured value. And `z2`'s first gate reading conflated two directions of conservatism, which is what
refuted B2 and which I kept as a second implemented reading rather than deleting.
