# 131. Formalising the rounding axis

I took no part in this topic. My job here is to plug the holes, determine the bounds, and state the
result exactly, which is a different job from attacking, and I have tried to keep it that way. Where I
found something I disagree with, section 7 holds it, separately, so a later reader can tell the
formalisation from the objection.

The topic converged well and its convergence is unusual in one respect worth naming at the start: most
of what it established is **proved rather than swept**. Its objects are functions on an infinite grid,
so an impossibility argument settles a question at every width at once, where the preceding topic's
objects were enumerations over a finite container and almost every claim there was bounded to the
widths somebody happened to run. Section 4 sorts the claims into those two kinds, because the
difference decides which of them a canon may state without a width in the sentence.

Everything below is a suggestion. Op ratifies, and per I12 an opinion given before the experts converge
is an ack rather than a ratification.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` with its "How to read an entry" section as normative.

**I13 (`INTENTS.md:214`) is the shape section 3 is written in.** Every arm carries a const-checkable
predicate over a region, and there is no universal. Two arms are stated on an argument rather than on a
sweep and say which.

**I15 (`INTENTS.md:299`) decides the entropy hole in section 5.4, and it decides it in a way worth
stating precisely.** I15 forbids runtime *validation*, not runtime code, so a random draw is not
forbidden by I15. What forbids it is I14.

**I14 (`INTENTS.md:268`) is what actually closes the entropy question.** No `std::time`, `std::fs`,
`std::net`, `std::thread`, which are the only sources of entropy a library can reach on its own. So
arvo cannot produce a draw at all, and a stochastic member's randomness must come from outside.

**I7 (`INTENTS.md:135`) is served by exactly one arm and by no other.** Accuracy across chains needs an
error bound rather than an error estimate, and section 3's arm R1 is the only rounding arm that
provides one, because the directed pair is the unique adjoint pair.

**I1 is untouched.** Nothing below presumes a strategy count, a strategy name, or a decomposition. The
arms are keyed on the rounding mode, the operation set, the fraction width and the coupling, all of
which survive the strategy set being reshaped.

**No ambiguity to hand back.** Every judgement below is a measurement, a proof, or a citation.

### 0.2 Test gate: passed, at 123 across 13, inherited with attribution

The dispatch permits inheriting a recent count rather than re-running, and disk is tight enough that
building what I do not need would be the wrong call. The count I inherit is `125` section 10's, which
records the eleventh run at 123 across 13 by `--manifest-path`, and I add that `122` section 0.2 is the
fifteenth run of the same gate at the same figure. The workspace-wide form returns a false green per
`117:35` and is not what any of those counts used.

I ran no test-bearing crate myself and claim no count of my own.

---

## 1. What converged, restated once so the formalisation has a subject

Two cold derivations, blind and parallel, reached the same three results. Rounding has its own
impossibility theorem, and unlike the overflow axis's it is unconditional: no deterministic mode is an
additive homomorphism off the grid, on a domain closed under negation or on a one-signed one alike,
because the obstruction is the domain's **density** against the grid rather than its closure under
negation. Monotonicity, meanwhile, is free: every deterministic mode has it. So the property pair that
decides character for the overflow policy degenerates here, in both directions at once, and rounding is
not a second copy of that axis.

Both then placed the axis's own fork at the deterministic-stochastic boundary. That was refuted by
construction: a single threshold drawn once and held across a pass is monotone on every realisation and
exactly unbiased, so the two properties were never in tension. The boundary was then relocated twice,
independently: to the **within-cell coupling**, with a uniqueness theorem showing the per-cell uniform
threshold is the only realisation-monotone unbiased law, and to **within-pass decorrelation**, the
property imaging practice actually reaches for a dithered quantiser to get. The last file resolved
those into one axis with two keyings, value and position, by running each party's construction against
the other's worst case.

That is the subject. Sections 3 through 5 state it exactly, bound it, and close the four holes.

---

## 2. What I reproduced before building on it

Formalising something means resting on it, so the load-bearing pieces were checked rather than carried.

**The `F = 0` vacuity**, `125` F4, reproduced exhaustively over all six modes at `W` in `{3, 4, 5}` and
both signednesses, with a non-retraction control that breaks it on every pair (`v1`). This is the claim
the whole widening in section 5.1 rests on and it was carried by three later files and re-derived by
none.

**The vocabulary claim**, `125` F9, reproduced at `W` in `{4, 6, 8}` and `F` in `{1, 2}`: bit-drop
equals floor on 0 of every sweep, and differs from toward-zero on signed rows only, never on unsigned
(`v2`).

**Staged narrowing**, `125` T4 and `126` Finding 5, reproduced on a third instrument: the directed
modes at 0 mismatches and the nearest modes at 16 to 32 (`v2`).

**The position-keyed member's cost and benefit**, `129`'s, reproduced as compile-time assertions with a
degenerate-keying control (`v3`).

**Not reproduced**, and named because the arms rest on them: the Fréchet uniqueness theorem, the
variance closed forms, `130`'s determinant computation, and the whole equivariance table. I read the
arguments, found them sound on inspection, and did not build a fourth instrument for any of them.

---

## 3. The arms, stated exactly

Per I13. A dimension listed with a range or `any` is a region established; a fixed value is that value
only; an absent dimension claims nothing anywhere it is present. Conventions stated once: everything
enumerative ran on one thread; the model sweeps are exact rational arithmetic that no instruction
selection can move, so `target features any` with that as the argument; and every predicate names the
domain and whether it is closed under negation, which is the dimension the preceding topic lost two
predicates to.

### R0. The vacuity arm, where no mode acts at all

> Where every operation in a derivation maps grid values to grid values, no rounding mode is invoked
> and every mode computes the same function. The realised operation is then invariant under the whole
> axis, stochastic members included.

*holds for: F = 0 with operations in {add, sub, mul, min}, and F any with operations in {add, sub,
min}; W any; signedness any; domain any, closed under negation or one-signed alike; rounding any,
including every member of the stochastic family; overflow any; threads any; target features any.*

**Stated on an argument, not on the sweep.** A mode is a retraction onto the grid, so it is the identity
on grid values, and the grid is closed under those operations. The argument names no width, no
signedness and no domain, which is why the predicate does not either. The sweep in `v1` is a control on
it: 0 disagreements over all six modes at `W` in `{3, 4, 5}`, both signednesses, with a non-retraction
mode disagreeing on every pair.

### R1. The certified-bounds arm, which is the only one that serves I7

> `floor` and `ceil` are the right and left adjoints of the grid inclusion, and adjoints are unique. So
> they are the only two modes carrying an exact one-sided order law, and a certified enclosure can be
> built on that pair and on no other.

*holds for: W any; F any; signedness any; domain closed under negation; rounding in {floor, ceil};
operations any; threads any; target features any.*

**Stated on an argument.** Adjoint uniqueness is a fact about the poset, not about a container. The
consequence is the one worth carrying: for a strategy in the spirit of I7, this is the difference
between an error estimate and an error bound, and no nearest mode and no stochastic member provides
one.

### R2. The staged-narrowing arm

> Right adjoints compose, so narrowing to a fine grid and then to a coarse one equals narrowing to the
> coarse one directly, exactly, for the directed modes. The nearest modes do not compose: an
> intermediate rounding can move a value across a tie boundary the direct rounding never crosses.

*holds for: the composition claim, rounding in {floor, ceil, toward_zero}, F any, W any, signedness
any, domain closed under negation, threads any, stated on an argument. For the failure, rounding in
{half_up, half_even}, F_exact in {4, 5}, F_intermediate in {2, 3}, F_final in {1, 2}, domain =
rationals over the swept window, threads = 1, measured at 16 to 32 mismatches of 129 points per row.*

The failure is an **existence** claim and existence claims carry: one witness establishes that the
nearest modes do not compose, at any width. The **rate** does not carry and is not claimed.

### R3. The commutation arm, and the one pairing that does not

> Every deterministic mode commutes with saturation, because a monotone grid-fixing map cannot move a
> value across a grid-aligned clamp boundary in one order and not the other. Against wrapping, the
> modes equivariant under translation by the quantum commute, which is `floor`, `ceil`, `half_up` and,
> because a span is an even multiple of the quantum, `half_even`. **`toward_zero` does not**, and the
> failure needs a negative off-grid intermediate meeting a wrap.

*holds for: the commutations, rounding in {floor, ceil, toward_zero, half_up, half_even} against
saturation and rounding in {floor, ceil, half_up, half_even} against wrapping, W >= 1, F any,
signedness any, domain closed under negation, threads any, stated on the equivariance argument. For the
non-commutation, rounding = toward_zero, overflow = wrap, signedness = signed or unsigned with signed
intermediates, threads = 1, measured with the witness `125` T7 names.*

**The design consequence is one sentence and it is the only place the two axes fail to decompose.** A
canon admitting `toward_zero` alongside wrapping must state the order, which is quantise first and
range-reduce second. For every other pairing the sentence is unnecessary because the maps commute.

### R4. The roundless-multiplication arm

> Multiplication is exact, hence mode-invariant, exactly on the operand pairs whose scaled integers'
> 2-adic valuations sum to at least the fraction width. The valuation is additive, so the condition is
> exact in both directions rather than merely sufficient.

*holds for: W any, F any, signedness any, domain any, operation = mul, rounding any, overflow any (the
claim is about quantisation; range effects belong to the overflow axis), threads any, stated on
valuation additivity, with an exhaustive control at F in {0, 2, 3} and W <= 6.*

This is I13 material in its purest form. Not "multiplication rounds", but a named region where it
provably does not, const-recognisable whenever either operand is an integer multiple of one.

### R5. The coupling arm, one axis with two keyings

> Unbiasedness fixes each element's marginal completely, so everything distinguishing stochastic
> schemes is the joint law. Within one cell, per-realisation monotonicity is exactly comonotonicity,
> the inversion probability is a single Fréchet parameter, and the realisation-monotone unbiased law is
> **unique**: a uniform threshold per cell. Across cells every coupling is monotonicity-safe.
>
> Per-realisation monotonicity therefore trades against **decorrelation of the rounding decision**, and
> that tension is **keyed** on whichever axis the coupling's randomness varies along. A construction
> keyed on one axis delivers nothing on the other.

*holds for: the uniqueness and the Fréchet impossibility, F any, W any, signedness any, domain closed
under negation, rounding = the unbiased stochastic family, coupling = any point of the Fréchet
interval, threads any, stated on distribution algebra with the linear system shown invertible at every
cell resolution. For the variance law, n any at f = 1/3, coupling in {comonotone, independent}, stated
on the scalar-multiple and independent-additivity identities with enumeration through n = 14. For the
keying divergence, one input shape (a constant value at 40 positions, one cell), keying in {global,
value, position}, F any, signedness any, threads = 1, measured.*

**The variance law's fraction is a fixed value and I will not widen it.** `130` says the closed forms
are algebraic in `f` and that it did not check a second one. Neither did I. So the predicate reads
`f = 1/3` and the widening is available to whoever runs a second fraction.

### R6. The entropy-free member

> A position-keyed deterministic dither decorrelates within a pass, pays the same monotonicity price
> the independent draw pays, and needs **no runtime entropy at all**: its threshold is integer
> arithmetic on the position, computable at compile time wherever the position is.

*holds for: toolchain = the pinned nightly, edition 2021, crate type = library, `#![no_std]`, feature
gates = none, no float types, threshold = a golden-ratio sequence in Q32, positions 0 to 255, value =
an exact tie, threads any, target features any.*

Section 5.4 is why this arm matters more than its size suggests.

---

## 4. The bounds: which claims carry, and by what kind of argument

The dispatch asks which claims are bounded to what was swept and which have an argument that carries
them further, and to name the kind. Sorting the topic's claims that way is most of what a canon needs
from this section, because a claim with a width in its statement cannot appear in a canon sentence
without the width.

**Carried by an algebraic obstruction**, which is neither a symmetry nor a saturation point nor an
induction, and which is the strongest kind available here. The no-additive-homomorphism theorem, on
both domains: a divisible group's image under a homomorphism is divisible, and the only divisible
subgroup of the grid is trivial. The one-signed form runs the divisibility by hand on the monoid. These
hold at every width because no width appears in either argument.

**Carried by a closure argument.** R0's vacuity: the grid is closed under the operation set, so the
retraction never fires. No width, no signedness, no domain.

**Carried by a uniqueness argument.** R1's adjunction, since adjoints are unique; and R5's per-cell
uniform threshold, since the marginal-determining system is lower-triangular with unit diagonal and
therefore invertible at every cell resolution, which `130` computed by explicit elimination rather than
reading off the shape.

**Carried by an equivariance argument.** R3's commutations, each of which is a statement about which
translations a mode respects.

**Carried by an induction on the element count.** R5's variance closed forms, which `130` extended past
enumeration by the scalar-multiple and independent-additivity identities. Carried in `n`, bounded in
`f`.

**Carried as an existence claim.** R2's nearest-mode composition failure, R3's `toward_zero` and wrap
non-commutation, and the stochastic non-monotonicity witness. One witness establishes each, and none of
their **rates** carries.

**Bounded to what was swept, and stated as such.** Every count in the topic. The double-rounding
mismatch rates. The decorrelation counts at 40 positions on one input shape. The keying divergence,
which `130` itself says it checked on one input shape. The bias measurement over 2000 consecutive ties,
which its own author says is adversarial and does not establish a general bias magnitude.

**The honest default the dispatch names holds here too**, and the topic is unusual only in how often it
escapes it: a claim swept small is a claim about small, and the escapes above are escapes because an
argument carries them, not because the sweep was wide.

---

## 5. The four holes

### 5.1 The `F = 0` widening, and it reaches more than half the pins

`125` F4 says any finding pinned to a rounding mode whose operations lie in the vacuous set widens to
`rounding any`. Three later files carry it and none re-derives it, and nobody has applied it to the
actual pins. Both halves are done in `v1`.

**The claim holds**, on the argument in R0, with the sweep as its control.

**Applied to the preceding topic**, extracting every predicate containing a truncation pin across the
panel and classifying each by its own stated fraction width and operation set:

```
  pins found                                            55
    WIDENS at F = 0     (ops in {add, sub, mul, min})   30
    WIDENS at any F     (ops in {add, sub, min})         8
    does not widen                                      14
    unclassifiable: the predicate names no operations    3
```

**So 38 of 55 pins widen to `rounding any`**, and the preceding topic's rounding restriction was real on
14 of them. Every one of those 14 has a multiplication at a nonzero fraction width, which is exactly
where R4 says rounding fires.

Two notes on the count. The dispatch says thirty-five, and 30 plus 8 is 38 rather than 35; my extractor
reads two predicate shapes across the whole panel and may be scanning a different set, so I report my
pattern and my number rather than reconciling to one I cannot see. And the three unclassifiable pins
are unclassifiable for a reason worth recording: **their predicates pin a rounding mode without naming
an operation set**, so nothing can decide whether rounding acts on them. That is a defect in those
predicates rather than in the extractor.

**One parser defect of my own**, found and fixed in view: a predicate listing two operation sets joins
them with "and", and my first version read the conjunction as an operation named `and`, which pushed
three pins into "does not widen" that in fact widen. The count above is the corrected one.

### 5.2 The vocabulary, settled by counting rather than by preference

`125` section 7 and `128` argue for six mode names retiring "truncation"; `126` kept the older spelling
and its own reply does not address it; `130` carries the confirmation without re-deriving it. So the
disagreement is located and both parties have seen the evidence.

It is settled by arithmetic, because "are these two modes the same operation" has an exact answer.
`v2`, at `W` in `{4, 6, 8}` and `F` in `{1, 2}`:

```
  bit-drop != floor        0 on every row, both signednesses
  bit-drop != toward_zero  4, 6, 16, 24, 64, 96 on the signed rows
                           0 on every unsigned row
```

**Two's complement bit-drop is `floor`. It is not `toward_zero`, and the two differ on signed domains
and nowhere else.** So on a signed domain the word "truncation" does not name a mode, and a predicate
spelled with it is not a predicate until a reader knows whether the probe implemented a shift or a
division. That is not a spelling preference and it is not settleable by whichever spelling is more
familiar.

**The settlement I would suggest**, which is `125`'s: name the six modes, and give `toward_zero` a note
saying it is the division and not the bit-drop. Neither existing spelling should win in the interim,
because the winner inherits the ambiguity.

**And one figure in the dispatch did not reproduce.** It cites 21,204 of 32,768 signed negative cases
at `W = 8`. Neither natural sweep gives it: integer division over all nonzero-divisor pairs at `W = 8`
gives 31,231 of 65,280, and single values at `W = 8` give 64, 96, 120 and 127 of 256 at `F` in
`{1, 2, 4, 7}`. I could not find the sweep that produces 32,768 cells and did not find the figure in
`125_probes`. **The direction is confirmed regardless and does not depend on the number**, so I record
the mismatch rather than chase it.

### 5.3 Double rounding, which threatens a different equality from the one it was aimed at

`126` names double rounding across chained carrier widths as a hazard the preceding topic's discharge
apparatus does not cover, at `126:511`. I think that names the wrong equality, and `v2` is how I found
out. It took three attempts and two of my own bookkeeping defects, both recorded in the probe.

**`122` 4.6's equality is not threatened.** Its rule is an equality between two arms that **both round
at every node**: one applies the range part at each node, the other defers it to the root while
applying the grid part at every node whose result leaves the grid. Double rounding happens identically
in both and cancels out of the comparison. Measured with 4.6 implemented as written, one grid for the
whole chain, which is the setting it is stated in: **0 differences for every mode including the nearest
ones**, over 256 cells per row with 64 to 128 cells where the exact result left the grid, so the
comparison was live.

**What is threatened is a different equality, between staged narrowing and direct narrowing**, which no
clause in the preceding topic states. Directed modes at 0 mismatches, nearest modes at 16 to 32. That
is R2, and it is the hazard `126` correctly identifies and slightly misplaces.

**And there is a third thing, which is neither and which I could not close.** When the grid coarsens
between nodes, `122` 4.6's phrase "the grid part must be applied at the result of every node whose
exact result can leave the grid" does not say **which** grid, and the two readings differ from the
reduce-at-every-node arm on 32 to 94 cells (read as each node's own output grid) and 124 to 170 (read
as the final grid). So with two grids in play the sentence is **ambiguous rather than wrong**, and
neither reading reproduces the arm it is meant to equal.

**I do not know the mechanism and I am not going to guess a fourth time.** Three mechanism guesses in
that probe were wrong, which is enough evidence that the honest output is the measurement plus a
statement that the cause is unestablished. What would decide it: an instrument that varies the grid
coarsening and the span alignment independently, which is one probe and which I did not build.

### 5.4 The entropy hole, and it makes one member decisive

Every stochastic member needs randomness. `128:177` closes its family map with "what no member escapes:
entropy at runtime". The question the dispatch asks is where a draw comes from under the operating
constraints, and whether the position-keyed scheme is the only member that survives them.

**The constraints forbid arvo from producing a draw at all.** I15 forbids runtime validation and not
runtime code, so it is not what closes this. I14 is: no `std::time`, `std::fs`, `std::net`,
`std::thread`, which are the only sources of entropy a library can reach on its own. So a stochastic
member's randomness must be supplied from outside arvo.

**And then I13's const instruction bites.** At `INTENTS.md:252` the admissible category is whatever is
available at const time, which reaches const data from outside the typestate. A seed supplied at const
time is admissible. But a const seed makes the scheme a fixed function, which is a **deterministic**
member of the family rather than a stochastic one. The disjunction is sharp: either the randomness is
const, in which case the member is deterministic, or it is not, in which case the consumer supplies it
at runtime and the arm's selection predicate is not fully const-available.

**The position-keyed member escapes the disjunction entirely**, and `v3` compiles it to check rather
than assert. Integer arithmetic on the position, `#![no_std]`, zero feature gates, zero float types, no
platform call, exit 0. Its findings are `const _: () = assert!(..)` items, so a wrong one is a build
failure rather than a printed number:

- a repeated value receives **both** decisions across 40 positions and across 256, so it decorrelates;
- a degenerate keying returning one threshold everywhere receives **one**, which is the control that
  makes the count mean something;
- it costs monotonicity on a ramp, which is `129`'s price confirmed rather than assumed;
- and the decisions at compile-time-known positions are `const` items, so a rounding decision gates an
  arm rather than branching at runtime.

**So the answer to the dispatch's question is yes, with one qualification.** Among members that
decorrelate, the position-keyed deterministic dither is the only one that survives I14 without the
consumer supplying entropy. The qualification is that the global shared threshold and the per-cell
family are not thereby excluded: they survive as **consumer-supplied-seed** members, where the draw
crosses the API boundary rather than being produced inside arvo. That is a real design, it is what
`arvo-toolbox-not-policer.md` would predict the substrate does, and it is not the same thing as arvo
having entropy.

---

## 6. What the canon should state

Written in the canon's register, and short, because most of what this topic established is a negative
result and negative results compress well. Every clause carries its predicate by reference to section
3 rather than repeating it.

> A **realisation map** factors into two decisions that act on disjoint parts of its input: a
> **quantisation**, which decides what happens to a value falling between representable points, and a
> **range policy**, which decides what happens to a value falling outside the representable extent.
> The two are separate axes and neither is a modifier of the other.
>
> The quantisation axis has **no homomorphic member among functions**, on any domain. The obstruction
> is that the domain is finer than the grid, so halves of grid steps exist whatever the sign, and no
> restriction of the domain relieves it. **Every deterministic member is order-preserving.** So the
> property pair that classifies the range policy degenerates here in both directions, and the
> quantisation axis selects character over a different property set: which **exact law** survives
> quantisation.
>
> Four such laws are available and no member carries more than one of the first three. **An exact
> one-sided order bound**, carried by the two adjoints of the grid inclusion and by no other member,
> because adjoints are unique; this is what makes certified enclosure possible and an accuracy-across-
> chains concern reachable. **Exact composition across precisions**, carried by the directed members;
> the nearest members make a staged narrowing depend on its staging. **Negation symmetry**, carried by
> the toward-zero member at the price of respecting no translation, which is the structure a wrapping
> range policy is built on. And **the additive law in expectation**, carried only by leaving the
> category of functions.
>
> The two axes **decompose**: they govern disjoint regions of the input and compose canonically as
> quantise-then-reduce. The composition order is unobservable for every pairing but one, the toward-zero
> member against a wrapping range policy, and a canon admitting that pairing states the order.
>
> Where every operation in a derivation maps grid values to grid values, **no member acts at all**, and
> a claim predicated on a quantisation member over such an operation set holds for every member. That
> region is the whole axis at zero fraction width for the ring operations and for order selection, and
> it is the additive operations at every fraction width. Outside it, multiplication is exact exactly
> where the operands' 2-adic valuations sum to at least the fraction width.
>
> Beyond the functions, an **unbiased** quantisation exists only as a distribution over them, and
> unbiasedness fixes each value's marginal completely. Everything else is the **coupling**. Within one
> cell, order on every realisation is exactly comonotonicity, and the order-preserving unbiased law is
> unique. Order on every realisation and **decorrelation of the decision** are the two ends of that one
> parameter and cannot be held together, and the tension is **keyed** on whichever axis the coupling's
> randomness varies along: a member keyed on value delivers nothing to a consumer whose data varies by
> position, and the converse. These are different consumers wanting different things, and no single
> member serves both.
>
> **A member requiring randomness requires it from outside**, because a library with no platform
> dependency has no source of it. A member whose randomness is fixed at compile time is a deterministic
> member. A member keyed on **position** decorrelates, pays the same order price, and requires no
> randomness at all.

**Permanence.** Every sentence survives a rewrite in another language or decade. None names a
container, a width, a type parameter, a crate, a mode's spelling, or a count of strategies.

**Equivalence.** Three teams implementing this produce units that behave the same on what matters:
nobody looks for a homomorphic rounding; a certified-bounds consumer reaches for the adjoint pair and
finds nothing else offers one; a design that narrows in stages knows which members make that
associative; a claim over grid-closed operations is not split by a dimension that does not act on it;
and a design wanting decorrelation without entropy finds exactly one member. They differ in how the
modes are named, how many ship, and whether the coupling is a parameter or a set of arms.

**Where it is weaker than I would like.** The order-preserving side of the coupling family is one
member deep in the literature this topic drew on and nobody has enumerated the rest. The variance law
is stated at one fraction. And the third thing in section 5.3 is unresolved, so the sentence about the
two axes decomposing is true for one grid and I do not know that it is true across a chain of them.

---

## 7. Where I disagree, kept separate

One thing, and it is about the preceding topic rather than this one.

**`122` 4.6's grid clause is ambiguous once carriers chain**, per section 5.3's third result, and I
raise it as a finding of mine rather than fold it into the formalisation. It is not a disagreement with
anything this topic converged on. `126` was right that there is a hazard the discharge apparatus does
not name; it named the wrong equality, and the equality it should have named is the one I could not
close. **The mechanism is unestablished and I say so rather than offer a fourth guess.**

Nothing else. I looked for something to disagree with in the convergence and did not find it, which I
record because the dispatch asks for measurements rather than readings and I have no measurement
against any of it.

---

## 8. What did not settle

**The variance law at a second fraction.** `130` widened it in `n` and says explicitly it did not check
a second `f`. Neither did I. The closed forms are algebraic in `f` and that is an expectation.

**The order-preserving licence family's membership**, which is the preceding topic's open item arriving
here unchanged.

**The mechanism behind section 5.3's third result.**

**Whether the position-keyed member's monotonicity failure rate differs meaningfully from the
independent member's.** `129` says it did not measure this and that the two counts it has are not
comparable. Neither did I.

**Whether any consumer wants the value keying rather than the position keying.** `130`'s divergence
result says a design must choose, and nothing in the repository measures which one arvo's consumers
would reach for. That is I11's territory and it is the same wall this panel has hit before.

**And three pins in the preceding topic name a rounding mode without naming an operation set**, so
nothing can decide whether they widen. Those predicates want repairing by whoever owns them, per the
never-widen-in-place rule.

---

## 9. Findings, each with its predicate

**F131-1. The vacuity claim holds over all six modes and a non-retraction control breaks it.** Zero
disagreements over `{add, sub, mul, min}` at `F = 0`, at `W` in `{3, 4, 5}` and both signednesses,
across floor, ceil, toward_zero, half_up, half_even and a stochastic realisation; a mode that adds a
quantum to on-grid values disagrees on every pair. `W in {3, 4, 5}, F = 0, signedness in {unsigned,
signed}, operations in {add, sub, mul, min}, rounding = all six enumerated, domain = representable
operand pairs, threads = 1, target features any`. `v1_output.txt`. Controls: division disagrees on 154
of 240 signed and 180 of 240 unsigned pairs, and multiplication at `F = 2` on 32 of 64.

**F131-2. Thirty-eight of fifty-five truncation pins in the preceding topic widen to `rounding any`.**
Thirty at `F = 0` over grid-closed operations, eight at any `F` over the additive set, fourteen do not
widen, three are unclassifiable because their predicates name no operation set. `document set = every
panel file matching the extractor's two predicate patterns, pin pattern = "rounding = trunc" or
"rounding = truncation"`. `v1_output.txt`. Control: the extractor reports no pin on a predicate naming
`rounding any` and none on a predicate with no rounding dimension.

**F131-3. Two's complement bit-drop is floor and is not toward-zero, and the difference is confined to
signed domains.** Zero mismatches against floor on every row; 4, 6, 16, 24, 64 and 96 mismatches
against toward-zero on the signed rows and zero on every unsigned row. `W in {4, 6, 8}, F in {1, 2},
signedness in {signed, unsigned}, domain = all W-bit two's complement values, threads = 1, target
features any`. `v2_output.txt`. Reproduces `125` F9 on an independent instrument.

**F131-4. `122` 4.6's equality is not threatened by double rounding in the setting it is stated in.**
Zero differences between its two arms for every mode including the nearest ones, over 256 cells per row
with 64 to 128 cells where a node's exact result left the grid. `W = 4, F in {1, 2}, signedness =
unsigned, overflow = wrap, rounding = all five deterministic modes, one grid for the whole chain,
threads = 1, target features any`. `v2_output.txt`.

**F131-5. With the grid coarsening between nodes, neither reading of `122` 4.6's grid clause reproduces
the reduce-at-every-node arm.** 32 to 94 differences reading it as each node's own output grid, 124 to
170 reading it as the final grid. `W = 4, F chain in {2 to 1, 3 to 1}, signedness = unsigned, overflow
= wrap, rounding = all five deterministic modes, range held fixed across the chain, threads = 1, target
features any`. `v2_output.txt`. **The mechanism is unestablished.**

**F131-6. Staged narrowing equals direct narrowing for the directed modes and not for the nearest
ones.** Zero mismatches for floor, ceil and toward_zero; 16 to 32 of 129 for half_up and half_even.
`F_exact in {4, 5}, F_intermediate in {2, 3}, F_final in {1, 2}, domain = rationals over the swept
window closed under negation, threads = 1, target features any`. `v2_output.txt`. Reproduces `125` T4
and `126` Finding 5 on a third instrument.

**F131-7. A position-keyed dither decorrelates, costs monotonicity, and compiles under the operating
constraints with no entropy.** A repeated tie receives both decisions across 40 and across 256
positions; a degenerate keying receives one; a ramp shows a nonzero monotonicity violation count; and
the decisions at compile-time-known positions are `const` items. `toolchain = nightly-2026-05-28,
edition 2021, crate type = library, no_std, feature gates = 0, float types = 0, threshold = golden
ratio in Q32, positions 0 to 255, threads any, target features any`. `v3_output.txt`. All four are
compile-time assertions, so the build succeeding is the result.

**Unpriced.** Everything about cost. No bench harness ran, nothing was timed, and no claim here depends
on a magnitude.

---

## 10. Coverage, bounded

**Read in full.** `125`, `126`, `127`, `128`, `129`, `130`, `INTENTS.md`.

**Read in part.** `RULES.md` at the rung definitions. `122` at sections 4.4, 4.6 and its predicate
blocks, which is where this topic reaches into it. `126:511`, `128:177` and `125:326`, opened.

**Not read.** `125_probes` through `130_probes` sources, except a grep for the figure section 5.2
could not reproduce. Every panel file before `122` except through `122`'s own account. `OPTIONS.md`,
`AGREEMENTS.md`, `DROPLIST.md`.

**Reproduced.** The vacuity claim, the vocabulary claim, staged narrowing, and the position-keyed
member's two properties, each named in section 2 at the point it appears.

**Not reproduced**, and the arms rest on them: the Fréchet uniqueness theorem, the variance closed
forms, `130`'s determinant computation, the equivariance table, the adjunction argument, and every
count in the topic that I did not re-run.

**Not established by anything I ran.** Anything at a non-uniform value set. Any duration. The mechanism
in F131-5. Whether the extractor's fifty-five pins are the dispatch's thirty-five.

**Three defects of my own**, all recorded in the probes rather than repaired silently: a parser reading
the word "and" as an operation, a witness column printing two modes that agree, and two successive
wrong constructions of the double-rounding comparison, the second of which deferred more than `122` 4.6
licenses and whose failures therefore said nothing about 4.6.

---

## 11. Probe index

- `v1_the_f0_widening_and_every_pin_it_reaches.py`, `v1_output.txt`. The vacuity claim over all six
  modes with a non-retraction control, and every truncation pin in the preceding topic classified.
- `v2_the_vocabulary_and_which_equality_double_rounding_threatens.py`, `v2_output.txt`. Bit-drop against
  floor and against toward-zero at three widths; `122` 4.6 implemented as written in one grid and across
  a chain; staged against direct narrowing.
- `v3_which_stochastic_members_survive_the_operating_constraints.rs`, `v3_output.txt`. The position-keyed
  member compiled `#![no_std]` with its findings as compile-time assertions and a degenerate-keying
  control.
