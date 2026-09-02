# Nonlinear algebra, low-rank approximation, and structured matrices

**Date:** 2026-07-28
**Kind:** prior-art research, not design
**Governing canon:** `arvo/mock/design_rounds/202607281220_topic.the-ndim-and-shape-design.md`, decision D11.
Nothing below decides anything. Where two approaches differ, the question that separates them is
named and left open for the design round.

D11 records op's hunch about an unconventional win in curve representation, coming from the
nonlinear algebra side, attached to a name not yet recalled, and instructs research before any
direction is committed. This file has two halves. The first tries to name the person. The second
surveys what nonlinear algebra and structured numerical linear algebra actually offer a fixed-point,
allocation-free, const-sized foundation, independent of whether the name is ever recovered, because
the substantive question stands on its own.

## Part one: the name

Three candidates were already rejected as not it: Hackbusch (hierarchical matrices), Heckbert
(adaptive subdivision, flatness testing), Hausdorff (the curve-approximation error metric). Five more
were offered as unconfirmed leads: Eckart, Sturmfels, Hankel, Higham, Householder. A pattern worth
naming before the search: of the eight names on the table, six begin with H, and Hackbusch, Heckbert,
and Hausdorff (all three already ruled out) are joined by Hankel, Higham, and Householder among the
unconfirmed. Memory recall for a half-remembered name commonly clusters on the sound of names already
active in working memory, so an H-cluster search is worth running even though it was not asked for
directly.

### Eckart-Young-Mirsky: the strongest fit for "bounded optimal"

The theorem: for a matrix A with singular value decomposition, the best rank-k approximation in the
Frobenius norm or the spectral (2-) norm, among all matrices of rank at most k, is obtained by
truncating the SVD to its k largest singular values and vectors. The bound is exact and closed-form:
the 2-norm error equals the (k+1)-th singular value, and the Frobenius-norm error equals the root of
the sum of the squares of the discarded singular values. Leon Mirsky's 1960 generalisation extends the
result to every unitarily invariant norm, which is why the theorem is usually cited under all three
names together.

This is a textbook match for op's "bounded optimal" recollection: it is an approximation that is
simultaneously provably optimal (no rank-k matrix does better, in any of the norms the theorem covers)
and comes with an explicit, computable bound (the discarded singular values, known exactly once the
SVD is computed). Carl Eckart and Gale Young's original 1936 paper is the origin; Mirsky's 1960 paper
is the generalisation. Eckart himself was a physicist, not primarily a numerical linear algebraist,
which may be why the recollection attaches to a name without a strong secondary association to
nonlinear algebra as a field. See the theorem's standard modern statement and proof structure at
[Mirsky's theorem, Bohrium](https://www.bohrium.com/en/sciencepedia/feynman/keyword/mirsky_s_theorem)
and a worked derivation of the bound at
[A self-contained proof for Eckart-Young-Mirsky, 2020](https://kathysmindpalace.wordpress.com/2020/03/15/a-self-contained-proof-for-eckart-young-mirsky-theorem-in-low-rank-approximation-problems/).

Eckart-Young-Mirsky is decades-old, universally taught, and about matrices in general, not curves
specifically. If the recollection is "the nonlinear algebra side offers a bounded optimal
approximation" as a general principle applicable to the curve-representation problem (a curve's
control-point matrix, or a discretised sample matrix, admits a provably optimal low-rank summary),
Eckart is the strongest single-name match on the table. It is not, on its own, a curve-representation
result; it would have to be applied to curves rather than being a curve theorem itself.

### Sturmfels: central to the field, not obviously "bounded optimal"

Bernd Sturmfels is the correct answer to "who is the central figure in nonlinear algebra" as a named
field: with Mateusz Michałek he wrote the textbook that gave the field its current name, *Invitation
to Nonlinear Algebra* (AMS Graduate Studies in Mathematics 211, 2021), covering the Nullstellensatz,
primary decomposition, tropical geometry, and semidefinite programming across thirteen chapters. See
[the AMS bookstore listing](https://bookstore.ams.org/gsm-211/) and
[the book announcement](https://www.mathematik.uni-konstanz.de/working-group-real-geometry-and-algebra/news/news/meldungsdetails/the-invitation-to-nonlinear-algebra-book-project-with-bernd-sturmfels-is-now-published/).
Sturmfels is also the leading figure behind the *Numerical Nonlinear Algebra* survey line (see
[arXiv:2302.08585](https://arxiv.org/pdf/2302.08585)), which is the bridge between symbolic algebraic
geometry and the numerical homotopy-continuation methods covered in Part Two below.

He fits "central figure in nonlinear algebra" exactly, which is presumably why he was already on the
list. He does not obviously fit "bounded optimal approximation" as a specific technical recollection:
his work is about solving and describing algebraic varieties, not about approximation-with-a-bound in
the sense Eckart-Young-Mirsky provides. If the recollection is about the field rather than a specific
theorem, Sturmfels is the right anchor; if it is about a specific bounded-optimality result, he is
probably not it.

### Hankel, Higham, Householder: the three unconfirmed H-names, none a strong match

**Hankel** matrices (constant along anti-diagonals) are central to system identification and
signal-processing rank estimation: a system's order is literally the rank of its Hankel matrix of
impulse responses, and rank-minimisation of a Hankel matrix (via nuclear-norm relaxation) is a
well-studied convex-relaxation problem. See
[Hankel Matrix Rank Minimization, Fazel/Pong/Sun/Tseng, 2012](https://optimization-online.org/2012/08/3587/)
and the review
[Hankel low-rank approximation and completion, 2022](https://arxiv.org/pdf/2206.05103). This is a
genuine "low-rank plus a bound" story (nuclear norm relaxation gives a computable certificate), but it
is a specific application domain (system identification, time series), not a general theorem, and
nobody in the search results attaches "bounded optimal" language to a Hankel-named result the way
Eckart-Young-Mirsky carries it.

**Higham** (Nicholas J. Higham) is the author of the standard references *Accuracy and Stability of
Numerical Algorithms* and *Functions of Matrices: Theory and Computation* (see
[nhigham.com](https://nhigham.com/functions-of-matrices-theory-and-computation/)). His work is about
backward-error and forward-error bounds for numerical algorithms broadly, which is "bounded" in a
real sense, but it is a whole research program (matrix functions, mixed-precision algorithms,
conditioning) rather than one theorem, and it is not specifically about optimal low-rank
approximation or about curves. Weaker fit than Eckart.

**Householder** (Alston Householder) is the namesake of the Householder reflection, the standard
building block of numerically stable QR factorisation. Every Householder reflector is exactly
orthogonal and the resulting QR factorisation is backward stable (see
[Householder Reflections, CME 302 course notes](https://ericdarve.github.io/NLA/content/householder_reflections.html)
and the practical stability discussion at
[the MathWorks Householder QR post](https://blogs.mathworks.com/cleve/2016/10/03/householder-reflections-and-the-qr-decomposition/)).
This is a stability result about a factorisation method, not an optimality-with-a-bound result about
approximation. Weakest fit of the three unconfirmed names for "bounded optimal approximation"
specifically, though QR itself is load-bearing machinery for several things covered in Part Two.

### A candidate not on op's list, closest to the actual context: Farouki

D11's actual subject is curve representation (Euler spirals, biarcs, implicit fields, Beziers), and
the strongest match for "unconventional win from the nonlinear algebra side, applied to curves, with
a provable bound" that turned up in the search is not any of the eight names offered. It is **Rida T.
Farouki**, whose decades of work on Pythagorean-hodograph (PH) curves is explicitly built on
quaternion algebra (a genuinely nonlinear-algebraic structure: PH curves are constructed by squaring
quaternion or complex generator polynomials, which is where the "Pythagorean" name comes from, since
squaring forces the hodograph's components to satisfy a Pythagorean identity and gives the curve a
polynomial, rather than merely rational, arc-length function). See the survey
[Pythagorean Hodograph Curves: A Survey of Recent Advances](https://www.researchgate.net/publication/267024627_Pythagorean_Hodograph_Curves_A_Survey_of_Recent_Advances)
and the textbook *Pythagorean-Hodograph Curves: Algebra and Geometry Inseparable* (Farouki, Springer,
2008; the title itself states the connection between algebra and curve geometry that D11 gestures at).
The specific "bounded optimal" language shows up directly in this literature: PH interpolants achieve
proven optimal approximation order (order 6 for the general case,
[per the 2023 survey on optimal spatial PH interpolation](https://arxiv.org/pdf/2302.04632)), and
there is a dedicated line of work on L2-optimal approximation of general curves by PH curves with a
computable error bound (see
[On L2 approximation by planar Pythagorean-hodograph curves](https://www.researchgate.net/publication/388873184_On_L_2_approximation_by_planar_Pythagorean-hodograph_curves)).

This is offered as a finding, not a decision: if the recollection is about curves specifically rather
than about matrix approximation in general, Farouki is a materially better fit than any of the eight
names on the table, because PH curves are exactly the candidate D11's own bench matrix is missing (a
representation with an exact, non-approximate arc length, unlike Beziers, and a provable, bounded
approximation order, unlike a plain biarc fit). Whether this is what op actually recalled cannot be
settled from outside op's memory. It is recorded because it satisfies the recollection's content
("nonlinear algebra", "bounded optimal", applied to curves) more precisely than the confirmed rejects
or the unconfirmed leads do.

### Ranked shortlist

1. **Eckart** (Eckart-Young-Mirsky). Strongest literal match for "bounded optimal approximation" as a
   named theorem. General matrix result, not curve-specific; would need to be applied to a curve's
   discretisation or control structure to become relevant to D11's actual question.
2. **Farouki** (Pythagorean-hodograph curves, not on the original list). Strongest match for the
   *context* (curves, nonlinear algebra, a genuine unconventional win, a real bound), weaker match for
   the specific phrase "bounded optimal" as a theorem name, since the field usually says "optimal
   approximation order" rather than "bounded optimal approximation."
3. **Sturmfels**. Correct as "the field's central figure," not obviously a "bounded optimal" result.
4. **Hankel**. A real bounded-rank story (nuclear-norm relaxation for Hankel structure), but tied to a
   specific application domain rather than carrying the theorem's name the way Eckart-Young does.
5. **Higham, Householder**. Both real and load-bearing to Part Two below, neither is a strong
   "bounded optimal approximation" theorem-name match.

The recollection may also be of something not on this list at all. Two more candidates surfaced by
the search that carry a similarly strong claim to "bounded optimal" and were not named in D11: **Leon
Mirsky** himself (the generalisation half of the Eckart-Young-Mirsky theorem, sometimes cited alone as
"Mirsky's theorem" for the unitarily-invariant-norm case), and the **Kolmogorov n-width** (not a
person's surname but a named quantity, covered in Part Two, that is literally the definition of the
best possible bounded approximation error achievable by any n-dimensional linear method for a given
function class). If op's recollection is of a concept rather than strictly a single surname, Kolmogorov
n-width is worth having in the room during the design round, because it answers "how good can any
curve-approximation scheme with n degrees of freedom possibly be" independent of which representation
gets picked.

## Part two: what the field offers a fixed-point, allocation-free, const-sized foundation

This half stands regardless of whether Part One's name search lands. It surveys the literature by
axis, in the order op's brief names them, with the last-five-years and known-but-unproductised work
weighted over settled consensus per the shared context document.

### Low-rank approximation and its optimality bounds

Eckart-Young-Mirsky (above) is the classical answer and it is the standard against which everything
newer is measured. What has moved in the last fifteen years is how the approximation is computed, not
what optimality means.

**Randomised SVD** (Halko, Martinsson, Tropp, "Finding Structure with Randomness," SIAM Review 53(2),
2011, [PDF](https://users.cms.caltech.edu/~jtropp/papers/HMT11-Finding-Structure-SIREV.pdf)) is now
the field's default for large matrices: project onto a random low-dimensional subspace, orthonormalise,
then run a small deterministic SVD on the projected data. The paper proves tight probabilistic error
bounds relative to the true Eckart-Young-Mirsky optimum, not exact optimality, and as of the last
description found, remains the single most cited resource in randomised numerical linear algebra. The
bound is a probabilistic near-optimality guarantee, which is a genuinely different shape of "bounded"
than Eckart-Young-Mirsky's exact one, and worth distinguishing if the design round wants "bounded" in
the strict sense.

The five-years-and-unproven layer sits above randomised SVD, not in it: **randomly pivoted LU** (Gilles
and Wilber, 2026, [arXiv:2601.22344](https://arxiv.org/pdf/2601.22344)) samples pivots proportional to
the squared entries of the Schur complement, proves geometric expected convergence for matrices with
rapidly decaying singular values, and needs only O(k squared + m + n) storage for a rank-k
approximation rather than the O(mn) an SVD needs to even start from. That storage bound is the one
directly relevant to a const-sized foundation: it is expressed entirely in terms that could plausibly
become compile-time constants if k is fixed at the type level. The paper does not claim to beat
randomised SVD in accuracy; it claims comparable accuracy for a fraction of the memory, particularly on
structured (Cauchy-like) matrices. It is a 2026 preprint, unproven in the sense op's brief asks to be
told about: not yet a settled part of the canon, genuinely promising for exactly the allocation
constraint arvo carries.

**Tree tensor network Nyström** (2024, [arXiv:2412.06111](https://arxiv.org/pdf/2412.06111)) extends
streamable, single-pass randomised sketching to the tensor case (see the tensor-format discussion
below); relevant here because it is single-pass, meaning it never needs the full tensor materialised
at once, which is the tensor-domain analogue of the memory story randomly pivoted LU tells for
matrices.

**The rank-must-be-known-at-compile-time question.** Every construction above (Eckart-Young-Mirsky
truncation, randomised SVD, randomly pivoted LU) is naturally described with the target rank k as an
input the caller supplies, which maps cleanly onto a const generic. What none of them naturally give
is a compile-time-computable k for a given accuracy target: the rank needed to hit a given error bound
depends on the actual singular value decay of the actual data, which is a runtime quantity. A
const-sized foundation that wants "rank k, guaranteed accuracy" as a type-level contract either fixes
k and accepts whatever accuracy results (which fits arvo's const-generic style directly, and is
exactly what `Array<T, C>` and `Matrix<W, C>` already do for storage) or has to treat the rank-versus-
accuracy tradeoff as a runtime fact the type system cannot pin down, the same way `Cold` versus `Hot`
storage strategy is a runtime-informed but type-level-encoded choice today. This is the sharpest open
question separating "low-rank approximation fits arvo cleanly" from "low-rank approximation needs a
runtime escape hatch," and no result found here resolves it either way.

### CUR, skeleton, and interpolative decompositions

A distinct family worth naming alongside SVD-based methods: **CUR decomposition** (also called
skeleton decomposition) approximates a matrix using an actual subset of its own rows and columns
rather than a synthetic orthonormal basis. See the survey coverage in
[RSVDPACK, Voronin and Martinsson, 2015 line of work](https://arxiv.org/pdf/1502.05366) and the
practical algorithms in
[Efficient algorithms for CUR and interpolative decomposition](https://users.oden.utexas.edu/~pgm/Pubs/2017_voronin_CUR.pdf).
The tradeoff, stated plainly by the field itself: truncated SVD gives the best error (Eckart-Young-
Mirsky optimum), interpolative decomposition and QR-based methods give the same error as each other
and worse than SVD, and CUR gives worse error still, but CUR and ID inherit properties of the original
matrix (sparsity, non-negativity, and, critically for a `no_std` no-allocation foundation,
interpretability of which literal rows and columns were kept). For a foundation that ships sparse and
graph crates already, CUR is worth flagging because "the low-rank factors are literally rows and
columns of the original data" sidesteps the question of what an orthonormal basis vector even means in
fixed-point arithmetic, where accumulated rounding in a Gram-Schmidt-style orthogonalisation is a real
concern that Householder reflections were invented specifically to avoid (see Part One above). Whether
that tradeoff is worth the accuracy loss relative to SVD is exactly the kind of question D11's own
approach (bench the candidates, let the winner be selected per workload) is built to answer rather than
to be argued from first principles.

### Tensor decompositions and formats

**CP decomposition** (sum of rank-one tensors) gives the most compact representation when it exists,
but computing the CP rank, or even verifying a candidate rank-1 factorisation, is NP-hard in general
(see the fine-grained hardness results discussed in
[Near-Linear Time and Fixed-Parameter Tractable Algorithms for Tensor Decompositions, 2022](https://arxiv.org/pdf/2207.07417)).
That NP-hardness is a hard boundary for a compile-time-const design: nothing about arvo's toolchain,
however permissive on nightly features, makes an NP-hard rank-determination problem tractable at
compile time or at runtime for arbitrary input. CP decomposition is usable only where the target rank
is fixed externally (by the workload, by the design) and never discovered by the algorithm itself,
which again maps to a const generic supplied by the caller rather than one derived internally.

**Tucker decomposition** (a small dense core tensor plus one orthonormal factor matrix per mode) is
comparatively tractable: it is the direct multilinear generalisation of the matrix SVD (often called
higher-order SVD, HOSVD), and its multilinear rank (one integer per axis) is far easier to reason
about than CP rank. This maps unusually well onto D4's shape design: a Tucker decomposition's per-mode
ranks are exactly a shape's per-axis extents in the sense D4 already establishes (a sequence of
capacities, one per axis), so a Tucker-decomposed tensor's storage shape is expressible in the same
hlist-of-capacities vocabulary D4 settles for dense storage, without needing any new type-level
machinery. This is an observation about fit, not a recommendation to adopt Tucker; whether arvo wants a
compressed tensor representation at all is a separate and unaddressed question.

**Tensor train (TT)** decomposition (Oseledets and Tyrtyshnikov, 2009 to 2011, see the original
[SIAM J. Sci. Comput. paper](https://users.math.msu.edu/users/iwenmark/Teaching/CMSE890/TENSOR_oseledets2011.pdf))
factors an N-way tensor into a chain of N third-order tensors (the "cores"), with cost linear in the
tensor's dimensionality rather than exponential, which is TT's whole reason for existing (avoiding the
curse of dimensionality that a naive dense representation hits). Each core's size is bounded by the TT
ranks between consecutive modes, which, like Tucker's per-mode ranks, are plausibly const-generic
parameters chosen by the caller. Whether TT recompression (the algorithm that keeps ranks from growing
unboundedly as tensors are combined) can be written without dynamic allocation was not found addressed
anywhere in the literature searched; every reference implementation found (the TT-Toolbox,
[github.com/oseledets/TT-Toolbox](https://github.com/oseledets/TT-Toolbox)) targets MATLAB, which
allocates freely. This is a negative finding worth stating plainly: no evidence of an allocation-free
or const-sized TT implementation was found anywhere in this search.

**Hierarchical Tucker** sits between Tucker and TT, organising the modes into a binary tree rather
than a chain; it was not separately covered in the searches run here beyond the tree-tensor-network
Nyström paper cited above, which generalises streaming low-rank ideas to that tree structure. Treat
hierarchical Tucker as under-surveyed in this pass rather than as ruled out.

### Hierarchical and structured matrices

Hackbusch's H-matrices (already rejected as the recalled name, but real and relevant machinery) divide
a matrix into a block hierarchy and replace off-diagonal blocks with low-rank approximations,
achieving near-linear complexity for matrix-vector products, additions, and inversions on matrices
that would otherwise be dense (see the introduction at
[EUDML, An introduction to hierarchical matrices](https://eudml.org/doc/249049)). **HSS (hierarchically
semiseparable)** matrices are a related, more restrictive format (nested bases across the hierarchy)
that trades some generality for simpler recursion. Both formats sit directly above the sparse and
spectral layers arvo already ships (`arvo-sparse`'s CSR and RCM reordering, `arvo-spectral`'s Laplacian
and Fiedler-vector machinery), and both are, in every reference surveyed, described and implemented in
terms of dynamically sized block trees whose depth and block sizes depend on the input matrix's actual
structure at runtime. Nothing found here suggests a const-sized H-matrix or HSS matrix is a solved
problem anywhere in the field; if arvo wanted this layer, it would be doing genuinely novel work rather
than porting an existing const-generic formulation.

**Butterfly factorization**, inspired directly by the divide-and-conquer structure of the Cooley-Tukey
FFT, factors certain structured matrices (those with the "complementary low-rank" property that
Fourier-like transforms have) into a product of sparse, hierarchically-structured factors, achieving
near-optimal memory and runtime up to polylogarithmic factors (see
[Butterfly Factorization, Li, Yang, Martin, Ho, Ying](https://web.stanford.edu/~lexing/BF.pdf) and the
2024 error-bound refinement at [arXiv:2411.04506](https://arxiv.org/pdf/2411.04506)). Because it
generalises the FFT's own recursive structure, and because arvo already reasons about fixed, compile-
time-known recursive halving (the shape hlist itself is exactly this shape), butterfly factorization is
worth flagging as the structured-matrix family most naturally compatible with a const-generic recursive
type, more so than H-matrices or HSS. It was not found described anywhere in a fixed-point or
allocation-free setting; every reference surveyed uses floating point and dynamic allocation.

### Structured matrix classes with fast algorithms (displacement rank)

Toeplitz (constant along diagonals), Hankel (constant along anti-diagonals), Vandermonde, and Cauchy
matrices are unified by **displacement rank** (Kailath, Kung, and Morf, 1979): applying a displacement
operator (built from shift or diagonal-scaling operators specific to each matrix class) to a matrix of
one of these four classes yields a low-rank result, and that low rank is what fast algorithms exploit.
Displacement rank bounds are small and fixed by the matrix class itself (at most 2 for Toeplitz and
Hankel, at most 1 for Vandermonde and Cauchy), independent of the matrix's size, which is the single
most const-generic-friendly fact found anywhere in this survey: the "rank" here is a property of the
matrix's algebraic class, known at the type level the moment the class is chosen, never a runtime
discovery. See the overview in
[Fast Algorithms for Displacement and Low-Rank Structured Matrices, 2018](https://arxiv.org/pdf/1807.03437)
for the modern statement, and note that Toeplitz systems specifically convert to Cauchy-like systems
via the FFT to exploit this structure for superfast solvers (the FFT step itself is the part least
compatible with fixed point, since FFT roundoff accumulation in fixed-point arithmetic is a known hard
problem the search did not find addressed for this specific application). This class of structured
matrices is arguably the most directly implementable of everything surveyed in this file within arvo's
actual constraints, precisely because the displacement rank is a small compile-time constant rather
than a runtime discovery, but no fixed-point implementation of any of it was found.

### Nonlinear algebra proper: Groebner bases, real algebraic geometry, homotopy continuation

Sturmfels and Michałek's *Invitation to Nonlinear Algebra* (cited in Part One) is the field's own
current self-definition: polynomial systems, varieties, the Nullstellensatz, primary decomposition,
tropical geometry, and semidefinite programming, unified as the natural extension of linear algebra
once the equations are allowed to be nonlinear. Two specific mechanisms deserve honest treatment
against arvo's constraints.

**Groebner bases** are the symbolic-computation backbone of the field: a canonical generating set for a
polynomial ideal, computable by Buchberger's algorithm and refinements (F4, F5). The complexity is the
decisive fact for this survey. Worst-case, Groebner basis computation is **doubly exponential** in
either the number of variables or the maximum input degree, both in the time taken and, critically, in
the size of the output itself: the basis can have doubly-exponentially many elements (see the
foundational lower bounds, Mayr and Meyer 1982 and Hùynh 1986, summarised in
[Groebner Bases and Their Complexity](https://leokayser.github.io/assets/pdf/papers/Groebner_Bases_and_Their_Complexity.pdf)).
This is a direct, structural conflict with `#![no_std]`, no-alloc, const-sized storage: there is no
const generic that can bound an output whose worst-case size is doubly exponential in an input the
type system does not and cannot see at compile time. The generic case is far better behaved (the same
source notes near-polynomial behaviour for systems with finitely many zeros at infinity), but "usually
fine" is not a foundation for a const-sized type, and exact Groebner basis computation also
categorically requires exact (rational or modular) arithmetic rather than fixed-point or floating-point
approximation, since the algorithm's correctness depends on exact zero-tests. This is a clear negative
finding: Groebner bases, as classically formulated, do not fit arvo's constraints, and nothing found in
this search suggests a bounded, fixed-point-compatible restriction of the technique exists yet.

**Numerical algebraic geometry and homotopy continuation** solve polynomial systems numerically rather
than symbolically: construct an easier "start system" sharing the target system's structure, then
track solution paths from the start system to the target as a homotopy parameter varies, using
predictor-corrector methods (see the survey framing at
[Numerical Algebraic Geometry: A New Type of Computational Framework](https://arxiv.org/pdf/1203.4235)
and the Julia implementation [HomotopyContinuation.jl](https://arxiv.org/pdf/1711.10911)). This avoids
Groebner bases' combinatorial blowup, but trades it for a different set of costs that are equally hard
to reconcile with arvo's constraints: the number of paths to track is generically the Bezout number (a
product of the polynomial degrees, which can be very large for even modestly sized systems), each path
needs adaptive step-size control and typically extended or arbitrary precision near singular points to
avoid path-tracking failure, and the whole method's numerical reliability rests on floating-point
predictor-corrector iteration rather than anything expressible as a fixed const computation. Every
production tool surveyed here (Bertini, HomotopyContinuation.jl) targets a dynamically sized, runtime-
adaptive floating-point environment. No evidence was found of a fixed-point or const-sized formulation
of homotopy continuation anywhere in the literature searched. This is the clearest "not affordable in
this setting" finding of the whole survey: not merely unproductised, but structurally opposed to the
constraint set on essentially every axis (allocation, adaptivity, precision).

### Tropical and min-plus algebra

The tropical (min-plus, or dually max-plus) semiring replaces ordinary addition and multiplication with
minimum (or maximum) and addition. Under this replacement, matrix multiplication becomes the classic
min-plus product used for all-pairs shortest paths, and many combinatorial optimisation problems
(shortest path, scheduling, throughput analysis in discrete-event systems) become linear algebra over
this semiring instead of nonlinear optimisation over the reals (see the lecture notes at
[Optimization and Tropical Geometry](https://page.math.tu-berlin.de/~joswig/teaching/VL+PR-Optimization+and+Tropical+Geometry-SS19/lecture1.pdf)
and the connection to arvo's own existing domain at
[Min-plus matrix multiplication, background survey](https://en.wikipedia.org/wiki/Min-plus_matrix_multiplication)).
This connects directly and concretely to `arvo-graph`'s existing `topo_sort`, `longest_path`, and
`upward_rank`/`downward_rank` machinery: those are already tropical linear algebra, whether or not they
are framed that way in the current code, since longest-path-in-a-DAG is exactly max-plus matrix power
iteration and topological ordering is a structural precondition for it to terminate.

The clearest recent, concrete, and directly relevant find of this entire survey is
**PALMA** (N'guessan, 2026, [arXiv:2601.17028](https://arxiv.org/html/2601.17028v1)), a C library
implementing five tropical semirings (max-plus, min-plus, max-min, min-max, Boolean) for ARM-based
embedded systems, deliberately using pure 32-bit signed integer arithmetic rather than floating point
"for determinism and predictability," supporting both dynamic allocation and static caller-provided
buffers for hard real-time use, and using ARM NEON SIMD to process four semiring operations in
parallel. It reports 2,274 million operations per second peak on a Raspberry Pi 4 and up to 11.9 times
speedup over classical Bellman-Ford for single-source shortest paths. This is essentially a working
proof that a tropical-algebra layer over integer (and, by extension, fixed-point) arithmetic, with an
explicit no-allocation mode, is not a hypothetical: someone shipped one in 2026, on hardware
constraints comparable to what arvo already targets. It is a small, single-author, very recent
preprint, exactly the "twelve stars solved the exact problem" case the shared research brief asks to be
found. Nothing here suggests it is production-hardened or peer-reviewed yet; it is offered as evidence
the approach works, not as a library to depend on.

### Approximation theory as an engineering tool

**Chebyshev approximation and the Remez algorithm** compute the minimax (uniform-error-optimal)
polynomial approximation to a function on an interval, via the Chebyshev equioscillation theorem
(the optimal error alternates sign at a specific number of points) and an iterative exchange procedure
(see the overview at [Minimax approximation algorithm, background survey](https://en.wikipedia.org/wiki/Minimax_approximation_algorithm)
and the practical treatment in [Boost's Remez documentation](https://www.classes.cs.uchicago.edu/archive/2013/fall/51025-1/boost_1_50_0/libs/math/doc/sf_and_dist/html/math_toolkit/backgrounders/remez.html)).
This is, along with Eckart-Young-Mirsky, one of the two cleanest instances of "bounded optimal
approximation" as a literal, named, provable property found in this entire survey, and it is squarely
an approximation-theory rather than a nonlinear-algebra result, worth keeping distinct from Part One's
name search for exactly that reason. The practical caveat found: Remez's coefficient-solving step uses
increasingly ill-conditioned Vandermonde-type systems as the polynomial degree grows, which the field's
own documentation flags as sometimes needing extended precision. Whether that ill-conditioning is
survivable in fixed-point at the polynomial degrees a curve or transcendental-function approximation
would realistically need was not established either way in this search; it is a concrete, benchable
question rather than an answered one.

**Barycentric rational approximation and the AAA algorithm** (Nakatsukasa, Sete, Trefethen, "The AAA
Algorithm for Rational Approximation," SIAM J. Sci. Comput., 2018,
[arXiv:1612.00337](https://arxiv.org/abs/1612.00337)) is the most concrete, actively-developed, and
recent of the approximation-theory candidates. AAA builds a rational approximant in barycentric form,
greedily selecting support points to avoid the exponential instabilities that plague naive rational
interpolation, in roughly forty lines of reference code with no user-tunable parameters. A "Lawson
phase" extension iteratively reweights the least-squares fit toward the true minimax solution (see
[The First Five Years of the AAA Algorithm](https://people.maths.ox.ac.uk/trefethen/nak_sete_tref_revised.pdf)
for the most recent survey of its adoption, extensions, and known failure modes). Barycentric rational
forms are numerically well-behaved by construction (no polynomial-degree blowup, no Vandermonde
ill-conditioning of the kind Remez suffers), and unlike a Bezier's control polygon, rational barycentric
weights do not sit on the curve either, so this is not automatically a fit for D11's curve-authoring
concern about control points lying off the curve; it is closer kin to D11's implicit-distance-field
candidate than to the on-curve candidates. Directly relevant to arvo's existing needs: this is the
tool of choice in the field today for turning an arbitrary sampled function into a compact rational
form, which is exactly the shape of problem a fixed-point transcendental-function approximation (sqrt,
trig, reciprocal, the kind of thing `arvo-numeric-contracts`' `Sqrt` and `Recip` traits already name)
reduces to. No fixed-point or const-sized implementation of AAA was found anywhere in this search; it
is unproven for this setting in the same way most of Part Two is.

**Kolmogorov n-width** is not an algorithm but a quantity: the smallest possible worst-case
approximation error achievable by any n-dimensional linear subspace, for a given class of functions
(or curves, or matrices). It is the theoretical ceiling every specific method above (Chebyshev, AAA,
truncated SVD via Eckart-Young-Mirsky) is trying to approach or match. See the formal treatment at
[The Kolmogorov N-width for linear transport](https://www.uni-ulm.de/fileadmin/website_uni_ulm/mawi.inst.070/urban/papers/AGU2023.pdf).
Its practical value to a design round is as a diagnostic rather than as something to implement: it
tells you the best any n-parameter representation could possibly do for a given problem class, which is
the correct standard against which to bench D11's own curve candidates, independent of which one wins.

### Sketching (a category the brief did not name but the search kept surfacing)

**Randomised sketching** (Woodruff, "Sketching as a Tool for Numerical Linear Algebra," Foundations and
Trends in Theoretical Computer Science 10(1-2), 2014,
[arXiv:1411.4357](https://arxiv.org/abs/1411.4357)) compresses a large matrix by multiplying it with a
smaller random matrix (Gaussian, Hadamard, or the sparser Clarkson-Woodruff family), then solves the
much smaller problem instead of the original, for least squares, low-rank approximation, robust
regression, and graph sparsification. This is the mechanism underlying Halko-Martinsson-Tropp's
randomised SVD above, generalised beyond low-rank approximation to a family of numerical linear algebra
problems. It is included here as a named category because it recurs as the shared machinery behind
several of the "recent and unproven" candidates above (randomised SVD, tree tensor network Nyström),
and because a sketching matrix, once its shape is fixed, is itself const-sized in exactly the sense
arvo already wants: a Gaussian or Hadamard sketch of fixed dimensions is a compile-time-known-shape
linear map, and generating one deterministically (a fixed, seeded pseudo-random construction rather
than a runtime-random one) is a plausible const-evaluable primitive, though no implementation of that
specific idea was found in this search.

## What could not be found

Stated plainly, per the shared research brief's instruction that a negative result is real information.

No fixed-point implementation of any low-rank, tensor, structured-matrix, or approximation-theory
algorithm surveyed here was found anywhere in the literature searched. Every reference implementation
located (nalgebra's SVD, faer, the TT-Toolbox, RSVDPACK, Remez.jl, baryrat, HomotopyContinuation.jl)
targets floating point, and every one that was checked for allocation behaviour allocates dynamically
for anything beyond the smallest fixed-size cases. `nalgebra`'s `SMatrix<T, R, C>` const-generic
statically-allocated matrix type is the closest match in spirit to arvo's own storage model found
anywhere (see
[the const-generics integration writeup](https://www.dimforge.com/blog/2021/04/12/integrating-const-generics-to-nalgebra/)),
but its SVD implementation, as far as this search could establish, still routes through the general
dynamically allocated decomposition machinery rather than a specialised stack-only path, which suggests
that even the Rust ecosystem's most const-generic-forward linear algebra library has not solved
allocation-free decomposition for the general case.

No evidence was found of a compile-time (const generic, type-level) formulation of matrix or tensor
rank anywhere, for any of the decomposition families surveyed. Every method treats the target rank (or,
for CP decomposition, the rank itself) as either a caller-supplied runtime parameter or, for CP rank
specifically, an NP-hard quantity to discover. This is consistent across the entire survey and is
probably the single most load-bearing negative finding for the design round: nothing in this literature
offers a way to know the "right" rank at compile time from the shape of the problem alone. A const-sized
foundation adopting any of this would be choosing to fix the rank as a caller-supplied constant (a
strategy-marker-like axis) rather than deriving it, which is a design choice this file does not make,
consistent with the shared brief's instruction not to.

No evidence was found either way on whether the ill-conditioning the Remez algorithm's own
documentation flags for high-degree polynomial fits is survivable in fixed-point arithmetic at the
degrees a realistic curve or transcendental-function approximation would need. This reads as a genuinely
open, benchable question rather than a settled negative.

No hierarchical Tucker literature was surveyed beyond one adjacent streaming-algorithm paper; treat
that format as under-covered by this pass rather than ruled out or endorsed.

No literature was found connecting any of the structured-matrix or tensor-decomposition machinery
surveyed here directly to rotor or motor representations from D10's geometric-algebra decision, despite
both areas nominally sharing "algebra beyond plain linear algebra" as a description. If there is a
connection (a rotor's bivector components admitting a structured or low-rank treatment, for instance),
it was not found in this search and would need its own pass.
