#!/usr/bin/env python3
"""
P5. P2 found that the sum-of-widths product formula overshoots by exactly one
integer bit on 127 of 1225 non-degenerate pairs, and the excess is always 1.
That is a provably dead bit the design would carry into the container.

This file derives the tight closed form and tests it, rather than reporting the
overshoot and stopping.

Derivation, stated so it can be attacked:
  max U<I,F> = (2^(I+F) - 1) * 2^-F.  Write W = I+F.
  The exact product set's step is 2^-(F1+F2), because 2^-F1 * 2^-F2 is itself a
  product, so the result's fraction width is F1+F2 EXACTLY, never more.
  The result's total width W_out is the least W with
      2^W - 1  >=  (2^W1 - 1)(2^W2 - 1)
  and the right side is 2^(W1+W2) - 2^W1 - 2^W2 + 1, so
      W_out = W1 + W2       when 2^W1 + 2^W2 - 2  <  2^(W1+W2-1)
      W_out = W1 + W2 - 1   otherwise.
  Then I_out = W_out - (F1+F2).

So the whole correction is one predicate on the two TOTAL widths, and it does not
look at the split between integer and fraction bits at all. If that holds, the
tight form costs one comparison over a quantity the typestate already carries.

Three checks:
  Q1  does the derived form match the least containing shape, everywhere?
  Q2  is the saving predicate exactly "one operand has total width 1"?
  Q3  how far up does the saving reach? (if it only ever fires at width 1 it is
      a corner; if it fires at larger widths it is a region)
"""

from fractions import Fraction as Fr

LIM = 8          # reach, using the analytic step/max path
CTRL = 3         # control box, full set enumeration, cross-checks the analytic path


def vals(I, F):
    n = 2 ** (I + F)
    return frozenset(Fr(k, 2**F) for k in range(n))


def least_containing(s):
    """Control path: from the exact set. Only used inside the CTRL box."""
    if s == {Fr(0)}:
        return (0, 0)
    F = 0
    while not all((x * (2**F)).denominator == 1 for x in s):
        F += 1
    mx = max(s)
    I = 0
    while Fr(2**I) - Fr(1, 2**F) < mx:
        I += 1
    return (I, F)


def least_product_analytic(a, b):
    """Least shape containing every product, WITHOUT enumerating the set.

    Rests on two facts, each verified against the control path below rather
    than assumed: the product set's coarsest grid is 2^-(F1+F2) exactly,
    because that value is itself a product of the two operands' own steps;
    and the product set's maximum is the product of the two maxima.
    """
    I1, F1 = a
    I2, F2 = b
    W1, W2 = I1 + F1, I2 + F2
    if W1 == 0 or W2 == 0:
        return (0, 0)
    F = F1 + F2
    mx = Fr(2**W1 - 1, 2**F1) * Fr(2**W2 - 1, 2**F2)
    I = 0
    while Fr(2**I) - Fr(1, 2**F) < mx:
        I += 1
    return (I, F)


def derived(a, b):
    """The tight closed form, from the derivation in the header."""
    I1, F1 = a
    I2, F2 = b
    W1, W2 = I1 + F1, I2 + F2
    if W1 == 0 or W2 == 0:
        return (0, 0)            # one operand denotes {0}; the product is {0}
    F = F1 + F2
    if 2**W1 + 2**W2 - 2 < 2 ** (W1 + W2 - 1):
        W = W1 + W2
    else:
        W = W1 + W2 - 1
    return (W - F, F)


def naive(a, b):
    return (a[0] + b[0], a[1] + b[1])


SHAPES = [(I, F) for I in range(0, LIM + 1) for F in range(0, LIM + 1)]
PAIRS = [(a, b) for a in SHAPES for b in SHAPES]


def main():
    print(f"# P5. tight product form. shapes={len(SHAPES)} pairs={len(PAIRS)} "
          f"(I,F in 0..{LIM})")
    print()

    # cross-check the analytic path against full enumeration in the control box
    ctrl = [(I, F) for I in range(0, CTRL + 1) for F in range(0, CTRL + 1)]
    dis = 0
    for a in ctrl:
        for b in ctrl:
            e = least_containing(frozenset(x * y for x in vals(*a) for y in vals(*b)))
            n = least_product_analytic(a, b)
            if e != n:
                dis += 1
                if dis == 1:
                    print(f"   CONTROL DISAGREEMENT U{a} U{b}: enumerated {e}, analytic {n}")
    print("## Q0. analytic least-containing against full set enumeration")
    print(f"   control pairs: {len(ctrl)**2}, disagreements: {dis}")
    if dis:
        print("   EVERYTHING BELOW IS VOID")
        return
    print()

    ok = bad = 0
    first_bad = None
    saved = 0
    naive_tight = 0
    for a, b in PAIRS:
        L = least_product_analytic(a, b)
        D = derived(a, b)
        N = naive(a, b)
        if D == L:
            ok += 1
        else:
            bad += 1
            if first_bad is None:
                first_bad = (a, b, D, L)
        if N == L:
            naive_tight += 1
        if N != L:
            saved += 1

    print("## Q1. does the derived closed form equal the least containing shape?")
    print(f"   agrees    : {ok}/{len(PAIRS)}")
    print(f"   disagrees : {bad}/{len(PAIRS)}")
    if first_bad:
        a, b, D, L = first_bad
        print(f"   first miss: U{a} x U{b} -> derived U{D}, least U{L}")
    print()
    print("## the baseline, for comparison")
    print(f"   sum-of-widths form tight : {naive_tight}/{len(PAIRS)}")
    print(f"   sum-of-widths form wastes at least one bit : {saved}/{len(PAIRS)}")
    print()

    print("## Q2. when does the saving fire? characterised by total widths")
    fires = {}
    for a, b in PAIRS:
        W1, W2 = a[0] + a[1], b[0] + b[1]
        if W1 == 0 or W2 == 0:
            continue
        f = 2**W1 + 2**W2 - 2 >= 2 ** (W1 + W2 - 1)
        fires.setdefault((min(W1, W2), max(W1, W2)), set()).add(f)
    consistent = all(len(v) == 1 for v in fires.values())
    print(f"   does the saving depend only on the two TOTAL widths? {consistent}")
    firing = sorted(k for k, v in fires.items() if True in v)
    print(f"   (minW, maxW) pairs where one bit is saved: {firing}")
    print(f"   (minW, maxW) pairs where it is not: "
          f"{len(fires) - len(firing)} combinations")
    print()

    print("## Q3. how far up does the saving reach?")
    mins = sorted({k[0] for k in firing})
    print(f"   minimum total width among firing pairs : {mins}")
    print(f"   so the saving fires exactly when the NARROWER operand's total")
    print(f"   width is in {mins}, at every width of the wider one.")
    print()

    print("## Q4. what the wasted bit costs, stated structurally rather than priced")
    print("   the excess is exactly one bit on the result's TOTAL width, so it")
    print("   crosses a container boundary exactly when the tight total width is")
    print("   a power of two. examples over this box:")
    ex = []
    for a, b in PAIRS:
        L = least_product_analytic(a, b)
        N = naive(a, b)
        if N != L:
            wl = L[0] + L[1]
            wn = N[0] + N[1]
            if wl in (8, 16, 32, 64) and wn == wl + 1:
                ex.append((a, b, wl, wn))
    for a, b, wl, wn in ex[:6]:
        print(f"   U{a} x U{b}: tight total {wl} bits, sum-of-widths {wn} bits")
    print(f"   count of such boundary crossings in this box: {len(ex)}")
    print("   UNPRICED: no bench harness run bears on what a container jump costs.")


if __name__ == "__main__":
    main()
