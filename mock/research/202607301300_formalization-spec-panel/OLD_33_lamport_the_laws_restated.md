# 33. The laws restated, against the identity that now exists

**Member:** Leslie Lamport. Basement lens: say precisely what is claimed, over what, under which
relation, quantified over what, and keyed on what. A law whose statement is imprecise is not a weak
law; it is not a law. Where reasoning stalls, write the smallest model that captures the disputed
behaviour and let the checker enumerate what intuition could not.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: every suite reports
`ok`, zero failures anywhere, nine ignored (one unit-test ignore, six doctest ignores in one crate,
two in another), matching the counts files 31 and 32 report from their own runs, so nothing regressed
under this dispatch. I did not re-audit `identity_laws.rs` or the nine compile-fail pairs under
`crates/arvo/tests/ui/`; file 31 walked that surface in detail (`31:8-19`) and nothing below touches
it. I read the bodies of what I did touch: there is no `Monotone`, `Distributes`, `Associative`,
`AddAssoc` or `Magma` anywhere in `mock/crates/` (`grep -rn` returns nothing for any of them), and
there is no `arvo-algebra-contracts` directory, so every algebraic law in this review is at design
stage and nothing in the suite could be tautological about one. Canon gate: the governing calls are
the D-numbered ones in `202607301000_topic.inherited-state-from-the-formalization-round.md` and
`202607301200_topic.the-formalization-spec.md`, subordinate to op's seventh checkpoint
(`30b_op_checkpoint_seven.md`) and to the identity contract files 31 and 32 settled. Section 9 says
where this file argues near a standing call.

**What I read:** `26_consolidation_two.md` in full. `30b_op_checkpoint_seven.md` in full.
`31_arntzen_settling_the_identity_contract.md` in full, section 4 as the primary input.
`32_aaltonen_does_identity_lower_well.md` sections 0, 7 and 8. The directory listed once. Of the
design rounds, `202607301000_topic.inherited-state-from-the-formalization-round.md` at D13, D16, D45,
D46, D47, D51 and D52, read directly rather than trusted from any file's citation of them, because
this review has already caught citation drift twice (`30:51-57`, `31:110-112`). No other panel file
fetched; I did not reread my own file 18, and nothing below assumes it.

**What I compiled**, separated from what I reasoned. Five probes in `33_probes/`, every claim a
`const` assertion so the compiler either accepts the file or refuses it. Sections 1 through 6 rest on
those, and each cites the probe by name. **Everything in sections 7 and 8 is reasoned**, built on the
compiled facts but not itself compiled, and marked as such. Two probes refused on their first build,
each time because a claim of mine was wrong rather than the code: I had the interior-safety bound at
`n` where it is `n-1`, and I had quantified the homomorphism condition over a set where it is
vacuous. Both are recorded in `33_probes/OUTCOMES.md` and in place below, because a probe that only
ever passed is not evidence that it was checking anything.

## 0. What a law is, in one sentence

**A law is a claim that two terms built from an operation stand in a stated relation, over the value
set of a numeral, quantified over a stated domain, and keyed on every parameter its proof used.**

Five things have to be named for that sentence to mean anything: the terms, the relation, the value
set, the quantifier, and the key. The algebraic half of this design has been carrying disagreements
that are, every one of them, a disagreement about which of the five was left unstated. The identity
half now settles two of the five outright and makes a third computable. What follows says all five,
for every law the design has, in the form the next consolidation could take close to verbatim.

The organising claim of this file is that the algebra's four open questions are not four questions.
They are one question asked at four places: **which of the five slots was elided.** The relation
question elides the relation. The accumulator question elides a quantifier parameter. The
"belongs to a type or an operation" question elides the key. The ladder question elides the value set
and its order. Naming all five slots at once dissolves three of the four and sharpens the fourth into
a naming call rather than a mathematical one.

## 1. The relation: there are three, they are ordered, and the design should name all three

The consolidation states the relation question as undecided and says it decides how `Precise` reads
(`26:608-617`). It also says the design currently reports one fused verdict where the mathematics
supports at least two independent facts, value-agreement and definedness-agreement (`26:218-221`).
Both are correct. The missing piece is that this is not a gap in the vocabulary. It is a vocabulary
the design has not imported.

### 1.1 The notion the design does not name is a hundred years old

A partial algebra is a carrier with operations that may be undefined at some arguments. Its equations
come in three standard strengths (Kleene's usage, systematised by Burmeister for partial algebras):

| Name | Written | Claim |
|---|---|---|
| weak equation | `t1 =w t2` | if both sides are defined, they are equal |
| existence equation | `t1 =e t2` | both sides are defined, and they are equal |
| Kleene equation | `t1 ~= t2` | both defined and equal, or both undefined |

The design's "partial associativity, a notion the design does not name and the imported vocabularies
do not carry" is the weak equation. It does not need inventing, it needs importing, and the import
brings the other two with it, which is the part that pays.

Nothing here is a new mechanism. It is a name for a distinction the design already measured and then
discarded by fusing.

### 1.2 The three relations, and what each buys a caller

Probe 2 (`33_probes/probe_2_interior_safety_upgrades_weak_to_kleene.rs`) measures all fourteen
groupings of a five-element fold over all `8^5 = 32768` inputs of an eight-value signed numeral under
`Precise` addition, at four accumulator ranges. Value disagreements: **zero, at every width**.
Definedness splits: **216, 36, 0, 0** as the accumulator widens. The weak equation holds
unconditionally for this operation; the Kleene equation holds only above a threshold.

That separation is what lets a combinator state what it needs instead of accepting or refusing a
fused verdict:

**Value agreement (weak).** Whenever the regrouped fold returns, it returns the same number the
sequential fold would have returned. This is **soundness of the regrouping**: it never produces a
wrong answer. A combinator that regroups purely for speed, and whose caller treats a refusal as an
error either way, needs exactly this and nothing more.

**Definedness invariance.** The regrouped fold returns exactly when the sequential fold returns. With
value agreement this is the Kleene equation. This is **completeness of the regrouping**: it never
produces a spurious refusal on a computation that would have succeeded. A combinator whose caller
branches on a refusal, or whose refusal is a user-visible outcome rather than a bug report, needs
this too.

**Event invariance.** The regrouped fold performs the same multiset of quantisation events as the
sequential one. This is strictly stronger than value agreement, and it is what a caller propagating an
error bound needs, because two groupings can deliver the same number by different numbers of roundings
and a bound computed from one does not apply to the other. The consolidation's graded reading already
names quantisation events as a component of the grade alongside refusal causes (`26:205-213`); this is
that component read as a relation rather than as a type.

So the ordering is: weak, then Kleene (weak plus definedness invariance), then graded equality (Kleene
plus event invariance). The design should name all three, and should name the fused one `Associative`
because that is what a reader will assume `Associative` means, with the weaker fact carrying an
explicit name rather than the strong one carrying a qualifier.

### 1.3 What this settles about `Precise`, and what it leaves as a naming call

The consolidation frames the fork as a question about what `Precise` is for, one reading giving its
zero numeric spread its own weaker law name, the other saying a fold whose definedness depends on
grouping is unusable regardless of what the values do (`26:608-617`). Measured, `Precise` addition
**satisfies the weak equation and fails the Kleene equation below the accumulator threshold, and
satisfies both above it** (probe 2, all four rows).

That is not a compromise between the two readings. It is the observation that they are answers to
different questions, and both are right about the question they answer. The first reading is right
that `Precise` has a real, statable property that signed clamping does not: clamping fails even the
weak equation, because its regrouping diameter is genuine numeric divergence (`26:709-712`), whereas
`Precise`'s is entirely definedness. The second reading is right that a combinator whose caller
branches on refusals cannot use the weak equation alone.

Both are served by naming the two facts and letting the combinator state which it requires. What is
**not** settled by this, and I am not going to manufacture a preference where the design has not
decided one, is whether the design ships `Precise` with a combinator surface that only offers the
Kleene-requiring form. That is a question about what a `Precise` consumer expects, exactly as file 31
declined to resolve the parallel dither question (`31:180-189`), and it is the same shape of question:
a property that only bites near a boundary is the shape of bug nobody writes a test for.

The reading I would push back against, and state so the next member can weigh it, is the one that
says the weak equation is not worth naming because a spurious refusal is bad enough. It is worth
naming for a reason independent of `Precise`: the weak equation is the only one of the three that
**wrapping addition and saturating addition can be compared under at all**, since one satisfies the
Kleene equation vacuously (it is total, so definedness invariance is free) while failing value
agreement, and the other is the reverse case. A relation under which a total operation automatically
scores well is a relation that flatters totality, and the design has three total resolutions and one
partial one.

## 2. The equality the relation is over, which identity now settles

Every relation in section 1 is built from an equality on results. The algebra half has been assuming
one without naming it. The identity half has now stated it, and the statement is stronger than
anything the algebra half would have been entitled to assume.

### 2.1 Laws are stated on values, never on data, and this is a contract rather than a convention

File 31 carries the charter forward verbatim: "`Lowering` changes no value. `Encoding`, nested inside
it, may change which datum carries a value. Every operation whose result depends on that is declared a
datum-level operation, and **no law may read one**" (`31:361-363`).

That sentence does three things for the algebra that nobody has collected:

It **fixes the equality**. Law equality is equality of values after `decode`, not equality of stored
data. Signed zeroes are one value. Decimal cohort members are one value. A NaN payload is invisible.
The section-retraction triple (`31:370-374`) is what makes this well defined: `decode . encode = id`
on values always, so a value has a unique meaning regardless of which datum carried it.

It makes `Lowering`-blindness **contractual rather than mechanical**. File 11 tried to secure it with
a crate split, which was tested and failed because the crate owning `Number` is by construction a
crate where `Lowering` has methods a where-clause can name (`26:513-521`), and fell back on a phantom
carrier proof. Neither is needed if the law's key simply does not have a `Lowering` slot. Under the
const-fn-is-the-key discipline (`26:174-186`), a parameter that is not in the parameter list cannot be
read, and reading it anyway fails with `E0425` at the point of use. The enforcement is scope
resolution, which is the cheapest mechanism available and needs no crate boundary and no phantom type.

It **separates two kinds of derived fact that must not share a vocabulary**. Laws are value-level.
`DatumDeterministic` (`31:404-408`) is datum-level. They are both derived `const fn`s keyed on a
composition, they look identical at the call site, and one of them may read `Encoding` while the other
may not. The design should mark the distinction at the declaration, for the same reason D16 puts the
derived/asserted distinction at the declaration: from a call site the two are indistinguishable.

### 2.2 The one place value equality is not an equivalence relation, and how identity fixes it

A law is quantified over a value set. For a numeral with `Specials`, that set contains NaN, and IEEE
`==` is not reflexive at NaN. So value equality on a specials-carrying numeral is a partial
equivalence relation, and every associativity statement is false at NaN for a boring reason that has
nothing to do with associativity.

This is not a problem the algebra half has to solve, because the identity half already supplies the
pieces. `Specials` is an identity member, so NaN-ness is a **value** fact; the NaN payload is a datum
fact living in `Encoding::Canonical` (`31:357`); and `TotalOrd` already ships as a `pub const trait`
in `arvo-numeric-contracts` (`mock/crates/arvo-numeric-contracts/src/lib.rs:65`), with a doc comment
that says in as many words that it exists so float-bearing arvo types can have a strict-NaN-policy
total order without conflicting with partial `PartialOrd`.

So: **law equality is the equality induced by the composition's total order, not by `PartialEq`.** Two
results are law-equal when they are the same point of the numeral, or the same special. This is
reflexive at NaN, it is datum-blind by construction (the payload is not a value), and it needs no new
trait, because the trait it needs is already declared and already justified in its own doc comment for
exactly this reason.

I have not compiled this. It is the one place in sections 1 through 6 where I am reasoning from the
settled contract rather than measuring, and I flag it as the first thing to check.

### 2.3 What the settled quantiser does to the value the law sees

File 31 settles the quantiser as a composite: round on the unbounded-exponent extension of the grid,
then classify the rounded result against the range and resolve (`31:378-384`). This is the single most
useful thing the identity half hands the algebra half, and it has not been used yet.

The composite is `quantize = resolve . classify . round`. The two stages read different axes:
`Direction` drives `round`, `Resolution` drives `resolve`. They have different structure, and the
composite's properties are derivable from the stages rather than measured for the whole.

Probe 5 (`33_probes/probe_5_direction_enters_the_key_iff_the_lattice_opens.rs`) checks the first stage
directly: **every `Direction` instance is monotone**, over every denominator tested, with a wrapping
rule as the negative control that the same check refuses. So the round stage never costs a composition
its monotonicity. Every failure of monotonicity in the design comes from the resolve stage, and
specifically from the two resolutions that are not directions: `ReduceModulo` and `SubstituteZero`.

That is a clean division of labour, and section 5 uses it.

## 3. The key: what a law is keyed on, and what identity removes from the key

The consolidation's key rule is right and I am not going to restate the argument for it: a law's key
must include every parameter the underlying proof actually used, and expressing the fact as a `const
fn` whose parameters are its key gets completeness in the "forgot a parameter" direction for free
(`26:148-186`). What follows is what the settled identity contract now lets that key be, stated
exactly.

### 3.1 The key, stated

A law fact about an operation is keyed on:

| Slot | Why it is in the key | Can it be elided |
|---|---|---|
| the operation | the same recovery map has different structure under different operations, measured twice independently (`26:765-768`) and reproduced in probe 4 | never |
| the operand numeral or numerals | the value set the law quantifies over, and the lattice the exact result may or may not land in | never |
| the result numeral | `mul_full` maps into a different numeral than its operands, so the law is not an equation in one algebra until this is named | never for widening operations |
| `Quantisation::Direction` | in the key exactly when the exact result can leave the operand lattice, which is computable (section 3.2) | derivably, and the derivation is one predicate |
| `Quantisation`'s resolutions | which recovery map fires past each range end, which is the whole variation in structural class | never |
| `Growth` | decides whether a quantiser is present at all between the exact operation and the result | never |
| the accumulator numeral | measured to change the verdict with no axis changed (`26:151-157`), reproduced in probe 2 | never, for any law about a fold |
| the arity or unroll factor | the accumulator's sufficiency is a closed form in it | never, for any law about a fold |
| `Lowering`, including `Encoding` | nothing: it is forbidden from the key by the charter at `31:361-363` | always, by construction |

The last row is the one identity settles. Every previous statement of the key had to argue for
`Lowering`-independence. It is now a contract, and the const-fn-key mechanism enforces it by omission.

### 3.2 `Direction` is in the key exactly when the exact result can leave the operand lattice

The consolidation records, as two measurements about two operations, that `Precise` addition never
rounds in range while `Precise` multiplication rounds on roughly half of pairs (`26:167-172`). Those
are not two facts. They are one predicate evaluated at two operations.

Probe 5 measures it. Over a sixteen-value window, changing `Direction` across all four instances:

- **addition on a common numeral**: identical results at every operand pair, all four directions.
- **narrowed multiplication**: the directions disagree at **128 of 256** operand pairs, exactly half,
  matching the consolidation's "roughly half" from an independent model.
- **`mul_full`**: identical results at every operand pair again.

The deciding predicate is a statement about the numeral, not about the operation, and probe 5 checks
it against direct exhaustive computation in both directions over a grid of rational numerals:

**Additive lattice closure holds exactly when `bias / adjustment` is an integer.** The shipped
`AddClosed` gate on `Bias = Zero` (`26:326-331`) is the special case. There are numerals with nonzero
bias that are additively closed and that the shipped gate would refuse.

**Narrowed-multiplicative lattice closure additionally requires the adjustment itself to be an
integer**, and the bias too. Setting the two free integers to `(0,0)`, then `(1,0)`, then `(1,1)` in
`m = q*k1*k2 + b*(k1+k2) + (b^2 - b)/q` forces the three conditions in turn.

The consequence needs no case analysis. **Every fixed-point numeral with at least one fractional digit
has quantum below one, so its adjustment is not an integer, so narrowed multiplication is never
lattice-closed for any of them.** That single line is why multiplication needs `mul_full` and addition
does not, and it replaces the design's current framing of that asymmetry as two measured behaviours
with one derived condition.

The design rule that follows, and it is checkable rather than judged: **`Direction` enters a law's key
exactly when a `quantize` call sits between the exact operation and the result.** Since the design
already funnels every such call through one named map (`26:236-243`), this is syntactically visible in
the operation's own definition rather than requiring an analysis.

### 3.3 `mul_full` is not a binary operation, and its associativity is a claim about numerals first

This is the sharpest thing the settled identity does to the algebra, and it is not in any file.

File 31 settles the product numeral: `adjustment = gcd(A1*A2, A1*B2, A2*B1)`, `bias = B1*B2`
(`31:399-400`). So `mul_full` is a family of maps `N1 x N2 -> mulnum(N1, N2)`. It is not an operation
on a set. Which means **"`mul_full` is associative" does not typecheck as an equation until the
numeral-level map is known to be associative**: without that, `(x*y)*z` and `x*(y*z)` are values in two
different numerals and there is no relation to state between them.

Nobody has said this, and it is a precondition of the multiplicative half's headline claim that
`mul_full` is "commutative, associative, distributing over exact addition, at every strategy, with
laws free because no quantiser is present" (`26:236-238`). The laws are free of *quantiser* trouble.
They are not free of this.

The condition holds. Probe 3 (`33_probes/probe_3_product_numeral_is_associative.rs`) checks
`mulnum(mulnum(N1,N2),N3) = mulnum(N1,mulnum(N2,N3))` in both components over every ordered triple
from a 6x5 grid of (adjustment, bias) pairs, 27000 triples, plus commutativity over the same grid. It
holds, and the reason is visible in the closed form the probe also verifies:

**The n-ary product numeral's bias is the all-bias monomial `B1*B2*...*Bn`. Its adjustment is the gcd
of every monomial that carries at least one adjustment.** At arity three that is seven terms: one
`AAA`, three `AAB`, three `ABB`. The set is symmetric under permutation of the factors, so no
bracketing can favour one, and associativity and commutativity both follow from the symmetry rather
than from arithmetic luck.

Two consequences worth carrying:

**The formula generalises rather than replacing.** With every bias zero it collapses to the plain
product of adjustments, which is the shipped exact-product width rule, verified at arity three in the
probe (`4*6*3 = 72`, bias zero). File 31 records the same collapse at arity two (`31:212-215`) and
calls it the load-bearing property. It survives to n factors.

**The cross terms are load-bearing at arity three too.** The negative control: for the triple with
adjustments `(4,6,3)` and biases `(2,4,5)`, the true adjustment is 4 and the naive cross-term-free
product is 72, which fails containment. File 31 established this at arity two (`31:218-224`); the
same failure mode is present at three, so the closed form is not an arity-two coincidence extended by
optimism.

The honest limit carries too, and I confirm rather than extend it: the lattice **contains** the
product set and is not claimed to be the finest one (`31:226-228`), which is the safe direction for
closure and the wrong direction for a tight width bound.

## 4. The accumulator: a combinator parameter with a derived side condition, stated in mathematical coordinates

The consolidation holds three readings as live: the accumulator as a side condition with a closed-form
threshold (its current best reading), as a free combinator parameter, or as an eleventh `Policy` axis
(`26:619-622`). The identity half decides this, and the argument is short.

### 4.1 It is not an axis, and the reason is now structural rather than aesthetic

An axis of `Policy` is a property of a **type**, held by every value of it. The settled identity
contract makes this precise: `Numeral` says which numbers exist and `Policy` says what happens when a
result does not land in them, and both are properties of every value of the composition. An
accumulator is not a property of any value. It exists inside one combinator, for the duration of one
fold, and two folds over the same values may legitimately use different ones.

Putting it on the type would also collide with `arvo-toolbox-not-policer.md` in a way the other
readings do not: a `Policy` axis instance is a choice the consumer makes once at the type, whereas the
sufficiency condition below is a **lower bound**, and a consumer must stay free to exceed it.

So the reading that survives is the first two combined, and they were never in competition: **the
accumulator is an ordinary parameter of the combinator, its sufficiency is a derived side condition,
and it is in the law's key.** The combinator has an arity and a destination numeral; the required
accumulator follows from those two by closed form; the consumer picks any accumulator at or above it.
The middle reading (a free parameter with no side-condition reasoning) is the same design with the
check deleted, and the check is what makes the fold's law true.

### 4.2 The threshold, in mathematical coordinates rather than in stored bits

D69 is overturned: identity is parameterised in mathematical coordinates, with total width derived on
the physical side (`30b:9-13`). The interior-safety threshold is currently written the other way
round, as a width formula in bits (`26:157-158`, `26:269`), which is the radix-two encoding of a
statement that is radix-free. Since the design has no radix axis and every other formula is written
generically over a radix nothing carries (`26:643-647`), restating this one costs nothing and removes
one of the places the missing radix would have to be introduced.

**Interior safety, stated in value coordinates.** Let the destination numeral `N` have value set
`V(N)`, and the accumulator numeral `M` have value set `V(M)`. A fold of arity `n` over `N` with
accumulator `M` is interior-safe when both of:

1. **Lattice refinement.** `V(M)`'s lattice refines `V(N)`'s: every exact sum of members of `V(N)` is
   a point of `V(M)`. For addition this is the additive-closure condition of section 3.2 applied to
   the pair.
2. **Range containment.** `(n-1) * [min V(N), max V(N)] is contained in [min V(M), max V(M)]`.

The two conditions correspond exactly to the two stages of the settled quantiser: refinement is the
condition that the **round** stage is the identity at every interior node, and containment is the
condition that the **resolve** stage is. Interior safety is precisely "the quantiser is the identity
in the interior", which is a better sentence than a width inequality because it says why.

For a multiply-accumulate, apply the same two conditions with `N` replaced by the product numeral
`mulnum(N1, N2)` from section 3.3. That is the whole multiplicative case; it is not a separate rule.

Converted to radix-two widths, condition 2 is `ceil(log2(n-1))` extra digits above the operand width,
which is the form the consolidation gives at `26:157-158`. Its other statement, `acc >= product_width
+ ceil(log2 n)` at `26:269`, is one digit wider, so both are safe and the design carries two spellings
of one condition. The value-coordinate statement subsumes both, and I would ship it as the definition
with the width formulas as its radix-two corollary rather than the reverse.

### 4.3 Interior safety upgrades all three relations at once, and this is a theorem

Probe 2 measures it; the proof is three lines and worth stating because it explains the measurement
rather than merely reporting it.

Under interior safety, every proper subtree of any grouping computes an exact value that lies in
`V(M)`, so no quantiser fires anywhere in the interior. The root therefore receives the exact total,
which is grouping-independent because exact addition on the rationals is associative. One quantisation
fires, at the store, on the same argument under every grouping. Therefore:

- the **value** is grouping-independent (weak equation),
- the **definedness** is grouping-independent (Kleene equation),
- the **quantisation-event multiset** is grouping-independent, being exactly one event (graded
  equality).

All three at once, from one condition. This is the unification the consolidation was reaching for when
it said that at or above the threshold "the recovery map's own properties become irrelevant and only
the accumulator matters" (`26:161-163`). The sharper statement is that the recovery map's properties
become irrelevant **because it is applied once to a grouping-independent argument**, and a function
applied once to a fixed argument has no structure left to depend on.

### 4.4 The bound is sufficient and not necessary, measured

Probe 2's third row is an accumulator strictly narrower than the closed form that still shows zero
definedness splits. The reason is that the destination numeral's own range prunes the inputs that
could have produced one: an input whose interior sum escapes a slightly-too-narrow accumulator has a
total that would not have fitted `N` anyway, so both groupings refuse and there is no split.

I would still ship the closed form, and say plainly that it is conservative. It is a bound in the
arity and the two numerals alone, it requires no reasoning about which inputs can occur, and the
necessary condition depends on the destination range in a way that makes a combinator's correctness
argument depend on facts about its inputs. That is the same "safe direction" reasoning file 31 applied
to the product lattice (`31:226-228`), and it should be stated for the same reason: a reader who
thinks a bound is tight will try to shave it.

This is also where my own arithmetic was wrong. My first draft had the bound at `n` rather than `n-1`
and predicted a split at an accumulator that shows none; the const assertion refused, and the
consolidation's own droplist had `K = n - 1` all along (`26:717`). The compiler caught it, not the
citation.

## 5. The atoms: what is independently checkable, and the classification that needs the same treatment

D51 already rules that law markers are derived by blanket impls over the composition rather than
declared per type, and that a derived property cannot lie so it is a plain safe impl rather than D16's
`unsafe impl`. The consolidation independently reached the atomic-facts shape and named six candidates
(`26:189-201`). Nothing below reopens either. What it adds is the atom list stated with each atom's
relation and key, and one finding: the recovery-map classification has the same declared-versus-derived
problem the ladder had, and nobody has applied the fix to it.

### 5.1 The recovery map's three class names do not cover the design's own five maps

The consolidation classifies a recovery map as a homomorphism, a partial identity, or a retraction
(`26:76-82`). Probe 4 (`33_probes/probe_4_four_atoms_beat_three_class_names.rs`) computes four atomic
properties for each of the design's recovery maps over an exact domain:

| Map | total | fixes | monotone | homomorphic | placed by the three names |
|---|---|---|---|---|---|
| `ReduceModulo` | yes | yes | no | yes | homomorphism |
| clamp | yes | yes | yes | no | retraction |
| `Refuse` | no | yes | yes | no | partial identity |
| `SubstituteZero` | yes | yes | no | no | **nowhere** |
| dither, confined | yes | no | yes | no | **nowhere** |

Three findings, all compiled:

**The four atoms separate all five maps**, with five distinct signatures. The three names place three.

**Two maps have no class.** `SubstituteZero` is total and fixing, so it is not a partial identity, and
it is not order preserving, so it is not a retraction, and it is not a homomorphism. It is one of the
design's own four `Resolution` instances. The confined dithered entry point is total and monotone but
does not fix the destination numeral pointwise, so it is none of the three either; and it arrives from
the **identity** half (`31:386-388`), which is the concrete sense in which settling identity sharpened
this problem rather than dissolving it. The algebra half acquired a new recovery map while its
classification scheme was still three names wide.

**The three names overlap and are not a function of the map.** Clamping is a retraction and could in
principle also have been a homomorphism; nothing in the scheme prevents a map satisfying two. So "the
class of `phi`" is not well defined even where a class exists.

The fix is the one the design already chose one level up, and applying it costs nothing new:
**classify by the atomic signature, and let the three class names be derived conjunctions.** A
homomorphism is `H`. A partial identity is `F and not T`. A retraction is `T and F and M`. Each
implied law keeps its three-line proof; the proofs are unchanged, they are now stated over atoms that
are individually checkable, and a map outside all three named regions still has a signature and still
gets whatever laws its atoms imply, instead of falling off the scheme.

This also matters for the coherence ceiling the consolidation records (`26:191-195`): three mutually
non-specific impls are refused by Rust's coherence checker, and marker-trait conjunctions are not. The
recovery-map classification would hit that same ceiling the moment a fourth class name were added, and
the two unclassified maps above are exactly the pressure to add one.

### 5.2 The atom list

Each row states the relation the atom is asserted under and its key. `Op` is an operation marker in
the sense D13 already ratifies for `Monoid<Add>` and `Monoid<Mul>`, so the operation being a trait
parameter is a restatement rather than a new call.

| Atom | Claim | Relation | Key beyond `(Op, numerals)` |
|---|---|---|---|
| `ValueAssociative<Op>` | regroupings that return, return the same value | weak equation | quantisation, growth, accumulator, arity |
| `DefinednessInvariant<Op>` | regroupings agree on whether they return | equality of definedness | quantisation, growth, accumulator, arity |
| `EventInvariant<Op>` | regroupings perform the same quantisation events | multiset equality | quantisation, growth, accumulator, arity |
| `Commutative<Op>` | operand order does not change the result | law equality | quantisation, growth |
| `HasIdentity<Op>` | a value acts neutrally on both sides | law equality | quantisation, growth |
| `Idempotent<Op>` | applying to a value twice with itself is the value | law equality | quantisation, growth |
| `Monotone<Op>` | each partial application is order preserving | the value order | quantisation only (section 6.1) |
| `MonotoneQuantiser` | the quantiser as a unary map is order preserving | the value order | `Direction` and both resolutions |
| `LatticeClosed<Op>` | the exact result stays in the operand lattice | set membership | the numerals only |
| `Total<Op>` | the operation refuses nowhere on its domain | definedness | resolutions only |

Two notes on the list.

`Monotone<Op>` and `MonotoneQuantiser` are the two differently-scoped uses of the word the
consolidation says should stay separate traits because they are different arities of a different
concept (`26:199-201`). They stay separate here. But they are not independent: section 6.1 shows the
binary one is derivable from the unary one plus the exactness of the underlying operation, which is a
relationship worth recording so a future reader does not check them twice.

`Distributes` is **not** on the list, for the reason section 6.1 gives.

## 6. The ladder question: it is not a ladder, and the reason is measurable

The consolidation asks whether the algebra is a ladder at all, given the algorithms need distributivity
over a lattice operation rather than associativity, and the preset that satisfies one fails the other
(`26:126-137`). D47 rules that the ladder goes as deep as the theory does and is not truncated to where
other ecosystems stopped. Nothing below truncates anything. It says what the rungs actually are.

### 6.1 On a chain, `Distributes` is not an atom: it is `Monotone`

Probe 1 (`33_probes/probe_1_monotone_is_distributivity.rs`) checks, over a sixteen-value signed model
and for three binary operations including a deliberately non-monotone control, that

**monotone in each argument holds if and only if distributivity over `max` holds, if and only if
distributivity over `min` holds.**

Checked as an equivalence, in both directions, with a witnessed failing side so it is not two
properties that happen to agree everywhere tested. The proof is two lines each way: if `a + -` is
monotone and `b <= c` then `a + max(b,c) = a + c = max(a+b, a+c)`; conversely if it distributes and
`b <= c` then `a + c = a + max(b,c) = max(a+b, a+c) >= a + b`.

So the design does not need a `Distributes<Op, Max>` atom. It needs `Monotone<Op>`, and distributivity
over the chain's lattice operations is a **theorem**, recorded once with its hypothesis rather than
checked per composition. The hypothesis is that the value order is total, which is a fact about the
numeral: it holds outright when `Specials = None`, and under the total order of section 2.2 otherwise.

Distributivity over an operation that is not a lattice operation of the value order, multiplication
over addition being the case that matters, is a genuinely separate fact and stays an atom.

This also connects the ladder to D16. Monotonicity of a whole composition is computable from its axis
instances: every `Direction` is monotone (probe 5), the clamping resolutions are monotone, and
`ReduceModulo` and `SubstituteZero` are not (probe 4's `M` column, and probe 5's negative control). So
for a composition arvo builds, `Monotone` is a **derived** property and a plain safe impl under D51.
For a foreign numeral reaching an arvo algorithm through the Stage G boundary (`26:460-471`),
monotonicity can only be **asserted**, and D16 is explicit that `MONOTONE = true` asserted is an
`unsafe impl` carrying a stated contract. The design gets both, from the same trait, with the risk
class visible at the declaration exactly as D16 requires. That reconciliation is reasoned, not
compiled.

### 6.2 No preset is a dioid, and saturating addition misses twice

Probe 1 also checks the named structure directly. Over `(max, op)` with the bottom element as `max`'s
identity (itself checked, so the annihilation question is asked about the right element):

**Wrapping addition** is associative and does not distribute. Not a dioid.

**Saturating addition** distributes and is not associative, and separately **fails the annihilation
axiom**: the bottom element does not annihilate, with the witness `sat(-8, 3) = -5`. Two independent
failures, so a reader who repaired associativity would still not have a dioid.

**`Precise` addition** is partial, so it is not a total semiring at all.

The consequence is worth stating plainly because it is the answer to "which algebra is this": **the
named structure that the textbook account of `longest_path` and min-plus DP reaches for is satisfied by
no composition the design currently ships, and the algorithms work anyway, because they use strictly
less than the named structure.** They use monotonicity of the weight operation. That is one atom, it
holds for the two presets under which those algorithms are correct, and it fails for the one under
which they are not, which is exactly the inversion the consolidation measured (`26:126-137`) and the
reason its droplist refuses an associativity gate (`26:699-703`).

Under D47 this is not an argument for removing the `Dioid` rung. It is an argument for the rung being
**derived** rather than declared, which D51 already requires: a derived structure over an empty set of
compositions is a true and useful statement, whereas a declared one would be an assertion nobody can
satisfy. A consumer asking whether their composition is a dioid gets a correct "no" and the atoms tell
them which axiom failed.

### 6.3 What would make the rung non-empty, and it comes from the identity half

A tropical semiring wants an additive identity that annihilates the multiplicative operation. On a
finite chain with no bottom-absorbing element, no clamping arithmetic can provide one, which is what
probe 1's annihilation failure is. **A numeral carrying `Specials` can.** Negative infinity is a
genuine identity for `max` and a genuine annihilator for addition, which is precisely why the textbook
max-plus algebra is stated over the extended reals rather than over a bounded interval.

So the honest statement of where the design stands: the ordered algebraic structure the graph and DP
algorithms are written against is available exactly to compositions whose numeral has the relevant
special, and every fixed-point composition shipped today has `Specials = None` and structurally cannot
have it. This is a real requirement on the identity contract discovered from the algebra side, and I
would scope it the way file 31 scoped block floating point (`31:314-317`): a requirement the contract
should not build itself shut against, not a design built now. The known hole in it is standard and
should be recorded with it, that the extended reals' `-inf + inf` is undefined, which is a refusal
cause the vocabulary already has a home for.

### 6.4 The unifying statement: the exact algebra is the good one

Three results in this file point at one sentence, and the design does not say it anywhere.

The tension between associativity and distributivity does not exist in the exact algebra. Exact
addition on the rationals is associative, commutative, has an identity, and is monotone, so it
distributes over `max` and `min` too. Exact multiplication is associative and commutative and
distributes over exact addition. Every failure of every law in this design is created by a quantiser,
and probe 5's `mul_full` column, probe 2's interior-safety rows, and the multiplicative half's whole
relocation argument (`26:226-243`) are three views of the same mechanism: **remove the quantiser from
the interior and the laws come back, unchanged, for free.**

So the design's real algebraic story is not a ladder of structures a composition climbs. It is one
exact algebra, plus a stated account of what each quantiser costs, plus the conditions under which a
combinator keeps the interior exact. The rungs are still worth having under D47, because a consumer
needs to know what their composition satisfies. But the rungs are the answer to "what did the
quantiser cost me", not the primary object.

There is a competing reading and I hold it honestly. If the rungs are secondary, one could argue the
design should present the exact algebra as the surface and treat quantised operations as derived,
which is close to what the `mul_full`/`quantize` split already does for multiplication. The reason I
do not push that further is that addition's case is genuinely different: a consumer adding two values
of one numeral expects a value of that numeral, and making the exact form the primary surface for
addition would change the ergonomics of the most common operation in the library to buy a symmetry.
What distinguishes the readings is whether anyone can name a consumer whose addition wants the widened
form by default; I cannot, and the MAC case (`26:263-276`) is the one place it is already true and is
already handled by the accumulator.

## 7. The laws, stated

Reasoned, not compiled: this section assembles sections 1 through 6 into the form the next
consolidation could take. Every claim in it traces to a probe or to a cited call, and where it does
not, it says so.

### 7.1 The frame

A **law** is a claim that two terms built from an operation stand in a stated relation, over the value
set of a numeral, quantified over a stated domain, keyed on every parameter its proof used. It is
expressed as a `const fn` whose parameters are its key (`26:174-186`), derived by blanket impl over the
composition rather than declared per type (D51), safe when derived and `unsafe impl` when asserted
(D16).

**Law equality** is equality of values, never of data. Two results are law-equal when they are the same
point of the numeral, or the same special. This is the equality induced by the composition's total
order, and it is reflexive at NaN because a special is a value while its payload is a datum. No law may
read a datum (`31:361-363`), which is what makes every law `Lowering`-blind and `Encoding`-blind by
construction rather than by mechanism.

**Three relations**, ordered, each an independent fact:

1. the **weak equation**, both sides equal where both are defined, which is soundness of a regrouping;
2. **definedness invariance**, which with the weak equation is the **Kleene equation**, and is
   completeness of a regrouping;
3. **event invariance**, the same multiset of quantisation events, which with the Kleene equation is
   **graded equality** and is what an error bound needs.

A combinator states which relation it requires. Code that never regroups requires none, states none,
and refuses nothing (`26:132-136`).

### 7.2 The quantiser, as the laws see it

`quantize = resolve . classify . round`. The `round` stage maps the exact result onto the unbounded
grid by `Direction`; the `resolve` stage maps the rounded result into the numeral by the resolutions
(`31:378-384`).

Every `Direction` instance is monotone and idempotent, and its fixed points are exactly the unbounded
grid, so the round stage never costs a composition its monotonicity (probe 5). All law-relevant
variation in monotonicity comes from the resolve stage, and specifically from `ReduceModulo` and
`SubstituteZero`, which are the two resolutions that are not directions (probe 4).

A recovery map is characterised by four independently derived atoms, **total**, **fixes**,
**monotone**, **homomorphic**, the last of which is keyed on the operation as well as the map. The
three named classes are derived conjunctions: homomorphism is `H`, partial identity is `F and not T`,
retraction is `T and F and M`. The design's five recovery maps have five distinct signatures, and two
of them lie outside all three named classes (probe 4).

### 7.3 Lattice closure, and when `Direction` is in the key

For a numeral with adjustment `q` and bias `b`:

**Additive lattice closure** holds exactly when `b/q` is an integer. `Bias = Zero` is the special case
the shipped `AddClosed` gate uses.

**Narrowed-multiplicative lattice closure** holds exactly when `q` and `b` are both integers and `q`
divides `b^2 - b`. No fixed-point numeral with a fractional digit satisfies the first conjunct.

Both are checked against direct exhaustive computation in both directions (probe 5).

**`Direction` is in a law's key exactly when the exact result can leave the operand lattice**,
equivalently, exactly when a `quantize` call sits between the exact operation and the result. For
addition on a closed numeral and for `mul_full`, all four directions produce identical results at every
operand pair; for narrowed multiplication they disagree at half of them (probe 5).

### 7.4 The product numeral

`mul_full` is a family of maps `N1 x N2 -> mulnum(N1, N2)`, not an operation on one set. Its laws are
statements about the numeral-level map first.

`mulnum(N1, N2)` has `bias = B1*B2` and `adjustment = gcd(A1*A2, A1*B2, A2*B1)` (`31:399-400`). At `n`
factors, the bias is the all-bias monomial and the adjustment is the gcd of every monomial carrying at
least one adjustment. The map is commutative and associative, which is why `mul_full`'s associativity
is statable at all, and it holds because the monomial set is symmetric under permutation of the factors
(probe 3). With every bias zero it collapses to the product of adjustments, which is the shipped
exact-product width rule at every arity. The lattice contains the product set and is not claimed to be
the finest one.

### 7.5 The fold, and interior safety

A fold of arity `n` over destination numeral `N` with accumulator numeral `M` is **interior-safe** when
`V(M)`'s lattice refines `V(N)`'s and `(n-1) * [min V(N), max V(N)]` is contained in
`[min V(M), max V(M)]`. The two conditions are exactly "the round stage is the identity in the
interior" and "the resolve stage is".

Interior safety implies the weak equation, definedness invariance and event invariance together,
because exactly one quantisation fires, at the root, on a grouping-independent argument. It is
sufficient and not necessary; the design states the sufficient form because it is a bound in the arity
and the two numerals alone (probe 2).

For a multiply-accumulate, apply the same two conditions with `N` replaced by `mulnum(N1, N2)`. In
radix two, condition two is `ceil(log2(n-1))` digits above the operand width. The accumulator is a
parameter of the combinator, its sufficiency is a compile-time check and never a derived type
(`26:278-286`, and the droplist at `26:726-729`), and both the accumulator and the arity are in the key
of every fold law.

### 7.6 The atoms and the derived structures

The atoms are those of section 5.2. `Distributes` over the value order's lattice operations is not an
atom: it is `Monotone` plus totality of the value order, and the derivation is recorded once (probe 1).
Distributivity over a non-lattice operation stays an atom.

Named structures are derived blanket impls over conjunctions of atoms, per D51. A structure whose
conjunction is empty over every composition the design can express is still declared, still derived,
and reports honestly that no composition satisfies it; under D47 the ladder goes as deep as the theory
does. `Dioid` over `(max, +)` is currently such a structure: wrapping addition fails distributivity,
saturating addition fails associativity and annihilation both, and `Precise` addition is partial (probe
1). A numeral carrying the appropriate special would make the rung non-empty, which is a requirement on
the identity contract rather than a design built now.

### 7.7 What no law may do

No law may read a datum, an `Encoding` member, or any `Lowering` member. No law may be stated under
`PartialEq` on a specials-carrying numeral. No law may omit the operation, the numerals, the
quantisation, the growth, or, for a fold, the accumulator and the arity. No combinator may require a
relation it does not state, and no operation may state a relation it does not need.

## 8. What this file does not decide

Three things, deliberately.

**D46 stands and I am not picking a mechanism.** D46 files the conditional-law finding (a law holding
under wrapping and failing under saturating) as a research question rather than a design question,
because choosing a mechanism accepts an unchecked premise, and names three things to establish first,
one of which is whether arvo's axis decomposition is itself split correctly. Everything above states
the **content** of the facts and the shape of their keys. It does not choose how a law that varies with
a `Policy` instance is dispatched. Section 6.1's finding is arguably an input to D46's third question,
since "more axes than the two overflow instances currently cover" is close to what the atom list is,
and I flag it as an input rather than an answer.

**Whether `Associative` names the fused fact or the weak one is a naming call.** Section 1 argues all
three relations should be named and that the fused one should carry the unqualified name, because that
is what a reader assumes. That is taste dressed as reasoning, and I label it as taste. What is not
taste is that three facts exist and are independently checkable.

**Whether `Precise` ships a combinator surface restricted to the Kleene-requiring form** is a question
about what `Precise` is for, which the consolidation holds open (`26:608-617`) and file 31 declined to
prejudge for the parallel dither question. I decline it too.

## 9. What this closes, what it sharpens, and what remains open

Closed, with a compiled check behind each:

The relation question. Three relations, ordered, standard vocabulary, and `Precise` reads as weakly
associative below the accumulator threshold and fully associative above it.

The accumulator question. A combinator parameter with a derived sufficiency side condition, in the key,
not an axis, with the threshold restated in radix-free value coordinates and shown to imply all three
relations at once.

Whether `Distributes` is an atom. It is not, on a chain; it is `Monotone`.

Whether the algebra is a ladder. It is not primarily; the exact algebra is the object and the rungs
record what a quantiser cost. The rungs still go in per D47.

Whether `mul_full`'s associativity is even statable. It is, because the product-numeral map is itself
associative and commutative, with an n-ary closed form.

Sharpened rather than dissolved:

The recovery-map classification acquired a fifth map from the identity half and its three class names
now cover three of five. The fix is the one the design already chose for the ladder.

The `(phi, Op)` key dependence, which the consolidation established by two independent measurements, is
now a predicate over the numerals: `Direction` is in the key exactly when the exact result leaves the
operand lattice, and that is one line per operation rather than a measurement per pair.

Open, and I am adding five items where I closed more:

1. **Law equality as the total order is reasoned, not compiled.** Section 2.2 argues that `TotalOrd`
   is already the right relation and needs no new trait. Writing an associativity check over a
   specials-carrying model under both `PartialEq` and the total order, and showing the first is not
   reflexive, is the next concrete thing to check.
2. **Event invariance has no measurement.** Sections 1.2 and 4.3 assert it and derive it from interior
   safety, but probe 2 counts values and definedness only. Counting quantisation events per grouping is
   a small extension to the same probe.
3. **The atom set is checked for addition and multiplication and not for division**, which the
   consolidation already flags with a prediction that division has no finite accumulator solution at
   all (`26:676-681`). Nothing above tests it, and if that prediction holds the two-case working
   assumption is wrong.
4. **The `Specials`-carrying dioid is scoped as a requirement, not designed.** Section 6.3 states what
   would make the rung non-empty and does not build it, including the `-inf + inf` hole.
5. **Compile-time cost of the atom ladder is unmeasured**, exactly as `26:668-674` and `32:352-356`
   both already say. Nothing in this file changes that, and the atom list is longer than the
   consolidation's six, which makes it worse rather than better. The blanket-impl overlap question D51
   attaches to its own decision as a sketch obligation is the place that lands.

## 10. Standing

Nothing here overturns a D-numbered call, op's seventh checkpoint, or anything files 31 and 32 settled.
Section 5 supports D51's derived-not-declared ruling by applying it to a place the design had not,
section 6.1 supplies the D16 reconciliation for `Monotone` in both risk classes, and section 6.2
explicitly declines to truncate a rung that D47 protects. Section 8 records where I stop, and D46 is
the reason for two of the three stops.

Two claims of mine were refuted by the compiler during this dispatch, and both refutations are in the
probe headers rather than only in `OUTCOMES.md`, because the next reader of a probe should see what it
caught before they see what it confirmed.
