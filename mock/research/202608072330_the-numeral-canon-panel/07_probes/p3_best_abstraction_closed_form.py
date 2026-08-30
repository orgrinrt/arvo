#!/usr/bin/env python3
"""p3. The best abstraction in closed form, and what its codomain is.

p2 enumerated inside a box and its Q2 and Q4 are dominated by box truncation:
2,523,262 of 2,796,636 "no least containing numeral" answers there are sets whose
answer simply lies above the box ceiling.  This instrument removes the ceiling by
computing the least containing numeral ANALYTICALLY, and cross-validates the
closed form against p2's enumeration on the region where both are defined.

CLOSED FORM.  For a finite nonempty set S of nonnegative rationals, the least
U<I,F> at radix 2 whose value set contains S is

    F* = the least F with every s in S an integer multiple of 2^-F
    I* = the least I with 2^I  >=  max S + 2^-F*

and it is genuinely least because the two coordinates are minimised
independently and the order on the family is componentwise.  So within the
unsigned family the best abstraction is a TOTAL function into the (I, F) plane.

THE POINT OF THE PROBE.  If that is right, then a best abstraction never fails to
exist for a mathematical reason.  It fails only when the pair (I*, F*) it names is
not an ADMITTED shape.  So the record's two closure conditions are not lattice
hygiene: they are the statement that the admitted shape space contains the
closed form's codomain.  This probe measures exactly which inputs push the closed
form outside each candidate admission policy.

FOUR QUESTIONS.

  Q1  cross-validate the closed form against p2-style enumeration
  Q2  the codomain: which inputs force width 0, and which force I < 0
  Q3  the join identity: is the order's join of two numerals equal to the closed
      form applied to the UNION of their value sets
  Q4  multiplication: is the closed form applied to the exact PRODUCT set equal
      to the sum-of-widths form, and equal to the corrected form file 06 derived
"""

import itertools
from fractions import Fraction as Q

# ------------------------------------------------------------- the closed form


def vset(I, F):
    w = I + F
    assert w >= 0
    q = Q(1, 2**F) if F >= 0 else Q(2 ** (-F), 1)
    return frozenset(k * q for k in range(2**w))


def alpha(S):
    """Least (I, F) with S contained in V(U<I,F>).  S nonempty, nonneg rationals.
    Returns (I, F) with no admission policy applied: this is the codomain, raw."""
    assert S and all(s >= 0 for s in S)
    # F*: least F with every s a multiple of 2^-F.  s = p/q in lowest terms is a
    # multiple of 2^-F iff q divides 2^F, so q must be a power of two and
    # F >= log2(q).
    F = 0
    for s in S:
        d = Q(s).denominator
        assert d & (d - 1) == 0, f"{s} is not dyadic"
        F = max(F, d.bit_length() - 1)
    m = max(S)
    # I*: least I with 2^I >= m + 2^-F.  I may be negative; total width I+F >= 0
    # always, since 2^-F <= m + 2^-F forces 2^I >= 2^-F hence I >= -F.
    need = m + Q(1, 2**F)
    I = -F
    while Q(2) ** I < need:
        I += 1
    return (I, F)


# -------------------------------------------------- Q1: cross-validate closed form


def q1(wmax=6):
    print("=== Q1. Closed form against enumeration, on the region both cover ===")
    shapes = [(I, F) for F in range(wmax + 1) for I in range(-wmax, wmax + 1)
              if 0 <= I + F <= wmax]
    vs = {}
    for (I, F) in shapes:
        vs.setdefault(vset(I, F), []).append((I, F))
    points = sorted(vs.keys(), key=lambda s: (len(s), sorted(s)))
    universe = sorted(set().union(*points))

    tests = []
    for r in (1, 2, 3):
        for c in itertools.combinations(universe, r):
            tests.append(frozenset(c))
    for a, b in itertools.combinations(points, 2):
        tests.append(a | b)
    tests = list(dict.fromkeys(tests))

    agree = disagree = outside = 0
    wit = None
    for S in tests:
        I, F = alpha(S)
        closed = vset(I, F)
        if closed not in vs:
            outside += 1  # closed form's answer lies above the enumeration box
            continue
        ups = [v for v in points if S <= v]
        minimal = [u for u in ups if not any(o < u for o in ups)]
        if len(minimal) == 1 and minimal[0] == closed:
            agree += 1
        else:
            disagree += 1
            if wit is None:
                wit = (sorted(S), (I, F), [vs[m][0] for m in minimal])
    print(f"  sets tested        : {len(tests)}")
    print(f"  agree              : {agree}")
    print(f"  disagree           : {disagree}")
    print(f"  answer outside box : {outside}")
    if wit:
        print(f"  first disagreement : S={wit[0]} closed={wit[1]} enumerated={wit[2]}")
    print("  (a disagreement here is a defect in the closed form and would void Q2-Q4)")


# ------------------------------------------------- Q2: the codomain of alpha


def q2(wmax=6):
    print()
    print("=== Q2. Which inputs push the closed form outside each admission policy? ===")
    shapes = [(I, F) for F in range(wmax + 1) for I in range(0, wmax + 1)
              if 1 <= I + F <= wmax]
    points = sorted({vset(I, F) for (I, F) in shapes},
                    key=lambda s: (len(s), sorted(s)))
    universe = sorted(set().union(*points))
    tests = []
    for r in (1, 2, 3):
        for c in itertools.combinations(universe, r):
            tests.append(frozenset(c))
    for a, b in itertools.combinations(points, 2):
        tests.append(a | b)
    tests = list(dict.fromkeys(tests))

    width0 = negI = both = ok = 0
    w0 = wn = None
    for S in tests:
        I, F = alpha(S)
        z = (I + F == 0)
        n = (I < 0)
        if z and n:
            both += 1
        elif z:
            width0 += 1
            if w0 is None:
                w0 = sorted(S)
        elif n:
            negI += 1
            if wn is None:
                wn = (sorted(S), I, F)
        else:
            ok += 1
    print(f"  sets tested                    : {len(tests)}")
    print(f"  answer inside I>=0, width>=1   : {ok}")
    print(f"  answer needs width 0 (the origin): {width0}")
    print(f"  answer needs I < 0             : {negI}")
    print(f"  needs both at once             : {both}")
    if w0:
        print(f"  width-0 witness : S={w0}")
    if wn:
        print(f"  I<0 witness     : S={wn[0]} -> U<{wn[1]},{wn[2]}>")
    print("  NOTE the two admissions are disjoint conditions on the SAME formula,")
    print("       which is why the record found them doing two different jobs.")


# ----------------------------------------------- Q3: the join is alpha of the union


def q3(wmax=5):
    print()
    print("=== Q3. Is the order's JOIN equal to the closed form applied to the union? ===")
    shapes = [(I, F) for F in range(wmax + 1) for I in range(0, wmax + 1)
              if 1 <= I + F <= wmax]
    vs = {}
    for (I, F) in shapes:
        vs.setdefault(vset(I, F), []).append((I, F))
    points = sorted(vs.keys(), key=lambda s: (len(s), sorted(s)))
    agree = disagree = outside = 0
    wit = None
    for a, b in itertools.combinations(points, 2):
        I, F = alpha(a | b)
        closed = vset(I, F)
        ups = [v for v in points if a <= v and b <= v]
        minimal = [u for u in ups if not any(o < u for o in ups)]
        if len(minimal) != 1:
            outside += 1
            continue
        if minimal[0] == closed:
            agree += 1
        else:
            disagree += 1
            if wit is None:
                wit = (vs[a][0], vs[b][0], (I, F))
    print(f"  pairs            : {len(list(itertools.combinations(points,2)))}")
    print(f"  join = alpha(cup): {agree}")
    print(f"  differ           : {disagree}")
    print(f"  join outside box : {outside}")
    if wit:
        print(f"  witness          : {wit}")
    print("  (identical by definition; measured so the definition is not merely asserted)")


# --------------------------------------------- Q4: multiplication, three forms


def tight_06(W1, W2):
    """The corrected total width file 06 derives, restated here to be checked
    against the closed form rather than inherited from it."""
    if 2**W1 + 2**W2 - 2 >= 2 ** (W1 + W2 - 1):
        return W1 + W2 - 1
    return W1 + W2


def q4(wmax=5):
    print()
    print("=== Q4. Multiplication: closed form vs sum-of-widths vs file 06's corrected form ===")
    shapes = [(I, F) for F in range(wmax + 1) for I in range(0, wmax + 1)
              if 1 <= I + F <= wmax]
    n = 0
    naive_eq = naive_gt = 0
    tight_eq = tight_ne = 0
    negI = width0 = 0
    wit_gt = wit_ne = wit_neg = None
    for (I1, F1) in shapes:
        for (I2, F2) in shapes:
            v1, v2 = vset(I1, F1), vset(I2, F2)
            prod = frozenset(x * y for x in v1 for y in v2)
            I, F = alpha(prod)
            n += 1
            if I < 0:
                negI += 1
                if wit_neg is None:
                    wit_neg = (f"U<{I1},{F1}>", f"U<{I2},{F2}>", (I, F))
            if I + F == 0:
                width0 += 1
            # naive
            if (I, F) == (I1 + I2, F1 + F2):
                naive_eq += 1
            else:
                naive_gt += 1
                if wit_gt is None:
                    wit_gt = (f"U<{I1},{F1}>", f"U<{I2},{F2}>",
                              f"naive U<{I1+I2},{F1+F2}>", f"best U<{I},{F}>")
            # file 06's corrected form: F = F1+F2, total width by the predicate
            W = tight_06(I1 + F1, I2 + F2)
            t = (W - (F1 + F2), F1 + F2)
            if t == (I, F):
                tight_eq += 1
            else:
                tight_ne += 1
                if wit_ne is None:
                    wit_ne = (f"U<{I1},{F1}>", f"U<{I2},{F2}>",
                              f"06 U<{t[0]},{t[1]}>", f"best U<{I},{F}>")
    print(f"  operand pairs                       : {n}")
    print(f"  sum-of-widths form IS the best      : {naive_eq}")
    print(f"  sum-of-widths form OVERSHOOTS       : {naive_gt}")
    print(f"  file 06's corrected form IS the best: {tight_eq}")
    print(f"  file 06's corrected form differs    : {tight_ne}")
    print(f"  best answer needs I < 0             : {negI}")
    print(f"  best answer has width 0             : {width0}")
    if wit_gt:
        print(f"  overshoot witness : {wit_gt}")
    if wit_ne:
        print(f"  06-differs witness: {wit_ne}")
    if wit_neg:
        print(f"  I<0 witness       : {wit_neg}")


# The default Q1 box (wmax=6) was KILLED after 25 minutes without finishing: its
# 3-subset enumeration over a universe of a few hundred rationals is quadratic in
# the shape count and cubic in the universe.  It is left in place unchanged as the
# slow method that licenses the fast one, exactly as file 06 kept its own two
# killed instruments.  The box actually run is passed on the command line.

if __name__ == "__main__":
    import sys
    w = int(sys.argv[1]) if len(sys.argv) > 1 else 6
    print(f"# Q1 box wmax={w} (default 6 was killed at 25 minutes without finishing)")
    q1(w)
    q2()
    q3()
    q4()
