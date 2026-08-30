# 57. The grading and the refutation

**Date:** 2026-08-09. **Position:** file three of unit two on the format-concept topic, after `55`
derived it cold, `56` attacked, and `55b` replied. Two jobs: adjudicate the refutation `55b`'s `p5`
raises against `42`, and second-read the grading result `55b`'s `p4` reports.

**Probes:** `57_probes/`, six of them, sources and outputs committed, each printing its own
instrument-validation block and exiting nonzero on failure. `57_probes/RUN.md` is the build line.

**Re-run before argued with.** `42_probes/p3`, `55_probes/p4`, `55_probes/p5` and `56_probes/q1` were
rebuilt on `nightly-2026-05-28` and diffed against their committed outputs. All four byte-identical,
regenerated outputs in `57_probes/rerun/`. Every count I take from any of them is a count I
regenerated.

**Read for this file:** `00_brief`, `INTENTS`, `42` in full with `42_probes/p3`'s source and output,
`55b` in full, `56` section 3.3 with `q1`'s source and output, `55_probes/p4` and `p5` sources and
outputs, and `OPTIONS.md`'s Q6, Q11 and Q12 entries. **I opened `42` and its probe directly and
before `55b`'s account of either**, which is what this dispatch was for. I did not open `35`, `18`,
`20`, `25`, `40` or `43`, and two findings below are bounded by that and say so.

## Status of this file

Nothing here settles anything, per the standing explore mode. What it does: refutes one criterion in
two of its readings and offers a replacement measured as an exact biconditional, confirms `55b`'s
semiring at nine widths and gives it a proof-shaped reason, bounds that semiring to a condition
`55b` did not state, separates two things the word "grade" is carrying, and measures a grading that
is real. One of my own probe hypotheses was refuted by my own probe and is on disk with the refutation
attached.

## 0. Gates

**The canon gate passes, in the second of `expert-dispatch-defends-the-canon.md`'s three situations.**
There is no canon; a panel is writing one, `00_brief.md:8-9` says so, and `INTENTS.md`'s own section
on how to read an entry records that no entry currently holds the ratified rung. So there is nothing
to defend, and nothing here may close a question.
What I checked the work against: `INTENTS.md` (the strategy set is open per I1, and I9 puts the
strategy at the position where it changes what the correct answer is), the acceptance criterion at
`00_brief.md:144-146`, and the forbidden-feature list at `00_brief.md:158-160`. Nothing in this file
proposes a mechanism, so the forbidden list is not engaged; every probe is plain integer arithmetic
with no type-level machinery at all, deliberately, so that nothing here is an artifact of a
representation choice.

**The test gate has no suite to run.** `mock/crates` is empty, which the panel instructed (`00_brief.md:167-169`) and which I confirmed, so there is no test surface to audit and reporting one would be a fabrication.
The panel's substitute is the probe discipline, and I applied it to the four instruments I depend on
by re-running them rather than citing them.

**One thing I will flag under the standing instruction to report unlicensed mechanisms even outside
the question.** The repository working tree carries deletions of the entire `docs/` tree and
modifications to committed bench artifacts under `mock/benches/` (`git status` at the head of this
dispatch: 24 deleted or modified `docs/` paths, plus modified `warm-clamp-arity-l2_n321040.csv` and
its `.meta.json` and `_findings.md`, plus three untracked new bench artifact triples). **A modified
committed bench artifact is a modified measurement record.** Under
`evidence-lives-in-the-repo-or-it-never-happened.md` the artifact trail is the thing that makes a
bench citable at all, and `20` and `22`'s numbers are cited live in `OPTIONS.md`'s Q6 and Q7. I did not
touch any of it, I do not know what changed it, and I am not the right person to adjudicate it. It
should be looked at before anyone cites those CSVs again.

## 1. The answer, before the working

**Job one.** `42`'s stated condition is refuted, and `55b`'s `p5` is not what refutes it. **`42`'s own
probe refutes it, in row one of its own four-block table, and did so before `55b` existed.** The
register's paraphrase is refuted too, and by the same row. What survives of `42` is its investigation,
its refutation of the clamp-counting hypothesis, and the H2 it wrote into its probe's comments but not
into its prose. `55b`'s `p5` is an independent re-derivation of `42` row 1 in the signed setting with
a finer instrument, and its counts are right.

The replacement criterion is **absorption**: the reduction `rho` is absorbing over an operand box when
`rho(rho(x) + y) == rho(x + y)` for every reachable exact sum `x` and every operand `y`. Measured
against 4248 configurations of clamped addition, absorption and associativity agree **exactly, with
zero violations in either direction** (`57_probes/p2_output.txt`). `42`'s condition read per fold
mispredicts 306,024 divergent triples over the same sweep; read per domain it mispredicts 1749 of the
4248 configurations. This is not a refinement of `42`. It is a different predicate that happens to
coincide with `42`'s on the cases `42` measured last.

**And absorption is not new to this panel.** It is `56`'s coherence law (`56_probes/q1`'s C-law),
which on this domain is the same predicate. So `56` was holding the correct criterion while
describing it, at `56:214-216`, as the coarser statement deferring to `42`'s finer one. **The ordering
is backwards.** Coherence is the correct statement and `42`'s is false, under both of its readings.

**Job two.** `55b`'s semiring is confirmed and it is not a coincidence of four bits: every
commutative-semiring axiom holds at M = 1, 2, 3, 7, 15, 31, 63, 127 and 255, exhaustively
(`57_probes/p3_output.txt`). More than confirmed, it is explained: `x ~ y iff x == y or both >= M` is
a congruence on the naturals for both operations, measured at five widths with zero violations, so
the saturating algebra **is** the quotient `N/~` and inherits every axiom rather than satisfying them
by luck. A congruence argument covers all widths at once in a way an axiom sweep never can. M = 1 comes
out as the Boolean semiring, which is a structurally different instance and is the sanity check I most
wanted.

**And it is bounded in a way `55b` does not state and the canon would have to.** The semiring is a
fact about the **integer grid**. At every fractional scale measured it collapses: nine of nine
configurations at F = 1, 2, 3 across M = 15, 31, 63 fail multiplicative associativity and
distributivity, while additive associativity survives everywhere. A law layer stating "unsigned
saturation is a semiring" without the scale condition would licence product reassociation and
distribution rewrites that are wrong on every fractional format arvo has.

**The blame lands on rounding, not on saturation**, which `42:191` asserted and `57_probes/p4` measures
by running the two factors separately: the range clamp alone is clean at every scale, the grid
coarsening alone breaks multiplicative associativity and distributivity, and the composite inherits the
coarsening's failure.

**On the word "grade".** `55b` uses it for a **ladder of algebraic strength**, ring over semiring over
magma. That is a partial order on theories. It is a real and useful classification and it is a
different object from a grading, which needs an index that composes. I went looking for a
compositionality failure to make the distinction bite and **did not find one**: over twelve
configurations the composite's law set equalled the meet of its factors' exactly. So the ladder came
out stronger than I dispatched myself to show, by my failing to refute it.

**There is a genuine grading here, and it is the precision axis rather than the law axis.** Measured:
widths are not a grade monoid for addition, since `max(W,V)+1` is not associative and
`g(g(5,0),0) = 7` against `g(5,g(0,0)) = 6`. What composes exactly is the reachable **interval**, under
Minkowski sum, exact at every fold length measured, with the width a sound but increasingly loose image
of it. And the grading has an operational form that is the useful one: divergence from the
exact-then-adapt answer as a function of accumulator width, which reaches zero at a computable index.

**The theorem the two jobs share.** Coherence is exactly the statement that **the grading collapses**.
For a coherent policy the accumulator grade is the format's own width and no widening is ever needed:
across sixteen unsigned rows spanning W = 3 to 5 and fold lengths 2 to 8, eager saturation in the
format itself agrees with exact-then-adapt at every accumulator width including the narrowest
(`57_probes/p6_output.txt`). For an incoherent policy the grade is real and costs bits. That is what
`56`'s two law families are, stated as one fact rather than two bundles.

**One measurement worth more than the framing around it.** The accumulator grade an incoherent fold
needs is **one bit less than the width that holds the exact sum**, uniformly, in all fifteen rows where
the question arises, across signed formats W = 3 through 6 and fold lengths 2 through 8. The final
adaptation absorbs the outermost bit, because the accumulator has to decide which side of the format
the result fell on rather than represent how far outside it fell.

## 2. Job one: the refutation, adjudicated

### 2.1 What `42` actually says, read at the source

`42:314-316`, in full and verbatim:

> The corrected hypothesis, tested as the discriminating case in the same file's second half:
> **associativity of a clamped operation holds exactly when at most one of its clamps can be triggered
> by any association order of the specific fold in question**, not when the code has only one clamp
> written.

`56:203-204` quotes that accurately. The register's Q12 compresses it further, to "associativity
survives exactly when the fold's actual trajectory cannot reach both clamped endpoints"
(`OPTIONS.md`, Q12's mechanism paragraph), and the register itself now records that the dispatching
agent wrote that compression in from a summary.

So there are two artifacts, not one: `42`'s sentence and the register's compression of it. **Both
fail**, and it matters which, so the repair lands in the right place.

### 2.2 `42`'s sentence has an unstated quantifier, and it is false under both readings

The phrase "the specific fold in question" does not say whether a fold is one operand sequence or a
family of them, and the condition's truth value differs. Both readings are measured in
`57_probes/p1` and `57_probes/p2`.

**Per fold**, over one triple: the condition says a triple cannot diverge if at most one clamp face is
triggered across its association orders. Over `p2`'s 4248-configuration sweep of clamped addition,
**306,024 divergent triples trigger at most one face**. Every one is a counterexample.

**Per domain**, over a whole operand box, which is the reading `56:208-211` adopts as the window form:
the condition says a box is associative if at most one face is reachable anywhere in it. It
**mispredicts 1749 of 4248 configurations**.

### 2.3 The counterexample was already inside `42`, in the row `42` published

This is the finding, and it is why the answer is none of the three the dispatch offered.

`42`'s table at `42:319-324` has four rows. Row one is a **ceiling clamp with no floor at all**, over
operands of both signs, and it measures **904 of 3375 associativity failures**. `42_probes/p3.out`
prints it, and I regenerated it byte-identically.

Under `42:315-316` as written, row one has exactly one clamp. At most one of its clamps can therefore
be triggered, trivially and unconditionally, because there is only one to trigger. The condition is
satisfied and it predicts associativity. The row measures 904 failures.

**`42`'s summary sentence is refuted by `42`'s own first row.** My `p1` reproduces that row with
per-divergence face attribution `42` did not take, and the attribution is unambiguous:

```
42 row1  ceiling only, operands both signs (top=3)   3375 triples   904 divergent
                                                     floor-only 0   ceiling-only 904   both 0
```

Every one of the 904 is a **ceiling-only** divergence, in a configuration with no floor. `55b`'s `p5`
finding of 448 ceiling-only divergences in the signed clamp is **the same phenomenon**, one sign domain
over.

How the sentence survived its own table: the table's third column is headed "is the floor structurally
reachable", and for row one it reads "yes, unconstrained". That column silently redefines "clamp" as
"boundary the exact excursion can reach", counting an **absent** clamp in an unbounded direction as a
reachable boundary. Under that redefinition the row is consistent; under the sentence's own word
"clamps" it is not. `42` is a careful file and this is the one seam in it, and it is a vocabulary slip
rather than a measurement error: **every number in `42_probes/p3` is correct and reproduces.**

### 2.4 The verdict, stated against the three answers the dispatch named

The dispatch offered three, and the true answer is a fourth that is sharper than any of them.

**Not "42 is refuted" simpliciter**, because `42`'s probe, its four rows, its refutation of the
clamp-counting hypothesis H1, and the H2 it states in `42_probes/p3`'s comments at lines 25 to 28 all
stand and all reproduce.

**Not "the register's paraphrase is refuted while 42 stands"**, which is the charitable answer and the
one I expected to land. `42`'s own words fail too, and they fail against `42`'s own data.

**Not "two instruments measuring different things, both hold."** They measure the same thing and one
of them is wrong.

**The verdict: `42`'s summary sentence is refuted, in both of its readings, by `42`'s own row one;
the register's compression is refuted by the same row; and `55b`'s `p5` independently re-derives that
row's mechanism in the signed domain with a better instrument.** `55b` was right that a second read
was owed and right about which direction the separation runs. It was wrong only in believing the
refutation was new, and it could not have known that without opening `42`, which it says it did not.

What `42` should have written is in `42_probes/p3` at lines 25 to 28, as hypothesis H2: operands
confined to a half-line that already contains the result associate, and letting the exact sum range
over both signs does not, with or without a clamp there. **H2 is not refuted by anything in this
panel**, `p5` included: `p5`'s Q = [-8, 7] is not sign-confined, so H2 predicts failure and `p5`
measures 952 of them. The prose generalised H2 into a clamp-counting sentence and the generalisation
is what broke.

### 2.5 The replacement, and what it costs to state

**A reduction `rho` is absorbing over an operand set B when `rho(rho(x) + y) == rho(x + y)` for every
exact sum `x` of elements of B and every `y` in B.** Sufficiency is one line: if it holds, both
association orders equal `rho(a + b + c)`.

Necessity is the half that could have failed and is the half a replacement needs, so it was swept
rather than sampled. `57_probes/p2` runs 4248 configurations per ambient operation, every combination
of floor in `{absent} ∪ [-6,6]`, ceiling likewise, and operand box `[blo, bhi]` with `blo ∈ [-5,0]`
and `bhi ∈ [0,5]`, each measured exhaustively over its box cubed:

| ambient operation | configurations | associative | absorbing | sufficiency violations | necessity violations |
|---|---|---|---|---|---|
| addition | 4248 | 1609 | 1609 | 0 | 0 |
| multiplication | 4248 | 1770 | 1617 | 0 | 153 |

**For clamped addition, absorption and associativity are the same predicate**, measured, with the
counts agreeing to the unit. For clamped multiplication absorption is sufficient and not necessary,
and `57_probes/p2b` characterises the exception rather than leaving it as a count: **all 153 are
configurations whose induced operation is constant**, where the clamp has collapsed the operation and
associativity is free. Residue: zero.

So the criterion, with its boundary stated: **absorption is equivalent to associativity for a clamped
addition, and equivalent for a clamped multiplication modulo operations the clamp has collapsed to a
constant.**

### 2.6 The closed form, and where the cheap version breaks

Absorption is a quantified predicate. For a monotone median clamp there is a closed form worth having,
and it is `55b`'s sign observation stated as a condition on the format rather than on the operands: a
reachable ceiling is absorbing exactly when every available translation is non-negative, a reachable
floor exactly when every translation is non-positive.

Restricted to a **genuine interval numeral system**, meaning `Q = [lo, hi]` with `0 ∈ Q` and operands
drawn from `Q`, that closed form has a clean consequence, and `57_probes/p1` section 3 sweeps every
such interval with `lo ∈ [-9, 0]` and `hi ∈ [0, 9]`:

> 100 intervals swept, 19 associative, **0 mismatches against the prediction "associative iff
> `lo == 0` or `hi == 0`"**.

Nineteen is exactly the count of intervals with `lo == 0` or `hi == 0`. **Saturating addition on an
interval numeral containing zero is associative if and only if the interval is sign-confined.** There
is nothing in between: unsigned associates, signed does not, and no bound placement rescues a
two-sided format.

That also dissolves why `42`'s condition looked right. On genuine interval numerals containing zero,
"exactly one bound reachable" and "sign-confined" are **coextensive**, so `42`'s sentence gives the
right answer on every case a real numeral system presents, and gives the wrong answer on `42`'s own
row one and on every individual fold. It is right for a reason that is not the reason.

The cheap form is not the criterion, and `p1` contains the case that shows it. The opposite-bound
mutant on `Q = [0, 15]` is sign-confined and has a reachable ceiling, so the closed form predicts
associativity; it measures **2240 divergences**. Absorption predicts the failure correctly. So the
closed form is a corollary for monotone median clamps and absorption is the criterion, and a canon
that stated only the sign form would be stating a theorem about a class it had not delimited.

### 2.7 `56` had the criterion and misfiled it

`56_probes/q1`'s C-law is `rho(a op b) == rho(rho(a) op rho(b))` over the window. With `b` drawn from
`Q` so that `rho(b) = b`, that is `rho(rho(a) op b) == rho(a op b)`, which is absorption.

`56:214-216` then says:

> `42`'s per-trajectory form stays the finer statement; the coherence law is the per-window form that
> also covers wrap.

**That ordering is backwards, and `57_probes/p2` measures the reversal.** Over the same 4248
configurations, coherence-as-absorption mispredicts nothing and `42`'s form mispredicts 1749
configurations or 306,024 triples depending on the reading. Coherence is not a coarsening of a finer
true statement. It is the statement, and the thing it was deferring to is false.

This also answers the item `56` closed its coverage with, that it could not construct the case
separating window coherence from `42`'s per-trajectory condition. **The case is `42` row one, and `56`
names it**, at `56:208-211`:

> my unsigned clamp is coherent over the nonnegative window (42's rows two and three) and incoherent
> over the signed window (42's row one)

So `56`'s C-law classifies row one as **incoherent**, `42`'s condition classifies row one as
**associative** (one clamp, so at most one triggerable), and row one measures 904 failures. The two
classifications disagree on that row, one of them matches the measurement, and `56` wrote both into
the same paragraph, five lines apart, and concluded that the one which gets it wrong is the finer
statement. The separating case was never missing. It was quoted.

### 2.8 What I would keep of `55b`'s mechanism, and what I would change

`55b` names the mechanism as **pullback**: divergence needs a clamp event followed by movement toward
the interior. That is right, and it is the dynamic description of the same fact absorption states
algebraically: a translation that moves a clamped value off its face is exactly a translation that
breaks `rho(rho(x) + y) == rho(x + y)`.

I would keep pullback as the explanation and use absorption as the condition, for three reasons that
are measurements rather than preferences. Absorption is checkable as a predicate over a format and its
operand set, and pullback as stated is a property of a trajectory. Absorption is already `56`'s
coherence law, so the panel gets one concept rather than two. And absorption extends to the mutant,
where `55b`'s companion observation about operand-sign mixture does not: the mutant on `Q = [0, 15]`
has zero mixed-sign operands available and diverges 2240 times.

`55b:178` says "operand-sign mixture is a proxy for pullback, not for bound count". Correct on both
halves, and the mutant row is where the proxy stops being one.

## 3. Job two: the grading, second-read

### 3.1 The semiring holds, at nine widths, and here is why it must

`55_probes/p4` reports the unsigned-saturation semiring at 4 bits with distributivity at zero
failures, and `55b:109-110` states the width transfer as "argued, unprobed". `57_probes/p3` probes it.

Every commutative-semiring axiom, exhaustively, at M = 1, 2, 3, 7, 15, 31, 63, 127, 255: additive
associativity, additive commutativity, additive identity, multiplicative associativity, multiplicative
commutativity, multiplicative identity, distributivity, and zero annihilation, all at **zero failures
in all nine rows**, with the count of elements lacking an additive inverse coming out at exactly M
each time, which is what makes it a semiring and not a ring.

The measurement is nine rows. The reason is all widths at once, and it is the part worth carrying:

> `x ~ y` iff `x == y` or both `>= M` is a **congruence** on `(N, +, ·)`. Measured at M = 1, 3, 7, 15,
> 31 over ambient windows up to `4M+3`: zero violations for either operation.

If the relation is a congruence then unsigned saturation is not merely *like* a semiring, it **is** the
quotient semiring `N/~`, and inherits every axiom from the naturals structurally. That upgrades
`55b`'s result from a measurement to a theorem shape, which is the difference between a law layer that
can state it and one that can only report it. `55b` was right to want this second-read and right about
the answer.

M = 1 deserves its own line because it is the instance that is not another size of the same thing:
two elements, min-clamped addition is disjunction, min-clamped multiplication is conjunction. The
**Boolean semiring**. A width sweep that only doubled would not have told anyone that.

### 3.2 The bound `55b` does not state, and the canon would have to

arvo's format concept is not the integer grid. `57_probes/p3` section 3 runs the same axioms with F
fractional bits, where the saturating multiply must rescale, `r = sat((ra · rb) >> F)`:

| M | F | +assoc | *assoc | distrib | verdict |
|---|---|---|---|---|---|
| 15 | 0 | 0 | 0 | 0 | semiring |
| 15 | 1 | 0 | 398 | 168 | not |
| 15 | 2 | 0 | 878 | 518 | not |
| 15 | 3 | 0 | 884 | 1158 | not |
| 31 | 1 | 0 | 1892 | 709 | not |
| 63 | 3 | 0 | 53032 | 23706 | not |

Nine of nine fractional configurations fail. **The additive commutative monoid survives every scale**,
which is what the absorption criterion predicts, since addition on a sign-confined interval is
absorbing whatever the grid. The multiplicative half does not survive any of them.

So the honest statement is: **unsigned saturating arithmetic is a commutative semiring on an
integer-grid format, at every width, by a congruence argument. On a format with fractional bits, the
additive monoid survives and the semiring does not.** A law layer that licensed product reassociation
or distribution from "unsigned saturation is a semiring" would be wrong on every `UFixed<I, F>` with
`F > 0`, which is most of them.

This is also an independent arrival at `42`'s section 3.3, which I opened: `42:189-190` reports both
laws holding "almost exclusively at `F == 0`". Two instruments, two authors, same boundary.

### 3.3 Which factor breaks it, measured rather than assumed

`42:191` attributes the fractional collapse to "the same destructive step, the rescaling right-shift
by `F`". That is an attribution, and `57_probes/p4` section 1 measures it by running the factors
apart. The reduction at F > 0 is a composite of a **grid coarsening** `x -> x >> F`, which changes the
ulp and leaves the range, and a **range clamp** `x -> sat(x)`, which changes the bounds and leaves the
ulp:

| factor | M | F | +assoc | *assoc | distrib |
|---|---|---|---|---|---|
| clamp only | 15 | 1 | 0 | 0 | 0 |
| coarsen only | 15 | 1 | 0 | 1160 | 512 |
| composite | 15 | 1 | 0 | 398 | 168 |
| clamp only | 31 | 2 | 0 | 0 | 0 |
| coarsen only | 31 | 2 | 0 | 16128 | 8192 |
| composite | 31 | 2 | 0 | 5352 | 2309 |

The clamp factor is clean at every row measured. The coarsening factor is not. **`42:191` was right,
and this is the isolated measurement of it.** Saying "saturation breaks distributivity" would be false
in a way that matters, because it would send a law layer to condition on the overflow axis when the
condition belongs to the scale axis. Under Q5's two-axis framing, this is a case where the two axes
demonstrably carry different law consequences, which is evidence for the split rather than for either
answer to Q6.

One detail worth keeping because it cuts the wrong way from how it reads: **the composite fails less
often than the coarsening alone** (398 against 1160). The clamp is masking rounding divergences by
collapsing distinct wrong answers onto the same bound. So a law-failure count measured on the composite
**understates** the rounding damage, and any table of failure percentages built from composite
measurements is a lower bound on the underlying error rather than a measure of it.

### 3.4 "Grade" is doing two jobs, and only one of them is a grading

`55b:110-113` writes:

> the design's law layer can state per policy an induced structure, and the licensed rewrites follow
> from the grade: **a group licenses reassociation and cancellation; a monoid or semiring licenses
> reassociation without cancellation; a magma licenses nothing.**

The classification is right and it is useful and I would keep it. The word is carrying something it
does not support. Ring over semiring over monoid over magma is a **ladder**: a partial order on
theories, ordered by which axioms hold. A **grading** is an index set with a composition law, so that
the index of a composite is computed from its parts' indices. These are different structures, and a
canon that adopted the second word for the first object would inherit a claim nobody has established.

I dispatched myself to make that bite by finding a composite whose law set is worse than its factors'
meet. **I did not find one.** `57_probes/p5` section 1, over twelve configurations spanning M = 7, 15,
31 and F = 0 to 3: the meet is respected in all twelve, and in every row the composite's law set
**equalled** the meet exactly rather than merely containing it.

So the honest report is that my hypothesis was refuted and `55b`'s position came out stronger than it
claimed. That is one instrument over twelve configurations of one factorisation and it is first-read.
And it establishes less than it looks like: a lattice meet is defined on law sets whatever the
operations are, so agreeing with it here is consistent with there being no relation at all in a case
nobody measured. The probe's original hypothesis is left in its header with the refutation attached,
and its first printed reading, which overstated in the other direction, is kept as
`p5_output.v1_overstated_reading.txt`.

**What I would put to `55b`:** the ladder is a real classification, it survived an attempt to break its
compositionality, and it is still not what "grade" means. The precise thing it is: a point in the
lattice of law sets. That is worth saying because the licensed-rewrites consequence follows from it
directly, without needing the word.

### 3.5 The grading that is real, and it is the precision axis

`57_probes/p5` section 2. Three findings, in the order they constrain each other.

**Widths are not a grade monoid for addition.** The rule `g(W, V) = max(W, V) + 1` is not associative:
`g(g(5,0),0) = 7` and `g(5,g(0,0)) = 6`. So sums do not grade over widths. The multiplicative rule
`g(W, V) = W + V` is associative, so products do.

**What composes exactly is the reachable interval.** For n operands from `Q = [-8, 7]`, the measured
reachable set of exact sums equals the Minkowski sum `[n·lo, n·hi]` at every n from 1 to 6, exactly.
The width is a sound image of it that loosens as n grows:

| n | measured range | exact bits | repeated max+1 | slack |
|---|---|---|---|---|
| 2 | [-16, 14] | 5 | 5 | 0 |
| 3 | [-24, 21] | 6 | 6 | 0 |
| 4 | [-32, 28] | 6 | 7 | 1 |
| 5 | [-40, 35] | 7 | 8 | 1 |
| 6 | [-48, 42] | 7 | 9 | 2 |

**So the quantity that grades is the value set, and the width is a lax abstraction of it.** That is the
same relationship interval arithmetic has to width analysis generally, and it says which of the two a
canon should make primary: state the intent in terms of the reachable set, and let width be the derived,
conservative thing an implementation computes.

**The operational grading, which is the one with teeth.** Accumulate n operands of a W-bit signed
saturating format into a width-w intermediate, saturating at each step, then adapt once. Divergence
from the exact-then-adapt answer as a function of w:

| n | tuples | divergences at w = 4, 5, 6, 7, 8, 9 |
|---|---|---|
| 2 | 256 | 0, 0, 0, 0, 0, 0 |
| 3 | 4096 | 476, 0, 0, 0, 0, 0 |
| 4 | 65536 | 14721, 0, 0, 0, 0, 0 |
| 5 | 1048576 | 333900, 4536, 0, 0, 0, 0 |

That is a grading in the sense the word carries: a family indexed by w, an index computed from the
operation and the operand count, and an exact statement of what each index buys. The adaptation is the
map that spends the grade back down to the format's width, and it is exactly where information is lost.
The 476 at n = 3 reproduces `56_probes/q1`'s chain-divergent count of 476 through a completely different
code path, which is a cross-check I did not plan and will take.

### 3.6 The one-bit finding

`p5`'s predicted grade `W + ceil(log2 n)` was **sound at every n and loose by exactly one bit at every
n**: measured sufficient widths 4, 5, 5, 6 against predicted 5, 6, 6, 7. A sound-but-loose bound is a
finding to attack, so `57_probes/p6` sweeps it: signed formats W = 3 through 6 and unsigned W = 3
through 5, fold lengths 2 through 8, every row exhaustive, capped at about 3.4e7 tuples.

**The gap is exactly one bit in all fifteen rows where the question arises.** Zero anomalies.

The mechanism: the naive bound asks the accumulator to hold the exact sum, and it does not have to. It
has to agree with the exact sum **after the final adaptation**, and the adaptation clamps into `Q`
regardless. An accumulator that saturates is harmless when the exact value was already outside `Q` on
the same side. **The accumulator has to decide which side of the format the result fell on, not
represent how far outside it fell**, and that is worth one bit on every fold. This is absorption again,
appearing as a width saving rather than as a law.

**What it bears on, and where I will not overreach.** `OPTIONS.md`'s Q6 cites `20`'s interior-safety
predicate `W + ceil(log2 n) <= width(accumulator)` and reports the clamp family's arity crossover
landing "exactly where its own interior-safety predicate says it should". I have not opened `20` and I
am **not** claiming that predicate is wrong. It is the right predicate for its own question, which is
whether a clamp fires **at all**. Mine is a different and weaker question: whether the answer equals
exact-then-adapt. The two differ by exactly the one bit measured here, and the useful contribution is
that they are different predicates and the panel has been using one phrase for both.

Whether that bit is worth taking is a bench question and I have run no bench, so it is unpriced.

### 3.7 The theorem the two jobs share

`57_probes/p6`'s unsigned rows all read "format width already suffices": across sixteen rows, W = 3 to
5, n = 2 to 8, eager saturation in the format itself agrees with exact-then-adapt at **every**
accumulator width, including the narrowest. And `57_probes/p4` section 2 measures the same split at
fold lengths 2 through 6:

| n | wrap | unsigned saturation | signed saturation |
|---|---|---|---|
| 2 | 0 | 0 | 0 |
| 3 | 0 | 0 | 476 (11.62%) |
| 4 | 0 | 0 | 14721 (22.46%) |
| 5 | 0 | 0 | 333900 (31.84%) |
| 6 | 0 | 0 | 6634292 (39.54%) |

So, stated as one fact rather than two law bundles:

> **Coherence is the statement that the grading collapses.** A coherent reduction needs no grade: its
> accumulator is the format itself, at any fold length. An incoherent one has a real grade, and the
> grade is what an implementation pays in accumulator width to get the exact-then-adapt answer.

That gives `56`'s two families their asymmetry honestly. The **adaptation laws face the source** and
are the properties of the map that spends the grade: retraction, monotone, nearest. The **coherence law
faces the target** and asks whether the grade exists at all. They are not two arbitrary bundles and
they are not competing, which is `55b:82-84`'s point arrived at from the other end.

It also closes `55b`'s stated open edge. `55b:225-226` lists "whether the pullback mechanism survives
fold lengths past three" as not established by anyone. For the coherence half it does: the split is not
a length-three artifact, the coherent policies hold at exactly zero through n = 6, and the incoherent
one degrades monotonically. The n = 2 zero is informative rather than trivial and it cost me a probe
failure to notice: at two operands there is only one association order, so **nothing** can diverge for
any policy, which confirms the divergence is a fact about association rather than about the reduction.

## 4. What I put to `55` and `56`, for the resumption

**To `42`'s author, if resumed.** Row one of your own table refutes your section 5.2 summary sentence,
and the H2 in your probe's comments at lines 25 to 28 is the thing that survives. Would you accept
absorption as the statement of H2 that generalises, given `p2`'s 4248-configuration biconditional?
And is the third column of `42:319-324`, "is the floor structurally reachable", counting an absent
clamp in an unbounded direction as a boundary, which is what let the sentence past its own data?

**To `55b`.** Three things.

Your `p5` is right and its counts reproduce. What it refutes was already refuted, in `42_probes/p3.out`
which was on disk when you wrote, and you could not have known since you say you did not open `42`.
Does the refutation read differently to you knowing the counterexample is `42`'s own row one rather
than your signed box?

Your semiring transfers, at nine widths, and the congruence argument makes it structural. Does the
`F == 0` bound change what you would want Q11 to be able to say? My reading is that it does: "state the
induced structure per policy" is not enough, because the induced structure is a function of the policy
**and** the scale, and the scale is the axis the panel has been treating as inert.

On "the algebras grade": I went to break the compositionality and failed, twelve rows, meet respected
and exact. So I am seconding your result and objecting only to the word. Would you accept "the induced
algebras form a ladder in the lattice of law sets", with the licensed-rewrites consequence unchanged?

**To `56`.** Your C-law is the criterion. `56:214-216` files it as the coarser form deferring to `42`'s
finer one, and `p2` measures the ordering the other way: coherence mispredicts nothing over 4248
configurations, `42`'s form mispredicts 1749 of them. And the separating case you closed your coverage
unable to construct is `42` row one, which you name at `56:208-211` as the window your own clamp is
incoherent over, five lines before deferring to a condition that calls that same row associative.
Would you restate the relation with coherence as primary?

## 5. What the register should gain

I have not edited `OPTIONS.md` or `INTENTS.md`, per the dispatch. What they should gain, for whoever
holds the repair:

**Q12's mechanism paragraph and its bracketed caution both want replacing, not amending.** The caution
currently marks `55b`'s refutation as ONE EXPERT with a second read of `42` owed. **This file is that
second read**, so the caution can be discharged, and it should be discharged in a direction neither the
paragraph nor the caution anticipates: the refutation stands and its source is `42_probes/p3.out` row
one rather than `55_probes/p5`. The register should carry the criterion as absorption, with `p2`'s
biconditional counts, and should record that `42`'s form fails under both quantifier readings with the
mismatch counts attached. The "reachability" vocabulary should go, since it is the word that let an
absent clamp count as a boundary.

**Q12 should gain the sign-confinement corollary as a separate line**, because it is the form that
answers the question Q12 actually asks: on an interval numeral containing zero, saturating addition is
associative iff `lo == 0` or `hi == 0`, 100 intervals swept, zero mismatches. That is a statement about
formats rather than about folds, and it is the one a consumer can check by looking at a type.

**Q11 should gain the induced structures with their scale condition.** `55b` deliberately did not
promote them and was right not to on one probe at one width. They now have nine widths and a congruence
argument, and they also have a bound: the semiring is `F == 0`, the additive monoid is every F, and the
break is attributable to the grid coarsening rather than to the overflow policy. Q11's second option,
"the numeral names its algebraic structure", should say that the structure is a function of the policy
**and** the scale, or it will licence the wrong rewrites on exactly the formats arvo exists for.

**Q11's third option should gain the one-bit measurement.** It currently says the accumulator is
derivable as "the width plus the log of the capacity". For the exact-then-adapt criterion that is one
bit loose, uniformly, in all fifteen rows where the question arises, and for a coherent policy the
whole term is zero and no accumulator is needed at all. Both are content that option does not have.

**Q6 should gain a line distinguishing two predicates it currently conflates**, interior safety
(no clamp fires) from adaptation agreement (the answer equals exact-then-adapt), which differ by one
bit. I have not opened `20` and this is not a claim against its numbers.

**A droplist entry.** "Associativity of a clamped operation is decided by how many of its bounds are
reachable", in either quantifier reading, is closed. Diagnostic: `42_probes/p3.out` row one, 904
ceiling-only divergences with no floor in the code, plus `57_probes/p2_output.txt`'s 306,024 and 1749
mismatch counts. What would reopen it: a demonstration that absorption and bound-reachability coincide
on some class where absorption is not simply the definition, which the interval sweep suggests is only
the genuine interval numerals, where they do coincide and where the bound-count form is right for the
wrong reason.

## 6. Bearing on the live options

**Q5 (one axis or two).** Evidence for two, from a direction the entry does not have. `p4` section 1
shows the overflow axis and the scale axis carry **different law consequences**: the range clamp is a
semiring at every scale, the grid coarsening is not at any, and the composite inherits from the
coarsening. A single axis cannot express "this format's addition is a monoid and its multiplication is
not", which is what every `F > 0` unsigned saturating format actually is.

**Q6 (Warm wraps or clamps).** Does not decide it, and sharpens the cost of each. Wrapping induces a
ring at every width with no accumulator ever needed. Unsigned clamping induces a semiring on the
integer grid, also with no accumulator ever needed, and an additive monoid at every scale. Signed
clamping induces neither: not a semigroup, and it carries a real accumulator grade of
`W + ceil(log2 n) - 1` bits. **The signed case is where the algebra is worst and it is the case Warm
would actually be**, since a general-purpose numeral is signed. That is a cost the entry does not
currently carry, and it cuts against clamping on grounds independent of the bench families it is
currently argued from.

**Q8 (one numeral family or several).** Weak evidence toward the cost of one family being real: the
induced algebra is a function of the sign domain, the overflow policy and the scale jointly, so a
single family has to carry all three in whatever it tells a fold. It does not decide the question and I
would not carry it further than that.

**Q11 and Q12.** Section 5.

**Kills nothing.** No option in the register is closed by anything here. One non-option claim inside
Q12 is closed, which is the mechanism paragraph.

## 7. What I could not determine

**Whether absorption is the criterion beyond addition and multiplication.** Swept for those two. A
subtraction, a shift, a division, or a mixed-operation chain could behave differently and none is
measured.

**Whether the one-bit accumulator saving generalises.** Fifteen rows, all signed saturation, W = 3
through 6, n = 2 through 8, exhaustive within each row. It is one policy, and the unsigned rows do not
test it because they are trivially zero. It is not a proof and I do not have the argument that would
make it one. My suspicion is that it is exact and follows from the final adaptation absorbing the
outermost bit, but a suspicion with fifteen supporting rows and no proof is what it is.

**Whether the ladder is genuinely compositional.** I failed to refute it over twelve configurations of
one factorisation. That is much weaker than establishing it, and I say so rather than banking it.

**Whether the semiring survives rounding modes other than truncation.** `p3` and `p4` truncate toward
zero, which is what `(a * b) / scale` does on positive operands. Round-to-nearest might restore
distributivity or might not, and it is a cheap probe somebody should run before the law layer states
anything about `F > 0` at all.

**Whether the accumulator grade is what any implementation should actually do.** That is a bench
question about container selection and I ran no bench. Every magnitude in this file is a count of
counterexamples, never a timing, and nothing here prices anything.

**Anything about `35`, `18`, `20`, `25`, `40` or `43`.** Not opened. Where this file's results touch
theirs, through the register or through `42`'s citations of them, I have said so and have not
reasoned about their contents.

## 8. Coverage, bounded honestly

**Read in full:** `00_brief.md`, `INTENTS.md`, `42` (all of it, with `42_probes/p3` source and output),
`55b`, `55_probes/p4` and `p5` sources and outputs, `56_probes/q1` source and output, `OPTIONS.md`
sections Q6, Q11, Q12. **Read in part:** `56` section 3.3 and its surroundings. **Not opened:** `55`
itself beyond what `55b` and `56` quote, `35`, `18`, `20`, `25`, `40`, `43`, `DROPLIST.md`, `seed/`,
`archive/`.

**Re-run before relied on:** `42_probes/p3`, `55_probes/p4`, `55_probes/p5`, `56_probes/q1`, all
byte-identical. **Not re-run:** `56_probes/q2` and `q3`, whose outputs I read but whose conclusions I
do not depend on.

**Built:** six probes, all exhaustive within their stated domains, all with instrument validation that
fires, all committed with sources and outputs. One of them (`p4`) failed its first run on an assertion
of mine that was wrong about the data, and both runs are on disk. One of them (`p5`) had its hypothesis
refuted by its own section one, and the hypothesis is still in its header.

**Everything measured here is plain integer arithmetic.** No fixed-point machinery, no type-level
construction, no arvo types, deliberately, so that nothing is an artifact of a representation. That is
also the coverage limit: these are results about clamped and wrapped integer algebras, and their
transfer to whatever arvo's format concept turns out to be is an argument I have not made.

**First-read, owed seconds:** the absorption biconditional; the congruence argument for the semiring;
the `F == 0` bound and its attribution to the coarsening factor; the one-bit accumulator gap; the
coherence-collapses-the-grading statement. **Seconded by me, from a different instrument:** `55b`'s
semiring (nine widths against its one), `55b`'s pullback mechanism (as absorption), `42:191`'s
attribution of the fractional collapse to the rescaling shift, `56`'s C-law as the correct criterion.

**Nothing here settles anything.** The mode is explore, there is no canon, and the two experts whose
work this file adjudicates should be resumed to answer section 4 before any of it is carried.
