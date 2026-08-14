#!/usr/bin/env python3
"""P7. What "composing" means, and whether a composite is a different kind of thing.

The working assumption uses "composition" for building a primitive out of four
axes. That is configuration: choosing a point in a product of parameter spaces.
It is not what the word means in the algebra, and I11 asks for something else
entirely, "contracts for things that compose to bigger units than just numerals
alone": vectors, complex numbers, intervals, dual numbers.

Those are constructions that take an algebra and return an algebra. This probe
checks two things about them.

  C1. CLOSURE. Is the result of a construction itself a primitive, by the same
      definition (a carrier with a total interpretation of the signature)? If
      yes, then "primitive" and "composite" are one concept and a canon should
      not carry two, because every consumer contract written for one applies to
      the other unchanged.

  C2. THE THEORY TRANSFORMS COMPUTABLY. Is the composite's law set a function of
      the base's law set and the constructor? If yes, that is a second and
      independent reason a law set cannot be a component of a primitive: it is
      not something you get to choose even when you are building something new
      out of parts you did choose.

Constructions tested, all over the same base:
  product2   componentwise pairs, the shape of a vector lane
  complex    pairs with the twisted multiplication (ac - bd, ad + bc)
  dual       pairs with eps^2 = 0: (ac, ad + bc)
  interval   ordered pairs under inclusion-correct arithmetic
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
        self.lo, self.hi = min(self.values), max(self.values)

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


# --------------------------------------------------------------- constructions

class Product2:
    name = "product2"

    def __init__(self, b):
        self.b = b
        self.values = [(x, y) for x in b.values for y in b.values]

    def add(self, u, v):
        return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))

    def sub(self, u, v):
        return (self.b.sub(u[0], v[0]), self.b.sub(u[1], v[1]))

    def mul(self, u, v):
        return (self.b.mul(u[0], v[0]), self.b.mul(u[1], v[1]))


class Complex:
    name = "complex"

    def __init__(self, b):
        self.b = b
        self.values = [(x, y) for x in b.values for y in b.values]

    def add(self, u, v):
        return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))

    def sub(self, u, v):
        return (self.b.sub(u[0], v[0]), self.b.sub(u[1], v[1]))

    def mul(self, u, v):
        a, bb = u
        c, d = v
        re = self.b.sub(self.b.mul(a, c), self.b.mul(bb, d))
        im = self.b.add(self.b.mul(a, d), self.b.mul(bb, c))
        return (re, im)


class Dual:
    name = "dual"

    def __init__(self, b):
        self.b = b
        self.values = [(x, y) for x in b.values for y in b.values]

    def add(self, u, v):
        return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))

    def sub(self, u, v):
        return (self.b.sub(u[0], v[0]), self.b.sub(u[1], v[1]))

    def mul(self, u, v):
        a, bb = u
        c, d = v
        return (self.b.mul(a, c),
                self.b.add(self.b.mul(a, d), self.b.mul(bb, c)))


class Interval:
    name = "interval"

    def __init__(self, b):
        self.b = b
        self.values = [(x, y) for x in b.values for y in b.values if x <= y]

    def _hull(self, xs):
        return (min(xs), max(xs))

    def add(self, u, v):
        return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))

    def sub(self, u, v):
        return (self.b.sub(u[0], v[1]), self.b.sub(u[1], v[0]))

    def mul(self, u, v):
        cs = [self.b.mul(u[0], v[0]), self.b.mul(u[0], v[1]),
              self.b.mul(u[1], v[0]), self.b.mul(u[1], v[1])]
        return self._hull(cs)


CONSTRUCTIONS = [Product2, Complex, Dual, Interval]


# ---------------------------------------------------------------------- laws

def check_closure(alg):
    """C1: is every operation total and closed on the carrier?"""
    vs = set(alg.values)
    for op in ("add", "sub", "mul"):
        f = getattr(alg, op)
        for u, v in product(alg.values, repeat=2):
            if f(u, v) not in vs:
                return False, f"{op} escaped the carrier at ({u}, {v}) -> {f(u, v)}"
    return True, "closed and total"


def laws(alg):
    vs = alg.values
    out = {}
    out["add_comm"] = all(alg.add(a, b) == alg.add(b, a)
                          for a, b in product(vs, repeat=2))
    out["mul_comm"] = all(alg.mul(a, b) == alg.mul(b, a)
                          for a, b in product(vs, repeat=2))
    out["add_assoc"] = all(alg.add(alg.add(a, b), c) == alg.add(a, alg.add(b, c))
                           for a, b, c in product(vs, repeat=3))
    out["mul_assoc"] = all(alg.mul(alg.mul(a, b), c) == alg.mul(a, alg.mul(b, c))
                           for a, b, c in product(vs, repeat=3))
    out["distrib"] = all(alg.mul(a, alg.add(b, c)) == alg.add(alg.mul(a, b), alg.mul(a, c))
                         for a, b, c in product(vs, repeat=3))
    return out


LAW_NAMES = ["add_comm", "mul_comm", "add_assoc", "mul_assoc", "distrib"]


def main():
    print("P7. composing primitives, and whether the result is a primitive")
    print("=" * 78)

    bases = [
        Base(2, 0, False, "sat"),
        Base(2, 0, False, "wrap"),
        Base(2, 1, False, "sat"),
        Base(2, 0, True, "wrap"),
    ]

    print("C1. closure: is a composite itself a carrier with a total")
    print("    interpretation of the signature?")
    all_closed = True
    for b in bases[:2]:
        for C in CONSTRUCTIONS:
            alg = C(b)
            ok, why = check_closure(alg)
            all_closed &= ok
            print(f"    {C.name:<10} over {b.label():<12} "
                  f"{'closed' if ok else 'NOT CLOSED: ' + why}  "
                  f"(|carrier| = {len(alg.values)})")
    print(f"    verdict: {'every composite is itself a primitive' if all_closed else 'CLOSURE FAILS'}")
    print()

    print("C2. how the law set transforms under each construction:")
    hdr = f"    {'base':<14} {'construction':<12}" + "".join(f"{n:>11}" for n in LAW_NAMES)
    print(hdr)
    print("    " + "-" * (len(hdr) - 4))
    table = {}
    for b in bases:
        bl = laws(b)
        row = "".join(f"{('yes' if bl[n] else '.'):>11}" for n in LAW_NAMES)
        print(f"    {b.label():<14} {'(base)':<12}{row}")
        table[(b.label(), "base")] = bl
        for C in CONSTRUCTIONS:
            cl = laws(C(b))
            row = "".join(f"{('yes' if cl[n] else '.'):>11}" for n in LAW_NAMES)
            print(f"    {'':<14} {C.name:<12}{row}")
            table[(b.label(), C.name)] = cl
        print()

    print("C2 read out: which constructions preserve which laws, over these bases")
    for C in CONSTRUCTIONS:
        preserved, broken, created = [], [], []
        for b in bases:
            bl, cl = table[(b.label(), "base")], table[(b.label(), C.name)]
            for n in LAW_NAMES:
                if bl[n] and cl[n]:
                    preserved.append(n)
                elif bl[n] and not cl[n]:
                    broken.append((b.label(), n))
                elif (not bl[n]) and cl[n]:
                    created.append((b.label(), n))
        print(f"    {C.name:<10} breaks: "
              f"{sorted({n for _, n in broken}) if broken else 'nothing the base had'}")
        if created:
            print(f"               CREATES a law the base lacked: "
                  f"{sorted({n for _, n in created})}")
    print()

    print("the componentwise construction is the interesting control:")
    prod_breaks = []
    for b in bases:
        bl, cl = table[(b.label(), "base")], table[(b.label(), "product2")]
        for n in LAW_NAMES:
            if bl[n] != cl[n]:
                prod_breaks.append((b.label(), n, bl[n], cl[n]))
    if not prod_breaks:
        print("    product2 reproduces the base's law set exactly, on every base.")
        print("    that is the classical fact that equational theories survive")
        print("    products, and it means a lane-wise composite inherits every")
        print("    rewrite the scalar was licensed for, with no new measurement.")
    else:
        print(f"    product2 DIVERGED from the base on {len(prod_breaks)} entries:")
        for lbl, n, was, now in prod_breaks:
            print(f"      {lbl} {n}: base={was} composite={now}")
    print()
    print("    the twisted constructions do not, so the law set of a composite")
    print("    is a function of the base's laws AND the constructor, and is")
    print("    read off rather than chosen, exactly as P2 found for the base.")


if __name__ == "__main__":
    main()
