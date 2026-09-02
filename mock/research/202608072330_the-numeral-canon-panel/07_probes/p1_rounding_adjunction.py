#!/usr/bin/env python3
"""p1. Is a rounding mode an adjoint to the embedding?

HYPOTHESIS.  Fix one numeral N with representable value set V, finite, a subset of
the rationals.  Let g be the embedding V -> Q (plain inclusion).  For a rounding
mode m, let a_m : Q -> V be the map that rounds an exact value into V.

The pair (a_m, g) is a monotone Galois connection with a_m as LOWER adjoint iff

    for all x in Q, v in V:   a_m(x) <= v   <=>   x <= g(v)                  (LOW)

and (g, a_m) is a Galois connection with a_m as UPPER adjoint iff

    for all x in Q, v in V:   g(v) <= x     <=>   v <= a_m(x)                (UPP)

Predicted, from the definitions of least-upper and greatest-lower element:
round-toward-plus-infinity satisfies LOW exactly; round-toward-minus-infinity
satisfies UPP exactly; round-toward-zero satisfies neither, because it IS ceil
below zero and floor above it, so it changes which adjoint it is at the origin;
round-away-from-zero, round-to-nearest-even and round-to-nearest-away satisfy
neither.

This is exhaustive over the (x, v) product for every numeral in the box, at an
exact rational representation.  No floating point anywhere.

SECOND QUESTION, same instrument.  The laws above are stated over the whole of Q.
Restricting the concrete side to the numeral's own range [min V, max V] is the
difference between "the adjunction holds" and "the adjunction holds where the
value fits".  Both are reported, because the gap between them is exactly the
overflow band and the claim that overflow sits outside the adjunction is worth
measuring rather than asserting.

OUTPUT is a table of failure counts.  Zero means the law holds on every pair.
"""

from fractions import Fraction as Q

# ---------------------------------------------------------------- shape space


def unsigned(I, F):
    """U<I,F>: {k * 2^-F : 0 <= k < 2^(I+F)}."""
    n = 2 ** (I + F)
    q = Q(1, 2**F)
    return [k * q for k in range(n)]


def signed(I, F):
    """S<I,F>: a symmetric-ish signed grid, {k * 2^-F : -2^(I+F-1) <= k < 2^(I+F-1)}."""
    half = 2 ** (I + F - 1)
    q = Q(1, 2**F)
    return [k * q for k in range(-half, half)]


# ------------------------------------------------------------- rounding modes
# Each takes an exact rational x and a SORTED list V, and returns an element of V.
# Out-of-range inputs clamp, which is what a saturating design does and is the
# only total choice; the in-range/full-range split below is what isolates that.


def _clamp(x, V):
    if x <= V[0]:
        return V[0]
    if x >= V[-1]:
        return V[-1]
    return None


def r_up(x, V):  # toward +infinity
    c = _clamp(x, V)
    if c is not None:
        return c
    return min(v for v in V if v >= x)


def r_down(x, V):  # toward -infinity
    c = _clamp(x, V)
    if c is not None:
        return c
    return max(v for v in V if v <= x)


def r_zero(x, V):  # toward zero
    return r_down(x, V) if x >= 0 else r_up(x, V)


def r_away(x, V):  # away from zero
    return r_up(x, V) if x >= 0 else r_down(x, V)


def _nearest(x, V, tie):
    c = _clamp(x, V)
    if c is not None:
        return c
    lo, hi = r_down(x, V), r_up(x, V)
    if x - lo < hi - x:
        return lo
    if x - lo > hi - x:
        return hi
    return tie(lo, hi)


def r_near_even(x, V):
    def tie(lo, hi):
        # "even" here means the even multiple of the step, which is the
        # fixed-point reading of ties-to-even.
        step = V[1] - V[0]
        return lo if (lo / step) % 2 == 0 else hi

    return _nearest(x, V, tie)


def r_near_away(x, V):
    return _nearest(x, V, lambda lo, hi: hi if abs(hi) >= abs(lo) else lo)


MODES = [
    ("toward +inf", r_up),
    ("toward -inf", r_down),
    ("toward zero", r_zero),
    ("away from zero", r_away),
    ("nearest, even", r_near_even),
    ("nearest, away", r_near_away),
]


# ------------------------------------------------------------------ the tests


def concrete_points(V, refine, out_of_range):
    """Exact rationals to test.  Refines the numeral's own step by `refine`, so
    off-grid values are present, and optionally reaches outside the range."""
    step = V[1] - V[0]
    fine = step / refine
    lo_k = 0
    hi_k = int((V[-1] - V[0]) / fine)
    if out_of_range:
        lo_k -= 2 * refine
        hi_k += 2 * refine
    return [V[0] + k * fine for k in range(lo_k, hi_k + 1)]


def check(V, mode, out_of_range):
    X = concrete_points(V, 4, out_of_range)
    low_fail = 0
    upp_fail = 0
    low_wit = None
    upp_wit = None
    for x in X:
        ax = mode(x, V)
        for v in V:
            if (ax <= v) != (x <= v):
                low_fail += 1
                if low_wit is None:
                    low_wit = (x, ax, v)
            if (v <= x) != (v <= ax):
                upp_fail += 1
                if upp_wit is None:
                    upp_wit = (x, ax, v)
    return low_fail, upp_fail, low_wit, upp_wit, len(X) * len(V)


def run():
    shapes = []
    for I in range(0, 4):
        for F in range(0, 4):
            if I + F >= 1:
                shapes.append((f"U<{I},{F}>", unsigned(I, F)))
    for I in range(1, 4):
        for F in range(0, 3):
            if I + F >= 2:
                shapes.append((f"S<{I},{F}>", signed(I, F)))

    print(f"shapes: {len(shapes)}")
    for out_of_range in (False, True):
        tag = "FULL RANGE (out-of-range x present, clamped)" if out_of_range else "IN RANGE ONLY"
        print()
        print(f"=== {tag} ===")
        print(f"{'mode':>16} | {'LOW fails':>10} | {'UPP fails':>10} | {'pairs':>9}")
        print("-" * 58)
        for name, mode in MODES:
            lf = uf = tot = 0
            lw = uw = None
            for _, V in shapes:
                a, b, w1, w2, n = check(V, mode, out_of_range)
                lf += a
                uf += b
                tot += n
                if lw is None:
                    lw = w1
                if uw is None:
                    uw = w2
            print(f"{name:>16} | {lf:>10} | {uf:>10} | {tot:>9}")
            if lf and lw:
                print(f"{'':>16}   LOW witness: x={lw[0]}, a(x)={lw[1]}, v={lw[2]}")
            if uf and uw:
                print(f"{'':>16}   UPP witness: x={uw[0]}, a(x)={uw[1]}, v={uw[2]}")

    # -- Q2: the insertion condition, a(g(v)) == v, per mode.
    print()
    print("=== Q2: is a . g = id on V (the Galois INSERTION condition)? ===")
    for name, mode in MODES:
        bad = 0
        for _, V in shapes:
            for v in V:
                if mode(v, V) != v:
                    bad += 1
        print(f"{name:>16} | a(g(v)) != v at {bad} of "
              f"{sum(len(V) for _, V in shapes)} representable points")

    # -- Q3: extensivity / reductivity of g . a, which is the closure-operator half.
    print()
    print("=== Q3: g(a(x)) >= x (extensive) and <= x (reductive), in range ===")
    for name, mode in MODES:
        up_bad = dn_bad = tot = 0
        for _, V in shapes:
            for x in concrete_points(V, 4, False):
                ax = mode(x, V)
                tot += 1
                if not ax >= x:
                    up_bad += 1
                if not ax <= x:
                    dn_bad += 1
        print(f"{name:>16} | not extensive at {up_bad:>5} | not reductive at {dn_bad:>5} | of {tot}")


if __name__ == "__main__":
    run()
