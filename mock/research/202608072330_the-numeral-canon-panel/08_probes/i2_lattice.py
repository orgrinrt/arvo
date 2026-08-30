#!/usr/bin/env python3
"""i2: does the general concept close where the two named families do not?

`03` establishes that the join of a fixed-point numeral and a float numeral has
two incomparable minimal upper bounds and no least one, and that no admission of
more fixed-point shapes repairs it. `07` prices closing the family under
intersection at a 16 to 34 percent enlargement, every added shape a segmented
grid neither family names.

This instrument asks the question one level up. If a format is a canonical
exponent FUNCTION rather than one of two parametric curves, four things become
measurable:

  Q1  Is the class closed under intersection? (Moore condition, so a tightest
      containing format exists.)
  Q2  Does the join of a fixed shape and a float shape exist in the general
      class, and is it unique?
  Q3  When it exists, does it land back on a named family, or off both?
  Q4  Is the general class strictly larger than the intersection-closure of the
      two named families? A tapered format is the test case.

Bias is held at zero throughout and every step is a power of two, so this
instrument measures Flocq's generic_format and not the design's wider affine
map. i1b measures the difference the bias makes; this one does not.
"""

from fractions import Fraction
import itertools

from i1_classify import binade, flt, posit, fixed
from i1b_classify import classify, name_family


def valuation(v: Fraction, r: int = 2) -> int:
    """The largest k with v a multiple of r^k, for nonzero r-adic v."""
    k = 0
    while v.denominator != 1:
        v *= r
        k -= 1
    while v % r == 0:
        v //= r
        k += 1
    return k


def fexp_of(values, r=2):
    """The tightest per-binade grid exponent carrying the given values.
    +inf (represented as None) where a binade holds nothing."""
    f = {}
    for v in values:
        if v == 0:
            continue
        e = binade(v, r)
        k = valuation(abs(v), r)
        f[e] = k if e not in f else min(f[e], k)
    return f


def realise(f, lo, hi, r=2):
    """The value set of the format with canonical exponent f, over [lo, hi]."""
    out = {Fraction(0)} if lo <= 0 <= hi else set()
    for e, k in f.items():
        step = Fraction(r) ** k
        start = Fraction(r) ** e
        end = Fraction(r) ** (e + 1)
        m = (start / step)
        m = int(m) if m.denominator == 1 else int(m) + 1
        while m * step < end:
            v = m * step
            if lo <= v <= hi:
                out.add(v)
            m += 1
    return sorted(out)


def tightest_containing(values, r=2):
    """alpha of a finite value set: the tightest format holding it. Total by
    construction, which is the whole point."""
    f = fexp_of(values, r)
    nz = [abs(v) for v in values if v != 0]
    return f, (min(nz) if nz else Fraction(0)), (max(nz) if nz else Fraction(0))


POOL = []


def build_pool():
    for I in range(0, 4):
        for F in range(1, 4):
            POOL.append((f"fixed U<{I},{F}>", [v for v in fixed(I, F) if v > 0]))
    for p in range(2, 5):
        for emax in range(1, 4):
            POOL.append((f"float p={p} e=-2..{emax}",
                         [v for v in flt(p, -2, emax, subnormals=False) if v > 0]))


def q1_intersection_closure():
    print("Q1: is the class closed under intersection?")
    both = 0
    inside = 0
    empties = 0
    for (n1, v1), (n2, v2) in itertools.combinations(POOL, 2):
        inter = sorted(set(v1) & set(v2))
        both += 1
        if len(inter) < 2:
            empties += 1
            continue
        res = classify("x", inter, 2)
        if res["verdict"] == "INSIDE":
            inside += 1
    print(f"    pairs                                    : {both}")
    print(f"    intersections with fewer than two values : {empties}")
    print(f"    the rest, and how many are again a format: {inside} of {both - empties}")
    print()


def q2_q3_joins():
    print("Q2/Q3: the join in the general class, and where it lands")
    kinds = {}
    total = 0
    contained = 0
    for (n1, v1), (n2, v2) in itertools.combinations(POOL, 2):
        if n1.startswith("fixed") == n2.startswith("fixed"):
            continue  # cross-kind pairs only, which is 03's case
        total += 1
        union = sorted(set(v1) | set(v2))
        f, lo, hi = tightest_containing(union)
        got = realise(f, min(union), max(union))
        if set(union) <= set(got):
            contained += 1
        res = classify("join", got, 2)
        if res["verdict"] != "INSIDE":
            kinds["NOT A FORMAT"] = kinds.get("NOT A FORMAT", 0) + 1
            continue
        fam = res["family"].split(":")[0]
        kinds[fam] = kinds.get(fam, 0) + 1
    print(f"    cross-kind pairs                    : {total}")
    print(f"    joins that contain both operands    : {contained} of {total}")
    print(f"    where the join's shape lands:")
    for k in sorted(kinds):
        print(f"        {k}: {kinds[k]}")
    print()


def q2b_the_witness():
    print("Q2b: 03's witness, worked")
    a = [v for v in fixed(0, 1) if v >= 0]     # {0, 1/2}
    b = [v for v in fixed(2, 0) if v >= 0]     # {0,1,2,3}
    union = sorted(set(a) | set(b))
    print(f"    U<0,1> = {[str(x) for x in a]}")
    print(f"    U<2,0> = {[str(x) for x in b]}")
    f, lo, hi = tightest_containing([x for x in union if x > 0])
    got = realise(f, 0, max(union))
    print(f"    canonical exponent of the join      : {dict(sorted(f.items()))}")
    print(f"    the join                            : {[str(x) for x in got]}")
    print(f"    contains both operands              : {set(union) <= set(got)}")
    res = classify("join", got, 2)
    print(f"    the join's family                   : {res.get('family', res['verdict'])}")
    # is it strictly below the two minimal upper bounds 03 names?
    u21 = [v for v in fixed(2, 1) if v >= 0]
    flo = [v for v in flt(2, -1, 1, subnormals=False) if v >= 0] + [Fraction(0)]
    print(f"    strictly inside U<2,1>              : "
          f"{set(got) < set(u21)}")
    print(f"    strictly inside the float p2 e-1..1 : "
          f"{set(got) < set(flo)}   "
          f"(float = {[str(x) for x in sorted(set(flo))]})")
    print()


def q4_taper_is_out_of_reach():
    print("Q4: is a tapered format reachable by intersecting the two named families?")
    print("    Intersection takes the canonical exponent pointwise maximum, so the")
    print("    reachable slopes are the maxima of the operands' slopes. Measured,")
    print("    not argued:")
    slopes = set()
    for (n1, v1), (n2, v2) in itertools.combinations(POOL, 2):
        inter = sorted(set(v1) & set(v2))
        if len(inter) < 3:
            continue
        f = fexp_of(inter)
        es = sorted(f)
        for i in range(len(es) - 1):
            if es[i + 1] - es[i] == 1:
                slopes.add(f[es[i + 1]] - f[es[i]])
    print(f"    slopes of the canonical exponent over every pairwise intersection: "
          f"{sorted(slopes)}")
    for n, es_ in ((8, 0), (8, 1), (10, 2)):
        pv = [v for v in posit(n, es_) if v > 0]
        f = fexp_of(pv)
        ks = sorted(f)
        sl = sorted({f[ks[i + 1]] - f[ks[i]]
                     for i in range(len(ks) - 1) if ks[i + 1] - ks[i] == 1})
        print(f"    slopes of posit<{n},{es_}>'s canonical exponent: {sl}")
    print()
    print("    A slope above one is not a maximum of slopes at most one, so a")
    print("    tapered format is not in the intersection-closure of the two")
    print("    named families, while it is in the general class (i1b).")
    print()


def q5_how_many_shapes():
    """How much bigger is the general class than the two named families, over
    one window? Counted by enumerating canonical exponent functions over a
    four-binade window with a bounded precision, rather than by sampling."""
    print("Q5: shape counts over a four-binade window, precision 0..3")
    W = [0, 1, 2, 3]
    all_f = set()
    for combo in itertools.product(range(0, 4), repeat=len(W)):
        all_f.add(tuple(combo))
    named = set()
    for c in range(0, 4):                       # fixed: constant
        named.add(tuple(c for _ in W))
    for c in range(-3, 4):                      # float: slope one
        v = tuple(e - c for e in W)
        if all(0 <= x <= 3 for x in v):
            named.add(v)
    for c in range(-3, 4):                      # float with a knee
        for knee in W:
            v = tuple(max(e - c, knee - c) for e in W)
            if all(0 <= x <= 3 for x in v):
                named.add(v)
    closure = set(named)
    changed = True
    while changed:
        changed = False
        for a, b in itertools.combinations(list(closure), 2):
            m = tuple(max(x, y) for x, y in zip(a, b))
            if m not in closure:
                closure.add(m)
                changed = True
    print(f"    canonical exponent functions in the window : {len(all_f)}")
    print(f"    named by fixed / float / float-with-a-knee : {len(named)}")
    print(f"    plus their intersection closure            : {len(closure)}")
    print(f"    still unnamed by the closure               : {len(all_f) - len(closure)}"
          f"  ({100.0 * (len(all_f) - len(closure)) / len(all_f):.1f} percent)")
    print()


if __name__ == "__main__":
    build_pool()
    print(f"pool: {len(POOL)} shapes ("
          f"{sum(1 for n, _ in POOL if n.startswith('fixed'))} fixed, "
          f"{sum(1 for n, _ in POOL if n.startswith('float'))} float)\n")
    q1_intersection_closure()
    q2_q3_joins()
    q2b_the_witness()
    q4_taper_is_out_of_reach()
    q5_how_many_shapes()
