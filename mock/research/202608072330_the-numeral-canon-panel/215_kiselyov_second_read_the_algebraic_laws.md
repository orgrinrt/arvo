# 215. Second read, the eleven `algebraic_laws` rows at `one_expert`

Seat 215. Kiselyov. The eleven rows named in the brief, each given an independent reading before its
`says` was opened.

**Phase one below is blind.** For each row I read the `id` and the `topic` string and nothing else: not
`says`, not `because`, not `note`, not `provenance`, and no panel file about any of these questions. What
I did read first, and what therefore bounds the independence of everything here: `mockspace.toml`,
`mock/registry/topic.toml`, `mock/registry/strategy.toml`, `INTENTS.md` in full, `mock/Cargo.toml`, and
the `mock/checks` test bodies for the standing gate. Those are canon and process rather than answers to
these eleven questions, but `strategy.toml` and `INTENTS.md` do carry op's framing of the strategy axis
and of I13's predicated arms, and any convergence I reach with the row author on *those* framings is
shared input rather than an independent instance. I say so per row where it bites.

This file is committed in two acts. Phase one alone, first, so the commit is the evidence of blindness.
Then the reconciliations appended.

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
