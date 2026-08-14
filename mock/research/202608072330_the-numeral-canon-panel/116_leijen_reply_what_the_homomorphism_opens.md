# 116. Reply: three concessions, and what the homomorphism opens

`112`'s author, resumed to answer `114`. This is the reply step `113` names: the file that attacked me
owes several replacements addressed to me, and I owe an answer with my own derivation still in context
rather than a fresh opinion.

Three claims are made against me. **I concede all three**, and I reproduced each on my own instrument
before conceding it, because a concession on someone else's numbers is not a concession. Two of them I
then re-predicate wider than `114` offers, which under I13 is a new claim of mine rather than an edit to
theirs.

The larger part of this file is not the concessions. `113` says the point of the exchange is whether any
of the offered material unlocks something the refuted party could not see before, and F114-1 does. The
homomorphism mechanism turns out to answer a question in the composite layer that `110` and I had both
measured without either of us seeing what it was, and it bounds `114`'s own newest arm in a way that file
could not have found, because the bound lives at a fraction width their sweeps do not reach.

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` with its "How to read an entry" section as normative, and against
`RULES.md`. Nothing in I1 through I18 forbids this work or is contradicted by it.

Two intents do load-bearing work below. **I13** (`INTENTS.md:214`) governs everything: section 5's result
is that two licence families are forced apart, which is the strongest possible statement that the work is
arms rather than a generalisation, and section 6 turns `114`'s one arm into three. **I15**
(`INTENTS.md:299`) is why every check discussed here is a const predicate and why an undischargeable
declaration is a build failure rather than a runtime one.

Every probe stays inside I14. No feature gates are used because no Rust is compiled in this file; the
work is model sweeps and one proof.

### 0.2 Test gate: passed, at 123 across 13, and it is the eleventh count

Every test-bearing bench variant crate, per crate, serially. Transcript at
`116_probes/p0_test_gate_run.txt` with the toolchain line.

```
grep -rnE '^[[:space:]]*#\[test\][[:space:]]*$' --include='*.rs' variants/ | grep -v '/target/' | wc -l
123
```

All 13 crates pass, zero failures, zero ignores. `wide-rung-shared` 30 in 4.64s,
`bitpack-write-contend-shared` 15 in 3.12s under `--test-threads=1`, and the rest under half a second
each. That agrees with the ninth and tenth counts and with mine in `112`.

I re-ran rather than cited because the gate is not waivable and a cited gate is not a run gate. I did not
re-audit the test bodies; `112` section 0.2 records what I read then and nothing in this file touches a
new crate.

---

## 1. What this reply concludes, stated first

> **F114-6 conceded, and it is the most useful correction I have received.** My sentence at `112:928` is
> false where the realisation map is a ring homomorphism. Reproduced on my own term enumeration at 76896
> and 12432 cells, zero unsound at `wrap` against 228 and 198 at `sat`, with the two checks disagreeing
> on 1908 and 366 cells so the comparison is live rather than vacuous.
>
> **F114-10 conceded, and re-predicated wider than `114` offers.** The domination is not a fact about
> one-sided declarations. It is a fact about declarations with a **non-negative lower bound**, which is
> strictly larger: under two-endpoint declarations with every lower bound at zero or above, the form is
> beaten on 0 cells across six terms, and under unrestricted two-endpoint declarations it is beaten on
> 1708. `114` names the negative constant as the mechanism and does not measure it separately; that
> measurement is the widening.
>
> **F114-12 conceded outright, and my F112-14 is superseded by it.** The formula reproduces all six cells
> of my measured table from the structure constants alone, with three distinct verdicts so the check is
> live. A table that grows with every construction is replaced by a sentence, which is what a canon wants.
>
> **What F114-1 opens, and this is the part worth the dispatch.** A wrapping map is a ring homomorphism
> and is not monotone; a saturating map is monotone and is not a ring homomorphism. **No map onto a finite
> value set is both**, and that is a proof rather than a sweep. So the root-check licence family, which
> needs the homomorphism, and the order-based licence family, which needs monotonicity, can never both be
> available at one overflow policy. `110` F12 and my own F112-12 were measuring the second family without
> either of us knowing there was a first one it traded against.
>
> **And a declared refinement is the only thing that escapes the trade.** On a discharged extent the map
> is the identity, which is both, so both families are available at once. That is something no choice of
> policy can buy, and it makes the refinement structural rather than incremental.
>
> **Arm W1 is bounded to `F = 0` or to a unit-grid declaration, which becomes three arms.** At `F > 0`
> under wrap the map is still a homomorphism for addition and subtraction and is **not** one for
> multiplication, 608 of 2116 at `F = 1` and 1234 at `F = 2`, so the root-only check goes unsound on 2079
> to 16063 cells the moment a product appears. `114` section 5.4 names `F > 0` as one of three things that
> would decide its file against itself and expects the fraction grid to be additive. It is not additive
> for multiplication, and the repair is a third arm rather than a retraction.

---

## 2. F114-6 conceded, reproduced first

### 2.1 Reproduced on an independent enumeration

`p1` builds the measurement from my own model, my own term enumeration and my own propagation, with
nothing imported from `114_probes`. My enumeration is larger than theirs because it assigns leaf names
independently of tree shape, so the cell counts do not match and are not meant to: two different
enumerations reaching the same verdict is stronger than two files agreeing on a number.

The mechanism first, because it is what the verdict rests on:

```
  uW3/wrap     R(R(a) op R(b)) != R(a op b) on      0 of 1452
  iW3/wrap                                          0 of 1452
  uW4/wrap                                          0 of 6348
  uW3/sat                                         189 of 1452
  iW3/sat                                         376 of 1452
  uW4/sat                                         885 of 6348
```

Then the licence, over every term at two and three leaf slots:

```
  primitive    cells   agree   root-lic  pernode-lic  ro-UNSOUND  pn-unsound  checks-differ
  uW3/wrap     76896   18672      14968        13060           0           0           1908
  iW3/wrap     12432    8066       7541         7175           0           0            366
  uW3/sat      76896   17127      14968        13060         228           0           1908
  iW3/sat      12432    7700       7541         7175         198           0            366
```

**The `checks-differ` column is reported before the verdict and is the condition-can-fire check.** At 1908
and 366 the two checks are genuinely different predicates on this sweep, so a zero in the unsound column
is a result rather than an artifact of the two rules coinciding. A halved-interval mutation is unsound on
148 of 432 cells at `uW3/wrap`, so the soundness counter is live at the policy where it reports zero.

And `114`'s "it is the ring, not the wrapping" reproduces: adding `min` to the signature makes the
identity fail on 164 of 484 and takes the root-only check to **7374** and **936** unsound cells at the two
wrapping bases.

### 2.2 So I concede, and the failure was mine at a step I can name

`114` section 3.2 is right that nothing was measured wrongly: F112-21 at `112:1116` carries
`overflow policy = sat` and is correct as stated. The sentence at `112:928` has no predicate at all.

**I would name the failure slightly more precisely than `114` does, because the fix differs.** `114` calls
it a predicated finding compressed into a sentence, and draws the lesson that a canon sentence should
carry its source finding's predicate. That is right and it is not quite what happened. My sentence was not
a compression of F112-21; it was a **generalisation over F112-21 and F112-22**, and generalising is where
the predicate went. Checking against the source finding would have caught it, and so would I13's own
requirement read against canon prose rather than only against findings: a universal claim is written out
as a predicate over every dimension that could move it, and my sentence quantified over none.

So the lesson I would carry is the stronger one: **an offered canon sentence is a finding and carries a
predicate, or it names the dimensions it is universal over.** Mine did neither.

### 2.3 The corrected claim, as a new claim rather than an edit

Per I13 a predicate is never widened in place, and `112` stands as written. This is the new claim:

> **A root-only discharge check is unsound where the realisation map is not a ring homomorphism.**
>
> `holds for: W in {3, 4}, F = 0, signedness in {unsigned, signed}, overflow policy = sat, rounding =
> trunc, radix = 2, operations in {add, sub, mul}, term shapes = every term at 2 and 3 leaf slots,
> declarations = one-sided, threads = 1, target features any`

and its companion, which is `114`'s and which I reproduce rather than claim:

> **Where the map is a ring homomorphism and every operation is a ring operation, the root's interval is
> the whole condition.** Same predicate with `overflow policy = wrap`.

**Note what the corrected predicate does not say.** It does not list `wrap`, so under I13 it makes no
claim there, which is exactly right: at `wrap` the finding is false. And it lists `F = 0`, which section 6
turns out to need.

---

## 3. F114-10 conceded, reproduced first, and re-predicated wider

### 3.1 The witness recomputed, then the sweep

`p2` recomputes `114` section 5.3's worked example rather than quoting it. The object under test is my own
form, so it is imported from my committed `112_probes/p9` rather than reimplemented, which is what `114`
did and is the honest choice: a reimplementation would test my reading of my own rule. What is independent
is the declaration enumeration.

```
  leaves both declared [-4, -1], term x * y, container [-4, 3]
    true reachable range   : [1, 16]
    corner rule            : [1, 16]
    my one-sided form      : [-8, 25]
    matches `114`'s worked [-8, 25] against [1, 16]: True
```

Over six terms at `iW3/sat` and `uW3/sat`:

```
  beaten under one-sided declarations : 0
  beaten under two-endpoint            : 1708
```

**Conceded.** My F112-24's "beaten on none" is false under two-endpoint declarations, and my own F112-23
already recorded that one-sided extents were all I swept. The two findings sat fourteen lines apart in my
own list and I did not point one at the other, which is the defect `114` names and it is fair.

### 3.2 And the region is larger than `114` gives it, which is a new claim

`114` states the mechanism as a negative constant flipping the sign of the coefficients it scales, and
does not measure the discriminator that mechanism implies. `p2`'s P5 does:

```
  iW3/sat  x * y        TWO-ENDPOINT, lo >= 0     BEATEN 0
  iW3/sat  x * x        TWO-ENDPOINT, lo >= 0     BEATEN 0
  uW3/sat  x * y        TWO-ENDPOINT, lo >= 0     BEATEN 0
  iW3/sat  (x + y) * z  TWO-ENDPOINT, lo >= 0     BEATEN 0
  iW3/sat  (x + y) - y  TWO-ENDPOINT, lo >= 0     BEATEN 0

    beaten with two endpoints but no negative lower bound: 0
```

**So the second endpoint costs nothing. The negative lower bound is the whole mechanism**, which is what
`114` says and is a strictly smaller region than the one it predicates on. The domination holds over every
declaration whose lower bound is at or above zero, which properly contains the one-sided declarations `112`
swept.

**Two further facts the sweep gives, both narrowing further.** The beaten cells are entirely on
multiplication terms: `x + y` and `(x + y) - y` are beaten on 0 under unrestricted two-endpoint
declarations, because the sign flip needs a product to exist at all. And on `(x + y) - y` under
two-endpoint declarations my form **wins**, licensing 550 against the interval rule's 330.

So the honest shape is not domination and not defeat. It is two arms, which is what I13 says it should be.

### 3.3 The corrected claim, as a new claim

> **A form over one-signed symbols with a corner cross-term bound is not beaten by the interval rule where
> every declared lower bound is at or above zero, and is beaten by it on multiplication terms where a
> declared lower bound is negative.**
>
> `holds for: W in {2, 3, 4}, F = 0, signedness in {unsigned, signed}, overflow policy in {sat, wrap},
> rounding = trunc, radix = 2, operations in {add, sub, mul}, term shapes = the six enumerated in p2 plus
> the thirteen in 112's p9, arity in {2, 3}, declarations = two-endpoint exhaustive at arity 2 and 3,
> declared lower bound >= 0 for the first half and unrestricted for the second, threads = 1, target
> features any`

**And I would take `114`'s arm S2 rather than defend my form**, for a reason that is mine rather than
theirs. Analysing why my form loses leads to exactly their repair: the product of two forms is a bilinear
expression whose interval I compute by summing coefficient contributions independently, which decouples
the cross term from the symbols it came from. Recoupling it at the node is the per-node intersection. I
arrived at S2 from the mechanism rather than from their file, which makes it a second instance of the
repair rather than a concession to it.

---

## 4. F114-12 and F114-13 conceded, and my table is superseded

`p5` derives the three constructions' structure constants and applies `114`'s formula, then compares
against what my own committed `112_probes/p5b` **measured**, not against `114`'s account of my table.

```
  construction   signed?   L1 norms   neg row   verdict
  product2       unsigned  [1, 1]     False     1 * m^2  -> componentwise
  product2       signed    [1, 1]     False     1 * m^2  -> componentwise
  dual           unsigned  [1, 2]     False     2 * m^2  -> 2x componentwise
  dual           signed    [1, 2]     False     2 * m^2  -> 2x componentwise
  complex        unsigned  [2, 2]     True      NONE fires soundly
  complex        signed    [2, 2]     True      2 * m^2  -> 2x componentwise

  distinct verdicts the formula produces: 3   (live)
  6 agree, 0 disagree
```

**Six of six, from the constants alone, without measuring anything.** My F112-14 reported a table of three
constructions and called the per-construction rule a joint fact with the base's signedness. That is true
and it is now a consequence rather than a finding: the L1 norm gives the multiple and the negative-entry
bit interacts with the signedness. A table is a design obligation that grows with every construction
anyone adds; a formula is a sentence. **Take the formula.**

`114`'s own bound on it is right and I have nothing to add: it covers **bilinear** constructions, and
`interval` is a hull rather than a bilinear form, so `110` F12's monotonicity predicate remains its own
obligation. Section 5 is why those two obligations are not arbitrary.

---

## 5. What F114-1 opens: the two licence families are forced apart

This is the part `113` asks for. F114-1 is a statement about one algebraic property of the realisation
map. There is a second property in the panel already, measured by `110` and by me for a different purpose,
and putting the two in one sentence changes what the overflow policy **is**.

### 5.1 Two properties, two licence families, and they do not overlap

`110` F12 (`110:523`) establishes that the interval construction is closed exactly on **monotone** bases,
and my F112-12 that a declared extent discharges that predicate over a wrapping base. Neither of us asked
what monotonicity had to do with anything else. `114` establishes that wrapping is a **ring homomorphism**
and builds arms W0 and W1 on it. Nobody asked what that had to do with monotonicity.

`p3` measures both properties on the same maps:

```
  primitive        ring hom   monotone     interval ill-ordered
  uW3/wrap             True      False           546 / 1296
  iW3/wrap             True      False           546 / 1296
  uW3/sat             False       True             0 / 1296
  iW3/sat             False       True             0 / 1296
  uW3/zero            False      False           728 / 1296
  uW3/reflect         False      False           568 / 1296

  policies with both properties: none
```

The two extra policies are there so the table is a measurement rather than a restatement of two points: a
flush-to-zero map and a reflecting map both have neither property, so "no policy has both" is not simply
"there are only two policies".

### 5.2 And the emptiness is forced, which is a proof rather than a sweep

> **No realisation map onto a finite value set is both a ring homomorphism and monotone, except a constant
> one.**
>
> Let `V` be finite with `|V| >= 2` and `R : Z -> V` surjective with `R(a op b) = R(R(a) op R(b))` for
> `op` in `{+, *}`. Then `V` carries an induced ring structure and `R` is a surjective ring homomorphism,
> so `V` is `Z/nZ` with `n = |V| >= 2`. Suppose `R` is also monotone for some total order on `V`. Then
> `R(0) = R(n)`, because `0` and `n` are congruent, and a non-decreasing map agreeing at `0` and `n` is
> constant on `[0, n]`. That interval contains a complete residue system, so periodicity makes `R`
> constant everywhere, contradicting `|V| >= 2`.
>
> The shorter form: a finite ring is not an ordered ring, because `1 > 0` forces characteristic zero.

`p3`'s P2 is the control on that argument, exhaustive over every map from a window of `Z` onto a small
value set:

```
  |V| = 2, window 9 points,     512 maps: homomorphic 3, monotone 10,  both 2, both AND non-constant 0
  |V| = 3, window 13 points, 1594323 maps: homomorphic 6, monotone 105, both 3, both AND non-constant 0
```

Each half has candidates, so "none passes both non-trivially" is a statement about the maps rather than
about an empty search. The maps passing both are the constants, which the proof predicts.

**This is the only result in this file I would carry to a real width without further work**, because it
quantifies over the value set's finiteness rather than over its size. Everything enumerative here is at
`W <= 4` and I have no transfer argument.

### 5.3 What it means, and it changes what an overflow policy is

The root-check family needs the homomorphism. The order-based family, which is `110` F12's interval
closure and anything else resting on the map preserving order, needs monotonicity. **A design cannot have
both at one policy, ever, and not because nobody has found the right map.**

So the overflow policy is not a preference among behaviours that differ only in what they compute at the
boundary. **It selects which family of algebraic licences the whole design can use.** That is a much
stronger claim about the axis than anything the panel has carried, and it is at the primitive level, which
is this topic's subject.

**It gives `115`'s conclusion its mechanism.** `115` section 1 says to aim the predicate at the check the
overflow policy selects. Section 5.2 is why the policy selects one: the policy fixes which algebraic
property the map has, the property fixes which check is sound, and no policy has both properties. The two
files were written in parallel and neither read the other; I read `115` after `p3` ran.

**And it makes one line of `114`'s arm W1 incomplete rather than wrong.** Arm W1 says checking every node
under a homomorphism "is conservative rather than load-bearing: it forgoes 7% to 30% of the available
licences and buys nothing". Within the policy that is exactly right and I reproduce it. Across the design
it is not free: taking arm W1 means being at `wrap`, and being at `wrap` costs the interval construction
outright, 546 of 1296 well-ordered pairs coming back ill-ordered. The cheap check is free **given** the
policy, and the policy is not free.

### 5.4 And this is what makes a refinement structural rather than incremental

On a discharged extent the map is the identity. The identity is a ring homomorphism and it is monotone.
`p3`'s P5:

```
  uW3/wrap  extent <= 3: discharged True   R is the identity on it: True   order preserved: True
  uW3/sat   extent <= 3: discharged True   R is the identity on it: True   order preserved: True
```

**So a declared refinement is the only mechanism in the design that escapes the trade.** A policy choice
buys one family and forfeits the other, permanently and provably. A discharged declaration buys both at
once, on its extent.

That is a much better argument for the refinement than anything in `112`. My section 3.4 placed it as the
carrier for a per-chain licence, which is true and is a statement about convenience. This is a statement
about capability: there is a licence pair no policy can hold, and the refinement holds it.

---

## 6. Arm W1 bounded, which makes it three arms

`114` section 5.4 names `F > 0` as one of three things that would decide its file against itself, and
expects the fraction grid to be additive, marking that as an expectation and not a result. It is not
additive. `p4b` measures it.

### 6.1 The homomorphism splits along the operation, not the policy

```
  primitive          add          sub          mul
  uW4F0/wrap      0/2116       0/2116       0/2116
  uW4F1/wrap      0/2116       0/2116     608/2116
  uW4F2/wrap      0/2116       0/2116    1234/2116
  iW4F2/wrap      0/2116       0/2116    1451/2116
```

The `F = 0` row is the control and it reproduces F114-1. At `F > 0` the map is still a homomorphism for
addition and subtraction and is not one for multiplication. Witness, printed by the probe:

```
    a = 1/4, b = 4: R(R(a)*R(b)) = 0, R(a*b) = 1
```

**The mechanism is `112` F112-4 read at this question.** The map has two regions. The completion region is
modular and stays a homomorphism at any `F`. The rounding region is entered only by multiplication,
because addition and subtraction are exact on the grid and a product lands on the finer grid `s^2` and has
to be requantised. Requantisation does not commute with reduction. So the homomorphism question is asked
of each region separately, which is why one operation keeps it and another loses it.

### 6.2 What that does to the licence

```
  primitive     no-mul terms                      with-mul terms
  uW4F0/wrap    0 unsound,  959 checks-differ      0 unsound,  118 checks-differ
  uW4F1/wrap    0 unsound,  959 checks-differ   2079 unsound,  190 checks-differ
  uW4F2/wrap    0 unsound,  959 checks-differ   6330 unsound,  387 checks-differ
  iW4F2/wrap    0 unsound,  565 checks-differ  16063 unsound, 1099 checks-differ
```

Both halves are live: 959 and 565 checks-differ on the no-mul rows, 118 to 1099 on the with-mul rows.

**The first run of this sweep was vacuous on exactly the rows that carry the good news**, and the run is
committed with the defect named in its own output. With one-sided declarations the root of an add/sub
chain over a one-signed domain is the widest node, so the two checks coincide by construction and the
zeros meant nothing. That is my own F112-23 biting a probe of mine for the third time. Two-endpoint
declarations make it live.

### 6.3 The third arm, and my own predicted mechanism for it was wrong

The grid part of a declaration restores the multiplicative homomorphism, which is `112` F112-4's grid part
doing at `F > 0` what the magnitude part does at `F = 0`. But not for the reason I predicted.

```
  operands on a grid of step 1/4: hom fails on 1234/2116, products already on the fine grid: False
  operands on a grid of step 1/2: hom fails on  144/529,  products already on the fine grid: True
  operands on a grid of step 1:   hom fails on    0/144,  products already on the fine grid: True
```

I predicted the homomorphism would return once products needed no requantisation. **The step-1/2 row
refutes that**: every product is already on the fine grid and the homomorphism still fails on 144 of 529.

The correct condition is visible in raw units. At fraction width `F` a value `v` has raw index `v * 2^F`
and a product's raw index is `raw_a * raw_b / 2^F`. Replacing `raw_a` by `raw_a + k n` adds
`k n raw_b / 2^F`, which is a multiple of `n` exactly when `raw_b` is a multiple of `2^F`, that is when
`b` is a whole multiple of the **unit**. Not when the product avoids requantisation.

### 6.4 So arm W1 is three arms

> **W1a.** `F = 0`, operations in the ring operations. `114`'s, reproduced.
>
> **W1b.** `F` any, operations in `{add, sub}`. The homomorphism survives any fraction width for the
> operations that never enter the rounding region.
>
> **W1c.** `F` any, operations in the ring operations, **operands declared on the unit grid**. The grid
> part of the declaration restores what the fraction width took.

`holds for: W = 4, F in {0, 1, 2}, signedness in {unsigned, signed}, overflow policy = wrap, rounding =
trunc, radix = 2, operations in {add, sub} for W1b and {add, sub, mul} for W1a and W1c, term shapes = every
term at 2 and 3 leaf slots, arity in {2, 3}, declarations = two-endpoint exhaustive at arity 2 and 4000
sampled at arity 3, threads = 1, target features any`

Three arms where `114` has one, and the third is a **declaration** discharging an algebraic obligation,
which is the same shape as everything else this topic has produced.

---

## 7. Where I hold, and what would decide it

I concede all three claims and hold nothing against them. Two places where I would push, both additions
rather than contests, and both stated so a later expert can attack them.

**Arm W1's cost accounting is within-policy and the policy is not free.** Section 5.3. What would decide
it: whether any real consumer wants both the root-check licence and an order-based construction on the
same values. That is I11's territory and nothing in the repository measures it, which is the same wall
`108` hit on a different question.

**The duality's consequence for the strategy axis is mine and is one expert's.** If the overflow policy
selects a licence family rather than a boundary behaviour, then it is doing more work in the design than
the panel has credited it with, and the strategy pair's first component carries more than an assignment.
I have not chased that and it belongs to whoever picks up the strategy topic next. What would decide it:
whether any other axis in the declared semantics has the same character, selecting a family of licences
rather than a behaviour. I would look at the rounding mode first, because section 6.1 shows it owns a
region of the map with its own algebraic character.

---

## 8. What I would build, with its predicate and its bound

`113` asks what the exchange unlocks. Three things, in the order I would build them.

**One. The unit-grid declaration as a first-class part of a refinement.** `112` F112-4 established that a
declaration has a magnitude part and a grid part and that each switches off one region of the map. Section
6.3 gives the grid part a second job that nobody predicted: it restores the multiplicative homomorphism at
`F > 0`, which is what makes arm W1 available at a fraction width at all. A design carrying only a
magnitude bound cannot express W1c. **Bound:** measured at `W = 4`, `F in {1, 2}`, wrap, and the raw-unit
argument in section 6.3 is width-independent while the sweep is not.

**Two. A policy-pair primitive, because the trade is forced.** Section 5.2 says no policy has both licence
families. A consumer wanting both on the same values has exactly two options: carry the values under two
policies, or declare a refinement that discharges. The second is strictly better where it applies and the
first is the fallback. What I would build is the const predicate that decides which, and it reads only the
declaration and the term's operation set, both of which are already const. **Bound:** I have not built it
and I do not know what it costs in monomorphisations, which is the question `114` section 7 and `115`
section 4 are both circling.

**Three. The order-based licence family enumerated, because only one member of it is known.** `110` F12
found `interval`. Monotonicity licenses more than that: any construction or rewrite resting on the map
preserving order, which includes comparison-carrying composites, min and max folds, and clamping
reassociation. Nobody has enumerated them, and section 5.2 says the whole family lives or dies with the
policy together. **Bound:** this is a survey rather than a measurement and I would expect it to take one
dispatch and produce a list rather than a number.

**And one thing I would not build.** The quadratic-aware product I was reaching for when `114` landed,
which would bound a product of two forms sharing symbols by optimising the quadratic rather than
linearising it. Arm S2 recovers all 92 and all 593 of the cells that motivated it, at a cost a design that
disjoins is already paying. The quadratic form would beat S2 only on terms where a leaf is squared and the
intersection is still loose, and I did not measure how many of those there are. **Recorded as not taken so
the next expert does not rediscover it as an obvious gap.**

---

## 9. Findings, each with its predicate

Conventions stated once. Everything enumerative ran on one thread and carries `threads = 1`. The model
sweeps are exact rational arithmetic no instruction selection can move, so they carry `target features
any` with that as the argument. Every enumerative finding is at `W <= 4` with **no transfer argument** to
any real width, so every predicate lists its width as a fixed set. The one exception is F116-4, which is
structural and says so.

**F116-1. `112:928` is false where the map is a ring homomorphism, reproduced on an independent
enumeration.** Zero root-only unsound at `wrap` over 76896 and 12432 cells, against 228 and 198 at `sat`,
with 1908 and 366 cells where the two checks disagree. `W = 3, F = 0, signedness in {unsigned, signed},
overflow policy in {sat, wrap}, rounding = trunc, radix = 2, operations in {add, sub, mul}, term shapes =
every term at 2 and 3 leaf slots with every leaf-name assignment, declarations = one-sided, threads = 1,
target features any`. `p1_output.txt`. Control: a halved-interval rule is unsound on 148 of 432 at
`uW3/wrap`.

**F116-2. The root-only check's soundness under wrap is a property of the ring, not of the wrapping.**
Adding `min` makes the homomorphism identity fail on 164 of 484 and the root-only check unsound on 7374
and 936 cells. Same predicate as F116-1 with `operations in {add, sub, mul, min}`. `p1_output.txt`.

**F116-3. My F112-24's domination holds over every declaration with a non-negative lower bound, which is
strictly wider than the one-sided declarations it was measured on, and fails only where a declared lower
bound is negative and a multiplication is present.** Beaten on 0 cells under two-endpoint declarations with
every lower bound at or above zero across six terms; on 1708 under unrestricted two-endpoint declarations,
all on multiplication terms; and on `(x + y) - y` the form wins 550 to 330. `W = 3, F = 0, signedness in
{unsigned, signed}, overflow policy = sat, rounding = trunc, radix = 2, operations in {add, sub, mul},
term shapes = the six enumerated in p2, arity in {2, 3}, declarations = two-endpoint exhaustive, threads =
1, target features any`. `p2_output.txt`. The hand witness from `114` section 5.3 recomputes exactly.

**F116-4. No realisation map onto a finite value set is both a ring homomorphism and monotone, except a
constant one.** Structural, with the argument in section 5.2, and controlled by exhaustive search: over
512 and 1594323 maps, 3 and 6 are homomorphic, 10 and 105 are monotone, and 0 are both and non-constant.
`value set size in {2, 3}, window = 9 and 13 consecutive integers, operations in {add, mul}, threads = 1,
target features any`. `p3_output.txt`. **The claim itself quantifies over finiteness rather than over
size**, so unlike everything else here it does not need a width transfer argument; the search is a control
on the argument rather than the evidence for it.

**F116-5. The two licence families are disjoint per policy, measured.** Wrapping is a homomorphism and not
monotone, with the interval construction ill-ordered on 546 of 1296 pairs; saturating is monotone and not a
homomorphism, with the interval construction closed on 0 of 1296; and two further policies have neither
property. `W = 3, F = 0, signedness in {unsigned, signed}, overflow policy in {sat, wrap, flush-to-zero,
reflect}, rounding = trunc, radix = 2, operations in {add, sub, mul}, construction = interval, threads = 1,
target features any`. `p3_output.txt`.

**F116-6. A discharged declared extent restores both properties at once.** On every discharged extent
swept the map is the identity and preserves order at both policies. `W = 3, F = 0, signedness = unsigned,
overflow policy in {sat, wrap}, rounding = trunc, radix = 2, operation = add, arity = 2, extents = upper
bounds in {1, 3, 7}, threads = 1, target features any`. `p3_output.txt`.

**F116-7. At `F > 0` the map is a ring homomorphism for addition and subtraction and is not one for
multiplication.** Zero of 2116 for add and sub at every fraction width swept; 608, 1234 and 1451 of 2116
for mul at `F in {1, 2}`. `W = 4, F in {0, 1, 2}, signedness in {unsigned, signed}, overflow policy =
wrap, rounding = trunc, radix = 2, operations in {add, sub, mul}, threads = 1, target features any`.
`p4b_output.txt`. Control: the `F = 0` rows are zero on all three operations.

**F116-8. So the root-only check at `F > 0` under wrap is sound on terms without a multiplication and
unsound on terms with one.** Zero unsound over 52992 cells on no-mul terms at every fraction width, with
959 and 565 cells where the checks differ; 2079, 6330 and 16063 unsound on with-mul terms at `F > 0`
against 0 at `F = 0`. Same predicate as F116-7 with `term shapes = every term at 2 and 3 leaf slots,
declarations = two-endpoint exhaustive at arity 2 and 4000 sampled at arity 3`. `p4b_output.txt`.

**F116-9. The multiplicative homomorphism at `F > 0` is restored exactly when the operands are declared on
the unit grid, and not when their products merely avoid requantisation.** Fails on 1234 of 2116 at grid
step `1/4`, 144 of 529 at step `1/2` where every product is already on the fine grid, and 0 of 144 at step
`1`. `W = 4, F = 2, signedness = unsigned, overflow policy = wrap, rounding = trunc, radix = 2, operation
= mul, ambient span = three times the container, threads = 1, target features any`. `p4b_output.txt`. This
**refutes a prediction of mine** recorded in that probe's header before it ran.

**F116-10. `114` F114-12's formula reproduces my own measured F112-14 table from the structure constants
alone.** Six of six cells agree, with three distinct verdicts so the check is live, and the quaternion real
part comes out at L1 norm 4 with a negative entry without the derivation mentioning the dimension.
`dimension in {2, 4}, structure constants in {-1, 0, 1}, signedness in {unsigned, signed}, constructions
in {product2, dual, complex, quaternion}, operation = mul, arity = 2`. `p5_output.txt`. My table is
superseded by the formula.

**F116-11. The suite is 123 across 13 and all of it passes.** `toolchain = nightly-2026-05-28, host = this
machine, --release, --test-threads=1, threads = 1`. `p0_test_gate_run.txt`. Eleventh independent count.

**Unpriced.** Everything about cost. No bench harness ran, nothing here was timed, and no claim in this
file depends on a magnitude. Section 8's second build names monomorphisation cost as the open question and
it stays unpriced.

---

## 10. Coverage, bounded

**Read in full:** `113`, `114` sections 1 to 6.3 and its findings list, `115` sections 1 and the headings
of the rest, `INTENTS.md`, and my own `112`.

**Read in part:** `114` sections 6.4 to 7 by heading only, `110` F12 and its surrounding section, `108`
section 7 as cited in `112`.

**Not read:** `114_probes` source, `115_probes` source, `OPTIONS.md`, `AGREEMENTS.md`, `DROPLIST.md`,
`RULES.md` beyond what `112` records, the archive, and every panel file before `108` except through
`112`'s own account of them.

**Not reproduced:** every number I quote from `114` that I did not measure, which is most of its file. What
I reproduced is exactly the three claims made against me, and each is named at the point it appears. In
particular I did **not** re-derive `114`'s arms W0, S1 or S2, nor its section 7 compile-time results, and
where I recommend arm S2 in section 3.3 I am recommending it on my own mechanism rather than on their
measurement.

**Citations checked by opening them.** `p6_check_my_own_citations.py` opens every `file:line` this file
leans on and tests the substring the claim depends on. **23 checked, 0 failing**, after one failed on the
first run because the phrase was wrapped across two lines. Mutation-tested: a wrong line number, a wrong
substring and a wrong file each add exactly one failure. **`115` is live while this is written**, so it is
cited by section rather than by line.

**Three defects of my own, all committed with the defect written into their own output rather than
overwritten.** `p3`'s root-only column is undersized and reports a false negative at `uW3/sat` where `p1`
reports 228, which is the hand-picked-rows failure inside a probe whose subject is a general theorem.
`p4`'s no-mul rows were vacuous because one-sided declarations make the two checks coincide. `p4`'s grid
section drew its operands from inside the container, so the homomorphism identity was trivially satisfied
and it measured nothing. Repairs are `p4b` and, for the first, `p1`'s larger sweep.

**The largest thing I did not do.** I did not attack `114`'s arm S1 or its exactness proof, which is the
part of that file with the most riding on it, and I did not touch `109` section 8's chain result, now
untouched by three consecutive members.

---

## 11. Probe index

All under `116_probes/`, each committed with its output as it ran, before this file was written.

- `p0_test_gate_run.txt`. Every test-bearing variant crate, per crate, serially. 123 pass.
- `p1_reproducing_the_root_only_claim_against_my_own_sentence.py`, `p1_output.txt`. F114-6 reproduced on
  my own term enumeration, with the homomorphism mechanism, the condition-can-fire check reported before
  the verdict, the `min` control, and a halved-interval mutation.
- `p2_reproducing_the_two_endpoint_beating_of_my_own_form.py`, `p2_output.txt`. F114-10 reproduced, the
  hand witness recomputed, and the non-negative-lower-bound discriminator `114` names but does not measure.
- `p3_the_two_licence_families_are_forced_apart.py`, `p3_output.txt`. The duality, the exhaustive search
  controlling the proof, two extra policies so the table is not two points, and **one undersized column
  named in its own output**.
- `p4_arm_w1_at_a_nonzero_fraction_width.py`, `p4_output.txt`. **Two vacuous sections**, kept, with the
  reason written into the output.
- `p4b_arm_w1_at_a_nonzero_fraction_width_repaired.py`, `p4b_output.txt`. The repair, and the refutation
  of my own predicted mechanism for the grid condition.
- `p5_the_formula_reproduces_my_own_table.py`, `p5_output.txt`. F114-12's formula against what my own
  committed `112_probes/p5b` measured, with the verdict set checked live first.
- `p6_check_my_own_citations.py`, `p6_output.txt`. Twenty-three citations opened and their content tested,
  one wrong on the first run, with three mutations confirming the instrument fails when it should.
