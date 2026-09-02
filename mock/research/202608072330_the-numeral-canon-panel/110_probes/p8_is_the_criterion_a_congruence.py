#!/usr/bin/env python3
"""P8. Is the identity criterion a congruence? The soundness check on my own answer.

An identity criterion says which primitives may be substituted for which. That
is only safe if it survives every context a primitive can appear in. In
rewriting terms it has to be a CONGRUENCE: if P and Q are the same primitive,
then C(P) and C(Q) must be the same primitive for every construction C. If that
fails anywhere, the criterion licenses a substitution that changes behaviour one
level up, and no inspection at the level where the substitution was made will
ever find it.

So this probe attacks my own answer at its weakest joint. It tests three
criteria of increasing strength against four constructions:

  weak     same value set. ignores what the operations compute.
  medium   same value set and same `add` table. ignores the rest of the signature.
  full     denotation-preserving isomorphism over the whole signature.
           (this is what P1 used and what I claim is the right criterion)

For each, take every pair the criterion calls the same, build both composites,
and ask whether the composites are still the same.

A NOTE ON THE POPULATION, because the first run of this probe was defective and
the defect is instructive. It swept no rounding modes, so `medium` and `full`
merged exactly the same 23 pairs and both scored zero failures, which made
`medium` look sound. It is not: P1 established that the rounding mode at F > 0
is separated by `mul` and NOT by `add`, so a criterion that reads only `add`
must merge two primitives that `mul` distinguishes, and the population simply
contained no such pair. A criterion cannot be tested against a population that
excludes the case it fails on, which is the "setup that helps" failure in a
probe rather than in a test. The population below varies rounding.
"""

from fractions import Fraction
from itertools import product


class Prim:
    """A primitive presented as codes plus a denotation, so that a pure change
    of encoding is expressible and the denotation stays separable from the
    operation tables."""

    def __init__(self, W, F, signed, policy, radix=2, rounding="near", enc="ident"):
        self.W, self.F, self.signed, self.policy, self.radix = W, F, signed, policy, radix
        self.rounding = rounding
        n = 1 << W
        self.ints = list(range(-(n // 2), n // 2)) if signed else list(range(0, n))
        self.step = Fraction(1, radix ** F)
        self.values = [Fraction(k) * self.step for k in self.ints]
        m = len(self.ints)
        if enc == "ident":
            perm = list(range(m))
        elif enc == "offset":
            perm = [(i + m // 2) % m for i in range(m)]
        elif enc == "gray":
            perm = [i ^ (i >> 1) for i in range(m)]
        elif enc == "shuffled":
            perm = [(7 * i + 3) % m for i in range(m)]
        else:
            raise ValueError(enc)
        assert len(set(perm)) == m
        self.enc = enc
        self.code_of_index = perm

    def label(self):
        s = "i" if self.signed else "u"
        return (f"{s}W{self.W}F{self.F}r{self.radix}/{self.policy}"
                f"/{self.rounding}/{self.enc}")

    def R(self, q):
        r = q / self.step
        if self.rounding == "trunc":
            k = int(r)
        elif self.rounding == "floor":
            k = r.numerator // r.denominator
        else:
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


# ------------------------------------------------------------- constructions

class Product2:
    name = "product2"
    def __init__(self, b):
        self.b = b
        self.values = [(x, y) for x in b.values for y in b.values]
    def add(self, u, v): return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))
    def sub(self, u, v): return (self.b.sub(u[0], v[0]), self.b.sub(u[1], v[1]))
    def mul(self, u, v): return (self.b.mul(u[0], v[0]), self.b.mul(u[1], v[1]))


class Complex:
    name = "complex"
    def __init__(self, b):
        self.b = b
        self.values = [(x, y) for x in b.values for y in b.values]
    def add(self, u, v): return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))
    def sub(self, u, v): return (self.b.sub(u[0], v[0]), self.b.sub(u[1], v[1]))
    def mul(self, u, v):
        a, bb = u; c, d = v
        return (self.b.sub(self.b.mul(a, c), self.b.mul(bb, d)),
                self.b.add(self.b.mul(a, d), self.b.mul(bb, c)))


class Interval:
    name = "interval"
    def __init__(self, b):
        self.b = b
        self.values = [(x, y) for x in b.values for y in b.values if x <= y]
    def add(self, u, v): return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))
    def sub(self, u, v): return (self.b.sub(u[0], v[1]), self.b.sub(u[1], v[0]))
    def mul(self, u, v):
        cs = [self.b.mul(u[0], v[0]), self.b.mul(u[0], v[1]),
              self.b.mul(u[1], v[0]), self.b.mul(u[1], v[1])]
        return (min(cs), max(cs))


class Product3:
    """A three-wide lane, to check the congruence at a second arity."""
    name = "product3"
    def __init__(self, b):
        self.b = b
        vs = b.values[:4]
        self.values = [(x, y, z) for x in vs for y in vs for z in vs]
    def add(self, u, v): return tuple(self.b.add(u[i], v[i]) for i in range(3))
    def sub(self, u, v): return tuple(self.b.sub(u[i], v[i]) for i in range(3))
    def mul(self, u, v): return tuple(self.b.mul(u[i], v[i]) for i in range(3))


CONSTRUCTIONS = [Product2, Complex, Interval, Product3]


# ------------------------------------------------------------------ criteria

def table(alg, ops=("add", "sub", "mul")):
    parts = [tuple(sorted(alg.values, key=str))]
    for op in ops:
        f = getattr(alg, op)
        parts.append(tuple(f(a, b) for a, b in product(alg.values, repeat=2)))
    return tuple(parts)


def crit_weak(p, q):
    """Same value set. Ignores every operation."""
    return set(p.values) == set(q.values)


def crit_medium(p, q):
    """Same value set and same add. Ignores sub and mul."""
    if set(p.values) != set(q.values):
        return False
    return table(p, ("add",)) == table(q, ("add",))


def crit_full(p, q):
    """Denotation-preserving isomorphism over the whole signature.

    Preserving the denotation forces the map, so this reduces to equality of
    the value sets and of every operation table read denotationally.
    """
    if set(p.values) != set(q.values):
        return False
    return table(p) == table(q)


CRITERIA = [("weak (value set only)", crit_weak),
            ("medium (value set + add)", crit_medium),
            ("full (denotation-preserving iso)", crit_full)]


def composites_agree(C, p, q):
    return table(C(p)) == table(C(q))


def main():
    print("P8. is the identity criterion a congruence with respect to composition?")
    print("=" * 78)

    pop = []
    for W, F, signed, policy, radix, rnd in product(
        [2, 3], [0, 1], [False, True], ["sat", "wrap"], [2, 3], ["near", "trunc"]
    ):
        pop.append(Prim(W, F, signed, policy, radix, rnd))
    # encoding variants of two configurations. the full criterion calls these
    # the same primitive, so they are the control in the other direction: if a
    # construction distinguished them, the criterion would be unsound rather
    # than merely strict.
    for enc in ["offset", "gray", "shuffled"]:
        pop.append(Prim(2, 1, False, "sat", 2, "near", enc))
        pop.append(Prim(3, 0, True, "wrap", 2, "near", enc))
    print(f"population: {len(pop)} primitives, rounding varied over "
          f"{{near, trunc}} so the criteria can come apart")
    print()

    print(f"  {'criterion':<34} {'pairs called same':>18} {'congruence failures':>21}")
    print("  " + "-" * 74)
    detail = {}
    for cname, crit in CRITERIA:
        same_pairs, fails = 0, 0
        examples = []
        for i in range(len(pop)):
            for j in range(i + 1, len(pop)):
                p, q = pop[i], pop[j]
                if not crit(p, q):
                    continue
                same_pairs += 1
                for C in CONSTRUCTIONS:
                    if not composites_agree(C, p, q):
                        fails += 1
                        if len(examples) < 4:
                            examples.append((C.name, p.label(), q.label()))
                        break
        detail[cname] = examples
        print(f"  {cname:<34} {same_pairs:>18} {fails:>21}")
    print()

    for cname, examples in detail.items():
        if examples:
            print(f"  {cname} breaks at, for instance:")
            for c, a, b in examples:
                print(f"    {c:<10} distinguishes")
                print(f"               {a}")
                print(f"               {b}")
            print()

    enc_pairs, enc_fail = 0, 0
    for i in range(len(pop)):
        for j in range(i + 1, len(pop)):
            p, q = pop[i], pop[j]
            if p.enc == q.enc or not crit_full(p, q):
                continue
            enc_pairs += 1
            for C in CONSTRUCTIONS:
                if not composites_agree(C, p, q):
                    enc_fail += 1
                    break
    print(f"  control: encoding-differing pairs the full criterion merges: {enc_pairs}")
    print(f"  control: of those, congruence failures:                      {enc_fail}")
    print()

    print("reading:")
    print("  a criterion with a congruence failure licenses substituting one")
    print("  primitive for another inside a composite and getting different")
    print("  behaviour out. nothing at the site of the substitution can detect")
    print("  it, because at that level the two really are interchangeable by")
    print("  the criterion that was applied.")
    print()
    print("  the weak criterion is what a reader reaches for when a primitive")
    print("  is thought of as a set of representable values. the medium one is")
    print("  what checking one operation buys, and it fails precisely where P1")
    print("  said it would: on a rounding mode that add cannot see and mul can.")
    print("  only the full criterion survives, and the encoding control shows")
    print("  it is not surviving by refusing to merge anything.")


if __name__ == "__main__":
    main()
