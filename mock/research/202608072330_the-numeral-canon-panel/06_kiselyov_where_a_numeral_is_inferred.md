# 06. Where a numeral is inferred, if anywhere

**Date:** 2026-08-08
**Position:** tests `03`'s option H, which observed in passing that the design's named operations derive
their result numerals by formula rather than by least upper bound, and that the lattice apparatus may
therefore have no caller.
**Probes:** `06_probes/`, seven instruments in two languages by three independent methods, with `RUN.md`
carrying every command, its exit code, and the two instruments that were killed for cost and kept as
controls.
**Reading:** `RULES.md`, `01_op_answers.md`, `04_op_no_settlements_tonight.md`,
`03_lamport_the_family_question_and_its_consequences.md`, `05_dispatcher_note_two_meet_questions.md`,
`SETTLED.md`, `DROPLIST.md`, `PERSONA_CALLS.md`, all in full and in that order. `seed/SETTLED_laws.md` in
full, because `03`'s option H cites it and the brief asks me to check that citation rather than inherit it.
`CANON_CANDIDATE.md` grepped and read in four passages as a starting text to re-derive from; it is never
cited below and every claim it suggested is re-established from arithmetic or from a probe. The closed
panel's tree was not opened.
**Register:** nothing here settles anything, per `04`.

## What this file found, stated before the argument

**Option H's core observation survives and its framing does not.** The product numeral is not the join:
measured at 1175 of 1296 operand pairs, with the 121 coincidences all in the corner where each coordinate
has a zero (`p1.out` Q3). So the thing `03` noticed is real. But the line it draws is in the wrong place.
**Within one family the join is a formula too.** Coordinatewise maximum is a total function of the
declared members, it needs no search, it cannot fail, and it compiles to the same kind of associated-type
fold as the sum does, gate-free, erasing to one constant (`p3`). So "formula, not lattice operation" is not
a distinction that separates anything, because the lattice operation *is* a formula here.

**The distinction that does separate them is who determines the answer.** Four buckets: the consumer
determines it, the operands determine it, the design determines it by a named rule, or nothing determines
it. **The fourth bucket is empty, and coherence says it must be empty.** An associated type names exactly
one type; where the order has two incomparable minimal upper bounds there is no way to name both, and
declaring the two honest readings side by side is refused at `E0119` (`p4_arm3.out`). So a design that
infers a target is not computing a join and could not be, whatever the order does. It is picking, in one
impl, and the only question is what the pick is a function of.

**No site I found needs an extremum.** Every place the design produces a numeral the consumer did not
write is a total function of declared members, a minimum along a chain, a named rule over a small closed
set, or a constant default. Section 2 gives the enumeration and section 4.1 gives the three candidates that
looked like counterexamples and are not.

**The order has a caller; the lattice has a reader.** The order is called at compile time, on a pair, as a
predicate: is this conversion lossless, does this accumulator hold what the fold can produce, does this
formula's answer contain the values. The lattice is quoted at proof time, in sentences about whether a
formula's answer is the *tightest* one. Those are different jobs and only one of them is an operation.

**And the tightness sentence is not currently true.** The natural closed forms overshoot. The
sum-of-widths product form is tight at 6100 of 6561 pairs and wastes exactly one bit on the other 461,
and the waste fires on an exactly characterised region: whenever the narrower operand's **total** width is
1 (`p5.out` Q1, Q2, Q3). I derived a tight form, it agrees at 6546 of 6561, and **the residual 15 are
exactly the pairs whose tight answer needs negative integer width.**

That last number is the one I would most want a second read on, because it contradicts a downstream
inference of `03`. `03` section 3.1 places negative integer width at the meet, and section 4.4 concludes
that if the meet has no caller then reading A's admissions are pure cost. **Negative integer width has a
caller at multiplication**, which is the operation with the least ambiguous caller in the design, and it
arrives there with no lattice anywhere near it.

## 0. Gates

### 0.1 The canon gate

There is no ratified canon for arvo; this panel is writing the first one, and `01` section 0 carries op's
correction that the rows in `SETTLED.md` marked RATIFIED were classified under a superseded reading. So
the defend-the-canon posture has no target and the governing material is the narrow set recording op in
the loop.

Nothing below asks for anything that material forbids, and one row bears on it directly rather than
tangentially. The acceptance criterion quoted at `SETTLED.md:65-71` requires that the typestate "derive
the matching container and numeral representations, then validate, and erase on lowering". **That
sentence is itself an inference site**, and it is the largest one in the design. This file treats it as
such rather than as background, which is section 2.3. Gate passed.

### 0.2 The test gate

Re-run rather than inherited from `03`, and the numbers agree with it, which is worth recording as an
independent arrival rather than as a repetition:

    $ find mock/crates -path '*tests*' -name '*.rs' | wc -l
          91
    $ grep -rl '#\[test\]' mock/crates --include='*.rs' | wc -l
          83
    $ grep -rn 'mulnum\|mul_full\|MulNum' mock/crates --include='*.rs' | wc -l
           0
    $ grep -rn 'fn join\|fn meet\|LeastUpper\|GreatestLower' mock/crates --include='*.rs' | wc -l
           0
    $ grep -rn 'Growth' mock/crates --include='*.rs' | wc -l
           0

So there is no suite to audit for this question: the surface this file is about has no test because it has
no source. I did not run the suite and I am saying that rather than implying it passed. The brief
separately declares `mock/crates` nuked and forbids citing it as evidence about what is correct; the three
zeros above are measurements of absence, which is the one thing a nuked tree can honestly report.

### 0.3 The brief's cheap factual claims, and one of `03`'s

The pin is as stated. `rust-toolchain.toml` carries `channel = "nightly-2026-05-28"` and
`rustc +nightly-2026-05-28 --version` reports `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.

**`03`'s option H cites `SETTLED_laws.md:165-178`, and that citation is half right in a way that matters.**
The passage says exactly what `03` says it says about the map's shape: "`mul_full` is a family of maps
`N1 x N2 -> mulnum(N1, N2)`, not an operation on one set" (`SETTLED_laws.md:167`). It says **nothing**
about how `mulnum` computes its answer. The word "formula" is `03`'s, and so is the sum-of-widths
arithmetic:

    $ grep -rn "mulnum\|mul_full" seed/ *.md | grep -v CANON_CANDIDATE
    seed/SETTLED_laws.md:145 ... the derived reason multiplication needs `mul_full` and addition does not.
    seed/SETTLED_laws.md:165 ### `mul_full` associativity does not typecheck without `mulnum` ...
    seed/SETTLED_laws.md:167 **Claim.** `mul_full` is a family of maps `N1 x N2 -> mulnum(N1, N2)` ...
    (the remaining five hits are all inside `03` itself)

    $ grep -rn "sum of the widths\|I1+I2\|F1+F2" . 
    03_lamport...:625  (one hit, in `03`)

So the record carries the *shape* of `mulnum` and not its *content*, and `03` supplied the content from
ordinary fixed-point arithmetic without saying so. That is not a defect in `03`'s reasoning, because the
arithmetic is right and I verified it. It is a defect in the citation, and it matters because the sum
turns out **not to be the tight answer** (section 7), which is a thing no reader would go looking for
while believing the sum was on the record.

**A second thing `03` did not have.** The claim it makes in passing already exists in the canon candidate
in a stronger and more general form, as a structural theorem quantifying over every operation rather than
over the thirteen it names. I will not cite the candidate, per the brief, so I state it as re-derived and
attack it on its own terms in section 4.4. That it was already there is worth knowing regardless: `03`'s
option H is a rediscovery, not a discovery, and two independent arrivals at the same observation is worth
more than one.

## 1. The carve I would use, and why not the brief's

The brief offers four classes: computed by a formula over the operands, selected as an extremum in an
order, chosen by a named rule, or written by the consumer. That carve does not survive contact with the
measurement, for one reason: **an extremum in an order can be a formula, and here it always is.**

Within the unsigned fixed-point family at one radix with zero bias, inclusion is the componentwise order
on `(I, F)`. I verified that rather than assuming it, by comparing set inclusion against the coordinate
order over a box and finding zero disagreements (`p1.out` Q0). So the join is coordinatewise maximum, and
coordinatewise maximum is a total function of four declared numbers. It has no search in it. Calling it
an extremum and the sum a formula puts two members of the same class on opposite sides of the line.

The carve that does the work is **what determines the answer**:

> **D0, the consumer determines it.** The result numeral is one the consumer wrote, either because it is
> an operand's own numeral or because the operation takes its target as a declaration.
>
> **D1, the operands determine it.** The result numeral is the image of a total function of the operands'
> declared members. Whether that function happens to be a lattice operation is a separate fact about it.
>
> **D2, the design determines it.** A constant (a default), or a choice among finitely many candidates
> made by a stated rule keyed on something other than the members.
>
> **D3, nothing determines it.** A search over candidates that may find none, or may find several with no
> way to prefer one.

The three questions the brief asks map onto this cleanly. "Does any site need an order" is "is any site in
D3". "What does an order buy where it has no caller" is "what work does the order do for D0, D1 and D2
sites". And the answer to the first, developed below, is that **D3 is empty and cannot be otherwise**,
which is a stronger statement than "no site happens to need one".

### 1.1 Why D3 cannot be inhabited, which is the load-bearing point

A design that infers a result numeral must do it somewhere, and the somewhere is an associated type on a
trait impl. **An associated type names exactly one type.** So a site cannot be in D3 and be implemented at
the same time: implementing it forces a choice, and the choice is then a fact about the impl rather than
about the order.

That is not an argument from taste, and it is checkable. `p4` declares the two honest readings of a
cross-family join side by side, exactly the pair `03` section 3.2 measures as the antichain of width two,
and the compiler refuses:

    error[E0119]: conflicting implementations of trait `JoinNum<Flt<_, _, _>>` for type `Uni<_, _>`
       --> p4_cross_family_join.rs:190:5
        |
    187 |     impl<...> JoinNum<Flt<P2, ELo, EHi>> for Uni<I1, F1> {
        |     ------- first implementation here
    190 |     impl<...> JoinNum<Flt<P2, ELo, EHi>> for Uni<I1, F1> {
        |     ^^^^^^^ conflicting implementation for `Uni<_, _>`

**A contract that will not compile is the result.** The type system is stating that "the set of minimal
upper bounds" has no expressible form as a projection, and the design must therefore name one answer, at
the impl, on grounds the order does not supply. That is a fact about the mechanism rather than about the
numerals, and it holds however the family question is answered.

Two consequences worth separating, because collapsing them is how the apparatus acquired the appearance of
necessity:

- **The lattice's totality was never what made the design implementable.** The design was always going to
  name one answer per impl.
- **The lattice's totality is what would make the named answer *canonical* rather than *chosen*.** Where a
  least upper bound exists and the impl names it, the impl is reporting a fact; where none exists and the
  impl names something, the impl is exercising a policy. Both compile identically. Only the canon can tell
  them apart, and only if it has the order.

That second bullet is section 5 in one line.

### 1.2 The same compiler fact, twice, and the second use is not the first

The candidate text uses `E0119` for a different purpose: to establish that no operation's growth can be
policy-dependent, because two impls disagreeing on the answer for one generic domain cannot coexist. I use
it to establish that an antichain has no representation. These are the same mechanism reaching two
conclusions, and they compose rather than duplicate: the first says the answer cannot vary, the second
says there is exactly one answer to vary from. Together they say **every inference site in this design is
a total function into a single named type, by construction of the language rather than by design choice.**

I am flagging this as a convergence rather than as corroboration. Two conclusions drawn from one compiler
diagnostic by two authors is one instance of evidence wearing two hats, which `RULES.md` warns about
directly.

## 2. The sites: every place a numeral appears that the consumer did not spell

Enumerated by walking the design's own operation surface rather than by recalling one. The operation
inventory below is **re-derived** from the canon candidate as a starting text, and it comes with a warning
attached to it in that text: the count that carried it was wrong at the source, with one file saying
eleven while its own probe declared ten growth traits over thirteen operations. So I treat the list as a
prompt for enumeration rather than as a fact, and every classification below is argued from what the
operation must do rather than from the list's authority.

Twenty sites, of which the last four are the second pass: the first enumeration went by binary arithmetic
and missed the unary operations, exact scaling, and one site that could exist and does not. **Eight turn
out not to be sites at all**, which is a result rather than an absence of one: a site that is
consumer-written narrows the map more than a site that is inferred.

| # | Site | What the consumer writes | What the design produces | Class |
|---|---|---|---|---|
| 1 | Container selection | the usage, in bits and bytes | the container | D1 then D2 |
| 2 | Stored width and payload shape | the usage and the strategy | the representation | D1 |
| 3 | `mul_full`'s result numeral | the two operands | the product numeral | D1 |
| 4 | `mulnum` over a ranged numeral | the two operands | exponent and bound sums | D1 |
| 5 | in-numeral `add`, `sub`, `mul`, `div` | the operand's numeral | nothing new | **D0** |
| 6 | `div_exact`, `div_floor`, `rem` | the operands | nothing new, or a refusal | **D0** |
| 7 | `quantize`'s target | the target numeral | nothing new | **D0** |
| 8 | Narrowing's target | the target numeral | nothing new | **D0** |
| 9 | A conversion's target | the target numeral | nothing new | **D0** |
| 10 | A fold's accumulator | the accumulator numeral | a sufficiency verdict | **D0** plus a check |
| 11 | A fold's per-element product type | nothing | the product numeral | D1, same as 3 |
| 12 | A typed constant's numeral | the context's numeral | nothing new | **D0** |
| 13 | Alias-tier omissions (`UInt<5>`) | five bits | fraction width, sign, strategy | D2, constants |
| 14 | `Resolve` on two strategies | two strategies | one strategy | D2, adjacent axis |
| 15 | A bitfield's parent container | the field widths | the parent | D1 |
| 16 | A mixed-numeral `add` or `sub` | ? | ? | **not found** |
| 17 | `Abs` on a signed numeral | the operand | an unsigned numeral of the same widths | D1 |
| 18 | `Sqrt`, `Recip` | the operand, and a target | nothing new, or a refusal | **D0** |
| 19 | Exact scaling by a power of the radix | the operand and the exponent | the shifted numeral | D1 |
| 20 | A numeral inferred from a literal's magnitude | (does not exist) | (nothing) | **absent** |

### 2.1 The eight that are not sites, and why that is the useful half

**Sites five, six, seven, eight, nine, twelve and eighteen all take their target from the consumer**, and
site ten does too with a check attached. In-numeral arithmetic returns the operand's own numeral, which
the consumer wrote; the quantiser, narrowing and conversion all take a declared target, which `03`
section 1 already establishes and which I am confirming rather than re-deriving; a typed constant is
built into the numeral its context names; `sqrt` and `recip` leave the family entirely and so must be
given somewhere to land.

That is eight of twenty sites where the answer to "where does the design produce a numeral the consumer
did not write" is **it does not**. The apparatus has no purchase on any of them, because there is nothing to
infer. And the two that look most like arithmetic, in-numeral `add` and `mul`, are exactly the ones the
closure conditions at `SETTLED_laws.md:138-146` are about: a numeral whose value set is closed under
addition needs no result numeral computed at all, because the operand's own is the answer.

**Site ten is the one that repays attention.** A fold's accumulator cannot have its numeral grow per
iteration, which the droplist records as impossible in principle rather than merely unbuilt, since a type
cannot depend on a runtime value. The replacement it names is fixing the per-element type and checking
accumulator sufficiency at compile time. So the accumulator numeral is **consumer-written**, and the
design's contribution is a **verdict** rather than a numeral. That verdict is an inclusion test, and
inclusion is the order. **This is the clearest caller the order has anywhere in the design, and it is not
a lattice operation.**

### 2.2 Site sixteen is the gap, and naming it is most of what this section is for

I could not find, anywhere in `SETTLED.md`, `seed/SETTLED_laws.md`, `DROPLIST.md`, `02_carried` or `03`,
an operation that adds or subtracts values from two **different** numerals. The additive material is all
about closure of one numeral's own value set. The multiplicative material is the only place a two-numeral
map is named.

Two readings, and the design has to pick one because they are not the same design:

> **The design has no mixed-numeral addition.** A consumer with two numerals converts one into the other
> first, and conversions name their target. Then site sixteen does not exist, addition joins the D0 block,
> and the inference surface is smaller than anyone has been assuming.

> **The design has mixed-numeral addition and nobody wrote down its result numeral.** Then it is a D1 site
> whose formula is unstated, and section 7 says what that formula is and measures it.

**This is cheap to settle and nobody has settled it**, and it decides more than its size suggests, because
addition is the operation a consumer writes most and multiplication is the one the record documents. If
the answer is the first reading, then every remaining inference site is multiplicative plus the container,
and the whole family question narrows to what `mulnum` does on a mixed pair. Which is where `03` section 1
arrived from the other direction.

### 2.3 Site one, which is the largest and is not usually counted

The acceptance criterion at `SETTLED.md:65-71` has the typestate derive the container and the
representation from a usage expressed in bits and bytes. That is an inference, it is ratified, and it runs
on every declaration rather than on arithmetic. It is bigger than every other site combined by the count
of times it fires.

It is also the design's own worked example of how an inference site is built, and it is instructive
because **it is not one mechanism but two stacked**:

- A **D1 step**: from the usage, compute a required width. A total function of the declared members.
- A **D2 step**: from the required width and the strategy, pick a container. Along a chain of widths this
  is a minimum-above, which is again a formula. But the container axis is not a chain: a historical
  reading of the tree finds the wide payload parameterised on bytes **and** alignment
  (`mock/crates/arvo-storage/src/layout_assertions.rs:15`, quoted here as an observation about a tree the
  brief declares nuked and never as evidence about what is correct), and two coordinates give incomparable
  elements at equal width. **The strategy is what breaks that tie.**

So the design has already met an antichain, at the container, and already answered it: **a named rule
keyed on something outside the order.** That precedent is worth more than an argument, because it is the
same shape as the cross-family question and it was resolved without anybody reaching for a lattice.

### 2.4 The second pass: unary operations, scaling, and one site that does not exist

The first enumeration walked the binary operations and stopped, which is the shape of mistake a taxonomy
makes when its examples are all binary. Four more categories, and two of them are the cleanest instances
of their classes in the whole list.

**Unary operations split.** `Abs` on a signed numeral is a pure D1 formula: the result is unsigned at the
same widths, computed from the operand's members and nothing else. `Sqrt` and `Recip` are D0, because
neither is closed in any fixed-point family: a reciprocal leaves the range and a square root leaves the
grid, so both take a declared target and quantise into it. The record's own account of `sqrt`'s overflow
band being inhabited exactly on numerals with no representable multiplicative identity
(`SETTLED_laws.md:213-227`) is a statement about that quantisation rather than about a derived numeral,
which is consistent with putting `sqrt` in D0.

**Exact scaling by a power of the radix is the cleanest D1 site in the design.** Multiplying by `r^k` is
exact, and it moves the split without changing the total width: `U<I, F>` scaled by `r^k` is
`U<I+k, F-k>`. Nothing is gained, nothing is lost, and the formula is a translation along a line. It is
worth naming precisely because it is the one operation where the result numeral is obviously determined
and nobody would think to reach for an order.

**And one site that could exist and does not.** No mechanism infers a numeral from a literal's magnitude.
A design could have `3.5` pick `U<2,1>` on its own; this one does not, and a typed constant is built into
the numeral its context already names. That is a real finding under the brief's own terms, because a site
the design declines to have narrows the map exactly as much as one it has. It is also the site where an
inference would be least defensible, since the literal's magnitude says nothing about the range the
consumer intends to store in it.

## 3. Classification, and the one thing every class shares

Restating the sixteen against the four determinants of section 1:

- **D0, consumer determines it:** sites 5, 6, 7, 8, 9, 10, 12, 18. Eight sites, and site 10 additionally
  owes a check.
- **D1, operands determine it:** sites 1 (first step), 2, 3, 4, 11, 15, 17, 19. Eight sites, all total
  functions of declared members.
- **D2, design determines it:** sites 1 (second step), 13, 14. Three sites, all named rules or constants
  over small closed sets.
- **D3, nothing determines it:** empty.
- **Not found:** site 16. **Absent by choice or by omission:** site 20.

**What every non-empty class shares is that the answer is a function.** D0's function is a projection (take
the target the consumer named). D1's is arithmetic over members. D2's is a lookup keyed on a marker. None
of the three has a search in it, none can fail to produce an answer, and none can produce two.

That is not a coincidence and section 1.1 says why: an associated type names one type, so anything
implementable is a function. **The apparatus was never deciding whether the design could work. It was
deciding whether the design's answers are the right ones**, which is a different question with a different
instrument.

## 4. Does any site need an order?

**No site needs an extremum**, on the reading of "need" that matters: no site is one where the design must
pick among candidates none of which is determined by a formula.

### 4.1 The three candidates that looked like they might, checked

**The join of two operands, as a result numeral.** No operation calls it. Multiplication's answer is not
it (1175 of 1296 pairs differ, `p1.out` Q3). Addition's answer is not it either, when addition is
mixed: the sum numeral needs one more integer bit than the join, and my instrument measures the
coordinatewise join agreeing with the additive formula at **0 of 1296** pairs (`p1.out` Q4). Subtraction's
candidate does coincide with the join at all 1296, and section 7.2 shows that candidate is simply wrong
and the corrected one does not.

So the join's score as a result numeral, across the sites that have one, is: multiplication 121 of 1296
and those all in the degenerate corner, addition 0 of 1296, subtraction 1296 of 1296 for a formula that
overshoots 545 times and is superseded. **The join is not what any arithmetic site wants.**

**The meet.** `03` section 1.1 looked for a caller and found none. I looked again with a different
question, asking not "is it invoked" but "is there any site whose answer is a greatest lower bound", and
found none either. An interchange numeral guaranteed lossless in both directions would be one, and the
design has no such operation. **Two independent looks, no caller.** That is two arrivals at a negative,
which is worth exactly as much as two arrivals at a negative ever are: it raises the cost of the claim
being wrong without establishing it.

**A common target across families.** This is the only real candidate and it dies at `E0119` rather than at
the order. Section 1.1.

### 4.2 What this does to the open structure question

`SETTLED.md:138-142` puts the family question as op's, and `01` section 1 records his call to lay the
consequences out first. On the reading developed here, the question keeps its importance and **changes
what it is about.**

It stops being "do the numerals form a lattice", because no operation asks. It becomes:

> **Is the admitted shape space closed under the clamp of every formula's answer?**

That is narrow, it is testable, and I tested one instance of it. The sum-of-widths product form always
lands on an admitted shape, so it is closed. **The tight product form does not**: at 15 of 6561 pairs its
answer has negative integer width (`p6.out`), and the design either admits that shape, or clamps upward
and gives up tightness, or refuses.

So the two admissions op was offered in the family question are not, on this reading, about closing a
lattice for an operation nothing invokes. **They are about whether a formula is allowed to be tight.**
That is a much more concrete thing to rule on, and it is the same ruling.

And the clamp itself is worth one sentence, because it is where D1 comes closest to D3: clamping a
formula's answer into the admitted space is unique only if the admitted space is closed under
coordinatewise operations. In a product of chains it is, so the clamp is a formula and the site stays in
D1. **In a space that is not a product of chains, the clamp is a search and the site falls into D3, where
`E0119` then refuses to implement it.** That is the sharpest statement I have of why the family question
is load-bearing at all, and it is not the reason the record gives.

### 4.3 What the classification costs when a family is added, which is the permanence test

`03`'s comparison table has a row for whether each reading survives a family being added later, and calls
it the row that separates the readings most and is discussed least. The D-classification gives it a
sharper answer than "yes" or "no", because the three classes scale differently and the difference is
countable.

- **D0 sites cost nothing.** The consumer names the target, so a new family adds no obligation. Eight
  sites, unaffected.
- **D1 sites cost quadratically.** A formula is a function of two operands' members, and members of
  different families are different coordinates, so each unordered pair of families needs its own formula.
  With `k` families that is `k(k+1)/2` formulas per D1 site, and there are eight D1 sites.
- **D2 sites cost linearly.** A named rule keyed on the family gains one row.

So the arithmetic is: adding one family to a design with `k` of them adds `8k` or so formulas at the D1
sites and `k` rows at the D2 sites. **That is the real price of the family question**, and it is a price
on the operation surface rather than on the order.

Two consequences worth stating, and both cut against where the record's effort has gone.

**It argues for pushing cross-family answers into D2.** A rule that refuses, or a rule that names a
preferred kind, is one line per family. A formula per pair of families is eight. `03`'s reading C (refuse
across families) and reading D (a stated tie-break) are both D2 answers and both are cheap under this
count; reading A's totality, if it were achievable across kinds, would be the expensive one, and section
3.2 of `03` measures that it is not achievable anyway.

**And it argues for `03`'s reading E more strongly than `03` does.** If the family relation is computed
from the members rather than declared, the D1 formulas are written once against the relation instead of
once per pair of families, and the quadratic collapses. `03` presents reading E as buying honesty in the
diagnostic. On this count it also buys the only escape from the quadratic, which is a larger claim than
the one it was offered under. **Untested**: whether the relation is expressible in the typestate is the
thing `03` names as needing a probe and neither of us has written one.

### 4.4 The register on that, kept honest

*This appears to hold*, over one family, at one radix, with zero bias, for four sites. I did not test the
ranged family, nonzero bias, the closed-interval adjustment the record names for normalised channels, or
any radix but two. Section 9 lists that as the largest hole in this file.

## 5. What an order buys where it has no caller

The brief asks this carefully, and it is right to, because conflating a use with a caller is how the
apparatus acquired the appearance of necessity. Six uses, and **two of them are genuine callers**, which
is a correction to the framing rather than an answer inside it.

### 5.1 Two genuine callers, neither of them a lattice operation

**The lossless-conversion predicate.** Every D0 site takes a declared target, and the design must say
whether reaching it loses values. That is exactly the four-condition inclusion test, recorded at
`SETTLED.md:118` at TWO EXPERTS, and it is called at compile time on an ordered pair. **The order has a
caller and it has had one all along.** What it does not have is a caller for its extrema.

**The accumulator sufficiency check.** Site 10. The consumer declares the accumulator, the design decides
whether it holds what the fold can produce. Another inclusion test on a pair.

Both are the order used as a **relation**. Neither asks for a bound, least or greatest. So the honest
statement is not "the order has no caller"; it is:

> The order is called, as a two-place predicate, at every conversion and every sufficiency check. Its
> extrema are called nowhere.

That distinction is worth keeping in the canon's own words, because a sentence saying the numerals form a
lattice will be read as a claim about what the design computes, and it is not one.

### 5.2 Four uses that are not callers

**Justification of every D1 formula.** `mulnum`'s answer is correct exactly because the product numeral
contains every product, which is an inclusion statement about value sets. Without the order there is no
way to say the formula is right, and with it, correctness is a theorem proved once rather than a check run
per compilation. This is the most important of the four and it is invisible in a caller census.

**Tightness.** "The derived numeral is the tightest honest answer" is a least-upper-bound claim. It is
quantified over at proof time, not computed. And section 7 measures it false as currently stated, which is
the strongest possible argument that the canon needs the join available as a **statement** even though
nothing invokes it: without it, nobody would have thought to check.

**The quotient, and antisymmetry with it.** `SETTLED.md:122` records that the order is on value sets, so
two numerals denoting the same values are one point. That is what makes "the result numeral" well defined
rather than "a result numeral", and it is upstream of every D1 formula being a function at all. `03`
section 7.5 closes the route of weakening the order and its argument applies here: weaken the order and
the formulas stop being functions into a well-defined codomain.

**The diagnostic.** When a D2 rule refuses, the order is what lets the message say why. "No numeral in
this design holds every value both of these can" is an order-level sentence. Without it the diagnostic can
only say that the two differ, which tells a consumer nothing they did not type.

### 5.3 The shape of the answer, in one sentence

**The order is infrastructure and the lattice is a theorem about it.** The design calls the relation, the
canon quotes the extrema, and the two are not in competition for the same budget. A canon that states the
order in full and states the lattice as a property of it, scoped to where it holds, is saying exactly what
is true and is not paying for a mechanism.

## 6. Does option H survive?

**Partly, and the part that survives is not the part it leads with.**

**Survives.** The design's named result-numeral operations are not lattice joins. Measured for the one
operation the record documents (1175 of 1296, `p1.out` Q3), argued for the additive one and measured at 0
of 1296 agreement (`p1.out` Q4), and confirmed for the remainder by their being D0 sites with no derived
numeral at all. `03`'s downstream inference from this also survives and is strengthened: **the structure
question is downstream of whether the design infers unwritten targets**, and the enumeration in section 2
says it infers far fewer than the record's framing implies.

**Does not survive.** "A formula is not a lattice operation." Within a family the join *is* a formula, the
two are the same kind of object, and `p3` shows they cost the same to express: both are gate-free
associated-type folds on the pinned nightly, both fold to one constant. `03` uses the contrast to argue
that the lattice has no caller, and the contrast does not carry the argument. **The argument survives on
different grounds**, which is section 1.1's coherence result: an implementable inference site is a
function whether or not the order is a lattice, so the lattice's presence or absence changes nothing about
what can be built.

**Corrected in a way that matters downstream.** `03` section 4.4 reasons that if the meet has no caller
then reading B costs nothing and reading A's admissions are pure cost. The first half stands. The second
does not, because negative integer width has a second potential caller at multiplication, in the corner
measured at `p6.out`. So the admissions are not pure cost even under the reading that kills the meet.

**Neither confirmed nor refuted, and it is the biggest thing left.** Whether the design has a
mixed-numeral addition at all. Section 2.2.

### 6.1 An honest note on convergence

`03` and I agree on the core observation. Under `RULES.md` that agreement is worth less than it looks:
`03` is my immediate predecessor and I read it before deriving, so this is inherited rather than found,
and the rung is ONE EXPERT with a second read still owed. What is genuinely independent is the
measurement, because `03` did not measure any of the arithmetic sites and its option H carries no
instrument at all.

## 7. Two things nobody asked for, found while checking

Reported because the brief's standing instruction is to name unlicensed findings, and because the second
one bears directly on op's open question.

### 7.1 The sum-of-widths product form wastes a bit, and the region is exact

The natural product formula, sum the integer widths and sum the fraction widths, is **not** the least
numeral containing every product. Over a box of 81 shapes and 6561 pairs it is tight at 6100 and wastes
exactly one bit of total width on 461 (`p5.out`). The waste region, characterised exactly rather than
described (`p6.out`):

- 160 pairs where one operand denotes only zero. Degenerate, and arguably not a case.
- 301 pairs where the narrower operand's **total** width is 1, meaning it denotes exactly two values.
- 16 pairs where the clamp hides the waste, which are the 15 negative-width pairs plus the doubly
  degenerate one.

The mechanism is one line. A numeral's reach is `2^I - 2^-F`, strictly below its own ceiling `2^I`. The
sum-of-widths form prices each operand at its ceiling, so when both reaches fall far enough below, the
product's maximum lands a whole binade below the formula's, and one integer bit is provably dead.

**The tight form, derived and tested.** Write `W = I + F`. The product set's step is exactly `2^-(F1+F2)`,
because that value is itself a product of the two steps, so the fraction width is `F1 + F2` and never
more. The total width is the least `W` with `2^W - 1 >= (2^W1 - 1)(2^W2 - 1)`, which resolves to

$$W_{\text{out}} = \begin{cases} W_1 + W_2 - 1 & \text{if } 2^{W_1} + 2^{W_2} - 2 \ge 2^{W_1 + W_2 - 1}\\ W_1 + W_2 & \text{otherwise}\end{cases}$$

and the integer width is `W_out - (F1 + F2)`. It agrees with the least containing shape at 6546 of 6561
(`p5.out` Q1). **The whole correction is one comparison on the two total widths and it never looks at the
integer-fraction split**, which is worth noting because the total width is a quantity the container
derivation already needs.

**And the saving predicate is simpler than the formula.** Measured: it depends only on the two total
widths, and it fires exactly when the narrower operand's total width is 1, at every width of the wider one
(`p5.out` Q2, Q3). So the tight form is the sum-of-widths form minus one bit when either operand holds at
most two values.

**What it costs to leave alone, stated structurally because it is unpriced.** The excess is one bit of
total width, so it crosses a container boundary exactly when the tight total is a power of two. Thirty
eight such crossings in this box, for instance `U<0,1>` against `U<1,7>` giving a tight total of 8 bits
against the formula's 9 (`p5.out` Q4). Under the erasure gate that width is real at lowering, so a dead
bit at a boundary is a container jump. **Unpriced.** No bench harness run bears on what a container jump
costs, and the word is used deliberately rather than reaching for a number.

**The finding is a correctness one before it is a size one.** A canon sentence claiming the derived numeral
is the tightest honest answer would be **false** as the formula currently stands, at 461 of 6561 pairs.
Either the formula changes or the sentence does.

### 7.2 Negative integer width has a caller, and it is not the meet

The residual 15 pairs where my tight form disagrees with the least **admitted** shape are exactly the
pairs whose tight answer has negative integer width (`p6.out`). The region, stated exactly rather than
gestured at: **one operand is `U<0,1>` and both operands are purely fractional.** Both facts measured true
over the whole set, not sampled.

Worked, because a witness beats a count. `U<0,1>` denotes `{0, 1/2}`. Its square is `{0, 1/4}`: two
values, at step `1/4`. Two values at that step is total width 1 with `F = 2`, so `I = -1`. The least
**admitted** shape, with `I` floored at zero, is `U<0,2> = {0, 1/4, 1/2, 3/4}`, which holds four values
where two were needed.

This is the same shape `03` section 3.1 found at the meet, which it describes as restoring exactness
rather than existence, and it names `U<0,2>` against `S<0,2>` as its witness. **The mechanism is identical
and the caller is different.** `03` locates negative width only at an operation nothing invokes and then
reasons, at its section 4.4, that this makes reading A's admissions pure cost. That inference does not
follow once multiplication is on the list.

**The honest size of it.** A corner, not a region, and shrinking: 7 of 625 pairs at box 4, 11 of 2401 at
box 6, 15 of 6561 at box 8 (`p6.out`). The count grows linearly while the pairs grow quadratically. I am
stating that plainly rather than letting the finding's sharpness stand in for its size.

**What would make it larger, untested.** Signed numerals, where negative integer width is less exotic;
repeated multiplication, where the corner may compound; the closed-interval adjustment the record names
for normalised channels, rotor components and direction cosines, which is exactly the purely-fractional
population this corner lives in. Each is a measurement nobody has taken and the third is the one I would
take first.

### 7.3 The tight form is realisable, so this is a proposal rather than a complaint

A derived formula nobody can build is a wish. The predicate above looks like it
needs exponentials of type-level naturals, which would be a wall, and it is not one:
for widths at least one it is exactly `min(W1, W2) == 1`. Proved in `p7`'s header
in three lines, and independently measured over the whole box by `p6`.

So the tight product numeral is a sum, an equality test against one, and a conditional
decrement. `p7` builds precisely that, compiles it gate-free on the pinned nightly with
the default solver, asserts four cases against values `p5` computed in Python from exact
rational value sets, and erases to a single constant. **A negative control confirms the
assertions can fail**: forcing the tight answer to equal the naive one at the case where
they must differ fails at `E0080` with the control's own message (`p7_negctl.out`).

`p3` and `p7` together also settle the feasibility half of the whole question. The formula,
the coordinatewise join and the tight corrected formula are all ordinary associated-type
folds, all gate-free, all erasing. **Nothing in this area is blocked by the forbidden-feature
set**, which is worth stating positively because the record's own note on the exponent
positions reports every const route closed, and a reader could carry that forward as a
general wall. It is a wall for const arithmetic and not for this.

## 8. Routes closed, each with the thing that closed it

**Arguing option H on feasibility grounds.** Closed by `p3`. The formula and the extremum
compile the same way, cost the same gates, and erase the same. Anyone reaching for "a formula
is buildable and a join is not" should stop.

**Representing an antichain of minimal upper bounds.** Closed by `E0119` in `p4_arm3.out`.
Not "hard", not "expensive": no expressible form as a projection.

**Expecting the type system to constrain the cross-family answer.** Closed by `p4_arm2.out`,
which compiles. It accepts any cross-family formula written. Correctness is entirely the
order's problem and not the compiler's, which is the opposite of the within-family situation
where the compiler at least refuses ambiguity.

**Reading `SETTLED_laws.md:165-178` as stating what `mulnum` computes.** Closed by the grep
in section 0.3. It states the map's shape and not its content, and the five other hits in the
panel are all inside `03`.

**"The two admissions in reading A are pure cost if the meet has no caller."** Closed by
`p6.out`. Negative integer width is what the tight product form needs in its corner, and that
is not the meet.

**My own hand-derivation of the waste region.** Closed by `p6.out`, which disagreed with it by
exactly sixteen pairs. The derivation is not in this file and the measurement is, which is the
correct outcome and is recorded rather than quietly dropped.

**Two instruments killed for cost, and kept.** `p1`'s brute-force least-containing search and
`p5`'s full-enumeration variant both failed to finish and were killed. Neither is deleted:
`p1` retains its slow method and **runs it as the control** on a small box, 324 pairs at 0
disagreements, and `p5` retains full enumeration as its own control at 256 pairs at 0
disagreements. A method too slow to use is still what licenses the method that replaced it.

## 9. Coverage, stated honestly

**What I read.** Everything the brief named, in full, plus `seed/SETTLED_laws.md` in full
because option H cites it and I was asked to check that citation rather than inherit it.
`CANON_CANDIDATE.md` grepped and read in four passages as a starting text; nothing in this
file cites it and every claim it suggested is re-established from arithmetic or from a probe.
The closed panel's tree was not opened, so every statement about `145`, `146`, `148` or `150`
here is a statement about the panel's own carried files reporting them.

**The largest bound, and it is large.** Every Python instrument is unsigned fixed-point, at
radix 2, with zero bias, in one family. So section 7's measurements are about that family and
nothing else. Untested: the ranged family, nonzero bias, the closed-interval adjustment the
record names for normalised channels and rotor components, and every radix but two. The third
of those is the one I would take first, because the negative-width corner lives exactly in the
purely-fractional population that adjustment is for, and a numeral built that way may put the
corner somewhere far less marginal than `p6` finds it.

**Not covered at all.** Whether the sign domain changes any of it. Whether the tight form
composes: `p5` measures one multiplication and says nothing about `x * y * z`, where the
saving may compound or may vanish. Whether the accumulator sufficiency check at site 10 is
expressible, which I asserted from the droplist's account and did not compile. Whether the
container derivation's tie-break is genuinely the strategy, which I have only as a historical
observation about a tree the brief declares nuked.

**Whether site 16 exists.** The mixed-numeral addition question in section 2.2 is the single
cheapest open item this file produces and I could not settle it from what I read.

**Everything here is unpriced.** No bench harness run bears on any of it. The `p3` and `p7`
asm reads are existence claims about erasure, not measurements, and the container-boundary
argument in section 7.1 is structural. Every number in this file is a count produced by a
named command in `06_probes/RUN.md`, and none of them is a magnitude.

**Owed under the two-expert rule, listed so nothing here is mistaken for agreed.** Every
section is a first read except where it re-derives `03`, and there the agreement is inherited
rather than found (section 6.1). Specifically owed a second read: section 1.1's coherence
argument that D3 cannot be inhabited, section 2's enumeration and its claim to be exhaustive,
section 2.2's claim that mixed-numeral addition is absent from the record, section 2.4's
second-pass sites, section 4.2's reframing of the family question as a closure-under-clamp question,
section 4.3's quadratic-versus-linear cost count, section 5.1's claim that the order has exactly two
callers, and section 7's tight form with its firing region.

## 10. What appears to be op's, and in what order

Stated as questions, per `04`. None of this is a recommendation and none of it settles.

**One, and it is cheap: does the design have a mixed-numeral addition?** Section 2.2. If it
does not, addition joins the six sites that infer nothing, the inference surface shrinks to
multiplication plus the container, and the family question narrows to exactly the surface `03`
section 1 identified from the other direction. If it does, its result numeral is unstated and
section 7 says what the formula would be. One sentence either way, and it changes what
everything else is about.

**Two: is the family question, operationally, a closure-under-clamp question?** Section 4.2.
On the reading here it stops being about whether the numerals form a lattice, because no
operation asks for one, and becomes whether the admitted shape space contains the clamp of
every formula's answer. That is narrower, it is testable, and the answer for the tight product
form is measured: no, at fifteen pairs in a box of 6561, and those fifteen are exactly where
negative integer width is needed. **This is a reframing of his own question rather than an
answer to it**, so it is his to accept or refuse before anyone builds on it.

**Three: may a canon sentence claim the derived numeral is the tightest honest answer?**
Section 7.1 measures the natural formula not tight at 461 of 6561 pairs, and section 7.3
builds a tight one that compiles and erases. So the choice is between changing the formula and
changing the sentence, and both are defensible: the tight form costs an extra type-level test
and, in its corner, an extra admitted shape. **This is the one place tonight where the finding
is a correctness finding rather than a size one**, because the sentence as it would naturally
be written is false.

**Four: does the design's own precedent at the container settle the cross-family shape?**
Section 2.3. The container axis already has incomparable elements and the design already
answers with a named rule keyed on the strategy, without a lattice anywhere. If that precedent
is accepted, `03`'s reading D is not an exotic fourth option but the shape the design already
uses, applied once more.

**Five: the price of a family is quadratic in the formulas and linear in the rules.** Section
4.3. Eight sites take a formula per unordered pair of families and three take one rule row per
family, so the family count is a cost on the operation surface rather than on the order. That
argues for answering cross-family questions with a rule, and it argues for `03`'s reading E on
a ground `03` did not give it: a computed family relation is the only shape here that collapses
the quadratic. Whether that relation is expressible under the forbidden-feature set is untested
by either of us and is the probe I would write next.

**Six, and it is a caution rather than a question.** `03` and this file agree on option H's
core, and under `RULES.md` that agreement is worth less than it looks, because I read `03`
before deriving. What is independent is the measurement, and the measurement corrects `03` in
two places (section 6). A reader who takes the convergence and drops the corrections will
carry forward exactly the inference that does not hold.
