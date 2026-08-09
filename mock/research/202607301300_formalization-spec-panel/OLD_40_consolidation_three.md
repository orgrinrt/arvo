# The current shape, third consolidation: identity and algebra assembled and tested

File 26 stood alone as the reference after the algebra sub-dive that file 12 opened. This document
replaces it. Fourteen deliverables ran since (files 27 through 39, plus checkpoints 30b, 34b, 39b),
and they close the thread file 26 opened without finishing: what a number's identity actually is,
how the laws over it are stated, and what both cost when lowered. The stretch also does something
file 26's own stretch could not, because the pieces were not yet in place to attempt it: it tests the
assembled design against op's own standard, MATLAB, IEEE 754 and SystemC not as inspiration but as a
pass/fail check, and it reads the two pieces of prior art eight separate members had flagged and none
had opened.

The shape of the stretch: files 27 through 30 build the identity half from nothing (a number's radix,
precision, exponent form, sign, and the encoding that carries it into bits) and settle most of it
inside four dispatches. Files 31 and 32 harden that settlement, one by recompiling every claim
independently, the other by asking whether it lowers to good code at all. Files 33 and 34 turn to the
algebra and restate every law against the now-settled identity, finding that four of its clauses were
compiled once and never checked against each other. Files 35 and 36 remove two axes from the ratified
table entirely and replace informal width arithmetic with a value-unique type-level encoding. File 37
replaces the review's long-standing three-way fork over which relation a law is stated under with a
single computed lattice, and shows the fork was never real. File 38 audits the whole assembly, finds
three careless restatements of a four-line trait and one real gap in the algebra's own account of
exactness, and merges everything into one shape. File 39 runs the standards test against that shape
and against the two long-flagged prior-art documents, and finds the shape passes everywhere except
one member, `Bias`, which the stretch had quietly assumed was an integer and which MATLAB's own
documentation says is not.

Op ratified three results at the ninth checkpoint, held one pending a repair, and left one tick open
with new corroboration recorded against it. Both are stated in section 2 in op's own words. Section 1
is the design as it now stands, with the evidence class of every load-bearing claim stated in the
sentence that makes it, following file 38's own four-bin discipline: machine-checked by construction,
machine-checked by bounded exhaustion at a model width (whose transfer to real widths rests on the
forbidden-feature bans in `unstable-features.md`), measured on the pinned toolchain and target, or
reasoned without a compiled artifact. Section 3 states what remains open, including the bias defect,
which op's own checkpoint says to consolidate around rather than wait on. Section 4 is the droplist,
carried forward from file 26 and extended.

## 1. The agreed shape

### 1.1 What a number is

Unchanged from file 26 in substance, sharper in its coordinates. A value of `Number<N: Numeral, S>`
is an integer k, drawn from a finite interval, together with a type-level rule injecting k into a set
of rationals (plus, for floats, a handful of data that are not rationals at all: `Specials`). The
numeral has two jobs, naming the representable set and naming the indexing, and the identity half's
whole contribution this stretch is separating those two jobs onto two different sides of the design
rather than deriving one from the other.

**D69 is overturned** (op, `30b`): identity is parameterised in mathematical coordinates, not encoding
coordinates. Precision and the exponent bounds are primitive; total width, the hidden bit, and field
encoding are derived on the physical side. Two independent readings reached this (files 27 and 28,
formed independently, one from the shipped facade's own declaration, one from Flocq's two-sided float
formalisation that CompCert ships on), which is the threshold this review's own discipline requires
before a call of this kind reaches op. The standing consequence: the off-by-one against real hardware
float formats that file 26 carried as unresolved (`26:649-652`) was never a gap to patch. It was the
parameterisation reporting that it pointed the wrong way.

### 1.2 The identity contract

Settled by files 30 and 31, re-verified independently by file 34, defended against three careless
restatements by file 38 (section 1.16 below), and corrected in one member by file 39.

```rust
pub const trait Numeral {
    type Radix:     Radix;        // 2 and 10 instantiated; any r expressible
    type Precision: Precision;    // significand digit count, primitive (D69)
    type Exponent:  ExponentForm; // where the exponent lives; nests the rest
    type Domain:    SignDomain;   // NonNegative | Symmetric | AsymmetricLow, a value fact
}

pub struct Implicit<const E: Exponent, A: Adjustment, B: Bias>;
pub struct Ranged<const EMIN: Exponent, const EMAX: Exponent, U: Underflow, S: Specials>;
```

Four top-level members rather than the original five, with `Adjustment`, `Bias`, `Underflow` and
`Specials` nested where they mean something (the spec's own `Underflow`-nests-because-a-constant-
exponent-has-no-bottom argument, applied consistently by file 27 rather than once). `Radix` closes
the radix gap file 26 carried open (`26:643-647`): without it the finest-inhabited-system derivation
of section 1.7 cannot be written generically, and decimal64, D58's own proof case, is a radix-ten
instance the prior axis set could not express at all.

**Specials as identity, measured cheap.** `Specials` (infinities and NaN, on the `Ranged` branch)
costs the arm its own `ExponentForm` instance can express and nothing else: `Implicit`'s classify
compiles to five branchless instructions, `Ranged` with `Specials` to six, both `csel`-based, no
branch, no shared body with a runtime flag (file 32, disassembled). `Implicit` genuinely has no arm to
write against specials, so there is no cost to eliminate, only a cost that was never possible to incur.

**Sign splits into a value fact and a datum fact.** File 28's three-instance `Sign` axis
(`Unsigned`/`TwosComplement`/`SignMagnitude`) bundled a value fact (the representable range) with a
datum fact (how many bit patterns carry zero). File 30 split it, exhaustively verified independent by
file 31: `SignDomain` (`NonNegative`/`Symmetric`/`AsymmetricLow`) on `Numeral`, `SignIndexing`
(`Unsigned`/`TwosComplement`/`SignMagnitude`/`OnesComplement`) on `Encoding` below. One measured
consequence: **`SC_SAT_SYM` is not a saturation mode.** The identical `TowardNegative` clamp delivers
`-8` under `AsymmetricLow` and `-7` under `Symmetric` (compiled, files 30 and 31 independently). It is
one fewer `Policy` instance and one fewer special case in the `conv-systemc` alias set.

**Nesting itself stands on the `Underflow` argument alone, and block floating point is not evidence
for it.** File 30 offered a shared-exponent block-floating-point format (the OCP microscaling family)
as a forward-provision argument for nesting over a flat axis list with a `WellFormed` predicate. File
31 found this does not survive scrutiny: a BFP mantissa's value depends on a shared exponent stored
*outside* the value it parameterises, so it is not a branch of `ExponentForm` under either shape, it
is a different kind of object, a composite numeral built from a pair. The nesting decision's own
standing is untouched; the BFP citation is withdrawn as evidence for it and scoped, per file 27's own
decimal-cohort precedent, as a requirement the identity contract must not build itself shut against,
not a design built now.

### 1.3 Encoding, nested inside Lowering

```rust
pub const trait Lowering {
    type Encoding:    Encoding;
    type StoredWidth: StoredWidth;
    type Layout:      StorageLayout;
    // Widening: RATIFIED removed. See 1.11.
}

pub const trait Encoding {
    type SignIndexing: SignIndexing;      // Unsigned | TwosComplement | SignMagnitude | OnesComplement
    type Fields:       FieldLayout;       // field widths, hidden bit, encoding bias, reserved codes
    type Canonical:    Canonicalisation;  // signed zero, preferred cohort, NaN canonicalisation
}
```

Nested inside `Lowering` rather than a third type parameter on `Number<N, S>`, so the two-parameter
fused form survives and the 1.8x rendered-diagnostic-length cost of a three-parameter split (measured
in file 26's own dive, `26:32-35`) is not paid a second time. `Lowering`'s charter is restated in one
sentence and this restatement is load-bearing for everything downstream of it: **`Lowering` changes no
value. `Encoding`, nested inside it, may change which datum carries a value. Every operation whose
result depends on that is declared a datum-level operation, and no law may read one.** (file 30,
carried unchanged through every subsequent file). This is what lets the identity inversion and the
datum/value distinction coexist without fighting, and it is enforced structurally rather than by
convention: a law's key, per section 1.8, is a `const fn` parameter list, and `Lowering` is not a
parameter, so reading it fails with `E0425` at the point of use (file 26's own const-fn-key mechanism,
now applied to a scope one level higher by files 34 and 37: a value-level fact declared in the
algebra-contracts crate cannot even name an `Encoding` or `Lowering` type, because the name does not
resolve, `E0433`).

Measured (file 32): a trivial `Canonicalisation` (identity, every `Specials = None` composition) adds
zero instructions to an operation body; a trivial `FieldLayout` (no hidden bit, no encoding bias)
produces byte-identical code to a hand-rolled shift-and-mask extractor on a packed sixteen-bit column.
A richer `Canonicalisation` (real NaN payload rules, decimal preferred-exponent selection more complex
than a range test) costs a small, branchless, measured constant for the simplest real cohort collapse
tested (seven instructions against two, `csel`-based, no branch), and whether every plausible
`Canonicalisation` instance stays branchless is unmeasured and open (section 3).

### 1.4 The crossing contract

Settled as a three-statement section-retraction pair rather than the two round-trip theorems file 28
first proposed, which file 30 found file 28's own section 2 contradicts (signed zero, NaN payloads,
and decimal cohorts each falsify "encode after decode is the identity on data"). Recompiled
independently by file 31 against the pinned nightly rather than trusted from file 30's `OUTCOMES.md`:

1. `decode ∘ encode = id` on values, always.
2. `encode ∘ decode` is idempotent on data, always. This is canonicalisation, and it is where a
   canonical encoding, decimal's preferred exponent, and NaN canonicalisation all live.
3. `encode ∘ decode = id` on data iff the encoding is injective, a derived boolean rather than an
   assumption.

All three are per-value statements over a finite datum set, so the model-width exhaustive mechanism
applies unchanged, and the same-text-monomorphised-twice discipline of the recovery-map witness
applies to the encode/decode pair verbatim. `Canonical` is load-bearing twice over: as this contract's
idempotent second statement, and, per file 34, as the definition of the equality every algebraic law
is stated under (section 1.8).

### 1.5 The quantiser

**Round first, classify second**, replacing the spec's original classify-then-round order (file 28,
recompiled by file 31, extended to the signed case by file 31's own new probe, and independently
re-verified by file 39 against IEEE 754-2019 clause 7 directly rather than through any panel file's
paraphrase). The map is: round on the unbounded-exponent extension of the grid by the direction
triple, then classify the rounded result against the range (including `Specials`, where present) and
resolve by the range rules. File 27's own proposed correction, that with infinity representable "past
the top is unreachable," is itself wrong: infinity does not remove the over-range position, it changes
what its neighbour is, and the midpoint that decides ties-to-even overflow lives on the unbounded grid
this amendment supplies, not between the largest finite and infinity, where no midpoint exists. With
this in place, three attributes checked exhaustively at a model float against oracles written
independently from the standard's own text (roundTiesToEven, roundTowardZero, roundTowardPositive)
agree exactly, on both signs, with no new axis and no new `Resolution` (files 30, 31). The overflow
band that motivated the fix is empty for same-format addition and inhabited for multiplication,
division, mixed-format addition and every float operation, which is why the additive-only stretch of
this review could not have found it.

**Dither is a zero-state extra argument, free once round-first exists.**
`quantize_dithered(exact, noise) = quantize(exact + noise)`; arvo never generates the noise, the
caller supplies it, and the function stays const-callable and stateless on arvo's side by construction
(file 29). It does not compose with `Refuse` as stated: a dithered value one quantum from either end
of a `Precise` numeral's range can refuse where the undithered value was total, on inputs the numeral
represents exactly (file 30, recompiled by file 31). Confining the dithered value to the numeral's
range before quantising restores totality and is a real, non-free cost (it costs the dither its
uniformity within one amplitude of either end). Whether the design confines by default or gates the
dithered entry point on totality is undecided and is the same shape of question as what `Precise` is
for (section 3).

**Shaping (error feedback) is a scan, not a fold**, and this dissolves the type-level conflict file 29
raised rather than needing a new refusal mechanism for it (file 30). A shaped fold cannot be
regrouped, because its state is a strict sequential dependency, and file 29 worried this needed a
type-level marker to prevent a scheduler from silently regrouping a feedback loop. File 30's answer:
name it correctly. Error feedback in every real instance (Floyd-Steinberg, delta-sigma converters) is
a stateful map producing a sequence, which is a scan, and a scan has no grouping freedom to forfeit
because it is sequential by definition. The one genuinely shaped fold, compensated summation, gets its
own named combinator (`fold_compensated`) sitting beside `fold`, which is the one that carries the
regrouping bound. Two function names with different bounds is a structural refusal that costs nothing
new. A counter-reading is carried honestly: if a future scheduler consumes combinators generically and
picks a strategy from a fact rather than a name, the name stops being a refusal and a marker is owed
after all; whether that scheduler exists is a hilavitkutin question the design has already ruled is
not arvo's to answer.

### 1.6 Membership and the number-system layer

**D38 and D39 are op's calls** (the `arvo-num-systems` crate: ℕ, ℤ, ℚ, ℝ, ℂ, ℍ, 𝕆, surreal,
hyperreal, p-adic, shipped even if nothing uses them, vocabulary fixed by mathematics; membership
through algebraic structure, depending on the algebra ladder). **D39 is held, not overturned**, at op's
seventh checkpoint (`30b`), despite two readings finding its stated mechanism does not compile and
that a membership predicate over the whole ambient set is vacuously true of everything.

The correction two independent readings agree on (files 27 and 28, formed independently, files 30 and
31 concurring): **membership licenses only the exact, widening operation family. Quantised in-numeral
operations get their laws from the algebra ladder, keyed exactly as section 1.8 keys them, never from
membership.** Inclusion into an ambient set is a homomorphism only for the exact family (`mul_full`,
exact addition), where no recovery map fires; every quantised in-numeral operation's recovery map is
precisely the measure of inclusion's failure to be a homomorphism, and the measured law inversions
already recorded in file 26 are its empirical face. `ExactWindow<Op, Rhs>` (file 28's window-closure
mechanism: the derivable fact that an exact operation stays inside an expressible window) is the
concrete content membership licenses, and it is not total where `Specials` exists (file 30): with
infinity representable, `∞ * 0` lands in no window, so `ExactWindow` gates on `Specials = None` as the
honest first ship.

**File 39 reads the crate's own topic file and offers a candidate reading for D39's honest content,
which it explicitly does not resolve and flags as needing a second independent read before anything
builds on it.** The topic's own text states the precision two panel members had missed: the predicate
is *inhabits*, not *equals*, so `Inhabits<Real>` being vacuously true of every finite numeral is a
correct fact about the top of the tower and not a refutation of the predicate. The honest derived
fact is the **finest inhabited system**, which exists and is unique because the tower is a chain, and
this is the identical move file 37 makes for the laws: do not choose a relation, report the finest one
that holds. Membership is then decidable from the identity axes alone (`Natural` iff
`Domain = NonNegative` and quantum and bias are naturals, etc.), the same derived-fact-keyed-on-
members shape the algebra ladder already uses, and the spec's own text already contains the first
instance of it (`impl<N: Numeral<Bias = Zero>> AddClosed for N {}`) without naming it as membership.
The finest-system derivation table's own agent-written contents are stale against the ratified
contract in three ways (it credits every fixed-point type to ℤ[1/2], missing the radix-ten case, the
`FullRange` case, and any rational bias) and should be derived from the `Numeral` members rather than
hand-fixed a second time.

### 1.7 The algebra: what a law is, and the finest view it holds at

**A law is a claim that the terms of one grouping class stand in a relation, under a stated view, over
the value set of a numeral, quantified over the class rather than pairwise, and keyed on every
parameter its proof used.** It is a `const fn` whose parameters are its key and whose return type is
`Never` or the finest view under which it holds, derived by blanket construction over the composition
rather than declared per type (D51), safe when derived and `unsafe impl` when asserted (D16).

**The finest-view mechanism replaces the three-relation fork.** This is op's third and final
ratification at the ninth checkpoint. File 33 imported the standard partial-algebra vocabulary (weak
equation, existence equation, Kleene equation) and file 26's fused-verdict question ("which relation
does the design state a law under, and it decides how `Precise` reads," `26:608-617`) looked like a
choice among these three. File 37 found it is not a choice: a term's meaning is a grade (a free
commutative monoid over refusal causes and quantisation events, exactly the object file 26's own
graded reading already names) and a value, a view is a monoid homomorphism out of the grade, and two
terms are equal under a view when the view sends their grades to the same thing and their values agree
wherever present. Compiled exhaustively over nine such views and nine compositions: the set of views
under which a law holds is downward closed and closed under join, so **every law has a unique finest
view**, and that view is the law's content. The named relations are three points of a nine-point
lattice, and the lattice is not a chain: `Hot` on a signed numeral and `Precise` below its accumulator's
interior-safety threshold sit at incomparable points (one preserves values and definedness while
losing quantisation events, the other preserves values and events while losing definedness), and
this is precisely why the open question about how `Precise` reads had resisted three files: the
vocabulary had no name for the point it actually sits at. Both of file 26's held readings were right
about their own half, and the lattice serves both without a trade.

**Law equality is the canonical quotient**, settled by file 34 and carried unchanged through 37/38:
two results are law-equal when canonicalisation sends their data to the same datum. The shipped
`arvo-numeric-contracts::TotalOrd` was found by file 34 to induce a *datum*-level order (it separates
signed zeros and orders NaN payloads, matching `f64::total_cmp`/IEEE 754's `totalOrder`), so it cannot
be the definition of law equality as file 33 first proposed; it survives, reclassified, and needs one
sentence declaring which level it operates at (open, section 3).

**The key.** The operation, whose marker carries whether its grade monoid is trivial (`IS_EXACT`,
quantiser presence); the operand numerals and, for a widening operation, the result numeral; the
`Quantisation` resolutions and, where a quantiser sits between the exact operation and the result, its
`Direction`; for a fold, the accumulator numeral and the arity. `Growth` is not in the key (section
1.11). `Lowering` cannot be named from where laws live (section 1.3). One correction to file 37's own
statement, found and repaired by file 38, compiled at an eight-value model: **`IS_EXACT` alone does
not trivialise an operation's grade monoid; `IS_EXACT` and `Total<Op>` together do.** Exactness kills
quantiser-generated events and causes; totality kills causes with no quantiser origin (divide-by-zero
is the design's own standing example). No shipped or designed operation is exact-and-partial today, so
nothing measured elsewhere is wrong, but the uncorrected sentence would be false the moment a value-
level exact division exists, which the type-level half of file 36's work already anticipates.

**Direction enters a law's key exactly when the exact result can leave the operand lattice.** File 33
derived this as a single predicate replacing two separately-measured facts (`Precise` addition never
rounds in range; `Precise` multiplication rounds on roughly half of pairs). Additive lattice closure
holds exactly when bias/adjustment is an integer (the shipped `AddClosed` gate on `Bias = Zero` is the
special case); narrowed-multiplicative closure additionally needs the adjustment and bias both
integers, which no fixed-point numeral with a fractional digit ever satisfies, which is the derived
reason multiplication needs `mul_full` and addition does not.

**The transfer rule, and why no consumer declares a waiver.** File 37's first mechanism let a consumer
declare a required view and checked the law against it; the compiler killed it, because the licence
check refused exactly the case the mechanism existed to handle, which turned out to mean two different
things had been run together. The repaired rule: **a regrouping publishes, in its own result grade,
exactly the grade generator classes its law fails to preserve. Tolerance is a transfer, never a
waiver.** Where the weak equation itself fails, the regrouping is refused outright rather than
published, because no publication rescues a genuine value divergence. There is no consumer-supplied
index to be too rich (the risk file 37's own field names as its worst failure mode); the caller's
contract is the ordinary type of the result, and a caller needing a definedness-faithful fold takes
`Folded<0>` and a `Precise` regrouping below interior safety delivers `Folded<1>`, refused by
`E0308` with no bespoke machinery. This is the coeffect-discharging-into-an-effect asymmetry file 26's
own graded reading names as its single spec-worthy sentence, here used to make itself unnecessary: the
permission-shaped fact became data-shaped, and the type system checks it for free. The published grade
is declared and checked, never computed (computing it in return position hits the same forbidden
`generic_const_exprs` wall as everything else touching a generic const in type position); understating
it refuses, overstating it compiles and is merely pessimistic, the same safe direction the design takes
everywhere on lattice containment.

**The mechanism is priced against the alternative file 33 first proposed (five derived marker traits)
and wins on both axes**: 0.130 ms/composition against 0.193 ms, 907 bytes against 1854, at
`--emit=metadata`, expressing nine points where the marker shape expresses eight of which five are
junk. It is two orders of magnitude below file 36's type-level gcd cost and is a neighbour to, not an
answer for, the still-open real-consumer compile-cost question (section 3).

**One sentence the design owes and does not yet state**: the evaluation strategy of a refusing
operand's sibling (strict evaluation accumulates its quantisation events; a left-to-right short circuit
does not). Measured to change the published grade and no law's verdict, at every composition tested.
File 39's standards test tilts this toward the strict reading (IEEE's sticky flags, SystemC's
per-variable flags, and MATLAB's overflow logging are all strict-evaluation shaped: an operand's flags
are raised by whatever computed it, regardless of its sibling) but does not decide it; the fold
combinators are the design's own object and the choice is op's.

### 1.8 The fold: two conditions, two relations

File 33 first stated interior safety in radix-free value coordinates: a fold of arity n over
destination numeral N with accumulator numeral M is interior-safe when M's lattice refines N's and
`(n-1) * [min V(N), max V(N)]` is contained in `[min V(M), max V(M)]`. File 34 found the
consolidation's two width-coordinate formulas for this (`ceil(log2(n-1))` and `ceil(log2 n)`) are not
two spellings of one condition, they are two separate conditions serving two separate promises.
**Interior safety** (the n-1 factor): no quantiser fires in the interior, so the fold is
grouping-invariant, which is the law's own condition, and file 33's three-line proof (an exact total
computed once at the root cannot depend on grouping) survives it unchanged. **Total safety** (the n
factor): the accumulator is invisible in the delivered function, so the fold equals
`quantize ∘ exact_sum`, which is the *specification's* condition and matches the DSP guard-bit sizing
the design already cites (eight guard bits for 256 MAC steps on the Motorola 56000). The two are
related by the refinement order, not by any view: below total safety a fold is strictly less defined
than its own specification, and interior safety can hold while total safety does not (a refusal the
destination would have absorbed can surface as an accumulator refusal). A combinator states which
condition it checked and the law it derives is keyed accordingly.

For a multiply-accumulate, the same two conditions apply with N replaced by the product numeral
`mulnum(N1, N2)`, with one repair for biased operands (file 34, exhaustively checked): the pairwise
closure predicate correctly reports a biased product numeral is not itself additively closed, and the
fix generalises file 31's gcd formula with a fourth monomial: the accumulator is the zero-bias numeral
with adjustment `gcd(A1A2, A1B2, A2B1, B1B2)`.

At interior safety, all three grade components (value, definedness, quantisation-event multiset) agree
at once, because exactly one quantisation fires, at the root, on a grouping-independent argument; this
is the strongest row of file 37's own compositions table and the design's clearest single argument for
where the widening effort belongs, since `Warm`/`Cold` go from having no law at any view to having
every one, purely by widening the accumulator, with no axis changed.

### 1.9 The multiplicative half, extended

`mul_full` is a family of maps `N1 x N2 -> mulnum(N1, N2)`, not an operation on one set, and its own
associativity does not typecheck until the numeral-level map's associativity is established first
(file 33, a precondition nobody had stated of file 26's own multiplicative-half headline claim). The
biased product numeral, `bias = B1*B2`, `adjustment = gcd(A1*A2, A1*B2, A2*B1)` (file 31's closure
formula for the consolidation's own open closure gap, `26:326-331`), collapses to the shipped exact-
product rule when both biases are zero, which is the property that decides whether a generalisation is
worth having. It generalises to n factors: the bias is the all-bias monomial, the adjustment is the
gcd of every monomial carrying at least one adjustment, associative and commutative because the
monomial set is symmetric under permutation of the factors (file 33, checked at arity three with a
negative control confirming the cross terms are load-bearing, not an arity-two coincidence).

**Distributes is not an atom on a chain: it is Monotone**, checked as a biconditional both ways (file
33), but the biconditional needs a stated relation to be true, which file 33's own probe left implicit
by testing only total operations. File 34 found the split: for a total operation on a totally ordered
value set, distributivity over the lattice operations is monotonicity, full stop; for a partial
operation, monotonicity gives only the weak-equation-level implication, and the Kleene-level statement
additionally depends on which of IEEE's two lattice-operation families is meant (`maximum`, which
propagates an undefined operand, or `maximumNumber`, which suppresses it; both are required by the
standards test, section 1.14). No preset the design ships or can spell is a dioid over `(max, +)`:
wrapping addition fails distributivity, saturating addition fails associativity and separately fails
the annihilation axiom, `Precise` addition is partial (file 33). This is not grounds to drop the
`Dioid` rung under D47 (the ladder goes as deep as the theory does); it is grounds for the rung being
derived rather than declared, reporting a correct "no" with the failing axiom named. A numeral carrying
an absorbing `Specials` element could make the rung non-empty, which is scoped as a requirement on the
identity contract discovered from the algebra side, not designed now.

### 1.10 Widening and Growth: two axes removed

**Both collapse, and neither is a renaming.** Op's eighth checkpoint directed the two table-touching
readings from file 34 to go to the same dispatch as one thread, because ruling on either alone would
leave the other's argument incomplete. File 35 answered both, compiled at native and multi-limb width.

`Widening`'s three old instances (`None`, `InContainer`, `PerOperation`) decompose entirely into three
pre-existing mechanisms: which primitive is named, what numeral type that primitive's return type is
(the `mul_full`/accumulator machinery the multiplicative half already built for an unrelated reason),
and that numeral's own `StoredWidth`/`Layout`. Measured, `-C opt-level=3`, no LTO: a direct wrapping
multiply, a composite `mul_full`-then-`quantize` call, and a `Precise`-shaped exact-widening call all
fold to the identical single instruction at native width; at a genuinely harder multi-limb width (128-
bit operands, a real 256-bit intermediate, where a truncated result needs fewer limb-products than a
full one) the composite form still folds to the direct hardware multiply's four instructions once the
optimiser can see through it, and forcing it opaque (a check-build-shaped negative control) pays the
real, non-folded cost, confirming file 34's own axis-legibility-versus-codegen-quality lesson at a
second operation and width. No preset loses anything: `Precise` never needed `PerOperation`'s own
vocabulary, it needed the accumulator-sufficiency check the multiplicative half already shipped.

`Growth` leaves the key because the operation's own name already determines quantiser presence, which
file 33's own key table stated twice without noticing the two rows said the same thing (`Growth`,
"decides whether a quantiser is present"; result numeral, "never for widening operations"). File 35
turns this from an observation into a structural fact by binding the exactness fact as an associated
const on the operation marker (`Op::IS_EXACT`), so the vocabulary to pair it with a contradicting
`Growth` instance no longer exists to be wrong.

Together, the Lattner gap file 26 carried since file 12 (`Growth::Exact` paired with `Widening::None`
having no implementable carrier, `26:52-59`) does not merely get a compatibility predicate; it becomes
**unstatable**, because neither name survives as an axis instance in the resulting vocabulary. This is
stronger than the consolidation's own framing anticipated.

**Whether `Growth` also leaves `Policy` entirely, not merely the key, is argued and not compiled**,
and stands as the one open ratification tick (section 2, section 3). File 35's own argument: `Growth`
was never a unary fact about one numeral at all, it describes a relationship between an operand numeral
and a result numeral, which is a fact about an operation's signature and belongs there, not on `S`.
File 39's standards test corroborates this from outside, in the standards' own vocabulary: IEEE 754's
`formatOf` operations put the destination format in the operation's name, MATLAB's product and sum
wordlength rules are stated as facts about the operation, and SystemC's expression precision is
per-expression, per-node. Three standards designed decades apart, independently, all place growth on
the operation's signature and none on a unary type property. This does not move the item's evidence
bin, because it is not a compile, but the reasoning is no longer only an internal type-shape argument.

### 1.11 The numeral encoding must be value-unique, and the naturals are the larger half of that obligation

Op adopted this obligation at the eighth checkpoint: numeral encodings must be value-unique as types,
or a law about a numeral-producing operation splits into a true value half and an ill-formed type
half. File 34 compiled the failure that motivated it (two bracketings of a biased triple product,
componentwise associative as arithmetic, spelled as two different types, refused by `E0308` with no
correct expected value to write down) and marked the rational adjustment as the piece needing repair,
recording in passing that the shipped width chain and integer adjustments already satisfied the
obligation. **That reassurance is false**, and file 36 found the false half is the larger job, because
it is under everything the design's own width-adder machinery touches. `UInt<UTerm, B0>` inhabits
`Width` with value zero, so does `UInt<UInt<UTerm, B0>, B0>`, and the adder propagates the spelling
rather than normalising it, refusing exactly as file 34's own construction did one layer down.

**The fix is a different encoding, not a normalisation pass.** `Pos ::= H | O<P: Pos> | I<P: Pos>`,
`Nat ::= Z | Pz<P: Pos>`, `Int ::= Z0 | Zpos<P: Pos> | Zneg<P: Pos>`, Coq's `positive`/`N`/`Z`
(Barras et al.), where the terminator is the leading one and there is nowhere to put a zero in front
of it. Uniqueness holds by induction on the value, with no normalisation operator anywhere in the
design because there is nothing it could do; the induction's hypothesis is that those are the only
impls, so `Pos`, `Nat` and `Int` are sealed, and file 36 committed both the sealed crate and a
genuinely separate downstream crate's refused attempt to reinstate the unsealed defect, one crate
away, invisible to arvo without the seal.

The rational adjustment's coprimality is a relation between two independently chosen components and
cannot be enforced by construction; it is enforced where it is observed, a conditional impl
(`N: Gcd<D, Out = H>`) so an unreduced ratio is a well-formed type that is not an `Adjustment`, with a
normalising consumer-facing alias (`Reduced<N, D>`) so two spellings of one quantum unify before
anything asks whether they do. The gcd is Stein's algorithm on this encoding (file 36 read typenum's
source directly rather than assuming Euclid, and it is Stein's too, so no algorithmic novelty is
claimed), and the encoding makes three of Stein's five steps pure impl selection and eliminates the
odd/odd step's halving entirely. Reduction needs exact division by an odd divisor, the classical
Hensel/LSB-first form, which this encoding reaches naturally because it reads least-significant-digit
first.

**Priced**, `--emit=metadata`, 400 compositions, min of three runs: the gcd costs 5.08 ms per
composition at 16-bit operands against typenum's own 15.55 ms (a 3.06x win, decomposed into 1.65x from
the algorithm's formulation and 1.87x from the encoding), and the full reduction (gcd plus exact
division) costs 12.07 ms, less than typenum's gcd alone. Over dyadic adjustments, which is every
composition arvo ships today, the full reduction costs 0.50 ms per composition because the gcd
terminates on its first impl. Zero symbols emitted at any size. One real debit not found a way around:
roughly 1.3 to 1.9 KB of crate metadata per composition, in type names.

**This whole mechanism is held, not ratified, pending a repair.** File 39 found a second defect,
independent of file 36's own scope and inside the mechanism file 36 built: file 36's own derivation,
"`bias = B1 * B2` is a signed multiplication, so `Bias` is a signed integer," establishes signedness
and assumes integrality without deriving it, and integrality is false against MATLAB's own
documentation ("the slope and bias can take on any value") and against the design's own ratified
closure formula, which mixes adjustments and biases inside one gcd, only defined if both live in the
rationals. Compiled: slope 1, bias 1/2 is a legal MATLAB numerictype whose value set is the
half-integers, disjoint from every integer-bias value set at every integer tested, with no rescue by
choice of unit. The repair uses only machinery file 36 already built (`Bias` becomes a signed,
gcd-normalised rational, the same normal form as `Adjustment`, composed rather than invented) and is
unbuilt at the trait level, the same shape op already ordered built for the rational adjustment.

### 1.12 The assembled trait table, and what it costs to build against the tree

```rust
// Every member that denotes a number is drawn from one value-unique, sealed,
// type-level encoding (held, pending the Bias repair, section 1.11):
//   Nat ::= Z | Pz<P>            P: Pos       precision, widths, exponent bounds
//   Pos ::= H | O<P> | I<P>      P: Pos       magnitudes
//   Int ::= Z0 | Zpos<P> | Zneg<P>            biases, corrected to a normalised rational (39)

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

Rewrite cost against the shipped tree is near zero: no shipped source names `Adjustment`, `Numeral`,
`FullRange`, `UTerm` or `AddWidth` (re-verified by grep in files 34 through 39), and the shipped
`IFixed` already computes width from precision-shaped parameters at its declaration site, so the
inversion moves which axis the parameters expand into, not the parameters themselves. The width chain
this stretch replaces is unshipped design-round material, so the replacement is one encoding swap in
the panel's own probe files, not a source rewrite.

### 1.13 The downstream contract, and the crate table

Both unchanged from file 26 and untouched by every deliverable in this stretch. arvo grows no build
harness of its own; a build layer reads every axis, acts freely on `Lowering`, acts on `Policy` only by
a transformation staying inside the axis's own declared envelope, and never acts on `Numeral`. The
check-build discipline (axis legibility reads a `-Cno-prepopulate-passes` build; codegen quality reads
a shipping-shaped one) gained one confirmed instance this stretch: file 32's apparent vectorisation
anomaly, initially read as a mystery about compilation-unit size, was file 34's own methodological
lesson arriving a second time. The anomaly was `-C lto=fat` deferring the vectoriser on an unlinked
`--emit=asm` build, asking a codegen-quality question with an axis-legibility flag set; once corrected,
the identity contract's compiled path and a bare `wrapping_add` fold to byte-identical machine code
under the build shape a consumer actually ships, the strongest form of erasure this review has
measured. The post-monomorphisation verifier, the semantics-free per-axis liveness check, the fold-
detection assertion, the layout-assertion precedent, the build-layer receipt requirement, and the three
ways to cross the Stage G boundary all stand exactly as file 26 recorded them. The six-crate split
stands as packaging, unchanged; the one open packaging item this stretch narrows rather than closes is
the algorithm-crate dependency edge onto `arvo-algebra-contracts` (`26:505-510`), for which D39's own
dependency arrow onto `arvo-num-systems` is now the first ratified fixed point, per file 39's reading
of the topic file directly.

## 2. The lead designer's calls

Op made three ratifications, held one item, and left one tick open at the ninth checkpoint (`39b`),
after two prior checkpoints (`30b`, `34b`) whose standing instructions govern every file in this
stretch and remain in force.

**The seventh checkpoint (`30b`) overturned D69** on the two independent readings of files 27 and 28,
held D39 rather than overturning it (two negative findings are not grounds to withdraw a call whose
positive content is still being worked out), and reframed the whole dispatch from finding-and-
refuting toward convergence, in op's own words: "I think we want to start getting them to converge on
solutions, together, building up from each other and strengthening the spec." Every file from 31
onward is written under that instruction: strengthen a predecessor's surviving proposal and carry it
forward as shape rather than restating that it survived; replace what fails, with the replacement as
the deliverable; leave the design more settled than it was found.

**The eighth checkpoint (`34b`) adopted numeral value-uniqueness** on file 34's compiled failure, sent
the `Widening`/`Growth` pairing to the same dispatch as one thread because ruling on either alone would
leave the other's argument incomplete, and issued a posture instruction that outranks the three calls
where they conflict, in op's own words: "let's be novel here too, or at least attempt to find
solutions that seem unsolvable, but only in lack of prior art on it... in worst case, just fall back to
what we've been doing and find solutions anyway." The distinction drawn is between a thing that is
impossible (a real constraint of the type system, monomorphisation, or the forbidden-feature list) and
a thing nobody has done (an absence of prior art, not a constraint), with the second explicitly not
license to permute the axes already in hand or to propose a mechanism without compiling it.

**The ninth checkpoint (`39b`)** opened by refusing the ratification it was offered and asking a
question none of the four preceding files had asked themselves: "if we lose widening and growth, do we
still retain the behavior therein, so the strategies make sense?" The answer, drawn from file 35's own
text, is that the trade between accuracy and efficiency never lived in those two axes at all: `Hot`,
`Warm` and `Cold` narrow immediately, and `Precise` does not call `quantize` in a fold interior because
its accumulator numeral *is* the product or sum numeral, sized by the accumulator-sufficiency check the
multiplicative half already built for an unrelated reason. `Quantisation` plus accumulator sufficiency
carry the whole trade, both of which stay; `Widening` was a name for a combination of which primitive is
called and what it returns, and the new shape needs no derivation rule because there is nothing left to
derive.

Op's second concern at the same checkpoint, stated while unable to read any file directly: "I want to
ensure this round of experts didn't lose sight of the intent in their effort to compile and get a valid
shape... I am now relying on your short summaries, I can not know for sure." File 39 was dispatched
specifically to close this, not to reassure, and found that files 35 and 37, the two files that changed
the ratified table, mention MATLAB, IEEE 754 and SystemC exactly zero times between them. Both passed
the standards test once run, and both came out stronger than "still representable": the `Widening`
removal is *required* by the standards, not merely tolerated, because MATLAB's `SpecifyPrecision` mode
quantises into a third, consumer-chosen numeral the old axis could not name at all. The one defect the
check found is in file 36 (`Bias = Int`), which had checked its own claims and was still wrong, against
a standard's own documentation.

**Ratified, three:**

1. `Widening` leaves `Lowering`.
2. `Growth` leaves the law key.
3. The finest-view mechanism replaces the three-relation fork.

**Held, one.** The value-unique encoding replacing the width chain stays a recommendation until the
rational-bias repair is compiled. It is the piece carrying the known defect, and a known-wrong member
inside a settled contract is the worst thing to freeze.

**Open, unchanged, with new corroboration recorded against it.** `Growth` leaving `Policy` entirely
(tick 3) stays open exactly as op called it at the eighth checkpoint, now with all three standards'
own vocabulary stated as external support and still no compiled check.

**Order of work, stated by op**: consolidate first, repair after. This document records the bias
defect as a known open item per section 3, rather than waiting on its repair.

Everything file 26 recorded as governing principle stands unchanged and applies to every file in this
stretch. The standard is optimal and ideal, capable of representing MATLAB, IEEE 754 and SystemC as a
test rather than an inspiration, an abstraction that cannot express one of them being a defect rather
than an accepted boundary; this stretch is the first to actually run that test against a settled
result, and section 1.6/1.10/1.11 above are what it found. The algebraic laws stay in arvo. The
existing code is irrelevant and every member assumes the design is being rewritten wholesale, except
where the current shape can be kept at no real cost, in which case it should be. The intent outranks
every instruction and is inferred rather than read literally, so no member resolves to a single angle
on anything substantive. Every member owes a constructive deliverable, and the files that moved this
stretch furthest each built something: the exact identity coordinates, the section-retraction crossing
contract, the finest-view lattice, the value-unique naturals, the biased-product closure. Only op's
calls are final, and even those go stale the moment something better surfaces, which is exactly what
happened to D69, and what the ninth checkpoint's own bias correction is an instance of one level down.

## 3. What is open

**The bias defect, consolidated around rather than waited on, per op's own stated order of work.**
`Bias` as currently stated in the merged shape (`Int`, a plain signed integer) is wrong: it makes a
legal MATLAB numerictype unrepresentable (slope 1, bias 1/2), contradicts the ratified biased-
multiplication closure formula's own rational algebra, and traces back through three files (`31` to
`36` to the shape stated in `38`) with nobody re-reading the ratified spec sentence that names MATLAB
as the axis's reason for existing. The repair (`Bias` as a signed, gcd-normalised rational, same normal
form as `Adjustment`) is stated in section 1.11 and costs no new mechanism, only an unbuilt trait-level
composition of sign with reduction.

**`Growth` leaving `Policy` entirely** (open tick 3) needs either the compiled check file 35 itself
proposes (an exhaustive search of arvo's operation surface for a growth behaviour that cannot be read
off from primitive and target numeral) or a ratification on the reasoning already assembled, now
carrying all three standards' corroboration.

**Which relation `Precise` reads is mathematically settled and the ergonomic question is not.** The
finest-view mechanism establishes exactly what `Precise` preserves at every accumulator width (values
and quantisation events, not definedness, below interior safety); whether the shipped combinator
surface offers only the definedness-faithful form or offers the published-grade form and lets the
caller's type decide is a question about what a `Precise` consumer expects, declined by every file that
has touched it since file 26 first raised it.

**The evaluation strategy of a refusing operand's sibling** is a sentence the design owes, not a
question awaiting more evidence: measured to change the published grade at every composition tested
and no law's verdict, with the standards test tilting toward strict evaluation without deciding it.

**The `TotalOrd` level annotation** (datum-level, 5.10-shaped, forbidden to laws; or value-level,
specified, one NaN class placed consistently) is a one-sentence fork nobody has picked.

**The dither-versus-`Refuse` interaction** (confine the perturbed value to the numeral's range before
quantising, costing uniformity near the ends; or gate the dithered entry point on totality) is settled
as a mechanism and open as a design choice about `Precise`'s consumer contract, the same shape as the
`Precise`-surface question above.

**Division is untested past the atom set's own prediction.** The consolidation predicts no finite
accumulator solution exists for division at all (`26:676-681`); nothing in this stretch measured it,
and if the prediction holds, the working assumption that addition and multiplication are the only two
cases needing separate treatment is wrong. Division by a power of the radix is named as an exact
subfamily worth offering (file 28), distinct from correctly-rounded general division; neither is built.

**The real-consumer compile-cost bench** (the atom ladder and the const-fn-key discipline against a
real consumer's composition set, `26:668-674`) remains unpriced. Every mechanism-shape sweep this
stretch ran (the identity axes, the gcd, the view lattice) prices a different, narrower thing and is a
neighbour to this question, never an answer for it, as every file that ran one says explicitly.

**Four codegen regression tests are owed and none exists**: the multi-limb carry chain, the fold-
versus-direct-multiply fold at native and multi-limb width, the saturating-reduction non-vectorisation,
and the vectorisable-loop-idiom sensitivity file 34 found. Each is one committed test standing in for a
recommendation this review has now repeated four times.

**One cell of the ratified no-gaps claim has no construction anywhere in the review**: `SC_WRAP<n>` and
`SC_WRAP_SM<n>` with `n_bits > 0` (wrap while keeping the top n bits saturated, and its sign-magnitude
counterpart). Flagged, not built, by whoever next touches the quantiser.

**Richer canonicalisation's branchlessness is unmeasured**, and cross-word bitpacked field extraction
(a field genuinely straddling a word boundary in a tightly packed run, the actual worst case for Cold
storage) is untested; both are named limits on file 32's own measurements, not contradictions of them.

**`DatumDeterministic`** (file 31's correction to D70, deriving a datum-level companion to the value-
only `Deterministic` claim from whether `Encoding::Canonical` is paid at every step that could deliver
a non-canonical datum) is reasoned and named, not built as a real `const fn` against arvo's trait
shapes.

**Membership's candidate content needs a second independent read.** File 39's reading of the
`arvo-num-systems` topic file (the finest-inhabited-system derivation resolving the vacuity worry
against D39) is offered explicitly as a candidate, not a resolution, and carries its own stated
requirement that a second member read the same topic independently before anything is built on it.

**The reduction firing site** (normalise at every naming site, cheap to build and possibly redundant;
or normalise only where a derived numeral is produced) and **whether `FullRange` survives as its own
named `Adjustment` constructor** rather than being reduced into a bare ratio are both named and
unbuilt.

**Whether the exponent should become a type** under the value-unique discipline, rather than the const
parameter it is today, is a real fork nobody has opened.

**Preset divergence** (a consumer wanting a preset with one axis overridden) has a working, probe-
verified, unstable-feature-free mechanism (a generic parameter default projecting off the parent
preset) noted at op's seventh checkpoint as available and explicitly not adopted: op's call is that
this deserves more than the first mechanism that works, and a later member should take it further.

## 4. The droplist

Proposals or readings tested and found not to work, decided against, or superseded, stated with just
enough of their reasoning that a member who believes a retest would come out differently knows what has
to be overturned. Carried forward from file 26, then extended.

Relocating the algebraic-law machinery to hilavitkutin: refused by op directly and independently
undercut by measurement, the regrouping that motivated the move already happens inside arvo's own
licensed internals before any scheduler exists to relocate to.

Gating `arvo-graph`/`arvo-comb`/`arvo-spectral` on `AddAssoc` by default: admits the one preset whose
recurrences return wrong answers and refuses the two that compute correctly.

A documented traversal order substituting for a law: associativity is about grouping, not order.

Bounding a regrouping combinator on a numeric diameter budget rather than a boolean law: refused by
measurement, signed saturating addition's diameter grows to the whole representable range by a
five-element fold.

Predicting the accumulator-agreement threshold from a recovery map's monotonicity: refuted, every
non-homomorphism resolution reaches the same threshold regardless.

Computing type-level width arithmetic as a const generic under `min_generic_const_args`: refused
structurally at the definition site, the sound subset forbids arithmetic over a still-generic const
parameter on its own right-hand side. Replaced by type-level binary width encoding, itself later
replaced (below) by the value-unique `Nat`/`Pos`/`Int` encoding.

Growing an accumulator's own type on every iteration of a runtime-bounded loop: cannot work in
principle, a type cannot depend on a runtime-only value.

Declaring a fidelity-licence coercion as a trusted marker trait with no associated items: compiles
clean when corrupted, a corrupted grant produces a silently wrong answer with no diagnostic.

A pushed, registered build-layer manifest for monomorphisation recovery: strictly worse information
than the pull-shaped symbol-table read, and cannot be written from inside a generic function at all.

Treating `f64::mul_add` as a source-expressible fidelity liberty: it lowers to `llvm.fma`, an exact
IEEE operation with one defined answer, not a permission, and pessimises on targets with no hardware
FMA.

Citing a shipped `Monotone` law implementation for `(TowardNegative, T, TowardPositive)`: does not
exist as any implementation, only as an unlocked design-round proposal, and is false against its own
admitted compositions.

Assuming the recovery-map classification's cheapness transfers automatically to a new operation:
refuted twice independently, the classification is a property of the pair (map, operation), not of
the map alone.

"Past the top is unreachable" once infinity is representable (file 27): false. Infinity changes the
over-range position's neighbour rather than removing the position; the midpoint that decides overflow
lives on the round-first amendment's unbounded grid.

The unsigned faithfulness blanket over every `Resolution` pair (the original spec derivation):
refuted by compiled counterexample, `SubstituteZero` breaks associativity where clamping and modular
reduction preserve it.

Classify-then-round as the quantiser's order (the spec's original order): disagrees with all three
test standards on the band past the largest representable but within half a quantum of it. Replaced by
round-first, classify-second.

Two round-trip theorems as the crossing contract (`decode ∘ encode = id`, `encode ∘ decode = id`, both
as identities): the second is false the moment signed zero, NaN payloads, or decimal cohorts exist.
Replaced by the section-retraction triple (one identity, one idempotence, one derived boolean).

A single three-instance `Sign` axis bundling range and zero-count: under-determines the set and mixes
a value fact with a datum fact. Split into `SignDomain` (identity) and `SignIndexing` (encoding).

Block floating point as evidence for nesting `Adjustment`/`Bias`/`Underflow`/`Specials`: the nesting
decision stands on the `Underflow` argument alone; BFP is a different kind of object (a composite
numeral over a shared external exponent) and is not evidence for either shape being cheaper to extend.

Referential uniqueness (never let a consumer name a derived numeral by literal) as an alternative to
value uniqueness: fails the ordinary case of storing a product in a declared numeral, and is an
invariant living in a signature-writing convention, the class this review keeps finding rots silently.

Projecting a trait-level width computation back down into an ordinary const parameter to dodge the
value-uniqueness obligation entirely: refused, the feature named (`generic_const_args`) is neither the
permitted nor the forbidden one and is unvetted.

The claim that the shipped width chain and integer adjustments already satisfy value-uniqueness (a
passing sentence in file 34): false for the width chain, `UInt<UTerm, B0>` is a second spelling of
zero and the adder propagates rather than normalises it.

"Two spellings of one condition" for the interior-safety and total-safety accumulator formulas (file
33's own reading): they are two distinct conditions serving two distinct promises, not one condition
written twice.

The ordered three-relation ladder (weak, then Kleene, then graded, file 34's own section 3.3): replaced
outright by file 37's nine-point view lattice, which is not a chain and contains two shipped presets at
incomparable points.

The reification-stability generalisation ("the graded relation is the only one stable under a
Refuse-to-special reification," file 34): true of one reifier and false in general. Corrected to a
hypothesis about the reifying element: stability requires the reifying element to lie outside the value
set and absorb the operation, and nothing is stable under `SubstituteZero`.

`Op::IS_EXACT` alone as the statement that an operation's grade monoid is trivial (file 37): false in
general, exactness alone kills quantiser-generated causes and events but not causes with no quantiser
origin. Corrected to the conjunction with `Total<Op>`.

A consumer-declared required view as the mechanism gating a regrouping's licence (file 37's first
attempt): killed by the compiler mid-dispatch, the licence check refused exactly the case it existed to
handle. Replaced by the transfer rule (a regrouping publishes what it fails to preserve; no waiver).

The subset-domain reading of the view parameter (file 37's own first draft): the family of holding
relations under a subset domain is not closed under meet, so a law's content is not one object.
Replaced by the quotient-of-the-grade reading, which closes under join.

`Bias` as a plain signed integer (file 36): makes a legal MATLAB numerictype unrepresentable and
contradicts the ratified biased-multiplication closure formula's own rational algebra. Corrected to a
signed, gcd-normalised rational, the piece the value-unique encoding is held pending (section 1.11).

Three separately-restated `Numeral` member lists across files 35, 36 and 38's citing of them, none
matching the ratified contract (one resurrects the derived `LogicalWidth`, one drops `Radix` entirely,
one both): none of the reviews's compiled results depended on any of them, and the repair is a
discipline (quote the governing statement by line, or derive the list from one, never both from
recall), not a new mechanism.

The vacuity worry against D39 ("bounding on `Real` gets you `Real`'s operations, and the bound is
therefore vacuous"): correct about the top of the tower and wrong as a verdict on the predicate itself,
which is "inhabits," not "equals," and discriminates from the bottom. Resolved by reading the finest
inhabited system rather than the ambient set.
