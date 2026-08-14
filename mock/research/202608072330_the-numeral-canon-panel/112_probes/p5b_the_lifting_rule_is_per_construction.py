#!/usr/bin/env python3
"""
p5b. The repair for p5's unsound cells: a per-construction lifting rule.

WHAT p5 FOUND
-------------
The grade lifts through a construction, and the LIFTING RULE is not the base's
rule.  Using the componentwise rule `b*b <= hi` for complex multiplication is
UNSOUND: 3 of 16 pairs at extent <= 1 and 26 of 81 at extent <= 2 disagree
with the exact composite while the rule says the bound discharged.  `dual` is
unsound at 1 of 81.

That is the same structural fact `110` F11 reports from the law side, that the
componentwise product preserves its base's law set exactly while the twisted
constructions do not, arriving on the grade side.  It is also a real hazard for
a design that carries a refinement: a grade lifted by the wrong rule licenses
an arm that computes a different answer.

THE REPAIR, DERIVED RATHER THAN GUESSED
---------------------------------------
Read off each construction's multiplication:

  product2  (a0,a1)*(b0,b1) = (a0*b0, a1*b1)
            worst magnitude per component: |a|*|b|.  Rule: b^2 <= hi.

  dual      (a0,a1)*(b0,b1) = (a0*b0, a0*b1 + a1*b0)
            the second component sums two products.  Rule: 2*b^2 <= hi.

  complex   (a0,a1)*(b0,b1) = (a0*b0 - a1*b1, a0*b1 + a1*b0)
            both components combine two products.  Rule: 2*b^2 <= hi, and the
            first component also needs its NEGATIVE reachable, so over an
            unsigned base a0*b0 - a1*b1 can go below zero and no magnitude
            bound alone discharges it.

  interval  (a0,a1)*(b0,b1) = hull of the four corner products.
            Rule: b^2 <= hi, plus the base must be monotone ON THE EXTENT.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. Each construction under ITS OWN rule: unsound 0 at every extent swept.
P2. Each construction under the WRONG rule (the componentwise one): unsound
    > 0 for complex and for dual, reproducing p5.
P3. Complex over an UNSIGNED base is unsound at every non-trivial extent under
    any magnitude-only rule, because the real part is a difference and the
    extent cannot bound it below.  Over a SIGNED base it is dischargeable.
P4. So the canon sentence is that a construction carries its own grade
    transformer, not that a grade transforms through a construction.
"""

from fractions import Fraction
from itertools import product


class Base:
    def __init__(self, W, F, signed, policy):
        self.W, self.F, self.signed, self.policy = W, F, signed, policy
        self.step = Fraction(1, 2**F)
        if signed:
            self.klo, self.khi = -(2 ** (W - 1)), 2 ** (W - 1) - 1
        else:
            self.klo, self.khi = 0, 2**W - 1
        self.lo, self.hi = self.klo * self.step, self.khi * self.step

    def __repr__(self):
        return f"{'s' if self.signed else 'u'}W{self.W}{self.policy}"

    def values(self):
        return [k * self.step for k in range(self.klo, self.khi + 1)]

    def R(self, q):
        k = int(q / self.step) if q >= 0 else -int(-q / self.step)
        span = self.khi - self.klo + 1
        if self.policy == "sat":
            k = min(max(k, self.klo), self.khi)
        else:
            k = self.klo + (k - self.klo) % span
        return k * self.step

    def add(self, a, b):
        return self.R(a + b)

    def sub(self, a, b):
        return self.R(a - b)

    def mul(self, a, b):
        return self.R(a * b)


def m_product2(B, x, y):
    return (B.mul(x[0], y[0]), B.mul(x[1], y[1]))


def e_product2(x, y):
    return (x[0] * y[0], x[1] * y[1])


def m_dual(B, x, y):
    return (B.mul(x[0], y[0]), B.add(B.mul(x[0], y[1]), B.mul(x[1], y[0])))


def e_dual(x, y):
    return (x[0] * y[0], x[0] * y[1] + x[1] * y[0])


def m_complex(B, x, y):
    return (
        B.sub(B.mul(x[0], y[0]), B.mul(x[1], y[1])),
        B.add(B.mul(x[0], y[1]), B.mul(x[1], y[0])),
    )


def e_complex(x, y):
    return (x[0] * y[0] - x[1] * y[1], x[0] * y[1] + x[1] * y[0])


CTORS = {
    "product2": (m_product2, e_product2),
    "dual": (m_dual, e_dual),
    "complex": (m_complex, e_complex),
}

RULES = {
    "componentwise": lambda B, b: b * b <= B.hi,
    "twice-componentwise": lambda B, b: 2 * b * b <= B.hi,
    "twice-and-signed-below": lambda B, b: 2 * b * b <= B.hi and -(b * b) >= B.lo,
}


def score(B, ctor, rule, bound):
    m, e = CTORS[ctor]
    ext = [v for v in B.values() if 0 <= v <= bound]
    if len(ext) < 1:
        return None
    pts = [(a, c) for a in ext for c in ext]
    says = RULES[rule](B, bound)
    unsound = n = 0
    for x in pts:
        for y in pts:
            n += 1
            if says and m(B, x, y) != e(x, y):
                unsound += 1
    return says, unsound, n


def main():
    print("=" * 78)
    print("p5b. The lifting rule is per construction")
    print("=" * 78)

    for B in [Base(3, 0, False, "sat"), Base(4, 0, True, "sat")]:
        print()
        print(f"BASE {B}  (range {B.lo} .. {B.hi})")
        print()
        print(
            f"  {'ctor':<10} {'rule':<24} {'bound':>6} {'says':>9} {'unsound':>12}"
        )
        for ctor in ("product2", "dual", "complex"):
            for rule in ("componentwise", "twice-componentwise", "twice-and-signed-below"):
                for bound in (1, 2, 3):
                    r = score(B, ctor, rule, Fraction(bound))
                    if r is None:
                        continue
                    says, u, n = r
                    flag = "  <== UNSOUND" if (says and u > 0) else ""
                    print(
                        f"  {ctor:<10} {rule:<24} {bound:>6} "
                        f"{('discharge' if says else 'refuse'):>9} "
                        f"{str(u) + '/' + str(n):>12}{flag}"
                    )
            print()

    print("SUMMARY: the smallest rule that is sound for each construction")
    print()
    for B in [Base(3, 0, False, "sat"), Base(4, 0, True, "sat")]:
        for ctor in ("product2", "dual", "complex"):
            best = None
            for rule in ("componentwise", "twice-componentwise", "twice-and-signed-below"):
                bad = 0
                fired = 0
                for bound in (1, 2, 3):
                    r = score(B, ctor, rule, Fraction(bound))
                    if r is None:
                        continue
                    says, u, n = r
                    if says:
                        fired += 1
                        bad += u
                if bad == 0 and fired > 0 and best is None:
                    best = rule
            print(
                f"  base {str(B):<10} {ctor:<10} smallest sound rule: "
                f"{best if best else 'NONE of the three fires soundly'}"
            )
        print()

    print("INSTRUMENT CHECK")
    print()
    says, u, n = score(Base(3, 0, False, "sat"), "complex", "componentwise", Fraction(2))
    print(f"  the known-bad cell reproduces: complex/componentwise/2 -> {u}/{n}")
    says2, u2, n2 = score(
        Base(3, 0, False, "sat"), "product2", "componentwise", Fraction(2)
    )
    print(f"  the known-good cell reproduces: product2/componentwise/2 -> {u2}/{n2}")
    print(f"  the scorer separates them: {u > 0 and u2 == 0}")


if __name__ == "__main__":
    main()
