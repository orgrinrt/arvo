#!/usr/bin/env python3
"""Exact feasibility for three-coordinate weight cones, by polygon clipping.

Shared instrument. Nothing here is a design decision.

Every question the ladder asks has the shape: does a weight vector w in Q^3
exist with w >= 0 (or w > 0) satisfying a family of homogeneous inequalities
<w, g> >= 0, and optionally a sub-family strictly?

The system is homogeneous, so w may be normalised to the simplex
w1 + w2 + w3 = 1 without loss. Writing (x, y) = (w1, w2) and w3 = 1 - x - y,
the feasible set becomes a convex polygon inside the triangle
T = {x >= 0, y >= 0, x + y <= 1}, and each inequality is one half-plane. Exact
Fractions and Sutherland-Hodgman clipping keep this both fast and correct near
degenerate faces, which is where a floating-point solver would lie.

Three predicates, and the last two rest on one small fact about convex hulls.
For a polygon P given by its vertices, and any linear form L that is >= 0 at
every vertex, the centroid of the vertices satisfies L > 0 exactly when some
vertex does, because the centroid's value is the mean of non-negative terms.
So a point strictly satisfying a whole family exists exactly when each member
of the family is strict at some vertex, and the centroid is such a point.
"""

from fractions import Fraction

TRI = [(Fraction(0), Fraction(0)), (Fraction(1), Fraction(0)), (Fraction(0), Fraction(1))]


def to_halfplane(g):
    """<w, g> >= 0 on the simplex becomes a*x + b*y + c >= 0."""
    g1, g2, g3 = g
    return (g1 - g3, g2 - g3, g3)


def clip(poly, hp):
    """Sutherland-Hodgman against a*x + b*y + c >= 0. Exact."""
    a, b, c = hp
    if not poly:
        return poly
    out = []
    n = len(poly)
    for i in range(n):
        px, py = poly[i]
        qx, qy = poly[(i + 1) % n]
        vp = a * px + b * py + c
        vq = a * qx + b * qy + c
        if vp >= 0:
            out.append((px, py))
        if (vp > 0 and vq < 0) or (vp < 0 and vq > 0):
            t = vp / (vp - vq)
            out.append((px + t * (qx - px), py + t * (qy - py)))
    # Drop consecutive duplicates so degenerate results stay small.
    dedup = []
    for p in out:
        if not dedup or dedup[-1] != p:
            dedup.append(p)
    if len(dedup) > 1 and dedup[0] == dedup[-1]:
        dedup.pop()
    return dedup


def region(gs):
    """The polygon of w >= 0 on the simplex satisfying every <w, g> >= 0."""
    poly = list(TRI)
    for g in gs:
        poly = clip(poly, to_halfplane(g))
        if not poly:
            return poly
    return poly


def nonempty(poly):
    return bool(poly)


def has_strictly_positive_weights(poly):
    """Some point of poly has w1 > 0, w2 > 0 and w3 > 0."""
    if not poly:
        return False
    return (any(x > 0 for x, _ in poly)
            and any(y > 0 for _, y in poly)
            and any(x + y < 1 for x, y in poly))


def has_point_strict_on(poly, gs_strict):
    """Some point of poly has w > 0 and <w, g> > 0 for every g in gs_strict."""
    if not has_strictly_positive_weights(poly):
        return False
    for g in gs_strict:
        a, b, c = to_halfplane(g)
        if not any(a * x + b * y + c > 0 for x, y in poly):
            return False
    return True


def centroid(poly):
    n = len(poly)
    return (sum(x for x, _ in poly) / n, sum(y for _, y in poly) / n)


def weights_at(poly):
    """A representative weight vector from the polygon, for reporting."""
    x, y = centroid(poly)
    return (x, y, Fraction(1) - x - y)
