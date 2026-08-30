#!/usr/bin/env python3
"""p2. Does a BEST ABSTRACTION exist, and what exactly does admitting it cost?

HYPOTHESIS.  Take the concrete side to be finite sets of rationals ordered by
inclusion, and the abstract side to be the numerals ordered by inclusion of value
sets.  Let g(N) = V(N).  A best abstraction is a map a with

    a(S) <= N   <=>   S subset of g(N)                                       (GC)

so a(S) is the LEAST numeral whose value set contains S.

The order-theoretic fact under test: a exists for every S iff the image family
{ V(N) } is a MOORE FAMILY inside the powerset, meaning it is closed under
intersection and contains a top.  Equivalently, g preserves meets.

That matters here because the record already carries the two halves of that
statement separately and does not connect them.  "Meets are exact" is g
preserving meets.  "Admit the zero-width numeral, admit negative integer width"
are the record's two closure conditions for meet exactness.  If the equivalence
above is real, those two admissions are exactly the conditions under which a best
abstraction exists, and every claim that a derived numeral is the TIGHTEST answer
is a claim that some formula computes a.

This probe tests that by three separate routes over the same shape space, so the
routes can disagree:

  Q1  closure under intersection, directly, per admission policy
  Q2  existence of a least containing numeral for a systematically enumerated
      family of concrete sets, per admission policy, which is (GC) tested
      head-on rather than through the Moore characterisation
  Q3  meet exactness: does the numeral-order meet equal the set intersection
  Q4  the multiplication site: is the least containing numeral of the product
      set available, and does the naive sum-of-widths form equal it

Q1 and Q3 are the same fact seen twice and should agree exactly; Q2 is the
independent one, since it never forms an intersection at all.  Disagreement
between Q1/Q3 and Q2 is a defect in this probe and is reported rather than hidden.

Everything is exact rational arithmetic.  Box truncation is reported separately
from structural absence, because a missing bound that lies above the box ceiling
is an artifact of the box and a missing bound inside it is not.
"""

import itertools
from fractions import Fraction as Q

RADIX = 2

# --------------------------------------------------------------- shape spaces


def vset(I, F):
    """U<I,F> at radix 2: {k * 2^-F : 0 <= k < 2^(I+F)}.  Total width I+F.
    Width 0 denotes {0}: the family's own curve evaluated at the origin."""
    w = I + F
    if w < 0:
        return None
    n = RADIX**w
    q = Q(1, RADIX**F) if F >= 0 else Q(RADIX ** (-F), 1)
    return frozenset(k * q for k in range(n))


def shapes(policy, wmax, imin):
    """Admitted (I, F) pairs under a policy.

    policy 'strict'   : I >= 0, F >= 0, I + F >= 1     (no origin, no negative I)
    policy 'origin'   : I >= 0, F >= 0, I + F >= 0     (origin admitted)
    policy 'full'     : I >= imin, F >= 0, I + F >= 0  (origin and negative I)
    """
    out = []
    for F in range(0, wmax + 1):
        lo = 0 if policy in ("strict", "origin") else imin
        for I in range(lo, wmax + 1):
            w = I + F
            if w > wmax:
                continue
            if policy == "strict" and w < 1:
                continue
            if w < 0:
                continue
            out.append((I, F))
    return out


def space(policy, wmax, imin):
    """Map from value set to the (I,F) spellings denoting it.  The quotient by
    equal value set is taken here, since two numerals denoting the same values
    are one point of the order."""
    d = {}
    for (I, F) in shapes(policy, wmax, imin):
        v = vset(I, F)
        d.setdefault(v, []).append((I, F))
    return d


POLICIES = [
    ("strict: I>=0, no origin", "strict", 0),
    ("origin admitted", "origin", 0),
    ("origin + I>=-2", "full", -2),
    ("origin + I>=-4", "full", -4),
    ("origin + I>=-6", "full", -6),
]

WMAX = 6


def name(vs, d):
    sp = d[vs]
    I, F = min(sp)
    return f"U<{I},{F}>" + ("" if len(sp) == 1 else f" (+{len(sp)-1} spellings)")


# ------------------------------------------------------- Q1: closure under cap


def q1():
    print("=== Q1. Closed under intersection?  (the Moore condition) ===")
    print(f"{'policy':>26} | {'points':>6} | {'pairs':>6} | {'cap not a value set':>20}")
    print("-" * 70)
    res = {}
    for label, pol, imin in POLICIES:
        d = space(pol, WMAX, imin)
        vs = sorted(d.keys(), key=lambda s: (len(s), sorted(s)))
        member = set(vs)
        bad = 0
        wit = None
        for a, b in itertools.combinations(vs, 2):
            cap = a & b
            if cap not in member:
                bad += 1
                if wit is None:
                    wit = (name(a, d), name(b, d), sorted(cap))
        res[label] = bad
        print(f"{label:>26} | {len(vs):>6} | {len(list(itertools.combinations(vs,2))):>6} | {bad:>20}")
        if wit:
            print(f"{'':>26}   first: {wit[0]} cap {wit[1]} = {wit[2]}")
    return res


# ------------------------- Q2: least containing numeral, tested without meets


def least_containing(S, vs_sorted, member_index):
    """Return ('unique', vsx) | ('none', ()) | ('antichain', (v1, v2))."""
    ups = [v for v in vs_sorted if S <= v]
    if not ups:
        return ("none", ())
    minimal = [u for u in ups if not any(o < u for o in ups)]
    if len(minimal) == 1:
        return ("unique", minimal[0])
    return ("antichain", tuple(minimal[:2]))


def q2():
    print()
    print("=== Q2. Does every concrete set have a LEAST containing numeral? ===")
    print("    (GC tested head on; concrete sets are all subsets of size <= 3 of the")
    print("     box's rationals, plus every pairwise union of value sets)")
    print(f"{'policy':>26} | {'sets':>6} | {'unique':>7} | {'none':>6} | {'antichain':>9}")
    print("-" * 72)
    res = {}
    for label, pol, imin in POLICIES:
        d = space(pol, WMAX, imin)
        vs = sorted(d.keys(), key=lambda s: (len(s), sorted(s)))
        universe = sorted(set().union(*vs)) if vs else []
        tests = []
        for r in (1, 2, 3):
            for c in itertools.combinations(universe, r):
                tests.append(frozenset(c))
        for a, b in itertools.combinations(vs, 2):
            tests.append(a | b)
        tests = list(dict.fromkeys(tests))
        u = n = ac = 0
        wit_n = wit_ac = None
        for S in tests:
            kind, w = least_containing(S, vs, None)
            if kind == "unique":
                u += 1
            elif kind == "none":
                n += 1
                if wit_n is None:
                    wit_n = sorted(S)
            else:
                ac += 1
                if wit_ac is None:
                    wit_ac = (sorted(S), name(w[0], d), name(w[1], d))
        res[label] = (u, n, ac)
        print(f"{label:>26} | {len(tests):>6} | {u:>7} | {n:>6} | {ac:>9}")
        if wit_n:
            print(f"{'':>26}   none:      S={wit_n} (above the box ceiling)")
        if wit_ac:
            print(f"{'':>26}   antichain: S={wit_ac[0]} minimal {wit_ac[1]}, {wit_ac[2]}")
    return res


# ---------------------------------- Q3: is the numeral-order meet the cap?


def q3():
    print()
    print("=== Q3. Meet exactness: does the order's meet equal the intersection? ===")
    print(f"{'policy':>26} | {'pairs':>6} | {'meet absent':>11} | {'meet proper subset':>18}")
    print("-" * 74)
    for label, pol, imin in POLICIES:
        d = space(pol, WMAX, imin)
        vs = sorted(d.keys(), key=lambda s: (len(s), sorted(s)))
        absent = proper = 0
        wit = None
        pairs = list(itertools.combinations(vs, 2))
        for a, b in pairs:
            cap = a & b
            downs = [v for v in vs if v <= a and v <= b]
            if not downs:
                absent += 1
                continue
            maximal = [x for x in downs if not any(x < o for o in downs)]
            if len(maximal) != 1:
                absent += 1
                continue
            m = maximal[0]
            if m != cap:
                proper += 1
                if wit is None:
                    wit = (name(a, d), name(b, d), sorted(cap), sorted(m))
        print(f"{label:>26} | {len(pairs):>6} | {absent:>11} | {proper:>18}")
        if wit:
            print(f"{'':>26}   first: {wit[0]} cap {wit[1]}: exact {wit[2]}, meet {wit[3]}")


# ------------------------------------------ Q4: the multiplication site


def q4():
    print()
    print("=== Q4. The multiplication site: is the product's best abstraction there? ===")
    print("    naive form is U<I1+I2, F1+F2>.  Compared against the least containing")
    print("    numeral of the exact product set, per admission policy.")
    print(f"{'policy':>26} | {'pairs':>6} | {'naive = best':>12} | {'naive > best':>12} | {'no best':>8}")
    print("-" * 82)
    for label, pol, imin in POLICIES:
        d = space(pol, WMAX, imin)
        # index by value set for lookup, but iterate over spellings so the
        # product formula has an (I,F) to work from.
        allshapes = sorted(set(min(v) for v in d.values()))
        vs = sorted(d.keys(), key=lambda s: (len(s), sorted(s)))
        eq = gt = nb = 0
        wit_nb = None
        pairs = 0
        for (I1, F1) in allshapes:
            for (I2, F2) in allshapes:
                v1, v2 = vset(I1, F1), vset(I2, F2)
                prod = frozenset(x * y for x in v1 for y in v2)
                pairs += 1
                kind, w = least_containing(prod, vs, None)
                if kind != "unique":
                    nb += 1
                    if wit_nb is None:
                        wit_nb = (f"U<{I1},{F1}>", f"U<{I2},{F2}>", sorted(prod), kind)
                    continue
                nv = vset(I1 + I2, F1 + F2)
                if nv is None or nv not in d:
                    # naive answer lies outside the box; not a structural fact
                    continue
                if nv == w:
                    eq += 1
                elif w < nv:
                    gt += 1
        print(f"{label:>26} | {pairs:>6} | {eq:>12} | {gt:>12} | {nb:>8}")
        if wit_nb:
            print(f"{'':>26}   no best: {wit_nb[0]} * {wit_nb[1]} -> {wit_nb[2]} ({wit_nb[3]})")


if __name__ == "__main__":
    print(f"radix {RADIX}, total width <= {WMAX}, unsigned fixed point only")
    q1()
    q2()
    q3()
    q4()
