#!/usr/bin/env python3
"""
p5. The composite layer, which nobody in this unit has pushed on, and whether
    a declared extent is what discharges a construction's base predicate.

WHERE THIS SITS
---------------
`110` F10 to F13 are its composite results and `111` section 13 records that
it "did not attack `110`'s P7 and P8 composite results at all" and that "the
composite side is where nobody has pushed back".  This is that push, and it is
constructive rather than an attack: I go looking for a connection between
`110`'s composite finding and `111`'s refinement, on the suspicion that they
are one mechanism.

`110` F12: the interval construction is closed exactly on MONOTONE bases, 16
of 16, while the no-precondition hypothesis agrees on 8 of 16.  Wrapping is
the non-monotone case: adding two well-ordered intervals over a wrapping base
returned `(1, 0)`, whose lower bound exceeds its upper.

`110` closes that section with: "So `interval` is an arm with a const
predicate on its base, and the predicate is monotonicity, which wrapping does
not have.  The same shape as every other arm in this design, arrived at from
the composition side rather than the rewriting side, and I was not looking for
it."

THE QUESTION
------------
A wrapping base restricted to an extent on which nothing wraps is EXACT on
that extent, and an exact operation is monotone.  So the predicate `110`
measures should be dischargeable by a declaration rather than only by a choice
of base.

If that holds, then the refinement is not a separate mechanism sitting beside
the composite layer.  It is what supplies the const predicate the composite
layer already wanted, and one thing serves both.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. Reproduce `110` F12's shape: interval is closed on monotone bases and not
    on wrapping ones, ungraded.
P2. Under a declared extent whose propagated bound discharges, interval IS
    closed over a wrapping base.
P3. The grade LIFTS through every construction tested: the composite's grade
    is computable from its components' grades, and predicts the composite's
    licence with zero unsound predictions.
P4. The lift is not the same rule for every construction.  Componentwise
    product should lift pairwise; complex should need a different rule because
    its multiplication mixes the components; interval should lift by the hull.
P5. Quantify the win: how many (base, extent) pairs gain the construction.
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
        s = "s" if self.signed else "u"
        return f"{s}W{self.W}F{self.F}{self.policy}"

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

    def mul(self, a, b):
        return self.R(a * b)

    def sub(self, a, b):
        return self.R(a - b)


def is_monotone(B):
    """`110` F12's predicate: does add preserve the order in each argument?"""
    vs = B.values()
    for c in vs:
        for i in range(len(vs) - 1):
            if B.add(vs[i], c) > B.add(vs[i + 1], c):
                return False
    return True


def is_monotone_on(B, ext):
    """The same predicate, restricted to a declared extent."""
    for c in ext:
        for i in range(len(ext) - 1):
            if B.add(ext[i], c) > B.add(ext[i + 1], c):
                return False
    return True


# ---------------------------------------------------------------------------
# The four constructions, as `110` P7 has them.
# ---------------------------------------------------------------------------


def product2_add(B, x, y):
    return (B.add(x[0], y[0]), B.add(x[1], y[1]))


def product2_mul(B, x, y):
    return (B.mul(x[0], y[0]), B.mul(x[1], y[1]))


def complex_add(B, x, y):
    return (B.add(x[0], y[0]), B.add(x[1], y[1]))


def complex_mul(B, x, y):
    return (
        B.sub(B.mul(x[0], y[0]), B.mul(x[1], y[1])),
        B.add(B.mul(x[0], y[1]), B.mul(x[1], y[0])),
    )


def dual_mul(B, x, y):
    return (
        B.mul(x[0], y[0]),
        B.add(B.mul(x[0], y[1]), B.mul(x[1], y[0])),
    )


def interval_add(B, x, y):
    return (B.add(x[0], y[0]), B.add(x[1], y[1]))


def interval_mul(B, x, y):
    cs = [B.mul(a, b) for a in x for b in y]
    return (min(cs), max(cs))


def well_ordered(p):
    return p[0] <= p[1]


def interval_closed(B, carrier):
    """Does interval addition keep a well-ordered pair well-ordered?"""
    ivs = [(a, b) for a in carrier for b in carrier if a <= b]
    bad = n = 0
    for x in ivs:
        for y in ivs:
            n += 1
            if not well_ordered(interval_add(B, x, y)):
                bad += 1
    return bad, n


def main():
    print("=" * 78)
    print("p5. A declared extent discharges a construction's base predicate")
    print("=" * 78)

    bases = [
        Base(W, F, sg, pol)
        for W in (2, 3)
        for F in (0, 1)
        for sg in (False, True)
        for pol in ("sat", "wrap")
    ]

    # ---- P1: reproduce 110 F12's shape, ungraded ------------------------
    print()
    print("P1. Ungraded: interval closure against the monotonicity predicate")
    print()
    print(f"  {'base':<12} {'monotone':>9} {'ill-ordered pairs':>19} {'agree':>7}")
    agree = disagree = 0
    for B in bases:
        mono = is_monotone(B)
        bad, n = interval_closed(B, B.values())
        closed = bad == 0
        ok = mono == closed
        agree += int(ok)
        disagree += int(not ok)
        print(
            f"  {str(B):<12} {str(mono):>9} {str(bad) + '/' + str(n):>19} "
            f"{str(ok):>7}"
        )
    print()
    print(f"  monotonicity predicts closure on {agree}/{agree + disagree} bases")
    nopre_agree = sum(1 for B in bases if interval_closed(B, B.values())[0] == 0)
    print(
        f"  the no-precondition hypothesis (always closed) agrees on "
        f"{nopre_agree}/{len(bases)}"
    )

    # ---- P2: does a declared extent discharge it ------------------------
    print()
    print("P2. Graded: interval closure over a WRAPPING base under a declared extent")
    print()
    print(
        f"  {'base':<12} {'extent':<12} {'propagated fits':>16} "
        f"{'monotone on it':>15} {'ill-ordered':>12}"
    )
    gained = considered = 0
    for B in [b for b in bases if b.policy == "wrap"]:
        vs = B.values()
        for bound_k in range(0, 2**B.W):
            hi = B.klo * B.step + bound_k * B.step
            ext = [v for v in vs if B.lo <= v <= hi]
            if len(ext) < 2:
                continue
            considered += 1
            # the propagated bound for one addition of two extent members
            prop_lo, prop_hi = 2 * ext[0], 2 * ext[-1]
            fits = B.lo <= prop_lo and prop_hi <= B.hi
            mono = is_monotone_on(B, ext)
            bad, n = interval_closed(B, ext)
            if fits and bad == 0:
                gained += 1
            if bound_k in (1, 2, 2 ** (B.W - 1)) or (fits and bad == 0 and gained <= 6):
                print(
                    f"  {str(B):<12} {'<= ' + str(hi):<12} {str(fits):>16} "
                    f"{str(mono):>15} {str(bad) + '/' + str(n):>12}"
                )
    print()
    print(
        f"  over wrapping bases: {gained} of {considered} declared extents give a "
        f"closed interval construction"
    )

    # the crucial cross-check: does the propagated bound PREDICT closure
    print()
    print("  Does the propagated bound predict interval closure, in both directions")
    unsound = conservative = exact = 0
    for B in [b for b in bases if b.policy == "wrap"]:
        vs = B.values()
        for bound_k in range(0, 2**B.W):
            hi = B.klo * B.step + bound_k * B.step
            ext = [v for v in vs if B.lo <= v <= hi]
            if len(ext) < 2:
                continue
            prop_lo, prop_hi = 2 * ext[0], 2 * ext[-1]
            fits = B.lo <= prop_lo and prop_hi <= B.hi
            bad, _ = interval_closed(B, ext)
            if fits and bad > 0:
                unsound += 1
            elif (not fits) and bad == 0:
                conservative += 1
            else:
                exact += 1
    print(
        f"    unsound {unsound}, conservative {conservative}, exact {exact}, "
        f"total {unsound + conservative + exact}"
    )

    # ---- P3 and P4: does the grade lift through a construction ----------
    print()
    print("P3/P4. Does the grade lift through a construction, and by which rule")
    print()

    def lifts(B, ctor_add, ctor_mul, grade_rule, ext_bound, label):
        vs = B.values()
        ext = [v for v in vs if 0 <= v <= ext_bound]
        if len(ext) < 2:
            return None
        pts = [(a, b) for a in ext for b in ext]
        unsound = n = 0
        for x in pts:
            for y in pts:
                n += 1
                says = grade_rule(B, ext_bound)
                exact_add = (x[0] + y[0], x[1] + y[1])
                got = ctor_add(B, x, y)
                if says and got != exact_add:
                    unsound += 1
        return label, unsound, n, grade_rule(B, ext_bound)

    B = Base(3, 0, False, "sat")
    for bound in (1, 3, 7):
        for label, ctor_add, ctor_mul in [
            ("product2", product2_add, product2_mul),
            ("complex", complex_add, complex_mul),
            ("dual", product2_add, dual_mul),
            ("interval", interval_add, interval_mul),
        ]:
            r = lifts(
                B,
                ctor_add,
                ctor_mul,
                lambda BB, b: 2 * b <= BB.hi,
                bound,
                label,
            )
            if r:
                lbl, u, n, says = r
                print(
                    f"  {lbl:<10} extent <= {bound}: grade says "
                    f"{'discharge' if says else 'refuse   '}, "
                    f"componentwise-add unsound {u}/{n}"
                )
        print()

    print("  Multiplication, where the componentwise lift is NOT the rule:")
    for bound in (1, 2, 3):
        ext = [v for v in B.values() if 0 <= v <= bound]
        pts = [(a, b) for a in ext for b in ext]
        rows = []
        for lbl, ctor, rule in [
            ("product2", product2_mul, lambda b: b * b <= B.hi),
            ("complex", complex_mul, lambda b: b * b <= B.hi),
            ("dual", dual_mul, lambda b: b * b <= B.hi),
        ]:
            u = n = 0
            for x in pts:
                for y in pts:
                    n += 1
                    got = ctor(B, x, y)
                    if lbl == "product2":
                        want = (x[0] * y[0], x[1] * y[1])
                    elif lbl == "complex":
                        want = (
                            x[0] * y[0] - x[1] * y[1],
                            x[0] * y[1] + x[1] * y[0],
                        )
                    else:
                        want = (x[0] * y[0], x[0] * y[1] + x[1] * y[0])
                    if rule(bound) and got != want:
                        u += 1
            rows.append((lbl, u, n, rule(bound)))
        for lbl, u, n, says in rows:
            print(
                f"    {lbl:<10} extent <= {bound}: pairwise rule says "
                f"{'discharge' if says else 'refuse   '}, unsound {u}/{n}"
            )

    print()
    print("INSTRUMENT CHECK")
    print()
    b1, n1 = interval_closed(Base(3, 0, False, "wrap"), Base(3, 0, False, "wrap").values())
    b2, n2 = interval_closed(Base(3, 0, False, "sat"), Base(3, 0, False, "sat").values())
    print(f"  wrapping base, full carrier : {b1}/{n1} ill-ordered")
    print(f"  saturating base, full carrier: {b2}/{n2} ill-ordered")
    print(f"  the closure test can fail and can pass: {b1 > 0 and b2 == 0}")


if __name__ == "__main__":
    main()
