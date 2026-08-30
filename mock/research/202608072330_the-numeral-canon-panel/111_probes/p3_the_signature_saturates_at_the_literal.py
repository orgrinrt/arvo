#!/usr/bin/env python3
"""P3. The signature does decide the count, and it stops deciding it the moment
the signature can express a literal.

`110` F4 counts the same 288 configurations under five signatures and reports 84
distinct primitives under {add} and 186 under {add,mul}, concluding that "how
many primitives are there" is not well posed until somebody fixes the signature,
and that a canon enumerating axes without fixing a signature has not defined the
thing it is enumerating axes of.

Two things about that are worth separating, and this probe separates them.

**Monotonicity is by construction, not by measurement.** Adding an operation
refines the partition: two primitives separated under a signature stay separated
under any superset of it, because the separating term is still a term. So the
count is non-decreasing in the signature no matter what the arithmetic does, and
"the count more than doubles" is a measurement only of the MAGNITUDE, never of
the direction. This probe checks monotonicity holds in the data, which is a
check on the instrument rather than a finding.

**The count is bounded above, and the bound is reached by one nullary
operation.** Two primitives with the same value set are separated by some term
exactly when their realisation maps differ somewhere that a term can reach. Every
term's argument to R is a rational. So no signature can separate more than "R
differs somewhere on Q", and a signature containing a constant injection over Q
reaches that bound at depth one. If that holds, then:

    the identity relation SATURATES at the signature {literal}, and no
    operation added afterwards can split anything further.

which would deflate F4's practical bite considerably. arvo has a literal by I3's
ergonomic imitation of a native Rust primitive, so it is already at the bound and
does not have to declare a closed signature to have stable identity. It has to
have a constructor, which it cannot avoid.

Prediction recorded before running: the count under {literal} equals the count
under {literal, add, sub, mul, neg, half, recip, fma}, and both are greater than
or equal to every count without the literal.
"""

from fractions import Fraction
from itertools import product

# --------------------------------------------------------------- the model


class Prim:
    def __init__(self, W, F, signed, policy, rounding, radix):
        self.W, self.F, self.signed = W, F, signed
        self.policy, self.rounding, self.radix = policy, rounding, radix
        n = 1 << W
        self.ints = list(range(-(n // 2), n // 2)) if signed else list(range(0, n))
        self.step = Fraction(1, radix ** F)
        self.values = [Fraction(k) * self.step for k in self.ints]
        self.lo, self.hi = min(self.values), max(self.values)

    def name(self):
        s = "i" if self.signed else "u"
        return (f"{s}W{self.W}F{self.F}r{self.radix}/"
                f"{self.policy}/{self.rounding}")

    def R(self, q):
        k = q / self.step
        n, d = k.numerator, k.denominator
        fl = n // d
        if self.rounding == "trunc":
            g = Fraction(int(k)) if k >= 0 else -Fraction(int(-k))
        elif self.rounding == "floor":
            g = Fraction(fl)
        else:
            rem = k - fl
            g = Fraction(fl + 1) if rem > Fraction(1, 2) else (
                Fraction(fl) if rem < Fraction(1, 2) else
                Fraction(fl if fl % 2 == 0 else fl + 1))
        v = g * self.step
        if self.lo <= v <= self.hi:
            return v
        if self.policy == "sat":
            return self.hi if v > self.hi else self.lo
        span = self.hi - self.lo + self.step
        return ((v - self.lo) % span) + self.lo

    # ---- operations, each landing its exact result back through R
    def add(self, a, b):
        return self.R(a + b)

    def sub(self, a, b):
        return self.R(a - b)

    def mul(self, a, b):
        return self.R(a * b)

    def neg(self, a):
        return self.R(-a)

    def half(self, a):
        return self.R(a / 2)

    def recip(self, a):
        return self.R(Fraction(1) / a) if a != 0 else self.R(Fraction(0))

    def fma(self, a, b):
        return self.R(a * b + a)


BINARY = {"add", "sub", "mul", "fma"}
UNARY = {"neg", "half", "recip"}


def literal_arguments(p):
    """The rationals a literal can name, sampled so every grid cell is probed at
    its interior and at its midpoint, plus points outside the range so the
    policy is probed too.

    The sampling is principled rather than dense-and-hopeful: R is constant on
    the interior of each grid cell and can only change at a cell boundary or at
    the range limits, so a sample containing each cell's midpoint, each cell's
    two quarter points and every boundary determines R on Q for this family."""
    out = []
    lo_k = (p.lo / p.step).numerator
    hi_k = (p.hi / p.step).numerator
    for k in range(lo_k - 2, hi_k + 3):
        base = Fraction(k) * p.step
        out.append(base)
        out.append(base + p.step / 2)
        out.append(base + p.step / 4)
        out.append(base + p.step * Fraction(3, 4))
    return out


def table(p, ops):
    """The interpretation of the signature. Carrier elements are free variables,
    exactly as in 110's probe, and `literal` is the nullary operation whose
    arguments are the rationals a consumer can write."""
    t = [tuple(p.values)]
    for name in sorted(ops):
        if name == "literal":
            t.append(tuple(p.R(q) for q in literal_arguments(p)))
        elif name in UNARY:
            t.append(tuple(getattr(p, name)(a) for a in p.values))
        else:
            t.append(tuple(getattr(p, name)(a, b)
                           for a, b in product(p.values, repeat=2)))
    return tuple(t)


SIGNATURES = [
    ("{add}", {"add"}),
    ("{add,sub}", {"add", "sub"}),
    ("{add,mul}", {"add", "mul"}),
    ("{add,sub,mul,neg}", {"add", "sub", "mul", "neg"}),
    ("{add,sub,mul,neg,half}", {"add", "sub", "mul", "neg", "half"}),
    ("{add,sub,mul,neg,half,recip,fma}",
     {"add", "sub", "mul", "neg", "half", "recip", "fma"}),
    ("{literal}", {"literal"}),
    ("{literal,add}", {"literal", "add"}),
    ("{literal,add,sub,mul,neg,half,recip,fma}",
     {"literal", "add", "sub", "mul", "neg", "half", "recip", "fma"}),
]


def main():
    configs = []
    for W, F, signed, policy, rounding, radix in product(
        [2, 3, 4], range(0, 3), [False, True], ["sat", "wrap"],
        ["near", "trunc", "floor"], [2, 3]
    ):
        if F > W:
            continue
        configs.append(Prim(W, F, signed, policy, rounding, radix))

    print("P3. does the signature keep deciding the count?")
    print("=" * 78)
    print(f"configurations swept: {len(configs)}")
    print("  W in {2,3,4}, F in 0..=2, signedness any, policy in {sat,wrap},")
    print("  rounding in {near,trunc,floor}, radix in {2,3}")
    print()
    print(f"  {'signature':<42} {'primitives':>10}")

    partitions = {}
    for label, ops in SIGNATURES:
        classes = {}
        for p in configs:
            classes.setdefault(table(p, ops), []).append(p)
        partitions[label] = classes
        print(f"  {label:<42} {len(classes):>10}")

    # ---- check 1: monotone in the signature. This is by construction and is
    # checked as an instrument check, not reported as a finding.
    print()
    print("instrument check, monotone under signature extension:")
    chain = ["{add}", "{add,sub}", "{add,sub,mul,neg}",
             "{add,sub,mul,neg,half}", "{add,sub,mul,neg,half,recip,fma}"]
    ok = all(len(partitions[chain[i]]) <= len(partitions[chain[i + 1]])
             for i in range(len(chain) - 1))
    print(f"  {' <= '.join(str(len(partitions[c])) for c in chain)}   {'ok' if ok else 'VIOLATED'}")

    # ---- check 2: does the literal saturate?
    print()
    print("the saturation claim:")
    lit = len(partitions["{literal}"])
    lit_all = len(partitions["{literal,add,sub,mul,neg,half,recip,fma}"])
    lit_add = len(partitions["{literal,add}"])
    print(f"  {{literal}}                                       {lit}")
    print(f"  {{literal,add}}                                   {lit_add}")
    print(f"  {{literal, everything}}                           {lit_all}")
    print(f"  adding every operation to the literal splits nothing: "
          f"{lit == lit_all == lit_add}")

    # the stronger form: the partitions are the same, not merely the same size
    same = (set(map(frozenset, (tuple(sorted(p.name() for p in v))
                                for v in partitions["{literal}"].values())))
            == set(map(frozenset, (tuple(sorted(p.name() for p in v))
                                   for v in partitions["{literal,add,sub,mul,neg,half,recip,fma}"].values()))))
    print(f"  and the partitions are identical, not merely equinumerous: {same}")

    biggest_without = max(len(partitions[l]) for l, _ in SIGNATURES
                          if "literal" not in l)
    print(f"  finest signature without a literal reaches               {biggest_without}")
    print(f"  the literal alone reaches                                {lit}")

    # ---- check 3: what the literal separates that the operations do not
    print()
    print("what the literal separates that {add,sub,mul,neg,half,recip,fma} does not:")
    coarse = partitions["{add,sub,mul,neg,half,recip,fma}"]
    fine = partitions["{literal}"]
    fine_of = {}
    for key, ps in fine.items():
        for p in ps:
            fine_of[p.name()] = key
    shown = 0
    for key, ps in coarse.items():
        keys = {fine_of[p.name()] for p in ps}
        if len(keys) > 1 and shown < 6:
            shown += 1
            names = sorted(p.name() for p in ps)
            print(f"  one class of {len(ps)} splits into {len(keys)}: "
                  f"{names[0]} ... {names[-1]}")
    if shown == 0:
        print("  nothing: the two partitions agree")


    # ---- check 4: cross-check against p4. p4 reports that radix at F = 0 is
    # unobservable at 0 of 108 pairs even with rational constants. If the
    # literal here split any pair differing ONLY in radix at F = 0, the two
    # probes would disagree and one of them would be wrong.
    print()
    print("cross-check against p4, radix-only pairs at F = 0:")
    fine_key = {}
    for key, ps in partitions["{literal}"].items():
        for q in ps:
            fine_key[q.name()] = key
    split = same_cls = 0
    for a in configs:
        for b in configs:
            if a is b or a.F != 0 or b.F != 0:
                continue
            if (a.W, a.signed, a.policy, a.rounding) != (b.W, b.signed, b.policy, b.rounding):
                continue
            if a.radix == b.radix:
                continue
            if fine_key[a.name()] == fine_key[b.name()]:
                same_cls += 1
            else:
                split += 1
                print(f"  SPLIT {a.name()} vs {b.name()}")
    print(f"  radix-only pairs at F = 0: {same_cls + split}, "
          f"in the same class {same_cls}, split {split}")
    print(f"  agrees with p4's 0 of 108: {split == 0}")

    print()
    print("-" * 78)
    print("reading:")
    print("  F4 is right that the count is a function of the signature, and the")
    print("  function is constant above the literal. A design that can write a")
    print("  literal is already at the finest identity its realisation map")
    print("  supports, so it does not have to declare a closed operation set to")
    print("  have stable identity. It has to have a constructor, which it cannot")
    print("  avoid having.")


if __name__ == "__main__":
    main()
