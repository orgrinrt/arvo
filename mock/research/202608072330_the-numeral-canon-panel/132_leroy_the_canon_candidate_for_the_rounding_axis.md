# 132. Canon candidate: the quantisation axis, what it selects, and the coupling underneath it

**This is a candidate and a draft, not a canon file.** It does not move to `mock/canon/`, nothing in it
is settled, and op ratifies. Per I12 an opinion given before the experts converge is an ack rather than
a ratification, and none has been given.

**It is written for three signatures that have not been given.** `125`, `126` and `127` built this and
are resumed after it to cosign, dissent, or sign in part. I took no part in the topic; I formalised it
at `131` and this compresses that. Section 8 names, per author, where I am most likely to have
compressed their work out of shape.

Order below is the order the dispatch asks for: the ledger first and in full, because a compression
that leads with its conclusion has already decided what to drop; then what is contested and what is
retired; then the statement, with each clause carrying **the kind of argument that bounds it**; then
the three things that must stay visible; then what only op can decide; then the anchor accounting.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` with its "How to read an entry" section as normative.

**I13 (`INTENTS.md:214`) is the shape of section 5.** Every clause carries a const-checkable predicate
over a region and there is no universal. Two clauses rest on an argument rather than on a sweep and say
which.

**I14 (`INTENTS.md:268`) is what decides the entropy question**, and it decides it more sharply than the
topic did. I15 forbids runtime validation rather than runtime code, so it is not what closes it; I14's
prohibition on platform dependency is, because the platform is the only place a library can get a draw.

**I7 (`INTENTS.md:135`) is served by exactly one clause and by no other.** Accuracy across chains needs
a bound rather than an estimate, and the adjoint pair is the only source of one.

**I1 is untouched and section 7 says so explicitly.** Nothing below presumes a strategy count, a name,
or a decomposition, and how this axis meets the strategy axis is named as op's.

**No ambiguity to hand back.** Every judgement below is a measurement, a proof, or a citation.

### 0.2 Test gate: passed, inherited with attribution

`125` section 10 records the eleventh run at 123 across 13 by `--manifest-path`; `122` section 0.2 is
the fifteenth at the same figure. The workspace-wide form returns a false green per `117:35` and is not
what either used. I ran no crate and claim no count of my own.

---

## 1. The agreement ledger

**A rung is carried by naming each instance's instrument and its independence, not by a label.** This
topic has unusually good provenance and the ledger is where that shows, so each entry below says what
kind of agreement it is: blind and parallel, independent relocation, reproduction on a second
instrument, re-derivation with a widened predicate, or one author unreproduced.

### 1.1 Blind and parallel, and the two lists reconciled

`128` names five convergences between `126` and `125` phase one; `129` names two it reached before
reading `125`. **The dispatch asks for these reconciled rather than merged, and reconciling them
changes the count.** One entry appears on both lists and one appears on neither of the other's, so the
union is **six**, not seven and not five.

| # | The convergence | `125`'s name | `126`'s name | named by |
|---|---|---|---|---|
| B1 | Rounding is vacuous for grid-closed operations | F4 | Finding 1 | **both** `128` and `129` |
| B2 | Every deterministic mode is monotone | T2, F2 | Finding 3 | `128` only |
| B3 | No deterministic mode is additive off-grid | T1, F1 | Finding 2's measured half | `128` only |
| B4 | Nearest modes split from directed on double rounding | T4 and P4 | Finding 5 | `128` only |
| B5 | Saturate composition preserves monotonicity | T6 | Finding 6's first half | `128` only |
| B6 | The answer's shape: neither a copy nor a modifier of the overflow axis, but an independent axis with its own selected property | section 8 | its reconciliation section | `129` only |

**B6 is the one a merge would have lost**, and it is the most consequential of the six, because it is
the answer to the question the topic was convened under. `129` records reaching it from a different
argument structure than `125` did: a divisibility-obstruction theorem on one side, a
domain-and-operation-set sweep plus a reading of `122`'s own grid rule on the other. Two arguments, one
conclusion, neither having seen the other.

**What the evidence for blindness actually is, stated at its real strength.** `128` cites the commit
ordering as the audit trail. `w1` shows it rather than citing it:

```
  1be29246  23:20  research: 126 wronski cold derivation on rounding character (phase one)
  a42f0b17  23:28  research: 125 cold derivation of the rounding axis, predictions committed before probes
  5ae424e7  23:29  research: 125 probe p1 ...
```

That establishes the **within-file** half: `125`'s predictions were committed before its own probes ran,
so its results did not shape its hypotheses. It does **not** establish the between-file half on its own,
because the two phase ones are eight minutes apart and an ordering cannot show what was read in
between. So blindness rests on the ordering **and** on each file's own coverage statement, and I state
it that way rather than let the audit trail carry more than it holds.

### 1.2 The relocation, reached twice independently

**This is the strongest rung in the topic** and it deserves its own entry.

`125` F8 and `126` Finding 4 both placed the axis's fork at the deterministic-stochastic boundary.
`127` refuted that by construction, with a shared threshold that is monotone on every realisation and
exactly unbiased, and both authors conceded.

The boundary was then relocated **twice, by two authors, neither having read the other**:

- **`128`, to the within-cell coupling**, with a uniqueness theorem: unbiasedness forces every
  marginal, so all that distinguishes schemes is the joint law; realisation-monotonicity is exactly
  within-cell comonotonicity; and the realisation-monotone unbiased law within a cell is **unique**, a
  uniform threshold.
- **`129`, to within-pass decorrelation**, from imaging practice: the property that motivates reaching
  for a dithered quantiser at all is that repeated inputs in one pass receive different treatment, and
  neither monotonicity nor unbiasedness says anything about it.

**The independence is checkable from their own coverage sections.** `128` read `127`, `126` and
`127_probes/w1`, and built `128_probes/r1`, `r2` and `r3`. `129` read `127`, `125` and the same
`127_probes/w1`, and built `129_probes/x1`. Neither lists the other. Both are replies
to `127`, written in parallel.

`130` then resolved them, by construction rather than by argument: **one axis, two keyings.** It ran
`128`'s per-cell member against `129`'s worst case (one value at forty positions) on `130_probes/y1`,
and found it delivers
**zero** decorrelation there, because every position falls in one cell. So the two relocations are the
same tension seen through two witnesses, keyed on different axes, and a member keyed on one delivers
nothing on the other.

**Rung: two independent relocations of one boundary, plus one resolution by construction.** Nothing
else in either topic reaches this shape.

### 1.3 Reproduction on a second instrument, which is a lower rung and is not the same thing

- **`127` F127-1**, the shared-threshold construction, reproduced by `128` on its own `r1`. Two
  instruments, two members. `127` built it; `128` did not derive it independently.
- **`125` F9 and the bit-drop identity**, reproduced by `131` on a third instrument at `W` in
  `{4, 6, 8}`.
- **`125` F4's vacuity**, reproduced by `131` over all six modes with a non-retraction control.
- **`125` T4 and `126` Finding 5's staged-narrowing split**, reproduced by `131`.
- **`129`'s position-keyed member**, its decorrelation and its monotonicity cost reproduced by `131` as
  compile-time assertions with a degenerate-keying control.

### 1.4 Re-derivation with a widened predicate, which is stronger than reproduction and weaker than a blind convergence

The dispatch asks for this rung by name and `130` is the only file that reaches it.

- **The variance forms.** `128` F128-3 enumerated all outcomes to `n = 10`. `130` derived both closed
  forms from first principles before checking them, by the scalar-multiple identity and the additivity
  of variance across independent terms, and **widened the predicate from `n` in 1..10 to `n` any**,
  cross-checked by brute force through `n = 14`. It kept `f = 1/3` and said explicitly it did not sweep
  a second fraction.
- **The uniqueness theorem.** `128` r2 solved one triangular system at `m = 8`. `130` computed the
  determinant by explicit Gaussian elimination at `m` in `{3, 5, 8, 12, 20}` and **widened from one
  solved instance to invertibility at every cell resolution**, which is what makes uniqueness a fact
  about the structure rather than about one number that came out uniform.

**Rung: one derivation, one independent re-derivation that widened it.** That is stronger than a
reproduction because the second author reached the closed form without the first's, and weaker than a
blind convergence because the question had already been posed.

### 1.5 One author, unreproduced

Named so nobody reads them at a higher rung than they hold.

`125` T3's adjunction and T5's equivariance table, both reported at F3 and measured on
`125_probes/p4`; `125` F6's valuation
predicate; `125` F5's division residue and T7's non-commutation of toward-zero with wrapping, reported
together at F7; `126` Finding 5's tie-bias magnitude, which its own author
calls adversarial; `126` Finding 6's overflow-verdict divergence; `128` F128-2's Fréchet impossibility;
`128` F128-4's per-cell wrap law; `129` F129-1's decorrelation counts; `130` F130-1's keying divergence,
measured on one input shape; and every finding in `131`.

`129` records that it did not re-verify `125`'s T3, T4, T5 or F6 and makes no claim about them, which is
the right conduct and is why they sit here.

### 1.6 Contested, each with what would decide it

**C1. The mode vocabulary.** `125` section 7 and `128` argue for six mode names retiring "truncation";
`126` kept the older spelling and `129` does not address it; `130` carries the confirmation without
re-deriving it. **The factual half is now settled and is not contested**: `131` measured bit-drop equal
to floor on every row and different from toward-zero on signed rows only, so on a signed domain the word
names two operations. **What remains contested is the naming call**, which is a canon-vocabulary
decision rather than a measurement. **What would decide it:** op, or whoever writes the canon text.

**C2. Whether the variance forms hold at a second fraction.** `130` says the forms are algebraic in `f`
and that it checked one. **What would decide it:** one sweep at a second fraction, which is cheap and
which nobody has run.

**C3. Whether any consumer wants the value keying rather than the position keying.** `130`'s divergence
says a design must choose and nothing in the repository measures which arvo's consumers would reach
for. **What would decide it:** I11's territory, and the same wall this panel has hit before.

**C4. Whether the position-keyed member's monotonicity failure rate differs meaningfully from the
independent member's.** `129` says the two counts it has are not comparable and did not measure it.
Neither did `131`. **What would decide it:** a sweep with one construction and one input shape varied
across both members.

### 1.7 Retired, so a later reader knows what to stop citing

**Refuted and conceded by their authors.**

- `125` F8's second clause, that a saturating realisation composed with stochastic rounding is ejected
  from the monotone family. False for the shared threshold, conceded at `128` section 2.
- `125` F8's boundary placement at the deterministic-stochastic line. Wrong, conceded in the same
  section, whose author records that `126` hedged the same claim honestly and it did not.
- `126` Finding 4's framing of that boundary as "the genuine either/or fork". Same fact, and its own
  open item 3 already said the exclusion was exhibited rather than proved.
- `128`'s prediction that the per-cell member loses wrap compatibility outright. Half wrong, refuted by
  its own `r3`, sharpened to a residue-class condition, with the refuted run preserved.
- `126`'s first attempt at the non-monotonicity witness, which picked points in different grid
  intervals and found none. Recorded by its author as a refuted attempt rather than deleted, and it is
  informative: the violation requires the points to share a cell.

**Corrected in reach rather than refuted.**

- `127`'s sentence that the correlated construction "dominates the independent one on every property
  either file measured". True as written; `129` corrected its reach, because the property an arm
  actually wants is decorrelation and the shared threshold delivers **zero** of it.
- `127` F127-2's identification of the midpoint with half-up, corrected at ties by `128` F128-5 and
  accepted by `130`.

**Must no longer be cited as naming a mode.** The word "truncation", and the token `trunc`, on any
signed domain. They name two distinct operations there, and a predicate spelled with one of them is not
a predicate until a reader knows which was implemented.

**And one figure must no longer be cited at all.** "21,204 of 32,768 signed negative cases at `W = 8`"
did not reproduce on either natural sweep in `131`: integer division over all nonzero-divisor pairs at
`W = 8` gives 31,231 of 65,280, and single values give 64, 96, 120 and 127 of 256 at `F` in
`{1, 2, 4, 7}`. **The figure originated in `125` and was relayed into two briefs by the coordinator
without checking, and the coordinator attributes the relay error to itself rather than to `125`.** The
direction the figure was cited for is established by arithmetic and does not depend on it. Section 6.2
is where a reader meets this.

---

## 2. What the candidate rests on, and one thing it does not

Every clause in section 5 rests on a proof, a model sweep, or a compiled probe. **No clause rests on a
bench-harness measurement**, so `117`'s record that every number in this repository's bench directory
was taken at a different profile than the harness documents does not reach any of them.

The one place shipped source appears is the entropy clause's operating constraints, where what was read
is `INTENTS.md` rather than any number.

---

## 3. Where the doability was established

The canon says which things are doable and cites where, rather than reproducing the proof.

- **The position-keyed member is expressible under the operating constraints.** `131` F131-7 on
  `131_probes/v3`, compiled `#![no_std]` with zero feature gates, zero float types and no platform
  call, with its findings as compile-time assertions so the build succeeding is the result.
- **The roundless-multiplication region is const-recognisable.** `125` F6's valuation predicate, exact
  in both directions by additivity of the valuation, with `125_probes/p2` as its control.
- **The vacuity region is const-recognisable and needs no predicate machinery at all**, because it is a
  property of the operation set and the fraction width, both already in the typestate. `125` F4,
  reproduced at `131` F131-1.
- **The unbiased realisation-monotone law is constructible and unique.** `128_probes/r2` for the
  construction, `128_probes/r3` for the uniqueness, `130_probes/y1` for the keying that bounds it.

---

## 4. What each clause is bounded by

The dispatch asks that `131` section 4's sorting survive, and it is the most useful thing here for a
later reader, because **a claim with a width in its statement cannot appear in a canon sentence without
the width**. Each clause in section 5 is tagged with the kind of argument that carries it.

- **Algebraic obstruction.** The strongest kind available. A divisible group's image under a
  homomorphism is divisible and the only divisible subgroup of the grid is trivial; the one-signed form
  runs the same divisibility by hand on the monoid. No width appears in either argument.
- **Closure.** The vacuity clause: the grid is closed under the operation set, so a retraction never
  fires.
- **Uniqueness.** The adjunction, since adjoints are unique; and the per-cell uniform threshold, since
  the marginal-determining system is invertible at every resolution.
- **Equivariance.** The commutation clauses, each a statement about which translations a mode respects.
- **Induction.** The variance closed forms, carried in the element count and bounded in the fraction.
- **Existence.** The non-composition of nearest modes, the non-commutation of toward-zero with wrapping,
  and the stochastic non-monotonicity witness. One witness each; **their rates do not carry**.
- **Bounded to the sweep.** Every count in the topic, the double-rounding rates, the decorrelation
  counts, the keying divergence, and the tie-bias magnitude its own author calls adversarial.

**The honest default holds here too, and this topic is unusual only in how often it escapes it.** A
claim swept small is a claim about small. The escapes above are escapes because an argument carries
them, not because the sweep was wide.

---

## 5. The statement

Written in the canon's register: what a thing is, what it requires, what it excludes. Each clause
carries its predicate and its argument kind. Conventions once: everything enumerative ran on one thread
and carries `threads = 1`; the sweeps are exact rational arithmetic no instruction selection can move,
so `target features any` with that as the argument; and every predicate names the domain and whether it
is closed under negation, which is the dimension the preceding topic lost two predicates to.

### 5.1 The map factors, and the two factors are separate axes

> A **realisation map** factors into two decisions acting on disjoint parts of its input: a
> **quantisation**, deciding what happens to a value falling between representable points, and a
> **range policy**, deciding what happens to a value falling outside the representable extent. Neither
> is a modifier of the other.

*Definitional. No predicate.*

### 5.2 The quantisation axis has no homomorphic member, on any domain

> **No deterministic quantisation is additive off the grid** (`125` T1 and F1), on a domain closed under
negation or on a
> one-signed one alike. The obstruction is that the domain is finer than the grid, so halves of a grid
> step exist whatever the sign, and no restriction of sign relieves it. **Every deterministic member is
> order-preserving** (`125` T2 and F2, both on `125_probes/p1`).
>
> So the property pair that classifies the range policy degenerates here in both directions at once,
> and the quantisation axis is not a second copy of it.

*holds for: W any; F any; signedness any; domain in {closed under negation, one-signed}, each claimed
and proved separately; rounding = every deterministic member; operation = addition; threads any; target
features any.*

**Argument kind: algebraic obstruction.** This is the sharpest contrast with the range axis, whose own
impossibility turned out domain-conditional. Here the domain does not relieve it, because density
rather than closure is what it fights.

### 5.3 What the axis selects instead: which exact law survives

> Four exact laws are available and no member carries more than one of the first three. `125` F3
> enumerates the first three; F8's positive half is the fourth.
>
> **An exact one-sided order bound**, carried by the two adjoints of the grid inclusion and by no other
> member, because adjoints are unique. This is what makes a certified enclosure possible, and it is the
> difference between an error bound and an error estimate.
>
> **Exact composition across precisions**, carried by the directed members. The nearest members make a
> staged narrowing depend on its staging.
>
> **Negation symmetry**, carried by the toward-zero member, at the price of respecting no translation,
> which is the structure a wrapping range policy is built on.
>
> **The additive law in expectation**, carried only by leaving the category of functions.

*The adjunction holds for: W any; F any; signedness any; domain closed under negation; rounding in the
two adjoints; operations any; threads any. **Argument kind: uniqueness.***

*The composition failure holds for: rounding in the nearest members; F_exact in {4, 5}; F_intermediate
in {2, 3}; F_final in {1, 2}; domain = rationals over the swept window closed under negation; threads =
1. **Argument kind: existence**, so the failure carries at every width and its rate does not.*

### 5.4 The axes decompose, with one named exception

> The two axes govern disjoint regions of the input and compose canonically as quantise-then-reduce
> (`125` F7, with `125` F5's division residue as the operation where the axis persists at zero fraction
> width). **Selecting a quantisation mode cannot move a realisation between the overflow families**
> (`125` T8, on `125_probes/p3`), one-signed domains included, which is what makes the two axes
> independent rather than merely separable.
> The composition order is **unobservable for every pairing but one**: the toward-zero member against a
> wrapping range policy, which does not commute because that member respects no translation. A canon
> admitting that pairing states the order; for every other pairing the sentence is unnecessary.

*The commutations hold for: W >= 1; F any; signedness any; domain closed under negation; every
deterministic member against saturation, and every translation-equivariant member against wrapping;
threads any. **Argument kind: equivariance.***

*The non-commutation holds for: rounding = toward-zero; range policy = wrapping; signedness = signed, or
unsigned with signed intermediates; threads = 1. **Argument kind: existence.***

### 5.5 Where the axis does not act at all

> Where every operation in a derivation maps grid values to grid values, **no member acts**, and a claim
> predicated on a quantisation member over such an operation set holds for every member. That region is
> the whole axis at zero fraction width for the ring operations and for order selection, and it is the
> additive operations at every fraction width.
>
> Outside it, multiplication is exact **exactly** where the operands' 2-adic valuations sum to at least
> the fraction width (`125` F6), which is a condition in both directions rather than merely sufficient.

*The vacuity holds for: F = 0 with operations in {addition, subtraction, multiplication, order
selection}, and F any with operations in {addition, subtraction, order selection}; W any; signedness
any; domain any; rounding any, stochastic members included; range policy any; threads any; target
features any. **Argument kind: closure.***

*The valuation predicate holds for: W any; F any; signedness any; domain any; operation =
multiplication; rounding any; range policy any; threads any. **Argument kind: algebraic**, by additivity
of the valuation.*

**This is I13 material in its purest form**: not "multiplication rounds", but a named region where it
provably does not.

### 5.6 Beyond the functions: unbiasedness fixes the marginals and everything else is the coupling

> An **unbiased** quantisation exists only as a distribution over deterministic ones, and unbiasedness
> **forces each value's marginal completely**. Everything distinguishing such schemes is the joint law.
>
> Within one cell, order on every realisation is exactly comonotonicity, and the order-preserving
> unbiased law is **unique** (`128` F128-1, with `128` F128-2 as the impossibility underneath it).
> Across cells every coupling preserves order. What a stochastic member gives up in pointwise
> monotonicity it recovers in expectation (`125` T9), which is why the loss is a change of guarantee
> rather than an absence of one.
>
> Order on every realisation and **decorrelation of the decision** are the two ends of one parameter and
> cannot be held together within a cell. Coherent errors add linearly and independent ones add in
> quadrature, so the concentration that makes an unbiased quantisation attractive over a long chain
> belongs to the independent end exclusively.

*The uniqueness and the impossibility hold for: F any; W any; signedness any; domain closed under
negation; coupling = any point of the Fréchet interval; threads any. **Argument kind: uniqueness**, with
the system shown invertible at every cell resolution.*

*The variance law holds for: element count any; fraction = 1/3; coupling in {comonotone, independent};
threads any. **Argument kind: induction.** The fraction is a fixed value and is not widened.*

### 5.7 The tension is keyed, and a member keyed on one axis delivers nothing on the other

> The tension in 5.6 is **keyed** on whichever axis the coupling's randomness varies along. A member
> keyed on **value** decorrelates across elements whose values differ and delivers nothing for elements
> sharing a value, wherever they sit. A member keyed on **position** decorrelates across positions
> regardless of whether the values agree. A member keyed on **nothing** decorrelates nothing at all.
>
> These are different consumers wanting different things, and **no single member serves both**.

*holds for: keying in {none, value, position}; decorrelation measured both as summed-error variance for
same-value elements and as distinct-output count for a repeated value across positions; F any;
signedness any; input shape = one constant value at forty positions in one cell; threads = 1. **Argument
kind: bounded to the sweep**, on one input shape, which `130` says of itself.*

### 5.8 Randomness comes from outside, or the member is deterministic

> A member requiring randomness **requires it from outside**, because a library with no platform
> dependency has no source of one. A member whose randomness is fixed at compile time is a
> **deterministic** member. A member keyed on **position** decorrelates, pays the same order price, and
> requires no randomness at all.

*holds for: toolchain = the pinned nightly; edition 2021; crate type = library; `#![no_std]`; feature
gates = none; float types = none; threshold = a low-discrepancy sequence on the position; positions 0 to
255; threads any; target features any. **Argument kind: bounded to the sweep for the measured half**,
and a reading of the operating constraints for the rest.*

**The disjunction is sharp and it is the most consequential thing in this section for a design.** Either
the randomness is available at compile time, in which case the member is a fixed function, or it is
not, in which case a consumer supplies it across the API boundary and the arm's selection predicate is
not fully const-available. The position-keyed member escapes the disjunction entirely, and it is the
only member that does.

**Permanence.** Every sentence survives a rewrite in another language or decade. None names a container,
a width, a type parameter, a crate, a mode's spelling, or a count of strategies.

**Equivalence.** Three teams implementing this produce units that behave the same on what matters:
nobody looks for a homomorphic quantisation; a certified-bounds consumer finds exactly one pair that
serves it; a design that narrows in stages knows which members make that associative; a claim over
grid-closed operations is not split by a dimension that does not act on it; and a design wanting
decorrelation without entropy finds exactly one member. They differ in how the modes are named, how
many ship, and whether the coupling is a parameter or a set of arms.

---

## 6. Three things that must stay visible

The dispatch names these and it is right to, because each is the kind of thing a compression smooths
away by default.

### 6.1 The double-rounding hole is open and the mechanism is unestablished

`126` named double rounding across chained carrier widths as a hazard the preceding topic's discharge
apparatus does not cover. `131` took it and found three separate things, of which only two are settled.

**`122` 4.6 is not at risk** (`131` F131-4). Its rule is an equality between two arms that **both round
at every node**,
so double rounding is common to them and cancels. Measured with 4.6 implemented as written and one grid
for the whole chain: **zero differences for every mode**, nearest ones included, over 256 cells per row
with 64 to 128 cells where a node's exact result left the grid.

**What is at risk is staged-versus-direct narrowing**, and **no clause in either topic states it**.
Directed members at zero mismatches, nearest members at 16 to 32 (`131` F131-6). That is 5.3's
composition clause and it is the hazard `126` correctly identified and slightly misplaced.

**And there is a third thing, which is neither, and which is open.** Once the grid coarsens between
nodes, `122` 4.6's phrase "the grid part must be applied at the result of every node whose exact result
can leave the grid" **does not say which grid**, and the two readings differ from the
reduce-at-every-node arm on 32 to 94 cells and on 124 to 170 cells respectively (`131` F131-5, on
`131_probes/v2`). So with two grids in
play the sentence is **ambiguous rather than wrong**, and neither reading reproduces the arm it is meant
to equal.

**The mechanism is unestablished and nobody should build on a guess about it.** `131` made three
mechanism guesses and all three were wrong, at which point it stopped rather than making a fourth.
**What would decide it:** an instrument varying the grid coarsening and the span alignment
independently, which is one probe and which has not been built.

### 6.2 A figure that did not reproduce, and whose relay error is the coordinator's

Section 1.7 records it. It is repeated here because a reader meeting the vocabulary argument will meet
the figure with it: **the direction is established by arithmetic and the specific figure is not.**
Bit-drop is floor on every row measured and differs from toward-zero on signed rows only (`131` F131-3,
on `131_probes/v2`), which is what the argument needs. The count of 21,204 of 32,768 originated in `125`, was relayed into two briefs
without checking, and the coordinator attributes that to itself rather than to `125`.

### 6.3 The widening reaches 38 of 55 pins, and stating it as all of them would be the overreach

`125` F4's widening is real and `131` F131-1 verified it on the argument with a sweep as control, on
`131_probes/v1`. Applied to the preceding topic's actual pins (`131` F131-2, same probe):

```
  pins found                                            55
    widen at F = 0   (grid-closed operations)           30
    widen at any F   (additive operations)               8
    do not widen                                        14
    unclassifiable: the predicate names no operations    3
```

**So 38 of 55 widen and the restriction was real on 14.** Every one of those 14 has a multiplication at
a nonzero fraction width, which is exactly where 5.5 says the axis acts. **And the three unclassifiable
pins are a defect in those predicates**: they pin a quantisation member without naming an operation set,
so nothing can decide whether the member acts on them. They want repairing by whoever owns them, per
the never-widen-in-place rule.

---

## 7. What only op can decide

**Whether any of section 5 belongs in the canon at all, or is a design-tier matter.** My reading, and it
is a reading rather than a finding: **5.1 through 5.7 are canon** and **the pairing recommendations are
design**. The impossibility, the decomposition, what the axis selects, the vacuity region and the
coupling structure are intent, they survive a total rewrite, and they are what a design would be derived
from. "Pair a wrapping range policy with floor" is a concrete selection a design makes, it names a mode
spelling, and it would need editing when the implementation moves. That line is mine and op may draw it
elsewhere.

**How this axis meets the strategy axis.** I1 leaves the strategy set open at every level, including how
many there are and what they are called, so nothing here proposes a mapping and nothing here should be
read as implying one. What the topic establishes is that a strategy weighing accuracy across chains
(I7's concern) has exactly one quantisation member available to it, and that a strategy wanting
decorrelation without runtime entropy has exactly one. Whether those constraints belong to a strategy at
all is op's.

**The vocabulary retirement**, per C1. The arithmetic is settled and the naming is not a measurement.

**Whether a consumer-supplied-seed surface should exist.** 5.8 says randomness crosses the API boundary
or the member is deterministic. `arvo-toolbox-not-policer.md` would predict the substrate ships the
choice rather than making it, which argues for the surface existing. That is a design posture question
and I raise it rather than answer it.

---

## 8. For the three signatures this draft is written for

**To `125`.** Section 1.7 retires two clauses of your F8 and section 1.1 credits four of the six blind
convergences to `128`'s reading of your phase one rather than to your own account. Your T3, T5, T7 and
F6 sit at one expert unreproduced in section 1.5, which is accurate and is lower than a reader of your
file would guess. And section 6.2 attributes the relayed figure's error away from you; if you would
rather state its provenance yourself, that is yours.

**To `126`.** Section 1.1 records B6, the shape of the answer, as reached by you independently and named
by `129` alone, which is a credit `128`'s list does not carry. Section 1.7 retires your Finding 4's fork
framing while recording that you hedged it and `125` did not. And C1 records your vocabulary position as
contested on the naming rather than on the facts, which is a narrower disagreement than `128` framed it
as; if you read the facts differently, the ledger is where to say so.

**To `127`.** Section 1.3 places F127-1 at reproduction rather than at blind convergence, because you
built it and `128` reproduced it. Section 1.7 corrects the reach of your "dominates" sentence rather
than the sentence. And section 1.2 gives the relocation to `128` and `129` rather than to the attack
that forced it; if you think the attack deserves a rung of its own in that entry, say so, because I
considered giving it one and did not.

**To all three.** Section 1's rung column is where I am most likely to have flattered the topic. B6 in
particular rests on two authors' own accounts of what they knew when, which is the least checkable kind
of evidence in the ladder, and section 1.1 says what the commit ordering does and does not add to it.

---

## 9. Anchor accounting

Counted on `119_probes/r1`'s patterns, reused rather than rebuilt, with this section excluded from the
computation. `w1_output.txt` carries the per-class lists.

**Two patterns in `r1` were widened, and both gaps are the failure this check exists to catch.** Its
finding pattern was written for a topic that numbered everything `F<file>-<n>`, and this topic names
half its results `T1` through `T9`, so counting on `r1` alone would have silently dropped nine anchors
of the class most load-bearing here. Its probe pattern matches stems beginning `p`, `q`, `r` and `s`,
and the probes in this stretch are named `v` and `w`, so on the first run it reported zero probe
references in a file that cites four. Both are widened in `w1` rather than by editing `r1`, which
belongs to another file and is the instrument the preceding candidate was measured on.

That a pattern can under-report and still print a plausible number is the argument for the set
difference over the count, and both gaps were found by reading the lists rather than the totals.

```
  class          in the seven-file union   in 132   not carried
  finding ids                         28       25             3
  probe stems                         15       13             2
  line anchors                         7        7             3
  theorems                             9        9             0
```

The `in 132` column counts anchors of that class present in this file, so it exceeds the carried count
where this file introduces anchors of its own; `not carried` is the set difference against the union,
which is the check. **Every anchor in the three `not carried` cells belongs to the preceding topic**:
`F122-2`, `F122-4` and `F122-5` from the realisation-map candidate, probe stems `q3` and `q5` from
`118`, and line anchors `116:486`, `119:598` and `122:642`. Nothing from this topic's own seven files is
dropped.

The first run was not like this. It carried 14 findings of 28 and 2 probe stems of 15, and the missing
set included every one of `125`'s theorem-bearing findings and all three of `131`'s probes. Restoring
them was a second pass over the finished text, at the points of use rather than in a list, which is the
same two-pass shape the two preceding candidates both needed.

**The stripper fired, on the paragraph above.** Naming `F122-2`, `F122-4`, `F122-5` and the three line
anchors in order to account for them makes all six present in this file's text, so the unstripped count
reads 28 findings of 28 and 10 line anchors of 7: a clean sheet, produced entirely by the sentence
admitting it is not clean. `w1` reports `finding +3, line_panel_norm +3`, which is that gap exactly.

This is the guard's designed case and it is worth stating rather than burying, because the failure it
prevents is invisible without it. An author accounting honestly for what it dropped disables the
instrument that would have found it, and the more careful the accounting, the more complete the
disabling. The preceding candidate hit the same thing from the other direction, its accounting naming
four ids the body did not carry.

Two anchors are under-reported and I state it rather than let the zero stand: `q3` and `q5` appear in
the paragraph above as bare stems, and the probe pattern requires a filename or a `NNN_probes/` prefix,
so the stripper shows `probe_stem +0` where it should show `+2`. The direction is conservative, since
the effect is to leave them counted as not carried, which they are.

**A line anchor into shipped source is not owed the same preservation** and none is carried, because the
code tier is the one that gets rewritten and a canon that anchors to it cannot be its oracle.

---

## 10. Coverage, and the probe index

**Read in full.** `125`, `126`, `127`, `128`, `129`, `130`, my own `131`, `INTENTS.md`.

**Read in part.** `RULES.md` at the rung definitions. `122` at 4.6 and its predicates. `128:177`,
`126:511`, `125:326`, `129`'s and `128`'s coverage sections, opened.

**Not read.** `125_probes` through `130_probes` sources. Every panel file before `122` except through
`131`'s account.

**Reproduced nothing new in this file.** The candidate is a compression of `131`, which did its own
reproductions and names them. Its two probes are the anchor instrument and the commit-ordering check.

**Probes.**

- `w1_the_anchor_count_and_the_blind_commit_ordering.py`, `w1_output.txt`. The anchor inventory with a
  theorem class `r1` does not see, and the commit ordering shown rather than cited, with a statement of
  what it does and does not establish.
