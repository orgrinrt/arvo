# 28. What identity must express, and what the proof side and the layout side each own

**Member:** Xavier Leroy. Verified-systems lens: state exactly what each layer claims, in the
coordinates the claim is provable in, and put the correspondence between layers on the list of
obligations rather than in the fog between them. I wrote file 10, on what the apparatus certifies;
per the brief I do not assume its conclusions here, and where this file touches the same ground it
re-derives.

**Gate:** run before this work, myself, not inherited from file 27. Full mock workspace suite: 654
passed, 0 failed, 9 ignored. One ignore is the catalogued divide gap
(`crates/arvo/tests/fixed_point_div.rs:111`, tracked #5); the other eight are `ignore`-fenced
doctest examples, pre-existing, not gap markers. File 27 reported "1 ignored" because it counted
unit tests only; the counts are consistent, stated here so the next member does not read a
discrepancy into them. Test bodies read in the surface this file touches: `identity_laws.rs` is the
honest post-audit full-matrix shape and says so in its own module doc. Canon gate: the governing
calls are the D-numbered ones in `202607301200_topic.the-formalization-spec.md` and
`202607300800/202607291900_topic.the-number-systems-crate.md`; the spec draft's own charter is "the
point of sending it out is to have it torn apart" (`202607301200:18-19`), so this work is what the
round licenses.

**What I read:** `26_consolidation_two.md` in full; `27_carmack_what_a_number_is.md` in full plus
its three probes and `27_probes/OUTCOMES.md`; `202607301200_topic.the-formalization-spec.md` in
full; `202607291900_topic.the-number-systems-crate.md` in full; the `identity_laws.rs` test bodies.
Directory listed; no other panel file fetched.

**What I compiled:** the full suite (above) and two probes in `28_probes/`, both consisting
entirely of `const` assertions so that compiling is the check, with a negative control run to
confirm the assertions are live (`28_probes/OUTCOMES.md`). **Everything else is reasoned**, and the
two external anchors I reason from most (Flocq's two-sided float formalization, CompCert's NaN
policy) are cited as prior art I know first-hand, not as claims about arvo's tree.

## 0. Brief-breaking checks, and a false derivation in the spec itself

The brief's three factual premises check out against the spec text: the identity contract does
parameterise the float branch by exponent-field width and derive precision from total width
(`202607301200:81-96, 117-121`), no `Radix` member exists in the `Numeral` contract
(`202607301200:40-46`), and file 27's two compile-refusals are what its probe record says they are
(`27_probes/OUTCOMES.md`).

The new finding: **the spec's faithfulness derivation is false in one cell, and the false cell is a
mode the vocabulary exists to express.** `202607301200:210-216` grants `AddAssoc` to unsigned
addition under every `Resolution` pair:

> `impl<A: Resolution, B: Resolution> AddAssoc for ((A, B), Unsigned) {}`

on the reasoning that "one end is unreachable and the rule is truncated addition whatever it does
there". Checked exhaustively at a 5-bit model (`28_probes/probe_1_unsigned_blanket_refuted.rs`,
compiled): ReduceModulo, clamping and Refuse-under-Kleene are associative; **SubstituteZero is
not**, witness `(25 + 10) + 5 = 5` against `25 + (10 + 5) = 0`. SubstituteZero is SystemC's
`SC_SAT_ZERO` (`202607301200:141-147` puts it in the vocabulary precisely so that SystemC is
expressible), so a consumer writing the SystemC alias set would receive an associativity licence
that is measurably false. The "whatever it does there" reasoning is wrong because the reachable
end's rule is the entire behaviour of unsigned same-format addition; what the derivation needed was
a per-resolution condition at the reachable end, stated in section 5 below.

This is the same failure the consolidation names as its second general failure mode, "prose claims
about the design's own state are checked by nothing" (`26_consolidation_two.md:111-119`), now
exhibited by the spec's central derivation rather than by a citation. The spec's own sketch
obligation 3 (`202607301200:333-334`) would not have caught it: it asks the derivation to reproduce
the law sketch's refusals for seven cases, and the false cell is an eighth case nobody listed. The
defence that works is the one the workspace already has: the derivation's claim, written over the
whole resolution matrix as a model-width exhaustive check, refuses in one cell. Thirty lines of
const fn found it.

## 1. The coordinates question: the proof wants one set, the layout wants the other, and a verified artifact keeps both

The brief asks whether one parameterisation can serve both the mathematical specification and the
physical layout. The answer from the one place this exact question has been settled under machine
checking is: **no, and it should not try.** Flocq, the floating-point formalization CompCert ships
on, keeps both parameterisations, each on its own side, with the correspondence between them a
proven theorem rather than a convention.

The shape, stated concretely because it is the shape I am proposing arvo adopt:

- The semantic type is parameterised by **mathematical coordinates**: radix, precision, and the
  exponent bound (binary64 is the instance at precision 53, with the bottom of the exponent range
  derived from the pair, and the whole family of formats is one exponent function per format, FLX
  for unbounded, FLT for gradual underflow, FTZ for flushed, which is exactly the spec's own
  `Underflow` triple, `202607301200:99-103`). No field width appears anywhere on this side. Every
  rounding theorem, error bound and operation-correctness proof is stated and proved here.
- The **encoding** is a separate pair of functions, parameterised by field widths (exponent field,
  trailing significand field), mapping the semantic datum to a bit pattern and back. The hidden
  bit, the exponent's encoding bias and the reserved codes exist only inside these functions.
- The correspondence is two **round-trip theorems**: decode after encode is the identity on data,
  and encode after decode is the identity on bit patterns. Those theorems are the entire contract
  between the sides, and everything either side proves transfers through them.

Read against this, file 27's inversion proposal (`27:82-123`) is the correct move, and I add the
second half it needs: it is not enough to move width derivation to the `Lowering` side; the
**round-trip law is a new derived fact the design owes**, one per (numeral, encoding) pair, and it
is exactly the shape the existing apparatus already checks cheaply. Both directions are per-value
statements over a finite datum set, so the model-width exhaustive mechanism
(`26_consolidation_two.md:72-75`) applies unchanged, and the same-text-monomorphised-twice
discipline of the recovery-map witness applies to the encode/decode pair verbatim. Without this
law stated, the inversion just relocates the off-by-one risk from the identity contract into an
unchecked derivation; with it, the derivation cannot drift silently. This is the crossing the brief
asked about: **what crosses between the coordinate systems is the datum, and the only thing either
side may assume about the other is the round-trip pair.**

On the standing of the inversion itself: D65 and D69 are op's calls, and file 27 argued against
them on the spec's own evidence. I have formed my reading independently of file 27's (from the
Flocq precedent, which file 27 does not cite) and it agrees, grounded in the spec's own text: D58
states "the standard separates a format, which values are representable, from its encoding, how
those values are laid out in bits" (`202607301200:70-72`), and D65 derives format parameters
(precision, minimum exponent) from an encoding parameter (the exponent field's width,
`202607301200:83-85`), which is D58's split with the arrow pointing backwards. The spec's own
provenance note pre-flagged D69's "not derivable" claim as suspect (`202607301200:358-359`). Two
independent readings now agree, each grounded in quoted text; the call remains op's to restate, and
the counter-reading (interchange formats are defined width-first in the standard's own table) is
carried honestly at `27:115-123` and survives as the `conv-ieee754` alias direction.

## 2. The datum and the value are different things, and the identity contract should say so once

File 27 found signed zero breaks the injectivity of the value map and proposed housing it in a
`Specials` member (`27:195-201`). Taking it further: signed zero is one instance of a structural
fact that appears three times in the standards the design is measured against, and the identity
contract should state the general fact once rather than meet its instances one by one.

**A numeral defines two things: a finite set of data, and an interpretation of each datum as a
value.** The interpretation is generically non-injective, and the three standard instances of the
collapse are:

1. **Signed zero.** Sign-magnitude indexing maps the datum (negative, 0) and the datum
   (positive, 0) to the same rational. IEEE 754 distinguishes the data (copysign, division
   behaviour) while defining numeric equality on the values.
2. **NaN payloads.** Many data, no value at all. The payload is a refusal cause carried in-band,
   which file 27 already said (`27:203-207`); the datum/value frame makes it precise: NaN data are
   data whose interpretation is a refusal, so the float branch's fallibility is value-shaped
   rather than `Outcome`-shaped, and the D30 witness is where it is discharged into the type-level
   effect.
3. **Decimal cohorts.** This one is mandatory for the design's own proof case. D58 leans on
   decimal64 (`202607301200:72-75`), and IEEE decimal formats represent one value by several data
   on purpose: a cohort is the set of representations of one value at different exponents, and the
   standard's operations specify a **preferred exponent** per operation to pick the cohort member
   delivered (754-2019, 5.2). A design that admits decimal as a numeral admits non-injective
   interpretation as a core case, not a float wrinkle. Cohort choice is then a policy-shaped axis
   that only exists when the interpretation is non-injective, the same nesting argument the spec
   already used for `Underflow` (`202607301200:98-99`).

Four consequences follow from stating the distinction once at the identity, and each lands on an
existing open item:

**Equality splits, and the laws must name their relation.** Datum equality and value equality
coincide exactly when the interpretation is injective, which is derivable from the axes (binary,
finite, two's complement: injective; sign-magnitude, or specials present, or decimal: not). The
open question "which relation a fold-law is stated under" (`26_consolidation_two.md:608-617`)
gains a prior question with a mechanical answer: whatever relation is chosen, it is a relation on
**values**, and the identity layer must supply the projection under which two data are compared.
Kleene equality over data is simply wrong the moment sign-magnitude or decimal enters, because a
lawful regrouping may legitimately deliver a different cohort member or a differently signed zero.

**`Deterministic` (D70) splits, and as stated it is false for float compositions.** The blanket at
`202607301200:234-236` claims a composition-keyed determinism. For any composition whose
operations lower to hardware float instructions, the delivered **datum** is not portable: NaN
payload propagation differs across architectures, which is a settled fact of shipping hardware,
and CompCert had to parameterise its entire float semantics by a per-target NaN payload policy in
its architecture-description module to keep its own correctness theorem true. arvo faces the same
fork: either `Deterministic` is two facts (value-deterministic portably, datum-deterministic only
per target or after paying for NaN canonicalisation at operation boundaries), or the blanket is a
prose claim checked by nothing, which is the consolidation's named failure mode
(`26:111-119`). The datum/value split is what makes the honest version statable at all.

**The quantiser's domain is signed exact values, and the sign is an operation-side input.** IEEE
6.3: the sign of an exact zero sum depends on the operand signs and the rounding attribute (x plus
negative x is positive zero under every attribute except roundTowardNegative), while a zero
product's sign is the XOR of the operand signs under every attribute. So the delivered datum at
zero is not a function of the exact value; it is a function of (exact value, sign), with the sign
computed by the operation before quantisation. The spec's Quantisation contract
(`202607301200:149-157`) types the map as exact-value-to-representable; for any sign-magnitude
numeral its domain must be the signed exact value, or the design will rediscover this as a
special case inside every operation. Flocq's operations do exactly this: sign first, by the
operation's own rule, then round the magnitude.

**Storage and encoding speak in data; laws speak in values.** This is the same one-distinction
discipline as section 1: the round-trip theorems are stated on data, the algebraic apparatus on
values, and the interpretation is the only bridge. Stating it once at the identity keeps every
downstream artifact answering to one definition instead of three ad-hoc carve-outs.

## 3. The `Sign` axis under-determines the representable set, by the spec's own sorting test

`Sign` is "`Unsigned` or `Signed`, unchanged from what ships" (`202607301200:116`). Apply D54's own
test (`202607301200:33-36`): at the same width, a two's-complement signed numeral represents an
asymmetric range with one zero, and a sign-magnitude signed numeral represents a symmetric range
with two zeros. Different representable sets, same axis instance. So `Signed` alone does not
determine the set, which means the axis is incomplete as identity, not merely coarse.

The standards the design is tested against sit on both sides: MATLAB fi and SystemC fixed point
are two's complement; IEEE floats are sign-magnitude over the significand; and SystemC's
`SC_SAT_SYM` exists precisely because the symmetric range is a real target even in the
two's-complement world. The fix is one axis with three instances, `Unsigned`,
`TwosComplement`, `SignMagnitude` (a fourth, ones' complement, becomes expressible for free and no
test standard needs it), and it buys two simplifications elsewhere:

- **Signed zero stops being a special.** It is the derived datum/value collapse of
  `SignMagnitude` at k equal to zero, which is where it actually comes from, rather than an entry
  in a `Specials` list beside infinities it has nothing in common with. File 27's `Specials`
  member then carries only the data genuinely outside the affine map, infinities and NaN, which
  sharpens its charter.
- **The faithfulness derivation's signed case becomes honest.** "Signed addition reaches both
  ends" (`202607301200:214-215`) is a two's-complement fact; a sign-magnitude integer numeral has
  a symmetric range and its own reachability story. Deriving reachability from the sign form
  instead of asserting it per signedness is the same move as deriving the range (D73,
  `202607301200:230-232`).

## 4. Quantisation classifies the wrong thing, and specials ride on the fix

File 27 found that overflow-to-infinity has no expression in the five-position vocabulary
(`27:183-193`). The defect is one level deeper than a missing datum, and it is present in the
finite fragment too, where no infinity exists to blame.

**The five positions are assigned to the exact value; every standard in the test set assigns them
to the rounded value.** IEEE 754-2019 (7.4) defines overflow as the condition that the result
rounded "as though the exponent range were unbounded" exceeds the largest finite number; SystemC
applies the quantisation mode first and the overflow mode to its output; MATLAB fi rounds, then
applies the overflow action. The spec instead sorts the exact result and applies `OverRange` when
it lies past the top (`202607301200:130-134`). The two orderings disagree on the band past the
largest representable but within half a quantum of it, and
`28_probes/probe_2_classify_after_rounding.rs` (compiled, all-const) pins the three consequences:

- Under `Refuse`, classify-first refuses on the band where IEEE's trap-on-overflow does not trap
  and SystemC and MATLAB both deliver the largest finite. `Precise` as redesigned would refuse
  computations every reference implementation accepts.
- Under clamping the two orderings agree at every input (checked exhaustively), which is why the
  shipped clamping presets could never have surfaced this.
- Under `ReduceModulo`, classify-first is not even well-defined: the reduction of a band value
  lands back in the band, so the resolution's output is in the situation it was invoked to
  resolve, and the spec states no second rule. Round-first has no such case.

The band is empty for same-format addition, whose exact sums are on-grid (the consolidation's
partial-identity measurement, `26:169-171`, is the same fact), and inhabited for multiplication,
division, mixed-format addition and every float operation. So the additive half of the review
could not have seen this, and the multiplicative half's relocation of rounding into `quantize`
(`26:229-243`) makes the fix land in exactly one place.

**The amendment**, small in text and load-bearing: the quantisation map is "round on the
unbounded-exponent extension of the grid by the direction triple, then classify the rounded result
against the range and resolve by the range rules". One sentence replaces one sentence; the five
situations survive; only what they are situations **of** changes.

With that amendment, specials integrate instead of being bolted on:

- **Overflow-to-infinity becomes expressible and correctly conditional.** With infinity as a datum
  (file 27's `Specials` on the `Ranged` branch, which I second), "nearest" against infinity is
  meaningless (no midpoint exists at infinite distance), which is precisely why IEEE defines the
  top cell by the unbounded-grid device rather than by neighbour comparison. Round-first is that
  device. The `OverRange` resolutions then act on a rounded result, and IEEE's coupling (ties
  modes carry overflow to infinity, roundTowardZero to the largest finite, the directed modes
  asymmetrically) is not a new axis but four rows of the `conv-ieee754` alias table, which is
  where D66's own test says convention-specific coupling belongs (`202607301200:275-283`).
- **The underflow mirror is a flag question, not a value question, and it is target-divergent.**
  Whether tininess is detected before or after rounding is implementation-defined in IEEE 754
  (7.5) and hardware genuinely disagrees (x86 detects after rounding, ARM before). The delivered
  values agree; only the underflow **event** differs. In this design's terms that is a
  refusal-cause/grade fact, not a numeral or quantisation fact, so it belongs in the effect
  vocabulary (the graded reading's cause set, `26:203-214`) as one axis with two instances, and a
  portable `Deterministic` claim about grades must name which. Recorded here so it does not
  surface later as an identity question, because it is not one.

## 5. The corrected faithfulness derivation, and what it teaches the classification machinery

Probe 1's result rewrites the spec's two-impl story (`202607301200:210-216`) into the per-end,
per-resolution statement the arithmetic actually supports. For same-format addition, with
reachability derived from the sign form (section 3):

- A reachable end resolved by **ReduceModulo** preserves associativity (the recovery is a
  homomorphism onto the cyclic group; both ends must then be modular, which is the spec's signed
  case stated per-end).
- A reachable end resolved by a **clamping direction** preserves associativity **only when the
  operand domain is confined to that end's side** (unsigned overflow-clamp: the absorption
  identity, compiled in probe 1). With both ends reachable it fails, which is the consolidation's
  measured signed-clamp result (`26:131-137`).
- A reachable end resolved by **Refuse** preserves Kleene associativity (definedness of every
  grouping is the single condition "the true sum is in range", compiled in probe 1).
- A reachable end resolved by **SubstituteZero** preserves nothing (compiled counterexample).

Two things follow beyond the fix itself. First, the derivation's condition set now includes
**reachability and operand confinement**, which are numeral-side facts, confirming from a new
direction the consolidation's rule that a law's key must include every parameter the proof used
(`26:148-152`): the spec's blanket silently keyed on signedness when the proof actually keys on
(resolution at each reachable end, confinement). Second, the structural classification of the
recovery map (`26:77-88`) is too coarse for exactly this case and should say so in its own
statement: signed and unsigned clamping have the same classification (retraction) and opposite
associativity verdicts, so the classification transfers across widths and arities but not across
**domains**, just as it does not transfer across operations. The class of the pair (phi, Op) is
really the class of the triple (phi, Op, reachable region), and writing the third component down
is what makes the three-line proofs honest.

## 6. Membership: the correction stands, and window-closure is what the layer can actually derive

On D39's operative sentence ("a consumer bounding on `Real` gets the field operations because that
is what being real means", `202607291900:61-67`): I formed my reading before checking file 27's
and they agree, so the correction now has the two independent, canon-grounded readings the
workspace requires, and the call is op's to restate. My grounding is the topic's own inhabits
precision (`202607291900:80-84`): membership is a per-value statement, an inclusion of sets, and
an inclusion delivers the ambient operations only where the numeral's operation IS the ambient
operation restricted, which is the exact family and nothing else. For every quantised operation
the recovery map is the measure of that failure, and the consolidation's law inversions
(`26:126-137`) are its measured face. The honest sentence is file 27's: membership licenses the
exact operation family; quantised operations get laws from the ladder, keyed as section 1.4 keys
them.

What I add is the piece that makes the layer derive something instead of merely classifying:
**window-closure**. The membership layer names the ambient set; the numerals are finite windows
into it; and the question "which operations may be offered exactly" is precisely "for which
operations does the exact result of any two windows land in some expressible window". That is a
derivable, per-(system, operation, adjustment-form) fact, and it organises three previously
separate items into one mechanism:

- **Addition and multiplication at `Unit` adjustment close** (widths add, exponents add), which is
  the multiplicative half's verified adder (`26:249-258`) restated as the closure witness. The
  membership layer's operative trait is then something like `ExactWindow<Op, Rhs>` with an
  associated output numeral computed by that same adder, and the law it carries (inject of the
  exact op equals the ambient op of the injects) is a per-value statement checkable at model
  width by the existing apparatus. This is what a bound can honestly carry across Stage G: not
  "the values are real", but "this operation, at these windows, is the ambient operation, with
  its laws inherited rather than re-proved".
- **The closure gap (`26:326-331`) is the failure of window-closure for adjustments, and it has a
  constructive fix.** A `FullRange` quantum's product leaves the `FullRange` form because the
  adjustment family is not closed under multiplication. Generalise the adjustment to an
  **unnormalised rational pair** of type-level naturals: quanta multiply by multiplying numerators
  and denominators, the pair grows exactly the way widths already grow, no gcd normalisation is
  needed for closure (normalisation is a `quantize`-shaped explicit step, like narrowing), and
  the growth per operation is bounded and checkable by the same interior-safety discipline as
  accumulator width (`26:281-287`). Reasoned, not compiled, but the mechanism is the same
  typenum-style adder file 25 verified, applied twice. The alternative in the consolidation
  (`MulClosed` gating with explicit renormalisation) survives as the cheaper first ship; the
  rational pair is the shape that makes UNORM-times-UNORM expressible rather than refused, and
  the two can coexist (gate now, generalise when a consumer arrives).
- **The exact family is per operand-restriction, not only per operation.** Division fails
  window-closure in every system (the consolidation's prediction that division has no finite
  accumulator solution, `26:678-681`, is this fact in accumulator clothing), **except** division
  by a power of the radix, which is an exponent shift and exact in any dyadic-or-finer system.
  That subfamily is worth naming in the design because it is what MATLAB, SystemC and every DSP
  actually use for scaling, and offering it as an exact operation (distinct from correctly-rounded
  general division) removes a whole class of needless quantisation events. Correctly-rounded
  general division (`26:317-321`) is then the fallible complement, exactly as shipped.

Interior safety also reads more simply in this frame, seconding and sharpening `27:243-252`: a
lawful fold is one whose interior lives in the exact family (where the inclusion is a
homomorphism and the ambient laws transfer), with one quantisation at the root; the checked
accumulator bound is the window-containment condition for "the interior stays exact"; and the
membership layer is the type-level name of where those interiors live. Nothing new has to be
built for this reading; it is the same const assertion with a better-specified meaning.

On the shape that delivers membership: probe A's finest-system projection with marker dispatch is
compile-verified in both directions by file 27 and I have nothing to overturn there. One caution
from the verification side: the finest-system **derivation table** is the single point where a
wrong row silently misclassifies every numeral it matches, it is exactly the kind of prose-shaped
fact the consolidation's failure mode warns about, and it is cheap to guard: the table's claim
per row is per-value and finite ("every value of a numeral matching this row lies in the named
set"), so it takes the same model-width exhaustive witness as everything else. That check should
be a stated obligation of the num-systems crate, not an optional nicety, because the layer's
whole promise is that a blanket impl conditioned on structure cannot lie, and the table is where
structure is read.

## 7. Proposals, costs, and standing

1. **Adopt the two-coordinate structure with the round-trip law as a first-class derived fact.**
   Identity in mathematical coordinates (radix, precision, exponent bounds, sign form); encodings
   on the `Lowering` side carrying field widths, hidden bit, encoding bias, reserved codes; two
   round-trip theorems per (numeral, encoding) pair, model-width checked by the existing witness
   apparatus. This is the Flocq/CompCert shape, it subsumes file 27's inversion, and it converts
   the unresolved `Stored`-versus-IEEE item from a bug into an obligation with a mechanical check.
   Cost: the axis table rewrite file 27 already priced, plus one new derived-fact family whose
   checking machinery already exists.
2. **State the datum/value distinction once, at the identity.** Non-injective interpretation is
   entered by sign-magnitude (signed zero), specials (NaN), and decimal (cohorts, with preferred
   exponent as the policy-side choice that exists only when interpretation is non-injective).
   Consequences to write down: laws are relations on values; `Deterministic` splits into value-
   and datum-determinism (as stated, D70 is false for hardware-lowered float compositions);
   the quantiser's domain is signed exact values with the sign computed operation-side.
3. **Extend `Sign` to `Unsigned` / `TwosComplement` / `SignMagnitude`.** Required by D54's own
   test; derives signed zero and per-end reachability instead of asserting them; leaves `Specials`
   carrying only infinities and NaN.
4. **Amend quantisation to round-then-classify** (one sentence): round on the unbounded grid by
   the direction triple, then resolve the rounded result against the range. Probe-verified
   divergence otherwise, in the finite fragment, against all three test standards; `ReduceModulo`
   is ill-defined without it; overflow-to-infinity and IEEE's mode coupling then land as
   `conv-ieee754` alias rows rather than new axes.
5. **Replace the unsigned faithfulness blanket with the per-end, per-resolution table of
   section 5.** The current blanket is refuted by compiled counterexample at the `SC_SAT_ZERO`
   cell. Add the reachable-region component to the recovery-map classification's stated key.
6. **Correct D39's operative sentence and build window-closure as the membership layer's derived
   content**: `ExactWindow` with the verified width adder as its computation, the rational-pair
   adjustment as the closure fix (or `MulClosed` gating as the cheap first ship), and
   divide-by-radix-power named as the exact subfamily of division. The finest-system derivation
   table carries a model-width witness obligation.

Standing of the contested calls: the D65/D69 inversion and the D39 correction each now have two
independent expert readings grounded in quoted canon text (files 27 and 28, formed independently;
mine argued from prior art file 27 did not use). Per the workspace's own rule that resolves
nothing: both remain op's to ratify, and this file's function is to make the ratification question
precise rather than to close it.

What I did not do: no compile-cost measurement of any proposal (belongs in `mock/benches/`, still
unpriced per `26:668-674`); the rational-pair adjustment is reasoned on the verified adder, not
itself compiled; the decimal cohort machinery is scoped as a requirement on the identity contract,
not designed in full, because no decimal arithmetic exists or is proposed yet and the requirement
is what keeps the door from being built shut.
