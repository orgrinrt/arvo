#!/usr/bin/env python3
"""i3: the two things a value-set test cannot see.

i1b classifies value sets. Five of the representations it calls INSIDE differ
from a plain fixed-point numeral in no value-set respect at all: a residue
number system, a thermometer code, a carry-save form, negabinary and a mixed
radix all denote an interval of integers. What separates them is the map from
datum to value, which is invisible to every test in i1b.

Two properties of that map decide whether the design's machinery still works:

  Q1  Is it injective? Where it is not, the crossing contract needs a retraction
      rather than a bijection, which the droplist records the design already
      adopting for a different reason ("Two round-trip theorems as the crossing
      contract ... the second is false the moment signed zero, NaN payloads, or
      decimal cohorts exist. Replaced by the section-retraction triple.").
  Q2  Does the datum order agree with the value order? The record carries a
      datum-level total order distinct from the value-level one
      (`SETTLED.md:116`). Where the two disagree, comparison cannot be a bit
      comparison.

And one property of the DENOTATION, which is a third axis again:

  Q3  Does a datum denote one value, or a set? `07` section 2.3 finds that a
      rounded datum already denotes the set of exact values that produced it.
      This asks whether that is enough to serve an interval consumer without
      the numeral changing, which decides whether excluding interval arithmetic
      costs anything.
"""

from fractions import Fraction
import itertools
import random


# ------------------------------------------------------------------ Q1 and Q2

def enc_unsigned(n):
    return {k: Fraction(k) for k in range(1 << n)}


def enc_twos_complement(n):
    out = {}
    for k in range(1 << n):
        v = k - (1 << n) if k >= (1 << (n - 1)) else k
        out[k] = Fraction(v)
    return out


def enc_ones_complement(n):
    out = {}
    for k in range(1 << n):
        v = -((~k) & ((1 << n) - 1)) if k >= (1 << (n - 1)) else k
        out[k] = Fraction(v)
    return out


def enc_sign_magnitude(n):
    out = {}
    for k in range(1 << n):
        mag = k & ((1 << (n - 1)) - 1)
        out[k] = Fraction(-mag if k >> (n - 1) else mag)
    return out


def enc_gray(n):
    return {k: Fraction(k ^ (k >> 1)) for k in range(1 << n)}


def enc_negabinary(n):
    out = {}
    for k in range(1 << n):
        v = 0
        for i in range(n):
            if (k >> i) & 1:
                v += (-2) ** i
        out[k] = Fraction(v)
    return out


def enc_thermometer(n):
    """n bits, value = popcount, but only the runs of leading ones are legal.
    Every bit pattern is decoded, so the illegal ones show as redundancy."""
    return {k: Fraction(bin(k).count("1")) for k in range(1 << n)}


def enc_carry_save(n):
    """A carry-save datum is a pair (sum, carry) denoting sum + 2*carry."""
    out = {}
    half = n // 2
    for s in range(1 << half):
        for c in range(1 << half):
            out[(s << half) | c] = Fraction(s + 2 * c)
    return out


def enc_rns(moduli):
    """A residue datum is the tuple of residues, packed. Injective on the range
    below the product, by the remainder theorem."""
    M = 1
    for m in moduli:
        M *= m
    out = {}
    for x in range(M):
        key = tuple(x % m for m in moduli)
        out[key] = Fraction(x)
    return out


def enc_stochastic(n):
    """A datum is an n-bit stream; the value is the count of ones over n."""
    return {k: Fraction(bin(k).count("1"), n) for k in range(1 << n)}


def enc_decimal_cohorts(p, emin, emax):
    """A decimal datum is (significand, exponent) and cohorts are the whole
    point: 1.0 and 1.00 are distinct data denoting one value."""
    out = {}
    for e in range(emin, emax + 1):
        for m in range(0, 10 ** p):
            out[(m, e)] = Fraction(m) * Fraction(10) ** e
    return out


def report(name, enc, ordered_by_key=True):
    values = {}
    for k, v in enc.items():
        values.setdefault(v, []).append(k)
    mult = sorted((len(ks) for ks in values.values()))
    injective = mult[-1] == 1
    # datum order against value order, only where the key is an integer
    keys = [k for k in enc if isinstance(k, int)]
    order_ok = None
    if ordered_by_key and len(keys) == len(enc):
        seq = [enc[k] for k in sorted(keys)]
        order_ok = all(seq[i] <= seq[i + 1] for i in range(len(seq) - 1))
    print(f"  {name:28s} data={len(enc):>6} values={len(values):>6} "
          f"injective={str(injective):>5} "
          f"max data per value={mult[-1]:>5} "
          f"bit order = value order: {order_ok}")
    return injective, order_ok


def q1_q2():
    print("Q1/Q2: injectivity of the datum map, and whether the bit order is "
          "the value order")
    report("unsigned binary, 6 bits", enc_unsigned(6))
    report("two's complement, 6 bits", enc_twos_complement(6))
    report("one's complement, 6 bits", enc_ones_complement(6))
    report("sign magnitude, 6 bits", enc_sign_magnitude(6))
    report("Gray code, 6 bits", enc_gray(6))
    report("negabinary, 6 bits", enc_negabinary(6))
    report("thermometer, 6 bits", enc_thermometer(6))
    report("carry-save, 6 bits", enc_carry_save(6))
    report("residue (3,5,7)", enc_rns((3, 5, 7)), ordered_by_key=False)
    report("stochastic stream, 6 bits", enc_stochastic(6))
    report("decimal with cohorts p=2", enc_decimal_cohorts(2, -1, 1),
           ordered_by_key=False)
    print()


# ---------------------------------------------------------------------- Q3

def q3_intervals(I=3, F=3, trials=20000, seed=11):
    """Can an interval consumer be served by the numeral plus directed
    rounding, without the numeral changing?

    An interval is a pair of numeral values. Outward rounding takes the low end
    down and the high end up. The test is whether the outward-rounded result
    contains every exact result."""
    rng = random.Random(seed)
    step = Fraction(1, 2 ** F)
    grid = [k * step for k in range(2 ** (I + F))]
    lo_g, hi_g = grid[0], grid[-1]

    def down(x):
        if x <= lo_g:
            return lo_g
        return min(hi_g, (x / step).__floor__() * step)

    def up(x):
        if x >= hi_g:
            return hi_g
        n = x / step
        k = n.__floor__()
        if n != k:
            k += 1
        return max(lo_g, k * step)

    def nearest(x):
        c = min(grid, key=lambda g: (abs(g - x), g))
        return c

    fails_outward = 0
    fails_one_sided = 0
    fails_nearest = 0
    clamped = 0
    for _ in range(trials):
        a, b = sorted(rng.sample(grid, 2))
        c, d = sorted(rng.sample(grid, 2))
        for op in ("add", "mul"):
            if op == "add":
                exact_lo, exact_hi = a + c, b + d
            else:
                exact_lo, exact_hi = a * c, b * d
            if exact_hi > hi_g or exact_lo < lo_g:
                clamped += 1
                continue
            if not (down(exact_lo) <= exact_lo and up(exact_hi) >= exact_hi):
                fails_outward += 1
            if not (up(exact_lo) <= exact_lo):
                fails_one_sided += 1
            if not (nearest(exact_lo) <= exact_lo and nearest(exact_hi) >= exact_hi):
                fails_nearest += 1

    print(f"Q3: interval soundness over U<{I},{F}>, {trials} random interval "
          f"pairs, add and multiply")
    print(f"    results outside the numeral's range, skipped : {clamped}")
    print(f"    outward rounding (down the low, up the high) : {fails_outward} failures")
    print(f"    one directed mode used for both ends         : {fails_one_sided} failures")
    print(f"    round to nearest used for both ends          : {fails_nearest} failures")
    print()
    print("    So an interval consumer needs both directed modes and nothing")
    print("    else from the numeral. The numeral does not change; what the")
    print("    design owes is that both directions are reachable per operation.")
    print()


if __name__ == "__main__":
    q1_q2()
    q3_intervals()
