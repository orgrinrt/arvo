"""SUPERSEDED IN ITS COUNTS. Kept as the audit trail of an error worth seeing.

The sweep counts this file prints are VOID. They were produced against an
enumeration whose precision bound (max_p=3) silently excluded shapes the family
contains, so pairs were reported as meet failures when their intersection was
exactly a shape the generator had left out. The clearest instance:
`NN(64,1/8) ^ NN(64,1/4)` has intersection `NN(32,1/4)`, and the count 32 was
never generated.

What survives is `decisive()` at the bottom, whose two witnesses rest on an
argument that needs no complete enumeration, and which are re-derived cleanly in
decisive.py. Read that file for the numbers and this one only for the mistake.

Original docstring follows.
"""

"""Does the count-quantum coupling break the join with no bias at all?

This is the check that separates my own result from 148's. In a family where the
step is a power of the radix and the count is a power of the same radix, my
closure probe finds the join total. 148 reports 81 join failures in 351 unbiased
pairs, which is the same slice, so one of the two models is richer than the
other.

The difference is the value map. 148 records a five-point progression at quantum
one quarter realised as radix five, precision one, `FullRange<1>`, exponent minus
one (`148:351`). Under a map whose quantum is a power of the radix that is
impossible, since 5^e is never 1/4. So the family carries at least one map in
which the quantum is NOT a power of the radix, and the natural such map spreads
r^p points across a unit interval, giving quantum 1/(r^p - 1).

That decouples the count from the quantum's radix, and the pairs (count, quantum)
the family admits become an irregular set rather than a ladder. This probe builds
that set and asks whether the join survives it, with the bias held at zero
throughout, so that any failure found is a failure of the coupling alone.
"""

from fractions import Fraction as Q
from itertools import combinations


def admissible(max_r=8, max_p=3, exps=range(-3, 3)):
    """Pairs (count, quantum) the family admits, under two value maps."""
    out = set()
    for r in range(2, max_r + 1):
        for p in range(1, max_p + 1):
            n = r ** p
            if n > 64:
                continue
            for e in exps:
                # map one: the step is a power of the radix
                out.add((n, Q(r) ** e))
                # map two: r^p points spread across a unit interval, so the step
                # is a unit fraction of one less than the count
                if n > 1:
                    out.add((n, Q(r) ** e / (n - 1)))
    return sorted(out)


def valuesets(pairs):
    """Unbiased value sets: every window is anchored at the origin."""
    u = {}
    for (n, q) in pairs:
        # non-negative: indices 0 .. n-1
        u.setdefault(frozenset(Q(k) * q for k in range(n)),
                     f"NN(n={n},q={q})")
        # symmetric: indices -(n-1) .. n-1
        u.setdefault(frozenset(Q(k) * q for k in range(-(n - 1), n)),
                     f"SY(n={2*n-1},q={q})")
        # asymmetric low: indices -n .. n-1
        u.setdefault(frozenset(Q(k) * q for k in range(-n, n)),
                     f"AL(n={2*n},q={q})")
    return u


def report(u, label, show=4):
    els = list(u)
    meet_fail = join_fail = 0
    ex_j, ex_m = [], []
    for a, b in combinations(els, 2):
        inter, un = a & b, a | b
        lows = [c for c in els if c <= inter]
        tops = [c for c in lows if not any(c < d for d in lows)]
        if len(tops) != 1:
            meet_fail += 1
            if len(ex_m) < show and tops:
                ex_m.append((u[a], u[b], [u[t] for t in tops]))
        ups = [c for c in els if un <= c]
        bots = [c for c in ups if not any(d < c for d in ups)]
        if len(bots) != 1:
            join_fail += 1
            if len(ex_j) < show and len(bots) >= 2:
                ex_j.append((u[a], u[b], [u[t] for t in bots],
                             len(un), [len(t) for t in bots]))
    print(f"\n=== {label} ===")
    print(f"  elements {len(els)}  pairs {len(els)*(len(els)-1)//2}")
    print(f"  meet not unique: {meet_fail}   join not unique: {join_fail}")
    print("  (bias is zero everywhere, so every failure here is the coupling)")
    for e in ex_j:
        print(f"  JOIN: {e[0]}  v  {e[1]}")
        print(f"        union has {e[3]} points; minimal covers of sizes {e[4]}:")
        for name in e[2]:
            print(f"          {name}")
    for e in ex_m:
        print(f"  MEET: {e[0]}  ^  {e[1]}  -> maximal lower bounds {e[2]}")


if __name__ == "__main__":
    # map one only: the step is a power of the radix. This is the family my
    # closure probe covers, generalised to every radix rather than radix two.
    only_powers = [(n, q) for (n, q) in admissible()
                   if any(Q(r) ** e == q for r in range(2, 9) for e in range(-3, 3))]
    report(valuesets(only_powers), "step is a power of the radix, unbiased")

    # both maps: the count and the step are no longer tied to one radix
    report(valuesets(admissible()), "both value maps present, unbiased")


# ---------------------------------------------------------------- decisive

def decisive():
    """The two witnesses do not depend on the enumeration being large enough.

    The argument is the one 148 states at 148:346. If a least upper bound
    existed it would sit inside every upper bound, hence inside both of the
    incomparable minimal ones found, hence equal both by minimality, which is a
    contradiction. So it suffices to exhibit two incomparable covers and to show
    that the only set between them and the union is not an admissible value set.
    The dual argument settles the meet.
    """
    adm = set(admissible())
    print("\n=== the two witnesses, checked without reference to the sweep ===")

    # JOIN. {0,1/8} and {0,1/4}, bias zero throughout.
    A, B = {Q(0), Q(1, 8)}, {Q(0), Q(1, 4)}
    C1 = {Q(k, 8) for k in range(4)}          # count 4, quantum 1/8
    C2 = {Q(k, 24) for k in range(9)}         # count 9, quantum 1/24
    un = A | B
    print(f"  join witness: union {sorted(map(str, un))}")
    print(f"    cover one  {sorted(map(str,C1))}  admissible: {(4, Q(1,8)) in adm}")
    print(f"    cover two  count 9 quantum 1/24    admissible: {(9, Q(1,24)) in adm}")
    print(f"    covers the union: {un <= C1} and {un <= C2}")
    print(f"    incomparable: {not (C1 <= C2) and not (C2 <= C1)}")
    between = C1 & C2
    print(f"    the only candidate strictly between is {sorted(map(str, between))}")
    print(f"    which needs count 3 at quantum 1/8, admissible: "
          f"{(3, Q(1,8)) in adm}")

    # MEET. {0,2,4,6} and {0,1,2,3,4}, bias zero throughout.
    D = {Q(0), Q(2), Q(4), Q(6)}               # count 4, quantum 2, radix 2
    E = {Q(k) for k in range(5)}               # count 5, quantum 1, radix 5
    inter = D & E
    L1, L2 = {Q(0), Q(2)}, {Q(0), Q(4)}
    print(f"\n  meet witness: intersection {sorted(map(str, inter))}")
    print(f"    operands admissible: {(4, Q(2)) in adm} and {(5, Q(1)) in adm}")
    print(f"    lower bound one {sorted(map(str,L1))} admissible: {(2, Q(2)) in adm}")
    print(f"    lower bound two {sorted(map(str,L2))} admissible: {(2, Q(4)) in adm}")
    print(f"    incomparable: {not (L1 <= L2) and not (L2 <= L1)}")
    print(f"    the only candidate strictly above both and inside the "
          f"intersection is {sorted(map(str, inter))}")
    print(f"    which needs count 3 at quantum 2, admissible: "
          f"{(3, Q(2)) in adm}")
    print("\n  Both witnesses carry bias zero. Neither depends on how far the "
          "enumeration reached.")


