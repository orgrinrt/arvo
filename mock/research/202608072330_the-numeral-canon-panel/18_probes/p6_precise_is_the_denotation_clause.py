"""p6. How often a numeral operation produces a point denotation, and what Precise is.

`07` section 2.3 establishes that a ROUNDED datum stands for the set of exact values that
produced it, and that the set is a point only for one mode. Read together with `08` section 5's
first clause, that has a consequence nobody in the panel has stated:

  A numeral datum denotes one rational when it was CONSTRUCTED, and denotes a set the moment an
  inexact operation produced it. So the denotation clause is a statement about the constructor,
  not about every datum in flight.

Which reframes an item the record carries as open. `SETTLED.md:173` lists "`Precise` on
`inexact`" as open since `145`, and `01` section 4 records op offering three options and acking
the third. Under the reading above:

  Precise-refusing-on-inexact IS the strategy that demands its data keep a POINT denotation.

That is not a new mechanism, it is a name for one the design is already arguing about. This
probe measures how large the demand is, so the reframing carries a magnitude rather than only a
sentence.

DOMAIN, with every count. Numeral U<I,F> unsigned at three shapes. Operands range over every
ordered pair of grid values. An operation is EXACT when its true rational result is on the grid
and in range; it is INEXACT when the true result is in range but off the grid; it is OUT OF
RANGE otherwise, which `07` section 2.2 separates from the quantiser and which is counted
separately here for that reason.

Run: python3 p6_precise_is_the_denotation_clause.py
"""

from fractions import Fraction as F


def classify(int_bits, frac_bits):
    step = F(1, 2 ** frac_bits)
    n = 2 ** (int_bits + frac_bits)
    vals = [k * step for k in range(n)]
    top = vals[-1]
    grid = set(vals)

    out = {}
    for name, op in (("add", lambda a, b: a + b),
                     ("sub", lambda a, b: a - b),
                     ("mul", lambda a, b: a * b),
                     ("div", lambda a, b: (a / b) if b != 0 else None)):
        exact = inexact = oor = undef = 0
        for a in vals:
            for b in vals:
                r = op(a, b)
                if r is None:
                    undef += 1
                    continue
                if r < 0 or r > top:
                    oor += 1
                elif r in grid:
                    exact += 1
                else:
                    inexact += 1
        out[name] = (exact, inexact, oor, undef)
    return n, out


def main():
    print("# share of operand pairs whose result keeps a POINT denotation")
    print(f"{'numeral':>10} {'values':>7} {'op':>5} {'exact':>9} {'inexact':>9} "
          f"{'out-of-range':>13} {'undefined':>10} {'exact share of in-range':>25}")
    for ib, fb in ((2, 2), (3, 3), (4, 4)):
        n, res = classify(ib, fb)
        for name, (e, i, o, u) in res.items():
            inrange = e + i
            share = e / inrange if inrange else float("nan")
            print(f"{f'U<{ib},{fb}>':>10} {n:>7} {name:>5} {e:>9} {i:>9} {o:>13} {u:>10} "
                  f"{share:>24.2%}")
    print()
    print("# reading")
    print("  Addition and subtraction keep the point denotation on every in-range pair,")
    print("  because the grid is closed under them. Multiplication and division do not, and")
    print("  the share that does is small and shrinks with the fraction width.")
    print()
    print("  So a strategy that demands a point denotation is not a marginal restriction on")
    print("  a numeral's operation set. It admits the additive part outright and refuses most")
    print("  of the multiplicative part, which is the same shape as the record's open")
    print("  question about Precise, arrived at from the denotation clause instead of from")
    print("  the strategy axis.")


if __name__ == "__main__":
    main()
