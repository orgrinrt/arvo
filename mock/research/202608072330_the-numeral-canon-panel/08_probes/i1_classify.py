#!/usr/bin/env python3
"""i1: mechanically classify a representation's value set.

One test carves the whole survey, and it is not a taste judgement:

    Fix a radix r. For each r-binade [r^e, r^(e+1)), is the set of denotable
    magnitudes falling in it an arithmetic progression whose step is an integer
    power of r, with every value a multiple of that step?

If yes, the representation has a canonical exponent FUNCTION fexp(e), with the
step in binade e equal to r^fexp(e), and the representation is exactly the set
of multiples of r^fexp(e) in each binade, intersected with a range. That is the
one-format concept in its general form.

The shape of fexp then names the family:
    fexp constant                -> fixed point
    fexp(e) = e - c              -> float, unbounded exponent (Flocq's FLX)
    fexp(e) = max(e - c, m)      -> float with gradual underflow (Flocq's FLT)
    anything else                -> tapered / segmented; no name in the design

If no, the representation is outside the concept, and WHICH clause fails says
which layer it belongs to.

No claim here is about how anything is written. Every number is a count.
"""

from fractions import Fraction
from math import gcd
import sys


# ---------------------------------------------------------------- primitives

def is_r_adic(v: Fraction, r: int) -> bool:
    """Is v = m * r^q for integers m, q? Equivalently: v's denominator in
    lowest terms divides some power of r."""
    d = v.denominator
    while d > 1:
        g = gcd(d, r)
        if g == 1:
            return False
        while d % g == 0:
            d //= g
    return True


def binade(v: Fraction, r: int) -> int:
    """The unique e with r^e <= |v| < r^(e+1). v must be nonzero."""
    a = abs(v)
    e = 0
    while a >= r:
        a /= r
        e += 1
    while a < 1:
        a *= r
        e -= 1
    return e


def is_power_of(x: Fraction, r: int):
    """If x = r^k for an integer k, return k, else None."""
    if x <= 0:
        return None
    k = 0
    while x >= r:
        x /= r
        k += 1
    while x < 1:
        x *= r
        k -= 1
    return k if x == 1 else None


# ---------------------------------------------------------------- the test

def classify(name, values, r, note=""):
    """values: an iterable of Fractions (the denotable values, 0 permitted).

    Returns a dict of facts. Nothing is rounded and nothing is sampled: the
    whole set is walked."""
    vs = sorted(set(values))
    nonzero = [abs(v) for v in vs if v != 0]
    mags = sorted(set(nonzero))

    out = {
        "name": name,
        "radix": r,
        "count": len(vs),
        "note": note,
    }

    # clause 1: every value is m * r^q
    bad = [v for v in mags if not is_r_adic(v, r)]
    out["r_adic"] = (len(bad) == 0)
    out["n_not_r_adic"] = len(bad)
    out["first_not_r_adic"] = str(bad[0]) if bad else None
    if bad:
        out["fexp"] = None
        out["uniform_per_binade"] = False
        out["family"] = "OUTSIDE: values are not m * r^q"
        return out

    # clause 2: uniform grid per binade, step an integer power of r
    by_binade = {}
    for v in mags:
        by_binade.setdefault(binade(v, r), []).append(v)

    fexp = {}
    failures = []
    for e in sorted(by_binade):
        xs = sorted(by_binade[e])
        if len(xs) == 1:
            # a single value in a binade pins no step; take its own valuation,
            # which is the coarsest grid carrying it.
            k = 0
            x = xs[0]
            while x.denominator != 1 or x % r == 0:
                if x.denominator != 1:
                    x *= r
                    k -= 1
                else:
                    x /= r
                    k += 1
            fexp[e] = k
            continue
        diffs = {xs[i + 1] - xs[i] for i in range(len(xs) - 1)}
        if len(diffs) != 1:
            failures.append((e, "non-uniform", len(diffs)))
            continue
        step = diffs.pop()
        k = is_power_of(step, r)
        if k is None:
            failures.append((e, "step not a power of the radix", str(step)))
            continue
        if any((x / step).denominator != 1 for x in xs):
            failures.append((e, "values not multiples of the step", None))
            continue
        fexp[e] = k

    out["uniform_per_binade"] = (len(failures) == 0)
    out["n_binades"] = len(by_binade)
    out["failures"] = failures[:4]
    out["n_failed_binades"] = len(failures)
    if failures:
        out["fexp"] = None
        out["family"] = "OUTSIDE: not a union of uniform grids by binade"
        return out

    out["fexp"] = dict(sorted(fexp.items()))
    out["family"] = name_family(out["fexp"])
    out["n_distinct_fexp"] = len(set(fexp.values()))
    return out


def name_family(fexp: dict) -> str:
    """Classify the shape of the canonical exponent function."""
    es = sorted(fexp)
    if len(es) == 0:
        return "degenerate: no nonzero values"
    vals = [fexp[e] for e in es]
    if len(set(vals)) == 1:
        return "FIXED (fexp constant)"
    slopes = {fexp[es[i + 1]] - fexp[es[i]] for i in range(len(es) - 1)}
    gaps = {es[i + 1] - es[i] for i in range(len(es) - 1)}
    if slopes == {1} and gaps == {1}:
        c = es[0] - fexp[es[0]]
        return f"FLOAT, unbounded exponent (fexp(e) = e - {c})"
    # clamped affine: constant below a knee, slope one above
    knee = None
    ok = True
    for i in range(len(es) - 1):
        d = fexp[es[i + 1]] - fexp[es[i]]
        g = es[i + 1] - es[i]
        if d == 0 and knee is None:
            continue
        if d == g:
            if knee is None:
                knee = es[i]
            continue
        ok = False
        break
    if ok and knee is not None:
        return f"FLOAT with gradual underflow (fexp constant below e = {knee}, slope 1 above)"
    return "TAPERED / SEGMENTED (fexp of no named shape)"


# ---------------------------------------------------------- representations

def fixed(I, F, r=2):
    q = Fraction(1, r ** F)
    return [k * q for k in range(r ** (I + F))]


def hub(I, F, r=2):
    """Half-unit-biased: values sit at cell midpoints, phase r^-F / 2."""
    q = Fraction(1, r ** F)
    return [k * q + q / 2 for k in range(r ** (I + F))]


def flt(p, emin, emax, r=2, subnormals=True):
    """Radix-r float, precision p (significand digits), exponent range
    [emin, emax]. With subnormals, the smallest binade is filled down to
    the fixed grid r^(emin-p+1)."""
    out = [Fraction(0)]
    for e in range(emin, emax + 1):
        step = Fraction(1, r ** (p - 1 - e)) if p - 1 - e > 0 else Fraction(r ** (e - p + 1))
        m0 = r ** (p - 1)
        for m in range(m0, r ** p):
            out.append(Fraction(m) * step)
    if subnormals:
        step = Fraction(1, r ** (p - 1 - emin)) if p - 1 - emin > 0 else Fraction(r ** (emin - p + 1))
        for m in range(1, r ** (p - 1)):
            out.append(Fraction(m) * step)
    return out


def posit(n, es):
    """The full positive posit value set at n bits, es exponent bits.
    Decoded from bit patterns, not from a formula, so the decode is the
    instrument rather than the claim."""
    out = [Fraction(0)]
    for bits in range(1, 1 << (n - 1)):
        v = posit_value(bits, n, es)
        if v is not None:
            out.append(v)
    return out


def posit_value(bits, n, es):
    mask = (1 << n) - 1
    if bits == 0:
        return Fraction(0)
    if bits == (1 << (n - 1)):
        return None  # NaR
    neg = (bits >> (n - 1)) & 1
    if neg:
        bits = (-bits) & mask
    body = [(bits >> i) & 1 for i in range(n - 2, -1, -1)]
    r0 = body[0]
    i = 0
    while i < len(body) and body[i] == r0:
        i += 1
    k = (i - 1) if r0 == 1 else -i
    j = i + 1 if i < len(body) else i
    ebits = body[j:j + es]
    fbits = body[j + es:]
    e = 0
    for b in ebits:
        e = e * 2 + b
    e <<= (es - len(ebits))
    frac = Fraction(0)
    for idx, b in enumerate(fbits):
        frac += Fraction(b, 2 ** (idx + 1))
    val = Fraction(2) ** ((1 << es) * k + e) * (1 + frac)
    return -val if neg else val


def decimal(p, emin, emax):
    out = [Fraction(0)]
    for e in range(emin, emax + 1):
        for m in range(10 ** (p - 1), 10 ** p):
            out.append(Fraction(m) * Fraction(10) ** e)
    return out


def fixed_slash(P, Q):
    """Rational format: every p/q with p <= P, q <= Q."""
    out = set()
    for q in range(1, Q + 1):
        for p in range(0, P + 1):
            out.add(Fraction(p, q))
    return sorted(out)


def integer_interval(lo, hi):
    return [Fraction(k) for k in range(lo, hi + 1)]


def negabinary(nbits):
    out = []
    for bits in range(1 << nbits):
        v = 0
        for i in range(nbits):
            if (bits >> i) & 1:
                v += (-2) ** i
        out.append(Fraction(v))
    return out


def mixed_radix(bases):
    n = 1
    for b in bases:
        n *= b
    return integer_interval(0, n - 1)


def stochastic(N):
    return [Fraction(k, N) for k in range(N + 1)]


def double_double(p, emin, emax):
    """Unevaluated sums a + b of two floats with |b| <= ulp(a)/2. Small
    parameters only; the point is the shape, not the size."""
    base = [v for v in flt(p, emin, emax, subnormals=False) if v > 0]
    out = {Fraction(0)}
    for a in base:
        e = binade(a, 2)
        ulp = Fraction(2) ** (e - p + 1)
        for b in base + [-x for x in base] + [Fraction(0)]:
            if abs(b) <= ulp / 2:
                out.add(a + b)
    return sorted(out)


def lns_rational_fraction(F, kmin, kmax):
    """Logarithmic number system: value = 2^(k / 2^F). Report how many of the
    denotable values are rational at all, which is decidable without
    constructing them: 2^(k/2^F) is rational exactly when 2^F divides k."""
    total = kmax - kmin + 1
    rat = sum(1 for k in range(kmin, kmax + 1) if k % (2 ** F) == 0)
    return total, rat


# ---------------------------------------------------------------- reporting

def show(res):
    print(f"--- {res['name']}  (radix {res['radix']}, {res['count']} values)")
    if res.get("note"):
        print(f"    note: {res['note']}")
    print(f"    every value m*r^q      : {res['r_adic']}"
          + ("" if res["r_adic"] else f"   ({res['n_not_r_adic']} not, first {res['first_not_r_adic']})"))
    if res["r_adic"]:
        print(f"    uniform grid per binade: {res['uniform_per_binade']}"
              + ("" if res["uniform_per_binade"]
                 else f"   ({res['n_failed_binades']} of {res['n_binades']} binades fail; {res['failures']})"))
    print(f"    family                 : {res['family']}")
    if res.get("fexp") is not None:
        f = res["fexp"]
        keys = sorted(f)
        shown = ", ".join(f"{e}:{f[e]}" for e in keys[:10])
        more = "" if len(keys) <= 10 else f", ... ({len(keys)} binades)"
        print(f"    canonical exponent     : {{{shown}{more}}}  distinct={res['n_distinct_fexp']}")
    print()


def main():
    cases = []

    cases.append(classify("fixed U<3,3>", fixed(3, 3), 2))
    cases.append(classify("fixed U<0,4>", fixed(0, 4), 2))
    cases.append(classify("HUB fixed I=2 F=2", hub(2, 2), 2,
                          "half-unit-biased; phase is half a step"))
    cases.append(classify("float p=3 e=-2..3, no subnormals",
                          flt(3, -2, 3, subnormals=False), 2))
    cases.append(classify("float p=3 e=-2..3, with subnormals",
                          flt(3, -2, 3, subnormals=True), 2))
    cases.append(classify("fp8 E4M3-shaped (p=4, e=-6..8, subnormals)",
                          flt(4, -6, 8, subnormals=True), 2))
    cases.append(classify("posit<8,0>", posit(8, 0), 2))
    cases.append(classify("posit<8,1>", posit(8, 1), 2))
    cases.append(classify("posit<10,2>", posit(10, 2), 2))
    cases.append(classify("decimal p=2 e=-2..2, at radix 10",
                          decimal(2, -2, 2), 10))
    cases.append(classify("decimal p=2 e=-2..2, at radix 2",
                          decimal(2, -2, 2), 2,
                          "same set, asked at the binary radix"))
    cases.append(classify("fixed-slash P=7 Q=7", fixed_slash(7, 7), 2))
    cases.append(classify("residue number system, moduli (3,5,7)",
                          integer_interval(0, 3 * 5 * 7 - 1), 2,
                          "value set only; the encoding is the whole point and is not a value fact"))
    cases.append(classify("thermometer / unary, 16 levels",
                          integer_interval(0, 16), 2,
                          "value set only"))
    cases.append(classify("negabinary, 6 digits", negabinary(6), 2))
    cases.append(classify("mixed radix (factorial base 5!)",
                          mixed_radix([2, 3, 4, 5]), 2))
    cases.append(classify("stochastic stream, N=16", stochastic(16), 2,
                          "value set only; a datum is a bit stream and denotes a distribution"))
    cases.append(classify("double-double p=3 e=-1..2",
                          double_double(3, -1, 2), 2))

    for c in cases:
        show(c)

    tot, rat = lns_rational_fraction(3, 1, 64)
    print(f"--- logarithmic number system, base 2, F=3 fraction bits")
    print(f"    denotable magnitudes 2^(k/8), k = 1..64 : {tot}")
    print(f"    of those, rational at all               : {rat}")
    print(f"    so values of the form m*2^q             : {rat} of {tot}")
    print()

    # The headline count, printed rather than asserted in prose.
    inside = [c for c in cases if c["r_adic"] and c["uniform_per_binade"]]
    outside = [c for c in cases if not (c["r_adic"] and c["uniform_per_binade"])]
    print(f"SUMMARY over {len(cases)} value sets, at the radix each is asked at:")
    print(f"  inside  the one-format concept: {len(inside)}")
    print(f"  outside the one-format concept: {len(outside)}")
    fams = {}
    for c in inside:
        key = c["family"].split(" (")[0].split(",")[0]
        fams[key] = fams.get(key, 0) + 1
    for k, v in sorted(fams.items()):
        print(f"    {k}: {v}")
    print("  outside, by clause:")
    for c in outside:
        print(f"    {c['name']}: {c['family']}")


if __name__ == "__main__":
    main()
