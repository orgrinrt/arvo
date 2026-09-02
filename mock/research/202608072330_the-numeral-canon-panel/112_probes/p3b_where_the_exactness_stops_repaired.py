#!/usr/bin/env python3
"""
p3b. Where the propagated grade stops being exact, with the witnesses repaired.

WHY
---
`111` F111-9 and F111-10 report "zero unsound and zero conservative" for the
propagated bound, over four sweeps.  p2b reproduces that exactly on an
independent implementation with a conservative counter that fires under
mutation.  But every sweep in both files, mine included, has one shape:

    a LEFT-NESTED chain of ONE operation over INDEPENDENT leaves.

On that shape the corner rule is tight by construction.  For `add` over
intervals the reachable sum's endpoints are attained by taking both operands
at their own endpoints, so the propagated interval IS the reachable set's
hull, and a conservative verdict is impossible.  So the zero is a fact about
the shape swept, not about the rule, and reported without the shape it reads
as a property of the mechanism.

Interval arithmetic has a classical failure with a name: the DEPENDENCY
PROBLEM.  Where a term mentions the same leaf twice, or where two leaves are
correlated, the corner rule treats the occurrences as independent and the
propagated interval is strictly wider than the reachable set.  `x - x` is the
textbook case: reachable set {0}, propagated interval [-hi, hi].

If the dependency problem shows up here, then the exactness claim needs the
shape in its predicate, and a design that relies on it needs to know which
terms it holds for.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. On independent-leaf single-operation chains: conservative 0, reproducing
    `111` F111-9 and p2b.  This is the control.
P2. On a term with a REPEATED leaf: conservative > 0.  The rule refuses on
    terms whose reachable results all sit inside the container.
P3. Unsound stays 0 on every shape.  The corner rule over-approximates, so it
    can lose an optimisation and cannot license a wrong answer.
P4. The lost region is not marginal: on `x - x`-shaped terms at W = 4 the rule
    should refuse on most or all extents while every reachable answer is in
    range.
P5. A rule that tracks the reachable set exactly recovers the lost cells, and
    is not const-computable at real widths, so the residue is a real design
    cost rather than a bug to fix.
"""

from fractions import Fraction
from itertools import product


class Prim:
    def __init__(self, W, F, signed, policy, rounding, radix=2):
        self.W, self.F, self.signed = W, F, signed
        self.policy, self.rounding = policy, rounding
        self.step = Fraction(1, radix**F)
        if signed:
            self.klo, self.khi = -(2 ** (W - 1)), 2 ** (W - 1) - 1
        else:
            self.klo, self.khi = 0, 2**W - 1
        self.lo, self.hi = self.klo * self.step, self.khi * self.step

    def values(self):
        return [k * self.step for k in range(self.klo, self.khi + 1)]

    def quantise(self, q):
        x = q / self.step
        if self.rounding == "trunc":
            return int(x) if x >= 0 else -int(-x)
        raise ValueError(self.rounding)

    def complete(self, k):
        span = self.khi - self.klo + 1
        if self.policy == "sat":
            return min(max(k, self.klo), self.khi)
        if self.policy == "wrap":
            return self.klo + (k - self.klo) % span
        raise ValueError(self.policy)

    def general(self, name, a, b):
        return self.complete(self.quantise(exact(name, a, b))) * self.step


def exact(name, a, b):
    if name == "add":
        return a + b
    if name == "sub":
        return a - b
    if name == "mul":
        return a * b
    raise ValueError(name)


# --------------------------------------------------------------------------
# Terms as trees over named leaves, so a leaf may appear more than once.
# --------------------------------------------------------------------------

# ("leaf", name) | ("op", opname, left, right)


def leaves_of(t):
    if t[0] == "leaf":
        return {t[1]}
    return leaves_of(t[2]) | leaves_of(t[3])


def eval_exact(t, env):
    if t[0] == "leaf":
        return env[t[1]]
    return exact(t[1], eval_exact(t[2], env), eval_exact(t[3], env))


def eval_general(P, t, env):
    if t[0] == "leaf":
        return env[t[1]]
    return P.general(t[1], eval_general(P, t[2], env), eval_general(P, t[3], env))


def corner_grade(t, grades):
    """The propagated grade: corners, occurrence by occurrence, no correlation."""
    if t[0] == "leaf":
        return grades[t[1]]
    g = corner_grade(t[2], grades)
    h = corner_grade(t[3], grades)
    cs = [exact(t[1], a, b) for a in (g[0], g[1]) for b in (h[0], h[1])]
    return (min(cs), max(cs))


def node_grades_ok(P, t, grades):
    """Does every node's propagated grade sit inside the container?"""
    if t[0] == "leaf":
        g = grades[t[1]]
        return P.lo <= g[0] and g[1] <= P.hi
    if not node_grades_ok(P, t[2], grades) or not node_grades_ok(P, t[3], grades):
        return False
    g = corner_grade(t, grades)
    return P.lo <= g[0] and g[1] <= P.hi


def reachable_ok(P, t, grades, values):
    """Does every REACHABLE intermediate sit inside the container?

    This is the oracle the corner rule approximates.  It enumerates, so it is
    not available as a const predicate at a real width.
    """
    names = sorted(leaves_of(t))
    doms = [[v for v in values if grades[n][0] <= v <= grades[n][1]] for n in names]
    if any(len(d) == 0 for d in doms):
        return None
    for tup in product(*doms):
        env = dict(zip(names, tup))
        if not _node_ok(P, t, env):
            return False
    return True


def _node_ok(P, t, env):
    if t[0] == "leaf":
        return True
    if not _node_ok(P, t[2], env) or not _node_ok(P, t[3], env):
        return False
    v = eval_exact(t, env)
    return P.lo <= v <= P.hi


def arms_agree(P, t, grades, values):
    names = sorted(leaves_of(t))
    doms = [[v for v in values if grades[n][0] <= v <= grades[n][1]] for n in names]
    if any(len(d) == 0 for d in doms):
        return None, 0
    n = 0
    for tup in product(*doms):
        env = dict(zip(names, tup))
        n += 1
        if eval_exact(t, env) != eval_general(P, t, env):
            return False, n
    return True, n


def sweep(P, t, bound_range, label):
    names = sorted(leaves_of(t))
    unsound = conservative = exactcell = cells = 0
    values = P.values()
    for bs in product(bound_range, repeat=len(names)):
        grades = {n: (Fraction(0), Fraction(b)) for n, b in zip(names, bs)}
        says = node_grades_ok(P, t, grades)
        agree, n = arms_agree(P, t, grades, values)
        if agree is None:
            continue
        cells += n
        if says and not agree:
            unsound += 1
        elif (not says) and agree:
            conservative += 1
        else:
            exactcell += 1
    total = unsound + conservative + exactcell
    print(
        f"  {label:<46} unsound {unsound:>4}  conservative {conservative:>4}  "
        f"exact {exactcell:>4}  of {total:>4} extents, {cells} tuples"
    )
    return unsound, conservative, exactcell, total


def show(t):
    if t[0] == "leaf":
        return t[1]
    return f"({show(t[2])} {t[1]} {show(t[3])})"


def main():
    print("=" * 78)
    print("p3. Where the propagated grade stops being exact")
    print("=" * 78)

    L = lambda n: ("leaf", n)
    O = lambda op, a, b: ("op", op, a, b)

    P4 = Prim(4, 0, False, "sat", "trunc")
    P4w = Prim(4, 0, False, "wrap", "trunc")
    S4 = Prim(4, 0, True, "sat", "trunc")

    print()
    print("CONTROL: independent leaves, one operation, left-nested")
    print("(the shape every sweep in `111` and in my p2b has)")
    print()
    sweep(P4, O("add", L("x"), L("y")), range(0, 16), "x + y, unsigned sat W=4")
    sweep(P4w, O("add", L("x"), L("y")), range(0, 16), "x + y, unsigned wrap W=4")
    sweep(
        P4,
        O("add", O("add", L("x"), L("y")), L("z")),
        range(0, 16),
        "(x + y) + z, unsigned sat W=4",
    )
    sweep(P4, O("mul", L("x"), L("y")), range(0, 16), "x * y, unsigned sat W=4")

    print()
    print("THE DEPENDENCY PROBLEM: a leaf mentioned twice")
    print()
    sweep(S4, O("sub", L("x"), L("x")), range(0, 8), "x - x, SIGNED sat W=4")
    sweep(
        S4,
        O("sub", O("add", L("x"), L("y")), L("x")),
        range(0, 8),
        "(x + y) - x, SIGNED sat W=4",
    )
    sweep(
        P4,
        O("sub", O("add", L("x"), L("y")), L("y")),
        range(0, 16),
        "(x + y) - y, unsigned sat W=4",
    )
    sweep(
        S4,
        O("mul", L("x"), O("sub", L("y"), L("y"))),
        range(0, 8),
        "x * (y - y), SIGNED sat W=4",
    )

    print()
    print("MIXED OPERATIONS with independent leaves, to separate the two causes")
    print()
    sweep(
        S4,
        O("sub", O("add", L("x"), L("y")), L("z")),
        range(0, 8),
        "(x + y) - z, SIGNED sat W=4",
    )
    sweep(
        P4,
        O("mul", O("add", L("x"), L("y")), L("z")),
        range(0, 16),
        "(x + y) * z, unsigned sat W=4",
    )

    print()
    print("THE ORACLE: what a reachable-set rule would license instead")
    print()
    for label, P, t, rng in [
        (
            "(x + y) - y, unsigned sat W=4",
            P4,
            O("sub", O("add", L("x"), L("y")), L("y")),
            range(0, 16),
        ),
        (
            "x * (y - y), SIGNED sat W=4",
            S4,
            O("mul", L("x"), O("sub", L("y"), L("y"))),
            range(0, 8),
        ),
        (
            "(x + y) * z, unsigned sat W=4",
            P4,
            O("mul", O("add", L("x"), L("y")), L("z")),
            range(0, 16),
        ),
        ("x - x, SIGNED sat W=4", S4, O("sub", L("x"), L("x")), range(0, 8)),
    ]:
        names = sorted(leaves_of(t))
        corner_yes = reach_yes = both = 0
        for bs in product(rng, repeat=len(names)):
            grades = {n: (Fraction(0), Fraction(b)) for n, b in zip(names, bs)}
            c = node_grades_ok(P, t, grades)
            r = reachable_ok(P, t, grades, P.values())
            if r is None:
                continue
            corner_yes += int(c)
            reach_yes += int(r)
            both += 1
        print(
            f"  {label:<30} corner licenses {corner_yes:>4}/{both}, "
            f"reachable-set licenses {reach_yes:>4}/{both}"
        )

    print()
    print("INSTRUMENT CHECK")
    print()
    u, c, e, t = sweep(
        P4, O("add", L("x"), L("y")), range(0, 16), "control repeated for the check"
    )
    print(f"  the control's conservative counter is {c}, which is the claim under test")
    u2, c2, e2, t2 = sweep(
        P4,
        O("sub", O("add", L("x"), L("y")), L("y")),
        range(0, 16),
        "correlated-leaf case for the check",
    )
    print(f"  the same counter on a correlated leaf is {c2}")
    print(f"  so the counter is live and the control's zero is a result: {c2 > c}")
    u3, c3, e3, t3 = sweep(
        S4, O("sub", L("x"), L("x")), range(0, 8), "x - x, the witness p3 chose"
    )
    print(
        f"  and a repeated leaf is NOT sufficient on its own: x - x gives {c3}, "
        "because its propagated interval never leaves the container"
    )


if __name__ == "__main__":
    main()
