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
