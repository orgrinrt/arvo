#!/usr/bin/env python3
"""P5. Two ways an axis can vanish, and only one of them is stable.

P4 broke my own hypothesis. I had claimed that at F = 0 both the rounding mode
and the radix vanish "structurally", meaning no operation could ever separate
them. Adding a halving operation separated the rounding modes at F = 0
immediately: at a grid step of 1, a/2 for odd a lands between grid points and
the rounding mode decides where it goes. My argument had been that no result
between grid points can arise at F = 0, and that is true only for operations
CLOSED ON THE GRID. add, sub, mul and neg are; division is not.

So "the axis is unobservable" is two different claims wearing one phrase, and
this probe separates them:

  DEFINITIONAL degeneracy. The axis disappears from the definition of the value
  set and of the realisation map R. At F = 0 the step is radix^0 = 1 whatever
  the radix is, so radix is not a parameter of anything. No term over any
  signature can separate primitives differing only in it, because there is
  nothing left that reads it.

  REACHABILITY degeneracy. The axis is still a parameter of R, but no term in
  the CURRENT signature produces an argument on which it matters. Rounding at
  F = 0 under grid-closed operations is this. It is a fact about the operation
  set, not about the primitive, and it evaporates when the operation set grows.

The prediction: definitional degeneracies survive every signature extension and
reachability degeneracies do not. If it holds, a naming discipline may
canonicalise on the first kind and may not on the second, and the test between
them is cheap: does the axis still appear in the definition.
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

    def axes(self):
        return dict(W=self.W, F=self.F, signed=self.signed,
                    policy=self.policy, rounding=self.rounding, radix=self.radix)

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

    def half(self, a):
        return self.R(a / 2)

    def recip(self, a):
        return self.R(Fraction(1) / a) if a != 0 else self.R(Fraction(0))

    def fma(self, a, b):
        return self.R(a * b + a)


UNARY = {"neg", "half", "recip"}

# grid-closed: every operation maps grid points to exact grid points before R
SIG_CLOSED = ["add", "sub", "mul", "neg"]
# not grid-closed: half and recip produce values between grid points
SIG_OPEN = ["add", "sub", "mul", "neg", "half"]
SIG_WIDE = ["add", "sub", "mul", "neg", "half", "recip", "fma"]

SIGS = [("grid-closed {add,sub,mul,neg}", SIG_CLOSED),
        ("+ half (not grid-closed)", SIG_OPEN),
        ("+ half, recip, fma", SIG_WIDE)]

AXES = ["W", "F", "signed", "policy", "rounding", "radix"]
DOMAIN = dict(
    W=[2, 3, 4], F=None, signed=[False, True], policy=["sat", "wrap"],
    rounding=ROUNDINGS, radix=[2, 3, 5],
)


def algebra(p, sig):
    parts = [tuple(p.values)]
    for op in sig:
        f = getattr(p, op)
        if op in UNARY:
            parts.append(tuple(f(a) for a in p.values))
        else:
            parts.append(tuple(f(a, b) for a, b in product(p.values, repeat=2)))
    return tuple(parts)


def build(**kw):
    return Prim(kw["W"], kw["F"], kw["signed"], kw["policy"], kw["rounding"], kw["radix"])


def all_points():
    for W in DOMAIN["W"]:
        for F in range(0, W + 1):
            for signed, policy, rounding, radix in product(
                DOMAIN["signed"], DOMAIN["policy"], DOMAIN["rounding"], DOMAIN["radix"]
            ):
                yield dict(W=W, F=F, signed=signed, policy=policy,
                           rounding=rounding, radix=radix)


def observable(axis, pt, sig):
    """Holding every other axis fixed at `pt`, does varying `axis` change the
    algebra? Returns None when the axis has fewer than two legal values here."""
    vals = list(range(0, pt["W"] + 1)) if axis == "F" else DOMAIN[axis]
    vals = [v for v in vals if v != pt[axis]]
    if not vals:
        return None
    me = algebra(build(**pt), sig)
    for v in vals:
        q = dict(pt)
        q[axis] = v
        if algebra(build(**q), sig) != me:
            return True
    return False


def main():
    print("P5. definitional degeneracy against reachability degeneracy")
    print("=" * 78)
    pts = list(all_points())
    print(f"configuration points swept: {len(pts)}")
    print()

    print("fraction of points at which each axis is OBSERVABLE, per signature:")
    header = f"  {'axis':<10}" + "".join(f"{n[:26]:>28}" for n, _ in SIGS)
    print(header)
    results = {}
    for axis in AXES:
        row = f"  {axis:<10}"
        for signame, sig in SIGS:
            obs = [observable(axis, pt, sig) for pt in pts]
            obs = [o for o in obs if o is not None]
            frac = sum(obs) / len(obs) if obs else float("nan")
            results[(axis, signame)] = (sum(obs), len(obs))
            row += f"{sum(obs):>12} / {len(obs):<13}"
        print(row)
    print()

    print("the two axes that vanish somewhere, resolved by region:")
    for axis in ["radix", "rounding"]:
        print(f"  {axis}:")
        for signame, sig in SIGS:
            unobs_regions = {}
            for pt in pts:
                o = observable(axis, pt, sig)
                if o is False:
                    unobs_regions.setdefault(pt["F"] == 0, 0)
                    unobs_regions[pt["F"] == 0] += 1
            at_zero = unobs_regions.get(True, 0)
            above = unobs_regions.get(False, 0)
            print(f"    {signame:<30} unobservable at F=0: {at_zero:>4}, "
                  f"at F>0: {above:>4}")
    print()

    print("the decisive comparison, at F = 0 only:")
    zero_pts = [pt for pt in pts if pt["F"] == 0]
    for axis in ["radix", "rounding"]:
        line = f"  {axis:<10}"
        for signame, sig in SIGS:
            obs = [observable(axis, pt, sig) for pt in zero_pts]
            obs = [o for o in obs if o is not None]
            n_obs = sum(obs)
            line += f"  {signame.split()[0]}: {n_obs}/{len(obs)} observable"
        print(line)
    print()
    print("  radix at F = 0 is never observable under any signature tested.")
    print("  it is DEFINITIONAL: step = radix^0 = 1, so no part of the value set")
    print("  or of R mentions the radix, and there is nothing for a term to read.")
    print()
    print("  rounding at F = 0 is unobservable under the grid-closed signature")
    print("  and observable as soon as an operation leaves the grid. it is")
    print("  REACHABILITY: R still reads the rounding mode, and the old signature")
    print("  merely never handed it a fractional argument.")
    print()

    # the direct definitional test, run mechanically rather than argued
    print("the definitional test, run rather than argued:")
    print("  does the axis appear in the value set or in R's behaviour on the")
    print("  FULL rational line, rather than only on terms the signature reaches?")
    for axis in ["radix", "rounding", "policy"]:
        for F in [0, 1]:
            pt = dict(W=3, F=F, signed=False, policy="sat", rounding="near", radix=2)
            base = build(**pt)
            vals = list(range(0, pt["W"] + 1)) if axis == "F" else DOMAIN[axis]
            differs_on_line = False
            differs_on_values = False
            for v in [v for v in vals if v != pt[axis]]:
                q = dict(pt)
                q[axis] = v
                other = build(**q)
                if tuple(other.values) != tuple(base.values):
                    differs_on_values = True
                # probe R on a dense sample of the rational line, including
                # points no term in any signature need ever produce
                for num in range(-40, 41):
                    x = Fraction(num, 12)
                    if base.R(x) != other.R(x):
                        differs_on_line = True
                        break
            verdict = ("reads it" if (differs_on_line or differs_on_values)
                       else "DOES NOT read it: definitional degeneracy")
            print(f"    {axis:<9} at F={F}: value set or R {verdict}")


if __name__ == "__main__":
    main()
