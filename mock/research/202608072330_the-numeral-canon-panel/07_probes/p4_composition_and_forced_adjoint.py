#!/usr/bin/env python3
"""p4. Two things an adjunction predicts, tested by a method that does not use
the adjunction's defining biconditional at all.

p1 tested the defining biconditional pointwise.  This instrument tests two
CONSEQUENCES instead, so a mode passing p1 and failing here, or the reverse,
would be a real disagreement between two methods rather than a repetition.

FIRST CONSEQUENCE: the forced adjoint.  If a monotone map a has a right adjoint
at all, that adjoint is unique and is forced to be

    g'(v) = max { x in C : a(x) <= v }.

So the question "is a an adjoint" splits into two: does g' exist and satisfy the
biconditional with a, and if so, is g' the plain embedding.  A mode can fail to
be an adjoint to the EMBEDDING while still being an adjoint to something, and
that distinction is invisible to p1.  Measured here.

SECOND CONSEQUENCE: composition, which is the double-rounding question.
Adjunctions compose: a2 . a1 is left adjoint to g1 . g2.  So for two grids with
V2 contained in V1 contained in Q, rounding into V1 and then into V2 must equal
rounding into V2 in one step, for any mode that is an adjoint, and nothing forces
it for a mode that is not.  This is the classical double-rounding statement and
the frame predicts it rather than needing it measured, so measuring it is a test
OF THE FRAME.

The precondition the frame supplies is NESTEDNESS of the grids, which is the same
relation file 03's reading E identifies as the family seam by a completely
different route.  Non-nested grids are measured too, to check the precondition is
doing work rather than decorating a result that holds anyway.
"""

from fractions import Fraction as Q
import itertools

# ------------------------------------------------------------- grids and modes


def grid(step, lo, hi):
    """{lo, lo+step, ..., <= hi} as an exact sorted list."""
    n = int((hi - lo) / step)
    return [lo + k * step for k in range(n + 1)]


def _clamp(x, V):
    if x <= V[0]:
        return V[0]
    if x >= V[-1]:
        return V[-1]
    return None


def r_up(x, V):
    c = _clamp(x, V)
    return c if c is not None else min(v for v in V if v >= x)


def r_down(x, V):
    c = _clamp(x, V)
    return c if c is not None else max(v for v in V if v <= x)


def r_zero(x, V):
    return r_down(x, V) if x >= 0 else r_up(x, V)


def r_away(x, V):
    return r_up(x, V) if x >= 0 else r_down(x, V)


def _near(x, V, tie):
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
    step = V[1] - V[0]
    return _near(x, V, lambda lo, hi: lo if (lo / step) % 2 == 0 else hi)


def r_near_away(x, V):
    return _near(x, V, lambda lo, hi: hi if abs(hi) >= abs(lo) else lo)


MODES = [("toward +inf", r_up), ("toward -inf", r_down), ("toward zero", r_zero),
         ("away from zero", r_away), ("nearest, even", r_near_even),
         ("nearest, away", r_near_away)]


# ------------------------------------------------ Q1: the forced right adjoint


def q1():
    print("=== Q1. The forced right adjoint g'(v) = max{x : a(x) <= v} ===")
    print("    Reported: does g' satisfy the biconditional with a, and is g' the")
    print("    embedding.  'adjoint to something else' is the interesting middle case.")
    print()
    V = grid(Q(1, 4), Q(-2), Q(2))
    C = grid(Q(1, 16), Q(-2), Q(2))  # in-range concrete points only
    for name, m in MODES:
        gprime = {}
        ok = True
        for v in V:
            cand = [x for x in C if m(x, V) <= v]
            gprime[v] = max(cand) if cand else None
        # does the biconditional hold with g' in place of the embedding?
        fails = 0
        for x in C:
            for v in V:
                lhs = m(x, V) <= v
                rhs = (gprime[v] is not None and x <= gprime[v])
                if lhs != rhs:
                    fails += 1
        is_embedding = all(gprime[v] == v for v in V)
        print(f"{name:>16} | biconditional-with-g' fails at {fails:>5} "
              f"| g' is the embedding: {is_embedding}")
    print()
    print("    A mode with 0 failures and g' = embedding is a lower adjoint to the")
    print("    embedding.  0 failures with g' != embedding would be an adjoint to a")
    print("    different upper map.  Nonzero failures means not an adjoint at all,")
    print("    since the forced candidate is the only one that could have worked.")


# ------------------------------------------- Q2: composition, nested and not


def q2():
    print()
    print("=== Q2. Double rounding: is rounding through an intermediate grid the same ===")
    print("    as rounding once?  Nested grids first, then non-nested.")
    print()
    cases = [
        ("nested   1/16 -> 1/4  -> 1/1", Q(1, 16), Q(1, 4), Q(1, 1)),
        ("nested   1/32 -> 1/8  -> 1/2", Q(1, 32), Q(1, 8), Q(1, 2)),
        ("nested   1/64 -> 1/16 -> 1/4", Q(1, 64), Q(1, 16), Q(1, 4)),
        ("NON-nest 1/12 -> 1/4  -> 1/3", Q(1, 12), Q(1, 4), Q(1, 3)),
        ("NON-nest 1/36 -> 1/9  -> 1/4", Q(1, 36), Q(1, 9), Q(1, 4)),
        ("NON-nest 1/30 -> 1/6  -> 1/5", Q(1, 30), Q(1, 6), Q(1, 5)),
    ]
    print(f"{'case':>28} | " + " | ".join(f"{n:>14}" for n, _ in MODES))
    print("-" * (30 + 17 * len(MODES)))
    for label, fine, mid, coarse in cases:
        C = grid(fine, Q(0), Q(4))
        V1 = grid(mid, Q(0), Q(4))
        V2 = grid(coarse, Q(0), Q(4))
        nested = set(V2) <= set(V1)
        row = []
        for _, m in MODES:
            bad = sum(1 for x in C if m(m(x, V1), V2) != m(x, V2))
            row.append(f"{bad:>14}")
        print(f"{label:>28} | " + " | ".join(row) + f"   [V2 in V1: {nested}]")
    print()
    print("    Columns are counts of exact values where two-step rounding differs from")
    print("    one-step.  Zero means the composition law holds on that grid triple.")


# ------------------ Q3: a witness for the classical nearest double-rounding gap


def q3():
    print()
    print("=== Q3. A named witness for round-to-nearest, since a count is not a proof ===")
    C = grid(Q(1, 16), Q(0), Q(4))
    V1 = grid(Q(1, 4), Q(0), Q(4))
    V2 = grid(Q(1, 1), Q(0), Q(4))
    for name, m in MODES:
        w = [x for x in C if m(m(x, V1), V2) != m(x, V2)]
        if w:
            x = w[0]
            print(f"{name:>16} | x={x}  one-step -> {m(x, V2)}  "
                  f"two-step -> {m(m(x, V1), V2)}  (via {m(x, V1)})")
        else:
            print(f"{name:>16} | no witness on this triple")


if __name__ == "__main__":
    q1()
    q2()
    q3()


# --------------------------------------------------------------- appendix, added
# after Q1's result came out the opposite way round from the prediction.  Every
# monotone mode turned out to be a lower adjoint to SOMETHING, because the forced
# candidate is a right adjoint exactly when the map is monotone, and all six are.
# So the separating question is not "is it an adjoint" but "what is its upper
# adjoint", which is what the abstract element MEANS.  Printed here, and the
# composition test is rerun over a signed range so toward-zero and away-from-zero
# stop coinciding with floor and ceil.


def q1b():
    print()
    print("=== Q1b. What each mode's forced upper adjoint actually is ===")
    print("    g'(v) = max{x : a(x) <= v}.  The gap g'(v) - v is what a datum of")
    print("    that mode denotes beyond its own value.")
    V = grid(Q(1, 4), Q(-1), Q(1))
    C = grid(Q(1, 32), Q(-1), Q(1))
    for name, m in MODES:
        gp = {}
        for v in V:
            cand = [x for x in C if m(x, V) <= v]
            gp[v] = max(cand) if cand else None
        gaps = sorted({gp[v] - v for v in V if gp[v] is not None})
        print(f"{name:>16} | g'(v) - v takes values {gaps}")
    print("    step is 1/4 and the concrete lattice's own tick is 1/32, so a gap of")
    print("    7/32 is 'a quarter minus one tick', i.e. the whole half-open cell.")


def q2b():
    print()
    print("=== Q2b. Composition over a SIGNED range, so toward-zero separates ===")
    cases = [
        ("nested   1/16 -> 1/4  -> 1/1", Q(1, 16), Q(1, 4), Q(1, 1)),
        ("nested   1/32 -> 1/8  -> 1/2", Q(1, 32), Q(1, 8), Q(1, 2)),
        ("NON-nest 1/12 -> 1/4  -> 1/3", Q(1, 12), Q(1, 4), Q(1, 3)),
    ]
    print(f"{'case':>28} | " + " | ".join(f"{n:>14}" for n, _ in MODES))
    print("-" * (30 + 17 * len(MODES)))
    for label, fine, mid, coarse in cases:
        C = grid(fine, Q(-2), Q(2))
        V1 = grid(mid, Q(-2), Q(2))
        V2 = grid(coarse, Q(-2), Q(2))
        nested = set(V2) <= set(V1)
        row = [f"{sum(1 for x in C if m(m(x, V1), V2) != m(x, V2)):>14}" for _, m in MODES]
        print(f"{label:>28} | " + " | ".join(row) + f"   [V2 in V1: {nested}]")


q1b()
q2b()


# --------------------------------------------------------------- second appendix
# Q2b refuted the prediction that only the two infinity-directed modes compose.
# Toward-zero and away-from-zero compose too, on nested grids, over a signed range,
# and neither is an adjoint to the embedding (p1 measures both failing the
# biconditional).  So being an adjoint is SUFFICIENT for the composition law and
# not necessary, and the honest claim is narrower.
#
# The refinement under test here: a mode composes on nested grids exactly when it
# is directed on each cell of the COARSE grid, that is, when its direction never
# changes inside a coarse cell.  Toward-zero switches direction at 0, which is a
# point of every grid here, so no coarse cell contains the switch.  A mode whose
# switch point lands strictly inside a coarse cell should therefore fail, and
# that is a decisive test rather than an argument.


def make_pivot_mode(pivot):
    def m(x, V):
        return r_down(x, V) if x >= pivot else r_up(x, V)
    return m


def q4():
    print()
    print("=== Q4. Does the composition law track the pivot's position? ===")
    print("    'away from p' rounds down above p and up below it.  With V2 the")
    print("    integer grid and V1 the quarter grid, a pivot at 0 or 1 sits on both")
    print("    grids; a pivot at 1/2 or 1/4 sits inside a coarse cell.")
    C = grid(Q(1, 32), Q(-2), Q(3))
    V1 = grid(Q(1, 4), Q(-2), Q(3))
    V2 = grid(Q(1, 1), Q(-2), Q(3))
    print(f"    [V2 in V1: {set(V2) <= set(V1)}]")
    for p in [Q(0), Q(1), Q(2), Q(1, 2), Q(1, 4), Q(3, 4), Q(3, 2)]:
        m = make_pivot_mode(p)
        bad = [x for x in C if m(m(x, V1), V2) != m(x, V2)]
        on_coarse = p in set(V2)
        w = ""
        if bad:
            x = bad[0]
            w = f"  first x={x}: one-step {m(x, V2)}, two-step {m(m(x, V1), V2)} via {m(x, V1)}"
        print(f"    pivot {str(p):>4} | on the coarse grid: {str(on_coarse):>5} "
              f"| composition fails at {len(bad):>3}{w}")


q4()
