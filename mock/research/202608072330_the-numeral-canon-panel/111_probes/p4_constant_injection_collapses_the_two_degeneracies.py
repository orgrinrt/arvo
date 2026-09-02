#!/usr/bin/env python3
"""P4. The two degeneracies coincide once the signature can inject a constant,
and they come apart again exactly when the constants are restricted to the grid.

`110` F5 and F6 draw a distinction I think is right and under-stated:

  DEFINITIONAL   the axis has left the definition of the value set and of R.
                 Radix at F = 0: step = radix^0 = 1 whatever the radix is.
  REACHABILITY   the axis is still read by R, but no term in the CURRENT
                 signature produces an argument on which it matters. Rounding
                 at F = 0 under grid-closed operations.

and concludes that only the first may be canonicalised away. Its instrument is
`110_probes/p5_definitional_versus_reachability_degeneracy.py`, whose three
signatures are declared at lines 101 to 106:

    SIG_CLOSED = [add, sub, mul, neg]
    SIG_OPEN   = SIG_CLOSED + [half]
    SIG_WIDE   = SIG_OPEN + [recip, fma]

**Every one of them is over carrier-valued arguments only. None contains a
nullary operation.** An algebra with no constants has no closed terms, so
"reachable" in that probe means reachable from carrier elements taken as free
variables, which is a legitimate reading and is not arvo's.

arvo cannot have that signature. A consumer has to be able to get a value in,
and I3's ergonomic imitation of a native Rust primitive means writing a
literal. A literal that is not already a grid point is exactly an application of
R to an arbitrary rational. So arvo's signature contains a constant injection,
and this probe asks what that does to the distinction.

Three sweeps:

  arm 1  110's grid-closed signature, reproduced independently.
  arm 2  the same plus constants restricted to grid points.
  arm 3  the same plus constants over a dense rational sample, which is what a
         decimal literal is.

Prediction before running, recorded so it can be wrong: radix at F = 0 stays
unobservable in all three, and rounding at F = 0 becomes observable in arm 3 and
not in arm 2. If that holds, the two degeneracies COINCIDE under arm 3's
signature and the distinction survives only where a design restricts its own
literals to the grid.
"""

from fractions import Fraction
from itertools import product

ROUNDINGS = ["near", "trunc", "floor"]
POLICIES = ["sat", "wrap"]


class Prim:
    def __init__(self, W, F, signed, policy, rounding, radix):
        self.W, self.F, self.signed = W, F, signed
        self.policy, self.rounding, self.radix = policy, rounding, radix
        n = 1 << W
        self.ints = list(range(-(n // 2), n // 2)) if signed else list(range(0, n))
        self.step = Fraction(1, radix ** F)
        self.values = [Fraction(k) * self.step for k in self.ints]
        self.lo, self.hi = min(self.values), max(self.values)

    def axes(self):
        return dict(W=self.W, F=self.F, signed=self.signed,
                    policy=self.policy, rounding=self.rounding, radix=self.radix)

    # ---- the one realisation map: rounding between grid points, policy outside
    def R(self, q):
        k = q / self.step
        if self.rounding == "trunc":
            g = Fraction(int(k), 1) if k >= 0 else -Fraction(int(-k), 1)
        elif self.rounding == "floor":
            n, d = k.numerator, k.denominator
            g = Fraction(n // d, 1)
        else:  # nearest, ties to even
            n, d = k.numerator, k.denominator
            fl = n // d
            rem = k - fl
            if rem > Fraction(1, 2):
                g = Fraction(fl + 1)
            elif rem < Fraction(1, 2):
                g = Fraction(fl)
            else:
                g = Fraction(fl if fl % 2 == 0 else fl + 1)
        v = g * self.step
        if self.lo <= v <= self.hi:
            return v
        if self.policy == "sat":
            return self.hi if v > self.hi else self.lo
        span = self.hi - self.lo + self.step
        return ((v - self.lo) % span) + self.lo

    # ---- operations
    def add(self, a, b):
        return self.R(a + b)

    def sub(self, a, b):
        return self.R(a - b)

    def mul(self, a, b):
        return self.R(a * b)

    def neg(self, a):
        return self.R(-a)

    def const(self, q):
        return self.R(q)


BINARY = ["add", "sub", "mul"]
UNARY = ["neg"]

SIG_CLOSED = ("grid-closed {add,sub,mul,neg}", BINARY + UNARY, None)
SIG_GRID_CONST = ("+ constants restricted to the grid", BINARY + UNARY, "grid")
SIG_Q_CONST = ("+ constants over a dense rational sample", BINARY + UNARY, "rationals")


def dense_rationals(p, n=64):
    """A stand-in for what a consumer writes as a literal: rationals across the
    representable span, deliberately including points strictly between grid
    points so the sample is not accidentally grid-aligned."""
    out = []
    span = p.hi - p.lo
    for i in range(n + 1):
        out.append(p.lo + span * Fraction(i, n))
    # explicit half-steps, which is where a rounding mode first bites
    out += [Fraction(1, 2), Fraction(-1, 2), Fraction(3, 2), Fraction(1, 3),
            Fraction(2, 3), Fraction(5, 2), Fraction(-3, 2)]
    return [q for q in out if p.lo - p.step <= q <= p.hi + p.step]


def separates(p, q, ops, consts):
    """Is there a term over `ops` (plus the declared constants) on which the two
    primitives disagree? Carrier elements are free variables, as in 110's probe,
    so this is depth-one plus a constant injection."""
    if p.values != q.values:
        return True
    vals = p.values
    for name in ops:
        if name in UNARY:
            for a in vals:
                if getattr(p, name)(a) != getattr(q, name)(a):
                    return True
        else:
            for a, b in product(vals, repeat=2):
                if getattr(p, name)(a, b) != getattr(q, name)(a, b):
                    return True
    if consts == "grid":
        for a in vals:
            if p.const(a) != q.const(a):
                return True
    elif consts == "rationals":
        for a in dense_rationals(p):
            if p.const(a) != q.const(a):
                return True
    return False


def definitionally_reads(p, q, n=400):
    """Does R itself differ anywhere on a dense sample of the rational line,
    independent of any signature? This is 110's own second test, and the point
    of this probe is that it is the constant-injection test with the constants
    ranging over Q."""
    span = p.hi - p.lo
    for i in range(n + 1):
        x = p.lo - span / 4 + (span * Fraction(3, 2)) * Fraction(i, n)
        if p.R(x) != q.R(x):
            return True
    for x in (Fraction(1, 2), Fraction(-1, 2), Fraction(3, 2), Fraction(1, 3)):
        if p.R(x) != q.R(x):
            return True
    return False


def sweep(axis):
    """All configuration pairs at F = 0 differing only in `axis`."""
    pairs = []
    for W, signed, policy in product([2, 3, 4], [False, True], POLICIES):
        if axis == "radix":
            for r1, r2 in [(2, 3), (2, 5), (3, 5)]:
                for rounding in ROUNDINGS:
                    pairs.append((Prim(W, 0, signed, policy, rounding, r1),
                                  Prim(W, 0, signed, policy, rounding, r2)))
        else:
            for m1, m2 in [("near", "trunc"), ("near", "floor"), ("trunc", "floor")]:
                pairs.append((Prim(W, 0, signed, policy, m1, 2),
                              Prim(W, 0, signed, policy, m2, 2)))
    return pairs


def main():
    print("P4. what constant injection does to the two degeneracies, at F = 0")
    print("=" * 78)

    for axis in ("radix", "rounding"):
        pairs = sweep(axis)
        print(f"\naxis varied: {axis}   ({len(pairs)} configuration pairs, all at F = 0)")
        print(f"  {'signature':<44} {'observable':>10}")
        for label, ops, consts in (SIG_CLOSED, SIG_GRID_CONST, SIG_Q_CONST):
            hits = sum(1 for p, q in pairs if separates(p, q, ops, consts))
            print(f"  {label:<44} {hits:>4}/{len(pairs):<5}")
        d = sum(1 for p, q in pairs if definitionally_reads(p, q))
        print(f"  {'R differs somewhere on the rational line':<44} {d:>4}/{len(pairs):<5}")

    # ---- the coincidence claim, stated as a per-pair agreement rather than as
    # two totals, because two totals agreeing is weaker than every cell agreeing.
    print("\n" + "-" * 78)
    print("coincidence: does 'reachable under {ops + rational constants}' agree")
    print("cell by cell with 'R differs somewhere on the rational line'?")
    agree = disagree = 0
    for axis in ("radix", "rounding"):
        for p, q in sweep(axis):
            a = separates(p, q, SIG_Q_CONST[1], SIG_Q_CONST[2])
            b = definitionally_reads(p, q)
            if a == b:
                agree += 1
            else:
                disagree += 1
                print(f"  DISAGREE {p.axes()} vs {q.axes()}: reach={a} def={b}")
    print(f"  agree {agree}, disagree {disagree}")

    print("\ncoincidence under GRID-RESTRICTED constants, the same question:")
    agree2 = disagree2 = 0
    for axis in ("radix", "rounding"):
        for p, q in sweep(axis):
            a = separates(p, q, SIG_GRID_CONST[1], SIG_GRID_CONST[2])
            b = definitionally_reads(p, q)
            if a == b:
                agree2 += 1
            else:
                disagree2 += 1
    print(f"  agree {agree2}, disagree {disagree2}")


    # ---- the three cells where rounding stays unobservable even with rational
    # constants, named rather than left as a residue in a ratio.
    print("\n" + "-" * 78)
    print("the pairs where rounding stays unobservable under rational constants:")
    for p_, q_ in sweep("rounding"):
        if not definitionally_reads(p_, q_):
            a = p_.axes()
            print(f"  W={a['W']} {'signed' if a['signed'] else 'unsigned'} "
                  f"{a['policy']}: {p_.rounding} vs {q_.rounding}")
    print("  the overflow policy erases the disagreement: over an unsigned set")
    print("  every negative argument clamps to the same endpoint under sat, and")
    print("  trunc and floor differ only on negatives. Under wrap they separate.")
    print("  So observability of one axis is a joint fact with another, which is")
    print("  a reason a per-axis degeneracy verdict is the wrong unit.")

    print("\n" + "-" * 78)
    print("reading:")
    print("  the two notions COINCIDE when the constants cover the ambient domain")
    print("  and COME APART when the constants are restricted to the grid.")
    print("  So the distinction is not between two kinds of degeneracy. It is one")
    print("  notion evaluated at two signatures, and 'definitional' is the value")
    print("  it takes at the largest signature a design will ever admit.")


if __name__ == "__main__":
    main()
