#!/usr/bin/env python3
"""
p2b. The repair of p2, with both of p2's defects fixed, and the two regions of
     R separated by the two parts of the grade.

WHAT WAS WRONG WITH p2, IN ITS AUTHOR'S OWN PROBE
-------------------------------------------------
1. The `conservative` counter could not fire.  It asked whether the measured
   arm agreed with the general arm when the rule refused, but the measured arm
   FELL BACK to the general arm at a refusing node, so the two were the same
   function by construction.  Repaired here by measuring conservatism against
   the FULLY cheap arm, which is what "the rule declined a freedom it had"
   actually means.

2. Q4's grid bound was passed the primitive's own step as the declared leaf
   step, which is the finest grid there is, so a product of two leaves was
   never on the primitive's grid and the grid bound could never discharge.
   Repaired here by declaring a coarser leaf step, which is what a grid
   declaration is.

WHAT THIS MEASURES
------------------
A grade is two independent parts, because R has two regions (`110` section 2):
completion acts outside the range, rounding acts between grid points.

  - a MAGNITUDE part: an interval the value is declared to lie in.
  - a GRID part: a step the value is declared to be a multiple of.

Each is propagated by its own rule.  The claim under test is that each part
switches off exactly one region of R and leaves the other alone, so a design
does not need two mechanisms and does not get one mechanism that over-claims.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. With conservatism measured properly, the magnitude rule is still exact in
    the unsound direction and is NOT exact in the conservative direction: some
    extents that refuse would in fact have agreed.  `111` F111-9 reports zero
    conservative; I expect that not to reproduce once the counter can fire,
    because the corner rule is an interval bound and interval bounds are not
    tight on a term with repeated leaves.

P2. Magnitude discharged => two overflow policies agree on the term.
P3. Magnitude discharged => two total widths agree, and two signednesses
    agree.  (`111` section 9.6 expected signedness not to have a propagable
    quantity.)
P4. Magnitude discharged does NOT make two rounding modes agree at F > 0.
P5. Grid discharged DOES make two rounding modes agree, and does NOT make two
    overflow policies agree.
P6. No discharge of either kind merges the two assignments as types.
"""

from fractions import Fraction
from itertools import product


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

    def quantise(self, q):
        x = q / self.step
        if self.rounding == "trunc":
            k = int(x) if x >= 0 else -int(-x)
        elif self.rounding == "floor":
            k = x.numerator // x.denominator
        elif self.rounding == "near":
            fl = x.numerator // x.denominator
            frac = x - fl
            if frac > Fraction(1, 2):
                k = fl + 1
            elif frac < Fraction(1, 2):
                k = fl
            else:
                k = fl if fl % 2 == 0 else fl + 1
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
        return self.complete(self.quantise(exact(name, args))) * self.step


def exact(name, args):
    if name == "add":
        return args[0] + args[1]
    if name == "sub":
        return args[0] - args[1]
    if name == "mul":
        return args[0] * args[1]
    raise ValueError(name)


class Grade:
    """A declared magnitude interval and a declared grid step."""

    def __init__(self, lo, hi, step):
        self.lo, self.hi, self.step = lo, hi, step

    def __repr__(self):
        return f"[{self.lo},{self.hi}]@{self.step}"


def propagate(name, g, h):
    corners = [exact(name, (a, b)) for a in (g.lo, g.hi) for b in (h.lo, h.hi)]
    step = g.step * h.step if name == "mul" else min(g.step, h.step)
    return Grade(min(corners), max(corners), step)


def mag_ok(P, g):
    return P.lo <= g.lo and g.hi <= P.hi


def grid_ok(P, g):
    return (g.step / P.step).denominator == 1


def leaf_set(P, g):
    return [
        v for v in P.values() if g.lo <= v <= g.hi and (v / g.step).denominator == 1
    ]


def eval_general(P, name, vals):
    acc = vals[0]
    for v in vals[1:]:
        acc = P.general(name, (acc, v))
    return acc


def eval_fully_cheap(name, vals):
    """No R at all, anywhere.  Exact arithmetic."""
    acc = vals[0]
    for v in vals[1:]:
        acc = exact(name, (acc, v))
    return acc


def rule_discharges(P, name, grades, want_mag, want_grid):
    g = grades[0]
    ok = (mag_ok(P, g) if want_mag else True) and (grid_ok(P, g) if want_grid else True)
    for h in grades[1:]:
        g = propagate(name, g, h)
        ok = (
            ok
            and (mag_ok(P, g) if want_mag else True)
            and (grid_ok(P, g) if want_grid else True)
        )
    return ok


def exactness(P, name, arity, bound_range, leaf_step, want_mag, want_grid, uniform):
    """Both error directions, each against the FULLY cheap arm."""
    unsound = conservative = exactly = cells = extents = 0
    combos = (
        [(b,) * arity for b in bound_range]
        if uniform
        else list(product(bound_range, repeat=arity))
    )
    for bs in combos:
        grades = [Grade(Fraction(0), Fraction(b) * leaf_step, leaf_step) for b in bs]
        sets = [leaf_set(P, g) for g in grades]
        if any(len(s) == 0 for s in sets):
            continue
        extents += 1
        says = rule_discharges(P, name, grades, want_mag, want_grid)
        agrees = True
        for vals in product(*sets):
            cells += 1
            if eval_fully_cheap(name, list(vals)) != eval_general(P, name, list(vals)):
                agrees = False
                break
        if says and not agrees:
            unsound += 1
        elif (not says) and agrees:
            conservative += 1
        else:
            exactly += 1
    return unsound, conservative, exactly, extents, cells


def axis_on_term(PA, PB, name, arity, grades, want_mag, want_grid):
    ok = rule_discharges(PA, name, grades, want_mag, want_grid) and rule_discharges(
        PB, name, grades, want_mag, want_grid
    )
    sets = [
        [v for v in leaf_set(PA, g) if v in set(PB.values())] for g in grades
    ]
    diff = cells = 0
    for vals in product(*sets):
        cells += 1
        if eval_general(PA, name, list(vals)) != eval_general(PB, name, list(vals)):
            diff += 1
    return diff, cells, ok


def types_merge(PA, PB):
    if PA.values() != PB.values():
        return False
    for nm in ("add", "sub", "mul"):
        for x in PA.values():
            for y in PA.values():
                if PA.general(nm, (x, y)) != PB.general(nm, (x, y)):
                    return False
    return True


def main():
    print("=" * 78)
    print("p2b. The grade repaired: two parts, two regions of R")
    print("=" * 78)

    print()
    print("Q1 REPAIRED. Both error directions against the fully cheap arm.")
    print()
    print(
        f"  {'setting':<44} {'unsound':>8} {'conserv':>8} {'exact':>7} {'extents':>8}"
    )
    settings = [
        ("W=4 sat  add arity=2, all bound pairs", 4, 0, "sat", "trunc", "add", 2, False),
        ("W=4 wrap add arity=2, all bound pairs", 4, 0, "wrap", "trunc", "add", 2, False),
        ("W=4 sat  mul arity=2, all bound pairs", 4, 0, "sat", "trunc", "mul", 2, False),
        ("W=5 sat  add arity=2, all bound pairs", 5, 0, "sat", "trunc", "add", 2, False),
        ("W=4 sat  add arity=3, uniform bounds", 4, 0, "sat", "trunc", "add", 3, True),
        ("W=5 wrap add arity=3, uniform bounds", 5, 0, "wrap", "trunc", "add", 3, True),
    ]
    for label, W, F, pol, rnd, name, arity, uni in settings:
        P = Prim(W, F, False, pol, rnd)
        u, c, e, ex, cells = exactness(
            P, name, arity, range(0, 2**W), Fraction(1), True, False, uni
        )
        print(f"  {label:<44} {u:>8} {c:>8} {e:>7} {ex:>8}")

    print()
    print("  A conservative cell, named rather than left inside a ratio:")
    P = Prim(4, 0, False, "sat", "trunc")
    named = 0
    for a, b in product(range(16), repeat=2):
        grades = [Grade(Fraction(0), Fraction(a), Fraction(1)), Grade(Fraction(0), Fraction(b), Fraction(1))]
        says = rule_discharges(P, "add", grades, True, False)
        sets = [leaf_set(P, g) for g in grades]
        agrees = all(
            eval_fully_cheap("add", list(v)) == eval_general(P, "add", list(v))
            for v in product(*sets)
        )
        if (not says) and agrees and named < 3:
            print(
                f"    bounds ({a},{b}): rule refuses because {a}+{b} > 15, "
                f"and every pair inside the extents still agrees"
            )
            named += 1

    print()
    print("Q2-Q6. Which part of the grade licenses which axis substitution")
    print()
    print(
        f"  {'axis moved':<30} {'grade':<26} {'disch':>6} {'term differs':>14} "
        f"{'types merge':>12}"
    )

    def row(label, PA, PB, name, arity, grades, want_mag, want_grid, gtag):
        d, cells, ok = axis_on_term(PA, PB, name, arity, grades, want_mag, want_grid)
        m = types_merge(PA, PB)
        print(
            f"  {label:<30} {gtag:<26} {str(ok):>6} "
            f"{str(d) + '/' + str(cells):>14} {str(m):>12}"
        )

    one = Fraction(1)
    sat4 = Prim(4, 0, False, "sat", "trunc")
    wrp4 = Prim(4, 0, False, "wrap", "trunc")
    g7 = [Grade(Fraction(0), Fraction(7), one)] * 2
    g15 = [Grade(Fraction(0), Fraction(15), one)] * 2
    row("overflow policy", sat4, wrp4, "add", 2, g7, True, False, "magnitude <= 7")
    row("overflow policy", sat4, wrp4, "add", 2, g15, True, False, "magnitude <= 15")

    w3 = Prim(3, 0, False, "sat", "trunc")
    g3 = [Grade(Fraction(0), Fraction(3), one)] * 2
    row("total width 3 vs 4", w3, sat4, "add", 2, g3, True, False, "magnitude <= 3")
    row("total width 3 vs 4", w3, sat4, "add", 2, g7, True, False, "magnitude <= 7")

    sgn4 = Prim(4, 0, True, "sat", "trunc")
    row("signedness", sat4, sgn4, "add", 2, g3, True, False, "magnitude <= 3")
    row("signedness", sat4, sgn4, "add", 2, g7, True, False, "magnitude <= 7")

    # rounding, at F = 2, with a coarse declared grid on the leaves
    rt = Prim(6, 2, False, "sat", "trunc")
    rn = Prim(6, 2, False, "sat", "near")
    quarter = Fraction(1, 4)
    fine = [Grade(Fraction(0), Fraction(3), quarter)] * 2
    coarse = [Grade(Fraction(0), Fraction(3), one)] * 2
    row("rounding trunc vs near", rt, rn, "mul", 2, fine, True, False, "magnitude <= 3, grid 1/4")
    row("rounding trunc vs near", rt, rn, "mul", 2, coarse, True, True, "grid 1 (integers)")
    row("rounding trunc vs near", rt, rn, "mul", 2, coarse, True, False, "grid 1, magnitude only")

    sw = Prim(6, 2, False, "wrap", "trunc")
    row("overflow policy at F=2", rt, sw, "mul", 2, coarse, True, True, "grid 1 (integers)")
    row(
        "overflow policy at F=2",
        rt,
        sw,
        "mul",
        2,
        [Grade(Fraction(0), Fraction(15), one)] * 2,
        False,
        True,
        "grid 1, grid part only",
    )

    print()
    print("The licence is denotational and is not representational")
    print()
    diff_bits = sum(1 for k in range(16) if (k if k < 8 else k - 16) != k)
    d, cells, ok = axis_on_term(sat4, sgn4, "add", 2, g3, True, False)
    print(f"  unsigned vs signed, bit patterns whose value differs: {diff_bits}/16")
    print(f"  unsigned vs signed, arithmetic on a discharged extent: {d}/{cells}")
    print(
        "  so the grade licenses substituting the OPERATION and never "
        "reinterpreting the BITS"
    )

    print()
    print("INSTRUMENT CHECK")
    print()
    u, c, e, ex, cells = exactness(
        Prim(4, 0, False, "sat", "trunc"),
        "add",
        2,
        range(0, 16),
        Fraction(1),
        True,
        False,
        False,
    )
    print(f"  baseline W=4 sat add: unsound {u}, conservative {c}, exact {e}, extents {ex}")
    print(
        f"  all three counters are live: "
        f"{sum(1 for x in (u, c, e) if x > 0)} of 3 non-zero"
    )

    global propagate
    good = propagate

    def loose(name, g, h):
        return Grade(min(g.lo, h.lo), max(g.hi, h.hi), min(g.step, h.step))

    propagate = loose
    u2, c2, e2, ex2, _ = exactness(
        Prim(4, 0, False, "sat", "trunc"),
        "add",
        2,
        range(0, 16),
        Fraction(1),
        True,
        False,
        False,
    )
    propagate = good
    print(f"  with a broken (max, not sum) rule: unsound {u2}, conservative {c2}")
    print(f"  the unsound counter detects it: {u2 > u}")

    def tight(name, g, h):
        # a rule that always claims the exact result set, which is unreachable
        # in a const predicate but is the tightest possible answer
        return propagate_exact(name, g, h)

    def propagate_exact(name, g, h):
        vs = [
            exact(name, (a, b))
            for a in frange(g.lo, g.hi, g.step)
            for b in frange(h.lo, h.hi, h.step)
        ]
        return Grade(min(vs), max(vs), g.step * h.step if name == "mul" else min(g.step, h.step))

    def frange(lo, hi, step):
        out, x = [], lo
        while x <= hi:
            out.append(x)
            x += step
        return out

    propagate = tight
    u3, c3, e3, ex3, _ = exactness(
        Prim(4, 0, False, "sat", "trunc"),
        "add",
        2,
        range(0, 16),
        Fraction(1),
        True,
        False,
        False,
    )
    propagate = good
    print(
        f"  with an exact (enumerate the result set) rule: "
        f"unsound {u3}, conservative {c3}"
    )
    print(
        f"  so the conservatism is the CORNER rule's, not the grade's: "
        f"{c3 < c}"
    )


if __name__ == "__main__":
    main()
