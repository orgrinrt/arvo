#!/usr/bin/env python3
"""p12. Is the spread coordinate a decision axis or the harness measuring itself.

p1b reports that adding a third coordinate, the interquartile range of the
per-batch samples, rescues `bitpack-carrier-packed` from being dominated in
every region: it returns to the Pareto front at 4 of 6 regions. p9 reports that
a weighting on that coordinate produces a section neither of the other two
produces. Both of those rest on the spread differences between arms being real
rather than being the sampling noise of an estimate computed from 80 samples.

That is testable on the committed data and it should be tested rather than left
as a caveat, because if the spread differences are not distinguishable from zero
then p1b's rescue is an artifact and `97` section 10's two-arm dominance finding
stands unqualified.

Method. For each region and each pair of arms on the Pareto front under the
three-coordinate model, bootstrap the interquartile range of each arm's 80
committed samples and put a two-sided 95% interval on the difference. A CI
crossing zero means that pair's spread ordering is not measurable, and the
rescue that depends on it is not either.

The specific pairs that matter are the ones that DO the rescuing: at each region
where `packed` is on the three-coordinate front but off the two-coordinate one,
the arm that dominates it under (time, bytes) must fail to dominate it under
(time, bytes, spread), which happens exactly because its spread is worse. That
is one comparison per region and it is the whole load-bearing set.
"""

import csv
import glob
import os
import random
import re
import statistics

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

RESAMPLES = 4000
SEED = 20260814


def samples():
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = {}
        for row in csv.DictReader(open(path)):
            per.setdefault(row["variant"], []).append(float(row["algo_ns"]))
        out[n] = per
    return out


def iqr(xs):
    q = statistics.quantiles(sorted(xs), n=4)
    return q[2] - q[0]


def med(xs):
    return statistics.median(xs)


def boot_diff(a, b, stat, rng):
    diffs = []
    for _ in range(RESAMPLES):
        ra = [a[rng.randrange(len(a))] for _ in range(len(a))]
        rb = [b[rng.randrange(len(b))] for _ in range(len(b))]
        diffs.append(stat(ra) - stat(rb))
    diffs.sort()
    return (diffs[int(0.025 * len(diffs))], diffs[int(0.975 * len(diffs)) - 1])


def dominates(x, y):
    return all(u <= v for u, v in zip(x, y)) and any(u < v for u, v in zip(x, y))


def main():
    per_region = samples()
    regions = sorted(per_region)
    arms = sorted(BYTES_PER_ELEM)
    rng = random.Random(SEED)

    table3 = {n: {a: (med(per_region[n][a]), BYTES_PER_ELEM[a], iqr(per_region[n][a]))
                  for a in arms} for n in regions}
    table2 = {n: {a: (med(per_region[n][a]), BYTES_PER_ELEM[a]) for a in arms}
              for n in regions}

    target = "bitpack-carrier-packed"
    print(f"the arm whose verdict the third coordinate changes: {target}")
    print(f"resamples {RESAMPLES}, seed {SEED}, two-sided 95% bootstrap intervals\n")

    load_bearing = 0
    measurable = 0
    for n in regions:
        dom2 = [b for b in arms if b != target and dominates(table2[n][b], table2[n][target])]
        dom3 = [b for b in arms if b != target and dominates(table3[n][b], table3[n][target])]
        status = ("rescued by spread" if dom2 and not dom3
                  else "dominated under both" if dom3
                  else "on the front under both")
        print(f"n = {n:8d}  {status}")
        if not (dom2 and not dom3):
            continue
        load_bearing += 1
        for b in dom2:
            a_s = per_region[n][b]
            b_s = per_region[n][target]
            lo, hi = boot_diff(a_s, b_s, iqr, rng)
            obs = iqr(a_s) - iqr(b_s)
            spans = lo <= 0 <= hi
            if not spans:
                measurable += 1
            print(f"    its (time, bytes) dominator: {b}")
            print(f"      IQR difference (dominator minus {target.split('-')[-1]}): "
                  f"{obs:10.1f} ns   CI [{lo:.1f}, {hi:.1f}]   "
                  f"{'NOT measurable' if spans else 'measurable'}")
            lo2, hi2 = boot_diff(a_s, b_s, med, rng)
            print(f"      median difference for scale:                  "
                  f"{med(a_s) - med(b_s):10.1f} ns   CI [{lo2:.1f}, {hi2:.1f}]")
    print()
    print(f"regions where the third coordinate does the rescuing: {load_bearing}")
    print(f"of those, comparisons whose spread difference is measurable: {measurable}")
    print()

    print("and the same question asked of every arm pair, so the answer is not")
    print("about one arm")
    total = meas = 0
    for n in regions:
        for i, a in enumerate(arms):
            for b in arms[i + 1:]:
                lo, hi = boot_diff(per_region[n][a], per_region[n][b], iqr, rng)
                total += 1
                if not (lo <= 0 <= hi):
                    meas += 1
    print(f"  arm pairs across all regions: {total}")
    print(f"  pairs whose IQR difference excludes zero: {meas} "
          f"({100.0 * meas / total:.1f}%)")
    print()

    print("reading")
    if load_bearing and measurable == load_bearing:
        print("  Every comparison the rescue depends on has a spread difference that")
        print("  excludes zero, so the third coordinate is separating arms rather")
        print("  than separating one estimate from another. p1b's rescue stands and")
        print("  `97` section 10's two-arm finding is coordinate-relative as p1b")
        print("  states it.")
    elif measurable == 0:
        print("  None of the comparisons the rescue depends on has a measurable")
        print("  spread difference, so the rescue is an artifact of an estimate and")
        print("  the third coordinate should be dropped. `97` section 10's two-arm")
        print("  finding then stands unqualified and p1b's rescue is withdrawn.")
    else:
        print(f"  {measurable} of {load_bearing} of the load-bearing comparisons are")
        print("  measurable, so the rescue holds at some regions and not others and")
        print("  the honest form of p1b's claim names those regions.")
    print()
    print("  What this does NOT establish: that the spread of a bench arm is a")
    print("  thing a consumer should weigh. It establishes only that the arms")
    print("  differ on it by more than the estimate's own noise. Whether tail")
    print("  behaviour belongs in the coordinate set is a design question and this")
    print("  probe does not touch it.")


if __name__ == "__main__":
    main()
