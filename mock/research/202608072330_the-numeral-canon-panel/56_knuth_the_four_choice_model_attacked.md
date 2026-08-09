# 56. The four-choice model, attacked

**Date:** 2026-08-09
**Position:** file two of unit two on the topic "what is the one format concept, and what must it
cover". `55` derived the model computed = adapt(exact) cold, decomposed a format into four choices
F = (D, Q, R, E), and then read `08`, which is mine. This file attacks that model with my own
derivation, answers what `55` said about `08`, and states what I would put to `55` when it is
resumed.
**Probes:** `56_probes/`, three instruments plus a rerun record of `55`'s three, committed as
made. Every count below is from a named output file there.
**Reading:** `INTENTS.md`, `00_brief.md`, `55` in full including its probe sources and outputs,
`08` (my own), the register sections `55` touches (Q4, Q5, Q6, Q10, Q11, Q12, Q13, Q16, and the
wrapping entry `55` added), and `42` at the passages its reachability mechanism lives in, opened
because I lean on it. I did not open `25`, `35`, `18` or `43` this dispatch; where I use their
results I say the register or `55` is my source.
**Register:** nothing here settles anything. Op's explore mode is in force per `00_brief.md`.

## 0. Gates

**Canon gate.** There is no ratified canon; this panel is writing the first one, and the brief
says so. The governing material is op's intent catalogue plus the acceptance criterion, and
nothing below proposes work either forbids. The strategy set is open per I1, and nothing below
presumes a count.

**Test gate.** There is no suite. `find mock/crates -name '*.rs' | wc -l` returns 0; the tree the
brief declares nuked is empty in fact. I did not run a suite and I am saying so rather than
implying one passed.

**The brief's cheap factual claims, checked.** The pin resolves:
`rustc +nightly-2026-05-28 --version` reports `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.
`55`'s three probes exist, compile on the pin, and their committed outputs reproduce byte for
byte: I recompiled all three from `55_probes/*.rs` and diffed my reruns against `p1_output.txt`,
`p2_output.txt` and `p3_output.txt`; all three diffs are empty (`56_probes/RUN.md`, rerun
section, with the captured reruns committed beside it). So every number `55` cites from its own
probes is a number I regenerated independently before trusting it, which was the first thing I
did.

## 1. Verdict, before the argument

**The model survives, and not in the form it was stated in.** Four corrections, each with the
evidence that carries it, and one of them is large.

**First, F is not four choices; it is a dependency diagram with one nearly-constant node.** R's
own definition ("the total maps D to Q", `55:53`) is ill-typed until D and Q are fixed, and E's
until Q is. So the model is D, then Q given D, then R and E each given what precedes them, and
"four choices" read as a product invites an independence the definitions themselves refuse.
Meanwhile the record's scoping theorem (`08:113-117`, TWO EXPERTS) fixes D at the rationals for
every arvo numeral, so in arvo the D slot has exactly one live question in it, and it is the
wrapping question. Section 2.

**Second, and this is the large one: the R slot carries two independent law families, and `55`'s
"wrap is not an adaptation" is one half of a symmetric statement whose other half it did not
test.** Define the adaptation laws (monotone, distance-minimising) and the coherence law
(rho(a op b) = rho(rho(a) op rho(b)), the homomorphism-shaped law that makes chains exact).
Measured exhaustively at 4 bits: signed saturation satisfies the adaptation laws and fails
coherence (476 of 4096 triples chain-divergent); wrap fails both adaptation laws and satisfies
coherence for addition and multiplication (0 divergent); unsigned add-only saturation satisfies
**both** over a nonnegative window and loses coherence the moment the window admits negative
operands; the opposite-bound mutant satisfies neither (`56_probes/q1_output.txt`). All four
cells of the two-by-two are inhabited, so neither law family subsumes the other, and expelling
wrap from the slot while retaining saturation is a filing choice needing an argument `55` has
not yet given. Section 3.

**Third, the wrapping fork `55` put on the register deflates by one.** Its options two (wrap as
a change of ambient domain) and three (wrap as a named composite) are the same mathematics
wearing different bookkeeping: once a numeral's Q is declared, the section out of Z/2^N is fixed
by the declaration, every measurement either option predicts is identical, and `55`'s own
"conversion out is policy-laden" cost dissolves because the policy is the type. The live fork is
option one against the pair, and the two-by-two gives the pair its precise form. Section 4.

**Fourth, the phase parameter `55` conceded belongs in Q, and it is not a passive parameter.**
An affine membership predicate written once covers the biased grid; round-to-nearest onto the
biased grid keeps every adaptation law exhaustively, so R is untouched by the repair. And a
half-step-biased grid is not closed under exact addition: 0 of 256 exact sums land on it, every
one sits exactly half a step away (a systematic tie), and the grid contains neither zero nor
one (`56_probes/q2_output.txt`). So phase decides whether the identity adaptation ever occurs
and promotes the tie rule from corner case to dominant policy. Section 5.

And one completion of `55`'s refinement of `08`: for a signed value set, raw-order agreement and
raw-adder correctness are mutually exclusive over bijective encodings, measured and argued at 4
bits (`56_probes/q3_output.txt`). Memcmp-sortability is purchasable by encoding, as `55` said,
and the purchase price is the hardware adder. Section 6.

## 2. The model is a dependency diagram, and its equality is unstated

### 2.1 The tower

Take `55`'s own definitions. Q is "which elements of D the format denotes exactly" (`55:47-48`):
no Q without D. R is "the total maps D to Q that the format permits", monotone with respect to
D's order (`55:53-57`): no R without both. E is "the relation between elements of Q and bit
patterns" (`55:62-63`): no E without Q. R and E are independent of each other, because R is
value-level and E is pattern-level, which is the conjugation invariance `55`'s probe 3 rests on.

So the honest logical form is a dependency diagram: D, then Q(D), then R(D, Q) and E(Q) each
independently. The falsifiable challenge, which I put to `55` directly in section 9: name one
member of R that is well-typed before D and Q are chosen. By R's own definition there is none.

This is a correction of form rather than a refutation, and it has one payoff and one
consequence. The payoff: `55` argues E is "ordered after" Q (`55:64-67`) as if that needed
arguing; under the dependent form it is a theorem, and so is the identity ordering generally.
The consequence: `55:218-220` claims "the acceptance criterion's derivation order is forced:
usage demand, then Q, then E, then container". That conflates two different orders. The
dependency order constrains **identity** (what a format is, which two are equal). The
**derivation** the acceptance criterion names runs from a bits-and-bytes demand, and `55`'s own
probe 3 shows a fixed pattern budget buys different value sets under different encodings, so Q
and E are determined **jointly** by a budget, as one constraint solve over the diagram, not
sequentially. Both orders are coherent; they are not the same claim, and the criterion's
direction is the joint one.

### 2.2 The equality the model never states

`55:47-49` says Q "is the format's identity in the strongest sense: two formats with the same Q
are the same format wearing different clothes", and `55`'s alternative C is rejected on exactly
this ground (`55:290-298`). But the tuple F = (D, Q, R, E) makes equality tuple-wise unless
stated otherwise. Which is it? If identity is Q alone, then R and E are parameters of an
**instance** and not of the **format**, and the four-choice model is really a two-part
statement: a format is (D, Q); a realisation adds a choice from R and an E. If identity is the
tuple, then two's complement and offset binary are different formats, which `55`'s own probe 3
check 1 was built to deny.

This is not pedantry, because the register's Q10 keys on exactly this: whether inclusion is
decided on denotation or on declaration. A model whose equality is unstated cannot answer Q10's
question when it reaches formats. My own position, from `08` and unchanged: identity is
value-level, so (D, Q) is the format and R and E are realisation choices; and then the model's
four slots split into an identity half and a realisation half, which is a sharper statement than
"four choices" and one I would rather the canon candidate carry. Put to `55` in section 9.

### 2.3 The D slot is nearly constant in arvo

Two independent squeezes. The scoping theorem (`08:113-117`, TWO EXPERTS, and I flag it exactly
as `55` did: carried, not re-derived) makes every arvo value a rational, so D = the rationals
serves every numeral. And the op-set argument closes the apparent counterexample of "the
integers as D": exact division of integers leaves the integers (7/2 is no integer), so a D that
supports the op vocabulary must be closed under it, and for arvo's vocabulary that is the
rationals with the integer formats' Q sitting inside them. "D = Z" in a presentation is a
convenience, not a choice.

So in arvo the four-choice model instantiates as one constant and three choices, **unless** wrap
is filed as a domain change, in which case Z/2^N joins the D slot and the slot's entire live
content is that one question. That is worth saying plainly because it re-weights the model: the
wrapping question is not one boundary case among several; it is the whole of what the D
parameter decides in this design.

## 3. The R slot carries two law families

### 3.1 The two-by-two, measured

`55`'s probe 2 established that wrap fails monotonicity and distance minimisation, and is a ring
homomorphism image, exhaustively at 4 bits. I reran it (it reproduces) and then asked the
question it did not: **what happens when the same two law families are checked on every member
of the pool, symmetrically?**

Define, for a total retraction rho onto Q with ambient window W:

The **adaptation laws**: rho monotone on W, and distance-minimising onto Q.
The **coherence law**: rho(a op b) = rho(rho(a) op rho(b)) for all a, b in W. This is the law
that makes chains exact, since by induction it gives reduce-eagerly = reduce-once over any
fold.

`56_probes/q1_two_law_families.rs`, exhaustive, signed Q = [-8, 7], unsigned Q = [0, 15],
window [-64, 64]:

| member | adaptation laws | coherence (+) | chain-divergent triples of 4096 |
|---|---|---|---|
| signed two-sided saturation | holds | fails | 476 |
| two's-complement wrap | fails | holds, also for multiplication | 0 |
| unsigned add-only saturation, nonnegative window | holds | holds | 0 |
| opposite-bound mutant | fails | fails | 897 |

All four cells inhabited. And the third row's coherence is conditional on the window: the same
clamp over a window admitting negative operands loses coherence (`q1_output.txt`, last check),
which is the reachable-floor condition.

### 3.2 What the two-by-two does to `55`'s conclusion

`55`'s argument for expelling wrap from R is that wrap "fails every property the other members
of R share" (`55:152-154`). The symmetric measurement shows the slot's members fail each
other's properties **both ways**: saturation fails the property wrap has (coherence, hence
chain exactness), exactly as wrap fails the properties saturation has (monotone, nearest). A
"set of adaptation maps" containing signed saturation is a set whose members do not share
chain-exactness, and if heterogeneity of law profile is grounds for expulsion, saturation is
expellable by the same reasoning from a slot defined around coherence. Since both cannot be
expelled without emptying the slot, heterogeneity alone cannot be the criterion.

The alternative that keeps the slot honest: **one slot, two named law families, each member
classified**. Every generic sentence over the slot is then quantified over the family that
carries it, no member is an exception to anything, and the two families' independence is a
measured fact rather than a footnote. This is `55`'s alternative A-wide (`55:300-303`) with the
"lost uniformity" cost it could not construct a payer for now priced the other way: the
uniformity was never there, because signed saturation already breaks the slot's uniformity in
the coherence coordinate.

To be precise about what I am not claiming: `55` is right that wrap is not an adaptation, and
my probe re-establishes it with an independent instrument. What does not follow is the jump
from "not an adaptation" to "therefore a different ambient domain". The two-by-two shows the
slot was never homogeneous, so wrap's failure of the A-laws is evidence about the slot's
structure, not about wrap's homelessness.

### 3.3 The unification with the register's Q12 material

The register's Q12 carries `35`'s divergence table (unsigned wrapping 0, signed wrapping 0,
unsigned saturating 0, signed saturating 70.1%) and `42`'s mechanism, which I opened and quote:
associativity of a clamped operation "holds exactly when at most one of its clamps can be
triggered by any association order of the specific fold in question" (`42:315-316`), with the
four-block table at `42:319-324` isolating reachability from code shape, and a general pattern
sentence at `42:343-347` quantifying over trajectories.

The coherence law is the algebraic object that pattern sentence gestures at. Window-level
coherence is the window-uniform case of `42`'s per-trajectory condition: my unsigned clamp is
coherent over the nonnegative window (42's rows two and three) and incoherent over the signed
window (42's row one), and wrap is coherent over every window, which is why its folds
reassociate exactly and no reachability analysis is ever needed for it. So Q12's table, `42`'s
mechanism, and `55`'s wrap-exactness are three views of one classification: **a policy's chains
are exact precisely where its reduction map is coherent over the reachable operands.** `42`'s
per-trajectory form stays the finer statement; the coherence law is the per-window form that
also covers wrap.

Independence, bounded honestly: I read the register's Q12 and `55`'s probe 2 before building
q1, so this is not a cold arrival. The instrument is independent (my checker, my counts, one
new cell measured that neither `35`, `42` nor `55` states: the mutant's profile and the signed
clamp's chain-divergence count of 476 at 4 bits). The idea of coherence as the unifier is mine
this dispatch, first-read, owed a second.

## 4. The wrapping fork deflates by one

`55` put three options on the register for where wrapping lives. Option one files wrap in the
adaptation slot with a permanent exception. Option two makes it a change of ambient domain, a
numeral denoting residues. Option three makes it a named composite, add-then-reduce on an
integer-denoting numeral.

**Options two and three are observationally equivalent given a declared Q, and the register
entry should say so before someone spends a dispatch trying to measure the difference.** The
argument is one paragraph. Under option three, the induced operation on representatives,
a op_wrap b = reduce(a op b), is exactly the group operation of Z/2^N transported along the
bijection between Q and the residue classes; that transport is what `55`'s probe 2 check 7
verifies (I reran it) and what my q1 re-verifies for addition and multiplication. Under option
two, the datum denotes the residue class and Q names its representatives. Every operation,
every law, every stored pattern, and every printed value is identical under the two readings;
they differ only in which sentence the canon carries about what the datum "is".

And the cost `55` attached to option two, that conversion out of a wrapped numeral is
policy-laden because Z/2^N has no ring embedding into Z (`55:157-163`), dissolves under either
reading once Q is declared. The section is not chosen at conversion time; it was chosen at
**declaration** time, because declaring Q = [0, 2^N - 1] or Q = [-2^(N-1), 2^(N-1) - 1] is
precisely choosing the representative range. Rust makes the same choice the same way: `u8` and
`i8` are the two sections, picked in the type, and no cast of a stored value consults a
runtime policy. There is a residual policy question only for a hypothetical numeral that
declares residue-ness without a representative range, and nothing in the design proposes one.

What genuinely distinguishes option one from the pair, then, is not any measurement but where
the law sentences live. Under option one every generic sentence over the adaptation slot
carries a wrap exception, which is `55`'s stated cost, and section 3 shows the exception list
would be longer than `55` thought, because chain-exactness sentences would need a saturation
exception in the other direction. Under the pair, with the two-family classification, no
sentence carries an exception: each member's laws follow from its cell. That is also Q12's
candidate reframing ("state per strategy which properties the arithmetic has") arriving from a
third direction, after `25`'s axis heterogeneity (per the register; I did not open `25`) and
`55`'s kind distinction.

One honest caveat on the deflation: the equivalence is between options two and three **as
semantics**. As canon drafting they differ in which tier states the laws (domain laws against
composite theorems), and drafting cost is real. The claim is that no probe, bench or consumer
program can distinguish them, so the choice should be made on drafting economy and never sent
to an expert as an empirical question.

## 5. The phase parameter belongs in Q, and it is not passive

### 5.1 Placement, by construction

`55` conceded (phase two, 2a, `55:382-389`) that its slot-function Q bakes in phase zero, and
asked where the phase parameter belongs. `56_probes/q2_affine_membership.rs` answers by
construction. An affine membership predicate written once, x in Q iff x = B (mod S) within
bounds, matches direct enumerations at B = 0 and B = S/2 (16 and 16 values, mutant with the
bias dropped detected). Round-to-nearest **onto the biased grid** keeps all four adaptation
laws (total, retraction, monotone, distance-minimising), exhaustively over a window past both
bounds, with the wrong-target mutant failing retraction as it must.

So the repair is contained in Q: the slot function generalises to the affine form, R's laws are
untouched, E is untouched, and the four-slot decomposition survives the concession intact. That
is a support result for `55`, and it is the located answer to the dispatch's question of where
the parameter belongs.

Worth one sentence of history: `08`'s first instrument and `55`'s phase-one Q erred at this
same coordinate in opposite directions. Mine tested phase zero and called a half-unit-biased
format outside (`08:630-634`, kept as a defect); `55`'s could not express the biased grid at
all and conceded on reading `08`. Two independent instruments erring at one coordinate is weak
but real evidence that the phase coordinate is where instruments err, so the canon's sentence
for Q should carry the affine form **explicitly** rather than leaving phase to be inferred.

### 5.2 The arithmetic consequence

The biased grid is not closed under exact addition. Exactly: the sum of two points of the
B = S/2 grid lies on the phase-zero grid, never on the biased one (0 of 256 exhaustively), and
its distance to the nearest biased-grid point is exactly S/2, every time
(`56_probes/q2_output.txt`). The algebra behind the measurement is one line, (B + kS) + (B +
jS) = 2B + (k + j)S, which is on the B-phase grid only when B = 0 modulo S; the probe is there
so the line is checked rather than trusted.

Three consequences the law layer must know. On a half-unit-biased format **every addition
adapts**; the identity-adaptation case, adapt(exact) = exact, never occurs for addition. Every
adapted sum is a **tie**, so the round-to-nearest tie rule stops being a corner case and
becomes the policy that decides every addition. And the grid contains neither zero nor one, so
the predecessor's no-representable-one finding reappears as a phase consequence: a biased
numeral is not even a monoid carrier under exact addition, which bears on the register's Q11
option that a numeral names its algebraic structure. A structure-naming mechanism must be able
to say "this numeral has no additive identity and no unit", and phase is one of the parameters
that decides it.

Untested here: multiplication on biased grids (the product of two biased points is a
polynomial in B and lands nowhere simple), and the interaction of a magnitude-dependent slot
function with a per-binade bias, which is what a full half-unit-biased float would need. Both
stated as gaps.

## 6. The encoding axis: order and arithmetic are exclusive for signed sets

### 6.1 Answering the refinement first

`55` refined `08`'s raw-order finding: excess-K encodings are monotone, so raw-order agreement
(and with it memcmp-sortability) is purchasable by encoding for signed sets (`55:409-423`). I
accept the refinement, with one small correction of its framing: `08`'s sentence was
pool-scoped by its own words, "plain unsigned is the only one **of the eight integer-keyed
encodings** where it holds" (`08:263-266`), so no quantifier in `08` needs correcting. What
`55` added is not a repair but the general theorem the pool-scoped sentence could not state:
raw order agrees exactly for the monotone encodings, and monotone signed encodings exist. That
addition is real and I carry it.

### 6.2 The completion: the purchase has a price, and the trade is forced

`56_probes/q3_signed_encoding_trade.rs` adds the second pattern-level property that matters:
whether the plain binary adder on raw patterns implements the value operation, (e(a) + e(b))
mod 16 = e(wrap(a + b)). Exhaustively at 4 bits over the signed set [-8, 7]:

| encoding | raw order agrees | raw adder correct (of 256 pairs) |
|---|---|---|
| two's complement | no | 256 |
| offset binary (excess-8) | yes | 0, and the defect is the constant 8 mod 16, every pair |
| scrambled control | no | 26 |
| unsigned identity, on [0, 15] | yes | 256 |

And the exclusivity is not an accident of the pool. A monotone bijection between two finite
totally ordered sets is unique, namely the sorted correspondence; the probe constructs it and
confirms it is offset binary. So for a signed value set the **only** bijective encoding with
raw-order agreement is excess-K, and excess-K fails the adder property by the constant K.
Therefore no bijective encoding of a signed set has both properties, while the unsigned
identity has both. The trade is forced by signedness (precisely, by K being nonzero in the
unique monotone encoding), and the world's hardware already sits at both ends of it: exponent
fields are biased because floats must sort as integers, and integer ALUs are two's complement
because adds must be free.

### 6.3 What this does to the design questions

It sharpens `55`'s own flagged hazard (`55:271-275`, two strategies over one format disagreeing
on pattern-level properties). The hazard is not hypothetical taste: for signed columns,
"memcmp-sortable" and "hardware-addable" are competing purchases that no single bijective
encoding makes together, so a strategy choosing E is choosing which one the column gets, and a
consumer wanting both on one column needs either a re-encoding pass at the boundary (one
add or xor per element, the constant-defect measurement says exactly what it costs at the
pattern level) or a redundant encoding, which my uniqueness argument does not cover and nobody
has examined. This connects to the register's Q13 (which axes may a build arm move): E is
unobservable in denotation and observable in pattern-level properties, so it sits exactly on
the boundary Q13's classification has to place.

## 7. What `55` said about `08`, answered in full

**The phase concession (2a).** Accepted and repaired; section 5 contains the repair and the
evidence that it costs R nothing. `55`'s statement that the concept is "strictly wider than my
phase-one Q in the phase coordinate" is correct and matches `08`'s finding that the design's
concept is wider than `generic_format` in phase and narrower in exponent (`08:630-634`).

**The raw-order refinement.** Accepted as an extension, with the quantifier note in 6.1, and
completed into the forced trade in 6.2.

**The independence bounding.** `55` bounded our agreement honestly: two independent instruments
reconstructing one shared piece of literature, worth more than a read and less than two cold
derivations (`55:363-369`). I endorse the bound and confirm it from my side: `08` names Flocq
as the source of the canonical exponent and its `fexp`, so the shared-literature discount is
correct at both ends. Where the convergence is genuinely two-directional is the sentence "the
concept is the function, the named shapes are points on it", which `55` reached from the
provable-once test and `08` reached from the design's axis tables; same sentence, two
derivations, one shared literature underneath both.

**The non-finites collision (2b).** `55`'s handling is right: under the scoping theorem only
the escape-codes placement stays live, and it flagged the theorem as carried rather than
beyond question. I add one observation: the machinery escape codes need is the
section-retraction triple the record already adopted for signed zero and decimal cohorts
(`08:272-275`), so the E-level placement has precedent inside the design rather than being an
invention. Still open, correctly.

**Its account of my question one.** `55` notes its phase one landed on the general-function
side of `08`'s question one by derivation (`55:373-378`). Counted honestly per its own
bounding: a second voice with an independent instrument and shared literature, not a second
cold derivation. The register should keep counting it that way.

## 8. Fits and kills against the register

**The wrapping entry (added by `55`).** Fits, and this file amends it twice: the
observational-equivalence deflation (section 4) collapses its three options to two live ones,
and the two-by-two (section 3) gives the surviving pair its mechanism while re-pricing option
one's exception cost upward. Kills nothing in it; the entry's structure survives with fewer
forks.

**Q12.** Strong fit. The coherence law is the window-level form of `42`'s reachability
condition and covers wrap's exactness with the same instrument, so Q12's candidate reframing
("state which properties the arithmetic has") gains a generator: the properties follow from
the reduction map's cell in the two-by-two plus the reachable window. Nothing killed.

**Q5.** Fits the product-of-axes reading, as `55` said, and adds that the overflow axis's
values are heterogeneous **in law kind**, which is `25`'s heterogeneity finding (per the
register) made algebraic. If the axis carries the two-family classification, the
heterogeneity stops being a wording problem.

**Q11.** The biased-grid result (no identity, no unit, nothing closed under exact addition)
is a concrete case the structure-naming option must be able to express. Fits; adds a test
case.

**Q13.** The encoding trade gives E's classification content: unobservable in denotation,
observable in pattern-level properties, with a forced trade for signed sets. Fits the
"state per axis" shape.

**Q10.** Section 2.2's equality gap is Q10's question arriving at the format concept itself:
the model must state whether identity is (D, Q) or the tuple. A voice for the
denotation-decides direction, not a second read of `03`'s predicate.

**Q16.** Unaffected; noted only that everything here uses sense one.

## 9. What I would put to `55` when it is resumed

Stated so each can be answered or refuted concretely.

1. **The dependency form.** Name one member of R that is well-typed before D and Q are fixed.
   If there is none, does "a format is the answer to four questions" survive as the canon
   phrasing, or should the concept be stated as an identity half (D, Q) and a realisation half
   (R-choice, E), per 2.2?

2. **The equality.** Is format equality Q-equality (your `55:47-49`), (D, Q)-equality, or
   tuple equality? Your rejection of alternative C requires the first or second; your tuple
   suggests the third. Pick one and say what R-difference and E-difference mean under it.

3. **The symmetric expulsion.** Signed saturation fails coherence exactly as wrap fails the
   adaptation laws (`56_probes/q1_output.txt`, all four cells). By your own criterion,
   "a set whose members do not share the slot's properties", saturation is expellable from a
   coherence-defined slot. What licenses expelling wrap and retaining saturation, other than
   which family was named first?

4. **The deflation.** Name one observable, any probe, bench, or consumer program, that
   distinguishes wrap-as-domain from wrap-as-composite once the numeral's Q is declared. If
   none exists, do you accept that your register entry's options two and three should be
   marked one semantics with two drafts?

5. **The conversion cost.** Given that declaring Q is choosing the section (as `u8` against
   `i8` does), where precisely does a residual conversion policy survive? Construct the case
   or withdraw the cost.

6. **The locus.** Which of your four choices excludes block floating point? Q's definition
   (`55:47-51`) nowhere says Q is fixed by the type alone, and your 3d excludes the storage
   layer by fiat rather than by the model. Does F need `08`'s locus clause ("fixed by the
   type alone", `08:554-560`) as a rider on Q, and if not, what does the work instead?

7. **Totality against the imitation intent.** Your requirement 2 (`55:237-240`) demands a
   total adaptation and files trap as "the refusal to be total". Under I3, Warm imitates
   Rust, whose debug arms panic on overflow: the partial case is the **default-imitating**
   case, not a corner. Does your model file this as the R-member being selected by the build
   arm, and is that your intended reading of the register's Q13?

## 10. What the register should gain

Reported here; I have edited neither `OPTIONS.md` nor `INTENTS.md`.

The **wrapping entry** should gain three sentences: that options two and three are
observationally equivalent given a declared Q, with the fork between them a drafting choice
that no measurement can settle (section 4); that the slot's members classify along two
independent law families with all four cells inhabited (`56_probes/q1_output.txt`); and that
option one's stated cost was undercounted, since chain-exactness sentences would carry a
saturation exception in the mirror direction.

The **Q12 entry** should gain the coherence law as the window-level form of `42`'s
reachability condition, with the note that it also covers wrapping's exactness, so the
divergence table, the reachability mechanism and the wrap-exactness result are one
classification measured three ways.

A **new line** under the unasked questions: for a signed value set, raw-order agreement and
raw-adder correctness are mutually exclusive over bijective encodings (unique monotone
bijection, constant adder defect K; `56_probes/q3_output.txt`); redundant encodings
unexamined. Bears on the strategy-picks-E hazard, on Q13's classification of E, and on any
sortable-column story the container derivation tells.

The **format-concept material** this topic is converging toward should carry: the dependency
form of the four slots with the equality question stated (section 2); the affine (phase)
parameter inside Q with R untouched (`56_probes/q2_output.txt`); the systematic-tie and
no-identity consequences of nonzero phase, as a Q11 test case; the locus clause as still
doing work no slot of F does; and the note that D is fixed at the rationals by the scoping
theorem, so the D slot's live content in arvo is exactly the wrapping question.

## 11. Coverage, bounded honestly

**What the probes establish.** The two-family two-by-two with all cells inhabited, exhaustive
at 4 bits, one window pair, addition and multiplication for the coherence checks (q1). The
affine membership predicate against enumerations, the adaptation laws of nearest-onto-biased,
and the closure, tie and identity facts, at one grid geometry, step one quarter, bias one
eighth (q2). The order and adder classification, the constant defect, and the
sorted-correspondence uniqueness at 4 bits (q3). Plus the byte-identical reruns of `55`'s
three probes on the pin.

**What is argued and not probed.** Width transfer of all three results (the algebra is
width-generic on its face; no probe ran past 4 bits or a second geometry). The op-set
argument that D must be closed under the vocabulary. The observational equivalence of the
wrap options, which is an argument about transports, checked only through the hom property
the probes verify at 4 bits. The uniqueness of the monotone bijection is proved for
bijections only; **redundant encodings are wholly unexamined** and could conceivably buy
both pattern-level properties at the price of patterns, which nobody has checked.

**What I read and did not.** `55` in full with its probes; `08`; `42` at lines 315 to 349;
the register's entries named in section 8. I did not open `25`, `35`, `18`, `43` or `03`
this dispatch, and every statement about them above is sourced to the register or to `55`
and marked so. The seed sweeps were not reopened.

**What is first-read here and owed a second.** The two-family classification and its
inhabitation (3.1); the symmetric-expulsion argument (3.2); the coherence unification (3.3,
instrument independent, idea first-read); the observational-equivalence deflation (4); the
placement-plus-consequence result for phase (5); the exclusivity theorem for signed
encodings (6.2). None of it is settled by this file, per the standing mode.

**Nothing here is priced.** Every number is a count from a committed probe; no bench harness
run bears on any of it, and the one cost-flavoured remark (one correction op per element to
re-encode) is a pattern-level count, not a measurement.

**What I could not determine.** Whether the canon should state the format concept as four
slots, or as the identity-plus-realisation split of 2.2; that is a drafting choice on which
my derivation gives a preference and no proof. Whether redundant encodings break the
exclusivity of 6.2. Whether a magnitude-dependent slot function composes with a per-binade
bias without disturbing the adaptation laws. And whether coherence over the reachable window
is exactly equivalent to `42`'s per-trajectory condition or strictly coarser on some fold
shape nobody has constructed; I believe coarser, and I could not build the separating case
in this dispatch.
