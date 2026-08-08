#!/usr/bin/env python3
"""
s2: the map from vocabulary A to vocabulary B is not total. Name what it misses.

s1 established the two vocabularies agree on the objects they both cover. This one asks
the other direction: given an A-numeral, is there a (W, F) presenting it?

The test is decidable by construction. A (W, F)-presentable value set is exactly
    { k * 2^{-F} : k in [0, 2^W) }
so: take the set, require 0 in it, require the sorted differences to be a single value s,
require s = 2^{-F} for some integer F, require |V| = 2^W for some natural W, and require
W, F both natural.

The representations below are built from 08's own classification table (08:303-331), one
per row that classifies INSIDE the format concept. Every one of them is a numeral by 08's
membership predicate. The question is only whether (W, F) can name it.

Run:  python3 s2_which_numerals_the_wf_keying_misses.py
"""

from fractions import Fraction as Q


def is_pow(n, base):
    """n == base^k for a natural k."""
    if n < 1:
        return None
    k = 0
    v = 1
    while v < n:
        v *= base
        k += 1
    return k if v == n else None


def wf_presentable(values):
    """Is this value set (W, F)-presentable at radix two, adjustment one, bias zero,
    phase zero? Returns (True, (W, F)) or (False, reason)."""
    vs = sorted(values)
    if not vs:
        return False, "empty"
    if vs[0] != 0:
        return False, f"does not start at zero (min is {vs[0]})"
    if len(vs) == 1:
        return True, (0, 0)
    steps = {vs[i + 1] - vs[i] for i in range(len(vs) - 1)}
    if len(steps) != 1:
        return False, f"{len(steps)} distinct steps, not one progression"
    (s,) = steps
    # s must be 2^{-F}, F a natural: numerator 1, denominator a power of two.
    if s.numerator != 1:
        return False, f"step {s} is not a negative power of two (numerator {s.numerator})"
    F = is_pow(s.denominator, 2)
    if F is None:
        return False, f"step {s} denominator is not a power of two"
    W = is_pow(len(vs), 2)
    if W is None:
        return False, f"cardinality {len(vs)} is not a power of two"
    return True, (W, F)


# --- the A-numerals, built from 08's table ------------------------------------------

def fixed_point(W, F):
    """constant canonical exponent, radix 2. 08:305."""
    return {Q(k, 2**F) for k in range(2**W)}


def hub(W, F):
    """half-unit-biased: same grid, phase half a step. 08:306, 'the only case the Bias
    axis earns'."""
    step = Q(1, 2**F)
    return {step * (k + Q(1, 2)) for k in range(2**W)}


def ranged(count, F):
    """a grid whose value count is not a power of the radix. 08:307, 'expressed by
    Ranged, not by the anchored family'."""
    return {Q(k, 2**F) for k in range(count)}


def binary_float(p, emin, emax):
    """canonical exponent slope one: f(e) = e - p + 1. 08:308. Non-negative half."""
    out = {Q(0)}
    for e in range(emin, emax + 1):
        step = Q(2) ** (e - p + 1)
        for k in range(2 ** (p - 1)):
            out.add(Q(2) ** e + step * k)
    return out


def float_with_subnormals(p, emin, emax):
    """knee then slope one: the gradual-underflow family. 08:309, and 08:424-431 measures
    it to be the meet of a fixed shape and a float."""
    out = binary_float(p, emin, emax)
    step = Q(2) ** (emin - p + 1)
    for k in range(2 ** (p - 1)):  # the subnormal run below 2^emin, constant step
        out.add(step * k)
    return out


def posit_like(useed_slope):
    """canonical exponent of slope two or more somewhere. 08:311, 'slope two, no named
    shape'. A stand-in with the right f-shape, not a real posit encoding."""
    out = {Q(0)}
    for e in range(-6, 7):
        fe = useed_slope * e  # slope two rather than one
        step = Q(2) ** fe
        v = Q(2) ** e
        while v < Q(2) ** (e + 1):
            out.add(v)
            v += step
    return out


def decimal(digits, F10):
    """radix ten. 08:310. The same set is inside at radix ten and outside at radix two
    (08:230-233), which is why the radix is a parameter of the question."""
    return {Q(k, 10**F10) for k in range(10**digits)}


def nonzero_bias(W, F, bias):
    """bias non-zero: the affine map's B term, seed/SETTLED_laws.md:274."""
    return {Q(k, 2**F) + bias for k in range(2**W)}


def adjustment_not_one(W, F, adj):
    """adjustment not one: the affine map's A term, which changes the spacing."""
    return {adj * Q(k, 2**F) for k in range(2**W)}


CASES = [
    ("fixed point W=4 F=2",              fixed_point(4, 2),                  "constant f"),
    ("fixed point W=8 F=8",              fixed_point(8, 8),                  "constant f"),
    ("zero-only numeral W=0",            fixed_point(0, 0),                  "constant f"),
    ("negative derived I: W=1 F=4",      fixed_point(1, 4),                  "constant f"),
    ("HUB W=4 F=2 (phase half a step)",  hub(4, 2),                          "constant f, phase 1/2"),
    ("Ranged, 12 values at F=2",         ranged(12, 2),                      "constant f, count not 2^k"),
    ("Ranged, 100 values at F=0",        ranged(100, 0),                     "constant f, count not 2^k"),
    ("float p=4 emin=-3 emax=3",         binary_float(4, -3, 3),             "f slope one"),
    ("float p=3 emin=-2 emax=2",         binary_float(3, -2, 2),             "f slope one"),
    ("float+subnormals p=4 e=-3..3",     float_with_subnormals(4, -3, 3),    "f knee then slope one"),
    ("posit-shaped, f slope two",        posit_like(2),                      "f slope two"),
    ("decimal 2 digits, F10=1",          decimal(2, 1),                      "radix ten"),
    ("bias = 1/2, W=4 F=2",              nonzero_bias(4, 2, Q(1, 2)),        "bias non-zero"),
    ("adjustment = 3, W=4 F=2",          adjustment_not_one(4, 2, Q(3)),     "adjustment not one"),
]


def main():
    print("s2: which A-numerals admit a (W, F) presentation")
    print()
    print(f"{'representation':<36} {'A-shape':<26} {'(W,F)?':<8} detail")
    print("-" * 108)
    hits = misses = 0
    miss_reasons = {}
    for name, values, shape in CASES:
        ok, detail = wf_presentable(values)
        if ok:
            hits += 1
            print(f"{name:<36} {shape:<26} {'YES':<8} W={detail[0]} F={detail[1]}  |V|={len(values)}")
        else:
            misses += 1
            miss_reasons[name] = detail
            print(f"{name:<36} {shape:<26} {'NO':<8} {detail}")
    print("-" * 108)
    print(f"presentable: {hits}/{len(CASES)}    not presentable: {misses}/{len(CASES)}")
    print()
    print("Every case above is INSIDE 08's format concept (08:303-331). So the misses are")
    print("numerals the concept admits and the (W, F) keying cannot name.")
    print()
    print("grouped by which A coordinate the (W, F) pair does not carry:")
    print("  canonical exponent not constant : float, float+subnormals, posit-shaped")
    print("  phase not zero                  : HUB")
    print("  radix not two                   : decimal")
    print("  bias not zero                   : bias = 1/2")
    print("  adjustment not one              : adjustment = 3")
    print("  reach not a power of the radix   : Ranged (both rows)")


if __name__ == "__main__":
    main()
