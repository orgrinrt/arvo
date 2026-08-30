#!/usr/bin/env python3
"""P9. The rationalisability check as a decision procedure, not an assertion.

Section 2.4 of `97` proposes one constraint on a design-tier table: that some single
weighting explain every row at once. It calls that checkable and cheap. P1 showed it
for two cost dimensions by sweeping the direction, which is exact and does not
generalise, so the claim as it stood was an assertion about the general case.

This is the decider. Given a table assigning an arm to each region, and a cost vector
per arm per region in k dimensions, it answers exactly whether a non-negative,
non-zero weighting w exists with

    w . cost(chosen(e), e)  <=  w . cost(a, e)     for every region e and every arm a

That is a homogeneous linear feasibility question. The feasible set is a cone inside
the non-negative orthant, so it is pointed, so it is non-trivial exactly when it has
an extreme ray, and every extreme ray of a pointed cone in k dimensions lies on k - 1
linearly independent tight constraints. So the decider enumerates those, exactly, in
rational arithmetic. No sampling, no tolerance, no floating point comparison.

Two checks are run.

One: the decider against P1's direction sweep, on the committed carrier data, over
ALL 15625 sections. The two do NOT return the same number, and the discrepancy is the
useful part rather than a defect in either. P1 counts sections that arise as THE
argmin under some direction, with ties broken deterministically. This counts sections
that are AN argmin, allowing ties. Where two arms cost the same under a weighting, a
design genuinely may take either, so the weak count is the honest formalisation of
"some weighting explains this table" and it is the larger and more conservative
number. The gap decomposes exactly, and the decomposition is checked below.

Two: the decider at three cost dimensions, where a direction sweep does not apply,
against cases whose answers are known by construction. One of those expectations was
mine and was wrong before the decider corrected it; the case is kept with the
correction stated, because a probe that quietly agrees with its author is worth less
than one that does not.
"""

import csv
import glob
import itertools
import os
import statistics
from fractions import Fraction as F

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.abspath(os.path.join(HERE, "..", "..", "..", "benches"))

BITS = {
    "bitpack-carrier-d16": 16,
    "bitpack-carrier-d32": 32,
    "bitpack-carrier-d64": 64,
    "bitpack-carrier-packed": 13,
    "bitpack-carrier-packed-simd": 13,
}


# ------------------------------------------------------------------ the decider


def nullspace_ray(rows, k):
    """The 1-dimensional kernel of k-1 rows in R^k, by cofactor expansion.

    Returns None when the rows are not independent, which shows up as the zero
    vector rather than as an exception.
    """
    ray = []
    for j in range(k):
        sub = [[r[i] for i in range(k) if i != j] for r in rows]
        ray.append(((-1) ** j) * det(sub))
    return None if all(c == 0 for c in ray) else ray


def det(m):
    n = len(m)
    if n == 0:
        return F(1)
    if n == 1:
        return m[0][0]
    total = F(0)
    for j in range(n):
        minor = [[row[i] for i in range(n) if i != j] for row in m[1:]]
        total += ((-1) ** j) * m[0][j] * det(minor)
    return total


def rationalisable(constraints, k):
    """Is there w >= 0, w != 0, with w . d <= 0 for every d in constraints?

    `constraints` are the difference vectors cost(chosen) - cost(alternative).
    """
    # every constraint, including the k non-negativity ones, as rows of "<= 0"
    rows = [list(d) for d in constraints]
    for j in range(k):
        e = [F(0)] * k
        e[j] = F(-1)
        rows.append(e)

    for combo in itertools.combinations(range(len(rows)), k - 1):
        ray = nullspace_ray([rows[i] for i in combo], k)
        if ray is None:
            continue
        for sign in (1, -1):
            w = [sign * c for c in ray]
            if all(c >= 0 for c in w) and any(c != 0 for c in w):
                if all(sum(r[i] * w[i] for i in range(k)) <= 0 for r in rows):
                    return True
    return False


def constraints_for(section, costs, arms, k):
    out = []
    for region, chosen in zip(costs, section):
        for a in arms:
            if a == chosen:
                continue
            out.append([region[chosen][i] - region[a][i] for i in range(k)])
    return out


# ------------------------------------------------------------------ check one


def strict_sections(costs, arms):
    """P1's method, recomputed here so the decomposition is self-contained.

    THE argmin under each direction, ties broken by the first arm in sorted order,
    which is what a deterministic table-writer would produce.
    """
    import math
    breaks = set()
    for region in costs:
        for i in range(len(arms)):
            for j in range(i + 1, len(arms)):
                t1, b1 = region[arms[i]]
                t2, b2 = region[arms[j]]
                dt, db = t1 - t2, b1 - b2
                if db == 0:
                    continue
                tan = -dt / db
                if tan > 0:
                    th = math.atan(float(tan))
                    if 0.0 < th < math.pi / 2:
                        breaks.add(th)
    edges = [0.0] + sorted(breaks) + [math.pi / 2]
    out = set()
    for i in range(len(edges) - 1):
        th = (edges[i] + edges[i + 1]) / 2.0
        wt, wb = math.cos(th), math.sin(th)
        sec = []
        for region in costs:
            best, bestv = None, None
            for a in sorted(arms):
                v = wt * float(region[a][0]) + wb * float(region[a][1])
                if bestv is None or v < bestv - 1e-15:
                    best, bestv = a, v
            sec.append(best)
        out.add(tuple(sec))
    return out


def load_committed():
    regions = []
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(os.path.basename(path).split("_n")[1].split(".")[0])
        by = {}
        for r in csv.DictReader(open(path)):
            by.setdefault(r["variant"], []).append(float(r["algo_ns"]))
        regions.append((n, {v: (F(statistics.median(xs)) / n, F(BITS[v]))
                            for v, xs in by.items() if v in BITS}))
    regions.sort()
    return [c for _, c in regions]


def check_one():
    costs = load_committed()
    arms = sorted(BITS)
    k = 2
    total = 0
    good = 0
    witnesses = []
    for section in itertools.product(arms, repeat=len(costs)):
        total += 1
        if rationalisable(constraints_for(section, costs, arms, k), k):
            good += 1
            witnesses.append(section)
    print("check one: the decider against P1's direction sweep")
    print("  regions %d, arms %d, sections %d" % (len(costs), len(arms), total))
    print("  AN argmin, ties allowed   (this decider) : %d" % good)
    print("  THE argmin, ties broken   (P1's sweep)   : 9")
    print()
    print("  the two differ, and the difference is exactly the tie degeneracy.")

    # the two packed arms cost the same on the residency axis, so a weighting that
    # ignores time cannot tell them apart, and every mixture of them across the six
    # regions is weakly admissible.
    tied = sorted(a for a in arms if BITS[a] == 13)
    mixtures = set(itertools.product(tied, repeat=len(costs)))
    strict = strict_sections(costs, arms)
    union = mixtures | strict
    print("  arms tied on the residency axis          : %s" % ", ".join(
        a.replace("bitpack-carrier-", "") for a in tied))
    print("  mixtures of those across %d regions       : %d^%d = %d"
          % (len(costs), len(tied), len(costs), len(mixtures)))
    print("  strict sections, recomputed here         : %d" % len(strict))
    print("  in both sets                             : %d" % len(mixtures & strict))
    print("  union                                    : %d" % len(union))
    print("  decider's count                          : %d" % good)
    print("  the union IS the decider's set           : %s" % (union == set(witnesses)))
    print()
    print("  So the honest figure for the claim in section 2.4 is the weak one: %d of" % good)
    print("  %d, or %.3f%%, because at a tie the design really may take either arm."
          % (total, 100.0 * good / total))
    print("  Both figures are three orders below the section count.")
    return good


# ------------------------------------------------------------------ check two


def check_two():
    """Three cost dimensions, where a direction sweep does not apply.

    Two cases with a known answer, so the decider is shown able to say no as well
    as yes. A decider that always says yes would pass check one too.
    """
    print()
    print("check two: three cost dimensions, two cases with a known answer")
    arms = ["a", "b", "c"]
    k = 3

    # case A. Arm 'a' is the unique minimum on EVERY coordinate in both regions, so
    # every weighting picks it and the section (a, a) must be rationalisable.
    regA = [
        {"a": (F(1), F(1), F(1)), "b": (F(2), F(2), F(2)), "c": (F(3), F(3), F(3))},
        {"a": (F(1), F(1), F(1)), "b": (F(4), F(4), F(4)), "c": (F(5), F(5), F(5))},
    ]
    ok = rationalisable(constraints_for(("a", "a"), regA, arms, k), k)
    print("  A: a dominates everywhere, section (a, a)      -> %s  (expected True)" % ok)
    bad = rationalisable(constraints_for(("b", "b"), regA, arms, k), k)
    print("  A: a dominates everywhere, section (b, b)      -> %s  (expected False)" % bad)

    # case B. A section that picks a cheap-on-axis-0 arm in one region and a
    # cheap-on-axis-1 arm in the other, where the two demands on the weighting
    # contradict each other.
    regB = [
        {"a": (F(1), F(9), F(5)), "b": (F(9), F(1), F(5)), "c": (F(5), F(5), F(5))},
        {"a": (F(1), F(9), F(5)), "b": (F(9), F(1), F(5)), "c": (F(5), F(5), F(5))},
    ]
    same = rationalisable(constraints_for(("a", "a"), regB, arms, k), k)
    print("  B: identical regions, section (a, a)           -> %s  (expected True)" % same)
    contra = rationalisable(constraints_for(("a", "b"), regB, arms, k), k)
    print("  B: identical regions, section (a, b)           -> %s  (expected True)" % contra)
    print("     I expected False here and the decider was right. All three arms share")
    print("     the third coordinate, so the weighting (0, 0, 1) makes every arm tie,")
    print("     and under a tie either pick is an argmin. The case is kept with the")
    print("     correction rather than deleted.")

    # so a case that really is contradictory needs no coordinate on which the arms
    # all agree: then no weighting can tie them and the section must be refused.
    regC = [
        {"a": (F(1), F(9)), "b": (F(9), F(1)), "c": (F(4), F(4))},
        {"a": (F(1), F(9)), "b": (F(9), F(1)), "c": (F(4), F(4))},
    ]
    contra2 = rationalisable(constraints_for(("a", "b"), regC, ["a", "b", "c"], 2), 2)
    print("  C: identical regions, no shared coordinate, section (a, b)")
    print("                                                 -> %s  (expected False)" % contra2)
    print()
    print("  A section that picks differently in two identical regions, where nothing")
    print("  ties the arms, cannot be an argmin of one weighting, and the decider says")
    print("  so. That is the shape of every inconsistency the check exists to catch,")
    print("  and case B is the reminder that a tie is not an inconsistency.")
    return ok and not bad and same and contra and not contra2


def main():
    print("P9. rationalisability as a decision procedure")
    print("exact rational arithmetic throughout, no sampling and no tolerance")
    print()
    good = check_one()
    ok = check_two()
    print()
    print("=" * 66)
    print("decider's weak count on the committed data, decomposed exactly   : %d" % good)
    print("decider answers every constructed case correctly                : %s" % ok)
    print()
    print("So the constraint section 2.4 proposes is not merely stateable, it is")
    print("decidable, in the number of cost dimensions the design actually uses, by")
    print("a procedure that runs once where a table is written and never at compile")
    print("time or at run time. What it costs to keep is one offline check.")


if __name__ == "__main__":
    main()
