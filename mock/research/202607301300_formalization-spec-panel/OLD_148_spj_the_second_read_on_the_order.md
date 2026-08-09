# The second read on the order, and the `From` spelling

**Date:** 2026-08-07
**Position:** after `146_chlipala_the_order_and_the_cast.md`, which overturned three findings of `145` on a
single reading and said its disputes owe a second read. This is that second read, plus the soundness
question on the `From` spelling.

**Method, stated first because it is the whole point of the dispatch.** Every answer in sections 1 to 4 was
derived and compiled before either `145` or `146` was opened. The derivations are in `148_probes/`, and
each section says where it lands against each file after the fact. Sections 5 and 6 were written after
reading both, because the questions they answer are about those files' own arguments.

**Canon gate.** There is no ratified canon for arvo; this panel is writing the first one, and `145c` is the
standing correction on how the shipped source may be read. Nothing here cites `mock/crates` in support of a
claim about the design, and nothing was written to it.

## Verdict, in one place

On question one, four conditions, and I reach the same count as `146` by a different route, with a
different and I think better statement of them: two coordinates rather than four conditions, the affine
lattice and the range interval, each ordered by inclusion. Exact over 331,776 ordered pairs.

On question two, the order is componentwise, but not in any coordinate the numeral declares. `146` is right
that the sign domain is not a partition, and right that the declared coordinates do not decompose it. It is
one input to the range coordinate, along with precision, and the two are coupled.

On question three, both files are wrong, in opposite directions and for the same underlying reason. The
family is a **meet-semilattice and not a join-semilattice**. The meet is the half that survives, not the fragile
one. The join fails in 81 of 351 unbiased pairs, decisively, before bias enters at all.

On question four, no adjudicating column, and `145` is right about that, but `146` is right that the schema
as it stands does not say. The resolution is a sentence rather than a column, and the sentence is not the
one `146` derives: the question decomposes into two questions with two different answers, both already
forced by the layer-keying rule.

On the `From` spelling, the coherence argument is sound and is stronger than the reason given for it. But
**the route as `146` spells it does not compile in this arrangement**: `146_probes/f03` carries
`#![feature(generic_const_args)]` and its runner passes `-Znext-solver=globally`, and it fails without the
flag. Both are outside what the design permits. Carrying the same condition as a trait bound rather than as a
const projection removes the dependency entirely: exit 0 on the default solver with no feature gates, over
the design's own binary width encoding, at widths to 128. The by-reference surface is the right call-site
spelling and the wrong bound surface, and the two can be separated for nothing.

And one thing the dispatch did not ask about, found while checking the `Ranged` conditions: **`146`'s third
instance of the coordinate-coupling pattern is an artefact of a `Ranged` value-set model that the design's own
ratified sentence contradicts.** Its named counterexample is a clean inclusion under the design's model, with
zero missing values, and the replacement condition it offers would make the design refuse valid conversions.
Section 2.3b.

## What each file should keep, so this is not read as a demolition

`145` keeps: the meet being exact, which is the durable half of its lattice claim and which I confirm at 0
failures over 351 unbiased pairs; narrowing as the quantiser with the operation set to the identity; the
target strategy's row as the resolver; no new key column; `Hot`'s narrowing not being monotone.

`146` keeps: the condition count and the phase condition, which are right and which `145` gets wrong in a way
that is unsound rather than merely narrow; the sign domain as a coordinate rather than a partition; the
cardinality restatement of the antichain; the observation that C1, C2 and C3 live exactly where the
adjudication question is invisible, which is the sharpest thing in either file; the `&` as the structural
coherence evasion; the ten dead routes; and the `TryFrom` result, which I re-checked in my own spelling.

## 1. The value set, written down

Everything below rests on one sentence of the design and one of its consequences.

**The affine value map** (`124:1.1`): the value of a stored integer `k` under a numeral is
`Adjustment * radix^exponent * k + Bias`. Write `q(N) = A · r^E` for the **quantum** and `b(N) = B` for the
**offset**.

**The index interval** is fixed by `Precision` and `Domain`. The design does not state the three intervals
in one place, but `124:1.2`'s `SC_SAT_SYM` cell does state the discriminating fact, that the identical
`TowardNegative` clamp delivers `-8` under `AsymmetricLow` and `-7` under `Symmetric`. That pins them:

$$
K(N) \;=\;
\begin{cases}
[\,0,\; r^p - 1\,] & \text{Domain} = \mathsf{NonNegative}\\
[\,-(r^p - 1),\; r^p - 1\,] & \text{Domain} = \mathsf{Symmetric}\\
[\,-r^p,\; r^p - 1\,] & \text{Domain} = \mathsf{AsymmetricLow}
\end{cases}
$$

So for an `Implicit` numeral,

$$V(N) \;=\; \{\, q(N)\cdot k + b(N) \;:\; k \in K(N) \,\}.$$

**The consequence that does the work.** That set is the intersection of two things which are individually
much simpler than it:

$$V(N) \;=\; \mathcal{L}(N) \,\cap\, I(N), \qquad
\mathcal{L}(N) = b(N) + q(N)\mathbb{Z}, \qquad
I(N) = [\,v^-(N),\, v^+(N)\,]$$

where $\mathcal{L}$ is an **affine lattice** in $\mathbb{Q}$ and $I$ is the closed interval spanned by the
numeral's own extreme values. Every claim in sections 2 and 3 is a claim about those two objects.

**`Ranged` is not of this shape and that is the family's real dividing line.** The design's own statement
(`124:1.5`, quoting `58:220-224`) is that a `Ranged` numeral denotes the union over `e` in `[EMIN, EMAX]` of
the grids with quantum `radix^(e - p + 1)` restricted to `[radix^e, radix^(e+1))`, plus the bottom grid
extended to zero under `Gradual`. A union of grids with different quanta is not a single lattice, so
$V = \mathcal{L} \cap I$ has no `Ranged` instance and the two families need separate statements. Section 2
gives the `Ranged` one.

**And note what `Ranged<EMIN, EMAX, U, S>` does not carry.** It has no `Adjustment` and no `Bias`; those are
members of `Implicit<E, A, B>`. So the phase condition below is structurally an `Implicit`-only condition,
which is worth saying plainly because it means **the condition set is keyed on the exponent form**, the same
key `124:1.21` derives for both strategy contracts. That is one key, doing three jobs, rather than three
facts that happen to agree.

## 2. Question one: the conditions for inclusion

### 2.1 What I derived

For `Implicit` numerals with $|V(N_1)| \ge 2$:

$$V(N_1) \subseteq V(N_2)
\iff
\underbrace{\mathcal{L}(N_1) \subseteq \mathcal{L}(N_2)}_{\text{lattice}}
\;\wedge\;
\underbrace{I(N_1) \subseteq I(N_2)}_{\text{range}}$$

Necessity of the range half is immediate, since $v^-(N_1)$ and $v^+(N_1)$ are themselves members of
$V(N_1)$. Necessity of the lattice half is the only step that needs $|V(N_1)| \ge 2$: the differences of
$V(N_1)$ generate $q(N_1)\mathbb{Z}$, so the affine lattice generated by $V(N_1)$ is exactly
$\mathcal{L}(N_1)$, and a set contained in an affine lattice generates an affine lattice inside it.

Unfolded into conditions on the declared members, the lattice half is two conditions and the range half is
two more, which is where the count of four comes from:

| | condition | vacuous when |
|---|---|---|
| G, grid | $q(N_1) / q(N_2) \in \mathbb{Z}$ | never |
| P, phase | $\bigl(b(N_1) - b(N_2)\bigr) / q(N_2) \in \mathbb{Z}$ | both biases are zero |
| R-, floor | $v^-(N_1) \ge v^-(N_2)$ | both domains are `NonNegative` and both biases zero |
| R+, ceiling | $v^+(N_1) \le v^+(N_2)$ | never |

The degenerate case is worth naming rather than leaving to a reader: when $|V(N_1)| = 1$, which happens at
`Precision = 0` under `NonNegative`, G is vacuous and the whole condition is membership of the single point.

### 2.2 What the probe says

`148_probes/p1_inclusion_conditions.rs`, exhaustive over 576 numerals and 331,776 ordered pairs, spanning
radices two and three, precisions one to three, exponents minus two to one, both `Adjustment` constructors
(`Unit` and `FullRange`), four biases including two that are off-phase from each other, and all three sign
domains. The oracle is elementwise set inclusion over exact rationals, not a formula. 8,109 of the pairs
include.

| candidate | false positives | false negatives |
|---|---|---|
| grid and ceiling only | 17,037 | 0 |
| G, P, R-, R+ | 0 | 0 |
| lattice inclusion and interval inclusion | 0 | 0 |
| product order on (top magnitude, grid, domain) | 0 | 5,853 |
| product order on (precision, exponent, domain) | 2,064 | 5,853 |

The two-condition reading is not merely incomplete. It is **unsound**: it claims 17,037 inclusions that do
not hold. Of those, 6,549 are sole phase failures and 5,229 are sole floor failures.

The phase witness the probe emits is the same shape `146` reports and I confirm it independently:

```
source: r=2 p=1 e=-2 A=1/1 B=0/1   dom=NonNegative     values [0 .. 1/4] step 1/4
target: r=2 p=1 e=-2 A=1/1 B=1/3   dom=AsymmetricLow   values [-1/6 .. 7/12] step 1/4
  quantum ratio integral: true
  target range strictly contains source range
  source has 2 values, 0 of them are in the target
```

Equal quanta, containing range, and not one shared value. Two numerals can be arbitrarily close in every
declared coordinate and share nothing.

### 2.3 The `Ranged` conditions

`148_probes/p2_ranged_inclusion.rs`, 384 numerals and 147,456 ordered pairs.

The right coordinate for `Ranged` is the **exponent function** $\varphi_N$, which returns the exponent of the
local quantum at a given magnitude binade:

$$\varphi_N(e) \;=\; \max\bigl(e - p + 1,\; \mathrm{EMIN} - p + 1\bigr) \ \text{under Gradual}, \qquad
\varphi_N(e) \;=\; e - p + 1 \ \text{under Abrupt}.$$

Then inclusion is $\varphi_{N_2} \le \varphi_{N_1}$ pointwise over the binades the source inhabits, plus the
two range endpoints, plus domain containment, plus (unmeasured here, and additive) $\mathrm{Specials}_1
\subseteq \mathrm{Specials}_2$. Measured: **0 false positives and 0 false negatives over all 73,728
same-radix pairs.**

| candidate | false positives | false negatives |
|---|---|---|
| product order on (p, EMIN, EMAX, U, domain) | 0 | 4,572 |
| product order after replacing EMIN by EMIN - p + 1 | 882 | 2,808 |
| pointwise exponent function plus range | 0 | 1,296, all cross-radix |

Three things fall out that I have not seen stated.

**The change of basis that looks obvious is unsound.** Replacing `EMIN` by the finest-quantum exponent, which
is the natural move and the one that makes "a significand digit buys an exponent" true, produces 882 false
positives. The reason is `Underflow = Abrupt`: an abrupt format has a hole between zero and `radix^EMIN`
that the finest-quantum coordinate cannot see. Witness, from the probe:

```
source: r=2 p=1 EMIN=-3 EMAX=-3 Abrupt  values {0, 1/8}
target: r=2 p=2 EMIN=-2 EMAX=-2 Abrupt  values {0, 1/4, 3/8}
  both have finest quantum 2^-3; the source's 1/8 is in the hole
```

**"Adding a significand digit costs an exponent" is real, and it is exactly the incompleteness of the naive
order.** 1,764 of the 4,572 pairs the naive product order misses have the target's precision strictly larger
**and** its `EMIN` strictly larger, so the target is above the source while being worse on one declared
coordinate. The digit paid for the exponent. This is the third instance of the coupling `146` names, and I
confirm it.

**Cross-radix inclusion is real and is not only degenerate.** The oracle finds 1,296 cross-radix inclusions,
552 of which have a source with at least four values. A radix-two numeral holding $\{-2,-1,0,1,2\}$ sits
inside a radix-three numeral, because integers are integers. Any inclusion witness the design computes must
either handle this or say in words that it is a same-radix relation and that cross-radix embeddings are real
and simply not offered. Silence here would be a false statement of the order rather than an omission.

### 2.3b `146`'s third instance is an artefact of a substituted model

This is the sharpest disagreement in the file and I want it stated with its evidence rather than as a
verdict.

`146` reports a third instance of the coordinate-coupling pattern, in the `Ranged` family, and grounds it on
this sentence: "adding a significand digit while holding the exponent window loses values off the bottom. A
value $3 \cdot 2^{-3}$ at precision two has to be written $6 \cdot 2^{-4}$ at precision three, and $-4$ is
outside the window." Its counterexample is `p2 [-3,0]` into `p3 [-3,0]`, componentwise true and inclusion
false, at 201 of 2,304 pairs.

That reading takes `EMIN` and `EMAX` to bound the **quantum** exponent, so the significand is an integer in
$[r^{p-1}, r^p-1]$ multiplied by $r^e$ directly. The design's own sentence takes them to bound the **binade**:

> A `Ranged` numeral denotes the union, over `e` in `[EMIN, EMAX]`, of the grids with quantum
> `radix^(e - p + 1)` restricted to `[radix^e, radix^(e+1))` (`124:1.5`, quoting `58:220-224`)

Under the design's sentence the quantum in a binade is $r^{e-p+1}$, so raising the precision makes every
binade's grid **finer** and the window stays where it is. The exact pair `146` names, evaluated against that
sentence, is a clean inclusion:

```
146's third-instance pair, Underflow=Gradual: source has 10 values, target has 20,
  source values NOT in target: 0 []
146's third-instance pair, Underflow=Abrupt:  source has 9 values, target has 17,
  source values NOT in target: 0 []
  at e=-3 the quantum is r^-4 for the source and r^-5 for the target: the target grid is finer everywhere
```

And the naive product order on the declared members has **zero false positives** across all 73,728 same-radix
pairs, so "more precision is at least as good" is true under the design's model rather than false.

Two consequences, and the second is the one that costs something.

**The third instance is not a third instance.** The pattern `146` names is real and its first two instances
are real, and I confirm both. The `Ranged` one is a model substitution.

**And the replacement condition `146` offers would make the design refuse valid conversions.** It proposes
$e^{\min}_B \le e^{\min}_A - d$ with $d = p_B - p_A$. Under the design's own model that is strictly
stronger than necessary: at $d = 1$ and equal windows it refuses an embedding the value sets say holds. A
condition that is too strong is not a safe default here; it removes conversions the design intends to offer,
and it removes them silently, because a refused `From` looks exactly like a pair that was never meant to
convert.

**Which model the design should have is op's**, and I am not asking for it to change. I am saying the
sentence already ratified pins it, and the condition has to be derived from that sentence rather than from a
re-derivation of what a float is.

### 2.4 Where this lands

**`146` is right and `145` is wrong on the count.** Two conditions is not an incomplete statement of the
order; it is an unsound one, and the probe's 17,037 false positives are a stronger form of `146`'s finding
than a disagreement count would be.

**I would not state it as four conditions.** Two coordinates is the same content and it survives a rewrite
better, because it says why the four are four: the lattice contributes two and the interval contributes two,
and both halves are inclusions of familiar objects rather than a list. The four-condition form also invites
the reader to think the conditions are independent, and P depends on $q(N_2)$, which is G's own subject.

**`145`'s 2,025 pairs are a slice, not a sample, and that is the more useful way to say it.** Restricted to
one bias, one adjustment, one radix and one sign domain, two conditions are exact. The finding is not that
`145` measured badly. It is that the slice it measured is the slice on which the design's own preset tables
and worked examples live, which is why the gap survived to a second file.

## 3. Question two: componentwise, and the sign domain

**The order is componentwise. It is not componentwise in any coordinate the numeral declares.**

In the coordinates $(\mathcal{L}, I)$ it is exactly a product order, each factor under inclusion, and
section 2 measures that at zero error over 331,776 pairs. In the declared coordinates it is not, and the
probe separates two different failures that are easy to conflate:

**The product order on `Precision`, `Exponent` and `Domain` is unsound**, 2,064 false positives. Raising
precision and lowering the exponent together can shrink the top of the range, and the naive order does not
see it.

**After changing basis to (top magnitude, grid, domain) it becomes sound and incomplete**, 0 false positives
and 5,853 false negatives, of which 120 lie inside the slice where radix, adjustment and bias all agree.
Those 120 are the sign-domain coupling in isolation. The probe's witness:

```
source: r=2 p=1 e=-2 A=1 B=0 dom=AsymmetricLow  values {-1/2, -1/4, 0, 1/4}
target: r=2 p=2 e=-2 A=1 B=0 dom=Symmetric      values {-3/4 .. 3/4}
```

The sign domain goes **down** the rank `NonNegative < Symmetric < AsymmetricLow` and the inclusion holds
anyway, because precision went up and paid for it. Symmetrically, `AsymmetricLow` into `Symmetric` at equal
precision needs a strictly larger integer part, since $-r^{p}$ is one quantum past $-(r^{p}-1)$.

**So the sign domain is not a partition, and `146` is right about that.** I would put the positive statement
differently, and the difference matters for what gets written down. The sign domain is not "a coordinate of
the order" either. It is **one of the two inputs to the range coordinate**, the other being precision, and
they are coupled because both move the same endpoints. It touches the lattice coordinate not at all: the
sign domain never changes $q$ or $b$.

That is a sharper statement than either file's and it is checkable: in the probe, sweeping the sign domain
with everything else fixed changes $v^-$ and never changes $q$ or $b$.

**`146`'s self-limitation is correct and I would state it more strongly.** It says its own section on this
is subject to the same bound it applies to `145`, namely that a sweep is only as wide as its grid. It is.
My grid is wider on the axes that matter here (three sign domains crossed with two radices, two
adjustments and four biases, all varying simultaneously rather than one at a time) and the finding survives.
That is a second instance, not a confirmation of the first, because the two grids were built independently
and the second was built before the first was read.

## 4. Question three: lattice, or join-semilattice, or neither

Both files are wrong here, in opposite directions, and the underlying reason is the same for both.

### 4.1 The instrument, because the obvious one is worthless

Searching a finite grid of numerals for a greatest lower bound answers nothing. A unique maximum inside the
grid may be beaten from outside it, and an absent one may be supplied from outside it. So
`148_probes/p3_lattice_or_not.rs` does not do that.

It uses the fact that makes the question decidable. **A lower bound's value set sits inside the finite set
$V_1 \cap V_2$**, and that finiteness pins every parameter: the count pins the radix and precision together,
the elements pin the quantum, the placement pins the bias. So the lower bounds can be enumerated
**completely**, over every radix from two upward, every precision, every exponent, both adjustment
constructors, every bias and all three sign domains.

The upper-bound side is enumerated directly, and its completeness argument is the dual one. An upper bound
must contain the union, so its lattice contains the lattice the union generates, so its quantum divides that
lattice's. That pins the quantum to a unit fraction of one known value, and the requirement to cover the
union then pins the placement to finitely many offsets per count. The probe walks that region and takes the
minimal elements. **Two incomparable minimal upper bounds settle the question rather than merely suggesting
it**, for the reason spelled out in 4.2, and the probe reports "undetermined" separately from "absent" so a
bounded search is never read as a fact about the family.

One thing this made me correct in my own reasoning. I expected a cardinality obstruction, on the grounds
that a numeral's value set has $r^p$, $2r^p-1$ or $2r^p$ elements. That obstruction is much weaker than it
looks, because **the radix is a free parameter and every integer from two upward is a radix**, so an
11-element run is a numeral (radix eleven, precision one) whenever its quantum is expressible at radix
eleven. The probe found realisations by hand-checkable routes I had missed, for instance a five-point
progression at quantum one quarter as radix five, precision one, `FullRange<1>`, exponent minus one.

### 4.2 What it says

Two sweeps over 351 unordered pairs each, plus six named witnesses.

| slice | meet absent | join present | join absent, decided | join undetermined |
|---|---|---|---|---|
| unbiased, radix two, all three domains | **0** | 249 | **81** | 21 |
| biased, radix two, `NonNegative` | **184** | 198 | **100** | 53 |

**The family is a meet-semilattice and not a join-semilattice.** In the unbiased slice the meet is present in
every one of 351 pairs, and equals the intersection exactly in each of the three unbiased witnesses inspected.
The join is decisively absent in 81 of them, before any bias is involved.

"Decisively" is doing work and the argument is worth stating, because a bounded search cannot normally decide
an absence. If a least upper bound existed it would be contained in every upper bound, hence in each of the
two incomparable minimal ones found, hence equal to both by their minimality, which is a contradiction. And
the minimality is complete rather than region-bounded: anything strictly inside a found upper bound has at
most its cardinality and at most its grid refinement, so it is inside the searched region by construction.
The 21 and 53 pairs in the "undetermined" column are the ones where the search found no upper bound at all,
and those are reported separately rather than counted as failures.

**Why the meet works unbiased, stated as the mechanism rather than the measurement.** Every unbiased value
set contains the origin and is anchored at it: `NonNegative` starts there, `Symmetric` and `AsymmetricLow`
are centred there. So the intersection is anchored too, and among the anchored windows inside it there is a
largest. Bias removes the anchor, and the meet goes with it. The probe's case F is the clean witness: two
numerals on the same grid, in phase, whose intersection is six points at quantum one quarter, with **three
incomparable maximal lower bounds**, of sizes five, five and two. No numeral has the whole six-point
intersection as its value set, and among the numerals that do fit inside it there is no largest.

**Why the join fails, which is the finding neither file has.** An upper bound's index interval is a window
whose size is $r^p$, $2r^p - 1$ or $2r^p$. The union of two value sets has a hull of whatever size it
happens to have. When no available window size matches the hull exactly, the smallest window that covers it
**slides**, and every placement is an upper bound and no placement contains another. The probe's case D
exhibits minimal upper bounds of sizes 15, 27 and 63 over a union of 8 points, pairwise incomparable.

**Where each file lands.** `145` reports a lattice with exact meets and joins that overshoot at all 1,080
incomparable pairs. The exact-meet half is confirmed and is the durable part. The lattice claim is not:
overshooting is what an upper bound does, and the question is whether a **least** one exists, which for 81
of my 351 unbiased pairs it does not. `146` reports the inversion, the meet failing at 663,026 of 1,016,064
pairs while the join always exists. The meet half is right under bias and wrong without it, and I measure
the failure rate at 184 of 351 rather than roughly two thirds, on a different grid, so the two are not
comparable as numbers. The join half I take to be refuted: 81 decided failures unbiased and 100 biased.

I want to be careful about one way `146` could be right and I could be reading it wrongly. If it means that
**an** upper bound always exists, that is true and my sweep agrees. The word "join" carries "least", and the
design cannot use a join for anything unless it is least, since a non-least upper bound is a choice rather
than a consequence.

### 4.3 What follows for the canon, which is the part that matters

The design should not rest anything on lattice structure. Where it needs a common target for two numerals,
a mixed-strategy operation or a widening result, **that target is a named rule and not a mathematical
consequence**, because the least upper bound frequently does not exist and something has to pick among the
incomparable minimal ones.

That is the same defect the panel has now named four times, seen from the other side. `139b` adopted the
check that a written artifact standing in for a derivation is a defect named where it appears. This is a
**derivation standing in for a decision**, which is the same error mirrored, and it is the more dangerous
one, because a decision presented as a consequence never gets ratified.

## 5. Question four: does the narrowing schema need an adjudicating column

Three things are being asked at once here and separating them is most of the answer.

### 5.1 What I derived before reading either file

**A conversion is decode then quantise, and the two halves are keyed on opposite sides.**

Reading the source datum is governed by `<S1 as Lowering<N1::Exponent>>`: its `Container` is the type the
bits are in and its `StoredWidth` says how many there are. Producing the target value is governed by
`<S2 as Policy<N2::Exponent>>`: its `Quantisation` says the resolutions. Neither of those is a choice. Each
is forced by which contract carries the fact, which is the layer-keying rule the design already has
(`124`, the design-rules section).

So there is no adjudication to perform. The question "whose strategy decides" only looks like one question;
it is two questions with two different answers, and each answer is already determined.

**And half of the remaining ambiguity is closed by the preset key, before any argument about identities.**
`124:1.21` establishes that both contracts are keyed on the exponent form, and that one preset name denotes
two rows, one per number kind. The quantiser in a conversion is quantising onto the **target's** grid, so the
exponent form in the key is `N2::Exponent`. A reading that took the source's row would have to say which
exponent form it took it at, and taking it at `N1::Exponent` is not merely undesirable, it is nonsense at a
mixed-kind conversion: it would apply `ReduceModulo` to a float target, and floats have no modulus. So the
exponent-form half of the key is forced to the target by the quantiser's own subject.

What remains genuinely open is only the **preset-name** half, and for that I reach `146`'s answer by a
different route, in section 5.3.

### 5.2 On the column: `145` is right, and its argument for it is not

`146` proposes the schema record which side the resolutions come from. Every cell of that column reads
"target". Under `143b`, which op called settled canon, a constant is a function, and a function whose value
is derivable from its inputs is not data. Under `139b`'s standing check, a written artifact standing in for a
derivation is a defect named at the point it appears. A column whose every cell is derivable from the
quantiser's own subject is exactly that artifact.

So: **no column. A sentence.** The sentence is not "the target's strategy decides", which reads as a
stipulation. It is the decomposition:

> A conversion reads its source through the source's lowering and writes its target through the target's
> policy. The schema's resolutions column is the target's because the value being produced is of the target's
> type; the source contributes the datum's shape and nothing else.

That is one sentence, it survives a rewrite, and three independent implementations of it behave the same,
which is the equivalence test.

**But `145`'s argument for the absence of the column does not hold**, and it fails for a reason neither file
states. `145` offers the absence as "the cheapest available evidence that it is the quantiser rather than
something beside it". The absence of a column is evidence of nothing either way, because a derivable fact
never needs a column regardless of whether the mechanism carrying it is new. The inference has no direction.
`146` is right that the strongest claim in `145`'s narrowing section is the one to attack; I would attack it
on its logic rather than on its scope.

### 5.3 The identity-statement step, which is what the dispatch asks about

`146` derives the answer rather than choosing it, on the ground that the source reading breaks `Precise` and
the join reading breaks `Hot`, leaving one reading with both identities intact. **The load-bearing step is
that those identity statements are commitments rather than descriptions. It does not hold as stated.**

What op ratified at `70b` is **the two tables**, cell by cell. `Precise`'s fixed-point row reads
`Refuse`/`Refuse` for `OverRange`/`UnderRange`; `Hot`'s reads `ReduceModulo`/`ReduceModulo`. Those cells
describe **what the quantiser does under that preset**. The sentences `146` leans on, that "`Precise`'s
identity requires a refusing branch" and that a hardware instruction is "unconditional and infallible by
construction", are the panel's own glosses on those cells, written in `124:1.21` as the derivation of the
row rather than as a separate ratified claim. `124` says as much about the neighbouring cell in the same
paragraph: "The nearest-rounding ground is a re-derivation and is marked as one."

Read as descriptions of quantiser behaviour, the cells do not forbid the source reading at all. Under the
source reading nobody applies `Precise`'s row and gets a wrong answer; `Precise`'s row simply is not
consulted. To get from there to a contradiction you need the extra premise that the cell is a **type
invariant** over every value of `Number<N, Precise>`, and that premise is exactly what is not ratified.
`what-you-can-observe-is-what-you-guaranteed`, which `146` cites, governs the perimeter of an invariant; it
does not establish that there is one.

**So the conclusion is right and the argument offered for it is weaker than it reads.** That matters, because
an argument that rests on an unratified gloss will be re-opened by the first reader who checks the
ratification record, and the answer will look unsettled when it is not.

### 5.4 Two grounds that do not need the step

**One, from the preset key, and it is nearly a typing argument.** Section 5.1: the resolutions are a function
of the preset name and the exponent form, the exponent form is the target's, and at a mixed-kind conversion
the source's own row is not applicable at all. That closes the case where the two numerals differ in kind
outright, and it does so from `124:1.21`, which is ratified.

**Two, from two ratified cells of one column rather than from a gloss on one.** `Precise`'s column carries
both `Refuse`/`Refuse` and `StoredWidth = doubled`, and `124:1.21` states the doubled storage's purpose as
letting "a chain of operations retain more than one operation's exactness before a narrow forces a decision".
The narrow is the decision point the doubled storage exists to defer. Under the source reading a `Hot` source
makes that decision on `Precise`'s behalf, at the one site the doubled storage was paid for. The two cells
are then inconsistent with each other, and both are op's.

That is a better argument than the identity gloss because both of its premises are ratified cells, and
because it names a cost rather than a violated slogan.

### 5.5 Where this lands

`145`'s conclusion stands: the narrowing is the quantiser with the operation set to the identity, resolved by
the target strategy's row, and no new key column. `146` is right that `145` states the answer without an
argument, right that the three readings are materially different away from the embedding region, and right
that C1, C2 and C3 could not have found the divergence because they live where it is invisible. That last
observation is the most valuable thing in `146`'s section 6 and it should survive whatever else does.

I would not add the column, I would not rest the derivation on the identity sentences, and I would replace
`145`'s "no column is needed, therefore the narrowing is the quantiser" with nothing at all, because it is
not an inference.

## 6. The soundness question: the `From` spelling

Three questions were asked: is it sound, is the coherence argument watertight rather than merely untriggered,
and is the by-reference surface the right thing for a numeric primitive. My answers are yes, yes for a reason
other than the one given, and no.

### 6.1 The coherence argument is watertight, and it is watertight structurally

`146`'s reason is that `&Fixed<..>` and `Fixed<..>` "cannot unify", checked at the widths tried. That
understates it, and the stronger statement is what makes the result trustworthy rather than lucky.

Unification of a nominal ADT with a reference type fails **at the head constructor**, before any parameter is
looked at. There is no substitution of widths, sign domains, strategies, lifetimes or anything else that
changes a `&` into a struct, because the failure is one level above where the substitution happens. So the
argument is not "no counterexample was found among the cases tried"; it is that the search space has no
counterexample to find, for a reason that is a property of the type grammar rather than of arvo's types. That
distinction is precisely what the dispatch asks about, and the answer is that the property is structural.

Two further checks I would want before calling it closed, and both pass.

**Is `impl<T> From<T> for T` the only foreign impl that could overlap?** `Fixed` is arvo's, so the orphan rule
means only arvo can write `From<X> for Fixed<..>` unless the foreign impl has an unconstrained `Self`. In
core, only the reflexive impl does. A future core blanket of the shape `impl<'a, T, U> From<&'a T> for U`
would break the argument, and it would also break every crate in the ecosystem, so it is not a live risk.

**Is the impl itself well formed under the orphan rules?** `Self` is local, and `I1`, `F1`, `I2`, `F2` are
positions in the local type or const parameters. Nothing is uncovered.

**And I re-checked the wall that would have made the `&` unnecessary.** `146`'s f05 closes the by-value
structural route in a Peano encoding and diagnoses it as ambiguity rather than as a fact about Peano. If that
diagnosis were wrong, the design's own binary encoding would give a by-value `From` and the token would be
avoidable, so it is the one check worth repeating. `148_probes/p4g_strict_by_value.rs` builds it over the
design's `H | O<P> | I<P>` with a strictly irreflexive order, so the diagonal has no impl at all, and gets
`E0119` against core's reflexive impl on **both** the default and the next solver. The diagnosis transfers.
The `&` is doing real work and nothing cheaper does it.

### 6.2 The route as `146` spells it does not compile in this arrangement

This is the finding I would have most wanted a second read to produce, and it is checkable in two commands.

`146_probes/f03_ref_source_full.rs` carries `#![feature(min_generic_const_args, generic_const_args)]` and
`146_probes/run.sh` compiles it with `-Znext-solver=globally`. Both are outside the arrangement:

- `generic_const_args` is **not** on `unstable-features.md`'s allowed list. `min_generic_const_args` is; they
  are different features.
- The standing record, quoted in the design's own text at `124:1.2`, says `generic_const_args` "needs
  `-Znext-solver=globally`, mutually exclusive with the rest of the arrangement per the workspace's own
  record", and `unstable-features.md`'s 2026-05-29 resolution says the same in the other direction, that the
  flag "is mutually exclusive with `generic_const_exprs` at the compiler level, so migration is all-or-nothing
  with no incremental validation path".

Measured, on the pin, from `148_probes/run.sh`:

```
f03 WITH -Znext-solver=globally: exit=0
f03 WITHOUT it:                  exit=1
```

Without the flag it fails `E0308` at the call site: the impl is never selected. So **the spelling `146`
recommends is compiled evidence for a configuration the design has ruled out**, and its verdict sentence, "it
compiles gate-free of the coherence question", is true of the coherence question and not of the feature
question. That is not a small caveat. It is the difference between an answer and an answer conditional on
reversing a settled call.

I do not think this sinks the route. I think it locates the defect somewhere `146` did not look, and the
repair is one the workspace already has written down.

### 6.3 The repair, and it is the workspace's own standing answer

`a-refused-bound-wants-a-trait-not-a-feature.md`: when a bound is refused, break the constraint into pieces
that hold on their own, carry them on a trait, and bound on the trait instead of on an expression. The
feature `146` reaches for is needed only because its witness is an **associated-const projection sitting in a
const-argument position**, which is exactly the shape that rule names.

The order is a relation between widths. Carry it as a relation.

`148_probes/p4a_from_ref_trait_bound.rs`, exit 0, **default solver, no `-Z` flags, no feature gates at all**:

```rust
impl<I1: Nat, F1: Nat, I2: Nat, F2: Nat, G, S> From<&Fixed<I1, F1, G, S>> for Fixed<I2, F2, G, S>
where I1: Le<I2>, F1: Le<F2>
```

One impl. No enumeration. The `&` still does the coherence work, unchanged, and `p4b_by_value_control.rs` is
the same file by value and gets `E0119`, which is the control showing the token rather than the bound is what
defeats the overlap. Everything `146` establishes about the route survives: `(&a).into()` reaches it, the
reflexive case stays core's, a generic `A: Into<B>` reaches it, and a degenerate zero width is fine on both
sides.

**Two things it adds.**

A **higher-ranked bound over the numeral rather than over the reference** compiles, which is what a downstream
library actually wants to write:

```rust
pub fn hrtb<A: Copy, B>(a: A) -> B where for<'x> &'x A: Into<B> { (&a).into() }
```

So the `&` does not have to propagate into every generic signature downstream, which was my main worry about
the surface and which `146` does not address.

And it works over **the design's own width encoding** rather than over a toy one.
`148_probes/p4c_binary_widths.rs` builds `Pos` as `H | O<P> | I<P>`, the sealed binary positive `124:1.2`
names, with the order as a mutually recursive `PLe`/`PLt` pair, and compiles clean at widths up to 128, where
the type is eight constructors deep rather than 128.

### 6.4 The diagnostic, which the encoding decides and which nobody has priced

This is where the two encodings part company and it is worth a paragraph because it is the consumer-facing
half of the whole design.

Over Peano widths the refusal is unusable. `p4d_negative_controls.rs` produces the design's message and then
unrolls the entire tower, hides "7 redundant requirements", and writes a long-type file to disk.

Over the binary encoding the refusal is short, and **it loses the design's message entirely**, surfacing at
the innermost recursive bound instead (`p4e_binary_negative.rs`):

```
error[E0277]: the trait bound `H: PLt<H>` is not satisfied
```

That is the internals of a comparison, shown to a consumer who wrote `.into()`.

`#[diagnostic::do_not_recommend]` on every recursive impl fixes it, and I have not seen this combination
proposed anywhere in the panel. `p4f_do_not_recommend.rs`:

```
error[E0277]: this numeral does not embed into that one
    |     (&a).into()
    |     ^^^^ no exact embedding here
help: the trait `Le<Pv<O<O<O<H>>>>>` is not implemented for `Pv<I<O<I<H>>>>`
    = note: an embedding needs the target integer digits and fraction digits to be both at least
            the source. Where either shrinks the conversion is lossy and is written, and the
            strategy names what it does with what does not fit.
```

The design's own words at the consumer's line, both numerals named, internals suppressed. **The residual cost
is that the widths print as `Pv<I<O<I<H>>>>` rather than as 13**, and I could not find a way to make rustc
print the alias. That is a real and unsolved ergonomic cost of type-level widths, it is not specific to this
conversion, and it should be recorded where the width encoding is decided rather than here.

### 6.5 Is by reference the right surface for a numeric primitive

No, not on its own, and the reason is not the token.

`impl From<&A> for B` gives `&A: Into<B>` and does **not** give `A: Into<B>`. So every existing generic API
in the ecosystem written as `fn f(x: impl Into<B>)` is unreachable with a numeral by value, and every
downstream bound has to be written over a reference or a higher-ranked one. `146` prices the cost as "one `&`
at the call site". The call site is the cheap part. The bound surface is the expensive part, and it is the
part a library exports.

**The arm I would put beside it, and it costs nothing.** The `&` exists only to dodge core's reflexive impl.
An **arvo-owned** conversion trait has no such impl to dodge, so the identical condition can be carried by
value with one blanket impl and no coherence question at all:

```rust
pub trait Embed<T> { fn embed(self) -> T; }

impl<I1: Nat, F1: Nat, I2: Nat, F2: Nat, G, S> Embed<Fixed<I2, F2, G, S>> for Fixed<I1, F1, G, S>
where I1: Le<I2>, F1: Le<F2>
```

`148_probes/p5_erasure_and_by_value_arm.rs` compiles both impls side by side, exit 0, and they do not
conflict. The by-value bound `A: Embed<B>` is the natural one and needs no lifetime. The reflexive case is
arvo's here and should be included, since a numeral does embed into itself.

**And both erase.** I re-established the erasure claim rather than inheriting it, because `146`'s was measured
on a different impl under a different solver. Same file, `-O`, aarch64, the pinned nightly:

```
_scalar_via_from  = _scalar_by_hand
_scalar_via_embed = _scalar_by_hand
```

LLVM emitted one body and two aliases. Over a slice the three routes emit three bodies of 56 instructions
each, differing in nothing but local label numbering and the panic-location constant that carries each
function's own line number. So the `&` buys nothing at the machine level. It buys reachability through the
language's `From` and `Into`, and that is the whole of what it buys.

This is an ad-hoc compile check on one file, not a bench, and it proves an identity claim, which is what such
a check can prove.

**So the composition I would put forward, rather than a winner:**

| | carries | costs | who it is for |
|---|---|---|---|
| `Embed<T>` by value, arvo's own trait | the condition, one blanket impl, no coherence question, reflexive case included | a name nobody outside arvo knows | arvo's own code and every generic bound downstream |
| `From<&A> for B` on top | reachability through `.into()`, `Into`, and the ecosystem's vocabulary | one `&`, `?` unreachable, ecosystem `impl Into<B>` unreachable by value | the consumer writing at a call site |

The second is derived from the first: one impl, whose body is `self.embed()`. `146`'s single-arm answer pays
the bound-surface cost everywhere in order to buy the call-site spelling, and the two can be separated for
nothing.

**Whether both ship is op's**, and there is a real argument for the `From` alone, which is that two spellings
for one conversion is a vocabulary cost and this workspace has a standing preference against saying a thing
twice. I put the composition forward because the costs land on different people and because a library's
bound surface outlives its call-site spelling.

### 6.6 What I confirmed of `146` unchanged

`TryFrom` does not join, and I re-checked it in my own spelling on the default solver rather than inheriting
it: `p4h_tryfrom_beside.rs` gets `E0119` against `impl<T, U> TryFrom<U> for T where U: Into<T>`, the same
diagnostic `146`'s f07 reports. Its reading of that as agreement between a coherence wall and a design intent
stands.

The refusal carrying the design's own text stands, and improves with `do_not_recommend`.

The enumeration of ten dead routes stands. I added one, the strict-order-by-value route in the design's own
binary encoding, which is the eleventh and which fails the same way.

## 6b. What of this is canon, and what is audit trail

Per `the-canon-is-intent-not-implementation.md`, the canon carries the intent and points here for the
evidence. In the canon's register, and with no code in it:

> **The order.** A numeral denotes a set of values. One numeral **embeds** in another exactly when its value
> set is contained in the other's. For a constant-exponent numeral the value set is an affine lattice
> intersected with an interval, and the embedding is the conjunction of two inclusions, one per factor: the
> lattice must refine and pass through the source's phase, and the interval must cover at both ends. For a
> ranged numeral the value set is a union of grids, one per magnitude band, and the embedding is the
> pointwise comparison of the two local-grid functions together with the same two endpoints. The condition
> set is keyed on the exponent form, which is the key both strategy contracts already take.
>
> **On the declared coordinates.** The order is not the product order on precision, exponent and sign domain.
> Precision and sign domain jointly determine one endpoint and neither determines it alone, so a numeral can
> lose ground on one and gain the embedding on the other. Any predicate stated over the declared coordinates
> is a specialisation of the order and holds only where each coordinate controls one condition alone; the
> order is stated over the value set.
>
> **The structure.** Under inclusion the numerals have greatest lower bounds wherever their value sets are
> anchored at a common point, and do not in general have least upper bounds. A covering numeral's index
> interval has one of the sizes its radix and precision allow, and where no allowed size matches the span of
> two numerals taken together, the smallest covering numeral slides and no placement is least. **So a common
> target for two numerals is a named rule and not a consequence**, and the design states the rule.
>
> **The conversion.** A conversion reads its source through the source's lowering and writes its target
> through the target's policy. It is the quantiser with the operation set to the identity, so it introduces
> no situation, no resolution and no key the design does not already have.
>
> **The cast.** An exact conversion between numerals is available through the language's own conversion
> trait, conditioned on the order, with the condition carried as a relation between the numerals' own
> parameters rather than enumerated over them. The reflexive case belongs to the language. The conversion is
> total on the source's value set, value-preserving, and erases to the code the operation would have emitted
> written by hand. Where the order does not hold the refusal names both numerals and names the remedy.

Everything else in this file is audit trail: the counts, the probes, the diagnostics, the assembly. Those
age. The sentences above are meant not to.

## 7. What I could not settle, and the alternatives I did not take

**The `Ranged` model is op's and I did not resolve it.** Section 2.3b shows the design's ratified sentence and
`146`'s reading disagree, and shows which conditions follow from each. Which one op wants is a design call. I
would note only that the ratified sentence is IEEE's own convention, which is the standard section 0.1 names.

**The join's absence needs a decision, not a derivation, and I did not make it.** Section 4.3 says the design
must name a rule for picking a common target where the least upper bound does not exist. I did not work out
what that rule should be. The obvious candidates are the smallest covering window anchored at the source's
own phase, or at zero, or at the join of the two lattices with the hull rounded up in a stated direction;
each is a design call with observable consequences and none of them is forced.

**I did not price the `Le` recursion's compile-time cost.** A mutually recursive trait resolved at every
conversion site is trait-solver work, and at width 128 the recursion is eight deep with two traits in play.
`arvo-compile-time-last.md` says that cost is the one we pay willingly, and it also says the claim should be
measured rather than assumed. It is unmeasured. Measuring it needs the bench harness rather than a spike, and
the honest sentence is that the question is unpriced.

**I did not check whether the sign domain and strategy belong in the `From` impl's parameter list.** `146`'s
f11 shows one impl can cover both axes and correctly leaves whether it should to op, citing `145`'s X12. I
have nothing to add and did not want to add a third opinion to a question two files have already left open in
the same place.

**The alternatives I did not take, listed so the next reader starts from them rather than from nothing.**

*A newtype wrapper on the source instead of `&`.* `Widening(a).into()`. Structurally identical to the `&` for
coherence purposes and strictly worse at the call site, since the wrapper is arvo's name rather than the
language's punctuation. Not compiled; the reasoning is the same one line.

*A one-element array, `From<[A; 1]> for B`.* Same trick, `[a].into()`, worse. Not compiled.

*An inherent method named `into`.* `146` compiled and priced this (f06, f15) and rejected it on shadowing. I
agree and did not repeat it. An inherent method under a **different** name is not shadowing anything, and
that is what `Embed::embed` is in section 6.5, arrived at from the other direction.

*Making the target's width a projection of the source's.* `146`'s f14 closes the strategy-slot version. The
width version needs const arithmetic in type position, which is `generic_const_exprs`, forbidden. Under
type-level widths it becomes an associated-type projection against a free parameter, which is the same
ambiguity f14 reports. Not compiled, closed by the same diagnosis twice.

*`min_specialization`.* `146`'s f13 closes it: core's impl is not `default`, so there is nothing to
specialise. Nothing to add.
