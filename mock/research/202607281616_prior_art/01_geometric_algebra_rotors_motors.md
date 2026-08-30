# Prior art: rotors, motors, and geometric algebra for rotation

**Date:** 2026-07-28
**Kind:** research, not design. Written against `00_context.md`; see that file for provenance and
scope. Nothing here decides anything.

D10 in `202607281220_topic.the-ndim-and-shape-design.md` grounds rotation on rotors extending to
motors, names Gunn's plane-based projective geometric algebra with Dorst and Lasenby as the lineage,
and records that the component-count concern (a computed extent, `1 + n(n-1)/2`) is not a blocker
given the toolchain's unstable-feature posture, with the hlist as the fallback since a bivector basis
is the set of 2-subsets of the axes and so is derivable by a type-level fold rather than arithmetic in
type position. That is the ratified starting point for everything below.

The field split into two mutually citing but organisationally separate communities over the last
decade, and the split matters more than either camp's internal content, because it is the live
disagreement a design round would otherwise have to discover the hard way. One camp (Gunn, de
Keninck, Dorst, Roelfs, the bivector.net site and its associated SIGGRAPH courses) settled on a
convention where vectors represent planes and the highest grade represents points, calling this
plane-based PGA. A second camp (Eric Lengyel, working independently and publishing as rigid geometric
algebra) argues that convention is inverted and a "giant kludge," and ships an alternative he calls
the geometric antiproduct. Both camps ship production code. Both target exactly the rotor-and-motor
rigid-motion problem D10 names. Neither is settled consensus; a design round choosing which
convention to encode is choosing a side in an active argument, not adopting a textbook result.

## The plane-based PGA lineage and its internal disagreement

Gunn's own account traces the homogeneous model to 2011, developed further into what he named
projective geometric algebra by 2017 and 2020 (Charles Gunn, "Geometric algebras for euclidean
geometry," *Advances in Applied Clifford Algebras*, 2017, https://arxiv.org/pdf/1411.6502; Charles
Gunn, "Doing euclidean plane geometry using projective geometric algebra,"
https://www.semanticscholar.org/paper/65155f0f122a888b51479d4ba809e9588748374b). The SIGGRAPH 2019
course, co-taught with de Keninck, is the most-cited entry point (Charles Gunn and Steven De Keninck,
"Geometric algebra and computer graphics," ACM SIGGRAPH 2019 Courses,
https://www.semanticscholar.org/paper/dce9679a71432558c483d3e1b343dad4b7364e21). Gunn's later
elementary treatment of kinematics and rigid body mechanics within PGA covers `n = 2` and `n = 3` in
detail and names the properties D10 leans on: uniform representation of points, lines, and planes,
robust parallel-safe meet and join, a single sandwich form for isometries, native automatic
differentiation, and tight integration of kinematics with rigid body mechanics.

Inside this camp there is a genuine and documented technical fork, not merely a difference of
emphasis. Roelfs and de Keninck's "Graded Symmetry Groups: Plane and Simple" (*Advances in Applied
Clifford Algebras*, 2023, originally arXiv July 2021, https://arxiv.org/abs/2107.03771) proves the
invariant decomposition theorem: any composition of `k` linearly independent reflections decomposes
into `⌈k/2⌉` commuting factors, each a product of at most two reflections. This is the structural
result a rotor or motor rests on (a rotor is literally a product of an even number of reflections, and
the commuting-factor decomposition is what later underwrites closed-form logarithms). The paper argues
this graded structure is invisible to the matrix Lie algebra picture and is the actual justification
for treating rotation as a geometric-algebra object rather than as a matrix from the start, which is a
sharper and more recent claim than the textbook "quaternions are the even subalgebra" framing D10
already states.

Eric Lengyel, publishing independently at rigidgeometricalgebra.org and projectivegeometricalgebra.org,
disagrees with the bivector.net convention on three concrete points, laid out in "Projective Geometric
Algebra Done Right" (https://terathon.com/blog/pga-done-right.html) and "Dual Approaches to Projective
Geometric Algebra" (https://terathon.com/blog/dual-pga.html), with the full treatment in "Foundations
of Projective Geometric Algebra" and "Projective Geometric Algebra and Rigid Transformations"
(https://terathon.com/foundations_pga_lengyel.pdf, https://www.terathon.com/pga_rigid_transformations_lengyel.pdf).
First, dimensional assignment: the bivector.net convention represents points by the highest-grade
element (trivectors in 3D PGA) and planes by vectors, which Lengyel calls backwards, arguing that in
4D projective space points should be one-dimensional objects and planes three-dimensional ones.
Second, the wedge and antiwedge operations invert their conventional Grassmann meaning under the
bivector.net scheme, where wedge becomes a dimension-decreasing meet and antiwedge a
dimension-increasing join, the opposite of standard usage. Third, normalisation: Lengyel's convention
requires the components involving the projective basis element to have unit magnitude, while the
alternative requires the components that exclude it to be unit, which he presents as a symptom of the
same inversion rather than an independent complaint. He characterises the alternative as forcing
"turning the algebra on its head." NASA maintains a MATLAB implementation of G(3,0,1) explicitly built
on Lengyel's convention (https://github.com/nasa/Rigid-Geometric-Algebra), which is evidence the
disagreement has consumers on both sides rather than being a purely rhetorical dispute. Neither side
has retracted; both conventions are live in shipping code as of this pass. A design round adopting
plane-based PGA per D10 is adopting the bivector.net side of this fork specifically, and should say so
rather than treating "PGA" as one settled thing.

## The Dorst / Lasenby / Doran line

Two textbooks anchor the field and are frequently cited together without being the same book. Leo
Dorst, Daniel Fontijne, and Stephen Mann, *Geometric Algebra for Computer Science: An Object-Oriented
Approach to Geometry* (Morgan Kaufmann, revised edition), builds GA as a natural extension of linear
algebra aimed at a computer-science and graphics reader, and its revised edition covers the conformal
model in a five-dimensional representation space in detail. Chris Doran and Anthony Lasenby,
*Geometric Algebra for Physicists* (Cambridge, 2003, full text at
http://deferentialgeometry.org/papers/Doran,%20Lasenby%20-%20Geometric%20Algebra%20for%20Physicists%20(2003).pdf),
targets a physics reader and treats rotors as the generalisation of the complex-number rotation trick
to arbitrary signature, including Lorentz and conformal transformations, which is closer to D10's own
framing (the rotor as the object that generalises the complex numbers at rank 2 and the quaternions at
rank 3). No specific published feud between these two textbooks surfaced in this pass; where Dorst's
line and the Doran and Lasenby line diverge is emphasis (computer science and incidence geometry
versus physics and Lie-algebra structure) rather than a contradicted technical claim, and this pass
did not find one. Dorst himself sits on the plane-based PGA side of the fork above: he co-authored the
normalisation and exponential-map paper cited below with de Keninck, so "Dorst" in D10's lineage list
is best read as pointing at that later, PGA-specific body of work rather than at the 2007 textbook's
conformal-model chapters.

Lasenby continues active output specifically on the rotor-reconstruction problem: "Reconstructing a
rotor from initial and final frames using characteristic multivectors" (*Mathematical Methods in the
Applied Sciences*, 2024, https://onlinelibrary.wiley.com/doi/full/10.1002/mma.8811), which is closer
to the kind of estimation problem a real consumer (fitting a rotor to observed frame pairs) would
actually hit, and worth flagging as a recent, unsettled corner rather than textbook material.

## Conformal GA and what it buys over PGA

Conformal geometric algebra (CGA) embeds Euclidean space in a five-dimensional representation with
two added null dimensions, against PGA's single added dimension. CGA's added expressiveness is round
objects (spheres, circles) as first-class blades and a wider transformation group including dilation
and inversion, not merely rigid motion. gafro, a robotics library explicitly benchmarked against the
fastest known GA libraries (Gaël Löw et al., "gafro: Geometric Algebra for Robotics," 2023,
https://arxiv.org/abs/2310.19090, code at https://github.com/idiap/gafro), chose CGA over PGA for
exactly this reason: manipulator kinematics wants spheres and circles as native objects for
reachability and collision primitives, not only rigid transforms. It reports competitive performance
against GATL and Versor, two libraries independently identified as the previous fastest in the field.
D10's scope is rotation and rigid motion specifically (`Affine` as rigid motion plus scale), which is
exactly the case where PGA is the narrower, cheaper tool and CGA's extra two dimensions and round
primitives are unused weight. This pass found no argument in either direction that PGA is
insufficient for D10's stated scope; the tradeoff is real and documented but does not bear against the
D10 choice as stated.

## Implementation practice: hand-tuned fixed algebras versus code generation

There is a real bimodal split in how shipping GA code gets written, and it maps directly onto the
question arvo has to answer (a hand-specialised container per dimension versus a generic engine).

**Hand-specialised, fixed dimension and signature.** Jeremy Ong's klein
(https://github.com/jeremyong/klein, docs at https://www.jeremyong.com/klein/) targets exactly
`P(R*_{3,0,1})`, 3D PGA, and nothing else. It hand-works out every non-zero product entry and
implements the algebra with SSE, achieving no runtime overhead by fixing the algebra, the dimension,
and the instruction set simultaneously, in exchange for zero generality: klein cannot be retargeted to
a different dimension or signature without a rewrite. It ships `Motor::from_rotor_and_translator()`
and applies rotors and motors to planes, points, and lines within one unified sandwich operation.
Lichtso's Rust crate `geometric_algebra` (https://docs.rs/geometric_algebra,
https://crates.io/crates/geometric_algebra) takes the same shape in Rust: it pre-generates concrete
modules for the 1D through 3D Euclidean, hyperbolic, and positive-projective algebra families (EPGA,
HPGA, PPGA) rather than exposing a single generic engine, ships a `simd` module, and its
`Transformation` trait implements the sandwich product `self * other * self.reversion()` directly
against those fixed types. As of this pass it is pre-1.0 (0.3.0), thinly documented, and last touched
recently but by a single maintainer, which matters for how much this pass should lean on it as proof
of a mature pattern versus proof the pattern is merely tried.

**Code generation from a symbolic description of the algebra.** Ganja.js (Steven De Keninck,
https://github.com/enkimute/ganja.js, https://enkimute.github.io/ganja.js/) is a Clifford-algebra code
generator: given a signature, it produces JavaScript, C++, C#, Rust, or Python classes at generation
time, generates product code lazily on first use, and performs coefficient-level symbolic
optimisation including common-subexpression elimination and variable prefetching, paying the symbolic
analysis cost once rather than per call. Garamon (Stéphane Breuils, Vincent Nozick, Laurent Fuchs,
"Garamon: A Geometric Algebra Library Generator," *Advances in Applied Clifford Algebras*, 2019,
http://www-igm.univ-mlv.fr/~vnozick/data/garamon.pdf) is a C++ template library generator with the
same shape: it stores only the non-zero blades a given object's structure permits, precomputes
products for low dimension and switches to recursive product computation for higher dimension because
recursion has better asymptotic complexity than the naive per-element approach, and is reported
competitive with GATL and Versor, the same two libraries gafro benchmarks against, though its own
numbers degrade sharply at 16 dimensions (tens of seconds per operation in the published benchmark),
which is a useful data point on where naive generic-dimension code generation stops paying off.
GAALOP takes the code-generation idea further still, converting a GA expression to optimised
coefficient-level code across C++, C#, Rust, CUDA, and Julia targets for algebras up to 32 dimensions.

The most recent and most directly relevant entry is kingdon (Martin Roelfs, "The Willing Kingdon
Clifford Algebra Library," 2025, https://arxiv.org/abs/2503.10451, code at
https://github.com/tBuLi/kingdon, docs at https://kingdon.readthedocs.io/). Kingdon is
input-type-agnostic (it runs the same symbolic pipeline over PyTorch tensors, NumPy arrays, or SymPy
symbols) and its central technique is symbolically optimising the unary and binary GA operators and
then leveraging the sparsity of the specific input at hand to just-in-time compile an expression of
optimal computational complexity for that sparsity pattern, rather than for the algebra's full dense
`2^n` basis. Eelco Hoogendoorn's numga (https://github.com/EelcoHoogendoorn/numga), built on JAX and
NumPy, names the same tradeoff explicitly from the other direction: it offers a choice between sparse
and dense execution of an operator, where dense execution produces a much smaller compute graph to
compile but performs many multiplications by zero at runtime, while unrolling over only the non-zero
terms produces a larger graph that is faster to run. This is the same choice arvo's own strategy-axis
philosophy already names for unrelated reasons (arvo ships tools and does not pick the threshold on
the consumer's behalf), and it is worth noting that the field's own state of the art treats "which
sparsity strategy" as a per-workload choice rather than a settled default, which is corroborating
rather than novel evidence for that stance.

**Rust crates targeting genuinely generic dimension.** Two crates attempt what a hlist-of-capacities
shape would need: dimension as a compile-time parameter rather than a fixed hand-tuned target.
`wedged` (https://docs.rs/wedged, https://lib.rs/crates/wedged) uses `typenum` rather than bare const
generics for its dimension parameter, giving types like `VecN<T, N>` and `Blade<T, N, G>` with trait
bounds such as `T: AllocBlade<N, U1>` to express dimensionally polymorphic code; it is generic over
scalar type down to requiring only `ClosedAdd` and `ClosedDiv`-style bounds, and its rotor type is
explicitly documented as unifying complex numbers and quaternions across all dimensions, which is the
same claim D10 makes about rotors generalising. It shows no explicit `no_std` support and is early
(0.1.1). `amari-core` (https://docs.rs/amari-core, repository at
https://github.com/justinelliottcobb/Amari) is the closer structural match to arvo's own constraints:
it supports arbitrary signature `Cl(P, Q, R)` via generic parameters, builds an optimised Cayley table
per signature, supports `no_std`, and ships AVX2-accelerated paths on x86/x86_64. Neither crate has
reached a stable release, and this pass found no independent published benchmark of either against
the fixed-dimension libraries above; their existence establishes the pattern is attempted in Rust
specifically, not that it is proven competitive with a hand-specialised target like klein.

The overall shape of the field, as of this pass: nobody ships one library that is simultaneously
generic over dimension at compile time, exploits per-object sparsity rather than the dense `2^n`
basis, and hand-tunes to SIMD the way klein does for its one fixed target. The three properties trade
against each other in every library surveyed here, and a hlist-of-capacities approach generic over
rank, generating the bivector basis as a type-level fold of 2-subsets rather than storing it, would be
attempting a combination the survey did not find already shipped, which is worth flagging as the
actual open engineering question rather than a solved problem being ported in.

## Numerical behaviour under composition and renormalisation

The standard, textbook-level comparison holds and needs only a short paragraph. A rotation matrix
carries nine entries for three degrees of freedom (six redundant constraints), so repeated
composition under floating-point rounding drifts away from orthogonality and needs a re-orthogonalisation
step, typically Gram-Schmidt or an SVD-based polar decomposition, which is itself a nontrivial and
comparatively expensive operation. A quaternion carries four components for three degrees of freedom
(one redundant constraint), stays close to the unit sphere under repeated multiplication because
there are fewer numbers to drift, and its renormalisation is a single scalar divide. A three-dimensional
rotor is algebraically the even subalgebra element isomorphic to a unit quaternion, so its drift and
renormalisation behaviour under composition is the same as a quaternion's, not better or worse; the
orthogonality constraint the rotor's algebra enforces is structurally the same constraint a quaternion's
unit-norm condition enforces. Rotor renormalisation is described in the literature as the multivector
analogue of the same Gram-Schmidt or SVD idea applied to matrices, scaled down to the rotor's smaller
component count.

Where the literature goes further than the textbook comparison, and where D10's "the component count
needs care" framing is directly addressed, is de Keninck and Dorst's "Normalization, Square Roots, and
the Exponential and Logarithmic Maps in Geometric Algebras of Less than 6D" (*Mathematical Methods in
the Applied Sciences*, 2022, https://arxiv.org/abs/2206.07496). This paper is a signature-agnostic
treatment of exactly the operations a rotor-and-motor implementation needs (normalise, take a square
root, exponentiate a bivector into a rotor or motor, take the logarithm of a rotor or motor back to a
bivector) across every geometric algebra of dimension under 6, with efficient closed-form
implementations named specifically for the algebras a graphics or physics consumer actually uses
(`R_4`, `R_{3,1}`, `R_{3,0,1}`, `R_{4,1}`), stated as intended to lower the adoption barrier for
software maintainers who would otherwise reach for the quaternion or matrix baseline out of habit. Its
existence as a 2022 paper, four years after Gunn's PGA framing and three years after klein, is itself
informative: the closed-form machinery for making rotors and motors as cheap to normalise and
interpolate as quaternions is recent, not textbook-settled, and D10's stated fallback (the hlist
derives the bivector basis rather than computing it in type position) sidesteps the representational
half of this problem without addressing the numerical half, which this paper is the current reference
for.

## How the sparse structure of a multivector gets exploited

D10 already states the arithmetic fact correctly: a full multivector in an `n`-dimensional algebra has
`2^n` basis blades, which is unaffordable to store or compute densely past small `n`, while a rotor
needs only the even-grade blades and specifically only a scalar plus `n(n-1)/2` bivector components.
Every library surveyed above treats this gap between the dense basis and the object's actual grade
structure as the central performance question, and answers it one of three ways. Klein and Lichtso's
`geometric_algebra` fix the dimension ahead of time and hand- or generation-time-enumerate exactly the
non-zero products for that one fixed structure, paying zero runtime cost for the sparsity because the
sparsity pattern is baked into the generated code and never inspected at runtime. Garamon and GAALOP
generate per-signature code at build time from a symbolic description, storing only the blades a given
object type's grade permits and switching representation strategy (precomputed tables at low
dimension, recursive computation at high dimension) as a function of the signature given to the
generator. Kingdon and numga defer the sparsity decision to the actual runtime shape of the input,
symbolically specialising an expression to the specific sparsity pattern the caller's data exhibits
(numga's explicit sparse-versus-dense JAX compute-graph tradeoff is the clearest statement of this:
sparse execution produces a smaller runtime cost but a larger graph to compile, and the choice is
exposed to the caller rather than picked once for the whole library). None of the three approaches is
free: the first pays in generality (a new dimension needs a new hand-tuned target), the second pays in
build-time generation infrastructure, the third pays in either JIT warmup cost or a larger compiled
artifact. For a `#![no_std]`, const-generic, monomorphisation-is-the-dispatch stack, the closest
structural analogue among these is Garamon and GAALOP's build-time generation from a signature, not
kingdon's runtime JIT, since arvo's whole model is that the compiler, not a runtime symbolic engine,
resolves the sparsity pattern per instantiation. This pass found no library that does what a
hlist-typed rotor would do (derive the sparsity pattern as a type-level fold over the axis list at
monomorphisation time, with zero generation-time tooling and zero runtime dispatch), which is again
the open question rather than a ported solution.

## Fixed point and integer arithmetic: a negative result

This pass found no published work on general-purpose fixed-point or scaled-integer geometric algebra
arithmetic, in the sense arvo means by fixed point (a `UFixed`/`IFixed`-style scaled integer as the
primary representation for rotor and motor components, replacing float throughout). Every rotor and
motor implementation surveyed above, without exception, uses IEEE floating point as its component
type. This is worth stating as a real gap rather than papering over with adjacent results, because two
adjacent bodies of work look at first glance like they might answer the question and do not.

Lipschitz and Hurwitz integer quaternions are a genuine, well-studied number-theoretic construct: a
Lipschitz quaternion has all-integer components, a Hurwitz quaternion has all-integer or all-half-integer
components, and Hurwitz integers admit Euclidean division and support a theory closely paralleling
algebraic number theory (see, for instance, the treatment in "Some Connections Between the Arithmetic
and the Geometry of Lipschitz Integers," https://arxiv.org/pdf/1201.5817). The unit Lipschitz
quaternions form the order-8 quaternion group `{±1, ±i, ±j, ±k}`; the unit Hurwitz quaternions extend
this to the order-24 binary tetrahedral group. This is real integer arithmetic on quaternions, but it
realises only a finite, discrete set of exact rotations (the symmetries of specific regular
polytopes), not an arbitrary rotation approximated to a fixed number of fractional bits the way a
`UFixed<I, F, S>` rotor component would be. It answers a different question (which exact rotations
does a lattice of integers realise) than the one D10's constraint raises (how does an arbitrary rotor
behave under scaled-integer arithmetic with rounding).

FPGA and coprocessor hardware for Clifford algebra is the second adjacent body of work, and it is
adjacent by name only. Franchini and Gentile's line of work (CliffoSor, Quad-CliffoSor, ConformalALU;
see "Fixed-Size Quadruples for a New, Hardware-Oriented Representation of the 4D Clifford Algebra,"
*Advances in Applied Clifford Algebras*, 2010, https://link.springer.com/article/10.1007/s00006-010-0258-0,
and the FPGA multiplier paper at https://ieeexplore.ieee.org/document/4669311/) reports real hardware
speedups, 23 times for the Clifford product and 33 times for sums and differences against a software
library baseline, but "fixed-size" in this literature names a compact storage layout (mapping a
variable-length multivector into a fixed four-component quadruple structure for a specific 4D
algebra), not fixed-point number representation. The quadruple components are not stated to be scaled
integers in the sources surveyed; the paper title's "fixed" is doing algebraic-structure work
(bounding the storage shape to enable simple hardware), not numeric-representation work. Reading past
the shared word "fixed" into the actual papers is what separates this from a real answer to the
question.

So the honest statement is a negative result: nobody in this survey has published a general treatment
of fixed-point geometric algebra, comparable to the abundant literature on fixed-point quaternion
arithmetic in embedded and DSP contexts (Q-format quaternion libraries exist and are common in
embedded 3D and orientation-filter work, though this pass did not find a peer-reviewed treatment of
their error behaviour comparable to the float case). A design round building fixed-point rotors is
building something without a literature to check its numerical claims against, not something with an
established practice to follow. The relevant open questions (how a bivector's rounding error
propagates through the exponential map into a rotor, how renormalisation behaves in scaled-integer
arithmetic given that a rotor's unit-norm constraint is a nonlinear condition on limited-precision
values, whether the `1 + n(n-1)/2` component count changes the error-accumulation story relative to a
quaternion's fixed four components) are open, not merely under-surveyed.

## Generic-over-dimension geometric algebra in a type system, particularly Rust

Covered in structural detail above; the summary specific to "in a type system" is short. `wedged`'s
`typenum`-based dimension parameter and `amari-core`'s const-generic signature parameter are the two
Rust attempts at reifying dimension as a compile-time type-level quantity rather than a runtime field,
and both are pre-1.0 with no independent adoption evidence found in this pass. Outside Rust, Garamon
and GAALOP achieve the same generality by code generation from a build-time-supplied signature rather
than by the host language's own type system expressing genericity, which sidesteps whatever the host
language's generic system cannot express (Garamon's own paper notes it switches from precomputed
tables to recursive computation as dimension grows, which is a strategy decision the generator makes
about the target code, not something the generic type system decides for the generator). This pass
found no prior art for expressing a bivector-count-parameterised rotor type as a type-level fold over
a heterogeneous axis list the way D10's fallback proposes; the closest analogues (`wedged`'s
`Blade<T, N, G>` and `amari-core`'s `Cl<P, Q, R>`) parameterise dimension and grade as bare numbers
rather than deriving the bivector set structurally from a list of axes, so the specific mechanism
arvo's hlist would use is, as far as this survey found, unattempted rather than merely rare.

## Interpolation

Slerp on unit quaternions is textbook material and needs no further citation here; it is the baseline
D10 implicitly measures against. The rotor equivalent is the same operation under a different name and
is not a separate invention: since a three-dimensional rotor is algebraically a unit quaternion, slerp
between two rotors is slerp, computed via the same exponential-and-logarithm-map machinery de Keninck
and Dorst formalise generically in the 2022 paper cited above. The genuinely separate requirement,
because D10 names motors explicitly and motors cover translation as well as rotation, is the motor
equivalent, which is screw linear interpolation (ScLERP). ScLERP generalises slerp from the unit
sphere to the dual-quaternion (equivalently, 3D-motor) manifold, using the same exponential of a power
of the dual quaternion, and produces constant angular and translational velocity along a screw motion
between two rigid poses, in contrast to naive linear interpolation of dual-quaternion components,
which does not hold constant velocity (Ben Kenwright, "Dual-Quaternion Interpolation," 2023,
https://arxiv.org/pdf/2303.13395; a Python reference implementation and worked derivation is at
https://github.com/apat20/PyScLERP). Since a 3D motor and a unit dual quaternion are the same object,
ScLERP for motors is not a new algorithm to invent, it is the existing dual-quaternion result restated
in GA vocabulary, and the general-dimension motor case again reduces to the same de Keninck and Dorst
exponential and logarithmic map machinery, since a motor's bivector generator is exponentiated the
same way a rotor's is, just over a larger (rotation-plus-translation) bivector.

## What this pass did not find

Stated plainly, per the brief. No general fixed-point or scaled-integer geometric algebra literature
exists as far as this survey found; the two adjacent bodies of work (Lipschitz and Hurwitz integer
quaternions, and FPGA "fixed-size" Clifford hardware) answer different questions and should not be
read as filling this gap. No independent third-party benchmark comparing `wedged` or `amari-core`
against a hand-specialised library like klein was found; their existence proves the const-generic and
typenum-generic patterns are attempted in Rust, not that either is competitive. No published dispute
between the Dorst-Fontijne-Mann textbook and the Doran-Lasenby textbook was found; the real, sourced
disagreement in this field is the Gunn and de Keninck bivector.net convention against Lengyel's rigid
geometric algebra convention, and that is the one a design round should treat as live. No prior art
was found for deriving a rotor or motor's bivector-component set as a type-level fold over a
heterogeneous axis list rather than as a bare numeric parameter; every generic-dimension
implementation surveyed, in Rust or elsewhere, parameterises by a dimension number rather than by a
structural list. That absence cuts both ways: it is not evidence the hlist approach is wrong, only
that it would not be following an established pattern if a design round adopts it, and any claim about
its numerical or compile-time behaviour would need to be established by benching arvo's own
implementation rather than by citation.
