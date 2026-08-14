#!/usr/bin/env python3
"""P2. Is "law set" a component of a primitive, or a reading taken off one?

The working assumption lists a LAW SET as one of the things a primitive is a
composition of. A composition's components are coordinates: you can hold the
others fixed and vary this one. So the assumption is testable directly.

  If law set is a coordinate, then somewhere in the configuration space there
  are two primitives agreeing on every other coordinate and differing in their
  law set. Equivalently: the law set is FREE.

  If law set is a projection, then it is a function of the rest, it can never
  be varied independently, and it is strictly LESS information than the
  primitive (many primitives share one law set).

This probe decides it by exhaustion over a swept configuration space, and in
passing checks a second claim: that overflow policy and rounding policy are not
two mechanisms but two regions of ONE map

    R : Q -> V     (exact rational result -> representable value)

with saturation and wrapping being what R does outside the range and rounding
being what it does between grid points. If that is right, then "the operations"
of a primitive are fully determined by (value set, R, signature), and there is
nothing left over for a law set to be.

CONTAMINATION DECLARED. The workspace rule `arvo-always-optimal-internals.md`
is auto-loaded into every agent context in this workspace, and it already
carries a related claim about when a law holds, attributed to another panel
member's probe. This probe was not written to check that claim and does not
claim independence from it. What is tested here is a different question: not
WHEN a law holds, but whether the law set is a coordinate of a primitive.
"""

from fractions import Fraction
from itertools import product

# ---------------------------------------------------------------- the model


class Prim:
    def __init__(self, W, F, signed, policy, rounding, radix=2):
        self.W, self.F, self.signed = W, F, signed
        self.policy, self.rounding, self.radix = policy, rounding, radix
        n = 1 << W
        self.ints = list(range(-(n // 2), n // 2)) if signed else list(range(0, n))
        self.step = Fraction(1, radix ** F)
        self.values = [Fraction(k) * self.step for k in self.ints]
        self.lo, self.hi = min(self.values), max(self.values)

    def key(self):
        return (self.W, self.F, self.signed, self.policy, self.rounding, self.radix)

    def label(self):
        s = "i" if self.signed else "u"
        return f"{s}W{self.W}F{self.F}/{self.policy}/{self.rounding}"

    # ---- the single realisation map
    def R(self, q):
        k = self._to_grid(q)
        n = 1 << self.W
        if self.policy == "wrap":
            if self.signed:
                k = ((k + n // 2) % n) - n // 2
            else:
                k = k % n
            return Fraction(k) * self.step
        # saturate: clamp on the grid
        klo, khi = self.ints[0], self.ints[-1]
        k = klo if k < klo else (khi if k > khi else k)
        return Fraction(k) * self.step

    def _to_grid(self, q):
        r = q / self.step
        if self.rounding == "trunc":
            return int(r)
        fl = r.numerator // r.denominator
        frac = r - fl
        if frac > Fraction(1, 2):
            return fl + 1
        if frac < Fraction(1, 2):
            return fl
        return fl if fl % 2 == 0 else fl + 1

    def add(self, a, b):
        return self.R(a + b)

    def sub(self, a, b):
        return self.R(a - b)

    def mul(self, a, b):
        return self.R(a * b)

    def neg(self, a):
        return self.R(-a)


# ---------------------------------------------------------------- the laws

def law_add_comm(p):
    return all(p.add(a, b) == p.add(b, a) for a, b in product(p.values, repeat=2))


def law_add_assoc(p):
    return all(
        p.add(p.add(a, b), c) == p.add(a, p.add(b, c))
        for a, b, c in product(p.values, repeat=3)
    )


def law_mul_comm(p):
    return all(p.mul(a, b) == p.mul(b, a) for a, b in product(p.values, repeat=2))


def law_mul_assoc(p):
    return all(
        p.mul(p.mul(a, b), c) == p.mul(a, p.mul(b, c))
        for a, b, c in product(p.values, repeat=3)
    )


def law_distrib_add(p):
    return all(
        p.mul(a, p.add(b, c)) == p.add(p.mul(a, b), p.mul(a, c))
        for a, b, c in product(p.values, repeat=3)
    )


def law_distrib_sub(p):
    return all(
        p.mul(a, p.sub(b, c)) == p.sub(p.mul(a, b), p.mul(a, c))
        for a, b, c in product(p.values, repeat=3)
    )


def law_add_zero(p):
    z = Fraction(0)
    return z in p.values and all(p.add(a, z) == a for a in p.values)


def law_mul_one(p):
    o = Fraction(1)
    return o in p.values and all(p.mul(a, o) == a for a in p.values)


def law_add_monotone(p):
    return all(
        (p.add(a, c) <= p.add(b, c))
        for a, b, c in product(p.values, repeat=3)
        if a <= b
    )


def law_neg_involutive(p):
    return all(p.neg(p.neg(a)) == a for a in p.values)


LAWS = [
    ("add_comm", law_add_comm),
    ("add_assoc", law_add_assoc),
    ("mul_comm", law_mul_comm),
    ("mul_assoc", law_mul_assoc),
    ("distrib_add", law_distrib_add),
    ("distrib_sub", law_distrib_sub),
    ("add_zero", law_add_zero),
    ("mul_one", law_mul_one),
    ("add_monotone", law_add_monotone),
    ("neg_involutive", law_neg_involutive),
]


def law_set(p):
    return tuple(f(p) for _, f in LAWS)


def op_table(p):
    """The full interpretation: what makes two primitives the same algebra."""
    return (
        tuple(p.values),
        tuple(p.add(a, b) for a, b in product(p.values, repeat=2)),
        tuple(p.sub(a, b) for a, b in product(p.values, repeat=2)),
        tuple(p.mul(a, b) for a, b in product(p.values, repeat=2)),
        tuple(p.neg(a) for a in p.values),
    )


# ---------------------------------------------------------------- the sweep

def main():
    configs = []
    for W, F, signed, policy, rounding in product(
        [3, 4], [0, 1, 2], [False, True], ["sat", "wrap"], ["near", "trunc"]
    ):
        configs.append(Prim(W, F, signed, policy, rounding))

    print("P2. is a law set a coordinate of a primitive, or a projection of one?")
    print("=" * 78)
    print(f"configurations swept: {len(configs)}")
    print(f"  W in {{3,4}}, F in {{0,1,2}}, signed in {{no,yes}},")
    print(f"  policy in {{sat,wrap}}, rounding in {{near,trunc}}, radix = 2")
    print()

    algebras = {}
    for p in configs:
        algebras.setdefault(op_table(p), []).append(p)
    lawsets = {}
    for p in configs:
        lawsets.setdefault(law_set(p), []).append(p)

    print(f"distinct algebras (full operation tables): {len(algebras)}")
    print(f"distinct law sets over {len(LAWS)} laws:      {len(lawsets)}")
    print()

    # TEST 1. Is the law set a function of the algebra? If two configs have the
    # same operation tables they must have the same law set, or the laws are
    # reading something the algebra does not contain.
    bad = 0
    for table, ps in algebras.items():
        sets = {law_set(p) for p in ps}
        if len(sets) != 1:
            bad += 1
            print(f"  FUNCTION FAILS: {[p.label() for p in ps]} share an algebra, differ in laws")
    print(f"TEST 1 law set is a function of the algebra: {'PASS' if bad == 0 else 'FAIL'}")
    print(f"  ({len(algebras)} algebra classes checked, {bad} carried more than one law set)")
    print()

    # TEST 2. Is it INJECTIVE? If distinct algebras can share a law set, then
    # the law set is strictly less information than the primitive, and cannot
    # be a coordinate that reconstructs it.
    collisions = [(ls, ps) for ls, ps in lawsets.items() if len({op_table(p) for p in ps}) > 1]
    print(f"TEST 2 law set is injective on algebras: {'yes' if not collisions else 'NO'}")
    for ls, ps in collisions[:4]:
        distinct = {}
        for p in ps:
            distinct.setdefault(op_table(p), p)
        names = [p.label() for p in distinct.values()]
        held = [n for (n, _), v in zip(LAWS, ls) if v]
        print(f"  one law set {{{', '.join(held) if held else 'none'}}} is shared by "
              f"{len(distinct)} DIFFERENT algebras, e.g. {names[:4]}")
    print()

    # TEST 3. Can the law set be varied while every other coordinate is held
    # fixed? This is the direct test of "is it a coordinate".
    print("TEST 3 can a law set be varied with every other coordinate fixed?")
    seen = {}
    free = 0
    for p in configs:
        k = p.key()
        if k in seen and law_set(seen[k]) != law_set(p):
            free += 1
        seen[k] = p
    print(f"  configurations sharing all other coordinates but differing in law set: {free}")
    print(f"  verdict: law set is {'FREE (a coordinate)' if free else 'DETERMINED (a projection)'}")
    print()

    # TEST 4. The other reading: a law set as a DEMAND. Then it is a query over
    # the space rather than a coordinate of a point in it. Show what it selects.
    print("TEST 4 law set read as a demand rather than a component:")
    for demand in [
        ("add_assoc",),
        ("add_assoc", "distrib_add"),
        ("add_assoc", "mul_assoc", "distrib_add"),
        ("distrib_sub",),
    ]:
        idx = [i for i, (n, _) in enumerate(LAWS) if n in demand]
        sat = [p for p in configs if all(law_set(p)[i] for i in idx)]
        feats = set()
        for p in sat:
            feats.add((f"F={p.F}", p.policy, p.rounding, "signed" if p.signed else "unsigned"))
        fs = sorted({f[0] for f in feats})
        ps = sorted({f[1] for f in feats})
        print(f"  demand {str(demand):<45} satisfied by {len(sat):>3}/{len(configs)} configs; "
              f"F in {{{','.join(fs) if fs else '-'}}}, policy in {{{','.join(ps) if ps else '-'}}}")
    print()

    # A readable slice of the actual law table, so the numbers above are
    # inspectable rather than asserted.
    print("law table, unsigned, radix 2, nearest rounding, W=4:")
    hdr = "  " + " ".join(f"{n[:11]:>11}" for n, _ in LAWS)
    print(f"  {'config':<20}" + hdr)
    for F in [0, 1, 2]:
        for policy in ["sat", "wrap"]:
            p = Prim(4, F, False, policy, "near")
            row = " ".join(f"{('yes' if v else '.'):>11}" for v in law_set(p))
            print(f"  {p.label():<20}   {row}")


if __name__ == "__main__":
    main()
