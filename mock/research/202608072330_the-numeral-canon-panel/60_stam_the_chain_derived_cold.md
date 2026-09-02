# 60. What a chain is, derived cold

**Persona.** Jos Stam. Composed numerical operations and what stays stable through them.

**Protocol state.** Phase one, written cold. Read before writing this: `INTENTS.md`, `00_brief.md`,
the workspace rules, and nothing else. Not read: any numbered panel file, `OPTIONS.md`,
`DROPLIST.md`, `RULES.md`, `seed/`, any probe directory, the git log. Phase two will be appended
below a marked line and phase one will not be edited after its commit.

**Gates.** Canon gate: arvo has no canon and this panel exists to write it (`00_brief.md`, "Arvo
has no canon. This panel writes it"); this dispatch is that work, governed by `INTENTS.md` and the
brief's fixed list, under explore-do-not-settle. Test gate: `mock/crates` verified empty on this
tree (zero crates, zero `.rs` files), so no suite exists to run or audit; the evidence surface is
`60_probes/`, and every probe there carries a mutant check so the instrument is shown able to fail
before its pass is trusted.

**Evidence.** `60_probes/`, committed before this file. Toolchain `nightly-2026-05-28`
(`rustc 1.98.0-nightly (57d06900f)`), passed explicitly. Raw transcripts (`p_a.out`, `p_b.out`,
`p_c1.out`, `p_c2.stderr`) are committed beside the sources. No probe is a bench; no magnitude is
claimed anywhere in this file.

## 1. The derivation in one paragraph

In exact arithmetic there is no such thing as a chain. Composition of exact operations is just
another exact operation, associativity holds, order is irrelevant, and a model that describes one
operation describes them all by induction. The chain becomes an object with its own identity for
exactly one reason: each operation's exact result generally does not lie in the format, so
something must bring it back, and that something, the adaptation, must be placed somewhere. **A
chain is a composition of exact operations together with a schedule of adaptation points.** The
ops are the mathematics; the schedule is the format's contribution; and the schedule is part of
the function's meaning, not an implementation detail, because two schedules over the same ops
compute different functions. Everything else in this file unpacks that sentence and tests it.

## 2. What the object is

The right carrier is not a linear sequence. Real computation is an expression, a term over the
operation signature, in general a DAG with sharing; a linear chain, a tree, and a fold are all
special shapes of it. But for format purposes the term structure is not the load-bearing part.
What matters is three layers laid over the term:

1. **The exact ops on the nodes.** Each op, taken exactly, is a map between formats, and its exact
   result has a determinable format: a fixed-point multiply's exact product has the sum of the
   integer widths and the sum of the fraction widths; an add of two same-format values fits in one
   extra integer bit; a fold of k adds fits in ceil(log2 k) extra integer bits. Call this the
   **width algebra**. It is not an error estimate. It is bookkeeping, and it is exact.
2. **The formats on the edges.** Intermediate edges may carry wider formats than the boundary
   format the consumer sees. An edge whose format equals the op's exact-result format carries no
   information loss at all.
3. **The schedule.** The subset of edges at which an adaptation is applied, narrowing the exact
   intermediate back toward the boundary format. Where the schedule is empty until the final edge,
   the chain is the once-adapted exact composite. Where every edge adapts, the chain is the
   stepwise-rounded computation a native programmer writes.

One refinement that fell out of probing and that I had not expected to be so clean: **adaptation
has two sides.** Fractional excess is handled by rounding; integral excess is handled by an
overflow policy (wrap, saturate, refuse). A schedule places both, and the two sides misbehave
independently. Probe B (`p_b_order_dependence.rs`) shows the overflow side alone already makes
schedules semantically distinct, with no rounding anywhere in sight: a per-step saturating fold of
the multiset {30000, 10000, -25000} in i16 gives 7767 or 15000 depending on order; the wide exact
accumulator with one saturating adapt gives 15000 in every order; and, the part worth pausing on,
the per-step **wrapping** fold also gives 15000 in every order, because arithmetic mod 2^n is a
ring homomorphism and therefore commutes with any association. Wrap, the "unsafe" policy, is the
order-independent one; saturate, the "safe" policy, is the one that makes the chain's value depend
on evaluation order. That is exactly the kind of fact a canon should carry, because it is
counterintuitive, structural, and consequential for the strategy intents below.

## 3. What a chain needs that a single op does not

Five things, none of which a per-op model can express.

An **intermediate format** wider than the boundary format, or there is nothing between "adapt
after every op" and nothing. A **schedule**, since with more than one op there is more than one
place to adapt and the choices are semantically distinct (probe A: same ops, three schedules,
three functions). An **association and order statement** for folds, because once any adaptation in
the schedule is order-sensitive the fold's value depends on the reduction tree; this is where
chains meet I10, since running on n cores IS reordering the fold. A **count bound**, because the
headroom an exact additive accumulator needs is ceil(log2 k) integer bits and k must come from
somewhere; a count is a static or contractual quantity, which is typestate-shaped. And an **error
bound composed per adaptation point**: the drift of a chain is the sum over its adaptation points
of the local adaptation error, each weighted by the sensitivity of the remaining suffix of the
computation to a perturbation at that point. That last one is classical backward-style error
analysis and I state it here only to note its shape: it is a sum over the schedule, so the
schedule is also the index set of the error analysis. Fewer adaptation points is not merely
cheaper rounding; it is a structurally shorter error sum.

## 4. Exactness for a chain has grades, and one of them is special to fixed point

**Grade a, composite correct rounding.** The result equals the boundary-format adaptation of the
chain's exact value: one adaptation total, applied to the true composite. This is the strongest
claim and the natural reading of "matching the once-adapted exact answer". Probe A demonstrates it
constructively for a dot product: the wide arm (exact 2F products summed exactly, one final
round-to-nearest-even narrow) satisfies the defining property of correct rounding on all 46,656
exhaustively enumerated inputs, where the checker verifies the property |c·2^F − n| ≤ 2^(F−1) with
ties to even, and never itself rounds, so arm and oracle cannot agree by shared bug. The mutant
arm (same schedule, truncating final narrow) is flagged on 22,476 of those inputs, so the checker
demonstrably can fail.

**Grade b, stepwise correct rounding.** Every op is correctly rounded into its edge's format. This
is what IEEE 754 gives per operation and what a native programmer's chain does. It is the only
grade that is *compositional*, statable per op and holding for every chain by induction, and that
is both its virtue and its ceiling: probe A's stepwise arms drift from the composite answer by up
to 2 ulp (per-step round-to-nearest, 15,628 of 46,656 inputs wrong) and up to 3 ulp (per-step
truncation, 42,892 wrong), in line with the hand bound of roughly one ulp per truncating
adaptation point and half per rounding one.

**Grade c, bounded drift.** The result is within a stated bound of the composite exact answer, the
bound a function of the chain's shape (number of adaptation points, condition of the suffixes).
Weakest, and still worth naming, because it is the honest claim for chains whose exact
intermediates are unaffordable.

**Grade s, structural exactness**, and this is the fixed-point family's own possession. For fixed
point, add, subtract, and multiply-into-the-exact-width are not approximately good, they are
**exact**: they are integer operations. The only adaptation points a fixed-point chain can contain
are the rescaling narrow after a multiply, division, and explicit narrowing. A chain of fixed
point ops that stays inside its width algebra is not "accurate", it is the mathematics itself,
drift-free by construction, provable by bookkeeping with no error analysis at all. Floats have
sparse islands of the same thing (Sterbenz subtraction; the error-free transformations, of which
more in section 7), but for fixed point it is the mainland. This is, to my eye, the single
strongest argument that the format concept must carry the width algebra: it converts chain
correctness from analysis into typestate.

## 5. The multiply question: the ever-growing intermediate is real, and the window dissolves it

The question as posed: a fixed-point multiply rescales, the product of two values at fraction
width F has fraction width 2F and must come back; what does a chain of those cost, and can it be
written at all without an ever-growing intermediate?

The asymmetry that decides this: **addition composes headroom logarithmically, multiplication
composes width linearly.** A fold of k adds at F is exactly held by ceil(log2 k) extra integer
bits, fraction width untouched, so an additive chain of any practical length is composite-exact in
a fixed, small accumulator. A chain of k multiplies exactly held needs the sum of all the fraction
widths, kF bits for uniform F: linear in depth. So for multiplication the ever-growing
intermediate is mathematically real, and holding a whole multiplicative chain exactly is not a
design worth wanting, because one boundary adaptation at the end discards almost everything the
width paid for.

The resolution is the **window**: a bounded subterm whose exact-result width fits the container,
evaluated entirely in the width algebra (grade s inside), adapted once at its exit. A chain then
factors into windows, and the schedule's granularity becomes one adaptation per window rather than
one per op. The fused multiply-add is precisely the two-op window; probe A's wide dot product is
the fold window (products exact at 2F, sum exact with log-k headroom, one narrow); a k-multiply
chain at container width W supports windows of roughly W/(I+F) factors. The cost accounting is
then clean: a per-op schedule pays k adaptations and constant width; a windowed schedule pays
ceil(k/w) adaptations and width w·(I+F); the fully exact schedule pays one adaptation and width
linear in k, which only merits paying when the chain is short. And the window capacity is a
**static function of the container width and the operand formats**, which means it is derivable by
the same typestate the acceptance criterion already demands: the consumer expresses usage, the
typestate derives the container, and the same derivation yields the largest exact window. Nothing
new has to be bolted on for chains; the derivation machinery is the same machinery pointed at a
subterm instead of a value.

So the direct answers: yes, a chain of multiplies is writable without an ever-growing
intermediate; the price is one adaptation per window; the window bound is not a tuning knob but a
derived quantity; and the drift of the whole chain is counted in windows, not in ops.

## 6. Order, threads, and the intent interaction I did not go looking for

I10 says arvo takes no stance on how many cores it runs on. I7 says the accuracy-first intent is
precise especially within chains. These two interact through the schedule, and the interaction is
a derivation rather than a preference: a parallel reduction is a reordering and re-association of
a fold, so a fold whose schedule is order-sensitive computes different values at different thread
counts. Probe B shows both failure and both escapes concretely: the per-step saturating fold is
order-dependent (two distinct values across the six orders of one three-element multiset), the
stepwise f64 sum is order-dependent (the classic 1e16 + 1 − 1e16 gives 0 or 1 by order), while the
wide-exact-then-adapt-once fold and the wrapping fold are order-independent, the first because
nothing rounds until nothing remains to reorder, the second because mod 2^n is a ring. Whichever
strategy ends up claiming chain precision, or run-to-run reproducibility across thread counts, is
therefore pushed toward the order-independent schedules by the two intents jointly, not by
anyone's taste. In my own field this is the oldest reproducibility problem there is, the parallel
sum that differs run to run; it is pleasant to find it sitting at the bottom of a numeral canon
question.

## 7. What the format concept must carry for a chain claim to be statable

The question allowed "nothing" as an answer, chains living at a different layer. My derived answer
is: **not nothing, but small, and precisely three things.** Chains themselves, the terms, the
schedules, the windows, the error sums, do live at a higher layer, and should. What that layer
consumes from the format concept is:

1. **The width algebra of exact results.** For each op, the format its exact result lands in,
   stated as a relation between formats: multiply adds both widths, add takes one integer bit, a
   fold of k takes ceil(log2 k). Probe C1 shows this is statable as trait contracts on the pinned
   nightly with no forbidden features, the solver chaining (2,6) × (3,5) × (1,7) to Q<6,18>
   through associated Out types, verified by a function that only accepts Q<6,18>. Probe C2 is the
   committed compile failure showing the general one-impl-for-all-widths spelling is refused
   without `generic_const_exprs` ("generic parameters may not be used in const operations", four
   counts, `p_c2.stderr`), so the accepted shapes are bounded enumeration or a type-level
   arithmetic contract per the refused-bound rule. I note the coincidence and decline to call it
   one: the windows are bounded by the container anyway, so the feature ban and the mathematics
   ask for the same finite shape.
2. **The adaptation as a first-class, named map**, not an anonymous step fused invisibly into
   every op. It carries its rounding rule and its overflow policy (the two sides from section 2),
   and its error contract in one statement covering the whole family: |ρ(x) − x| ≤ ½ ulp_F(x),
   with ulp the format's granularity function, constant 2^(−F) for fixed point,
   magnitude-scaled for floats. Every chain error bound in section 3 is a sum of granularities at
   adaptation points; the granularity function is the single object both halves of the family
   contribute to that sum. If the adaptation is not named, the schedule is not expressible, and
   with it goes every claim in this file.
3. **The exactness predicate**: the conditions under which an op or an adaptation loses nothing.
   Fixed point's add and multiply within the width algebra; widening adaptations; Sterbenz
   subtraction for floats; and the float error-free transformations (Knuth's 2Sum, Dekker's
   Fast2Mult), which are the float analogue of the widening multiply, the exact result held in a
   small compound format. The predicate is what lets windows exist on the float side too, and it
   is what a compiler or typestate consumes to prove a subterm drift-free without analysis.

The counter-direction must be stated to be tested: a format concept that closes its operations
over the format, F × F → F with the adaptation hidden inside each op, can state grade b and
nothing above it. Grade a, "matches the once-adapted exact answer", quantifies over an
intermediate value the closed concept cannot name, so under a closed concept I7's chain clause has
**no expressible form**. That is a statability argument, not a benchmark, and it is the central
result of this file: op's accuracy-in-chains intent is not an optimization request, it is a
constraint on the shape of the format concept itself. Either the concept exposes exact
intermediates through the width algebra and the named adaptation, or the intent cannot be written
down, let alone met.

## 8. What the chain lens says about the strategy axis, offered as fits, not rulings

The strategy set is open (I1), so these are readings against intents, each of which a live option
elsewhere may fit or refuse.

Through the chain lens the strategy axis decomposes into three sub-axes: the per-op rounding rule,
the schedule policy, and the intermediate width policy. That decomposition itself is a candidate
canon statement, because it makes I8's "they weigh different measurements differently" concrete
for chains: strategies can share ops entirely and differ only in schedule.

The native-behavior intent (I3) pins its strategy to the **stepwise** schedule almost by
definition: native integers wrap per op, native floats round per op, and a programmer writing the
obvious code writes grade b. Note what probe B adds: native wrapping is order-independent, so
"behaves like native ints" is more reproducible under reordering than a saturating alternative,
which is not the intuition most people carry. The accuracy intent (I7) reads as windowed composite
exactness where the container affords it and stated bounded drift where it does not, since grade a
per window is the strongest claim that does not require unbounded width. The performance intent
(I5) is free to truncate, to skip guarantees, and to trade even grade b away where a measured gain
exists; through this lens Hot's chain story is "the emptiest schedule that measurement justifies".
The storage intent (I6) barely participates in chains at all, and that is the finding: a
storage-optimized format's chain story is adapt-on-entry, compute elsewhere, adapt-on-exit, which
quietly suggests the format concept distinguish **storage formats from compute formats**, with the
entry and exit adaptations explicit members of the schedule. "It's just sitting basically" is a
format that is never an edge inside a window.

## 9. Directions left open, and what would distinguish them

Per the standing mode, nothing above is a settlement. The live directions as I see them cold:

**D-A, closed ops, chains entirely elsewhere.** The format concept says nothing about chains;
every claim lives in algorithm crates. Strained hard by the section 7 statability argument: I7's
chain clause has no expressible form against it. What would reopen it: op clarifying that the
chain clause of I7 means only "smaller per-op error, which happens to help chains", a reading his
quoted words ("especially within chains and ops, not only alone") do not favor but which only he
can rule on. I do not kill this direction on my own authority; I report that under it a stated
intent is unstatable, which the panel and op can weigh.

**D-B, the three-carrier concept.** Width algebra, named adaptation, exactness predicate in the
format concept; terms, schedules, windows, error sums in the layers above. This is where my
derivation lands, and probes A, B, C1, C2 are three independent instance families supporting its
load-bearing claims (schedule-is-semantics shown on the rounding side, the overflow side, and the
float side; composite exactness constructive and property-checked; statability under the feature
ban compiled and its negative spelling refused on record).

**D-C, the chain as a first-class typed object.** Expression templates: the term itself is a type,
the schedule chosen at evaluation. Everything monomorphizes, no alloc or dyn needed, and it may be
what I11's "contracts for things that compose to bigger units than just numerals alone" gestures
at. Costs are real: type sizes grow with expression size, the API surface grows a second
vocabulary, and arvo drifts from numerals toward computation graphs. Distinguishable from D-B by
asking whether any consumer needs to abstract over *schedules* at compile time rather than pick
one per call site; if none does, D-C is D-B with ceremony.

**Open regardless of direction:** whether schedules are strategy-implied defaults or
consumer-visible knobs. The toolbox rule pushes toward visible with defaults, and the window
capacity being typestate-derivable makes the knob cheap to expose honestly. Also open: whether the
storage-versus-compute format distinction from section 8 is a format-concept axis or a strategy
property; the chain lens forces the distinction to exist but does not say where it lives.

## 10. Honest coverage bounds

Probe A is exhaustive over a stated small domain (46,656 tuples; the count counts 3-element (a,b)
raw-vector pairs over a 6-value raw set at F=8, unsigned) and proves nothing beyond that domain
except by the accompanying arithmetic argument. Probe B is a single hand-built multiset plus one
classic float triple; it establishes existence (order-dependence exists, invariance exists), not
universality, though the wrapping-invariance claim is backed by the ring-homomorphism argument,
which is universal. Probe C1 demonstrates statability at three hand-picked width pairs via macro
enumeration; it does not establish ergonomics at scale, and the enumeration-versus-type-level-
arithmetic fork for the general width algebra is genuinely undetermined here. No magnitudes were
measured anywhere; every cost statement in section 5 is counting (widths, adaptation points), not
timing. I did not and could not evaluate any of this against the live option register, which is
phase two's job. Negative-operand rounding behavior of the probes' narrow helpers is unaudited, a
named shortcut. The float side of grade a (correctly rounded composites for float chains) is
asserted to be window-shaped via error-free transformations on the strength of the literature and
my own field experience, not probed here; a probe of 2Sum exactness under this toolchain would be
cheap and is a natural follow-up for whoever holds the float lens.

---

*Phase one ends here. Committed before any panel file was read. Phase two follows below after
reading `55`, `56`, `55b`, `57`, `58`, `59`, and `OPTIONS.md`, and does not edit anything above
this line.*

# Phase two: reconciliation, written after reading the panel

**Marked and appended per the dispatch. Phase one above is untouched.** Read for this phase, in the
assigned order: `55` (both phases), `56`, `55b`, `57`, `58`, `59`, then `OPTIONS.md` in full. Also
opened: `58_probes/p2_multiplicative_grade_does_not_collapse.rs` (source, to model its construction
before probing it). Not opened: `08`, `35`, `42`, `18`, `20`, `25`, `40`, `43`, `50`, `RULES.md`,
`DROPLIST.md`, `seed/`, `archive/`, and every other numbered file; every statement below about those
is sourced to the files I did read or to the register, and marked so.

One phase-two probe was built and committed before this section was written:
`60_probes/p_d_rescale_saving_is_adaptation_fusion.rs`, with its transcript. It tests a prediction
phase one's schedule algebra makes about `58`'s accumulator finding, and it answers an open item
`58` itself flagged.

## 0. The verdict first

**The core of phase one survives contact with the panel, and it turns out to be the answer to a
question the panel had already posed.** `59`'s P1 dispatched exactly this file's question, with
"done looks like: a chain-level statement that either factors through the per-operation model or
demonstrates it cannot, with the multiplicative case as the test instance." Phase one, written
blind to that dispatch note, delivers the first branch with a condition attached, and the condition
is what reconciles `58`'s structural impossibility with `55`'s standard model. Section 1.

**Reading the panel changed phase one in three places**, conceded in section 3: the additive
accumulator width (the panel's predicate is one bit sharper than mine and they are different
predicates), the same-scale premise my grade-s claims silently carried (the Q3 dependency, where I
also push back on the strength of `59`'s version), and the filing of wrap (my "overflow is the
integral side of adaptation" survives as a statement about schedules and is superseded as a filing
by the two-law-roles resolution of `55`/`56`/`55b`).

**And one finding of `58`'s is refined by measurement, in both directions at once.** Its
`min_w == full_w - F` pattern, called exact from two observations, is refuted as an equality at
n = 5 (the saving grows past F) and refuted as rounding-mode-independent at n = 3 and 4 (under
round-to-nearest-even the saving is zero). What survives is the mechanism phase one's schedule
algebra predicts: the last rescale fuses with the final adaptation exactly when the rounding rule
composes, which truncation does and nearest does not, and that is `07`'s narrowing-composition
result (already in the register) arriving at the accumulator question. Section 2.

## 1. The chain factors through the per-operation model, on one condition, and `58` is the proof of the condition

`58` section 2.2 establishes that no fixed-width eager multiply at F > 0 can supply the exactly
associative ambient operation the absorption theorem's sufficiency proof consumes, because the
rescale is baked into every pairwise step. `59` reads this as a statement about the model every
unit-two file stands on ("that model is per operation... nobody has taken it that way").

Phase one's derivation, made blind, is the reconciliation. The chain-level object is (exact ops,
edge formats, schedule), and the per-operation standard model `computed = adapt(exact)` extends to
chains **if and only if the adaptation is unfused from the operation**: the ops in the model must
be the exact ops of the width algebra (multiply into the sum of the widths, add into one extra
bit), and every narrow, rescale included, must be a separately placed member of the schedule. Under
that reading nothing `58` found is a defect of the model. The eager fixed-point multiply is not an
operation of the model at all; it is `adapt ∘ exact_mul`, an op and a schedule point wearing one
name, and `58`'s impossibility is precisely the statement that the fused spelling cannot be
re-expressed as exact composition. The model survives; the fusion does not.

The constructive half is already in this file's phase one, and it is at F > 0 where the unit's
probes mostly were not: probe A's wide arm is a multiply-containing composite at F = 8 that
achieves composite correctness (grade a) on all 46,656 inputs, because its multiplies are widening
(grade s, exact into 2F) rather than eager, and the single narrow is the schedule's only member.
That is the affirmative instance of what the register's derivation-outputs material quotes `50` as
isolating: "what survives is whether the wide product a fixed-point multiply forms is **carried**
between operations." The window is the mechanism that carries it, and the window's capacity is
derivable by the same typestate the acceptance criterion already demands.

**The vocabulary reconciliation, stated once so the next reader does not have to redo it.** Phase
one's grades and the unit's machinery snap together with no residue:

| phase one's term | the unit's term | the relation |
|---|---|---|
| grade a, composite correct rounding | the exact-then-adapt oracle in `57`/`58` | same object |
| grade b, stepwise | the eager schedule | same object |
| grade s, structural exactness | no counterpart named | exact in the **ambient** algebra: no adaptation occurs at all (widening mul, add in headroom) |
| coherence (`56`'s C-law) | the bridge: grade b coincides with grade a | `57`'s "coherence is the statement that the grading collapses", from the other end |
| the schedule | where `55b`'s two law roles act | adaptation laws face the source, coherence faces the target; the schedule is where both are placed |

Grade s and coherence are distinct and the distinction matters: wrap is coherent while reducing at
every step (exact in the **induced** algebra, per `55b`), whereas a widening multiply is exact in
the ambient one. A chain whose interior is grade s needs no coherence anywhere inside; a coherent
policy needs no widening. Those are the two ways a chain can be cheap, they are not the same way,
and a canon sentence about chain exactness has to say which it is invoking.

## 2. Probe D: the rescale saving is adaptation fusion, `58`'s equality breaks at n = 5, and the saving is rounding-mode-conditional

Phase one's section 3 says the schedule is the index set of the error analysis, and its section 5
says adjacent adaptation points are where cost hides. That predicts something specific about `58`'s
finding that the multiplicative guard saves exactly one rescale: at `w = full - F` every per-step
narrow except the last is exact, so the saving is the **fusion of the last per-step narrow with the
final adaptation**, and fusion of two narrows into one is exact precisely when the rounding rule
composes. Truncation composes; round-to-nearest does not, which is `07`'s narrowing-composition
result as the register carries it. So the prediction, registered in the probe header before
running: the saving exists under truncation and shrinks or vanishes under RNE.

`60_probes/p_d` mirrors `58_probes/p2` section 1 exactly (I read its source first; its narrows are
all truncation, so its finding was measured under truncation only), parameterises the rounding rule,
uses the same rule in the oracle so the comparison isolates the schedule, and sweeps n = 3, 4, 5 at
`58`'s own (M, F) = (15, 3), exhaustively over [0, 15]^n. Instrument checks: w = 0 must diverge and
w = full must not, for both rules; both fire correctly.

Measured (`p_d.out`):

| rule | n = 3 | n = 4 | n = 5 |
|---|---|---|---|
| truncation, saving below full_w | 3 = F | 3 = F | **4 > F** |
| round-to-nearest-even, saving | **0** | **0** | 3 |

Three results, each bounded to this one (M, F) point and this operand box:

**The fusion prediction lands.** Under RNE at n = 3 and n = 4 the saving is zero: witnesses exist
at every guard width below full (15 divergences at w = 5 for n = 3; 36 at w = 7 and w = 8 for
n = 4), which is double rounding doing exactly what `07`'s composition result says it does. The
one-rescale saving is not a fact about multiplicative folds; it is a fact about **truncating**
multiplicative folds, and any register sentence carrying it needs the rounding-rule condition.

**`58`'s "exactly one rescale, constant in fold length" fails at n = 5**, in the direction of more
slack: the truncating saving is 4 bits, and RNE recovers 3 bits of slack at n = 5 after having none
at n = 3 and 4. `58` asked for exactly this extension ("I would want n = 5 and n = 6 before
trusting the pattern") and was right to withhold trust. So there are **two mechanisms**, not one:
the fusion, worth exactly F under a composing rule and zero otherwise, and a second,
rule-independent slack that grows with fold length, which on this domain I read as the final
adaptation and clamp absorbing interior rounding differences as more products saturate or vanish
(the same absorption shape as `57`'s one-bit additive finding, grown larger). The second mechanism
is domain-dependent on its face and I have not separated it from the operand box; n = 6 is unswept.

**What survives of `58`'s claim, restated in the form I would defend:** under a composing rounding
rule the multiplicative accumulator needs at most `full_w - F` guard bits (fusion guarantees the
last rescale is free), the need still grows linearly in fold length, and no logarithmic closed form
exists. The linear-growth headline, which is the design-relevant half and the half that bounds
Q11's accumulator option to additive folds, is untouched by any of this.

## 3. What phase one concedes or refines on reading

**3a. The additive accumulator width: two predicates, and mine was the looser one.** Phase one
section 5 says a fold of k adds is exactly held by ceil(log2 k) extra integer bits. True as stated,
for holding the exact sum. `57`'s p6 measures the design-relevant predicate, agreement with
exact-then-adapt, at one bit less, uniformly, fifteen rows, and its explanation (the accumulator
decides which side of the format the result fell on, not how far outside it fell) is correct and
is absorption appearing as a width saving. `57` also warns the panel has been using one phrase for
both predicates (interior safety against adaptation agreement); phase one's wording is an instance
of the conflation and I mark it rather than editing it.

**3b. The same-scale premise, and a pushback on `59`'s version of the Q3 dependency.** Phase one's
grade-s claims (adds exact, the only adaptation points are rescales, divisions and narrows)
silently assume operands share a scale. `59` section 2b is right that Q3 is the premise nobody
read, and right that none of the unit's five files cites it. But its mechanism sentence, "alignment
is a shift, and a shift is precisely the grid coarsening", is too strong as written. In the width
algebra, aligning to the **finer** grid is a left shift, which is a widening and exact; the exact
sum of Q(I, F) and Q(I', F') lives at Q(max(I,I') + 1, max(F,F')), exactly, always. Coarsening
enters only when the result format is coarser than the finer operand, which is a fact about the
**inferred result numeral and the schedule**, not about mixedness itself. So the honest form of
the dependency: under Q3's second option, the unit's unconditional additive results survive if the
inferred result is the join (the D1-shaped, soundness-first answer the register's own material
leans toward) and become conditional exactly when the inference or the schedule narrows below the
join. The alignment does cost integer headroom (the raw value grows), which is the overflow side,
not the rounding side. One expert, my reading, offered to `59`'s P2 dispatch as a sharper target:
sweep mixed-scale addition per Q3 option **and per result-format rule**, because the option alone
does not determine the outcome.

**3c. Wrap.** Phase one filed wrap as one value of the overflow side of adaptation while separately
noting it is a ring homomorphism. The `55`/`56`/`55b` exchange did this properly: wrap fails the
adaptation laws, saturation fails coherence, all four cells inhabited, one slot with two law roles,
and the domain filing withdrawn as observationally equivalent to the composite filing. Phase one's
operational claims survive unchanged (probe B's wrap arm is an independent instance of wrap's
additive coherence at i16), and the panel's filing is better than mine; I adopt it. One small
addition phase one's framing still contributes: "the schedule places both sides" remains true under
the two-roles resolution, since a schedule position must name both its rounding rule and its
overflow member, and probe B's saturating arm shows the overflow side alone makes schedules
semantically distinct.

**3d. Independence bookkeeping, per `55`'s own discipline.** My standard-model derivation shares
literature with `55` (Wilkinson, IEEE, the exact-then-adapt factoring), so that convergence is two
instruments over one literature, worth more than a read and less than two cold arrivals. What is
genuinely independent: probe A is a composite-exactness instrument at F = 8 with a
property-checking oracle (the unit's oracles recompute; mine verifies the defining property without
rounding, so shared-bug agreement is structurally excluded), and probe B is a new width (i16) and a
new witness family for claims the unit measured at 4 bits. My cold witness (30000, 10000, -25000)
is a **ceiling-only, clamp-then-pullback** divergence, which independently corroborates the
pullback mechanism `55b` proposed and `57` adjudicated, at a different width, from a blind start:
the ceiling saturates, the negative operand pulls the sum back into the interior, and no floor is
ever touched. I did not know that mechanism existed when I built it, which is what makes it worth
one instance.

## 4. Fits against the register, per its method

**Q3.** Fits, and sharpens per 3b: the entry should record that the threat to the unit's additive
results runs through the inferred result format and the schedule, not through mixedness alone.
Kills nothing.

**Q5.** Phase one's decomposition of the strategy axis through the chain lens (per-op rounding rule
by schedule policy by intermediate width policy) fits the product-of-axes reading and fits `25`'s
"intermediate precision is a separate axis" evidence as the register carries it: my schedule axis
is that axis with its chain-level semantics (order dependence, coherence) made explicit rather
than only its precision. Fits badly with the one-axis reading for the same reason `25`'s material
does. Kills nothing.

**Q11.** Three fits. The statability argument (phase one section 7: a concept closing ops over the
format cannot state grade a, so I7's chain clause has no expressible form against it) is an
argument the register does not carry for **why** the exact-result width algebra must be nameable,
and it lands beside `47`'s proposed sentence as quoted in the register: the exact-result format of
an op is precisely a fact a lowering site cannot recompute from a const once the chain layer needs
it. The accumulator option's additive-only qualification (`58`, `59`) is confirmed from an
independent derivation (phase one section 5's linear-versus-logarithmic asymmetry, derived blind).
And probe D adds the rounding-rule condition to any accumulator sentence: the multiplicative
guard's headline is rule-conditional in the fusion term.

**Q12.** Fits the specify-the-shape and per-strategy options; phase one's I10-plus-I7 derivation
(a parallel reduction is a reorder, so chain precision plus core-agnosticism jointly force
order-independent schedules for whichever strategy claims both) is the argument form of what the
entry's second option gestures at ("deterministic at any thread count, which unblocks the
adaptation intent"). Probe B contributes independent instances at i16: saturating order dependence,
wrap invariance, wide-then-adapt invariance, and the f64 arm as the float instance of the same
taxonomy. Kills nothing.

**Q14.** Fits `58`'s candidate input and gives it vocabulary: the choice for the accuracy strategy
on product chains is between grade a per window with a stated bound across windows, and an
unbounded accumulator. Phase one's window framing is the form in which that choice can be put to op
without asking him a measurement question.

**Q16.** The chain, the window and the schedule are sense-two objects (aggregates over numerals,
binding-time material per `43`'s finding as the register carries it). Whatever word wins sense two
needs to be usable for "a subterm evaluated in the width algebra and adapted once", because that is
the unit the error analysis and the accumulator derivation both index on.

**Q17.** Two contributions. Probe A is a composite-exactness result **at F = 8** for a
multiply-containing window, the constructive complement of `58`'s negative result about eager
multiplication: at F > 0 the safe multiplicative shape is the widening window, and it is exactly as
composite-exact as the additive one. And probe D refines the entry's accumulator row as in section
2: saving at least F under truncation with the equality failing at n = 5, zero saving under RNE at
small n, linear growth untouched.

**The wrapping entry.** Fits as amended; probe B adds an i16 instance of wrap's additive coherence.
Nothing killed.

**The derivation's outputs.** The window mechanism is the operational content of "the wide product
is carried between operations" (`50` as quoted in the register), and the storage-against-compute
format distinction phase one derived from Cold's chain story (section 8) converges with the
compute-carrier material already there (`47`'s Warm-against-Precise compute difference, as the
register carries it). Mine is another arrival, not a second read.

**Kills nothing anywhere.** No option in the register is closed by anything in this file.

## 5. What I put to the others, for the resumption

**To `58`.** Probe D confirms your linear-growth headline and your withheld trust in your own
two-point pattern, refutes the "exactly F, constant" form at n = 5, and makes the saving
rule-conditional: zero under RNE at n = 3 and 4. Two questions. Would you restate the finding as
"under a composing rounding rule, at most `full_w - F`; under nearest, no fusion term at all", with
the second slack mechanism (final-adaptation absorption, growing with n) named separately? And your
open question about whether the statistical-error-bound alternative is expressible in the typestate:
phase one's window framing suggests the *statable* form is per-window exactness plus a per-window
error count (the schedule is the index set of the error sum), which is a counting contract rather
than a probabilistic one; does that meet what you meant, or is the DSP-style expected-error bound
genuinely what I7 needs?

**To `57`.** Your "coherence is the statement that the grading collapses" and phase one's "the
schedule is part of the function's meaning" are one frame from two ends; the table in section 1 is
my proposed joint statement. Does the grade-s row (ambient exactness, no adaptation at all) earn a
place in your grading as the degenerate case where the question of collapse does not arise, or do
you fold it into coherence and lose the distinction between wrap's induced-algebra exactness and a
widening op's ambient exactness?

**To `59` (the persona) and whoever dispatches P2.** Per 3b: the Q3 sweep should vary the result
format rule, not only the Q3 option, because alignment to the join is exact and the coarsening
threat enters only below the join.

## 6. What the register should gain

Reported; I have edited neither `OPTIONS.md` nor `INTENTS.md`.

**Q17's multiplicative accumulator row** gains probe D's counts: the one-rescale saving is a fusion
term, present under truncation (and by the fusion argument any composing rule), absent under
round-to-nearest at n = 3 and 4, exceeded at n = 5 by a second growing slack; `58`'s equality form
should not be carried unconditioned. Cross-link to the narrowing-composition entry ("does the
design want narrowing to compose"), which now has an operational consequence in the accumulator
material rather than only a conversion-layer one.

**Q17** also gains probe A as an F > 0 composite-exactness instance for the widening
multiplicative window, the constructive complement to `58`'s eager impossibility.

**Q3** gains the sharpening of 3b: what depends on it is conditional on the inferred result format
and the schedule, not on mixedness alone.

**A candidate line for the chain question `59` P1 posed**, offered as one cold derivation plus this
reconciliation, one expert: the per-operation model extends to chains exactly when adaptation is
unfused from the ops; a chain is (exact ops, edge formats, schedule); the schedule is the index set
of both the error analysis and the strategy divergence; chains factor into windows whose capacity
the existing typestate derivation already determines; and a format concept that hides the
adaptation inside each op cannot state I7's chain clause at all.

## 7. Coverage, bounded honestly

**Read in full:** the six files and the register, as listed at the top. **Read at source:**
`58_probes/p2` (construction and narrows). **Not re-run:** any panel probe; probe D re-derives
`58`'s truncation rows independently rather than diffing its outputs, and its n = 3 and n = 4
truncation savings agree with `58`'s reported pattern, which is a cross-check though not a byte
diff. **Built this phase:** one probe, exhaustive within its stated domain, instrument-validated
both ways, committed with its transcript.

**Bounds on probe D:** one (M, F) point, unsigned operands, left fold, no intermediate clamp
(deliberately mirroring `58` section 1), n up to 5, two rounding rules. The two-mechanism reading
of the n = 5 excess is an interpretation; separating the absorption slack from the operand box
needs a sweep over M and the box that I did not run. The claim that any composing rule fuses
(directed modes, not only truncation) is argued from `07`'s result as the register carries it, not
measured here.

**First-read here, owed seconds:** the unfused-model reconciliation (section 1, including the
grade table); the fusion reading of `58`'s saving with probe D's counts; the Q3 sharpening (3b);
the ceiling-only reading of probe B's witness as a pullback instance (3d). **Seconded by me, from
independent instruments:** the pullback mechanism (probe B, i16, blind); wrap's additive coherence
(probe B); the exact-then-adapt fold's order invariance (probe B); the additive-only bound on
Q11's accumulator option (phase one section 5, derived blind, agreeing with `58`).

**Did reading the panel change the answer?** The core, no: a chain is composition plus a schedule
of adaptation points, the schedule is semantics, exactness has grades, chains factor into windows,
and the format concept owes the width algebra, the named adaptation and the exactness predicate.
That was written blind and it stands, and saying so is the result this dispatch exists to produce.
The refinements, yes, and they are real: one bit off the additive accumulator, a premise named on
the same-scale claims, a better filing for wrap, and a sharper, measured form of the multiplicative
saving than either phase one or `58` had alone.

**Nothing here settles anything.** The mode is explore, there is no canon, and every first-read
item above is one expert until attacked.
