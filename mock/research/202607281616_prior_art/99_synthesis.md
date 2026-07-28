# Synthesis: eight passes, and what they leave for the design round

**Date:** 2026-07-28
**Kind:** synthesis of this directory. Not design, and it decides nothing.
**Covers:** `00_context.md`, the six external passes `01` through `06`, and the two in-stack audits
`07` and `08`.

Six external passes ran in parallel against the prescribed restructure and the questions it leaves
open, plus two in-stack audits reading arvo's own manifests and source. This file states what they
found together that no single pass could produce, which parts of the ratified design the field
corroborates, which parts it challenges, and what the round should stop assuming.

Nothing here decides anything. Where a pass and a ratified decision disagree, the decision stands and
the disagreement is recorded so the round can weigh it.

## The finding that five passes reached independently

**The mathematics arvo wants to build has no fixed-point literature, and this was confirmed from five
unrelated directions on the same day.**

Geometric algebra: no fixed-point implementation exists, and the two adjacent bodies of work that look
like one are separated out explicitly so they are not mistaken for it, being Lipschitz and Hurwitz
integer quaternions, and FPGA "fixed-size" Clifford hardware where fixed-size means storage layout
rather than numeric representation. Curves: no source treats clothoid or Fresnel evaluation in fixed
point, and the closest real precedent is FreeType's F26Dot6 quadratic rasterisation, which says nothing
about cubic evaluation, arc length or higher precision. Nonlinear algebra: no fixed-point
implementation of any surveyed algorithm exists anywhere in the literature searched, and no
compile-time formulation of rank exists for any decomposition family. Type-level shape: no
fixed-point-arithmetic-plus-shape-typing precedent anywhere. And in the sibling repository, colour:
no general-purpose fully-integer colour-management pipeline, with every perceptual-space,
gamut-mapping and spectral paper written in reals with float as an unstated assumption.

**The sixth pass found the opposite, and the contrast is the actual finding.** Fixed point is not
neglected. It is thriving, as the default representation in digital signal processing, in
safety-critical embedded control, in financial and blockchain-deterministic arithmetic, and in
constant-time cryptography. ARM's CMSIS-DSP ships `q7_t` and `q15_t` and `q31_t` with saturating
multiply-accumulate as a first-class instruction. Ethereum has no float instruction at all, and its
fixed-point libraries hand-roll `exp`, `log` and `sqrt` over `uint256`. Uniswap V3 picked `Q64.96`
specifically so `sqrt(price)` fits exactly.

So the shape is not "fixed point is unexplored". It is that **fixed point is alive in engineering and
absent from the mathematics**, and arvo sits precisely at the join. Every pass that looked at a
mathematical domain found nothing; the pass that looked at engineering practice found a rich,
production-hardened literature that has never been connected to any of it.

That is a coherent picture rather than five disappointments, and it says the design round should expect
to derive rather than adopt, in the geometry, curve, algebra and decomposition work specifically.

## What one pass could not have found

**The nonlinear-algebra lead in D11 has an answer, and it took two passes pointed in opposite
directions to surface it.**

The curves pass reports plainly that D11's nonlinear-algebra research lead has no located connection to
curve representation. The nonlinear-algebra pass, searching from the algebra side, surfaced a candidate
that was not on the list of eight names offered: **Rida Farouki's Pythagorean-hodograph curves**, a
curve family constructed through quaternion algebra, carrying a literal optimal-approximation-order
bound, which it judged a better fit for the recollection than any of the eight, precisely because it
matches the actual context rather than only the phrase.

The curves pass then cites the same family a third time without connecting it, in Jüttler and Šír on
biarc-to-PH-quintic offsets, flagging that the PDF could not be parsed.

Three arrivals at one family, from three directions, none of which recognised the other two. On the
name question itself the pass ranks Eckart, from Eckart-Young-Mirsky, as the strongest literal match
for "bounded optimal approximation", notes Sturmfels fits "central figure in nonlinear algebra" but not
the bounded-optimal phrasing, and adds Mirsky and the Kolmogorov n-width as candidates in case the
recollection is of a concept rather than a surname. All of that remains the lead designer's to resolve.

**A second cross-repository connection bears on `Identity<Op>`.** kirjo's colour pass found that
Direct3D's UNORM convention divides by `2^n - 1` rather than `2^n`, so the all-ones bit pattern lands
on exactly 1.0, and that the same specification separately defines ordinary `i.f` fixed point for
subpixel positions. Two conventions, deliberately coexisting, because a colour channel needs an exact
one and a raster position does not. arvo's reasoning that `UFixed<0, F, S>` has no multiplicative
identity is correct for arvo's encoding; UNORM is a different encoding at the same bit width where it
does. That reframes the question as one about scale convention rather than about bit width. Full
statement in `kirjo/mock/research/202607281616_prior_art/99_synthesis.md`.

## What the field corroborates

**D5's count-parameterised-by-carrier move has direct shipped precedent.** `generic-array` over
`typenum` is the same shape, and typenum's choice of a binary rather than Peano-unary encoding is the
one piece of transferable settled folklore for `Cardinal::succ`.

**D4's shape encoding has a working precedent that shipped.** Accelerate's `Z :. Int :. Int` is
structurally identical, in snoc rather than cons order. Gibbons's Naperian functors are the categorical
account of what the `Cons<H, T>::Array<E>` composition actually is. And Futhark's size types are the
field's clearest demonstration of exactly the failure D4 is built to avoid, where `zip (concat A B)
(concat A C)` fails on syntactic equality of size expressions. D4's stated reason for keeping the
arithmetic in value position is confirmed by someone else hitting the wall.

**Fixed point as a scalar is production-validated where the input is adversarial.** Servo tracked
large-`f32` layout coordinates as a precision bug and moved to `app_units` at 1/60 of a CSS pixel,
which WebKit already used and Gecko originated in 2002. Reported in lato's set, and it is the strongest
external endorsement of the numeric choice this stack has made.

**Tropical algebra has a 2026 positive that fits the constraints exactly.** PALMA is a fixed-point
integer tropical-algebra library for ARM embedded systems with a static-buffer no-alloc mode. Tropical
algebra is a genuine semiring, and it connects to the shortest-path and scheduling work `arvo-graph`
already does. It is the only surveyed algorithm family that arrived already satisfying the constraints.

**Structured matrices are the const-generic-friendly family.** Displacement rank, covering Toeplitz,
Hankel, Cauchy and Vandermonde, was flagged as the one decomposition neighbourhood whose structure is
compile-time expressible.

## What the field challenges

**`generic_const_exprs` was never the constraint, and this synthesis said the opposite in its first
revision.** The baseline handed to every pass described GCE as a pervasive arvo dependency. It is not.
It is **forbidden**, arvo had already migrated away from it before these passes ran, and a sketch has
since proved the last remaining use expressible without it.

Corrected, with what each pass actually contributes:

The type-level pass found the Rust project's own 2026 goals describing GCE as fundamentally flawed and
being replaced by `min_generic_const_args`. That is **corroboration of a migration arvo completed
ahead of upstream**, not a risk. The `Capacity` trait with its associated `Array` type replaced the
`const N: Cap` plus `cap_size(N)` form, and `arvo-comb`, `arvo-graph` and `arvo-spectral` each dropped
their gate because of it. The sketch at `mock/research/sketches/202607282100_container-projection-without-gce/`
closes the remainder: the Pattern C container projection reproduces as typestate and compiles clean
with **zero feature gates**, including the caller-threads-its-own-generic case.

Two genuinely new findings came out of the revision, and both matter more than the original framing.

**Precedent exists for the exact move arvo is making.** The `fixed` crate's stable 1.x line encodes
fractional-bit count as a `typenum` type rather than a const generic, and `generic-array`'s
`ArrayLength` trait does the same type-with-associated-container move generally, predating Rust's const
generics and stating its own equivalence to `const N: usize`. So width-as-typestate is not novel
territory; it is the pre-const-generics idiom, reached again from the other side.

**The compile-time warning does not transfer, and the distinction is depth versus breadth.** The
shapeless ten-minute build and frunk's `recursion_limit` wall are costs of **recursive chain depth**
through type-level induction. A per-width associated-type impl table is **flat breadth**: one impl per
width per family, resolved by a single lookup rather than a fold. Those are different costs and the
literature on the first says little about the second. The pass went looking for the breadth question
directly and found the rustc-dev-guide's own trait-resolution chapter carrying an open `TODO` where
that answer would be, which it reports as the negative result it is.

So the open cost question is real but narrower than feared, and it is a bench question rather than a
literature one.

**The compile-time cost of D4 is unmeasured and the comparable precedent is alarming.** Shapeless
documented roughly ten-minute compile times on about seventy lenses, two orders of magnitude, from
recursive typeclass induction over an hlist. Scala 3's response was not discipline but replacing
typeclass recursion with compiler-native induction, via native tuples, match types and a synthesised
`Mirror`. frunk hits rustc's `recursion_limit` directly from `Plucker` resolution. Nobody has measured
the specific case D4 needs, rank-depth hlists under current Rust const generics. That is a bench
question and it is the one that could invalidate D4 on cost rather than on correctness.

**A monoid law cannot be stated unconditionally over arvo's own types.** The algebra pass derived a
counterexample against shipped code: signed `Saturating` addition is not associative under two-sided
clamping, while `Wrapping` addition is a full abelian group. Both are `OverflowPolicy` instances sealed
at `arvo-strategy/src/axes.rs:38`. So whether `Combine<Op>` satisfies its law depends on a **second**
type-level axis, the strategy, not only on the operation marker that `Identity<Op>` is parameterised
by. The pass found no literature at all on laws conditional on a second type-level parameter, across
Haskell, Scala, Rust and the proof-carrying Agda approach. This is the sharpest design-relevant finding
in the set, because the narrow gap that `202607281547` describes turns out to have a shape nobody has
solved.

**The algebra ladder should probably not be climbed far, on two independent data points.** NumHask
collapsed Ring, Field, Distributive and Module to type synonyms and narrowed to `SemiField`. The three
Rust attempts tell a story in their abandonment: `alga` used the operation-marker shape arvo has
chosen and was succeeded by the narrower `simba` with no stated reason; `noether` rebuilds the same
tower citing `alga` without explaining why a fresh crate was needed. The pass found no stated reason
for either transition, which is itself worth knowing before a fourth attempt.

**D10's citation picks a side in a live fork without saying so.** Projective geometric algebra has an
unresolved technical split between the bivector.net convention (Gunn, de Keninck, Dorst, Roelfs) and
Eric Lengyel's rigid geometric algebra, differing on dimensional assignment, on what wedge and
antiwedge mean, and on normalisation convention. Both have shipping code and consumers. D10 cites
"Gunn's plane-based projective geometric algebra, with Dorst and Lasenby", which lands on one side of a
fork the round does not appear to know exists. Separately, no documented feud exists between Dorst and
Lasenby; they diverge in emphasis, and Dorst sits on the PGA side of the real fork via his 2022 paper
with de Keninck.

**D10's fallback is novel rather than conventional.** No library was found that derives the bivector
basis as a compile-time type-level fold over an axis list. The three strategies in use are baking at
generation time, generating per signature at build time, and deferring to a runtime JIT on actual input
sparsity. The hlist fold D10 names as the fallback if the component count proves awkward in type
position is unattempted territory, not a known technique.

## What closes cleanly

Two doors the nonlinear-algebra pass shut with reasons rather than shrugs, which is worth having in
writing so nobody reopens them speculatively. **Groebner bases** are doubly exponential in the worst
case, structurally incompatible with const-sized no-alloc storage. **Homotopy continuation** conflicts
on every axis of the constraint set. **CP rank is NP-hard**, which is a hard wall in the tensor-format
neighbourhood regardless of implementation effort.

And on the curve side, one mathematical fact bounds a candidate permanently: exact distance to a cubic
Bezier is a degree-six root-finding problem, so by Abel-Ruffini it has no closed form. That is a
ceiling on the implicit and signed-distance candidate, not an engineering difficulty.

## What the round should stop assuming

**That the facade situation forces the restructure.** `07` shows `Cap` is declared at
`arvo-storage/src/platform.rs:73` rather than in the facade, that `arvo-tensor`'s rule is therefore
satisfiable as written, and that fifty of the fifty-one facade imports across seven crates name a
symbol already reachable from a permitted dependency. Not one of the seven imports `UFixed` or
`IFixed`. The restructure may well be right; this particular argument for it does not hold, and a
mechanical fix is available independently.

**That D2 places everything.** `08` walks every declared symbol to a destination and finds three that
the decision does not place: the `Pred` family, which is unmentioned and is the one symbol that breaks
when the facade goes terminal; the euclidean-division traits, ambiguous between contracts and
concretes; and the `bitfield!` macro, which expands to `Bits` and so argues for `arvo-bits` over
`arvo-numeric`. `Enumerator` is already recorded as unsettled.

**That D4 answers rank polymorphism.** It answers rank-generic **types**. Remora, the field's formal
treatment of rank-polymorphic **functions**, works over existentially quantified dependent shape rather
than an hlist of capacities, so D4's representation is unvalidated for the function question
specifically. The distinction matters because "a function generic over rank is unwritable today" is
what D4 names as the actual gap.

**That arvo's vocabulary covers why a consumer would choose it.** The number-systems pass names two
properties that are load-bearing in adjacent fields and silent in arvo. **Deterministic reproducibility
across compilers and targets** is why blockchain and lockstep simulation reach for fixed point, and it
is a different argument from the strategy tradeoff arvo documents. **Constant-time execution** is
treated as a correctness property in cryptography, and it is in direct tension with the
always-optimal-internals principle, since the fastest path and the constant-time path frequently differ
by a data-dependent early exit. Whether arvo's audience wants either is not the pass's call; that both
are unnamed is the finding.

## Coverage, stated because it varies across the set

The session-wide search budget of 200 calls was exhausted partway through this arc, shared across all
eleven passes in it. `01`, `03` and `06` ran with search largely available. `02`, `04` and `05` ran
partly or wholly on `WebFetch` against already-known URLs, and each says so in its own closing section
rather than absorbing the gap.

Named gaps worth knowing: the DSP bit-width-allocation compiler literature beyond one catalogue was not
completed; Spire's primary algebra documentation returned 403, so those claims rest on secondary
sources; a 2025 paper on Bezier curves over digital grids returned 403 and is flagged unread rather
than dropped; and no controlled compile-time or binary-size comparison across hierarchy depth exists to
cite for any of it.

`07` and `08` are in-stack and read declarations rather than documents, because the two documents that
describe arvo's layer structure disagree with each other and with the source about where `Cap` lives.
