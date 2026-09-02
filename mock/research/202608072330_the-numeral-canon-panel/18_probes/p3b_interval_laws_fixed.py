"""p3b. Which laws survive if a datum denotes an interval. Corrected from p3.

p3 had two defects and both were setup that helps, which is the failure mode the workspace's
test gate names. They are recorded here rather than quietly fixed, because a probe's value is
what it proved and a probe that proved the wrong thing has to say so.

  DEFECT 1. The `A - A == 0` row sliced the pair list as `pairs[:len(IVALS)]`, which holds the
  FIRST argument fixed at (0,0). So it tested one interval, the only one for which the law is
  true, and reported 136 successes. Corrected here to quantify over all 136.

  DEFECT 2. The point-numeral control rows used the same outward-rounding multiply as the
  interval rows, so a point input produced a two-element output and the control measured
  outward rounding rather than the point algebra. Corrected here: the point control uses one
  directed mode, which is what a point numeral has.

DOMAIN, with every count. Base numeral U<2,2> unsigned, 16 grid points k/4 for k in 0..15, held
as integer tick counts so all arithmetic is exact integer arithmetic. Interval data are the 136
ordered pairs (lo, hi) with lo <= hi. A tuple is SKIPPED when any sub-expression leaves the
representable range, matching `08` section 4.5 and `07` section 2.2, which both separate
overflow from the quantiser. The checked count is therefore the in-range subdomain and is
reported beside every result.

Run: python3 p3b_interval_laws_fixed.py
"""

from itertools import product

FRAC, INT = 2, 2
UNIT = 1 << FRAC
NVALS = 1 << (INT + FRAC)
LO, HI = 0, NVALS - 1

IVALS = [(a, b) for a in range(NVALS) for b in range(a, NVALS)]
PVALS = [(v, v) for v in range(NVALS)]


def ok(t):
    return LO <= t <= HI


def add(x, y):
    lo, hi = x[0] + y[0], x[1] + y[1]
    return (lo, hi) if ok(lo) and ok(hi) else None


def sub(x, y):
    lo, hi = x[0] - y[1], x[1] - y[0]
    return (lo, hi) if ok(lo) and ok(hi) else None


def mul_outward(x, y):
    p = [x[0] * y[0], x[0] * y[1], x[1] * y[0], x[1] * y[1]]
    lo = min(p) // UNIT
    hi = -((-max(p)) // UNIT)
    return (lo, hi) if ok(lo) and ok(hi) else None


def mul_floor(x, y):
    """One directed mode, which is what a point numeral has. Both ends round down."""
    p = [x[0] * y[0], x[0] * y[1], x[1] * y[0], x[1] * y[1]]
    lo = min(p) // UNIT
    hi = max(p) // UNIT
    return (lo, hi) if ok(lo) and ok(hi) else None


def contains(outer, inner):
    return outer[0] <= inner[0] and inner[1] <= outer[1]


def law(name, lhs, rhs, domain):
    eq = ne = sk = inside = 0
    wit = None
    for args in domain:
        a, b = lhs(*args), rhs(*args)
        if a is None or b is None:
            sk += 1
            continue
        if a == b:
            eq += 1
        else:
            ne += 1
            if contains(b, a):
                inside += 1
            if wit is None:
                wit = (args, a, b)
    n = eq + ne
    rate = (eq / n) if n else float("nan")
    print(f"{name:40s} checked={n:8d} skipped={sk:8d} holds={eq:8d} ({rate:7.2%}) "
          f"fails={ne:7d} lhs-inside-rhs={inside:7d}")
    if wit:
        print(f"{'':40s} witness args={wit[0]} lhs={wit[1]} rhs={wit[2]}")


def main():
    print(f"# U<{INT},{FRAC}> unsigned: {NVALS} grid points, {len(IVALS)} interval data")
    print()

    pairs_i = list(product(IVALS, IVALS))
    pairs_p = list(product(PVALS, PVALS))
    trip_i = list(product(IVALS, IVALS, IVALS))
    trip_p = list(product(PVALS, PVALS, PVALS))

    print("## interval denotation, outward rounding")
    law("commutativity of +", lambda a, b: add(a, b), lambda a, b: add(b, a), pairs_i)
    law("commutativity of *", lambda a, b: mul_outward(a, b),
        lambda a, b: mul_outward(b, a), pairs_i)
    law("A - A == 0  [corrected from p3]", lambda a: sub(a, a),
        lambda a: (0, 0), [(a,) for a in IVALS])
    law("associativity of +",
        lambda a, b, c: add(add(a, b), c) if add(a, b) else None,
        lambda a, b, c: add(a, add(b, c)) if add(b, c) else None, trip_i)
    law("associativity of *",
        lambda a, b, c: mul_outward(mul_outward(a, b), c) if mul_outward(a, b) else None,
        lambda a, b, c: mul_outward(a, mul_outward(b, c)) if mul_outward(b, c) else None,
        trip_i)
    law("distributivity",
        lambda a, b, c: mul_outward(a, add(b, c)) if add(b, c) else None,
        lambda a, b, c: add(mul_outward(a, b), mul_outward(a, c))
        if (mul_outward(a, b) and mul_outward(a, c)) else None, trip_i)
    print()

    print("## point denotation, one directed mode  [corrected from p3]")
    law("commutativity of +", lambda a, b: add(a, b), lambda a, b: add(b, a), pairs_p)
    law("commutativity of *", lambda a, b: mul_floor(a, b),
        lambda a, b: mul_floor(b, a), pairs_p)
    law("A - A == 0", lambda a: sub(a, a), lambda a: (0, 0), [(a,) for a in PVALS])
    law("associativity of +",
        lambda a, b, c: add(add(a, b), c) if add(a, b) else None,
        lambda a, b, c: add(a, add(b, c)) if add(b, c) else None, trip_p)
    law("associativity of *",
        lambda a, b, c: mul_floor(mul_floor(a, b), c) if mul_floor(a, b) else None,
        lambda a, b, c: mul_floor(a, mul_floor(b, c)) if mul_floor(b, c) else None, trip_p)
    law("distributivity",
        lambda a, b, c: mul_floor(a, add(b, c)) if add(b, c) else None,
        lambda a, b, c: add(mul_floor(a, b), mul_floor(a, c))
        if (mul_floor(a, b) and mul_floor(a, c)) else None, trip_p)
    print()

    print("## how many of the 136 interval data are their own additive inverse")
    self_inv = sum(1 for a in IVALS if sub(a, a) == (0, 0))
    print(f"  {self_inv} of {len(IVALS)}, and they are exactly the degenerate intervals")
    print(f"  degenerate count = {len(PVALS)}")


if __name__ == "__main__":
    main()
