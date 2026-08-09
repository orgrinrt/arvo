# 18: Say what is claimed

**Reviewer:** Leslie Lamport (specification lens: write down what the system is supposed to do, as a
mathematical object, before anything implements it, and treat every quantifier as load-bearing until
shown otherwise).

**What I read.** `16b_op_design_the_shape_not_the_code.md`, `16c_op_the_downstream_contract.md`,
`16d_op_the_spirit_outranks_all.md`, `17b_op_checkpoint_six.md` and `13c_op_the_standard_and_the_mode.md`
first, as the brief directs. Then `11_current_shape_draft.md` in full, then the dive in order:
`13_mcsherry_where_the_laws_belong.md`, `14_dolan_which_algebra_is_this.md`,
`15_willsey_what_a_law_is_for.md`, `16_fallin_laws_as_backend_licences.md`,
`17_orchard_are_these_all_grades.md`, and the probe directories for 13, 14 and 17. I listed the panel
directory before reading inside it, per the standing instruction. On source I read almost nothing by
design: I ran three greps over `mock/crates/` to check claims in the draft and in files 13 and 14 before
reasoning from them, and section 0 reports what one of them found.

**What I compiled and ran**, as distinct from what I reasoned about. Four probes, committed at
`18_probes/01` through `04`, all `rustc -O` against the pinned toolchain, plus a re-run of
`17_probes/07`. The full arvo suite, `cargo test --workspace` in `arvo/mock`: **654 passed, 0 failed, 9
ignored**, which reproduces file 13's figure exactly. Every number in sections 2 through 6 comes out of
those probes and is cited to the probe that produced it. Everything else is argument, offered as
directions rather than rulings, and where I hold more than one reading I say so and leave the choice
where it belongs.

**On the test gate.** The suite is green and the surface I was dispatched into is not in it. Greps for
`Magma`, `Semigroup`, `Monoid`, `AddAssoc`, `Associative`, `Commutative`, `Idempotent`,
`DistributesOver`, `Faithful`, `Kleene`, `Deterministic` and `ConstantTime` over `mock/crates/` return
zero hits each, confirming what files 12 through 16 each reported. So there is no law machinery to audit
and no law test to read, and the gate has nothing to refuse over. I note it rather than skip it, because
"the suite is green" and "the suite says nothing about the subject" are both true here and the second is
the one that matters.

## 0. Two premise checks, and the second one changes what two members were building on

**The brief's measurement reproduces exactly.** I rebuilt and ran `17_probes/07_the_grade_is_the_diameter.rs`
rather than take the number: `Refuse` at a five-element fold over a signed three-bit model, 32768 inputs,
14 tree shapes, gives max gap 0, worst answer cardinality 1, and 10992 grouping-dependent refusals.
Signed `Saturate` on the same instrument gives max gap 7 at `[3, 3, 2, -4, -4]`. The finding this dive is
extending is real and I am reasoning from a re-run, not a citation.

**The draft's claim that a `Monotone` law implementation ships is false, and two members reasoned from
it.** `11_current_shape_draft.md:699-703` says "The one shipped `Monotone` law implementation only covers
the 'nearest, with some tie rule' family of rounding rows". Nothing named `Monotone` exists in
`mock/crates/` at all. The two hits in the whole tree are
`arvo-graph/tests/waist.rs:46` (a test function named `no_waist_when_width_monotone`) and
`arvo-sparse/src/csr.rs:325` (a doc comment about `row_ptr` being non-decreasing). Neither is a law
implementation and neither has anything to do with rounding.

What does exist is a proposal in a design-round topic file, at
`mock/design_rounds/202607301100_topic.the-formalization-talk.md:1113`, carrying **Status: open**:

```rust
impl<T: Direction> Monotone for (TowardNegative, T, TowardPositive) {}
```

File 13 built on the draft's wording (`13_mcsherry...md:194-196`, "`Monotone` already exists as a partial
implementation on the rounding side (draft 5.1)") and file 14 built on file 13
(`14_dolan...md:419-428`, an open item about whether "the draft's existing, differently-scoped use of the
same word" unifies with a new one). The word "shipped" did work in both. It should not have, and the
correction is not cosmetic: section 7 shows the impl above is not merely unshipped, it is false for
compositions it admits.

I am reporting this under the brief's own instruction to check a factual claim before reasoning from it,
not as an audit of code nobody is defending. The claim is in the spec, the spec is the subject, and the
spec is wrong about its own state.

## 1. What a law claim consists of, written out

The design is accumulating properties faster than it is defining them. `AddAssoc`, `Monotone`,
`AddClosed`, `Faithful`, `Deterministic`, `ConstantTime`, translation stability, and now partial
associativity are each asserted somewhere in prose, checked by a different mechanism or by none, and at
least three of them were found on inspection to mean something other than what they said. That is not a
sequence of unrelated mistakes. It is what happens when claims are written in a form that does not force
their quantifiers to be visible.

So before proposing anything, here is the form. Every claim of the kind this design makes about
regrouping has six parameters, and the current text pins two of them.

| parameter | what it ranges over | what the draft currently says |
|---|---|---|
| `Op` | the operation the law is about | implicit, and addition is assumed throughout |
| `S` | the numeral the operands inhabit | the composition, via the ten axes |
| `A` | the numeral the running intermediate inhabits | not named anywhere, in any axis or law key |
| `n` | the arity, how many values are folded | not named; every worked case is binary |
| `G` | which groupings are quantified over | not named; "regrouping" is used informally |
| `R` | the relation two answers are compared under | Kleene equality, at `11_current_shape_draft.md:266-270` |

A claim is then, in full:

> For every arity `n` in `N`, every pair of groupings `g` and `h` in `G(n)`, and every tuple
> `x₁ … xₙ` of values of `S`: `fold[Op, A, S]_g(x) R fold[Op, A, S]_h(x)`.

Ordinary associativity is the instance `n = 3`, `G(3)` both trees, `A = S`, `R` equality on total values.
That instance is so familiar that the other four parameters read as scenery, which is exactly why they
went unwritten. Sections 3, 4 and 6 show that three of the four are load-bearing here, in the sense that
moving one flips the verdict for a preset the design has already made a decision about.

Two remarks on the form itself before using it.

**`R` is a parameter, not a fact.** There is no canonical equality on partial or approximate values. The
design chose one, Kleene equality, in a sentence that reads as a technicality
(`11_current_shape_draft.md:266`, "under Kleene equality for the refusing case"), and the choice decides
`Precise`'s entire law. Section 2 works through the candidates.

**`A` is not a lowering concern, although it looks like one.** Applying the draft's own sorting test at
`11_current_shape_draft.md:139-142` to the accumulator: changing it changes the arithmetic performed, so
by the design's own test it is `Policy`, not `Lowering`. Section 6 measures how much it changes.

## 2. The three relations, what each buys, and what each costs

There are three relations in ordinary use on values drawn from `V + {undefined}`, and the review has been
using the name "Kleene equality" for the whole family. Writing `⊥` for undefined:

**Kleene equality**, `x ≃ y`, holds when both are `⊥`, or both are defined and equal. It is reflexive,
symmetric and transitive, so it is a genuine equivalence relation and every ordinary equational habit
survives under it.

**Existential equality**, `x ≐ y`, holds when both are defined and equal. It is symmetric and transitive
and **not reflexive**: `⊥ ≐ ⊥` is false. It is a partial equivalence relation, and the failure of
reflexivity is not a technicality, because it means you cannot write "the fold equals itself" and cannot
substitute a term for itself inside a larger claim without first knowing the term is defined.

**The refinement order**, `x ⊑ y`, holds when `x` is `⊥`, or both are defined and equal. It is a partial
order, the flat domain ordering, and Kleene equality is exactly `⊑` in both directions.

`18_probes/01_three_equalities_and_two_quantifiers.rs` measures each resolution against all three, at
every arity from 2 to 6, exhaustively over a signed three-bit model. The signed table:

| resolution | Kleene | existential | left ⊑ all | all ⊑ left | diameter | definedness disagreements |
|---|---|---|---|---|---|---|
| `Wrap` (`Hot`) | yes | yes | yes | yes | 0 | 0 |
| `Saturate` (`Warm`/`Cold`) | NO | NO | NO | NO | 7 | 0 |
| `Refuse` (`Precise`) | NO | yes | NO | NO | 0 | 10992 |
| `SubstituteZero` | NO | NO | NO | NO | 7 | 0 |

(rows at `n = 5`; the full arity sweep is in the probe output, and the pattern is stable from `n = 3`
upward. `Refuse`'s definedness disagreements grow 80, 1100, 10992, 96152 across `n = 3` to `6`.)

Three things fall out, and the third is the one I did not expect.

**No single relation separates the four.** Kleene separates `Wrap` from everything else. Existential
separates `Refuse` from `Saturate` and `SubstituteZero`. Neither separates `Saturate` from
`SubstituteZero`, which the numeric diameter does not either at this width, and which
`14_dolan...md:154-161`'s decoding does. So a law vocabulary with one relation in it is describing one
column of a four-column table, whichever column it picks.

**Existential equality is what `Precise` has, and it is not an equality.** Every grouping of a `Precise`
fold that returns at all returns the same number, at every arity measured. That is file 17's finding and
it reproduces. But the relation under which it is stated is not reflexive, so a law stated under it does
not license the substitutions people expect a law to license. Concretely: from "every defined grouping
agrees" you may not conclude "the four-way split agrees with the sequential fold", because the sequential
fold might be the one that refused. That is not pedantry, it is the next paragraph.

**Neither direction of the refinement order holds for `Refuse`, so the obvious contract is unavailable.**
The natural thing to want, and the thing I went looking for, is "the regrouped answer refines the
sequential one", meaning a combinator may refuse more often than a sequential fold but never returns a
different number. The probe says no: `left ⊑ all` and `all ⊑ left` both fail from `n = 3`. Refusal is not
monotone in grouping in either direction. `(127 + 1) + (-1)` refuses where `127 + (1 - 1)` returns, and
`(-1 + 1) + 127` returns where `-1 + (1 + 127)` refuses, so the sequential fold is sometimes the more
defined grouping and sometimes the less.

The consequence for the design is a contract clause, not a law: a combinator that regroups over a
refusing composition **cannot promise anything about definedness relative to the sequential answer**, and
must therefore declare which of two things it does. Either it refuses whenever any grouping in its
schedule would refuse, which is file 17's proposal at `17_orchard...md:373-376` and which is sound but is
a different function from the sequential fold, or it commits to one schedule and its definedness is
whatever that schedule's is. The design cannot derive this. It has to be stated in the combinator's
contract, and section 11 is where I put it.

**What each choice of `R` buys and costs, stated plainly.** Kleene is the right relation for a *statement*
you want to reason equationally with, and it is the honest one to publish, because it reports
`Precise` as failing and `Precise` does fail at the thing Kleene measures. Its cost is that it fuses two
independent facts and reports the conjunction, so a consumer who only needs the numeric half cannot see
it. Existential is the right relation for the numeric half specifically, and its cost is that it is not
an equality and cannot be reasoned with as one. The refinement order is honest and would be the best of
the three if it held, and it does not hold here, which is a measurement rather than a preference.

So my reading, and I hold it against the alternative below: **the design should not pick one relation. It
should name the two facts separately**, because that is what the measurement says they are, and because
`Refuse` and `Saturate` differ in exactly which one they have. Fusing them under Kleene equality is what
put `Precise` in the same column as signed clamping, and file 17 is right that they do not belong there.

The alternative I hold, and it is not weak, is file 17's own
(`17_orchard...md:378-383`): a fold whose *definedness* depends on grouping is unusable regardless of
what the values do, because a consumer who regroups for speed and then gets a refusal has been surprised
in a way no numeric guarantee compensates for. Under that reading the fusion is correct and one relation
is right. The choice turns on whether a refusal is an answer or the absence of one, and that is a
question about what `Precise` is for.

## 3. Partial associativity: the binary form is not the statement, and the gap is measurable

File 17 named "partially associative, meaning associative on its domain of definedness"
(`17_orchard...md:371-373`) and called it a standard notion, which it is. But every statement of it in
the review, including that one, is the **binary** statement: for all `a`, `b`, `c`, if both bracketings
are defined then they are equal.

The thing a combinator needs is the **n-ary** statement: over the Catalan-many groupings of an
`n`-element fold, any two that are defined agree. For a total operation these are the same claim, because
the generalised-associativity proof rewrites any grouping into any other through a chain of single
applications of the binary law. For a partial operation that proof does not go through: every step of the
chain passes through an intermediate grouping, and an undefined intermediate breaks the chain, so two
groupings at the ends of a broken chain can be defined and disagree while every binary instance holds.

That is an argument, and this review's own record is that arguments in this area survive or fall on
whether someone compiles them. `18_probes/02_existential_associativity_does_not_lift.rs` searches every
partial magma on a three-element carrier, 4⁹ = 262144 tables, checks binary existential associativity
over all 27 triples, and for those that pass checks all five groupings of every one of the 81 quadruples:

```
existentially associative (binary):            31789
  of those, Kleene associative (binary):       442
  of those, with a DEFINED disagreement at 4:  6072
```

Nineteen percent of the binary-existentially-associative partial magmas on three elements have two
defined, disagreeing groupings of four elements. The witness the probe prints:

```
  operation table (rows = left operand, `_` = undefined):
           0   1   2
      0    2   2   0
      1    0   _   _
      2    _   _   _

  input: a=0 b=0 c=1 d=0
    ((ab)c)d   = _
    (a(bc))d   = 2
    a((bc)d)   = _
    a(b(cd))   = 0
    (ab)(cd)   = _
```

Two defined groupings, values 2 and 0, and the three intermediates that would connect them are all
undefined.

**So the arity quantifier is load-bearing and cannot be left implicit.** A design that names "partially
associative" and states it binary, then uses it to license a four-way accumulator split, has an unproven
step in exactly the place the review has already twice found unproven steps. And the fix is not to state
it at every arity, because the statement then quantifies over an unbounded set that no exhaustive check
reaches. The fix is section 4.

I want to be precise about what this does not show. It does not show that arvo's `Refuse` addition is in
the bad region. It is not. What it shows is that the binary law is not the *reason* it is not, so
whatever the design writes down had better be the actual reason.

## 4. What does the lifting is a property of the recovery map, and it is derivable rather than checkable

`18_probes/03_the_reason_is_a_property_of_phi.rs` classifies each recovery map structurally, then checks
whether the class predicts the law. Three properties, each checked over the exact domain rather than
assumed:

A map `phi` is a **homomorphism** when `phi(x + y) ≃ phi(phi(x) + phi(y))` for every exact pair, which is
the two-sided form of the draft's own translation-stability identity. It is a **partial identity** when,
wherever it returns at all, it returns its argument unchanged. It is a **retraction** when it is total,
fixes the representable set pointwise, and preserves order.

| resolution | homomorphism | partial identity | retraction | Kleene assoc, n = 2..6 | existential assoc, n = 2..6 |
|---|---|---|---|---|---|
| `Wrap` (`Hot`) | yes | no | no | yes at every n | yes at every n |
| `Saturate` (`Warm`/`Cold`) | no | no | yes | no from n = 3 | no from n = 3 |
| `Refuse` (`Precise`) | no | yes | no | no from n = 3 | **yes at every n** |
| `SubstituteZero` | no | no | no | no from n = 3 | no from n = 3 |

The class predicts the law, and each prediction has a one-line proof that mentions no width, no arity and
no search:

**A homomorphism gives Kleene associativity at every arity**, because every grouping evaluates to `phi`
of the exact sum. Push `phi` outward through each step by the homomorphism property; the exact sum does
not depend on grouping; done.

**A partial identity gives existential associativity at every arity**, because every grouping that
returns at all returns the exact sum unchanged. Two groupings that both return therefore return the same
number. Definedness is untouched by the argument, which is exactly why Kleene fails and existential does
not.

**A retraction gives neither**, in general, and the signed clamping row is the instance. Dolan already
proved the same boundary from ordered-algebra (`14_dolan...md:192-201`), reaching it as "quotient by a
subgroup, or retract onto a convex sublattice, and the third option is not to complete at all". The
classification here is the same three options read as a specification device rather than as a fact about
what completions exist.

**And the partial-identity theorem does not depend on the shape of the domain.** If it needed the domain
to be an interval, it would be a weaker theorem than the one worth stating, so the probe checks it over
every subset. All 256 subsets of the eight-value model, crossed with arities 2 to 5, 1024 pairs: every
defined grouping agreed, every time. 633 of the 1024 also happened to be Kleene-associative, which is a
fact about which subsets are closed and is not what is being claimed.

### 4.1 What this does to the ledger, which is the part I would act on

`11_current_shape_draft.md:826-828` puts these facts in the bin "machine-checked by bounded exhaustion at
a small model width", and `11_current_shape_draft.md:840-842` records the width-uniformity transfer
argument as something that "stays prose forever, is never mechanical". That is honest and it is correct
about the mechanism the design currently plans.

It is not correct about the mathematics. Two of those rows do not need a check at any width, because they
follow from a property of the recovery map that the design can read off a constructor. So the proposal is
narrow and concrete: **classify the recovery map, derive the law from the class, and check only the
classification.** The classification is a statement about `phi` alone, at a single argument, which is a
far smaller exhaustive check than a fold over `n` operands, and `08_fog_the_union_and_what_it_costs.md`'s
wall (quadrupling per bit, 28.45 seconds at eight bits, refused at nine, cited by
`14_dolan...md:180-182`) is a wall this move mostly steps around rather than climbs.

There are two costs and I do not want them buried.

The classification itself still needs checking, and it is still bounded exhaustion at a model width, so
the transfer argument does not disappear, it shrinks. What it shrinks to is a statement about a unary
function rather than about an n-ary fold, which is a genuinely smaller thing to be trusting.

And the derivation step, "homomorphism implies Kleene associativity at every arity", is a mathematical
argument that lives in prose. It joins the draft's own trusted bin at
`11_current_shape_draft.md:832-834` ("the statement of the checked identities themselves, roughly thirty
lines: a wrong statement of a theorem certifies the wrong thing everywhere it is used"). So this trades
a large mechanical check for a small mechanical check plus a short proof. Whether that is the right trade
depends on how much you trust a three-line argument against a compiled exhaustive sweep, and I hold that
the argument is more trustworthy here precisely because it is three lines and quantifies over everything,
where the sweep is thousands of cases and quantifies over one width. Someone could reasonably hold the
opposite, and the honest form is to record the derivation next to the check rather than instead of it.

## 5. Where the property stops, and it stops at multiplication

The partial-identity argument has a precondition that nobody has stated, and stating it makes a
prediction that turned out to be testable.

For `phi` to be a partial identity **on an operation**, the exact result of that operation applied to two
representable values has to be representable whenever it is in range, so that the in-range rounding never
fires. For addition on a fixed-point numeral that holds, for a reason that fits on one line: the sum of
two multiples of the quantum is a multiple of the quantum, so the representable set is a subgroup of the
exact one under addition.

For multiplication it fails, by the same line read backwards: a product of two values carrying `F`
fractional bits carries `2F`, so it is generically not a multiple of the quantum, and the in-range
rounding fires on ordinary inputs long before any range boundary is reached.

`18_probes/04_where_the_partial_identity_stops.rs` models `Precise` as the preset table describes it
(`11_current_shape_draft.md:327`: nearest ties-to-even in range, refuse out of range), on a signed Q2.2
numeral, which is the first time anyone in this review has measured `Precise` multiplication at all.
File 15's probe covers wrapping and saturating multiply against the shipped truncating body, which is a
different function.

```
is `phi` a partial identity, i.e. does the in-range rounding ever fire?
  operand pairs:                       256
  addition,       rounding fired on:   0
  multiplication, rounding fired on:   128
```

| operation | n | Kleene | existential | diameter |
|---|---|---|---|---|
| `Precise` + | 3 | false | true | 0 |
| `Precise` + | 5 | false | true | 0 |
| `Precise` * | 3 | false | **false** | 2 |
| `Precise` * | 5 | false | **false** | 5 |

Witness at `n = 3`: inputs `[-0.5, -0.75, -2.0]`, groupings returning `-1.0` and `-0.75`, a quarter of a
unit apart on a range four units wide.

**So `Precise`'s law is not a property of `Precise`.** It is a property of the pair `(Precise, addition)`,
and the design's law key has to carry the operation for the same structural reason it has to carry the
numeral. That is a stronger statement than "multiplication is untested", which the draft already says at
`11_current_shape_draft.md:776-779`. It says the property currently being written down for `Precise`
is one that multiplication is known in advance not to have, so any vocabulary keyed on the composition
alone will assert a false thing about multiplication the moment multiplication exists.

It also predicts, without further measurement, exactly which multiplication cases would recover the
property: any composition where the product lands on the representable lattice. `Growth::Exact` with an
accumulator carrying `2F` fractional bits is the case, and it is the same accumulator observation as
section 6, arriving from the operation side.

## 6. The accumulator is a quantifier, and it moves the answer for two presets

Nothing in the ten axes names the numeral the running intermediate of a fold inhabits. `Growth` is close
and is not it: `Growth` says how much of the exact result to keep *within* one operation
(`11_current_shape_draft.md:163`), and the accumulator question is what type the value has *between*
operations. The draft's own open item at `11_current_shape_draft.md:688-692` ("the spec never states
whether quantisation fires per operation or is deferred") is the same gap seen from the other side, and
it correctly notes that "the two readings give different associativity answers for the identical
composition".

`18_probes/01`, section 3, measures how much. Same operand numeral, signed `[-4, 3]`, `n = 5`, four
accumulator windows, where scale 1 holds the running value at the operand numeral and scale 5 is wide
enough that no intermediate can leave it:

| resolution | acc x1 | acc x2 | acc x3 | acc x5 |
|---|---|---|---|---|
| `Wrap` (`Hot`) | Kleene, diam 0 | Kleene, diam 0 | Kleene, diam 0 | Kleene, diam 0 |
| `Saturate` (`Warm`/`Cold`) | diam 7 | diam 3 | Kleene, diam 0 | Kleene, diam 0 |
| `Refuse` (`Precise`) | 10992 refusals | 558 refusals | Kleene, diam 0 | Kleene, diam 0 |
| `SubstituteZero` | diam 7 | diam 7 | diam 4 | Kleene, diam 0 |

Signed saturating addition goes from regrouping diameter 7 to Kleene associativity with **no axis
changed**. So the draft's stated consequence at `11_current_shape_draft.md:334-338`, "Only `Hot` folds
(has a true `AddAssoc`) for signed values", is not a fact about `Hot` and `Warm`. It is a fact about a
`(numeral, accumulator)` pair in which the accumulator was silently taken to be the numeral itself.

The mechanism behind the threshold is worth stating because it is the thing to specify. The failures at
scale 2 are cases where a clamped intermediate is brought back **inside** the numeral's range by a later
addition, so it disagrees with the path that never clamped. Once the accumulator is wide enough that a
clamped intermediate can never re-enter the numeral's range, every clamped path collapses to the same
settled value and the disagreement is gone. That is a condition on widths and arity, not on values.

**And a condition on widths and arity is expressible in this language.** The safe form,
`acc_width ≥ numeral_width + ceil(log2 n)`, guarantees no intermediate of an `n`-element fold overflows
the accumulator at all, and it is a const-generic bound over parameters a combinator already has: `n` is
the unroll factor of a fixed-arity split, which is a const. No dependent types, no `generic_const_exprs`,
just an inequality on consts. It is weaker than the measured threshold (scale 3 sufficed where the safe
form asks for scale 8) and it is sound, which is the right side to err on for a bound the compiler
enforces.

I hold two readings on what to do with this and I resolve neither.

**Reading one: the accumulator becomes visible in the combinator, not in the axes.** A fold is
`fold<E, A>(xs: &[E], ...) -> A` and always was; the design just never wrote the second parameter down.
Under this reading nothing is added to the ten axes, the law key gains `A`, and the whole McSherry
over-strictness finding (`13_mcsherry...md:230-234`, 1024 to 1) partly dissolves, because the presets
that "do not fold" fold perfectly well into a wide enough accumulator. This is the cheaper move and it
keeps the axis table as it stands, which `16d` says is worth something on its own.

**Reading two: the accumulator is an eleventh `Policy` axis.** By the draft's own sorting test at
`11_current_shape_draft.md:139-142`, changing it changes the arithmetic performed while leaving the
representable set alone, which is the definition of `Policy`. Under this reading a composition carries
its own accumulation discipline, a consumer selects it once rather than at every fold, and the law is
derivable from the composition alone again. The cost is an eleventh axis on a design that has spent this
round arguing its ten are the right ten, and a second cost is that it puts a fold-shaped concept inside a
type that is also used for values that are never folded.

What I would not do is leave it where it is, because where it is, is a law stated about an operation
whose intermediate type is unspecified, and that law does not denote.

## 7. The other properties, restated precisely, and what survives

The brief asks what happens to the accumulated properties when they are restated at this precision. Going
through them.

### 7.1 `Monotone`, which does not survive as written

Section 0 established that the only written `Monotone` implementation is the blanket impl at
`202607301100_topic.the-formalization-talk.md:1113`, quantified over the three midpoint members of a
five-member quantiser. The two range members are not mentioned.

`18_probes/03`, section C, holds the triple at exactly the rows that impl admits, `(TowardNegative, T,
TowardPositive)` for `T` in `{ToEven, ToOdd, AwayFromZero}`, and varies only the range members. Quantum
4, representable set `{-8, -4, 0, 4, 8}`, exact domain `[-24, 24]`, monotonicity checked under the
generous existential reading that skips pairs where either side refuses:

| over-range | under-range | monotone | counterexample |
|---|---|---|---|
| `Clamp` | `Clamp` | yes | |
| `ReduceModulo` | `ReduceModulo` | **NO** | `phi(-24) = -4 > phi(-8) = -8` |
| `SubstituteZero` | `SubstituteZero` | **NO** | `phi(-24) = 0 > phi(-8) = -8` |
| `Refuse` | `Refuse` | yes | |
| `Clamp` | `ReduceModulo` | **NO** | `phi(-24) = -4 > phi(-8) = -8` |

Identical for all three tie rules. So the impl is not imprecise about its scope. **It is false for
compositions it admits**, and it admits them because its premise names three of the five members that
decide the answer. Dolan's torsion-group argument (`14_dolan...md:163-181`) already proves the
`ReduceModulo` row can never be monotone at any width; the impl asserts it anyway, from a premise that
cannot see it.

And nothing anywhere could catch this, for the reason file 17 found on a different surface
(`17_orchard...md:285-310`): a marker impl with no associated items has no body to be wrong in, so a
corrupted or over-broad grant compiles clean. That finding arrived there on the fidelity licence and op
adopted a witness for it at `17b`. It applies verbatim here, to the only law implementation the design
has written down, and section 9 is where I put the general form.

The other confusion Dolan flagged (`14_dolan...md:419-428`), whether this `Monotone` and a
`Monotone<Additive, TotalOrd>` are one trait, resolves cleanly once the quantifiers are written: this one
is monotonicity of a unary map with respect to one order, and the other is order-preservation of a binary
operation's partial application. Different arity, different subject. File 17 reached the same conclusion
(`17_orchard...md:469-474`) and I agree with it: two names.

### 7.2 Translation stability, which survives with its quantifier corrected

The draft states it at `11_current_shape_draft.md:259-261` as `phi(phi(x) + c) == phi(x + c)` "for every
exact sum `x` and every representable `c`". The quantifier over `x` ranges over the exact space, which at
`Growth::Exact` is unbounded, so the statement as written is not exhaustively checkable at any width, and
every probe in this review has silently bounded it. That is fine and it should be written down: the
checked statement is over a bounded exact window, and the claim that the window suffices is another
instance of the transfer argument in the trusted bin.

McSherry's over-strictness finding (`13_mcsherry...md:210-234`: one map in 65536 is translation-stable,
1024 are genuinely fold-associative) is, under section 1's form, a statement that the design derives a
claim at one setting of the parameters from a claim at a stronger setting. That is sound, which McSherry
verified (`stable but NOT associative: 0`), and it is over-strict by construction. Section 4's
classification is one way to keep the soundness and drop most of the over-strictness, because it derives
the law from what `phi` *is* rather than from a sufficient condition on what it satisfies.

### 7.3 `AddClosed`, which changes meaning under partiality and should be checked

The draft gates the law on an `AddClosed` condition keyed on `Bias = Zero`
(`11_current_shape_draft.md:285-287`). Closure is a claim that the operation's result lies in the
carrier. For a **partial** operation that is true by construction, since refusing is not producing a
value outside the carrier, so `AddClosed` has different content for `Precise` than for `Hot` and the
design should say which content it means. I did not measure this and I flag it as a question rather than
a finding: it may be that `AddClosed` is doing bias-exclusion work only and closure is the wrong word for
it, in which case the fix is the name.

### 7.4 `Deterministic`, which survives only once its quantifier is named

"The property holds for one composition" (`11_current_shape_draft.md:303-305`) leaves open what it
quantifies over. Determinism across two runs of one binary is a different claim from determinism across
two builds of one source, which is different again from determinism across two targets. File 17 showed
(`17_orchard...md:405-412`) that a `Relaxed` composition cannot have it at all, and that the reason is
equational rather than incidental: `Relaxed` replaces a function by a relation, so there is no function
for two runs to agree on. Lattner reached the same place from the toolchain side. So the property
survives, it is real, and it needs the quantifier written or it will be read as the strongest of the
three readings and be false.

### 7.5 `ConstantTime`, which does not survive as a type-level claim and should stop pretending to

The draft already says most of this at `11_current_shape_draft.md:308-316` and file 17 added the sharp
part (`17_orchard...md:543-546`): delivery decides it, and delivery is not one of the ten axes. Under
section 1's form, the claim's parameters include the target, the toolchain version and the codegen
choices, none of which any type mentions. It is not an under-specified property. It is a property of a
different object.

### 7.6 The "checked text is the executed text" claim, which is the strongest thing here and is not a law

Thread C's fifth pass (`11_current_shape_draft.md:620-631`) is the design's best mechanism and it is
worth saying exactly what it claims, because it is not an equation and reading it as one undersells it.
It claims that two monomorphisations of one generic function have the same behaviour at two widths. That
is not a law about arithmetic. It is a claim about the compiler, and it is discharged by the two feature
bans (`11_current_shape_draft.md:842-846`), because those are what make a body unable to ask which width
it is running at.

The draft notes that this dependency "was not previously written down anywhere, including in the rule
that states the bans themselves". It now is, in `unstable-features.md`'s "The forbidden list is
verification infrastructure" section, which is the right place for it. The half of it that is
mechanisable is the ban itself, as a compile-fail test asserting that `specialization` and `TypeId` do
not appear, and I would write that test, because a dependency this load-bearing that is guarded only by a
rule file is guarded by a document nobody compiles.

## 8. Equations, refinements, or an ordering: three readings, and I resolve none of them

The brief asks whether laws should be equations at all. Three shapes are available and they are not
competing answers to one question, which is the first thing to say about them.

**Equations under a chosen relation**, which is the current shape. The design picks `R` per fact and
states `fold_g R fold_h`. Cheap, familiar, checkable at a model width, and it is what every worked
example in this review actually did. Its cost is the one section 2 measured: one relation describes one
column, so the design needs several facts rather than one, and the temptation is to pick a relation that
fuses them and report the conjunction.

**A relational denotation, with laws as inclusions.** Give every operation a denotation
`⟦op⟧ : V × V → P(V + {⊥})` and state laws as inclusions between relations rather than equations between
functions. This is the only one of the three that covers partiality and approximation with **one**
definition, which is what the brief asks about directly. Under it: a partial function is a relation with
at most one output, an approximating operation is a relation with several, and the regrouping spread
`Spread_n(x) = { fold_g(x) : g a grouping }` is one object with three readable properties. `|Spread| = 1`
always is Kleene associativity. `|Spread \ {⊥}| ≤ 1` is existential associativity. Anything larger is the
approximate case, and file 17's "the replacement of a function by a relation"
(`17_orchard...md:410-412`) is exactly this object named from the other end.

Its cost is that it is a specification device and not a checking device. Inclusion of subsets of `V` is
not something the type system can decide, and the design would be writing the spec in one vocabulary and
checking it in another, which is precisely the seam Thread C's fourth pass found things hiding in. I
think it is still worth having, in prose, as the definition every checked fact is an instance of, and I
would not build machinery for it.

**A refinement order between an ideal operation and a realised one**, `Impl ⊑ Spec`. This is the shape
Fallin's concern wants (`16_fallin...md:311-320`) and it is genuinely a different axis from the other
two: it compares an implementation against a specification, where the other two compare two groupings of
one implementation. Both axes are real and the design needs both.

The one thing I can say against using the refinement order for the *regrouping* axis is a measurement
rather than a taste: section 2 showed neither direction holds for `Refuse`. So the attractive sentence
"the regrouped answer refines the sequential one" is not available, and a design that reached for the
ordering there would be reaching for something false. For the implementation-against-specification axis
the ordering is exactly right and is what Thread C's fifth pass already does informally.

**Where this is taste and should be labelled as taste.** Whether to publish the relational denotation at
all, given that nothing checks it, is a judgement about whether a definition that only humans read earns
its space. I think it does, because the three properties above are currently three separate paragraphs in
three separate files and they are one object. Someone could hold that an unchecked definition in a spec
is a liability, and that is a defensible position about this design specifically, given how much of this
review has been about prose claims and executed code disagreeing.

## 9. What can be checked, given the constraints, sorted by the form of the claim

No dependent types, no higher-kinded types, monomorphisation as the only dispatch, `adt_const_params` and
the const-traits family available, `generic_const_exprs` and full `specialization` forbidden, `#![no_std]`,
no `alloc`. Under those constraints the checkability of a claim is decided by **the form of its
quantifiers**, and this is mechanical enough to be a rule.

| the claim's outermost quantifier | what can enforce it | example |
|---|---|---|
| over types and consts only | the type system, by construction | "no two contradictory law claims for one composition" |
| over consts, with an arithmetic side condition | a const bound the compiler evaluates | `acc_width ≥ numeral_width + ceil(log2 n)` |
| over values, in a bounded space | a `const` block, exhaustively, at a model width | a classification checked against the recovery rule |
| over values, unbounded | nothing mechanical; a derivation plus a stated transfer | translation stability at `Growth::Exact` |
| over machine code or targets | nothing at compile time; a runtime measurement | `ConstantTime` |

Three consequences I would act on.

**The classification move in section 4 changes which row a fact sits in, and that is its whole value.**
"Is `phi` a partial identity" quantifies over values in a bounded space, at a single argument. "Does
every grouping of an n-element fold agree" quantifies over values, groupings and arity, and its arity
quantifier is unbounded. Moving from the second to the first plus a derivation is the difference between
row three and row four, and it is available for two of the four resolutions for free.

**Every claim in row one needs to be checked that it is really in row one.** The `Monotone` impl in
section 7.1 looks like a row-one claim, a marker impl over types with nothing to evaluate, and it is
actually a row-three claim wearing a row-one costume: the property it asserts quantifies over values and
nothing checks it. That is the general form of file 17's licence finding and of the `Monotone` defect
here, and the test is simple enough to write down: **if the property's statement mentions a value, the
impl must produce something a checker can consume.**

The shape available under these constraints is the one Thread C already uses. A property whose statement
quantifies over values is declared alongside an associated `const` block that discharges it at a model
width, so an implementor who omits it fails at their own definition site with `E0046` and an implementor
who states it falsely fails the const evaluation. That is what "the check is the typestate" means for a
row-three claim, and the design has built it once, for resolution constructors
(`11_current_shape_draft.md:281-284`). It has not applied it to any law marker.

**And one thing the constraints genuinely forbid, which should be stated so nobody spends a dispatch on
it.** The universally quantified statement "for every input, the executed code agrees with the
specification" is not expressible, because it quantifies over values in a type. The draft says this
already and says it correctly at `11_current_shape_draft.md:861-868`, and I would only add that it is
worth repeating next to each row-four claim rather than once at the end, because the place it gets
forgotten is the place a claim is written.

## 10. Where precision does not pay, stated as recommendations against precision

The brief asks this directly and I want to give it real answers rather than a token one.

**`ConstantTime` should stay informal, and should stop being a marker.** Its quantifiers reach outside
anything the design can see. A precise statement of it would have to name a target, a toolchain version
and a set of codegen flags, and would be falsified by a compiler bump with no source edit. The honest
form is what the draft already proposes: a measurement in `mock/benches/`, dated, per target, cited from
prose. Making it precise would produce a claim that is exactly as unreliable and looks more reliable,
which is the worst combination available.

**The width-uniformity transfer argument should stay prose, and this is not a concession.** It is a
statement about monomorphisation being uniform, and the thing that makes it true is the two feature bans,
which are a policy rather than a theorem. Attempting to mechanise the argument itself would mean encoding
"the compiler does not do X" inside the compiler. What is worth mechanising is the *precondition*, which
is the compile-fail test in section 7.6, and that is a small piece of work with a clear payoff.

**Commutativity probably does not need any of this apparatus.** It quantifies over two values and one
operation, it has no arity question, no grouping question and no accumulator question, and every
resolution measured in file 13's probe 03 has it (`13_mcsherry...md:289-298`, every operation in the
table commutative). A property that is true of everything in scope and cheap to state is one where the
precision would buy nothing until something in scope lacks it.

**And the algebra ladder's upper rungs should not be made precise yet.** Dolan's finding
(`14_dolan...md:332-343`) is that no `Number<N, S>` carrying a `Policy` can be an exact ring or field, for
a structural reason. Writing precise statements of ring and field axioms for a family that provably
cannot satisfy them is precision spent on the wrong object. The precise versions belong wherever the
number systems live, and that is D47's depth mandate pointing at a different crate rather than at a
deeper hierarchy in this one.

## 11. The downstream contract, designed

`16c`'s obligation is that every boundary this design stops at gets a design rather than an observation.
The boundary that falls to me is the one between arvo's derived facts and whatever performs a regrouping,
and files 16 and 17 have between them established that this boundary is not a compiler backend: nothing
survives type erasure (`16_fallin...md:296-308`), and for fixed point there is no LLVM concept to receive
a licence at all (`16_fallin...md:170-185`). I take that as settled for the *lowering* question and I am
answering the specification question, which is different and is unowned.

**What a regrouping combinator reads out of arvo's types.** Three facts, keyed on the operation, the
element numeral, the accumulator numeral, and nothing else:

| fact | what it asserts | which resolutions have it |
|---|---|---|
| every grouping agrees, definedness included | Kleene associativity at every arity | homomorphism recovery, or any recovery with a sufficient accumulator |
| every grouping that returns agrees | existential associativity at every arity | partial-identity recovery |
| neither | nothing is licensed | retraction recovery at the same-width accumulator |

The first two are derived from the recovery map's structural class per section 4, not searched. The key
carries the operation because of section 5, and the accumulator because of section 6.

**What arvo needs back from the combinator, which is the half nobody has stated.** A combinator that
consumes the second fact must declare its refusal discipline, because section 2 measured that neither
direction of the refinement order holds and so nothing about definedness can be derived. Two disciplines
are available and they are different functions:

*Refuse-if-any* evaluates its schedule and refuses whenever any grouping in it would have refused, which
is file 17's proposal at `17_orchard...md:373-376`. Sound, and it returns a refusal on inputs where the
sequential fold returns a value, so a consumer who swaps a sequential fold for this one sees new refusals
and must be told so in the signature.

*Committed-schedule* fixes one grouping and reports that grouping's definedness. Also sound, cheaper, and
it means two builds with different unroll factors can disagree about definedness, which interacts with
`Deterministic` in section 7.4 and needs to be said out loud.

Neither is derivable and the combinator must pick one and name it. That is the requirement flowing back
across the boundary, and it is a contract clause rather than a mechanism.

**And one thing a build layer must be forbidden from doing, extending file 17's version of the same
point.** File 17 established (`17_orchard...md:580-586`) that any build-layer mechanism selecting among
arvo's implementation variants after type checking reopens Thread C's fourth-pass gap. The specification
version is narrower and, I think, less obvious: **a build layer that chooses an unroll factor, a chunk
size, or an accumulator width is choosing parameters of the law's statement, not of its lowering.** Per
section 6 the accumulator decides the verdict, and per section 3 the arity does. So an unroll-factor knob
is a knob over what was proven, and if such a knob is ever wanted it has to be an input to the type-level
selection, exactly as file 17 says for variant selection. I would put that sentence next to file 17's,
because the two failure modes look different and are the same one.

## 12. What I would flag for the next member, unresolved

**The relation question in section 2 is a decision about what `Precise` is for**, and I have argued it
from the measurement rather than from the purpose. Whether a refusal is an answer or the absence of one
decides whether the design names one fused fact or two separate ones, and that is not mine to settle. I
have carried both readings and I lean to two facts; file 17 leans the other way and its reason is good.

**The accumulator question in section 6 has two shapes and I resolved neither.** Making it a combinator
parameter is cheaper and keeps the axis table; making it an eleventh `Policy` axis is what the design's
own sorting test says it is. Both are real, they are not compatible, and the choice interacts with
`arvo-num-systems` and with the `Growth` axis in ways I did not trace.

**I did not build the classification mechanism, and the record here is that each unbuilt shape in this
thread had a hole the next member found by compiling it.** Section 4's proposal is that `Resolution`
constructors declare their structural class and the law derives from it. Four consecutive Thread C passes
each said their shape should be cheap. Read this one with that suspicion.

**Section 5 predicts which multiplication compositions recover the partial-identity property** (any where
the product lands on the representable lattice, which is `Growth::Exact` with a `2F`-fractional
accumulator) and I did not test the prediction, only the failure it explains. Whoever runs the
multiplication dive should test it early, because if it holds it says multiplication's law is the same
theorem as addition's rather than a new problem, and the whole dive has been assuming the opposite.

**The `Monotone` impl in section 7.1 is false and I did not check whether anything else in the design's
proposed law implementations has the same shape.** The defect is that a marker's premise names fewer
members than the property depends on, and that is a pattern rather than an incident. A pass over every
proposed law impl in `202607301100_topic.the-formalization-talk.md` and its siblings, asking only "does
the premise mention every member the property quantifies over", is cheap and is a grep-plus-reading job
nobody has done.

**I did not read `arvo-num-systems` or `notko-hlist`**, both of which file 17 also flagged
(`17_orchard...md:622-624`), and either could change section 8's relational-denotation cost argument,
since a type-level set is exactly what the second one extracted and the draft records as reviewed by
nobody (`11_current_shape_draft.md:56`).

**And the two feature bans now carry a load nobody has pinned.** Section 7.6 names the compile-fail test
that would guard them. It does not exist, it is small, and the argument that most depends on it is the
one the draft calls its irreducible core.
