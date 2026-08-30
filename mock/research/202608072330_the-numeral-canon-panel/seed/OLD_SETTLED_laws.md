# Survival sweep: laws, algebra, conversion

Extraction and verification pass over the panel's law list, algebraic structure, monoid vocabulary,
membership and inclusion, and the conversion relations between numerals. This file reports claims settled
within that theme and confirmed still standing at the end of the panel (`150`), and separately the claims
that looked settled and did not survive. Nothing here is a proposal. Where the record is genuinely
unresolved I say so and stop, rather than adjudicate on my own authority.

The theme has two separate disputes worth distinguishing before the list, because both concern "lattice
structure among numerals" and conflating them corrupts the record. The first runs through files 33 to 39
and 130 to 139: whether the panel's own view lattice (the grade-monoid, finest-view mechanism that gives
every law its content) is correctly mapped onto the three literature relations named for it. The second
runs through files 145 to 150: whether arvo's numeral *types* (`UFixed<I,F>` against `binary32` against
each other) form a lattice under value-set inclusion. Both produced a compiled refutation of a claim that
had read as settled. Neither refutation is the other's.

## Coverage, stated honestly

Read in full: `149`, `150`, `08b`, `13b`, `13c`, `30b`, `34b`, `39b`, `44b`, `48b`, `53b`, `57b`, `60`,
`62b`, `67b`, `68b`, `69`, `135`, `136`, `137b`, `138b`, `139b`, `144b`, `145b`, `145c`. Read in targeted
sections with citations followed to source: `124` (the twelfth consolidation, sections 0.3, 0.4, 1.2, 1.4,
1.6, 1.7, 1.8, 1.9, 1.10, 1.16, 1.25), `146`, `148`. Grepped across the full 313-entry directory for the
theme's vocabulary (law, identity, homomorph, monoid, lattice, semilattice, meet, join, antichain,
quantis, narrow, widen, conversion, embed, order, inclusion) to locate every candidate file, then followed
the heaviest hits.

Not read: the bulk of files 14 through 32 (the first identity/laws deep dive) and 89, 131, 138, 143 in
full; their content reaches me only through citations in `124`, `135` and `136`, which I did not
independently re-verify against those primary files line by line. Where a claim rests only on that
secondhand path I say so. I did not audit `124` or `110` for further entailment failures beyond what `135`
and `136` themselves already found and named; those two documents are 7099 and 6954 lines and I read
perhaps a fifth of each, chosen by section relevance rather than end to end.

## Survivors

### The algebra ladder is a monoid hierarchy, and it is op's own naming, twice

**Claim.** `Monoid<Op>` is `Identity<Op> + Combine<Op>`; `Combine<Op>` is renamed `Magma<Op>`; a magma is a
set with a binary operation and no law claimed, a semigroup is a magma whose operation is associative; the
ladder is written to the depth the theory goes, not to the depth arvo's numerals currently reach.

**Settled at:** `124:3622-3626` (quoting D23) and `124:1607-1617` (quoting D75).

**Provenance:** RATIFIED. D23 (from round `202607291200`, op, 2026-07-29, `inherited:1087-1094`): "moves
`Identity` and `SignedIdentity` to `arvo-algebra-contracts`, on the reasoning that identity is algebraic
structure, `Monoid<Op>` is `Identity<Op> + Combine<Op>`". D75 (from round `202607301100`, op, 2026-07-30,
`talk:1802-1808`), quoted directly in `124:1608`: "`Combine<Op>` is `Magma<Op>`, and the ladder is named
in full." These are op's own numbered decision-register entries, which section 0.4 of the consolidation
establishes as one half of the ratified rung (the other half being the checkpoint files).

**Rests on:** nothing else in the theme; this is the naming act itself.

### No shipped or spellable preset is a dioid over (max, +), and the reason is three separate axiom failures

**Claim.** Wrapping addition does not distribute over the lattice operations; saturating addition is not
associative and separately fails the annihilation axiom; `Precise` addition is partial. None of the three
is a reason to drop the `Dioid` rung from the ladder, because the ladder reports a derived "no" with the
failing axiom named rather than refusing to declare the rung.

**Settled at:** first established `13_mcsherry_where_the_laws_belong.md` (exhaustively over 64 directed
graphs and 625 weight vectors, per `13b:44-46`), restated as N1 through N4 in the harvested law list at
`135:174-184`, and reconciled with D47's dioid-obligation ruling via D75 at `124:1596-1606` (quoting
`talk:1802-1812`).

**Provenance:** ONE EXPERT for the axiom-failure compile (McSherry, file 13, never independently
re-derived by a second member, though relied on without contest by every later file that touches the
ladder). RATIFIED for the consequence ("derived rather than declared"): D75, quoted at `124:1599`, "it is
D75's own reconciliation with D47 rather than a panel derivation: the two do not conflict once *declaring*
and *implementing* are separated, D47's sketch-and-bench obligation attaching to implementations... and
not to declarations."

**Rests on:** McSherry's file-13 compile is what makes the ratified reconciliation possible; if the axiom
failures were wrong, D75's "derived, not declared" clause would have nothing to derive.

### The finest-view mechanism: grade is a free commutative monoid, a view is a monoid homomorphism out of it, every law has a unique finest view

**Claim.** A term's meaning is a grade (a free commutative monoid over refusal causes and quantisation
events) and a value. A view is a monoid homomorphism out of the grade. Two terms are equal under a view
when the view sends their grades to the same thing and their values agree wherever present. The set of
views under which a law holds is downward closed and closed under join, so every law has a unique finest
view, and that view is the law's content. The resulting nine-point lattice (three detail levels, Ignore
/ Presence / Exact, for each of two generator classes) is not a chain: `Hot` on a signed numeral and
`Precise` below its accumulator's interior-safety threshold sit at incomparable points.

**Settled at:** `39b:1-11` ("op's third and final ratification"), restated `124:1378-1391`.

**Provenance:** RATIFIED. Op, `39b`: "The finest-view mechanism replaces the three-relation fork." This
ratification is unqualified and has been relied upon without contest through the end of the panel; file
144 cites it verbatim at `144:186-189` as "the ratified argument" grounding an unrelated question, as late
as checkpoint thirty-seven (`144b:62-67`, "Same conclusion, and it does not evaporate if a fifth posture
is ever added").

**What does not survive, and what does.** The general mechanism (V1, V2, V3, V5 in file 136's harvest: the
homomorphism equations, join-closure of the domain, and the pullback-is-conjunction lemma) is compiled
exhaustively over all nine views, both readings of the ambiguous middle detail level, and all 81 ordered
view pairs, with zero failures (`136:107-126`). It is unaffected by the refutation below. See casualties
for what is not.

**Rests on:** the grade being a genuine free commutative monoid over the five clause-7-style generators
(a construction, not itself contested); the identification with IEEE 754's sticky flag register, "bit for
bit, over the five clause-7 exceptions with no adaptation needed" (`124:1461-1463`, sourced `58:280-283`).

### The `TotalOrd` split: value-level law-usable order against datum-level `totalOrder`, and it is a precondition of the distributivity law rather than a naming cleanup

**Claim.** The shipped bit-comparator `TotalOrd` (routing through `total_cmp_f32`/`f64`) is IEEE's own
`totalOrder`: it separates `-0.0` from `0.0` and orders NaN by payload, which are two data denoting one
value. A `const` assertion that two data denoting the same value must compare `Equal` refuses to compile
against it (`E0080`) and compiles clean against a canonicalise-then-compare body. The design's answer is
therefore to split the trait: rename the shipped mechanism `totalOrder`, non-law-usable; give `TotalOrd`
the canonicalise-then-compare body. The fork between the two readings is per-numeral, decided by whether
the numeral's encoding is injective (fixed-point numerals are, by construction; float numerals are not),
so `arvo-graph` and `arvo-comb`'s shipped weight types clear the fork for free and only `arvo-spectral`
carries a live consequence.

**Settled at:** design intent first stated `58:547-551` (consolidation five); independently compiled by
Dolan, `60_dolan_value_or_datum.md` sections 1.1 through 1.6, with a genuine `const`-refusal (`60:94-105`)
as the sharpest evidence in the file; confirmed as the second independent compiled arrival at `62b:135-143`
("60 is the second independent compiled arrival at it, so the two-expert threshold is met on the
direction"); ratified `68b:40` ("The TotalOrd split confirmed").

**Provenance:** RATIFIED, and it clears the source-justification-sweep audit that voided a neighbouring
claim in the same stretch: file 69 explicitly checks Lamport's `33:186-198` use of the trait and Dolan's
`60:145-165` use of a real consumer's code and licenses both, because in each case "the design conclusion
does not rest on the doc comment... the doc comment is offered as corroboration that the needed mechanism
is already declared" (`69:161-175`), the opposite of the shape the sweep was hunting.

**A dependency found later, strengthening rather than weakening it.** File 136 shows the split is not
cosmetic: distributivity's biconditional (law M7, "holds exactly when the operation is monotone") is
stated over the value-set order and the lattice operations `min`/`max`, and under the *shipped* order
those are not well defined on the value set (`max(-0.0, +0.0)` and `max(+0.0, -0.0)` disagree while the
two arguments are law-equal), so "under the shipped order M7's two sides are quantified over different
objects and the biconditional is not well formed" (`136:474-478`). The split is therefore a precondition
of M7, not only of law equality generally.

**Rests on:** the crossing contract's statement 3 (an encoding is injective iff no value has two data,
`124:1062-1074`), and the finest-view mechanism's canonical-quotient definition of law equality.

### Additive and multiplicative closure conditions, compiled exhaustively

**Claim.** A numeral's value set is closed under addition exactly when `bias / adjustment` is an integer
(so the shipped design's gate on `Bias = Zero` is one special case, and numerals with nonzero bias exist
that are additively closed and that a `Bias = Zero` gate would wrongly refuse). The narrowed multiplicative
product is closed exactly when the adjustment and the bias are both integers and the adjustment divides
`bias^2 - bias`, and no fixed-point numeral with a fractional digit satisfies the first conjunct, which is
the derived reason multiplication needs `mul_full` and addition does not.

**Settled at:** `33_lamport_the_laws_restated.md` (probe-established; cited by every later consolidation at
`124:1414-1426` as "from `33`, where the probe established them, rather than from any later carrier: a
restoration that cites the most recent carrier inherits every paraphrase between it and the source, and
this condition had been restored once in a stronger and different form that its own parenthetical
falsified", `111:37-95`); harvested as A1 and M5 at `135:139-156`.

**Provenance:** ONE EXPERT (Lamport, file 33), compiled exhaustively in both directions, never
independently re-derived, but relied upon without contest through file 136.

**Correction that travels with it.** The word "shipped" attached to the `AddClosed` gate is wrong.
`AddClosed`, `Bias` and `Adjustment` each return zero hits across `mock/crates/` (`136:397-402`); op
independently flagged `110:1473`'s "AddClosed... shipped" wording as one of two prior calls to re-evaluate
as stale (`137b:87-100`). The underlying law is untouched; only the claim that the gate exists in shipped
source is wrong, and it is a designed (not-yet-built) gate. Report the law, not the "shipped" word.

**Rests on:** the value-unique encoding (`44b`, ratified) and the `Numeral` identity contract's `Bias`
and `Adjustment` members being signed, gcd-normalised rationals.

### `mul_full` associativity does not typecheck without `mulnum` associativity first

**Claim.** `mul_full` is a family of maps `N1 x N2 -> mulnum(N1, N2)`, not an operation on one set, and its
own associativity claim does not typecheck until the numeral-level map's associativity is established
first, a precondition nobody had stated of the multiplicative half's own headline claim.

**Settled at:** `40:358-385`, harvested as M1 and M2 at `135:151-152`, with the typechecking dependency
made explicit at `135:337-343`: "M1 is not an optional lemma... It is the thing without which M2 cannot be
written down."

**Provenance:** ONE EXPERT (the multiplicative-half deep dive, files 24 to 25; not independently
re-derived, though never contested).

**Rests on:** nothing upstream in this theme; it is foundational to the multiplicative-half construction.

### Numeral membership in the abstract number-system chain (ℕ, ℤ, ℚ) is scoped, unique, and independent of every branch above ℚ

**Claim.** Every arvo value is `m . r^q` for integer `m`, `q`, and integer radix `r >= 2`, so every arvo
value set is a finite set of rationals. The finest inhabited system of any arvo numeral therefore exists,
is unique, and lies on the sub-ℚ chain (ℕ ⊂ ℤ ⊂ ℚ, canonical embeddings), independent of every branch of
the wider ten-member vocabulary above ℚ (ℝ, ℂ, ℍ, 𝕆, surreal, hyperreal, each p-adic).

**Settled at:** `91:183-195` (`124:1321-1325`).

**Provenance:** TWO EXPERTS. `124:1321`: "Two independent reads established that every arvo value is
`m . r^q`..." This is separate from and corrects the mechanism's earlier stated justification ("unique
because the tower is a chain"), which was refuted against the full ten-member vocabulary by Ostrowski's
theorem (`68:301-309`) since the surreals, hyperreals, and each p-adic are pairwise incomparable branches
rather than a chain.

**What governs it, RATIFIED.** D38 and D39 (both from round `202607291900`), op's own calls: the
`arvo-num-systems` vocabulary is ten members fixed by mathematics, shipped even if nothing uses them yet;
membership is through algebraic structure. D39 was challenged twice (its stated mechanism does not
compile; a naive membership predicate is vacuously true of everything) and held rather than overturned at
`30b`, on the reasoning recorded there: "The narrower reading, that its honest content is closure over an
exact and widening family, is a candidate rather than a conclusion." That narrower reading (the predicate
is `Inhabits`, not `Equals`) became the design's working answer (`40:227-235`) and is what the two-read
uniqueness theorem above is stated against.

**Open, and it is op's**, per `124:1341-1353`: whether the branch count depends on which embedding
signature (ordered-field against plain field) is meant, and whether the seven upper vocabulary members are
a second, upward-closure relation or genuinely anticipatory vocabulary for value sets arvo does not yet
build. Neither reading is adopted; both are offered symmetrically.

**Rests on:** D38/D39 (ratified), and, externally, Ostrowski's theorem and Conway's universal-embedding
theorem, cited rather than compiled per the review's standing practice for standard mathematics no
toolchain here could check.

### An identity-free numeral's absent multiplicative identity is the same absent element that opens `sqrt`'s overflow band

**Claim.** The overflow band for correctly-rounded `sqrt` has a closed-form emptiness criterion and is
inhabited exactly on the numerals that have no representable multiplicative identity.

**Settled at:** `124:2167-2170`, harvested as E3 at `135:244`.

**Provenance:** TWO EXPERTS. `124:2169`: "the same absent element that breaks the multiplicative identity
opens the root's overflow band, discovered independently by two members in the same stretch." `135:250-254`
restates it as the row's own significance: "It is the only row whose key contains the existence of an
identity element, which makes it the row that catches an identity-free numeral shipping with an `Identity`
impl it should not have."

**Rests on:** the identity contract's existing (and, per the next entry, incomplete) `Identity<Op>`
mechanism.

### The identity element, where it exists, is unique; a representable zero is exactly A1's closure condition; multiplicative closure and a representable identity are independent conditions

**Claim.** Three derived facts about the shipped-but-incomplete `Identity<Op>` trait. Uniqueness: where an
identity element of an operation exists in a numeral's value set, it is unique (the one-line monoid
argument `e = e * e' = e'`). Representable zero: zero is in a numeral's value set exactly when `bias /
adjustment` is an integer, which is A1's own closure condition restated for the additive identity
specifically. Independence: multiplicative closure and a representable multiplicative identity are
independent conditions, in both directions over the rationals and in one direction over the integers.

**Settled at:** `136:322-326` (I3, I5, I6), compiled by search over 1,200 `(bias, adjustment, denominator)`
triples for I5 with zero disagreements, and with witnesses in both failing directions for I6.

**Provenance:** ONE EXPERT (Willsey, file 136), derived and compiled within the file, not independently
re-derived elsewhere, but unchallenged through the end of the panel.

**What is explicitly a gap, not a survivor, and should not be read as settled.** The identity *equations*
themselves, `x + e = x` / `e + x = x` and `x * e = x` / `e * x = x`, are asserted by the shipped mechanism's
own doc comment but stated as a checked law nowhere in the design (`136:320-321`, "stated nowhere"). The
same is true of the order axioms (reflexivity, transitivity, antisymmetry on the canonical quotient,
totality minus the unplaced NaN class): all four are named as gaps at `136:453-456`, not as settled
claims. I report this distinction because a superficial read of the harvested law-list table could mistake
a "stated nowhere" status cell for a settled result; it is the opposite.

**Rests on:** A1 (the additive closure condition, ONE EXPERT above) for I5 specifically.

### The conversion order between numerals: four conditions, not two, and it is a real geometric object (grid, phase, both endpoints), not an approximation of one

**Claim.** A numeral `a` includes into numeral `b` (every value `a` can denote, `b` can also denote)
exactly when four conditions hold together: `b`'s grid is at least as fine as `a`'s, the two grids are
phase-aligned, and `b`'s range reaches at least as far as `a`'s at both the low and the high end. A
two-condition test on integer and fraction width alone (the form first proposed) is not merely incomplete,
it is unsound: over its own sweep it produces 17,037 false positives, roughly evenly split between phase
failures and floor failures, because every numeral in the sweep that established it had bias zero, which
made the phase condition read as a triviality throughout.

**Settled at:** `149:11-33`, reconciling `146` (which found four conditions and a compiled counterexample:
a target on a strictly finer grid whose range strictly contains the source's, that still represents none of
the source's values) and `148` (which derived the same four independently, explaining why they are four by
giving the value set as an affine lattice intersected with an interval, so inclusion is inclusion in each
factor).

**Provenance:** TWO EXPERTS, and it meets the workspace's stated bar precisely: "Two experts independently
agree that `145`'s two-condition order is wrong, and each derived its own before reading the other's"
(`149:13-14`).

**Rests on:** the affine value map (`124`'s section 1.1, `Adjustment * radix^exponent * k + Bias`), and the
sign-domain discriminant recorded at `124:986-991` (the `SC_SAT_SYM` cell), used by `148` to pin the index
interval.

### Inclusion of value sets among fixed-point (or, separately, among float) numerals of one family is a distributive lattice; meets are exact, joins strictly overshoot

**Claim.** Within the anchored fixed-point family alone, the value-set inclusion order is a distributive
lattice: it is the componentwise order on a product of three chains (fraction-width, negated low bound,
high bound), and a componentwise order on a product of chains is always a distributive lattice. Meets
computed coordinatewise land back on the design's own two curves (unsigned, signed) in every case, subject
to two closure conditions the design must grant (admit the zero-width numeral; admit negative integer
width). Joins land back on a curve in every case with no closure condition needed at all, which is the
opposite of the usual intuition that unions are harder than intersections. The same argument, applied to a
different triple of coordinates (significand precision, reach, finest step), makes the float family alone
a distributive lattice too.

**Settled at:** first asserted `145_dolan_the_conversion_story.md` (`145b:74-77`, adopted by op subject to
a second read: "The numerals form a lattice, meets are preserved exactly, joins strictly overshoot at all
1,080 incomparable pairs"); the meet-exactness half independently re-verified at 0 failures over 351
unbiased pairs by `148` (`148:51-52`, "`145` keeps: the meet being exact... which I confirm at 0 failures
over 351 unbiased pairs"); given its full closed-form derivation, its two isolated closure conditions, and
cross-validation by three independent methods (exhaustive set enumeration, closed-form predicates checked
against enumeration over 5,184 shape pairs with zero disagreements, and direct construction) at
`150_knuth_what_structure_the_numerals_form.md` sections 1 through 3.

**Provenance:** TWO EXPERTS for the within-fixed-point result as originally stated (145, then 148's
independent reconfirmation of the meet). The full, closure-condition-isolated, cross-validated derivation
at file 150 is ONE EXPERT (Knuth) and is unratified: the panel record ends at file 150 with no checkpoint
responding to it, so its refinements (the (Z)/(N) closure conditions, the extension to the float family,
and the sharp statement that the *inclusion conditions themselves* used by 145/146/148 were four
componentwise conditions all along) are the strongest available account but carry only one expert's
signature.

**What this claim is not, and the distinction is load-bearing.** This result is scoped to one family taken
alone. See the next entry and the casualties section for what happens once fixed-point and float numerals
are considered together in one order, which is a different question this same file (150) answers, and
answers in the opposite direction.

**Rests on:** the four-condition inclusion test (previous entry), restated by `150` in closed form as a
componentwise order on `(F, -L, G)` (or, for float, on `(p, reach, finest step)`).

### Two distinct numerals with the same number of representable values are incomparable (the cardinality antichain), and it is universal, not a coordinate artifact

**Claim.** If numeral `A`'s value set includes into numeral `B`'s value set and the two have the same
(finite) cardinality, the two value sets are equal. So two distinct numerals of the same size cannot
include one another, for every bias, every adjustment, every radix, every sign domain, in every family the
design admits, including ones nobody has written down yet. This is strictly more general than a claim
about matching integer-and-fraction-width coordinates (which is only a componentwise-order argument valid
inside one fixed-point family), and it fails to coincide with the narrower coordinate-based statement for
`Ranged` (float) numerals, where cardinality and precision come apart: two float-shaped numerals of equal
significand precision but different exponent ranges can stand in strict inclusion.

**Settled at:** `146_chlipala_the_order_and_the_cast.md` section 4 (the general cardinality form, compiled
at 254,016 equal-cardinality ordered pairs over 1,008 numerals with zero exceptions), confirmed at
`148:56-58` ("`146` keeps: ... the cardinality restatement of the antichain").

**Provenance:** TWO EXPERTS (146 derived it, 148 independently reviewed and endorsed it in the "what each
file should keep" reconciliation).

**Rests on:** the affine value map and finite cardinality of every arvo value set (the same fact the
membership-uniqueness result above rests on).

### Narrowing is quantisation with the operation set to the identity, resolved by the target strategy's own row, with no new marker or key column; `Hot`'s narrowing is not monotone

**Claim.** Narrowing a numeral is not a new mechanism. It is the same quantiser the design already uses for
in-numeral operations, applied to the identity operation, resolved by whichever strategy's row the target
carries. No new axis and no new key column are needed for narrowing specifically. `Hot`'s narrowing fails
monotonicity, on the same footing as the already-established refutation of wrapping-addition
distributivity.

**Settled at:** `145b:84-87` (adopted by op subject to a second read), confirmed at `148:51-53` ("`145`
keeps: ... narrowing as the quantiser with the operation set to the identity; the target strategy's row as
the resolver; no new key column; `Hot`'s narrowing not being monotone").

**Provenance:** TWO EXPERTS (145 asserted and compiled it; 148 independently re-examined and confirmed it
survives, in the same file that separately overturns 145's *inclusion-order* claim, which is why the two
should not be conflated: 148 attacks one 145 claim and defends another in the same document).

**Rests on:** the quantiser's existing law family (Q1 through Q7 in the harvested list) and the strategy
door's existing per-preset resolution mechanism.

### A conversion needs a new key column naming which of the two involved numerals' strategies adjudicates it

**Claim, stated as a settled negative.** The design's current schema does not say which of a conversion's
two strategies (source, target) adjudicates the conversion's behaviour, and this gap is real: `146`
measures 33 percent disagreement across lossy conversions depending on which strategy's row is consulted,
and both `146` and `148` independently show `145`'s original claim that no new key column is needed cannot
be detected from `145`'s own test setup, because every reading `145` checked coincides exactly on the
embedding region its three checks lived in.

**Settled at:** `149:35-42`.

**Provenance:** TWO EXPERTS (146 and 148 agree the gap is real and that 145 could not have found it, each
from its own derivation). Adopted as a negative only: the gap's existence is settled; its remedy (a new
trait, a new field, a resolution rule) is not, and `149` says so explicitly.

**Rests on:** the strategy door mechanism and the four-condition inclusion order.

## Casualties

### The specific literature-relation-to-lattice-point identification inside the finest-view mechanism

**What looked settled.** `124:1394-1397` (sourced `40:251-267`, under the umbrella of the 39b
ratification): "The named relations are three points of a nine-point lattice": the weak equation at
(Ignore, Ignore), the Kleene equation at (Presence, Ignore), and graded equality at (Exact, Exact).

**What killed it, and where.** `136` section 2.4 (V4). The Kleene equation's actual definition ("both
terms defined or both undefined, and where defined the values agree") coincides with `(Presence, Ignore)`
only if definedness is recoverable from the grade's cause component, which is a claim (`V4`) the design's
own division chapter refutes for every numeral carrying infinity or NaN: `x/0` under `Specials = infinity`
raises a `divideByZero` cause and is *defined* (it delivers the far point); `0/0` under `Specials = NaN`
raises `invalid` and is *defined* (it delivers NaN). Both land on defined results while carrying a nonempty
cause component, which `(Presence, Ignore)` cannot distinguish from an undefined term. The probe that had
made the identification read as safe set its own `def` flag from its own cause counter by construction
(`136:217-222`, "The model made the invariant true rather than testing it"), which is why twenty-four files
carried it unchallenged.

**Op's response.** `137b:87-100`: op declared the prior calls resting on this claim, and the neighbouring
"`AddClosed`... shipped" claim, stale rather than patched, quoting his own standing principle that a
ratification holds only under the evidence available when it was made. He asked for a re-evaluation
establishing which calls rested on the refuted identification and whether "definedness... may want to be a
third view axis rather than a sentence to correct" (`139b`, echoing `136`'s own three offered repairs).

**Status at the end of the panel.** Not resolved. `136` offers three repair shapes (add a definedness axis,
giving eighteen points; split the cause generators by whether they refuse, which the author argues does not
work; assert V4 as a precondition and scope the nine-point lattice to numerals carrying no `Specials`) and
recommends the first without deciding it, explicitly naming it op's call because it changes the size of a
shipped lattice (`136:631-633`). No later file or checkpoint through `150` revisits it. Meanwhile the parts
of the mechanism V4 does not touch (the homomorphism equations, join closure, the pullback-conjunction
lemma, and the "not a chain" fact specifically) are unaffected and are relied on as late as `144`/`144b`.
**Only the specific three-way name-to-point mapping is dead, and only for the middle point, and only where
`Specials` is populated.** Do not read this as the finest-view mechanism failing; it did not.

### D2's harvested view-column entry, "Kleene, since the claim is about definedness"

**What looked settled.** `135:230`, harvested directly from the standing base, assigning division's
solution-set law (D2) the Kleene view because the law is explicitly about definedness.

**What killed it.** The same V4 refutation. `136:258-264`: "the one row in fifty-four whose view cell was
chosen *because* the law is about definedness is the row where the view named does not mean definedness."
Not a separate defect; the same one, landing on the harvested table's most load-bearing cell for exactly
the reason that made it look most carefully chosen.

### "Every equal-precision family is an antichain" (the coordinate-restricted, `145`-original phrasing)

**What looked settled.** `145:239`, adopted at `145b:79-82` pending a second read, presented as "the
structural proof of op's withdrawal at `130b`."

**What killed it.** `146` section 4: the claim is true for fixed-point numerals (where precision determines
cardinality) and **false for `Ranged` numerals** (`146:331-332`), where precision and cardinality come
apart. A compiled counterexample: two float-shaped numerals of equal significand precision (three digits),
exponent ranges `[-2,2]` and `[-4,4]`, stand in strict inclusion (41 values against 73, one a proper subset
of the other), which directly contradicts "antichain."

**What survives from it.** The general cardinality theorem (equal cardinality implies incomparable, listed
as a survivor above) is not merely a repair, it is described by `146` as the theorem the coordinate version
was gesturing at without stating it correctly. Report the cardinality form; do not report the
equal-precision form, which is false as stated.

### `145`'s conclusion that no coherent `From` between numerals is expressible

**What looked settled.** `145` established that a blanket `impl<A,B> From<A> for B` between numerals
collides with `core`'s own `impl<T> From<T> for T` (the compiler cannot see the two never overlap), that a
computed-order witness does not rescue coherence, and concluded from this that `From` is unavailable
between numerals.

**What killed it.** Op, directly, at `145b:56-59`: "That conclusion is the shape op has refused eight times
this session... 'therefore no `From`' does not follow from it; what follows is that the obvious spelling
fails and a spelling has to be found." The coherence collision itself is not in dispute; the inference from
it to "no solution exists" is refused. `148` subsequently supplies a compliant candidate spelling (a trait
bound rather than a const argument, compiling gate-free on the default solver to 128 bits), reported above
where relevant, but that candidate is one expert's proposal with a second read still owed (`149:107-109`)
and is not itself counted as settled here.

### `146`'s `From` compile claim, specifically

**What looked settled.** `146` reported a working `From` spelling as compiling.

**What killed it.** `148` found the probe carries `#![feature(generic_const_args)]` and its runner passes
`-Znext-solver=globally`, neither of which the design permits; it exits 1 without the flag (`149:69-73`).
Under this workspace's standing rule that a probe is evidence for exactly what it proved, this one proved
something about a configuration the design forbids. The claim as `146` stated it does not stand. Its
coherence *argument* (that the collision is structural, at the head constructor, rather than merely
untriggered on the cases tried) is separately confirmed by `148` on the permitted solver and survives; the
compiled spelling built on the forbidden features does not.

### The two "closing observations" in `145` about shipped source

**What looked settled.** `145`'s report that the shipped tree carries six `From` and four `TryFrom` impls
with `Cold` absent, and that its narrowing tests assert outside the value set through `from_raw`.

**What killed it.** Op, `145b:33-40`: both may be true and neither is a canon finding, because the panel's
own chain rule (`the-canon-design-code-chain.md`) forbids treating dead, superseded source as evidence
about the design, and a finding about a shipped test's contents settles nothing once the whole tree is
declared dead. Note the correction is about admissibility, not about the design content of `145`, which
`145c` explicitly says stands unaffected.

## The most load-bearing open item

**Whether arvo's numerals, taken as one order across every family the design ships, form a lattice, a
semilattice in one direction, or neither.** This is the theme's central unresolved dispute and it is
unresolved for a real, load-bearing reason rather than by default.

Three files gave three incompatible answers over the panel's last five numbered files: `145` said lattice,
exact meets, overshooting joins; `146` said join-semilattice only, with the meet failing under bias at
663,026 of 1,016,064 pairs; `148` said meet-semilattice and explicitly not a join-semilattice, the inverse
of `146`. `149` (the dispatcher's own adjudication, not an expert file) states plainly that none of the
three agrees with either of the other two on this question and that a third read is owed, briefed against
the mechanism rather than the counts, since counts from three differing models cannot adjudicate between
the models (`149:44-59`).

`150` (Knuth) is that third read, and it is the strongest account on the record: it dissolves the apparent
three-way contradiction by showing each file was answering a true statement about a different shape space
(within one family, both operations are total, because the shape space is closed under both under the
right conditions; across the fixed-point and float families together, both operations fail, and the join
failures specifically fall on pairs of fixed-point numerals whose join existed before floats were added to
the same order, so adding floats did not merely fail to extend the lattice, it removed a join that had
already been there). This is cross-validated by three independent construction methods within the same
file and holds at every width the enumeration reached, with the closed-form predicate agreeing with full
enumeration over 5,184 shape pairs before being trusted at widths enumeration cannot reach.

It is not adopted. File 150 is one expert, unratified, and the panel's record ends with it: there is no
`150b`, and no later file responds to it. Its own last line states the genuinely undetermined question
plainly: "whether arvo's numerals are one ordered family or several... Everything above follows from that,
and nothing decides it from the outside." That is op's, and it was still open when the panel stopped.

Do not treat `145`'s within-fixed-point-family lattice claim as having "won" this dispute merely because it
is closest to `150`'s within-family conclusion. `145`'s original inclusion test was itself unsound (see the
first conversion-order survivor above), and `150`'s within-family result rests on the corrected four-
condition test plus two closure conditions `145` never stated, not on `145`'s own original derivation.

## Summary count

Nineteen survivors reported above, of varying strength (three RATIFIED outright by a checkpoint or decision
register entry, roughly ten at TWO EXPERTS, the remainder ONE EXPERT and unchallenged through the end of
the panel). Six casualties, one of which (the D2 view-cell entry) is the same underlying refutation as
another (the literature-relation identification) landing a second time on its sharpest instance, so five
independent failure points. One item is reported separately as the theme's most load-bearing open question
rather than as either a survivor or a casualty, because it is neither: it was never settled, and the
panel's own best attempt at settling it arrived in the last file with no ratification and no rebuttal.

The three survivors that most constrain what a canon text could say, in order: the `TotalOrd` split (because
it is now shown to be a precondition of the distributivity law rather than a naming choice, so any canon
statement of M7 is unstateable without it); the finest-view mechanism minus its broken middle identification
(because it is what gives every one of the fifty-four-plus harvested laws its content, and the repair choice
for the broken identification changes the lattice's own size); and the four-condition inclusion order
between numerals (because it is the base every later conversion, antichain, and lattice-or-not claim in the
theme is built on, and the two-condition version it replaced was not merely weaker but unsound).
