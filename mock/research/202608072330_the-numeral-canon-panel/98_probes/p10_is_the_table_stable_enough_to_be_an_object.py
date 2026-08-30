#!/usr/bin/env python3
"""p10. Is the section stable under the measurement noise of its own bench run.

`97` section 2.4 proposes that the section is the design-tier artifact and the
objective is the canon-tier one, on the permanence argument that "a canon
stating the table is wrong the next time somebody measures". That argument is
usually made about a rerun on different hardware, or a year later. It can be
made much more cheaply and much more sharply, against the run that already
exists.

The harness took 80 samples per variant per region. Resampling those 80 with
replacement gives another table the same run could plausibly have produced.
If the section a fixed weighting selects moves under that resampling, then the
table is not stable across a rerun of the same bench on the same machine on the
same afternoon, and the permanence argument is not a prediction about the future
but a fact about the data in hand.

Three things measured, all at a fixed weighting so that only the measurement
moves:

  1. How many distinct sections a fixed weighting produces across resamples, and
     how often the modal one appears.
  2. Which regions are the unstable ones, since a section that is stable at five
     of six regions and coin-flips at the sixth is a different thing from one
     that is unstable everywhere.
  3. Whether the set of arms dominated in every region is stable, since p1b and
     `97` section 10 both rest on that set.

This does not measure hardware variation, a different toolchain, or a different
workload. It measures the weakest kind of instability there is, which makes any
instability it finds a lower bound.
"""

import csv
import glob
import json
import os
import random
import re
import statistics
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

BYTES_PER_ELEM = {
    "bitpack-carrier-d16": 2.0,
    "bitpack-carrier-d16-control": 2.0,
    "bitpack-carrier-d32": 4.0,
    "bitpack-carrier-d64": 8.0,
    "bitpack-carrier-packed": 13.0 / 8.0,
    "bitpack-carrier-packed-simd": 13.0 / 8.0,
}

WEIGHTINGS = [
    ("speed-first", (1.0, 1.0 / 32, 1.0 / 32)),
    ("storage-first", (1.0 / 32, 1.0, 1.0 / 32)),
    ("tail-first", (1.0 / 32, 1.0 / 32, 1.0)),
]


def raw_samples():
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = {}
        for row in csv.DictReader(open(path)):
            per.setdefault(row["variant"], []).append(float(row["algo_ns"]))
        out[n] = per
    return out


def table_from(per_region, rng=None):
    table = {}
    for n, per in per_region.items():
        row = {}
        for arm, s in per.items():
            xs = s if rng is None else [s[rng.randrange(len(s))] for _ in range(len(s))]
            xs = sorted(xs)
            q = statistics.quantiles(xs, n=4)
            row[arm] = (statistics.median(xs), BYTES_PER_ELEM[arm], q[2] - q[0])
        table[n] = row
    return table


def normalise(table):
    regions = sorted(table)
    arms = sorted(table[regions[0]])
    lo = [min(table[r][a][k] for r in regions for a in arms) for k in range(3)]
    hi = [max(table[r][a][k] for r in regions for a in arms) for k in range(3)]
    return {r: {a: tuple((table[r][a][k] - lo[k]) / (hi[k] - lo[k]) if hi[k] > lo[k] else 0.0
                         for k in range(3)) for a in arms} for r in regions}


def section(table, w):
    regions = sorted(table)
    arms = sorted(table[regions[0]])
    tab = normalise(table)
    return tuple(min(arms, key=lambda a: (sum(wi * ci for wi, ci in zip(w, tab[r][a])), a))
                 for r in regions)


def dominates(x, y):
    return all(a <= b for a, b in zip(x, y)) and any(a < b for a, b in zip(x, y))


def dominated_everywhere(table):
    regions = sorted(table)
    arms = sorted(table[regions[0]])
    out = []
    for a in arms:
        if all(any(dominates(table[r][b], table[r][a]) for b in arms if b != a)
               for r in regions):
            out.append(a)
    return tuple(sorted(out))


def short(sec):
    return [a.replace("bitpack-carrier-", "") for a in sec]


def main():
    per_region = raw_samples()
    regions = sorted(per_region)
    base = table_from(per_region)
    counts = {n: len(next(iter(per_region[n].values()))) for n in regions}
    print(f"regions {len(regions)}, samples per arm per region: "
          f"{sorted(set(counts.values()))}")
    print("resamples: 2000, seed 20260814, bootstrap with replacement over the "
          "committed samples\n")

    rng = random.Random(20260814)
    tables = [table_from(per_region, rng) for _ in range(2000)]

    for label, w in WEIGHTINGS:
        obs = section(base, w)
        secs = Counter(section(t, w) for t in tables)
        modal, freq = secs.most_common(1)[0]
        print(f"{label}")
        print(f"  section on the committed medians: {short(obs)}")
        print(f"  distinct sections across resamples: {len(secs)}")
        print(f"  modal section appears {freq} of 2000 ({100.0 * freq / 2000:.1f}%)"
              f"{'  and is the committed one' if modal == obs else '  and is NOT the committed one'}")
        unstable = []
        for i, r in enumerate(regions):
            per_r = Counter(s[i] for s in secs.elements())
            if len(per_r) > 1:
                top = per_r.most_common()
                unstable.append((r, [(a.replace('bitpack-carrier-', ''), c) for a, c in top]))
        if unstable:
            print("  regions where the pick moves:")
            for r, dist in unstable:
                print(f"    n = {r:8d}: {dist}")
        else:
            print("  no region's pick moves")
        for s, c in secs.most_common(4):
            print(f"    {c:5d}  {short(s)}")
        print()

    print("dominated in every region")
    base_dom = dominated_everywhere(base)
    doms = Counter(dominated_everywhere(t) for t in tables)
    print(f"  on the committed medians: "
          f"{[a.replace('bitpack-carrier-', '') for a in base_dom]}")
    for s, c in doms.most_common():
        print(f"    {c:5d}  {[a.replace('bitpack-carrier-', '') for a in s]}")
    print()

    print("reading")
    print("  Every number above comes from ONE bench run, resampled. No new")
    print("  measurement was taken and none of this is a bench: it is an")
    print("  uncertainty estimate over a committed artifact.")
    print()
    print("  Where a section moves under this, it would move under a rerun, and")
    print("  a canon sentence naming it would be wrong by the following week.")
    print("  Where it does not move, this establishes only that it survives the")
    print("  weakest perturbation available, which is not the same as permanence.")


if __name__ == "__main__":
    main()
