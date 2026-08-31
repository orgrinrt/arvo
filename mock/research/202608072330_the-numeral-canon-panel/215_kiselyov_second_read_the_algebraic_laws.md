# 215. Second read, the eleven `algebraic_laws` rows at `one_expert`

Seat 215. Kiselyov. The eleven rows named in the brief.

**The brief asked for a blind derivation and the schema does not allow one.** A row's `id` is its claim
written as a slug, so reading the id to find the question has already handed over the answer. What phase
one below actually is: eleven conclusions given to me, with my own justification and my own predictions
about what each row would turn out to say, written down and committed before I could see whether either
matched. That is weaker than a cold derivation and stronger than a review, and calling it blind would be
a false claim of independence, which under a model where agreement promotes to canon is the expensive
kind of wrong. The section headed "What independence I actually had" states the bound in full, including
the four predictions I got wrong, which is the evidence that the contamination is bounded.

For phase one I read each row's `id` and `topic` and nothing else: not `says`, not `because`, not `note`,
not `provenance`, and no panel file on any of these questions. What I did read first, and what therefore
bounds everything here: `mockspace.toml`, `mock/registry/topic.toml`, `mock/registry/strategy.toml`,
`INTENTS.md` in full, `mock/Cargo.toml`, and the `mock/checks` test bodies. Those are canon and process
rather than answers, but `strategy.toml` and `INTENTS.md` carry op's framing of the strategy axis and of
I13, so any agreement I reach with a row author on *those* is shared input rather than a second instance.

Committed in stages: phase one alone at `b2441acb`, then the instruments at `46b3aa68`, `12398de5` and
`4d097ecf`, then the reconciliations. The instruments for probes 1 and 2 were designed, run and committed
before I opened a single row.

---

## Phase one: what I derive from the id alone

### 1. `a_composed_expressions_region_is_never_inherited_from_its_parts`

**The question the id names.** Given sub-expressions each carrying a region under I13, is the composite's
region a function of the parts' regions?

**My derivation.** No, and for three separate reasons, of which only the first is usually noticed.

*The regions are stated over different things.* A part's region is a predicate over that part's own
inputs. In `f(g(x))` the inputs to `f` are `g`'s outputs, so the composite's region is not `R_f ∩ R_g` but
`R_g ∩ g⁻¹(R_f)`. That pullback is not computable from `R_f` and `R_g`: it needs `g`'s image, which the
predicate does not carry. So intersection is not merely wrong, it is not even type-correct.

*The composite can fail where every part holds.* Each multiply in `(a·b)·c` can be individually within
range while the intermediate `a·b` escapes it under one association and not the other. Every part's
region is satisfied at its own site and the composite's law fails.

*The composite can hold where a part fails.* Errors cancel. Under wrapping arithmetic `(a ⊕ b) ⊖ b = a`
exactly, for every `a` and `b`, because the wrap is a group quotient, even though `⊕` alone escapes its
declared range. So the composite's region is not bounded above by the intersection either, which kills
the weaker reading where inheritance means "at most".

**So I expect to agree with the prohibition and to want it sharpened.** "Never inherited from its parts"
is true when *parts* means *the parts' predicates*. It is false when *parts* means *the parts' semantics*:
if each part carries its reachable interval as well as its predicate, the composite's region is computable
by pushing intervals forward and pulling the predicate back. That is not a quibble, it is the whole
design consequence: it says what a row has to carry for composition to work at all, and it is the same
quantity row 10 is about. A prohibition with no accompanying "and here is what does compose" is the shape
`every-finding-carries-its-predicate.md` warns about, a region nobody looked for.

**Region I expect to be able to defend:** `operation ∈ {add, mul}`, `policy ∈ {wrap, saturate}`,
`signedness ∈ {signed, unsigned}`, `I + F ≤ 8`, `threads = 1`, exhaustive.

### 2. `a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains`

**The question.** When does a law stated over declared numerals survive to the realised machine
operations?

**My derivation.** This is the congruence condition and I believe it is exactly right, because a
congruence *is* an equivalence closed under every one-hole context, and "every nesting it contains" is
that closure spelled out. The content is in the quantifier. A realisation that is a homomorphism on the
leaves and on the observed result, but not on the intermediates, does not transfer an equation, because
an equation's two sides differ precisely in how they nest. Associativity is the minimal witness: the two
sides agree on their leaves and on nothing else.

Concretely, "congruence at every nesting" is the abstract form of "no intermediate is rounded, clamped or
otherwise moved", which is what one would say operationally.

**Two things I expect to have to say.**

*The direction is sufficiency and must not be stated as a biconditional.* A law can hold in the realised
algebra without the map being a congruence anywhere. Commutativity survives saturation, which is not a
congruence for anything. Wrapping multiply is associative while wrapping is not exact at any nesting.
Accidental laws are the majority of the useful ones, so an iff here would refuse exactly the arms I13
asks for.

*Congruence is the wrong word if the intended map is `Declared → Machine` rather than `Machine →
Declared`.* Laws travel *up* a surjective homomorphism for free and travel *down* only when the map is
injective on the reachable set. The precise statement wants both halves: homomorphic and injective on
what is reachable. If the row says congruence and means only the first, it has proved the useless
direction.

### 3. `a_law_layer_answers_whether_a_law_reaches_a_lowering_the_backend_cannot_prove`

**The question.** What is the law layer *for*?

**My derivation, and this is the one I expect to refuse.** As a criterion for whether a law earns an arm,
this is right and is `small-wins-compound-into-the-program.md` restated at the law layer: the typestate
knows a bound the backend never learns, the proof does not survive lowering, and supplying it is the
work. I have no objection to that as design guidance.

As a statement about what the law layer *answers*, it is a category error and it fails the canon's own
permanence test.

*It conflates truth with usefulness.* Whether `(a+b)+c = a+(b+c)` is true over a format is a fact about
the format. Whether LLVM already knows it is a fact about LLVM. A layer that answers only the second
cannot serve a consumer who needs the first, and I expect row 11 to be about exactly that consumer.

*"The backend cannot prove" is not a stable predicate and therefore cannot appear in a canon sentence.*
It ranges over the toolchain pin, the target triple, the optimisation level and the surrounding code. A
canon sentence has to survive a total rewrite in another language and another decade;
`the-canon-is-intent-not-implementation.md` puts it as: would this still be true and useful after the
implementation is rewritten? A definition keyed to what one version of one backend fails to derive is
false the week the backend improves, and it is false silently, because the sentence does not change. Any
finding of this shape carries `toolchain = <the pin>, target = <triple>, opt-level = 3` and holds
nowhere else, which is a fine predicate for a *finding* and a disqualifying one for a *definition*.

**So I expect: refuse as a definition, keep as a design criterion, and say the two are different tiers.**
I intend to check the empirical half rather than assert it, because "the backend cannot prove this" is
itself a claim and is cheap to test: emit the assembly for a wrapping chain and a saturating chain and
look at whether the reassociation happened.

### 4. `a_min_plus_fold_needs_an_absorbing_top_and_wrapping_supplies_none`

**The question.** Does a min-plus fold need an absorbing top, and does wrapping have one?

**My derivation, and the wrapping half is a one-line theorem.** The min-plus (tropical) semiring is
`(S ∪ {⊤}, min, +)` where `⊤` is the identity for `min` and the annihilator for `+`. A fold needs it
twice over: as the monoid identity, so an empty or split fold has a value and the fold can be
reassociated and spread across lanes; and as the annihilator, so "unreachable" propagates through
concatenation instead of becoming a finite cost.

Wrapping supplies none, and the proof needs no search. Addition mod `2ⁿ` is a *group*. In a group,
`T + x = T` implies `x = 0` by cancellation. So no element absorbs anything but the identity, at every
width, at both signednesses, for all `n`. That is a genuine universal and I can state it as one.

**And the useful half is the region, which the id does not mention.** Saturating arithmetic *does* supply
an absorbing top, and unsigned saturating supplies a complete one. In unsigned saturating `uₙ`:
`MAX +sat x = MAX` for every `x`, because every `x` is non-negative; and `min(MAX, x) = x` for every `x`.
So `MAX` is simultaneously the `min`-identity and the `+sat`-annihilator, `+sat` is associative on
non-negatives, and `+sat` distributes over `min` because adding a constant is monotone and monotone maps
commute with `min`. That is every semiring axiom. **Unsigned saturating is a min-plus carrier and
wrapping is not.** Signed saturating gives an absorbing top only on the non-negative cone, because
`MAX +sat (−1) = MAX − 1`.

The honest caveat, which I expect to have to supply: `MAX` is a *lax* top. It is a reachable value, so
"no path" and "a path costing MAX" are the same bit pattern. The arm carries `all reachable costs <
MAX` and is unsound without it.

I intend to prove all of this by exhaustive search at `u8` and `i8` rather than by the argument alone,
with wrapping as the negative control: an instrument that cannot report "no absorbing element exists" is
not an instrument.

### 5. `a_range_clamp_and_a_coarsening_are_two_independent_sufficient_causes_of_multiplicative_non_associativity`

**The question.** What breaks `(a·b)·c = a·(b·c)` in fixed point?

**My derivation.** Two mechanisms, and they are genuinely independent in the sense that each has a
witness where the other is absent.

*Coarsening alone.* At `F > 0` a product carries `2F` fractional bits and is rescaled back to `F`. The
two associations rescale at different points, so they differ, with no overflow anywhere. Sufficient.

*Range clamp alone.* At `F = 0`, no rounding at all, signed saturating `i8`: `a = 100, b = 2, c = −1`
gives `(100·2 → sat 127)·(−1) = −127` against `100·(2·(−1) = −2) = −200 → sat −128`. Sufficient.

**But I expect to narrow the clamp half, because I do not think it survives unsigned.** Unsigned
saturating multiply looks associative to me and I expect to prove it exhaustively. The argument: `sat` is
monotone, and multiplication by a non-negative is monotone. If `ab ≤ MAX` nothing was lost. If `ab > MAX`
then `sat(ab) = MAX`, and for `c ≥ 1` both `MAX·c` and `abc` exceed `MAX` so both saturate; for `c = 0`
both sides are `0`. Both associations equal `sat(abc)`.

Similarly, wrapping multiply is associative at both signednesses because `ℤ/2ⁿ` is a ring. So of the
policies in play, **only signed saturating breaks associativity by clamping**, and the row as its id
states it would be reporting a prohibition over a space where it holds in one cell of four.

The region table I expect to be able to defend at `F = 0`: wrapping associative at both signednesses;
unsigned saturating associative; signed saturating not. And at `F > 0` non-associative under a per-step
rescale for every policy, with the escape being a deferred rescale rather than a different policy.

### 6. `a_resolved_strategy_never_inherits_a_composed_operations_laws_but_dominates_them_monotonically`

**The question.** How does the strategy resolved for a composite relate to the laws of the parts?

**My derivation is split. I expect to agree with the first clause and refuse the second.**

*No inheritance* follows from row 1 plus one extra reason specific to strategies: resolving a composite to
one strategy *changes the realisation map at the intermediates*. A law established for a part under the
part's own strategy is a statement about a different lowering than the one the composite will use. So it
does not transfer, and it does not transfer for a stronger reason than mere non-compositionality.

*Monotone domination* presupposes an order on strategies along which law sets are monotone, and I believe
both halves are refutable.

The order does not exist by op's own premise. I8 says the strategies weigh *different measurements*
differently, and I13 rejects a universal solution because the strategies make one impossible by premise.
Speed, footprint and accuracy are incomparable axes; there is no lattice to be monotone in.

And even granting some fidelity order, law-set containment runs *against* it. Wrapping is the less sound
lowering by any reading, and wrapping multiply is associative. Saturating is the more careful lowering,
and signed saturating multiply is not associative. So a strategy that is "more precise" can have strictly
*fewer* algebraic laws than one that is "less precise". If that holds, monotone domination is refuted by
a two-line counterexample, and it is the same counterexample as row 5's.

**I expect this to be the row I refuse hardest**, and I expect the refutation to be cheap and exhaustive.

### 7. `a_trajectory_condition_lifts_into_a_declaration_exactly_when_it_survives_closure`

**The question.** When can a condition about the actual run (the sequence of intermediates a computation
passes through) become a static declaration about a type?

**My derivation.** A declaration is a claim about every program anyone can write against the type. A
trajectory condition is a claim about one run. So lifting requires the condition to hold on every
reachable trajectory, and closure under the admitted operations is the standard way to establish that:
`P` lifts if `P` holds at every constructor and `P(x) ∧ P(y) ⟹ P(x ⊕ y)` for every admitted `⊕`. That is
inductiveness, and closure is exactly it.

**Sufficiency is clear. Necessity is where this gets interesting, and I think the answer is that the iff
is true at the library boundary and false inside a chain.**

In general, closure and reachability come apart: `P` can be an invariant (true on all reachable states)
without being inductive (closed), which is the ordinary reason one strengthens an invariant. Where they
come apart, a non-closed condition is still a valid declaration and the iff is false.

But arvo is a library, and I11 says so in op's own words: we do not know how end users will use us. At a
library boundary the consumer is universally quantified over the admitted operations, so the reachable
set *is* the closure, and the iff holds exactly.

Inside a statically known chain the schedule is fixed and known, so the reachable set is a strict subset
of the closure, and a condition that fails closure can still be a sound declaration for that chain.

**So I expect two arms, not one law:** `boundary = public` gives the biconditional; `boundary = internal
chain with a known schedule` gives sufficiency only, and the gap is precisely what a chain-local
declaration is allowed to exploit. If the row states the iff unconditionally, it is right where arvo
meets its consumers and wrong exactly where the composite work lives.

### 8. `absorption_is_the_correct_associativity_criterion_and_literal_coherence_is_a_strictly_narrower_law`

**The question.** What criterion decides associativity, and how does literal coherence sit against it?

**My derivation, and I expect to agree with this one and to find it is the best row of the eleven.**
Write a realised operation as `op(a,b) = r(a ⋆ b)` where `⋆` is the exact operation on the wide
intermediate and `r` is everything the realisation does to get back to the declared format: rescale,
round, clamp. Then

```
op(op(a,b),c) = r(r(a ⋆ b) ⋆ c)        op(a,op(b,c)) = r(a ⋆ r(b ⋆ c))
```

and both collapse to `r(a ⋆ b ⋆ c)` exactly when `r` is absorbed by a later application:

```
r(r(x) ⋆ y) = r(x ⋆ y)      and      r(x ⋆ r(y)) = r(x ⋆ y)
```

That is an absorption law, it is a condition on `r` and `⋆` alone rather than on triples, and it is the
right criterion because it does not care *why* `r` moves a value. Rounding and clamping enter it the same
way, which means **absorption subsumes row 5's two causes rather than sitting beside them**, and applying
it mechanically should reproduce row 5's region table. Checking it costs `n²` where associativity costs
`n³`, which is a real and separate reason to prefer it.

*Literal coherence* I read as `r` fixing the exactly-representable values, `r(lit) = lit`. That is
absorption restricted to arguments already in the image of `r`, so strictly narrower is right, and
strictly is right too: unsigned saturating fixes every representable value *and* satisfies full
absorption, while signed saturating fixes every representable value and fails absorption. So literal
coherence holds where absorption fails, which is what makes it strictly narrower rather than equivalent.

**What I will actually check.** Whether absorption and associativity agree as *sets* over
`{add, mul} × {wrap, saturate} × {signed, unsigned} × F ∈ {0, 2, 4}` at `u8`/`i8`. The best available
outcome is a cell where they disagree, which would refute the criterion; I expect none and will say so
only after running it with the controls in place.

### 9. `the_laws_of_a_format_are_derived_from_two_hypotheses_rather_than_enumerated_per_policy`

**The question.** Are a format's laws a table indexed by policy, or consequences of a small number of
hypotheses?

**My derivation.** Derived, and rows 2, 5 and 8 are three faces of the same derivation, which is itself
evidence for the claim. But **I expect to argue that two hypotheses do not cover the space, and that the
count is three.**

If the two hypotheses are exactness and containment, that is the isomorphism case: `r` is the identity on
the reachable set, the realised algebra is isomorphic to the ideal one, every ideal law transfers. Fine,
and it misses the two most useful cells.

Wrapping violates containment flagrantly and keeps every ring law, because it is a *quotient
homomorphism* `ℤ → ℤ/2ⁿ` and `ℤ/2ⁿ` is a commutative ring. Its laws come from the target's structure, not
from staying in range.

Unsigned saturating violates exactness and keeps associativity, because `sat` is a *closure operator*:
monotone, idempotent, inflationary on the relevant cone, and compatible with a monotone `⋆`. Its laws
come from `r` being absorbed, which is row 8.

Signed saturating is none of the three, which is exactly why it has no laws worth having.

**So the derivation I expect to defend is a classification by what kind of map the realisation is** —
isomorphism on the reachable set, quotient homomorphism, or closure operator compatible with the
operation — and each kind delivers a different law family. If the row's two hypotheses are the general
ones (the map is a homomorphism for every operation in the term, and the law is valid in the target
algebra) then the classification is a consequence and I agree outright. If they are the specific ones
(exact, and in range) then the row is sound and incomplete, and its incompleteness deletes the two arms
that pay.

### 10. `the_reachable_interval_is_the_true_grade_and_width_is_its_lax_abstraction`

**The question.** What grades a numeral: the interval it can actually reach, or the width it declares?

**My derivation.** The interval is the precise invariant and width over-approximates it, so as a statement
about *value semantics* this is right and the word *lax* is the right word. A grading assigns to each term
an object determined by its operands' grades. Interval arithmetic is such a grading and is exact for the
monotone operations. Width is a grading too, `w(a·b) = w(a) + w(b)`, and its homomorphism is an
inequality rather than an equality, which is what lax means. There is a Galois connection between them,
with width the abstract side.

The consequence is the payoff and is straight I13: containment is decided by the reachable interval, so a
law that fails on width grounds can hold on interval grounds. `Fixed<8,0> · Fixed<8,0>` grades to 16 bits
by width; if both operands are known `≤ 15` the product is `≤ 225` and never leaves 8. Every law needing
containment holds there and is currently refused by the width grade.

**Two narrowings I expect to have to make.**

*Width is not purely a lax shadow of the interval, because width also fixes storage.* Two numerals with
the same reachable interval and different widths have different footprints, different packing and
different lowerings. The interval says nothing about that. Under I6 and I17, footprint is not a derived
quantity to be optimised away, and I17 is explicit that the storage-minimising path is not traded off.
So the claim holds on the value axis and is false on the storage axis, and a canon sentence that says
width is *the* lax abstraction without naming the axis will be read as licensing the trade I17 forbids.

*The interval grade must be const or it is not a grade.* I15 forbids a runtime check, so an interval
bound that is only known at runtime cannot gate an arm. The interval grade is usable exactly when the
bounds are const-available, which per op at `83` means anything reachable at const time, typestate
included. That is a real predicate on the arm and it hands the question back to row 7.

### 11. `the_two_law_families_have_two_consumer_classes`

**The question.** Are there two families of laws, and do they serve two distinct consumers?

**My derivation.** I expect two families to be real and the *closure at two* to be the load-bearing and
unargued part.

The split I would draw: laws consumed by the **lowering**, which license a rewrite arvo performs on the
consumer's behalf; and laws consumed by the **downstream program**, which let a person reason about their
own code. They have genuinely different admission criteria, which is what makes them families rather than
a taxonomy:

- A rewrite law must be exact and const-decidable. An approximate law licenses nothing, because a rewrite
  has to be observationally equivalent, and per I15 the decision cannot be a runtime check.
- A reasoning law may be approximate with a stated bound, and that is often exactly what a consumer wants
  ("reordering costs at most one ulp"). Such a law is worthless to the first family and valuable to the
  second.

Per I11 the second family is the selling point, since the value is what composes on top; the first is the
microkernelling. Both real, and the criterion genuinely differs, so the distinction earns its place.

**What I expect to refuse is the word two**, unless the row argues completeness. At least two more
candidates present themselves and neither is obviously the same as either family: laws as **oracles** for
the check harness, which need only be decidable and may be probabilistic; and laws as part of what a
**strategy promises**, since under I8 a strategy's law set is one of the measurements it weighs and a
consumer selecting a strategy is selecting a law set. A count is a completeness claim and completeness
claims are the ones that fail quietly. If the row says "two" and means "at least these two, which differ
in admission criterion", I agree. If it means the partition is exhaustive, it owes an argument I do not
expect to find.

---

## What I intend to build, and the case each instrument must be able to fail

Stated before running anything, because a probe whose failing case cannot be named is not an instrument.

- **Associativity and absorption, exhaustive at 8 bits.** Must report signed saturating multiply as
  non-associative and must report wrapping multiply as associative. If it reports everything associative
  the instrument is broken and the numbers are void.
- **Absorbing element search.** Must report *none* for wrapping at both signednesses, and must report
  `MAX` for unsigned saturating addition. A search that finds an absorber under wrapping has a bug,
  because the group argument forbids one.
- **Composite region.** Must exhibit both directions: a composite failing where every part holds, and a
  composite holding where a part fails. One direction alone does not establish non-inheritance.
- **Interval against width.** Must exhibit a triple where the width grade refuses a law the interval
  grade admits, and must confirm the law actually holds there by exhaustive check rather than by the
  interval argument.
- **What the backend already does.** An ad-hoc spike, not a bench, and called that: it can establish that
  a reassociation did or did not happen in emitted assembly, and it cannot price anything.

---

## What independence I actually had, stated before any verdict

The dispatcher asked for a blind derivation and the schema does not permit one. A row's `id` is its
claim, spelled as a slug: `a_min_plus_fold_needs_an_absorbing_top_and_wrapping_supplies_none` is the
conclusion, not the question. So reading the id to find out what the question is has already told me the
answer, and phase one above is not what its own heading calls it.

**What it actually is: eleven conclusions handed to me, with my own justification and my own predictions
written down before I could see whether either matched.** That is a weaker thing than a cold derivation
and a stronger thing than a review, and it should be called by its name.

**Independent, and I will defend these as independent.** The arguments: group cancellation forbidding a
wrapping absorber, the congruence-to-quotient route, the monotone-clamp mechanism. The instruments:
probes 1 and 2 were designed, run, and committed at `46b3aa68` before I opened a single row, so every
number in them is uncontaminated by what any row says.

**Not independent.** The eleven propositions themselves. I was checking, not deriving.

**The evidence that the contamination is bounded is that I got things wrong.** My prediction of what row
8 was about (that literal coherence means `r` fixes the representable values) is not what the row says.
My prediction of what row 11's two families were (compiler against programmer) is not what the row says.
My prediction of row 10's subject was wrong enough that I cannot corroborate it at all. And probe 1's
own control refuted my stated prediction of the interval boundary, `k = 5`, with `k = 11`. A reading
contaminated by the rows' contents does not mispredict four of them.

**One section is not independent by construction and is labelled so.** Probe 3 section 1 takes row 9's
closed form as an input and tests it out of sample. That is not derivation, it is a prediction test, and
in this case it is the stronger instrument precisely because the form came from somewhere else.

## Provenance first, because standing is now a promotion path

The dispatcher's later steer put provenance ahead of agreement, and it was right to: an agreement I
cannot ground is worth less than a refusal. Every row's `provenance` was walked to the file that actually
did the work.

**Five of eleven stop at a consolidation that restated the finding, and never name the member file that
established it. In all five the establishing file is named inside the very text the row cites**, so the
information was on screen at porting time and was dropped in the port.

| row | `provenance` names | what actually established it | reaches it |
|---|---|---|---|
| never inherited | `90` twice | `79:27` for the count, `79:112-121` for the statement | no |
| law layer pays | `90` twice | `80` section 5.2 | no |
| congruence at every nesting | `97_dolan` twice | `97` and `97_probes/p2_congruence_predicts_the_laws.py` | yes |
| min-plus absorbing top | `35_mcsherry` | `35` | yes |
| clamp and coarsening | `58_wronski` twice, `57_orchard` | `58_probes/p2`, `57_probes/p4` | yes |
| resolved strategy | `77_amin` | `77` | yes |
| trajectory lifts on closure | `90` twice, `91` | `82`, with `82_probes/p1_box_lifting_of_p4.rs` | no |
| absorption against coherence | `61` twice | `61` | yes |
| two hypotheses | `63` three times, `74` | `57b:247-297`, `57_probes/p9_output.txt` | no |
| reachable interval | `57_orchard` heading anchor | `57` | yes |
| two law families | `74` | `67:647-654` | no |

The trajectory row is the odd one: its own `note` gives the instrument's path, `82_probes/p1_box_lifting_of_p4.rs`,
while its `provenance` names neither `82` nor that probe. The row knows where its evidence is and does
not cite it.

**Two cited numbers were recomputed rather than taken.** Probe 4, committed at `4d097ecf`.

- The never-inherited row's 82.7484% reproduces exactly: **13,882,880 of 16,777,216** triples, unsigned
  saturating, `(a+b)-c` against `a+(b-c)`, at eight bits. The negative control, the same composition
  under wrapping, reports zero, which a group requires, so the instrument is not simply reporting
  failure.
- The min-plus row's absorption sweep reproduces over its whole stated rectangle: `W in 2..=10`,
  `F in 0..=W`, unsigned, **63 cells**, the top absorbing at all 63 saturating cells and at none of the
  63 wrapping ones.

## The eleven, one section each

### 1. `a_composed_expressions_region_is_never_inherited_from_its_parts` — agree

**Provenance defective** as above. **Number verified exactly.**

I reach the same conclusion from a different argument and a different instrument, and I add the direction
the row does not have. The row's witness runs one way: a composite failing where its parts hold. Probe 1
section E establishes the other way, which matters because it kills the weaker reading that inheritance
at least bounds the region from above. Under unsigned wrapping at eight bits, **32,640 of 65,536 pairs
have `a+b` leaving the declared range**, so a per-part region excludes them, and `(a+b)-b == a` on
**all 65,536**. The composite's region is strictly larger than the intersection of its parts'.

Predicate for my instance, and it is mine rather than the row's: `total_width: W = 8`,
`fraction_width: F in {0, 2, 4}`, `signedness: signedness in {unsigned, signed}`,
`overflow_policy: overflow policy in {wrap, saturate}`, `rounding: rounding in {truncate, nearest}`,
`operation: operations {add, mul}`, `arity: arity = 3`, `threads: threads = 1`. The wrapping direction
additionally at `operation: operations {add, sub}`, `arity: arity = 2`.

### 2. `a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains` — agree with a narrowed region

**Provenance reaches a member file.** The row is careful in the way that matters: its `gap` already
refuses the biconditional, saying only the sufficient direction is proved and the necessary one is
measured rather than derived. I would have raised exactly that and there is nothing left to raise.

**The width intersection is empty and this is the important sentence in my file about this row.** Their
predicate is `W in {4, 5, 6}`. Mine is `W = 8`. Intersecting over values rather than over names, the two
of us agree about **no width at all**. The convergence therefore does not raise this row's standing on
the width axis, and anybody reading two instruments and inferring a widened width region is reading the
union.

Where we do intersect: `fraction_width: F in {0, 2}`, `signedness: signedness in {unsigned, signed}`,
`overflow_policy: overflow policy in {wrap, saturate}`, `rounding: rounding in {truncate, nearest}`,
`operation: operations {add, mul}`, `arity: arity = 3`, `threads: threads = 1`. Their `target_features:
target features any` does not intersect with mine as `any`, because I varied no target features at all
and under the notation an unvaried axis is one value, not a universal.

My separate instance, which is not a widening of theirs and is appended rather than merged: over 40
configurations at `W = 8`, absorption and measured associativity agree on every cell, in both
directions, with zero cells associative while absorption fails.

### 3. `a_law_layer_answers_whether_a_law_reaches_a_lowering_the_backend_cannot_prove` — agree with the second sentence, refuse the first

**Provenance defective**: cites `90` twice, established at `80` section 5.2.

**The second sentence is right and I corroborate it independently.** "Where a law pays is strictly
narrower than where it is true." Probe 2 reaches it from a construction the source did not use, and the
agreement is sharp:

- The backend proves wrapping-add associativity and acts on it. The assembler **aliases the two symbols**,
  `_wrap_right = _wrap_left`. Expressing that law buys nothing.
- The backend does not have unsigned saturating-add associativity, which probe 1 proves true
  exhaustively at eight bits. Two distinct bodies, no reassociation.
- The reduction is where it pays. `wrap_reduce` vectorises with 16 vector ops; `sat_reduce` emits **zero**
  and runs a byte at a time; the same computation with the reassociation written out emits **15
  `uqadd.16b`**, the hardware saturating add over sixteen lanes. The instruction exists, the backend
  reaches for it once the law is supplied, and will not supply the law itself.

**The first sentence I refuse, as a canon sentence, on two grounds.**

It fails permanence. "What the backend cannot prove" ranges over the toolchain, the target and the
optimisation level, and a canon sentence has to survive a rewrite in another language in another decade.
My own evidence for it is indexed `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `LLVM 22.1.6`,
`aarch64-apple-darwin`, and that index is not decoration: the sentence becomes false the week LLVM learns
to reassociate a saturating reduction, and it becomes false silently, because its own text does not
change. That is a fine predicate for a finding and a disqualifying one for a definition.

And it is refuted by the panel's own row 11. If the law layer answers only what reaches a lowering, then
the second consumer class that row names, and the third I name below, read a layer that does not answer
their question. **A law's truth is a fact about the format; whether the backend already knows it is a
fact about the backend. A layer defined by the second cannot serve a consumer who needs the first.**

The repair is one word of scope rather than a rewrite: this is the criterion for whether a law **earns an
arm**, and it is not what a law layer answers.

### 4. `a_min_plus_fold_needs_an_absorbing_top_and_wrapping_supplies_none` — agree, with three additions

**Provenance reaches a member file.** **Region overlap genuine**, not a point: probe 4 section 2 sweeps
the identical rectangle, `W in 2..=10` and `F in 0..=W` unsigned, all 63 cells, and reproduces absorption
at every saturating cell and none of the wrapping ones.

The wrapping half needs no sweep at all and I would state it as a theorem rather than a measurement:
addition modulo `2^n` is a group, and in a group `T + x = T` gives `x = 0` by cancellation, so no
element absorbs anything but the identity, at every width and both signednesses. The sweep is a control
on that, and it is a real one because it could have disagreed.

**Addition one, the signed top, which the row explicitly declines.** Its note says it says nothing about
a signed top. There is none: probe 1 section C finds no absorbing element for signed saturating addition
at eight bits, because `MAX + (-1) < MAX`. My instance, at `total_width: W = 8`, `fraction_width: F = 0`,
`signedness: signedness = signed`, `overflow_policy: overflow policy in {wrap, saturate}`,
`operation: operation = add`, `arity: arity = 2`, `threads: threads = 1`.

**Addition two, and it goes past the row's own `gap`.** The gap names monotonicity as the second property
a min-plus computation needs, measured elsewhere and unfiled. I checked the whole structure rather than
either property: at `W = 8`, `F = 0`, unsigned, saturating, with `min` as the sum and saturating `+` as
the product, **every semiring axiom holds**: product associativity, product commutativity, product
identity `0`, sum identity `TOP`, `TOP` annihilating, and distributivity. Unsigned saturation is a
complete min-plus carrier, which is a stronger and more useful statement than the absence of a
counterexample.

**Addition three, and it is a correction of emphasis.** Wrapping fails **two** axioms, not one. It has no
annihilating top and **it also fails distributivity**. The row names only the absorbing top, so a reader
repairing wrapping by reserving a sentinel value would still not have a min-plus carrier. That is
consistent with the retirement the row's gap points at, and it is worth stating positively rather than
as the reason a repair was retired.

**The caveat the arm carries.** `TOP` is a reachable value rather than a fresh point, so "no path" and "a
path costing `TOP`" are one bit pattern. The arm is sound only under `all reachable costs < TOP`, and
that condition is exactly the kind row 7 is about.

### 5. `a_range_clamp_and_a_coarsening_are_two_independent_sufficient_causes_of_multiplicative_non_associativity` — agree, with the region the row does not give

**Provenance reaches two member files.** The row's own note is unusually careful about what its predicate
does not transfer to, and it is right on every count.

My probe 1 appeared to contradict it: unsigned saturating multiply at `F = 0` is associative on all
16,777,216 triples at eight bits, while the row has a clamp alone breaking associativity at unsigned.

**It is not a contradiction, and finding out why took a refutation of my own.** My first isolation
rescaled `2F` bits back to `F` at every step, so coarsening fired throughout and the clamp was never
isolated; it reported 10,890 bad triples and I read that as refuting my mechanism. It refuted the test.
That run and its diagnosis are kept in probe 3's source rather than deleted.

The corrected isolation carries no rescale anywhere: scale grows `F`, `2F`, `3F` and the clamp applies to
the exact numerator against the logical bound, so the clamp is the only lossy step. The no-clamp control
reports **zero at every fraction width**, which is what says the harness is not lossy somewhere else.
With the clamp on and every operand allowed the law breaks at `F > 0`, **360,012 bad at `F = 2` and
1,496,400 at `F = 4`**, reproducing the clamp-alone result independently at a width the row does not
cover. With the clamp on and no operand below one, **zero at every fraction width**.

**So the clamp cause has a nameable region and the row gives only the prohibition.** An intermediate
clamp preserves multiplicative associativity on any operand set whose non-zero members are at least one,
because multiplication can then never bring a magnitude back down past the bound the clamp pinned it to.
`F = 0` unsigned satisfies that for free, since no operand below one exists there, which is why the two
results were never in conflict. Signedness is a third route out of the same set, via a negative factor.

This does not weaken the row. Coarsening remains independently sufficient, visible in probe 1's `F > 0`
wrapping columns where no clamp fires at all. What it adds is where the clamp cause does not reach.

My instance: `total_width: W = 8`, `fraction_width: F in {0, 2, 4}`, `signedness: signedness = unsigned`,
`overflow_policy: overflow policy = clamp`, `operation: operation = mul`, `arity: arity = 3`,
`threads: threads = 1`.

### 6. `a_resolved_strategy_never_inherits_a_composed_operations_laws_but_dominates_them_monotonically` — refuse the domination clause, agree the non-inheritance clause

**Provenance reaches a member file, singly, consistent with its standing.**

The non-inheritance clause is right and follows from row 1 plus one reason specific to strategies:
resolving a composite changes the realisation map at the intermediates, so a law established for a part
under the part's own strategy is a statement about a different lowering.

**The domination clause is refuted, exhaustively, at eight bits, by probe 1.**

| operation | signed wrapping | signed saturating |
|---|---|---|
| add | associative, 0 of 16,777,216 | **not** associative, 4,177,792 of 16,777,216 |
| mul | associative, 0 of 16,777,216 | **not** associative, 62,476 of 16,777,216 |

If a resolution rule resolves toward the more conservative side, and saturation is the more conservative
policy, then **the resolved strategy has strictly fewer algebraic laws than one of its operands had.**
Law-set containment runs against conservatism rather than with it. There is no ordering of these two
policies in which the law sets are monotone, because wrapping dominates saturation on laws and saturation
dominates wrapping on soundness.

The row's escape clause, "whichever operand's laws the resolution rule is defined to dominate", is
either vacuous, if the rule is defined to dominate whatever it happens to dominate, or false, if it means
the resolved laws contain the weaker operand's.

**And the row has an internal defect that blocks promotion independently of the refutation.** Its `says`
makes a claim about the laws of numeral operations. Its `predicate` scopes to
`operation: operation = resolve (strategy join)` and lists no width, no signedness, no policy and no
fraction width. Under I13's absence rule the law-set claim therefore holds in no situation where a width
exists, which is every situation a numeral is in. The predicate covers the join operator; the sentence
talks about the operations the join's result performs.

**The charitable reading, and I think it is what the author meant.** The `because` argues about
*exactness*, not about algebraic identities: an operator exact under one strategy and exact under another
need not be exact under the resolved one. Exactness plausibly is monotone in conservatism. Algebraic law
richness is not. **The row conflates two orderings under the word "laws", and in the `algebraic_laws`
topic the reading that loses is the one it needs.** Two arms, not one meta-law.

Refusing costs little: the row is `sentence_kind = "argument"` and its note offers it as a direction
rather than a settlement.

### 7. `a_trajectory_condition_lifts_into_a_declaration_exactly_when_it_survives_closure` — agree with a narrowed region, and the row contradicts itself

**Provenance defective**, and peculiarly so: the `note` names the instrument as
`82_probes/p1_box_lifting_of_p4.rs` while the `provenance` names neither `82` nor that probe.

**The row's `says` and its own `gap` cannot both be true.** The `says` states "exactly when", a
biconditional. The `gap` states "Stated as one-directional sufficiency by its own author." The
biconditional is the strongest claim in the sentence and it is the one the gap withdraws. This matters
rather than being pedantry: "exactly when" licenses **refusing** a declaration that fails closure, and
the refusing direction is precisely the one the row says is unestablished. A totality claim needs what
enforces it named, and here nothing does.

**What I add rescues the biconditional inside a named region rather than deleting it.** Closure and
reachability come apart in general, which is the ordinary reason an invariant needs strengthening to an
inductive one, and where they come apart a condition can fail closure and still be a sound declaration.
But arvo is a library, and op says so in I11: *"We are a library, not a program, so we don't know how end
users will use us."* **At a public boundary the consumer is universally quantified over the admitted
operations, so the reachable set is exactly the closure and the biconditional holds.** Inside a chain
whose schedule is statically known, the reachable set is a strict subset of the closure, and the gap
between them is exactly what a chain-local declaration is entitled to exploit.

So: two arms. `boundary = public API` gives the iff. `boundary = internal chain with a known schedule`
gives sufficiency only. The row as written is right where arvo meets its consumers and wrong exactly
where the composite work lives, and both halves are worth having.

This is a derivation rather than a measurement and I mark it as such. I did not build an instrument for
it, and it needs one before it is more than an argument.

### 8. `absorption_is_the_correct_associativity_criterion_and_literal_coherence_is_a_strictly_narrower_law` — agree on the criterion, width intersection empty

**Provenance reaches a member file.**

**What I independently corroborate is the criterion half.** Over 40 configurations at `W = 8` spanning
both signednesses, both policies, `F in {0, 2, 4}`, both rounding modes and `{add, mul}`, absorption and
measured associativity agree on **every cell in both directions**: no cell is associative while
absorption fails, and none is the reverse.

**And I have to bound what that establishes, because half of it is a theorem and measuring a theorem
proves nothing.** If both absorptions hold then both associations collapse to the exact value rounded
once, so associativity follows by construction. The informative half is the converse, and the informative
number is the count of cells associative while absorption fails. It is **zero**. That is *observed*
necessity, not derived necessity, and it sits at the same standing as row 2's gap rather than above it.

**The width intersection with the row is empty**, `W = 4` against `W = 8`, so the convergence raises
nothing on that axis. Their arity is 2 and my absorption is the triple form, so the two formulations do
not straightforwardly intersect there either. What my instance does have that theirs does not: both
signednesses, both policies, three fraction widths and both roundings, where the row's own note says its
evidence is unsigned only and does not transfer to the signed domain.

**One drift worth naming.** The `id` says literal coherence is "a strictly narrower law". The `says` says
the two are **the same** law on the domain a real fold presents and that coherence is **wrong** where
they part, never merely imprecise. Narrower and wrong-where-different are different claims, and the id
states the harmless one. Since the id is what a reader greps and what the promotion list prints, the
weaker sentence is the one that travels.

### 9. `the_laws_of_a_format_are_derived_from_two_hypotheses_rather_than_enumerated_per_policy` — agree, and this is the one to promote first

**Provenance defective**: cites `63` three times and `74` once, both consolidations, while `63:389-392`
itself names the establishing source as `57b:247-297` with `57_probes/p9_output.txt`.

**This row earns the strongest check I could build, and it passed it.** Rather than agree with it, I took
its two closed forms as a **predictor** and ran them out of sample at `W = 8`, where the row has no
evidence at all. The predictor is written in probe 3 as a function that never calls the measurement.

**24 of 24 cells agree, in both directions, with the predictor returning both polarities across the
cube.**

**The decisive cell was chosen because it could have failed.** The closed form says the kernel is a
multiplicative congruence iff the range is mirror-symmetric or nonnegative. So on the mirror-symmetric
signed range `[-127, 127]` under saturation at `F = 0`, the form predicts multiplication **associative**,
while the ordinary reading that signed saturation breaks associativity predicts the opposite. Measured:
**0 bad triples of 16,581,375**. The closed form is right and the ordinary reading is wrong.

**And the differential is what no account keyed on signedness can produce.** On that one range,
saturating, `F = 0`: multiplication fails on **0** triples and addition fails on **4,129,024**. The two
operations split on a single range, exactly as two separate closed forms require, because
mirror-symmetric is a multiplicative congruence and is not sign-confined.

It also retro-explains every saturating cell of probe 1, which I measured and committed before reading
this row: unsigned is nonnegative so multiplication associates and is sign-confined so addition does;
two's complement is neither, so both fail. I did not have that explanation when I took those numbers.

**On the region, and it cuts the same way as rows 2 and 8.** Their predicate is `W = 4`; mine is `W = 8`;
the intersection over values is empty. But an out-of-sample prediction that could have failed and did not
is a different kind of evidence from an overlap, and for a `theorem` row it is the better kind. Under the
notation the row still claims nothing at `W = 8`, so mine is a new claim appended beside it:
`total_width: W = 8`, `fraction_width: F in {0, 2}`,
`signedness: three range geometries, nonnegative and two's complement and mirror-symmetric`,
`overflow_policy: overflow policy in {wrap, saturate}`, `operation: operations {add, mul}`,
`arity: arity = 3`, `threads: threads = 1`.

**One defect in the row's predicate.** It reads `fraction_width: F in {0, one unnamed value above 0}`.
An unnamed value is not a region and nothing can gate on it. My instance names `F = 2` and the row should
either name its own or drop the entry.

### 10. `the_reachable_interval_is_the_true_grade_and_width_is_its_lax_abstraction` — cannot settle, plus one finding on its note

**Provenance reaches a member file.**

**I cannot second this row, because my instrument does not test what it claims.** Its claim is about the
width *composition rule*, `g(W,V) = max(W,V) + 1`, failing associativity for addition while the
reachable-interval rule composes exactly, at `W = 4`, signed, addition, chain length 1 to 6. I measured
containment regions for multiplication. That is a different proposition and agreeing with it from where I
stand would be the courtesy agreement the brief warns about. **"I read this and could not arrive at it
separately" is my verdict on the measured claim.**

What I have instead is adjacent and is offered as its own finding rather than as corroboration. For
signed saturating multiplication the containment a law needs is at the **nested position and nowhere
else**. The largest symmetric operand interval `[-k, k]` carrying associativity at eight bits is
**`k = 11`**, set by the pairwise product (`11² = 121` fits `[-128,127]`, `12² = 144` does not), not by
the triple product. I predicted `k = 5` from the triple product and the control refuted me. Once both
inner products are exact the two associations hand the outer clamp the same exact value, so the outer
clamp cannot separate them however hard it saturates.

And the pairwise-containment predicate, which is what a const gate would actually test, is **sufficient
and not necessary**: of 16,777,216 triples, **16,497,435** have an inner product leaving the range and
**16,434,959** of those associate anyway. The gate refuses 16,434,959 triples on which the law holds.
That is the price of const-decidability stated as a number instead of an adjective, and it is large.

**The finding on the row's normative note.** It proposes "the reachable set as the intent, width as the
derived conservative quantity an implementation computes from it". On the value axis that is right. Width
also fixes storage, and a reachable interval says nothing about footprint: two numerals with one interval
and two widths pack differently and lower differently. I17 is op's, it says the storage-minimising path
is not deprioritised and that arguments for downgrading it are not entertained, and a canon sentence
calling width a derived quantity will be read as licensing that trade by anybody who does not already
know I17. **The sentence needs its axis named.** Width is the lax abstraction *of the value semantics*
and is primary for footprint, and those are two statements rather than one.

### 11. `the_two_law_families_have_two_consumer_classes` — agree the families, refuse the closure at two

**Provenance defective**: cites `74` alone, a consolidation, whose own text at `74:632` names `67:647-654`.

**My phase-one derivation of this row was wrong**, and I record that rather than quietly replacing it. I
derived the two families as lowering-consumed against programmer-consumed. The row's two are
order-transport-consumed against reassociation-consumed. Both of the row's consumers are machine
consumers.

**That the two named families are distinct and independently inhabited I accept**, though it is carried
from the format topic as a law row rather than established here, so seconding it is seconding somebody
else's measurement I have not repeated.

**What I refuse is the closure the sentence implies.** "Each family has a distinct consumer that reads it
and not the other" reads as an exhaustive mapping, and there is at least a third consumer neither family
covers: **a downstream program's author, reasoning about their own code.** op in I11: *"We are a library,
not a program, so we don't know how end users will use us, however, our main selling point are the algo
crates ... as well as the contracts for things that compose to bigger units than just numerals alone."*
That consumer reads a law to know what they may write, and the laws useful to them include approximate
ones with a stated bound, which are worthless to both of the row's families: a rewrite must be
observationally exact, and order transport needs the order rather than a bound.

**This is the same finding as my refusal at row 3, arriving from the other side**, and the two should be
read together. Row 3 defines the law layer by what reaches a lowering. Row 11 already names a consumer
that is not the lowering. **The two rows are in the same topic, both at `one_expert`, and they cannot
both be promoted as written.**

## What I would do before any of these is promoted

**Repair the five provenance chains.** The establishing file is named inside the cited text in every one
of the five, so this is transcription rather than research: `79` for the never-inherited row, `80` for
the law layer, `82` for the trajectory row, `57b` for the two hypotheses, `67` for the two law families.
None of the five is checkable until that lands, and under the ratification model `standing` is now the
promotion gate.

**Resolve the row 3 and row 11 conflict before either moves.** They are in one topic, at one standing,
and one defines a law layer by a consumer the other says is one of at least two.

**Fix the row 7 `says` and `gap` contradiction.** "Exactly when" against "one-directional sufficiency by
its own author" is not a hedge, it is two different sentences.

**Name the unnamed fraction width in row 9's predicate**, or drop the entry. An unnamed value cannot be
gated on.

**And treat every width intersection in this file as empty until somebody says otherwise.** Three of the
rows I second (2, 8, 9) are evidenced at `W = 4` to `W = 6` and I am at `W = 8`. Intersecting over values
rather than names, our agreement about width is empty in all three cases. The agreement is real about
the other axes and about the propositions; it is not a widening, and a reader who reads two instruments
and takes the union has taken something neither of us established.

## The eleven verdicts

| row | verdict | provenance reaches its establisher |
|---|---|---|
| composed region never inherited | agree, second direction added | no, `79` |
| congruence at every nesting | agree, width intersection empty | yes |
| law layer answers what the backend cannot prove | agree the second sentence, refuse the first | no, `80` |
| min-plus needs an absorbing top | agree, three additions | yes |
| clamp and coarsening independently sufficient | agree, region added | yes |
| resolved strategy dominates monotonically | refuse the domination clause | yes |
| trajectory lifts exactly when it survives closure | agree with a narrowed region, row self-contradicts | no, `82` |
| absorption is the associativity criterion | agree, width intersection empty | yes |
| laws derived from two hypotheses | agree, strongest of the eleven | no, `57b` |
| reachable interval is the true grade | cannot settle, one finding on its note | yes |
| two law families, two consumer classes | agree the families, refuse the closure at two | no, `67` |

## Outside the question I was asked

**`proposal.standing` is a declaration nothing reads.** 99 rows carry it, across four values
(`one_expert` 71, `two_experts` 17, `cross_topic` 6, `three_or_more` 5), and `grep -rn standing
mock/checks/src/` returns six lines, every one of them inside a single branch: `shape.rs:171-185` reads
`probe.get("standing")`, and `shape.rs:229-230` is a comment about it. The field read belongs to a
**probe** row and never to a proposal's. Nothing checks that a `two_experts` row names two files, that
its provenance resolves to two distinct experts, or that the value is even in the declared set. The
positive control for that grep is `predicate`, which the same command shape does find being read, at
`predicate.rs:29` and `shape.rs:265`, so the zero is about the field rather than about the instrument.
Under the ratification
model this is now the promotion gate, and it is the tautological-declaration shape the repo's own
`the-test-gate.md` names: ask what value would make it fail, and the answer is none.

I found this before the dispatcher's steer arrived and it says another seat has since landed a check at
`mock/checks/src/provenance.rs` on `research/canon-registry`. So this is convergence rather than a new
report, and I record it as corroboration from a different starting point rather than as a second finding.

**The suite is real and I say so because a green suite is otherwise worthless evidence.** 120 tests, all
passing. I read the bodies in the surface I touch rather than the names. `every_predicate_names_a_declared_axis.rs`
carries four planted negative controls, each asserting the exact `kind` string, and its own module
comment says the first version would have passed against a registry with the check commented out.
`what_one_field_obliges_another_to_carry.rs` plants two probe rows with **identical control text
differing only in `standing`** and asserts the arm reads the field rather than the prose. That is what a
control looks like. I found no tautological test and no sampled law in what I read.

**A caution about the three rows I second whose evidence is at four to six bits.** Rows 2, 8 and 9 are
evidenced below eight bits and I am at eight. Nothing in this workspace's own rules licenses carrying a
property from a model width to a real one for free: `unstable-features.md` states that closing the
specialisation and `TypeId` doors is **necessary and not sufficient** for that transfer, and that a
property checked at a model width needs its own transfer argument, named rather than assumed. Row 9 has
one and it is the best kind: a closed form that predicts, tested out of sample at a width outside its
evidence, and it held on 24 of 24 cells. Rows 2 and 8 do not have one, and their agreement with me is
about the proposition rather than about any width.

**One unexploited win, noted and not pursued.** The vectorised saturating reduction in probe 2 folds its
sixteen lanes back to a scalar with sixteen dependent `csel` steps. A log-depth `uqadd` tree does it in
four. It is small, it is real, and it is unpriced: pricing it needs `mock/benches/` and I did not take a
timing anywhere in this file.
