#!/usr/bin/env python3
"""P7b. A construction on primitives is a predicated arm, like everything else.

P7 found something it was not looking for. The interval construction is NOT
closed over a wrapping base: adding two well-ordered intervals produced
(1, 0), whose lower bound exceeds its upper, so the result is not in the
carrier and "interval over a wrapping primitive" is not a primitive at all.

That is not a defect in the construction. It is the construction having a
PRECONDITION, and the obvious candidate is monotonicity: interval arithmetic
computes on endpoints and relies on the operations preserving order. Wrapping
does not preserve order, which P2 already measured directly (add_monotone is
false for every wrapping configuration in its table).

So this probe tests the prediction:

    interval over B is closed  iff  B's operations are monotone

and, more generally, that each construction carries a stateable predicate on
its base, with a computable effect on the resulting theory. If that holds, then
composition is the same shape as everything else here: an arm that applies on
the region where its predicate holds and nowhere else.
"""

from fractions import Fraction
from itertools import product


class Base:
    def __init__(self, W, F, signed, policy, radix=2):
        self.W, self.F, self.signed, self.policy = W, F, signed, policy
        n = 1 << W
        self.ints = list(range(-(n // 2), n // 2)) if signed else list(range(0, n))
        self.step = Fraction(1, radix ** F)
        self.values = [Fraction(k) * self.step for k in self.ints]

    def label(self):
        s = "i" if self.signed else "u"
        return f"{s}W{self.W}F{self.F}/{self.policy}"

    def R(self, q):
        r = q / self.step
        fl = r.numerator // r.denominator
        fr = r - fl
        k = fl + 1 if fr > Fraction(1, 2) else (fl if fr < Fraction(1, 2)
                                               else (fl if fl % 2 == 0 else fl + 1))
        n = 1 << self.W
        if self.policy == "wrap":
            k = ((k + n // 2) % n) - n // 2 if self.signed else k % n
        else:
            klo, khi = self.ints[0], self.ints[-1]
            k = klo if k < klo else (khi if k > khi else k)
        return Fraction(k) * self.step

    def add(self, a, b):
        return self.R(a + b)

    def sub(self, a, b):
        return self.R(a - b)

    def mul(self, a, b):
        return self.R(a * b)


def is_monotone(b):
    """Does every operation preserve the order of its arguments?"""
    for x, y, c in product(b.values, repeat=3):
        if x <= y:
            if b.add(x, c) > b.add(y, c):
                return False, f"add: {x}<={y} but add({x},{c})>add({y},{c})"
            if c >= 0 and b.mul(x, c) > b.mul(y, c):
                return False, f"mul: {x}<={y}, {c}>=0, but mul({x},{c})>mul({y},{c})"
    return True, "monotone"


def is_ring(b):
    """Associative, distributive, with additive inverses present."""
    vs = b.values
    assoc = all(b.mul(b.mul(x, y), z) == b.mul(x, b.mul(y, z))
                for x, y, z in product(vs, repeat=3))
    dist = all(b.mul(x, b.add(y, z)) == b.add(b.mul(x, y), b.mul(x, z))
               for x, y, z in product(vs, repeat=3))
    return assoc and dist


class Product2:
    name = "product2"
    def __init__(self, b):
        self.b = b
        self.values = [(x, y) for x in b.values for y in b.values]
    def add(self, u, v):
        return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))
    def mul(self, u, v):
        return (self.b.mul(u[0], v[0]), self.b.mul(u[1], v[1]))


class Complex:
    name = "complex"
    def __init__(self, b):
        self.b = b
        self.values = [(x, y) for x in b.values for y in b.values]
    def add(self, u, v):
        return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))
    def mul(self, u, v):
        a, bb = u; c, d = v
        return (self.b.sub(self.b.mul(a, c), self.b.mul(bb, d)),
                self.b.add(self.b.mul(a, d), self.b.mul(bb, c)))


class Interval:
    name = "interval"
    def __init__(self, b):
        self.b = b
        self.values = [(x, y) for x in b.values for y in b.values if x <= y]
    def add(self, u, v):
        return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))
    def mul(self, u, v):
        cs = [self.b.mul(u[0], v[0]), self.b.mul(u[0], v[1]),
              self.b.mul(u[1], v[0]), self.b.mul(u[1], v[1])]
        return (min(cs), max(cs))


CONSTRUCTIONS = [Product2, Complex, Interval]


def closed(alg):
    vs = set(alg.values)
    for op in ("add", "mul"):
        f = getattr(alg, op)
        for u, v in product(alg.values, repeat=2):
            if f(u, v) not in vs:
                return False, f"{op} at ({u},{v}) -> {f(u,v)}"
    return True, ""


def main():
    print("P7b. does each construction carry a predicate on its base?")
    print("=" * 78)

    bases = []
    for W, F, signed, policy in product([2, 3], [0, 1], [False, True], ["sat", "wrap"]):
        bases.append(Base(W, F, signed, policy))
    print(f"bases swept: {len(bases)}  (W in {{2,3}}, F in {{0,1}}, "
          f"signed in {{no,yes}}, policy in {{sat,wrap}})")
    print()

    props = {}
    for b in bases:
        mono, _ = is_monotone(b)
        props[b.label()] = dict(monotone=mono, ring=is_ring(b))

    print("base properties, and whether each construction is closed over it:")
    print(f"  {'base':<14} {'monotone':>9} {'ring':>6}   "
          + "".join(f"{C.name:>12}" for C in CONSTRUCTIONS))
    rows = {}
    for b in bases:
        p = props[b.label()]
        cells = []
        for C in CONSTRUCTIONS:
            ok, why = closed(C(b))
            cells.append(ok)
            rows[(b.label(), C.name)] = (ok, why)
        line = (f"  {b.label():<14} {('yes' if p['monotone'] else 'no'):>9} "
                f"{('yes' if p['ring'] else 'no'):>6}   "
                + "".join(f"{('closed' if c else 'NOT'):>12}" for c in cells))
        print(line)
    print()

    print("testing the predicted predicate for each construction:")
    for C in CONSTRUCTIONS:
        for prop in ["monotone", "ring", None]:
            agree = 0
            for b in bases:
                ok, _ = rows[(b.label(), C.name)]
                pred = True if prop is None else props[b.label()][prop]
                if ok == pred:
                    agree += 1
            name = prop if prop else "always (no precondition)"
            mark = "  <== exact" if agree == len(bases) else ""
            print(f"  {C.name:<10} vs predicate '{name:<24}' "
                  f"agrees on {agree}/{len(bases)} bases{mark}")
        print()

    print("counterexamples, where a construction failed:")
    shown = 0
    for (bl, cn), (ok, why) in rows.items():
        if not ok and shown < 6:
            print(f"  {cn} over {bl}: {why}")
            shown += 1
    if shown == 0:
        print("  none")
    print()

    print("reading:")
    print("  product2 has no precondition. componentwise construction preserves")
    print("  every equation the base satisfies, so it is total on primitives and")
    print("  a lane-wise composite is licensed for exactly the rewrites its")
    print("  scalar was, with no separate measurement.")
    print()
    print("  interval is PARTIAL. it computes on endpoints and needs the base's")
    print("  operations to preserve order, and a wrapping base does not, so the")
    print("  result is not even a carrier: an ordered pair comes back inverted.")
    print("  that is a construction carrying a const predicate on its base,")
    print("  which is the same shape as every other arm in this design.")


if __name__ == "__main__":
    main()
