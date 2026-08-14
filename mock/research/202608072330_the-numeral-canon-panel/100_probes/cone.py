"""Exact rational weight-cone machinery, written for `100`'s probes.

A section `sigma : R -> A` over a cost map `c : R x A -> Q^d` is rationalisable
by a weight vector `w` when

    for all r in R, for all a in A:   <w, c(r, sigma(r))> <= <w, c(r, a)>

which is a homogeneous linear system `M w >= 0` with `M` the matrix of
difference vectors `c(r, a) - c(r, sigma(r))`. Together with `w >= 0` that is a
pointed polyhedral cone, so it is non-trivial exactly when it has an extreme
ray, and every extreme ray of a pointed cone in `Q^d` lies on `d - 1`
linearly independent tight constraints.

Everything here is `fractions.Fraction`. No floating point, no tolerance, no
sampling. This is the same decision procedure `97_probes/p9_the_decider.py`
describes and `98_probes/cone.py` implements differently; it is written again
here rather than imported so that a disagreement between them is visible.

Three questions this answers, and `100`'s p2 turns on the difference:

  `nonempty(...)`     is the section explained by SOME admissible weighting
  `admits(w, ...)`    is the section explained by THIS weighting
  `forced(w, ...)`    is the section the UNIQUE minimiser under this weighting
"""

from fractions import Fraction
from itertools import combinations


def _diffs(section, costs, regions, arms):
    """M: one row per (region, alternative arm), the difference vector."""
    rows = []
    for r in regions:
        chosen = costs[r][section[r]]
        for a in arms:
            if a == section[r]:
                continue
            rows.append(tuple(costs[r][a][k] - chosen[k] for k in range(len(chosen))))
    return rows


def _nullvec(rows, d):
    """A nonzero vector orthogonal to every row, when the rows span d-1 dims.

    Exact Gaussian elimination over Fraction, returning None when the rows do
    not have rank d-1.
    """
    m = [list(r) for r in rows]
    piv = []
    row = 0
    for col in range(d):
        sel = None
        for i in range(row, len(m)):
            if m[i][col] != 0:
                sel = i
                break
        if sel is None:
            continue
        m[row], m[sel] = m[sel], m[row]
        pv = m[row][col]
        m[row] = [x / pv for x in m[row]]
        for i in range(len(m)):
            if i != row and m[i][col] != 0:
                f = m[i][col]
                m[i] = [a - f * b for a, b in zip(m[i], m[row])]
        piv.append(col)
        row += 1
    if row != d - 1:
        return None
    free = [c for c in range(d) if c not in piv][0]
    v = [Fraction(0)] * d
    v[free] = Fraction(1)
    for i, col in enumerate(piv):
        v[col] = -m[i][free]
    return tuple(v)


def _extreme_rays(rows, d):
    """Extreme rays of {w : M w >= 0, w >= 0}, exactly."""
    ineqs = list(rows)
    for k in range(d):
        e = [Fraction(0)] * d
        e[k] = Fraction(1)
        ineqs.append(tuple(e))
    out = []
    seen = set()
    for sub in combinations(range(len(ineqs)), d - 1):
        v = _nullvec([ineqs[i] for i in sub], d)
        if v is None:
            continue
        for cand in (v, tuple(-x for x in v)):
            if all(sum(a * b for a, b in zip(row, cand)) >= 0 for row in ineqs):
                if any(x != 0 for x in cand):
                    g = max(abs(x) for x in cand)
                    key = tuple(x / g for x in cand)
                    if key not in seen:
                        seen.add(key)
                        out.append(key)
    return out


def nonempty(section, costs, regions, arms, d, strict=False):
    """Is the section an argmin under some admissible weighting.

    `strict=False` asks for `w >= 0` and not all zero. `strict=True` asks for
    `w > 0` on every coordinate, which is the rung `98` section 2.3 shows is the
    one carrying the no-dominated-arm guarantee.
    """
    rays = _extreme_rays(_diffs(section, costs, regions, arms), d)
    if not rays:
        return False
    if not strict:
        return True
    # The sum of every extreme ray is a relative-interior point of the cone, so
    # it has the widest support any point of the cone has.
    s = [sum(r[k] for r in rays) for k in range(d)]
    return all(x > 0 for x in s)


def admits(w, section, costs, regions, arms):
    """Is THIS weighting one that explains the section, everywhere at once."""
    for r in regions:
        chosen = sum(wi * ci for wi, ci in zip(w, costs[r][section[r]]))
        for a in arms:
            if sum(wi * ci for wi, ci in zip(w, costs[r][a])) < chosen:
                return False
    return True


def forced(w, section, costs, regions, arms):
    """Is the section the unique minimiser under this weighting."""
    for r in regions:
        chosen = sum(wi * ci for wi, ci in zip(w, costs[r][section[r]]))
        for a in arms:
            if a == section[r]:
                continue
            if sum(wi * ci for wi, ci in zip(w, costs[r][a])) <= chosen:
                return False
    return True


def argmin_section(w, costs, regions, arms, tiebreak="first"):
    """The section a generator emits, under a declared tie-break policy."""
    out = {}
    for r in regions:
        vals = [(sum(wi * ci for wi, ci in zip(w, costs[r][a])), a) for a in arms]
        best = min(v for v, _ in vals)
        tied = [a for v, a in vals if v == best]
        out[r] = tied[0] if tiebreak == "first" else tied[-1]
    return out
