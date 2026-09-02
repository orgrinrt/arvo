#!/usr/bin/env python3
"""Closing my own open item: do the two admissibility tests actually disagree?

Section 5 of `101` proposes two measured tests for whether an estimator may be a
cost coordinate, and reports that they disagree about the tail excess:

  RESOLUTION.    Does the estimator's signal clear its own noise floor, measured
                 against the family's byte-identical control pair?
  EXPRESSIVENESS. Does the estimator add sections a strictly positive weighting
                 can reach, over `{median, candidate}`?

The resolution number `p5` reports is a dynamic range, `(max - min)/min` across
arms, which one extreme arm can carry. `100`'s p8 uses a better statistic for the
same purpose: the fraction of arm PAIRS whose bootstrap confidence interval for
the difference excludes zero. `101` section 10 records not having built it as
something I could not settle. This builds it.

So the table below carries three numbers per estimator per family: the noise
floor against the control pair, the fraction of arm pairs separated, and the
exact count of reachable sections. If separation and expressiveness rank the
candidates the same way, section 5's composition collapses to one test. If they
disagree, the composition stands and the disagreement is the result.

One resample is drawn per arm per region and sorted once, and every estimator is
computed from that same sorted resample, so the estimators are compared on
identical draws rather than on independent ones.

This reads committed artifacts. It is NOT a bench, no measurement was taken,
and no number here prices anything.

Run:  python3 p11_separation_against_expressiveness.py
"""

import csv
import glob
import itertools
import math
import os
import random
import re
import statistics
from collections import defaultdict
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

SEED = 20260814
RESAMPLES = 1000

PAIRS = {
    "bitpack-carrier-width": ("bitpack-carrier-d16", "bitpack-carrier-d16-control"),
    "bitpack-contend-decode": ("bitpack-contend-d16", "bitpack-contend-d16-control"),
    "bitpack-contention": ("bitpack-contend-d16", "bitpack-contend-d16-control"),
    "bitpack-wide": ("bitpack-wide-d16", "bitpack-wide-d16-control"),
}


def qs(xs, p):
    """Quantile of an ALREADY SORTED list."""
    if len(xs) == 1:
        return xs[0]
    k = p * (len(xs) - 1)
    lo = int(math.floor(k))
    hi = min(lo + 1, len(xs) - 1)
    return xs[lo] + (xs[hi] - xs[lo]) * (k - lo)


def all_estimators(sorted_xs):
    """Every candidate, from one sorted sample. Order fixed by NAMES."""
    med = qs(sorted_xs, 0.5)
    p95 = qs(sorted_xs, 0.95)
    p99 = qs(sorted_xs, 0.99)
    mad = statistics.median(sorted([abs(x - med) for x in sorted_xs]))
    return (
        med,
        statistics.fmean(sorted_xs),
        qs(sorted_xs, 0.75) - qs(sorted_xs, 0.25),
        qs(sorted_xs, 0.90) - qs(sorted_xs, 0.10),
        mad,
        p95,
        p99,
        p95 - med,
        p99 - med,
    )


NAMES = [
    "median",
    "mean",
    "iqr  (spread)",
    "idr  (spread)",
    "mad  (spread)",
    "p95  (level)",
    "p99  (level)",
    "p95-med (excess)",
    "p99-med (excess)",
]


def samples(family):
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, f"{family}_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = defaultdict(list)
        with open(path) as fh:
            for row in csv.DictReader(fh):
                per[row["variant"]].append(float(row["algo_ns"]))
        out[n] = dict(per)
    return out


def bootstrap(per_region, arms, rng):
    """RESAMPLES estimates of every estimator, per arm per region."""
    out = {}
    for n, per in per_region.items():
        out[n] = {}
        for a in arms:
            xs = per[a]
            k = len(xs)
            draws = []
            for _ in range(RESAMPLES):
                r = sorted(xs[rng.randrange(k)] for _ in range(k))
                draws.append(all_estimators(r))
            out[n][a] = draws
    return out


def feasible2(constraints):
    lo, hi = Fraction(0), Fraction(1)
    lo_open = hi_open = True
    for g in constraints:
        a, c = g[0] - g[1], g[1]
        if a == 0:
            if c < 0:
                return False
            continue
        b = -Fraction(c, a)
        if a > 0:
            if b > lo:
                lo, lo_open = b, False
        else:
            if b < hi:
                hi, hi_open = b, False
    if lo > hi:
        return False
    if lo == hi and (lo_open or hi_open):
        return False
    return True


def reachable(table, arms):
    regions = sorted(table)
    n = 0
    for sec in itertools.product(arms, repeat=len(regions)):
        cons = []
        for r, a in zip(regions, sec):
            for b in arms:
                if b != a:
                    cons.append(tuple(table[r][b][i] - table[r][a][i] for i in range(2)))
        if feasible2(cons):
            n += 1
    return n


def main():
    rng = random.Random(SEED)
    print("=" * 78)
    print("THREE NUMBERS PER ESTIMATOR: FLOOR, SEPARATION, EXPRESSIVENESS")
    print("=" * 78)
    print("floor      = 95th percentile of |E(A) - E(B)| / E(A) across resamples,")
    print("             for the two arms the bench declares byte-identical, in percent")
    print("separated  = arm pairs whose 95% bootstrap interval for the difference")
    print("             excludes zero, over the arms that genuinely differ")
    print("sections   = exact count reachable by a strictly positive weighting on")
    print("             {median, candidate}; 1 means the candidate adds nothing")
    print()
    for fam, (ca, cb) in PAIRS.items():
        per = samples(fam)
        arms = sorted(set.intersection(*[set(p) for p in per.values()]))
        real = [a for a in arms if not a.endswith("-control")]
        boot = bootstrap(per, arms, rng)
        n_pairs = len(real) * (len(real) - 1) // 2 * len(per)
        enumerable = len(real) ** len(per) <= 100000
        print(f"  {fam}   arms={len(real)} regions={len(per)} pairs={n_pairs}"
              f"{'' if enumerable else '   (too large to enumerate sections)'}")
        print(f"    {'estimator':18s} {'floor %':>9s} {'separated':>12s} {'sections':>9s}")
        seps, sects = [], []
        for i, name in enumerate(NAMES):
            # floor, from the control pair
            floors = []
            for n in sorted(per):
                d = sorted(
                    abs(boot[n][ca][j][i] - boot[n][cb][j][i]) / abs(boot[n][ca][j][i])
                    for j in range(RESAMPLES)
                    if boot[n][ca][j][i]
                )
                floors.append(qs(d, 0.95) * 100)
            floor = statistics.fmean(floors)
            # separation, over pairs of genuinely different arms
            sep = 0
            for n in sorted(per):
                for a, b in itertools.combinations(real, 2):
                    diffs = sorted(
                        boot[n][a][j][i] - boot[n][b][j][i] for j in range(RESAMPLES)
                    )
                    lo, hi = qs(diffs, 0.025), qs(diffs, 0.975)
                    sep += (lo > 0) or (hi < 0)
            # expressiveness
            if enumerable:
                t = {
                    n: {
                        a: (
                            Fraction(statistics.median(per[n][a])).limit_denominator(10**12),
                            Fraction(all_estimators(sorted(per[n][a]))[i]).limit_denominator(10**12),
                        )
                        for a in real
                    }
                    for n in per
                }
                r = reachable(t, real)
                sect = str(r)
                seps.append(sep)
                sects.append(r)
            else:
                sect = "-"
            print(
                f"    {name:18s} {floor:9.2f} {sep:6d} of {n_pairs:<3d} {sect:>9s}"
            )
        if len(seps) > 2:
            print(
                f"    correlation between separation and sections, over the "
                f"{len(seps)} candidates: {statistics.correlation(seps, sects):+.3f}"
            )
        print()

    print("=" * 78)
    print("WHAT THE TWO TESTS DO TO EACH OTHER")
    print("=" * 78)
    print("  If separation and expressiveness ranked candidates alike, section 5's")
    print("  composition would collapse into one test and the simpler one would win.")
    print("  Read the two columns against each other per family above.")


if __name__ == "__main__":
    main()
