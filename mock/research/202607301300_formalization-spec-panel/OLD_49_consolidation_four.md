# The current shape, fourth consolidation: the spine rule, the carrier rule, and what they closed

File 40 stood as the reference after the algebra sub-dive that file 12 opened and files 27 through 39
finished. This document replaces it. Eight deliverables ran since (files 41 through 48), read against
two checkpoints: op's tenth (`44b`), and a fifth-through-ninth-shaped set of calls made overnight by
op's own persona, dispatched at Fable tier while op slept, recorded as `48b`. They close three threads
40 left open (the bias defect, held pending a repair; division's no-finite-accumulator prediction,
reasoned and never compiled; the seal, believed closed twice and open both times) and, in closing them,
produced two design rules that between them decided four further open items nobody had connected.

The shape of the stretch: file 41 (Chlipala) built `Bias` as a rational, sealed, and found `Adjustment`
itself had no seal, a hole nobody had flagged. File 42 (Arntzen) found the seal file 41 recommended did
not reach the actual hole, which sat one layer below in `Pos`/`Nat` as everything composes with them,
closed that, and built the generic multiplication trait file 41 had argued could not exist. File 43
(Smith) tested the consolidation's own no-finite-accumulator division prediction and found it false in
the ratified coordinates, true in the coordinates it was originally written in, and sharpened into a
third growth class; it also built the exact-division-by-constant subfamily and corrected the overflow
band for division. File 44 (Ringer) swept the review for claims that survived a coordinate change
without being re-derived, found a third instance of the defect the other two already exhibited, a
near-miss that checked out on inspection, and a reopened item, then proposed a grounding convention.
File 45 (Leroy) built the ground registry, widened its vocabulary, and backfilled every load-bearing
claim in the third consolidation against it. File 46 (Peyton Jones) found the seal open a third time,
on `Adjustment`, in the copy of the tower everything actually composes with, and closed it with a
compiled adversary enumerated by introduction route rather than by attack anyone happened to think of.
File 47 (Muratori) did something no member had done in thirty prior files: wrote the consumer's code,
found a real ergonomic defect in how a number is spelled and a real defect in how a fold's grade is
declared, and proposed fixes for both plus the evaluation-strategy sentence. File 48 (Giesen) attacked
47's proposals, found one supporting citation wrong and repaired it into a stronger argument, found a
missing half of an algebra and built it, and assembled the whole stretch under one already-stated rule.

Op ratified the value-unique encoding in full (the rational bias included) and held division for a
later stretch at the tenth checkpoint, then slept and handed the remaining checkpoints of the night to
his own persona by explicit instruction, recorded in full in `48b`. The persona made five calls. Four
land in this document as the panel's strongest available answer, clearly marked as persona-decided
rather than op-decided, because a persona call carries less authority than op's own and dies the moment
op reads it and says otherwise. The fifth the persona declined outright as genuinely op's to make.
Section 2 states which is which, in the words each checkpoint used.

**Verification.** Every claim below tagged compiled or measured was run against one pinned toolchain:
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, resolved from the repo's
`rust-toolchain.toml` pin (`channel = "nightly-2026-05-28"`). The design surface this review builds has
no shipped source: `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` from the repo
root returns nothing (exit 1), and the same command with `FullRange\|UTerm\|AddWidth` in place of the
first pattern is likewise empty, backing the rewrite-cost-near-zero claim in section 1.18. This is the
corrected form of a command four files (41 through 44) inherited with a wrong path (`crates/` does not
exist at the repo root; the shipped crates live under `mock/crates/`); file 45 found the drift and file
47 repeated it one file later, which is why the corrected command lives here rather than only in a
finding section. `cargo test --workspace`, run from `mock/`, has reported 654 passed, 0 failed, 9
ignored, summed per binary rather than trusted from a headline, identically across every one of files
41 through 48. Every probe directory under `41_probes/` through `48_probes/` carries an `OUTCOMES.md`
with the verbatim build commands and error text that reproduces its file's claims.

## The two rules

Two design rules were not the headline result of any single file. They were named once, by the persona
checkpoint, as the stretch's real product, and the claim is correct: apply either rule to a question
this review had been carrying as open, and the question closes.

**The spine rule.** A quantity that is computed and then has to appear in a type is a type. A quantity
that only ever has to be read is a const. Stated first, understated, by file 47 (`47:503-508`): "in this
design, a quantity that has to be computed and then appear in a type is a type, and a quantity that
only has to be read is a const." File 48 corrected its reach: the rule is not new at that point, it is
what op's tenth checkpoint had already ratified for the width chain and the biased-product formula
(`44b`, "the encoding is ratified"), restated in general form and, when taken seriously, forces open a
fork the review had been carrying as merely unopened. Applied to the fold's published grade, it says the
grade must be a type, not a caller-declared const parameter, which dissolves the `generic_const_exprs`
wall file 37 had reported as a hard limit of the design (it was a limit of const-land, not of the
design; file 47 built the projection, file 48 hardened it). Applied forward, it says the exponent bounds
must become a type the moment the exact-widening family reaches a `Ranged` numeral, because the exact
product's exponent bounds are computed from the operand bounds and have to appear in the result
numeral's type (section 1.15). Two closures from one sentence, and the rule is the same one, run twice,
by two different files that did not know they were running it.

**The carrier-at-birth rule.** A closed vocabulary that a guarantee quantifies over owes its seal and
its adversary at birth, not after three passes. Stated by file 48 (`48:2.3`) from a controlled
comparison the review ran without meaning to: the value-unique numeral tower needed four separate
passes to close (file 36 built it unsealed and orphaned its own seal demonstration; file 41 sealed
`Bias` and missed that `Adjustment` had no seal at all; file 42 found the real hole one layer below,
in `Pos`/`Nat`, and closed it without carrying file 41's `Adjustment` finding into the fix; file 46
found `Adjustment` still open in the fixed copy and closed it a third time, with a checklist rather than
an attack). The fold's `Grade` type, built one file later under the same principle from the start (file
48's own `probe_2b`/`probe_2c`, two lines, zero cost), took one pass. Four passes against one is the
argument, and it is a measured comparison, not an assertion: the checklist (a private supertrait closing
the direct-impl and named-supertrait routes; every parameter of every impl, including the seal's own
blanket, re-bounded on carrier traits so a laundering route through an upstream blanket cannot mint a
foreign inhabitant) is two lines per carrier, and running it once, at declaration time, is cheaper than
running an attack hunt after the fact, however many times.

Both rules are stated in full, with their compiled evidence, in sections 1.11 (the encoding), 1.12 (the
seal), and 1.14 (the grade), where each first earned its place. They are named here because the next
member reading this document should apply them prospectively, to `SignDomain`, `SignIndexing`, the view
lattice's nine points, and any future closed vocabulary a guarantee quantifies over, rather than wait
for a third or fourth pass to find the hole the checklist would have prevented.

## 1. The agreed shape

### 1.1 What a number is

Unchanged from file 40. A value of `Number<N: Numeral, S>` is an integer k, drawn from a finite
interval, together with a type-level rule injecting k into a set of rationals (plus, for floats, a
handful of data that are not rationals at all: `Specials`). The numeral names two things, the
representable set and the indexing, and D69 (op, `30b`) put them on two different sides of the design:
identity is parameterised in mathematical coordinates (precision, exponent bounds), not encoding
coordinates (total width, hidden bit, field encoding, which are derived).

### 1.2 The identity contract

```rust
pub const trait Numeral {
    type Radix:     Radix;        // 2 and 10 instantiated; any r expressible
    type Precision: Precision;    // significand digit count, primitive (D69), a Nat
    type Exponent:  ExponentForm; // where the exponent lives; nests the rest
    type Domain:    SignDomain;   // NonNegative | Symmetric | AsymmetricLow, a value fact
}

pub struct Implicit<const E: Exponent, A: Adjustment, B: Bias>;
pub struct Ranged<const EMIN: Exponent, const EMAX: Exponent, U: Underflow, S: Specials>;
```

Four top-level members, with `Adjustment`, `Bias`, `Underflow` and `Specials` nested where they mean
something. `Radix` closes the radix gap that would otherwise leave decimal64 (a radix-ten instance)
inexpressible. `Sign` splits into `SignDomain` (a value fact, on `Numeral`) and `SignIndexing` (a datum
fact, on `Encoding`, section 1.3); the split is what makes `SC_SAT_SYM` not a saturation mode (the
identical `TowardNegative` clamp delivers `-8` under `AsymmetricLow` and `-7` under `Symmetric`).
Nesting stands on the `Underflow` argument alone (`Underflow` has no bottom under a constant exponent,
so it nests); block floating point was withdrawn as evidence for the nesting shape, since a BFP
mantissa's value depends on a shared exponent stored outside the value it parameterises, which makes it
a different kind of object, not a branch of `ExponentForm`.

`Bias` and `Adjustment` are both now signed, gcd-normalised rationals, value-unique and sealed. Section
1.11 is the full statement; every reference to either member elsewhere in this document assumes the
rational, sealed shape ratified at `44b`.

### 1.3 Encoding, nested inside Lowering

```rust
pub const trait Lowering {
    type Encoding:    Encoding;
    type StoredWidth: StoredWidth;
    type Layout:      StorageLayout;
    // Widening: RATIFIED removed. See 1.10.
}

pub const trait Encoding {
    type SignIndexing: SignIndexing;      // Unsigned | TwosComplement | SignMagnitude | OnesComplement
    type Fields:       FieldLayout;       // field widths, hidden bit, encoding bias, reserved codes
    type Canonical:    Canonicalisation;  // signed zero, preferred cohort, NaN canonicalisation
}
```

`Lowering` changes no value. `Encoding`, nested inside it, may change which datum carries a value. Every
operation whose result depends on that is a datum-level operation, and no law may read one: a law's key
is a `const fn` parameter list, `Lowering` is not a parameter, and reading it fails `E0425`/`E0433` at
the point of use. Measured: a trivial `Canonicalisation` adds zero instructions; a trivial `FieldLayout`
produces byte-identical code to a hand-rolled shift-and-mask extractor; a richer `Canonicalisation`
costs a small, branchless, measured constant. Whether every plausible `Canonicalisation` instance stays
branchless is unmeasured (section 3).

### 1.4 The crossing contract

Three statements, each a per-value claim over a finite datum set, model-width exhaustive:

1. `decode ∘ encode = id` on values, always.
2. `encode ∘ decode` is idempotent on data, always. This is canonicalisation.
3. `encode ∘ decode = id` on data iff the encoding is injective, a derived boolean.

`Canonical` is load-bearing twice: as this contract's idempotent statement, and as the definition of the
equality every algebraic law is stated under (section 1.7).

### 1.5 The quantiser

Round first, classify second: round on the unbounded-exponent extension of the grid by the direction
triple, then classify the rounded result against the range (including `Specials`, where present) and
resolve by the range rules. Infinity does not remove the over-range position, it changes what its
neighbour is; the midpoint that decides ties-to-even overflow lives on the unbounded grid this amendment
supplies. Verified against IEEE 754-2019 clause 7 directly (file 39): roundTiesToEven, roundTowardZero,
roundTowardPositive agree exactly with oracles written independently from the standard's text.

Dither is a zero-state extra argument: `quantize_dithered(exact, noise) = quantize(exact + noise)`,
const-callable and stateless by construction. It does not compose with `Refuse` as stated; confining a
dithered value to the numeral's range before quantising restores totality at the cost of uniformity near
either end. Whether the design confines by default or gates the dithered entry point on totality is
undecided (section 3). Shaping (error feedback) is a scan, not a fold; the one genuinely shaped fold
(compensated summation) gets its own named combinator (`fold_compensated`) beside `fold`, the same
two-names-different-bounds idiom the fold surface now uses a second time for the sequential/regrouping
split (section 1.14).

**The overflow band, per member, corrected.** The band (the region between the largest representable
value and half a quantum past it, where round-first and classify-first disagree) is empty for
same-format addition (compiled), inhabited for multiplication on roughly half of pairs (compiled),
inhabited for division only once operand and result precisions decouple, empty at same precision
(compiled, file 43; the original blanket claim, "inhabited for division", was written in dyadic
coordinates and never re-derived after D69), and inhabited for mixed-format addition except when one
operand's quantum divides the other, a structural degeneracy nobody had named before file 44 compiled
it (36 inhabited, 4 empty, out of 40 swept triples; the closed-form status of the dividing-quantum
condition is a proof owed, not yet built). The fourth member of the original sentence, "every float
operation," has no derivation anywhere in the review: it entered at file 28 on the strength of its two
siblings, both of which needed correction once checked. It is struck from the sentence entirely, per
file 45's finding and the persona checkpoint's direction (`48b`, "the float member ... is struck rather
than carried: a claim with no derivation anywhere in forty-four files was never a claim"). The open item
that replaces it, the `Specials`-carrying model-float check, is section 3's largest single item
(the float model).

### 1.6 Membership and the number-system layer

D38 and D39 are op's calls (the `arvo-num-systems` crate ships; membership through algebraic structure).
D39 is held, not overturned. Membership licenses only the exact, widening operation family;
quantised in-numeral operations get their laws from the algebra ladder, never from membership.
`ExactWindow<Op, Rhs>` is the concrete content membership licenses, gated on `Specials = None`.

File 39's candidate reading for D39's honest content (membership is *inhabits*, not *equals*; the
derived fact is the finest inhabited system, unique because the tower is a chain) is offered explicitly
as a candidate, not a resolution, and still awaits the second independent read the review's own
two-expert discipline requires before anything builds on it (section 3).

### 1.7 The algebra: what a law is, and the finest view it holds at

A law is a claim that the terms of one grouping class stand in a relation, under a stated view, over the
value set of a numeral, quantified over the class, keyed on every parameter its proof used. It is a
`const fn` whose parameters are its key and whose return type is `Never` or the finest view under which
it holds, derived by blanket construction (D51), safe when derived and `unsafe impl` when asserted
(D16). The finest-view mechanism (op, `39b`) replaces the three-relation fork entirely: a term's meaning
is a grade (a free commutative monoid over refusal causes and quantisation events) and a value, a view
is a monoid homomorphism out of the grade, and two terms are equal under a view when the view sends
their grades to the same thing and their values agree wherever present. The set of views under which a
law holds is downward closed and closed under join, so every law has a unique finest view, and the named
relations (weak, Kleene, graded equation) are three points of a nine-point lattice that is not a chain.

Law equality is the canonical quotient: two results are law-equal when canonicalisation sends their data
to the same datum. The key is the operation (whose marker carries `IS_EXACT`), the operand numerals and,
for a widening operation, the result numeral, the `Quantisation` resolutions and, where a quantiser sits
between the exact operation and the result, its `Direction`; for a fold, the accumulator numeral and the
arity. `Growth` is not in the key (section 1.10). `IS_EXACT` alone does not trivialise an operation's
grade monoid; `IS_EXACT` and `Total<Op>` together do (exactness kills quantiser-generated events and
causes, totality kills causes with no quantiser origin). File 43's `div_floor`/`rem` are the design's
first genuinely exact-and-partial operations, which moves this correction from prospective to
load-bearing.

Direction enters a law's key exactly when the exact result can leave the operand lattice. File 44
rebuilt the closure predicate's own probe fresh against the current pin and confirmed it was written
over a general rational bias from the start, before the design's shipped `Bias` type agreed; it survived
the coordinate change by luck of the right kind, not by re-derivation, and its own English ("the bias
too, an integer," `33:270`) describes a value-level condition on a specific numeral's bias, not a
type-level constraint that `Bias` must be integer-typed. The next reader should read it that way.

A regrouping publishes, in its own result grade, exactly the grade generator classes its law fails to
preserve; tolerance is a transfer, never a waiver. This mechanism now ships as a type projection rather
than a caller-declared const (section 1.14); the transfer rule's wording is unchanged and enforced more
strongly by the projected form, since a caller can no longer overstate by typo.

One sentence the design still owes and does not state: the evaluation strategy of a refusing operand's
sibling. Resolved as a design direction at the persona checkpoint; section 1.14 states the sentence in
full.

### 1.8 The fold: two conditions, two relations

Interior safety (the n-1 factor: no quantiser fires in the interior, grouping-invariant) and total
safety (the n factor: the accumulator is invisible in the delivered function, `fold = quantize ∘
exact_sum`) are two distinct conditions serving two distinct promises, not two spellings of one
condition. For a multiply-accumulate, N is replaced by the product numeral `mulnum(N1, N2)`; the biased
product's accumulator is the zero-bias numeral with adjustment `gcd(A1A2, A1B2, A2B1, B1B2)`, now built
at the type level (file 42's three-rational gcd generalises the machinery it needs, and file 43's
division remainder numeral lands on the identical gcd quantum independently, a triple corroboration). At
interior safety, all three grade components (value, definedness, quantisation-event multiset) agree at
once, because exactly one quantisation fires, at the root, on a grouping-independent argument.

### 1.9 The multiplicative half

`mul_full` is a family of maps `N1 x N2 -> mulnum(N1, N2)`. The biased product numeral's closure formula,
`bias = B1*B2`, `adjustment = gcd(A1*A2, A1*B2, A2*B1)`, is now built in full at the type level: the bias
half by file 41 (`BiasMulGeneric`/`BiasProduct`, unified over all nine sign combinations by file 42) and
the adjustment half by file 42 (a three-way gcd, reading "gcd of three rationals" as a generator of the
additive subgroup the three products jointly generate, via `Lcm(A, B) = A * (B / gcd(A, B))`, no new
arithmetic primitive). Distributes is not an atom on a chain, it is Monotone, checked both ways; for a
total operation over a totally ordered value set this is full-strength monotonicity, for a partial
operation it is weaker and additionally depends on which of IEEE's two lattice-operation families is
meant. No preset the design ships is a dioid over `(max, +)`; the rung is derived, reporting a correct
"no" with the failing axiom named (D47: the ladder goes as deep as the theory does).

### 1.10 Widening and Growth: two axes removed

`Widening`'s three old instances decompose into three pre-existing mechanisms (which primitive is
named, what numeral type its return type is, that numeral's own `StoredWidth`/`Layout`). `Growth` leaves
the key because the operation's own name already determines quantiser presence, bound as `Op::IS_EXACT`.
Whether `Growth` also leaves `Policy` entirely, not merely the key, is the review's last open
ratification tick (tick 3): argued from three independent standards' own vocabulary (all three place
growth on the operation's signature, none on a unary type property) and still no compiled check. The
persona checkpoint names this the next work item after the float model (section 3, item 3 of the
direction for the next four): "compile it or kill it, no more corroboration."

### 1.11 The value-unique encoding: ratified, sealed, priced

**Ratified in full at `44b`.** Everything the ninth checkpoint's hold on this mechanism was waiting on
is done. The identity encoding is `Nat ::= Z | Pz<P: Pos>`, `Pos ::= H | O<P: Pos> | I<P: Pos>`, and
`Adjustment`/`Bias` as signed, gcd-normalised rationals over `Pos` pairs (`Adjustment` unsigned magnitude
plus reduction; `Bias ::= BZero | BPos<N, D> | BNeg<N, D>`, sign carried on the constructor, deliberately
not composed through the abstract `Adjustment` trait). Uniqueness holds by induction on the value, with
no normalisation operator anywhere: the induction's hypothesis is that the declared constructors are the
only impls, which is what the seal (section 1.12) supplies.

**MATLAB's own witness, the axis's reason for existing, is representable and every unreduced pair is
refused.** Slope 1, bias 1/2 (`BPos<H, O<H>>`) and biases 1/2, 5/2 both represent directly; an unreduced
pair (`BPos<P6, P12>`, six-twelfths unspelled) cannot reach a `Bias`-bounded position, refused with
`E0271` carrying the actual gcd in the error text.

**The composition wall, corrected.** File 41 found that a fully generic `BiasMul<N1, D1, N2, D2>` trait,
mirroring `Adjustment`'s own composition shape, does not compile: naming the `Reduce` trait itself as a
bound forces the solver to eagerly confirm its one blanket impl, which recurses into `Pz<P>`'s wrapped
pattern with no base case for an abstract input, `E0275` diverging, corroborated by an independent SIGBUS
inside `rustc_trait_selection` on a naive `#![recursion_limit]` raise. File 41 concluded the design
"cannot have a generic `BiasMul` trait" and shipped bare type aliases instead. File 42 isolated the
actual boundary more narrowly: the divergence is specifically about naming `Reduce` (or any trait with
exactly one matching blanket impl) as a bound; the identical constituent facts, spelled directly as an
ordinary generic impl's own where-clauses rather than through `Reduce`, compose cleanly, and file 42
built the trait file 41 said could not exist (`BiasMulGeneric`, extended to all nine sign combinations
and unified with `BiasProduct`), measured at the same compile cost as the bare-alias form and marginally
smaller crate metadata. The design rule this leaves, stated for the spec: **every trait in a chain that
reaches a consumer-facing signature either pattern-matches on constructor heads or has finite,
non-recursive obligations; `Reduce`, and anything routed through it, never appears in such a chain, only
at concrete numerals.** File 48 confirmed the constraint is load-bearing rather than academic by
compiling the collision directly, in the grade-projection combinator (section 1.14): a plausible
refactor computing a headroom ratio through `Reduce` on the fold's own signature hits the identical
`E0275`. The positive control (file 47's working chain) and the negative control (file 48's collision)
ship together as a compile-fail pair the day the mechanism lands in the tree.

**Priced.** The bias magnitude costs essentially what `Reduce` alone already costs at a comparable
width (13.61 ms/composition against 12.07 for `Reduce` at 8-bit operands, `--emit=metadata`); the full
sign-plus-magnitude composition costs 19.10 ms/composition at that width and roughly an order of
magnitude more at 16-bit operands (up to 32-bit products), the harder, more realistic case. Over dyadic
magnitudes, the shape every fixed-point numeral this stack ships today actually uses, the cost drops to
roughly 1.55 ms/composition, the same qualitative cheap-common-case result file 36 found for
`Adjustment`'s own reduction. Zero symbols emitted at any size (`nm -g`, `--emit=link`); the metadata
debit is real and larger than `Adjustment`'s own reduction debit, the honest cost of reducing a wider
intermediate product. The generic trait form costs the same as the alias form to compile and is
marginally smaller in metadata; there is no compile-cost argument for keeping the alias shape.

**`Int` is dropped from the ratified table.** `Int ::= Z0 | Zpos<P> | Zneg<P>` carried the comment
"biases, corrected to a normalised rational," citing the very correction that removed its only
prospective consumer: `Bias`'s sign is carried on the `BPos`/`BNeg` constructor over `Pos` pairs, never
through `Int`. Three independent reads (file 45, then file 47 by a different route, then file 48) agree
`Int` has an empty grounding set. File 48 answered the one serious objection (a future signed exponent
might need it): a signed exponent lands on the same constructor-sign shape `Bias` already uses, sealed
and attacked twice over, so even the fork that would consume `Int` does not consume `Int`. **This is a
persona-decided call at `48b`, edited into a ratified table, flagged loudest for op's review**: section
2 states it exactly as the checkpoint did. The encoding is three lines in the audit trail (files 36, 41)
should a use ever materialise that the constructor-sign shape cannot serve.

### 1.12 The seal: carriers sealed, contracts open

**The seal was open three times, at three different layers, and is now closed with a compiled
adversary rather than an argument (the opening narrative above names the three passes; here is what
each actually left open).** File 36's own seal lived in an orphaned file nobody's construction touched.
File 41 sealed `Bias` correctly (bounded on `N: Pos + Gcd<D, Out = H>`, not on the abstract
`Adjustment`) and found `Adjustment` itself open: a genuinely separate crate implements it directly on
a fabricated, unreduced pair, `NUM = 6, DEN = 12`, compiles clean, reaches an `A: Adjustment`-bounded
position. File 42 found file 41's own recommended fix (seal `Adjustment` the way `Bias` is sealed) does
not reach the deeper hole, which is in `Pos`/`Nat` themselves: every consuming position bounds on them,
and a downstream crate can fabricate a foreign `Pos` with a `Gcd` impl claiming unconditional
coprimality with no Stein computation performed. File 42 sealed `Pos`/`Nat` and left `Adjustment`
exactly as file 41 shipped it, unsealed, in the fixed copy. File 46 found `Adjustment` still open a
third time in that copy and closed it with one private supertrait and one blanket impl, measured at
zero cost against noise.

**"Closed" is now discharged by enumeration over introduction routes, not over attacks anyone thought
of.** In this language there are exactly four routes by which a downstream crate can introduce a new
obligation for a foreign trait: direct impl on a local type (refused, `E0277`, on the private
supertrait), implementing the supertrait itself (refused, `E0603`, unnameable), re-impl on an existing
inhabitant (refused, `E0117`, orphan rule, before any seal is consulted), and a downstream blanket over a
type parameter (refused, `E0210`, uncovered parameter). Fabricated-`Pos` laundering through an upstream
blanket is refused at the root (`E0277` on `PosSealed`); malformed genuine types at bounded positions are
refused at the bound (`E0271`, carrying the actual gcd in the error). File 42's one argued-not-compiled
residual (whether a downstream can hold a lying `Gcd` fact about a genuine inhabitant and spend it) is
now compiled in both halves by file 46: coherence **admits** a downstream `impl Gcd<LocalRhs> for H`
(a genuine inhabitant, a foreign, local `Rhs`), and it cannot be spent, because the other operand
position in every consuming site is itself `Pos`-bounded and refuses first. Type erasure (`&dyn Pos`)
is refused, `E0038`, not dyn-compatible; the workspace's own no-`dyn` rule closes the route before
coherence is even asked.

**The two-obligation checklist, run once per carrier, is what makes a fourth pass unnecessary.** A
carrier (a trait whose inhabitant set a guarantee quantifies over) owes exactly two things: sealed at
the trait (a private supertrait, closing the direct-impl and named-supertrait routes; the orphan rules
close the other two for free), and carrier-bounded through the parameters (every impl of the trait,
including the seal's own blanket, sits on a closed constructor whose every type argument is re-bounded
on carrier traits, or re-establishes the defining property directly). Running the checklist over the
review's own history explains every finding above as the one line each pass missed: file 41's `Bias`
satisfied both (bounded on the condition, not on the abstract `Adjustment`, deliberately); the `Pos` it
bounded on satisfied neither in the composing copy (file 42's finding); file 42's `Pos`/`Nat` satisfied
both; `Adjustment` satisfied the second and not the first (file 46's finding).

**The guarantee's quantification, stated where the guarantee is stated.** In any crate graph containing
the sealing crate(s), compiled on the pinned toolchain with `specialization` and `TypeId` forbidden, the
inhabitants of the carrier traits are exactly the declared constructor sets, and two types inhabiting
one carrier denote the same value if and only if they are the same type. The guarantee quantifies over
inhabitant introduction only. It does not quantify over, and is not threatened by: helper-trait impls on
downstream-local types (admitted, unreachable into a bounded position); foreign-parameter impls of
unsealed traits on genuine inhabitants (admitted by coherence, unreachable, because every consuming
position re-bounds both operands on carriers); observation (any downstream may read the associated
consts and recurse structurally over the public constructors, which is how derived facts, convention
crates, and the numeral-notation alias table of section 1.16 are all legitimately built). It rests on,
and goes stale with: the seal modules staying private, every impl of a carrier trait keeping the
two-obligation discipline, the `specialization`/`TypeId` bans, and the pinned solver's coherence
behaviour, which a shipped compile-fail suite pins the day the tower lands in the tree.

**The design rule this earns**, applied one step further by file 48: **seal the carriers, open the
contracts.** `Numeral`, `Policy`, `Lowering`, the convention traits, the algebra ladder: open, a
downstream implements them. `Pos`, `Nat`, `Adjustment`, `Bias`, `Grade` (section 1.14), and any future
closed-vocabulary axis a guarantee quantifies over: sealed, a downstream instantiates them. The one real
cost is locus, not capability: a carrier's constructor set is fixed at its declaring crate, so whether
`FullRange` survives as its own named `Adjustment` constructor (section 3) is an edit to the sealing
crate whichever way it is decided, never an addition a consumer could make on their own.

**The seal is compiled and verified twice over, not once.** File 47 ran the second read the review's own
two-expert discipline requires, reproducing both of file 46's decisive probes from source before reading
file 46's own table, and agreeing with the finding and the fix. `BiasProduct<Rhs: Bias>` (one operand
position in the tower that was declared without a bound, harmless because the output is itself
carrier-bounded, but the one place the tower's reachability argument needed a special case rather than a
uniform one) is a one-token fix the persona checkpoint calls landed. The tower's by-construction claims
are, as of this document, `conditional on seal-owed` in the review's own artifact no longer: the sealed,
attacked copy is what every probe since file 46 composes with. They remain conditional in the shipped
tree, where nothing exists yet, until the tower plus its adversary land as compile-fail tests (section
3), the same discipline that already lives at `mock/crates/arvo/tests/ui/`.

### 1.13 Division: held, with its shape recorded

**Not adopted this stretch.** Op held it explicitly at `44b`: the finding stands, the operation surface
waits until the rest of the algebra settles, and nothing about it expires in the meantime.

**The no-finite-accumulator prediction is false in the ratified coordinates, true in the coordinates it
was originally written in, and sharper than either reading alone.** In dyadic coordinates (radix-power
quanta only), the prediction is true: no dyadic grid at any width contains 1/3. Under the ratified
rational adjustment, the finite accumulator exists: for operand numerals with divisor index bound K, the
numeral with adjustment `(A1/A2) / lcm(1..K)` contains every quotient exactly, checked exhaustively at
two model widths, with the lcm proven the least possible denominator. What was actually being reached
for is real: the accumulator's width is Theta(2^p) bits, exactly 5, 12, 23, 51, 95, 190, 370 bits for
p = 2..8, against 2p for multiplication's exact product. Division is a third growth class, exponential
rather than the linear (fold) or quadratic-in-precision (multiplication) growth the design's other two
cases carry.

**The exact subfamily generalises from radix powers to any representable constant, at zero new
mechanism.** `div_exact` is a family of maps `N -> divnum(N, C)` for a type-level nonzero rational
constant `C = cn/cd`: adjustment `A * (cd/cn)` reduced, bias `B * (cd/cn)` reduced, both through the
already-built rational multiplication with the constant's components swapped. The operation is total and
exact by construction (the constant's numerator is `Pos`-bounded, so a zero divisor has no spelling),
and by the `IS_EXACT`-with-`Total` correction (section 1.7) every law holds at every view. Division by a
radix power is the special case that stays in the operand's dyadic family; the general case (dividing by
a sample rate, a window length, a fixed gain) is exact under the ratified coordinates in a way the
design had been naming only the special case for.

**General division needs no new quantiser.** `quantize(exact quotient)`, one quantisation, the
round-first pipeline unchanged. The finite exact carrier is not a numeral at any practical width; it is
the Euclidean pair `(q, r)`, `a = q*b + r`, `0 <= r < b`, whose remainder lands on the same gcd quantum
the MAC accumulator formula already computes. The proposal this earns: two single-valued operations,
`div_floor` and `rem`, each exact, each partial on the divisor's nonzero-ness, bound by a compiled
Euclidean law, with general `div` implemented from the pair rather than from a wide quotient. These are
the design's first genuinely exact-and-partial operations, the case section 1.7's `IS_EXACT`-with-
`Total` correction anticipated before any operation inhabited it. IEEE's own cause split (x/0 nonzero
finite is `divideByZero`, infinity result; 0/0 is invalid, NaN) is a correction the enumeration owes,
reasoned from the standard and awaiting the float model to compile.

**No division fold is owed.** A sequential fold by divisors is division by the running product; its
lawful shape is the design's existing fold machinery (fold the divisors through `mul_full` under MAC
interior safety, divide once at the root), not a new combinator.

### 1.14 The grade is a type: projection, join, evaluation, and the fold surface

**Adopted as spec shape at the persona checkpoint, built by file 47, attacked and hardened by file 48.**
The published grade moves from a caller-declared, positional const bitmask (`regroup_fold::<0, 0, 1, 4,
0, 1>`, six unnamed integers a reviewer cannot read and a caller has to compute by hand) to an ordinary
associated-type projection: `Folded<<(<Hd as InteriorSafety<Am1>>::Out, Top, Bot, Dom) as
FoldGrade>::Out>`, compiled clean with no unstable feature at all. File 37's own reason for the
caller-declared shape ("computing the grade in return position hits the `generic_const_exprs` wall") was
a limit of putting a computed quantity in const position; the spine rule says the quantity should have
been a type from the start, and once it is, the wall is not near the mechanism.

**A second wall is near it, and the projection avoids it only by an unstated structural property that
now has a name.** Files 41 and 42 established that naming a trait with exactly one unconditional
blanket impl as a bound (`Reduce` above all) forces eager confirmation and diverges (`E0275`). File 47's
projection chain avoids this by accident: `Cmp`'s impls pattern-match on constructor heads, so the
solver has no unconditional candidate for an abstract operand and defers, and `InteriorSafety`'s single
blanket impl has finite, non-recursive obligations. File 48 compiled the collision a plausible refactor
would hit (computing the safety margin as a reduced headroom ratio through `Reduce` on the fold's own
signature: `E0275`, the exact composition-wall signature, now in a consumer-facing combinator). The
constraint stated in section 1.11 is what makes the projection safe, and it ships as a compile-fail pair
(file 47's working chain as the positive control, file 48's collision as the negative) the day the
mechanism lands.

**The join half of the grade algebra was missing, and file 47's own recommendation required it.** "An
operation's grade is the join of its operands' grades" (file 47) plus "the grade is a type" (file 47,
section 3.2) together make the join a type-level operation. File 47 compiled only the lattice order
(`WeakerThan`, nine impls); file 48 built the join itself (sixteen constructor-headed impls, no blanket,
sitting on the safe side of the composition wall by construction) and checked the whole matrix, not a
sample: join-bitmask agreement, commutativity and associativity as type equalities, identity and
absorption, and order-join compatibility, with the seven negative order pairs named for the compile-fail
suite rather than sampled away. `combine<G1, G2>(..) -> Graded<<G1 as Join<G2>>::Out>` is one signature
carrying both file 47's semantics and file 48's mechanism.

**`Grade` is sealed, applying the carrier-at-birth rule to the newest carrier the review has minted.**
File 47 shipped the perimeter argument distributed (a private field on `Folded` alone); file 48 sealed
`Grade` itself at two lines, attacked on both introduction routes, refused with `E0277` and `E0603`.
`Definite` (file 47's diagnostic bound) inherits the closure for free.

**The fused spec statement**, assembling the evaluation-strategy sentence and the `Precise` combinator
surface into one item, since after file 48 they were never two:

> Every operand of an operation is evaluated: an operation's grade is the join of its operands' grades
> with the operation's own contribution, whether or not any operand refused, so a term's report is a
> function of the term, invariant under the regrouping the transfer rule licenses, the reordering
> commutativity licenses, and the schedule the executor picks. (An implementation may skip work this
> cannot observe.) The fold surface is two named combinators: `fold`, which regroups and publishes, by
> projection, exactly the grade classes its law fails to preserve; and `fold_sequential`, which regroups
> nothing, publishes nothing, is faithful by construction, and is named for what it costs. Both are
> strict; a short circuit is not implementable on `fold` under a pluggable executor without a
> cancellation protocol that belongs to the scheduler, and offering it on `fold_sequential` alone would
> make the two combinators' reports disagree on identical data. The caller's type picks the door;
> `Definite`-style bounds carry the remedies; overstatement is `.weaken`, explicit and bounded on the
> sealed grade lattice, whose join and order are checked over the whole matrix.

The evidence behind it: file 47 first argued the short circuit would break grouping-invariance and
refuted its own hypothesis by compiling it (an exhaustive check over all 81 four-leaf terms under three
groupings shows the short circuit is grouping-invariant after all, because a short circuit visits
leaves left to right and regrouping never moves leaves). What survives is worse and is what the
recommendation actually rests on: the short-circuit report is not invariant under reordering (three
orders of the identical channel multiset produce reported event counts of 0, 1, and 2, all with the
identical delivered outcome), which contradicts the design's own droplist entry against a documented
traversal order substituting for a law, and degrades exactly on the worst data (a sample that refuses
early in the traversal order reports nothing to rescale). File 48 killed the one wrong supporting witness
(file 47's claim that hilavitkutin's RCM renumbering permutes a fold's traversal order) by reading the
consumer's own canonical design at the cited lines: RCM produces a row reordering (WU execution order)
and a column reordering (arena memory layout), and neither touches the per-record index a fold walks.
What is actually true downstream is stronger than what file 47 claimed: under hilavitkutin's default
executor, execution is deterministic and record-ordered, so the short circuit would at least be
deterministic there, but work-stealing is explicitly not the default and is consumer-pluggable, and
under a stealing executor a short-circuited regrouping fold's report becomes run-to-run nondeterministic
on unchanged data, or requires a cross-morsel cancellation protocol the design has already ruled belongs
to the scheduler, not to arvo. The one cost file 47 conceded to strict evaluation (an operand whose
sibling already refused is still computed) is not a semantic cost once the as-if rule is applied: where
the report is unread the work is unobservable and skippable, where it is read the work is the product,
so the scalar-path objection file 47 left open has no case left to argue.

The `Precise` surface question resolves the same way: one combinator with a defaulted grouping type
parameter (the shape that would keep both doors ceremony-free) is refused by the language itself
(defaults for generic parameters are not allowed in that position, a known future-incompatible issue).
One combinator only leaves a consumer whose accumulator cannot be widened enough with an unwritable
program, which is the policing posture the design's own toolbox rule forbids. Two named combinators,
`fold` and `fold_sequential`, is the design's own `fold_compensated`-beside-`fold` idiom applied a second
time, and it is what the fused block above states.

The one sub-item genuinely left open inside this block is per-application against per-value-moved event
counting (probe 5's own assumption, section 3), which the persona checkpoint explicitly declined to
call.

### 1.15 The spine rule forces the exponent fork open, and settles what `Int`'s drop would have owed

**Reasoned, a first read, not compiled.** The exact-widening family's numeral-level maps compute result
numerals from operand numerals. For `Implicit` numerals every computed member is already a type
(adjustment through `Reduce`, bias through `BiasProduct`). For `Ranged` numerals the exact product's
exponent bounds are `EMIN1 + EMIN2` and `EMAX1 + EMAX2`, arithmetic over const parameters whose result
must appear in the result numeral's type: a const computed in type position, the identical wall that
already pushed width arithmetic out of const generics. So the moment the exact family reaches `Ranged`
numerals (which `ExactWindow`'s own `Specials = None` gate already contemplates), the exponent must
become a type or the family is unwritable there. The fork the third consolidation carried as "a real
fork nobody has opened" is opened by the spine rule and answered yes; building `Ranged` numerals and
compiling `mulnum` over two model instances is the second read this needs before it hardens (section 3).

This also supplies the argument file 45's `Int`-drop lean was missing: the strongest objection to
dropping `Int` is that a future signed exponent might need a signed encoding, and the spine rule's
derivation shows that consumer, once it exists, lands on the constructor-sign shape `Bias` already uses
(`EZero | EPos<P> | ENeg<P>` over the sealed `Pos`), not on `Int`. Section 1.11 states the drop; this is
why it survives the future case as well as the past one.

### 1.16 The numeral notation

**Persona-decided at `48b`: a consumer writes any number as a literal, unbounded range, emitted
constructors, zero table. The vehicle inside that intent is not itself decided.** File 47 found that
declaring a numeral by hand is unusable at the widths the design actually needs (IEEE binary64's
precision is a six-constructor-deep nest; a Q0.15 quantum is fifteen deep), that a mistyped numeral is a
silently well-formed different numeral (one dropped or changed constructor, still legal, still sealed,
still unique, and still wrong), and that the resulting type mismatch is unreadable (`E0308` differing by
one character forty columns in, with nothing in the message naming the intended decimal value). File 47
proposed a bounded alias table (0 through 1024, generated, one const-asserted alias per row, resolving
by name rather than by type-level arithmetic so it costs the trait solver nothing) plus a `pow2!`
sibling. File 48 found this defect is not hypothetical: file 43's exact-division-by-constant subfamily
divides by sample rates and window lengths (44100, 48000, 4096), far past any table a metadata budget
tolerates, and a bounded table fails on exactly the constants the design's own operation surface exists
to divide by, loudly (`E0425`, naming the missing row) but not usefully. The persona checkpoint rejected
the bounded form on principle rather than only on the exact-division finding: a fixed-size table is a
hardcoded threshold of the kind the workspace's own toolbox rule forbids arvo from shipping anywhere.

The alternative is a digit-emitting macro (unbounded range, zero table, zero trait-solver cost, computing
the encoding at macro-expansion time and emitting the literal constructors) against either a genuinely
intricate declarative `macro_rules` decimal-to-binary muncher or a proc-macro crate (compile-time-only
std dependency, the notko `#[profile]` precedent; arvo currently ships only declarative macros, so this
is a real dependency-surface decision). The persona checkpoint calls this vehicle question
sketch-decidable rather than checkpoint-shaped: sketch the declarative form first, escalate to the
proc-macro crate only if it proves genuinely hairy or slow, and either way the macro owes a whole-matrix
const-assert test over its emitted encodings and every negative case forced through a signature (the
seal's own lesson: a bare type alias defers its bound checks and a suite built from aliases is green
while asserting nothing).

**What is not fixed by any of this.** rustc expands type aliases in diagnostics, so the intended decimal
value never surfaces in an `E0308` regardless of the notation layer; the only partial mitigation found
is an `on_unimplemented` note where a mismatch is expressible as a bound rather than a type equality
(section 1.14's `Definite` diagnostic is the working instance of this pattern), and nobody has built the
numeral-specific version. The seal seam this notation layer touches is benign, checked against the
seal's own quantification rather than assumed: an alias is observation (any downstream may read the
public constructors and recurse structurally), nothing new inhabits `Nat`, and the macro's expansion
resolves in the crate that declares it, so a consumer's own crate gains no declaration at all.

### 1.17 Claim provenance: the grounding registry

**Adopted at `44b` (the field itself, decision-shaped) and widened by file 45, operated successfully by
file 48 on the stretch's own first real mechanism change.** File 44 diagnosed why a claim can survive
its own foundation moving: a consolidation compresses a conjunctive claim's conclusion and discards
which of its members were actually checked (the overflow-band sentence carried two uncompiled members on
the strength of two compiled siblings for sixteen files), and a ratified coordinate change has no
automatic query for what depends on it (D69's own consequence was found stale twice, by luck, by
members who happened to be standing next to the specific sentence for an unrelated reason). The fix op
adopted: every load-bearing claim carries `grounded on: <slug>`, naming the decision its derivation
actually leans on, so the next overturn's blast radius is a grep rather than a hand sweep.

File 45 found the field necessary and not sufficient: a `grounded on:` vocabulary of decisions alone
cannot record what every compiled result, every measured figure, and every bounded-exhaustion result
also rests on, namely the toolchain pin, the host target, the build-flag discipline, and the
forbidden-feature bans that let a model-width check transfer to real widths. The vocabulary widens from
decisions to four kinds of ground:

| kind | rung | examples |
|---|---|---|
| ratified decisions | op-ratified, governing | `d69`, `vu`, `enc`, `seal-owed`, `div-held`, `grounding` |
| settled shapes | panel-settled, presumed correct, overturnable with evidence | `round-first`, `crossing`, `bias-rational` |
| physical grounds | facts about the environment, change by act not argument | `pin`, `host`, `flags`, `model`, `ffl` |
| tree grounds | facts about the shipped source at a commit | `tree` |

The attachment rules keep the convention cheap: every bounded-exhaustion claim is grounded on `model +
ffl + pin` by definition of its evidence bin; every measured claim is grounded on `pin + host + flags`;
a compiler refusal is additionally grounded on `pin` until a compile-fail test pins it; a conjunctive
claim grounds per member, with `unknown` written where no derivation exists rather than inherited from a
sibling. This division of labour is what makes the field's hand-written residue small enough to survive:
only the decision-shaped and settled-shape grounds are hand-written per claim, and the physical floor
attaches by evidence bin.

File 45 backfilled every load-bearing claim in the third consolidation against this registry (its own
section 3, not reproduced here in full; the next consolidation's own claims, section 1 above, carry
their grounds inline where load-bearing). The backfill caught, without looking for defects specifically:
`Int` was a ratified tier whose only consumer had already been removed (section 1.11); every
by-construction claim about the numeral tower was conditional on the seal, which the same checkpoint
had deferred as a follow-up task, a condition the grounding field is what makes visible rather than
assumed (section 1.12 states it discharged); the "every float operation" overflow-band member had no
grounding anywhere and was struck rather than carried unverified; and a verification command (the
`grep -rln "Adjustment\|Bias\|Numeral"` check) had been copied through four files with a path that does
not reproduce as written, corrected in this document's own verification paragraph.

File 48 operated the widened field on its first real mechanism change (the grade's move from const to
projected type) and it worked: one ground (`ffl`) retires from the transfer-rule row, because the
declared-not-computed shape was grounded on the `generic_const_exprs` ban and the projected shape needs
no feature; one new ground is added for the projection-chain structural constraint; and the
`const-fn-key` ground gains a one-sentence note that the same unnameability is enforced by the crate DAG
for trait-impl carriers, not only for const-fn laws. The convention's own perimeter is stated honestly,
not assumed away: no tier detects an unwritten grounding, and the residual is caught only by the
consolidation author re-deriving per-member status at the point a claim is rewritten, which is the same
act that catches a stale claim in the first place.

Two tiers beyond the current one are named, not built: a mockspace registry namespace
(`{{ ground::d69 }}`-shaped references, resolved and reported dangling at render time) for when these
claims graduate into `*.md.tmpl` design documents, and a probe-header line (`//! grounded on: <slugs>`)
for the artifact itself, cheap and already informally present in files 41 through 48's own probe
headers.

### 1.18 The assembled trait table, and what it costs to build against the tree

```rust
// Every member that denotes a number is drawn from one value-unique, sealed,
// type-level encoding, sealed and attacked on every introduction route (1.11, 1.12):
//   Nat ::= Z | Pz<P>            P: Pos       precision, widths, exponent bounds
//   Pos ::= H | O<P> | I<P>      P: Pos       magnitudes
//   Bias ::= BZero | BPos<N, D> | BNeg<N, D>  N, D: Pos, N: Gcd<D, Out = H>   signed rational

pub const trait Numeral {                 // ratified: identity contract, 30/31
    type Radix:     Radix;
    type Precision: Precision;            // a Nat; primitive (D69)
    type Exponent:  ExponentForm;         // Implicit<E, A: Adjustment, B: Bias> |
                                          //   Ranged<EMIN, EMAX, U: Underflow, S: Specials>
    type Domain:    SignDomain;           // a value fact
}

pub const trait Policy {
    type Quantisation: Quantisation;      // unchanged
    // Growth removed from the key: RATIFIED. Removed from Policy entirely: OPEN (tick 3).
}

pub const trait Lowering {
    type Encoding:    Encoding;           // SignIndexing, Fields, Canonical
    type StoredWidth: StoredWidth;        // a Nat, same encoding
    type Layout:      StorageLayout;
    // Widening removed: RATIFIED.
}
```

`Int` is dropped, per section 1.11; nothing in the table above names it. Rewrite cost against the
shipped tree remains near zero: no shipped source names `Adjustment`, `Numeral`, `Bias`, `FullRange`,
`UTerm` or `AddWidth` (this document's own verification paragraph reruns the grep with the corrected
path and confirms it empty), and the shipped `IFixed` already computes width from precision-shaped
parameters at its declaration site.

### 1.19 The downstream contract, and the crate table

Unchanged from file 26 through file 40, untouched by every deliverable in this stretch. arvo grows no
build harness of its own; a build layer reads every axis, acts freely on `Lowering`, acts on `Policy`
only by a transformation staying inside the axis's own declared envelope, and never acts on `Numeral`.
The post-monomorphisation verifier, the semantics-free per-axis liveness check, the fold-detection
assertion, the layout-assertion precedent, the build-layer receipt requirement, and the three ways to
cross the Stage G boundary all stand exactly as file 26 recorded them. The six-crate split stands as
packaging, unchanged.

## 2. The lead designer's calls

Op made one set of ratifications at the tenth checkpoint, then, asleep, delegated the remaining
checkpoints of the night to his own persona by explicit instruction: "keep this going this same way, but
in place of my checkpoints, have my persona answer your questions much the same way, but instead of ask
user tool, you'll use sub agent fable level ... this instruction and override on its wording will trump
it for tonight, up until I come back tomorrow morning." Everything the persona decided is
**persona-decided, not op-decided**, stated in those words at `48b`, and every line of it dies the
moment op reads it and says otherwise. The distinction is preserved throughout this section rather than
folded into one undifferentiated list of "ratified" calls.

**Op's tenth checkpoint (`44b`).** Four calls. The value-unique encoding is ratified in full: the
rational bias is built, the MATLAB witness representable, unreduced pairs refused, priced at 1.55
ms/composition over the dyadic case every shipped numeral uses, and the API shape (generic trait against
bare alias) settled by a compiled form costing the same as the alias. Sealing `Pos`/`Nat` at the layer
where they actually compose is named as an implementation task, owed, not gating the shape. Division is
held for a later stretch: file 43's finding stands and is recorded, the operation surface waits until
the rest of the algebra settles, and nothing about the finding expires in the meantime. Every claim
records what it is grounded on, and the existing consolidation gets backfilled, because a consolidation
compresses a conjunctive claim's members and a ratified coordinate change has no query for what depends
on it; op's own words on why: "the next overturn's blast radius is a grep rather than a hand sweep
across forty-odd files." Standing: the convergence directive from `30b` and the novelty posture from
`34b` both hold unchanged, the intent outranks every instruction and is inferred rather than read
literally, and only op's calls are final, "which is the entire reason the grounding field now exists."

**The persona checkpoint (`48b`), five calls, four landing in this document.**

The grade projection is adopted as the spec's shape, not as a candidate, with three attachments named as
part of the adopted shape rather than as follow-ups: the projection-chain structural constraint ships as
spec text with its compile-fail pair, `Grade` is sealed at birth per file 46's carrier principle, and the
join algebra is part of the mechanism, whole-matrix. In the persona's own words: "the two-expert
threshold was met the right way: 47 built it, 48 attacked it, rebuilt every probe fresh, and the repairs
made it stronger."

The numeral spelling is the digit-emitting macro, and the bounded table is rejected on principle: "a
1024-row table is a stored copy of a computable function, and its bound is a hardcoded threshold of
exactly the kind arvo refuses to ship anywhere else." The vehicle inside that intent (declarative macro
against proc-macro crate) is explicitly not itself a checkpoint item: "sketch-decidable."

The evaluation sentence is adopted as the fused block of section 1.14, whole, because after file 48 the
evaluation sentence and the `Precise` surface "were never two items." The persona's own reading of the
evidence: "the evidence is one-sided in a way this review rarely gets," citing file 47's self-refutation
of its own grouping-invariance hypothesis, file 48's replacement of the wrong RCM witness with a
stronger, canon-sourced one, and the as-if rule dissolving the one conceded cost. The per-application
against per-value-moved event-counting sub-item is explicitly declined: "that one is genuinely op's and
the persona declined to take it."

`Int` is dropped from the ratified table. "This edits a ratified table, so it is flagged loudest for
op's morning read. Restoring it is one line if he disagrees." Two adjacent items close alongside it: the
seal question is closed (the quantification block of section 1.12 is taken verbatim), and
`BiasProduct<Rhs: Bias>` lands as a one-token fix.

**Direction for the next four, in the order the persona chose "by what unblocks the most."** First, the
float model: four independent demands (the struck overflow-band member, the IEEE cause split, division's
float path, the `Ranged` exponent compile) have arrived at it, making it "the keystone pick rather than
the deferrable one." Second, the exponent-as-type fork, second read plus compile, riding along with the
float model's own `Ranged`-numeral machinery. Third, tick 3 (`Growth` leaving `Policy`), closed by the
compiled check file 35 itself proposed: "compile it or kill it, no more corroboration." Fourth, the owed
test debt landed as committed artifacts in one dispatch (the five codegen regression tests, the seal
adversary in its shipping compile-fail shape, the projection-chain pair): "a recommendation this review
has repeated four times is not a recommendation any more, it is a defect in the review." The membership
second read (D39's finest-inhabited-system reading, owed since file 39) queues behind these as a read
rather than a build. Division stays held exactly as `44b` left it.

**One item flagged out-of-band, for op and the workspace rather than for this review**: the pin-hash
discrepancy file 45 found (`workspace.md` records `cced03bfd` for the pinned channel date; every
measured record in this review, independently confirmed in this document's own verification paragraph,
records `57d06900f` for the same channel date) needs a one-line reconciliation in `workspace.md`,
outside this panel, with the measured record winning per the workspace's own provenance discipline.

**Standing, unchanged and restated by both checkpoints in the same words.** The convergence directive
from `30b` and the novelty posture from `34b` both hold. The intent outranks every instruction, is vague
on purpose, and is inferred rather than read literally, so no member resolves to a single angle on
anything substantive. Where the current shape can be kept it should be, and rewrite cost is the
tiebreaker between designs otherwise equal against the intent. Only op's calls are final, and even those
go stale the moment something better surfaces, which is what happened to D69 and to the tenth
checkpoint's own bias correction one section over, and is exactly why the grounding field exists.

## 3. What is open

**The float model.** The largest unbuilt object in the review, named the keystone by the persona
checkpoint because four independent threads now arrive at it: the struck "every float operation"
overflow-band member (section 1.5), the IEEE divide-by-zero-versus-invalid cause split (section 1.13),
division's float path generally, and the `Ranged`-numeral compile the exponent fork needs (section
1.15). Building a `Specials`-carrying model numeral and running the files-30/31 exhaustive-check shape
against it is what the next member should treat as the single most valuable build available.

**The exponent-as-type fork, second read plus compile.** File 48's spine-rule derivation (section 1.15)
is a first read, reasoned only; a second member should form an independent reading from `40:690-691`
(the fork's original statement), the `Ranged` declaration, and the exact-widening family's own
`Specials = None` gate, before the fork's answer hardens. The float model supplies the `mulnum` over two
model `Ranged` numerals nearly for free, so the same member or the next can take both in one pass.

**Tick 3, `Growth` leaving `Policy` entirely.** The last open ratification tick. Argued three separate
times (file 35's own case, file 39's standards corroboration, this document's restatement) with no
compile. The persona checkpoint's own direction: compile the exhaustive search of arvo's operation
surface file 35 itself proposed, or drop the argument; no fourth round of corroboration without an
artifact.

**The owed test debt, five items, none built.** The multi-limb carry chain, the fold-versus-direct-
multiply fold at native and multi-limb width, the saturating-reduction non-vectorisation, the
vectorisable-loop-idiom sensitivity, and (new this stretch) the `div_floor`/`rem` fusion into one
hardware divide. Alongside them, two compile-fail pairs the review has now specified in full but not
shipped: the seal's adversary (section 1.12, positive and negative already committed as probes) and the
projection-chain constraint (section 1.11/1.14, likewise already compiled both ways). The persona
checkpoint's own framing: a recommendation repeated four times is a defect in the review, not a
recommendation.

**The membership second read.** File 39's finest-inhabited-system candidate for D39's honest content is
explicitly offered as a candidate, not a resolution, and the review's own two-expert discipline forbids
building on it until a second independent member forms their own reading of the topic file.

**The reduction firing site and whether `FullRange` survives as its own named constructor.** Both named
and unbuilt since file 26. File 44 connected the survives-as-its-own-constructor branch to a proof
obligation nobody had stated: value-uniqueness would require proving `FullRange`'s own reduction agrees
with `Adjustment`'s `Reduce` on every value where both are defined, which is file 01's twenty-nine-file-
old finding restated as exactly what that branch owes. The reduces-to-a-bare-ratio branch dissolves the
finding by construction, since there is nothing left for it to be about.

**The numeral notation's vehicle.** Persona-called sketch-decidable, not itself a checkpoint item
(section 1.16): declarative `macro_rules` muncher first, proc-macro crate only if that proves genuinely
hairy, either way owing a whole-matrix const-assert test with every negative case forced through a
signature.

**The decoder-ring diagnostic.** File 47's stated non-fix: no route found to put a decimal value into an
`E0308` for a mismatched numeral. The `on_unimplemented` route works only where the mismatch is
expressible as a bound; whether a member with deeper knowledge of rustc's diagnostic surface can do
better in twenty minutes is named and left untried.

**The `TotalOrd` level annotation.** Datum-level (5.10-shaped, forbidden to laws) or value-level
(specified, one NaN class placed consistently): a one-sentence fork nobody has picked.

**The dither-versus-`Refuse` interaction.** Confine the perturbed value to the numeral's range before
quantising (costing uniformity near the ends) or gate the dithered entry point on totality: settled as a
mechanism, open as a design choice about `Precise`'s consumer contract.

**The per-application against per-value-moved event-counting reading**, for the fold's grade. Left
exactly where file 43 stated it and where the persona checkpoint explicitly declined to make the call:
genuinely op's.

**The real-consumer compile-cost bench.** Unpriced since file 26. Every mechanism-shape sweep this
stretch ran (the bias magnitude, the seal, the grade projection) prices a different, narrower thing and
is a neighbour to this question, never an answer for it, as every file that ran one says explicitly.

**`SC_WRAP<n>` and `SC_WRAP_SM<n>` with `n_bits > 0`.** The one cell of the ratified no-gaps claim with
no construction anywhere in the review. Flagged, not built, unchanged since file 26.

**Richer canonicalisation's branchlessness, and cross-word bitpacked field extraction.** Both named
limits on file 32's own measurements, unmeasured beyond what file 32 checked.

**`DatumDeterministic`.** File 31's correction to D70, reasoned and named, not built as a real `const fn`
against arvo's trait shapes.

**The `E0275`-diagnostic residual.** File 42's finding that the composition wall fires with a worse,
anonymous diagnostic on a rigid non-inhabitant than on a fully abstract parameter (`46:6.2` extends this
to concrete non-inhabitants). Recorded, not resolved; belongs to whoever next touches the wall's
residual.

**The `Gcd`-for-a-local-`Rhs`-on-a-sealed-`Self` coherence question.** Argued moot in section 1.12
(every consuming position also bounds the other operand on a carrier trait), compiled in one direction
by file 46 (`impl Gcd<LocalRhs> for H` is admitted by coherence and cannot be spent), not compiled in the
other direction a distrustful member might still want to run.

**Whether `Int`'s drop is correct.** Persona-decided, three independent reads in agreement, flagged
loudest of everything in this document for op's morning review per section 2. Restoring it is one line.

**The crossing contract, the dithered entry point, and the membership predicate have never been typed
from the outside.** File 47's own closing observation, carried forward explicitly because it names a
method as much as a finding: the one dispatch that wrote consumer code for the fold and the numeral
declaration found two real defects nobody had seen from inside the mechanism's own vocabulary, and
nothing has yet done the same for these three surfaces.

## 4. The droplist

Proposals or readings tested and found not to work, decided against, or superseded, stated with just
enough of their reasoning that a member who believes a retest would come out differently knows what has
to be overturned. Carried forward from files 26 and 40, then extended.

Relocating the algebraic-law machinery to hilavitkutin: refused by op directly and independently
undercut by measurement.

Gating `arvo-graph`/`arvo-comb`/`arvo-spectral` on `AddAssoc` by default: admits the one preset whose
recurrences return wrong answers and refuses the two that compute correctly.

A documented traversal order substituting for a law: associativity is about grouping, not order. This
principle is what the short-circuit finding (section 1.14) turns out to violate, thirty-nine files after
it was first stated.

Bounding a regrouping combinator on a numeric diameter budget rather than a boolean law: refused by
measurement.

Predicting the accumulator-agreement threshold from a recovery map's monotonicity: refuted.

Computing type-level width arithmetic as a const generic under `min_generic_const_args`: refused
structurally. Replaced by type-level binary width encoding, itself later replaced by the value-unique
`Nat`/`Pos`/`Bias` encoding.

Growing an accumulator's own type on every iteration of a runtime-bounded loop: cannot work in
principle.

Declaring a fidelity-licence coercion as a trusted marker trait with no associated items: compiles clean
when corrupted.

A pushed, registered build-layer manifest for monomorphisation recovery: strictly worse than the
pull-shaped symbol-table read.

Treating `f64::mul_add` as a source-expressible fidelity liberty: it lowers to `llvm.fma`, an exact
operation, not a permission.

Citing a shipped `Monotone` law implementation for `(TowardNegative, T, TowardPositive)`: does not
exist as any implementation.

Assuming the recovery-map classification's cheapness transfers automatically to a new operation:
refuted twice independently.

"Past the top is unreachable" once infinity is representable: false.

The unsigned faithfulness blanket over every `Resolution` pair: refuted by compiled counterexample.

Classify-then-round as the quantiser's order: disagrees with all three test standards.

Two round-trip theorems as the crossing contract: the second is false the moment signed zero, NaN
payloads, or decimal cohorts exist.

A single three-instance `Sign` axis bundling range and zero-count: under-determines the set and mixes
a value fact with a datum fact.

Block floating point as evidence for nesting: the nesting decision stands on the `Underflow` argument
alone.

Referential uniqueness as an alternative to value uniqueness: fails the ordinary case of storing a
product in a declared numeral.

Projecting a trait-level width computation back down into an ordinary const parameter: refused, the
feature named is unvetted.

The claim that the shipped width chain and integer adjustments already satisfy value-uniqueness: false
for the width chain.

"Two spellings of one condition" for interior safety and total safety: two distinct conditions serving
two distinct promises.

The ordered three-relation ladder: replaced outright by the nine-point view lattice.

The reification-stability generalisation: true of one reifier and false in general.

`Op::IS_EXACT` alone as the statement that an operation's grade monoid is trivial: false in general,
corrected to the conjunction with `Total<Op>`.

A consumer-declared required view as the mechanism gating a regrouping's licence: killed by the compiler
mid-dispatch. Replaced by the transfer rule, itself now carried by a type projection rather than a
declared const (section 1.14).

The subset-domain reading of the view parameter: not closed under meet. Replaced by the quotient-of-
the-grade reading.

`Bias` as a plain signed integer: made a legal MATLAB numerictype unrepresentable. Corrected to a
signed, gcd-normalised rational, now built and sealed (section 1.11, 1.12).

Three separately-restated `Numeral` member lists across files 35, 36 and 38: none of the review's
compiled results depended on any of them.

The vacuity worry against D39: correct about the top of the tower and wrong as a verdict on the
predicate itself.

The consolidation sentence "`Pos`, `Nat` and `Int` are sealed" (`40:446`): false twice over, once of a
copy nothing composed with (file 42's finding against file 36's own orphaned demonstration) and once of
`Adjustment`, which the sentence never named and which was open in the copy everything actually
composes with until file 46 closed it. `Int` is now dropped from the ratified table entirely (section
1.11), so no future version of this sentence should name it at all.

File 41's own conclusion that "the design cannot have a generic `BiasMul` trait": too strong. File 42
built the trait, `BiasMulGeneric`, unified over all nine sign combinations, at the same measured
compile cost as the bare-alias form file 41 shipped instead.

The hypothesis that the short circuit breaks a term's grade under regrouping: refuted by its own
author's exhaustive compile (file 47, 81 four-leaf terms, three groupings, both readings). What
survives is the weaker and worse property that the report is not invariant under reordering, which is
what actually decided the evaluation-strategy sentence.

The claim that hilavitkutin's RCM renumbering is exactly a permutation of a column's traversal order
(file 47): false against the consumer's own canonical design. RCM permutes which column sits at which
arena offset and which WU runs when; a fold's per-record index is untouched by both.

"The `generic_const_exprs` wall is not near the problem" as a claim about the grade projection in
general (file 47): true of the wall it checked, false of the other one. The `Reduce`-as-bound
divergence (`E0275`) is one where-clause away from the same combinator, compiled by file 48; the
projection-chain structural constraint (section 1.11) is the design rule that keeps it clear.

A bounded alias table (0 through 1024) as the shape for numeral notation: rejected on principle by the
persona checkpoint as a hardcoded threshold of exactly the kind arvo refuses to ship elsewhere, and
rejected on evidence by file 48, whose out-of-range refusal on a real division constant (48000) shows
the bound is not merely inelegant, it is wrong for the design's own operation surface.

Digit-munching a numeral through the tower's own `Dbl`/`DblInc` combinators as the notation layer: puts
a projection chain in every declared type, which is the class of construction the design has spent this
whole stretch keeping out of consumer-facing signatures.

A single fold combinator with the grouping strategy as a defaulted generic parameter (the shape that
would keep a ceremony-free common path and a sequential escape hatch under one name): refused by the
language itself, not by the design; defaults for generic parameters are not permitted in that position.

The overflow-band claim "inhabited for multiplication, division, mixed-format addition and every float
operation" as one undifferentiated sentence: two of its four members were carried on the strength of
their siblings for sixteen files and both needed correction once checked (division: empty at same
precision; mixed-format addition: inhabited except a dividing-quantum degeneracy). The fourth member
("every float operation") had no derivation anywhere in the review and is struck rather than corrected.

The consolidation prediction that division has no finite accumulator solution at all: true only in the
dyadic coordinates it was originally written in, false in the ratified rational coordinates, where the
accumulator exists at Theta(2^p) bits, a finding sharper than either the original prediction or its
naive negation.
