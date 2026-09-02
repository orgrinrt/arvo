# The transfer argument

Leslie Lamport, file 66. I wrote file 18 (say what is claimed) and file 33 (the laws restated). Thirty
files have landed since 33 and two defects in it were found and corrected by later members; I assumed
nothing in either file still holds, did not rely on either, and where this file needs something they
said I re-derived it here.

**What I read, stated precisely.** `63_consolidation_six.md` in full, per the standing instruction that
it is the only required reading and is self-contained. The two deliverables since:
`64_chlipala_the_owed_second_reads.md` and `65_pesce_pricing_the_l0_migration.md`, both in full. I
`ls`ed the panel directory once at the start. Behind those, because this dispatch is a check of one
specific derivation and its sources: `10_leroy_what_is_actually_certified.md` sections 4 and 5 (where
the transfer argument was made, once, and where its four legs are named);
`08_fog_the_union_and_what_it_costs.md` at its exhaustion table and the paragraph after it, plus
`08_probes/README.md`; `50_fog_the_float_model.md` sections 1, 2, 5.2 and 5.3 and its closing residuals
paragraph (the source of both items in my dispatch); `45_leroy_what_each_claim_rests_on.md` at its
ground table, for the `model` and `ffl` rows. Outside the review: the ratified rule this rests on,
`.claude/rules/unstable-features.md`, at its "The forbidden list is verification infrastructure"
section, read fresh and quoted below.

**What I compiled and measured against what I reasoned.** Five artifacts in `66_probes/` with an
`OUTCOMES.md`. Sections 2, 3.2, 3.3, 5 and 6 are compile or run results and say which; every count in
them comes from a probe that ends in an assertion rather than in a printed table. Section 1 is a close
reading of two texts against each other. Sections 3.1, 3.4, 4 and 7 are reasoned, and I mark the one
place in section 3.3 where the experiment overturned my own prediction, because that is the finding
rather than an embarrassment to be smoothed. The only wall-clock numbers anywhere are compile times.

**Gates.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same with
`FullRange\|UTerm\|AddWidth` both exit 1, empty, run fresh from the repo root. `git status --short`
shows one untracked path, `mock/research/202607301300_formalization-spec-panel/66_probes/`; no file
under `mock/crates/` was edited, so no test-suite re-run is claimed and none is needed. The pin
resolves to `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, from
`rust-toolchain.toml`, confirmed fresh before any probe ran.

---

## 1. What the argument currently says, and the word in it that is wrong

The ratified text, quoted exactly, from `unstable-features.md`:

> **`specialization` and `TypeId` are what let a property checked at a small model width transfer to the
> real widths.** A design that verifies a claim exhaustively at, say, eight bits and relies on it at
> sixty-four is relying on there being no way for a type to observe which instantiation it is in and
> behave differently. Full `specialization` is exactly such a way, and `TypeId` is another. With either
> available, a check at the model width establishes nothing about the real one, because the
> implementation is free to differ there. Without them, monomorphisation is uniform and the transfer is
> sound.

Every sentence but the last is correct and I would not change a word of it. The last sentence is
false, and it is false in the precise way this review has caught three times in other places: it
promotes a necessary condition to a sufficient one.

The condition the bans buy is a statement about the *implementation*: there is one parametric function
and no instantiation can be given a different body. Call that **implementation uniformity**. What
transfer needs is a statement about the *proposition*: the truth value of the claim does not move as
the parameters move. Call that **property uniformity**. Implementation uniformity does not imply
property uniformity and cannot, because a proposition about a function's outputs quantifies over a
value set that itself moves with the parameters. One function, checked at one instantiation, says
nothing about a different instantiation's value set unless something says the two are related.

The source this rule was drawn from already knew this. File 10 gave the transfer argument **four
legs**, in decreasing order of solidity (`10`, section 5). Leg one is parametricity, enforced by the
bans, and it is the leg the rule kept. Leg two is that the per-width primitive operations are trusted
rather than checked. Leg four is that rustc's two evaluators agree. And leg three, which is the one
that matters here, reads in full:

> **Leg three: width-uniformity of phi's behaviour.** phi's classification (stable, refuses) is checked
> at the model; the claim that a rule stable at 3 bits is stable at 128 is a property of the rule's
> shape, arguable in prose per 03's frontier, and never mechanical.

"A property of the rule's shape, arguable in prose, never mechanical" is property uniformity, named
correctly, and named as unproved. The compression from four legs into one sentence kept the leg that
is mechanical and dropped the three that are not, and then attached the conclusion of all four to the
one that survived. That is how the sentence came to say "the transfer is sound".

This correction is not an artifact of the two-width shape. It applies to the single-width case that
was argued, and the two-width case merely makes it visible. So the first thing this file owes is the
correction at the level where it belongs, and the second is the argument the two-width shape actually
needs.

*grounded on: `tree` (`.claude/rules/unstable-features.md`, the "forbidden list is verification
infrastructure" section, quoted verbatim above; `10_leroy_what_is_actually_certified.md` section 5,
legs one through four, quoted at leg three), read fresh, this file.*

---

## 2. Transfer already fails, twice, with the bans in force

An argument this shape is best refuted rather than debated, and the refutations are cheap.

### 2.1 The design's own, re-derived: the radix

`63:220-223` records that a rounding tie is reachable only at an even radix, since `2 * lost == R^s`
has no solution for odd `R`. I did not take that from the formula, because the formula is the argument
and the quantiser is the thing. `probe_3_radix_is_not_uniform.rs` counts exact half-ulp ties
encountered by the model's own quantiser over every pairwise sum, at radices 2 through 13, at `p = 2`,
span 3, `Abrupt`:

| r | parity | ties | roundings |
|---:|---|---:|---:|
| 2 | even | 14 | 40 |
| 3 | odd | 0 | 318 |
| 4 | even | 226 | 1,212 |
| 5 | odd | 0 | 3,280 |
| 10 | even | 4,702 | 63,360 |
| 11 | odd | 0 | 94,270 |
| 13 | odd | 0 | 188,448 |

Every even radix reaches a tie; no odd radix reaches one; and the rounding column is what keeps the
odd rows from being vacuous, since a row with zero ties and zero roundings would be reporting that the
quantiser was never asked the question. Confirmed, independently, and I record it as a second read of
`63:220-223` rather than as a new finding.

The point for this file: `Rad<P>` ranges over every `P: AtLeastTwo` (`63:173-174`), the bans were in
force throughout, monomorphisation was uniform throughout, and the property's truth value moved with a
`Numeral` member anyway.

### 2.2 New, and it is the two-width shape's own: the exponent span

`probe_2_span_saturation_flip.rs`. The property is **absorption-freedom**: for all `x, y` in the value
set with `y` nonzero, `quantise(x + y) != x`. It is exhaustive over the whole value set at every cell,
never sampled, and it is a property the design cares about, since absorption is exactly what makes a
format-level fold order-dependent and is what `63:246`'s exact accumulator exists to escape.

Radix two, `Underflow = Abrupt`:

| p \ span | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|---|
| 2 | TRUE | TRUE | FALSE | FALSE | FALSE | FALSE |
| 3 | TRUE | TRUE | TRUE | FALSE | FALSE | FALSE |
| 4 | TRUE | TRUE | TRUE | TRUE | FALSE | FALSE |

Radix ten gives the same threshold at `p = 2` (TRUE at spans 1 and 2, FALSE from 3). The witness the
probe asserts, at `r = 2, p = 3, Abrupt, span 4`: `x = 4 * 2^1 = 8`, `y = 4 * 2^-2 = 1`, and
`quantise(x + y) = x` with `y` nonzero. One ulp of `x` is 2, `y` is exactly half of it, the tie breaks
to even, and the smaller operand vanishes.

Same precision. Same code. Same bans. `EMAX` moved by one and the property's truth value moved with
it. That is the counterexample, and it is on the axis the two-width shape adds.

*grounded on: `pin`; `66_probes/probe_2_span_saturation_flip.rs`, `probe_3_radix_is_not_uniform.rs`
(both compiled, exhaustive, this file); `tree` (`63:220-223`, `63:173-174`, `63:246`).*

---

## 3. The transfer argument, stated for the two-width shape

### 3.1 What is quantified over what

The sentence a transfer argument is about has this shape, and writing it out is most of the work:

> For every instantiation `θ` in the index set `Θ`, for every `x` (and `y`, `z`) in the value set
> `V(θ)`, `P(θ, x, y, z)` holds.

Exhaustive validation establishes the inner quantifier at one `θ₀`. Transfer is the claim that the
outer quantifier follows. So the argument has exactly two things to name: **what `Θ` is**, and **why
`P` is constant on it**.

For a `Ranged` numeral, `Θ` has six coordinates, and the design's own table
(`63:154-163`) names all six:

| coordinate | source | moves the value set how |
|---|---|---|
| `r`, the radix | `Numeral::Radix` | changes the grid's generator and, with it, which fractions are exactly representable |
| `p`, the precision | `Numeral::Precision` | refines every grid |
| `EMIN` | `Ranged`'s first parameter | moves the window's floor |
| `EMAX` | `Ranged`'s second parameter | moves the window's ceiling |
| `U`, the underflow policy | `Ranged`'s `U: Underflow` | fills or leaves the hole below `r^EMIN` |
| `S`, the specials | `Ranged`'s `S: Specials` | adds points that are not rationals at all |

The single-width argument covered one of these, `p`, for one shape, and covered it by naming a
mechanism rather than by naming a set. Six is not one, and the interesting part is that the six do not
all need the same kind of argument. Three kinds suffice, and they are of different strengths.

### 3.2 `EMIN` and `EMAX`: a symmetry, and it is a proof

The strongest ground available, and the good news in this file.

Let `σ_k` scale an exact value by `r^k`, and let `F + k` be the format with the window shifted by `k`.
The claim:

> **(E)** For every exact rational `v` and every `k`,
> `quantise_{F+k}(σ_k v) = σ_k(quantise_F v)`, with the same classification outcome.

If (E) holds then `EMIN` and `EMAX` do not appear in the index set of any claim at all. Only the span
`n = EMAX - EMIN + 1`, and each operand's position relative to the two ends, can matter. **Two of the
six coordinates collapse into one, and they collapse by an exact symmetry rather than by an argument
about a rule's shape.** This is a group action of `ℤ` on the family, and it is exact in `ℚ` because
`r^k` is exactly invertible; nothing is approximated anywhere.

`probe_1_exponent_shift_symmetry.rs` checks (E) over the whole matrix of small formats: radices 2 and
10, precisions 2 and 3, spans 1 through 4, both underflow policies, shifts of -3, -1, 1, 2 and 5,
applied to every value and to every exact pairwise sum, which is the set the quantiser is actually
applied to. **509,660,160 checks, zero failures.** Addition end to end in the shifted format:
254,830,080 checks, zero failures.

Two things this probe does that the claim needs and that a sampled version would not have supplied.

**It is not vacuous, and it says so.** A check that passes because the quantiser ignores the window
would pass identically. The negative control shifts the window and deliberately does *not* shift the
value: 8 of 13 values disagree. The window is read.

**The condition is compiled, not assumed.** (E) needs the value map to be **homogeneous** in the
exponent. An additive constant at the value level breaks it immediately, because `σ_k` does not commute
with an affine map whose constant term is nonzero. The second negative control adds one: 29
disagreements out of 51. So the symmetry is not free, it is conditional, and the condition is that no
`Numeral` member contributes a nonzero additive constant to the value. **`Ranged<EMIN, EMAX, U, S>`
carries no `Bias` member (`63:162`), so the condition holds today by construction.** It is `Implicit<E,
A, B>` that carries `B: Bias`, and `Implicit` has a single fixed exponent and no window to shift, so
the condition is satisfied where it is needed and irrelevant where it is not. That is a happy
arrangement and it should be written down as a condition rather than left as an accident, because the
day `Ranged` gains a bias-shaped member the symmetry dies silently and nothing fails.

**One corollary, and it is convergence rather than criticism.** Addition commutes with `σ_k`, so an
additive claim transfers along this axis directly. Multiplication does not: `σ_k x · σ_k y` is
`σ_{2k}(x·y)`, so a product's equivariant home is a window shifted by `2k`, not by `k`. That is exactly
what the design's own `mul_full: N1 x N2 -> mulnum(N1, N2)` already provides, since `mulnum` computes
the exponent sum at the type level (`63:338-340`). The probe checks it: quantising the product into the
`mulnum` target window shifted by `2k` agrees, 254,830,080 checks, zero failures. **The multiplicative
half of the design was already the shape the symmetry needs**, which is worth recording as a second
independent payoff of `mulnum` beyond the standard-conformance one `63:338-344` found.

### 3.3 The span: a saturation argument, with a threshold that is measured and coupled

The span does not have a symmetry. What it has is a saturation, and the honest form of the argument is:

> **(S)** `P`'s truth value depends on the span only through a quantity that stops changing past a
> stated threshold `T`, and the model's span exceeds `T`.

For the additive properties, the quantity is the largest reachable exponent difference between two
operands, and the threshold is where the smaller operand stops being able to influence the sum. My
reasoned prediction before running anything was `span >= p + 2`: absorption needs a difference of
`p + 1`, and a normal-only format offers at most `span - 1`.

**The measurement says `p + 1`, and the experiment beating the argument by one is the reason to run
it.** Round-half-even absorbs at an exact half-ulp tie when the retained digit is even, which reaches
one binade further than the strict-inequality reasoning allows. The witness in section 2.2 is exactly
that case.

**And the threshold is coupled to a second coordinate, which is the finding this section exists for.**
Under `Underflow = Gradual` the bottom grid extends `p - 1` further down, so the reachable exponent
difference is larger at every span, and absorption is present from **span 2** for every precision
checked, independent of `p`:

| `Underflow` | absorption first reachable at |
|---|---|
| `Abrupt` | span `p + 1` |
| `Gradual` | span 2, at `p` = 2, 3 and 4 |

Both rows are asserted in the probe rather than read off the table.

So the saturation threshold for the span is a function of `p` **and** of `U`. A model is adequate for
an additive claim only if its span clears the threshold its own underflow policy sets. That coupling
is not stated anywhere in the review and it is the concrete content of "the two-width transfer is a
different argument from the precision one".

**Applied to the two models the review has actually run**, and this is the part I want to be exact
about, because it would be easy to write this section as though something were broken and nothing is:

- File 50's fold check ran at `p = 4`, `e` in `[-3, 4]`, so span 8 (`50:386`). Threshold under `Abrupt`
  is 5, under `Gradual` is 2. **Clears it, with room.**
- File 50's band model ran at `p = 4` with a six-binade range (`50:636-639`). **Clears it.**

Both models are adequate, and they are adequate by luck rather than by design, because nothing told
their authors what the threshold was. The gap is not in what was checked; it is that a future member
choosing a model has no way to know that `p = 8` with a three-binade span would be a model in which
absorption is unreachable and every additive claim checked there would transfer to nothing.

### 3.4 The precision: unchanged, and it was never argued in the first place

`p` needs an argument of the third kind, and neither the single-width case nor this one has one. There
is no symmetry (refining a grid is not a group action on the family) and saturation is not generally
available (many properties genuinely change as the grid refines). What is available, per property, is
an induction: a stated reason why `P` at `p + 1` follows from `P` at `p`.

The honest position is that the design has zero such inductions written down, and that this is the
same position it was in before the float side existed. The two-width shape did not make the precision
axis worse; it made the *absence* visible by contrast with an axis that does have a clean answer.

### 3.5 What extends from the single-width argument unchanged, and why it is genuinely trivial

Worth recording as trivial rather than left as an assumption, since a later reader will otherwise
re-derive it.

**Leg one extends completely and without modification.** The `specialization` and `TypeId` bans are a
statement about the *language*: no instantiation of a generic item can be given a different body, and
no code can ask which instantiation it is in. That quantifies over type parameters generically. It
does not know or care how many numeric parameters a numeral carries, what they mean, or whether two of
them interact. A numeral with six parameters gets exactly the same guarantee a numeral with one gets,
for exactly the same reason.

So the correct statement is: **the mechanical half of the transfer argument is width-count-agnostic and
needs no extension; the non-mechanical half was never a single argument, and the two-width shape adds
coordinates to it rather than complicating it.** The bans do not get weaker when a second width
arrives. They were never doing the work that the second width makes people look for.

*grounded on: `pin`; `66_probes/probe_1_exponent_shift_symmetry.rs` and `probe_2_span_saturation_flip.rs`
(compiled, exhaustive, this file); `tree` (`63:154-163`, `63:162`, `63:173-174`, `63:338-344`,
`50:386`, `50:633-641`); reasoned for 3.1, 3.4 and 3.5.*

---

## 4. What the design must do instead: a transfer ground per claim, from a closed vocabulary of four

The replacement is small, it fits the machinery the review already has, and it is the same move
`45`'s grounding registry already made one level up. `45:166-167` gives every claim a **ground**, and
two of its rows are `model` ("bounded exhaustion at a model width, with the width stated per claim")
and `ffl` ("the forbidden-feature list ... as the transfer basis from `model` to real widths"). The
`ffl` row is the sentence section 1 corrects: it is not a transfer basis, it is one of four legs, and
it is the only leg that is uniform across claims.

So `ffl` splits. It keeps its real job, which is a precondition on every `model` claim without
exception, and it hands back the job it cannot do. A `model` claim then carries, in addition, **one
transfer ground per coordinate of its index set**, drawn from a closed vocabulary of four:

| ground | what it asserts | strength | who supplies it |
|---|---|---|---|
| `symmetry` | an exact group action carries the model instance onto every target instance, under a stated condition | a proof | the claim's author, once per axis, not once per claim |
| `saturation` | the claim's dependence on the coordinate stops changing past a stated threshold `T`, and the model's coordinate exceeds `T` | a proof given the threshold, and the threshold is a number the claim states | the claim's author |
| `induction` | the claim at `t+1` follows from the claim at `t` by a stated argument | a proof, in prose, per `10`'s leg three | the claim's author |
| `unargued` | the claim is a claim about the model instance and nothing else | not a transfer at all, and says so | nobody: this is the default a claim gets when it names no ground |

Four things about this shape, in the order they matter.

**`unargued` is the default and it must be, because that is what makes the scheme honest.** A claim
that names no ground for a coordinate does not silently inherit one. It is recorded as a fact about the
model, exactly as `10` recorded the actual-width preservation check as unavailable rather than as
assumed. This is the same discipline as `63:449`'s `unreproducible` ground: a marker whose whole job is
to be visible.

**The vocabulary is closed and it is sealed by the same rule everything else in this design is.**
`63:128-131`'s carrier-at-birth rule says a closed vocabulary a guarantee quantifies over owes its seal
and its adversary at birth, and file 64 discharged exactly that condition for `Arity` by showing an
unsealed vocabulary is forgeable in practice. A transfer ground is a vocabulary a guarantee quantifies
over. It gets the same treatment: four constructors, sealed, and the seal's own free diagnostic (the
"the following other types implement" listing this review has now found at five carriers) does the
work of telling a claim's author what the four are.

**The grounds are per axis, not per claim, wherever they can be.** `symmetry` for the exponent offset
is proved once (section 3.2) and every additive claim over a `Ranged` numeral cites it. The
threshold in a `saturation` ground is per property, but the *quantity* that saturates (the reachable
exponent difference) is per axis. So the cost of this scheme is one table for the design plus one line
per claim, not one proof per claim.

**Each ground owes a residue.** `10`'s `Total::refused()` panic is the pattern and it is the right one:
a place where, if the ground is wrong, the failure is loud and attributed rather than silent. For a
`saturation` ground the residue is nearly free, because the threshold is a number: a `const` assertion
at the model's own declaration that the model's coordinate clears the threshold it claims. That is one
line, it is checked at build time, and it would have made section 3.3's coupling visible to file 50's
author at the moment the model was declared rather than to me two stretches later.

### The spec text

Offered in the form the next consolidation could take, five sentences.

> Every claim established by bounded exhaustion at a model instance names the index set it is
> quantified over, coordinate by coordinate, and carries one transfer ground per coordinate: `symmetry`
> (an exact group action carries the model onto the target, under a stated condition), `saturation`
> (the dependence stops changing past a stated threshold the claim names, and the model clears it),
> `induction` (a stated argument from `t` to `t+1`), or `unargued`, which is the default and which
> records the claim as a fact about the model instance and nothing else. The `specialization` and
> `TypeId` bans are a precondition on every one of these and a substitute for none of them: they
> establish that one parametric function is one function, which is necessary for any transfer and
> sufficient for none. For a `Ranged` numeral the exponent offset carries `symmetry`, proved once by the
> exact scaling action on the value set and conditional on no `Numeral` member contributing an additive
> constant to the value; the exponent span carries `saturation` on the largest reachable exponent
> difference, whose threshold is `p + 1` under `Abrupt` and 2 under `Gradual`; the precision and the
> radix carry `unargued` until someone supplies better, and the radix is known to be genuinely
> non-uniform, since a rounding tie is reachable at every even radix and at no odd one. A model
> declaration states its own coordinates and asserts, at build time, that each clears the threshold of
> every `saturation` ground it will be cited for. A claim whose ground is later found wrong fails
> loudly at a named site rather than quietly at the consumer's width.

*grounded on: `tree` (`45:166-167` for the ground registry and the `ffl` row's current wording;
`63:128-131` for carrier-at-birth; `63:449` for the `unreproducible` precedent;
`10` section 5 for the residue pattern); reasoned, this file, for the scheme itself.*

---

## 5. A correction to the ratified rule's supporting measurement: the wall is a step budget, not a width

Checked because the dispatch says to check cheap factual claims including the consolidation's, and
because this particular number is the ground the entire transfer story stands on.

`63:456-458` states: "the cost quadruples per bit and rustc refuses at nine bits under a step-budget
lint, **both structural and machine-independent**, reproducing across two separate rebuilds now."

**The quadrupling reproduces and is structural.** One exhaustive const-eval sweep over all ordered
pairs of `N`-bit values, cheapest non-vacuous body, on the pin: 0.10s at 7 bits, 0.29s at 8 (2.9x),
1.05s at 9 (3.6x). It is the complexity of the quantification and it cannot be otherwise.

**The refusal at nine bits is not structural. It is a fact about the predicate.** My cheap sweep
**compiles clean at nine bits** and refuses at ten. The same nine-bit quantification with ten extra
arithmetic steps per pair refuses. `long_running_const_eval` is a step budget; the width at which it
bites is that budget divided by the per-instance step cost, and both my numbers and file 8's are
correct for their own predicates. File 8 measured **five constructors with a stability check each**
(`08:440-441`) and refused at nine; one cheap sweep reaches ten.

Nothing the rule argues changes. Exhaustive validation at a real width remains unavailable by a very
wide margin, and 2^128 pairs is not close to any budget. What changes is what a reader takes away.
"Refused at nine bits" reads as a ceiling on the *width*, and a member designing a model will read it
as a licence to check at eight and a prohibition on checking at nine. The true statement is a ceiling
on the *total step count*, which means a cheaper predicate buys a wider model, and that is directly
useful: the span coordinate in section 3.3 needs a model whose span clears `p + 1`, and whether that
model fits under the budget is a question about the predicate's cost, not about a bit count.

Proposed rewording for op, one clause: replace "rustc refuses at nine bits under a step-budget lint,
both structural and machine-independent" with "rustc refuses once the total const-eval step count
exceeds `long_running_const_eval`'s budget, which for the five-constructor stability check file 8
measured falls at nine bits and for a cheaper predicate falls one bit later; the quadrupling per bit is
structural, the width at which the budget bites is a fact about the predicate." One line, the ban
untouched, the argument strengthened rather than weakened. This is one expert's read and it wants a
second, per the standing convention.

*grounded on: `pin`, `host`; `66_probes/gen_exhaustion.sh`, `heavy9.rs`, `OUT_probe_0.txt` (compiled,
this file); `tree` (`63:456-458`, `08:440-450`, `08_probes/README.md`), all read fresh.*

---

## 6. `Underflow = Abrupt` under an unnormalised significand: one meaning, and a hole in the crossing contract

The smaller item, and it turns out to be the same kind of question.

`63:328` records the chain: no radix above two has a constant leading digit to hide, so its significand
is stored unnormalised and a value has one datum per representable exponent shift, a cohort. The
question is what `Abrupt` means there, since at radix two it is unambiguous.

**Two readings are candidates and exactly one is available, and the design's own axis split is what
decides it.**

*Reading A, datum-level.* The hole is "the data whose exponent field is at its minimum and whose
significand is below `r^(p-1)`". At radix two this coincides with the value-level reading because a
normalised significand cannot be smaller. Under an unnormalised significand it does not coincide with
anything: a datum with a small significand is an ordinary cohort member of a perfectly normal value,
and removing that class removes cohort members rather than values. **That is a fact about which datum
carries a value, which is precisely what `63:179-181` puts on `Encoding` inside `Lowering`, and which
no law may read.** So Reading A is not an `Underflow` member at all; it is an `Encoding::Canonical`
choice wearing an `Underflow` costume, and it is the same category error file 50 already caught once,
when it moved flush-to-zero off `Numeral` on the identical grounds (`50:485-488`).

*Reading B, value-level.* The hole is the interval `(0, r^EMIN)`: no nonzero value of smaller
magnitude is representable. This is a statement purely about the value set, it is the same statement
`Abrupt` makes at radix two, and it satisfies file 50's own requirement that both `Underflow` instances
"change what is representable" (`50:490-492`). Available, and it is the meaning.

So the answer is **exactly one**, not two and not none, and it needs no new mechanism. That closes the
residual as stated.

**But answering it exposes something the crossing contract does not say, and that is the real finding
here.** `63:186-191` gives three statements: `decode ∘ encode = id` on values; `encode ∘ decode`
idempotent on data; `encode ∘ decode = id` on data iff the encoding is injective. All three are
round-trip statements. **None of them says that `decode` lands inside the value set.** At radix two
that is free: a normalised encoding cannot name a value in the hole. Under an unnormalised significand
it is not free, because the exponent field must reach `EMIN - p + 1` for the smallest normal value to
have a full-precision representation, and the datum `(m = 1, Q = EMIN - p + 1)` then names
`r^(EMIN-p+1)`, which is strictly inside the hole.

`probe_4_abrupt_under_unnormalised.rs` measures the whole two-by-two matrix, counting data whose
decoded value is not in `V(N)`:

| radix | p | `Underflow` | encoding | data | escaping |
|---:|---:|---|---|---:|---:|
| 2 | 3 | `Abrupt` | normalised | 12 | 0 |
| 2 | 3 | `Abrupt` | unnormalised | 21 | **4** |
| 2 | 3 | `Gradual` | unnormalised | 21 | 0 |
| 10 | 3 | `Abrupt` | normalised | 2,700 | 0 |
| 10 | 3 | `Abrupt` | unnormalised | 2,997 | **108** |
| 10 | 3 | `Gradual` | unnormalised | 2,997 | 0 |

**Exactly one cell of the matrix leaks, and it is `Abrupt` with an unnormalised significand.** Under
`Gradual` the same data decode into the subnormal grid, which is in `V(N)`, so nothing escapes and the
question never arises. At radix ten with three digits, 108 of 2,997 data, 3.6%, decode to a value the
numeral does not have.

**The fix is one statement, and it belongs on the crossing contract rather than on `Underflow`.** Add,
in front of the existing three:

> 0. `decode` is total into the value set: every datum denotes a value the numeral has.

At radix two under either underflow policy this is free and a reader will wonder why it is written
down. Under an unnormalised significand with `Abrupt` it is an obligation on the encoding, discharged
by excluding the data whose value falls in the hole, and it is the first configuration in this design
where the three existing statements do not imply it. Stating it costs a line and it turns a silent
configuration-dependent hazard into a condition an encoding either meets or does not.

**A second, smaller observation a consumer will want and the spec should offer.** At radix two,
`Abrupt` buys something besides the hole: the minimum-exponent encodings with a sub-normalised
significand become unused, which is how flush-to-zero hardware reuses them and why abrupt underflow has
a performance story. Under an unnormalised significand that dividend does not exist, because those
encodings are cohort members of normal values and were never free. So `Abrupt` on a decimal `Ranged`
numeral is a pure representability restriction with no encoding or performance return. That is not an
argument against offering it, and per `arvo-toolbox-not-policer.md` the axis stays available. It is an
argument for saying so where the axis is documented, so a consumer reaching for `Abrupt` out of habit
learns at the point of choice what it costs and what it does not buy.

This is one expert's reading and it wants its second, per the standing convention, with the specific
instruction that the second read check statement 0 against the two operations `63:338` carves out as
datum-dependent (`quantize`, `roundToIntegralExact`), which I did not examine.

*grounded on: `pin`; `66_probes/probe_4_abrupt_under_unnormalised.rs` (compiled, whole matrix, this
file); `tree` (`63:179-181`, `63:186-191`, `63:328`, `63:338`, `50:476-492`), read fresh; ratified rules
(`arvo-toolbox-not-policer.md`, the "Warn but never police" section).*

---

## 7. Where I checked myself, because this file's subject is the shape of error it could most easily commit

My brief names the failure precisely: an argument that quantifies over a set the design does not range
over, and therefore proves something true and useless. Three checks, stated so a later reader can
attack them.

**Is (E) vacuous?** It would be if the quantiser ignored the window. Negative control: window shifted,
value not, 8 of 13 values disagree. It does not ignore it.

**Is (E)'s condition load-bearing or decorative?** It would be decorative if the symmetry survived an
affine value map. Second negative control: 29 of 51 disagree with a nonzero bias. It is load-bearing,
and `Ranged` satisfies it by carrying no `Bias`.

**Is the radix result "no ties at odd radices" a statement about a set the quantiser never visits?** It
would be if the odd-radix rows had no roundings at all. They have 318 at `r = 3` and 188,448 at
`r = 13`. The quantiser was asked and answered.

And one I could not close, which I record rather than smooth. **The span threshold `p + 1` is measured
over `p` in 2 to 4 and spans up to `p + 4`, at radices 2 and 10.** It is a small window, the reasoning
behind it is clean (an operand influences a sum until it falls below half an ulp, and half an ulp is
`p` binades down), and the measurement corrected the reasoning by one, which is why I trust the
measurement over the reasoning. But I have not proved the threshold in general and I am not going to
assert it as one. Written as a `saturation` ground under section 4's scheme, it is a threshold a claim
states and a model asserts against, and if it is wrong by one somewhere the assertion is where that
surfaces. That is the point of making it a number rather than a belief.

---

## What a consolidation could take, close to verbatim

The transfer from a model instance to a real one has never rested on what the ratified rule says it
rests on. `unstable-features.md`'s sentence "without them, monomorphisation is uniform and the transfer
is sound" promotes a necessary condition to a sufficient one, and the file it was drawn from knew
better: `10` gave the argument four legs, of which the bans are one, and named leg three, the
uniformity of the property itself, as "a property of the rule's shape, arguable in prose, never
mechanical". The compression kept the mechanical leg and attached all four legs' conclusion to it.
Transfer already fails twice with the bans in force. A rounding tie is reachable at every even radix
and at no odd one, re-derived here from the quantiser rather than the formula over radices 2 through
13 with the roundings counted so the odd rows are not vacuous; and absorption-freedom, which is
exhaustively TRUE at span `p` and FALSE at span `p+1` with the precision, the code and the bans all
held fixed, so a property's truth value moves when `EMAX` moves by one. For the two-width shape the
correct argument is not one argument at all. The exponent offset carries an exact symmetry: the
quantiser commutes with a scaling of the value by `r^k` when the window shifts by `k`, checked over
509,660,160 instances with zero failures across two radices, two precisions, four spans, both underflow
policies and five shifts, with two negative controls that both disagree, so `EMIN` and `EMAX` leave the
index set entirely and only the span survives, conditional on no `Numeral` member contributing an
additive constant to the value, which `Ranged` satisfies by carrying no `Bias`. Multiplication is
equivariant only into a window shifted by `2k`, which is exactly the shape `mulnum` already computes, a
second independent payoff from the multiplicative half. The span carries a saturation argument whose
threshold is measured rather than argued, `p + 1` under `Abrupt` and 2 under `Gradual` for every
precision checked, so the threshold is coupled to a second coordinate and a model's underflow policy is
part of its own adequacy; both models this review has run (file 50's fold at `p = 4` span 8, its band
at `p = 4` and six binades) clear it, by luck rather than by design, since nothing told their authors
what the threshold was. The precision and the radix carry nothing, exactly as before, and the two-width
shape did not make that worse, it made the absence visible. The mechanical half of the argument extends
to any number of widths without modification, because the bans are a statement about the language and
do not know how many parameters a numeral has; recording that as trivial is worth a line so nobody
re-derives it. What the design owes instead is one transfer ground per coordinate of a claim's index
set, from a closed sealed vocabulary of four, `symmetry`, `saturation`, `induction` and `unargued`,
with `unargued` the default so a claim that names nothing is recorded as a fact about its model and
nothing else, and with each ground owing a residue that fails loudly if it is wrong, which for a
saturation ground is a build-time assertion that the model clears the threshold it cites. Separately,
the rule's own supporting measurement needs one clause corrected: the quadrupling per bit reproduces
and is structural, but the refusal is not a fact about nine bits, it is a step budget, and a cheap
single sweep compiles clean at nine and refuses at ten while the same nine-bit sweep with a heavier
body refuses, so a cheaper predicate buys a wider model and a member choosing one should know that.
Finally, `Underflow = Abrupt` under an unnormalised significand has exactly one available meaning, the
value-level hole in `(0, r^EMIN)`, because the datum-level reading is a fact about which datum carries
a value and therefore an `Encoding::Canonical` choice rather than a `Numeral` member, the identical
category split file 50 already made when it moved flush-to-zero off `Numeral`. Answering it exposes a
gap in the crossing contract: its three statements are all round trips and none of them says `decode`
lands inside the value set, which is free at radix two and is not free in exactly one cell of the
matrix, `Abrupt` with an unnormalised significand, where 108 of 2,997 data at radix ten with three
digits decode into the hole. The fix is one statement in front of the existing three, that `decode` is
total into the value set, plus a note where the axis is documented that `Abrupt` on a decimal numeral
is a pure representability restriction whose encoding-space dividend at radix two does not exist there.
