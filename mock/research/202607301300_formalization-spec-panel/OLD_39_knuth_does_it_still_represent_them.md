# 39. Does it still represent them

**Member:** Donald Knuth. I wrote file 01, the first deliverable of this review, thirty-eight files
ago; the design has been rebuilt several times since and I carry nothing from that file forward. My
habit is the one this dispatch names: a representability claim is worth exactly the construction
behind it, and a construction is worth exactly what was checked. So each standard below gets a
concrete construction in the standard's own vocabulary, compiled where a compile can carry it, and
where I found the merged shape unable to carry one, I say precisely which member fails, on which
witness, and what the repair costs. The other half of the dispatch, the two unread prior-art pieces,
I read in full rather than summarising from the eight files that flagged them; one of them turned
out to already contain the answer to a question two panel members had re-derived, worse, from
scratch.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: summed from the
per-binary lines rather than read off a headline, 654 passed, 0 failed, 9 ignored, matching the
counts files 31 through 38 each report, so nothing regressed under this dispatch. The shipped test
surface bearing on this stretch is `crates/arvo/tests/identity_laws.rs` plus the compile-fail pairs
under `crates/arvo/tests/ui/`; files 37 and 38 each read the body in full and reported the
full-matrix discipline kept, and I confirmed the ui directory carries the per-strategy
no-multiplicative-identity pairs both files describe rather than re-auditing a body two members
audited nine days apart with the same result. Canon gate: `26_consolidation_two.md`,
`30b_op_checkpoint_seven.md` and `34b_op_checkpoint_eight.md` govern, all read in full. The
representability standard I measure against is op's own, stated once in the consolidation and not
reopened: MATLAB, IEEE 754 and SystemC "not as inspirations but as a **test**, where an abstraction
that cannot express one of them is a defect rather than an accepted scope boundary" (`26:534-541`).
Nothing below overturns a D-numbered call or either checkpoint; one finding below shows a
convergence-stretch sentence contradicting a ratified spec sentence, and the ratified sentence wins.

**What I read:** `26_consolidation_two.md` in full. `30b` and `34b` in full. `35`, `37`, `38` in
full, `31` section 4 in full, per the brief. Reached into `30:74-134` (the contract's origin),
`36:195-236` (the normal form and the `Bias` derivation, read before I criticised it, so the
criticism is of what it says rather than of a paraphrase), `33:235-243` (the key table), and the
governing spec topic `mock/design_rounds/202607301200_topic.the-formalization-spec.md:105-121` (the
D61/D68/D69 axis definitions, which name MATLAB directly). The two prior-art pieces:
`mock/design_rounds/202607300800/202607291900_topic.the-number-systems-crate.md` in full (D38/D39,
op's calls inline), and `~/Dev/clause-dev/notko/docs/202607281547_design.notko-gains-the-hlist.md`
in full. The directory listed once: 38 numbered deliverables plus probe directories. One external
check: MATLAB's own documentation for the bias domain, because a finding rests on it (sources at the
end).

**What I compiled or measured, separated from what I reasoned.** Three probes in `39_probes/`, each
with a row and its negative control in `39_probes/OUTCOMES.md`, all against the workspace pin
(`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, confirmed from inside the repo). All three are
compile-time artifacts (const evaluation and type checking); no probe contains a timer, and no
number below is a runtime measurement. Probe 1 is arithmetic (exhaustive const assertions over
rational algebra). Probe 2 is a spelling probe: it establishes that the trait shapes stated at
`38:318-356` carry the constructions as types, using local stand-in traits mirroring that section,
and its evidentiary value is vocabulary-shaped rather than arithmetic (the arithmetic is files
25/33/35's compiled work, which I do not re-verify). Probe 3 is exhaustive const evaluation over a
model grade monoid. Sections 1 through 4 are compiled except where marked; sections 5 and 6 are
close reading of design documents, marked as such; section 7 is reasoning on all of it.

## 0. The verdict, stated first

**Files 35 and 37 pass the representability test, and each comes out strengthened by it.** The
`Widening` removal is not merely harmless to the three standards; the standards require the shape
that replaced it, and one MATLAB mode (`SpecifyPrecision`) is a construction the removed axis could
not name at all (section 4.1). The finest-view mechanism is not merely compatible with the
standards; each of its three detail levels is the exact carrier of a shipping standard's observable
side channel, which no earlier file had noticed and which probe 3 checks (section 4.2). And all
three standards state growth per operation rather than per numeral, which is external corroboration,
in the standards' own vocabulary, for exactly the half of file 35 its author marked reasoned rather
than compiled (section 4.3).

**One representability defect exists in the merged shape, and it is not from 35 or 37.** File 36's
sentence "So `Bias` is a signed integer" (`36:222`), carried into file 38's verbatim-intended
contract as "Bias is an Int" (`38:337-338`), makes a legal MATLAB numerictype unrepresentable:
slope 1, bias one half. MATLAB's bias "can take on any value" per MathWorks' own documentation, the
spec's ratified D68 sentence claims the scaling "in full"
(`202607301200_topic.the-formalization-spec.md:111-114`), and the design's own ratified
biased-multiplication closure formula (`31:397-400`) already computes in rational-bias algebra, so
the integer reading contradicts both the standard and the contract that carries it. Probe 1 compiles
the witness, the general boundary of what integer bias can reach, and the closure formula running
exactly over non-integer biases. The repair uses machinery file 36 itself built and costs no new
mechanism (section 1.3).

**The two prior-art pieces are read.** `arvo-num-systems` is a topic file carrying two of op's own
ratified calls (D38, D39); its agent-derived inhabits table is stale against the ratified identity
contract, its "inhabits, not equals" precision dissolves the vacuity worry two panel members
raised against D39 once it is read as the finest inhabited system, and its ratified dependency direction answers part of the open
crate-edge question at `26:505-510`. `notko-hlist` is a design note, no source; it contradicts
nothing this review has decided, it independently corroborates op's laws-stay-in-arvo call from
outside the panel, and the panel's two type-level number towers do not duplicate it, because a
Peano-shaped count abstraction and a heterogeneous list are different objects and the note's own
naming call assigns type-level naturals to arvo, which is what file 36 built (sections 5 and 6).

## 1. MATLAB Fixed-Point Designer (compiled, probe 1; spelling in probe 2)

### 1.1 The construction, in MATLAB's vocabulary

MATLAB's general scaling: a real-world value is `Slope * StoredInteger + Bias`, with
`Slope = SlopeAdjustmentFactor * 2^FixedExponent` and the adjustment factor in `[1, 2)`. Both slope
and bias "can take on any value" and are stored as doubles, hence are dyadic rationals in practice.
The carriers, member by member, in the merged shape at `38:318-356`:

| MATLAB | Carrier | Touched by 35/37? |
|---|---|---|
| `FixedExponent` | `Implicit<E, A, B>`'s `E` (`31:335`) | no |
| `SlopeAdjustmentFactor` | `Adjustment = Ratio<N, D>`, gcd-reduced (`36:199-217`) | no |
| `Bias` | `Bias` on `Implicit` | no, and see 1.3 |
| `WordLength`, signedness | `Precision` (D69) plus `Domain` | no |
| rounding methods (`Ceiling`, `Floor`, `Zero`, `Convergent`, `Nearest`, `Round`) | `Quantisation` directions (`TowardPositive`, `TowardNegative`, `TowardZero`, `ToEven`, and the two tie rules as midpoint directions) | no |
| `Saturate` / `Wrap` overflow actions | range-end `Resolution`s (clamp, `ReduceModulo`) | no |
| `ProductMode` / `SumMode` (`FullPrecision`, `KeepLSB`, `KeepMSB`, `SpecifyPrecision`) | call placements of `mul_full` / `quantize` (`26:236-243`), completed by the `Widening` removal, section 4.1 | yes, strengthened |
| `fimath` attached to the object | the composition's `S`: a strategy's `Mul`/`Add` impl is exactly the named call sequence, which is where file 35 puts the old axis content (`35:162-167`) | yes, carried |
| overflow logging (counts per operation) | the grade at the Exact detail level, probe 3, section 4.2 | yes, gained |

Probe 1's part (a) checks the decomposition itself: slope 3/8 normalises to adjustment 3/2 in
`[1, 2)` times `2^-2`, coprime, which is precisely the reduced-`Ratio` carrier file 36 shipped. A
dyadic-rational slope always lands on this carrier, and since MATLAB stores slope as a double, the
carrier covers everything MATLAB can itself represent. That is the honest scope of the claim: the
design represents MATLAB's model up to MATLAB's own storage, and an irrational slope, which neither
system can store, is representable in neither.

### 1.2 What is static in arvo and runtime in MATLAB

`fimath` is runtime-mutable data on a MATLAB object; arvo's `S` is a type parameter. The model (the
policy travels with the value) is carried; the phase (compile time against runtime) differs, and
that is the stack's standing global choice (sizes and policies const at type level), made long
before this review and not a cost introduced by 35 or 37. I state it so the table above is not read
as claiming more than it shows.

### 1.3 The defect: `Bias = Int` cannot carry MATLAB's bias, and the contract's own formula says so

File 36 derived the member's domain in one sentence: "`bias = B1 * B2` (`31:399-400`) is a signed
multiplication. So `Bias` is a signed integer" (`36:221-222`). The premise establishes signedness.
Integrality was assumed, not derived, and it is false against the standard the axis exists for. The
ratified D68 sentence defining the axis reads: "The value of a stored integer `k` is
`Adjustment * radix^exponent * k + Bias`, which is MATLAB's general scaling in full"
(`202607301200_topic.the-formalization-spec.md:111-112`). MATLAB's bias is unrestricted ("the slope
and bias can take on any value", MathWorks, sources below).

Probe 1, all const-evaluated, negative control committed:

- **The witness.** Slope 1, bias 1/2: a legal MATLAB numerictype whose value set is the
  half-integers. For every integer bias the value set is the integers; the two sets are disjoint.
  Checked exhaustively over integer biases -1000 through 1000, with the parity argument covering the
  rest.
- **The exact boundary.** `{A*k + B} = {A*k + B'}` iff `B - B'` is an integer multiple of `A`, so
  integer bias reaches exactly the biases in `Z + A*Z = (1/q)*Z` for `A = p/q` in lowest terms.
  Checked: for slope 3/8 the reachable subgroup is `(1/8)*Z`, bias 1/8 is reachable (from integer
  bias -1) and bias 1/16 is not, exhaustively over the same range. Denominating `Bias` in quanta
  instead of absolute units moves the reachable set to `A*Z` and fails on the same witness, so no
  choice of unit rescues the integer reading.
- **The contract already disagrees with the integer reading.** The ratified closure formula,
  `adjustment = gcd(A1*A2, A1*B2, A2*B1)`, `bias = B1*B2` (`31:397-400`), mixes adjustments and
  biases inside one gcd, which is only defined if they live in one algebra, the rationals. Probe 1
  instantiates it with biases 1/2 and 5/2 and checks closure exhaustively over 17 x 17 operand
  pairs: every product lands on `adjustment * k + bias` with integer `k`. The formula the contract
  ships computes in rational-bias algebra; `Bias = Int` is inconsistent with the contract's own
  section 4.6, not only with MATLAB.

**The repair, using only machinery file 36 already built.** `Bias` is a signed, gcd-and-sign
normalised rational: zero, or a sign (file 36's `Zpos`/`Zneg` shape) applied to a reduced
`Ratio<N: Pos, D: Pos>`. Value-uniqueness extends by exactly the normal form 34b already adopted and
ordered priced (gcd reduction plus one zero by construction); the multiplication the closure formula
needs is file 36's probe 6 signed multiplication composed with its probe 4 reduction. No new
mechanism, no new feature, and the compile cost is bounded by the type-level gcd file 36 already
measured at 5.08 ms per composition (`38:131-135`), paid only by biased compositions, which every
ordinary numeral (`Bias = Zero`) never instantiates. I have not built the trait-level composition of
sign with reduction; that is the one unbuilt piece, it is the same shape as what 34b already
ordered built for rational adjustments, and I flag it as work rather than risk.

I also note what this defect is an instance of, because file 38 named the pattern one file earlier:
prose about the design's own state, checked by nothing (`26:111-119`, `38:165-203`). The derivation
chain ran `31:399-400` (signed multiplication, correct) to `36:222` (signed integer, unsupported) to
`38:337-338` (an Int, verbatim-intended), and nobody along it re-read the D68 sentence that names
MATLAB as the axis's reason for existing. The representability test in `13c`/`26:534-541` is
precisely the check that catches this class, and this file exists because it had not been run over
the convergence stretch.

## 2. IEEE 754 (spelling compiled, probe 2; flags compiled, probe 3)

### 2.1 The construction, in the standard's vocabulary

IEEE 754-2019 separates a **format** (radix, precision, emin, emax: clause 3.2's parameters) from
its **encodings** (clause 3.5: decimal formats have two, BID and DPD, of one format) from the
**rounding-direction attributes** (clause 4, chosen per operation, not per format) from the
**status flags** (clause 7, sticky, accumulated). The merged shape carries each on a distinct
member, and probe 2 spells the sharpest case, decimal32, directly:

- **Format**: one `Numeral` with `Radix = Ten`, `Precision = 7`,
  `Ranged<-95, 96, Gradual, Ieee754Specials>`, `Domain = Symmetric`. This is the radix-ten
  instantiation the ratified contract reserves (`31:329` "2 and 10 instantiated") and that closed
  the consolidation's radix item.
- **Two encodings of one format**: `Bid` and `Dpd` as two `Encoding` instances (`Fields` differing)
  under two `Lowering`s over the **same** `Numeral`. Probe 2's check is a function whose signature
  demands a shared numeral and distinct encodings; it type-checks. The `Widening` removal did not
  touch `Encoding`, so the separation that motivated the whole identity/policy/lowering split
  survives the collapse intact.
- **Cohorts and preferred exponent**: `Encoding::Canonical` (`31:357` lists preferred cohort by
  name). Untouched.
- **Rounding-direction attributes**: `Quantisation` on `Policy`, separate from the numeral, which is
  the separation the standard mandates. The standard additionally permits a *dynamic* rounding mode;
  arvo's is static per composition or call site, the same phase difference as 1.2, standing and not
  new.
- **Correctly rounded operations**: exact operation then one quantisation for add/subtract/multiply
  (the exact intermediate of two finite-precision operands is finite-precision, and it is a named
  numeral post-35); division and square root defined as fused correctly-rounded operations with no
  intermediate (`26:317-320`), which the `Widening` axis never carried, so its removal costs the
  IEEE construction nothing anywhere.
- **formatOf operations**: the standard's own name for operations whose destination format is part
  of the **operation**, not of the operands. Section 4.3 takes this up; it is the standard agreeing
  with file 35's section 2.2 in so many words.

### 2.2 The flags, which the design now represents better than before file 37

IEEE's five status flags are sticky booleans, ORed across every operation since the last reset.
Probe 3 checks, exhaustively over 256 x 256 grade pairs at multiplicities 0 through 3, that the
Presence projection out of file 37's grade monoid is a monoid homomorphism: the flag register of a
compound term is the pointwise OR of its parts' registers, order-free by commutativity. So **IEEE's
flag register is the image of the grade under the Presence view**, not a mechanism the design would
need to add. The invalid-operation case, which delivers a NaN *and* raises the flag, is file 37's
reification with the grade kept: probe 3 models a refusing and a reified operation with identical
grades and different value presence, and every view is blind to the difference, which is exactly the
standard's semantics (the flag does not record whether a payload was substituted). File 37's
out-of-set absorbing special hypothesis (`37:269-274`) is precisely NaN: outside the numeric value
set proper, absorbing under arithmetic, which is why the Kleene verdicts survive it.

## 3. SystemC fixed point (spelling compiled, probe 2; flags compiled, probe 3)

### 3.1 The construction, in the standard's vocabulary

`sc_fixed<W, IW, Q, O, N>`: total wordlength, integer wordlength, quantisation mode, overflow mode,
and the saturated-bits count. Probe 2 spells `sc_fixed<8, 3, SC_RND, SC_SAT>`: a `Numeral` with
precision 8 and implicit exponent -5 (the quantum `2^(IW - W)`), `Domain = AsymmetricLow` (two's
complement), and a `Quant` instance with the midpoint direction `TowardPositive` (SC_RND rounds
ties toward plus infinity) and clamp at both ends. The mode table, each an instance of the existing
vocabulary: SC_TRN is `TowardZero`-shaped truncation toward negative for two's complement (the
preset-table ambiguity the consolidation already flags at `26:288-295` applies here identically),
SC_RND_CONV is `ToEven`, SC_WRAP is `ReduceModulo`, SC_SAT_ZERO is `SubstituteZero`, and SC_SAT_SYM
is **not** a quantisation but a `Domain` (a symmetric numeral), which is file 31's own settled
reading (`31:94-112`) and which probe 2 records as a comment where a reader would otherwise reach
for a sixth resolution.

**One corner of the ratified no-gaps claim is constructed nowhere.** The consolidation's sentence
"this vocabulary reproduces every named rounding and overflow mode in IEEE 754, SystemC and MATLAB's
Fixed-Point Designer with no gaps" (`26:48-50`) is ratified and predates files 35/37, and for
`SC_WRAP<n_bits>` and `SC_WRAP_SM<n_bits>` with `n_bits > 0` (wrap while keeping the top `n` bits
saturated, and sign-magnitude wrap) I found no file in the review that builds the construction. I
did not build it either; it is one cheap probe, it belongs to whoever next touches the quantiser,
and I flag it rather than counting it against 35/37, which did not touch the quantiser at all.

### 3.2 Deferred quantisation, which is the removal's own shape

SystemC computes expression interiors at expanded precision and fires quantisation and overflow
handling at assignment to a declared variable. That is, member for member, the post-35 shape:
`mul_full` into a named product numeral, `quantize` at the store. Probe 2's part (c) spells both
call placements (SystemC's at-the-assignment, MATLAB's per-operation) as two placements of one pair,
which is the consolidation's own verified result (`26:236-243`) restated as types with **no axis
anywhere**, since the axis is gone. SystemC's global bound on intermediate wordlength (the
`sc_fxtype_params` context) is a runtime vehicle for a bounded-intermediate model; the model is
expressible as an explicit `quantize` at the chosen depth, and the runtime context itself is the
same phase difference as 1.2, noted and not new.

### 3.3 The per-variable flags

`sc_fxnum::overflow_flag()` and `quantization_flag()` report whether the last assignment overflowed
or quantised: the Presence view again, read per assignment rather than accumulated. Probe 3's
homomorphism covers it (a single assignment's grade is a join over the expression it stores).

### 3.4 A perimeter fact about the compile gate, found by a failed negative control

While pinning probe 2's refusal (`ReduceModulo` at the midpoint slot), my first negative control, a
bare `type Illegal = Quant<ReduceModulo, ...>` alias, **compiled clean**, because Rust does not
enforce a struct's bounds at a type-alias declaration, only at use sites. The corrected control in a
fn-parameter position refuses with E0277 as the consolidation's gate describes (`26:47-48`). The
consequence for the spec is one sentence: the resolution-position gate holds at every position where
the type is *used* and not at an alias that merely names it, so a preset table written as bare type
aliases exercises nothing until something consumes the aliases. This is the observation-surface
reading of a guarantee (file 10's rule) arriving at the quantiser: the gate's perimeter is the set
of checked positions, and an alias is not one. Recorded in the probe header per the panel's
practice of keeping the control the compiler killed.

## 4. What the two changes actually do to representability

### 4.1 The `Widening` removal is required by the standards, not merely tolerated

The old axis had three instances: `None`, `InContainer`, `PerOperation` (`35:74-79`). MATLAB's
`SpecifyPrecision` product mode quantises the exact product into a **third, consumer-chosen**
numeral, neither the operand's own container nor a doubled copy of it. That destination is not any
of the three instances; the axis could classify where a widened value lives only relative to the
operand's own storage, and a mode whose whole content is "the destination is an arbitrary other
numeral" had no spelling. Post-35, it is one line: `quantize::<Prod, Spec, _>(mul_full(a, b))`,
probe 2's `matlab_specify_precision`. SystemC's per-assignment model likewise never fit the axis
(the intermediate lives in the expression, not in any operand's container) and fits the named
intermediate exactly. So section 1.3 of file 35, which argued the axis's job was already done by
machinery built for other reasons, understated its own case: for two of the three standards the
axis was not redundant but **insufficient**, and the replacement is the first shape that represents
them. That is the strongest form of the representability test's answer: the removal did not
preserve expressiveness, it supplied expressiveness the ratified table lacked.

### 4.2 The view lattice's three detail levels each carry a shipping standard's observable

File 37 justified the three levels (Ignore, Presence, Exact) internally, from the lattice needing
them (`37:360-366`). The standards test adds the external half, probe 3: IEEE's sticky flags and
SystemC's per-variable flags are the Presence level of both generator classes, and MATLAB's
overflow logging (counts per operation) is the Exact level. Every nontrivial detail level in the
mechanism is the exact carrier of an observable some shipping standard already hands its users, so
none of the nine views is decoration and the rich-index worry file 37 took head on (`37:407-414`)
has a second, external answer: the index's points are not invented, three of them ship today under
other names. This also supplies a concrete input to the evaluation-strategy sentence file 37 left
owed (`37:227-242`): all three standards' semantics are strict-evaluation shaped (an operand's
flags are raised by the operations that computed it, regardless of what its sibling does), so the
representability test tilts the owed sentence toward the strict reading. An input, not a ruling;
the fold combinators are the design's own object and op's call.

### 4.3 The standards state growth per operation, which is the evidence file 35's reasoned half lacked

File 38 correctly returned "`Growth` leaves `Policy`" to the trusted-without-artifact bin, because
file 35 marked it reasoned rather than compiled (`38:146-150`, `35:309-310`). The check file 35
named was "whether any consumer-facing operation exists, or could exist, whose growth behaviour
genuinely cannot be read off from (which primitive, which target numeral)" (`35:243-246`). The
three standards answer it from outside, in their own vocabulary:

- IEEE 754's **formatOf** operations put the destination format in the operation's name; the
  operand formats do not determine it, and no per-format attribute carries it.
- MATLAB's product and sum wordlength rules are stated as facts about the multiply and the sum
  (`ProductWordLength = W1 + W2` and its mode variants), never as a property of one operand's type
  alone.
- SystemC's expression precision is per expression node, with the variable's own `W/IW` mattering
  only at the assignment that quantises.

Three independent standards, designed decades apart for exactly this domain, each place growth on
the operation's signature and none on a unary type property. This is not a compile and I do not
move the item's bin: it remains reasoned. But the reasoning is no longer only a type-shape argument
from inside the design; it is the design's own external test pointing the same way, and I know of
no cheaper corroboration op could ask for short of the exhaustive operation-surface search file 35
declined. The ratification tick at `38:251-253` should carry this paragraph with it.

## 5. `arvo-num-systems`, read (close reading, not compiled)

The piece is a topic file, `202607291900_topic.the-number-systems-crate.md`, carrying two ratified
calls with op inline: **D38** (a crate for number-system membership: ℕ, ℤ, ℚ, ℝ, ℂ, ℍ, 𝕆,
surreal, hyperreal, p-adic, shipped "even if nothing uses them yet", the vocabulary "fixed by
mathematics rather than invented here") and **D39** (membership defined through algebraic
structure, `arvo-num-systems` depends on `arvo-algebra-contracts`, so the algebra ladder is
upstream work). Both are op's; the literature grounding and the inhabits table are marked the
agent's. Four findings from actually reading it:

**The topic contains the reading that dissolves the vacuity worry, and the dissolution has the
same shape file 37 gave the relation question.** `30b:18-28` holds D39 against "two readings
finding that its stated mechanism does not compile and that a membership predicate over the whole
ambient set is vacuously true of everything". The topic's own precision paragraph
(`202607291900:80-84`) states the predicate: "the predicate is **inhabits**, not **equals**.
`Natural` asserts that every value of the type is a natural number, not that the type represents
all of ℕ", the reading "that makes the bound decidable and composable". Stated exactly: inhabits
discriminates at the lower rungs (`UFixed<8, 0>` is `Natural`, `UFixed<8, 4>` is not) and is
indeed vacuously true at any system containing all of a type's values, ℝ included, so the vacuity
finding is correct about the top of the tower and wrong as a verdict on the predicate. The honest
derived fact is the **finest inhabited system**, which exists and is unique because the tower is a
chain, and that is the identical move file 37 made for the laws: do not choose a relation, report
the finest one that holds. The panel had the pattern in hand and the answer to D39's content
sitting unread in the round directory at the same time, and connected neither to the other. That
is the concrete cost of the eight-file unread flag, and it is what the flag was warning about.

**Membership is a derived fact keyed on `Numeral` members, which is machinery the panel already
built.** Inhabits-membership is decidable from the identity axes alone: `Natural` iff
`Domain = NonNegative` and the quantum and bias are naturals; `Integer` iff quantum and bias are
integers; dyadic iff radix two, unit adjustment, dyadic bias; `Rational` for any reduced-`Ratio`
adjustment and rational bias. That is precisely the shape the review ratified for every other
derived fact: a `const fn` (or derived marker) whose parameters are its key (`26:174-186`), with
the blanket-impl-over-member-conjunction mechanism of the algebra ladder (`26:188-201`), reported
as the finest inhabited system per the paragraph above. The spec already contains the first
instance without naming it as membership:
`impl<N: Numeral<Bias = Zero>> AddClosed for N {}`
(`202607301100_topic.the-formalization-talk.md:1543`). So the panel is not duplicating
`arvo-num-systems`; it has been building the prerequisite D39 names ("the const assertions and
typestate machinery that statically prove membership", `202607291900:76-78`), and the crate is a
consumer of the ladder, exactly as D39's dependency arrow says. This is also a candidate answer to
`30b`'s open "what membership can honestly promise": the honest content is inhabits, derived from
the numeral's own members, licensing exactly what the membership's defining structure licenses. One
reading among the ones held; I do not resolve D39's content here, and a second member's independent
read of the same topic is owed before anything builds on this paragraph.

**The agent-derived inhabits table is stale against the ratified contract, in three ways.** The
table (`202607291900:97-103`) says every fixed-point type inhabits ℤ[1/2]. Against `31:328-333`:
a `Radix = Ten` numeral (ratified: "2 and 10 instantiated") inhabits ℤ[1/10], not ℤ[1/2]; a
`FullRange<F>` adjustment (D61, quantum `1/255`-shaped) leaves the dyadics entirely; and a rational
bias (section 1.3 above) does too. The rows were correct for the axis set that existed on
2026-07-29 and are narrower than the contract ratified since. D38 and D39 themselves are untouched
by this; the table is marked as the agent's derivation and it is the part that goes stale, which is
the provenance ladder working as intended. The crate's eventual design should derive its table from
the `Numeral` members per the previous paragraph rather than fixing the rows by hand a second time.

**The ratified dependency direction partially answers an open packaging question.** `26:505-510`
holds open whether the algorithm crates gain an edge onto `arvo-algebra-contracts`, noting "the
crate does not exist yet" and "nobody has made this call". For `arvo-num-systems` the call **is**
made, by op, in D39: the algebra-contracts crate exists in the ratified plan and has at least one
ratified downstream edge. The algorithm-crate edge remains open, but the consolidation's framing
("a layering decision, not a fact derivable from the mathematics") should cite D39 as the first
fixed point of that layering rather than treating the crate as hypothetical.

## 6. `notko-hlist`, read (close reading, not compiled)

The piece is a design note in the notko repository (no `mock/` there, deliberately), recording
decisions from an arvo round: `Empty`/`Cons` as the one shared heterogeneous type-level list that
three repositories independently reinvented, with `Cardinal` (a const trait: `ZERO` and `succ`,
the count type being each consumer's own, for orphan-rule reasons the note derives), `Length`,
`#[marker]` `Contains`/`ContainsAll` with the diagnostic attributes carried over from
hilavitkutin, and `Concat`. **No source exists**; the consolidation's "does not exist as shipped
source yet" (`26:663-665`) is confirmed by the tree (the notko repo has no such crate directory).
Findings:

**No contradiction with anything this review decided, and one outside corroboration.** The note's
sharpest boundary sentence: notko gets the **structural** folds (`Length`, `Concat`,
`ContainsAll`), and "a value-level fold that reduces with an identity and an associative combine
needs the algebra, so it belongs where the algebra is", naming `arvo-algebra-contracts`
(`202607281547:80-88`). That is op's laws-stay-in-arvo call (`26:543-547`) reached independently,
from the placement side, in a different repository's design note. Mutual agreement of unratified
artifacts is not corroboration when they copy each other; this one is dated before the panel's
convergence stretch and reasons from the orphan rule rather than from the panel's framing, so it is
as close to independent as the workspace offers.

**The panel's number towers do not duplicate it.** File 36's `Nat`/`Pos`/`Int` are type-level
numbers, not lists; `Cardinal` is a value-level count abstraction over a consumer's own type, not a
Peano tower. The note's naming call even assigns the territory: "`Natural` ... claims a
mathematical primitive, which is arvo's territory ... It also leaves `Natural` free for arvo"
(`202607281547:67-71`). File 36 building the type-level naturals in arvo is that call being
honoured, not a fourth reinvention of the list.

**Where it still bears on this review, concretely.** Two sites. `Specials` is a set of special
values on `Ranged` (`31:336`); if it ever grows past a closed enum of preset bundles (IEEE's set,
none), a type-level set of markers is its natural shape and the note's `Contains`/`ContainsAll`
with `#[diagnostic::on_unimplemented]` is the shipped-quality mechanism for it, including the
`recursion_limit` lesson. And any future per-cause refinement of the published grade (file 37's
mechanism deliberately collapsed to two generator classes with declared consts, which is why the
pressure shrank since the consolidation flagged the pair at `26:661-666`) would be a type-level set
again. Neither site needs it today. The note's `Cardinal` needs `const_trait_impl`, which the stack
already gates, and `#[marker]` sits on the watch list (`marker_trait_attr`), both already priced by
existing policy. My recommendation matches file 38's: the next member who builds a type-level set
starts from this note rather than from a fifth `struct Cons`.

## 7. The deltas for the third consolidation, stated to be taken close to verbatim

1. **`Bias` is a signed, normalised rational, not an integer.** In the contract block at
   `38:336-339`, replace "Bias is an Int: one zero, by construction" with: `Bias` is zero or a
   signed reduced ratio (an `Int`-signed `Ratio<N: Pos, D: Pos>` with `Gcd<D, Out = H>` on the
   magnitude), one zero and one spelling per value by the same normal form as `Adjustment`;
   `Bias = Zero` is every ordinary numeral and instantiates none of it. Reason: MATLAB's bias is
   unrestricted and D68 claims the scaling in full; witness slope 1, bias 1/2 (39 probe 1). The
   droplist gains: "`Bias` as a type-level integer (36:222): refuted against MATLAB's bias domain
   and against the ratified closure formula's own rational algebra, 39 probe 1."
2. **The representability table for the two removals.** `Widening`'s removal is required by MATLAB
   `SpecifyPrecision` and SystemC per-assignment quantisation, which the removed instance set could
   not name (39 section 4.1, spelled in probe 2); `Growth`-per-operation is how all three standards
   state growth (formatOf, ProductWordLength, per-expression precision), carried as external
   support for ratification tick 3, still reasoned rather than compiled (39 section 4.3).
3. **The standards' observables are views.** IEEE 754 clause 7 sticky flags and SystemC's
   per-variable flags are the Presence view of the grade; MATLAB's overflow logging is the Exact
   view; IEEE's deliver-NaN-and-raise-invalid is reification with the grade kept (39 probe 3,
   exhaustive at multiplicities 0 through 3). Each nontrivial detail level of the view lattice
   carries a shipping standard's observable, and the evaluation-strategy sentence the design owes
   gains a standards-side input: all three are strict-evaluation shaped.
4. **The quantiser gate's perimeter.** The `ReduceModulo`-at-midpoint refusal fires at checked use
   positions and not at bare type aliases (39 probe 2's killed control); a preset table of aliases
   exercises the gate only when consumed. One sentence next to the gate's statement.
5. **The unread pair is read.** The consolidation's standing line at `26:661-666` closes as
   follows: `arvo-num-systems` is D38/D39, op-ratified, its mechanism is the review's own
   derived-fact machinery keyed on `Numeral` members (first instance already in the spec at
   `202607301100:1543`), its agent-derived inhabits table is stale against the ratified contract
   and should be derived rather than re-fixed, and D39's dependency arrow is the first ratified
   edge onto `arvo-algebra-contracts`. `notko-hlist` is a design note with no source,
   contradicting nothing, corroborating laws-in-arvo from outside, wanted at exactly two future
   sites (`Specials` as a set, any per-cause grade refinement) and duplicated by nothing the panel
   built. The inhabits reading is offered as a candidate for D39's honest content and needs a
   second independent read before anything builds on it.
6. **The one unconstructed cell of the no-gaps claim.** `SC_WRAP<n>` / `SC_WRAP_SM<n>` with
   `n_bits > 0` has no construction anywhere in the review; one probe, owed by whoever next
   touches the quantiser (39 section 3.1).

## 8. What this file does not decide

The five ratification ticks of `38:244-262` remain op's; this file adds evidence to ticks 1 and 3
and adds the `Bias` correction as a sixth tick sharing 34b's already-adopted normal-form
obligation. D39's honest content is offered a candidate reading, not resolved, and carries an
explicit second-read requirement. The evaluation-strategy sentence stays unchosen; section 4.2 is
an input. The `SC_WRAP<n>` cell, the dither/`Refuse` choice, the `TotalOrd` annotation, division,
the real-consumer compile-cost bench, and the four missing codegen regression tests stand exactly
where file 38 left them. I built no trait-level signed-rational `Bias`; the repair in 1.3 names its
parts and its cost source but is design, not a compiled artifact, and by this review's own record
the first compile of any new piece finds a hole, so it should be compiled before the consolidation
carries it as more than a stated member.

## 9. Open, net

Closed by this file, each with an artifact: whether the two removals cost the design any of the
three standards (no, and probe 1/probe 2/probe 3 plus sections 1 through 4 are the constructions
the brief asked for); the standing unread flag at `26:661-666`, spent, with findings; the vacuity
worry against D39, located as an artifact of the ambient-set reading the topic itself had rejected.
Opened by this file, one defect with its repair attached (`Bias = Int`, section 1.3) and one owed
probe (`SC_WRAP<n>`, section 3.1). The net motion is toward settled: the merged shape at 38 section
5 needs one member corrected and otherwise passed the design's own hardest stated test.

## 10. Standing

Nothing here overturns a D-numbered call, `30b`, or `34b`; the one correction is of a
convergence-stretch derivation (`36:222`) against a ratified spec sentence
(`202607301200:111-112`), and by the provenance ladder the ratified sentence wins without my
needing to be right about anything else. Where I contradict a panel file I name the line and the
artifact: `36:221-224` and `38:337-338` against probe 1. Where I strengthen one, the strengthening
is section 4's, and it is the kind the checkpoint asked for: the standards test was the design's
own stated standard, it had not been run over the two changes under examination, and run honestly
it returned not the neutral "still representable" the brief feared was the best case, but
"representable only now", which is the pleasant surprise a test is for. My first negative control
for probe 2 was killed by the compiler (a type alias checks nothing), and it is kept in the probe
header and in section 3.4, because a control that never failed is not evidence that it was
controlling anything.

Sources for the MATLAB bias domain: [Compute Slope and Bias](https://www.mathworks.com/help/fixedpoint/ug/slope-bias-scaling.html)
("the slope and bias can take on any value"), [numerictype](https://www.mathworks.com/help/fixedpoint/ref/embedded.numerictype.html)
(SlopeAdjustmentFactor in [1, 2), automatic renormalisation).
