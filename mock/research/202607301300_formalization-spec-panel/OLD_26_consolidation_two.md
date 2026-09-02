# The current shape, second consolidation: the algebra dive

File 11 stated the shape after ten members reviewed the numeral/policy/lowering restructuring in depth.
Fifteen more members (files 12 through 25) then ran, on a second review the lead designer (op) opened
because the first review, though deep, had lost the wider taxonomy. File 12, deliberately barred from
reading the panel transcripts and given only file 11, found a missing axis and an entire absent area of
the design within one dispatch: mixed-operand arithmetic, the arithmetic-fidelity distinction between
`FastFloat` and `StrictFloat`, and the fact that nobody had traced the algebraic-law machinery into
arvo's own algorithm crates. That finding opened a sub-dive into where algebraic laws belong and what
they are, which occupied the rest of this stretch (files 13 through 25) and is not finished.

This document replaces file 11 as the sole reference for the design's current state. It stands alone: no
file in the panel directory is assumed read. Every claim that rests on a measurement states the
measurement inline. Where a panel member's finding was later shown wrong by a member who compiled
something, only the surviving position is stated here; the overturned ones are named, briefly, in the
droplist at the end, with enough of their reasoning that a later expert who wants to overturn the
droplist itself knows what has to be defeated.

Two things carry from file 11 unchanged. Only op's calls are final, and even those are understood to go
stale the moment something better surfaces. And where this document cites a downstream consumer's
existing code, that is evidence about arvo's own design gaps, never license to preserve current
consumer behaviour for its own sake.

## 1. The agreed shape

### 1.1 The three contracts and the one type

Unchanged from file 11. `UFixed`, `IFixed`, `FastFloat` and `StrictFloat` are four compositions of one
generic type, `Number<N: Numeral, S>` where `S: Policy + Lowering`, differentiated by where the exponent
lives (`Numeral`), what a result does when it does not land in the numeral (`Policy`), and what it costs
to hold and compute (`Lowering`, which by its own charter changes no answer). The sorting test: change an
axis and ask whether the representable set changed (`Numeral`), whether the same values are representable
but the arithmetic differs (`Policy`), or whether only cost changed (`Lowering`). The two-parameter fused
form (`Number<N, S>`, not a three-parameter split) stands; the split was tried, shown to cost roughly 1.8x
in rendered diagnostic length with no real typing benefit over the fused form, and abandoned.

### 1.2 The axes

Table unchanged from file 11 (`ExponentForm`, `Adjustment`, `Bias`, `Sign`, `LogicalWidth` under
`Numeral`; `Growth` under `Policy`, plus `Quantisation` described below; `StoredWidth`, `Widening`,
`Layout` under `Lowering`). `Underflow` nests inside `Stored` rather than standing as its own axis;
number-system membership is derived, not an axis.

`Quantisation` is the single map from an exact result onto the representable set, replacing separate
rounding and overflow axes. An exact result sits in one of five positions (strictly below the midpoint,
on it, strictly above, past the top, past the bottom); the first three take a `Direction`, the last two
take any `Resolution` (a direction, `ReduceModulo`, `SubstituteZero`, or `Refuse`). `ReduceModulo` at a
midpoint position fails to compile, since it is a `Resolution` but not a `Direction`. This vocabulary
reproduces every named rounding and overflow mode in IEEE 754, SystemC and MATLAB's Fixed-Point Designer
with no gaps needing their own name.

One structural gap, found by Lattner (file 12) and not yet resolved: `Growth::Exact` with
`Widening::None` has no implementable carrier (the exact intermediate has to live somewhere), so the axis
product contains unlowerable points with no compatibility predicate stated anywhere. Three of the four
presets sit on the line "`Widening` is derivable from `(Growth, StoredWidth)`"; `Precise` (exact growth,
doubled width, yet per-operation widening) is the one deviation, and whether it has a real reason or
whether `Widening` collapses to nine axes is open. The sharper structural point Lattner drew from this:
`Policy` may constrain `Lowering` (sufficiency), but `Lowering` must never inform `Policy` or the laws
(independence). That asymmetry is not yet written into the spec text.

### 1.3 The verification apparatus

Four pieces exist or are proposed, cumulative, each closing a hole the previous pass's compilation found.

**The recovery-map witness.** A recovery rule (`phi`) is one `[const]` generic function, instantiated
once at a small model width (checked exhaustively at compile time) and once at the composition's real
width (the literal code that runs), the same text monomorphised twice rather than authored twice. Three
prior passes each had a hole the next found by compiling: totality without truth, a checked classification
pointed at a private copy of `phi` the runtime pipeline never called, and the fix, making the checked and
executed functions the same text.

**Bounded exhaustive const checks at a model width.** The mechanism the witness runs on. Cost quadruples
per bit, measured at 28.45 seconds at eight bits, refused by `#[deny(long_running_const_eval)]` at nine.

**Structural classification of the recovery map**, the newest piece and the one this dive's own
measurement shows is the cheapest. A map `phi` is a **homomorphism** when it commutes with the operation
(`phi(x + y) ~= phi(phi(x) + phi(y))`), a **partial identity** when it returns its argument unchanged
wherever it returns at all, or a **retraction** (total, fixes the representable set pointwise, order
preserving). Each class implies a law by a three-line proof, checked at a single argument, not searched
over arity: a homomorphism gives Kleene associativity at every arity (wrapping addition, the sole
instance); a partial identity gives existential associativity at every arity (`Precise` addition); a
retraction gives neither in general (signed clamping). This classification is maximally robust along
every axis that only changes which values a fixed operation's `phi` sees (width, radix, arity,
accumulator, growth boundary, verified over all 256 subsets of an eight-value model crossed with arities
2 through 5, every case agreeing). It earns zero free transfer across a change of operation: the same
`phi` classified for addition is a different question for multiplication, confirmed independently by two
separate probes (a fixed-point multiply's shipped truncation is neither a homomorphism nor a partial
identity: rounding fires on roughly half of in-range operand pairs, against zero for addition).

**The fidelity-licence door**, section 1.6 below.

Two overlaps between pieces are named and unresolved: whether the classification's O(1) structural proof
should *replace* the exhaustive fold-level check for the cases it covers, or run alongside it on purpose
as belt-and-braces (both are defensible; claiming both benefits, cheaper and independently redundant, at
once is not available); and two independently-arrived-at closures for the fidelity coherence gap (a
door-only shape and an unconditional-blanket-plus-truth-marker shape) both work and neither has been
picked.

**The ledger, updated.** File 11's four-bin ledger (machine-checked by construction; machine-checked by
bounded exhaustion; trusted with nothing beneath; validated per artifact) needs two additions the dive
found missing entirely: that a composition's declared axes are honoured by the bodies that run under
them, and that any build layer acting on a licence acted only inside it. Both move from unstated to
"validated per artifact" to the extent the mechanisms in section 1.6 are built. A specific silent-break
mode was found and is now named as a standing risk: **a model can be inadequate in two structurally
different ways, and the apparatus only guards one.** Leroy's runtime panic on an unreachable refusal
catches the case where a model *undercounts refusals* (loud, by construction). Nothing catches the case
where a model is too narrow to see a *value* disagreement and quietly returns a wrong number with no
refusal at all; this was found once, by an accumulator sweep run out of curiosity rather than by any
standing rule (section 1.4). No mechanical fix for the second case exists yet.

A second general failure mode, found repeatedly and independent of any specific mechanism: **prose
claims about the design's own state are checked by nothing.** The strongest instance was a spec sentence
claiming a `Monotone` law implementation shipped; it does not exist as any implementation (only as an
unlocked, open design-round proposal), two subsequent members built on the false citation before a third
caught it with a single grep, and the proposal itself, once checked, is false for the compositions its
own premise admits (see the droplist). A check that never ran and a check that would have passed look
identical from outside; the only defence found is a member re-grepping a claim before building on it, and
this dive's own record is that even careful members skip this when the claim reads as background rather
than as the question they were sent to answer.

### 1.4 Algebraic laws: what they are and where they attach

This is the largest revision to file 11's shape. Four findings, cumulative.

**No arvo trait bound gates ordinary usage on an algebraic law by default.** `arvo-graph`'s ranking and
longest-path routines are max-plus recurrences (addition applied once per node, grouping pinned by the
graph, `max` doing the actual reduction); `arvo-comb`'s DP is the min-plus dual. Neither folds, so
neither needs associativity by default. Measured exhaustively over 64 DAGs and 625 weight vectors (and
independently over the min-plus dual): wrapping addition (`Hot`) is associative but **fails**
distributivity over max/min; saturating addition (`Warm`/`Cold`) is **not** associative but **satisfies**
distributivity. A gate on associativity would admit the one preset under which `longest_path` returns a
value that is not the longest path under any grouping, and refuse the two under which it works. A law is
required only by the specific combinator that performs a regrouping (a chunked parallel fold, an n-way
unrolled accumulator, a future tropical matrix power), and that combinator states the fact it needs; code
that never regroups states nothing and refuses nothing. Signed saturating addition's classification is a
**retraction** (clamp), not a homomorphism, which is exactly why it distributes over max/min while
failing associativity: a retraction preserves order but not the group operation.

**Where regrouping happens, it happens inside arvo's own licensed internals, not only in a future
scheduler.** A committed bench (`hilavitkutin/mock/benches/fold_strategy_n1024_findings.md`) already
measured that splitting one accumulator into four beats one accumulator by roughly 2x on a single thread
(110ns against 55ns at N=1024), from instruction-level parallelism, before any thread exists. That
regrouping is exactly the transformation `arvo-always-optimal-internals.md` already licenses arvo to
perform inside its own bodies without asking anyone. Op ruled directly that the laws stay in arvo (not
relocated to a downstream engine, since a consumer wanting the algebraic vocabulary should not have to
pull in a whole pipeline execution engine to get it), in a place still to be designed within arvo.

**A law's key must include every parameter the underlying proof actually used, and two were silently
defaulted rather than omitted.** The operation and the accumulator both belong in a fold-law's key.
"Only `Hot` folds for signed values" (file 11's stated consequence) is not a fact about `Hot` and `Warm`;
it is a fact about a `(numeral, accumulator)` pair in which the accumulator was silently taken to be the
numeral itself. Measured: signed saturating addition's regrouping diameter drops from 7 to full Kleene
associativity (diameter 0) with **no axis changed**, purely by widening the accumulator. The mechanism:
once the accumulator is wide enough that no interior node of any grouping can leave the numeral's own
range, `phi` applies at most once per grouping, at the root, to the exact sum, and a map applied once to
a grouping-independent argument cannot depend on the grouping. This threshold, **interior safety**, is a
closed form in the arity (`accumulator width >= product-or-sum width + ceil(log2(n-1))` in the additive
case, section 1.5 for the multiplicative case), not something searched over. It resolves cleanly into
one design rule, held as the current best reading: **the accumulator is a side condition on the law,
stated as a closed form, deciding whether the recovery map is even in the key.** Below the threshold the
map's structural class is the whole answer; at or above it, the recovery map's own properties become
irrelevant and only the accumulator matters. The alternative readings (accumulator as an eleventh
`Policy` axis; accumulator as an ordinary combinator parameter with no side-condition reasoning) are both
held as live but weaker.

The operation belongs in the key for a parallel, measured reason: the same structural classification is
a property of the pair `(phi, Op)`, not of `phi` alone. `Precise` addition is a partial identity (the sum
of two multiples of the quantum is a multiple of the quantum, so in-range rounding never fires: measured
0 of 256 pairs). `Precise` multiplication is not (a product carries `2F` fractional bits, generically not
a multiple of the quantum: in-range rounding fires on roughly half of pairs, and existential associativity
fails outright from arity 3, with a witnessed counterexample). Nothing about how carefully the addition
case was checked transfers to multiplication; the transfer has to be redone from the operation side.

**A derived fact is best expressed as a `const fn` whose parameters are its key, with no separate key
annotation to keep in sync.** Omitting a parameter the proof depends on is then caught for free by
ordinary scope resolution (`E0425: cannot find value in this scope`) at the point the omitted parameter
would have been used; the completeness check costs nothing beyond writing the proof as a term rather than
as prose. This closes only the "prose forgot a parameter" failure mode, not the separate "a check exists
somewhere but nothing routes execution through it" failure mode (an unwired check), which needs a
different fix (the door, in section 1.6). Both failure modes were independently demonstrated and both
need to be guarded; conflating them into one mechanism was tried and does not work. A named consequence
of the const-fn-is-signature shape: because a bare parameter list, not a struct, is what carries the key,
Rust does not force an unused parameter to be named (unlike a struct field), so completeness in the
"carries too much" direction is a discipline rather than a mechanism, and the design should err toward
key inflation deliberately (over-strictness is sound and costly; under-keying is unsound and invisible in
the artifact that has it).

**Named structures should be derived, not primary.** The draft's own instinct ("a structure is a magma
plus the laws it happens to satisfy") had not been carried past `Magma` and `AddAssoc`, and the original
per-`Resolution` blanket-impl shape for `AddAssoc` hit a real coherence ceiling (a third true fact needing
its own impl makes three mutually non-specific impls, refused by Rust's coherence checker, with no escape
under this workspace's permitted features). The fix, confirmed sound and cheap by three independent
members: treat associativity, commutativity, has-identity, idempotence, distributes-over, and monotone
each as its own atomic, independently-derived marker fact (a `[const]` boolean fold or a zero-item
marker trait, per the existing `Resolution`-lemma-fold mechanism), with named structures (`Monoid`,
`Semiring`, `Dioid`) as derived blanket impls over conjunctions of atoms. Marker-trait conjunctions do
not collide under coherence the way the original per-`Resolution` impls did, because `T: Associative<Op>
+ Commutative<Op>` being simultaneously true is a conjunction, not competing evidence with a winner to
pick. The two currently-differently-scoped uses of the word `Monotone` (unary quantisation-function
monotonicity, and order-preservation of a binary operation's partial application) are different arities
of a different concept and should stay two separate traits, not one.

**One graded reading of the whole apparatus, offered as the strongest available unifying account, held
as one reading among several.** Effect-shaped facts (fallibility: total or partial) accumulate along a
term as a free commutative monoid over refusal causes and quantisation events, with a total-to-fallible
coercion that the type system checks for free (an incomplete implementation fails to compile,
`error[E0004]: non-exhaustive patterns`). Coeffect-shaped facts (fidelity: what liberties a body may
take) are a **bounded meet-semilattice with a downward-closed coercion**, not a semiring proper (neither
addition-for-contraction nor multiplication-for-nesting is exercised), and its coercion carries no data,
so a corrupted grant compiles clean with zero diagnostic; this asymmetry (data-shaped coercions are
checked by the type system for free, permission-shaped coercions are not) is the single sentence this
reading would put in the spec. Under this reading associativity itself splits into two conditions that
the draft's Kleene-equality fusion checks as one: does the *grade* (the set of refusal causes) agree
across groupings, and does the *value* agree given it does. They come apart for `Precise`: measured
diameter 0 at every fold length (every grouping that returns, returns the same number), against 10992
grouping-dependent *refusals* out of 32768 inputs at a five-element fold. So `Precise`'s regrouping
sensitivity is entirely a definedness phenomenon with zero numeric spread, which the current fused
Kleene-equality account puts in the same failed-law column as signed clamping's genuine numeric
divergence. Whether the design should therefore report two separate facts (value-agreement,
grade-invariance) instead of one fused fact is held open (section 3); neither reading resolves the other
and both are defensible.

### 1.5 The multiplicative half

Untested at file 11's time, now substantially built and measured as a coherent design, treated by this
dive as its most converged result.

**Multiplication is exact; rounding lives entirely in an explicit narrowing step, never in the product
itself.** The product of a `Q(I1,F1)` value and a `Q(I2,F2)` value is, bit for bit, a `Q(I1+I2, F1+F2)`
value with nothing dropped: widths add, quanta multiply, exactly as `Growth::Exact`'s own stated sentence
already says. This is the multiplicative half's headline finding: what earlier files called
"multiplication always quantises" is true of the composite operation "multiply, then force back into the
operand's own numeral", never of multiplication itself. Relocating narrowing out of the product gives
`mul_full` (widening, total, exact, commutative, associative, distributing over exact addition, at every
strategy, with laws free because no quantiser is present to break anything) and `quantize` (a named map
between numerals carrying the whole `Quantisation` apparatus, fired only at explicit sites: a store, a
narrowing, the tail of a convenience wrapper). SystemC's per-assignment quantisation and MATLAB's
per-operation `ProductMode`/`SumMode` policy, which the design's own open firing-site question
(per-operation against deferred) had treated as a fork needing a choice, both fall out as different call
patterns of the same shared `mul_full`/`quantize` pair rather than as two implementations of narrowing
that could silently drift apart from each other; this was verified directly (two convenience wrappers
with character-for-character identical bodies, differing only in where the caller places the `quantize`
call).

**The mechanism.** The literal proposal to compute widths via `<W1 as AddWidth<W2>>::Out` as a const
generic under `min_generic_const_args` was tried and is dead, structurally: that feature's `type const`
item explicitly forbids arithmetic over a still-generic const parameter on its own right-hand side
(`error: generic parameters may not be used in const operations`, refused at the definition site, the
identical wall the workspace's capacity-as-a-type migration already hit for the same reason). The working
replacement needs no unstable feature at all: widths become distinct zero-sized **types** (a binary digit
chain, the technique the `typenum` crate has shipped on stable Rust for a decade), and width addition is
an ordinary ripple-carry adder built from trait dispatch, resolved at monomorphisation. Verified: exhaustive
at a small model (every 2-bit sum, 16 pairs, checked as compile-time assertions), and at realistic scale
(13+7=20, 3+2=5, the exact widths a `UFixed<13,3>` times `UFixed<7,2>` product needs). Products of
products compose correctly with no special-casing (`Q(2,2) x Q(2,2) x Q(2,2)` widens twice, matching
`13*6*4`). Measured cost: a concrete `mul_full` instantiation disassembles to exactly four instructions
(a standard widening-multiply sequence), with zero symbols or code referencing the phantom width types
anywhere in the generated output. The consumer-facing literal-width sugar (`UFixed<13, 3>` spelled with
plain integer literals rather than a binary type tree) needs a bounded, macro-generated dispatch table
from literal to type (the same table-versus-projection shape and cost tradeoff already measured for
container selection in file 11's section 3.9; the projection form should win for the same reason).

**The load-bearing multiplicative shape is the multiply-accumulate (MAC), not a chained product, and its
interior-exactness bound is logarithmic, not linear.** A naive chained product's interior-exactness bound
grows linearly in both integer and fractional bits with the fold length (`(n-1)*F` fractional bits
measured directly). But real fixed-point signal-processing workloads (dot products, FIR/IIR filters,
correlations, arvo's own one real fold in `arvo-spectral`) are bilinear MACs: products at depth one,
addition above them. A MAC's interior-exactness bound is the product width plus addition's own
logarithmic term, `acc >= product_width + ceil(log2 n)`, matching real DSP silicon (the Motorola 56000's
eight guard bits above a 48-bit product are exactly `ceil(log2 256)`, sized for 256 MAC steps). Measured
directly: a per-operation-quantised MAC's grouping diameter reaches 15 raw units on a 16-value range by a
five-element fold (the answer set spans nearly the whole representable set), while the wide-accumulator
MAC's diameter is 0 at every arity by construction, because its interior is exact integer addition
(associative), and the only approximation anywhere is the single quantisation at the store. The lawful
form is measurably the fast form: a per-element quantise-and-clamp is a serial dependency that defeats
vectorisation, while the wide-accumulate shape is exactly what a hardware MAC instruction wants.

**Sizing the accumulator is a checked bound, not a derived type**, the same discipline Thread C already
uses for leaf facts, applied to a numeric-sufficiency claim: `const { assert_accumulator_sufficient::<N>
(PI::VALUE, AccI::VALUE) }` inside the fold, refusing to compile when the accumulator is too narrow (a
3-guard-bit accumulator against a 256-step MAC refuses; an 8-guard-bit one compiles and runs correctly),
with the check itself provably free at runtime (disassembly shows a plain accumulation loop, zero trace
of the check anywhere in generated code). This dissolves the loop and fold questions together: neither
needs the accumulator's type derived from a runtime trip count (which cannot work: a type cannot depend
on a value only known at runtime), both need the accumulator's sufficiency checked against a compile-time
arity or unroll factor.

**Rounding bias is a real system property, not a naming nicety, and the shipped `>> FRAC` truncation is
one specific filter among several with different bias structure.** On a two's-complement value, an
arithmetic right shift rounds toward negative infinity (a constant-offset bias, drifting linearly with
sustained accumulation: measured roughly `-K/2` quanta after `K` operations, the textbook DC ramp,
manifesting as limit cycles in a feedback loop), which is a different filter from round-toward-zero (a
sign-dependent, symmetric bias that is actually stabilising in a feedback loop). The preset table's word
"truncate" for `Hot` is ambiguous between the two and should name which one is meant; the shipped body
answers `TowardNegative`. Round-to-nearest-even is zero-mean and random-walks (`~q*sqrt(K/12)`, no ramp),
confirming it as the right default for `Warm`/`Cold`/`Precise`. First-order error feedback (carrying the
previous quantisation residual forward) bounds total error within one quantum forever, but needs state, so
it belongs in a combinator's accumulator object rather than in the type-level policy, for the identical
reason stochastic rounding is already excluded from the design (resolution constructors are pure ZSTs).
**Double rounding is real under `Growth::Narrowed`**: measured, a nearest-then-nearest two-step narrowing
disagrees with a single nearest rounding on 12 to 19 percent of inputs. Round-to-odd at the intermediate
step does restore exact agreement with single rounding, but only above a precondition (the intermediate
must carry at least two more fractional bits than the destination, `W >= F + 2`); below that precondition
round-to-odd is measurably *worse* than the naive two-step rounding it was meant to fix, and this validity
range must ship as a checked const bound alongside the credit given to it.

**A real multiplicative law survives, and it inverts across presets the same way the additive one does.**
Scaling by a nonnegative constant is monotone under any monotone rounding rule composed with clamp
(a composition of monotone maps), and a monotone map commutes with max/min on a chain, so quantised
scaling distributes exactly over max/min under clamp-based presets. Under wrap it fails entirely, by the
same torsion-group argument that already rules out monotonicity for wrapping addition. So the presets an
associativity-style gate would admit for multiplication (`Hot`) are exactly the ones that fail this
ordered-structure law, and the ones it would refuse (`Warm`/`Cold`/`Precise`) are exactly the ones that
hold it, mirroring the additive finding precisely and directly relevant to `arvo-graph`/`arvo-comb`'s
weight-scaling operations.

**Division is defined as the correctly-rounded quotient**, one quantisation, no intermediate, matching
IEEE 754's own definition and the only coherent meaning `Exact` growth can have for an operation whose
exact result is generally not representable at any finite width. Its laws are half-ulp bounds only, no
associativity claim of any kind. Reciprocal-multiply (`a * b.recip()`) is a distinct, licensed liberty
from true division (two roundings against one), currently taken **unconditionally**, with no gate, inside
`arvo-spectral`'s power-iteration and Fiedler-vector routines. The shipped divide-by-zero behaviour
(silently substituting the numerator) is a real defect: a refusal cause with no home in the vocabulary,
papered over with a fabricated total answer.

**A closure gap is open and unresolved**: the exact-product construction is stated for `Unit` adjustment
and `Zero` bias. It does not extend cleanly; a `FullRange<F>` value's square carries an adjustment not
itself of `FullRange` form, and a biased numeral's product generates cross terms, so neither `Adjustment`
nor `Bias` is closed under multiplication the way `AddClosed` already gates addition on `Bias = Zero`.
Whether the fix is a general rational-adjustment constructor or a `MulClosed` condition with explicit
renormalisation (matching how UNORM multiplication in graphics practice always renormalises) is undecided.

### 1.6 The downstream contract

Op's standing obligation from this stretch: every place the design deliberately stops short because the
mechanism belongs to a build layer, a code generator, or another repository owes a concrete design of
that boundary, never merely an observation that it exists. This section states what the dive built
against that obligation. arvo grows no build harness of its own anywhere in what follows; every mechanism
below is either fully internal to arvo, or is read-only from outside arvo with nothing pushed and nothing
registered.

**A cross-contract delegability rule, derived from the contracts' own charters and not yet in the spec
text.** A build layer may read every axis of every instantiation. It may act freely on `Lowering` axes,
because by their own definition acting there changes no answer. It may act on a `Policy` axis only by a
transformation whose every result lies inside the envelope that axis's own instance declared (never by
substituting a different body). It may never act on `Numeral`. This subsumes an earlier, too-broad
prohibition against a build layer selecting among arvo's variants "after type checking has run" (which,
read literally, would also forbid the ordinary optimiser, which rewrites checked text constantly and
which nobody proposes to forbid); the real distinction is substitution against transformation, not before
against after.

**Whether a proposed fidelity axis is `Policy` or `Lowering` is unresolved and load-bearing for the whole
delegability question**, held as reasoning rather than as a ratified classification. Read through the
design's own sorting test (same representable set, differing arithmetic answer), fidelity sorts as
`Policy`: measured value changes are total losses of the result (a four-element sum reads `2` under one
liberty set and `0` under another), not rounding-level noise. The counter-reading, held honestly and not
taken here: if `Lowering`'s "changes no answer" is read as "changes no *mathematical* answer" rather than
literally, a reassociated float sum and a strict one could be treated as the same number computed at two
costs, which is what a numerical analyst thinking in error bounds would say, and it would make the whole
question delegable for free. Neither reading has been adopted.

**Monomorphisation prints the composition into the compiled symbol; it does not erase it, but the
channel this creates is empty in a real shipping build and only recoverable in a dedicated check build.**
Measured: with a v0-mangled symbol and no `#[inline]`, every generic argument of every instantiation is
in the symbol table exactly, by value and by marker-type name (a real reader was built and demonstrated
recovering axis instances from `llvm-nm` output with no source, no build-script cooperation and no arvo
change). But on an ordinary optimised build with no artificial `#[inline(never)]`, the channel is not
merely narrow, it is **empty**: zero of twenty-four measured monomorphisations were nameable, with or
without fat LTO, because two separate inliners run (rustc's own MIR inliner, then LLVM's), and defeating
either one alone is not enough. The channel reopens completely under a **check build**
(`-Cno-prepopulate-passes -Zinline-mir=no --emit=llvm-ir`), which is not the shipping artifact, is
measurably *faster* than a release build (no optimisation pipeline runs), costs disk and nothing else, and
was verified byte-for-byte reproducible across cold builds once the release-target build itself is not
independently broken (see the shipped-tree finding below). So the corrected statement of file 20's own
constraint: the intent is legible in whatever build a reader asks for it in, at the cost of two flags on
a build nobody was going to ship.

**A working post-monomorphisation verifier exists as a sketch (68 lines) and closes a gap an earlier
member believed could not be closed internally at all.** It catches both an under-claim (a body regroups
under a licence its own declared grade does not grant) and an over-claim (a licence grants a liberty no
body in the composition ever exercises), against a real check build, in about 1.5 seconds over 95MB of
arvo's own IR. Three corrections were needed before it worked cleanly, each found by running it rather
than by design: the marker scoping must be structural (which crate defined the generic argument), not a
maintained name list, or it drowns in false positives from unrelated proc-macro dependencies; the
over-claim direction must be checked per **composition** (union over every operation instantiated for
it), not per operation, or every operation that legitimately does not exercise a given liberty reports a
false positive; and a rule must match the callee **name** rather than only the emitted instruction,
because a stable function like `f64::mul_add` stays an un-inlined call under the flags that keep the
composition itself visible, with the licensed instruction one level further down in a function carrying
no axis information at all.

**A cheaper, semantics-free check catches most of the same ground with no rules and no vocabulary at
all**: for each axis, does varying it alone, with everything else fixed, ever change one single
instruction anywhere in the program. It caught two of three planted defects with zero maintenance cost,
and missed exactly the case where a liberty was granted to the wrong instance (the axis still generated
code, just the wrong code); only a rule that knows what a liberty actually is catches that one.

**The cheapest and most durable form is one assertion, in the ordinary test suite, with no tool at all.**
`assert_ne!(op::<A> as usize, op::<B> as usize)`: if two monomorphisations compile to identical code, the
compiler folds them to one address, so address inequality *is* the statement that an axis generated
something. Measured correct from `-Copt-level=2` upward, including under fat LTO, never reporting a live
axis as inert at any level tested (the unsafe direction is fully absent); below level 2 it is safe but
useless (reports every axis as live). The cost of a new axis under this mechanism is one more assertion,
written by whoever adds the axis, in a release-profile test. A cruder variant, comparing the first bytes
of two functions' machine code, was tried and found unusable in both directions at low optimisation
levels; the function address is the better observation precisely because the compiler computed it rather
than the check computing it.

**A working precedent for the "declaration matches type, says nothing about a body" layer already ships
in arvo today**, found by grep and worth keeping as the pattern for size/alignment-shaped `Lowering`
claims specifically: 73 compile-time layout assertions (`arvo-storage/src/layout_assertions.rs`) already
pin each preset's declared axis instances against its actual const discriminant. These are honest,
correct, and structurally the same shape whose limit the whole downstream-contract dive is about: they
relate two **declared** things to each other and say nothing about whether any body honours either one.
`Lowering` claims about size or alignment should keep extending this pattern (cheap, already working);
`Lowering` and `Policy` claims about generated **code shape** need the check-build machinery above,
because no const assertion can see inside a compiled body.

**A build layer that rewrites emitted code under a licence must emit a receipt at the moment it acts**,
naming the symbol it edited, the transformation applied, and the axis instance read as the licence,
checkable offline against a check build. This is not optional: an LLVM pass consuming a licence must run
before the inliner (to reach the vectoriser), and after the inliner runs the mapping from an edited
instruction back to the composition that licensed it does not exist in any artifact and cannot be
reconstructed by anyone, including the pass itself. Whether the receipt requirement is ever paid depends
on an open fork: if fidelity ends up **function-shaped** (a licensed liberty names a specific,
deterministic algorithm arvo writes itself, e.g. a blocked reduction or a fused-multiply-add call), no
pass, no rewriting and no receipt is ever needed, because arvo owns the operation and never needs a
backend's cooperation. If fidelity ends up **envelope-shaped** (a liberty is a permission that any answer
within it is acceptable, C's fast-math model), the receipt is mandatory. Measured against the toolchain
directly: three of four candidate float liberties (reassociation, sign-zero cancellation, reciprocal
approximation) are fully source-expressible with no unstable feature and no build cooperation at all.
The fourth, contraction, is not what it first appeared to be: `f64::mul_add`, believed to lower to
`llvm.fmuladd` (a genuine licence, either answer acceptable), actually lowers to `llvm.fma` (an exact
IEEE operation with one defined answer, not a permission at all), is unavailable under `#![no_std]`
without either an unvetted feature or the already-forbidden `core_intrinsics`, and on a target with no
hardware FMA unit it compiles to a **pessimising** libm call rather than a free choice, the opposite of
what a licence should ever cost. The corrected reading: `Fused` belongs in the design as a distinct named
**operation** (exact, one answer, IEEE-defined, a `Lowering`-shaped cost fact), and `Contract` (the actual
permission, "either answer is acceptable") is the one real residue that genuinely cannot be expressed from
portable `no_std` source today, and needs either the receipt-and-pass machinery or the unvetted feature
path.

**A second, previously unmeasured residue exists with no channel to close it at all.** Saturating integer
reductions do not vectorise (`Precise`'s own preset selection), correctly, because saturating addition is
non-associative; unlike the float case, source-level regrouping does **not** recover parallel lanes (four
scalar saturating adds instead of one, regardless of how the source is written), and there is no LLVM IR
flag for integer saturating arithmetic to grant in the first place. The only route to lane parallelism
here is arvo hand-writing per-architecture vector kernels itself (the hardware instructions exist:
`uqadd` on aarch64, `paddus*` on x86), which is a real, currently unpriced cost that lands on arvo rather
than on any build layer.

**Multi-limb (`WideBits`) arithmetic has no residue today, for a fragile reason worth stating rather than
relying on silently.** A 256-bit carry chain already compiles cleanly because LLVM recognises the
`carrying_add` idiom and there is nothing to vectorise in a serial carry chain; but `core::arch::aarch64`
has no carry-propagating intrinsic to fall back to if that idiom recognition ever regresses under a
toolchain bump, unlike x86_64 which has one. This is a dependency on an optimiser heuristic holding, not
a guarantee, and it costs one codegen test to make falsifiable.

**Generic algorithm crates sit behind a second, real erasure boundary that neither the LLVM boundary nor
the fact-key boundary analyses cover, and this dive names it "Stage G."** arvo's own architecture rule
(stated independently of this dive) forbids `arvo-graph`, `arvo-comb`, `arvo-spectral` and `arvo-sparse`
from importing `Number`, `UFixed` or `IFixed` directly, or depending on the facade at all; every public
entry point in those crates is generic over a bare type parameter bounded on arithmetic-operation traits.
At the point such a body is type-checked, a concrete `S` genuinely does not exist as far as the body is
concerned (confirmed by compiling: asking for the name inside the body fails with `E0425`, "cannot find
type", not a permission error). Unlike LLVM's erasure, this boundary is not structurally necessary; it is
a deliberate workspace choice, made for good reasons (reusable algorithm code, no coupling to arvo's own
concrete vocabulary), and is therefore negotiable from arvo's own side at a real, priced cost. A live,
concrete instance of the resulting gap: `arvo-spectral`'s power-iteration and Fiedler-vector routines take
the reciprocal-approximation liberty unconditionally, because the `Recip` bound they carry has no `S` to
read and no way to be told a fidelity licence exists at all.

Three ways to cross Stage G exist, and they are complementary rather than competing, each compiled and
verified:

1. **Enrich the bound.** A zero-item marker trait, parameterised by the operation and any const-generic
   key dimension the caller already knows locally (e.g. an unroll arity), blanket-implemented once per
   composition satisfying the derived fact. Gives a friendly, propagatable diagnostic
   (`error[E0277]: the trait bound ... is not satisfied`, naming both the failing composition and which
   compositions would have worked) that the `const`-fn-key mechanism in section 1.4 explicitly cannot
   offer on its own. Best when the fact's key is small, stable and worth a nice error, which covers most
   of the algebra ladder.
2. **Make the composition reachable.** A small trait projecting a composition's own numeral and policy
   back out through the bound that erased them (`trait Numeric { type N: Numeral; type S: ...; }`, one
   blanket impl), letting the generic body call any `const`-fn-keyed fact directly on `F::N`/`F::S`. No
   unstable feature, no dependent typing risk (the projection is resolved by name, never by a runtime
   value). Trades the friendly diagnostic away (a panic at the use site rather than a refusal at the
   signature) for the ability to ask a question the bound was not pre-authorised for.
3. **Keep the generic core fact-blind by design, and license at a new, shallower entry point closer to
   where the composition is still concrete.** The deeply generic body stays exactly as it is today,
   correct for every numeral shape a consumer could supply, and a new function
   (`power_iteration_fast::<N, S>`) sits above it, checks whichever fact licenses a liberty, and either
   delegates unmodified or dispatches to a body written to take the liberty deliberately. Costs the
   existing public API nothing and requires no change to any of the four generic entry points that exist
   today. This is the option the dive would build first, because it is strictly additive.

A predicted, untested asymmetry in how far the ten-axis system can be extended from outside arvo: whether
a downstream crate can grant an *existing* arvo operation a fact about a *new*, foreign numeral it defines
itself was not tested (only the reverse, a foreign operation gaining facts about arvo's own numerals,
which the orphan rule permits in both directions tried). The untested direction nests the local type
inside a foreign generic constructor rather than at the impl's own top level, which usually defeats the
orphan rule without a `#[fundamental]` flag arvo does not control, so it is predicted, weakly, to fail
where the tested direction succeeded.

**A crate-dependency decision this all rests on is undecided.** The algebra ladder's atomic facts need a
home (the existing crate table already reserves one, `arvo-algebra-contracts`), and enriching the bound
(move 1 above) or projecting the composition (move 2) both require `arvo-graph`, `arvo-spectral`,
`arvo-comb` and `arvo-sparse` to gain a new dependency edge onto that crate (or onto wherever `Numeral`/
`Policy` end up) that today's forbidden-imports lints do not name because the crate does not exist yet.
Nobody has made this call; it is a layering decision, not a fact derivable from the mathematics.

### 1.7 Crates

The six-crate split from file 11 (`arvo-numeral`, `arvo-policy`, `arvo-lowering`, `arvo-strategy`,
`arvo-numeric`, `arvo-algebra-contracts`) stands as **packaging**, not as a load-bearing correctness
mechanism: the strongest architectural argument for the split (that a crate boundary alone makes a law
provably unable to read `Lowering`) was tested directly in file 11's own dive and failed, since the one
crate that must own the physically real `Number` type is, by construction, a crate where `Lowering` has
methods a where-clause can name. The real closure mechanism is the phantom-type proof described in file
11 (a law proven about a purely phantom carrier with no bound on `Lowering` at all), independent of any
crate split. This dive adds one unresolved packaging question on top: whether `arvo-graph`,
`arvo-spectral`, `arvo-comb` and `arvo-sparse` gain a dependency edge onto the algebra-contracts crate
(section 1.6). File 11's other open packaging questions (where `Width`/`Exponent` and the container
projection live, whether `Bits<N, S, Sign>`'s `S` should re-bound to `Lowering` alone) are untouched by
this dive and remain exactly as open as file 11 left them.

## 2. The lead designer's calls and the established principles

Op made four calls in this stretch and issued four posture corrections. All eight govern every expert
who reads this document, and none is optional context.

**The standard everything is measured against, stated once and not reopened per question**: optimal and
ideal, never merely adequate or conventional or the smallest change from what ships; representative of
the actual mathematics, not a structure adjacent to it that happens to be easier to encode; and capable
of representing MATLAB, IEEE 754 and SystemC not as inspirations but as a **test**, where an abstraction
that cannot express one of them is a defect rather than an accepted scope boundary. The consequence op
draws from this directly: the abstractions and the typestate are what matter, not the packaging, the
crate graph, or which preset carries which value. A member facing a choice between a cleaner abstraction
and a cheaper arrangement resolves toward the abstraction and reports the cost rather than trading the
abstraction away. No member consults op's preference on a question of this kind; it has already been
answered.

**The algebraic laws stay in arvo**, in a place still to be designed within arvo, never relocated to
hilavitkutin or any other downstream repository. A downstream consumer should not have to pull in a whole
pipeline execution engine to get arvo's own algebraic vocabulary, and this reasoning was independently
reinforced (not merely asserted) once a committed bench showed the regrouping that would have motivated
the relocation already happens inside arvo's own internals, on one thread, before any scheduler exists.

**The existing code is irrelevant, and every member must assume the design is being rewritten wholesale.**
The current shape being broken is the premise of the whole exercise, not a finding available within it;
demonstrating it again, however rigorously, spends a dispatch on something already agreed. Reading source
stays legitimate for exactly two purposes: checking a factual claim in a brief before reasoning from it
(which has repeatedly caught real false premises in this review), and understanding a mechanism well
enough to design its replacement. Auditing the implementation as though it were the subject is not.

**Every boundary the design deliberately stops short of owes a concrete downstream design, never an
observation that the boundary exists.** For every piece of the design a member touches: how it works for
a downstream target doing the lowering, concretely, what that target reads and can determine and what it
does with it; and what arvo needs back from that target where it cannot express something on its own
side. This is written down, designed for, never handwaved and never left implicit for a later reader to
notice is missing. What is explicitly not wanted: faulting arvo or the design for being structurally
unable to express something a build harness would be required to close, when growing that harness inside
arvo would be painful to maintain, hard to keep compatible with the build layers that already exist, and
inconvenient for every downstream consumer to adopt. What is very much wanted: a genuinely new answer to
one of these boundaries that nobody has thought of yet, developed rather than merely mentioned.

**Where the current shape can be kept at no real cost, it should be kept.** The best design sacrifices
none of the genuine improvements and costs the least additional rewrite against the codebase as it
stands. This is the explicit counterweight to "assume it is all being rewritten": that instruction is a
license to stop *defending* the existing code, never an instruction to *discard* working shape for a
marginally better one or for symmetry alone. Rewrite cost is real and is the tiebreaker between two
designs that are otherwise equal against the intent.

**The intent outranks every instruction in every checkpoint, including these ones, and it is vague on
purpose.** The existing code, however inadequate, was itself an attempt at the same intent the redesign
is still reaching for; the version of the intent that governs is the one as reshaped and reworded through
the actual design talks, wherever op's calls have moved it, never a literal reading of any single
sentence. Because the intent cannot be read literally, only inferred and evaluated against, this is
explicitly subjective work in the precise sense that two competent readers can infer differently from the
same intent and both be reasoning honestly. A member who finds themselves certain what the intent
requires should read that certainty as a signal to go find the reading they have discarded. No member
resolves to one angle on anything substantive; every member carries more than one reading where more than
one exists and states what distinguishes them, leaving the choice where it belongs.

**Every member owes a constructive deliverable, not only adversarial findings.** Correctly testing and
breaking a claim is genuinely valuable in this review and has overturned real findings repeatedly,
including several a member found broken in their own freshly-built work by compiling it, but it is the
floor, not the whole of the work. Where something fails, the member states what should replace it, in
enough detail that someone could build from it. Where a proposal holds, the member takes it further than
its own author did rather than stopping at confirmation. Where an unnamed gap is found, the member
proposes a shape that fills it. A low-confidence proposal, marked as such, beats a finding offered alone.
The files that moved this design furthest in this stretch each built something (the exact-product
relocation that made the multiplicative half tractable; a working 68-line verifier catching a defect an
earlier member believed uncatchable; three concrete mechanisms for crossing the Stage G boundary; a
graded reading that deleted a mechanism the design believed it needed); the least valuable contributions
were correct catalogues of defects in code nobody was proposing to keep.

## 3. What is open

This stretch is not finished, by op's own framing of how the review runs: a deep dive, a consolidation, a
fresh outside read, and repeat until the design is concrete, valid and ideal, with nothing less accepted
as a stopping condition. Measured against that bar, this stretch produced real, converged, load-bearing
machinery (the multiplicative half above all, and a working start on the downstream contract), but the
algebraic-structure domain that the whole sub-dive was opened to settle is not settled, and several
threads were opened and explicitly left for the next round rather than closed. What follows is not a
polished residue; it is the honest state of a dive still in progress.

**Which relation a fold-law should be stated under is undecided, and it decides how `Precise` reads.**
No single relation (Kleene equality, existential equality, the refinement order) separates all four
measured resolutions; the design currently reports one fused verdict (Kleene) where the mathematics
supports at least two independent facts (value-agreement, definedness-agreement). One reading says
`Precise`'s zero numeric spread deserves its own, weaker law name (partial associativity, associative
on its own domain of definedness) distinct from signed clamping's genuine numeric divergence. The
opposing reading, held with equal weight, says a fold whose *definedness* depends on grouping is unusable
regardless of what the values do, so fusing the two facts under one relation is correct and `Precise`
belongs exactly where it currently sits. This is stated as a question about what `Precise` is *for*, and
it is explicitly not a call any member has made.

**Whether the accumulator is a side condition (a closed-form threshold, the current best reading), a
free combinator parameter, or an eleventh `Policy` axis is undecided.** All three readings are internally
consistent; they are not compatible with each other, and the choice interacts with `arvo-num-systems` and
with the `Growth` axis in ways nobody has traced through.

**Whether a law attaches to a type (a magma, the noun the draft already declares and never actually uses)
or to nothing (an edge of a rewrite system with no carrying object) is undecided**, and the two readings
happen to agree on the mechanism (a scoped `const` fn is the key either way) while disagreeing on the
vocabulary, which is corroboration of the mechanism and not of the noun.

**The fidelity axis is proposed, extensively tested, repeatedly corrected, and not adopted.** It is not
in the ratified ten-axis table. Whether it should be a binary axis (`Strict`/`Relaxed`) or an axis with
more instances (`Exact`, `Fused`, `Blocked<N>`, `Envelope<L>`, separating "a deterministic algorithm
choice" from "a genuine permission" rather than conflating them under one axis) is undecided and was
found, on measurement, to be exactly the distinction three earlier members had been silently collapsing.
Whether fidelity is `Policy` or `Lowering` decides the whole build-layer delegability question and is
itself undecided (section 1.6). The fidelity-licence witness mechanism (the "door": recompute the
coercion relation inline, at every consumption site, rather than declare and trust it) took three
attempts to get right within a single dispatch, the first two each broken by compiling them, and even the
working shape has an unclosed half: nothing internal to arvo can catch a grade that *promises* a liberty
no body ever exercises (as opposed to a body taking a liberty its grade never granted, which the door does
catch). Whether the design should mandate one canonical fidelity-gated entry point per operation family,
the way there is exactly one funnel for quantisation, is a real open question with no default answer yet.

**No axis in the design names a radix.** Every formula (the quantum, the affine value map, `FullRange`'s
own definition) is written generically over a radix nothing carries, and the one proof case motivating
the identity/policy/lowering split at all (IEEE 754's two encodings of one decimal format) is a radix-ten
example the current axis set cannot express, and arvo has no executable arithmetic at any radix but two.
This is unresolved from file 11 and untouched by this stretch.

**`Stored<BITS, U>` does not determine a real IEEE format** (the significand derivation is off by one
against real hardware, missing the hidden leading bit; the reserved exponent codes and the exponent's own
encoding bias are unnamed), and whether the general parameterised form should ship now or wait for a real
consumer of a non-hardware float format is unresolved from file 11 and untouched by this stretch.

**Whether quantisation fires per operation or is deferred is now understood to require expressing
*both*, not choosing between them**, since SystemC and MATLAB sit on opposite sides of exactly this
question and both must be representable under `13c`'s own test; the multiplicative half's
`mul_full`/`quantize` split answers this for multiplication specifically, but the addition-side
consequence (which reading decides whether the shipped `Warm` preset's migration is "wrap becomes clamp"
or something larger) is unresolved.

**`arvo-num-systems` and `notko-hlist` remain unread by this entire stretch of the review.** Six separate
members, independently, flagged this same unread pair as a likely input to the cost picture for any
type-level-set-shaped mechanism (the graded fallibility reading's `P(Cause)`, the algebra ladder's
vocabulary placement, the multiplicative half's type-level width arithmetic). `notko-hlist` specifically
does not exist as shipped source yet; it is itself at design stage. This is the single cheapest, most
repeatedly-flagged open item in the whole document and no dispatch has yet spent the time on it.

**Compile-time and monomorphisation cost of the newly proposed mechanisms is asserted, not measured, in
several places**: the atomic-fact ladder's cost against the current per-`Resolution` shape; the
const-fn-key discipline's cost as the product of a real consumer's composition set; the Stage G
bound-projection mechanism's cost under the same product; the fact-that-a-composition-and-arity together
form a search space nobody has priced against `08_fog_the_union_and_what_it_costs.md`'s already-measured
const-eval wall. None of these belong in this document as claims; they belong in `mock/benches/`, and
none of them have landed there yet.

**Whether `Distributes`/`Monotone` unify across their two different arities, and whether the whole
atomic-fact set survives being pushed past addition into multiplication and eventually division, is
tested for exactly two operations and not for a third.** Division's own interior-exactness bound is
predicted (not measured) to have no finite accumulator solution at all, since the exact quotient is
generically not representable at any width; if that prediction holds, the dive's working assumption that
there are two cases (addition, multiplication) rather than three is wrong.

**The build-layer receipt mechanism, and both fidelity witness closures (door-only versus
unconditional-blanket-plus-marker), are designed and not built.** This dive's own repeated record is that
the first two attempts at any new piece of this apparatus have each had a hole the next member found only
by compiling it; every unbuilt item in this section should be read with that specific suspicion, not as
a minor formality.

## 4. The droplist

Proposals or readings tested and found not to work, or decided against, stated with enough of their
reasoning that a member who believes a retest would come out differently knows what has to be overturned.

**Relocating the algebraic-law machinery to hilavitkutin, on the theory that associativity is
specifically the contract of parallel reduction.** Refused by op directly. Independently undercut by
measurement: the regrouping that would have motivated the move already happens inside arvo's own
licensed internals, on a single thread, worth roughly 2x, before any scheduler exists to relocate to.

**Gating `arvo-graph`/`arvo-comb`/`arvo-spectral` on `AddAssoc` (or any associativity fact) by default.**
Measured directly to admit the one preset (`Hot`, wrapping) whose recurrences return wrong answers under
these algorithms' own stated specifications, and refuse the two presets (`Warm`/`Cold`, saturating) that
compute correctly, because associativity and the distributivity these algorithms actually need are
different, complementary laws that invert across the same presets.

**"A documented traversal order substitutes for a law."** Wrong axis: associativity is about grouping,
not order, and contiguous chunking preserves element order exactly while still changing grouping, so a
documented-order combinator over a non-associative operation still disagrees with the sequential answer.

**Bounding a regrouping combinator on a numeric "diameter budget" rather than a boolean law.** Tried
directly and refused by measurement: signed saturating addition's regrouping diameter grows to the entire
representable range by a five-element fold, so there is no useful budget to bound against for the
arithmetic that motivated the idea.

**Predicting the accumulator-agreement threshold from the recovery map's monotonicity** (expecting
monotone maps like clamp/refuse to reach agreement strictly earlier than a non-monotone one like
substitute-zero). Refuted by exhaustive measurement: every non-homomorphism resolution reaches the same
threshold (`K = n - 1`, interior safety) regardless of whether it is monotone.

**Computing type-level width arithmetic as a const generic under `min_generic_const_args`**
(`<W1 as AddWidth<W2>>::Out` with the addition inside the feature's own `type const` item). Compiled and
refused structurally at the definition site, the identical wall the workspace's own capacity-as-a-type
migration already hit; the feature's sound subset explicitly forbids arithmetic over a still-generic
const parameter on its own right-hand side. Replaced by type-level (typenum-style) binary width encoding,
which needs no unstable feature and is verified working.

**Growing an accumulator's own *type* on every iteration of a runtime-bounded loop.** Cannot work in
principle, not merely unbuilt: a type cannot depend on a value only known at runtime. Replaced by fixing
the per-element product's type and checking accumulator sufficiency as a compile-time bound.

**Declaring a fidelity-licence coercion as a trusted marker trait with no associated items
(`impl ViewC<Relaxed> for Strict {}`).** Compiles clean when corrupted, with zero diagnostic, because a
permission-shaped coercion carries no data for the compiler to check against; a hand-verified attack
(a deliberately wrong grant) produced a silently wrong numeric answer that the type system had no way to
see. Two follow-up fixes were also tried and also failed: a fully generic blanket derivation hit the same
generic-const-in-type-position wall as the width-arithmetic attempt above, and porting the recovery-map
witness's own `WITNESS`-constant shape onto the same trait is disarmable exactly the way one existing
resolution constructor's own classification was already disarmed, because the implementor writing the lie
also controls the check for the lie inside the same impl block. Replaced by recomputing the relation
inline, in an ordinary `const {}` block, at every consumption site, never declared and trusted.

**A pushed, registered build-layer manifest** (a `linkme`/`inventory`-style `#[used] #[link_section]`
record per composition, emitted at each consumer's declaration site). Considered and rejected in favour
of the pull-shaped symbol-table read: a pushed manifest records what a consumer *declared*, which is
strictly worse information than what actually got *instantiated* and silently misses every composition
reached only through generic code, and it cannot be written at all for a generic function, because Rust
forbids an item declared inside a generic function body from naming that function's own type parameters.

**Treating `f64::mul_add` as file 17's fourth source-expressible fidelity liberty (contraction), lowering
to `llvm.fmuladd`.** Wrong on inspection: it lowers to `llvm.fma`, a distinct, exact IEEE operation with
one defined answer rather than a licence to pick either answer; it is unavailable under `#![no_std]`
without an unvetted or forbidden feature; and on a target with no hardware FMA unit it compiles to a
pessimising libm call, the opposite of what a licence should cost. `Fused` now belongs in the design as a
named operation, not as a fidelity permission; `Contract` (the genuine permission) remains a real,
unclosed residue.

**Citing "the one shipped `Monotone` law implementation"** (`impl<T: Direction> Monotone for
(TowardNegative, T, TowardPositive) {}`) as existing, shipped design. It does not exist as any kind of
implementation in the source tree; it is an unlocked, open proposal in a design-round topic file, and two
members built directly on the false citation before a third caught it with a single grep. Checked against
its own admitted compositions and found false: its premise names only three of the five quantiser members
that actually decide monotonicity, and it asserts monotonicity for a resolution (`ReduceModulo` at both
range ends) that a separate, independent proof already rules out for every width, by the same
torsion-group argument that rules out wrapping addition's monotonicity.

**Assuming the recovery-map classification's cheapness transfers automatically to a new operation.**
Refuted twice, independently, by two different members probing multiplication before either had seen the
other's result: the classification is a property of the pair `(phi, Op)`, not of `phi` alone, and
multiplication needs its own full pass through the whole apparatus rather than inheriting addition's.
