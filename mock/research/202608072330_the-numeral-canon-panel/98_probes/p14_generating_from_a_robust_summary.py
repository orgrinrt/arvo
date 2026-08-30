#!/usr/bin/env python3
"""p14. If the table is generated, it can be generated better than from a point estimate.

p10 found that a fixed weighting applied to the committed medians produces a
section that is one draw from a wide distribution: 30, 8 and 77 distinct sections
across 2000 resamples, and for one of the three weightings the committed section
is not even the modal one.

That is a criticism of the point estimate, not of the weighting, and it suggests
a cheap improvement available only under the generation order of section 3.
A generator has the samples, not just their medians, so it can pick the section
that most resamples agree on rather than the section the medians happen to give.

Two selection rules, compared:

  point       apply the weighting to the median of each arm's samples, which is
              what a person reading a findings file does and what a hand-written
              table records.
  modal       apply the weighting to each of many resamples and take the section
              that wins most often.

The measurement is agreement: what fraction of resamples produce each rule's
section. A rule whose section is produced by more resamples is the one a rerun
is more likely to reproduce, which is the only sense in which a section can be
"right" when the underlying quantity is an estimate.

This is not a bench. It is an uncertainty estimate over a committed artifact,
and it takes no new measurement.
"""

import csv
import glob
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

RESAMPLES = 2000
SEED = 20260814


def raw():
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


def short(sec):
    return [a.replace("bitpack-carrier-", "") for a in sec]


def main():
    per_region = raw()
    base = table_from(per_region)
    rng = random.Random(SEED)
    tables = [table_from(per_region, rng) for _ in range(RESAMPLES)]

    print(f"resamples {RESAMPLES}, seed {SEED}, committed samples only, no new measurement\n")

    improved = 0
    for label, w in WEIGHTINGS:
        point = section(base, w)
        counts = Counter(section(t, w) for t in tables)
        modal, modal_n = counts.most_common(1)[0]
        point_n = counts.get(point, 0)
        print(f"{label}")
        print(f"  point-estimate section   {short(point)}")
        print(f"    reproduced by {point_n} of {RESAMPLES} resamples "
              f"({100.0 * point_n / RESAMPLES:.1f}%)")
        print(f"  modal section            {short(modal)}")
        print(f"    reproduced by {modal_n} of {RESAMPLES} resamples "
              f"({100.0 * modal_n / RESAMPLES:.1f}%)")
        if modal != point:
            improved += 1
            diff = [(i, a, b) for i, (a, b) in enumerate(zip(point, modal)) if a != b]
            print(f"  THEY DIFFER at {len(diff)} of {len(point)} regions, and the modal "
                  f"one is reproduced {modal_n - point_n} times more often")
            for i, a, b in diff:
                print(f"    region index {i}: point picks {a.replace('bitpack-carrier-', '')}, "
                      f"modal picks {b.replace('bitpack-carrier-', '')}")
        else:
            print("  they agree, so the point estimate was already the most reproducible one")
        print()

    print(f"weightings where the two rules disagree: {improved} of {len(WEIGHTINGS)}\n")

    print("reading")
    print("  The improvement is available only under the generation order. A")
    print("  hand-written table records a winner somebody read off a findings")
    print("  file, and a findings file reports medians. A generator holds the")
    print("  samples, so it can pick the section a rerun is most likely to")
    print("  reproduce instead of the section this run happened to produce.")
    print()
    print("  It is a small win and it is cheap to keep: the generator already has")
    print("  the samples, and the extra cost is resampling them once, offline, at")
    print("  the moment the table is written. It costs nothing at compile time and")
    print("  nothing at run time, which is the bound on taking a win.")
    print()
    print("  What it does NOT do is make the section stable. The modal section for")
    print("  the least stable weighting is still reproduced by only a minority of")
    print("  resamples, so the underlying instability is a property of the")
    print("  measurement rather than of the selection rule. Choosing the modal one")
    print("  makes the shipped answer the most likely one; it does not make it")
    print("  likely.")


if __name__ == "__main__":
    main()
