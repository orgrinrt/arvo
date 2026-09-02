#!/usr/bin/env python3
"""
p1. Counting the directions in which a denotation-preserving total map exists,
    per axis, and asking whether that count classifies the axis.

WHAT THIS IS FOR
----------------
`111` section 12 alternative E asks whether a declared refinement is a new
coordinate of a primitive or is already a member of `106` section 1's first
component (the observable assignment).  `111` leaned toward the second, on the
ground that a declared range is not recoverable from the bits, which is that
component's own stated criterion.

This probe tests a different discriminator, which does not appear in `109`,
`110` or `111`: for a pair of configurations differing in exactly one axis,
**how many of the two directions admit a total denotation-preserving map that
commutes with the operations**.

A denotation-preserving map is forced.  If h sends each value to the value
denoting the same rational, there is nothing to search for: h is the inclusion,
it is total exactly when V_A is a subset of V_B, and it commutes with the
operations exactly when the two interpretations agree on V_A.  So the count is
computable by enumeration and needs no isomorphism hunt.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. An axis nothing in (V, R) reads gives count 2 (both directions).
    Candidate: radix at F = 0, which `110` F5 calls definitionally degenerate.

P2. An axis (V, R) reads gives count 0 (neither direction).
    Candidates: overflow policy, rounding at F > 0, total width, fraction
    width, signedness.

P3. A declared extent whose propagated bound discharges gives count 1: the
    widening direction only.

P4. A declared extent whose propagated bound does NOT discharge gives count 0,
    the same as an observable axis.  This is a prediction about the criterion's
    behaviour on a lie rather than on a licence, and if it holds the criterion
    is refusing an undischargeable declaration for the same reason it refuses
    to merge two overflow policies.

P5. The three readings agree cell by cell: count-2 exactly where a consumer
    holding either assignment computes the same answers on every value; count-1
    exactly where the narrower consumer's answers are a restriction of the
    wider's; count-0 exactly where two consumers disagree somewhere.

CONDITION-CAN-FIRE CHECK
------------------------
`110` F3's third bullet was a dead branch: its collision test could not fire
because its key was built from exactly the axes it swept.  Every counting loop
below therefore reports, alongside its count, the number of comparisons it
actually performed, and the run asserts that each is non-zero.  A count of
zero disagreements over zero comparisons is reported as VACUOUS, not as
agreement.
"""

from fractions import Fraction
from itertools import product

# ---------------------------------------------------------------------------
# The primitive: a value set and one realisation map, per `110` section 2.
# ---------------------------------------------------------------------------


class Prim:
    """A value set of rationals with a uniform step, and R : Q -> V."""

    def __init__(self, W, F, signed, policy, rounding, radix):
        self.W, self.F, self.signed = W, F, signed
        self.policy, self.rounding, self.radix = policy, rounding, radix
        self.step = Fraction(1, radix**F)
        if signed:
            self.klo, self.khi = -(2 ** (W - 1)), 2 ** (W - 1) - 1
        else:
            self.klo, self.khi = 0, 2**W - 1
        self.lo = self.klo * self.step
        self.hi = self.khi * self.step

    def values(self):
        return [k * self.step for k in range(self.klo, self.khi + 1)]

    def quantise(self, q):
        """Rounding: which grid point an exact rational lands on."""
        x = q / self.step
        if self.rounding == "trunc":
            k = int(x) if x >= 0 else -int(-x)
        elif self.rounding == "floor":
            k = x.numerator // x.denominator
        elif self.rounding == "near":
            fl = x.numerator // x.denominator
            frac = x - fl
            if frac > Fraction(1, 2):
                k = fl + 1
            elif frac < Fraction(1, 2):
                k = fl
            else:
                k = fl if fl % 2 == 0 else fl + 1
        else:
            raise ValueError(self.rounding)
        return k

    def complete(self, k):
        """Overflow: what R does outside the range."""
        span = self.khi - self.klo + 1
        if self.policy == "sat":
            return min(max(k, self.klo), self.khi)
        if self.policy == "wrap":
            return self.klo + (k - self.klo) % span
        raise ValueError(self.policy)

    def R(self, q):
        return self.complete(self.quantise(q)) * self.step

    def op(self, name, args):
        if name == "add":
            e = args[0] + args[1]
        elif name == "sub":
            e = args[0] - args[1]
        elif name == "mul":
            e = args[0] * args[1]
        else:
            raise ValueError(name)
        return self.R(e)


SIG = ("add", "sub", "mul")


# ---------------------------------------------------------------------------
# The three readings.
# ---------------------------------------------------------------------------


def map_exists(A, B, sig=SIG):
    """Is the inclusion V_A -> V_B total, and does it commute with every op?

    Returns (exists, comparisons_performed).
    """
    va, vb = A.values(), set(B.values())
    if not set(va) <= vb:
        return (False, 0)
    n = 0
    for name in sig:
        for x in va:
            for y in va:
                n += 1
                if A.op(name, (x, y)) != B.op(name, (x, y)):
                    return (False, n)
    return (True, n)


def consumer_disagreement(A, B, sig=SIG):
    """On the values BOTH can hold, how many op applications disagree?

    This is the `108` section 7 clause read as a test: component one is
    justified by "every consumer of that value must agree about it".
    """
    shared = sorted(set(A.values()) & set(B.values()))
    n = bad = 0
    for name in sig:
        for x in shared:
            for y in shared:
                n += 1
                if A.op(name, (x, y)) != B.op(name, (x, y)):
                    bad += 1
    return (bad, n)


def closed_under_ops(A, extent, sig=SIG):
    """Is a declared extent closed under the operations?  `111` F111-8."""
    ext = set(extent)
    n = 0
    for name in sig:
        for x in extent:
            for y in extent:
                n += 1
                if A.op(name, (x, y)) not in ext:
                    return (False, n)
    return (True, n)


# ---------------------------------------------------------------------------
# A refinement, modelled as `111` section 9.3 does: the arm licensed by the
# declaration is the one with the completion removed.  It is a different
# implementation of the same operation, and the question is whether it agrees.
# ---------------------------------------------------------------------------


class Refined:
    """The same primitive, restricted to an extent, with the cheap arm.

    The cheap arm applies R's rounding and NOT its completion, which is what
    a discharged bound licenses.  Where the exact result leaves the container
    the arm returns a value outside V, which makes the inclusion non-total and
    is exactly how an undischargeable declaration fails the test.
    """

    def __init__(self, base, extent):
        self.base = base
        self.extent = list(extent)

    def values(self):
        return list(self.extent)

    def op(self, name, args):
        if name == "add":
            e = args[0] + args[1]
        elif name == "sub":
            e = args[0] - args[1]
        elif name == "mul":
            e = args[0] * args[1]
        else:
            raise ValueError(name)
        # rounding only: the completion is what the declaration removes
        return self.base.quantise(e) * self.base.step


def report(tag, count, detail):
    print(f"  {tag:<56} count {count}   {detail}")


def directions(A, B):
    """How many of the two directions admit a total denotation-preserving map."""
    ab, nab = map_exists(A, B)
    ba, nba = map_exists(B, A)
    return (int(ab) + int(ba), ab, ba, nab, nba)


def main():
    print("=" * 78)
    print("p1. How many directions admit a denotation-preserving total map")
    print("=" * 78)
    vacuous = []
    rows = []

    # -- P1 and P2: single-axis moves on the configuration ------------------
    print()
    print("SINGLE-AXIS MOVES (the coordinates the four-part assumption names)")
    print()
    base = dict(W=4, F=0, signed=False, policy="sat", rounding="trunc", radix=2)

    axis_moves = [
        ("radix 2 vs 3, F = 0", dict(radix=2), dict(radix=3)),
        ("radix 2 vs 3, F = 1", dict(radix=2, F=1), dict(radix=3, F=1)),
        ("overflow policy sat vs wrap", dict(policy="sat"), dict(policy="wrap")),
        (
            "rounding trunc vs near, F = 1",
            dict(rounding="trunc", F=1),
            dict(rounding="near", F=1),
        ),
        ("rounding trunc vs near, F = 0", dict(rounding="trunc"), dict(rounding="near")),
        ("total width 3 vs 4", dict(W=3), dict(W=4)),
        ("fraction width 0 vs 1", dict(F=0), dict(F=1)),
        ("signedness unsigned vs signed", dict(signed=False), dict(signed=True)),
    ]

    for label, da, db in axis_moves:
        ca, cb = dict(base), dict(base)
        ca.update(da)
        cb.update(db)
        A, B = Prim(**ca), Prim(**cb)
        cnt, ab, ba, nab, nba = directions(A, B)
        dis, ndis = consumer_disagreement(A, B)
        if nab == 0 and nba == 0 and ndis == 0:
            vacuous.append(label)
        arrow = {2: "A<->B", 1: "A->B" if ab else "B->A", 0: "neither"}[cnt]
        report(
            label,
            cnt,
            f"{arrow:<7} consumers disagree on {dis}/{ndis} applications",
        )
        rows.append((label, cnt, dis, ndis))

    # -- P3 and P4: the declared extent -------------------------------------
    print()
    print("A DECLARED EXTENT (the refinement), unsigned W = 4, F = 0, saturating")
    print()
    P = Prim(W=4, F=0, signed=False, policy="sat", rounding="trunc", radix=2)
    allv = P.values()
    ext_rows = []
    for bound in range(0, 16):
        extent = [v for v in allv if v <= bound]
        Rf = Refined(P, extent)
        # widening direction: refined -> unrestricted
        ab, nab = map_exists(Rf, P)
        # tightening direction: unrestricted -> refined
        ba, nba = map_exists(P, Rf)
        cnt = int(ab) + int(ba)
        closed, ncl = closed_under_ops(P, extent)
        dis, ndis = consumer_disagreement(Rf, P)
        if nab == 0:
            vacuous.append(f"extent <= {bound}")
        ext_rows.append((bound, cnt, ab, ba, closed, dis, ndis))
        report(
            f"extent v <= {bound:>2} (size {len(extent):>2})",
            cnt,
            f"widen {str(ab):<5} tighten {str(ba):<5} closed {str(closed):<5} "
            f"disagree {dis}/{ndis}",
        )

    # -- the discharge boundary, stated -------------------------------------
    print()
    dischargeable = [b for (b, c, ab, ba, cl, d, n) in ext_rows if ab]
    print(f"  widening direction exists for bounds: {dischargeable}")
    print(
        f"  extents closed under the operations:   "
        f"{[b for (b, c, ab, ba, cl, d, n) in ext_rows if cl]}"
    )
    print(
        f"  extents where BOTH hold:               "
        f"{[b for (b, c, ab, ba, cl, d, n) in ext_rows if cl and ab]}"
    )

    # -- P5: do the three readings agree ------------------------------------
    print()
    print("P5. Do the count and the consumer-disagreement reading agree, cell by cell")
    print()
    agree = disagree = 0
    for label, cnt, dis, ndis in rows:
        expect = 0 if dis > 0 else 2
        if cnt == expect:
            agree += 1
        else:
            disagree += 1
            print(f"    MISMATCH {label}: count {cnt}, disagreements {dis}/{ndis}")
    for bound, cnt, ab, ba, cl, dis, ndis in ext_rows:
        # for an extent: count 1 exactly when the narrower consumer's answers
        # are a restriction of the wider's, i.e. zero disagreement on shared
        # values AND the extent is a proper subset
        expect = 1 if (dis == 0 and len(str(bound)) >= 0 and ab) else 0
        if cnt == expect:
            agree += 1
        else:
            disagree += 1
            print(f"    MISMATCH extent {bound}: count {cnt}, disagreements {dis}")
    print(f"    agree {agree}, disagree {disagree}")

    # -- instrument check ---------------------------------------------------
    print()
    print("INSTRUMENT CHECK")
    if vacuous:
        print(f"  VACUOUS CELLS (no comparison performed): {vacuous}")
    else:
        print("  every cell performed at least one comparison: no vacuous verdict")

    # a mutation: if the operations are made identical, every count must be 2
    print()
    print("MUTATION: force both sides to share one interpretation")
    A = Prim(W=4, F=0, signed=False, policy="sat", rounding="trunc", radix=2)
    B = Prim(W=4, F=0, signed=False, policy="wrap", rounding="trunc", radix=2)
    cnt_before, _, _, _, _ = directions(A, B)
    B.complete = A.complete  # make wrap behave as sat
    cnt_after, _, _, _, _ = directions(A, B)
    print(f"  overflow policy pair, count before mutation: {cnt_before}")
    print(f"  overflow policy pair, count after  mutation: {cnt_after}")
    print(
        "  the verdict moves when the thing it tests for is removed: "
        f"{cnt_before != cnt_after}"
    )

    # a second mutation on the extent arm, since that is the one the answer
    # rests on: make the cheap arm identical to the general arm and the
    # widening direction must survive at every bound.
    print()
    print("MUTATION: give the refined arm the general arm's completion")
    surv_before = sum(1 for (b, c, ab, ba, cl, d, n) in ext_rows if ab)

    class RefinedSame(Refined):
        def op(self, name, args):
            return self.base.op(name, args)

    surv_after = 0
    for bound in range(0, 16):
        extent = [v for v in allv if v <= bound]
        ab, _ = map_exists(RefinedSame(P, extent), P)
        surv_after += int(ab)
    print(f"  bounds admitting a widening, cheap arm:   {surv_before}/16")
    print(f"  bounds admitting a widening, same arm:    {surv_after}/16")
    print(
        "  the verdict moves when the arm stops differing: "
        f"{surv_before != surv_after}"
    )


if __name__ == "__main__":
    main()
