# 03. The family question, and what each answer costs

**Date:** 2026-08-08
**Position:** the first working member of this panel. Takes the task `01` section 1 records as op's:
lay the consequences of the three readings out before he rules.
**Probes:** `03_probes/`, five instruments, with `RUN.md` carrying every command and its exit code.
**Register:** nothing here settles anything, per `04`. Every statement below is *this appears to hold*,
*this reading survives*, or *this route is closed and here is what closed it*.
**Reading:** `RULES.md`, `01_op_answers.md`, `SETTLED.md`, `DROPLIST.md`,
`02_carried_what_replaces_the_two_refutations.md`, `seed/SETTLED_laws.md`, `04_op_no_settlements_tonight.md`,
`PERSONA_CALLS.md`, all in full and in that order. `CANON_CANDIDATE.md` read at its stale-list header and
nowhere else. The closed panel's tree was not opened, so every citation of the form `150:377` below is
`02_carried` or `SETTLED_laws` reporting that file, never that file itself. Section 9 says what that costs.

## What this file found, stated before the argument

The three readings op was offered are not three answers to one question. They are answers to **three
different questions**, and the measurements separate them cleanly enough that the separation is the main
thing worth carrying to the morning.

**The promise attached to reading A does not survive contact with its own scope.** `SETTLED.md:124` states
the result it rests on as holding "within one radix, zero bias, and a closed shape space". The reading as
offered in `01` carries the third condition and drops the first two. All three instruments agree that
dropping either of the dropped ones breaks the promise, and the break is structural rather than a bound
artifact. So reading A appears to be reading C wearing reading A's clothes: it is one family only if the
family is already narrow.

**Two operations are being carried as one and they behave differently.** The join is the operation a
derived result numeral needs. It is total inside a uniform-grid family at one radix and its failures across
families are of a kind that admitting more shapes cannot repair. The meet is the one the two admissions in
reading A are for, and **I could not find, in everything I read, an operation the design needs that the meet
answers.** If that holds after a second read, reading A is buying closure for an operation with no consumer.

**Three distinct failure modes are hiding under the phrase "the operation is partial", and each takes a
different repair.** No bound at all; bounds present with no extremal one; an extremal one that is not the
exact intersection. The first is repaired by admitting an endpoint shape, the second cannot be repaired by
admitting anything larger, and the third is repaired by admitting shapes in between. A canon sentence that
says "partial" says none of that.

**The predicate-amendment claim from `02_carried` section 1.6 appears to hold**, in the precise form it
states and no stronger. Two enumerations in two languages attribute every disagreement to a source carrying
fewer than two values, 188 of 188 and 28 of 28, with zero unexplained.

**The radix does not carve what it is assumed to carve.** Radix 2 against radix 4 behaves as one family.
Radix 2 against radix 3 does not. Radix 2 against radix 6 fails in a third way that neither of the other two
pairs shows. So "each radix a family" appears to be the wrong shape of the question; the seam that the
measurements find is the **step set**, and it is computable from the members rather than declared.

**Three further readings are on the table that op's three do not cover**, plus one re-decomposition that is
not a fourth answer, plus one route that closes. They are section 7. The one that narrows the map most is
that **the design's named operations derive their result numerals by formula rather than by least upper
bound**, so the join may have no caller either, and the structure question may be downstream of a different
question: whether the design infers a common numeral the consumer did not write.

**Nothing bearing on this question sits at the RATIFIED rung.** Section 0.3 checks that, since `04` asks
which rows are acks.

## 0. Gates

### 0.1 The canon gate

There is no ratified canon for arvo. This panel is writing the first one, `SETTLED.md:3-14` says its own top
rung was classified under a reading op has since corrected, and `01` section 0 carries the correction. So the
defend-the-canon posture has no target and the governing material is the narrow set that records op in the
loop: the acceptance criterion at `135b:12-16` quoted in `SETTLED.md:63-74`, and the rows the two files mark
RATIFIED.

Nothing below asks for anything that material forbids. The erasure gate in particular is untouched: every
operation discussed here is a compile-time question about which numeral a typestate derives, which is the
layer the gate says erases on lowering rather than a layer it forbids. Gate passed.

### 0.2 The test gate

`02_carried` section "Gates" reports the suite as 16 crates, 91 files under a `tests` path, 83 containing
`#[test]`, and reports that grepping the shipped source for this theme's vocabulary returns zero for
`SignDomain`, `Quantisation`, `trait Numeral` and the rest. I re-ran the two counts that bear on my own
question rather than inheriting them:

    $ grep -rn --include='*.rs' -l 'fn meet\|fn join\|LeastUpper\|GreatestLower' mock/crates | wc -l
      0
    $ grep -rn --include='*.rs' 'AddClosed\|Numeral\b' mock/crates | wc -l
      0

So there is no suite to audit for this question, because nothing under test touches it. The brief separately
declares `mock/crates` nuked and forbids citing it as evidence about what is correct, which is the mutation
order doing its job. I did not run the suite, and I am saying that rather than implying it passed.

### 0.3 Which rows bearing on this question are acks

`04` asks for this explicitly. I went through every row in `SETTLED.md` and `seed/SETTLED_laws.md` that
bears on the family question and checked its rung:

| Row | Where | Rung as recorded |
|---|---|---|
| The structure question is a closure question about the shape space | `SETTLED.md:122` | ONE EXPERT |
| The deciding ingredient is whether refinement and reach move together | `SETTLED.md:123` | ONE EXPERT |
| Within one radix, zero bias, closed shape space: both total, meet exact | `SETTLED.md:124` | TWO EXPERTS |
| Inclusion needs grid, phase and both endpoints | `SETTLED.md:118` | TWO EXPERTS |
| The cardinality antichain | `SETTLED.md:119` | ONE EXPERT |
| Within one family it is a distributive lattice, meets exact, joins overshoot | `SETTLED_laws.md:278-313` | TWO EXPERTS for the original, ONE EXPERT for the closure-condition version |

**Not one of them is at RATIFIED**, so the ack-versus-ratification re-reading `04` asks for does not fire on
this question at all. That is a clean result and it is worth stating positively: the family question is
open all the way down, and no earlier op statement has to be reinterpreted before working on it.

The one RATIFIED row anywhere near it is the acceptance criterion, and it is about the erasure gate rather
than about the structure. **Whether even that one followed a convergence I could not determine** from the
material I was given, because `SETTLED.md:76` records only that "a later checkpoint records the gate as met"
without saying whether the experts had stopped disagreeing when op stated it. What would decide it is the
checkpoint immediately before `135b`, which is in the closed panel's tree and which I was told not to open.

### 0.4 The brief's cheap factual claims, checked before reasoning from them

The pin is as stated: `rust-toolchain.toml` carries `channel = "nightly-2026-05-28"` and
`rustc +nightly-2026-05-28 --version` reports `rustc 1.98.0-nightly (57d06900f 2026-05-27)`. The three
readings are quoted in `01` section 1 as the brief gives them. `02_carried` section 1.6 does make the
predicate claim in the form the brief attributes to it. `SETTLED.md:138-142` does carry the open item in the
form the brief restates.

One claim in the brief is worth qualifying rather than accepting. It presents reading A as "both operations
become total and the meet exact". `SETTLED.md:124`, which is the row that result comes from, states it
**with three conditions**, of which the brief's summary carries one. That difference is not a quibble; it is
most of section 3.

## 1. What the question is a question about

"One family or several" reads as a question about labels, and labels are cheap to argue about and settle
nothing. Underneath it there is a question with an operational answer, and stating it that way makes the
three readings comparable.

**For which pairs of numerals must the design produce a common target without being told one?**

That is the whole content. Everything the three readings differ about is downstream of it:

- Where a common target must be produced and the shape space contains one, the operation is derivable and
  the consumer writes nothing.
- Where a common target must be produced and the shape space contains none, either the space grows, or the
  operation refuses, or a rule picks among candidates.
- Where no common target must be produced, because the design requires the target to be named, the question
  does not arise for that operation at all.

The third row is why the answer already given for conversions matters here. `02_carried` section 2.4 has a
conversion's target **named** rather than derived, and its section 2.3 has the target governing. So
conversions are already outside this question. What is left inside it is the operations whose result numeral
is **derived from the operands**: the `Resolve`-shaped ones, and the multiplicative `mulnum` map that
`SETTLED_laws.md:165-178` records as a family of maps `N1 x N2 -> mulnum(N1, N2)`.

**So the family question is, operationally, the question of what `mulnum` and its siblings do on a mixed
pair.** That is a much smaller surface than "the structure of the numeral space", and it is the surface every
consequence below lands on.

### 1.1 Two operations, and only one of them has a caller

The record carries meet and join together, as a lattice question. They are not symmetric in what they are
for.

**The join answers "what holds both".** That is exactly what a derived result numeral is: an operation on
two numerals produces values from both, so the result numeral must contain both value sets, and the least
such is the tightest honest answer. Every derived-result operation wants a join.

**The meet answers "what both hold".** Its uses are an interchange type guaranteed lossless in both
directions, or a question about what survives a round trip. Both are real questions. Neither appears, in
anything I read, as an operation the design has.

I looked for one. `SETTLED.md`, `SETTLED_laws.md`, `02_carried` and `DROPLIST.md` mention the meet only
inside the lattice discussion, as a thing measured (`146`'s 663,026 failures, `148`'s 351 unbiased pairs,
`150`'s exactness), never as a thing called. **Bounded honestly: I did not read the closed panel's tree, so
a caller may exist there and be invisible to me.** If one does not, the two closure admissions reading A
turns on are being bought for an operation nothing invokes, and that reframes op's call substantially.

### 1.2 Three failure modes, which "partial" hides

Every instrument here reports the two operations' failures split three ways rather than as one number,
because the three take different repairs and a canon sentence saying "partial" tells a reader none of it.

**F1, no bound at all.** No shape in the space is above (or below) both. Repaired by admitting a shape: a
top, a bottom, an empty numeral, the origin. Where the required grid is simply not in the admitted step set,
it is not repairable by admitting anything of the same kind, which is the cross-radix case in section 5.2.

**F2, bounds present and none extremal.** Two or more minimal upper bounds, incomparable. **Adding larger
shapes cannot repair this**, because a shape above the existing minimal ones is not below them. It is
repaired only by admitting a shape strictly between the operands and the current minimal ones, or by a rule
that picks. This is the fixed-point-against-float case and it is the one that decides the question.

**F3, the extremum exists but is not the exact intersection.** A greatest lower bound exists and is a proper
subset of the set intersection. Repaired by admitting shapes in between, which is what negative integer
width turns out to be for.

Three instruments produce witnesses for all three, cited in place below.

## 2. The geometry, stated once

So the three readings are compared against one object rather than three descriptions of it.

A numeral denotes a finite set of rationals. For the uniform-grid families the set is

$$V = \{\,b + kq \;:\; 0 \le k < n \,\},\qquad q>0,\; n \ge 0$$

with $q$ the step, $b$ the phase, $n$ the count, and endpoints $L = b$, $G = b + (n-1)q$. For the anchored
unsigned fixed-point family at radix $r$, $q = r^{-F}$, $b = 0$ and $n = r^{I+F}$. The float families are
not of this form: their value set is a union of arithmetic progressions whose step coarsens with magnitude,
which is the entire source of the difficulty.

The order is inclusion of value sets, quotiented by equal value set, since two numerals denoting the same
values are one point of the order. That quotient is not optional and it has content: the instruments report
it as "label collisions folded", and at radix 2 against radix 4 it folds `U<2,0>` and `U4<1,0>` onto one
point (`i5.out`, Q20b).

**The ambient order on all finite sets of rationals is a complete lattice for free**, meet being
intersection and join being union. `SETTLED.md:122` records that, and it is the move that makes everything
else a closure question: the numerals are a subset of the ambient, restriction does not preserve
completeness, and an operation can fail only by naming a set the design admits no shape for.

**The deciding ingredient, restated in this notation.** An upper bound of $V_1$ and $V_2$ needs a step
dividing both steps and a range covering both ranges. Within one anchored family at one radix those two
requirements move together: buying a finer step means buying digits, which buys reach at the same time, so
the requirements are satisfied in a single coordinate and the minimum is unique. A float buys reach without
buying a uniformly fine step, so the two requirements come apart and the minimum need not be unique. That is
`SETTLED.md:123`'s "refinement and reach move together", and every measurement below is an instance of it.

## 3. Reading A: one family, closed shape space

**The reading.** One ordered family. The shape space is closed under both operations, which per the offered
statement means a zero-width numeral exists and negative integer width is allowed. Both operations total,
the meet exact.

### 3.1 What becomes derivable, and what the measurements say instead

**Inside the unsigned fixed-point family at radix 2 with zero bias, both operations are total.** Measured
three ways. The meet is present for every pair at every box size tried (`i1.out` Q2, `i2.out` Q8 at
lim 6 and 7, `i3.out` C2), and every reported join failure is a shape lying above the enumeration's own
ceiling rather than a structural absence (`i2.out` Q11: 252 joins present, 126 missing with the join shape
outside the box, **0** missing with it inside). So within that scope the reading delivers exactly what it
promises, and the promise is not an artifact.

**The zero-width numeral is what the meet needs, and it is needed for a reason worth stating.** Refusing the
origin shape costs 36 meets at lim 6 and 49 at lim 7 in Python (`i2.out` Q8) and 36 at lim 6 in Rust
(`i3.out` C2), the two instruments agreeing on the number from different representations. The first absence
in both is `U<0,1>` against `U<1,0>`, whose value sets are $\{0, \tfrac12\}$ and $\{0, 1\}$ and whose
intersection is $\{0\}$.

That gives a sentence worth having whichever reading op takes:

> The zero-width numeral is not a new kind of object. It is the family's own curve evaluated at the origin,
> and admitting it is the absence of a carve-out rather than the presence of an extension.

**Negative integer width turns out to be for F3 rather than for F1**, which no document I read says. Across
two sign domains at one radix with zero bias, the meet exists for every pair and is a proper subset of the
intersection for 40 of them (`i2.out` Q13, `i3.out` C4). Witness in both: `U<0,2>` against `S<0,2>`, sets
$\{0,\tfrac14,\tfrac12,\tfrac34\}$ and $\{-\tfrac14, 0, \tfrac14\}$, intersection $\{0,\tfrac14\}$,
greatest lower bound $\{0\}$. The intersection needs step $\tfrac14$ and two values, so $I+F = 1$ with
$F = 2$, so $I = -1$. Admitting negative integer width takes the undershoot from 40 to 4 in Python at
$I \ge -3$ and to **0** in Rust at $I \ge -4$, the residual four in Python being at that run's own floor
(`i2.out` Q16, `i3.out` C4).

So the two admissions do different jobs and are not a pair: one restores existence, the other restores
exactness.

### 3.2 Where the reading's promise stops, and this is the part that matters

`SETTLED.md:124` states the result with three conditions: **one radix, zero bias, closed shape space.** The
reading as offered carries the third. Each dropped one breaks something, structurally.

**Bias.** With four biases admitted in one space, 670 of 1326 pairs have no lower bound because their value
sets are disjoint, and a further 130 have a nonempty intersection that names no shape (`i2.out` Q14).
Disjointness is structural: two numerals whose phases are incompatible modulo their steps share nothing, and
the only set contained in both is empty. **So a biased space needs an empty numeral, which is a third
admission the reading does not name**, and it is a different object from the zero-width numeral: the origin
denotes $\{0\}$ and the empty numeral denotes nothing. The 130 is partly an artifact of my finite bias list,
since a space with arbitrary bias would contain the singleton those pairs want, and I am saying so rather
than counting it.

**Radix.** Two radices in one space produce 96 cross-radix pairs with no upper bound at all, of which 60 are
structural in a sense no enlargement touches: the required step has both 2 and 3 in its denominator and the
admitted step set contains only powers of $\tfrac12$ and powers of $\tfrac13$ (`i2.out` Q12). Section 5.2
takes this further, because the radix turns out not to be the seam.

**Kind.** This is the decisive one. Put the float family in the same space as fixed-point and both
operations fail structurally: 220 pairs with upper bounds and none least, 90 with lower bounds and none
greatest, at box 4, rising to 302 and 129 at box 5 (`i1.out` Q4). These are F2 failures, and **F2 is not
repairable by admitting more shapes above.**

The clean witness, which three instruments produce and which I also derived by hand before running any of
them:

> $U\langle 0,1\rangle = \{0, \tfrac12\}$ and $U\langle 2,0\rangle = \{0,1,2,3\}$ have the join
> $U\langle 2,1\rangle$ among fixed-point numerals alone. With a float present they have two minimal upper
> bounds, $U\langle 2,1\rangle$ and $F\langle p2, e{-1}..1\rangle = \{0,\tfrac12,\tfrac34,1,\tfrac32,2,3\}$,
> neither containing the other, and no least upper bound.

`i1.out` Q5 counts 18 such joins present before floats and absent after, at that box. `i3.out` C3 reproduces
the same pair and the same two minimal bounds in Rust and prints the non-containment both ways. The
arithmetic reason is one line: the exact union $\{0,\tfrac12,1,2,3\}$ needs a uniform shape with step
$\tfrac12$ and ceiling 3, which is 7 values, and 7 is not a power of the radix, so no uniform shape sits
between.

**And neither admission helps.** The origin and negative integer width both add uniform-grid shapes, and no
uniform-grid shape can sit between those operands and $U\langle 2,1 \rangle$, for the count reason just
given. So:

> **Reading A's two named admissions do not make the join total once two kinds share one order.** They close
> the uniform-grid family and they do not close the union of it with a float family. The reading is
> deliverable at the scope `SETTLED.md:124` states and not at the scope the option statement implies.

That is the sharpest thing in this file and it is the one I would most want a second read on.

### 3.3 What has to be named under reading A

The real cost of the reading is the sentences that exist only because of it.

**The zero-width numeral's members.** It denotes $\{0\}$. It still has a declared radix, a declared step, a
declared sign domain. Nothing in the order distinguishes $U\langle 0,0\rangle$ from $U\langle -3,3\rangle$,
both denoting $\{0\}$, and the quotient folds them; the canon has to say whether they are one numeral or two
spellings, and the answer bears on value-uniqueness, which `DROPLIST.md` records as already having killed
one width encoding for exactly this ("`UInt<UTerm, B0>` is a second spelling of zero").

**Negative integer width's meaning, and its floor.** $I$ can go arbitrarily negative in the mathematics.
Something has to say where it stops, or say that it does not, and if it does not then the shape space is
infinite in a direction the container derivation has to survive.

**The empty numeral, if bias is admitted.** Whether it exists, what its members are, and what arithmetic in
it means. An operation on it produces nothing, which is a value-level statement the law family has to
tolerate.

**The order predicate's amendment.** Section 6. Admitting the shapes is not sufficient, because the
four-condition predicate does not see a singleton declared at a fine grid as being below a coarse-grid
numeral even though its value set is.

**Which sign domain the derived numeral carries.** Once two sign domains are in one order, the meet is a set
which may be expressible in either, and nothing in the order picks. This is new here and it is not in the
material I read.

**And, if the reading is taken across kinds, a rule that does not exist yet**: which of two incomparable
minimal upper bounds is the answer. That is not a closure condition. It is a tie-break, and it is section 7.

### 3.4 What it costs a consumer

The consumer-facing consequence of totality is the one that took me by surprise, and it runs against the
reading's own selling point.

**Totality converts a diagnosable refusal into a silent degenerate answer.** Under reading A, a consumer
whose two numerals have little in common gets a compile-time success whose result is the zero-width numeral.
An expression that in reading B would have refused with "these two have no common numeral" instead compiles
and produces a type that holds exactly one value. The most common shape of that, from the measurement, is a
meet: `U<0,1>` against `U<1,0>` gives $\{0\}$ (`i2.out` Q8, `i3.out` C2), and neither operand is exotic.

For the join the same effect is milder but present: `SETTLED_laws.md:278-288` records joins as strictly
overshooting at every incomparable pair, so a derived result numeral is honest but wider than the values
warrant, and a consumer who expected tightness gets a wider container than they asked for. The erasure gate
means that width is real at lowering.

So the trade under reading A is: **nothing refuses, and the consumer has to notice for themselves that the
answer is degenerate.** That is a defensible design and it should be chosen knowing that is what it is.

## 4. Reading B: one family, shape space not closed

**The reading.** One ordered family, no zero-width numeral, the meet partial.

### 4.1 What becomes derivable

Everything reading A derives, minus the pairs whose meet is the origin. Measured: 36 pairs of 378 lose their
meet at lim 6, 49 of 630 at lim 7 (`i2.out` Q8), so roughly one pair in ten in that box, and the fraction
grows with the box rather than shrinking, since the pairs that meet at the origin are the ones far apart in
both coordinates and there are more of those as the box grows.

The join is unaffected. Every join failure in the uniform family is a box artifact (`i2.out` Q11), and no
join in that family is the origin, since a join of two nonempty sets contains both.

**So reading B is a statement about the meet and about nothing else**, inside one uniform family. That is
worth stating plainly because the reading is phrased as a general "not closed" and its actual content, at
the scope where reading A works, is one operation and one shape.

### 4.2 What has to be named

**The refusal, as a first-class outcome with a diagnostic.** If the meet is partial, its failure is
something a consumer meets, so the canon owes what it says. The measurement gives it something precise to
say: the failure is F1 with a computable witness, "these two numerals share only the value zero, and this
design has no numeral for that".

**Which of the three failure modes the partiality is.** Under reading B at the narrow scope, it is F1 only.
Under reading B at a wider scope it is all three, and they read very differently to a consumer. The canon
has to say which it means, and this is the place the current vocabulary is thinnest: `SETTLED.md:138-142`
and the option statement both say "the meet goes partial" without distinguishing them.

**Nothing else.** That is the striking thing about reading B and the reason it deserves more weight than a
reading defined by an absence usually gets. It requires no new shape, no new member, no floor on negative
width, no empty numeral, no rule about which sign domain a derived numeral carries. **Its named-things list
is one entry long.**

### 4.3 What it costs a consumer

An expression that would have produced a degenerate numeral refuses instead. Whether that is a cost or the
point is exactly the question, and the honest framing is that it is a cost at the moment it happens and a
benefit at the moment the alternative would have been shipped.

The measured shape of it: at the box sizes tried, the refusal falls on pairs whose grids and ranges are far
apart, which is where a consumer is most likely to have made a mistake rather than to have meant it. That is
weak evidence and I am marking it weak: it is one enumeration's distribution, not a study of what consumers
write.

### 4.4 The thing that makes reading B larger than it looks

If section 1.1 holds and the meet has no caller, **reading B costs nothing at all**, because the operation it
makes partial is one nothing invokes, and the diagnostic it owes is never printed. That would make it the
cheapest of the three by a wide margin, and it would make reading A's admissions pure cost.

I cannot settle whether the meet has a caller, for the reason in section 1.1, and this is the single
highest-value thing a second read could resolve. It is one grep of the closed panel's tree by someone
licensed to open it.

## 5. Reading C: several families

**The reading.** Fixed point and float, and possibly each radix, are separate ordered families. Both
operations fail across families, so a common target is always a named rule.

### 5.1 What becomes derivable

**Within each family, everything.** The uniform fixed-point family at one radix is measured total in both
operations above. The float family alone is measured total in both operations, meets exact, joins present,
zero failures at both sizes tried: 630 of 630 pairs at $p \le 3$ and 1128 of 1128 at $p \le 4$ (`i2.out`
Q15). That is a genuinely clean result and it is the strongest single piece of support reading C has: the
float family is a lattice on its own without needing any of reading A's admissions.

**Across families, nothing, and the canon says so once.** That is the reading's whole content and its whole
appeal.

### 5.2 The seam is not where the reading puts it

Reading C offers two candidate seams: kind, and radix. The measurements support the first and refute the
second, and they suggest a third that is better than either.

**The radix is not a seam.** Radix 2 against radix 4 gives 98 cross-pair joins present, 31 with no upper
bound inside the box, and **zero** of the F2 kind. Radix 2 against radix 8 and radix 3 against radix 9 give
the same shape of answer (`i4.out` Q17). The 31 are not classified by that instrument, so I am not calling
them artifacts on the measurement; the reason to expect they are is one line: $4^{-b} = 2^{-2b}$, so the two
step sets are nested, and a nested pair's missing upper bound is the same box truncation `i2.out` Q11
classifies for the single-radix case. What the measurement does establish without qualification is the
absence of F2, which is the failure mode that no enlargement repairs. The quotient sees it directly, folding
`U<2,0>` and `U4<1,0>` onto one point (`i5.out` Q20b).

**Radix 2 against radix 3 is a seam, and it fails as F1**, 96 cross pairs with no upper bound at all and
zero of the F2 kind (`i4.out` Q17). No step in $\{2^{-i}\} \cup \{3^{-j}\}$ divides both a dyadic and a
triadic step, and no enlargement of either side supplies one.

**Radix 2 against radix 6 is a third thing and it is new.** 52 pairs with no upper bound and **3 with upper
bounds present and none least** (`i4.out` Q17), the only radix pair to show F2 at all. The witness
(`i5.out` Q21): $U2\langle 0,1\rangle = \{0,\tfrac12\}$ against $U6\langle 1,0\rangle = \{0,1,2,3,4,5\}$ has
minimal upper bounds $U2\langle 3,1\rangle$ and $U6\langle 1,1\rangle$, incomparable. Radix 6 shares the
factor 2 with radix 2, so some steps are common and others are not, and that partial overlap produces the
same failure shape as fixed-point against float.

So the seam the measurements find is **the step set**, specifically whether the two admitted step sets are
nested. Nested means one family, disjoint-beyond-the-integers means F1, partially overlapping means F2.
That is a relation between two numerals computable from their members, not a label attached to a kind.

**And the kind is not quite the seam either.** Splitting a fixed-plus-float space by whether the value set
is an arithmetic progression rather than by what it is declared to be, three float-declared shapes land on
the uniform side, and one of them, $F\langle p1, e0..0\rangle$, is the same point of the order as
$U\langle 1,0\rangle$ (`i4.out` Q19). So a family boundary drawn on the declaration cuts through numerals
that denote identical value sets. That is a real cost of reading C in the kind-shaped form: it makes two
names for one set live in different families, and the order cannot see the difference.

### 5.3 What has to be named under reading C

**The family relation itself.** Whether it is a label on a numeral or a relation computed from members. The
measurements push toward the second and it is more work: a computed relation has to be expressible in the
typestate under the forbidden-feature set, and I did not test that.

**The rule for a cross-family derived result**, at every operation whose result numeral is derived. Per
section 1, that is `mulnum` and its siblings, not conversions, which already name their target. The rule can
be a refusal, and a refusal is the cheapest thing to state.

**Nothing about shapes.** No zero-width numeral, no negative width, no empty numeral. Reading C's admission
list is empty, which is its real economy and is easy to miss when comparing it against a reading whose
promise is totality.

### 5.4 What it costs a consumer

A mixed-kind derived-result expression does not compile without a named target. The cost is an annotation,
at a site the consumer can see, with a diagnostic that can name both operands and say which rule refused.

Two things make that cheaper than it sounds. Conversions already name their target under `02_carried`
section 2.4, so the annotation burden falls only on derived-result operations. And the refusal is uniform:
"the kinds differ, so name the target" is one rule with no case analysis, where reading A's totality has the
consumer reasoning about whether the derived numeral degenerated.

The cost that is real: a consumer writing a fixed-point value against a float value in an expression is
doing something ordinary, and refusing it every time is a visible tax. **Unpriced**, and it is a taste
question rather than a measurement, since nothing here can say how often that expression is written.

## 6. The predicate-amendment claim, checked

`02_carried` section 1.6 claims the four-condition order's grid and phase clauses are vacuous on a numeral
carrying fewer than two values, so the four conditions are sufficient for inclusion always and necessary
only where the source carries at least two values. The brief asks me to treat it as a claim to check.

**It appears to hold, in the form it states.** Three arrivals.

**The argument, which quantifies over nothing.** A source carrying two or more values pins its own step: the
step is a difference of two of its values, so any set containing it contains that difference as a difference
of two of its own values, and the grid clause is forced. The phase clause then follows, since the source's
floor is in the target and the target's phase is its floor modulo its step. A source carrying one value pins
nothing: a singleton lies on every grid and in every phase, so its **declared** step is not recoverable from
its value set, and a predicate that reads the declaration reads something the order cannot see.

**Python.** 1936 ordered pairs over unsigned fixed-point at radices 2 and 3 with $I$ from $-3$ to 3, so
singletons at fine declared grids are present. 188 disagreements between the predicate and true set
inclusion, **188 attributed to a source carrying fewer than two values, 0 unexplained** (`i2.out` Q7).
Witness: $U2\langle -3,3\rangle$ denotes $\{0\}$ at declared step $\tfrac18$, is genuinely included in
$U2\langle -2,2\rangle$, and the predicate says no because $\tfrac14 \nmid \tfrac18$.

**Rust, scaled integers, different containment algorithm.** 484 ordered pairs, 28 disagreements, **28
attributed to the same cause, 0 unexplained** (`i3.out` C1), same first witness.

**And the instrument that got it wrong is kept.** My first Python instrument reported 1024 pairs and zero
disagreements, which would have refuted the claim. It is wrong for the reason the prior panel's own
`sign_domain.py` was wrong: its shape list contained exactly one numeral carrying fewer than two values, and
that one had the coarsest declared step in the box, so the predicate was never offered the case that breaks
it. It is left in `i1_shape_space.py` unfixed, with the failure named in its own header and in `RUN.md`,
because a setup that helps is easier to recognise next time from an instance than from a rule.

### 6.1 Where the amendment lands in the consequence lists

`02_carried` section 4 puts it under reading A, on the ground that admitting the zero-width shape is not
sufficient without it. That is right and it is narrower than it needs to be. **The amendment is owed under
reading B too**, because the predicate is the design's statement of what inclusion is, and a predicate that
is not necessary on a region is wrong on that region whether or not the design admits shapes there. Under
reading B the region is not reachable as a meet, and it is still reachable as a declaration: a consumer can
write a singleton numeral directly, and asking whether it converts into another numeral is an ordinary
question the predicate answers wrongly.

So it belongs in every reading's list, which makes it cheap to decide: it is one clause, it changes no
answer above the degenerate region, and both a Python and a Rust enumeration confirm the region it changes
is exactly the sources carrying fewer than two values.

Two shapes for it, and they are not equivalent:

> **Amend the predicate.** State inclusion as the four conditions where the source carries two or more
> values, and as membership of the source's single value where it carries one. Reads as a case analysis and
> is exact.

> **Amend the quotient.** State that the order is on value sets, that a shape denotes a value set, and that
> the four conditions characterise inclusion where both shapes are non-degenerate, with the degenerate cases
> decided by the value sets directly. Reads as a scoping and pushes the case analysis into the definition of
> the order rather than into the predicate.

The second appears to survive the permanence test better, since it stays true if the shape space changes,
where the first names a count that a differently-shaped family could make wrong. I have no second read on
that and it is a taste question as much as a technical one.

## 7. The categories the three readings do not cover

`04` asks for breadth by category before depth by variant, and says a reading that can be closed is worth as
much as a consequence set. Four here, of which one is closed, two survive, and one is not a fourth answer at
all.

### 7.1 D: the tie-break reading, and it survives

**The shape.** Stop requiring a unique least upper bound. Define the operation as returning the set of
**minimal** upper bounds, which is nonempty whenever any upper bound exists, and name a rule that picks one.
Totality is then bought with a policy instead of with closure, and the family question stops deciding it.

**Why this is not exotic.** It is what C's usual arithmetic conversions do, and it is the shape the design
already uses on a neighbouring axis: `Resolve` picks one strategy from two by a stated rule rather than by a
lattice.

**What makes it cheap or expensive is the width of the antichain**, and that is measurable. Over
fixed-point against float, the widest antichain of minimal upper bounds is **2**, and it stays 2 across
three box sizes with the shape count going 26, 56, 107 and the pair count going 325, 1540, 5671
(`i5.out` Q20). It stays 2 with three sign domains and a second commensurable radix added, at 76 points and
2850 pairs (`i5.out` Q20b). The widest pair is the same one every time,
$U\langle 0,1\rangle$ against $U\langle 2,0\rangle$ with $U\langle 2,1\rangle$ and
$F\langle p2,e{-1}..1\rangle$.

A binary choice is the cheapest possible tie-break: one sentence naming which of two kinds wins, with no
case analysis and no ordering to define.

**What has to be named.** The rule, and its justification. "Prefer the uniform-grid candidate" and "prefer
the candidate of the left operand's kind" are both statable in one line and they give different answers, so
the canon has to pick and say why. Also whether the operation exposes the antichain to the consumer or only
its choice; exposing it is more honest and more vocabulary.

**What it costs a consumer.** Everything compiles, and the derived numeral is a policy's answer rather than
a forced one. Same shape of cost as reading A, without needing the shape space to close, and with the
degeneracy risk removed, since a minimal upper bound of two nonempty sets is never degenerate.

**The bound on this.** The width 2 is measured over one float family shape and one fixed-point family shape
at moderate sizes. Adding decimal numerals, or several float families with different radices, could widen
it, and I did not test that. Under reading C's own logic those would be further families and would fail
outright rather than widening an antichain, but under reading D they would be more candidates. That is the
first thing to measure if op wants this reading taken further.

### 7.2 E: the seam is the step set, and it survives

**The shape.** Neither "one family" nor "families by kind". Two numerals are in one family exactly when
their admitted step sets are nested, which is a relation computed from their members rather than a label.

**The evidence is section 5.2** and it is the strongest measured support any of these readings has: radix 2
with radix 4 behaves as one family with zero structural failures, radix 2 with radix 3 fails as F1 in 96
pairs, radix 2 with radix 6 fails as F2 in 3 and F1 in 52, and the three outcomes track the nesting of the
step sets exactly (`i4.out` Q17, `i5.out` Q21).

**What it buys.** The family relation stops being a declaration nobody can check and becomes a fact the
order itself can see. It also explains the fixed-point-against-float failure with the same mechanism rather
than a separate one: a float's step set is not nested in a uniform family's, it partially overlaps it, which
is the radix-6 shape.

**What has to be named.** The relation, precisely. And the fact that it is not an equivalence relation:
nesting is not transitive across the shapes here in the way a family label would be, so "same family" may
have to be a relation on pairs rather than a partition. I did not check transitivity and it is the obvious
next measurement.

**What it costs a consumer.** Whether an expression compiles depends on a computed relation rather than on
two names, so the diagnostic has to explain a computation. That is harder to write well and it is more
honest, since it explains the actual obstruction.

### 7.3 H: neither operation may have a caller, and this one narrows everything

**The shape.** Section 1.1 asks whether the meet has a caller. Push the same question at the join and the
answer is less obvious than it looks, because the operations the design actually names do not compute joins.

`SETTLED_laws.md:165-178` gives `mul_full` as a family of maps $N_1 \times N_2 \to \mathrm{mulnum}(N_1,N_2)$,
and `mulnum` is a **formula** over the operands' members. For fixed-point the product's width is the sum of
the widths, which is not the join of the operands: the join of $U\langle I_1,F_1\rangle$ and
$U\langle I_2,F_2\rangle$ is at $(\max I, \max F)$ and the product is at $(I_1+I_2, F_1+F_2)$. Different
numerals, and only one of them is what multiplication needs. The additive story is the same shape:
`SETTLED_laws.md:138-146` gives closure conditions under which a numeral's own value set is closed under
addition, so the result numeral is the operand's own and no common target is computed at all.

So the operations named in the record derive their result numerals by formula, and a formula is not a
lattice operation.

**Where a join would then genuinely be called.** One place: an expression whose operands are numerals the
consumer did not unify, where the design chooses to infer a common target rather than refuse. That is
exactly the case reading C refuses by rule. **So the lattice structure is load-bearing only to the extent
that the design promises to infer a common numeral the consumer did not write.**

**And if it promises nothing there, the structure question dissolves into a predicate question.** What
survives is the inclusion order itself, which answers "is this conversion lossless" and needs a predicate
rather than a lattice, and which two experts have already agreed on in four-condition form
(`SETTLED.md:118`). Meets and joins would be facts about that order rather than operations the design
performs, and their totality would be a curiosity.

**Why this is a category and not a quibble.** It changes what op is choosing between. Under readings A, B
and C he is choosing a structure. Under this one he is first choosing **whether the design infers unwritten
targets at all**, and the structure question is downstream of that answer and possibly empty.

**Bounded, and the bound is the same one as section 1.1.** I did not read the closed panel's tree. A caller
for either operation may exist there. What I can say is that in `SETTLED.md`, `SETTLED_laws.md`, `DROPLIST.md`
and `02_carried`, meets and joins appear only as measured properties of the order and never as operations
invoked by anything. If that survives a grep by someone licensed to run it, this category outranks the other
four.

### 7.4 F: the ambient-and-realisation decomposition, which is not a fourth answer

**The shape.** The canon defines meet and join in the ambient lattice of finite rational sets, where they
are total and free, and defines a partial `realise` from a value set to a numeral. Every operation is then
the composite, and every structural failure in the design has one address: realisation.

**Why this is worth writing down even though it decides nothing.** It changes what a canon sentence
quantifies over. Under readings A, B and C the canon says which operations are total, and adding a family
later invalidates that sentence. Under this decomposition the canon says the composite is defined exactly
where the ambient result is realisable, and adding a family later adds a **theorem** about where
realisability holds rather than invalidating a sentence. That is the permanence test coming out clearly
different for one of them.

**Why it is not a fourth answer.** It does not decide whether the singleton is realisable, which is op's
question in different words. It relocates the question rather than resolving it, and a reader who took it
for an answer would find the same call in front of them one layer down.

**What it costs.** A concept consumers never see enters the canon. And it is only honest if the ambient
object genuinely has no type-level existence, which it does not under the erasure gate, so the canon has to
be explicit that the ambient is a specification device and never a thing a consumer holds.

### 7.5 G: order the numerals by something other than inclusion, and this route is closed

**The shape.** If the lattice fails under inclusion, use a different order: embeddability up to rounding, or
refinement alone, or reach alone.

**Closed, and here is what closes it.** Refinement alone and reach alone are each a total order per family
and each a lattice, but neither is the relation any operation needs: an operation's result numeral has to
hold the operands' **values**, and a relation that ignores range or ignores grid does not say that. And
inclusion-up-to-rounding is not an order at all in the direction wanted, because rounding is not injective,
so the antisymmetry that would make it a partial order fails: two distinct numerals can each round-trip into
the other with loss, and the quotient that repairs it identifies numerals denoting different value sets,
which is the thing `SETTLED.md:122`'s quotient exists to avoid.

I spent time on this before checking it and record it so nobody repeats it: **the order is not the free
parameter.** `02_carried` section 1.2's antisymmetry argument is the same move in a different place, and the
lesson generalises. Weakening the order to rescue a structure rescues it into a different question.

## 8. What each reading looks like in one table

Comparison, not a ranking. Every cell is sourced above.

| | A, closed | B, not closed | C, several | D, tie-break | E, step-set seam |
|---|---|---|---|---|---|
| Join inside a uniform family, one radix | total | total | total | total | total |
| Meet inside a uniform family, one radix | total | partial, F1 | total | total | total |
| Join across kinds | **not delivered**, F2 stands | partial, F2 | refused by rule | total by policy | refused by rule |
| Shapes that must be admitted | origin, negative width, plus empty if bias | none | none | none | none |
| Rules that must be stated | derived numeral's sign domain; negative-width floor | the refusal's diagnostic | the family relation; the cross-family rule | the tie-break | the nesting relation |
| Predicate amendment owed | yes | yes | yes | yes | yes |
| Consumer sees | silent degenerate results | some refusals | uniform refusals | policy-chosen results | computed refusals |
| Survives adding a family later | no, sentence invalidated | yes | yes, list grows | yes if antichain stays narrow | yes, relation unchanged |

The last row is the permanence test and it is the one that separates them most and is discussed least in the
material I read.

The table has no column for section 7.3, deliberately. That reading does not sit beside the others, it sits
above them: if neither operation has a caller, every column here is a column about properties nothing
invokes, and the table is a description of the order rather than a comparison of designs.

## 9. Coverage, stated honestly, and what I could not settle

**What I read.** Everything the brief named, in full. Nothing else in the panel, and nothing in the closed
panel's tree, which the brief forbids.

**What that costs, named because it is the largest weakness in this file.** Every statement I make about
`145`, `146`, `148` and `150` is a statement about `SETTLED.md`, `SETTLED_laws.md` and `02_carried`
reporting them. In particular **I have not read `150`, and section 3.2 disagrees with the summary of it that
I was given.** `SETTLED_laws.md:278-297` reports `150`'s closure conditions as making meets "land back on
the design's own two curves"; my instruments find the order-theoretic meet present for every pair in the
unsigned family with **or without** those conditions, and find the conditions doing two different and
narrower jobs (the origin for existence, negative width for exactness). The most likely explanation, and it
is a guess I am marking as one, is that `150` computes the meet **coordinatewise on $(F, -L, G)$** and asks
whether that triple names a shape, which is a different question from whether a greatest lower bound exists.
`i2.out` Q10 measures the gap between those two questions directly: over 351 pairs, the componentwise triple
names a shape 155 times and names none 196 times, while the greatest lower bound exists all 351 times. **If
that is what happened, the closure conditions are conditions on a formula rather than on the meet**, and the
consequence lists change accordingly. Someone licensed to open `150` should check this before anything is
built on either reading of it.

**The 81-versus-zero discrepancy** that `SETTLED.md:143-146` names as unresolved is untouched here. My
instruments are about the shape spaces, not about that slice, and I did not build a third instrument for it.
It stands where it was.

**Whether the meet has a caller** is the highest-value open thing this file produces and I could not settle
it from what I read. Section 1.1.

**Whether the nesting relation is transitive**, which reading E needs and which I did not measure.

**Whether the antichain stays at width 2** once decimal numerals or several float radices are present.
Measured stable at 2 across three box sizes and two shape-set enlargements, not measured beyond that.

**Whether a computed family relation is expressible in the typestate** under the forbidden-feature set.
Reading E needs it and I wrote no probe for it. `a-refused-bound-wants-a-trait-not-a-feature` suggests the
shape it would take, and suggesting is not compiling.

**Everything here is unpriced.** No harness bench bears on any of it, the Rust instrument is a compile and
run check rather than a measurement, and the Python instruments are enumerations. Every number above is a
count produced by a named command, and none of them is a magnitude.

**Routes closed, with what closed each.** Ordering the numerals by something other than inclusion, closed by
the antisymmetry and injectivity arguments in section 7.4. Reading A delivering totality across kinds by its
two named admissions, closed by the count argument in section 3.2 and by three instruments producing the
same two minimal upper bounds. The radix as the family seam, closed by radix 2 against radix 4 showing zero
structural failures in `i4.out` Q17. My own first instrument's Q1, closed by its own successor and kept in
place as the record of a setup that helps.

## 10. What appears to be op's, and in what order

Stated as questions rather than as a recommendation, per `00_brief.md` and `04`.

**One, and it is prior to the three readings: does either operation have a caller?** If the meet does not,
reading A's admissions are cost with no benefit and reading B's partiality is invisible. If the join does not
either, which section 7.3 argues is live because the design's named operations derive result numerals by
formula rather than by least upper bound, then the structure question is downstream of a different question:
**does the design infer a common numeral the consumer did not write?** That is a question about the operation
list rather than about the order, and it is cheap to answer.

**Two: is the promise attached to reading A the promise he was choosing?** The reading as offered says both
operations become total. What the record supports is that both become total **within one radix, at zero
bias, in one uniform-grid family**. Across kinds the join stays broken in a way no admission repairs. If
that reading of it is right, the choice is not between one family and several, it is between "one narrow
family plus a rule for everything outside it" and "several families plus a rule for everything between
them", which are closer together than they look.

**Three: the predicate amendment, in one of the two shapes in section 6.** Owed under every reading, cheap
under every reading, and its two spellings differ in whether the case analysis lives in the predicate or in
the definition of the order.

**Four: whether the tie-break reading is on the table at all.** It makes the family question stop deciding
totality, at the price of an answer that is chosen rather than forced, and the price looks small because the
antichain is two wide. Whether "chosen rather than forced" is acceptable in this design is a question only
he can answer, and it is the one place tonight where a persona steer would be worth having on the record.

**Five: whether the seam is the step set.** The measurements point at it and it is a reframing of his own
question rather than an answer to it, so it is his to accept or refuse before anyone builds on it.

**Owed under the two-expert rule, listed so nothing here is mistaken for agreed:** every section of this
file is a first read. Section 3.2's finding that reading A's admissions do not close the cross-kind join,
section 1.1's claim that the meet has no caller, section 1.2's three failure modes, section 6.1's claim that
the amendment is owed under every reading, section 7.1's antichain width, section 7.2's step-set seam, section 7.3's claim that the named
operations derive by formula rather than by join, and section 9's reading of what `150` probably measured. Not one of them has a second read, and per `04` none of
them would settle anything even if it did.
