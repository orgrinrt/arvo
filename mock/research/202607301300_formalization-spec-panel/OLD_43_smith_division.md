# 43. Division

**Member:** Julius O. Smith III. I wrote file 24, on the multiplicative half, nineteen files back;
the relocation it built (rounding moves from the multiplication to the narrowing, `mul_full` +
`quantize`) survived the stretch and is load-bearing in the ratified shape, and I carry none of its
other conclusions forward unexamined. Its section 6 already sketched division in prose
(`24:389-425`); everything there was reasoned, nothing compiled, and this dispatch exists because
the same is true of every division sentence in the review, including the consolidation's own
prediction. The habit of mind this file runs on is the one I have used on every divider I have ever
shipped: the exact quotient is not a number you compute, it is a number you compare against, and
the remainder is how you compare.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0
failed, 9 ignored, summed per binary, matching files 41 and 42 exactly. The division-relevant test
surface read in full rather than counted: `mock/crates/arvo/tests/fixed_point_div.rs` (117 lines,
real assertions with deliberate container-overflow setups, one correctly-formed catalogue red at
line 111, `#[ignore = "catalogue: ... tracked #5"]`). It pins the shipped truncate-toward-zero and
div-by-zero-returns-numerator behaviour my own file 24 condemned (`24:415-425`); that is regression
pinning of the pre-design shape, legitimate under the redesign posture, and nothing in it is
tautological. `grep -rln "Adjustment\|Bias\|Numeral" crates/ --include="*.rs"` returns nothing, the
same empty result files 36, 41 and 42 record: the design surface this file works on has no shipped
source. Canon gate: `40_consolidation_three.md` in full before a line of code; nothing below
overturns a ratified call, and the two corrections below are to unratified consolidation sentences
(`26:678-681` carried into `40:651-655`, and `40:178-180`), both in the reasoned-without-artifact
evidence bin by the consolidation's own four-bin discipline, which is exactly the bin this dispatch
was sent to empty.

**What I read:** `40_consolidation_three.md` in full, the base reading. `41_chlipala_the_rational_
bias.md` and `42_arntzen_the_observation_surface.md` in full, the two deliverables since it. By
exception, where the consolidation compresses something I build against: `26:600-688` (the open
list carrying the prediction in its original wording), `28:315-355` (the closure fix and the
divide-by-radix-power naming), `24:389-425` (my own division sketch, reread as a claim list rather
than trusted), and `42_probes/vu_nat_sealed.rs` / `vu_bias_sealed.rs` as source, since probe 3
composes with them. `ls` of the review directory once: 42 numbered deliverables plus probe
directories before this one.

**What I compiled or measured, separated from what I reasoned.** Everything load-bearing is
compiled: `43_probes/` holds five probes plus the two sealed-tower copies, every one built fresh
against the pin (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`) while writing this file, all
outcomes and the Python pre-computations in `43_probes/OUTCOMES.md`. All numbers below are
const-eval counts, exact integer arithmetic, or bit lengths; no timer ran anywhere, and no runtime
cost claim is made (that bench belongs in `mock/benches/` and is named in section 6 rather than
faked here). The reasoned-only items are marked in place: the signed extensions of probes 2, 4 and
5, the per-application event reading probe 5's header states and cites, the IEEE cause-split
reading in section 3, and every sentence of the proposed shape in sections 2 through 5 that
composes compiled pieces into a spec recommendation.

## 0. The verdict, stated first

**The prediction is false as stated, true as it was originally coordinatised, and its practical
force survives in a sharpened, now-measured form.** `26:678-681` predicts "no finite accumulator
solution at all" for division. Compiled (probe 1): under the ratified identity contract, whose
adjustment is a rational (D69's overturn plus the closure fix of files 28/36/41), the finite
accumulator EXISTS. For operand numerals with divisor index bound K, the numeral with adjustment
`(A1/A2) / lcm(1..K)` contains every quotient exactly; checked exhaustively at two model widths,
and lcm is the least possible denominator (every d <= K occurs as a lowest-terms quotient
denominator, so any common quantum's denominator is divisible by all of them; also checked
exhaustively). What the prediction was reaching for is real and now has a number: the accumulator's
width is **Theta(2^p) bits**, exactly 5, 12, 23, 51, 95, 190, 370 bits for p = 2..8, against 2p
for multiplication's exact product and p + ceil(log2(n-1)) for an addition fold. Division is not
the operation with no accumulator; it is the operation whose accumulator is **exponentially wide**,
which is a third growth class, and the consolidation's conditional ("if the prediction holds, the
two-case working assumption is wrong") fires with a sharper reason than it anticipated: there are
three cases because there are three growth rates, not because one case lacks an object.

**How the prediction went stale is worth one paragraph, because it is the review's own recurring
failure class arriving again.** In the dyadic coordinates the prediction was written in (file 11's
era, radix-power quanta only), it is TRUE, and probe 1 compiles that half too: no dyadic grid at
any width contains 1/3 (2^F mod 3 is never 0). The identity contract then moved to rational
adjustments, and nobody re-ran the prediction across the coordinate change. That is the same shape
as the `Bias = Int` defect file 39 found (a derivation's assumption surviving a ratified change
that invalidated it, `40:469-479`), one section over.

**The exact subfamily is larger than the design has been naming, and it costs nothing.** File 28
named division by a power of the radix (`28:329-336`). Compiled (probe 3): under the rational
adjustment, division by ANY fixed nonzero representable constant is exact, and its numeral-level
map is the already-built rational multiplication of files 41/42 with the constant's components
swapped: adjustment `A1 * (cd/cn)`, bias `B1 * (cd/cn)`, both through `PMul` + `Reduce` on the
sealed tower, at concrete types, zero new mechanism. The radix-power case is the special case
where the result stays in the operand's dyadic family and the lowering is an exponent shift.
Reciprocal-multiply, the licensed liberty my file 24 flagged at the value level (`24:403-413`,
two roundings against one), is a THEOREM at the numeral level: type-level rationals have no
rounding to double. And totality is by construction: the constant's numerator position is
`Pos`-bounded, no constructor produces zero (`41:132-141`), so divide-by-zero is unspellable for
this subfamily, and with file 38's correction (`40:279-287`) IS_EXACT together with Total
trivialises the grade: every law at the finest view, by construction.

**General division needs no new quantiser and one corrected sentence.** Division is
`quantize(exact quotient)`, one quantisation, the round-first pipeline unchanged, exactly as my
file 24 stated and IEEE defines. But the consolidation's overflow-band sentence (`40:178-180`,
"inhabited for multiplication, division, ...") is wrong for division, and it was never compiled:
for same-precision division, any dyadic scales, the band is EMPTY at every p in 2..=8 (probe 2,
exhaustive, with an algebraic residue proof covering the low-shift half), while same-precision
multiplication inhabits it in the same probe. Division patterns with ADDITION on this axis. The
band is inhabited for division exactly when operand and result precisions decouple, which the
ratified MATLAB SpecifyPrecision requirement makes first-class (`40:580-583`); probe 2's witnesses
(196/13 strictly inside the band; 31/2 on the tie) show round-first doing precisely its job there.
Correction owed: division moves from the "inhabited" list to a per-format-triple statement.

**The relocation question, answered honestly: the template does not transfer, and what transfers
instead changes the carrier's kind.** File 24's move made multiplication tractable because the
exact product exists at 2p bits, so rounding could relocate to the narrowing of a real typed
object. For general division no such object exists at usable width (that is what the tested
prediction now precisely says), so there is nothing to relocate onto, AS A NUMERAL. The finite
exact carrier of a/b is the Euclidean pair (q, r), a = q*b + r, 0 <= r < b, and correct rounding
onto any result grid is a function of a remainder, never of the exact quotient. Compiled (probe 4)
at mixed quanta: the pair is exact at every input, the quotient's index bound equals the
identity-axis formula exactly, the remainder lands on the gcd quantum the MAC machinery already
computes (`40:345-350`), and the scaled-remainder rounding agrees with a never-dividing
definition-shaped oracle on every pair. So: not the template applied again, and not a failure to
find one; the same idea survived by changing what the exact intermediate IS. Had I assumed the
template transferred, I would have proposed a `divnum(N1, N2)` result numeral, and probe 1's width
table (370 bits at p = 8) is what that proposal runs into.

**A law over division claims less than a law over multiplication, and the lattice already has the
words for how much less.** Division is not associative in the rationals, so there is no
associativity atom to state or lose. The laws it has: the defining law (correct rounding against
the pair, probe 4), the Euclidean law (exact, probe 4), bounds (half an ulp, file 24's framing
unchanged), and the round-trip `div(mul_full(a, b), b) = a`, whose finest view probe 5 computes
exhaustively: values agree wherever defined, definedness fails at exactly the zero divisors,
events never agree (one against zero). Its finest view is the weak-equation corner of file 37's
lattice, computed rather than assigned, and no division-specific extension to the lattice
vocabulary was needed, which answers the dispatch's question about what the finest-view mechanism
"does with a quotient": it does what it does with everything else.

## 1. The prediction, tested (probe 1)

The compiled content, briefly, since the verdict above carries most of it. Three claims and a
table:

1. **Dyadic half (the prediction's original coordinates): TRUE.** The quotient set of any two
   dyadic-quantum numerals contains 1/3 (index 1 over index 3), and 2^F is never divisible by 3,
   checked by residue iteration to F = 1000 (the residues cycle 1, 2). No dyadic accumulator at
   any width. The prediction was correct when written.
2. **Rational half (the ratified coordinates): FALSE.** Zero-bias operands, divisor indices
   1..=K: every quotient k1/k2 lies on the grid with relative denominator L = lcm(1..K), because
   k2 divides L. Exhaustive at p = 3 (L = 420) and p = 4 (L = 360360). Biased operands change
   nothing structural: the divisor's value set is still a finite set of rationals, a finite set of
   rationals has an lcm of denominators, and the same construction goes through (reasoned, not
   compiled; the zero-bias case is the compiled witness).
3. **Minimality: the lcm is forced, not chosen.** 1/d is a quotient for every d <= K, so any
   common quantum's denominator is a multiple of every d, hence of L. Exhaustive at p = 3: no
   b < 420 works.

| operand precision p | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|
| division accumulator, bits | 5 | 12 | 23 | 51 | 95 | 190 | 370 |
| multiplication (`mul_full`), bits | 4 | 6 | 8 | 10 | 12 | 14 | 16 |

The growth is lcm(1..K) ~ e^K (the prime number theorem's psi function), so the bit count is
K * log2(e) + O(p): exponential in p. At p = 16 the accumulator would be on the order of 94,500
bits. The p = 7 and p = 8 rows exceed u128 and are computed with a fixed-size const bignum from
prime powers, cross-checked against Python's `math.lcm` and against the u128 path where both
exist.

**What the prediction holds FOR, stated precisely, per the dispatch's own demand.** Over a divisor
NUMERAL: no accumulator at practical width (it exists at exponential width, which for every
engineering purpose is the same refusal with a better derivation). Over a divisor CONSTANT: the
prediction never applied, the accumulator is one reduced ratio wide, and that is section 2. The
review has been discussing these as one case; they are the two ends of the same operand-restriction
axis file 28 already named for exactness in general (`28:329`), and the accumulator question
separates them as cleanly as the exactness question does.

## 2. The exact subfamily: division by a representable constant (probe 3)

The shape, stated for the next consolidation to take nearly verbatim:

**`div_exact` is a family of maps `N -> divnum(N, C)`, indexed by a type-level nonzero rational
constant C = cn/cd, with `divnum` computed from the identity axes alone: same radix, same
precision, same domain, adjustment `A * (cd/cn)` reduced, bias `B * (cd/cn)` reduced. The index
map is the identity: k passes through untouched, which is what exact means and why the operation
is free at runtime (an exponent reinterpretation when C is a radix power, a type-level reduction
otherwise). The operation is total and exact by construction: C's numerator is `Pos`-bounded, so
a zero divisor has no spelling (the tower's own no-constructor induction, `41:132-141`), and by
`40:279-287` IS_EXACT with Total gives the trivial grade, hence every law at every view.**

Probe 3 compiles all of it with machinery from files 36/41/42 unmodified (the sealed tower, per
file 42's fix, which composed without friction: one more small corroboration that the seal
obstructs nothing legitimate). Witnesses: 3/4 divided by 4 is 3/16 (the exponent-shift case);
3/4 divided by 3/2 renormalises 6/12 to 1/2; 2/3 divided by itself is 1; the bias 1/2 divided by
3/2 is 1/3 through the same alias. The value-uniqueness obligation holds through the operation
because the output is spelled by `Reduce`, the same normal form everything else uses.

Three consequences worth their sentences:

- **File 28's divide-by-radix-power is the right instinct scoped to the wrong coordinates**, the
  mirror image of the prediction's failure: it was named under dyadic quanta, where radix powers
  are the only exactly-invertible constants. Under the ratified rational adjustment every
  representable nonzero constant is exactly invertible at the numeral level. MATLAB, SystemC and
  every DSP scaling idiom (divide by a sample rate, by a window length, by a fixed gain) land in
  this subfamily, not only the power-of-two scalings.
- **The value-level "exact division" file 38 anticipated should not exist as a new operation.**
  A division that is exact when the quotient happens to land on the grid and refuses otherwise is
  general division with `Refuse` resolution, already expressible through the one quantisation
  funnel; adding a second operation for it would re-split what the design just unified. The
  exact-and-partial slot file 38's corrected sentence guards is instead inhabited by section 4's
  `div_floor`/`rem`, see below.
- **Sign is an identity-axis consequence, not an operation feature**: dividing by a negative
  constant flips `SignDomain` the same way biased multiplication's sign dispatch works in files
  41/42, and the magnitude machinery is shared. Not compiled here (probe 3 is magnitudes only);
  flagged as the same small extension file 41's sign handling already demonstrates one operation
  over.

## 3. General division: the pipeline, the band, and the causes

**The pipeline statement the spec should carry**: division has two disjoint paths, split by
whether the exact quotient exists. Where the divisor is nonzero, the exact quotient is a rational,
and the ratified quantiser applies UNCHANGED: round on the unbounded grid by the direction triple,
classify the rounded result against the range including `Specials`, resolve by the range rules.
Where the divisor is zero, there is no quotient to round; that path never enters the quantiser and
is a refusal cause with no quantiser origin, resolved by the numeral's own vocabulary (refuse;
reify into `Specials` where they exist; substitute per `Policy`, the checked classification my
file 24 already demanded replace the shipped silent numerator-return, `24:415-425`).

**The band correction (probe 2).** For same-precision division the two quantiser orders agree
everywhere: the overflow band is empty for every p in 2..=8 and every dyadic scale combination.
The membership condition reduces to `2*K*k2 < k1*2^m <= (2*K+1)*k2`, and for shifts m <= p+1
there is a two-line residue proof (2Kk2 is congruent to -2k2 mod 2^m, which always falls short of
the next multiple by more than the window's width); the remaining shifts are swept exhaustively.
The same probe shows same-precision multiplication inhabiting its band (121/64 at p = 4, F = 3),
so `40:178-180`'s list is right about multiplication and wrong about division, and the error was
invisible because nobody compiled the division case separately. Once precisions decouple, the
band inhabits immediately (196/13 into a p = 4 integer numeral sits strictly inside (15, 15.5):
round-first delivers 15 in range, classify-first wrongly declares overflow; 31/2 sits on the tie
and correctly overflows through ties-to-even). The design conclusion is pleasant: the round-first
amendment needs no division-specific text, and the same-precision family is additionally immune
to the one failure the amendment exists to prevent.

**The cause split, reasoned from the standard and owed a sentence in the spec.** IEEE 754-2019
clause 7 distinguishes x/0 with x nonzero and finite (divideByZero, result the correctly signed
infinity) from 0/0 (invalid, result NaN). These are two different causes with two different
standard-mandated reifications, and the design's cause enumeration currently carries
divide-by-zero as one member (`40:284-285`). Under the standards test (`13c`, run by file 39) a
`conv-ieee754` composition must express both, so the enumeration owes the split: two members, or
one member with a datum-bearing distinction. The reification into `Specials` is also where file
34's stability hypothesis (`40:788-792`: the reifying element must lie outside the value set and
absorb the operation) gets its standard-blessed instance: infinity absorbs division from the
right. Marked reasoned; the compile that would pin it is a model-float exhaustive check of the
two-path pipeline against IEEE's tables, the same shape files 30/31 used for the attributes, and
it belongs to whoever builds the float division probe.

**Direction and the key.** `40:288-294` ratifies: direction enters a law's key exactly when the
exact result can leave the operand lattice. For general division over a divisor numeral the exact
quotient leaves EVERY lattice of practical width (section 1), so `Direction` is in the key
unconditionally. For the exact subfamily it never is. The operand-restriction axis and the
key-membership predicate coincide, which is the tidy form of the dispatch's "say what it holds
for": one predicate, divisor-is-a-constant, decides exactness, totality, grade triviality,
accumulator existence at linear width, and direction's presence in the key, all at once.

## 4. The pair: division's finite exact carrier (probe 4)

The compiled facts: at mixed quanta (dividend on 1/4, divisor on 1/3, remainder forced onto
gcd = 1/12), for every pair, `a = q*b + r` exactly with `0 <= r < b`; the quotient's observed
maximum equals the identity-axis bound `floor(maxV(N1)/minposV(N2))` exactly; the remainder's
numeral is the zero-bias gcd-quantum numeral the MAC accumulator formula already computes
(`40:345-350`, biases folding in through the same four-monomial gcd, which file 42's three-rational
gcd probe already generalises); and correct rounding onto the result grid is a function of the
scaled remainder, verified against a definition-shaped argmin oracle that never divides. A
double-rounding control (probe 4, CLAIM D) shows rounding through an intermediate grid diverges
from rounding once on 29 of 240 pairs, pinning for division what file 24's probe 03 pinned for
multiplication: the single quantisation is load-bearing.

**The proposal this earns, and the shape question it forces.** The design has no pair-valued
result anywhere, and I do not propose introducing one. The pair ships as **two single-valued
operations, `div_floor` and `rem`** (floor and Euclidean remainder; the direction-variant
quotients are one `Direction` parameter away and share the machinery), each exact, each partial
on the divisor's nonzero-ness, jointly bound by the Euclidean law as a compiled cross-operation
law in the algebra crate. General `div` remains the atomic `quantize(exact quotient)` surface,
implemented FROM the pair (that is what probe 4's rounding-agreement claim licenses: the
implementation reads q and r, never a wide quotient), with the implementation ladder underneath
(restoring, SRT, Newton against the reciprocal, per target) staying in
`arvo-always-optimal-internals.md`'s bench-validated bin exactly as file 24 left it.

**`div_floor` and `rem` are the design's first exact-and-partial operations**, inhabiting the
case file 38's correction anticipated before any operation did (`40:279-287`: IS_EXACT alone does
not trivialise the grade; Total is also required). Their grade monoid is nontrivial purely in the
cause component: no quantisation events ever, one refusal cause family. The correction stops
being prospective the day these ship, which is one more reason to prefer this shape: it exercises
a guard the design already paid for.

**Why not expose the pair as one operation returning two numerals?** Because every consumer I
have ever seen wants one of three things: the rounded quotient (general `div`), the floor
quotient and remainder as separate integer-and-residue quantities (modular indexing, hashing,
fixed-point argument reduction), or the remainder alone (periodic phase). Two named operations
serve all three without a product type, the law binds them as tightly as a pair would, and the
optimiser fuses the two calls into one hardware divide when both are used (the fusion is a
codegen-regression test to add to the four the consolidation already owes, `40:662-665`; noted,
not built).

## 5. Laws over division, and the fold that is not owed

Probe 5's computed row, in the lattice's own terms: `div(mul_full(a, b), b) = a` has finest view
at the weak-equation corner: value-preserving, definedness-losing (exactly the zero divisors),
event-losing (one against zero). The law is real and useful (it is the algebraic content of "the
divider inverts the exact multiplier"), and its verdict needed nothing added to file 37's
mechanism. The defining law and the Euclidean law are per-value statements over finite sets,
model-width exhaustive, the same machinery as everything else. The bound family (half an ulp per
operation, relative error composition) is file 24's section 5 unchanged, with division
contributing one event unconditionally, as already stated there.

**No division fold is owed.** A sequential fold by divisors is division by the running product,
so its lawful shape is the one the design already has: fold the divisors through `mul_full` under
the MAC interior-safety machinery, divide once at the root. The fold's interior is multiplicative
and inherits sections 1.8 and 1.9 of the consolidation verbatim; the division contributes its one
quantisation at the root, which is the same "one quantisation, at the root, on a
grouping-independent argument" sentence the ratified fold story already carries (`40:352-357`).
A fold whose interior is division itself has, by section 1, a generically inexact interior at
every practical width, so the design should not offer a lawful-looking combinator for it; the
composition through the product is not a workaround, it is the mathematically unique way to make
the interior exact.

## 6. What this file does not decide

**The signed variants of probes 2, 4 and 5 are not built.** The unsigned results are exhaustive;
the signed band has a second band at the bottom of the range and the signed Euclidean convention
(floor against truncate) interacts with the shipped truncate-toward-zero the current tests pin.
Reasoned expectation: no structural change; the compile is owed before the spec sentence hardens.

**The float-division path against IEEE's tables is not built.** Section 3's cause split and the
infinity reification are reasoned from the standard's text; the model-float exhaustive check
(the files 30/31 shape, extended to the two-path pipeline) is the compile that would move them
bins. It needs the `Specials`-carrying model numeral, which no probe in the review has built yet.

**The per-application event reading.** Probe 5 assumes events count per quantiser application,
the type-level over-approximating reading `40:279-287` commits to. Under a per-value-moved
reading one lattice point of section 5's row changes and nothing else does. This is a
one-sentence spec fork of the same kind as the sibling-evaluation sentence the consolidation
already lists open (`40:639-641`), and it is op's, not mine.

**Whether `div_floor`/`rem` ship in the first contracts crate or wait for a consumer** is a
packaging call. The machinery they need (gcd quantum, index bounds from identity axes) exists;
the operations themselves are three signatures and two impls away, and nothing in this file
pretends they are built.

**The runtime cost of the divider ladder** (restoring against SRT against Newton-with-refinement,
per strategy and width) is a bench, belongs in `mock/benches/` under the harness, and no number
in this file speaks to it. The four codegen regression tests the consolidation owes gain a fifth
candidate (the `div_floor`+`rem` fusion, section 4); recorded, not built.

**The lcm accumulator as a shippable object**: nothing here proposes shipping it. Its existence
corrects a sentence and quantifies a refusal; a 370-bit accumulator for 8-bit division is a
derivation, not a product. If a future consumer wants exact rational division chains, that is
`arvo-num-systems`' Q territory (`40:209-241`), not a fixed-point accumulator.

## 7. Standing

The consolidation's division section can now be rewritten from measured ground: the prediction
comes out of the reasoned bin split cleanly in two (true in its original coordinates, false in
the ratified ones, exponential-width in force), the overflow-band sentence gains its division
correction, the exact subfamily generalises from radix powers to representable constants with
compiled machinery and zero new mechanism, general division keeps the ratified quantiser
untouched with a two-path statement and a cause split owed to the standard, and the largest
unbuilt operation acquires its carrier (the Euclidean pair as two exact partial operations bound
by a compiled law) plus its first computed lattice row. My own earlier move did not transfer as a
template, and saying so is the point of having tested it: the relocation survived by changing the
kind of the exact intermediate, not by being reapplied. Every correction here touches unratified
prose only; the three ratified calls stand untouched and are, if anything, corroborated (the
finest-view lattice absorbed division without extension, and the value-unique tower composed a
new operation without modification). All of the proposed shape sentences are
two-expert-agreement-shaped, not mine to close; I have tried to leave each with a compiled
artifact attached so that agreeing costs a read and disagreeing costs a named compile.
