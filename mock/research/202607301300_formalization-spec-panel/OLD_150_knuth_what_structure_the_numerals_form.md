# What structure the numerals form, and why three careful readers got three answers

**Date:** 2026-08-07
**Position:** after `149_checkpoint_where_the_three_reads_stand.md`. Settles the structural question that
`145`, `146` and `148` answer three incompatible ways.
**Probes:** `150_probes/`, five files, four independent instruments, cross-validated against each other.
**Toolchain claim in the brief, checked:** `rust-toolchain.toml` pins `nightly-2026-05-28` and that channel
reports `rustc 1.98.0-nightly (57d06900f 2026-05-27)`. Both as stated. No Rust was needed for this question,
which is arithmetic rather than trait solving.

## Verdict, stated before the argument

The disagreement is not about the order. It is about which shapes the design admits, and all three files
answer a closure question they did not know they were answering. Once the closure question is separated out,
the three stop contradicting each other and each turns out to describe a different slice.

The structure is **not one answer but a boundary**, and the boundary has a single ingredient on either side
of it. Holding the bias at zero throughout, so that no phase failure is possible:

| family | binary meets | binary joins |
|---|---|---|
| one radix, shape space closed downward | total, and equal to the intersection | total, strictly overshooting |
| one radix, no zero-width numeral | **fail** | total |
| one radix, with a bias | **fail** | total |
| two or more radices | **fail** | **fail** |
| a fixed-point family beside a float family | **fail** | **fail** |

So: **within a single radix and with the shape space closed, the order is a distributive lattice.** Add a
second radix, or set a fixed-point family beside a float family, and it is **neither a meet-semilattice nor a
join-semilattice**. It is a poset and nothing more.

**The single ingredient is whether refinement and reach move together.** Inside one radix, adding a digit
multiplies the count and refines the grid at the same time, so a finer cover always contains the coarser one
and the least cover is forced. Across radices, and between fixed point and float, that monotonicity has no
reason to hold, and a cover on a finer grid can be strictly shorter than the coarser one it was meant to
refine. Then neither contains the other, and there is no least anything.

**Which of the three this lands on: none of them, and each of them is right about its own slice.** `145` is
right where it looked. `146` is right about the meet and its join claim fails on one unchecked clause at
`146:226`. `148` is right that the join fails and right about the shape of the mechanism, and its meet
verdict is stated over the family while its evidence is a slice. **`146` and `148` are not exact inverses**,
contra `149:50`; they agree about the meet, and `148` supplies the reconciliation for the join itself at
`148:396`.

**What is genuinely undetermined, and is op's:** whether arvo's numerals are one family or several. That is
not a mathematical question and nothing above decides it.

## 1. The question, stated so that it has an answer

Let $N$ be the numerals the design admits. Each denotes a finite set of exact values
$V(n) \subset \mathbb{Q}$. Define

$$a \sqsubseteq b \iff V(a) \subseteq V(b),$$

a preorder, and a poset $(\hat N, \sqsubseteq)$ after quotienting by $V(a) = V(b)$.

**There are two lattices in the neighbourhood and the three files conflate them.**

The ambient order on *all* finite subsets of $\mathbb{Q}$ is a complete lattice with meet $\cap$ and join
$\cup$. That is free and says nothing about numerals. The order on $\hat N$ is that order **restricted to a
subset**, and restriction does not preserve lattice structure. In $\hat N$,

$$a \sqcap b = \max\{\, c : V(c) \subseteq V(a) \cap V(b) \,\}, \qquad
  a \sqcup b = \min\{\, c : V(c) \supseteq V(a) \cup V(b) \,\},$$

and each exists only if that extremum exists. The intersection and the union are always defined **as sets**.
The question is whether the shape space contains a best approximation to them from the correct side.

**So the structural question is a closure question about the shape space, not a question about the order.**

That reframing is what dissolves the disagreement, and it was reached before reading any of the three. It
also predicts, correctly, where each of them went wrong: each computed an ambient extremum, observed that it
was well defined, and assumed the design admits it.

## 2. The derivation

### 2.1 The order is a product of three chains

Take a numeral whose value set is a run of equally spaced points. Such a set is determined by

$$q = \text{the quantum (the step)}, \qquad L \le G, \qquad L, G \in q\mathbb{Z} + b,$$

with $V = \{L, L+q, \dots, G\}$ and $b$ the phase. Containment requires the second grid to be at least as
fine, in phase, and to reach at least as far on both sides:

$$V_1 \subseteq V_2 \iff
  \underbrace{q_2 \mid q_1}_{\text{grid}} \ \wedge\
  \underbrace{b_1 \equiv b_2 \!\!\pmod{q_2}}_{\text{phase}} \ \wedge\
  \underbrace{L_2 \le L_1}_{\text{floor}} \ \wedge\
  \underbrace{G_1 \le G_2}_{\text{ceiling}} .$$

That is the four-condition order `146` and `148` derived independently, and `148`'s two-coordinate form (the
affine lattice and the interval, each ordered by inclusion) is the same statement folded. **This file adopts
it and adds nothing to it**, which is the settled part of `149` and it survives everything below.

With the phase held fixed, the remaining three conditions are a componentwise order on the triple
$(q^{-1}, -L, G)$. **A componentwise order on a product of chains is a distributive lattice**, with

$$\wedge = (\min q^{-1},\ \max L,\ \min G), \qquad \vee = (\max q^{-1},\ \min L,\ \max G).$$

Both operations are computed coordinatewise. **The only way either can fail is for the coordinatewise answer
to name a shape the design does not admit.** That is the whole of the matter, and everything below is a
survey of which shapes get named and which are missing.

### 2.2 Closure in the anchored fixed-point family, and the two conditions it needs

Take the anchored case first: bias zero, radix two, so every value set contains the origin. Shapes are
parameterised by an integer width $I$ and a fraction width $F$, unsigned or signed:

$$\text{unsigned } (L,G) = (0,\ 2^{I} - 2^{-F}), \qquad
  \text{signed } (L,G) = (-2^{I-1},\ 2^{I-1} - 2^{-F}).$$

Two closure conditions arise, and they do different jobs:

- **(Z)** admit the numeral whose value set is exactly $\{0\}$, that is, zero total width;
- **(N)** admit negative integer width, so a grid may be finer than one whole unit while still starting at
  zero.

`150_probes/closure.py` builds the entire finite order under all four combinations, both signs, and reports
the verdict directly. The result is exact, with no residue:

| (Z) | (N) | meet total | meet exact | join total |
|:---:|:---:|:----------:|:----------:|:----------:|
| no  | no  | **no**     | n/a        | yes        |
| no  | yes | **no**     | n/a        | yes        |
| yes | no  | yes        | **no**     | yes        |
| yes | yes | yes        | yes        | yes        |

**(Z) alone decides whether the meet exists. (N) decides whether it equals the intersection. Neither touches
the join.** That asymmetry is the opposite of the usual intuition, on which a union is the harder operation.

Both failures are concrete. $U0.1 = \{0, \tfrac12\}$ and $U1.0 = \{0,1\}$ intersect in $\{0\}$, and with no
numeral denoting $\{0\}$ the pair has no lower bound at all. $U0.3 = \{0,\tfrac18,\dots,\tfrac78\}$ and
$I0.3 = \{-\tfrac12,\dots,\tfrac38\}$ intersect in $\{0,\tfrac18,\tfrac14,\tfrac38\}$, a perfectly good
four-point grid whose shape needs $2^{I} = \tfrac12$, that is $I = -1$; refuse negative width and the best
lower bound collapses to $\{0\}$.

**Both are decisions about vocabulary, not about mathematics.** The design may admit or refuse a zero-width
numeral. What it cannot do is refuse it and still claim a meet.

The join needed neither condition, and the boundary cases the finite enumeration flagged were confirmed to be
enumeration artifacts by recomputing the same pairs in a strictly larger box: 1176 pairs drawn from
$I,F \le 4$, all 1176 with a unique join once computed in $I,F \le 7$.

## 3. Isolating what actually breaks it, one ingredient at a time

The four-condition order admits a phase, so the anchored analysis above is a slice rather than the family.
Four ingredients could plausibly break the structure, and the honest way to find out is to vary one at a time
with everything else held fixed. **The bias is held at zero for the first three**, so that no phase failure
can be responsible for what is found.

### 3.1 A bias breaks the meet and leaves the join alone

With a free affine offset, two grids can be disjoint. `150_probes/poset.py` case C: $U0.1@0 = \{0,\tfrac12\}$
against $U0.1@\tfrac14 = \{\tfrac14, \tfrac34\}$, two legal numerals sharing no value. The empty set is not a
numeral, so there is no lower bound, so no meet. This reproduces `146`'s finding and `148`'s biased row, and
those two agree.

### 3.2 A second value map does not break anything on its own

`148` records two adjustment constructors and a five-point progression at quantum one quarter realised at
radix five (`148:351`). A quarter is not a power of five, so under at least one map the quantum is not a power
of the radix, and the natural such map spreads $r^p$ points across a unit interval, giving quantum
$r^{e}/(r^p - 1)$. At radix two that yields quanta $2^e$, $2^e/3$, $2^e/7$, $2^e/15$.

That map couples the count to the quantum, which looked like the culprit. It is not.
`150_probes/fullrange.py` adds it and nothing else, at radix two, bias zero: **zero pairs with more than one
minimal upper bound, over 6216 pairs against a search pool strictly wider than the operand set.** Removing the
map gives the same zero over 2415 pairs.

### 3.3 A second radix breaks both, and this is the ingredient

`150_probes/radix_mix.py` holds the bias at zero and the value maps fixed, and varies only the set of radices:

| radices | pairs | join not unique | meet not unique |
|---|---:|---:|---:|
| two | 1225 | **0** | **0** |
| two and three | 6441 | **28** | **18** |
| two, three and five | 15753 | **30** | **36** |

The witness carries the mechanism in its own numbers. For the pair at quantum $\tfrac14$ over indices
$[-4,3]$ joined with quantum $\tfrac12$ over $[-1,1]$, the two minimal upper bounds are

- quantum $\tfrac18$, count 17, **reaching 1**, and
- quantum $\tfrac14$, count 15, **reaching $\tfrac74$**.

**The finer cover is the shorter one.** Refining the grid did not extend the range, so neither cover contains
the other, and there is no least. The meet witness has the same shape in the other direction, with maximal
lower bounds at quantum $\tfrac18$ reaching $\tfrac18$ and at quantum $\tfrac14$ reaching $\tfrac14$.

**Why one radix is safe and two are not.** Within a radix, adding a digit multiplies the count by $r$ and
refines the quantum by $r$ at the same window shape, so the finer cover contains the coarser one by
construction and the minimal cover is forced. That is a monotonicity, not a coincidence, and it is what makes
the single-radix family a lattice. Across radices there is no such tie: the counts available at a given
quantum are whatever the radices that can express that quantum happen to offer, and the reach that comes with
them is unrelated to the refinement.

### 3.4 The same break, arriving from a different direction: fixed point beside float

This is the appearance that governs arvo, and it is not in any of the three files.

A binary float with $p$ significand digits over exponents $[a,b]$ is determined by $p$, its reach $b+1$, and
its finest step $a-p+1$, and containment among floats is again componentwise, so the float family alone is a
lattice by the same argument. Put the two families in one order and both operations fail.

`150_probes/predicate.py` derives inclusion from shape parameters with no set materialised, checks the
derivation against brute-force enumeration over **5184 ordered shape pairs spanning both families with zero
disagreements**, and only then goes to widths anyone would ship.

**The join fails.** Take $U20.0$, the integers below $2^{20}$, and $U0.20$, the multiples of $2^{-20}$ below
one. The least fixed-point cover is $U20.20$, carrying 40 significant digits. A float covers the union too:
`binary32` has 24 significand digits and each operand needs 20. And

$$U20.20 \not\subseteq \texttt{binary32}\ (40 > 24), \qquad
  \texttt{binary32} \not\subseteq U20.20\ (\text{reach } 2^{128}).$$

Two incomparable covers, neither refinable, so no least common target exists.

Note what that pair is: **two fixed-point numerals.** Their join existed in the fixed-point family and was
$U20.20$. Adding floats to the same order **removed a join that was already there**, so the fixed-point family
is a sub-poset of the whole and not a sublattice of it. Over $I,F \le 8$ with the four shipped float formats
present, 675 pairs lose their join this way and every one of them is fixed against fixed.

**The meet fails, and worse.** $U20.20 \sqcap \texttt{binary32}$ has as its fixed-point lower bounds those
with $I \le 20$, $F \le 20$, $I+F \le 24$, whose maximal elements are the seventeen shapes
$U4.20, U5.19, \dots, U20.4$, pairwise incomparable. No greatest lower bound, and the obstruction is an
antichain of seventeen equally good answers rather than a missing degenerate shape. The smallest witness is
$U1.8 \sqcap \texttt{bfloat16}$, with the two maximal lower bounds $U0.8$ and $U1.7$.

**It is the same mechanism as 3.3.** A float trades precision against reach along a diagonal; a fixed-point
numeral fixes the two independently. `binary32` refines further and reaches further than $U20.20$ but carries
fewer significant digits. A diagonal constraint cutting across a product order produces antichains, and an
antichain is exactly what a meet or a join cannot resolve.

## 4. The one mechanism, stated once

Everything in section 3 is one statement seen from four sides.

> The order is a lattice exactly when refinement and reach are independent coordinates whose shape space is
> closed under componentwise extremum. It stops being a lattice as soon as two shapes trade refinement
> against reach along a diagonal, because a diagonal across a product order produces antichains.

A second radix introduces such a diagonal. A float family beside a fixed-point family introduces such a
diagonal. A free phase introduces a different defect, disjointness, which kills the meet alone. Missing
degenerate shapes introduce a third, a missing bottom, which also kills the meet alone.

`148`'s sliding window is a special case of the first: when placement is free, the covers at one size differ
by translation and none contains another. `150_probes/sliding.py` builds that mechanism directly, without
going through either of the other representations, and confirms it. Covering $\{0\}$ and $\{2\}$ under a
power-of-two size ladder with free integer placement gives exactly two minimal covers, $[-1,2]$ and $[0,3]$.
**The mechanism is sound.** The same construction with placement anchored to zero gives one cover, so
anchoring removes the freedom the mechanism needs, and in the anchored fixed-point family nothing slides:
1176 pairs drawn from $I,F \le 4$, joins computed in $I,F \le 7$, zero non-unique.

## 5. Where the three reads land

### 5.1 `145`: right about its slice, and its slice is a real one

`145` reports a lattice with exact meets and overshooting joins. Under one radix, zero bias, and a shape
space closed downward, **that is correct**, and `150_probes/closure.py` reproduces it exactly: 300 pairs, 300
unique meets, all 300 equal to the intersection, 300 unique joins, 200 of them strictly overshooting.

Its two-condition order was refuted by two independent experts and that refutation stands. But the structural
conclusion it drew is not collateral damage: it is the correct structure of the slice it examined, and if the
design chooses to live in that slice the conclusion is available.

### 5.2 `146`: the meet is right, and the join fails on one clause

`146:226-227` reads:

> The join does. The least arithmetic progression containing two others always exists, with quantum the
> greatest common divisor of the two quanta and their offset difference, **and range the union's hull**.

The first half is a theorem and it is correct. The lattice coordinate joins: any cover's grid must contain
the grid the union generates, which pins the quantum to $\gcd(q_1, q_2, b_1 - b_2)$, uniquely. `148` states
the same at `148:340` and I derive it independently.

**The emphasised clause is the unchecked step.** It assumes the design admits a numeral whose window is
exactly the union's hull. Where the window's size is drawn from a ladder, the hull is generally not an
admissible window, and the join has to go up, at which point it can go up in more than one incomparable way.

The witness is hand-checkable and lives in `150_probes/decisive.py`. Join $\{0, \tfrac18\}$ with
$\{0, \tfrac14\}$. The gcd is $\tfrac18$ and the hull is $\{0, \tfrac18, \tfrac14\}$, three points, which is
what `146` prescribes. Three points at quantum $\tfrac18$ requires a count of three, which requires radix
three, whose quanta are powers of three and never $\tfrac18$. **So the prescribed join is not a numeral.** Two
covers that are, $\{0,\tfrac18,\tfrac14,\tfrac38\}$ and the nine-point grid at $\tfrac1{24}$, intersect in
exactly the union, which settles the absence without any appeal to minimality.

So `146` proved one coordinate joins and concluded the product joins. It is a closure assumption, in exactly
the place section 1 predicts.

### 5.3 `148`: the mechanism is sound, the verdict outruns the evidence twice

`148`'s join finding is right and its mechanism is valid; I built it independently and it holds
(`sliding.py`). Its instrument is better than the obvious one and its decisiveness argument at `148:346` is
correct, and I have used that argument rather than inventing another.

**Two scoping problems.**

First, the meet verdict. `148:363` says "**The family is a meet-semilattice and not a join-semilattice**",
and the sentence immediately after it says "In the unbiased slice the meet is present in every one of 351
pairs". Its own table one row up reports **184 of 351 meet absences in the biased slice**, and its own prose
at `148:378` says "Bias removes the anchor, and the meet goes with it". The meet verdict is stated over the
family and evidenced on a slice, while the join verdict is stated over the family and evidenced over the
family. That asymmetry is what makes `146` and `148` read as exact inverses at `149:50` when on the meet they
agree.

Second, the mechanism offered for the unbiased meet. `148:375` argues that every unbiased value set contains
the origin, so the intersection is anchored, "and among the anchored windows inside it there is a largest".
**Anchoring gives a lower bound. It does not give a greatest one.** With bias zero throughout,
$\{0,2,4,6\}$ (count 4, quantum 2, radix two) against $\{0,1,2,3,4\}$ (count 5, quantum 1, radix five)
intersect in $\{0,2,4\}$, whose maximal lower bounds are $\{0,2\}$ and $\{0,4\}$, incomparable and both
anchored. Three points at quantum 2 needs a count of three, hence radix three, whose quanta are powers of
three. So $\{0,2,4\}$ is not a numeral, and the argument that settles it needs no minimality either.
`148`'s own case F is this shape and its own description says the two numerals are "on the same grid, in
phase", which is to say unbiased, while the file files the case under bias.

**What I could not reproduce, stated as a disagreement rather than an adjudication.** `148` reports 81 decided
join failures in the unbiased radix-two slice. Two independent instruments of mine find zero there:
`decisive.py` (two upper bounds meeting exactly in the union) over 26565 pairs, and `minimal_ubs.py`
(minimal upper bounds over an explicitly bounded region, checked stable between search depths 8 and 10) over
5356 pairs. Adding `148`'s second value map at radix two still gives zero over 6216 pairs. The failures appear
only when a second radix enters. Either its radix-two slice admits shapes my reconstruction does not, or one
of the two families is described differently. **This is the one place a third instrument is genuinely owed**,
and per the spike rule I cite its probe for what it proved rather than rebuilding it.

## 6. What the structure gives and what it withholds

Stated for the family the design actually has, meaning more than one radix or a float family beside a
fixed-point one, since that is the case that governs unless op narrows it.

**Total, and derivable, so the canon may state them as consequences:**

- **The order itself.** Four conditions, decidable from the declared members, no search. Two numerals are
  always comparable or not, and the answer is a conjunction of four arithmetic tests.
- **The grid of any common target.** The quantum is forced to $\gcd(q_1, q_2, b_1 - b_2)$ by a proof, not a
  convention. This is the durable half of `146`'s join argument and it holds in every configuration examined.
- **Embedding along $\sqsubseteq$.** Where $a \sqsubseteq b$ holds, the conversion is total, exact, and
  determined. Nothing in this file touches that, and it is the case the design cares most about.
- **The existence of upper bounds.** Some common target always exists. `146` is right about that and `148`
  agrees explicitly at `148:396`.

**Partial, so the canon must say what happens when they are absent:**

- **The meet.** Absent under a bias (disjoint grids), absent across radices, absent across families, absent
  without a zero-width numeral.
- **The join.** Absent across radices and across families. Present within one radix.

**Absent, so the canon may not appeal to them at all:**

- **A least common target as a derived consequence.** This is `148`'s load-bearing conclusion and I confirm
  it, on independent evidence and by a different route. Where two numerals need a common target, the target
  is a **named rule**, not a mathematical consequence, because nothing in the order distinguishes among the
  incomparable minimal covers.
- **Any argument of the form "take the join" in a mixed-strategy operation, a widening result, or a
  conversion.** The phrase names something that frequently does not exist.
- **Distributivity, modularity, complements, and everything that rests on them.** These are properties of a
  lattice and there is no lattice to have them.

**And one thing the structure gives that is easy to miss.** The failures are not scattered. Every one of them
is an antichain produced by a refinement-against-reach diagonal, which means a **choice rule stated once, in
those terms, covers all of them**. The design does not need a rule per situation. It needs one rule that says
which way to break a tie between more refinement and more reach, and that rule is short, is stateable in the
canon as intent, and is checkable against every case in this file.

## 7. What is op's

Three calls, in the order they gate each other.

**One, and it decides everything else: is the numeral space one family or several?** If arvo's numerals are
one radix with the shape space closed downward, the order is a distributive lattice, a common target is
derived, and the canon says so with a proof behind it. If the space carries more than one radix, or carries
floats beside fixed point, the order is a poset with neither operation total, and the canon must name a
choice rule. Both are workable and the text differs completely. Nothing in mathematics decides this and I do
not think an expert should: it is a question about what arvo is for.

**Two, if the space is one family: does a zero-width numeral exist, and does negative integer width exist?**
The first decides whether the meet exists at all. The second decides whether it equals the intersection.
Section 2.2 is the whole answer and the table there is the decision.

**Three, if the space is several families: which way does the tie break?** More refinement or more reach.
Every failure in this file is that tie and one sentence settles all of them. The obvious candidates are the
coarsest cover (fewest values, cheapest, loses reach), the longest cover (widest reach, loses resolution), and
a stated preference for staying inside the source's own family. I have deliberately not picked, because
`148:775` reports the same question and `149:107` carries it to op already, and re-proposing an answer to a
question already in front of him is the failure mode the panel's own rules name.

## 8. What I did not settle, and what a later reader should attack

Four things, so the next pass starts from a list rather than from nothing.

**`148`'s 81 unbiased radix-two join failures.** Section 5.3. My two instruments find zero and its instrument
finds 81, and the difference is in the shape space rather than in the arithmetic. Resolving it needs its
probe read for its admissibility set rather than its conclusions.

**Whether the sign domain is a coordinate or an input.** `146` and `148` disagree and `149:63` records it
open. My analysis treats signedness as a two-valued placement of the window, which is enough to compute with
and is not a claim about which of the two readings is right. Under that treatment it changes nothing: the
mixed-sign anchored family is a lattice on the same terms as the unsigned one: `closure.py` reports
2628 pairs with a unique exact meet and a unique join, and `sliding.py` confirms the join over 1176
pairs drawn from a smaller box than the one the joins are computed in.

**Whether a bottom element is wanted for its own sake.** I have shown that admitting $\{0\}$ makes the meet
total in the single-radix family. I have not asked whether a numeral denoting exactly zero is a good idea for
any other reason, and it plainly interacts with things this file does not touch.

**The approach I tried and dropped.** I first attempted to settle the question by finding the richest shape
space in which the lattice survives, on the theory that the design would want the most permissive closed
family. That went nowhere useful, because the closures that repair the meet (a bottom, negative widths) are
cheap and the one that would repair the join across radices does not exist: you would have to admit a numeral
at every count and every quantum independently, which dissolves the radix as a concept. **That is worth
recording as a dead route**, because it means the mixed-radix join cannot be rescued by widening the
vocabulary, and a choice rule is the only remaining shape.

**One methodological finding, recorded because it nearly cost this file its conclusion.** My first sweep of
the coupled family reported large meet and join failure counts at radix two. They were an artifact of my own
enumeration bound: `NN(64,1/8) ^ NN(64,1/4)` looked like a failure only because the count 32 at quantum
$\tfrac14$ had been excluded by a precision limit in my generator, and the intersection was exactly that
shape. The counts were wrong and the two hand-checked witnesses were not, because their argument needs two
bounds and an inadmissibility, never a complete enumeration. **A count over a truncated shape space is worth
nothing here, in any of the three files or in mine**, which is why `149`'s instruction not to adjudicate by
counting is right for a reason stronger than the one it gives: the counts are not merely incomparable across
models, they are individually unreliable within a model unless the enumeration bound is part of the argument.
Every number in this file that carries weight is attached to a witness that survives without it.
