# The cumulative droplist, carried forward

Extracted verbatim from `202607301300_formalization-spec-panel/124_consolidation_twelve.md`, sections 6 and 7, at the moment that panel closed. Section 6 is the cumulative droplist; section 7 is reversals inside a stretch, kept under a separate name because a same-stretch reversal is not a real removal and conflating the two made an earlier droplist report on material that had never left the base.

## 6. The droplist, cumulative

**Every real removal across the whole panel, in one place, one line each.** The droplist went delta-only at the
fifth consolidation, so a reader of the tenth saw eight entries out of roughly ninety (`109:560-564`); the
eleventh reassembled it and this one carries it forward. **Entries retiring a proposal born inside the stretch
that absorbed it are not on this list**; they are section 7, under their own name, because file 109 found that
sixty-six of the eighty-seven entries ever authored are of that kind and only twenty-one correspond to a real
removal from a predecessor (`109:539-558`).

**The stated purpose of an entry**, reproduced from the third consolidation's own preamble because the fourth
carried the preamble and stripped the reasoning it promises (`109:566-574`): entries are stated **with just
enough of their reasoning that a member who believes a retest would come out differently knows what has to be
overturned**. Where a later document stripped an entry's reasoning, it is restored here.

**Relocating the algebraic-law machinery to hilavitkutin**, on the theory that associativity is specifically the
contract of parallel reduction: refused by op directly, **and independently undercut by measurement: the
regrouping that would have motivated the move already happens inside arvo's own licensed internals, on a single
thread, worth roughly 2x, before any scheduler exists to relocate to.** The fourth consolidation carried this
entry with neither the theory nor the measurement, eight lines below the preamble promising both.

**Gating `arvo-graph`/`arvo-comb`/`arvo-spectral` on `AddAssoc` (or any associativity fact) by default**:
measured directly to admit the one preset (`Hot`, wrapping) whose recurrences return wrong answers under these
algorithms' own stated specifications, and refuse the two (`Warm`/`Cold`, saturating) that compute correctly,
because associativity and the distributivity these algorithms need are different, complementary laws that
invert across the same presets. `Monotone<Add>` is the atom the refusal was reaching past (section 1.20).

**"A documented traversal order substitutes for a law"**: wrong axis. Associativity is about grouping, not
order, and contiguous chunking preserves element order exactly while still changing grouping.

**Bounding a regrouping combinator on a numeric diameter budget rather than a boolean law**: refused by
measurement, since signed saturating addition's regrouping diameter grows to the entire representable range by
a five-element fold, so there is no useful budget to bound against.

**Predicting the accumulator-agreement threshold from the recovery map's monotonicity**: refuted by exhaustive
measurement. **Every non-homomorphism resolution reaches the same threshold (`K = n - 1`, interior safety)
regardless of whether it is monotone.**

**Computing type-level width arithmetic as a const generic under `min_generic_const_args`**: refused
structurally at the definition site; the feature's sound subset explicitly forbids arithmetic over a
still-generic const parameter on its own right-hand side. Replaced by type-level binary width encoding, itself
later replaced by the value-unique `Nat`/`Pos`/`Bias` encoding.

**Growing an accumulator's own *type* on every iteration of a runtime-bounded loop**: cannot work in principle,
not merely unbuilt, since a type cannot depend on a value only known at runtime. Replaced by fixing the
per-element product's type and checking accumulator sufficiency as a compile-time bound; a renormalising step
is the property that closes the gap (section 1.8).

**Declaring a fidelity-licence coercion as a trusted marker trait with no associated items**: compiles clean
when corrupted, with zero diagnostic, because a permission-shaped coercion carries no data for the compiler to
check against; a hand-verified wrong grant produced a silently wrong numeric answer. Two follow-up fixes also
failed: a fully generic blanket derivation hit the generic-const-in-type-position wall, and porting a
`WITNESS`-constant shape onto the same trait is disarmable exactly the way one existing resolution
constructor's classification was already disarmed, **because the implementor writing the lie also controls the
check for the lie inside the same impl block.** Replaced by recomputing the relation inline in an ordinary
`const {}` block at every consumption site. **The principle the entry was trying to satisfy is op's adoption
and no consolidation stated it**: `17b:19-30`, **"a fidelity grant is checked rather than asserted, on the same
footing as the recovery map that the earlier verification thread ended up witnessing."** Op also recorded what
that does *not* settle: the shape, since a licence witness is not a port of the recovery-map witness, there
being no returned value to check a grant against, "which is precisely why the corruption went undetected." The
replacement satisfies the adoption in substance, because recomputing inline is checking rather than asserting,
and without the principle stated a reader could learn only that one particular way of not checking failed
(`112:466-473`).

**A pushed, registered build-layer manifest** for monomorphisation recovery: strictly worse information than
the pull-shaped symbol-table read, since it records what a consumer *declared* rather than what got
*instantiated*, silently misses every composition reached only through generic code, and **cannot be written at
all for a generic function**, because Rust forbids an item declared inside a generic function body from naming
that function's own type parameters.

**Treating `f64::mul_add` as a source-expressible fidelity liberty (contraction)**: it lowers to `llvm.fma`, a
distinct, exact IEEE operation with one defined answer rather than a licence to pick either; it is unavailable
under `#![no_std]` without an unvetted or forbidden feature; and on a target with no hardware FMA unit it
compiles to a pessimising libm call, the opposite of what a licence should cost. **`Fused` belongs in the
design as a named operation, not as a fidelity permission.**

**Citing "the one shipped `Monotone` law implementation"** as existing, shipped design: it does not exist as any
implementation, only as an unlocked design-round proposal, **and two members built directly on the false
citation before a third caught it with a single grep.** Checked against its own admitted compositions and found
false: it names three of the five quantiser members that decide monotonicity, and asserts monotonicity for a
resolution (`ReduceModulo` at both range ends) that a separate proof rules out for every width.

**Assuming the recovery-map classification's cheapness transfers automatically to a new operation**: refuted
twice independently. **The classification is a property of the pair `(phi, Op)`, not of `phi` alone.**

**"Past the top is unreachable" once infinity is representable**: false. Infinity changes the over-range
position's neighbour rather than removing the position; the midpoint that decides overflow lives on the
round-first amendment's unbounded grid.

**The unsigned faithfulness blanket over every `Resolution` pair**: refuted by compiled counterexample.
`SubstituteZero` breaks associativity where clamping and modular reduction preserve it. **What it refutes,
named because the entry did not name it** (`113:427-438`): the two-impl law derivation in the round's own
ratified body, `impl<A: Resolution, B: Resolution> AddAssoc for ((A, B), Unsigned) {}` at `talk:1187-1203` and
`spec:203-222`, which carries no D-number and is not among the items `spec:356-359` marks as the agent's own.
Its reasoning was that unsigned addition can only leave the range above, so one end is unreachable and the rule
is truncated addition whatever it does there. **The counterexample is that `SubstituteZero` at the reachable end
is not truncation.** The signed impl, bounded on `Faithful` rather than on `Recovery`, is unaffected. **Op's
part is only whether the refutation stands against his own text.**

**Classify-then-round as the quantiser's order**: disagrees with all three test standards on the band past the
largest representable but within half a quantum of it. Replaced by round-first, classify-second.

**Two round-trip theorems as the crossing contract** (`decode ∘ encode = id` and `encode ∘ decode = id`, both as
identities): the second is false the moment signed zero, NaN payloads, or decimal cohorts exist. Replaced by
the section-retraction triple.

**A single three-instance `Sign` axis bundling range and zero-count**: under-determines the set and mixes a
value fact with a datum fact. Split into `SignDomain` (identity) and `SignIndexing` (encoding), and section 1.2
states what that split buys.

**Block floating point as evidence for nesting `Adjustment`/`Bias`/`Underflow`/`Specials`**: the nesting
decision stands on the `Underflow` argument alone (section 1.2); BFP is a different kind of object, a composite
numeral over a shared external exponent, and is not evidence for either shape being cheaper to extend.

**Referential uniqueness as an alternative to value uniqueness**: fails the ordinary case of storing a product
in a declared numeral, and is an invariant living in a signature-writing convention, the class this review
keeps finding rots silently.

**Projecting a trait-level width computation back down into an ordinary const parameter** to dodge the
value-uniqueness obligation: refused, the feature named (`generic_const_args`) being neither the permitted nor
the forbidden one and unvetted.

**The claim that the shipped width chain and integer adjustments already satisfy value-uniqueness**: false for
the width chain, since `UInt<UTerm, B0>` is a second spelling of zero and the adder propagates rather than
normalises it.

**"Two spellings of one condition" for interior safety and total safety**: two distinct conditions serving two
distinct promises (section 1.8).

**The ordered three-relation ladder** (weak, then Kleene, then graded): replaced outright by the nine-point
view lattice, which is not a chain and contains two shipped presets at incomparable points.

**"Partial associativity" as a named gap in the algebra vocabulary**, adopted by op at `17b:40-50` after file 17
measured that `Precise` has zero numeric spread across groupings and that its regrouping sensitivity is
entirely about which groupings are defined at all, with op recording that "the design does not name it, and no
standard vocabulary carried in the spec covers it": **retired with the ladder above.** The nine-point view
lattice supersedes it, and the point it names is stated positively at section 1.7, `Precise` below interior
safety sitting where a view "preserves values and events while losing definedness", which is the definedness
reading the partial law was for. **The entry as first written retired the ladder and said nothing about op's
adopted name going with it**, so a reader found an op adoption in `17b` with nothing anywhere saying what
became of it (`112:377-397`).

**`ffl` as a member of the physical-grounds row** (`63:447` lists `pin`, `host`, `flags`, `model`, `ffl`; the
registry section above lists the first four): removed, because the transfer-ground scheme replaces what `ffl`
was credited with, per `68`'s own section heading, and `109:398` flagged it as used and undefined. **The
removal is probably correct and it was silent**, which under this document's own rule that section 6 is the
cumulative diff is the thing the rule forbids (`111:170-176`).

**The reification-stability generalisation** (that the graded relation is the only one stable under a
`Refuse`-to-special reification): true of one reifier and false in general. Under an out-of-set absorbing
special, Kleene is stable too; under `SubstituteZero`, nothing is.

**`Op::IS_EXACT` alone as the statement that an operation's grade monoid is trivial**: false in general,
corrected to the conjunction with `Total<Op>` (section 1.7).

**A consumer-declared required view as the mechanism gating a regrouping's licence**: killed by the compiler
mid-dispatch, because the licence check refused exactly the case the mechanism existed to handle. Replaced by
the transfer rule, carried by a type projection rather than a declared const.

**The subset-domain reading of the view parameter**: not closed under meet. Replaced by the
quotient-of-the-grade reading.

**`Bias` as a plain signed integer**: made a legal MATLAB numerictype unrepresentable (slope 1, bias 1/2),
which is the standard's own test failing. Corrected to a signed, gcd-normalised rational, built and sealed.

**Three separately-restated `Numeral` member lists across files 35, 36 and 38**: none of the review's compiled
results depended on any of them.

**The candidate closed form for the overflow band, `q_result <= 2 * lattice`**: refuted by exhaustive
enumeration in both directions (753/1000 addition, 639/1000 multiplication). Replaced by the two-clause
lattice-plus-reachability form (section 1.5).

**`Specials` as a three-instance chain** (none, infinities-only, IEEE): the middle rung's witness demand exposed
that the axis was the wrong shape entirely. Replaced by the two-fact product.

**Absorbing a decimal numeral's quantum into its rational adjustment**, dispensing with a separate exponent
axis: does not compile at any real decimal format's exponent range, against 64 ms flat for the
radix-and-exponent spelling of the same grid (section 1.17).

**A finer-grained reassociation licence than the four-flag `algebraic_add` bundle grants**: does not exist on
the stable-track surface as tested. The workaround (discharge each companion permission separately) is sound
because each is independently checked, not because the bundle became narrower.

**The claim that the vectorisable-loop-idiom finding held unconditionally**: it holds, but only under
`-C codegen-units=1`, inherited by accident from an earlier unrelated investigation and never identified as
load-bearing until it was checked directly against the real crate.

**A bounded numeral-notation table**: refused on principle, and a second, independent route to the identical
refusal appears at the notation macro's own face layer, where a const-generic face cannot be structurally
sealed.

**Treating the algorithm crates' `Precise` exile as the problem to solve**: the presets the design admits today
both return wrong answers under the exact bound they satisfy. **The exile was never wrong; the admission was
silently wrong**, and no amount of readmitting `Precise` addresses a defect living in the crates' own return
type.

**The three-instance reading of `Underflow`** as one axis carrying flush-to-zero alongside gradual and abrupt:
flush-to-zero changes no representable set and is a `Quantisation` resolution, not a `Numeral` fact wearing
one's clothes.

**The door as a projection from the strategy alone**, with a software fallback refined where the numeral is
host-implemented: refused by coherence (`E0119`) and, separately, by `min_specialization` twice; the only
opener is a forbidden feature.

**`Cold`'s door justified as "follows the semantics-first side"**, a storage fact that does not imply an
arithmetic-lowering answer: replaced by a justification that does.

**The framing that a datum-level `TotalOrd` makes none of the algorithm crates' outputs law-expressible**:
refuted by compile as a design-wide verdict, correct only for `arvo-spectral`.

**A pure `macro_rules!` decimal-to-binary muncher**: refused structurally, not merely found expensive. No
fragment specifier, restringify trick, or const-generic escape reaches a decimal literal's digits.

**Pricing a "checked" declaration sweep against an unused type alias** rather than one whose bound is actually
forced: produced a misleading result in the wrong direction on its own first attempt. Corrected by forcing the
bound in both arms before comparing, and the control is now a stated convention (the conventions section).

**A universal unreproducibility claim grounded on compiling one file in isolation** rather than reading the
directory it ran in: refuted by the committed recipe and the rebuild, and the `unreproducible` ground's own
exhibit was struck while the ground survived on its second exhibit.

**The last sentence of `unstable-features.md`'s transfer argument**, "without them, monomorphisation is uniform
and the transfer is sound": refuted twice by compiled counterexample with the bans in force, promoting a
necessary condition (implementation uniformity) to a sufficient one for a different claim (property uniformity)
the rule's own source had already named unproven.

**The two-mechanism enumeration of ways an instantiation can be observed** (`specialization`, `TypeId`) as
exhaustive: refuted by a shipped, permitted third mechanism, const-tag container dispatch, demonstrated with a
property true at eight bits and false at nine, no gate, one parametric body.

**The "refused at nine bits" wall as a width ceiling**: refuted; it is a total-step-count budget, and a cheaper
predicate compiles clean at nine and refuses one bit later.

**"Exactly one cell of the matrix leaks"**: refuted; the matrix held `Specials` fixed, the full product leaks at
six of eight cells, and the correct framing is a family of configurations (section 1.4).

**Widening the crossing contract's target through the quantiser** as an alternative to statement 0's obligation
on the encoding: refused on every escaping datum tried, against a 2,701-value negative control confirming the
quantiser is otherwise the identity. **Not a preference, an arithmetic fact.**

**Cross-call-site face identity as something needing a mechanism**: refused; nothing that affects compilation is
keyed on the face, per-site display is the better diagnostic, and unifying faces would build the exact
false-refusal failure the layer-keying rule forbids.

**A committed sketch's universal claim that the facade's "only live GCE constructs are two static asserts"**:
refuted by a whole-crate compile, two of 478 spans, and it is the origin of the whole-crate-compile convention.

**`63:816-817`'s claim that the facade fix "touches every consumer of `Bits`, `UFixed` and `IFixed`"**: refuted;
three of the four things a consumer writes are unaffected and the real public break is twenty-one call sites
naming `Fixed<I,F,S>`/`Signed<I,F,S>` directly.

**Route X for the facade migration** (const-keyed projection, only the computed width lifted to a type, `I`/`F`
staying bare consts): refused structurally, six ways across two compiled attempts.

**Route Y for the facade migration**: fails guarantee parity three separate ways, each a compiler diagnostic
rather than an argument. The two-dimensional impl table refuses correctly at type-check but is priced on a
ceiling the toolbox rule forbids the substrate to set below what it dispatches through, failing outright below
width 64 and costing 30.0 s at a 256 ceiling, roughly quartic. A host-staged witness compiles clean and fast
and **is caught only at `--emit=link`, not at `--emit=metadata`, which is the command a consumer's editor
actually runs**, silently re-opening the `UFixed<0, F>::ONE` defect. A consumer-emitted per-declaration impl is
refused by the orphan rule, `E0117`, **with rustc's own diagnostic naming route Z's shape as the remedy.**

**The capacity unification's naive spelling**, "the shared carrier answering directly for the backing array":
refused four ways, citing the forbidden `generic_const_exprs` and, behind the compiler's own suggested
successor, the inductive step `2 * P::VAL`, which `min_generic_const_args` cannot express either.

**The feasibility probe's implicit claim that the capacity unification's whole load-bearing path was compiled**:
it was not; the probe declared the capacity trait as a bare const and never reached the associated array type
the domain exists for.

**The working "two instances" resolution of `Layout::Bitpacked`**: superseded. The axis has one meaning; the
byte-aligned reading was always `Layout::Dense` at a narrow `StoredWidth`.

**File 32's own bitpacked measurement, treated as a measurement of `Layout::Bitpacked`**: it was always a
measurement of `Layout::Dense` at a narrow width, correctly built and mislabelled.

**The hardware-reachability theorem's original statement**, "reachable only in a uniformly-`Hot` expression":
corrected to four cells of sixteen once `Warm`'s door moved to `HostFloat<E>`.

**File 59's strategy-door table**, "every row below is derived from what the preset already means for
fixed-point arithmetic in the shipped tree": **void**, and it is the exhibit the whole `tree-meaning`
prohibition rests on.

**The nine-bit companion's original characterisation**, "the first point at which the padding half of the
crossing contract has observable content": superseded. It measured the ungoverned container level, not statement
P's content.

**`67b`'s naming principle** applied as written: dead. It forbids op's own `79b` intent pillar and was never
op's own hand.

**The "quantify over every inhabitant of the carrier type" amendment to statement 0's quantifier**: dead, killed
by a compiled asymmetry (`E0004` against a warn-level lint). **It laundered a trusted-base fact into a provable
one.**

**The one-clause fix for the mutation gap** ("re-canonicalises on release", stated with no enforcement mechanism
named): superseded by the two-tier repair.

**File 80's exact fold-width construction, `AllOnes` recursing on the value of `P`**: superseded. It does not
exist above binary128, refusing at rustc's default recursion limit and then at `Nat::VAL`'s carrier once that
limit is raised, both accidental ceilings coinciding at 128 and neither stated anywhere.

**The byte-image chapter's own prior framing**, "an invertible external image takes the crossing contract's
statement structure verbatim" (two statements): superseded; the structure has three.

**`90b`'s division instinct, alternative 1, `Hot`'s cell placed on the `Door`**: dead, killed on four compiled
or silicon-read facts, and the fork's other alternative carried the identical smuggle at a different address
(section 1.13).

**The proposed door-side domain-preservation equation**: superseded. It guards a region the entry-level totality
refusal, ratified at the same checkpoint that adopted the equation, already forecloses unconditionally, **and a
check on a door guarding a precondition the type's own construction has made unconditionally true for every
value that can exist is the definition of a vacuous guard**, which belongs in the suite as a regression pin, not
in the ratifying text.

**File 94's reader-quantified replacement naming test**: superseded. It cannot fail in the hands of the person
running it, the identical defect it correctly diagnosed in the parity suite it struck.

**File 93's citation of section 1.3's second, weaker sentence** as the guard the `Door` placement violates:
corrected. The operative sentence is the first.

**File 98's periphery assessment**, "five or six crate-level subjects no panel file has ever examined", with an
unbounded error bar: false. The ground is op-ratified, distilled into the panel's required reading from its
first hour, and row-rechecked at file 74 before the claim was written.

**D10's own storage argument for rotors over matrices, `1 + n(n-1)/2` components**: wrong from rank 4,
reversing its own comparison against matrix storage at rank 7. The decision survives on the grounds it also
gives; the count does not.

**The capacity claim "checked to agree in an inline const block at the one construction door"**: false above
rank 0. The trait-method route bypasses the door entirely.

**The ratified sentence that the array grammar's pairing is "forced by the language, not chosen"**: **the second
clause is true and the first is false, twice** (section 1.27). Recorded as a correction to ratified text rather
than a droplist entry, and the replacement sentence is offered rather than adopted, because a call about
ratified text needs two independent agreements and the record has two compiles from two members but one design
reading.

**The ratified sentence that a datum-keyed digest "masks the container straight to the fields' own width"**:
corrected by one word to the placement map's **occupancy** (section 1.22). A widening rather than a
replacement: the two masks compute the same bit pattern for every numeral that exists today.

**The persona's third clause of the truth-contract shape**, that the fifteen declarations bind on the
exit-carrying part: **backwards**, ratified as such at `108b:136-141`. Binding them on the exit refuses the
multi-lane instance at the impl, and that instance is the entire thing the generic branch buys.

**`103:198-201`'s perimeter-rule citation on `Bool`**: struck. The rule's own Boundary section excludes a type
with no invariant, and a decorative citation is worse than none **because it makes a taste question look
settled**, and because the two grounds put the call in different hands.

**"Is there a second truth type? there is, and it is shipped" as the ground for the fork's lean**: does not
survive deleting its shipped-source citation. Replaced by the variety-closure theorem, with `MaskOps` demoted
to a witness.

**The claim that the derived-storage construction is new to this panel**: false. It is
`76_probes/b1_structural_array.rs`, recorded as WORKS with two controls, thirty-one files before it was
re-derived.

**The claim that the derived-storage construction costs a quadratic in the number of capacities**: false. The
type machinery is free; the cost was one operation per element from a structurally recursive `filled`, and
rewriting it as a provided method over the projected slice collapsed 3.24 s to 0.12 s.

**The five-way grouping of the route-multiplicity finding**: retired. It is three defects with three existing
owners plus one non-instance, and **route multiplicity is a defect only relative to a guarantee.**

**The candidate fifth clause on the pricing pillar** for the route question: not adopted. The three instances it
would govern are governed better by the pricing pillar, which names the repair where the clause names the
symptom; the fourth is two-organs; and the survival mechanism is the separation requirement, which would have
caught three of the four in advance and was not run. **The precedent decides against its own citation:** the
correct response to a requirement that works but goes unrun is a moment naming when it runs, not a new
requirement.

**`91:12-13`'s claim** that the definitional-completeness line and the separation requirement were applied to
everything the ninth consolidation absorbed: false, at three of its own sentences, grep-checkable. A fabricated
diligence claim, not a design error, in the same register as file 79's search sentence.

**File 79's own diligence sentence** ("I searched `[Aa]rity` across every file; the hits are all fold-arity"):
false, confirmed independently three times. The substance of its conclusion survives on grounds it did not
give.

**File 82's three offered resolutions for `quantize`'s apparent new failure kind**: none adopted; the premise
dissolved instead.

**The eleventh consolidation's completeness line scoped to "this document's own new prose"**: struck. The line
quantifies over the whole ratifying text, and the exemption removed exactly the population a restoration
consists of, which is where six undefined terms were sitting (`111:503-520`).

**`117:493-494`'s trusted-base sentence** that an unrelated `lowering` dependency makes the contract split's
mechanism go silently with no diagnostic anywhere: **false**, refuted by re-running the projection with
`Lowering` fully in scope in the same crate, where it still refuses at `E0220` and names its own repair
(section 1.25).

**Keying `Policy` and `Lowering` on the numeral, as the repair for a preset denoting two rows**: refuted at exit
0. Two `Implicit` numerals, one number kind, disagreeing on what `Warm` means, with nothing in the language
relating them, which is the layer-keying rule failing in its original direction (section 1.21).

**Per-kind preset markers, as the other repair for the same defect**: refuted at exit 0 and on ratified text.
`Number<Binary32, WarmFixed>` type-checks at a value position, and D52 makes compositions public and bindable so
aliases hide the mispairing only for the four pairs they cover; D72 gives `arvo-strategy` four named markers
that the eight-marker shape deletes (section 1.21).

**Renaming the `Precision` marker trait, as the repair for the bridge's token collision**: not taken. It
rewrites a name `74b` ratified under op's own "the mechanism unifies and the vocabulary does not", plus two
bounds and a three-member alias family, for no gain, and `16d:14-15`'s tiebreaker decides against it (section
1.23).

**A second ceiling on the total width, enforced by an emitted marker per row**: refuted three ways. It refuses a
legitimate product at precision 80; it cannot be narrowed to the written site, because a `where` clause on a
type alias is refused and its repair is an unvetted feature; and its refusal shows a consumer eleven numeral
trees and no number (section 2).

**File 119's reading that the bridge table's cost is linear**: does not survive the extension past 4096 rows.
Each doubling costs between 3.3x and 5.6x, so the curve is roughly quadratic, the same character the per-width
container dispatch already measured.

**File 121's reading that the braces in `NatOf<{ I }>` are a general requirement**: half right. They are
required exactly when the const parameter's name also names a type in scope, which it does here structurally,
and renaming the parameters compiles unbraced (section 1.23).

---

## 7. Reversals inside a stretch, recorded separately

**These are proposals born and retired inside the stretch that absorbed them.** They are not removals from a
standing base, and mixing them with section 6 is what made the droplist read as a standing record while
reporting on nothing that left it (`109:554-558`). Recorded here in brief, because they are worth keeping and
are not what a reader diffing two documents needs.

The eleven-crate taxonomy's own suggested changes, offered and folded in. The persona checkpoints' calls that op
individually corrected or superseded at `68b`, `78` and `108b`. File 95's finisher clause on the uncheckable
field, adopted and reversed within the stretch. File 94's replacement naming test, adopted and retired within
the stretch. The five-way route grouping and its candidate clause, proposed and argued down one file later. A
draft claim that the derived-storage construction was new, corrected before shipping. A draft claim that it was
quadratic, corrected by its own first measurement. Two draft negatives that failed their own greps and were
narrowed rather than shipped. A prediction that the placement composite needed stating, refuted by the optimiser
performing the collapse itself. The eleventh consolidation's own draft ordering of section 1.27's three columns,
which changed once the compile-time measurement was isolated from the constructing body. **And from the
repair stretch**: a brief's premise that two compile failures were live in the standing base, stale by one file
and reproduced anyway before being set aside; a proposal to renumber or prefix op's frozen register, declined
in the same file that evaluated it; a proposal to mint D-numbers for op's unnumbered decisions, declined for
the same reason; and a first test-gate run whose green result came from a pipeline's last stage rather than
from `cargo`, caught and re-run to a log within the same file.
---

