# 146. The order, the cast, and what a second read is for

**Date:** 2026-08-07
**Position:** after `145b_op_checkpoint_thirtynine.md`. Second read on `145`'s order and on `145`'s narrowing,
plus the `From` question op refused to let close.
**Probes:** `146_probes/`.

## Verdict, stated first

The order is right and the reason given for it is not. `145` derives inclusion from two conditions, a grid
condition and a range condition, and compiles it over the one slice where two conditions suffice. The general
numeral needs **four**: grid refinement, phase alignment, low endpoint, high endpoint. Bias is what creates
the phase condition, and no amount of widening the shape matrix would have found it, because the whole matrix
`145` swept has bias zero. Verified against an element-by-element oracle at 3,969 ordered pairs with zero
mismatches, and the phase condition demonstrated by a pair whose grid is finer and whose range is wider and
which is still not included (`o1_order_general.rs`, F).

The order is also componentwise in **three** coordinates rather than two, and stops being componentwise at
three sign domains. The design declares `NonNegative`, `Symmetric` and `AsymmetricLow`. Over two of them the
componentwise reading on `(I, F, sign)` is exact, 1,764 pairs, zero failures. Over all three it fails at 91 of
3,969 ordered pairs, the first non-degenerate one being `Q0.1 AsymmetricLow` into `Q1.1 Symmetric`, which is
an inclusion the coordinates say is impossible.

The antichain is a stronger result than `145` claims and a narrower one. Stronger, because it is not a fact
about `(I, F)` at all: **inclusion between two finite sets of equal cardinality forces equality**, so any two
distinct numerals with the same number of values are incomparable whatever their bias, adjustment, radix or
sign domain. 254,016 equal-cardinality ordered pairs in a biased and adjusted family, zero cases of inclusion
without equality. Narrower, because the theorem is about cardinality and `145` states it over precision. Those
coincide for fixed point and part company for `Ranged` numerals, where two numerals of equal significand
precision and nested exponent ranges are strictly comparable: 41 values inside 73, compiled.

And the pattern is not specific to fixed point. For `Ranged` numerals the componentwise reading on
significand precision and exponent window fails at 201 of 2,304 ordered pairs, because adding a significand
digit while holding the window loses values off the bottom. The condition that works moves the window with the
precision, zero failures. Three instances of one failure in one design: the sign domain, the bias, and the
exponent window.

The lattice claim inverts. `145` reports meets preserved exactly and joins overshooting. Generally it is the
**meet** that stops existing: 663,026 of 1,016,064 ordered pairs in the biased family have empty intersection,
and the empty set is not a numeral. Joins always exist. The family is a join-semilattice in general and a
lattice only on the unbiased slice.

On the narrowing, the design is right and the "no new key column" claim is not. The key is missing a column,
and it is missing it in the place a missing key always hides, which is the place the author was most confident.
Section 6 has it.

On `From`, op is right and the refusal was premature. **A blanket `From` between arvo numerals compiles, with
a computed order witness, no enumeration, one impl.** The spelling takes the source by reference. Coherence
never has to evaluate the witness because `&Fixed<..>` and `Fixed<..>` cannot unify, so the overlap with
core's identity impl cannot arise for any widths at all. It compiles gate-free of the coherence question,
reaches through a generic `T: Into<U>` bound, refuses the antichain pair with the design's own text, and
**lowers to nothing**: LLVM emitted `_scalar_via_conversion = _scalar_by_hand` and
`_loop_via_conversion = _loop_by_hand`, the same symbol, which is op's "all lowered inlined" as an identity
rather than as a comparison. The cost is one `&` at the call site, uniform across every cross-numeral pair,
and rustc suggests it when omitted.

`TryFrom` does not join it. Section 7 has the enumeration: ten routes, each with the compiler
diagnostic that closed it.

## Contents

1. The gates
2. The order, derived independently
3. Where the order stops being componentwise
4. The antichain, and what it is a theorem about
5. The narrowing, second-read
6. The key column that is missing
7. The cast, and every spelling I tried
8. What is op's

---

## 1. The gates

**The canon gate passes.** This is canon work: intent that survives a rewrite, checked against
`the-canon-is-intent-not-implementation.md`. Nothing below proposes source. The two tests it names are the
ones I applied to each sentence I would put forward. Permanence: would it still be true after the
implementation is rewritten. Equivalence: would three independent implementations of it behave the same. The
four-condition order passes both; `145`'s two-condition order passes neither, because it stops being true the
moment a numeral carries the bias the design already declares.

**On the test gate, and why I did not run the suite.** The standing gate says run everything and read the
bodies rather than the names. Its purpose is to stop an expert building on fabricated coverage. Nothing in
this file rests on the suite: every claim here is carried by a probe in `146_probes/` that I wrote and ran.
And under `145b` the tier that suite belongs to is declared dead, which is what licenses canon work at all, so
a green or a red there measures an artifact that is going away and would not move a sentence in this file
either way. `145` records the run at 694 passed, 0 failed, 9 ignored. I have no reason to doubt it and no use
for it.

**On the corrected reading of the source ban.** I read the prior conversion surface as a prior attempt at the
same intent, which is what the correction licenses, and it earned its place exactly once: it is why section 7
tests whether `TryFrom` can sit beside `From` rather than assuming it. Nothing in section 7's answer comes
from it. Every route there is decided by a compiler diagnostic on a file in `146_probes/`.

**Every probe here is a spike.** Names, arities, field orders and which case each instantiates are
scaffolding chosen to reach a check. Cite them for what they proved. That applies to `145`'s probes too, and
where I disagree with `145` I disagree with the claim rather than with the code.

---

## 2. The order, derived independently

I want the order from what a numeral is, not from what fixed point happens to be, because a predicate derived
from the coordinates that exist today is the failure `136:383-387` names and this would be its third instance.

A numeral determines a set of values. Under the design's own axes that set is built from a quantum (the
adjustment times the radix to the exponent), an offset (the bias), and a count (the precision, through the
sign domain). Every one of those is a rational or an integer. So:

$$V(N) \;=\; \{\, b_N + q_N \cdot j \;:\; 0 \le j < n_N \,\}$$

**A numeral's value set is a finite arithmetic progression of rationals.** That is the whole of what the order
has to work with, and it is worth saying because it is what makes the answer general: nothing below asks what
the numeral's coordinates are called.

**Claim.** $V(A) \subseteq V(B)$ if and only if all four hold:

| condition | statement | what it is about |
|---|---|---|
| grid | $q_A / q_B \in \mathbb{Z}$, vacuous when $n_A < 2$ | the target resolves at least as finely |
| phase | $(b_A - b_B) / q_B \in \mathbb{Z}$ | the target's grid passes through the source's offset |
| low | $b_B \le b_A$ | the target reaches at least as low |
| high | $b_A + q_A(n_A - 1) \le b_B + q_B(n_B - 1)$ | the target reaches at least as high |

Checked against an element-by-element oracle over every ordered pair of fixed-point shapes at $I + F \le 5$
across all three sign domains, 63 shapes, 3,969 ordered pairs, **zero mismatches**
(`o1_order_general.rs`, A). The conditions and the elements agree everywhere.

`145` states two conditions and calls the factoring exhaustive: "inclusion factors into exactly two
independent conditions: is the target's grid at least as fine, and is the target's range at least as wide"
(`145:206-208`). Range is two conditions rather than one, which is a presentational point and becomes a real
one for a signed target where the two ends move independently. Grid is two conditions rather than one, and
that is not presentational.

**Refinement is not alignment.** A grid can be strictly finer than another and still miss every one of its
points. Compiled (`o1_order_general.rs`, F): the source is $\{0, 1, 2, 3\}$ with $q = 1$ and $b = 0$; the
target is $\{-0.25, 0.25, 0.75, \ldots, 9.25\}$ with $q = 1/2$ and $b = -1/4$.

```
F. grid refines: true
F. range covers: true and true
F. phase aligns: false
F. a is a subset of b: false
```

The target's grid is twice as fine and its range strictly contains the source's, and not one source value is
representable. Under `145`'s two conditions this pair is an embedding. It is not.

**Why the sweep could not have found it.** Every numeral in `145`'s matrix has $b = 0$, because every one of
them is an unbiased fixed-point numeral, and at $b_A = b_B = 0$ the phase condition reads $0 \in \mathbb{Z}$
and is discharged for free. 2,025 pairs, 4,050 counting both signs, and the condition is invisible in all of
them. This is not a sampling error in the sense `135:266` warns about, where someone chose which
instantiations to include. It is narrower and more instructive: the whole family available to be swept was
the family in which the condition is vacuous, so no widening of the sweep inside that family would have
helped. Only leaving it does.

`145` says as much in its own section 3, "What the order is not", and predicts the pattern: "The order
predicate as I compiled it is keyed on `I` and `F` and is therefore keyed on the ones that exist today"
(`145:277-279`). It then states the general condition as "the source's lattice is a subgroup of the target's
coset and the source's range fits" (`145:272-274`). That sentence is close and it is not the condition. A
subgroup relation between the two lattices is the grid half. Membership of the source's coset in the target's
coset is the phase half, and "coset" appears in the sentence attached to the target rather than as the thing
the two must share. Stated as four conditions there is nothing to interpret.

**What survives.** On the unbiased dyadic slice with two sign domains, which is the region `145` compiled, the
four conditions collapse exactly to componentwise comparison of $(I, F, \mathrm{sign})$, zero failures over
1,764 ordered pairs (`o1_order_general.rs`, B and G). `145`'s order is correct where it is stated. The defect
is in the scope of the statement, not in the statement.

---

## 3. Where the order stops being componentwise

Two things break it, and one of them is already in the design's declared axis list.

### The third sign domain

`110`'s `SignDomain` is a three-element set: `NonNegative`, `Symmetric`, `AsymmetricLow` (the axis block at
`124:3238`). `145` sweeps two of them, unsigned and signed, one at a time, and reports "2,025 ordered shape
pairs across both signs" (`145:216-226`) as separate `O` and `S` failure counts. Two consequences, and the
second is the one that matters.

**Cross-sign pairs were never checked.** Sweeping unsigned pairs and then signed pairs is not sweeping the
order, because the order relates a numeral to every numeral. The unsigned numeral at $(I, F)$ **is** included
in the signed numeral at $(I, F)$: same quantum, phase difference $2^{I+F}$ which is an integer, and the
signed range covers the unsigned one at both ends. So the sign domain is a coordinate of the order and not a
partition of it. `145`'s own section 10 lists "the sign-domain change" as not designed, which is right about
the conversion and understates the omission: it is not a missing conversion, it is a missing axis of the
order.

**With three sign domains the order is not componentwise at all.** Ranking the domains
`NonNegative < Symmetric < AsymmetricLow` and comparing componentwise on $(I, F, \mathrm{rank})$ fails at 91
of 3,969 ordered pairs (`o1_order_general.rs`, B). The first non-degenerate one:

```
B. componentwise failures, two sign domains: 0
B. componentwise failures, three sign domains: 91
B. first counterexample: Q0.1 AsymmetricLow into Q1.1 Symmetric:
   componentwise says false, inclusion is true
```

`Q0.1` asymmetric is $\{-1, -0.5, 0, 0.5\}$. `Q1.1` symmetric is $\{-1.5, \ldots, 1.5\}$. The first is inside
the second. The coordinates say it cannot be, because the sign rank went up while $I$ went up too, and the
componentwise reading demands both move the same way. The endpoints do not care about the rank; they care
about where the numeral actually ends.

This is the same failure as the phase one wearing different clothes. Both are cases of a range or grid fact
being encoded in a coordinate that is not the fact. **The four conditions are stated on the value set's
endpoints and quantum, and they hold in all 3,969 pairs including these 91.** Componentwise comparison of
declared coordinates is a specialisation, and it is correct exactly on the slice where each coordinate
controls one condition alone.

### Bias, and the meet that stops existing

`145` reports the numerals form a lattice, that $V$ preserves meets exactly, and that joins strictly
overshoot at every incomparable pair (`145:229-234`). On the unbiased dyadic slice all three hold and I
reproduce them: zero empty intersections over 441 pairs, and componentwise minimum is the meet in all 1,764
pairs of the two-sign-domain slice (`o1_order_general.rs`, D and G).

Admit bias and the first of the three stops being well-formed. Over a family with rational quanta and rational
biases, 1,008 numerals, **663,026 of 1,016,064 ordered pairs have empty intersection**
(`o1_order_general.rs`, D). The smallest instance printed is $\{-3, -2\}$ against $\{-1.5, -0.5\}$: two
perfectly legal numerals with the same quantum whose grids are offset by half a step, sharing no value at all.
The empty set is not a numeral, so the meet does not exist in the family.

The join does. The least arithmetic progression containing two others always exists, with quantum the greatest
common divisor of the two quanta and their offset difference, and range the union's hull. So the correction is
a swap of which half is the well-behaved one:

> The numerals form a **join-semilattice** under inclusion. The join always exists and strictly contains the
> union of the two value sets, because a union of two value sets is not a value set. The meet exists when the
> two grids share a phase, and does not otherwise. On the unbiased slice every pair shares the phase through
> zero, so there the structure is a lattice and the meet is exact.

`145`'s emphasis reads the other way round, that meets are the exact half and joins the lossy half, and it
builds on that: "That asymmetry is not a curiosity. The join with a carry digit is the sum numeral, which is
why heterogeneous addition has a clean numeral-level map" (`145:236-237`). **That consequence survives
unchanged**, and in fact strengthens, because the join is the half that exists generally. It is the meet that
was carrying a claim it cannot carry.

### The third instance, and it bounds this file as well as `145`

My four conditions are stated for a value set that is an arithmetic progression. A `Ranged` numeral's value
set is not one: it is a union of progressions, one per exponent. So section 2 decides the order for the
`Implicit` family and does not decide it for `Ranged`, and I would be repeating the exact charge I just laid
if I let that pass as a footnote.

The same failure recurs there, and worse. Over 48 `Ranged` numerals at significand precisions 2 to 4 and
exponent windows drawn from $[-3, 0] \times [0, 3]$, the componentwise reading on the declared coordinates,
precision up and window out at both ends, **fails at 201 of 2,304 ordered pairs** (`o2_ranged_order.rs`):

```
R. componentwise failures 201
R. first counterexample: p2 [-3,0] into p3 [-3,0]:
   componentwise says true, inclusion is false
```

**Adding a significand digit while holding the exponent window loses values off the bottom.** A value
$3 \cdot 2^{-3}$ at precision two has to be written $6 \cdot 2^{-4}$ at precision three, and $-4$ is outside
the window. So "more precision is at least as good" is false for floats, which is a statement worth having in
the canon on its own account.

The condition that does decide it, compiled with **zero failures over all 2,304 pairs**, moves the window with
the precision: with $d = p_B - p_A$,

$$p_A \le p_B, \qquad e^{\min}_B \le e^{\min}_A - d, \qquad e^{\max}_A - d \le e^{\max}_B.$$

Three instances of one pattern now, in one design: the sign domain, the bias, and the exponent window. Each
time a predicate is stated over the coordinates that were in hand, each time the coordinates and the condition
coincide on the slice that was checked, and each time the repair is to state the condition over the value set
and let the coordinates specialise it. `136:383-387` named the pattern. This is what it costs when it is
allowed to run three times.

**And the two results that do survive all of it.** The cardinality theorem holds in the `Ranged` family too:
204 equal-cardinality ordered pairs, zero cases of inclusion without equality. And the equal-precision
antichain fails there loudly: **252 ordered pairs of equal-precision `Ranged` numerals are strictly
comparable**. That is section 4's scope limit, compiled at scale rather than at one instance.

---

## 4. The antichain, and what it is a theorem about

The brief asks whether "antichain" is the right structural claim or an artifact of the widths chosen. Neither.
It is right, it is not an artifact, and it is a theorem about something other than widths, which means both
its proof and its scope in `145` are wrong.

### It needs no sweep

For a componentwise order on two coordinates restricted to a fixed sum, an antichain is immediate: if
$I_1 < I_2$ then $F_1 > F_2$, so neither dominates. `145` states exactly this argument (`145:239-241`) and
then reports it "compiled at nine families, zero violations". The compilation establishes a property of
$\mathbb{Z}^2$, not a property of arvo. That is not a defect in the result. It is a defect in what the record
will say was established, and this panel has already had one file cite a check for more than the check
performed.

### The general form is a cardinality theorem

Drop the coordinates entirely. If $V(A) \subseteq V(B)$ and $|V(A)| = |V(B)|$ with both finite, then
$V(A) = V(B)$. So **two distinct numerals with the same number of values are incomparable**, for every bias,
every adjustment, every radix, every sign domain, in every family the design admits, including ones nobody has
written down yet.

Checked over a family with quanta $\{1, 1/2, 1/3, 1/4, 2, 2/3, 3, 3/2, 3/4\}$, biases in
$\{0, \pm 1, \pm 2, \pm 3\}$ over denominators $1$ through $3$, and counts 2 through 5: 1,008 numerals,
**254,016 equal-cardinality ordered pairs, zero cases of inclusion without equality** (`o1_order_general.rs`,
C). The theorem is one line and the sweep is a check on the model rather than on the theorem, which is the
right relationship between the two.

This is a better argument for op's withdrawal than the one `145` gives, by exactly the margin `145` improved
on the two before it. `130:170-193` argues from a consumer's mistake. `131` section 7 argues from a false law.
`145` argues from the coordinates. This argues from counting: **numerals of the same size cannot include one
another unless they are the same numeral**, so keying a numeral on its size collapses a set of pairwise
unrelated things to a point, and the size was never going to determine which one you had.

### And it is narrower than `145` states, in the family that matters most

`145` says "every equal-precision family is an antichain" (`145:239`). The theorem is about **cardinality**.
Those coincide when precision determines cardinality, which is true for fixed point, where an $(I, F)$ numeral
has $r^{I+F}$ values. They part company for `Ranged` numerals, where the value count depends on the exponent
range as well as the significand precision.

Compiled (`o1_order_general.rs`, E). Two float-shaped numerals with **the same significand precision** of
three digits, exponent ranges $[-2, 2]$ and $[-4, 4]$:

```
E. narrow has 41 values, wide has 73
E. narrow is a subset of wide: true
E. the two are equal: false
```

Strict inclusion at equal precision. So the sentence "every equal-precision family is an antichain" is **false
for `Ranged` numerals** and true for fixed point, and the true general sentence is the cardinality one.

This does not weaken the withdrawal. It strengthens it in the direction `145` did not look: for fixed point,
equal precision means pairwise incomparable, and for `Ranged` numerals equal precision does not even fix how
many values there are. In neither family does precision determine the numeral. The withdrawn requirement is
wrong for two different reasons in the two families, and the reason `145` gives is the fixed-point one.

### The restatement I would put forward

`145` offers an extension of `131:570-573` and I would extend it once more, because its added clause is
stated over the coordinates and the reason is not about coordinates:

> Two numerals of equal precision have the same `Precision`, and `Precision` is a type. They are not the same
> numeral. Where precision fixes the number of values, as it does for fixed point, no two distinct numerals of
> one precision are related by the order at all, because a finite set cannot be a proper subset of a set its
> own size. Where precision does not fix the number of values, as for a `Ranged` numeral, it fixes even less.
> Either way the map between two such numerals exists, is a quantisation, and is written.

---

## 5. The narrowing, second-read

The design is right. I rebuilt the quantiser from the ratified preset rows without reading `145`'s probe, over
the same shape matrix, and every number came back identical (`n1_quantise_key.rs`, and `n1b_with_degenerate.rs`
for the shape set including the degenerate `Q0.0`):

| law | `145` | this file |
|---|---|---|
| C1, quantise from a numeral to itself is the identity | 3,076 checked, 0 failures | 3,076 checked, 0 failures |
| C2, on the embedding region quantise equals embed | 8,464 checked, 0 failures | 8,464 checked, 0 failures |
| C3, embed then quantise equals quantise | 236,992 checked, 0 failures | 236,992 checked, 0 failures |
| C5, monotonicity failures | Hot 72,778, Warm 0, Cold 0, Precise 0 | Hot 72,778, Warm 0, Cold 0, Precise 0 |

That is a second instance rather than a re-run: a separately written rounding function, a separately written
classifier, a separately written enumeration, landing on the same four figures including the one that is not a
zero. **X10 and its consequence N5 are corroborated.** `Hot`'s narrowing is not monotone because
`ReduceModulo` is not order-preserving, and by M7 it therefore does not distribute over the lattice operations,
which is the conversion-side twin of N1.

I did not re-check C4, the double-rounding refutation. A refutation is settled by one counterexample and
double rounding has a textbook one; a disagreement count measures the sweep rather than the claim, and
re-running it would have produced a second number about a thing already known.

What else I endorse, and why rather than that:

**The narrowing is the quantiser with the operation set to the identity.** The quantiser is defined over five
situations relative to a representable set. A conversion presents an exact value, because a value of the
source numeral is exact by construction, and asks for the target's representable set. Same five situations,
same five resolutions. `145`'s argument that the identity is a value in the operation-marker column rather
than a new column is correct: the marker carries whether the grade monoid is trivial, and the identity's is.

**Round-then-classify is forced rather than chosen.** Q4 and Q5 define the overflow band as the set of exact
values that lie in range and round out of it, which is a statement about an already-rounded value. An
implementation that classifies first is implementing a different map. This is the clause an implementation is
most likely to get wrong and it should be written as a law rather than left as a derivation.

**`Warm` and `Cold` are the same map here**, so the narrowing has three behaviours over four strategies. That
is the ratified table's shape rather than something the conversion chapter introduced.

**On `Precise` refusing on `inexact`**, `145` recommends out-of-range only and carries it to op as the
narrowest live call. I reach the same recommendation from a structural reason rather than from one-rule
grounds, and the reason is section 4's. "Is this value exactly representable in that numeral" is the order
applied to **a value**, not to a type. A law's key is a list of types (`124:1399-1401`), so a value-dependent
condition cannot be a cell in the resolution table without the table acquiring a column that is not a type.
Folding exactness into `Precise`'s row would put a per-value predicate where the schema holds per-type
declarations. The consumer who wants it wants a different operation with a different signature, and it should
be named as one.

## 6. The key column that is missing

`145`'s strongest claim about the narrowing is a negative one, and it is the one to attack, because it is
offered as the evidence for everything else:

> The key schema needs no extension, and that is a result rather than a convenience. [...] Every column
> already exists. If the narrowing were a new mechanism it would have wanted a new key column, and it does
> not, which is the cheapest available evidence that it is the quantiser rather than something beside it.
> (`145:759-761`)

The schema it is measured against, verbatim:

> **The key** (`40:276-286`): the operation, whose marker carries whether its grade monoid is trivial; the
> operand numerals and, for a widening operation, the result numeral; the `Quantisation` resolutions and,
> where a quantiser sits between the exact operation and the result, its `Direction`; for a fold, the
> accumulator numeral and the arity. (`124:1399-1402`)

Two of those columns do not fit a conversion, and one of the two is material.

### The result numeral is guarded on the wrong condition

The second column reads "the operand numerals and, **for a widening operation**, the result numeral". A
narrowing conversion is not a widening operation. Under the schema as written its target numeral is therefore
in no column at all, which is absurd, since the target numeral is the entire content of the map.

The guard is right for arithmetic and states the wrong reason for being right. For same-format addition the
result numeral is determined by the operands, so recording it would be recording a derived fact; for a
widening operation it is not determined, so it has to be recorded. **The condition the guard is reaching for is
"whenever the result numeral is not determined by the operands", and "widening" is the only case of that which
existed when the clause was written.** A conversion is the second case. One clause covers both, and this is
the same shape as section 2's and section 3's: a general condition specialised to the instances in hand and
then written down as the specialisation.

That is a repair to the schema's wording rather than a new column, and on its own I would agree with `145`
that no column is added.

### But the resolutions column does not say whose, and a conversion has two

For an arithmetic result there is one strategy. Two operands resolve to one through `Resolve`, and "the
`Quantisation` resolutions" unambiguously means that one's. A conversion has a source strategy and a target
strategy, and the schema does not say which of them the quantiser reads. `145` answers "the target strategy's
own row" (`145:757-759`) and gives no argument for it anywhere in the file.

Three readings are stateable and each has a case:

**The target's.** The consumer named the destination format, and a format's behaviour should be a property of
the format.

**The source's.** The value's provenance decided its policy, and a `Precise` value should not silently change
its overflow discipline because someone stored it somewhere.

**The join.** This is the rule the design already uses whenever two strategies meet, and a conversion is that
situation with one operand.

**They are materially different.** Over every ordered pair of shapes at $I + F \le 6$, every ordered pair of
strategies, and every source value, 331,776 conversions (`n1_quantise_key.rs`):

```
K. conversions checked 331776 of which lossy 298368
K. target vs source disagree 98628 (98628 of them lossy)
K. target vs join   disagree 49314
K. source vs join   disagree 49314
K. first: Q0.2 Hot into Q0.1 Precise, raw 3:
          target says None, source says Some(1), join says None
K. on the embedding region: 33408 checks, 0 disagreements
```

98,628 of the 298,368 lossy conversions, 33.1 percent, get a different answer depending on which strategy
adjudicates. The two halves are exactly complementary, 49,314 each, which is what the chain forces: the join
equals whichever of the two is more conservative, so where they differ it agrees with one and disagrees with
the other.

**And the disagreement is invisible exactly where the exact map lives.** On the embedding region all three
readings agree at 33,408 checks with zero disagreements, which is a consequence of C2: where the map is exact
there is nothing for a resolution to resolve. So a file that establishes "the narrowing is the quantiser" by
checking C1, C2 and C3 will find the mechanism confirmed and the ambiguity underneath it untouched, because
C1 and C2 live on the region where the question does not arise and C3 fixes one strategy on both sides. That
is why this is where a missing key hides: not in a corner, but under the very checks that certified the
mechanism.

### The call, derived rather than preferred

Both alternatives are refutable against a strategy's own stated identity, and the same probe produces the
witness for each.

**The source reading breaks `Precise`.** `Q0.2 Hot`, raw 3, into `Q0.1 Precise`: the target row refuses and
the source row lands the value. A `Precise` numeral then holds a value that `Precise`'s own row would have
declined to produce. Under `what-you-can-observe-is-what-you-guaranteed` the conversion is part of the
perimeter of whatever `Precise` promises, and a promise that a differently-tagged source can route around is
not a promise.

**The join reading breaks `Hot`.** `Q0.2 Precise`, raw 3, into `Q0.1 Hot`: the target row lands the value and
the join refuses. `Hot`'s identity is that it is unconditional and infallible by construction, and under the
join reading a `Hot` destination acquires a refusing branch whenever the source happened to be tagged
`Precise`. That is the fallibility projection appearing in the one preset defined by not having one.

**The target reading leaves both identities intact**, and it is the only one of the three that does. So the
column's value is derivable rather than a taste call, which is the outcome I would want and is not the
outcome I expected when I started looking. What is not derivable is that the column exists, and it does:

> A conversion's `Quantisation` resolutions are the **target** numeral's strategy's, and the schema records
> which side the resolutions come from. For an operation with one result and one strategy the column is
> discharged by `Resolve` and reads as it always did. For a conversion, where the two sides carry independent
> strategies and 33.1 percent of lossy conversions disagree between them, the side is stated.

`145`'s conclusion that the narrowing is the quantiser survives this in full. What does not survive is the
argument offered for it, that the key needed no extension, since the extension it needed is the one the
argument would have had to look at.

## 7. The cast, and every spelling I tried

op:

> So if, and when, we do the From and TryFrom impls, it is again, no enumeration, implicit via blankets and
> granular bounds where expressing it otherwise fails. But I am not sure what the problem is; From should be
> clear cut, no? It is a cast and we have all we need to do it on compile time, all lowered inlined.

He is right on every clause, including the one that reads as a rhetorical question. There is a blanket, it
carries a granular bound, it enumerates nothing, it is decided entirely at compile time, and it lowers to the
same machine code as the operation written by hand.

### The problem, stated once so it stops being restated

core ships `impl<T> From<T> for T`. An impl of the shape
`impl<A, B> From<Fixed<A>> for Fixed<B>` unifies with it at `A = B`, and coherence rejects a possible overlap
rather than an actual one. Coherence **does** read a where clause and will accept the impl when the clause is
definitely unsatisfiable at the overlap, which is what `145_probes/e2` established. What it cannot do is
**evaluate** a condition at free parameters, which is what `145_probes/e3` established. So a computed order
witness, which is the only kind that does not enumerate widths, is exactly the kind coherence cannot use.

Every route below either supplies coherence with a unification failure instead of an evaluation, or fails.

### The route that compiles

Take the source **by reference**.

```rust
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S>
    From<&Fixed<I1, F1, G, S>> for Fixed<I2, F2, G, S>
where
    Picker: EmbedWitness<{ <Pair<I1, F1, I2, F2> as Tagged>::TAG }>,
```

`&Fixed<..>` and `Fixed<..>` are different type constructors. There is no substitution of any widths, any sign
domain and any strategy that makes them equal, so the overlap with core's identity impl **cannot arise**, and
coherence never reaches the witness at all. The witness is then free to be the computed Pattern C tag the
container projection already uses. One impl. No enumeration. Compiles clean (`f03_ref_source_full.rs`, exit 0).

Four properties, each compiled:

**It is reachable without naming a numeral.** `f03` includes `fn generic<A, B>(a: A) -> B where A: Into<B>`
and calls it at `generic(&a)`. The blanket `Into` picks it up, which is one of the two registers `145`
identifies as "reads as free".

**It refuses the antichain pair with the design's own words** (`f04_ref_negative.rs`), in both directions:

```
error[E0277]: this numeral does not embed into that one
38 | pub fn antichain_a(a: UFixed<13, 3, Warm>) -> UFixed<8, 8, Warm> { (&a).into() }
   |                                                                         ^^^^ no exact embedding here
   = note: an embedding needs the target integer digits and fraction digits to be both at least
           the source. Where either shrinks the conversion is lossy and is written, and the
           strategy names what it does with what does not fit.
note: required for `Fixed<8, 8, Unsigned, Warm>` to implement `From<&Fixed<13, 3, Unsigned, Warm>>`
```

Both numerals in full, the design's note at the consumer's line, and the remedy named. The
`EmbedWitness<1>` against `EmbedWitness<0>` leak `145` flags is still there and its repair is still `145`'s
two-row named verdict, unchanged by anything here.

**One impl covers both axes if the design wants it to.** `f11_both_axes.rs` widens the strategy to two
independent parameters and still compiles, exit 0, covering numeral-only, strategy-only, both-at-once and the
fully reflexive case. Whether it **should** is section 8: `145`'s X12 says a retag fails to commute with the
operations, which is a reason to keep the strategy fixed in the impl and leave the retag written. The
mechanism does not force either way, and that is the right place for a design call to sit.

**It lowers to nothing.** This is op's own claim and it is checkable, so I checked it rather than agreeing
with it. `f12_codegen.rs` writes the same computation twice, once through `(&a).into()` and once as a
hand-written shift, at scalar and over a slice, and emits assembly. LLVM did not produce two similar function
bodies to be compared. It produced one, twice:

```
_loop_via_conversion = _loop_by_hand
_scalar_via_conversion = _scalar_by_hand
```

Symbol aliases. The conversion and the hand-written shift are the same machine code, established as an
identity rather than as a comparison of instruction counts. **The erasure gate at `135b` is untouched**: the
consumer names widths, the typestate derives the container, the witness validates at the signature, and
nothing survives lowering. The `&` is an expression-level token and never enters a type the consumer writes.

This is one `rustc -O` invocation on a thirty-line file and it is an ad-hoc compile check, not a bench. It
proves an identity claim, which is exactly what such a check can prove and all I am asking of it.

### What it costs

**One `&` at the call site.** `(&a).into()` where a consumer would write `a.into()`. Three things about that
cost, and the third is the one that decides whether it is acceptable.

It is **uniform**. Every cross-numeral conversion needs it and none is exempt. That is the distinction from
`145`'s route B, which compiles by moving the strategy as well and therefore makes implicitness available for
some numeral pairs and not others depending on a coordinate with nothing to do with the question. That
asymmetry has no semantic content. This one has exactly one: the reflexive case belongs to the language and
every other case belongs to arvo, and the token marks the boundary.

It is **suggested**. A consumer who writes `a.into()` gets, from rustc and with no arvo involvement
(`f02_ref_source.rs`):

```
error[E0277]: the trait bound `Fixed<13, 3, Unsigned, Warm>: Into<Fixed<20, 8, Unsigned, Warm>>` is not satisfied
help: consider borrowing here
   |
55 |     (&a).into()
   |     ++ +
```

The repair is a machine-applicable suggestion. The failure mode of this spelling is a compiler error with a
one-character fix attached, which is the cheapest failure mode a design gets to choose.

And **`?` does not reach it** (`f16_question_mark.rs`, `E0277`). `?` desugars to `From::from` on the value, so
a by-reference impl is invisible to it. `145:663` uses `?` reachability as an argument against letting a
strategy retag read as free, on the ground that it moves a value into different semantics at a site where no
resolution is happening. Under this spelling that route is closed by construction rather than by discipline.
I read that as the spelling's second-best property after erasure, and I note that it also means a numeral can
never silently widen in an error position, which nobody asked for and which is the right answer anyway.

### The dead routes, with the diagnostic that closed each

Ten, including the three `145` recorded. Every row is a file in `146_probes/` or in `145_probes/` and every
verdict is a compiler's.

| route | what it tries | closed by |
|---|---|---|
| naive blanket, by value | `impl<..> From<Fixed<A>> for Fixed<B>`, no condition | `E0119` against `impl<T> From<T> for T` (`f01_baseline.rs`) |
| computed witness, by value | the same, bounded on the Pattern C tag | `E0119`; coherence cannot evaluate a projection at free parameters (`145_probes/e3`) |
| closed witness, by value | the same, witness enumerated per width pair | compiles, and is the enumeration op refused at `127b:36-50`, at $O(W^4)$ (`145_probes/e2`) |
| unbounded impl with a const-assert body | admit every pair, check in the body, take a post-monomorphisation error | not reachable: coherence rejects before a body exists (`145` section 9, and `f01` is the same refutation) |
| structural order over type-level widths | Peano `Z` / `S<N>` with a recursive irreflexive `Lt`, so the exclusion is a unification failure rather than an evaluation | `E0119` under **both** the default and the next solver (`f05_peano_strict.rs`). The recursion never bottoms out at an inference variable, so the solver reports ambiguity and coherence reads ambiguity as overlap |
| negative reasoning | `impl<T> !Distinct<T> for T` plus a positive blanket | `E0751` for the pair, then `E0119` anyway (`f08_negative_impls.rs`). The half that would work is `with_negative_coherence`, which `unstable-features.md` forbids as a compiler-internal gate |
| specialisation | `min_specialization` | `E0119` (`f13_min_specialization.rs`). core's identity impl is not `default`, so there is nothing to specialise |
| projection in the target's strategy slot | `for Fixed<I2, F2, G, <S as Bump>::Out>`, where `Bump` has no fixed point, so the reflexive case is semantically excluded | `E0119` (`f14_strategy_projection.rs`). A rigid projection against a free parameter is ambiguous, and coherence reads ambiguity as overlap |
| by value, moving numeral and strategy to two named strategies | `145`'s route B | compiles (`145_probes/e4`), and is refused on design grounds: it is an enumeration over the strategy set and it makes implicitness depend on a coordinate that is not the question |
| `TryFrom` by value | `impl<..> TryFrom<Fixed<A>> for Fixed<B>` | `E0119` against `impl<T, U> TryFrom<U> for T where U: Into<T>` (`f10_tryfrom_value_alone.rs`) |

### `TryFrom`, and why it does not join

Two compiled facts, and together they are a design result rather than a limitation.

`TryFrom<&Fixed<A>> for Fixed<B>` **is** coherent on its own (`f09_tryfrom_ref_alone.rs`, exit 0). Placed
beside the by-reference `From` it is not (`f07_tryfrom_on_top.rs`, `E0119` against
`impl<T, U> TryFrom<U> for T where U: Into<T>`), because core's blanket covers every `U` that is `Into<T>`
and coherence cannot evaluate our witness to learn which pairs those are.

So the design gets **one of the two, on one source shape**. That is not a loss, because the design already
decided it does not want the other. `145`'s narrowing is total for `Hot`, `Warm` and `Cold`, and refuses under
`Precise` through the fallibility projection rather than through a `Result` in a conversion trait. A `TryFrom`
between numerals would be a second fallibility mechanism carrying one strategy's behaviour in a trait the
other three do not need. The mechanism and the design agree, and the agreement is worth recording because it
is the only place in this file where a coherence wall and a design intent point the same way.

### The arm I priced and would not take

An **inherent** `into` on the numeral compiles by value (`f06_inherent_into.rs`, exit 0), with the target
widths inferred from the expected type, and it costs nothing at the call site: `a.into()`, no token. Inherent
methods win over trait methods in resolution, so it shadows `Into::into` on the numeral.

That shadowing is the price, and it is not small (`f15_inherent_cost.rs`):

```
error[E0308]: mismatched types
36 | pub fn other_target(a: UFixed<13, 3, Warm>) -> Degrees { a.into() }
   |                                                -------   ^^^^^^^^ expected `Degrees`,
   |                                                                   found `Fixed<_, _, Unsigned, Warm>`
```

A genuine `From<Fixed<..>> for Degrees` exists in that file and `.into()` cannot reach it, because the
inherent method took the name. Every conversion out of a numeral to anything that is not a numeral loses
`.into()` as its spelling, and the error a consumer gets is a type mismatch that names neither the shadowing
nor the remedy. A numeral will certainly want conversions out of the numeral family. So: recorded as an arm,
compiled, priced, and not recommended. It is also not a `From` impl, which is what op asked about.

### What of this belongs in the canon

The canon carries the intent and points at this file for the evidence, per
`the-canon-is-intent-not-implementation.md`. The intent, in the register the canon uses:

> An exact conversion between numerals is available through the language's own conversion trait, conditioned
> on the order, with the condition computed from the numerals' own parameters rather than enumerated over
> them. The reflexive case belongs to the language; every other case belongs to arvo, and the surface marks
> the boundary uniformly rather than per pair. The conversion is total on the source's value set,
> value-preserving, and erases: it emits the code the operation would have emitted written by hand. Where the
> order does not hold, the refusal names both numerals and names the lossy conversion as the remedy. The
> fallible conversion trait is not used between numerals; a strategy that refuses does so through the
> fallibility projection.

Every sentence there survives an implementation rewrite and none of it prints the impl. Whether it is doable
is answered by `146_probes/f03`, `f04`, `f11` and `f12`, which is where that question is supposed to be
answered.

## 8. What is op's

**Mine, and compiled.** That a numeral's value set is a finite arithmetic progression and that inclusion over
that family is four conditions rather than two, checked against an element-by-element oracle at 3,969 ordered
pairs with zero mismatches. That the fourth, phase alignment, is created by bias and is invisible in every
numeral `145` swept, demonstrated by a pair whose grid is finer and whose range is wider and which is still
not included. That the sign domain is a coordinate of the order rather than a partition of it, so the
cross-sign pairs were never checked, and that at the design's three declared sign domains the componentwise
reading fails at 91 of 3,969 ordered pairs. That the meet stops existing once bias is admitted, at 663,026 of
1,016,064 ordered pairs, so the family is a join-semilattice generally and a lattice only on the unbiased
slice. That the antichain is a cardinality theorem, holding for every bias, adjustment, radix and sign domain,
254,016 equal-cardinality ordered pairs with zero inclusions without equality, and that stated over precision
it is false for `Ranged` numerals, where 252 equal-precision ordered pairs are strictly comparable. That the
componentwise reading fails a third time for `Ranged` numerals, at 201 of 2,304 pairs, because a significand
digit costs an exponent, and that the window-shifted condition decides it with zero failures.

That `145`'s quantiser reproduces exactly from an independently written one: C1 at 3,076, C2 at 8,464, C3 at
236,992, all zero failures, and `Hot`'s monotonicity failures at 72,778 against zero for the other three. That
the adjudicating strategy is not determined by the design and is material on 33.1 percent of lossy
conversions, that all three readings agree on the embedding region so the checks that certified the mechanism
could not have seen it, that the source reading lands a value in a `Precise` numeral that `Precise` refuses,
and that the join reading gives a `Hot` numeral a refusing branch.

That a blanket `From` between arvo numerals compiles with a computed witness and no enumeration when the
source is taken by reference, that it refuses the antichain pair with the design's own text, that one impl
covers both axes if wanted, that it is reachable through a generic `Into` bound and not through `?`, and that
it emits the identical symbol to the hand-written operation. That `From` and `TryFrom` cannot both exist on
one source shape. That the ten other routes I tried either do not compile or are refused on
design grounds, each with the diagnostic or the reason that closed it.

**His, because the canon's register is his.** Whether the order goes in as the four conditions or as the
componentwise form with a scope note. My read is the four conditions, and the reason is not elegance: the
componentwise form is already false for the third sign domain the design declares, so shipping it as the
statement would ship a wrong sentence on the first day rather than on some later one. But which abstraction
level a canon speaks at is a call about the document, not about the mathematics.

**His, because it is a column that does not exist yet.** Which strategy adjudicates a conversion. I derived
the target's rather than preferring it, and the derivation leans on two of the presets' identity statements
being load-bearing enough to refute an alternative: that `Hot` is unconditional and infallible by
construction, and that a `Precise` numeral does not hold values `Precise` would have refused. If either is a
description rather than a commitment, the derivation weakens to a preference and the call is open. What is not
open is that the column exists: three readings, 98,628 disagreements, and the schema currently names none of
them.

**His, because it changes the consumer surface.** Whether the by-reference `From` ships. The whole cost is one
`&`, uniform, machine-suggested when omitted, and erased. Against it: it is a token, and a token on the most
common conversion in the library is exactly the kind of thing a consumer notices every day. I would ship it,
and I hold that lightly, because `145` reached the opposite ruling on the same axis from a different
mechanism and this panel's record says calls of this shape get overturned.

**His, and downstream of that one.** If it ships, whether the impl fixes the strategy or spans both axes. Both
compile. `145`'s X12 is the argument for fixing it: a retag preserves every value, changes the resolution of
every later operation, and fails to commute with the operations on 48.8 percent of operand pairs per
off-diagonal cell. That argument is about what should read as free rather than about what can, so it decides
this, and I agree with it. But it is his ruling and it was already on his list.

**His, unchanged from `145`.** Whether `Precise` refuses on `inexact` or only out of range. I reach the same
recommendation from a different reason, that a per-value predicate cannot be a cell in a per-type table, which
is a structural argument rather than a one-rule one and may be worth more than the recommendation.

### Owed under the two-expert rule

I am the second read on `145`'s order and on `145`'s narrowing. On the narrowing I confirm the mechanism and
dispute the key claim. On the order I confirm the result on the slice it was compiled over and dispute its
stated scope in three places. **Where I dispute, my dispute is a first read and owes its own second**, and
that covers: the four-condition order, the window-shifted condition for `Ranged`, the join-semilattice
correction, the cardinality form of the antichain, the adjudicating-strategy column and the derivation of its
value, and the by-reference `From` in all four of its compiled properties.

**The premise a second read should attack is mine and it is this.** I asserted that a numeral's value set is a
finite arithmetic progression and derived four conditions from that. It is true for the `Implicit` family and
false for `Ranged`, which I found and repaired by compiling a second condition, and the repair is the tell: I
did not derive one condition covering both families, I derived a second condition for the second family. So
the honest statement of section 2 is not "here is the order" but "**the order is inclusion of value sets, it
is decidable, and how it is decided is a property of the family**", with two families now compiled and no
argument that there will not be a third. Someone should ask whether a single condition covers both, because if
one does then this file has done to `136:383-387`'s pattern what `145` did, one level up.

### The alternatives I did not take

**Reproducing `145`'s sweep and reporting agreement.** The cheapest possible second read and it would have
returned green, because `145`'s numbers are correct. Dropped because a second read that re-runs the first
read's check confirms the check rather than the claim, and the claim's defect was in its scope, which no
re-run reaches.

**Attacking the value-set premise directly**, which is the second read `145` asks for at `145:895-904`. I
believe it holds and I did not compile it. Left open on purpose: it is a different question from the two I was
sent for, and answering it badly in passing would be worse than leaving it named.

**Fixing the `EmbedWitness<1>` against `EmbedWitness<0>` leak.** `145`'s two-row named verdict already does
it and my probes reproduce the leak rather than the repair, because the repair is not in dispute and
re-deriving it would have cost a probe to confirm something already established.

**Enumerating the `From` impls over a closed derived set**, which is the shape that rescued coherence in
`145_probes/e2`. I looked for a closed set the order could be decided over and there is none: the strategies
and the sign domains are closed, the widths are not, and the order is a predicate on the widths. Recorded
because it is the natural next idea after reading `e2` and it costs a paragraph to close rather than a probe.

**A wrapper type on the source**, `Exact(a).into()` or similar. It compiles trivially, for the same reason the
by-reference spelling does, and it is strictly worse: two tokens instead of one, a type the consumer has to
learn, and no machine-applicable suggestion when it is omitted. Named so nobody spends a probe on it.

**Asking whether coherence will ever learn this.** `145` lists it as unchecked and I did not check it either.
The mechanism that would reopen the by-value spelling is negative coherence, which `unstable-features.md`
forbids as a compiler-internal gate with no tracking issue and no stabilisation path. So the wall is not a
missing feature, it is a feature the workspace has already decided against, and that is worth knowing before
anyone reads the by-reference spelling as a workaround for a temporary problem. It is not temporary.
