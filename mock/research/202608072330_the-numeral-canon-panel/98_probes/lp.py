#!/usr/bin/env python3
"""Exact rational feasibility for small linear systems, by Fourier-Motzkin.

Shared by the probes in this directory. Nothing here is a design decision; it
is an instrument, and it is exact on purpose. A count like "72 of 15625" is a
measurement, and a measurement taken in floating point near a degenerate face
is a measurement of the rounding.

A system is a list of rows `(coeffs, rhs)` meaning `sum(coeffs[k] * x[k]) >= rhs`,
with every entry a Fraction. `feasible(rows, nvars)` returns True when some
real assignment satisfies all of them.

Fourier-Motzkin eliminates one variable at a time: rows with a positive
coefficient on that variable are lower bounds, rows with a negative one are
upper bounds, and every (lower, upper) pair contributes one derived row without
it. The pair count is quadratic in the worst case, so rows are normalised by
their gcd and deduplicated after each pass, which is what keeps this usable.
"""

from fractions import Fraction
from math import gcd


def _normalise(coeffs, rhs):
    """Scale a row by a positive rational so equal half-spaces compare equal."""
    vals = [c for c in coeffs if c != 0]
    if not vals:
        return tuple(coeffs), rhs
    dens = 1
    for v in list(coeffs) + [rhs]:
        dens = dens * v.denominator // gcd(dens, v.denominator)
    ints = [int(c * dens) for c in coeffs]
    r = int(rhs * dens)
    g = 0
    for v in ints + [r]:
        g = gcd(g, abs(v))
    if g:
        ints = [v // g for v in ints]
        r //= g
    return tuple(Fraction(v) for v in ints), Fraction(r)


def feasible(rows, nvars):
    """True when `sum(a[k]*x[k]) >= b` holds simultaneously for some x."""
    cur = []
    seen = set()
    for coeffs, rhs in rows:
        c, r = _normalise(list(coeffs), rhs)
        key = (c, r)
        if key not in seen:
            seen.add(key)
            cur.append([list(c), r])

    for j in range(nvars):
        lower, upper, keep = [], [], []
        for coeffs, rhs in cur:
            if coeffs[j] > 0:
                lower.append((coeffs, rhs))
            elif coeffs[j] < 0:
                upper.append((coeffs, rhs))
            else:
                keep.append((coeffs, rhs))
        derived = []
        seen = set()
        for coeffs, rhs in keep:
            c, r = _normalise(list(coeffs), rhs)
            if (c, r) not in seen:
                seen.add((c, r))
                derived.append([list(c), r])
        for lc, lr in lower:
            a = lc[j]
            for uc, ur in upper:
                b = -uc[j]
                # a*x_j >= lr - rest_l  and  b*x_j <= rest_u - ur ; combine.
                nc = [b * lc[k] + a * uc[k] for k in range(nvars)]
                nc[j] = Fraction(0)
                nr = b * lr + a * ur
                c, r = _normalise(nc, nr)
                if all(v == 0 for v in c):
                    if r > 0:
                        return False
                    continue
                if (c, r) not in seen:
                    seen.add((c, r))
                    derived.append([list(c), r])
        cur = derived

    for coeffs, rhs in cur:
        if all(v == 0 for v in coeffs) and rhs > 0:
            return False
    return True


def f(x):
    """Exact Fraction from whatever the cost table holds."""
    return Fraction(x) if not isinstance(x, Fraction) else x
