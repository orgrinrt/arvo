#!/usr/bin/env python3
"""
p9b. Which of the two changes in p9's form is doing the work.

p9's one-sided form differs from the standard affine form in TWO ways at once,
and reporting the win without separating them would be reporting a bundle:

  (a) the symbol range is [0, 1] with a constant term, rather than [-1, 1]
      with a centre;
  (b) the multiplication's cross term is bounded by the CORNERS of the two
      residual ranges, rather than by the symmetric product of their radii.

Standard affine arithmetic uses (b)'s symmetric bound, which costs
+/- (radius_a * radius_b) whatever the residuals' signs.  The corner bound is
[0, ra_hi * rb_hi] when both residuals are non-negative, which is half as wide
and has the right sign.

This probe adds a third arm that keeps the STANDARD centre-and-radius basis and
changes ONLY the cross-term bound, so the two changes are separated.

PREDICTION, RECORDED BEFORE THE RUN
-----------------------------------
The corner cross term alone recovers the whole win, so arm (b) matches p9's
one-sided form on every row and the basis in (a) is incidental.  If instead the
basis matters, the third arm sits between the two and I will have found that
the two changes each contribute.
"""

from fractions import Fraction
from itertools import product
import importlib.util, sys
from pathlib import Path

def load(name, fn):
    spec = importlib.util.spec_from_file_location(name, Path(__file__).with_name(fn))
    m = importlib.util.module_from_spec(spec)
    sys.modules[name] = m
    spec.loader.exec_module(m)
    return m

p7 = load("p7", "p7_an_affine_grade_recovers_the_lost_licences.py")
p9 = load("p9", "p9_a_one_sided_form_attacks_the_affine_rules_weakness.py")
Prim, L, O = p7.Prim, p7.L, p7.O

_f = [0]

def cmul(a, b):
    """Standard centre/radius basis, CORNER cross-term bound."""
    if a.is_const():
        return p7.Aff(a.c0 * b.c0, {k: a.c0 * v for k, v in b.c.items()})
    if b.is_const():
        return p7.Aff(a.c0 * b.c0, {k: b.c0 * v for k, v in a.c.items()})
    out = p7.Aff(a.c0 * b.c0, {})
    for k, v in a.c.items():
        out.c[k] = out.c.get(k, Fraction(0)) + v * b.c0
    for k, v in b.c.items():
        out.c[k] = out.c.get(k, Fraction(0)) + v * a.c0
    ra, rb = a.radius(), b.radius()
    corners = [x * y for x in (-ra, ra) for y in (-rb, rb)]
    lo, hi = min(corners), max(corners)
    _f[0] += 1
    out.c0 += (lo + hi) / 2
    out.c[f"_h{_f[0]}"] = (hi - lo) / 2
    return out

def ceval(t, g):
    if t[0] == "leaf":
        return g[t[1]]
    a, b = ceval(t[2], g), ceval(t[3], g)
    if t[1] == "add":
        return p7.aff_add(a, b, 1)
    if t[1] == "sub":
        return p7.aff_add(a, b, -1)
    return cmul(a, b)

def cok(P, t, g):
    if t[0] == "leaf":
        lo, hi = g[t[1]].interval()
        return P.lo <= lo and hi <= P.hi
    if not cok(P, t[2], g) or not cok(P, t[3], g):
        return False
    lo, hi = ceval(t, g).interval()
    return P.lo <= lo and hi <= P.hi

def sweep(P, t, rng, label, leaf_lo=0):
    names = sorted(p7.leaves_of(t))
    vals = P.values()
    a_yes = b_yes = o_yes = orc = n = 0
    b_uns = 0
    for bs in product(rng, repeat=len(names)):
        lo = Fraction(leaf_lo)
        ga = {k: p7.aff_from_interval(lo, Fraction(v), k) for k, v in zip(names, bs)}
        go = {k: p9.one_from(lo, Fraction(v), k) for k, v in zip(names, bs)}
        gc = {k: p7.aff_from_interval(lo, Fraction(v), k) for k, v in zip(names, bs)}
        doms = [[v for v in vals if lo <= v <= b] for b in bs]
        if any(len(d) == 0 for d in doms):
            continue
        n += 1
        a_yes += int(p7.affine_ok(P, t, ga))
        o_yes += int(p9.one_ok(P, t, go))
        cb = cok(P, t, gc)
        b_yes += int(cb)
        orc += int(all(p7.node_ok_exact(P, t, dict(zip(names, tp))) for tp in product(*doms)))
        if cb and not all(
            p7.eval_exact(t, dict(zip(names, tp))) == p7.eval_general(P, t, dict(zip(names, tp)))
            for tp in product(*doms)
        ):
            b_uns += 1
    print(
        f"  {label:<30} symAff {a_yes:>5} CORNER-XTERM {b_yes:>5} "
        f"ONESIDED {o_yes:>5} oracle {orc:>5} of {n:>5}  unsound(b)={b_uns}"
    )
    return b_yes, o_yes, orc

def main():
    print("=" * 78)
    print("p9b. Isolating which change won")
    print("=" * 78)
    print()
    P4 = Prim(4, 0, False, "sat")
    S4 = Prim(4, 0, True, "sat")
    rows = []
    rows.append(sweep(P4, O("mul", L("x"), L("y")), range(0, 16), "x * y"))
    rows.append(sweep(P4, O("mul", O("add", L("x"), L("y")), L("z")), range(0, 16), "(x + y) * z"))
    rows.append(
        sweep(
            P4,
            O("mul", O("add", L("x"), L("y")), O("add", L("z"), L("w"))),
            range(0, 4),
            "(x+y) * (z+w)",
        )
    )
    rows.append(sweep(P4, O("sub", O("add", L("x"), L("y")), L("y")), range(0, 16), "(x + y) - y"))
    rows.append(
        sweep(S4, O("mul", L("x"), O("sub", L("y"), L("y"))), range(0, 8), "x * (y - y) SIGNED")
    )
    rows.append(sweep(S4, O("mul", L("x"), L("y")), range(-4, 8), "x * y, leaves -4..=b", -4))
    print()
    same = sum(1 for b, o, _ in rows if b == o)
    reach = sum(1 for b, o, orc in rows if b == orc)
    print(f"  rows where the corner cross term matches the one-sided form: {same}/{len(rows)}")
    print(f"  rows where the corner cross term alone reaches the oracle  : {reach}/{len(rows)}")
    print()
    if same == len(rows):
        print("  So the win is the CROSS-TERM BOUND, not the basis. The [0,1] symbol")
        print("  range in p9 is incidental and the two forms are the same mechanism.")
    else:
        print("  So the two changes each contribute and neither alone is the answer.")

if __name__ == "__main__":
    main()
