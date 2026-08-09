# 37. Which relation

**Member:** Dominic Orchard. I wrote file 17, on whether the design's axes are grades, twenty files
ago; the material has moved a long way and I carry none of that file's conclusions forward unexamined.
The habit of mind this dispatch wants is the one my field is: when a system is stating facts under
several relations, the relations are almost never the primitive object. What is indexed, and what
algebra the index carries, is the primitive object, and the relations are what you get by choosing how
much of the index to look at. The specific failure my field knows best is the opposite of the one this
review has been catching: not a claim that is too weak to be useful, but an index rich enough to state
everything and therefore constraining nothing. Section 5 is where that risk lives and I take it head
on rather than in a caveat.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0 failed, 9
ignored, matching the counts files 31 through 36 each report from their own runs, so nothing regressed
under this dispatch. I re-ran the negative greps rather than trusting the citation chain:
`grep -rn "Monotone\|Magma\|AddAssoc\|Distributes\|Associative" crates/ --include="*.rs"` returns zero
lines, and no `arvo-algebra-contracts` directory exists, so every algebraic law in this review is at
design stage and nothing in the suite could be tautological about one. I read the body of the one
shipped test that bears on the surface I touch, `crates/arvo/tests/identity_laws.rs`: its module doc
states the full-matrix discipline and the file keeps it, walking both signednesses, all four
strategies and integer and fractional splits from zero bits upward, with the case that cannot be
written at all pinned separately as a compile-fail under `crates/arvo/tests/ui/`. Nothing tautological
in it, and the shape of it is the shape this file's own argument depends on, which is why I read it
rather than counting it. Canon gate: `26_consolidation_two.md`, `30b_op_checkpoint_seven.md` and
`34b_op_checkpoint_eight.md` govern, all read in full. The question I was sent to settle is one the
consolidation itself holds open and explicitly records as not a call any member has made
(`26:608-617`); nothing below overturns a D-numbered call or either checkpoint, and where I contradict
a panel file I say so in place.

**What I read:** `26_consolidation_two.md` in full. `30b` and `34b` in full.
`33_lamport_the_laws_restated.md` in full. `34_giesen_the_three_halves_assembled.md` in full.
`35_dolan_does_widening_collapse.md` and `36_kiselyov_the_normal_form_and_its_price.md` in full.
`34_probes/OUTCOMES.md`. The directory listed once, 36 numbered files plus probe directories. I did
not reread my own file 17, and nothing below assumes it; where I use the graded reading I use the
consolidation's statement of it (`26:205-221`) rather than my own.

**What I compiled or measured, separated from what I reasoned.** Seven artifacts in `37_probes/`, each
with a row in `37_probes/OUTCOMES.md`, plus a compile-time sweep in `37_probes/price/` with its
generator, runner and `results.csv`. All against the workspace pin,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, confirmed with `rustc --version` from inside the repo,
file 36's note about resolving to stable 1.94 from outside it observed. Three of the seven are
committed refusing, on purpose. Sections 1 through 5 are compiled or measured except where marked;
sections 6 and 7 are reasoning built on those results and section 7 is written to be taken close to
verbatim. Two of my own claims were killed by the compiler mid-dispatch and both are kept in the probe
headers rather than only in `OUTCOMES.md`: I predicted wrapping addition was graded-associative and
the model said otherwise before I had written a line of the report, and my first mechanism had a
licence check that refused exactly the case the mechanism existed to handle.

## 0. The verdict, stated first

**The design does not choose a relation, because there is nothing to choose. Every law computes its
own.**

The three relations the review has been holding as alternatives are one relation with one parameter,
and the parameter is a quotient of the grade the design already has. Compiled, over nine such
quotients and nine compositions: the set of quotients under which a law holds is downward closed and
closed under join, so **every law has a unique finest quotient under which it holds**, and that
quotient is the law's content. There is no fork. The appearance of a fork came from naming three
points of a lattice and then asking which of the three is the design's relation, which is the same
question as asking which of `Hot`, `Warm` and `Cold` is the design's strategy.

The parameter's domain is not a chain, so "ladder" is the wrong shape and the rung count is the wrong
question. Four distinct finest quotients are realised by compositions the design ships or can spell,
and two of them are incomparable: signed wrapping preserves values and definedness while losing
quantisation events, `Precise` preserves values and events while losing definedness, and neither
implies the other. **The point the three-name ladder cannot express is exactly `Precise`'s**, which is
why the open question about how `Precise` reads has resisted three files: it is neither "weakly
associative" nor "not associative", it is associative with its refusals unpreserved, and the
vocabulary had no name for that.

Both readings the consolidation holds are therefore right, and they were never in competition.
`Precise` does have a real statable property signed clamping lacks (`26:611-613`), and a fold whose
definedness depends on grouping is genuinely unusable if that fact can be forgotten (`26:613-616`).
Section 5 is the mechanism that gives the first without permitting the second: a regrouping publishes
what its law fails to preserve in its own result grade, so the second reading's danger is a type error
rather than a warning. Compiled, including the type error.

Against the state I found it in, four corrections and two additions. File 34's reification lemma is
true of one reifier and false in general, and the hypothesis it needs is about the reifying element
rather than about the relation, which removes the only argument currently tilting the open question
(probe 2). Wrapping addition is not graded-associative, which nobody had measured and which I
predicted wrongly (probe 3). Event invariance, a standing unmeasured item since `33:787-789`, is
measured (probes 1 and 3). The classical existence equation is derived rather than named, and the
refinement order the consolidation lists as a candidate (`26:609`) belongs to a different fact
entirely, which file 34 had already isolated without naming it as an order (section 6). And the
mechanism costs 1.48x less compile time and 2.04x less metadata than the shape file 33 proposes, while
expressing nine points where that shape expresses eight of which five are junk (section 5, measured).

## 1. The relation is computed, not chosen

### 1.1 What the parameter is, and why it is a quotient rather than a subset

The design already has the object the relations are quotients of. The consolidation's graded reading
states it: effect-shaped facts accumulate along a term as a free commutative monoid over refusal
causes and quantisation events (`26:205-207`). Call that monoid the grade. A term's meaning is a pair,
the grade it accumulated and the value it delivered, with the value absent exactly when the grade
carries a refusal cause.

A **view** is a monoid homomorphism out of the grade. The relation is one relation:

> Two terms are equal under view `v` when `v` sends their grades to the same thing, and their values
> agree wherever both are present.

The three names in the literature are three views, and the identification is mechanical:

- The **weak equation** is the trivial view. Every grade goes to the unit, so the grade clause is
  vacuous and only the value clause survives, which is exactly "both defined implies equal".
- The **Kleene equation** is the view that collapses the cause submonoid to a two-element idempotent
  monoid and forgets events. Grade equality then says exactly that the two terms agree on whether they
  refused, which with the value clause is exactly the Kleene equation.
- **Graded equality** is the identity view. Nothing is collapsed, so grade equality is agreement on
  both the refusal causes and the quantisation events, which with the value clause is what file 33
  calls event invariance plus the Kleene equation.

My first version of this had the parameter as a set of grade generators the consumer tolerates, and it
is wrong in a way worth recording because it is the obvious first guess. Under a subset domain, the
family of holding relations is not closed under meet, so a law can have several incomparable minimal
answers and its content is not one object. The fix is that Kleene equality does not **drop** the cause
generators, it **collapses** their multiplicities to a boolean, and a collapse is a quotient rather
than a projection. Once the domain is quotients, the family closes under pullback, because two views
under which a law holds have a common refinement (send a grade to the pair of its two images) and the
relation at the pullback is precisely the conjunction of the two. That is a three-line argument and I
did not want it to be only an argument, so probe 1 asserts the closure directly rather than assuming
it.

### 1.2 The two closures, compiled

Probe 1 (`37_probes/probe_1_the_ladder_is_a_view_lattice.rs`) models a signed three-bit numeral, a
resolution per range end exactly as `Quantisation` gives (`26:44-48`), and a four-element fold over all
five groupings, all pairs, and all 4096 inputs, at nine views: three detail levels (Ignore, Presence,
Exact) for each of the two generator classes. Nine compositions, 65 seconds of const evaluation.

**Downward closure.** If a law holds while looking at more of the grade, it holds while looking at
less. Asserted at every composition. This is the fact that made a ladder look plausible.

**Join closure.** If a law holds at two views it holds at their pointwise join. Asserted at every
composition. This is the stronger fact and it is the one that matters, because it is what makes the
finest holding view unique. Without it, "the law's content" would not be one object and the design
would be back to keeping several booleans in sync.

Together: **every law has a unique finest view, and that view is what the law says.** A design that
names three relations is naming three of the nine points and leaving the other six unnameable; a
design that reports the finest view names all of them with one mechanism.

### 1.3 What the weak equation is, exactly, and the one thing to be careful of

The weak equation is the bottom of the lattice, and it is the only one of the named three that is not
an equivalence relation on terms. It is reflexive and symmetric and it is **not transitive**: with `b`
undefined, `a =w b` and `b =w c` both hold vacuously and nothing forces `a = c`. That is standard in
partial algebra and it is not a defect, but it has a consequence the design has to be deliberate
about, because one reading of what a law attaches to is an edge of a rewrite system (`26:625-627`) and
a non-transitive relation cannot be a rewrite system's edge relation: two regroupings each licensed by
a weak equation do not compose into one.

File 33 got this right and I want to say why it is load-bearing rather than incidental. Its statement
quantifies over the whole grouping class, "regroupings that return, return the same value"
(`33:483`), not over a pair of terms. The class-level statement is exactly "the pairwise weak equation
holds between every pair", and among the defined groupings that is an equivalence relation. So the
class quantification is what makes the weak rung composable, and a reader who reads
`ValueAssociative` as a binary relation and chains it has an unsound chain. **The quantifier slot is
doing work here that the relation slot cannot do**, which is the sharpest vindication of file 33's
five-slot frame that this file found: two of the slots interact, and the interaction is invisible if
you fill them one at a time.

## 2. What the model says

### 2.1 Four points, and two of them incomparable

The finest views, measured (probe 1, four-element fold, exhaustive):

| Composition | Finest view | In the old vocabulary |
|---|---|---|
| `Precise`, interior-safe accumulator | (Exact, Exact) | graded, and everything below it |
| `Hot`, unsigned wrapping | (Exact, Exact) | graded (probe 3) |
| `Hot`, signed wrapping | (Exact, Ignore) | Kleene, and no more |
| `Precise`, below interior safety | (Ignore, Exact) | has no name |
| Refuse at one end, reduce at the other | (Ignore, Ignore) | the weak equation, and no more |
| `Warm` / `Cold`, saturating | none | the law is false |
| `SubstituteZero` | none | the law is false |

Two of these are incomparable, and both are shipped presets rather than contrived compositions. Signed
`Hot` satisfies the Kleene equation and fails the view that keeps events. `Precise` below interior
safety is the exact reverse. Neither implies the other, so no linear order contains them, and the
consolidation's framing of the question as a choice among three ordered relations (`26:609`) could not
have been answered as asked.

**This is the answer to how `Precise` reads.** Its finest view is (Ignore, Exact): values agree
wherever both groupings return, quantisation events agree, and definedness does not. The
consolidation's first reading wanted a weaker law name for it and was right that one exists; its
second reading wanted the definedness failure not to be forgettable and was right that it must not be.
The reason the two readings looked incompatible is that the only weaker name available was the weak
equation, which discards the event agreement too, and accepting it as `Precise`'s law would have
thrown away a true fact in order to name a true fact. With the lattice there is no such trade:
`Precise`'s law is stated at its own point, which is strictly stronger than the weak equation and
incomparable with Kleene.

### 2.2 Event invariance, measured

`33:787-789` records event invariance as asserted and derived but never measured, and files 34 and 36
both re-flag it as still standing (`34:463-464`, `36:468`). It is measured here, as the event
component of every view in probe 1 and directly in probe 3, and the measurement is not what anyone
expected.

**Wrapping addition is not graded-associative on a signed numeral.** I predicted the opposite from a
counting argument, and the argument is sound: each addition of two numeral members produces an exact
sum less than twice the modulus, so at most one reduction fires and it moves the value by exactly the
modulus; the delivered result is the exact total less the modulus times the number of reductions, and
it is also the exact total modulo the modulus, so the number of reductions is the exact total divided
by the modulus, which is grouping-independent. Probe 3 asserts that identity exhaustively and it
holds.

The hypothesis it uses without naming is that the reduction is **one-directional**. On a signed
numeral both range ends reduce; a partial sum can leave through the top and a later one through the
bottom, the two reductions cancel in the value and both are counted in the grade. Witness (-4, -3, 3)
on a three-bit signed numeral: one grouping reduces twice and the other not at all, and they deliver
the same value. So the Kleene equation holds at that point and the graded one fails.

The consequence for the key is small and real. `Domain` is a `Numeral` member surviving both recent
removals (`35:298`, `36:378-382`) and is therefore already in every law's key transitively, the
operand numeral being a never-elided slot (`33:237`). What no file has said is that **a law's verdict
changes with it, and that only one component of the verdict does**: `ReduceModulo`'s value and
definedness components are `Domain`-independent and its event component is not. Different components
of one law read different parts of the key. That is an argument for computing the view per component,
which the mechanism in section 5 does for free, and against keying a whole law on the union of
everything any component reads, which is what a single fused verdict forces.

### 2.3 The evaluation strategy is an unstated slot, and it does not bite here

A refusing operand has a sibling. Under strict evaluation the sibling is evaluated and its
quantisation events accumulate; under the left-to-right short circuit a `?`-shaped implementation
gives, they do not. No file in this review states which the design means, and the design publishes the
grade under section 5's rule, so the grade has to be a function of the term.

Measured both ways. The **grade** differs: at `(1, 3, -4, -4)` under the balanced grouping, with
`Refuse` past the top and `ReduceModulo` past the bottom, the strict reading gives one cause and one
event and the short-circuit reading gives one cause and no event. The **verdicts** do not: every one of
the nine views has the same truth value under both strategies at every composition measured, asserted
in probe 1.

So this is an honest negative and I report it as one. The slot is unstated, it changes an object the
design publishes, and it changes no law in the model. It costs one sentence in the spec (section 7)
rather than a decision.

## 3. File 34's reification lemma, corrected

File 34 read the weak and Kleene distinction as unstable under a transformation the identity contract
supports, and concluded that the graded relation is the only reification-stable one and filed that as
input to the open question rather than an answer (`34:176-190`). That reading is currently the only
argument on the table tilting the question toward the graded rung, and it deserves more than the one
witness pair it was drawn from. Probe 2 tests it against two reifications, both of which the design
already has.

**Under an out-of-set absorbing special, file 34's finding is confirmed and is narrower than it
claimed.** Every view that keeps any cause information is preserved pointwise, Kleene and graded both,
and the weak equation is not. So graded is not the only stable one; Kleene is stable too, for a reason
that is easy to see once measured: when the reifying element is outside the value set, "this term
refused" remains observable in the value, so a definedness split becomes a value split of exactly the
same extent and the Kleene verdict is unchanged.

**Under `SubstituteZero`, no view is preserved, graded included.** All nine flip. `SubstituteZero` is
one of the design's own four `Resolution` instances (`26:44-48`), it delivers a refusal as an ordinary
member of the value set, and it does not absorb, so the continuation keeps computing. Witness at
`(3, 3, 1)` on a three-bit signed numeral: under `Refuse` both groupings refuse with one cause each,
so every view holds at that point; under `SubstituteZero` the left grouping delivers 1 and the right
delivers 3. The grades are **identical** on both sides. The graded relation had every piece of
information it could have had and still flipped, because what changed was the value the resolution
delivered rather than the grade it recorded.

So the corrected statement, and it is about the reifying element rather than about any relation:

> A reification preserves a view's verdict when the reifying element lies outside the numeral's value
> set and absorbs the operation. The first conjunct is what keeps "this term failed" observable in the
> value; the second is what stops the continuation from computing anything the refusing composition
> short-circuited away. Drop either and no view survives.

Two consequences. The first is that **reification stability is not a criterion for choosing the
relation**, because in general no relation has it, so the input file 34 filed to the open question is
withdrawn rather than weighed. The second is a reason the resolutions belong in the key that is
independent of the one already recorded at `33:240`: a law's verdict does not transport across a
change of resolution even when the grade is unchanged, so the resolutions are in the key for a
value-level reason as well as a grade-level one.

I want to be precise about what I am not saying. File 34's own use of the lemma, that an absorbing
special converts a Kleene failure into a weak failure, is correct and I reproduce it. What does not
survive is the generalisation to "the graded relation is the one that is stable", and with it the
suggestion that stability distinguishes the rungs.

## 4. What the two recent removals do to the answer

### 4.1 `Growth` leaving the key removes a slot from three atoms at once

File 35 removed `Growth` from `Policy` and from the key, on the sharper ground that it was a
relational fact sitting on a unary axis (`35:216-241`). File 33's atom table keys each of its three
fold atoms on "quantisation, growth, accumulator, arity" (`33:483-485`), so all three lose the same
slot together, which is worth noting only because three copies of one deletion is three chances to
delete it twice or once.

The residue is more interesting than the deletion. What `Growth` was carrying, per its own row, is
whether a quantiser sits between the exact operation and the result (`33:241`), and file 35 binds that
to the operation marker as an associated const (`35:204-214`). In the vocabulary of this file that
const has a name it did not have: **`Op::IS_EXACT` is the statement that the operation's grade monoid
is trivial.** An exact operation generates no causes and no events, so its grade is the unit, so every
view identifies every grade, so all nine collapse and the law is stated once with no parameter at all.

That is why the multiplicative half's laws are free (`26:236-238`) in a stronger sense than "no
quantiser is present": the parameter this whole file is about has a one-point domain there. And it is
a small argument for file 35's own preference between its two spellings (`35:396-403`): the
operation-name spelling is checkable by grep, and it is also the spelling under which the fact reads
as a statement about the algebra rather than about a lowering, which is the side of the design the
laws live on.

### 4.2 The normal form dissolves a candidate relation rather than adding one

File 36 made numeral encodings value-unique by construction, so two types denoting one numeral are one
type (`36:154-186`). For the relation question that is not a new constraint, it is the removal of a
would-be candidate. Before it, a law equating two terms of "the same numeral" needed a relation
answering whether two spellings denote one numeral, and file 34's probe 5b is that relation failing
with `expected Adj { num: 6, den: 12 }, found Adj { num: 1, den: 2 }` (`34:318-321`). After it, the
question is type identity, decided by rustc, and there is nothing for a law to state.

That leaves the design with three quotients that must not be confused, at three levels, each enforced
by a different mechanism, and the mechanism at each level is forced by how decidable the quotient is
there:

**Type level.** Two spellings of one numeral are one type. Decidable by the compiler, so enforced by
construction: the illegal spelling has no type (`36:166-174`).

**Datum level.** Two data carrying one value are law-equal. Decidable at runtime and cheaply, so
enforced by canonicalisation: compare after `Encoding::Canonical` (`34:270-277`).

**Grade level.** Two grades with the same image under the law's view are law-equal. Not decidable at
all, because which view a law survives is a fact about the law, so it is **computed** and reported,
which is section 5.

Naming those as one discipline at three levels is the compression, and the operational content is the
last column: nobody chose the three mechanisms, the decidability at each level chose them.

## 5. The mechanism, and the one thing that stops it being decoration

### 5.1 One const fn, and its return type is the whole change

The design already ratified that a derived fact is a `const fn` whose parameters are its key
(`26:174-186`). The mechanism here changes exactly one thing about that: **the return type is a
lattice element rather than a boolean.**

```rust
pub const fn add_assoc_view(
    top: Resolution, bot: Resolution, domain: Domain,
    arity: u32, headroom: u32,
) -> LawView
```

`LawView` is `Never` or `Finest(View)`, and a `View` is one detail level (Ignore, Presence, Exact) per
grade generator class. Nothing else is new. No axis is added, no trait family is introduced, no
unstable feature is reached for, and `generic_const_exprs` is not needed anywhere. Probe 4 compiles it
with every composition probe 1 measured, and the const fn's body IS probe 1's and probe 3's
measurements rather than a second statement of them.

Against file 33's three derived marker traits (`33:483-485`), three things.

**Three markers cannot name the presence level.** There are nine views, not eight, because a consumer
propagating an error bound needs event multiplicities while a consumer asking whether anything rounded
at all needs only presence, and those are different facts about one fold. Covering nine views with
markers takes five, whose conjunctions span thirty-two combinations of which nine mean anything, which
is the rich-index-that-constrains-nothing failure arriving in the markers' own form rather than in
mine.

**Three markers have three coherence surfaces and one invariant with no home.** Section 1.2's two
closures are relationships **among** the markers. Copies decorrelate, and this pair would decorrelate
silently, because a marker impl'd where it should not be looks exactly like one that should be. The
consolidation already records the coherence ceiling the per-`Resolution` shape hit (`26:191-195`); five
markers is more surface, not less.

**Three markers cannot state the law's content**, which is one object. A consumer asking how
associative a composition is gets three booleans and reconstructs the lattice element itself.

### 5.2 Priced, and the first comparison I ran was unfair to my own side

Compile-time sweep in `37_probes/price/`, `--emit=metadata` (type checking, trait selection and const
eval, no codegen, which is the honest shape because both mechanisms are entirely compile-time), counts
0 to 400 distinct compositions, min of three runs, baseline subtracted, linear across the whole range,
2 to 3 percent run-to-run noise.

| shape | ms per composition | metadata bytes per composition |
|---|---|---|
| five marker traits, impls asserted | 0.060 | 785 |
| one const fn returning the finest view | 0.130 | 907 |
| five marker traits, impls derived | 0.193 | 1854 |

The first row is the cheapest and it is the shape D51 forbids. A marker impl is a claim about a
composition and nothing in that row checks it; D51 rules that law markers are derived and that a
derived property cannot lie, which is why it is a plain safe impl rather than an `unsafe impl`
(`33:428-430`). Deriving the marker means computing the view, which is the second row, and then
carrying the impls on top, which is the third.

So the comparison that matters is the second row against the third: **1.48x cheaper in compile time
and 2.04x smaller in metadata**, expressing nine points where the marker shape expresses eight of
which five are junk. The const fn is not an alternative to the derivation. It **is** the derivation,
with the marker layer removed rather than added.

Calibration, because an absolute number alone means nothing: 0.130 ms per composition is two orders of
magnitude below file 36's 5.08 ms for the type-level gcd. The law mechanism is not where this design's
compile time goes. This prices the mechanism shape and is a neighbour rather than an answer to
`26:668-674`, which asks for the cost against a real consumer's composition set, exactly the
distinction file 36 drew about its own numbers (`36:470-471`).

### 5.3 The risk my field knows best, and what closes it

A view is coeffect-shaped. It is about what a consumer will put up with, and this review has already
measured what happens to a permission-shaped fact that carries no data: a corrupted grant compiles
clean with zero diagnostic, and both follow-up repairs failed too (`26:211`, and the droplist entry at
`26:730-742`). That is the same disease this mechanism could have caught. If a consumer declares which
view it needs, declaring a weak one is a waiver, and nothing checks a waiver. An index nobody can be
wrong about is an index that constrains nothing.

My first version had exactly that, and the compiler killed it before I could argue for it. It had the
consumer declare a required view, checked the law against the declaration, and also published a
deficit, and two of its own call sites died:

```
error[E0080]: evaluation panicked: this composition's fold law does not hold at
the required view: no regrouping of it is licensed at that detail
```

The two halves were fighting, because the licence check refuses exactly the case the transfer exists
to handle. Pulling on that found the real error, which is that "the consumer requires a view" is two
different things run together. Where the **weak** equation fails, the delivered values diverge and no
publication rescues anything, so that case is a hard refusal. Everywhere else the regrouping is always
sound and the only question is what it must say about itself, which is derived rather than requested.

**So no consumer declares anything.** The rule:

> A regrouping publishes, in its own result grade, exactly the grade generator classes its law fails
> to preserve. Tolerance is a transfer, never a waiver.

There is no consumer-supplied index left to be too rich, which is the cleanest available answer to the
risk and it arrived by removing a parameter rather than by adding a check. The consumer's contract
becomes the ordinary type of the result: a caller needing a fold whose definedness matches the
sequential one takes a `Folded<0>`, and handed the `Precise` regrouping below interior safety it gets

```
error[E0308]: mismatched types
   expected struct `Folded<0>`, found struct `Folded<1>`
```

with no bespoke machinery at all. That is the coeffect discharging into an effect, which is the
asymmetry my own file 17 identified as the single sentence worth putting in the spec (`26:209-212`),
used here to make itself unnecessary: the permission-shaped fact was turned into a data-shaped one, so
the type system checks it for free.

Three refusals are committed. `probe_4b` is the hard refusal for a composition with no law at any view
(saturating). `probe_4c` is a combinator understating its published grade, which the const assertion
refuses with the transfer rule's own sentence. `probe_4d` is the caller contract above.

One honest limit, and it is the shape file 35 already met from another direction. The published grade
is **declared and checked**, not computed, because computing it in return position is an expression
over a generic const parameter in type position and that is the wall the droplist records
(`26:719-724`), which file 36 re-verified from a fourth direction (`36:88-99`). Understating it is a
compile error; overstating it compiles and is merely pessimistic, which is the same safe direction
files 31, 33 and 34 all take on lattice containment.

## 6. The complete inventory of candidate relations

Six relations have appeared in this review under one heading or another. Where each lands, so that the
next consolidation does not carry a list of alternatives that are not alternatives.

**Type identity of numerals.** Not a relation a law is stated under. Settled by file 36's normal form
into type identity, decided by the compiler (section 4.2).

**Datum equality.** Forbidden to laws by the charter (`31:361-363`), and its plausible-looking
substitute, the equality induced by a 5.10-shaped total order, was found to read a datum (`34:247-277`).
Law equality is the canonical quotient. Every view in this file is built on it.

**The weak equation.** The bottom of the view lattice, the trivial view. Real, statable, not
transitive pairwise, and transitive as file 33 states it over the grouping class (section 1.3).

**The existence equation.** The fourth classical rung, and it is derived rather than named:
`t1 =e t2` is the conjunction of `Total<Op>`, which is already an atom in file 33's own list
(`33:492`), with the weak equation. Nothing needs to name it.

**The Kleene equation.** The view that collapses cause multiplicities and forgets events. One point of
the lattice, and not the design's answer, because there is no single answer.

**The refinement order**, which the consolidation lists as a candidate (`26:609`) and which nobody has
placed. It is not an equality and it is not a candidate for the law's relation at all. It is the
relation for a **different fact**, and file 34 isolated that fact without naming it as an order: at the
n-1 bound a fold is grouping-invariant but is not the function `quantize . exact_sum`, since it can
refuse where the destination would have clamped, and at the n bound it is (`34:199-219`). Those two
statements are `fold` below `quantize . exact_sum` in the definedness order, and `fold` equal to it.
So the refinement order is the relation of **specification conformance**, it is what file 34's total
safety asserts, and interior safety is the law's condition while total safety is the specification's.
Two conditions, two relations, and the pairing was the missing half of file 34's own finding.

That is the whole inventory: one relation with one parameter for the laws, one derived rung nobody
names, one order for conformance, and two quotients that belong to identity rather than to algebra.

## 7. The design, stated

Reasoned on the compiled results, written to be taken close to verbatim.

### 7.1 Law equality and the view

Law equality is the canonical quotient: two results are law-equal when canonicalisation sends their
data to the same datum, equivalently when they decode to the same value with each special as one
value-level class (`34:270-277`). No law reads a datum.

A term's meaning is a grade and a value, the grade being an element of the free commutative monoid
over the design's refusal causes and quantisation events, the value absent exactly when the grade
carries a cause. A **view** is a monoid homomorphism out of the grade. Two terms are equal **under a
view** when the view sends their grades to the same thing and their values are law-equal wherever both
are present.

The design names no single relation. Each law reports the **finest view under which it holds**, which
exists and is unique because the set of views under which a law holds is downward closed and closed
under join. The three relations in the literature are three points of that lattice: the weak equation
is the trivial view, the Kleene equation is the view collapsing cause multiplicities and forgetting
events, graded equality is the identity view. The existence equation is not a point; it is `Total<Op>`
conjoined with the weak equation, and is derived.

The lattice is not a chain. `Hot` on a signed numeral and `Precise` below interior safety have
incomparable finest views, and both are shipped presets.

### 7.2 What a law is

A law is a claim that the terms of one grouping class stand in the relation, under a stated view, over
the value set of a numeral, quantified over the whole class rather than pairwise, and keyed on every
parameter its proof used. It is expressed as a `const fn` whose parameters are its key and whose
return type is `Never` or the finest view. Derived by blanket construction over the composition rather
than declared per type (D51), safe when derived and `unsafe impl` when asserted (D16).

The key: the operation, whose marker carries whether its grade monoid is trivial (file 35's
`Op::IS_EXACT`); the operand numerals and, for a widening operation, the result numeral; the
`Quantisation` resolutions and, where a quantiser sits between the exact operation and the result, its
`Direction`; and, for a fold, the accumulator numeral and the arity. `Growth` is not in the key
(file 35). `Lowering` is not in the key and cannot be named from where laws live
(`31:361-363`, `34:421-433`).

The quantifier is the grouping class, not a pair of terms, and this is load-bearing rather than
stylistic: the weak equation is not transitive between two terms, so the class statement is strictly
stronger than the pairwise one and is the one a combinator that chains regroupings needs.

**The evaluation strategy is stated**, because a refusing operand's sibling contributes its
quantisation events under strict evaluation and does not under a left-to-right short circuit, and the
grade is published. Measured, the two readings give the same verdict at every view and every
composition in the model and different grades, so this costs a sentence rather than a decision, and
the sentence is owed because the grade is an object the design hands out.

### 7.3 The fold, and its two conditions under two relations

Interior safety (range factor `n-1`, plus span refinement) is the **law's** condition: no quantiser
fires in the interior, one fires at the root on a grouping-independent argument, and the finest view
is the top of the lattice, so every named relation holds at once. Total safety (range factor `n`) is
the **specification's** condition: the fold equals `quantize . exact_sum` and the accumulator is
unobservable. The two are related by the refinement order rather than by any view: below total safety
the fold is strictly less defined than its specification. A combinator states which it checked and the
law it derives is keyed accordingly (file 34's two contracts, `34:199-219`).

### 7.4 The transfer rule

A regrouping publishes, in its own result grade, exactly the grade generator classes its law fails to
preserve. Tolerance is a transfer, never a waiver. Where the weak equation itself fails the values
diverge and no publication makes the regrouping honest, so that case is refused outright rather than
published. No consumer declares which view it needs; the consumer's contract is the ordinary type of
the result, and a caller that cannot accept a published class does not typecheck.

The published grade is declared by the combinator and checked against the law, because computing it in
return position is an expression over a generic const parameter in type position and that is closed
(`26:719-724`, `36:88-99`). Understating it is a compile error. Overstating it compiles and is
pessimistic, the same safe direction the design takes on lattice containment everywhere else.

### 7.5 The views of the compositions the design ships

Stated as a table because a consumer will want it, and every row is measured rather than derived:

| Composition | Below interior safety | At interior safety |
|---|---|---|
| `Hot`, unsigned | graded, nothing published | graded |
| `Hot`, signed | events unpreserved, published | graded |
| `Warm` / `Cold` | no law at any view, regrouping refused | graded |
| `Precise` | definedness unpreserved, published | graded |

The right column is uniform, and it is measured for each resolution shape rather than extrapolated
from the refusing one, because a uniform column is exactly the kind of claim that is true of the row
someone checked. `Warm` is the striking entry: it goes from having no law at any view to having every
one, purely by widening the accumulator, with no axis changed. That is the design's strongest argument
for the accumulator being where the effort goes, since it is the one condition that makes the view
question disappear entirely.

## 8. What this file does not decide

**Which surface `Precise` ships** remains the what-is-`Precise`-for question the consolidation holds
(`26:608-617`) and file 33 declined (`33:749-751`). What I have settled is the mathematics under it,
which is that `Precise`'s law sits at a point the old vocabulary could not name and that both readings
of the fork were right about their own half. Whether the shipped combinator surface for `Precise`
offers only the definedness-faithful form, or offers the published-grade form and lets the caller's
type decide, is a question about what a `Precise` consumer expects, and section 5's mechanism supports
either. I decline it for the same reason files 31 and 33 declined their parallel questions.

**The `TotalOrd` level annotation** (`34:279-289`) is untouched and stands as file 34's fork.

**The evaluation strategy** is named as an owed sentence, not chosen. Strict and short-circuit give
the same verdicts in the model and different grades; which one the design means is a call about what a
refusing fold does, not a mathematical question, and it interacts with the fallibility ladder in
`notko` that nobody in this stretch has read.

**Multiplication and division** are untouched by this file's measurements. The view mechanism is
stated over one operation family, and the atom set's survival past addition is already standing
(`26:676-681`). Division's predicted lack of any finite accumulator solution would put it permanently
below interior safety, which under section 7.5 means it never reaches the top of the lattice, and that
is a prediction about a prediction and I have measured neither.

**Whether a view should be an `adt_const_params` const parameter for diagnostics.** Probe 4 packs it
into ordinary const parameters and needs no feature. A `View` struct as a const parameter would render
better in a type mismatch and costs an allowed feature; I did not measure the diagnostic difference
and file 36's own measurement of rendered diagnostic length (`36:354-358`) is the shape that question
wants.

## 9. Open, net

Closed by this file, each with an artifact: which relation the design states its laws under, and the
rung count, both answered by the finest-view result rather than by choosing (probes 1 and 4); event
invariance, standing unmeasured since `33:787-789` and re-flagged twice (probes 1 and 3); file 34's
reification lemma, corrected to a hypothesis about the reifying element (probe 2); the placement of the
refinement order, which had been listed as a candidate relation for four files (section 6); and the
mechanism's compile-time price against the shape file 33 proposes (section 5.2).

Opened by this file, one, and it is a sentence rather than a question: the design owes a stated
evaluation strategy for a refusing operand's sibling, because the grade is published and the two
readings give different grades. Measured to change no law's verdict in the model.

Standing from the predecessors, unchanged and untouched by me: the atom ladder's cost against a real
consumer's composition set (`26:668-674`), to which section 5.2's numbers are a neighbour; division
(`26:676-681`); richer canonicalisation branchlessness (`32:341-350`); D39's honest content; the
dither-versus-`Refuse` choice; the `TotalOrd` level annotation; the `Growth`-leaves-`Policy` reading
file 35 marks as reasoned rather than compiled; and `arvo-num-systems` and `notko-hlist` still unread
by anyone (`26:661-666`), which I did not read either and which bears on this file, since the view is a
type-level set-shaped object and the `notko` fallibility ladder is what a published grade would have to
lower into.

## 10. Standing

Nothing here overturns a D-numbered call, `30b` or `34b`. Section 5 applies D51's derived-not-declared
ruling and the const-fn key discipline (`26:174-186`) rather than replacing either, and the one thing
it changes is a return type. Section 4 takes files 35 and 36's removals as settled and reads their
consequences for the relation question rather than reopening them. The two places I contradict a panel
file are `34:176-190`, where the contradiction is compiled in probe 2 with a witness whose grades are
identical on both sides, and `33:112-115`, where the ladder's shape is corrected to a lattice with the
closure asserted rather than argued.

Two of my own claims were killed by the compiler during this dispatch. I predicted wrapping addition
was graded-associative from a counting argument that is sound under a hypothesis I had not noticed I
was using, and the model refused it before I had written a line of the report. And my first mechanism
had a licence check that refused exactly the case the mechanism existed to handle, which turned out to
be the design telling me that two different things had been run together; the repair removed a
parameter and removed the only place a consumer could have declared a permission, which is the place
this review has already measured a permission going undetected when corrupted. Both refusals are in
the probe headers rather than only in `OUTCOMES.md`, because a probe that only ever passed is not
evidence that it was checking anything.
