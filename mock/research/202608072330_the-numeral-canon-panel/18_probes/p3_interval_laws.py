"""p3. Which laws survive if a datum is allowed to denote an interval.

`08` section 4.5 measures that an interval consumer needs both directed rounding modes reachable
per operation and nothing else from the numeral, and concludes the exclusion costs nothing. That
measures the CONSTRUCTION. This probe measures the ALGEBRA, which is the other half and is what
the design's law layer would have to carry if the interval were admitted as a numeral rather than
built above one.

Four laws are checked, each in the equality form the record states for numerals and, where
equality fails, in the containment form interval arithmetic is known to satisfy instead:

  commutativity of +      A + B == B + A
  associativity of +     (A + B) + C == A + (B + C)
  distributivity          A * (B + C) == A*B + A*C, and the containment weakening
  additive inverse        A - A == 0, and the containment weakening

Arithmetic is outward-rounded, which `08` section 4.5 measures is the only rounding discipline
under which an interval result contains the exact result.

DOMAIN, with every count. Base numeral U<2,2> unsigned: 16 values k/4 for k in 0..15. Interval
data are the 136 ordered pairs. Every value is held as an integer count of quarter units, so all
arithmetic below is exact integer arithmetic and no floating point enters. Results falling
outside the base range are SKIPPED and counted separately, matching `08` section 4.5's protocol,
because overflow is a separate mechanism the record already separates from the quantiser
(`07` section 2.2).

Run: python3 p3_interval_laws.py
"""

from itertools import product

FRAC = 2
INT = 2
UNIT = 1 << FRAC          # one whole unit, in ticks
NVALS = 1 << (INT + FRAC)  # 16 grid points, 0..15 ticks
LO, HI = 0, NVALS - 1      # inclusive tick bounds of the representable range

IVALS = [(a, b) for a in range(NVALS) for b in range(a, NVALS)]


def in_range(t):
    return LO <= t <= HI


def add(x, y):
    """Interval addition. Exact on the grid, so no rounding is needed."""
    lo, hi = x[0] + y[0], x[1] + y[1]
    if not (in_range(lo) and in_range(hi)):
        return None
    return (lo, hi)


def sub(x, y):
    lo, hi = x[0] - y[1], x[1] - y[0]
    if not (in_range(lo) and in_range(hi)):
        return None
    return (lo, hi)


def mul(x, y):
    """Interval multiplication with OUTWARD rounding onto the grid.

    Products of two tick counts are in units of one quarter of a tick, so the low end
    rounds down and the high end rounds up, which is the outward discipline.
    """
    prods = [x[0] * y[0], x[0] * y[1], x[1] * y[0], x[1] * y[1]]
    lo_raw, hi_raw = min(prods), max(prods)
    lo = lo_raw // UNIT                      # floor
    hi = -((-hi_raw) // UNIT)                # ceil
    if not (in_range(lo) and in_range(hi)):
        return None
    return (lo, hi)


def contains(outer, inner):
    return outer[0] <= inner[0] and inner[1] <= outer[1]


def check(name, f, g, sample=None):
    """Compare two interval-valued expressions over the enumerated domain."""
    equal = differ = skipped = 0
    contained = 0
    witness = None
    domain = sample if sample is not None else None
    for args in domain:
        a = f(*args)
        b = g(*args)
        if a is None or b is None:
            skipped += 1
            continue
        if a == b:
            equal += 1
        else:
            differ += 1
            if contains(b, a):
                contained += 1
            if witness is None:
                witness = (args, a, b)
    total = equal + differ
    print(f"{name:38s} checked={total:9d} skipped={skipped:9d} "
          f"equal={equal:9d} differ={differ:8d} "
          f"of-which-lhs-inside-rhs={contained:8d}")
    if witness:
        args, a, b = witness
        print(f"{'':38s} first witness args={args} lhs={a} rhs={b}")
    return equal, differ, contained


def main():
    print(f"# base numeral U<{INT},{FRAC}> unsigned, {NVALS} grid points, "
          f"{len(IVALS)} interval data, outward rounding")
    print()

    pairs = list(product(IVALS, IVALS))
    print(f"# two-argument laws, over {len(pairs)} ordered pairs")
    check("commutativity of +",
          lambda a, b: add(a, b), lambda a, b: add(b, a), sample=pairs)
    check("A - A == 0",
          lambda a, b: sub(a, a), lambda a, b: (0, 0), sample=pairs[:len(IVALS)])
    print()

    triples = list(product(IVALS, IVALS, IVALS))
    print(f"# three-argument laws, over {len(triples)} ordered triples")
    check("associativity of +",
          lambda a, b, c: add(add(a, b), c) if add(a, b) else None,
          lambda a, b, c: add(a, add(b, c)) if add(b, c) else None,
          sample=triples)
    check("distributivity A*(B+C) vs A*B + A*C",
          lambda a, b, c: mul(a, add(b, c)) if add(b, c) else None,
          lambda a, b, c: add(mul(a, b), mul(a, c))
          if (mul(a, b) and mul(a, c)) else None,
          sample=triples)
    print()

    print("# the same four laws on the base numeral, where a datum denotes a point")
    pts = [(v, v) for v in range(NVALS)]
    ppairs = list(product(pts, pts))
    ptriples = list(product(pts, pts, pts))
    check("point: commutativity of +",
          lambda a, b: add(a, b), lambda a, b: add(b, a), sample=ppairs)
    check("point: A - A == 0",
          lambda a, b: sub(a, a), lambda a, b: (0, 0), sample=ppairs[:len(pts)])
    check("point: associativity of +",
          lambda a, b, c: add(add(a, b), c) if add(a, b) else None,
          lambda a, b, c: add(a, add(b, c)) if add(b, c) else None,
          sample=ptriples)
    check("point: distributivity",
          lambda a, b, c: mul(a, add(b, c)) if add(b, c) else None,
          lambda a, b, c: add(mul(a, b), mul(a, c))
          if (mul(a, b) and mul(a, c)) else None,
          sample=ptriples)


if __name__ == "__main__":
    main()
