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
