# 24: The Multiplicative Half

**Member:** Julius O. Smith III (signal-processing arithmetic lens: fixed-point multiplication,
accumulation, rounding, and where numerical error actually comes from in a system that runs for a
long time. The habit I bring is that a filter is analysed whole or not analysed, that an unexamined
rounding rule is an unplaced pole, and that in this field the exact form and the cheap form are
usually the same form.)

**Position:** eleventh member of the algebra dive, file 24. Not a synthesis; the dive continues.

**What I read.** The brief's five op files first (`16b`, `16c`, `16d`, `17b`, `13c`), then
`11_current_shape_draft.md` in full. Of the dive: `13_mcsherry_where_the_laws_belong.md`,
`14_dolan_which_algebra_is_this.md`, `15_willsey_what_a_law_is_for.md`,
`17_orchard_are_these_all_grades.md`, `18_lamport_say_what_is_claimed.md`, and
`23_bellard_the_smallest_thing_that_checks.md` in full; `21_rompf_what_a_fact_is_keyed_on.md`
sections 4 through 6 closely (the growth measurement my brief cites lives there);
`16_fallin`, `19_ringer`, `20_wingo`, `22_amin` by the passages the later files cite plus a grep
sweep for every multiplication mention. I listed the panel directory and every probe directory
before reading inside them. On source, only to check claims before reasoning from them:
`arvo-strategy/src/arith_macros.rs` (the six multiply bodies and the four divide bodies),
`arvo-spectral/src/power.rs:60-88`.

**What I compiled and measured**, as distinct from what I reasoned about: four probes at
`24_probes/`, all `rustc +nightly-2026-05-28 -O`, run to completion, outputs quoted verbatim in
sections 2 through 5. And `cargo test --workspace` in `arvo/mock`: 654 passed, 0 failed, 9
ignored, reproducing every prior member's figure. Everything else is argument, offered as
directions rather than rulings; where I hold more than one reading I say so and leave the choice
where it belongs.

**The gates.** No ratified canon governs this question; the op checkpoints and `13c`'s standard
are the operative posture, and the multiplication dive is the work the draft itself
(`11_current_shape_draft.md:776-779`), file 15, file 18 and file 21 each explicitly hand forward,
so the assigned work is aligned with the standing instructions. The suite is green and, as file 18
already recorded, contains nothing about the surface this dive designs; the multiplication surface
specifically has no test that asserts anything beyond the shipped truncation's own behaviour, and
per file 23 section 5.5 the release profile has no tests at all. Nothing to refuse over; noted
because "the suite says nothing about the subject" is again the operative fact.

---

## 0. The brief's premises, checked, and one number that did not reproduce

The brief makes three factual claims and I checked all three before building on them.

**The shipped multiply is unconditional truncation at every strategy, and distributivity and
multiplicative associativity fail against it.** Verified at source: `arith_macros.rs:34, 99, 148,
219, 284, 331` are all `wrapping_mul` (or a widened product) followed by an unconditional
`>> FRAC`, exactly as file 15 reported (`15_willsey...md:112-133`), with the counterexamples at
`15_willsey...md:142-148`. One sharpening the rest of this file depends on: on a two's-complement
signed value, `>> FRAC` is an arithmetic shift, which rounds **toward negative infinity**, not
toward zero. The preset table's word "truncate" (`11_current_shape_draft.md:327`) is ambiguous
between the two, they have different bias structure (toward-zero's bias is sign-dependent and
odd-symmetric; toward-negative-infinity's is a constant negative offset), and the design's own
vocabulary already separates them (`TowardZero` against `TowardNegative`,
`11_current_shape_draft.md:204-205`). The shipped body is `TowardNegative`. The table should say
which one `Hot` means, because section 3 shows the difference is a systems-level property, not a
naming nicety.

**`Precise` loses its law under multiplication and rounding is why.** Files 18 and 21 both
measured it (`18_lamport...md:340-354`, `21_rompf...md:361-374`) and my probe 01 reproduces the
addition half exactly: in-range rounding fired on 0 of 256 addition pairs. On multiplication my
count is **108 of 256**, not the 128 both prior files report. I do not read this as either number
being wrong. My in-range gate takes "in range" as "the floored product lands in `[-8, 7]`"; their
models gate differently, and the discrepancy is exactly the boundary question the draft already
holds open at `11_current_shape_draft.md:712-717` (where in-range ends is itself a design
decision, the last representable midpoint against the raw maximum). Two probes disagreeing by 20
cases purely on the in-range definition is small independent evidence that the boundary definition
is load-bearing for multiplication in a way it never was for addition, since for addition the
count is 0 under every gate.

**The growth of the intermediate: logarithmic for addition, linear in `F` for multiplication.**
File 21's measurement (`21_rompf...md:371-389`) stands as stated: the smallest accumulator under
which every grouping that returns agrees needs `(n-1)*F` fractional bits for a multiplicative
fold. Section 2 argues that this true measurement is answering a question the field does not ask,
and that the shape it should drive is not the one it appears to drive.

---

## 1. Where a product lives: multiplication does not round. Narrowing does.

The single most important structural fact about fixed-point multiplication, and the one the
current shape has not yet said out loud:

**The product of two fixed-point values is exact.** A `Q(I1, F1)` value times a `Q(I2, F2)` value
is, bit for bit, a `Q(I1 + I2, F1 + F2)` value, with no rounding anywhere. The widths add, the
quanta multiply, and the result is a member of the wider lattice, always. This is the draft's own
`Growth::Exact` sentence ("widths add, quanta multiply, nothing is dropped",
`11_current_shape_draft.md:163`) taken at its word. Every hardware multiplier ever shipped
computes this full product; a 16 by 16 multiplier produces 32 bits, and the DSP families built for
this arithmetic hold it (a 24 by 24 multiply on the DSP56000 family produces 48 bits into a 56-bit
accumulator; a 16 by 16 on the TI C55x produces 32 into a 40-bit accumulator).

So the finding that "multiplication always quantises" (file 15's two-independent-failure-sources
analysis at `15_willsey...md:156-166`, file 17's nonempty Event grade at `Growth::Exact`,
`17_orchard...md:125-127`) is true of one specific composite operation: **multiply and then force
the result back into the operand's own numeral**. It is not true of multiplication. The
unconditional `>> FRAC` in the shipped body is not part of the product; it is a narrowing bolted
onto the product's tail, and every law failure files 15 and 18 measured lives in the narrowing.

This relocation is not wordplay; it changes what the design should build, in three ways.

**The exact product should be a first-class typed value.** `mul_full : Number<Q(I1,F1), S> x
Number<Q(I2,F2), S> -> Number<Q(I1+I2, F1+F2), S>` is total, exact, commutative, associative, and
distributes over exact addition, at every strategy, because no quantiser is present to break
anything. It is the multiplicative operation that HAS laws, and they are free. In Orchard's
vocabulary this is the genuinely graded reading of multiplication: the *numeral itself is the
grade*, products add grades, and the Event multiset stays empty exactly as long as the type is
allowed to grow. File 17's "multiplication's Event grade is nonempty even at `Growth::Exact`"
(`17_orchard...md:606-609`) is then an artifact of holding the result type fixed, not a fact about
the operation.

**The quantiser becomes a named map between numerals, not a shadow inside every operation.**
`quantize : Number<Wide, S> -> Number<Narrow, S>`, carrying the whole `Quantisation` apparatus
(the triple, the range pair, the fallibility projection), applied at explicit narrowing sites:
stores, assignments, the tail of a composite convenience op. All of section 3.3's excellent
vocabulary survives untouched; what moves is *where it fires*. This is also where the whole
accumulator thread of files 18 and 21 lands naturally: a fold that accumulates in a wide numeral
and quantises once is not a special discipline needing an eleventh axis, it is the ordinary
composition `quantize . fold(add_exact) . map(mul_full)`.

**The cost is type-level width arithmetic, and it must be said honestly.** `Q(I1+I2, F1+F2)` in a
return position is a const expression computed from generic parameters in type position, which is
exactly the forbidden `generic_const_exprs` shape. The escape is the same one the workspace's
capacity-as-a-type migration already used (`unstable-features.md`, the GCE row): widths become
types and addition becomes an associated-type computation (`<W1 as AddWidth<W2>>::Out`), the
`notko-hlist` `Cardinal` machinery being one existing carrier for exactly this kind of type-level
arithmetic. I did not sketch this, and given this dive's record ("should be cheap" preceded a hole
four times in Thread C), it needs a sketch before anyone treats it as settled. But it is the same
species of move the design has already made once, not a new kind of risk.

Two readings on how far to take this, held rather than resolved. The strong reading makes
`mul_full` the *primitive* and in-type multiply a derived convenience (`quantize(mul_full(a,b))`),
which is where SystemC sits (section 7). The weak reading keeps in-type multiply primitive and
adds `mul_full` beside it, which is smaller against the current shape and keeps `16d`'s rewrite
cost down, at the price of two multiplication stories. I lean strong, because under the weak
reading the law machinery still has to explain per-op multiply's lawlessness case by case, where
under the strong reading the lawless thing is visibly a composition of a lawful thing and a
quantiser, and every law question reduces to one question about one map.

---

## 2. The load-bearing multiplicative shape is the MAC, and its growth is logarithmic

File 21 measured the multiplicative fold, `x1 * x2 * ... * xn`, and found interior exactness
costs `(n-1) * F` fractional bits, linear, against addition's `ceil(log2(n-1))` extra integer
bits (`21_rompf...md:386-389`). The measurement is right. The shape is the wrong one to size an
accumulator for, because it is not the shape that occurs.

In forty years of fixed-point signal processing, the multiplicative workload is overwhelmingly not
a chained product. It is the **multiply-accumulate**: `sum over i of a_i * b_i`. Dot products,
FIR filters, IIR state updates, correlations, matrix products, `power.rs:71`'s own
`sq_sum + ns[k] * ns[k]` (arvo's one genuine fold, and it is a MAC, not a product chain). The MAC
is a *bilinear* form: products appear at depth one, and everything above them is addition. Its
exact-interior requirement is therefore the product width plus addition's logarithmic term:

```
acc >= Q(I1 + I2 + ceil(log2 n),  F1 + F2)
```

Logarithmic in `n` again. The linear-in-`F` explosion only appears when products nest, and
products nesting without renormalisation is something the field's disciplines exist to prevent
(per-stage renormalisation in cascaded biquads, block floating point, log-domain representation
for long gain chains), because the dynamic-range growth is exponential no matter what the type
system does about it. Shipped silicon states the same bound: the 56000's eight guard bits above
its 48-bit product are exactly `ceil(log2 256)`, a contract that 256 MAC steps cannot overflow.

Probe 01 (`24_probes/01_the_mac_discipline.rs`) measures what the two disciplines do to the law
question, on the same signed Q2.2 model the dive has used throughout, comparing per-operation
quantisation (every product forced into the operand numeral, every partial sum requantised, all
groupings) against the wide accumulator (exact products summed exactly, one quantisation at the
store). Verbatim:

```
in-range rounding fired: add 0/256  mul 108/256

n | samples | diamA(rne) | worst|A-B|(rne) | biasA(rne) | diamA(floor) | worst|A-B|(floor) | biasA(floor) | diamB
2 | 65536  | 0  | 8  | -0.0660 | 0  | 8  | -0.1706 | 0
3 | 200000 | 7  | 10 | -0.1384 | 7  | 10 | -0.3384 | 0
4 | 200000 | 14 | 15 | -0.2029 | 14 | 15 | -0.4769 | 0
5 | 200000 | 15 | 15 | -0.2550 | 15 | 15 | -0.5911 | 0
```

By a five-element MAC the per-operation discipline's grouping diameter is 15 raw ulps on a
16-value range, meaning the answer set spans nearly the whole representable set, and its worst
error against the once-quantised answer is the full range. The wide accumulator's diameter is 0 at
every arity **by construction**: its interior is exact integer addition, which is associative, so
there is nothing for a grouping to change, and the single quantisation at the store is the only
approximation in the entire computation. (The per-op numbers conflate rounding and clamping error,
deliberately: forcing a product into the operand numeral clamps products that were never going to
stay in range, which is the honest per-op discipline and is what the shipped shape does.)

This is the answer to "what does an accumulator need to be", stated as the field states it:

**wide enough that the interior is exact, so that the only quantisation is the one at the
boundary.** Then associativity of the interior is not a law to derive, condition, or gate. It is
inherited from the integers. The entire law apparatus this dive has built collapses, for the MAC
shape, to one question: is the single boundary quantisation the map you meant. And the efficiency
answer coincides with the correctness answer, which in my experience is the signature of the right
design: a per-element quantise-and-clamp is a serial dependency that kills vectorisation, while
the wide-accumulate shape is exactly what a SIMD unit, a hardware MAC (`smlal`, `pmaddwd`), and
LLVM's own widening-multiply patterns want to see. The lawful form IS the fast form.

What this does to the standing threads:

Lamport's accumulator-as-quantifier (`18_lamport...md:369-427`) and Rompf's reading three
(`21_rompf...md:333-345`, the side condition in the arity) are both, I think, correct, and the MAC
bound above is their multiplicative instance: the side condition is
`acc >= product_width + ceil(log2 n)`, an inequality on consts, expressible today
(`18_lamport...md:400-407` already said so for addition). My evidence adds a third argument for
the combinator-parameter reading over the eleventh-axis reading: section 3's error feedback needs
carried *state*, and state can live in a combinator's accumulator object but not in a zero-sized
policy marker, so the combinator layer has to exist as a first-class home anyway.

Rompf's Kleene/existential split of the bound (`21_rompf...md:397-418`, fractional bits buy value
agreement, integer bits buy definedness agreement) survives intact and gains an engineering
reading: the fractional half of the accumulator is the *noise* budget and the integer half is the
*headroom* budget, which is precisely the vocabulary a DSP engineer sizes an accumulator with, and
they are independent decisions there too.

And file 18's prediction (`18_lamport...md:716-720`), that `Growth::Exact` with a `2F` accumulator
recovers the partial-identity property for multiplication, which file 21 confirmed at `n = 2` and
refuted from `n = 3` for the product chain (`21_rompf...md:420-424`): for the MAC shape the
prediction is simply true at every `n` with the logarithmic integer term added, which probe 01's
`diamB = 0` column is. The dive has been alternating between "multiplication is the same theorem
as addition" and "multiplication is a different problem", and the resolution is that it is the
same theorem *for the shape that occurs* and a different problem only for the shape that mostly
does not.

---

## 3. What rounding does to multiplication that it does not do to addition: bias, and drift

For addition the in-range rounding rule fires never (probe 01, 0 of 256); the whole addition-side
law story was about the range pair. For multiplication the in-range rule fires on roughly half of
all pairs, which means the *choice among the six direction markers* is, for the first time,
the dominant error source. What that choice controls is not any per-operation property. It is the
**mean** of the injected error, and in a system that runs for a long time the mean is everything,
because it integrates.

Probe 02 (`24_probes/02_bias_drift_and_error_feedback.rs`) accumulates K zero-mean quantised
products with the range rule isolated away entirely (the accumulator never saturates), so the
in-range rule's own contribution is the only thing measured. Verbatim:

```
K | floor-drift (quanta) | rne-drift (quanta) | feedback-drift (quanta)
256   | -135.7   | +2.3  | -0.719
1024  | -519.4   | +10.6 | -0.422
4096  | -2018.7  | +29.3 | -0.719
16384 | -8046.0  | +73.0 | -0.969
65536 | -32291.7 | +13.3 | -0.656
```

Three regimes, each a theorem the measurement confirms:

**Toward-negative-infinity (the shipped `>> FRAC`, the `Hot` table row) drifts linearly**, at
about `-K/2` quanta: each firing injects mean `-q/2`, the injections integrate, and after 65536
operations the accumulated answer is thirty-two thousand quanta below the true one. In an audio
system this is the textbook DC ramp; in an IIR feedback loop the same mechanism manifests as limit
cycles. This is what "truncate" costs in the one place its cost cannot be seen per-operation, and
it is why I flagged the `TowardZero`/`TowardNegative` ambiguity in section 0: toward-zero's bias
is sign-dependent (it decays magnitudes symmetrically, which in a feedback loop is actually
stabilising, suppressing limit cycles), toward-negative-infinity's is a constant offset. The two
"truncations" are different filters, and the preset table currently does not say which one `Hot`
is. The shipped body answers: `TowardNegative`.

**Round-to-nearest-even is zero-mean and random-walks** at about `q * sqrt(K/12)`: seventy quanta
after sixteen thousand operations, no ramp. This is why nearest-even is the field's default and
the table's choice for `Warm`/`Cold`/`Precise` is right.

**First-order error feedback stays inside one quantum, forever.** Carry the previous quantisation
residual and add it back before the next quantisation ("fraction saving" in fixed-point filter
practice, first-order noise shaping in the converter literature): total error is bounded by `q`
at every K, with one word of state. This is the field's real weapon against quantisation noise,
and the design cannot express it, for the same stated reason it excludes stochastic rounding
(`11_current_shape_draft.md:705-707`): the resolution is a pure function of position in the
zero-sized-marker const model, and feedback is stateful. I want this exclusion *recorded*, exactly
as the draft says the stochastic exclusion should be, and I want its consequence stated as a
design direction rather than a loss: **stateful quantisation disciplines live in the accumulator
object at the combinator layer**, where state is ordinary, not in the type-level policy. The
combinator that owns the wide accumulator of section 2 is precisely where a feedback quantiser, a
dithered quantiser, or a noise-shaped quantiser would sit, as a value with a `carry` field,
consuming the same type-level vocabulary for its per-step rule. One more argument that the
combinator layer is load-bearing and cannot be dissolved into the axes.

The Orchard connection, which I did not expect to be this literal: file 17's Event multiset (each
quantisation event as a `(width, rule)` term, `17_orchard...md:87-92`) is, read by a DSP engineer,
**the noise budget**. Each `(width, rule)` event has a known error model: mean `-q/2` for
toward-negative, mean 0 variance `q^2/12` for nearest. A composition's accumulated grade is
therefore not just bookkeeping; it is the input from which a derived `ErrorBound` fact family is
*computable*: worst-case (interval) bounds compose by addition, and the count of events is the
`10*log10(n)` dB SNR cost of quantising n times instead of once, which is the engineering number
the MAC discipline of section 2 exists to make small. The grade the design already carries is the
analysis the field already runs. Naming that connection costs a paragraph and buys the "bounds,
not laws" vocabulary of section 5 a mechanism.

---

## 4. Two quantisations in one operation: double rounding, and ToOdd's validity range

Under `Growth::Narrowed` a single multiply quantises twice (wide product to intermediate, then to
the destination), which the draft flags as unmodelled (`11_current_shape_draft.md:759-761`) and
which its section 5.1 already credits round-to-odd as the classical cure for
(`11_current_shape_draft.md:707-710`). Probe 03 (`24_probes/03_double_rounding_and_to_odd.rs`)
measures both halves of that credit, exhaustively, products carrying 6 fractional bits narrowed to
2 via an intermediate at W:

```
W = 3 (1 extra bit):  rne-then-rne mismatches 3072/16384, odd-then-rne mismatches 7168/16384
W = 4 (2 extra bits): rne-then-rne mismatches 2048/16384, odd-then-rne mismatches    0/16384
```

Nearest-twice disagrees with nearest-once on 12 to 19 percent of inputs, so `Narrowed` growth with
a nearest intermediate is *not* the composition of the two quantisations it names; it is a third
quantiser with no name in the vocabulary. Round-to-odd at the intermediate restores exact
agreement with the single rounding, which is the Boldo-Melquiond theorem the draft's Flocq
grounding already contains. And the row the field's folklore forgets: **below the theorem's
precondition (at least two more fractional bits at the intermediate than the destination),
round-to-odd is worse than the naive thing**, 7168 mismatches against 3072. The credit the spec
gives ToOdd is real and it must ship with its validity range as a const bound, `W >= F + 2`, an
inequality on consts, expressible today, and cheap. An approximation whose validity range is not
carried with it is how a correct mechanism becomes a wrong system, and this one's range is one
line.

This also gives the `Narrowed` axis its multiplication story: `Narrowed<W, Lsb>` after a product
is the field's "keep W fractional bits" and its correctness statement is exactly the double-
rounding question; the design can state, as a derived fact, that
`quantize_F . narrow_W` equals `quantize_F` precisely when the intermediate rule is `ToOdd` and
`W >= F + 2`, and is otherwise a distinct quantiser whose error model adds the second event to the
grade. That is a checkable row-three fact in Lamport's table (bounded exhaustion at a model
width), and probe 03 is its prototype.

---

## 5. Repeated multiplication: the laws that are dead, the law that survives, and bounds for the rest

**Dead, and the field agrees they are dead.** Associativity of quantised multiplication and
distributivity of quantised multiply over quantised add fail (file 15's counterexamples, my
reproduction of the mechanism), and no fixed-point or floating-point standard anywhere promises
either. IEEE 754 promises per-operation correct rounding and nothing at all about two operations
composed. The design should stop trying to find conditions under which these laws hold in-type,
because the field's answer is that they do not and the systems that work were built knowing it.

**The law that survives, measured.** Probe 04 (`24_probes/04_the_multiplicative_law_that_survives.rs`),
exhaustive on the Q2.2 model:

```
rule  | recovery | monotone for all c>=0 | commutes with max | order-reversing for c<0
floor | clamp    | true                  | true              | true
floor | wrap     | false                 | false             | false
rne   | clamp    | true                  | true              | true
rne   | wrap     | false                 | false             | false
```

Scaling by a nonnegative constant, `x -> Q(c * x)`, is monotone under every monotone rounding rule
composed with clamp, because a composition of monotone maps is monotone (multiplication by
`c >= 0`, floor, nearest, and clamp are each monotone; a one-line proof, no width sweep needed,
exactly the derivation-over-exhaustion move file 18's section 4 argues for). And a monotone map
commutes with max and min on a chain, so quantised scaling distributes over max exactly:
`Q(c * max(a,b)) = max(Q(c*a), Q(c*b))`. Under wrap all of it dies, at every rule, by Dolan's
torsion-group argument (`14_dolan...md:163-181`), which needs no new proof here.

This is Dolan's section 5 prediction (`14_dolan...md:451-457`, the ordered-ring positive cone)
confirmed at the quantised level, and it mirrors McSherry's addition finding
(`13_mcsherry...md:152-199`) with the same preset inversion: the law multiplication actually
supports after quantisation is the *ordered* one, it holds for exactly the presets an
associativity gate would refuse (`Warm`/`Cold`/`Precise`), and fails for the one it would admit
(`Hot`). For the tropical crates this is directly load-bearing: `arvo-graph`'s max-plus and
`arvo-comb`'s min-plus recurrences scale weights multiplicatively the day anyone normalises a
weight, and the fact they need is this one. The atomic fact worth naming is the positive cone,
`MulMonotone` for `c >= 0` with order reversal for `c < 0`, keyed on (rule, recovery) via
Lamport's classification, derived by composition-of-monotone-maps, checked at a model width.

**Bounds for everything else.** For chains that genuinely nest products, the honest contract is
not a law but a bound, and the field has exactly two shapes of it. Per-operation: correct
rounding, error at most half an ulp of the result's quantum (nearest) or one ulp (directed),
which is IEEE 754's entire multiplicative promise. Accumulated: relative errors compound as
`(1 + e_1)(1 + e_2)...(1 + e_n)`, bounded by the standard `n*u / (1 - n*u)` (Higham's gamma_n),
with `u = 2^-(F+1)` for nearest; and for the additive accumulation of quantisation noise, variance
`n * q^2 / 12` against `q^2 / 12` for quantise-once. These are quantified statements over bounded
value spaces, checkable by the same bounded-exhaustion machinery Thread C built (an inequality
witnessed at a model width instead of an equality), and they compose along Orchard's Event grade
per section 3. A `Bound`-family fact (`RelError <= gamma_n`, `AbsError <= n*q/2`) is what the
multiplicative half publishes where the additive half published `AddAssoc`, and it is what a
consumer sizing a filter actually wants to read: nobody has ever asked me whether a multiply is
associative, and everyone has asked me how many bits they need.

---

## 6. Division and the reciprocal

Division is the one operation where the exact-product story of section 1 has no analogue: the
exact quotient of two representable values is generically not representable at any finite width
(`11_current_shape_draft.md:693-697` already says this and correctly declares `Exact` growth
undefined for it). The field's resolution is old and clean and the draft's own parenthetical
guesses it right: **division is *defined* as the correctly-rounded quotient**, `quantize(exact
quotient)`, one quantisation, no intermediate, which is IEEE 754's own definition and the only
coherent meaning `Exact` growth can have for it. Its laws are bounds only (half an ulp per
operation), and it contributes one Event to the grade unconditionally, like multiplication's
narrowing and unlike multiplication itself.

Two things around it need saying.

**The reciprocal-multiply is a licensed liberty, and arvo already takes it unlicensed.**
`a * b.recip()` is not `a / b` (two roundings against one; the `arcp` row of file 17's liberty
table, `17_orchard...md:527-531`, measured the difference directly). `power.rs:74` computes
`sq_sum.sqrt().recip()` and multiplies by it at `power.rs:79`, which is the reciprocal-multiply
taken unconditionally in a generic function whose bound (`Recip`) can be satisfied by any
composition including `Precise`, exactly Wingo's finding (`20_wingo...md` section 0) arriving at
the multiplicative gate. Under the fidelity axis this is where multiplication meets it: `Div` is
the strict form, `mul-by-recip` is the `arcp`-licensed form, and the licence is per-composition.
Newton-Raphson refinement of a reciprocal (each iteration doubling the correct bits) is the
standard implementation ladder underneath and belongs to `arvo-always-optimal-internals.md`'s
per-artifact validation bin, not to the type story.

**The shipped divide-by-zero substitutes a value, and that is a `Cause`, not a resolution.**
`arith_macros.rs:38`: "div-by-zero returns the numerator." A made-up total answer on the one
input where no answer exists, silently, at the wrapping tier. Under Orchard's `P(Cause)` reading
(`17_orchard...md:189-222`) divide-by-zero is precisely the second member the `Cause` enumeration
has been waiting for, and the draft already names it prospectively
(`11_current_shape_draft.md:519-520`). The prior attempt is evidence for why the redesign is
happening: a refusal cause with no home in the vocabulary got papered over with a value, which is
the substitute-zero pathology (unstable under every analysis in this dive) shipped in a corner
nobody tested. The new shape should route it through the same quantisation-style resolution
choice (refuse, substitute, saturate-to-max as some DSPs do) with the same checked classification,
rather than hard-coding one.

---

## 7. The representability test: MATLAB and SystemC sit on opposite sides of the spec's open question

`13c`'s standard makes MATLAB, IEEE 754 and SystemC a test. Run the test on the multiplication
question and it returns something sharper than a gap list: the two fixed-point reference systems
resolve the draft's own unresolved firing-site question (`11_current_shape_draft.md:688-692`,
per-operation against deferred) in **opposite directions**, so the abstraction must express both,
and the choice the spec has been treating as a fork is actually a requirement to carry two sites.

Stated from field knowledge of both systems, to be pinned against vendor reference values by the
table the draft's section 5.1 already proposes (which is the right verification vehicle for this
and I did not duplicate it here):

**SystemC (IEEE 1666) quantises at assignment.** A mixed `sc_fixed` expression is computed exactly
(the arbitrary-precision `sc_fxval` carrier), and the quantisation and overflow modes fire when
the result lands in a target variable. That is precisely section 1's shape: exact typed
intermediates, one quantisation at the store. The deferred reading, as a shipped standard.

**MATLAB's Fixed-Point Designer quantises per operation, under per-operation-family policy.** A
`fi` object's `fimath` carries `ProductMode` and `SumMode` *separately*
(`FullPrecision`, `KeepLSB`, `KeepMSB`, `SpecifyPrecision`, with per-family word lengths), plus a
rounding method and overflow action. `FullPrecision` products are section 1's exact product
(widths add); `KeepLSB`/`KeepMSB` are, with startling exactness, the draft's `Narrowed<W, Anchor>`
axis with the anchor at either end (`11_current_shape_draft.md:163`), which is a genuine credit to
the Growth axis's design: the vocabulary is already the vendor's. The per-operation reading, as a
shipped standard, with defaults (floor rounding, saturate overflow) that pair the shipped `>> FRAC`
rule with `Warm`'s range recovery.

Two consequences.

**The firing site is expressible, not chooseable.** With section 1's shape (exact product as
primitive, quantiser as a named map), both standards fall out: SystemC is the primitive used
directly, MATLAB's modes are named compositions (`ProductMode = KeepLSB(W)` is
`narrow_W . mul_full`). Without it, one of the two reference systems is unrepresentable, which
under `13c` is a defect by definition. This is, I believe, the strongest single argument for the
strong reading in section 1, and it comes from the design's own stated test rather than from my
field's preference.

**Policy wants a per-operation-family key.** MATLAB's `ProductMode` and `SumMode` are independent
knobs. The draft's `Policy` carries one `Growth` and one `Quantisation` for the whole composition.
Representing a `fi` whose sums keep full precision while its products keep LSBs requires the
policy to be keyed by operation family, which is also exactly what file 18 proved from the
mathematics (`18_lamport...md:357-359`: "the design's law key has to carry the operation").
Standard and theorem agree; the axis table should follow. Whether that is `Quantisation<Op>` as
one keyed axis or a policy record with per-family members is a shape question I leave open; the
requirement itself I do not think is open any more.

---

## 8. The presets under multiplication, and the Adjustment/Bias closure gap

The preset table's values were chosen against addition, and mostly survive; what changes is what
must be *said* about them.

`Hot` (toward-negative, wrap, narrowed): the fast tier is fine to exist, per
`arvo-toolbox-not-policer.md`, and its multiplicative consequences are the section 3 DC ramp and
the section 5 loss of the ordered structure. Both belong in its documentation as stated
consequences ("this tier drifts under sustained accumulation; use the wide-accumulator combinator
for long folds"), the same way the draft already states `Hot`'s signed-fold exclusivity for
addition (`11_current_shape_draft.md:334-338`). And the table's "truncate" must be disambiguated
to `TowardNegative` (section 0), or changed to `TowardZero` deliberately if the limit-cycle-
suppression property is wanted, which is a real choice with a real difference and today it is
being made silently by an arithmetic-shift operator.

`Warm`/`Cold` (nearest-even, clamp, exact growth): correct, and under section 1's reading "exact
growth" for multiplication *means* the SystemC discipline: the wide product exists exactly and the
clamp-and-round fires at the store. The draft's open "on every store against on every operation"
consequence for `Cold` (`11_current_shape_draft.md:688-692`) resolves to per-store under this
reading, which is also the cheaper one for a bitpacked column.

`Precise` under multiplication needs a decision the addition-side design never forced. Per-op
`Precise` multiply quantises on half of all pairs (section 0), so "most precise" per-op can only
mean *correctly rounded*, the IEEE promise, whose law is the half-ulp bound of section 5, not any
associativity, and file 18's measured existential-associativity failure
(`18_lamport...md:347-354`) is simply what correct rounding composes to. The genuinely most
precise multiplication arvo can offer is not a rule triple at all; it is the exact typed product
of section 1, refusing nothing because nothing is dropped. Two readings for the preset: `Precise`
multiply returns the widened exact type (maximally precise, type grows, refusal only at an
eventual narrowing), or returns in-type correctly-rounded-and-refusing (the current table's
shape). The first is the more honest reading of the preset's stated intent ("the most precise at
the price of both storage and compute", `11_current_shape_draft.md:322-324`); the second is the
smaller change. I hold both and note that the first is only available at all if section 1's typed
product exists.

**The closure gap nobody has named.** The exact-product story of section 1 is stated for `Unit`
adjustment and `Zero` bias. It does not extend: the product of two `FullRange<F>` values carries
an adjustment of `(radix^F / (radix^F - 1))^2`, which is not of `FullRange` form, so the
`Adjustment` axis is not closed under multiplication; and a biased numeral's products generate
cross terms (`(a + b1)(c + b2)`), so `Bias` is not closed either, mirroring the `AddClosed` gate
the law key already carries for addition (`11_current_shape_draft.md:285-287`). The choices are a
general rational-adjustment constructor (closing the family, at real vocabulary cost) or a
`MulClosed` condition gating the typed product on `Unit`/`Zero` with an explicit immediate
renormalisation for UNORM-style types, which is what graphics practice does (the `/255`
renormalisation baked into every UNORM multiply). Either is defensible; silently defining
`mul_full` only where it happens to work is the one indefensible option, and today nothing says
any of this anywhere.

---

## 9. The downstream contract, designed

Per `16c`, for the boundary my subject touches: the lowering of the MAC discipline.

**What a downstream target reads out of the types.** The combinator of section 2 carries, in its
type, everything a lowering layer needs and nothing it does not: the element numerals (hence the
product width `I1+I2, F1+F2`), the arity or unroll factor `n` as a const, the accumulator numeral
(checked against `product + ceil(log2 n)` by a const bound), and the single boundary quantiser
with its rule triple. That is precisely the shape of a hardware MAC contract: a build layer or a
cfg-gated body (per `arvo-always-optimal-internals.md` Kind 1) can select `smlal`/`smlsl`,
`pmaddwd`, or a plain widening multiply-add chain, knowing the interior is exact integer
arithmetic and therefore *freely reassociable and vectorisable with no licence at all*, which is
file 16's own finding that integer wrapping addition needs no fast-math flag, arriving here as the
reason the wide-accumulator shape lowers well everywhere. The per-op shape, by contrast, cannot be
lowered to any MAC unit, because no silicon quantises between accumulation steps; the discipline
the mathematics wants and the discipline the hardware wants are the same discipline, and the
design should say so rather than treat lowering as a separate concern.

**What arvo needs back from the target.** For correctness: nothing. The whole discipline is
source-expressible (the same conclusion Orchard reached for fidelity, `17_orchard...md:562-570`,
holds here with less effort, since exact integer interiors need no licence). For performance: only
the Kind 1 cfg gates arvo already owns. If a build layer ever rewrites multiplicative code (fusing
a multiply-quantise-add into a MAC, changing the number of quantisation events), that rewrite
changes the Event grade and therefore the error model, so it is licensed by the fidelity axis or
not at all, and file 23's receipt requirement (`23_bellard...md` section 7) applies to it
verbatim. A build layer must never turn a quantise-per-op chain into a wide-MAC silently, even
though the wide answer is *better*: it is a different function, and better-but-different is still
the Thread C fourth-pass gap.

**One thing the spec should state so nobody rediscovers it.** The number of quantisation events in
a lowered body is observable in the artifact (each is a shift-and-round instruction cluster), so
file 23's rule-based verifier has a natural multiplicative rule: a composition whose declared
discipline is quantise-once must not contain per-element rounding sequences in its loop body.
That is a real, checkable, low-noise witness of exactly the property this file is about, and it is
the only one of file 23's rule shapes where I can name the instruction pattern with confidence.

---

## 10. What I would flag for the next member, unresolved

**The typed exact product needs a sketch before it is believed.** Type-level width addition under
the forbidden-features regime (`AddWidth` associated-type arithmetic, plausibly over
`notko-hlist`'s `Cardinal`) is the load-bearing mechanism of section 1 and this dive's record is
that unbuilt shapes have holes. If the sketch fails, the weak reading of section 1 (exact product
beside in-type multiply, at fixed widened types per width pair) is the fallback and most of the
argument survives, at more vocabulary cost.

**The 108-against-128 boundary question.** Two probes now disagree on the in-range multiplication
rounding count purely by their in-range gate, which is the draft's 5.1 midpoint-boundary item
biting a measurement inside this very dive. The vendor-reference-values table the draft proposes
would settle the gate; whoever builds it should include a multiplication row.

**Whether Policy splits per operation family** (section 7) is, I think, forced by the MATLAB test
plus file 18's theorem, but the shape of the split (keyed axis against policy record) is open, and
it interacts with the modifier/diagnostics work in draft 4.1 in ways I did not trace.

**The stateful-quantiser hook is named and not designed.** Section 3 argues error feedback,
dither, and noise shaping live in the combinator's accumulator object. Nobody has written that
object's contract (what state it may carry, how it settles, what its determinism claim becomes).
It is the multiplicative sibling of file 18's refusal-discipline clause and belongs in the same
combinator contract.

**The `ErrorBound` fact family is proposed and unbuilt**, including its connection to the Event
grade (section 3, section 5). It is the "bounds, not laws" half of this file made mechanical, it
is row-three checkable, and it is the piece I would build first after the typed product, because
it is what a consumer sizing a filter actually consumes.

**I did not read `arvo-num-systems` or `notko-hlist`.** Five members have now flagged the same
unread pair; section 1 makes the second of them load-bearing for the multiplicative half
specifically. That dispatch is overdue and cheaper than another probe.
