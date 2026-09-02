"""Truncation-proof test for the failure of binary meets and joins.

An earlier version of this probe counted pairs whose minimal upper bounds were
not unique inside a finite enumeration. That instrument is worthless and I caught
it reporting a false witness: `NN(64,1/8) ^ NN(64,1/4)` looked like a meet failure
only because the count 32 at quantum 1/4 had been excluded by the enumeration's
own precision bound. The intersection was exactly that shape.

The instrument used here needs no enumeration to be complete, and it is the one
148 states at 148:346.

  If a least upper bound L exists for A and B, then L is inside every upper
  bound. Take any two upper bounds C1 and C2. Then L is inside C1 and C2, so
  L is inside their intersection, and L contains the union. If that intersection
  is exactly the union, then L is the union. So exhibiting two upper bounds whose
  intersection is exactly the union, where the union is not an admissible value
  set, proves NO least upper bound exists, whatever else the family contains.

  Dually for the meet, with two lower bounds whose union is exactly the
  intersection.

Every failure reported below is decided by that argument. Nothing is inferred
from a pair being absent from a finite grid.
"""

from fractions import Fraction as Q
from itertools import combinations


def admissible(max_r=12, max_p=8, cap=4096, exps=range(-6, 7)):
    """Pairs (count, quantum). Generous bounds, since the test does not rely
    on the enumeration being complete."""
    out = set()
    for r in range(2, max_r + 1):
        for p in range(1, max_p + 1):
            n = r ** p
            if n > cap:
                break
            for e in exps:
                out.add((n, Q(r) ** e))                    # step a power of r
                if n > 1:
                    out.add((n, Q(r) ** e / (n - 1)))      # r^p points per unit
    return out


ADM = admissible()


def is_valueset(s):
    """Is this finite set of rationals the value set of some numeral, unbiased?

    A value set is a run of equally spaced points whose count and quantum are an
    admissible pair, placed as non-negative, symmetric or asymmetric-low.
    """
    xs = sorted(s)
    if len(xs) < 2:
        return len(xs) == 1 and xs[0] == 0
    q = xs[1] - xs[0]
    if any(xs[i + 1] - xs[i] != q for i in range(len(xs) - 1)):
        return False                                        # not equally spaced
    n = len(xs)
    lo = xs[0] / q
    if lo == 0 and (n, q) in ADM:
        return True                                         # non-negative
    if lo == -(n - 1) / 2 and n % 2 == 1 and ((n + 1) // 2, q) in ADM:
        return True                                         # symmetric
    if lo == -n / 2 and n % 2 == 0 and (n // 2, q) in ADM:
        return True                                         # asymmetric low
    return False


def sets(pairs):
    u = {}
    for (n, q) in sorted(pairs):
        u.setdefault(frozenset(Q(k) * q for k in range(n)), f"NN(n={n},q={q})")
        u.setdefault(frozenset(Q(k) * q for k in range(-(n - 1), n)),
                     f"SY(n={2*n-1},q={q})")
        u.setdefault(frozenset(Q(k) * q for k in range(-n, n)),
                     f"AL(n={2*n},q={q})")
    return u


def run(u, label, show=3):
    els = list(u)
    join_dec = meet_dec = 0
    wj, wm = [], []
    for a, b in combinations(els, 2):
        un, inter = a | b, a & b

        if not is_valueset(un):
            ups = [c for c in els if un <= c]
            for c1, c2 in combinations(ups, 2):
                if c1 & c2 == un:
                    join_dec += 1
                    if len(wj) < show:
                        wj.append((u[a], u[b], u[c1], u[c2], sorted(un)))
                    break

        if not is_valueset(inter):
            lows = [c for c in els if c <= inter]
            for l1, l2 in combinations(lows, 2):
                if l1 | l2 == inter:
                    meet_dec += 1
                    if len(wm) < show:
                        wm.append((u[a], u[b], u[l1], u[l2], sorted(inter)))
                    break

    print(f"\n=== {label} ===")
    print(f"  elements {len(els)}   pairs {len(els)*(len(els)-1)//2}")
    print(f"  DECIDED join failures: {join_dec}")
    print(f"  DECIDED meet failures: {meet_dec}")
    for w in wj:
        print(f"    join: {w[0]} v {w[1]}  covers {w[2]} and {w[3]}"
              f"  meeting in the union {[str(x) for x in w[4]]}")
    for w in wm:
        print(f"    meet: {w[0]} ^ {w[1]}  lower {w[2]} and {w[3]}"
              f"  spanning the intersection {[str(x) for x in w[4]]}")
    return join_dec, meet_dec


if __name__ == "__main__":
    r2 = {(n, q) for (n, q) in ADM
          if n in {2 ** p for p in range(1, 8)}
          and q in {Q(2) ** e for e in range(-5, 6)}}
    run(sets(r2), "operands radix two, bias zero (148's unbiased slice)")

    small = {(n, q) for (n, q) in ADM if n <= 16 and Q(1, 32) <= q <= 32}
    run(sets(small), "operands any radix up to 16 points, bias zero")
