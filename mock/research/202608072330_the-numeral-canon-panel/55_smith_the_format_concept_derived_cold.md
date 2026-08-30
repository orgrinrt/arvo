# 55. The format concept, derived cold

**Persona:** Julius O. Smith III. Signal processing, fixed-point arithmetic in shipped hardware,
numerical representation for computation.

**Phase one. Written blind.** I have read `INTENTS.md`, `00_brief.md`, and the workspace rules,
and nothing else in this panel. Every claim below is derived from those premises plus my own
subject, and every checkable claim has a committed probe in `55_probes/`. Where I state a result
from the numerical-analysis literature I say so; that is my own knowledge, not a panel read.

## 0. Where I started

I derive the format concept from the thing it must make possible, because that is the only
non-arbitrary anchor available: **arithmetic on represented numbers must be statable.** A format
concept that does not determine what "the correct answer" means for an operation is a storage
concept wearing a number's name. So the derivation runs backwards from the standard model of
machine arithmetic, which I have used my whole working life and which I will state before using.

The standard model (Wilkinson's rounding-error analysis; IEEE 754's correctly rounded
operations; every fixed-point requantisation step in every filter I have shipped) is:

> computed(x op y) = adapt(exact(x op y))

The operation is performed exactly in some ambient number system, and a total map takes the exact
result back onto the set of values the format can hold. Every arithmetic system I know that
deserves the name factors this way, and the factoring is what makes error analysis possible at
all: the error of a computation is the composition of the adaptation errors, and the adaptation
error is a property the format can state (half a quantum, one ulp, zero).

That single sentence forces the shape of the concept, and the rest of this file unpacks what it
forces, what it leaves open, and where its boundaries sit.

## 1. The concept, as the derivation produces it

A **format** is the answer to four questions, and an instance of the concept is a choice of all
four. I write them as a tuple for compactness, not as a proposed Rust shape.

**F = (D, Q, R, E)**

**D, the ambient domain.** Which number system the denoted values live in, with its exact
operations and its order (where it has one). The reals, the integers, the rationals, or a finite
ring. D is where "the exact result" is defined, so without D no operation has a correct answer
and intent I9 ("strategies are the variables that change what the correct answer is") has nothing
to act on. D also carries the algebraic laws: associativity of addition is a fact about D first
and about the format only through the adaptation.

**Q, the representable set.** Which elements of D the format denotes exactly. This is the
format's identity in the strongest sense: two formats with the same Q are the same format wearing
different clothes (probe 3 makes this literal). Q must be given constructively enough that
membership is decidable and the set is countable with known bounds, because the container
derivation needs its cardinality.

**R, the admissible adaptation maps.** The total maps D to Q that the format permits as the
"then round" half of the standard model. Round to nearest with its tie rules, round toward zero,
round up, round down, saturate at the bounds. Each is a retraction onto Q (fixes what is already
representable), and the ones worth admitting are monotone, so that adaptation never reverses an
order relation. R is a set, not a single choice, and that is deliberate: which member is used,
per operation, is exactly the kind of decision the strategy layer owns (section 6).

**E, the encoding.** The relation between elements of Q and bit patterns of some width. E is
part of the concept because the acceptance criterion requires the typestate to derive "the
matching container and numeral representations" from a bits-and-bytes demand, and probe 3 shows
that step is not free: for a fixed pattern budget, different encodings realise different value
sets, different redundancy, and different operational properties. But E is *ordered after* Q: the
value set is chosen, then realised. A concept that lets E participate in the format's identity
gets the equality of formats wrong (two's complement and offset binary would become different
numbers, when they are the same numbers filed differently).

### What an instance determines

An instance of F determines: the exact meaning of every value (the denotation, section 4); the
correct result of every operation up to the choice of adaptation map (the standard model); the
error metric (the quantum function, section 5); the ordering of values (inherited from D through
Q); the cardinality of Q and hence the minimal pattern demand; and, through E, the concrete bit
demand and the pattern-level properties (raw comparability, redundancy, padding).

### What an instance deliberately leaves open

Which adaptation map each operation uses (strategy territory). Which container carries the
patterns, at what alignment, with what padding (container derivation territory; the format
supplies the demand, not the carrier). Which algorithms compute the operations (anything that
lands on the same adapted exact value is conformant, which is precisely the freedom intent I5
needs: a Hot path that lands somewhere *else* is then a stated, bounded deviation rather than a
silent one). And the names: nothing in F presumes any strategy count or naming, which matters
because intent I1 says the strategy set is open.

## 2. Why this is one concept and not a family

The test I would apply is the one a mathematician applies to any proposed unification: **the
generic statements must be provable once, over the parameterization, rather than restated per
instance.** If membership, adaptation correctness, the error metric, ordering and conversion
each need per-instance definitions, the "one concept" is a shared name over a family.

The membership half of that test is checkable today, and probe 1 checks it. The known result
(Flocq's `generic_format`, the Coq formalization used to verify floating-point; I know it from
the literature, not from this panel) is that integers, fixed-point and floating-point are all
instances of one **slot function** phi: for each magnitude, phi says which power of the radix the
quantum sits at, and x is representable exactly when x / beta^phi(x) is an integer (plus range
bounds). The instances:

- integers: phi = 0, constant
- fixed-point with F fractional bits: phi = -F, constant
- scaled integers by beta^k: phi = k, constant
- floating-point with precision p and minimal slot emin: phi(x) = max(emin, e(x) - p), where
  2^(e-1) <= |x| < 2^e; subnormals fall out of the max with no special case

`55_probes/p1_one_membership_predicate.rs` writes that membership predicate **once**, feeds it
the three phi instances, and compares exhaustively against direct textbook enumerations that
never mention phi. All three value sets match exactly (16, 16, 47 values), the subnormal boundary
behaves (1/4 in, 1/8 out), and two mutants (dropping the emin clamp; letting the mantissa reach
2^p in the enumeration) are both detected, so the instrument can fail. One probe, one predicate,
three formats: that is the unification the "one format concept" phrase needs, demonstrated at
model scale.

The adaptation half unifies the same way: round-to-nearest, directed roundings and saturation
are all monotone retractions D to Q, definable once against any Q (probe 2 checks the saturation
case: it is a retraction, distance-minimising, monotone, exactly the profile of a rounding whose
input happens to be out of range). The quantum/ulp metric is beta^phi(x), defined once. Ordering
is inherited from D once. Conversion between two formats over the same D is adaptation and
nothing else: convert = adapt_target(value), one definition. I have probed the first of these
claims and state the rest as derivations; they are the same shape and I did not build five probes
to say one thing five times, but I bound my coverage honestly in section 8.

**Where the one-concept claim strains, and how far it stretches**: section 3.

## 3. Boundaries, with their costs named

### 3a. Wrapping is not an adaptation, and pretending it is one costs the algebra

This is the sharpest boundary my derivation found, and probe 2
(`p2_saturate_is_adaptation_wrap_is_domain.rs`) establishes it exhaustively at 4 bits.

Saturation is an adaptation map: a retraction onto Q, distance-minimising, monotone. It slots
into the standard model as one more member of R, its error is unbounded but statable, and it
costs associativity in the same way any adaptation does (952 signed counterexamples at 4 bits;
and, the honest flip side the probe also checks, unsigned add-only saturation is exactly
associative, so even that loss is conditional and derivable).

Wrapping is not. It is a retraction, but it is not distance-minimising (107 witnesses in the
probe window) and it is not monotone. What it *is*, exactly and exhaustively, is a ring
homomorphism: wrapped arithmetic is **exact arithmetic in Z/2^N**. Associativity holds without
exception. Nothing is being approximated; there is no error to analyse, because there is no
adaptation happening.

So the concept has two coherent ways to hold wrapping, and they are different claims:

- **Wrap as a D.** A wrapping format is F = (Z/2^N, all of it, {identity}, E). One concept,
  covering wrap, at the price that laws attach to D: the canon then says "associativity is
  exact when D is a finite ring, and holds up to adaptation error otherwise" rather than one
  uniform sentence. I consider this price honest, because that IS the mathematical situation.
- **Wrap as an operation policy.** Filing wrap next to saturate in R. This is the C-language
  framing and I think probe 2 refutes it as a matter of structure: wrap fails every property
  the other members of R share, and a "set of adaptation maps" containing one member that is
  not an adaptation is a family with a shared name, inside the concept, at its most
  load-bearing spot.

One further consequence of wrap-as-D that I want on the record because it is a real design
fact: **conversions across D are not adaptations.** Z/2^N does not embed in Z as a ring; going
from a wrapping value to an integer requires choosing a section (which residue), and that choice
is policy, not mathematics. The concept saying so is the concept correctly refusing to make a
choice it cannot ground. Casting out of a wrapped domain being policy-laden is something every
systems programmer has been bitten by; the format concept locating the bite precisely is a
feature.

### 3b. Non-finite values are outside Q and the concept should say where they live

Infinities and NaNs are not elements of any Q the slot function generates. Two coherent
placements: extend D to the affine extension (reals plus two infinities) and let Q include them,
with NaN as a carrier-level escape code under E; or keep D real and treat all non-finites as
E-level codes with defined propagation rules. The cost of excluding them from the concept
entirely is concrete: IEEE interop (an `f32`-shaped format that cannot say what its NaN patterns
denote cannot round-trip foreign data), and saturating semantics at infinity. I could not settle
the placement cold and both options should stay open; what I can say is that the choice is
E-adjacent either way, and does not disturb the finite core that probes 1 and 2 cover.

### 3c. Rationals and arbitrary scales sit at the edge

The slot-function Q generates sets of the form m times beta to a magnitude-dependent power. Two
things it does not generate: **arbitrary rational scales** (a currency format with quantum 1/100
in radix 2) and **stored-pair rationals** (p/q with both components carried). The first fits by
generalising the quantum from beta^phi to an arbitrary positive rational per slot; that is a
mild widening and Flocq itself has the fixed instance in that generality. The second does not
fit at all: the representable set of a pair format is not characterised by any quantum function,
and its arithmetic (exact in Q until the components overflow) has a different failure geometry.

The brief's own framing resolves this cleanly, and I flag it as the reading I favour: *the
primitives are named compositions over one format concept*. A stored-pair rational is a
**composition of two integer formats plus laws**, not a primitive format. Same for
error-carrying pairs (value plus running error bound, which a Precise-flavoured chain per I7
might want), intervals, and complex numbers. The one format concept covers the atoms; the
composition machinery covers the molecules. The cost of this boundary, named: exact rational
arithmetic is not a format instance, so any generic statement proved over F does not
automatically apply to it, and the composition layer owes its own laws.

### 3d. What else is deliberately outside

Strategy (section 6). The container (the format emits a demand; the carrier is derived).
Algorithms. Textual representation and parsing/printing (a denotation question, but for another
panel's file; nothing in F blocks it, since Q is a set of exact values that can be printed
exactly). Random bit patterns that decode to nothing: E must say which patterns are valid, and
validity is part of what the typestate validates per the acceptance criterion.

## 4. Values against bits: the denotation is the format's one non-negotiable sentence

What must a format say about the values, as against the bits? One thing, and everything else
follows from it: **the denotation.** Every valid pattern denotes a stated element of D, the map
is stated, and equality of denotation is decidable. That sentence is what makes the difference
between a numeral and a bag of bits, and it is the sentence that lets two different carriers of
one format agree (the desync-prevention that a typestate exists to buy: two realizations of one
format that disagree at one value are two formats, and the type system should make that
unrepresentable).

Probe 3 (`p3_encoding_is_a_separate_axis.rs`) pins the value/bit split exhaustively at 4 bits:

- Two's complement and offset binary denote the **same** 16 values through **different**
  pattern maps. The format (Q) is the invariant; the encoding is the realization.
- Sign-magnitude, same 16 patterns, denotes only 15 values, misses -8, and spends two patterns
  on zero. Encoding choice changes what a bit budget buys. So the bit-demand function is
  demand(Q, E), not demand(Q), and the acceptance criterion's derivation order is forced:
  usage demand, then Q, then E, then container. The criterion's plural, "container and numeral
  **representations**", is load-bearing and my derivation lands on the same plural from the
  other side.
- Raw unsigned comparison of patterns agrees with the value order for offset binary and
  disagrees for the other two. Operational properties (comparability by raw compare, and by
  extension memcmp-sortability, packability of monotone scans) live on E, not on Q. A strategy
  that cares about them is choosing an E, which is another reason E belongs inside the concept
  rather than being left to the container: the choice is observable in behaviour, not just in
  layout.

## 5. What arithmetic needs from the concept, stated as requirements

For arithmetic on a format to be statable at all, the concept must supply, per instance:

1. **The exact operations**, from D. Without them "correct" is undefined.
2. **A total adaptation**, from R. Without totality some exact results have no computed
   counterpart and the operation is partial in a way the type cannot see. (Saturate is total by
   construction; round-to-nearest is total once the bounds question is answered by either a
   widening contract or an overflow member of R; "trap" is the refusal to be total, and if the
   design wants it, it is honest to model it as a partial adaptation and let the fallibility
   ladder carry the partiality.)
3. **The quantum metric**, beta^phi(x), so that accuracy claims are statable. Intent I7 wants
   Precise to be "accurate within chains, not only alone"; a chain accuracy claim is a sum of
   per-step adaptation errors measured in this metric, so the metric is the precondition for the
   intent to be checkable rather than vibes. Same for I5: "provable meaningful gains" at the
   cost of accuracy requires the accuracy loss to be a number, and the quantum is its unit.
4. **The order**, inherited from D, so comparisons, clamps, min/max and monotonicity statements
   are generic. (In a finite-ring D there is no arithmetic-compatible order, and the concept
   saying so is again a feature: ordering a wrapped value is an E-level or policy-level act,
   which matches what probe 3 found about raw comparison.)

Everything in that list is per-format data, statable once over the parameterization. That is the
sense in which the algebraic laws "underneath" the brief's one-line architecture derive: each law
is proved in D, then transported through the adaptation with an error term, and the error term
is zero exactly when the adaptation was the identity on the operands involved.

## 6. Where strategy attaches, kept open on purpose

I1 says the strategy set is open, so I derived the concept to be strategy-count-agnostic, and I
note where the seam naturally falls rather than what sits on each side of it.

In this shape, a strategy is **a policy over the format's open choices**: which member of R each
operation uses, which E realises Q, which container carries E, which algorithm computes the
operation, and how much deviation from the adapted exact value is tolerable (for an intent like
I5's Hot, a stated error budget in the quantum metric; for I7's Precise, a chain-level budget).
The format defines the space of correct answers; the strategy picks the point, which is I9
almost verbatim, and I8's "they weigh different measurements differently" reads naturally as:
the measurements are taken in the metrics the format supplies (error in quanta, bits of demand,
operations' cost), and the weighting is the strategy. None of this presumes four strategies, or
any fixed number, which is what the openness of I1 requires of a format concept.

I flag one consequence for later files to attack: if a strategy may pick E (probe 3 shows E is
behaviourally observable), then two strategies over one format can disagree on pattern-level
properties while agreeing on every denotation. Whether that is a feature (Cold packs, Hot pads)
or a hazard (two views of one column disagree about raw order) is exactly a question about what
the typestate must keep aligned, and I could not settle it cold.

## 7. Alternative decompositions, and what would distinguish them

Explore mode, so the shape in section 1 is my derivation's output, not a ruling. The live
alternatives I can construct and what would decide among them:

**Alternative B: format = Q only.** The representable set is the identity; D, R, E all live
outside (in the operation layer, the strategy layer, the container layer). Slimmer, and closer
to how "format" is used colloquially. What it costs: arithmetic is not statable from the format
alone (requirement list in section 5 goes unmet locally), so every law needs a second concept
to quantify over, and the canon's "derived algebraic laws underneath" hang from two hooks
instead of one. Distinguisher: try to state "round-to-nearest is correct for this format" in
each decomposition; in B the sentence needs imports, in F it does not.

**Alternative C: format = the denotation function, encoding-first.** Identity is the map from
patterns to values; Q is its image. Covers everything a machine can hold, including redundant
and escape-coded patterns, with no widening needed for NaN. What it costs: format equality
becomes pattern-map equality, so two's complement and offset binary become different formats
(probe 3 check 1 is the direct counterexample to that being the right equality), and every
value-level theorem is stated up to an encoding isomorphism that the concept then has to keep
quotienting away. Distinguisher: ask which artifact the container derivation consumes; if the
answer is "the value set's cardinality first, the pattern map second", the identity is
value-level and C has the layers inverted.

**Alternative A-wide: F as in section 1 but with wrap filed in R.** Probe 2 is my argument that
this misfiles an algebra as an error policy. What would overturn it: a demonstration that some
generic statement the canon needs is *simpler* with wrap-in-R than with wrap-as-D, by enough to
pay for the lost uniformity of R's shared properties. I could not construct one.

A fourth answer nobody has written down stays admissible; the brief says so and I have no
standing to close the list.

## 8. Coverage, bounded honestly

What the probes establish: the one-membership-predicate unification of integer, fixed and float
value sets, exhaustively, at model parameters (5-bit scale, p=3 float), with a validated
instrument (P1). The adaptation/domain boundary between saturate and wrap, exhaustively at 4
bits signed and unsigned, instrument validated (P2). The value/bit split and its three
consequences, exhaustively at 4 bits over three encodings, instrument validated (P3).

What is derived but not probed: the once-over-the-parameterization statability of adaptation
correctness, quantum, order and conversion (section 2's second half); the composition reading of
rationals and pairs (3c); everything in section 6. What is stated from literature: the Flocq
`generic_format` unification, which P1 independently reconstructs at model scale but which I
name as prior art because it is one and I knew it.

What I could not determine cold: the placement of non-finites (3b); whether strategy may choose
E per use or E is fixed per format instance (6); whether the concept needs the arbitrary
rational quantum generalisation now or as a later widening (3c); and how many outputs the
derivation has, beyond noting that my derivation independently produces at least the two the
acceptance criterion's plural names, in the order Q then E then container.

Three probes, independent in what they attack (membership, adaptation, encoding) and sharing no
model beyond exact integer arithmetic. That meets the three-instances bar for the claims they
cover and does not extend it to the claims they do not.

---

# Phase two: reconciliation, written after reading the panel

**Marked and appended per the dispatch. Phase one above is untouched.** Read for this phase, in
order: `08` in full, then `32`, `34`, `36`, `37`, `38`, `39`, then `OPTIONS.md` in full. Nothing
else: not `RULES.md` beyond what phase one permitted, not the seed, not `DROPLIST.md`, not any
other numbered file, so every claim below about "the panel" is bounded to that set.

## The verdict first

**The core of phase one survives contact with the panel unchanged**: the format concept is one
parameterization; its identity is value-level; membership is one predicate through a slot
function; the encoding is a second, ordered, behaviourally observable axis; and arithmetic is
statable as exact-in-an-ambient-domain composed with a total adaptation. Reading the panel did
not move any of that, and saying so is the result this dispatch exists to produce.

**Three things in phase one are wrong or too narrow**, conceded below with what corrects them.
**One claim in `08` gets a refinement** from my probe 3. **And one question my phase one raises
is genuinely absent from the register**, written out in full at the end.

## 1. Where I converge with `08`, and how independent the convergence really is

`08`'s central object is the canonical exponent: the value map `Adjustment * radix^exponent * k +
Bias` (`08:179-186`), with the family named by the shape of the exponent function over binades,
and `ExponentForm`'s two instances exposed as a two-point sample of a function space
(`08:194-204`). My phase-one Q is the same object: the slot function phi, membership stated once
as `x / 2^phi(x)` integral plus bounds, with fixed, integer and float as instances (`p1`, 47 of
47 float values, both mutants detected).

**The independence of that agreement is limited, and I bound it rather than bank it.** Both of us
derive from Flocq's `generic_format`; I named it as prior art in phase one before reading anything,
and `08` names it too. So this is two independent *instruments* (my membership predicate against
`08`'s classifier `i1b`) reconstructing one shared piece of literature at model scale. Under the
panel's own independence discipline that is worth more than a read and less than two cold
derivations from nothing: the instruments are independent, the idea is not. Where the
convergence IS fully independent: I derived "the concept is the function, the named shapes are
points on it" from the one-concept test (generic statements provable once) with no access to
`ExponentForm`, and `08` derived it from surveying the design's axes. Same sentence, two
directions.

`08` then goes far past anything I built: the taper and segmented shapes, the meet and join
closure results, the boundary's three clauses (`08:554-560`), and the gate-free segmented
typestate probe. None of that conflicts with phase one; my concept's Q as a general slot
function is exactly the "canonical exponent as a member" reading `08` puts to op as its question
one (`08:709-715`), and I note that my phase one landed on the general-function side by
derivation rather than by preference, for what one more arrival is worth.

## 2. What phase one got wrong or too narrow

**2a. Phase. Conceded.** My slot-function Q bakes in phase zero: membership as "x over the
quantum is an integer" cannot say a grid is offset by half a step. The record's value map is
affine, with `Bias` a separate parameter, and `08` measured that a half-unit-biased format is
inside the design's concept and outside plain `generic_format`, a distinction its own first
instrument got wrong in exactly the way mine would have (`08:630-634`). So the concept is
**strictly wider than my phase-one Q in the phase coordinate**: Q needs the phase parameter, and
my "two formats with the same Q are the same format" claim survives only with phase inside Q.
My probe 1 remains correct for what it tested and tested a phase-zero slice.

**2b. Non-finites. Phase one's option (a) collides with a carried two-expert result.** I offered
"extend D to the affine extension and let Q include infinities" as one of two placements. The
record carries a scoping theorem, quoted at `08:113-117`: every arvo value is `m * r^q`, so every
arvo value set is a finite set of rationals. Under that, infinities and NaNs cannot be elements
of any Q, and only my second placement (non-finites as encoding-level escape codes with stated
propagation, outside the value set) remains live inside the record's boundary. I flag rather than
concede fully: the scoping theorem is carried at TWO EXPERTS and predates op's explore mode, and
I have not read its derivation, so I state the collision and its resolution direction without
treating the theorem as beyond question.

**2c. My "one concept" test was right but under-instrumented next to `08`'s.** I proposed
"generic statements provable once" and probed membership only. `08` ran the concept against
twenty-one representations with a mechanical classifier and found no rival parameterisation
(`08:377-384`), which is the strong form of the test I stated: not only are the named instances
one concept, nothing surveyed needs a second one. My phase one's derived-but-unprobed list
(adaptation, quantum, order, conversion each statable once) stays derived-but-unprobed; nothing
in the panel discharges it either, and it is still owed.

## 3. One refinement to `08`, from probe 3

`08` section 2.2 reports, from its `i3` instrument: "plain unsigned is the only one of the eight
integer-keyed encodings where [raw-order agreement] holds" (`08:263-266`). My `p3` measured
offset binary (excess-8) at 4 bits: **bijective onto the same value set as two's complement, and
raw-order agreement holds** (`p3_output.txt`: "raw compare matches value order: offset true").
So "only plain unsigned" is a fact about `08`'s pool, which apparently does not contain the
excess-K encodings, not a fact about encodings. The general statement is that raw-order
agreement holds for exactly the monotone encodings, and excess-K is the classical monotone
signed encoding; it is why IEEE 754 biases its exponent field, so that floats compare as
integers. This matters to the design because it means the operational property "sortable by raw
compare" is *purchasable by encoding choice* for signed value sets, not forfeited by signedness:
a strategy that wants memcmp-sortable columns can have them at the cost of the two's-complement
convention. Offered as a refinement, one probe, one width, and `08`'s claim corrected only in
its quantifier.

## 4. Fits and kills against the register, per its method

**Q4, what a datum stands for.** My framework fits the **point** reading plus adaptation-error
semantics: saturation stays an adaptation map with a point denotation and an error unbounded
above the bound, which is consistent with `18`'s measured result (absorbing sound only while the
computation stays at the endpoint) without needing the absorbing denotation at all: the
unsoundness `18` counts is, in my terms, adaptation error being consumed as if it were zero. It
fits the **set-admitted-generally** reading as a composition rather than a format instance
(phase one 3c), which agrees with the register's own note that intervals cost the order-and-law
layer. It kills nothing in Q4.

**Q5, one axis or two, and Q12's candidate reframing.** Probe 2 bears here and it is my main
non-`08` contribution. Wrap and saturate are not two values of one kind of axis: saturate is an
adaptation map (retraction, distance-minimising, monotone), wrap is exact arithmetic in a
different ambient ring and fails every adaptation property (`p2`, exhaustive at 4 bits). This
**fits well** the product-of-axes reading of Q5 and fits the axis-heterogeneity evidence `25`
already found (widen-op-narrow answering a different question than wrap and clamp). And it fits
Q12's closing reframing ("state per strategy which properties the arithmetic has") better than
any policy-label framing: under wrap the laws are exact group laws, under saturate they are
conditional on trajectory, and `42`'s reachability mechanism (associativity survives exactly
when the trajectory cannot reach both clamped endpoints) is independently corroborated by my
probe's honest flip side, unsigned add-only saturation exactly associative because its floor is
unreachable. Two instruments, two authors, same conditional, arrived at separately: that one is
a genuine independent convergence, not a read.

**Q10, the singleton-grid amendment.** My phase-one identity claim, format equality is denotation
equality (two formats with the same Q are one format), lands on the same side as Q10's first
option (decide inclusion on denotation rather than declaration) by derivation rather than by
examining `03`'s predicate. That is a second voice for the direction, not the second read `03`
asked for; the predicate itself I have not examined.

**Q16, which sense of composition.** Phase one used sense two throughout section 3c (pairs,
intervals, complex as compositions over numerals) without knowing the collision existed. The
brief's sentence uses sense one. My material is unaffected by which name wins, and I note that
3c needs the sense-two word whatever it ends up being.

**Q11 and the fold layer.** My section 5's requirement 3 (the quantum metric as the unit in
which accuracy claims are statable) is what Q11's "numeral names its algebraic structure" and
Q12's per-strategy law statements both consume. Fits; adds nothing new.

## 5. The absent question, written in full for the register

**Where does wrapping live: adaptation, ambient domain, or a third thing?** Proposed by this
file, from `55_probes/p2_saturate_is_adaptation_wrap_is_domain.rs`, exhaustive at 4 bits with a
validated instrument.

The register's Q5, Q6 and Q12 all treat wrap as a value of an overflow-policy axis beside
saturate. The probe establishes that the two have different mathematical kinds: saturate is a
monotone, distance-minimising retraction onto the representable set (an adaptation, the same
kind as a rounding), while wrap is neither distance-minimising nor monotone over the integers
and is instead exact ring arithmetic in Z/2^N (hom property and associativity exhaustive, zero
failures). Three options, none settled:

- **Wrap is an adaptation-slot value anyway.** The current implied filing. Keeps one axis
  vocabulary. Costs: the axis's shared properties (monotone, error-bounded-per-step, law
  transport with error terms) hold for every value except one, so every generic sentence over
  the axis carries a wrap exception, and the error metric is meaningless for wrap.
- **Wrap is a change of ambient domain.** A wrapping numeral denotes residues, its arithmetic is
  exact, its laws are group laws with no error term, and it has no meaningful order or quantum.
  Buys: law statements become exact where they are exact (which `35`'s measured 0% divergence
  for wrapping folds already shows operationally) and conditional only where adaptation exists.
  Costs: collides with the scoping theorem's "every arvo value is a rational" unless a residue
  is read as denoting a rational representative, and makes conversion out of a wrapped numeral
  visibly policy-laden (no ring embedding of Z/2^N into Z exists; a section must be chosen),
  which is honest but is a new sentence the canon must carry.
- **Wrap is a composite operation, not a format property at all.** "Add then reduce" as one
  named operation on an integer-denoting numeral, with reduction a stated non-adaptation map.
  Keeps the value set rational and the format concept clean. Costs: the exactness of wrapped
  chains is then a theorem about a composite rather than a law of a domain, and the strategy
  layer needs a way to say "this strategy's add is the composite" per `34`'s Hot licence.

What would distinguish them: whether the canon wants `35`'s and `42`'s associativity results
stated as laws of a domain (option two), exceptions on an axis (option one), or theorems about a
named composite (option three); and what the ordering and conversion story for wrapped values
must be in each. The distinction is consequential for Q12's reduction-order options, because
"exactly reassociable" is a domain fact under option two and a per-policy measurement under
option one.

## 6. Coverage of this phase, bounded

I read the files the dispatch named and nothing else, so: I have not verified `08`'s probes
beyond reading its claims; I have not read `03`, `18`, `25`, `35`, `42`, `43` or any other
numbered file except through `OPTIONS.md`'s and `08`'s accounts of them, and every fit/kill
statement above that touches those files is made against the register's account, not the
source. The two-expert rule applies to everything here: my convergences with `08` are first-plus-
instrument, my Q10 voice is not a second read, and the new option is ONE EXPERT until someone
attacks the probe. Reading the panel changed phase one in the two conceded places and the one
flagged collision, and nowhere else.
