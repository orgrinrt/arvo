# Number systems and fixed point: what the field does that arvo does not

**Date:** 2026-07-28
**Kind:** research, not design
**Governing:** `arvo/mock/design_rounds/202607281220_topic.the-ndim-and-shape-design.md`,
`arvo/mock/design_rounds/202607281547_topic.identity-already-exists-and-what-algebra-still-needs.md`.
Read `00_context.md` in this directory first; this file assumes its provenance rules, its constraint
list, and its instruction not to recommend.

**Correction, 2026-07-28.** An earlier revision of this document, following the `00_context.md` it was
written against at the time, framed the Rust `fixed` crate's need for the nightly `generic_const_exprs`
feature as a wall arvo shares. That premise was wrong. `generic_const_exprs` is not a constraint arvo
carries going forward: the lead designer has forbidden it outright, alongside full `specialization`,
under a standing gate that predates this workspace (a feature is allowed only if it is not proven
unsound or unstable and, absent a strong reason, is itself on the stabilisation path). arvo had already
migrated most of its own use of the feature away via the `Capacity` trait's associated `Array` type
before that gate was restated, and a sketch at
`mock/research/sketches/202607282100_container-projection-without-gce/` has since proven the one
remaining live use, the container projection in `arvo-strategy` and the `arvo` facade, expressible as
typestate with zero feature gates of any kind. Verified directly against source for this correction:
`generic_const_exprs` is gated at exactly `mock/crates/arvo-strategy/src/lib.rs:11` and
`mock/crates/arvo/src/lib.rs:25` today, matching the corrected `00_context.md`'s count, and the sketch's
own `FINDINGS.md` records a clean `cargo check` with zero `#![feature(...)]` lines. The observation
about the `fixed` crate hitting the `generic_const_exprs` wall in its own 2.0 alpha line is still
accurate on its own terms and still stands, in the CORDIC section below; only the claim that arvo
shares that wall has been removed, and a further passage there answers a follow-up question, raised by
this correction, about precedent for encoding width as a type with an associated container rather than
as a const-generic parameter. Nothing else in this document changed; every other section is independent
of the `generic_const_exprs` question.

arvo's identity is a single, specific bet: every logical number is fixed point, every container is
exact-width and const-sized, and the tradeoff between speed and precision is a type parameter
(`Hot` / `Warm` / `Cold` / `Precise`) rather than a runtime branch or a library-wide compile flag.
Floats exist only as a tagged escape hatch. Nothing grows, nothing allocates, and the size of every
value is known before the program runs. That is an unusual place to stand. Most of numerical
computing either accepts IEEE float and its rounding model wholesale, or reaches for fixed point only
inside a DSP kernel where the width is small and the operations are a handful of multiply-accumulates.
arvo asks fixed point to be the general-purpose default across an entire numeric and analysis stack,
with the const-generic width and the strategy marker doing the work a runtime dispatch would otherwise
do. This document surveys what the rest of the field has built around that same problem, fixed-width
and fixed-point arithmetic, the representations proposed as replacements for it, and the tooling built
to choose its parameters automatically, and reports where each piece would survive arvo's constraints
and where it would not.

## The settled ground, briefly

Two works sit underneath everything else in this document and do not need five years of freshness to
be relevant, because nothing since has replaced them. Donald Knuth's treatment of fixed-slash-floating
representations and multiple-precision arithmetic in *The Art of Computer Programming, Volume 2:
Seminumerical Algorithms* (3rd ed., Addison-Wesley, 1997) is still the reference for the multi-limb
algorithms (long multiplication, Karatsuba, Newton division, the classical square-root iterations)
that any const-width wide-integer library, arvo's `WideBits` included, is a restatement of. David
Goldberg's "What Every Computer Scientist Should Know About Floating-Point Arithmetic" (*ACM Computing
Surveys*, vol. 23, no. 1, March 1991, pp. 5 to 48) is still the standard citation for why IEEE float's
rounding model behaves the way it does, and by direct contrast, for why a format with a uniform ULP
across its whole range (fixed point) trades away float's graceful degradation at extreme magnitude for
predictable, uniform error everywhere it is defined. arvo's choice to make fixed point the default
and float the tagged exception is a bet against the world both of these works describe, not a bet
built in ignorance of it. Everything below is about what has been tried since, or is being tried now,
against exactly that bet.

## What modern fixed-point practice does that arvo does not

Fixed point never stopped being used. It is the default numeric representation in three domains that
have nothing to do with each other except that none of them trust IEEE float: digital signal
processing on hardware without a float unit, safety-critical embedded control loops, and blockchain
virtual machines where every node must reach the same answer bit for bit.

The DSP convention is the Q format, `Qm.n`, m integer bits and n fractional bits inside a fixed
container, almost always saturating rather than wrapping on overflow because a wrapped audio or
control sample is a much worse failure than a clipped one. ARM's CMSIS-DSP library ships this as
`q7_t` / `q15_t` / `q31_t` with saturating intrinsics baked into the instruction set on Cortex-M
cores, and TI's C2000 and C6000 DSP families do the same at the hardware level: saturating
multiply-accumulate is a first-class instruction, not a library convenience. arvo's `OverflowPolicy`
axis (`Saturating`, `Wrapping`) already covers this choice as a type-level parameter rather than a
runtime flag, which is more expressive than the DSP convention (where the saturating behaviour is
fixed per instruction, not chosen per value), but arvo's public surface (per `00_context.md`) does not
currently document a Q-format-style naming convention (`Q1.15`, `Q17.15`) that a DSP engineer would
recognise on sight, and the fixed-point literature treats saturating overflow as the default
assumption in a way arvo's `Warm`-default framing does not.

Financial and blockchain-deterministic arithmetic is fixed point for a different reason: determinism
across heterogeneous hardware. IEEE float's rounding behaviour, while specified, has enough
implementation-defined corners (extended precision on x87, fused-multiply-add availability,
compiler reassociation under different optimisation levels) that two nodes computing the same float
expression can disagree, which is fatal to blockchain consensus. Ethereum's EVM has no floating-point
instruction at all; every fixed-point library on it (Paul R. Berg's PRBMath, at
[github.com/PaulRBerg/prb-math](https://github.com/PaulRBerg/prb-math), and the older ABDKMath64x64,
surveyed at RareSkills, ["Fixed Point Arithmetic in Solidity"](https://rareskills.io/post/solidity-fixed-point),
and compared directly at Krushi Raj Tula's
["FixedPointMathLib vs ABDKMath64x64"](https://krushiraj.github.io/writings/fixed-point-vs-abdk-math/))
is a plain integer type with an implied scale factor and hand-written division, multiplication,
`exp`, `log`, and `sqrt` routines that are careful about overflow because the underlying integer is
already at its widest practical size (`int256`/`uint256`). PRBMath ships `SD59x18` and `UD60x18`
(59 or 60 integer digits, 18 fractional decimal digits, chosen so the fractional part matches Ether's
own 18-decimal convention) plus narrower variants (`SD1x18`, `SD21x18`) specifically so a struct field
can be packed smaller when the full 256-bit width is not needed, which is a hand-rolled version of
exactly the strategy-marker idea arvo already generalises. Uniswap V3's own price representation,
`Q64.96` (64 integer bits, 96 fractional bits, explained in the
[Uniswap V3 Development Book](https://uniswapv3book.com/milestone_3/more-on-fixed-point-numbers.html)
and in [Bloqarl's writeup](https://medium.com/@bloqarl/uniswaps-q64-96-explained-essential-security-tips-for-hook-developers-4bfc4afad2f7)),
is chosen so that `sqrt(price)` itself fits exactly, trading fractional precision for making a
downstream square-root-heavy computation exact. None of this is expressible as a design lesson arvo
is missing; `UFixed<I, F, S>` already generalises the `Qm.n` idea past any single fixed pair of
widths. What arvo does not have, and this literature does, is a documented deterministic-arithmetic
argument for choosing fixed point over float at all, separate from the strategy-tradeoff argument
arvo already makes. A consumer picking arvo for cross-node determinism (a distributed simulation, a
replay-critical game engine, a lockstep-network application) is choosing it for a property (bit-exact
reproducibility across compilers and targets) that the strategy-marker documentation does not
currently name as a reason to reach for `Precise` over `FastFloat`.

The constant-time argument is the sharpest of the three and the one arvo has not touched at all.
Cryptographic fixed-width arithmetic treats data-dependent branching and data-dependent memory access
as bugs, not performance details, because a variable-time division or comparison leaks the operand
through a timing side channel. RustCrypto's `crypto-bigint`
([github.com/RustCrypto/crypto-bigint](https://github.com/RustCrypto/crypto-bigint)) states this as a
design axiom: "constant-time by default", with any variable-time function explicitly named as such,
and ships `no_std`-friendly, const-generic, stack-allocated big integers audited by NCC Group. arvo's
`UArith` / `IArith` / `USaturating` / `ISaturating` families give no signal, in the surface enumerated
by `00_context.md`, about whether a given implementation path is constant-time, and the
always-optimal-internals principle that lets arvo's internals reach for whatever asm or SIMD
intrinsic benches fastest is, if anything, in tension with a constant-time guarantee, since the
fastest path and the constant-time path are frequently different paths chosen by a data-dependent
early exit. Whether arvo's audience needs this property at all is a question this document does not
answer; the finding is only that the property exists, is load-bearing in an entire adjacent field, and
is currently silent in arvo's own vocabulary.

## Alternative real-number representations, and where each actually wins

Four families propose to replace, not extend, the float-versus-fixed-point choice arvo has already
made. Each is evaluated here against arvo's own constraints: const size, no heap, no `dyn`, `#![no_std]`.

### Posits, unums, and takums: tapered precision as a third axis

Gustafson and Yonemoto's original pitch, "Beating Floating Point at its Own Game: Posit Arithmetic"
(*Supercomputing Frontiers and Innovations*, vol. 4, no. 2, 2017,
[researchgate.net/publication/322151112](https://www.researchgate.net/publication/322151112_Beating_Floating_Point_at_its_Own_Game_Posit_Arithmetic)),
replaces IEEE float's fixed-width exponent field with a variable-length "regime" field, so precision
tapers: values near magnitude one get the most significand bits, and precision falls away smoothly as
a value moves toward zero or infinity, rather than falling off a cliff at the edge of the normal
range the way IEEE float does with subnormals. This is a genuinely different axis from anything arvo
has. arvo's fixed-point format has a uniform ULP across its entire representable range by
construction (that is what fixed point means); posits and their descendants deliberately make the ULP
non-uniform, spending bits where the application says values are more likely to land. A recent survey,
"Navigating Posit Arithmetic: A Comprehensive Survey of Principles, Hardware, and Applications"
(*ACM Computing Surveys*, 2025, [dl.acm.org/doi/10.1145/3772284](https://dl.acm.org/doi/10.1145/3772284)),
and a division/square-root-focused hardware paper, "Posit Arithmetic Hardware Implementations with
The Minimum Cost Divider and Square Root" (*MDPI Electronics*, vol. 9, no. 10, 2020,
[mdpi.com/2079-9292/9/10/1622](https://www.mdpi.com/2079-9292/9/10/1622)), both confirm that posit
hardware is real and shipping, and both confirm the standing criticism: because the field width that
is regime versus significand is data-dependent, posit hardware costs more circuit area than an
equivalent-width IEEE float unit, for a format whose bit width is otherwise fixed like arvo's own
containers.

Takum arithmetic, introduced in 2024 by Laslo Hunhold at the Conference for Next Generation Arithmetic
(CoNGA 2024, [posithub.org/conga/2024](https://posithub.org/conga/2024/)) and detailed in "Beating
Posits at Their Own Game: Takum Arithmetic" (arXiv:2404.18603,
[arxiv.org/abs/2404.18603](https://arxiv.org/abs/2404.18603)), is a direct answer to posits' own
weakness: posits are excellent near magnitude one and degrade badly far from it, so takum changes the
tapering so dynamic range stays asymptotically constant in terms of bit-string length regardless of
how far a value sits from one. This is a 2024 result, actively being applied rather than merely
proposed: a 2025 paper, "Spectral Methods via FFTs in Emerging Machine Number Formats: OFP8, Bfloat16,
Posit, and Takum Arithmetics" (arXiv:2504.21197,
[arxiv.org/html/2504.21197](https://arxiv.org/html/2504.21197)), runs real FFT-based spectral methods
against all four formats to see which holds up numerically, which is exactly the kind of applied,
unproductised-but-working result the brief for this document asked to be weighted over settled
consensus.

None of the posit or takum family fits arvo's constraints as shipped. A variable field split forces
either a runtime decode step or, in a hardware implementation, dedicated leading-zero/leading-one
count circuitry; a software emulation of that decode is not the kind of thing that disappears into
zero-cost monomorphisation the way arvo's fixed-width dispatch does, because the decode's cost is a
function of the value, not of the type. The open question this raises for arvo, stated without an
answer: arvo's identity is that every axis of tradeoff (speed, precision, storage) is expressed as a
type parameter picked once at the call site; tapered precision is an axis where the tradeoff is
expressed **within a single value**, varying by magnitude, which is a different kind of parameter than
anything `Strategy` currently models, and it is not obvious what a const-generic, no-heap encoding of
that axis would even look like without either a lookup table sized to the type's width (which is
finite and knowable at compile time, so possibly affordable) or a genuinely variable-cost decode
(which is not).

### Block floating point and the microscaling formats

Block (or shared-exponent) floating point is an old DSP idea: store a single exponent for a whole
block of values and let each element carry only a narrow mantissa, amortising the exponent's storage
cost across the block. What is new is that the Open Compute Project standardised this in 2023 as
"Microscaling" (MX) specifically for machine-learning inference and training, and that the standard
has real silicon behind it now. "Microscaling Data Formats for Deep Learning" (Rouhani, Bita Darvish,
et al., 32 co-authors from AMD, Arm, Intel, Meta, Microsoft, NVIDIA, and Qualcomm, arXiv:2310.10537,
2023, [arxiv.org/pdf/2310.10537](https://arxiv.org/pdf/2310.10537)) defines four concrete formats
(MXFP8, MXFP6, MXFP4, MXINT8), each a block of 32 elements sharing one 8-bit power-of-two scale (the
`UE8M0` format, an 8-bit exponent with no mantissa of its own) against narrow per-element values, and
reports that MXFP4-weight, MXFP6-activation training of a generative language model loses only a
minor amount of loss versus full FP32, with no change to the training recipe. This is shipping
hardware, not a proposal: it is present in NVIDIA Blackwell and AMD CDNA4 GPUs, and a 2025 RISC-V ISA
extension paper, "MXDOTP: A RISC-V ISA Extension for Enabling Microscaling (MX) Floating-Point Dot
Products" (arXiv:2505.13159), proposes instruction-level support for it outside the GPU world
entirely. The formalism has itself become a research target: a 2026 paper builds a bit-exact
conformance-vector catalogue across 83 low-precision formats including MXFP4 specifically because
implementations were disagreeing at the bit level ("An 83-Format Numeric Catalog with Bit-Exact
Conformance Vectors", arXiv:2606.09686), which is a useful cautionary data point about how fragile
"standardised" narrow formats are in practice even after standardisation.

Block floating point is structurally interesting for arvo for a reason distinct from posits: it is
not a per-value encoding trick, it is an **array-level** property. A single `UFixed<I, F, S>` has no
concept of a scale shared with its neighbours; the scale, in arvo's model, is entirely fixed by the
type (`I` and `F` are compile-time constants, identical for every instance of the type). Block
floating point asks for a scale that lives at the level of a *collection* of values, which is exactly
the layer the ratified `arvo-shape` / `arvo-tensor` restructure (D1 through D4 in `00_context.md`) is
now building out: a shape is an hlist of capacities, and the backing storage is the recursive
composition of each capacity's array. Whether a block-shared-scale numeric format is expressible as
an axis of `Array<UFixed<I, F, S>, C>` (a per-block scale stored once, alongside N narrow mantissas,
rather than N independent full-width values) is a real open question this literature raises, not one
this document answers: it would be a new kind of container, sitting between arvo's current per-value
`UFixed` and a full float, whose defining property is that the scale is amortised across a block
rather than carried per element, and nothing in the currently-shipped or currently-ratified design
names that shape.

### Logarithmic number systems

A logarithmic number system (LNS) stores a fixed-point-encoded logarithm of the value instead of the
value itself, which turns multiplication and division into addition and subtraction, both cheap and
exact operations on the stored representation. The tradeoff inverts fixed point's usual cost profile:
addition, the operation fixed point does for free, becomes the expensive one in LNS, because summing
two numbers each known only by their logarithm requires evaluating `log(1 + 2^(-d))` for some
difference `d`, which is not closed-form and is usually done with either a lookup table or a
piecewise-polynomial approximation. Behrooz Parhami's 2020 survey, "Computing with Logarithmic Number
System Arithmetic: Implementation Methods and Performance Benefits" (*Computers & Electrical
Engineering*, 2020, [web.ece.ucsb.edu/~parhami](https://web.ece.ucsb.edu/~parhami/pubs_folder/parh20-cee-comp-w-lns-arith-final.pdf)),
is the standing reference, and a 2021 paper, "Low precision logarithmic number systems: Beyond
base-2" (arXiv:2102.06681), reports that choosing a non-power-of-two logarithm base can lower error
at very low bit widths versus the conventional base-2 LNS, which is a genuinely recent, still-being-
argued-over refinement rather than settled practice. Current hardware work is aimed at
multiply-accumulate specifically: table-lookup MAC frameworks from 2024 compile quantised neural
networks into reusable lookup-table arrays and report they scale to full ImageNet-sized models
(summarised via [emergentmind.com/topics/lns-multiply-accumulate-units](https://www.emergentmind.com/topics/lns-multiply-accumulate-units);
the primary sources for the specific 2023 to 2024 FPGA MAC and TLMAC claims were not independently
retrieved in this pass, flagged below).

LNS is compile-time-sizeable in the same way arvo already sizes everything: the stored logarithm is
itself a fixed-point value of a known width, and the interpolation table for the addition correction
term is a fixed-size array indexable by the fixed-width difference, so an `S: Strategy`-shaped LNS
container (fixed width, const-sized correction table, no allocation) is not obviously ruled out by
any of arvo's stated constraints. What it costs is arvo's central promise for `Add`: LNS turns the
operation that is currently free (integer addition on a fixed-point representation) into the one that
needs a table lookup or an approximation, while turning `Mul` and `Div`, the operations fixed point
already handles cheaply for narrow widths but not for wide ones, into pure addition regardless of
width. Whether that tradeoff is worth a fifth strategy marker, a wholly separate numeric type, or
nothing at all is exactly the kind of question the brief for this document says is not this
document's to answer; the finding is that the mechanism is real, has current (2020 to 2024) literature
behind it, and is not foreclosed by any constraint arvo already carries.

### Residue number systems

A residue number system (RNS) represents a value as its residues modulo a fixed set of coprime
moduli chosen once and shared by every value in the system (a "modulus set"). Addition,
subtraction, and multiplication are then fully independent per-residue operations with no carry
propagation between them at all, which is the property that makes RNS attractive for exactly the
kind of wide, parallel, cryptography-shaped arithmetic that `arvo-storage`'s `WideBits` already
targets. The cost is on the other side of the ledger: comparison, sign detection, division, and
overflow detection are all expensive in RNS, because none of them is a per-residue operation; they
require either a full base extension (reconstructing enough of the true value via the Chinese
Remainder Theorem or Mixed-Radix Conversion to answer the question) or a separate, redundant channel
carried alongside the residues specifically to make sign and overflow cheap to check (a Redundant
Residue Number System, RRNS). A 2025 open-source VHDL library targets exactly that redundant variant
for FPGA and ASIC integration ("RRNS Arith Lib", *ISFPGA 2025*,
[dl.acm.org/doi/10.1145/3706628.3708847](https://dl.acm.org/doi/10.1145/3706628.3708847)), and a 2024
paper proposes RISC-V instruction-set support specifically because general-purpose instruction sets
have no primitive for modular reduction against an arbitrary, program-chosen modulus set: "RISC-V
Word-Size Modular Instructions for Residue Number Systems" (Didier, Laurent-Stéphane, and Jean-Marc
Robert, *Future Technologies Conference 2024*, arXiv:2412.05286,
[arxiv.org/abs/2412.05286](https://arxiv.org/abs/2412.05286)) reports 2.76 to over 3 times the speedup
for RNS modular multiplication with the proposed instructions versus a pseudo-Mersenne-modulus
baseline, and 4.5 to 8 times fewer cycles against x86 for the same operation. A 2025 hybrid design,
"A Hybrid Residue Floating Numerical Architecture for High Precision Arithmetic on FPGAs"
(arXiv:2512.09155), combines RNS's parallel multiply-add with a floating-point-shaped exponent to try
to get both RNS's independence and float's dynamic range at once, which is worth naming as a live
research direction rather than a settled one.

RNS is compile-time-sizeable exactly like LNS and exactly like arvo's own multi-limb containers: a
fixed modulus set, chosen at compile time, gives a fixed number of fixed-width residues, which is a
const-sized array with no heap involvement of any kind. The genuine cost, comparison and division
being expensive, is the same shape of cost arvo's own strategy markers already exist to make
explicit rather than hidden: an RNS-backed container would need a `Strategy`-shaped signal that says
"comparisons and overflow checks on this type are not O(1) the way they are on `Bits<N>`", which
nothing in arvo's currently-shipped `Strategy` vocabulary (`Hot` / `Warm` / `Cold` / `Precise`, all of
which describe a speed-versus-storage tradeoff on operations that are uniformly cheap regardless of
strategy) is built to express, since RNS's cost asymmetry is per-operation, not per-strategy.

## Carrying error instead of losing it: interval, affine, and Taylor-model arithmetic

Every representation discussed so far answers "how do I store one number." A separate family answers
a different question: "how do I compute a bound that is guaranteed to contain the true result,
including every rounding error committed along the way." arvo's identity says nothing at all about
this axis today; every `UFixed` operation returns a single value, and the error committed by that
operation (quantisation, overflow-policy rounding, or a cross-strategy resolution) is discarded, not
carried forward.

**Interval arithmetic** is the simplest member of this family and the cheapest to afford: a value is
a pair, lower and upper bound, and every operation is defined so that the result interval is
guaranteed to contain the true result no matter what happened inside. This is a fixed-size pair of
whatever the underlying scalar type is, has an IEEE standard (IEEE 1788-2015), and fits arvo's
constraints without any tension at all: `Interval<T>` for any `T: TotalOrd + Bounded` is exactly the
shape of a two-field `repr(transparent)` struct arvo already builds constantly, and no search in this
pass turned up any reason a const-width, no-heap interval type over `UFixed` would not simply work.
The catch interval arithmetic is famous for, and which every source in this pass repeats, is that it
does not track correlation: `x - x` for an interval `x` does not collapse to zero, it produces an
interval as wide as `x` itself was, because interval arithmetic has no memory of the fact that both
occurrences of `x` are the same value. That blow-up compounds across a long computation and is the
entire reason the next two representations exist.

**Affine arithmetic** fixes exactly that problem by representing a value as a central estimate plus a
linear combination of shared "noise symbols," `x0 + x1*e1 + x2*e2 + ... + xn*en`, where each `ei`
ranges over `[-1, 1]` and, critically, the **same symbol `ei` is reused** across every value derived
from a common source of uncertainty. Because two derived quantities can share noise symbols, an
operation like `x - x` correctly collapses to an exact zero, and linear operations are computed
exactly rather than conservatively. The Wikipedia summary of the field
([en.wikipedia.org/wiki/Affine_arithmetic](https://en.wikipedia.org/wiki/Affine_arithmetic)) frames
it plainly as "meant to be an improvement on interval arithmetic," and this is precisely the
representation underlying "zonotope" methods in the neural-network-verification literature, which is
where the most active recent (2023 to 2025) engineering against this idea is happening, not in
classical numerical analysis. "Zonotope arithmetic relies on affine arithmetic," and a 2024 verifier,
PyRAT, places second at the 2024 neural network verification competition using it
(["Neural Network Verification with PyRAT"](https://arxiv.org/pdf/2410.23903)); further 2023 work
extends the idea to polynomial zonotopes for tighter, still-affine-rooted bounds on nonlinear layers
(Kochdumper, Schilling, Althoff, and Bak, NASA Formal Methods Symposium 2023, cited via
["Hybrid Zonotope-Based Backward Reachability Analysis for Neural Networks"](https://arxiv.org/pdf/2310.06921)).

The structural problem affine arithmetic has for arvo is the one interval arithmetic does not: **the
number of noise symbols is not fixed at compile time in the general formulation.** Every new source of
uncertainty introduced anywhere in a computation (every multiplication of two already-uncertain
values, which affine arithmetic cannot represent exactly and must approximate by introducing a fresh
symbol) grows the symbol count by one, unboundedly, for the lifetime of the computation. A general
affine value is, in the literature's own terms, a variable-length list, which is precisely the shape
`#![no_std]`-plus-no-`alloc` forbids. What the zonotope-verification literature does in practice,
without ever stating it as a numerics-library design principle, is cap the symbol count: a
verification pass over a fixed neural network architecture has a known, finite number of sources of
uncertainty (one per input dimension, roughly, plus one per approximated nonlinearity), so a real
implementation bounds the symbol count at however many the specific network needs and represents the
zonotope as a fixed-size matrix, not a growing list. That is the concrete, working-but-not-articulated
answer to the brief's question of whether any of this is affordable without a heap: **a
compile-time-bounded affine form, `Affine<T, const N: usize>` carrying a central value plus exactly N
noise coefficients and silently degrading to a wider (or interval-conservative) bound once N symbols
are exhausted, is exactly the shape the zonotope engineering community already builds informally, and
nothing in this pass found a productised, general-purpose, const-generic library that states this
as its explicit contract.** That absence is itself the finding: the technique is proven at the
application layer (neural network verification, reachability analysis) and has not been productised
as a general const-generic arithmetic primitive the way arvo's own `Bits<N>` productises multi-limb
integers.

**Taylor models** go one step further than affine arithmetic: instead of a linear (degree-one)
polynomial in the noise symbols plus an interval remainder, a Taylor model of order n is a
degree-n polynomial plus a remainder interval, `f(x) - P(x) in Delta` for the whole domain, which
tracks nonlinear correlation exactly up to the chosen order instead of only linear correlation. This
is a real, actively used technique for validated global optimisation and validated ODE integration; a
2023 paper on sharper one-dimensional enclosures ("Sharp Taylor Polynomial Enclosures in One
Dimension," arXiv:2308.00679) and the actively maintained Julia package `TaylorModels.jl`
([juliaintervals.github.io/TaylorModels.jl](https://juliaintervals.github.io/TaylorModels.jl/dev/))
are both current. For a **fixed dimension and a fixed polynomial degree**, the number of monomial
coefficients in a Taylor model is a compile-time-known constant (it grows combinatorially with degree
and dimension, but for any specific, chosen pair of degree and dimension it is a fixed integer), so
the same reasoning that makes bounded affine arithmetic const-sizeable applies here too: a
degree-bounded Taylor model over a fixed number of variables is, in principle, a fixed-size array of
polynomial coefficients plus one remainder interval, and nothing in this pass found a reason it
could not be `repr(transparent)`-shaped and heap-free. What this pass explicitly did not find, and
states here as the negative result the brief asked for: **no no_std, no-heap, const-generic Taylor
model implementation.** Every implementation located (the Coq formalisation, the Ariadne rigorous
function calculus, `TaylorModels.jl`) targets a garbage-collected or proof-assistant host language and
none states a fixed-degree, fixed-dimension, stack-only contract as a design goal. If such a library
exists, it was not found by this pass.

## Choosing the width automatically: the rewriting and word-length literature

arvo's `I` and `F` are chosen by the consumer, by hand, at every call site. A large body of tooling
exists whose entire purpose is to choose them (or the equivalent floating-point exponent width)
automatically, and the field has enough of it that a single fixed catalogue page,
[fptalks.org/community.html](https://fptalks.org/community.html), lists dozens of currently maintained
tools split into dynamic analysis, static analysis, and commercial categories. The load-bearing
distinction the brief's three named tools draw is: **Herbie rewrites the expression** to be more
accurate at the same precision, **Gappa proves a rounding-error bound** for a fixed expression and
precision, and **Daisy does both**, combining rewriting with precision (bit-width) tuning in one
framework.

Herbie (Panchekha, Pavel, Alex Sanchez-Stern, James R. Wilcox, and Zachary Tatlock, "Automatically
Improving Accuracy for Floating Point Expressions," *PLDI 2015*,
[herbie.uwplse.org/pldi15-paper.pdf](https://herbie.uwplse.org/pldi15-paper.pdf)) samples input
points, localises where rounding error actually accumulates, and searches a rule database (algebraic
identities, series expansions) for an equivalent expression with less error at the same working
precision. It is float-native in its published form, and this pass found no evidence Herbie itself
targets fixed point directly; what it demonstrates that is transferable to arvo's domain is the
method, error localisation by sampling plus targeted algebraic rewriting, which is representation-
agnostic and could in principle be pointed at a fixed-point expression tree the same way it is
pointed at a float one.

Gappa ([gappa.gitlabpages.inria.fr](https://gappa.gitlabpages.inria.fr)) is explicitly the tool of the
three that already treats fixed point as a first-class target, not an afterthought: it automates
rounding-error proofs for both floating-point and fixed-point expressions using interval arithmetic
under the hood, producing a formally checkable certificate of the bound rather than a sampled
estimate. Daisy (Darulova, Eva, Anastasiia Izycheva, Fariha Nasir, Fabian Ritter, Heiko Becker, and
Robert Bastian, "Daisy: Framework for Analysis and Optimization of Numerical Programs," *TACAS 2018*,
[link.springer.com/chapter/10.1007/978-3-319-89960-2_15](https://link.springer.com/chapter/10.1007/978-3-319-89960-2_15))
sits above both: it performs the accuracy analysis Gappa performs, applies Herbie-style rewriting, and
then tunes the mixed precision (per-variable bit width, floating or fixed) needed to meet a target
error bound, which is the closest existing tool to "given a computation and an error budget, choose
`I` and `F` per value automatically" that this pass located.

Two more tools bear directly on arvo's specific problem, not merely on the general one. TAFFO
(Cherubin, Stefano, and Giovanni Agosta; project at
[github.com/HEAPLab/TAFFO](https://github.com/HEAPLab/TAFFO); the underlying paper was not
independently re-fetched in this pass and this citation should be treated as recalled rather than
verified) is an LLVM-based framework whose entire job is converting floating-point LLVM IR into
fixed-point IR automatically, choosing the integer and fractional bit widths per value from a static
range analysis, which is essentially "infer `UFixed<I, F, S>` from a float program" as a compiler
pass. POP (Ben Khalifa, Dorra, *IINTEC 2019*) does the IoT-specific version of the same question
from the other direction: abstract interpretation to find the **minimal** input precision an embedded
computation can tolerate. FPTaylor and FPTuner (University of Utah, soarlab,
[github.com/soarlab/FPTaylor](https://github.com/soarlab/FPTaylor)) bound rounding error using
symbolic Taylor expansion of the partial derivatives, which is the direct algorithmic ancestor of the
Taylor-model rigorous-bound idea in the previous section, applied here to the narrower question of a
single error scalar rather than a full validated enclosure. A 2024 paper whose title suggests it is
squarely on this question, "Target-Aware Implementation of Real Expressions" (arXiv:2410.14025), could
not be read in full in this pass (the fetched content did not resolve to readable text); its title and
arXiv listing are recorded here, but its actual technique is not summarised because it was not
verified.

The consistent theme across this whole literature, worth stating plainly because it bears directly on
arvo's own division of labour between the substrate and the consumer: every one of these tools treats
bit-width selection as a program-level, whole-expression-graph optimisation problem, something a
compiler pass or an external analysis tool does over an entire computation, not something a single
type's author decides once. arvo's model, where the consumer names `I` and `F` by hand at each
`UFixed<I, F, S>` site, is the opposite division of labour: it is the substrate refusing to guess
(per the toolbox-not-policer principle already governing this codebase) rather than the substrate
inferring, and nothing in this literature contradicts that choice; it simply confirms that the
inference problem this literature solves is a real, hard, actively-researched one that arvo has
deliberately declined to solve on the consumer's behalf.

## Exact and rational arithmetic, where it is cheap enough to matter

Exact rational arithmetic (a numerator and a denominator, both exact integers, reduced to lowest
terms) is unaffordable in general precisely because the denominator's bit width is unbounded: enough
successive divisions push denominator growth past any fixed width, which is the textbook argument for
why general-purpose systems reach for float instead. The genuinely cheap subset is the **bounded**
one: a rational whose numerator and denominator are each capped at a fixed bit width, refusing (or
saturating, or falling back to an approximation) once a computation would need more. This is not a
new number system at all in arvo's own vocabulary; a bounded rational is already expressible as a
pair of arvo's existing exact-width integers (`Uint<N, S>` numerator over `Uint<N, S>` denominator),
with the reduction-to-lowest-terms step being the only new algorithmic content, and that step is a
GCD computation that is itself well within reach of a const-width integer type. This pass did not
locate a recent (last five years), specifically no-heap, const-generic bounded-rational library, and
states that plainly as a negative result rather than inferring one exists; the classical machinery
(continued fractions, the Stern-Brocot tree, Farey sequences) that would underlie such a type is
textbook material with no recent citation attached here, because none was independently verified in
this pass.

## Multi-limb, arbitrary-width integers at a compile-time-known width

This is the one area surveyed here where arvo's existing shape is already the field's converged
answer, not a departure from it. The cryptography and blockchain ecosystems needed exactly the same
thing arvo's `Bits<N, S, Sign>` to `WideBits<BYTES>` escalation already provides (per the arvo agent
instructions: native primitives for `N <= 128`, wide byte-sequence containers beyond that), and two
mature, independently developed Rust libraries land on the identical mechanism. `bnum`
([github.com/isaacholt100/bnum](https://github.com/isaacholt100/bnum)) uses const generics to let a
consumer pick any bit width from 2 up to `2^32 - 1`, signedness, and overflow behaviour (wrap,
saturate, or panic) as compile-time parameters with zero runtime cost, storing the value as an array
of digits internally chunked to whatever width benchmarking showed fastest, which is a direct
structural parallel to arvo's own strategy-marker-driven container dispatch. `crypto-bigint`
([github.com/RustCrypto/crypto-bigint](https://github.com/RustCrypto/crypto-bigint)) parameterises its
`Uint` type over a limb count directly, ships type aliases for common widths (`U128` through `U4096`),
is `no_std`-friendly, stack-allocated, and const-generic by design, and adds the constant-time
guarantee already discussed above as its central design axiom. That two independently maintained,
production-grade libraries in an adjacent field converge on "const-generic-parameterised, stack-only,
limb-array-backed" as the right shape for arbitrary-but-fixed-width integer arithmetic is
corroborating evidence that arvo's own `WideBits` design sits on well-trodden, actively-maintained
ground, not on an idiosyncratic invention. It does not settle any open design question inside arvo;
it confirms the general shape of the answer arvo already gave to a question the field has also asked.

## Division, reciprocal, square root, and the transcendentals: where fixed point actually breaks

Every source in this pass agrees on where fixed-point designs run into real trouble, and it is not
addition or multiplication (both closed-form and exact modulo the chosen overflow policy): it is
division, reciprocal, square root, and anything transcendental (`log`, `exp`, `sin`, `cos`), none of
which has a closed-form fixed-point algorithm and all of which are computed by iteration. Three
iteration families dominate the literature, and they trade off differently enough that the choice
between them is a real design surface, not a settled default.

**Newton-Raphson** iterates `x_{n+1} = x_n * (2 - d * x_n)` for a reciprocal (or the analogous update
for `1/sqrt(d)`), converges quadratically (roughly doubling correct bits per iteration once close),
and is self-correcting: a rounding error introduced partway through the iteration is damped out by
subsequent iterations rather than compounding. A 2024 paper on a 32-bit fixed-point implementation
("Design and Implementation of an Efficient 32-Bit Fixed-Point Newton-Raphson Division-Based
Reciprocal Computing Unit,"
[researchgate.net/publication/381723388](https://www.researchgate.net/publication/381723388))
reports a variable-latency architecture that only needs to compute an extra remainder-correction step
in roughly 9 percent of reciprocal and division cases and 12 percent of square-root and inverse-square-
root cases, which is a concrete, recent (2024) data point on how much of the iteration's worst-case
cost is avoidable in the common case.

**Goldschmidt's algorithm** (Markstein, Peter, "Software Division and Square Root Using Goldschmidt's
Algorithms," *Real Numbers and Computers*, cited via
[semanticscholar.org/paper/5b4217135be4559b5b237251c275d9cf74a47230](https://www.semanticscholar.org/paper/Software-Division-and-Square-Root-Using-Algorithms-Markstein/5b4217135be4559b5b237251c275d9cf74a47230))
restructures the same underlying convergence so that each iteration needs only two independent
multiplications rather than Newton-Raphson's dependent multiply-then-subtract-then-multiply chain,
which pipelines and parallelises better in hardware at the cost of losing Newton-Raphson's
self-correction: an error introduced early in a Goldschmidt iteration is not damped, it propagates,
so implementations need to be more careful about intermediate precision. Michael Morris's practical
FPGA writeup, "Computing Fixed-Point Square Roots and Their Reciprocals Using Goldschmidt Algorithm"
([fpgarelated.com/showarticle/1347.php](https://www.fpgarelated.com/showarticle/1347.php)), is a
worked, motion-controller-targeted fixed-point implementation rather than a purely theoretical one,
and a 2025 paper combines Goldschmidt division with Mitchell's classical logarithm-approximation
multiplication algorithm in one FPGA fixed-point divider design (arXiv:2508.14611), which is a direct
bridge between the division-algorithm literature and the logarithmic-number-system material above:
Mitchell's algorithm is, at root, a cheap hardware approximation of `log2`, and a 2024 paper unifying
multiplication, division, and square root under one binary-logarithm-based hardware design
("A Unified Hardware Design for Multiplication, Division, and Square Roots Using Binary Logarithms,"
[researchgate.net/publication/383683134](https://www.researchgate.net/publication/383683134_A_Unified_Hardware_Design_for_Multiplication_Division_and_Square_Roots_Using_Binary_Logarithms))
makes the connection explicit: the same log-domain trick that makes LNS multiplication cheap also
gives a cheap first approximation for a fixed-point Newton-Raphson or Goldschmidt seed.

**CORDIC** (Volder, Jack E., "The CORDIC Trigonometric Computing Technique," *IRE Transactions on
Electronic Computers*, 1959) is the oldest of the three and the one most purpose-built for fixed
point specifically: every iteration is a fixed-angle rotation implemented as only a shift and an add,
no multiplier required at all, gaining roughly one bit of precision per iteration (linear rather than
quadratic convergence, so it needs more iterations than Newton-Raphson for the same precision) in
exchange for needing no multiply-capable hardware whatsoever. This is why it is the standard choice
on DSP hardware that historically lacked a fast multiplier, and why it remains the standard family
for computing `sin`, `cos`, `atan2`, and related transcendentals in fixed point specifically (rather
than division or square root, which the Newton-Raphson and Goldschmidt families handle better). The
Rust ecosystem has settled on `cordic` as the crate providing this (referenced, alongside the
narrower `fixed_math` and `fixed_trigonometry` crates, via the discussion at
[users.rust-lang.org/t/fixed-point-no-std-sin-cos-calculation/64910](https://users.rust-lang.org/t/fixed-point-no-std-sin-cos-calculation/64910)),
built against the general-purpose `fixed` crate. The `fixed` crate's own history is a useful data point
here, corrected from the framing an earlier revision of this document gave it (see the correction note
at the top of this document): it is not evidence that arvo shares a `generic_const_exprs` constraint,
because arvo does not carry one. The crate's stable 1.x line encodes the fractional-bit count as a
**type**, not a const generic, through `typenum`'s `Unsigned` marker types (`FixedU32<U16>` names a
16-fractional-bit unsigned 32-bit value). Only its 2.0 alpha line moves the fractional-bit count to a
true const generic, and that alpha line is the one that requires the nightly `generic_const_exprs`
feature to build at all, a fact about `fixed`'s own migration path, not a fact about arvo. What the
split between `fixed`'s two lines does confirm, independent of the corrected framing, is that encoding
a bit-width as a type carrying an associated concrete container, rather than as a const-generic
parameter, is precedented, established practice, not an invention specific to arvo's own container
projection. `typenum` plus `generic-array`'s `ArrayLength` trait does the identical thing one level
more generally, and predates Rust's const generics (stabilised in Rust 1.51, 2021) by several years: a
type parameter `N: ArrayLength` carries an associated `ArrayType<T>`, and the crate's own documentation
states the equivalence directly, "consider `N: ArrayLength` to be equivalent to `const N: usize`"
([docs.rs/generic-array/latest/generic_array/trait.ArrayLength.html](https://docs.rs/generic-array/latest/generic_array/trait.ArrayLength.html)).
`fixed`'s 1.x fractional-bit encoding is the same mechanism applied to a scalar bit count instead of an
array length. Neither example is const-generic-shaped, neither ever needed `generic_const_exprs` to
exist, and both predate or run parallel to the identical move arvo's own container-projection sketch
just reproduced: a closed vocabulary of widths becomes an enum of marker types, and the const
expression that used to compute a container from a number becomes an associated type that a trait
resolves from a type.

## Comparison at a glance

| Representation | Fixed cost per value | Const-size / no-heap | Load-bearing cost that arvo's current axes do not name |
|---|---|---|---|
| Fixed point (arvo today) | Uniform ULP, exact `Add`/`Sub` | Yes, by construction | none, this is the baseline |
| Posit / takum (tapered) | Non-uniform ULP by magnitude | Field width fixed, but per-value decode cost varies | decode cost is a function of the value, not the type |
| Block floating point / MX | Shared exponent per block of N | Yes for a fixed block size, but the scale lives at array level, not per value | no arvo axis currently expresses a block-shared parameter |
| Logarithmic (LNS) | Cheap `Mul`/`Div`, costly `Add`/`Sub` | Yes, fixed-width log plus a const correction table | `Add` cost inversion has no strategy-marker analogue today |
| Residue (RNS) | Cheap `Add`/`Mul` per residue, costly compare/div | Yes, fixed modulus set gives a fixed-size residue array | per-operation cost asymmetry has no strategy-marker analogue |
| Interval | Guaranteed enclosure, no correlation tracking | Yes, a plain fixed-size pair | none structurally; only the correlation blow-up is a real cost |
| Affine (bounded) | Enclosure with linear correlation tracked | Yes only if the noise-symbol count is capped at compile time; unbounded in the general formulation | no productised const-generic library found for the bounded case |
| Taylor model (bounded degree) | Enclosure with nonlinear correlation up to order n | Yes only for fixed dimension and fixed degree; no productised no-heap library found | same as affine, one order higher |
| Bounded rational | Exact, until denominator growth is capped | Yes, expressible today as a pair of arvo's own exact-width integers | no reduction-to-lowest-terms primitive currently in arvo's surface |

## What this pass could not find

Stated plainly, per the brief, because a negative result here is load-bearing for whatever the design
round does next. No productised, general-purpose, const-generic, no-heap affine-arithmetic library
was found; the technique is proven at the application layer inside neural-network verification
tooling (zonotopes) but was not found packaged as a standalone numerics primitive with a stated
bounded-symbol-count contract. The same is true, one level further, for degree-bounded Taylor
models: every implementation located targets a garbage-collected or proof-assistant host and none
states a fixed-degree, fixed-dimension, stack-only design goal. No recent (last five years),
specifically const-generic, no-heap bounded-rational-arithmetic library was found; the concept is
straightforward from arvo's own existing primitives, but no prior art confirming someone has already
built and validated it was located. The 2410.14025 paper on target-aware implementation of real
expressions could not be read in full in this pass; its relevance to fixed-point bit-width selection
is suggested by its title only and is not confirmed. The primary sources behind the 2023-to-2024
FPGA LNS multiply-accumulate and table-lookup-MAC claims were not independently retrieved; the claim
is sourced to a secondary aggregator and should be treated as less certain than the directly-fetched
citations elsewhere in this document. No evidence either way was found on whether Herbie's
rewriting method has been applied directly to fixed-point (rather than floating-point) expression
trees; its transferability to arvo's domain is inferred from the method's representation-agnostic
description, not confirmed by a fixed-point-specific application. Finally, this pass ran out of its
web-search allowance partway through and could not complete planned searches on DSP-specific
bit-width-allocation compiler literature (MILP or ILP-based word-length optimisation) beyond what
the fptalks catalogue already surfaced (TAFFO, POP, FPTaylor); that gap is a limitation of this pass,
not a claim that no such literature exists.
