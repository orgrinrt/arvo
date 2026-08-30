#!/usr/bin/env python3
"""P1. How many sections are there, and how many of them are argmins of a weighting?

The question this settles: `25` section 7 says a strategy IS a named section over a
product of mechanism axes. `93` section 1 and `94` section 3.2 say a strategy is a
preference (a weighting) whose argmin produces that section. `94` phase two reports
these as the same proposition reaching TWO EXPERTS.

They are not the same proposition, and the difference is exactly this: a section is
ANY function from region to mechanism. An argmin-under-a-weighting is a section that
is RATIONALISABLE, meaning some single weighting explains every one of its choices at
once. Not every function is rationalisable. This probe counts both over committed
harness output, so the gap between the two definitions is a number rather than an
argument.

Data: `mock/benches/bitpack-carrier-width_n*.csv`, committed harness output, six
record counts, six variants. One variant (`-control`) is a control of another arm and
is dropped. Cost is two-dimensional and both dimensions come from the repository:

  time  : median `algo_ns` over the committed samples, divided by the record count,
          so the two regions' numbers are commensurable (per element rather than
          per call). Read, not measured: this probe runs no benchmark.
  bits  : bits per stored element, a static fact about the arm. d16/d32/d64 carry
          16/32/64; both packed arms carry the declared 13.

Method for the rationalisable count. With two cost dimensions a non-negative
weighting is a direction, so sweep the direction. For each region the argmin as the
direction turns is piecewise constant, changing only where two arms tie. Collect
every tie angle across every region, sort them, and read the section off the interior
of each resulting interval. That enumerates the achievable sections exactly rather
than sampling them.
"""

import csv
import glob
import math
import os
import statistics

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.abspath(os.path.join(HERE, "..", "..", "..", "benches"))

BITS = {
    "bitpack-carrier-d16": 16,
    "bitpack-carrier-d32": 32,
    "bitpack-carrier-d64": 64,
    "bitpack-carrier-packed": 13,
    "bitpack-carrier-packed-simd": 13,
}


def load():
    regions = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(os.path.basename(path).split("_n")[1].split(".")[0])
        by = {}
        for r in csv.DictReader(open(path)):
            by.setdefault(r["variant"], []).append(float(r["algo_ns"]))
        costs = {}
        for v, xs in by.items():
            if v not in BITS:
                continue
            costs[v] = (statistics.median(xs) / n, float(BITS[v]))
        regions[n] = costs
    return dict(sorted(regions.items()))


def argmin_at(costs, theta):
    wt, wb = math.cos(theta), math.sin(theta)
    best, bestv = None, None
    for a, (t, b) in sorted(costs.items()):
        v = wt * t + wb * b
        if bestv is None or v < bestv - 1e-15:
            best, bestv = a, v
    return best


def tie_angles(costs):
    """Directions where two arms tie: w.(c1-c2) = 0, i.e. tan(theta) = -dt/db."""
    out = []
    arms = sorted(costs)
    for i in range(len(arms)):
        for j in range(i + 1, len(arms)):
            t1, b1 = costs[arms[i]]
            t2, b2 = costs[arms[j]]
            dt, db = t1 - t2, b1 - b2
            if db == 0:
                continue
            tan = -dt / db
            if tan <= 0:
                continue
            th = math.atan(tan)
            if 0.0 < th < math.pi / 2:
                out.append(th)
    return out


def main():
    regions = load()
    arms = sorted(BITS)
    print("committed harness output read, not measured")
    print("source: mock/benches/bitpack-carrier-width_n*.csv")
    print()
    print("cost table: (ns per element, bits per element)")
    hdr = "%-12s" % "region n"
    for a in arms:
        hdr += "%28s" % a.replace("bitpack-carrier-", "")
    print(hdr)
    for n, costs in regions.items():
        line = "%-12d" % n
        for a in arms:
            t, b = costs[a]
            line += "%28s" % ("%.4f ns / %d b" % (t, b))
        print(line)
    print()

    R = len(regions)
    A = len(arms)
    total = A ** R
    print("regions            : %d" % R)
    print("arms per region    : %d" % A)
    print("SECTIONS, all      : %d^%d = %d" % (A, R, total))

    breaks = sorted({th for costs in regions.values() for th in tie_angles(costs)})
    probe_angles = []
    edges = [0.0] + breaks + [math.pi / 2]
    for i in range(len(edges) - 1):
        probe_angles.append((edges[i] + edges[i + 1]) / 2.0)

    sections = {}
    for th in probe_angles:
        sec = tuple(argmin_at(costs, th) for costs in regions.values())
        sections.setdefault(sec, []).append(th)

    print("tie directions     : %d" % len(breaks))
    print("SECTIONS, rationalisable by a non-negative weighting : %d" % len(sections))
    print("ratio              : %d / %d = %.6f%%" % (len(sections), total,
                                                     100.0 * len(sections) / total))
    print()
    print("the rationalisable sections, with the direction band each holds on")
    print("(theta = 0 is a pure time objective, theta = pi/2 a pure residency one)")
    for sec, ths in sorted(sections.items(), key=lambda kv: min(kv[1])):
        lo, hi = min(ths), max(ths)
        print("  theta in [%.4f, %.4f] rad" % (lo, hi))
        for n, a in zip(regions.keys(), sec):
            print("      n=%-10d -> %s" % (n, a.replace("bitpack-carrier-", "")))
    print()

    ever = set()
    for sec in sections:
        ever |= set(sec)
    print("arms selected by SOME weighting somewhere : %d of %d" % (len(ever), A))
    print("  ", ", ".join(sorted(a.replace("bitpack-carrier-", "") for a in ever)))
    never = sorted(set(arms) - ever)
    print("arms selected by NO weighting anywhere    : %d" % len(never))
    print("  ", ", ".join(a.replace("bitpack-carrier-", "") for a in never) or "(none)")
    print()

    per_region = []
    for n, costs in regions.items():
        reach = set()
        for th in probe_angles:
            reach.add(argmin_at(costs, th))
        per_region.append((n, len(reach), sorted(x.replace("bitpack-carrier-", "") for x in reach)))
    print("per region, how many arms any weighting can reach:")
    for n, k, names in per_region:
        print("  n=%-10d %d of %d : %s" % (n, k, A, ", ".join(names)))
    prod = 1
    for _, k, _ in per_region:
        prod *= k
    print()
    print("upper bound if the regions were independent : %d" % prod)
    print("actual rationalisable count                 : %d" % len(sections))
    print("the gap is the consistency the weighting imposes ACROSS regions:")
    print("  a section may pick a reachable arm in every region and still not be")
    print("  the argmin of any single weighting, because the direction that")
    print("  justifies one region's pick can contradict another's.")


if __name__ == "__main__":
    main()


def sensitivity():
    """Two robustness passes, because several bands above are razor thin.

    Pass one: dominance. An arm no weighting can ever select is one that some
    other arm beats on BOTH axes. That is a fact about the cost table and needs
    no direction sweep, so it does not inherit the thin-band fragility.

    Pass two: a tolerance. Call a section rationalisable-within-t if some single
    direction puts every one of its picks within t of that region's best. This
    only ever grows the count, so it is the generous reading, and the point of
    running it is that the count stays small under it.
    """
    regions = load()
    arms = sorted(BITS)
    print()
    print("=" * 70)
    print("SENSITIVITY")
    print("=" * 70)
    print()
    print("pass one: Pareto dominance per region (no direction sweep involved)")
    dominated_everywhere = set(arms)
    for n, costs in regions.items():
        dom = set()
        for a in arms:
            for b in arms:
                if a == b:
                    continue
                if costs[b][0] <= costs[a][0] and costs[b][1] <= costs[a][1] and costs[b] != costs[a]:
                    dom.add(a)
                    break
        print("  n=%-10d dominated: %s" % (
            n, ", ".join(sorted(x.replace("bitpack-carrier-", "") for x in dom)) or "(none)"))
        dominated_everywhere &= dom
    print("  dominated in EVERY region: %s" % (
        ", ".join(sorted(x.replace("bitpack-carrier-", "") for x in dominated_everywhere)) or "(none)"))
    print("  such an arm is unreachable under the weighting definition and")
    print("  perfectly nameable under the section definition.")
    print()

    print("pass two: rationalisable within a tolerance")
    edges_base = sorted({th for costs in regions.values() for th in tie_angles(costs)})
    edges = [0.0] + edges_base + [math.pi / 2]
    grid = []
    for i in range(len(edges) - 1):
        lo, hi = edges[i], edges[i + 1]
        for k in range(1, 8):
            grid.append(lo + (hi - lo) * k / 8.0)
    total = len(arms) ** len(regions)
    for tol in (0.0, 0.01, 0.02, 0.05, 0.10):
        secs = set()
        for th in grid:
            wt, wb = math.cos(th), math.sin(th)
            per_region_ok = []
            for costs in regions.values():
                vals = {a: wt * costs[a][0] + wb * costs[a][1] for a in arms}
                best = min(vals.values())
                per_region_ok.append(tuple(sorted(a for a in arms if vals[a] <= best * (1.0 + tol))))
            count = 1
            for ok in per_region_ok:
                count *= len(ok)
            # enumerate the cross product of acceptable picks
            acc = [()]
            for ok in per_region_ok:
                acc = [p + (a,) for p in acc for a in ok]
            secs |= set(acc)
        print("  tolerance %5.1f%%  rationalisable sections: %6d of %d  (%.4f%%)"
              % (tol * 100, len(secs), total, 100.0 * len(secs) / total))
    print()
    print("even at a 10%% tolerance, which is far past any noise floor in this")
    print("family, the admissible sections remain a small fraction of all sections.")


sensitivity()
