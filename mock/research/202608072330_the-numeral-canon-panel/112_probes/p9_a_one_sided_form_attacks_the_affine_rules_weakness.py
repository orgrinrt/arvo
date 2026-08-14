#!/usr/bin/env python3
"""
p9. Attacking the mechanism p7 identified, rather than accepting the union.

THE MECHANISM
-------------
p7 found the affine grade loses on multiplication and named why: an affine form
centres [0, b] at b/2 with radius b/2, so it is symmetric about its centre, and
the product of two symmetric forms carries a negative lower bound the interval
rule never had.  The symmetry is not forced by anything.  It comes from the
convention that noise symbols range over [-1, 1].

THE ATTACK
----------
Take the symbols over [0, 1] instead.  A leaf declared [0, b] becomes `b * e`
with no centre at all, and:

  (x + y) - y  ->  bx*ex + by*ey - by*ey  ->  bx*ex           exact, [0, bx]
  x * y        ->  bx*by * (ex*ey)        ->  fresh symbol at bx*by, [0, bx*by]

so the correlation cancels AND the product keeps its non-negative lower bound.
If that holds, the one-sided form should dominate both the interval rule and
the symmetric affine rule on a non-negative domain, and the union of section
6b becomes one rule rather than two.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. The one-sided form is sound on every row: unsound 0.
P2. It matches the corner rule wherever the corner rule wins (the pure
    multiplication rows), which the symmetric form does not.
P3. It matches the symmetric affine rule wherever that wins (the correlated
    rows), which the corner rule does not.
P4. Therefore it alone reaches the oracle on all ten rows, and the union of
    section 6b is not needed on a non-negative domain.
P5. It does NOT reach the annihilation case, which is not about ranges.
P6. On a SIGNED declaration it must lose something, because a one-sided symbol
    cannot express a leaf that straddles zero without a centre, so this is an
    arm with a predicate rather than a replacement.  I expect the signed rows
    to be where it fails and I want to see how.
"""

from fractions import Fraction
from itertools import product
import importlib.util
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "p7", Path(__file__).with_name("p7_an_affine_grade_recovers_the_lost_licences.py")
)
p7 = importlib.util.module_from_spec(spec)
sys.modules["p7"] = p7
spec.loader.exec_module(p7)

Prim, L, O = p7.Prim, p7.L, p7.O
exact = p7.exact


# ---------------------------------------------------------------------------
# The one-sided form: a constant plus a linear combination of symbols in [0,1].
# ---------------------------------------------------------------------------


class One:
    __slots__ = ("k", "c")

    def __init__(self, k, c=None):
        self.k = k  # the constant term
        self.c = dict(c or {})  # symbol -> coefficient, symbol in [0, 1]

    def interval(self):
        lo = self.k + sum(v for v in self.c.values() if v < 0)
        hi = self.k + sum(v for v in self.c.values() if v > 0)
        return (lo, hi)

    def is_const(self):
        return all(v == 0 for v in self.c.values())


_fresh = [0]


def one_from(lo, hi, sym):
    """A leaf declared [lo, hi] is lo + (hi - lo) * e."""
    return One(lo, {sym: hi - lo})


def one_add(a, b, sign=1):
    out = One(a.k + sign * b.k, a.c)
    for s, v in b.c.items():
        out.c[s] = out.c.get(s, Fraction(0)) + sign * v
    return out


def one_mul(a, b):
    """Exact when one side is constant; otherwise a fresh symbol over [0,1].

    (k1 + sum ai ei)(k2 + sum bj ej)
      = k1 k2 + k2 sum ai ei + k1 sum bj ej + (sum ai ei)(sum bj ej)
    The cross term ranges over [min, max] of the product of the two residual
    ranges, and is carried as one fresh symbol plus a constant shift so the
    bound stays tight when both residuals are non-negative.
    """
    if a.is_const():
        return One(a.k * b.k, {s: a.k * v for s, v in b.c.items()})
    if b.is_const():
        return One(a.k * b.k, {s: b.k * v for s, v in a.c.items()})
    out = One(a.k * b.k, {})
    for s, v in a.c.items():
        out.c[s] = out.c.get(s, Fraction(0)) + v * b.k
    for s, v in b.c.items():
        out.c[s] = out.c.get(s, Fraction(0)) + v * a.k
    ra_lo = sum(v for v in a.c.values() if v < 0)
    ra_hi = sum(v for v in a.c.values() if v > 0)
    rb_lo = sum(v for v in b.c.values() if v < 0)
    rb_hi = sum(v for v in b.c.values() if v > 0)
    corners = [x * y for x in (ra_lo, ra_hi) for y in (rb_lo, rb_hi)]
    lo, hi = min(corners), max(corners)
    _fresh[0] += 1
    out.k += lo
    out.c[f"_g{_fresh[0]}"] = hi - lo
    return out


def one_eval(t, g):
    if t[0] == "leaf":
        return g[t[1]]
    a, b = one_eval(t[2], g), one_eval(t[3], g)
    if t[1] == "add":
        return one_add(a, b, 1)
    if t[1] == "sub":
        return one_add(a, b, -1)
    return one_mul(a, b)


def one_ok(P, t, g):
    if t[0] == "leaf":
        lo, hi = g[t[1]].interval()
        return P.lo <= lo and hi <= P.hi
    if not one_ok(P, t[2], g) or not one_ok(P, t[3], g):
        return False
    lo, hi = one_eval(t, g).interval()
    return P.lo <= lo and hi <= P.hi


def sweep(P, t, rng, label, leaf_lo=None):
    names = sorted(p7.leaves_of(t))
    values = P.values()
    c_yes = a_yes = o_yes = or_yes = u_yes = 0
    c_uns = a_uns = o_uns = 0
    n = 0
    for bs in product(rng, repeat=len(names)):
        lo = Fraction(leaf_lo) if leaf_lo is not None else Fraction(0)
        gi = {k: (lo, Fraction(b)) for k, b in zip(names, bs)}
        ga = {
            k: p7.aff_from_interval(lo, Fraction(b), k) for k, b in zip(names, bs)
        }
        go = {k: one_from(lo, Fraction(b), k) for k, b in zip(names, bs)}
        doms = [[v for v in values if lo <= v <= b] for b in bs]
        if any(len(d) == 0 for d in doms):
            continue
        n += 1
        c = p7.corner_ok(P, t, gi)
        a = p7.affine_ok(P, t, ga)
        o = one_ok(P, t, go)
        oracle = all(
            p7.node_ok_exact(P, t, dict(zip(names, tup))) for tup in product(*doms)
        )
        agrees = all(
            p7.eval_exact(t, dict(zip(names, tup)))
            == p7.eval_general(P, t, dict(zip(names, tup)))
            for tup in product(*doms)
        )
        c_yes += int(c)
        a_yes += int(a)
        o_yes += int(o)
        or_yes += int(oracle)
        u_yes += int(c or a or o)
        if c and not agrees:
            c_uns += 1
        if a and not agrees:
            a_uns += 1
        if o and not agrees:
            o_uns += 1
    print(
        f"  {label:<30} corner {c_yes:>5} symAff {a_yes:>5} ONESIDED {o_yes:>5} "
        f"union {u_yes:>5} oracle {or_yes:>5} of {n:>5}   "
        f"unsound c={c_uns} a={a_uns} o={o_uns}"
    )
    return dict(corner=c_yes, aff=a_yes, one=o_yes, union=u_yes, oracle=or_yes, n=n, o_uns=o_uns)


def main():
    print("=" * 78)
    print("p9. A one-sided form against the symmetric one and the interval rule")
    print("=" * 78)
    print()

    P4 = Prim(4, 0, False, "sat")
    P4w = Prim(4, 0, False, "wrap")
    S4 = Prim(4, 0, True, "sat")

    print("NON-NEGATIVE DECLARATIONS (leaves declared 0..=b)")
    print()
    rows = []
    rows.append(("x + y", sweep(P4, O("add", L("x"), L("y")), range(0, 16), "x + y")))
    rows.append(("x * y", sweep(P4, O("mul", L("x"), L("y")), range(0, 16), "x * y")))
    rows.append(
        (
            "(x + y) + z",
            sweep(P4, O("add", O("add", L("x"), L("y")), L("z")), range(0, 16), "(x + y) + z"),
        )
    )
    rows.append(
        (
            "(x + y) - y",
            sweep(P4, O("sub", O("add", L("x"), L("y")), L("y")), range(0, 16), "(x + y) - y"),
        )
    )
    rows.append(
        (
            "(x + y) * z",
            sweep(P4, O("mul", O("add", L("x"), L("y")), L("z")), range(0, 16), "(x + y) * z"),
        )
    )
    rows.append(
        (
            "(x+y) * (z+w)",
            sweep(
                P4,
                O("mul", O("add", L("x"), L("y")), O("add", L("z"), L("w"))),
                range(0, 4),
                "(x+y) * (z+w)",
            ),
        )
    )
    rows.append(
        (
            "(x + y) - y  WRAP",
            sweep(P4w, O("sub", O("add", L("x"), L("y")), L("y")), range(0, 16), "(x + y) - y  WRAP"),
        )
    )

    print()
    print("SIGNED CONTAINER, leaves still declared 0..=b")
    print()
    rows.append(
        (
            "x * (y - y)  SIGNED",
            sweep(S4, O("mul", L("x"), O("sub", L("y"), L("y"))), range(0, 8), "x * (y - y)  SIGNED"),
        )
    )
    rows.append(
        (
            "(x - y) + y  SIGNED",
            sweep(S4, O("add", O("sub", L("x"), L("y")), L("y")), range(0, 8), "(x - y) + y  SIGNED"),
        )
    )
    rows.append(
        (
            "x*y - x*y  SIGNED",
            sweep(
                S4,
                O("sub", O("mul", L("x"), L("y")), O("mul", L("x"), L("y"))),
                range(0, 4),
                "x*y - x*y  SIGNED",
            ),
        )
    )

    print()
    print("LEAVES THAT STRADDLE ZERO, which is where P6 expects it to fail")
    print()
    rows_s = []
    rows_s.append(
        (
            "x + y, leaves -4..=b",
            sweep(S4, O("add", L("x"), L("y")), range(-4, 8), "x + y, leaves -4..=b", leaf_lo=-4),
        )
    )
    rows_s.append(
        (
            "x * y, leaves -4..=b",
            sweep(S4, O("mul", L("x"), L("y")), range(-4, 8), "x * y, leaves -4..=b", leaf_lo=-4),
        )
    )
    rows_s.append(
        (
            "(x + y) - y, leaves -4..=b",
            sweep(
                S4,
                O("sub", O("add", L("x"), L("y")), L("y")),
                range(-4, 8),
                "(x + y) - y, leaves -4..=b",
                leaf_lo=-4,
            ),
        )
    )

    print()
    print("SUMMARY")
    print()
    allr = rows + rows_s
    print(f"  rows swept                                        : {len(allr)}")
    print(f"  rows where the one-sided form is unsound          : {sum(1 for _, r in allr if r['o_uns'] > 0)}")
    print(
        f"  rows where it alone reaches the oracle            : "
        f"{sum(1 for _, r in allr if r['one'] == r['oracle'])}/{len(allr)}"
    )
    print(
        f"  rows where it beats the corner rule               : "
        f"{sum(1 for _, r in allr if r['one'] > r['corner'])}/{len(allr)}"
    )
    print(
        f"  rows where it beats the symmetric affine rule     : "
        f"{sum(1 for _, r in allr if r['one'] > r['aff'])}/{len(allr)}"
    )
    print(
        f"  rows where it is beaten by one of the other two   : "
        f"{sum(1 for _, r in allr if r['one'] < max(r['corner'], r['aff']))}/{len(allr)}"
    )
    print()
    print("  rows it does NOT reach the oracle on, named:")
    any_short = False
    for label, r in allr:
        if r["one"] != r["oracle"]:
            any_short = True
            print(
                f"    {label:<30} one-sided {r['one']}, oracle {r['oracle']}, "
                f"corner {r['corner']}, symAff {r['aff']}"
            )
    if not any_short:
        print("    none")

    print()
    print("THE ANNIHILATION CASE, since P5 predicts it is still out of reach")
    print()
    P = P4
    t = O("mul", O("add", L("x"), L("y")), L("z"))
    names = sorted(p7.leaves_of(t))
    yes = agree = 0
    for bx, by in product(range(0, 16), repeat=2):
        bs = {"x": bx, "y": by, "z": 0}
        go = {k: one_from(Fraction(0), Fraction(bs[k]), k) for k in names}
        doms = [[v for v in P.values() if 0 <= v <= bs[k]] for k in names]
        yes += int(one_ok(P, t, go))
        agree += int(
            all(
                p7.eval_exact(t, dict(zip(names, tup)))
                == p7.eval_general(P, t, dict(zip(names, tup)))
                for tup in product(*doms)
            )
        )
    print(f"  with z declared 0: one-sided licenses {yes}/256, arms agree {agree}/256")

    print()
    print("INSTRUMENT CHECK")
    print()
    print("  MUTATION: break the cancellation by renaming one occurrence")
    t = O("sub", O("add", L("x"), L("y")), L("y2"))
    names = sorted(p7.leaves_of(t))
    yes = tot = 0
    for bx, by in product(range(0, 16), repeat=2):
        go = {
            "x": one_from(Fraction(0), Fraction(bx), "x"),
            "y": one_from(Fraction(0), Fraction(by), "y"),
            "y2": one_from(Fraction(0), Fraction(by), "y2"),
        }
        tot += 1
        yes += int(one_ok(P4, t, go))
    print(
        f"    correlation broken: one-sided licenses {yes}/{tot}, "
        f"which should match the corner rule's 16"
    )


if __name__ == "__main__":
    main()
