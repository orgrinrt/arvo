#!/usr/bin/env python3
"""P1b. The gap P1 measured is structural, not a fact about one bench family.

P1 counted 9 rationalisable sections against 15625 sections on one committed family.
One dataset is one instance and one instance decides nothing, so this file establishes
the same gap by a route that does not use the data at all, and then checks that route
against random cost tables.

The argument. Fix the regions and the arms. A section is any assignment of an arm to
each region, so there are |A|^|E| of them, exponential in the number of regions. A
section is rationalisable when one weighting explains every choice at once. As the
weighting varies over the non-negative orthant, the argmin in one region changes only
when two arms tie, and each tie is a hyperplane through the origin in weight space.
The achievable sections are therefore the cells of an arrangement of at most
|E| * C(|A|, 2) such hyperplanes in k dimensions, which is at most

    sum over i = 0..k-1 of C(H, i)      where H = |E| * C(|A|, 2)

cells. That is POLYNOMIAL in the number of regions, of degree k - 1, against an
EXPONENTIAL count of sections. The gap widens without bound as regions are added, so
it is a property of what a weighting is rather than a property of arvo's benches.

Two checks below. The bound is compared against the exponential for a grid of sizes,
and for two cost dimensions the exact count is enumerated on random cost tables and
compared against the bound, which also confirms the bound is not vacuous.
"""

import itertools
import math
import random


def cell_bound(regions, arms, k):
    H = regions * (arms * (arms - 1) // 2)
    return sum(math.comb(H, i) for i in range(k))


def exact_count_2d(costs):
    """costs[region][arm] = (c1, c2). Enumerate achievable argmin sections exactly."""
    breaks = set()
    for row in costs:
        A = len(row)
        for i in range(A):
            for j in range(i + 1, A):
                d1 = row[i][0] - row[j][0]
                d2 = row[i][1] - row[j][1]
                if d2 == 0:
                    continue
                tan = -d1 / d2
                if tan > 0:
                    th = math.atan(tan)
                    if 0.0 < th < math.pi / 2:
                        breaks.add(th)
    edges = [0.0] + sorted(breaks) + [math.pi / 2]
    secs = set()
    for i in range(len(edges) - 1):
        th = (edges[i] + edges[i + 1]) / 2.0
        w1, w2 = math.cos(th), math.sin(th)
        sec = []
        for row in costs:
            vals = [w1 * c[0] + w2 * c[1] for c in row]
            sec.append(min(range(len(row)), key=lambda a: vals[a]))
        secs.add(tuple(sec))
    return len(secs), len(breaks)


def main():
    print("P1b. the gap between sections and rationalisable sections is structural")
    print()
    print("part one: the counts, from the bound rather than from any data")
    print("%8s %6s %4s %18s %14s %s" % ("regions", "arms", "k", "sections", "bound", "ratio"))
    for k in (2, 3):
        for arms in (4, 6):
            for regions in (2, 4, 6, 8, 12, 20):
                total = arms ** regions
                b = cell_bound(regions, arms, k)
                print("%8d %6d %4d %18d %14d %.3g"
                      % (regions, arms, k, total, b, b / total))
        print()

    print("part two: exact enumeration on random cost tables, two cost dimensions")
    print("checks the bound holds and is not vacuous, over 400 random tables each")
    print("%8s %6s %14s %10s %10s %10s" % ("regions", "arms", "sections", "bound", "max seen", "mean seen"))
    rnd = random.Random(20260814)
    for regions, arms in ((6, 5), (6, 6), (8, 4), (10, 5), (12, 6)):
        b = cell_bound(regions, arms, 2)
        seen = []
        violations = 0
        for _ in range(400):
            costs = [[(rnd.uniform(0.05, 1.0), rnd.choice([13.0, 16.0, 24.0, 32.0, 64.0]))
                      for _ in range(arms)] for _ in range(regions)]
            n, _h = exact_count_2d(costs)
            seen.append(n)
            if n > b:
                violations += 1
        print("%8d %6d %14d %10d %10d %10.2f"
              % (regions, arms, arms ** regions, b, max(seen), sum(seen) / len(seen)))
        assert violations == 0, "the bound was exceeded, the argument is wrong"
    print()
    print("no table exceeded the bound in any trial, and every observed count is")
    print("orders below the section count. So the finding P1 measured on committed")
    print("data is what the shape of the question forces, and the committed data is")
    print("one instance of it rather than the reason for it.")
    print()
    print("What this does NOT say. It does not say the weighting reading is right and")
    print("the section reading is wrong. It says they are different propositions with")
    print("a quantified gap, so an agreement recorded between them is an agreement")
    print("between two claims rather than two derivations of one claim.")


if __name__ == "__main__":
    main()
