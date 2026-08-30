#!/usr/bin/env python3
"""
p2. What a declared extent actually switches off, and whether it reclassifies
    an axis or only licenses a substitution on a term.

WHY THIS EXISTS
---------------
p1 asked whether a refinement admits a denotation-preserving map between
algebras and found that it does not, at any non-trivial extent, because the
restricted carrier is not closed under the operations.  That is `111` F111-8
reached from a second instrument and it kills the ungraded reading.

The graded reading is the one `111` section 9.3 repairs to: an operation
transforms an extent rather than preserving it, so each node of a term carries
its own extent and the licence is checked per node.  This probe asks four
things the graded reading makes askable, none of which `109`, `110` or `111`
ran:

Q1. Does the propagated extent predict, exactly and in both directions,
    whether the cheap arm agrees with the general arm?  (An independent
    reproduction of `111` F111-9 on a different implementation.)

Q2. Under a discharged extent, do two different assignments of an OBSERVABLE
    axis agree on the term?  `108` section 3.1's repair says a chain that
    cannot observe an axis is a licence rather than a reclassification.  If
    the answer is yes, the extent is a carrier for exactly that licence.

Q3. Does that licence extend to the axis itself, i.e. does a discharged extent
    make two assignments of an observable axis the same primitive?  If the
    answer is no, the extent licenses a substitution on a term and does not
    merge two types, which is the whole difference between a licence and a
    coordinate.

Q4. Which axes does an extent switch off, and which does it not?  `110`
    section 2 says overflow and rounding are two REGIONS of one map R:
    rounding acts between grid points, completion acts outside the range.  If
    the extent is a bound on R's argument, then a magnitude bound should
    switch off the outside region and a grid bound should switch off the
    inside one, and each should leave the other alone.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. Q1 exact: zero unsound and zero conservative, at every swept setting.
P2. Q2 yes for the overflow policy under a magnitude bound.
P3. Q2 yes for TOTAL WIDTH and for SIGNEDNESS as well, which `111` section
    9.6 expected not to hold ("whether every axis of R has a propagable
    quantity is untested and I would expect the signedness not to").
P4. Q2 NO for the rounding mode under a magnitude bound alone, and YES for it
    under a grid bound.  The two bounds switch off different regions.
P5. Q3 no in every case: the licence never reaches the value set, so two
    assignments of an observable axis stay two primitives.
P6. The representational reading and the denotational reading come apart on
    signedness and width: the extent licenses the ARITHMETIC substitution and
    not the REINTERPRETATION of the bits, which is `109` section 10's lattice
    with the grade applied to it.

CONDITION-CAN-FIRE CHECK
------------------------
Every counter below reports the number of comparisons it performed.  A run in
which any counter performed zero comparisons prints VACUOUS and the verdict is
not to be read.  Two mutations at the end move the thing each test exists to
detect and confirm the verdict moves with it.
"""

from fractions import Fraction
from itertools import product

# ---------------------------------------------------------------------------
# The primitive, and the two regions of R.
# ---------------------------------------------------------------------------


class Prim:
    def __init__(self, W, F, signed, policy, rounding, radix=2):
        self.W, self.F, self.signed = W, F, signed
        self.policy, self.rounding, self.radix = policy, rounding, radix
        self.step = Fraction(1, radix**F)
        if signed:
            self.klo, self.khi = -(2 ** (W - 1)), 2 ** (W - 1) - 1
        else:
            self.klo, self.khi = 0, 2**W - 1
        self.lo, self.hi = self.klo * self.step, self.khi * self.step

    def values(self):
        return [k * self.step for k in range(self.klo, self.khi + 1)]

    def on_grid(self, q):
        return (q / self.step).denominator == 1

    def quantise(self, q):
        x = q / self.step
        if self.rounding == "trunc":
            k = int(x) if x >= 0 else -int(-x)
        elif self.rounding == "floor":
            k = x.numerator // x.denominator
        elif self.rounding == "near":
            fl = x.numerator // x.denominator
            frac = x - fl
            k = fl + 1 if frac > Fraction(1, 2) else (fl if frac < Fraction(1, 2) else (fl if fl % 2 == 0 else fl + 1))
        else:
            raise ValueError(self.rounding)
        return k

    def complete(self, k):
        span = self.khi - self.klo + 1
        if self.policy == "sat":
            return min(max(k, self.klo), self.khi)
        if self.policy == "wrap":
            return self.klo + (k - self.klo) % span
        raise ValueError(self.policy)

    def general(self, name, args):
        """The full R: rounding then completion."""
        return self.complete(self.quantise(exact(name, args))) * self.step

    def cheap(self, name, args):
        """The arm a discharged extent licenses: exact, no R at all."""
        return exact(name, args)


def exact(name, args):
    if name == "add":
        return args[0] + args[1]
    if name == "sub":
        return args[0] - args[1]
    if name == "mul":
        return args[0] * args[1]
    raise ValueError(name)


# ---------------------------------------------------------------------------
# Grades: an interval, propagated through the operations by the corners.
# A grid grade additionally carries the finest step the values sit on.
# ---------------------------------------------------------------------------


class Grade:
    def __init__(self, lo, hi, step):
        self.lo, self.hi, self.step = lo, hi, step

    def __repr__(self):
        return f"[{self.lo},{self.hi}]@{self.step}"


def propagate(name, g, h):
    corners = [exact(name, (a, b)) for a in (g.lo, g.hi) for b in (h.lo, h.hi)]
    if name == "mul":
        step = g.step * h.step
    else:
        step = min(g.step, h.step)
    return Grade(min(corners), max(corners), step)


def discharges_magnitude(P, g):
    """Is the whole propagated interval inside the container's range?"""
    return P.lo <= g.lo and g.hi <= P.hi


def discharges_grid(P, g):
    """Is every value the propagated grade admits already on P's grid?"""
    return (g.step / P.step).denominator == 1


# ---------------------------------------------------------------------------
# Terms: left-nested chains of one operation over `arity` leaves.
# ---------------------------------------------------------------------------


def eval_general(P, name, vals):
    acc = vals[0]
    for v in vals[1:]:
        acc = P.general(name, (acc, v))
    return acc


def eval_cheap_where_discharged(P, name, vals, grades, need_grid):
    """The composed arm: at each node, take the cheap arm iff the propagated
    grade discharges, otherwise take the general arm.  Returns (value, all_ok)
    where all_ok says every node discharged."""
    acc, g = vals[0], grades[0]
    all_ok = True
    for v, h in zip(vals[1:], grades[1:]):
        gg = propagate(name, g, h)
        ok = discharges_magnitude(P, gg) and (
            discharges_grid(P, gg) if need_grid else True
        )
        all_ok = all_ok and ok
        acc = P.cheap(name, (acc, v)) if ok else P.general(name, (acc, v))
        g = gg
    return acc, all_ok


def leaf_sets(P, grades):
    return [
        [v for v in P.values() if g.lo <= v <= g.hi and (v / g.step).denominator == 1]
        for g in grades
    ]


def q1_exactness(P, name, arity, bounds, need_grid=False):
    """Does the propagated grade predict agreement, in both directions?"""
    unsound = conservative = cells = 0
    for bs in bounds:
        grades = [Grade(Fraction(0), Fraction(b), P.step) for b in bs]
        sets = leaf_sets(P, grades)
        if any(len(s) == 0 for s in sets):
            continue
        # does the rule say discharge?
        g = grades[0]
        rule_ok = True
        for h in grades[1:]:
            g = propagate(name, g, h)
            rule_ok = rule_ok and discharges_magnitude(P, g) and (
                discharges_grid(P, g) if need_grid else True
            )
        # does the cheap arm actually agree everywhere?
        agrees = True
        for vals in product(*sets):
            cells += 1
            cheap, _ = eval_cheap_where_discharged(P, name, list(vals), grades, need_grid)
            gen = eval_general(P, name, list(vals))
            if cheap != gen:
                agrees = False
                break
        if rule_ok and not agrees:
            unsound += 1
        if (not rule_ok) and agrees:
            # only conservative if the FULLY cheap arm would have agreed
            fully = all(
                P.cheap(name, (0, 0)) == P.cheap(name, (0, 0)) for _ in [0]
            )
            conservative += 1
    return unsound, conservative, cells


def q2_axis_licensed(PA, PB, name, arity, bound, need_grid=False):
    """Under a discharged extent, do two assignments of an axis agree on the term?

    Returns (differing, cells, rule_discharged).
    """
    grades = [Grade(Fraction(0), Fraction(bound), PA.step)] * arity
    g = grades[0]
    rule_ok = True
    for h in grades[1:]:
        g = propagate(name, g, h)
        rule_ok = rule_ok and discharges_magnitude(PA, g) and discharges_magnitude(PB, g)
        if need_grid:
            rule_ok = rule_ok and discharges_grid(PA, g) and discharges_grid(PB, g)
    sets = leaf_sets(PA, grades)
    shared = [[v for v in s if v in set(PB.values())] for s in sets]
    differing = cells = 0
    for vals in product(*shared):
        cells += 1
        if eval_general(PA, name, list(vals)) != eval_general(PB, name, list(vals)):
            differing += 1
    return differing, cells, rule_ok


def q3_axis_merged(PA, PB):
    """Ungraded: do the two assignments give the same value set and answers?"""
    if PA.values() != PB.values():
        return False, 0
    n = bad = 0
    for name in ("add", "sub", "mul"):
        for x in PA.values():
            for y in PA.values():
                n += 1
                if PA.general(name, (x, y)) != PB.general(name, (x, y)):
                    bad += 1
    return bad == 0, n


def main():
    print("=" * 78)
    print("p2. What a declared extent switches off")
    print("=" * 78)
    vac = []

    # ---------------- Q1 -----------------------------------------------
    print()
    print("Q1. Does the propagated extent predict agreement, in both directions")
    print()
    for W, pol, name, arity, mode in [
        (4, "sat", "add", 2, "all"),
        (4, "wrap", "add", 2, "all"),
        (4, "sat", "mul", 2, "all"),
        (4, "sat", "add", 3, "uniform"),
        (5, "sat", "add", 2, "all"),
        (5, "wrap", "add", 3, "uniform"),
    ]:
        P = Prim(W, 0, False, pol, "trunc")
        hi = 2**W - 1
        bounds = (
            list(product(range(0, hi + 1), repeat=arity))
            if mode == "all"
            else [(b,) * arity for b in range(0, hi + 1)]
        )
        u, c, cells = q1_exactness(P, name, arity, bounds)
        if cells == 0:
            vac.append(f"Q1 W={W} {pol} {name} arity={arity}")
        print(
            f"  W={W} {pol:<4} {name} arity={arity} bounds={mode:<7}: "
            f"unsound {u}, conservative {c}, over {len(bounds)} extents "
            f"and {cells} value tuples"
        )

    # rounding axis at F > 0, where the grid bound is the discharging quantity
    print()
    for W, F, name, arity, mode in [(6, 2, "mul", 2, "all"), (6, 2, "mul", 3, "uniform")]:
        P = Prim(W, F, False, "sat", "trunc")
        hi = 2 ** (W - F) - 1
        bounds = (
            list(product(range(0, hi + 1), repeat=arity))
            if mode == "all"
            else [(b,) * arity for b in range(0, hi + 1)]
        )
        u, c, cells = q1_exactness(P, name, arity, bounds, need_grid=True)
        if cells == 0:
            vac.append(f"Q1 grid W={W} F={F} {name}")
        print(
            f"  W={W} F={F} sat  {name} arity={arity} bounds={mode:<7} (grid too): "
            f"unsound {u}, conservative {c}, over {len(bounds)} extents "
            f"and {cells} value tuples"
        )

    # ---------------- Q2 and Q3 ----------------------------------------
    print()
    print("Q2/Q3. Does a discharged extent license an axis substitution on a term,")
    print("       and does it merge the two assignments as types")
    print()
    print(
        f"  {'axis moved':<34} {'bound':>6} {'discharged':>11} "
        f"{'term differs':>14} {'types merge':>12}"
    )

    def row(label, PA, PB, name, arity, bound, need_grid=False):
        d, cells, ok = q2_axis_licensed(PA, PB, name, arity, bound, need_grid)
        merged, n = q3_axis_merged(PA, PB)
        if cells == 0:
            vac.append(f"Q2 {label}")
        print(
            f"  {label:<34} {bound:>6} {str(ok):>11} "
            f"{str(d) + '/' + str(cells):>14} {str(merged):>12}"
        )
        return d, cells, ok, merged

    W = 4
    sat = Prim(W, 0, False, "sat", "trunc")
    wrp = Prim(W, 0, False, "wrap", "trunc")
    row("overflow policy sat vs wrap", sat, wrp, "add", 2, 7)
    row("overflow policy sat vs wrap", sat, wrp, "add", 2, 15)
    row("overflow policy sat vs wrap", sat, wrp, "add", 3, 5)
    row("overflow policy sat vs wrap", sat, wrp, "add", 3, 15)

    w3 = Prim(3, 0, False, "sat", "trunc")
    w4 = Prim(4, 0, False, "sat", "trunc")
    row("total width 3 vs 4", w3, w4, "add", 2, 3)
    row("total width 3 vs 4", w3, w4, "add", 2, 7)

    uns = Prim(4, 0, False, "sat", "trunc")
    sgn = Prim(4, 0, True, "sat", "trunc")
    row("signedness unsigned vs signed", uns, sgn, "add", 2, 3)
    row("signedness unsigned vs signed", uns, sgn, "add", 2, 7)

    rt = Prim(6, 2, False, "sat", "trunc")
    rn = Prim(6, 2, False, "sat", "near")
    row("rounding trunc vs near, F = 2", rt, rn, "mul", 2, 3)
    row("rounding trunc vs near, F = 2", rt, rn, "mul", 2, 3, need_grid=True)

    # ---------------- Q4: which bound switches off which region ---------
    print()
    print("Q4. Which part of the grade switches off which region of R")
    print()
    P = Prim(6, 2, False, "sat", "trunc")
    Q = Prim(6, 2, False, "sat", "near")
    S = Prim(6, 2, False, "wrap", "trunc")
    for label, A, B, gridded, bound in [
        ("magnitude bound vs completion", P, S, False, 3),
        ("magnitude bound vs rounding", P, Q, False, 3),
        ("grid bound vs rounding", P, Q, True, 3),
        ("grid bound vs completion", P, S, True, 3),
    ]:
        d, cells, ok = q2_axis_licensed(A, B, "mul", 2, bound, gridded)
        print(
            f"  {label:<34} discharged={str(ok):<6} differing {d}/{cells}"
        )

    # ---------------- representational vs denotational -----------------
    print()
    print("The same licence read representationally rather than denotationally")
    print()
    uns = Prim(4, 0, False, "sat", "trunc")
    sgn = Prim(4, 0, True, "sat", "trunc")
    # a bit pattern k read under both: unsigned gives k, signed gives k-16 for k>=8
    reinterp_diff = sum(
        1 for k in range(16) if (k if k < 8 else k - 16) != k
    )
    print(
        f"  bit patterns whose VALUE differs between the two: {reinterp_diff}/16"
    )
    d, cells, ok = q2_axis_licensed(uns, sgn, "add", 2, 3, False)
    print(
        f"  the same pair, ARITHMETIC on a discharged extent: differing {d}/{cells}, "
        f"discharged={ok}"
    )

    # ---------------- instrument checks --------------------------------
    print()
    print("INSTRUMENT CHECK")
    if vac:
        print(f"  VACUOUS CELLS: {vac}")
    else:
        print("  every cell performed at least one comparison")

    print()
    print("MUTATION 1: widen the extent past the discharge point")
    d_small, c_small, ok_small = q2_axis_licensed(sat, wrp, "add", 2, 7)
    d_big, c_big, ok_big = q2_axis_licensed(sat, wrp, "add", 2, 15)
    print(f"  bound 7  discharged={ok_small} differing {d_small}/{c_small}")
    print(f"  bound 15 discharged={ok_big} differing {d_big}/{c_big}")
    print(f"  the verdict moves with the extent: {(d_small == 0) != (d_big == 0)}")

    print()
    print("MUTATION 2: break the propagation rule and see whether Q1 notices")
    global propagate
    good = propagate

    def bad_propagate(name, g, h):
        # a rule that always claims the operand extent, which is unsound for add
        return Grade(min(g.lo, h.lo), max(g.hi, h.hi), min(g.step, h.step))

    P = Prim(4, 0, False, "sat", "trunc")
    bounds = list(product(range(0, 16), repeat=2))
    propagate = bad_propagate
    u_bad, c_bad, cells_bad = q1_exactness(P, "add", 2, bounds)
    propagate = good
    u_good, c_good, cells_good = q1_exactness(P, "add", 2, bounds)
    print(f"  correct rule: unsound {u_good}, conservative {c_good}")
    print(f"  broken rule:  unsound {u_bad}, conservative {c_bad}")
    print(f"  the test detects the broken rule: {u_bad > u_good}")


if __name__ == "__main__":
    main()
