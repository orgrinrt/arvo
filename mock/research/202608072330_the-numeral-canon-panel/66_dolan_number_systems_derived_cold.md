# Number systems as a canon-level concept, derived cold

This file is written under the panel's cold derivation protocol. Phase one below is derived only
from `INTENTS.md`, the workspace and arvo rule files, and op's acceptance criterion as quoted in the
dispatch: "have the typestate derive the matching container and numeral representations, then
validate, and erase." No panel file, option register, droplist, probe, git log, or commit message in
this directory was read before phase one was written and committed. Phase two, appended after that
commit, reconciles against the panel.

Three small probes back the claims below and are committed alongside this file in `66_probes/`:
`associativity_check.py` (does overflow policy change the algebraic structure), a similar
`redundant_representation_demo.py` (is one value having several encodings a defect or a tool),
`fixed_vs_float_generic_format.py` (are fixed and floating point one scheme or two), and a fourth,
`derive_validate_erase_pipeline.rs`, a compiled Rust sketch establishing that the pipeline op describes
is expressible under the pinned toolchain without any forbidden feature.

## The two words in the acceptance criterion already carry most of the answer

Op's sentence uses two different words for two different things, and the difference between them is
not decorative. It is the classical distinction between a **number** and a **numeral**: a number is
the abstract mathematical object; a numeral is a symbol, or a string of symbols, that denotes it. "5",
"V", and "101" in binary are three different numerals for one number. The same distinction gives
"number system" (the reals, the integers, the rationals, each as a structured set with its own
operations) its usual meaning, as opposed to "numeral system" (positional decimal, Roman numerals,
two's complement, IEEE 754), which names a scheme for writing numerals down.

Op writes "container" and "numeral representations" as two separate derived outputs, and both nouns
are load-bearing per the dispatch. Read literally against the classical distinction: **container** is
where the bits live (physical storage: width, alignment, layout). **Numeral representation** is how a
value is written into those bits (the encoding rule and its parameters). Neither is the **number
system** itself, which is the abstract structure being represented. The dispatch's own probing
question, "what distinguishes a number system from a representation of one," is asking for exactly
this classical distinction to be operationalized for arvo, and the vocabulary op already used answers
half of it without needing anything further derived.

## Five levels, not one

Unpacking "number system" into a working hierarchy, from most abstract to most concrete:

1. **Number.** An abstract value: the rational one tenth, the integer negative three, the boolean
   true.
2. **Number system.** A set of numbers together with an operation signature and the laws those
   operations obey over that set. "Unsigned dyadic rationals bounded to [0, 8) at granularity 1/32,
   under wrapping addition and multiplication" names one number system. Changing only the overflow
   policy on the same set, to saturating instead of wrapping, names a different one; see the next
   section for why that is not a loose way of speaking.
3. **Numeral representation scheme.** A general rule for mapping members of a number system's value
   set into bit patterns of some width: positional two's complement, IEEE-shaped mantissa-and-exponent,
   a residue number system over a chosen set of moduli, a signed-digit scheme over a chosen digit set.
   The scheme is parametrised; it is not yet a concrete type.
4. **Format.** A representation scheme with every parameter pinned: a specific integer/fraction bit
   split, a specific exponent and mantissa width, a specific modulus set. This is what a concrete arvo
   type such as `UFixed<3, 5, Hot>` names: a number system, a chosen representation scheme, and the
   scheme's parameters, all fixed.
5. **Container.** The physical storage a format's numerals occupy: bit width, alignment, byte layout.
   A format's logical numeral meaning does not change if the same bits are housed in a wider, padded,
   or differently aligned container; container is a separate axis from format, chosen for storage or
   SIMD-lane reasons rather than for numeral-meaning reasons.

This is not invented for the occasion. It falls directly out of arvo's own architecture as stated in
its own rules, read as a premise rather than as evidence from the panel: `Bits<N, S>` in `arvo-storage`
is exactly level 5 (container: opaque storage, strategy-dispatched), and `UFixed<I, F, S>` /
`IFixed<I, F, S>` in the facade crate are `repr(transparent)` newtypes layered directly over a `Bits`
container, which is exactly level 4 (format: a number system, a scheme, and its parameters, wrapping a
container). The hierarchy derived purely from op's wording predicts the layering the existing crate
table already describes. That is a genuine cross-check, not a coincidence manufactured to fit; it
comes from arvo's own `.claude/CLAUDE.md`, one of the premises this file was allowed to read, not from
the panel.

## Identity: value set, operations, encoding, or a combination

The dispatch asks directly whether a system's identity is its value set, its operations, its
encoding, or some combination. The five-level hierarchy above already implies the encoding
(representation scheme, format, container) is not identity-determining for the number system itself;
it is a separate, downstream choice about how the system's values get written down. What remains is
whether identity is the value set alone, or the value set together with the operations and their laws.

`66_probes/associativity_check.py` checks this directly rather than arguing it. It takes one value
set, the integers -8 through 8 inclusive, and two overflow policies over addition on that same set:
wrapping (modular) and saturating (clamped). Exhaustive search over all 4,913 ordered triples in that
range finds wrapping addition associative everywhere checked (a genuine group), and saturating
addition failing associativity on 1,152 of those 4,913 triples, 23.5 percent of the space, with the
smallest counterexample `((-8 + -8) + 1)` saturating to -7 against `(-8 + (-8 + 1))` saturating to -8.

Same value set, two different algebraic structures. If a "number system" is anything with a checkable
identity beyond a bag of representable values, wrapping-add-over-{-8..8} and saturating-add-over-{-8..8}
are two different number systems, not one system with two cosmetic overflow flavors. This gives a
direct, derivable answer to the dispatch's question: **identity is the value set together with the
operations and their laws.** Encoding is not part of it. Two number systems can share a value set and
differ, and two number systems can share an encoding scheme and differ.

This also settles, independently and before any panel material was read, a genuine content question
in op's own words: I9 states "the strategies aren't orthogonal to the threaded question you had, or
its answer, strategies are the variables that change what the 'correct' answer is for what we choose
as the path." If overflow policy is part of what fixes a number system's operations and laws, and
strategy is what selects overflow policy (among other things), then strategy is not merely tuning
storage or speed around a fixed mathematical answer. It is choosing which number system, in the strict
sense above, a computation is happening in. "Correct" then means correct with respect to that system's
own laws, and a wrapping result and a saturating result on the same inputs can both be correct, each
within its own system, with neither one being a degraded approximation of the other. That is exactly
what I9 states in its own words, arrived at here from an exhaustive arithmetic check rather than from
reading it.

## What op's pipeline actually asks the canon to specify

Putting the hierarchy and the identity result together, op's acceptance criterion decodes into a
four-stage pipeline, stated once and cleanly:

1. A consumer names a **shape** (the value-set parameters: integer/fraction split, sign, or an
   exponent/mantissa split, or whatever a future system needs) and a **strategy** (the axis that
   weighs measurements differently, per I8).
2. Shape and strategy together determine a concrete **number system**: not just the value set, but
   the operations and the laws they obey, since strategy supplies the overflow and rounding policy
   that the associativity probe shows is identity-determining.
3. The typestate **derives** the matching container and the numeral representation(s) for that system:
   which scheme, which parameters, which physical storage.
4. The typestate **validates** that a given container's bits are a legal numeral under the derived
   representation, then **erases**, discarding the compile-time proof and leaving only the container's
   bits behind, self-sufficient and zero-cost.

Whether the "number system" in stage 2 should be read as determined jointly by shape and strategy
(overflow policy included), or whether shape alone should be called "the number system" with strategy
treated as choosing among several law-sets a single system can be reasoned under, is a genuine fork
this file does not resolve. Op's own I9 leans toward the first reading. Both are carried forward
because the dispatch's explore-do-not-settle instruction applies here directly, and because nothing in
the premises forces a choice between them; either can be made to work mechanically, as the Rust probe
below demonstrates for the joint reading.

## What erasure requires of an admissible system

Stage 4 is not free. For erasure to be sound, validity must be **decidable from the container's bits
and the static type parameters alone**, with nothing else consulted, because once the typestate is
erased there is nothing left to re-check against and no runtime tag left to carry extra context. This
gives a genuine, canon-usable admissibility criterion, derived from the wording of the acceptance
criterion itself rather than assumed: a number system is admissible for arvo's typestate discipline
only if it has a **decidable, self-contained validity predicate** over its container's bits, one that
needs no environmental or runtime state beyond the type.

This rules some things out, and it rules them out for a reason internal to the pipeline, not by
appeal to taste. A representation whose legality depends on a locale, a runtime configuration flag, or
any other state outside the container's bits and the static type cannot be soundly erased under this
discipline, because there is nothing left after erasure to consult that state against.

A second, independent admissibility filter comes from arvo's own architectural constraint, read from
the premises rather than assumed: `#![no_std]`, no `alloc`, and sizes const at the type level, stated
plainly in arvo's own `.claude/CLAUDE.md` ("Sizes are const. No runtime grow"). This bounds
representability to schemes whose containers are finite and fixed at compile time. Unbounded exact
rational arithmetic (arbitrary-denominator fractions), arbitrary-precision integers, and continued
fractions of unbounded depth are excluded as core primitives by this constraint alone, independent of
any numeral-theoretic argument. A **bounded** approximation of any of those (a fixed number of
continued-fraction convergents, a fixed-denominator rational) remains admissible, because it fits in a
const-sized container with a decidable validity predicate; the unboundedness, not the underlying
mathematics, is what the architecture forecloses.

## Representations, plural: redundancy is a tool op's own intent needs

Op's word is "representations," plural, and the dispatch flags this as load-bearing. The most direct
reading, given the hierarchy above, is that the same number system, under one strategy, can have more
than one admissible numeral representation, and the derivation step legitimately picks among them
depending on where in a computation the value sits.

`66_probes/redundant_representation_demo.py` establishes, by exhaustive construction over a small
width, that this is not a hypothetical concern invented to justify the plural. Canonical unsigned
binary (digit set matching the base) is a bijection between digit strings and values, by construction:
16 strings, 16 values, zero redundancy. Signed-digit binary with digit set {-1, 0, 1} over the same
width is deliberately non-bijective: 81 digit strings collapse onto 31 distinct values, with 22 of
those values reachable by more than one valid encoding (the value -1, for instance, has at least four
distinct four-digit encodings in the width checked). This redundancy is not a defect anyone would
design away. It is the standard technique, used in real adder and multiplier hardware, for breaking
carry-propagation chains: a redundant representation lets digit-position combination happen in
constant depth instead of depth proportional to width, at the cost of an eventual canonicalization
pass.

This is directly relevant to I7: "Precise on the other hand is the one that sacrifices as much
performance and efficiency as makes sense, to be the most precise possible answer... especially within
chains and ops, not only alone." Being accurate across a chain of operations, not only per operation,
is exactly the property that classical numerical analysis buys with a deliberately redundant
in-flight representation: compensated summation (Kahan's algorithm and the TwoSum/TwoProduct
error-free transformations), double-double or multi-component floating point (representing one real
value as an unevaluated sum of several floats to carry more precision than any single float's width
would hold), and interval arithmetic (carrying a value as a bounding pair rather than a point) are all
cases where the value in flight, mid-chain, is represented with strictly more information than the
value at rest, boundary-crossing, or in storage. Canonicalizing (rounding, renormalizing, collapsing
to a point estimate) only happens when the value crosses back out to a caller or into storage.

Read this way, the plural "representations" is not describing an implementation detail to be chosen
once and forgotten. It is naming a real, load-bearing distinction the canon should keep separate: a
**working representation** (what a value looks like while it is being accumulated inside a chain of
operations) and a **storage representation** (what a value looks like when it is at rest, crossing a
type or API boundary). A single logical value legitimately having both, and them not being bit-
identical, is precisely the mechanism by which Precise can deliver on I7's "especially within chains
and ops, not only alone," and by which Cold's tight packing (I6) need not be the representation an
in-flight accumulator under a different strategy is forced to use. Redundancy, under this reading, is
not a defect to be designed out of the system. It is one of the tools the strategy axis has to work
with, and ruling it out a priori would foreclose the mechanism Precise's stated intent most plausibly
needs.

## Fixed point and floating point: one scheme or two

`66_probes/fixed_vs_float_generic_format.py` encodes the same four target values (0.1, 3.75, 1000.25,
and 2^-10) under two toy schemes at a matched twelve-bit budget: a fixed-point scheme with an
eight-bit fraction, and a floating-point scheme with a four-bit exponent and eight-bit mantissa. Both
schemes reconstruct a value as `mantissa * base ^ exponent`. The only structural difference the probe
exercises is where the exponent lives: a compile-time constant, never present in the bits, for fixed
point; a per-value field, stored inside the encoding, for floating point. On the small-magnitude
target 2^-10, which sits below the fixed scheme's constant step of 2^-8, the fixed encoding rounds it
to zero (absolute error 9.77e-4) while the floating encoding, whose exponent adapts to the value's own
magnitude, represents it exactly (absolute error 0).

This is not a novel observation; it matches Flocq's `generic_format`, the standard formalization used
in verified numerical code, where a format is a base, a mantissa bound, and an exponent function
`fexp(e)`, with a constant exponent function giving fixed point and `fexp(e) = e - p` giving floating
point. The probe exists to check this concretely rather than assert it from memory, and it holds.

Whether this means fixed point and floating point are, at the canon level, **one number system**
(dyadic rationals, parametrised by where the exponent function lives) or **two** is a genuine fork,
and the shared shape alone does not settle it. Two readings, both defensible, carried forward rather
than resolved:

**Reading A, unification.** Both are instances of one general scheme: bounded-mantissa dyadic
rationals under a chosen exponent function. Fixed and floating are two points on a continuum (constant
exponent function versus value-tracking exponent function), and a future block-floating-point scheme
(a group of values sharing one shared, periodically-rescaled exponent, common in DSP work) would be a
third point on the same continuum rather than a new special case. This reading gives the canon a
single unifying rule instead of an enumerated list, matching the style of the strategy-axis discussion
elsewhere in the premises.

**Reading B, genuine separation.** The identity result above says a number system is its value set
together with its operations and their laws, not merely its representable range. Floating-point
rounding is relative-error-bounded and its associativity failures have a different shape and a
different practical impact than fixed point's absolute-error, range-bounded rounding; a chain of fixed
adds accumulates a bounded absolute drift, while a chain of float adds accumulates a magnitude-relative
drift whose worst case depends on the operand history in a way fixed point's does not. If downstream
reasoning (in hilavitkutin or vehje, both named in I11 as consumers of "contracts for things that
compose to bigger units than just numerals alone") ever needs to reason generically about, say,
whether a chain of operations is associativity-safe to reorder, the two schemes may need to answer
that question differently even though both are dyadic-rational encodings under the same generic
format.

Which reading the canon adopts changes how much of the type-level machinery a fixed/float unification
can share (Reading A argues for one parametrised family; Reading B argues for two families that
happen to share a formalization but not a law set) without changing anything about how either scheme
is encoded. Nothing in the premises forces a choice, and this file does not make one.

## Is the set of number systems open or closed

I1 records op demoting the strategy set from "closed at exactly four" to explicitly open, within hours
of that closure being questioned, on the grounds that the four named strategies are "a prior attempt
at the intent, not the intent" itself. I11 states arvo is "a library, not a program," whose value is
what composes on top of it for consumers whose needs are not fully known in advance. Read together,
these point toward the same answer for number systems that I1 gives for strategies: the **concrete set
of number systems arvo will ever host cannot be closed**, because a foundation library serving unknown
downstream consumers over an unbounded future has no principled way to enumerate every value set and
law combination a future consumer might legitimately need.

What can and should be closed is not the set of systems, but the **contract** a new system must
satisfy to plug into the pipeline described above: a decidable, self-contained validity predicate over
a const-sized container; a derivation function from (shape, strategy) to (container, representation);
and, per the section above, honesty about which representation is canonical (bijective) versus
redundant, and about where the exponent or scale information lives. A small number of orthogonal
parameters, closed and well understood (sign, an exponent-placement axis, a width/precision split, an
overflow-and-rounding law, and, if the canon wants base-generality rather than binary-only, a base),
generate an open-ended family of concrete systems by instantiation, the same way arvo's own
architecture already treats `UFixed<I, F, S>` as one parametrised family rather than a fixed
enumeration of named integer-like types.

This mirrors, almost exactly, the shape of I1's correction: not "the set is closed at N named things,"
but "the axes are closed and well understood; the instantiations are open." Applying the same move
here rather than inventing a separate rule for number systems is the smaller, more economical answer
and is offered as such rather than as a settled conclusion.

## Bool, masks, and the scope of "number system"

Applying the identity definition strictly (a value set together with an operation signature and laws)
to `Bool` and to bit masks produces an uncomfortable but genuine result: a boolean under AND/OR/XOR/NOT
is the two-element field GF(2) (XOR as addition, AND as multiplication), and a fixed-width bit mask
under bitwise operations is the vector space GF(2)^N. Both are legitimate algebraic structures under
the strict definition above, even though almost nobody colloquially calls a boolean or a bitmask "a
number."

Both also already go through the same discipline this file derives for numeric formats: arvo's own
architecture, read from the premises, dispatches `Bool` and `Bits<N, S>` through the same strategy-
parametrised container machinery that `UFixed`/`IFixed` use, and the same derive-validate-erase shape
applies to them without modification. Whether the canon should therefore call `Bool` and masks "number
systems" (broadening the term to cover any typed, strategy-derived container with an algebraic
structure), or should keep "number system" narrow (ordered value sets with a notion of magnitude) and
introduce a sibling term for the unordered, Boolean-lattice case, is a naming and scoping question this
file has not found grounds in the premises to settle. Both choices are internally consistent; the
narrow reading keeps "number system" intuitive at the cost of needing a second term for something that
uses the identical mechanism; the broad reading keeps one term for one mechanism at the cost of
stretching "number" to cover things nobody would call numbers. Carried forward as open.

## Platform-width types

`USize` and `Cap`, named in arvo's own crate table, resolve their width from the target rather than
from a type parameter the consumer names. Under the shape family derived above (integer bits, fraction
bits, sign), a platform-width unsigned integer looks like the degenerate case fraction-bits equals
zero, integer-bits equals the target's pointer width, which would fold cleanly into the same family
as everything else. But the value set itself (which naturals actually exist, i.e. how many bits are
addressable) is not knowable from the type alone; it depends on where the program runs. This is a
genuine difference in kind from every other case examined here, where the value set is fully fixed by
the type's own parameters and never depends on the target.

Whether platform-dependent width is best modeled as an implicit, target-resolved instantiation of the
same shape family (an implicit width parameter fixed by `cfg(target_pointer_width)` rather than named
by the consumer), or is better treated as a genuinely separate axis because the abstract value set is
not knowable statically without knowing the target, is not settled by anything in the premises. Both
are workable; carried forward as open.

## Interoperation: conversion is not the same question as resolution

The dispatch asks what a system must expose for two of them to interoperate, and whether that
interoperation is conversion, embedding, or something else. The five-level hierarchy above suggests
these are, in fact, two separable questions rather than one, and conflating them is the likelier
mistake:

**Conversion (embedding).** Moving a value from one number system's value set into another's,
possibly losslessly (a widening of range or precision) or possibly lossily (a narrowing). This needs,
at minimum, a way to decode a source numeral back to its abstract value and a way to construct a
target numeral for that value, admitting failure when the target cannot represent it. Arvo's own
premises already name this shape directly: `arvo-bridge-home-rule.md` describes exactly this kind of
bridge trait (`ConstFrom<T>` for the lossless case, `ConstTryFrom<T>` for the fallible case), placed at
whichever crate layer can reach both the source and target types. This is a prior design attempt
present in the premises, offered as evidence of a workable shape, not as something this file adopts as
settled; it is consistent with everything derived above and needs no revision to fit.

**Resolution.** A separate question: when one expression combines two numerals from two *different*
number systems directly (an add between a `UFixed<3, 5, Hot>` and a `UFixed<2, 6, Precise>`, say),
which system's laws govern the result, and what format does the result inhabit? This needs a rule, not
just a conversion function, because an embedding alone tells you how to move a value across, not which
side's overflow policy, rounding law, or precision the combined operation should honor. Arvo's own
premises record a prior, pre-canon answer to a version of this question, worth naming as evidence
rather than adopting: `arvo-toolbox-not-policer.md` describes cross-strategy binary operations
resolving to the more conservative side's semantics with a compile-time warning rather than a refusal,
which is one workable resolution rule (a partial order over strategies, with the "more conservative"
side winning) among others that could be chosen (always require an explicit cast with no implicit
resolution at all, the stricter Rust-idiomatic answer; or a fuller numeric-tower model with an explicit
promotion order, the Python/Lisp-idiomatic answer).

Whether the canon wants any implicit resolution at all, or wants every cross-system operation to
require an explicit embedding first, is a genuine fork this file does not resolve, and it is
independent of the conversion question: a canon could have a rich, well-specified embedding
mechanism between number systems and still forbid using it implicitly inside a binary operator,
requiring every cross-system computation to name its embedding explicitly at the call site.

## Doability: the pipeline compiles, without any forbidden feature

A canon must be able to say which things are doable, not merely assert them. `66_probes/
derive_validate_erase_pipeline.rs` builds a minimal Rust sketch of the four-stage pipeline derived
above: a `Strategy` trait, a `Derive<S: Strategy>` trait carrying an associated `Container` type, a
`validate` function decidable from the container's bits alone, and an `erase` step. One shape (a toy
"Q3.5" unsigned dyadic rational) is given two separate `Derive` implementations, one per strategy, and
the two implementations deliberately choose two different concrete container encodings for the same
abstract shape (one plain, one bit-reversed with a reserved top bit), to exercise the "representations,
plural" claim mechanically rather than just assert it.

Compiled directly against the pinned `nightly-2026-05-28` toolchain with `rustc --edition 2024
--crate-type lib --test`, with no `#![feature(...)]` gate declared at all: it compiles cleanly with no
warnings, and all four of its tests pass, including one that asserts the two strategies' erased
outputs for related inputs are not bit-identical on purpose. No forbidden feature
(`generic_const_exprs`, `generic_const_args`, full `specialization`, `-Znext-solver=globally`) is
needed. The dispatch on which of two `Derive` implementations applies is ordinary generic monomorphi-
sation over the `Strategy` type parameter, not `dyn` and not `TypeId`, matching arvo's own no-dyn,
monomorphisation-is-the-dispatch rule read from the premises.

This establishes, by construction rather than by argument, that the shape+strategy to
container-and-representation pipeline, including the "more than one admissible representation for one
shape" property, is expressible under arvo's current architectural constraints without needing
anything the workspace forbids. It does not establish that arvo's real trait family should look like
this toy sketch; per the discipline governing probes, its names, arities, and exact trait shapes are
scaffolding chosen to make the one check possible, not a design proposal.

## What the premises do not license, and what was not found

Nothing examined here conflicts with any of op's intents as quoted in `INTENTS.md`, and nothing found
during this derivation contradicts arvo's own architectural rules read as premises. The strongest
candidate for a genuine tension, checked directly: I5 says Hot may sacrifice soundness for a proven
meaningful gain, and the associativity result above shows that changing overflow policy changes which
algebraic laws hold. Read together these are consistent rather than in tension: I5 already anticipates
that Hot's chosen laws will differ from, and be weaker than, another strategy's; the associativity
probe simply makes concrete what "different laws" can mean in practice. No unlicensed mechanism, and
no violation of a stated intent, was found in the course of this derivation.

## What is carried forward, not settled

- Whether the concrete "number system" in the pipeline is fixed by shape alone (with strategy choosing
  among law-sets a fixed system can be reasoned under) or determined jointly by shape and strategy
  (with a strategy change producing a genuinely different system). I9 leans toward the joint reading;
  neither is ruled out by anything else in the premises.
- Whether fixed-point and floating-point should be canonically treated as one parametrised scheme
  (differing only in where the exponent function lives) or as two genuinely separate number systems
  that happen to share a formalization but differ in their operational laws.
- Whether "number system," as a canon-level term, should broadly cover any typed, strategy-derived
  container with an algebraic structure (including `Bool` and bit masks under GF(2)), or should stay
  narrow to ordered, magnitude-bearing systems with a sibling term for the unordered case.
- Whether platform-width types (`USize`, `Cap`) are a target-resolved instantiation of the same shape
  family everything else uses, or a genuinely separate axis because their value set is not knowable
  from the type alone.
- Whether cross-system interoperation should ever resolve implicitly (with a rule for which system's
  laws win, as arvo's pre-canon `arvo-toolbox-not-policer.md` sketches for cross-strategy operations),
  or should require an explicit embedding at every call site with no implicit resolution at all.

What is not carried forward as open, because the probes settle it directly rather than merely arguing
it: encoding is not identity-determining for a number system, but the operations' laws (including
overflow and rounding policy) are; redundant, non-bijective numeral representations are a real,
useful, load-bearing tool rather than a defect, and are plausibly the mechanism I7's "accurate across
chains" needs; and the derive-validate-erase pipeline, including multiple admissible representations
per shape, is mechanically buildable today under every constraint the workspace and arvo currently
impose.

---

## Phase two: reconciliation against the panel

*(Written after the phase-one commit, reading the panel files, the option register, and the droplist
for the first time.)*
