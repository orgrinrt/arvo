#!/usr/bin/env python3
"""P4. Which collapses are safe to name away, and which are accidents.

P3 left a residue. The best canonicalisation found is sound but conservative:
it never merges two primitives that differ, and it splits a handful that are in
fact identical. The obvious next move is to close the gap by canonicalising on
OBSERVED table equality rather than on structural facts about the axes, which
would be exact by construction.

This probe tests whether that move is safe, and the prediction is that it is
not. Two collapses can look identical and be different in kind:

  STRUCTURAL. The axis is unobservable for a reason that holds for any
  arithmetic operation at all. At F = 0 the grid step is 1, so no result
  between grid points can ever arise and the rounding mode is unreachable;
  radix^0 = 1, so the radix never enters the value set. No operation, present
  or future, can separate these.

  ACCIDENTAL. The axis is unobservable because the particular operations in the
  signature happen not to reach the values that would separate it. Add one more
  operation and the collapse disappears.

If the residue is accidental, then an exact canonicalisation is a function of
the signature, and it silently changes the moment an operation is added: two
types that were one type become two, which is a breaking change nobody edited.
A conservative canonicalisation built only on structural degeneracies does not
move.

Method: take every collapse class under a base signature, extend the signature
with operations built from the same realisation map, and count which classes
survive.
"""

from fractions import Fraction
from itertools import product

ROUNDINGS = ["near", "trunc", "floor"]


class Prim:
    def __init__(self, W, F, signed, policy, rounding, radix):
        self.W, self.F, self.signed = W, F, signed
        self.policy, self.rounding, self.radix = policy, rounding, radix
        n = 1 << W
        self.ints = list(range(-(n // 2), n // 2)) if signed else list(range(0, n))
        self.step = Fraction(1, radix ** F)
        self.values = [Fraction(k) * self.step for k in self.ints]

    def label(self):
        s = "i" if self.signed else "u"
        return f"{s}W{self.W}F{self.F}r{self.radix}/{self.policy}/{self.rounding}"

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

    # base signature
    def add(self, a, b):
        return self.R(a + b)

    def sub(self, a, b):
        return self.R(a - b)

    def mul(self, a, b):
        return self.R(a * b)

    def neg(self, a):
        return self.R(-a)

    # extensions, each built from the SAME realisation map, so none of them is
    # a new mechanism. They only reach values the base four do not.
    def half(self, a):
        return self.R(a / 2)

    def third(self, a):
        return self.R(a / 3)

    def recip(self, a):
        return self.R(Fraction(1, 1) / a) if a != 0 else self.R(Fraction(0))

    def fma(self, a, b):
        # single-rounded a*b + a, which nests two exact ops under one R
        return self.R(a * b + a)

    def scale_by_step(self, a):
        return self.R(a * self.step)


UNARY = {"neg", "half", "third", "recip", "scale_by_step"}
BASE = ["add", "sub", "mul", "neg"]
EXTENSIONS = [
    ("+ half", ["half"]),
    ("+ third", ["third"]),
    ("+ recip", ["recip"]),
    ("+ fma", ["fma"]),
    ("+ scale_by_step", ["scale_by_step"]),
    ("+ all five", ["half", "third", "recip", "fma", "scale_by_step"]),
]


def algebra(p, sig):
    parts = [tuple(p.values)]
    for op in sig:
        f = getattr(p, op)
        if op in UNARY:
            parts.append(tuple(f(a) for a in p.values))
        else:
            parts.append(tuple(f(a, b) for a, b in product(p.values, repeat=2)))
    return tuple(parts)


def sweep(widths, radices):
    out = []
    for W in widths:
        for F in range(0, W + 1):
            for signed, policy, rounding, radix in product(
                [False, True], ["sat", "wrap"], ROUNDINGS, radices
            ):
                out.append(Prim(W, F, signed, policy, rounding, radix))
    return out


def classify(ps):
    """Is this collapse class structural or accidental, by the F = 0 test?"""
    return "structural" if all(p.F == 0 for p in ps) else "accidental?"


def main():
    print("P4. do the collapses survive a wider signature?")
    print("=" * 78)
    pop = sweep([2, 3, 4], [2, 3])
    print(f"population: {len(pop)} names")
    print()

    base_classes = {}
    for p in pop:
        base_classes.setdefault(algebra(p, BASE), []).append(p)
    collapsed = [ps for ps in base_classes.values() if len(ps) > 1]
    struct = [ps for ps in collapsed if classify(ps) == "structural"]
    accid = [ps for ps in collapsed if classify(ps) != "structural"]
    print(f"under the base signature {BASE}:")
    print(f"  primitives: {len(base_classes)} from {len(pop)} names")
    print(f"  collapse classes: {len(collapsed)}  "
          f"(structural, all F=0: {len(struct)};  other: {len(accid)})")
    print()

    print("how many collapse classes SURVIVE each signature extension:")
    print(f"  {'extension':<20} {'structural survive':>19} {'other survive':>15} "
          f"{'primitives':>11}")
    for name, ext in EXTENSIONS:
        sig = BASE + ext
        s_alive = sum(1 for ps in struct
                      if len({algebra(p, sig) for p in ps}) == 1)
        a_alive = sum(1 for ps in accid
                      if len({algebra(p, sig) for p in ps}) == 1)
        classes = {algebra(p, sig) for p in pop}
        print(f"  {name:<20} {s_alive:>7} / {len(struct):<9} "
              f"{a_alive:>6} / {len(accid):<6} {len(classes):>11}")
    print()

    sig_all = BASE + EXTENSIONS[-1][1]
    print("classes broken by the widest signature, with the operation that broke them:")
    shown = 0
    for ps in collapsed:
        if len({algebra(p, sig_all) for p in ps}) == 1:
            continue
        culprits = []
        for opname in EXTENSIONS[-1][1]:
            if len({algebra(p, BASE + [opname]) for p in ps}) > 1:
                culprits.append(opname)
        if shown < 8:
            print(f"  {[p.label() for p in ps]}")
            print(f"      broken by: {', '.join(culprits) if culprits else 'a combination only'}")
        shown += 1
    print(f"  ... {shown} classes broken in total")
    print()

    survivors = [ps for ps in collapsed if len({algebra(p, sig_all) for p in ps}) == 1]
    print(f"classes surviving every extension: {len(survivors)}")
    fs = {p.F for ps in survivors for p in ps}
    print(f"  fraction widths present among survivors: {sorted(fs)}")
    if fs == {0}:
        print("  every survivor is at F = 0, which is the structural degeneracy:")
        print("  a step of 1 makes the rounding mode unreachable and radix^0 = 1")
        print("  makes the radix absent from the value set. No operation built")
        print("  from this realisation map can separate them, because there is")
        print("  no value between two grid points for any of them to produce.")
    else:
        print("  NOT all survivors are at F = 0, so the structural test above is")
        print("  incomplete and the rule needs another clause.")
    print()

    broke = [ps for ps in accid if len({algebra(p, sig_all) for p in ps}) > 1]
    print(f"of the {len(accid)} non-structural collapse classes, {len(broke)} were")
    print("broken by adding operations that use the same realisation map.")
    print()
    print("consequence for a naming discipline:")
    print("  canonicalising on observed table equality would have merged those")
    print(f"  {len(broke)} classes under the base signature and been forced to")
    print("  un-merge them when an operation was added. Two types that were one")
    print("  type become two, with nothing in the design edited to cause it.")
    print("  Canonicalising only on the structural degeneracy is stable, at the")
    print(f"  price of leaving {len(accid) - len(broke)} genuine collapses unnamed.")


if __name__ == "__main__":
    main()
