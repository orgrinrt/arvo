# Curve representations: clothoids, biarcs, implicit fields, interpolating splines

**Date:** 2026-07-28
**Kind:** research, not design. See `00_context.md` for governing canon and provenance rules.
**Governs:** D11 in `arvo/mock/design_rounds/202607281220_topic.the-ndim-and-shape-design.md`, which places
curves in arvo as mathematics and refuses to pick a representation, naming Euler spirals and clothoids,
biarcs and piecewise-circular curves, implicit distance fields, and classical Beziers as the candidates to
bench, with the winner selected statically per workload.

D11 also names three workloads that pull in different directions. Rounded corners want an arc that is
exactly an arc, where a cubic Bezier only approximates a circle and does so with a known, nonzero
radial error. A heavy simulation (ikiuni) wants fixed-point rotation and, by extension, fixed-point curve
evaluation as a first-class thing rather than a float intermediate. A terrain editor wants a spline whose
control points sit on the curve, which none of Bezier's control points do by construction. Those three
pulls are the lens the rest of this document keeps in view, because a representation that wins one
workload is frequently the wrong tool for another, and the bench matrix D11 calls for exists precisely to
let the type system pick per instantiation rather than per project.

The classical Bezier and B-spline material is table stakes and gets one paragraph. Everything else gets
the depth the brief asked for, weighted toward the last five years and toward work that is productised or
at least prototyped rather than purely theoretical, because that is where this project's actual decision
sits.

## Classical Bezier and B-spline curves, briefly

A cubic Bezier is four points and a cubic polynomial blend. It is closed under affine transformation, its
control polygon bounds the curve (convex hull property), and de Casteljau evaluation is numerically stable
and cheap: a handful of lerps, no trigonometric or transcendental functions, no square roots. None of that
is in question. What it cannot do, and what motivates every other candidate in this document: it cannot
represent a circular arc exactly (a cubic Bezier is a degree-3 polynomial curve; a circle is not
algebraic in that sense in Cartesian form, though it is algebraic as an implicit conic), it has no closed
form for arc length (the arc-length integral of a cubic is the integral of the square root of a
degree-4 polynomial, and by the Abel-Ruffini theorem the resulting degree-5 equation has no algebraic
solution in general), and its control points, except the two endpoints, do not lie on the curve, which is
precisely the terrain-editor complaint D11 records. B-splines generalise Bezier to piecewise polynomials
with shared control points and inherit the same three properties (affine-closed, no exact arcs, no
closed-form arc length, control points off-curve) while adding knot-vector machinery. Rational forms of
both (NURBS, rational Bezier) fix the exact-arc problem at the cost of per-control-point weights, covered
below under conics.

## Euler spirals and clothoids: the developed line

An Euler spiral, also called a clothoid or Cornu spiral, is the curve whose curvature is linear in arc
length: kappa(s) = kappa_0 + kappa_1 * s. This single property is the whole representation, and it is
what makes almost everything else about the curve tractable in ways a Bezier is not.

The modern treatment of this curve as a first-class graphics primitive, rather than a road-design
curiosity, is almost entirely the work of Raph Levien, starting with his 2009 UC Berkeley PhD thesis
"From spiral to spline: optimal techniques for interactive curve design" (advised by Carlo Sequin), which
produced the Spiro toolkit later shipped in FontForge and Inkscape (Raph Levien, "From spiral to spline",
PhD thesis, UC Berkeley, 2009, https://digitalassets.lib.berkeley.edu/techreports/ucb/text/EECS-2008-111.pdf,
and the earlier "The Euler spiral: a mathematical history" tech report, 2008,
https://levien.com/phd/euler_hist.pdf). The thesis develops a numerical solver that finds the global
solution to a G2-continuous Euler-spiral spline through a set of points in tens of microseconds, and shows
how to convert the result into cubic Beziers that approximate the spiral to any desired precision (Chapter
9 of the thesis is specifically the Euler-spiral-to-cubic-Bezier fit).

That fitting problem, converting an Euler spiral segment into the closest cubic Bezier, is developed
further in Levien's blog series rather than in a second thesis, which is itself a data point about how
current this line of work is: "Fitting cubic Bezier curves" (March 2021,
https://raphlinus.github.io/curves/2021/03/11/bezier-fitting.html) and "Simplifying Bezier paths" (April
2023, https://raphlinus.github.io/curves/2023/04/18/bezpath-simplify.html) both build on the observation
that the optimal fitting Bezier's arc length tracks the source curve's arc length closely, which collapses
the search space for the fit from a general nonlinear optimisation to a narrow one-parameter search.

The result that matters most for a design that wants curves as mathematics rather than as a UI-manipulation
convenience is the 2021 finding that Euler spirals give a closed-form parallel curve (offset curve) where
Bezier curves do not. Levien's "Cleaner parallel curves with Euler spirals" (February 2021,
https://raphlinus.github.io/curves/2021/02/19/parallel-curves.html) shows that an Euler spiral's Cesaro
equation (curvature as a function of arc length) transforms under an offset by distance l into
kappa(s) = c / sqrt(s - s0) + 1/l, an analytically tractable form whose cusp (the point where the offset
curve self-intersects because the local radius of curvature equals the offset distance) is located in
closed form. A cubic Bezier's parallel curve, by contrast, is a degree-10 algebraic curve (Levien's later
"Parallel curves of cubic Beziers", September 2022,
https://raphlinus.github.io/curves/2022/09/09/parallel-beziers.html, states this explicitly), not remotely
tractable to compute directly, and the standard practice is to approximate it by subdividing and offsetting
line segments, which introduces spurious cusps that are not in the source curve at all: the Bezier
approximation of a smooth curve's offset can be less smooth than the true offset, purely as an artifact of
the representation. The clothoid's offset has no such artifact, and computing it costs a working reference
implementation "less than 100 lines of code" by Levien's own account, error bounds and cusp handling
included.

This work reached production form in "GPU-friendly Stroke Expansion" (Raph Levien and Arman Uguray,
Proceedings of the ACM on Computer Graphics and Interactive Techniques (HPG 2024), 2024,
https://arxiv.org/abs/2405.00127, full text at https://arxiv.org/html/2405.00127v1), which is the current
state of the art for turning arbitrary path strokes into GPU-rasterisable primitives and is shipping in
Google's Vello renderer. The pipeline fits Euler spiral segments to the source cubic Beziers under an error
tolerance, then lowers each spiral segment's two parallel offsets to line or circular-arc segments, which
are what the GPU rasteriser actually consumes. Two numerical details are worth recording because they are
exactly the kind of thing a bench matrix needs: first, the paper reports that at a fixed tolerance of 0.25
pixels, arc-segment output requires roughly a quarter the primitive count of line-segment output on one of
their test scenes (111,965 arcs versus 406,059 lines), which is the concrete cost argument for treating
circular arcs as a first-class output rather than a degenerate case of everything-is-a-polyline; second,
the whole pipeline targets 32-bit float throughout, with an explicit two-pass Newton refinement on CPU when
higher precision than the GPU path gives is wanted, and the paper contains no discussion of fixed-point
arithmetic at any stage. That absence is itself a finding, covered below in the fixed-point section.

Evaluating an Euler spiral segment directly requires the Fresnel integrals, C(t) = integral of cos(pi t^2 /
2) and S(t) = integral of sin(pi t^2 / 2), which have no elementary closed form. Every practical
implementation approximates them: the stroke-expansion paper uses a four-region piecewise polynomial fit
(sinc-based for small arguments, polynomial for larger ones) with a stated worst-case discrepancy of about
6 percent against the reference, tuned for the subdivision-density use case rather than for evaluating a
final point on the curve; higher-precision applications reach for degree-11 polynomial approximations of
the Fresnel integrals directly (referenced generically across the clothoid-fitting literature below) or for
offline lookup tables sampled at initialisation and interpolated at query time, which is the approach
described in real-time vehicle-path clothoid work: Brezak and Petrovic, "Real-time Approximation of
Clothoids with Bounded Error for Path Planning Applications", 2014,
https://lamor.fer.hr/images/50020777/Brezak2014.pdf, which builds a basic-clothoid lookup table offline and
queries it by arc length at runtime, explicitly because online Fresnel evaluation was too costly for their
control loop.

Fitting clothoids to endpoint tangent and curvature data (the G1/G2 Hermite interpolation problem, which is
the actual authoring-time question for a spline made of clothoid segments) is a nonlinear root-finding
problem with no closed form, and the reference software for it is Bertolazzi and Frego's work: "Fast and
accurate G1 fitting of clothoid curves", arXiv:1305.6644, 2013, https://arxiv.org/pdf/1305.6644, shipped as
the open-source G1fitting library (https://github.com/ebertolazzi/G1fitting) and its successor, the more
general Clothoids C++ library with a Matlab interface
(https://github.com/ebertolazzi/Clothoids, documentation at
https://ebertolazzi.github.io/Clothoids/README.html), which additionally handles clothoid splines, biarc
splines, and clothoid-clothoid and clothoid-arc intersection. This is the most mature open production
codebase for clothoid manipulation found in this pass, and it is float throughout; no fixed-point variant
exists.

A 2026 generalisation worth flagging precisely because it is unresolved: log-aesthetic curves extend the
Euler spiral by letting curvature be a general power of arc length rather than strictly linear, and the
family (parameterised by a single shape exponent) contains the clothoid, the logarithmic spiral, and the
involute of a circle as special cases, unifying them under one curvature-arclength law (Yoshida and Saito's
foundational papers on interactive log-aesthetic curve drawing, and the more recent generalisation via
similarity geometry: Yoshida, Saito et al., "Generalization of log-aesthetic curves via similarity
geometry", Japan Journal of Industrial and Applied Mathematics, 2018,
https://link.springer.com/article/10.1007/s13160-018-0335-7; and a further generalisation to
"superspirals" whose radius of curvature is a completely monotonic Gauss hypergeometric function, cited in
the same literature). The G1/G2 Hermite-interpolation problem for log-aesthetic curves is treated in
"Interactive G1 and G2 Hermite Interpolation Using Extended Log-aesthetic Curves", arXiv:2105.09762, 2021,
https://arxiv.org/pdf/2105.09762. Whether this generalisation is worth the extra shape parameter for a
numeric substrate is exactly the kind of question this document is not supposed to answer, but it is worth
naming because it means "clothoid" is not the ceiling of this line of research even in principle; it is one
point on a one-parameter family, and the family's arc-length and offset properties have not been surveyed
here in the same depth as the pure clothoid case.

## Biarcs and piecewise-circular curves: the classical CNC and robotics answer, under-represented in graphics

A biarc is a pair of circular arcs joined with matched tangent direction at the join point (G1
continuity), constructed from Hermite data (two endpoints, each with a tangent). Given the two endpoint
tangents, the join point lies on a one-parameter family of circles, which gives a free shape parameter to
tune (Bolton, "Biarc curves", Computer-Aided Design, 1975, is the originating reference for this
construction, cited throughout the later literature though the primary source itself was not directly
retrieved in this pass). A chain of biarcs, one per Hermite data pair along a curve, is called an arc
spline, and it is the dominant representation in two domains this project should weight heavily precisely
because graphics research under-serves them: CNC contouring and robot path planning, where every consumer
of the curve (a G-code interpreter, a motion controller) speaks circular arcs natively and nothing else.
"A practicable approach to G1 biarc approximations for making accurate, smooth and non-gouged profile
features in CNC contouring", Computer-Aided Design, ScienceDirect,
https://www.sciencedirect.com/science/article/abs/pii/S0010448506001370, and "Error-bounded biarc
approximation of planar curves", Computer-Aided Design, ScienceDirect,
https://www.sciencedirect.com/science/article/abs/pii/S001044850400003X, are representative of a
continuous line of CAD/CAM literature running from the 1990s through work as recent as 2025 ("Symplectifying
Biarcs", arXiv:2511.00163, https://arxiv.org/pdf/2511.00163, a 2025 paper revisiting the biarc construction
from a symplectic-geometry angle, which this pass did not have budget to read in full but flags as the most
recent active work found on the representation).

What a biarc makes exact, and what it does not, splits cleanly along the same axes as the clothoid, but
lands differently. Arc length is exact and trivial: each arc's length is its radius times its subtended
angle, no integral of any kind. Curvature is piecewise constant rather than linear, so a biarc is only
G1 (tangent-continuous) at the internal join by construction, not G2 (curvature-continuous); this is the
central tradeoff against the clothoid, which is G2 by construction because curvature varies continuously
through zero jump at segment boundaries when segments are chained correctly. Offsetting is exact and
essentially free: the offset of a circular arc by a fixed distance is another circular arc with the same
centre and an adjusted radius, so the offset of a biarc is another biarc, no approximation, no
subdivision, no root-finding, which is the strongest offset story of any representation surveyed here. Arc-arc
intersection is closed-form (elementary circle-circle intersection, at most two solutions from a quadratic),
where Bezier-Bezier intersection generally requires Bezier clipping, subdivision, or resultant computation.
Biarc-to-biarc distance and point-to-arc distance are both closed form (distance to a circle is
|distance to centre minus radius|, clamped to the arc's angular range).

Fitting a smooth source curve (a Bezier path, a scanned outline) with a biarc spline under an error bound is
an active, still-being-refined problem rather than a settled one: the compression literature reports ratios
between 15:1 and 36:1 (arc-spline segment count versus original polyline point count) at a fixed tolerance,
and the open question across the cited papers is almost always how many biarcs are needed to hit a target
tolerance, which is analogous to, and directly comparable against, the clothoid-fitting segment counts
Levien and Uguray report. A companion line of work fits biarcs specifically to serve as an intermediate
representation for computing exact Pythagorean-hodograph-quintic offsets (Juttler and Sir, "Approximating
curves and their offsets using biarcs and Pythagorean hodograph quintics", Computer-Aided Design, 2006,
https://www.ag.jku.at/pubs/2006sfj.pdf; this pass could not extract readable text from the PDF and is
citing it by title, venue, and URL only, without verifying its detailed claims), which suggests biarcs are
already used in the literature as a stepping-stone representation rather than only a terminal one, a role
this project's bench-matrix framing could reuse directly: a biarc fit as an intermediate stage on the way to
a different final representation, chosen per workload.

The rounded-corner workload is where the biarc's advantage over Bezier is starkest and most immediately
useful: a single circular arc, or two matched arcs, is the exact answer, at whatever radius and whatever
angle, with zero approximation error and a closed-form offset for the border-and-fill case that UI corner
rendering actually needs. The clothoid gives no advantage here over a plain arc because the corner has
constant, not linearly varying, curvature by definition; the clothoid's G2 property is wasted on a
workload that wants G1 exactness. The Bezier's approximation error for a quarter-circle corner is well
characterised: the standard four-cubic-Bezier circle approximation uses a control-point offset of kappa =
0.5522847498... times the radius, and even at that optimum the curve drifts outside the true circle by a
radius-dependent amount (commonly cited as around a quarter pixel at a 1000-pixel radius for a quarter-turn
arc; smaller arcs are proportionally tighter). That the industry still reaches for this approximation
routinely, rather than an exact arc primitive, appears to be inertia from SVG and Postscript path grammars
having no native arc-to-Bezier-free path element in common tooling, not evidence that the approximation is
actually preferred where an exact primitive is available.

## Implicit and signed-distance representations: exact where the algebra stays low-degree, then it stops

An implicit curve is the zero set of F(x, y) = 0. A circle is degree 2 and trivially implicit
((x-cx)^2 + (y-cy)^2 - r^2 = 0); distance to it is closed form (the |distance to centre minus radius|
formula already used above for biarcs). This is the exactness the biarc borrows from, restated at the
representation level rather than the parametric level.

For polynomial parametric curves, Loop and Blinn's "Resolution independent curve rendering using
programmable graphics hardware" (ACM Transactions on Graphics, SIGGRAPH 2005,
https://history.siggraph.org/learning/resolution-independent-curve-rendering-using-programmable-graphics-hardware-by-loop-and-blinn/)
showed that a quadratic or cubic Bezier curve can be classified into one of a small number of canonical
types (serpentine, cusp, loop, and so on for cubics) and, per type, mapped by a projective transform into a
fixed canonical implicit form (for a cubic, an implicit equation in the texture-space coordinates u, v, w
of the form u^3 - v*w = 0 or similar per type). That gives an exact, cheap-to-evaluate inside/outside test
per pixel (a sign check on the canonical implicit function, evaluated by simple interpolation of the mapped
per-vertex coordinates), which is why it became the standard GPU technique for filled vector-graphics
rendering. What it does not give is distance. The implicit function's zero set is exact, but its magnitude
away from the zero set is not a Euclidean distance in general (only the sign is trustworthy at coarse
scale), so Loop-Blinn answers "inside or outside" cheaply and exactly but does not answer "how far", which
is the reason later work (the stroke-expansion line above) exists at all: a filled-region test and a
stroke-width test are different problems, and Loop-Blinn only solves the first.

Exact point-to-curve distance for parametric polynomial curves is a root-finding problem whose degree grows
with the curve's degree. For a quadratic Bezier, the condition that the vector from a query point to a
curve point be perpendicular to the tangent at that point is a cubic equation in the curve parameter t,
solvable in closed form (Cardano's formula, or the trigonometric form for three real roots), which is
exactly why quadratic-Bezier signed distance fields are common in real-time font and vector-icon rendering
(searchable widely as "signed distance to quadratic bezier", with public reference implementations such as
the Shadertoy example at https://www.shadertoy.com/view/MlKcDD, and discussion threads such as
https://www.pouet.net/topic.php?which=9119). For a cubic Bezier the same perpendicularity condition is a
degree-5 polynomial in general position, pushed to degree 6 in some formulations depending on how the
tangent is expressed, and by Abel-Ruffini it has no closed-form solution once the degree exceeds four in
general; practical cubic-distance implementations either root-find numerically (Newton or a companion-matrix
eigenvalue solve on the polynomial coefficients), subdivide the cubic into quadratics and bound each with
the closed-form quadratic case, or fall back to sampling and refinement. A 2025 industry report on GPU
Bezier-distance work (secondary source, https://biggo.com/news/202510181915_GPU-Bezier-Curve-Distance-Calculations)
surveys exactly this split: polynomial root-finding on the degree-6 (or degree-5, depending on formulation)
equation, implicit-curve evaluation a la Loop-Blinn for the inside/outside half of the problem, and
subdivision into quadratics as the three live approaches, with the subdivision approach reported as the
pragmatic industry default because the closed-form cubic solve is numerically delicate near curve
self-tangency and near-degenerate control-point configurations.

Circular arcs and clothoids both sidestep this problem structurally, in different ways. A circular arc's
distance is closed form because the implicit form stays degree 2 regardless of how much of the circle is
swept. A clothoid segment has no simple closed-form point-to-curve distance (the perpendicularity
condition against a Fresnel-integral parametrisation is transcendental, not polynomial), so distance queries
against a clothoid are handled the same way cubic-Bezier distance is: Newton iteration from a good initial
guess, or bounding-primitive subdivision, not a closed form. This is worth flagging plainly because it
means the clothoid's analytic advantages (arc length, offset) do not extend to point-distance queries, and
a signed-distance-field workload (rounded-corner rendering by SDF rather than by rasterised fill, which is a
live and common technique) gets nothing extra from choosing a clothoid over a well-subdivided cubic unless
the query is restricted to distance-along-the-curve rather than Euclidean distance to it.

## Interpolating splines whose control points lie on the curve

This is the requirement D11 names for the terrain-editor workload specifically, and it separates the
candidates immediately, because Bezier and B-spline both fail it by construction (only Bezier's two
endpoints, and no B-spline control point in general, sit on the curve).

Catmull-Rom splines are the standard answer and are old (Catmull and Rom, 1974), but the modern, correct
form of the standard answer is younger than the name suggests and is itself a 2003-era refinement still
being cited and re-derived as recently as 2020: the centripetal Catmull-Rom parametrisation (as opposed to
the uniform or chordal parametrisation) chooses the parameter spacing between consecutive control points
proportional to the square root of the chord length, and this specific choice is what prevents the classic
Catmull-Rom failure modes: self-intersecting loops, cusps, and wide overshoot on unevenly spaced control
points, all of which the uniform parametrisation exhibits readily
(https://en.wikipedia.org/wiki/Centripetal_Catmull%E2%80%93Rom_spline, and the practical write-up at
https://splines.readthedocs.io/en/latest/euclidean/catmull-rom-properties.html). Every Catmull-Rom
segment is exactly convertible to a cubic Bezier segment with a fixed linear formula relating the two sets
of control points (arXiv:2011.08232, "Conversion Between Cubic Bezier Curves and Catmull-Rom Splines",
2020, https://arxiv.org/pdf/2011.08232), which means a terrain editor's on-curve authoring control points
and a rendering pipeline's off-curve Bezier control points are the same information under a fixed, cheap,
purely linear change of basis, with no fitting or approximation step at all. That linear relationship
is the strongest argument in this whole document for treating "control points on the curve" as a change of
basis on top of Bezier machinery, rather than as a wholly separate representation demanding its own
evaluation, offset, and intersection code paths: the curve stays cubic-polynomial throughout, only the
authoring-time parametrisation changes.

What Catmull-Rom does not give is curvature control at the interpolated points; curvature at each
control point is whatever falls out of the tangent estimate from its two neighbours, with no direct
handle. Kappa-curves (Yan, Schiller, Wilensky, Carr, and Schaefer, "k-curves: interpolation at local
maximum curvature", ACM Transactions on Graphics 36(4), SIGGRAPH 2017,
https://dl.acm.org/doi/10.1145/3072959.3073692, open implementation at
https://github.com/zhipeiyan/kappa-Curves) answer that directly: a piecewise-quadratic curve that
interpolates the given points and, unlike Catmull-Rom, places the local curvature maxima exactly at those
interpolated points rather than letting curvature wander between them, giving G2 continuity everywhere
except at true inflection points (where only G1 holds, which is mathematically forced: curvature crosses
zero at an inflection and a curvature-continuity claim either side of a sign change is not meaningful in
the same sense). This method is productised, not merely published: it is the curvature tool shipped in
Adobe Illustrator and Photoshop. A 2021 follow-up, epsilon-kappa-curves ("epsilon kappa-Curves: controlled
local curvature extrema", The Visual Computer, 2021,
https://link.springer.com/article/10.1007/s00371-021-02149-8), adds a shape parameter to control the
magnitude of those curvature extrema directly, addressing a specific complaint that the original method's
curvature peaks could be visually too sharp or too shallow with no author-facing knob to adjust them.

Both Catmull-Rom and kappa-curves are polynomial (Catmull-Rom is piecewise cubic, kappa-curves piecewise
quadratic), so both inherit the same arc-length-has-no-closed-form problem cubic and quadratic Bezier have,
and neither has an offset story better than plain Bezier's, because under the hood both convert to Bezier
segments for actual rendering. Their advantage over Bezier is purely at the authoring interface (control
points on the curve), not at the evaluation, offset, or intersection layer, which is a genuinely different
kind of advantage from the clothoid's or the biarc's: those two change what the underlying algebra can do
in closed form; the interpolating splines change what the control interface exposes while staying inside
ordinary polynomial algebra underneath.

## Rational and conic forms: the exact circular arc inside a Bezier-shaped world

A rational quadratic Bezier curve (a quadratic Bezier where each control point additionally carries a
scalar weight, and the curve is the weighted blend divided by the sum of weights) is exactly a conic
section, and the specific case of a circular arc is reached by setting the middle control point's weight to
cos(half the arc's subtended angle) (a standard CAGD result, given in Farin's textbook and restated
concisely at http://demofox.org/bezquadrational.html). This is the mechanism SVG's elliptical-arc path
command and most vector-graphics arc primitives actually use under the hood: an "arc" in a Bezier-based
path grammar is frequently, quietly, a rational quadratic Bezier rather than a true separate primitive
type, which means the exactness the biarc gets structurally is also reachable inside a rational-Bezier
representation, at the cost of a per-control-point weight and a division per evaluation.

The rational quadratic form has a hard limit: it cannot represent an arc whose subtended angle is pi or
more with a positive weight (the weight would need to go to zero or negative, which degenerates the
representation), so a full semicircle or larger sweep needs at least two rational-quadratic segments joined
at a shared point, or a move to rational quartic Bezier, which extends the representable sweep up to but
not including a full 2*pi turn without negative weights and additionally gives a better-conditioned
parametrisation than stacking rational quadratics ("A rational quartic Bezier representation for conics",
Computer Aided Geometric Design, https://www.sciencedirect.com/science/article/abs/pii/S0167839602000961).
A separate line of work generalises the same idea to rational cubic forms specifically tuned to minimise
parametrisation error against a true circle (rational cubic Timmer curves: Abbas et al., "The Representation
of Circular Arc by Using Rational Cubic Timmer Curve", Mathematical Problems in Engineering, 2014,
https://www.hindawi.com/journals/mpe/2014/408492/), which is evidence the "exact arc inside a
polynomial-curve family" problem is still being iterated on for reasons of conditioning and degree, not
because the basic rational-quadratic answer is wrong, but because a fixed-point implementation cares a
great deal about exactly this kind of conditioning question and this literature is a source of concrete
alternative formulations to bench against the classical rational quadratic.

A closely related, very recent (2024) real-world instance of the same exact-versus-approximate corner
tension D11 names for rounded corners: UI design tools have converged on "squircle" or "smooth corner"
geometry (a Lame-curve superellipse, not a circular arc at all) as the perceptually preferred corner shape,
approximated in practice by piecewise cubic Beziers rather than represented exactly, because the
superellipse itself has no simpler exact polynomial or rational form (background at
https://www.figma.com/blog/desperately-seeking-squircles/ and
https://squircle.js.org/blog/math-behind-squircles). This is a useful counter-example to keep in view
alongside the plain-circular-arc rounded-corner case: not every "rounded corner" workload wants a circle,
and the superellipse case is one where none of the four D11 candidates gives an exact answer, Bezier
approximation is the industry's actual practice today, and the open question is whether the exponent-parameterised
log-aesthetic family (above) or a dedicated superellipse rational form closes that gap, which this pass did
not find literature settling either way.

## Fixed-point numerical behaviour: the constraint that decides this, and where the literature goes quiet

This is the axis the brief flagged as decisive and under-published, and that framing held up under
research. No paper or production codebase found in this pass evaluates clothoids, biarcs, or interpolating
splines in fixed-point arithmetic as a primary target. Every source cited above that states a numeric
target states float: the GPU stroke-expansion paper targets 32-bit float explicitly and refines to 32-bit
precision with a Newton pass when more accuracy is wanted; the Clothoids C++ library and G1fitting are
double-precision throughout; the kappa-curves and Catmull-Rom sources are silent on numeric representation,
which in context means the unstated default of IEEE float. This is a genuine negative result rather than a
gap in this search: fixed-point Fresnel-integral evaluation specifically returned nothing in this pass, and
the general search terms around fixed-point curve evaluation surfaced almost entirely patents and embedded
how-to material rather than peer-reviewed or maintained-library treatment.

The one substantial, real-world fixed-point precedent for curve evaluation found in this pass is TrueType
font rasterisation. TrueType outlines are quadratic Bezier contours, and FreeType, the reference open-source
rasteriser, represents every coordinate internally in F26Dot6 fixed point (26 integer bits, 6 fractional
bits, so one unit is 1/64th of a pixel) throughout scan conversion, including the implicit midpoint
construction used when two consecutive off-curve control points appear in a contour
(https://freetype.org/freetype2/docs/glyphs/glyphs-6.html). This is decades of production use of
fixed-point quadratic Bezier evaluation at a fixed, coarse precision (6 fractional bits) chosen because
font rasterisation only ever needs sub-pixel accuracy relative to a target raster grid, not because 6 bits
is a generally correct precision for fixed-point curve work. It says fixed-point quadratic evaluation
(pure polynomial blending, no trigonometric or transcendental step) is a solved, shipped problem at low
precision; it says nothing about cubic evaluation at higher precision, nothing about arc-length or
Fresnel-integral evaluation in fixed point, and nothing about the error behaviour of a fixed-point de
Casteljau chain under repeated composition (each lerp introduces a rounding error, and a design that wants
this exact should ask, and this pass could not answer from the literature, how that error compounds across
a degree-3 or degree-5 evaluation chain at a given fractional-bit budget).

One additional recent paper (2025) surfaced whose title matches this exact question directly but which this
pass was not able to read in full (a 403 response blocked the fetch, and it was not re-attempted for
budget reasons): "Affine Invariance of Bezier Curves on Digital Grid", Mathematics (MDPI), volume 13, issue
22, article 3672, 2025, https://www.mdpi.com/2227-7390/13/22/3672. The title alone is enough to flag it as
directly on point (Bezier evaluation on an integer, i.e. digital, grid, and whether affine invariance
survives the rounding), and it should be read in full before any design round that finalises a fixed-point
curve strategy, but this pass is reporting its existence rather than its content, because the content was
not verified.

Circular arcs and biarcs are the one candidate with an obviously favourable fixed-point story purely by
construction, independent of any literature search: their defining equation (distance from a centre equals
a fixed radius) involves only a squared difference and a comparison, no trigonometric evaluation is needed
to test membership or compute an offset (only to convert an angle to a point, which is the one place sine
and cosine, or a fixed-point CORDIC-style rotor, are unavoidable), and their exactness under composition
(offset of an arc is an arc, exactly, not an approximation that accumulates error) means a fixed-point biarc
pipeline has no error-accumulation story to design around for the offset operation specifically, only for
point evaluation. Clothoids, by contrast, have transcendental Fresnel integrals at their evaluation core, no
matter how they are approximated, and every approximation strategy found in this pass (piecewise
polynomial, lookup table, degree-11 series) was designed and validated against a float baseline; whether
any of them behaves acceptably under fixed-point rounding, and at what fractional-bit width, is an open
question this pass could not resolve from published sources.

## How curve-representation choice is benched in the literature

The literature's own comparative practice supports the bench-matrix approach D11 already commits to,
rather than arguing for a single winner. The Linebender project (Levien's own graphics group, maintainers
of the kurbo Rust curve library and the Vello GPU renderer) maintains a standing comparison page across
exactly the same candidate space this document covers (https://linebender.org/wiki/curve-families/),
scoring cubic Bezier, quadratic Bezier, Euler spiral, circular arcs, rational conics, and elastica against a
fixed set of criteria: fitting cost and continuity of the fit, closure under affine and (for rational
forms) projective transformation, closure under subdivision, and the range of curvature behaviour each
family can represent (their "superellipticity range" metric). That page explicitly names Euler spiral's
weakness as struggling with large curvature variation within one segment and circular arcs' as only G0 by
default (G1 requires the biarc construction specifically), which matches the tradeoffs derived independently
above from the primary sources. No single family wins that comparison outright, which is offered by its own
maintainers as the reason kurbo ships multiple curve types rather than one.

The stroke-expansion paper's segment-count comparison (arcs versus lines at fixed tolerance, cited above)
and the biarc-compression-ratio literature's 15:1-to-36:1 figures are both instances of the same
methodology: fix an error tolerance, fix a source curve or corpus of source curves, count output primitives
or measure wall-clock cost, and let the number decide. Neither paper argues from first principles that its
representation is better in the abstract; both argue from measured output on representative inputs. That is
the same shape D11 already prescribes for arvo (bench per workload, let LLVM erase the loser), and the
concrete inputs to reuse for such a bench matrix are visible in this literature: a rounded-corner test case
(single arc versus quarter-circle Bezier versus biarc), a stroke-expansion test case (offset curve fidelity
and cusp count under Bezier, clothoid, and biarc-chain source representations), and an interpolating-authoring
test case (control-point placement accuracy and curvature smoothness under Catmull-Rom, kappa-curves, and a
biarc-through-points construction).

## What could not be found

Stated plainly, because a negative result here changes what a design round can assume exists.

No fixed-point (as opposed to floating-point) treatment of clothoid or Fresnel-integral evaluation was
found anywhere in this pass, in either the academic or production-library literature. Every clothoid
resource surveyed, from Levien's thesis through the 2024 GPU stroke-expansion paper to the Bertolazzi-Frego
Clothoids library, is float or double throughout, with no discussion of fixed-point suitability, error
propagation under fixed-point rounding, or a recommended fractional-bit budget.

No literature was found directly comparing biarc, clothoid, and rational-conic representations against each
other on a shared benchmark corpus with a shared error metric; each family has its own comparison
literature (biarc against the source curve it approximates, clothoid against cubic Bezier specifically for
stroke expansion, rational conic against a true circle) but no single paper runs all of them through one
harness the way D11 wants for arvo.

The 2025 MDPI paper on affine invariance of Bezier curves on a digital grid could not be read past its
title and venue in this pass; its content may already answer part of the fixed-point question above and
should be treated as unread rather than absent.

No connection was found between the "nonlinear algebra" research lead named in D11 (Sturmfels, Eckart-Young-Mirsky,
Hankel, Higham, Householder) and curve representation specifically. Sturmfels's own recent work
("Invitation to Nonlinear Algebra", with Michalek, AMS, 2021, and the broader numerical-nonlinear-algebra
literature it sits in) concerns polynomial systems, algebraic statistics, low-rank tensor and matrix
approximation, and related topics generally, and this pass found nothing in that literature that names
curve fitting, curve representation, or spline construction as an application area. If op's hunch is about
that literature, the connection remains unidentified after this pass and is a separate research question
from the four named candidates, not a fifth candidate this document can responsibly add.

## The questions that separate the candidates

Restated from the material above, without resolving them, per the brief.

Does the design want G2 continuity (curvature-continuous joins) as a default property of the curve family,
or is G1 (tangent-continuous) acceptable with G2 reserved for special cases. This is the clothoid-versus-
biarc question directly: the clothoid gives G2 by construction and pays for it with transcendental
evaluation and no closed-form point distance; the biarc gives G1 by construction, exact arc length, exact
offset, and closed-form distance, at the cost of a curvature discontinuity at every internal join.

Does the workload ever need Euclidean point-to-curve distance (a signed-distance-field query), or only
along-curve queries (arc length, offset, tangent, curvature at a parameter). Every representation surveyed
here handles along-curve queries well; only the circular arc (and by extension the biarc) handles Euclidean
distance in closed form for an arbitrary point, and cubic Bezier and clothoid both fall back to numerical
root-finding or subdivision for that specific query.

Is the authoring interface (control points on the curve) a property the underlying representation must have
natively, or can it be a change of basis on top of an otherwise-ordinary polynomial representation. The
Catmull-Rom-to-Bezier linear conversion shows this can be a pure basis change with no approximation cost at
all, which suggests the terrain-editor requirement may not need a structurally different curve family from
whatever wins the other two workloads, only a different control-point convention layered on top of it.

What fractional-bit budget and what composition depth (how many curve operations chain before a result is
consumed) does the fixed-point substrate actually need to support, and does any of the four candidates'
error behaviour under repeated fixed-point rounding actually get worse than the others at that budget. This
question has no answer in the literature at all right now; it is the one place a bench in this project would
be producing genuinely new knowledge rather than confirming or reproducing a published result, which is
also the reason it is the question the constraint in `00_context.md` (fixed point as the primary
representation, floats as a tagged wrapper) makes unavoidable here in a way it is not for any of the source
material surveyed above.
