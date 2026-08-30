#!/usr/bin/env python3
"""P3. What a name buys, and what it costs when it is not canonical.

The working assumption says a primitive is a NAMED COMPOSITION. A composition
of axes gives one name per point in the product space. That is only useful if
the naming is a bijection onto the primitives: one name per primitive, one
primitive per name.

P2 already showed it is not. 48 swept configurations produced 40 distinct
algebras, so 8 names were second names for something already named. This probe
measures the gap properly, over a wider space and under several signatures, and
then asks the question that actually matters for a design:

  is the collapse ARBITRARY, or is there a rule?

If arbitrary, the design must ship an explicit equality relation between names
and no canonical form is computable. If there is a rule, the rule is a
canonicalisation function on the axes, and if it is computable from the axes
alone it runs at const time, which is the only binding time this design has
(I15).

Method:
  1. Sweep a configuration space. Compute each configuration's full operation
     table, which is its algebra up to denotation-preserving isomorphism.
  2. Count names against primitives, under five signatures of increasing size.
  3. Extract the collapse classes and read a rule off them.
  4. Test the rule as a PREDICTOR on a disjoint held-out sweep, counting both
     error directions separately, because they are not equally bad: merging two
     different primitives is unsound and splitting one is merely wasteful.

Two candidate rules are kept, the first of which is WRONG. It is kept because
the way it fails is itself the result.
"""

from fractions import Fraction
from itertools import product

ROUNDINGS = ["near", "trunc", "floor"]
AXIS_NAMES = ["W", "F", "signed", "policy", "rounding", "radix"]


class Prim:
    def __init__(self, W, F, signed, policy, rounding, radix):
        self.W, self.F, self.signed = W, F, signed
        self.policy, self.rounding, self.radix = policy, rounding, radix
        n = 1 << W
        self.ints = list(range(-(n // 2), n // 2)) if signed else list(range(0, n))
        self.step = Fraction(1, radix ** F)
        self.values = [Fraction(k) * self.step for k in self.ints]

    def axes(self):
        return (self.W, self.F, self.signed, self.policy, self.rounding, self.radix)

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

    def add(self, a, b):
        return self.R(a + b)

    def sub(self, a, b):
        return self.R(a - b)

    def mul(self, a, b):
        return self.R(a * b)

    def neg(self, a):
        return self.R(-a)

    def le(self, a, b):
        return a <= b


SIGNATURES = {
    "{add}": ["add"],
    "{add,sub}": ["add", "sub"],
    "{add,mul}": ["add", "mul"],
    "{add,sub,mul,neg}": ["add", "sub", "mul", "neg"],
    "{add,sub,mul,neg,le}": ["add", "sub", "mul", "neg", "le"],
}


def algebra(p, sig):
    """The interpretation restricted to a signature. Two configurations with
    equal tables are the same primitive over that signature."""
    parts = [tuple(p.values)]
    for op in sig:
        if op in ("add", "sub", "mul", "le"):
            f = getattr(p, op)
            parts.append(tuple(f(a, b) for a, b in product(p.values, repeat=2)))
        else:
            f = getattr(p, op)
            parts.append(tuple(f(a) for a in p.values))
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


def canon_v1(p):
    """First candidate, read off the F = 0 collapses and nothing else.

    Kept after being falsified, because the way it fails is the result: a
    canonicalisation fitted to the collapses somebody noticed is not the
    canonicalisation, and asserting it is the only way to find out.
    """
    rounding = p.rounding if p.F > 0 else "*"   # nothing to round onto a step of 1
    radix = p.radix if p.F > 0 else "*"         # radix^0 == 1 for every radix
    return (p.W, p.F, p.signed, p.policy, rounding, radix)


def canon_v2(p):
    """Second candidate, after v1 was falsified at uW2F1r2/sat/{trunc,floor}.

    The missed clause: truncation toward zero and floor are the same map when
    no value is negative, so they collapse on every unsigned primitive at every
    F, not only at F = 0. v1 saw the F = 0 collapse because it is the loud one,
    and missed a collapse conditioned on a DIFFERENT axis than the one being
    collapsed. That is the general shape of what a hand-read rule misses.
    """
    if p.F == 0:
        return (p.W, p.F, p.signed, p.policy, "*", "*")
    rounding = p.rounding
    if not p.signed and rounding in ("trunc", "floor"):
        rounding = "downward(unsigned)"
    return (p.W, p.F, p.signed, p.policy, rounding, p.radix)


def score(rule, pop, sig):
    """Count both error directions of a candidate canonicalisation."""
    alg = {p.label(): algebra(p, sig) for p in pop}
    can = {p.label(): rule(p) for p in pop}
    merges, splits, examples = 0, 0, []
    for i in range(len(pop)):
        for j in range(i + 1, len(pop)):
            a, b = pop[i].label(), pop[j].label()
            same_rule, same_alg = can[a] == can[b], alg[a] == alg[b]
            if same_rule and not same_alg:
                merges += 1
                if len(examples) < 3:
                    examples.append(("MERGES two different primitives", a, b))
            elif same_alg and not same_rule:
                splits += 1
                if len(examples) < 3:
                    examples.append(("SPLITS one primitive in two", a, b))
    return merges, splits, examples


def main():
    print("P3. naming a composition, and whether the naming is canonical")
    print("=" * 78)

    configs = sweep([2, 3, 4], [2, 3])
    print(f"names in the swept product space: {len(configs)}")
    print("  W in {2,3,4}, F in 0..=W, signed in {no,yes}, policy in {sat,wrap},")
    print(f"  rounding in {{{','.join(ROUNDINGS)}}}, radix in {{2,3}}")
    print()

    print("how many DISTINCT primitives those names denote, per signature:")
    print(f"  {'signature':<24} {'primitives':>11} {'names':>7} {'second names':>13}")
    for sname, sig in SIGNATURES.items():
        classes = {}
        for p in configs:
            classes.setdefault(algebra(p, sig), []).append(p)
        print(f"  {sname:<24} {len(classes):>11} {len(configs):>7} "
              f"{len(configs) - len(classes):>13}")
    print()
    print("  the number of primitives is not a property of the axes alone.")
    print("  it moves with the signature, so 'how many primitives are there'")
    print("  is not well posed until the operation set is fixed.")
    print()

    full = SIGNATURES["{add,sub,mul,neg,le}"]
    classes = {}
    for p in configs:
        classes.setdefault(algebra(p, full), []).append(p)
    collapsed = {k: v for k, v in classes.items() if len(v) > 1}
    print(f"collapse classes under the full signature: {len(collapsed)}")
    varying = {}
    for ps in collapsed.values():
        for i, axis in enumerate(AXIS_NAMES):
            vals = {p.axes()[i] for p in ps}
            if len(vals) > 1:
                varying.setdefault(axis, set()).update(map(str, vals))
    print("axes that vary freely inside some collapse class:")
    for axis, vals in sorted(varying.items()):
        print(f"  {axis:<10} over {sorted(vals)}")
    fixed_axes = [a for a in AXIS_NAMES if a not in varying]
    print(f"axes that NEVER vary inside a class: {', '.join(fixed_axes)}")
    print("  those are unconditionally identity-bearing over this sweep.")
    print()

    # ---- stage one: the rule somebody would read off the loud collapses
    at_zero = sum(1 for ps in collapsed.values() if all(p.F == 0 for p in ps))
    print(f"collapse classes entirely at F = 0: {at_zero} of {len(collapsed)}")
    counterexamples = [ps for ps in collapsed.values() if any(p.F != 0 for p in ps)]
    print(f"collapse classes with F > 0:         {len(counterexamples)}")
    for ps in counterexamples[:3]:
        print(f"  counterexample to the F=0-only rule: {[p.label() for p in ps]}")
    print()

    print("scoring the two candidate canonicalisations on the FITTING sweep:")
    for name, rule in [("v1 (F=0 only)", canon_v1), ("v2 (+ unsigned down)", canon_v2)]:
        m, s, ex = score(rule, configs, full)
        print(f"  {name:<22} unsound merges: {m:>4}   conservative splits: {s:>4}")
        for kind, a, b in ex:
            print(f"      {kind}: {a} vs {b}")
    print()

    # ---- stage two: validate on a disjoint held-out sweep
    print("validating on a DISJOINT held-out sweep (W = 5, radix in {2,3,5}):")
    held = sweep([5], [2, 3, 5])
    held_classes = {}
    for p in held:
        held_classes.setdefault(algebra(p, full), []).append(p)
    print(f"  held-out names: {len(held)}, distinct primitives: {len(held_classes)}, "
          f"pairs checked: {len(held) * (len(held) - 1) // 2}")
    for name, rule in [("v1 (F=0 only)", canon_v1), ("v2 (+ unsigned down)", canon_v2)]:
        m, s, ex = score(rule, held, full)
        verdict = "EXACT" if m == 0 and s == 0 else ("UNSOUND" if m else "sound, conservative")
        print(f"  {name:<22} merges: {m:>4}  splits: {s:>4}   verdict: {verdict}")
        for kind, a, b in ex:
            print(f"      {kind}: {a} vs {b}")
    print()
    print("  a canonicalisation that SPLITS is safe and wastes names.")
    print("  a canonicalisation that MERGES is unsound: it hands one name to two")
    print("  primitives that compute different answers, and every consumer that")
    print("  substitutes along that name gets a wrong value rather than a slow one.")


if __name__ == "__main__":
    main()
