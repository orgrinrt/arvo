#!/usr/bin/env python3
"""
p4. Does `114`'s arm W1 survive a nonzero fraction width, and if not, where
    exactly does it stop.

WHY THIS QUESTION
-----------------
`114`'s arms W0 and W1 rest on F114-1: a wrapping realisation map is a ring
homomorphism.  Every sweep establishing them carries `F = 0`, and `114`
section 5.4 names `F > 0` as one of three things that would decide the file
against itself, saying only that it expects the fraction grid to be additive
and that this is "an expectation and not a result".

`112` F112-4 measured something that bears on it directly and neither file
connects them: the realisation map has two regions, and a magnitude bound
switches off the completion while a grid bound switches off the rounding.
The homomorphism question is therefore not one question.  It is asked of each
region separately, and the two regions can answer differently.

THE HYPOTHESIS
--------------
At `F > 0` the completion region is still modular and still a homomorphism,
and the ROUNDING region is not one for multiplication, because a product of
two grid values lands on the finer grid `s^2` and has to be requantised, and
requantisation does not commute with reduction.  Addition and subtraction are
exact on the grid, so they never enter the rounding region at all.

So the prediction is that arm W1 splits along the OPERATION rather than
failing outright: it survives at `F > 0` for a signature of {add, sub} and
fails there the moment a multiplication appears.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. At `F = 0` under wrap the map is a homomorphism for add, sub and mul,
    reproducing `114` F114-1.  This is the control: if it fails, my model is
    not the same object theirs is and nothing below transfers.
P2. At `F > 0` under wrap the map is a homomorphism for add and sub.
P3. At `F > 0` under wrap the map is NOT a homomorphism for mul.
P4. Consequently the root-only check at `F > 0` under wrap is sound on terms
    over {add, sub} and unsound on terms containing a multiplication.
P5. The failure is the rounding region rather than the completion region, so
    a declaration whose GRID part discharges (operands on a coarse enough
    grid that the product needs no requantisation) restores it.  That is
    `112` F112-4's grid part doing at `F > 0` what the magnitude part does at
    `F = 0`.

CONDITION-CAN-FIRE CHECK
------------------------
P1 is the control for the whole file: an `F = 0` row that is not a
homomorphism would mean the model is wrong rather than that arm W1 is bounded.
And the P4 sweep reports how many cells the root-only and per-node checks
disagree on, because a zero unsound count over zero disagreements says nothing.
"""

from fractions import Fraction
from itertools import product


class Fx:
    """A fixed-point primitive in raw integer units with an explicit scale."""

    def __init__(self, W, F, signed, policy, rounding="trunc"):
        self.W, self.F, self.signed, self.policy, self.rounding = W, F, signed, policy, rounding
        self.s = Fraction(1, 2**F)
        if signed:
            self.klo, self.khi = -(2 ** (W - 1)), 2 ** (W - 1) - 1
        else:
            self.klo, self.khi = 0, 2**W - 1
        self.n = self.khi - self.klo + 1
        self.lo, self.hi = self.klo * self.s, self.khi * self.s

    def __repr__(self):
        return f"{'i' if self.signed else 'u'}W{self.W}F{self.F}/{self.policy}"

    def values(self):
        return [k * self.s for k in range(self.klo, self.khi + 1)]

    def quantise(self, q):
        """The rounding region: land an exact rational on the grid."""
        x = q / self.s
        if self.rounding == "trunc":
            return int(x) if x >= 0 else -int(-x)
        if self.rounding == "floor":
            return x.numerator // x.denominator
        raise ValueError(self.rounding)

    def complete(self, k):
        """The completion region: land a grid index in the container."""
        if self.policy == "sat":
            return min(max(k, self.klo), self.khi)
        if self.policy == "wrap":
            return ((k - self.klo) % self.n) + self.klo
        raise ValueError(self.policy)

    def R(self, q):
        return self.complete(self.quantise(q)) * self.s

    def on_grid(self, q):
        return (q / self.s).denominator == 1


def ex(op, a, b):
    if op == "add":
        return a + b
    if op == "sub":
        return a - b
    if op == "mul":
        return a * b
    raise ValueError(op)


def hom_check(P, op, reach=3):
    """R(R(a) op R(b)) == R(a op b) over an ambient span of grid points."""
    span = [
        k * P.s for k in range(P.klo * reach, P.khi * reach + 1)
    ]
    bad = n = 0
    wit = []
    for a in span:
        for b in span:
            n += 1
            lhs = P.R(ex(op, P.R(a), P.R(b)))
            rhs = P.R(ex(op, a, b))
            if lhs != rhs:
                bad += 1
                if len(wit) < 2:
                    wit.append((a, b, lhs, rhs))
    return bad, n, wit


# ---------------------------------------------------------------------------


def terms(ops, nslots):
    names = [chr(ord("x") + i) for i in range(nslots)]
    if nslots == 2:
        return [("op", o, ("leaf", "x"), ("leaf", "y")) for o in ops]
    out = []
    for o1 in ops:
        for o2 in ops:
            out.append(("op", o1, ("op", o2, ("leaf", "x"), ("leaf", "y")), ("leaf", "z")))
    return out


def lv(t):
    return {t[1]} if t[0] == "leaf" else lv(t[2]) | lv(t[3])


def has_mul(t):
    if t[0] == "leaf":
        return False
    return t[1] == "mul" or has_mul(t[2]) or has_mul(t[3])


def ev_exact(t, env):
    return env[t[1]] if t[0] == "leaf" else ex(t[1], ev_exact(t[2], env), ev_exact(t[3], env))


def ev_pernode(P, t, env):
    return (
        env[t[1]]
        if t[0] == "leaf"
        else P.R(ex(t[1], ev_pernode(P, t[2], env), ev_pernode(P, t[3], env)))
    )


def corner(t, g):
    if t[0] == "leaf":
        return g[t[1]]
    a, b = corner(t[2], g), corner(t[3], g)
    cs = [ex(t[1], u, v) for u in a for v in b]
    return (min(cs), max(cs))


def sweep(P, ts, label):
    """Root-only soundness, split by whether the term contains a mul."""
    stats = {True: [0, 0, 0], False: [0, 0, 0]}  # cells, unsound, checks-differ
    for t in ts:
        names = sorted(lv(t))
        hm = has_mul(t)
        for bs in product(range(P.klo, P.khi + 1), repeat=len(names)):
            g = {}
            ok_decl = True
            for nm, b in zip(names, bs):
                lo, hi = (0, b * P.s) if b >= 0 else (b * P.s, 0)
                if lo > hi:
                    ok_decl = False
                g[nm] = (lo, hi)
            if not ok_decl:
                continue
            doms = [[v for v in P.values() if g[nm][0] <= v <= g[nm][1]] for nm in names]
            if any(len(d) == 0 for d in doms):
                continue
            stats[hm][0] += 1
            iv = corner(t, g)
            ro = P.lo <= iv[0] and iv[1] <= P.hi

            def pn(tt):
                if tt[0] == "leaf":
                    return P.lo <= g[tt[1]][0] and g[tt[1]][1] <= P.hi
                if not pn(tt[2]) or not pn(tt[3]):
                    return False
                i2 = corner(tt, g)
                return P.lo <= i2[0] and i2[1] <= P.hi

            if ro != pn(t):
                stats[hm][2] += 1
            if ro:
                bad = False
                for tp in product(*doms):
                    env = dict(zip(names, tp))
                    if ev_exact(t, env) != ev_pernode(P, t, env):
                        bad = True
                        break
                if bad:
                    stats[hm][1] += 1
    print(
        f"  {label:<34} no-mul: {stats[False][0]:>5} cells, "
        f"{stats[False][1]:>4} unsound, {stats[False][2]:>4} checks-differ   |   "
        f"with-mul: {stats[True][0]:>5} cells, {stats[True][1]:>4} unsound, "
        f"{stats[True][2]:>4} checks-differ"
    )
    return stats


def main():
    print("=" * 96)
    print("p4. Arm W1 at a nonzero fraction width")
    print("=" * 96)

    # ---- P1/P2/P3: the homomorphism, per region, per operation ------------
    print()
    print("P1/P2/P3. Is the map a homomorphism, per operation, per fraction width")
    print()
    print(f"  {'primitive':<18} {'add':>12} {'sub':>12} {'mul':>12}")
    for P in [
        Fx(4, 0, False, "wrap"),
        Fx(4, 1, False, "wrap"),
        Fx(4, 2, False, "wrap"),
        Fx(4, 0, True, "wrap"),
        Fx(4, 2, True, "wrap"),
        Fx(4, 0, False, "sat"),
        Fx(4, 2, False, "sat"),
    ]:
        cells = []
        for op in ("add", "sub", "mul"):
            bad, n, wit = hom_check(P, op)
            cells.append(f"{bad}/{n}")
        print(f"  {str(P):<18} {cells[0]:>12} {cells[1]:>12} {cells[2]:>12}")

    print()
    print("  a witness for the F > 0 multiplication failure under wrap:")
    P = Fx(4, 2, False, "wrap")
    bad, n, wit = hom_check(P, "mul")
    for a, b, lhs, rhs in wit:
        print(f"    a = {a}, b = {b}: R(R(a)*R(b)) = {lhs}, R(a*b) = {rhs}")

    # ---- P4: what that does to the root-only check ------------------------
    print()
    print("P4. The root-only check at F > 0 under wrap, split by term shape")
    print()
    for P in [
        Fx(4, 0, False, "wrap"),
        Fx(4, 1, False, "wrap"),
        Fx(4, 2, False, "wrap"),
        Fx(4, 2, True, "wrap"),
    ]:
        ts = terms(("add", "sub", "mul"), 2) + terms(("add", "sub", "mul"), 3)
        sweep(P, ts, f"{P}")

    # ---- P5: does the grid part restore it --------------------------------
    print()
    print("P5. Restricting the operands to a coarse grid, which is `112` F112-4's")
    print("    grid part of the declaration")
    print()
    P = Fx(4, 2, False, "wrap")
    for coarse_F in (2, 1, 0):
        step = Fraction(1, 2**coarse_F)
        grid = [v for v in P.values() if (v / step).denominator == 1]
        bad = n = 0
        for a in grid:
            for b in grid:
                n += 1
                if P.R(ex("mul", P.R(a), P.R(b))) != P.R(ex("mul", a, b)):
                    bad += 1
        prod_on_grid = all(
            P.on_grid(a * b) for a in grid for b in grid
        )
        print(
            f"  operands declared on a grid of step {step}: "
            f"hom fails on {bad}/{n}, every product already on the fine grid: "
            f"{prod_on_grid}"
        )
    print()
    print("  so the grid part of a declaration is what restores the homomorphism")
    print("  at F > 0, exactly as the magnitude part restores the completion.")

    print()
    print("INSTRUMENT CHECK")
    print()
    P0 = Fx(4, 0, False, "wrap")
    b0, n0, _ = hom_check(P0, "mul")
    P2 = Fx(4, 2, False, "wrap")
    b2, n2, _ = hom_check(P2, "mul")
    print(f"  F = 0 mul under wrap: {b0}/{n0}   (this is the control, must be 0)")
    print(f"  F = 2 mul under wrap: {b2}/{n2}")
    print(f"  the check separates the two fraction widths: {b0 == 0 and b2 > 0}")


if __name__ == "__main__":
    main()
