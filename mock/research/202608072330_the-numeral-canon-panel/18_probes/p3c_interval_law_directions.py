"""p3c. The direction of each law failure, and the signed additive-inverse question.

p3b left two things unresolved and both change what its table means.

  ONE. The `A - A == 0` row on an unsigned numeral skipped 120 of 136 data because A - A goes
  below zero and was counted as out of range. So the row reported 100% of what it checked and
  said nothing about the 120. Re-run here on a SIGNED numeral where A - A is in range, which is
  the only setting in which the question has an answer.

  TWO. p3b reported `lhs-inside-rhs` for both algebras, and on a point numeral containment
  between two distinct points is vacuously false, so the point rows' zeros carry no information.
  Here each failure is classified in BOTH directions, so a law that fails as an equality but
  holds as a containment is separated from one that fails outright.

This matters because a law that fails directionally is still a law: subdistributivity is a
statement the algebra satisfies. A law that fails in both directions is not weakened, it is
gone.

DOMAIN, with every count. Two numerals. Unsigned U<2,2>: 16 grid points 0..15 ticks, 136
interval data. Signed I<1,2>: 16 grid points -8..7 ticks, 136 interval data. Integer tick
arithmetic throughout, exact. Out-of-range sub-expressions are skipped and counted.

Run: python3 p3c_interval_law_directions.py
"""

from itertools import product

FRAC = 2
UNIT = 1 << FRAC


def make(lo_tick, hi_tick):
    vals = list(range(lo_tick, hi_tick + 1))
    ivals = [(a, b) for i, a in enumerate(vals) for b in vals[i:]]
    return vals, ivals, lo_tick, hi_tick


def ops_for(LO, HI):
    def ok(t):
        return LO <= t <= HI

    def add(x, y):
        lo, hi = x[0] + y[0], x[1] + y[1]
        return (lo, hi) if ok(lo) and ok(hi) else None

    def sub(x, y):
        lo, hi = x[0] - y[1], x[1] - y[0]
        return (lo, hi) if ok(lo) and ok(hi) else None

    def mul(x, y):
        p = [x[0] * y[0], x[0] * y[1], x[1] * y[0], x[1] * y[1]]
        lo = min(p) // UNIT
        hi = -((-max(p)) // UNIT)
        return (lo, hi) if ok(lo) and ok(hi) else None

    return add, sub, mul


def contains(outer, inner):
    return outer[0] <= inner[0] and inner[1] <= outer[1]


def law(name, lhs, rhs, domain):
    eq = sk = lin = rin = neither = 0
    wit_neither = None
    for args in domain:
        a, b = lhs(*args), rhs(*args)
        if a is None or b is None:
            sk += 1
            continue
        if a == b:
            eq += 1
        elif contains(b, a):
            lin += 1
        elif contains(a, b):
            rin += 1
        else:
            neither += 1
            if wit_neither is None:
                wit_neither = (args, a, b)
    n = eq + lin + rin + neither
    print(f"{name:34s} checked={n:8d} skip={sk:8d}  equal={eq:8d}  "
          f"lhs<rhs={lin:7d}  rhs<lhs={rin:7d}  incomparable={neither:7d}")
    if n:
        print(f"{'':34s} equality {eq / n:6.2%}   lhs-contained-in-rhs "
              f"{(eq + lin) / n:6.2%}")
    if wit_neither:
        print(f"{'':34s} incomparable witness args={wit_neither[0]} "
              f"lhs={wit_neither[1]} rhs={wit_neither[2]}")


def main():
    for label, (lo_t, hi_t) in (("unsigned U<2,2>", (0, 15)), ("signed I<1,2>", (-8, 7))):
        vals, ivals, LO, HI = make(lo_t, hi_t)
        add, sub, mul = ops_for(LO, HI)
        print(f"## {label}: {len(vals)} grid points, {len(ivals)} interval data")

        law("A - A == 0", lambda a: sub(a, a), lambda a: (0, 0), [(a,) for a in ivals])

        pairs = list(product(ivals, ivals))
        trip = list(product(ivals, ivals, ivals))

        law("distributivity",
            lambda a, b, c: mul(a, add(b, c)) if add(b, c) else None,
            lambda a, b, c: add(mul(a, b), mul(a, c))
            if (mul(a, b) and mul(a, c)) else None, trip)

        law("associativity of *",
            lambda a, b, c: mul(mul(a, b), c) if mul(a, b) else None,
            lambda a, b, c: mul(a, mul(b, c)) if mul(b, c) else None, trip)
        print()

    print("## how far A - A is from zero on the signed numeral")
    vals, ivals, LO, HI = make(-8, 7)
    add, sub, mul = ops_for(LO, HI)
    widths = {}
    for a in ivals:
        r = sub(a, a)
        if r is None:
            continue
        w = r[1] - r[0]
        widths[w] = widths.get(w, 0) + 1
    print(f"  width of A - A, in ticks, over the {sum(widths.values())} in-range data:")
    for w in sorted(widths):
        print(f"    width {w:3d} ticks : {widths[w]:5d} data")
    print("  width zero is exactly the degenerate data; every other datum loses its inverse.")


if __name__ == "__main__":
    main()
