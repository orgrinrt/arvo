# 18. The denotation clause: what it costs, and whether the design already breaks it

**Date:** 2026-08-08. **Register:** breadth pass. Nothing here settles. **Dispatch:** test the first
clause of `08`'s boundary sentence. What it excludes and who loses, what admitting the strongest
candidate costs, and whether the design already denotes sets without saying so.

I did not run `git log` in this repository before my answer was on disk, per `RULES.md:193-201`.

The short form, before the argument, because the file is long and the third question is the one
that moved. The clause is right about what it excludes and wrong about what it describes. Every
representation it turns away is genuinely a different layer, and the exclusion is cheap for all
but one constituency. But the clause is stated as a property of every datum, and the design
already has at least two places where a datum in flight stands for a set rather than a point, one
of which the design's own algorithm crates depend on for soundness. So the clause is a statement
about a numeral's **constructor** wearing the grammar of a statement about its **data**, and the
difference is measurable rather than a matter of phrasing.

## 0. Gates, and the brief's cheap factual claims

### 0.1 The canon gate

Nothing here conflicts with a ratified row. The clause under test is `08`'s own proposal, offered
in its section 5 as "the thing to attack", so attacking it is what the artifact asks for. I checked
`SETTLED.md` for a row bearing on denotation and found none: the closest are the `TotalOrd` split
at `:116`, the inclusion conditions at `:118`, and the sign-domain refutation at `:121`, none of
which says what a datum means. **The record has no denotation statement at all**, which is itself
the finding this file circles back to in section 4.

One open row turns out to be the same question as mine and is named where it arises: `Precise` on
`inexact` (`SETTLED.md:173`, open since `145`), in section 3.4. I did **not** engage the `Ranged`
model (`SETTLED.md:174`), which `08:373-374` treats as the home for one of the five encoding-only
cases, because nothing in the denotation clause bears on it.

### 0.2 The test gate

There is no suite to run. `mock/crates` is being nuked and is not evidence, per the dispatch, and
the panel's evidence is probes rather than tests. So the gate takes the form the panel actually
supports: I read the bodies of the probes I depend on rather than their headline numbers, and I
checked their feature gates.

```
grep -c '^#!\[feature' 07_probes/*.rs 08_probes/*.rs   ->  zero anchored gates in every file
```

`07_probes/p5_postfixpoint_accumulator.py:104-141` is the probe my section 3 extends, and reading
it rather than its summary changed what I built. Its accumulator is `U<3,0>` with elements from
`U<2,0>`, and its inner loop is `acc = R(acc + p, VA)` at line 126. **Additions only, over
non-negative elements.** So its zero-failure result under the absorbing reading is quantified over
a monotone non-decreasing operation set, which the file's prose does not say and which section 3.2
turns out to depend on entirely.

One defect in my own probes, recorded rather than quietly fixed, because the workspace's test gate
names this exact shape as setup that helps. `18_probes/p3_interval_laws.py` tested `A - A == 0`
over a slice that held the first argument fixed at the one interval for which the law is true, and
reported 136 successes. `18_probes/p3b_interval_laws_fixed.py` and `p3c` correct it, and the
corrected answer is 16 of 136. The original probe stays on disk with its output, since a probe that
proved the wrong thing is part of the trail.

### 0.3 The brief's cheap factual claims

Four were handed to me. Three hold and one is backwards.

**"`08` proposed a one-sentence boundary and its first clause is a denotation clause."** Holds,
verbatim at `08:556-560` and glossed at `:564`. The sentence is:

> A representation is a numeral when a datum denotes one rational, when the denotable magnitudes
> in each binade of some admitted radix form one arithmetic progression at one phase whose step is
> that radix to some power, and when the set is fixed by the type alone.

**"Intervals and stochastic streams fall outside on exactly that clause."** Holds for both, at
`08:320` and `:330`. **It does not hold for affine forms**, which the brief groups with them.
`08:331` marks affine forms outside on a set denotation **of runtime-varying arity**, and `08:492`
says plainly "the arity is the reason", a hard collision with the const-size constraint that no
format concept repairs. So affine forms fail two clauses and the denotation clause is not the one
doing the work. That matters for section 1, because it means removing the denotation clause would
not admit them.

**"`08` reports the design's named shapes are meet-closed and not join-closed."** Holds, at
`08:603`, with the measurement at `i2c.out` and `i2e.out`.

**"`07` reports that rounding sits in an adjunction with the embedding."** **Backwards.** That was
`07`'s own prediction and `07` refuted it. `07:309-330` measures all six rounding modes as lower
adjoints, and finds that only round toward positive infinity is adjoint to **the embedding**; the
others are adjoint to a map that returns a cell or a half-cell. The section is titled "What each
mode is actually adjoint to, which refuted my prediction". Reasoning from the brief's version would
have inverted section 2 of this file, since the whole content of `07:317-330` is that the upper
adjoint is not the embedding, which is precisely how a set enters the picture.

## 1. What the clause excludes, and who loses

The dispatch asks for the constituency by name and for what each would have to do instead, and it
sets the right standard: arvo ships tools rather than policy (`arvo-toolbox-not-policer.md`), so
"most consumers do not want it" is not a reason, and an exclusion nobody notices is free and should
be recorded as free.

I take the exclusions in order of how much they cost, rather than in `08`'s classification order.

### 1.1 The one that is not free: rigorous interval arithmetic

**Who.** Verified global optimisation and root isolation, where the answer is a certificate rather
than a number. Rigorous ODE and reachability solvers. Exact geometric predicates, where a sign
test has to be right and a filter reports "cannot decide" rather than guessing. Collision detection
and broad-phase culling with conservative bounds, which is a workload arvo's own downstream has.
Constraint solvers doing interval propagation. Static range analysis, which is a compiler workload
and one vehje would plausibly have.

**What they do instead.** Hold a pair of numerals and round the two ends in opposite directions.
`08:474-478` measures that this is sufficient: over twenty thousand random interval pairs under
addition and multiplication on `U<3,3>`, outward rounding fails zero times, while using one
directed mode for both ends fails 1036 and nearest for both fails 946.

**What it costs them, precisely, and it is not zero.** Three things, and `08` names only the first.

The condition `08` states, at `:483-484`, is that **both directed rounding modes must be reachable per
operation rather than fixed at the numeral**. That is a constraint on the design, not an absence,
and it is the sharpest thing in `08`'s section 4.5.

The second is the algebra, which section 2 measures. A consumer building the pair themselves gets
no law layer from the design, and the laws they need are not the numeral's laws.

The third is that they lose the value-level total order, which section 2.1 measures at 42.05% of
pairs decidable on a small numeral and falling with width. Every comparison in their code becomes a
three-way answer, and no amount of construction-above-a-numeral recovers it, because the loss is in
the object rather than in the machinery.

So the honest form of `08`'s "costs nothing, on one condition" is: **costs nothing to express, on
that condition, and costs the whole law and order layer, which the design was never going to
supply and which the consumer therefore has to write.** That is still a defensible exclusion. It is
not a free one.

### 1.2 Free, and recorded as free: stochastic streams

**Who.** Ultra-low-power inference hardware, some FPGA signal paths, and probabilistic computing
research. A stochastic stream is a bit sequence whose expected density is the value, so a datum
denotes a distribution rather than a set, which is a third kind of denotation and not merely a
wider one.

**What they do instead.** `Bits<N>` and their own decoder. The design ships the storage and has no
opinion about the mapping.

**What it costs.** Nothing that I can find. The whole point of a stochastic stream is that
arithmetic becomes bitwise and correlation-sensitive, so a numeral's arithmetic is not wanted. This
is the model exclusion: the constituency is real, the mechanism is genuinely elsewhere, and nobody
notices the boundary.

### 1.3 Free to exclude and worth a sentence: the storage-layer encodings

`08:510-521` covers block floating point, microscaling, delta and frame-of-reference, which fail
the third clause rather than the first. I read that section and agree with it and have nothing to
add, which per `RULES.md:99-101` is a result rather than a gap. The one thing I would carry
forward is `08`'s own framing at `:521`: what the design owes such a consumer is one sentence
saying the residual's numeral is the thing to declare, so a reader stops looking for a
block-floating-point axis.

### 1.4 Not excluded by this clause at all, and the brief says otherwise

**Affine forms and Taylor models.** Section 0.3. They fail on arity, and admitting set denotations
would not admit them. Their constituency is automatic differentiation with error tracking and
verified numerics with correlated variables, and their answer is the same as the interval answer
plus a growing symbol list the design cannot size.

**Exact reals and continued fractions.** `08:324` puts them outside on "infinite, not const-size".
Same shape: the denotation clause is not what turns them away.

The pattern is worth naming because it changes what section 4 can conclude. Naming rather than
counting, since a count needs a command: of the families ordinarily grouped as "denotes something
other than a point", the denotation clause is what excludes **interval and triplex forms** and
**stochastic streams**, and const-sizedness is what excludes **affine forms and Taylor models** and
**exact reals and continued fractions**. Those are `08`'s own table rows at `:330`, `:320`, `:331`
and `:324`. Const-sizedness is a constraint from an entirely different part of the design, so
**removing the denotation clause would admit two of the four and not the other two**, and the
clause is carrying less weight than the sentence's symmetry implies.

## 2. What admitting intervals would cost

Intervals are the strongest candidate: `08` already measures the construction as sufficient, the
constituency is real, and unlike affine forms nothing else about them collides with the design. So
I followed them through the four places the dispatch names.

### 2.1 The order, which is where it breaks

The value-level order is the one a law can quantify over: `A <= B` has to mean that every concrete
value `A` stands for is at most every concrete value `B` stands for, since that is the only reading
under which the comparison transfers to the values a law is about. Call it the separation order.

`18_probes/p1_order_under_set_denotation.py`, over the 16 values of `U<2,2>`:

| denotation | data | pairs | comparable | share |
|---|---|---|---|---|
| point | 16 | 120 | 120 | 100.00% |
| rounding cell | 16 | 120 | 120 | 100.00% |
| absorbing top | 16 | 120 | 120 | 100.00% |
| interval | 136 | 9180 | 3860 | **42.05%** |

And it gets worse with width, which rules out the reading that 16 values is the artifact:

```
U<1,1>  base values   4  interval data    10  comparable 68.89%
U<2,2>  base values  16  interval data   136  comparable 42.05%
U<3,3>  base values  64  interval data  2080  comparable 35.45%
```

**The first three rows are the finding, not the fourth.** A cell denotation and an absorbing top
keep the order total, and the reason is structural rather than lucky: those denotations are
pairwise disjoint, so the separation order on them is the order on their representatives. The
probe reports 15 overlapping pairs for the cell reading, which is exactly the 15 adjacent pairs
sharing a closed endpoint, an artifact of modelling a cell as a closed pair; a half-open model
gives zero and changes no comparability count.

Intervals overlap, at 6120 of 9180 pairs, and that is the whole of the loss.

So the cost to the order is not "intervals are partially ordered", which everyone knows. It is
that **`SETTLED.md:116` records the value-level order as RATIFIED and as a precondition of the
distributivity law**, and an interval numeral does not have one. The law layer would have to be
restated in a different order, which is section 2.2.

### 2.2 The laws, and one of them is not weakened but gone

`18_probes/p3b_interval_laws_fixed.py` and `p3c_interval_law_directions.py`, over `U<2,2>`
unsigned and `I<1,2>` signed, 136 interval data each, integer tick arithmetic, outward rounding,
out-of-range sub-expressions skipped and counted per `07:295-307`'s separation of overflow from the
quantiser.

| law | numeral | checked | equality | lhs contained in rhs | incomparable |
|---|---|---|---|---|---|
| commutativity of + | U<2,2> | 3876 | 100.00% | | 0 |
| commutativity of * | U<2,2> | 5520 | 100.00% | | 0 |
| associativity of + | U<2,2> | 54264 | 100.00% | | 0 |
| distributivity | U<2,2> | 89853 | 69.49% | **100.00%** | 0 |
| distributivity | I<1,2> | 384294 | 33.12% | **100.00%** | 0 |
| associativity of * | U<2,2> | 229710 | 65.91% | 82.56% | **1818** |
| associativity of * | I<1,2> | 857651 | 69.37% | 84.13% | **9524** |
| A - A == 0 | I<1,2> | 100 | 16.00% | 16.00% | 0 |

Three separate things, and they do not point the same way.

**Distributivity survives, weakened to an inclusion.** Every one of the 27414 unsigned and 257007
signed failures has the left side contained in the right, at 100% in both, which is classical
subdistributivity reproduced here rather than assumed. A law that fails directionally is still a
law: it is a statement the algebra satisfies and a consumer can rely on.

**Associativity of multiplication is gone, and this is the casualty.** It fails as an equality a
third of the time, and the failures split evenly between the two containment directions with a
residue of 1818 and 9524 pairs where **neither side contains the other**. Witness at
`p3c.out`: `((1,5), (2,3), (8,11))` gives `(0,11)` one way and `(1,12)` the other. There is no
weakening to state, because there is no direction to state it in. Exact interval multiplication is
associative; outward rounding at each step is what destroys it, so this is a cost of putting
intervals **on a grid** rather than a cost of intervals.

**The additive inverse goes, and its loss is quantified.** Only the 16 degenerate data satisfy
`A - A == 0`, and the other 84 in-range data have `0` strictly inside `A - A`, with the width
growing linearly:

```
width  0 ticks : 16 data     width  8 ticks : 12 data
width  2 ticks : 15 data     width 10 ticks : 11 data
width  4 ticks : 14 data     width 12 ticks : 10 data
width  6 ticks : 13 data     width 14 ticks :  9 data
```

That is the dependency problem, and it is the reason interval arithmetic is a different subject
rather than a wider numeral.

### 2.3 The shape space, the meet and the join

`08:603` measures the design's three shapes as meet-closed and not join-closed. An interval family
over a numeral does not change that, and the reason is that it does not touch the shape space at
all: an interval numeral's shape is a **pair of the same shapes**, so intersections and joins are
computed componentwise in the product, and a product of a meet-semilattice with itself is a
meet-semilattice with the same closure properties. I did not build a probe for this because the
argument is one line and a probe would only restate it, which `RULES.md:138` names as a defect.

What does change is `07`'s adjunction picture, and here the brief's inverted claim matters. `07`
establishes at `:317-330` that the upper adjoint of a rounding mode is not the embedding but a map
returning the cell. An interval-valued numeral **is that upper adjoint made into a type**. So
admitting intervals does not change what the adjunction is between; it names the right-hand side
that `07` found was already there. That is a genuinely surprising direction and it is the reason
section 3 exists.

### 2.4 The container derivation and erasure, which cost the least

The dispatch flags that an interval-valued numeral that cannot erase would be a real exclusion.
**It erases.** `18_probes/p4_interval_erasure.rs`, on the pin, zero anchored feature gates:

```
rustc +nightly-2026-05-28 --edition 2021 -O --emit asm --crate-type lib \
  p4_interval_erasure.rs --out-dir asm
```

The assembler's own output folds three symbol pairs:

```
_p4_typed_interval_add = _p4_raw_pair_add
_p4_typed_scalar_add   = _p4_raw_scalar_add
_p4_wide_interval_add  = _p4_raw_wide_pair_add
```

So a two-endpoint typed numeral lowers to exactly the hand-written pair of containers, at 16-bit
endpoints and at 128-bit endpoints alike. The four size and alignment assertions in that file
compare one declared type against **another declared type** rather than against a number the same
derivation produced, which is the line `17:686-689` draws through its class D.

One cost the erasure check cannot see, and it is worth reporting because it is the thing a reader
would expect erasure to catch. At 128-bit endpoints the aggregate stops fitting the return
registers and goes out through the indirect-return pointer:

```
_p4_raw_wide_pair_add:            _p4_wide_scalar_add:
    adds x9, x4, x0                   adds x0, x2, x0
    adc  x10, x5, x1                  adc  x1, x3, x1
    adds x11, x6, x2                  ret
    stp  x9, x10, [x8]
    adc  x9, x7, x3
    stp  x11, x9, [x8, #16]
    ret
```

Two stores through `x8` against a pure register return. **The typed and raw wide forms fold**, so
this is attributable to the aggregate's size and not to the abstraction, and the erasure gate is
met either way. It is a calling-convention consequence a consumer pays for holding two 128-bit
endpoints, on `aarch64-apple-darwin`, and no bench harness has run so its magnitude is
**unpriced**.

The container derivation itself: the stored width doubles and nothing else changes, because the
two endpoints are numerals of the same shape and the derivation runs twice. That is a much smaller
change than `08:453-461`'s encoding axis, which makes the stored width a function of the encoding
rather than of the value count.

### 2.5 So what admitting intervals would actually cost

Not erasure, not the container derivation, and not the shape space. It costs the value-level total
order outright, one law outright, and one law weakened to a containment. Given that
`SETTLED.md:116` makes the order a precondition of the law layer, admitting intervals as numerals
means the law layer is quantified over two different orders depending on which type it is talking
about, and that is a larger change to the canon than anything measured above.

**Keeping the exclusion is the answer I reach too**, on grounds `08` did not use. `08` excludes
intervals because the construction sits above the numeral and needs nothing new. I would exclude
them because the **algebra** sits above the numeral and needs a different order, which is a
stronger reason and survives the case where someone finds a construction that does need something
new.

## 3. Whether the design already denotes sets

This is the question the dispatch says nobody has looked at, and it is where the file's contribution
is. The answer is yes in one place that matters, no in two places that look like it, and the one
that matters is load-bearing for soundness.

### 3.1 The saturating top does denote a set, and the design depends on it

`07:586-601` measures an n-step saturating fold under two readings and finds that saturating is
**exactly as unsound as wrapping** under the point reading, at 512 of 1024, and **perfect** under
the absorbing reading, at 0 of 65536 at eight trips. Its own sentence, at `07:606-610`:

> The design's own algorithm crates already rely on the absorbing reading, because the droplist
> records that the saturating presets "compute correctly" under those algorithms' stated
> specifications. But the reading is nowhere in the record as a statement.

I reproduced the saturating row's shape independently at a different width and over a different
operand alphabet, and only that row: I did not build the wrapping or substitute-zero arms, so
nothing here bears on `07`'s other two resolutions. Then I went past it, because reading
`07_probes/p5_postfixpoint_accumulator.py:126` rather than `07`'s summary showed that its inner
loop is `acc = R(acc + p, VA)`. **Additions only.** Soundness of an abstraction is quantified over
an operation set, so the natural next question is which operations the reading survives.

`18_probes/p2_absorbing_top_operation_set.py`, on `U<3,3>` unsigned, 64 values, saturating at both
ends, every operation exact in range so saturation is the only abstraction, chains enumerated
exhaustively from all 64 start values:

| operation set | chains | unsound, point | unsound, absorbing |
|---|---|---|---|
| add only, 4 steps | 1024 | 768 | **0** |
| add only, 6 steps | 4096 | 4032 | **0** |
| scale up by two, 4 steps | 64 | 60 | **0** |
| add and multiply by zero, 3 steps | 512 | 48 | **0** |
| add and multiply by one, 4 steps | 1024 | 512 | **0** |
| **add and subtract, 4 steps** | 5184 | 2464 | **936** |

The first witness came back as a **bottom** clamp rather than a top one, which showed my model was
half of the question, so `18_probes/p2b_absorbing_both_ends.py` reads both endpoints as absorbing
and classifies every failure by which clamp caused it:

| steps | chains | point | top absorbing | both absorbing | top then down | bottom then up | never clamped |
|---|---|---|---|---|---|---|---|
| 3 | 1728 | 656 | 240 | 200 | 128 | 72 | **0** |
| 4 | 5184 | 2464 | 936 | 840 | 568 | 272 | **0** |
| 5 | 15552 | 8759 | 3415 | 3191 | 2248 | 936 | **0** |

The zero in the last column is the sanity check that the arithmetic is exact in range: no failure
occurs without a clamp. And the classification says the rest:

> **An absorbing endpoint is sound exactly while the computation stays at it.** The instant an
> operation moves off the endpoint into the interior, the interior datum's point denotation is
> asserted, and it is false, because the absorbed information cannot be recovered.

Minimally, at the top of `U<3,3>`: the datum `63/8` under the absorbing reading stands for
`[63/8, inf)`. A saturating add of 2 leaves it at `63/8`, still sound. A subtract of 1 gives
`55/8`, whose denotation is the point `{55/8}`, while the truth is `[55/8, inf)`. The abstraction
has claimed a point where it holds an unbounded set.

**So the design does denote a set, and the reading it depends on is sound only for a restricted
operation set, and the restriction is nowhere written down.** That is a sharper version of `07`'s
finding rather than a contradiction of it, and it is the single thing in this file I would most
want checked by someone else.

Two repairs are available and they are very differently priced, so both go down rather than one.

**Restrict, and say so.** Write the restriction into whatever states the absorbing reading: the
reading holds while the operation set cannot decrease. That costs a clause and no mechanism, it is
checkable by the probe above, and it leaves every algorithm the design's own crates run inside the
restriction, since a fold of non-negative terms is exactly the shape `07`'s probe models.

**Carry the set in the type**, which is an interval numeral, and which is exactly the thing the
denotation clause excludes. That is general, it costs everything section 2 measures, and it is the
loop closing: the two halves of this dispatch turn out to be one question, and the excluded
construction is the general form of a mechanism the design already uses in a special case.

I would take the first and I am not proposing it, because tonight settles nothing and because
whether the restriction actually holds of every algorithm arvo ships is a claim about `mock/crates`
and `mock/crates` is being nuked. Someone should check it against whatever replaces them.

### 3.2 The rounding preimage denotes a set, and it is the benign kind

`07:317-330` measures the forced upper adjoint's gap on a quarter-step numeral against a `1/32`
tick: `{0}` for round toward positive infinity, `{0, 7/32}` for the directed modes, and
`{0, 3/32, 1/8}` for nearest. A directed mode's datum stands for a cell; a nearest mode's for a
half-cell either side.

I did not re-measure this, since one reproduction of someone else's arithmetic adds nothing. What I
did instead is ask what it costs, and section 2.1's answer is: **nothing to the order.** Cells
partition, so the separation order on them is total, at 120 of 120 pairs. Datum equality and
denotation equality agree, at 16 distinct denotations for 16 data.

That is the useful distinction the panel does not have. **A partitioning set denotation is free;
an overlapping one is not.** The design has two partitioning ones and excludes the overlapping
one, and until now that has looked like an accident of which cases came up.

### 3.3 Two things that look like set denotations and are not

The dispatch offers a refusing conversion and an accumulator with headroom. Neither survives
contact, and both dissolve into something else worth naming.

**Headroom is a refinement, not a denotation.** Every datum of a wide accumulator denotes exactly
one rational, before and after the widening. What headroom creates is inhabitants no computation
reaches. `18_probes/p5_headroom_is_a_refinement_not_a_denotation.py`, source `U<3,3>`, reachable
set computed by exact closure rather than sampled:

| accumulator bits | inhabitants | trips | reachable | unreachable |
|---|---|---|---|---|
| 10 | 1024 | 1 | 64 | 93.75% |
| 10 | 1024 | 8 | 505 | 50.68% |
| 12 | 4096 | 8 | 505 | 87.67% |
| 14 | 16384 | 8 | 505 | 96.92% |

And the reachable set is contiguous at every trip count, so it is exactly `{v : v <= n * max}`.
That is a predicate on a value, which is a **refinement**, and it lives in a layer the design has
not named. The denotation clause is untouched by it.

The distinction is worth keeping sharp because the two failures look identical on the page and have
opposite consequences. A denotation change attacks `08`'s first clause. A refinement does not touch
the boundary and is a separate gap, and the gap is real: at 14 bits and eight trips, 96.92% of the
accumulator's inhabitants are states the design's own overflow reasoning has to consider and the
computation can never enter.

**A refusing conversion is partiality, not set-valuedness.** It is a partial map on data. It
becomes set-valued under one reading, where a refusal denotes the empty set, and that reading is
consistent with the containment order because the empty set is below everything. Consistency is not
a reason to adopt it. The reading buys a bottom element and the record does not currently ask for
one, so I am recording it as available and declining to propose it.

### 3.4 The connection nobody has drawn: `Precise` on `inexact` is this question

`SETTLED.md:173` carries "`Precise` on `inexact`" as open since `145`, and `01` section 4 records
op offering three readings and acking the third, the arm reading, immediately followed by the
correction that the base is too loose to settle it.

Under section 3.2, a numeral datum denotes one rational **when it was constructed**, and denotes a
set the moment an inexact operation produced it. Which means:

> A strategy that refuses on inexact is the strategy that demands its data keep a point
> denotation.

That is not a new mechanism. It is a name for the one the record is already arguing about, arrived
at from the denotation clause rather than from the strategy axis, and it gives the open question a
statement instead of a preference.

`18_probes/p6_precise_is_the_denotation_clause.py` measures the size of the demand, over every
ordered pair of grid values, with out-of-range results counted separately per `07:295-307`:

| numeral | operation | exact | inexact | exact share of in-range |
|---|---|---|---|---|
| U<2,2> | add, sub | 136, 136 | 0, 0 | **100.00%** |
| U<2,2> | mul | 90 | 72 | 55.56% |
| U<2,2> | div | 74 | 142 | 34.26% |
| U<3,3> | mul | 554 | 1067 | 34.18% |
| U<3,3> | div | 490 | 3318 | 12.87% |
| U<4,4> | mul | 3170 | 12486 | 20.25% |
| U<4,4> | div | 2914 | 60446 | **4.60%** |

Addition and subtraction keep the point denotation on every in-range pair, because the grid is
closed under them. Multiplication and division do not, and the share that does shrinks with the
fraction width: at `U<4,4>` a point-denotation strategy admits 4.60% of in-range divisions.

**So a strategy demanding point denotation is not a marginal restriction.** It admits the additive
part outright and refuses most of the multiplicative part, and that shape is the same whichever of
op's three readings is eventually taken. I am not proposing a reading. I am saying the question has
a measurable statement it did not have, and that whoever takes it next can start from a number.

## 4. Is the clause in the right place

Three answers survive my derivation, and I am not choosing between them, per `04`.

### 4.1 The clause is in the right place and its wording is wrong

The reading I find best supported. Everything the clause excludes belongs elsewhere, as section 1
confirms independently of `08`, and the two set denotations the design already has are the
partitioning kind that the order and equality survive intact, as section 2.1 measures. So the
membership test is correct.

What is wrong is that the clause reads as a property of **every datum at every moment**, and the
design has data in flight that stand for sets. The clause is a statement about the **constructor**:
the numeral's grid is a set of points and each datum names one of them. What an operation's result
stands for is a different question, and `07:317-330` and section 3.1 both answer it with a set.

Under this reading the fix is one sentence and no mechanism, and it would say something like: the
denotable magnitudes are points, and what a datum in flight stands for is fixed by the operation
that produced it. That is intent rather than implementation, so it fits what `RULES.md:68-83`
permits a canon to say.

### 4.2 The clause is doing two jobs and should be split

The second reading, and the one I would want attacked. The clause currently carries a membership
test and an unstated commitment to which denotation the laws quantify over. Section 3.1 shows the
second commitment is not uniform across the design: the fold's soundness needs the absorbing
reading and the round-trip laws need the point reading, and both are live.

Split, it would be a membership clause saying the denotable magnitudes are rationals, plus a
separate statement somewhere else saying what a datum stands for, per resolution and per rounding
mode. The cost is that the boundary sentence stops being one sentence, which is a real loss because
its being one sentence is most of why it is usable.

### 4.3 The clause is load-bearing for less than it appears

The third reading, and the least comfortable. Section 1.4 finds that half the families ordinarily
grouped as set-denoting are excluded by const-sizedness rather than by this clause, and section 2.4
finds that intervals erase and derive containers without difficulty. So the clause's actual
exclusion set is two of `08`'s table rows: interval and triplex forms at `08:330`, and stochastic
streams at `08:320`. One of those has a real constituency and the other does not notice.

That does not make the clause wrong. It makes it a smaller clause than the sentence's symmetry
implies, and a reader who reads three clauses of equal weight will overestimate what the first one
holds up. Worth saying out loud in whatever the canon eventually carries.

## 5. Alternatives I did not take, described so the next angle starts from here

**A lattice-valued numeral, where every datum denotes an element of a lattice and points are the
atoms.** This unifies everything in section 3 including the empty set for refusals. I declined it
because it admits intervals by construction and section 2 measures that as costing the order and
one law, so it buys unification at exactly the price the design was avoiding. Worth someone else's
attack, because if the order loss can be confined to a marked subset of the lattice it might be
cheaper than I found.

**A denotation index on the type, so `Num<shape, Point>` and `Num<shape, Cell>` are different
types.** This makes section 3.1's restriction checkable at type-check time: a subtract on a `Cell`
datum would refuse. It is the refinement-typed answer and it is the one my own instincts reach for.
I did not build it because it is a mechanism proposal and tonight is a breadth pass, and because
the index would have to thread through every operation signature, which collides with the
ergonomics bar at `SETTLED.md:109` in a way I have not priced.

**A partition predicate as the boundary instead of a singleton predicate.** Section 2.1 measures
that partitioning denotations keep the order and equality and overlapping ones do not, so
"a datum denotes one cell of a partition of the rationals" would admit the design's two quiet cases
and still exclude intervals. This is the most economical reframing I found and I am naming it
rather than proposing it, because it widens the boundary and a widened boundary needs the whole
survey rerun against it, which is `08`'s instrument and not mine.

**Measuring the compile-time cost of a two-endpoint numeral against a scalar one.** Not taken. It
is the measurement I would take first if intervals were seriously on the table, and it is
unpriced.

## 6. Routes closed, each with what closed it

**"Intervals are excluded because they cannot erase."** Closed. `18_probes/p4_interval_erasure.rs`
folds the typed and raw forms at both 16-bit and 128-bit endpoints on the pin. The erasure gate is
met and it is not a reason.

**"The exclusion of interval arithmetic costs nothing."** Closed as stated, and true as `08`
scoped it to the construction. It costs the value-level total order (42.05% of pairs decidable at
`U<2,2>`, falling to 35.45% at `U<3,3>`), multiplicative associativity outright, and the additive
inverse for all but the degenerate data.

**"Interval multiplication is associative, so admitting intervals keeps the multiplicative
structure."** Closed. Outward rounding destroys it in both directions, with 1818 unsigned and 9524
signed operand triples where neither association contains the other.

**"An accumulator with headroom denotes an interval."** Closed. Every datum denotes a point; the
type has unreachable inhabitants, which is a refinement predicate `{v : v <= n * max}`, contiguous
at every trip count measured.

**"The absorbing reading of a saturating top is a sound reading of the design."** Closed as an
unqualified statement. It is sound over a monotone non-decreasing operation set and fails at 936 of
5184 four-step chains once subtraction is present, with zero failures occurring without a clamp.

**"Rounding is adjoint to the embedding."** Closed by `07:309-330` before I started, and the brief
carried the refuted version. Only round toward positive infinity is; the others are adjoint to a
cell-valued map, which is how a set enters.

**"Affine forms are excluded by the denotation clause."** Closed. `08:331` and `:492` put the
exclusion on arity, and removing the denotation clause would not admit them.

## 7. Coverage, stated honestly

I read `RULES.md`, `01`, `04`, `SETTLED.md` and `17` in full. I read `08` sections 1.3, 1.4, 2.1,
2.2, 3.1 through 3.5, 4.3 through 4.9, 5 and 6 in full, and grepped the rest. I read `07` sections
2.2, 2.3, 2.4, 4.2 and 4.3 in full and its heading list, and grepped the rest. I read
`07_probes/p5_postfixpoint_accumulator.py:104-141` in full, which changed what I built. I did not
read `02`, `03`, `05`, `06`, `09` through `16`, `CANON_CANDIDATE.md`, `MORNING.md`, `DROPLIST.md`
or `PERSONA_CALLS.md`. Every claim I make about a file I did not read in full is from a grep with
its line number, and is marked as such. I did not read `mock/crates`, and I did not read the closed
predecessor panel.

I did not verify `08`'s survey classifications against its probes. Section 1 rests on my own
reading of the constituencies plus `08`'s table, so where the table is wrong my section 1 inherits
it.

Every numeral in every probe is small: `U<1,1>` through `U<4,4>` and `I<1,2>`. Nothing here has
been checked above 8 bits of logical width, and the transfer argument `17:589-595` describes rests
on monomorphisation uniformity, which is a property of the shipped design rather than of my Python.
The interval order and law results are combinatorial and I expect them to hold at every width; the
exactness shares in section 3.4 plainly move with width and the three rows show the direction.

I ran no bench harness. **Every magnitude here is unpriced**, including section 2.4's calling
convention observation, which is the one that most looks like it wants a number.

I built six probes and one of them had a defect I found and corrected, recorded in section 0.2. All
six are under `18_probes/` with their outputs. The Rust probe carries zero anchored feature gates
and the check has to be anchored, because the file's own comments mention the attribute and an
unanchored grep counts them, which is stated in the probe's header.

`p1`, `p2`, `p2b`, `p3b`, `p3c`, `p5` and `p6` share one author and one model, so per
`RULES.md:116-118` they are **one instance of evidence wearing seven hats**, not seven. The only
place I have three genuinely independent instances is section 3.1, where `07`'s probe, `07`'s
droplist citation and my extension arrive from different directions, and even there two of the
three are `07`'s.

## 8. What is op's, and what is not

**Not op's.** Whether intervals are excluded. Section 2 measures the cost and section 1 names the
constituency, and `08` and I now agree on the exclusion having derived it from different grounds,
which under `RULES.md:28-40` is worth a second read rather than a rung, since I read `08` first.

**Not op's.** Whether the absorbing reading is sound over an operation set that can decrease. That
is measured, at 936 of 5184, and `01` section 3 is explicit that a measurement dispute is not
escalated.

**Op's, and it is one sentence.** What a datum stands for. The design has never said, and section
3.1 shows two parts of it depend on incompatible answers. `07:613` already names this as the
question it would most want asked; this file adds that the answer has to be qualified by the
operation set, and that the qualification is where the cost is.

**Op's, downstream of that.** Whether the boundary sentence stays one sentence, per section 4.2.
Splitting it buys precision and costs the thing that makes it usable, and that is a taste call
about a canon rather than a technical one.

**Op's, and reframed rather than answered.** `Precise` on `inexact`, per section 3.4. It is the
same question one level down, and it now has a measured shape: a point-denotation strategy admits
the additive part entirely and 4.60% of in-range divisions at `U<4,4>`.
