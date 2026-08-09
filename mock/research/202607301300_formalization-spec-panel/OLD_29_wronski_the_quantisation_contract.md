# 29. The quantisation contract, and the sequence it was never asked about

**Member:** Bart Wronski. Signal-processing lens: sampling, reconstruction, quantisation, dithering and
noise shaping, and specifically the gap between what a rounding rule looks like on paper and what it
does to a signal over many samples. I have not read any other file in this round; per the brief my
question sits in `26_consolidation_two.md` and the two current-focus files, `27` and `28`.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: every crate green,
zero failures (`cargo test --workspace 2>&1 | grep -E "test result:|FAILED"` returns 37 zero-test
green results, a spread of small green counts, and one `12 passed; 0 failed; 1 ignored` line, no
`FAILED` anywhere), consistent with files 27 and 28's counts of the same suite (the ignore is the
catalogued divide gap, `crates/arvo/tests/fixed_point_div.rs:111`, tracked #5). I read
`crates/arvo/tests/identity_laws.rs` in full: it is the honest full-matrix shape its own module doc
describes (lines 1 to 20), and I grepped it and the rest of the tree for anything resembling
dithering, noise shaping, or a stochastic mechanism
(`grep -rln "dither\|noise_shap\|stochastic" mock/crates/`) and found nothing outside this round's own
files. The subject of my question is unbuilt in every direction; nothing here contradicts shipped
code because none exists to contradict.

**What I read:** `26_consolidation_two.md` in full; `27_carmack_what_a_number_is.md` in full;
`28_leroy_what_identity_must_express.md` in full; the `Quantisation` contract and the faithfulness
derivation directly in `mock/design_rounds/202607301200_topic.the-formalization-spec.md` (lines 40 to
230), to check the exact wording rather than trust a summary of it. `ls`'d the panel directory once;
fetched nothing else.

**What I compiled:** two probes in `29_probes/`, both `#![no_std]`, `const`-only, compiled with the
workspace's pinned nightly, each with a negative control that fails to compile
(`29_probes/OUTCOMES.md`). **What I measured:** nothing; no bench exists for anything proposed here,
and nothing here is offered as a bench result. **Everything else is reasoned**, and where I reason
from established results in my own field (dither theory, delta-sigma noise shaping, the classical DSP
literature on limit cycles) I say so and do not present the citation as something checked against
arvo's tree.

## 0. What the standards actually do, and what a converter does, checked against the spec text

Before the substance: the design's own quantisation vocabulary is at
`202607301200_topic.the-formalization-spec.md:125 to 157`. Five situations, a `Direction` for the
three in-range ones, a `Resolution` for the two out-of-range ones, no state anywhere (`Resolution` and
`Direction` are unit structs, section 1 of that file's trait declarations carry no data), and no
randomness anywhere (nothing in the trait or its sixteen instances takes an argument beyond the exact
value). File 28 already found the classification order is backwards against every one of the three
test standards (section 4, `28:205-258`, probe-verified) and proposed the fix: round on the unbounded
grid, then classify the rounded result against the range. I checked this independently against a
fourth source the panel has not cited, an analogue-to-digital converter's own textbook description,
and it agrees exactly: a converter's transfer function is a staircase defined over the ENTIRE input
range including saturation, computed as one function, not as a value-dependent dispatch between "round"
and "clip" as two separately-triggered code paths. `202607301200:118 to 120` already reaches for this
exact framing ("An analogue-to-digital converter quantizes and clips, and both are quantization") and
then the vocabulary built on top of it classifies the wrong argument anyway. I have nothing to
overturn in file 28's fix. I take it one step further, because the round-first amendment is not only a
bug fix. It is the load-bearing precondition for the entire subject of this file to be expressible at
all, which section 2 states precisely.

## 1. Error which is uncorrelated is worth more than error which is small

State the field's central, least intuitive result once, because everything below is an application of
it. A rounding rule that minimises the magnitude of each individual error, sample by sample, is not
thereby minimising the perceived or measured damage to the signal as a whole. What is damaging is
STRUCTURE in the error: a pattern that repeats, that tracks the input, that a downstream eye, ear, or
filter can lock onto. A larger error with no structure reads as noise, which every perceptual and
statistical system already has machinery to discount. A smaller error with structure reads as an
artefact, because structure is exactly what perceptual and statistical systems are built to detect.

`202607301200:105-108` already names the case where this bites hardest in a concrete, checkable way:
`FullRange<F>` exists to make UNORM expressible, and D61's own motivating use is an 8-bit colour
channel. Quantise a smooth linear gradient into `FullRange<8>` with any `Direction` in the current
vocabulary, nearest, toward zero, to even, and the error at each pixel is a deterministic function of
that pixel's position alone (probe 1 shows this precisely: two positions with the same value modulo
the quantum receive the identical error, always, from every `Direction` the contract can express,
because a `Direction` is a pure, memoryless function of the exact value and nothing else). The result
is banding: visible steps where the eye sees discrete bands of colour instead of a smooth ramp, and no
choice among the five positions removes it, because the five positions are a complete vocabulary for a
different, narrower problem than the one that produces the artefact. They answer "which representable
value is closest", which is the wrong question for a SEQUENCE of quantisations of a smoothly varying
signal. The right question is "how does the error correlate with the input across the sequence", and
nothing in `Numeral`, `Policy`, or `Lowering` as currently specced has a slot for that question, because
none of the three is a fact about a sequence at all. They are facts about one value.

This is the shape every thread below returns to. The design's quantisation vocabulary is necessary and,
within its own scope, close to complete (files 27 and 28 have found real holes in it, and I add one
more in section 2). It is also answering a strictly smaller problem than "how should a value be
mapped onto the representable set", because that larger problem, asked honestly, is a claim about a
whole computation, not about one map application, and the design has not yet said where that claim
lives.

## 2. The round-first fix is a precondition, not only a correction

File 28's amendment: "round on the unbounded-exponent extension of the grid by the direction triple,
then classify the rounded result against the range and resolve by the range rules" (`28:235-238`).
Read this against what a dithering quantiser actually is, and the amendment gains a second job it was
not written for.

A dithered quantiser is not a different rounding rule. It is the SAME rounding rule, applied to a
DIFFERENT input: the exact value plus an independent perturbation, added before rounding, on the
unbounded grid, with no special-casing near the range's edges (dither has no opinion about overflow;
it only ever touches the in-range rounding decision, and where it pushes a value past the top the
ordinary `OverRange` resolution takes over exactly as it would for any other value that landed there).
That is precisely the extended-grid input the round-first amendment introduces to fix the classify-order
defect. Before the amendment, "classify first" sorts the UNPERTURBED exact value into one of five
positions and only then rounds; there is no coherent point in that pipeline to inject a perturbation,
because the classification has already happened by the time the rounding step would receive one. After
the amendment, the rounder is a single function from the extended grid to the representable set, and
feeding it `exact + noise` instead of `exact` costs nothing new: same function, same five positions,
different input.

Concretely, and buildable today with no change to the axis table:

```rust
// the amended contract, per 28's fix, stated as the entry point a
// composition actually calls
fn quantize<T>(exact: ExactValue) -> Self::Fallibility<T>;

// same rounder, one extra argument, no new axis, no new Resolution, no
// change to Direction. arvo never generates `noise`; the caller does.
fn quantize_dithered<T>(exact: ExactValue, noise: ExactValue) -> Self::Fallibility<T> {
    Self::quantize(exact + noise)
}
```

Probe 1 checks the mechanism this buys: two call sites whose exact values share a residue class
modulo the quantum (3 and 23 at quantum 10) receive the identical error under the undithered path
(both -3, the banding fact stated as arithmetic) and different errors under the dithered path (7 and
-3) when two different externally supplied noise samples are added before rounding
(`29_probes/probe_1_dither_breaks_the_residue_correlation.rs`, compiled, negative control fails to
compile). This is a narrow, honest claim. It shows the map STOPS being a pure function of the exact
value's residue once an extra input exists. It does not show statistical independence between error
and signal across an ensemble of noise draws, which is the stronger claim the literature actually
proves for specific noise distributions (section 5) and which needs a real noise source to check, not
a two-point probe.

The reason this belongs in section 2 rather than section 5: file 28 priced the round-first amendment
as "one sentence replaces one sentence" (`28:236-238`) and stated its payoff as overflow-to-infinity
and `ReduceModulo`'s well-definedness. Both are real. The amendment's third payoff, that it is the only
place in the whole ten-axis design where dither could ever be wired in without inventing a new
mechanism, was not among the reasons given for making the change, and it is the strongest one I can
offer, because everything my field knows about quantisation depends on rounding being a function of an
extended input, and the pre-amendment contract did not have one.

## 3. Three mechanisms, three homes, and the design currently has room for zero of them

The consolidation records, correctly, that stochastic rounding was "noted and set aside" and that
first-order error feedback "needs state, so it belongs in a combinator's accumulator object rather
than in the type-level policy, for the identical reason stochastic rounding is already excluded from
the design (resolution constructors are pure ZSTs)" (`26_consolidation_two.md:295-299`). This is
correct as far as it goes and conflates two things that need to be told apart, because they need
different mechanisms and, I will argue in section 4 and 5, different homes.

There are three distinct tools in my field for the same underlying complaint, correlated quantisation
error, and they work by three different mechanisms:

**A direction (the design's existing vocabulary) controls bias, per value, with no memory.** It can
make the mean error zero across an ensemble of DIFFERENT inputs (round-to-even is exactly this), but
it cannot touch the error's dependence on any ONE fixed input, because a `Direction` is by
construction a pure function of the exact value alone. Feed it the same residue twice, get the same
error twice, forever. This is not a limitation of a particular `Direction`; it is what memorylessness
means. No cleverer choice among `TowardNegative`, `ToEven`, or any direction the vocabulary could add
fixes this, because the defect is not in which direction is chosen, it is in the shape of a pure
function.

**Error feedback, or noise shaping, redistributes error energy across the sequence without removing
its correlation with the input.** Carry the previous step's undelivered residual forward, add it to
next step's exact value before rounding (exactly probe 2's `shape` function), and the cumulative error
across the fold is bounded, in the simplest, first-order case, within one quantum forever, matching
the consolidation's own finding (`26:296-299`). What that description undersells is WHY this matters
beyond the bound: what error feedback actually does is push the error's spectral content toward high
frequency (in a spatial signal, toward high-frequency detail; in a temporal one, toward frequencies a
downstream low-pass stage or a human ear is least sensitive to). It is still fully deterministic; feed
the same sequence twice and the shaped output is identical both times. It is the mechanism behind
Floyd-Steinberg error diffusion in image dithering and every delta-sigma converter in modern audio and
RF hardware, and every one of those systems carries state. It cannot be a `Policy` axis, because
`Policy` is keyed on the composition alone (D54's own sorting test), and a shaper's behaviour is keyed
on the SEQUENCE, not the type.

**Dither adds an independent perturbation before rounding and is the only one of the three that can
make the error statistically INDEPENDENT of the input**, at the cost of raising the noise floor. This
is the field's oldest and best-quantified result (Lipshitz, Vanderkooy and Wannamaker's 1984 survey is
the standard citation; I know this literature first-hand and am citing it as established theory, not as
a claim checked against arvo's tree). Rectangular-PDF dither (a single uniform random value across one
quantum) decorrelates the MEAN of the error from the signal, removing the harmonic distortion that
banding is: the classic visible steps become an unstructured noise floor instead. It does not fully
decorrelate the error's VARIANCE from the signal, so the noise floor itself still faintly tracks the
input (audible or visible as noise modulation). Triangular-PDF dither (the sum of two independent
rectangular draws, or one triangular draw of twice the amplitude) removes that too, decorrelating both
first and second moments, at a further, well-quantified, fixed cost in noise-floor level. This needs a
real noise source: pseudo-random is the practical norm, but the theorem is about the noise's
statistical properties, not about where it came from, and arvo cannot own that source under the
constraint I was given and, independently, under the design's own stated reason `Resolution`
constructors stay ZSTs (`26:299`). Probe 1 is deliberately not a claim about this stronger result; it
checks only that the extra-input mechanism exists and does something, which is the prerequisite for
the stronger, ensemble-level claim to ever be checkable at all.

The design's current vocabulary has exactly one of these three (direction), zero mechanism for the
second (state, keyed on a sequence, nowhere in `Numeral`, `Policy`, or `Lowering`), and zero mechanism
for the third (an extra pure input, which section 2's amendment happens to make free but which nothing
in the contract as written today calls out as a place a caller could reach). None of the three is a
missing `Direction` variant. All three are missing at a structural level: the contract has no concept
of a sequence at all, and two of the three tools genuinely only make sense applied to one.

## 4. Where shaping lives, and what it costs the design to offer it

Section 3's error-feedback mechanism needs a home, and the consolidation's own reasoning from a
different corner of the design already supplies the right shape for it, which is worth stating
explicitly because it means nothing new has to be invented to place it correctly.

`26:125-137` establishes that regrouping licences live with the specific combinator that performs the
regrouping, not with the type: "a law is required only by the specific combinator that performs a
regrouping... and that combinator states the fact it needs; code that never regroups states nothing
and refuses nothing." Shaping is the same shape of fact, restated: it is a property of the specific
combinator that threads state across a fold, not of the numeral or the composition. The trait belongs
beside the combinator, not inside `Quantisation`, for the identical reason a fold's regrouping licence
belongs beside the fold rather than inside `Policy`.

`26:419-441` independently reaches the same classification from the fidelity side: `Fused` (an exact,
deterministic, one-answer algorithm arvo writes itself) belongs in the design "as a distinct named
operation... a `Lowering`-shaped cost fact", while `Contract` (a genuine permission, either answer
acceptable) is the residue that cannot be expressed as an ordinary function at all. Error feedback is
squarely on the `Fused` side of that split: it is a specific, deterministic, well-defined algorithm
(carry the residual, add it next step, subtract what was delivered), not a permission that any answer
in an envelope is acceptable. So it is a named operation with state, not a `Policy` axis and not a
fidelity licence either.

Concretely, matching probe 2's compiled shape:

```rust
/// A feedback kernel threaded through a strictly sequential fold. Not a
/// `Policy`: nothing here is keyed on the composition alone, because the
/// state is a property of a SEQUENCE of quantisation events, not of a value.
pub const trait ErrorShaper {
    type State: Copy;
    const INIT: Self::State;
    fn shape(state: Self::State, exact: ExactValue) -> ExactValue;
    fn update(state: Self::State, exact: ExactValue, delivered: Value) -> Self::State;
}
```

Probe 2 compiles a `FirstOrderFeedback` instance under `const_trait_impl`, `#![no_std]`, no
allocation, and threads it through a `const fn` fold, five steps of a sub-half-quantum constant input
(quantum 10, input 2). Plain per-step rounding drifts by a full quantum at five steps
(`PLAIN_TOTAL_ERROR == -10`, an independent five-step reproduction of the consolidation's DC-ramp
finding, `26:288-292`, not merely a citation of it). The shaped fold, identical inputs, identical
rounder, returns total error to zero and returns the carried state to zero as well
(`SHAPED_TOTAL_ERROR == 0`, `SHAPED_FINAL_STATE == 0`). This is the complementary answer to the
interior-safety fix (`26:149-164`), not a competitor to it: interior safety keeps a fold lawful by
widening the accumulator until no intermediate node can leave the numeral's range, which costs bits;
shaping keeps a NARROW accumulator's cumulative error bounded by carrying one word of state and
paying one subtract per step. Real DSP silicon offers both, chosen by the same budget tradeoff arvo
already documents for everything else (`arvo-toolbox-not-policer.md`): widen when bits are cheap and
latency-critical paths cannot afford the extra state and branch; shape when the accumulator width is
the fixed, non-negotiable constraint (an embedded codec's register file, a fixed audio bit depth) and
one word of carried state is cheaper than the silicon a wider accumulator would need.

**What shaping costs the rest of the apparatus, stated plainly because nobody should discover it by
surprise later:** a shaped fold cannot be reassociated. Section 1.4's entire regrouping-licence
machinery, the recovery-map classification, the interior-safety threshold, the bench-validated
four-way accumulator split that beats one accumulator by roughly 2x on a single thread
(`26:140-142`), all of it presumes a pure per-element `phi` and a combinator free to choose an
evaluation order. A shaper's `update` step reads the PREVIOUS step's state, which is a strict
sequential dependency by construction (probe 2's `shaped_fold` cannot be split into independent
partial folds and recombined the way `plain_fold` trivially can; the recombination is itself an open
design question with no free answer, not a detail to paper over). This is not a defect in the shaping
mechanism. It is what shaping IS: trading parallelism for spectral shape, which is exactly the
tradeoff real hardware makes when it chooses a delta-sigma converter (fully sequential, one sample at
a time, exquisite noise shaping) over a flash converter (fully parallel, no shaping at all). The design
should say this where it introduces shaping: a composition that carries an `ErrorShaper` forfeits any
`AddAssoc`-style regrouping licence on that fold, and the two should not be offered together on the
same accumulator without the conflict being visible at the type level, ideally as a refusal rather
than as silently wrong output from a scheduler that regrouped a sequential dependency because nothing
told it not to. I have not designed that refusal mechanism; I am naming that it is owed, in the same
spirit section 1.6 of the consolidation already applies to every boundary the design stops short of.

**One honest constraint on how far this generalises.** Real delta-sigma hardware uses shaping orders
higher than one (MASH structures, second- and higher-order noise transfer functions) for steeper
out-of-band pushing, at the cost of stability: a poorly designed higher-order feedback filter can make
the shaped error GROW without bound rather than stay contained, which is a real, well-documented
failure mode in that literature, not a hypothetical. Under this workspace's constraints (no dependent
types, no `generic_const_exprs`, monomorphisation only), a fully generic "arbitrary filter order N"
shape is the wrong reach: it either needs const-generic array arithmetic in a position the workspace
has already forbidden (`26:719-725`'s droplist entry on exactly this class of construction), or it
needs a stability PROOF per filter, which is a much larger undertaking than this file is proposing.
The buildable, honest version is a small, closed set of named shaper markers (`FirstOrderFeedback` as
shown, and at most one or two well-known, individually-verified higher orders if a real consumer ever
needs them), the same closed-vocabulary discipline the design already uses for `Direction` and
`Resolution`. Do not build a generic order-N shaper. Ship the ones that are provably stable and name
them.

## 5. Where dither lives, and the one place my field's answer does not fully transfer

Section 2 already gives the mechanism (`quantize_dithered`, an extra pure argument riding the
round-first amendment's extended grid) and the load-bearing property that makes it fit the substrate's
constraints without friction: **arvo never generates the noise.** The caller supplies exactly one
extra value per call, arvo adds it and rounds, and the entire function stays const-callable, stateless
on arvo's side, and compatible with "no source of randomness in the substrate" by construction, not by
policy. This is the sharpest correction I have to the consolidation's framing at `26:295-299`: it is
right that arvo cannot own an RNG, and right that a stateful shaper does not fit a `Policy` ZST, but it
reads "stochastic rounding" as one problem when it is two. A shaper needs state that PERSISTS across
calls, which genuinely cannot live in a pure marker type. Dither needs no persistent state on arvo's
side at all; it needs exactly one more argument at the call site, which is a strictly smaller ask than
the consolidation credited it, because the randomness the mechanism needs was never going to be arvo's
to supply anyway.

Where the interface point should live, concretely: not a new axis on `Quantisation` (a dither amplitude
would have no honest default, per `arvo-toolbox-not-policer.md`'s own standing rule against arvo
guessing a workload-shaped constant), but a second entry point sitting beside `quantize`, taking the
noise as an ordinary parameter of the numeral's own extended-grid type. A consumer who never dithers
never sees it and pays nothing; a consumer who does (a colour pipeline quantising a gradient into
`FullRange<8>`, an audio codec truncating to a fixed bit depth) supplies its own noise, at whatever
distribution it has decided to pay for.

**This is also where the design's Stage G boundary work already built the right crossing mechanism,
and it deserves a fourth worked example alongside the three the consolidation names (`26:476-495`).**
A blue-noise or ordered-dither pattern (the standard graphics-pipeline answer to the exact UNORM8
banding case D61 motivates) is inherently spatial: it assigns a distinct noise value per PIXEL,
conditioned on a 2D or N-D neighbour topology arvo's scalar `Number<N, S>` type knows nothing about and
has no business knowing about. That topology belongs to whichever consumer owns the raster domain
(the workspace names one, a colour-and-raster crate downstream of arvo). The honest division of labour:
arvo ships the scalar mechanism (`quantize_dithered`, one extra input, zero state, works today), and a
raster-domain consumer builds the spatial pattern generator on top of it, exactly the "new, shallower
entry point closer to where the composition is still concrete" move consolidation's option 3 already
licenses (`26:489-495`), instantiated here as: arvo's scalar dithered-quantize is the concrete entry
point; the raster crate is the shallower caller that knows the topology and feeds it. Nothing about
this needs arvo to grow a build harness, an image-space concept, or a neighbour-aware anything. It
needs one extra argument on one function, already free from section 2's amendment.

**Where my field's answer does not fully transfer, stated honestly rather than glossed over.** Every
result I have cited (rectangular versus triangular PDF, the decorrelation guarantees, the
noise-floor cost) is proven for a UNIFORM quantiser: a constant quantum, the fixed-point case. arvo's
float branch has a quantum that varies with the exponent (`Stored`/`Ranged`, `26:643-652`, file 27's
own inversion proposal keeps this true even after the identity rewrite). Scaling the dither amplitude
to the LOCAL quantum (proportional to `radix^exponent`, which a caller can already read off the
numeral) is the obvious generalisation and costs nothing structurally, since it is still one extra
argument to the same function. Whether the classical decorrelation theorems still hold, in the same
strong sense, for a signal that crosses a binade boundary mid-sequence, where the effective quantum
itself changes between one sample and the next, is a genuinely open question in my own field as far
as I know it, and I do not have a citation I trust enough to assert an answer either way here. This is
worth flagging precisely because it is the one place in this file where I am not simply porting an
established result: fixed-point dithering is a solved problem: floating-point dithering across
variable quanta is not one I can certify, and a design that ships the mechanism should say so rather
than imply the fixed-point guarantees extend for free.

## 6. What this does and does not ask of the ratified calls

Nothing here overturns a D-numbered call. The two open threads I extend are file 28's round-first
amendment (section 4 of `28`, itself not yet ratified, open per the consolidation's own framing) and
the consolidation's stochastic-rounding-excluded note (`26:295-299`, a reasoned aside, not a call). My
own addition is a new sibling category, not a new axis on any of the three existing contracts: a
sequence-scoped shaping mechanism living with the combinator that folds, matching the design's own
established rule that regrouping licences live where the regrouping happens (`26:125-137`), plus a
zero-state extra-argument entry point for dither that the round-first amendment already makes free.
Both are additive. A consumer who never shapes and never dithers writes nothing new and pays nothing,
exactly the shape section 6 of file 27 already asks of any new mechanism.

## 7. Summary of proposals, and what each costs

1. **Take file 28's round-first amendment as load-bearing for more than the classification-order
   fix.** It is the only place in the design where a caller could ever inject a pre-rounding
   perturbation without inventing a second rounding pipeline. State this as a stated consequence of
   the amendment, not merely as a fortunate side effect. Cost: none beyond what file 28 already
   priced; this is a framing addition, not a new mechanism.
2. **Add a zero-state, extra-argument entry point for dither, beside `quantize`, never as a new
   `Quantisation` axis.** `quantize_dithered(exact, noise)` calling the same rounder on
   `exact + noise`. arvo never generates `noise`; the type is the same extended-grid value the
   round-first amendment already introduces. Probe-verified mechanism (`29_probes/probe_1`). Cost:
   one function per composition, no new state, compatible with the no-randomness-in-the-substrate
   constraint by construction.
3. **Add an `ErrorShaper` const trait, owned by the fold combinator, never by `Policy`.** A closed set
   of named, individually-verified instances (`FirstOrderFeedback` shown; do not build a generic
   order-N shape under this workspace's forbidden-feature list). Probe-verified mechanism and bound
   (`29_probes/probe_2`). State explicitly, at the point this ships, that a shaped fold forfeits
   `AddAssoc`-style regrouping on that accumulator, because the feedback is a strict sequential
   dependency by construction, not a detail to discover later from a scheduler that regrouped it
   silently. Cost: the trait, the closed instance set, and the stated conflict with the regrouping
   machinery; no cost to a consumer who never shapes.
4. **Name the spatial dithering case as a Stage G crossing, owned by the raster-domain consumer, not
   by arvo.** arvo ships the scalar mechanism from proposal 2; a downstream crate with a neighbour
   topology (2D image space, a filter kernel's tap positions) builds the pattern generator on top of
   it, per consolidation's already-licensed "shallower entry point" move (`26:489-495`). Cost: zero to
   arvo; this is a boundary statement, not new machinery, and it is exactly the concrete UNORM8/D61
   worked example the design already has, given its actual answer.
5. **Flag, rather than resolve, the floating-quantum dither question.** Fixed-point dithering theory
   is solved and ports cleanly. Whether the same guarantees hold across a variable quantum with
   binade boundaries is open in my own field as far as I know it, and the design should not imply
   otherwise by silence. Cost: a documentation sentence now; a real answer needs literature I do not
   have, or a bench once a floating consumer exists to motivate one.

The place I would push hardest if this were mine to decide alone: proposal 3's forfeiture statement.
Every other proposal here is additive and costs a consumer who ignores it nothing. This one is not: it
is a warning that two pieces of machinery the design is building in the same round, sequential error
feedback and parallel regrouping, are mutually exclusive on the same fold, and the design has no
mechanism yet to say so at the type level. Left unstated, the failure mode is not a compile error. It
is a scheduler that regrouped a feedback loop because nothing told it the loop was there, and the
resulting bug would not look like a rounding bug. It would look like a shaped signal that occasionally,
silently, is not shaped at all.
