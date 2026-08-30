#!/usr/bin/env python3
"""p8. The tail coordinate's problems are a property of the estimator, not of the
idea, and a different estimator of the same intent fixes most of them.

Section 7 of `100` finds one configuration where no differential band both
absorbs the measurement noise and refuses a generator defect, and it is the
tail-weighing one. `98`'s F-98-12 independently finds that the same coordinate's
differences "are not distinguishable from zero, at all four regions where the
rescue occurs", while separating 58 of 90 arm pairs in general. `98` p10 finds
the tail-weighing section by far the least stable of three.

Three findings about one coordinate, all negative, and all of them measured on
ONE estimator of it: the INTERQUARTILE RANGE of the per-batch samples.

THE OBSERVATION THIS PROBE TESTS. The interquartile range is the spread of the
MIDDLE FIFTY PERCENT. It is, by construction, the statistic that discards both
tails. A strategy weighing "tail behaviour" and reading the IQR is reading a
number whose defining property is that the tail has been removed from it, and it
is also a DIFFERENCE of two order statistics, so its sampling variance is roughly
the sum of theirs while its magnitude is the gap between them. Both properties
push the same way: high noise, small signal.

If that is the mechanism, then estimators that keep the tail and are not
differences should be both more stable and more separating, with no change to
what the strategy intends.

SIX ESTIMATORS OF THE SAME INTENT, on the same committed samples:

  iqr        P75 - P25, the one every prior file used
  idr        P90 - P10, interdecile range, still a difference but wider
  mad        median absolute deviation, robust, not a difference of quantiles
  p95        the 95th percentile itself, absolute rather than a spread
  p99        the 99th percentile itself
  trimsd     standard deviation after trimming 10% from each end

TWO MEASUREMENTS PER ESTIMATOR:

  1. STABILITY. Distinct sections across 2000 bootstrap resamples under a
     weighting that reads only that coordinate, so nothing else can move.
  2. SEPARATION. Of all arm pairs at all regions, how many have a bootstrap
     confidence interval on the difference that excludes zero. This is `98`'s
     p12 question asked of six statistics rather than one.

And then the payoff, section 2 of the output: `100` section 7's false alarm sweep
rerun with each statistic in the third coordinate, which is where the band
tension either survives the swap or does not.

A COORDINATE THE INSTRUMENT CANNOT RESOLVE CANNOT SUPPORT A DIFFERENTIAL, which
is section 7's conclusion. This probe asks whether the instrument cannot resolve
the TAIL, or cannot resolve the IQR.

Not a bench. Reads committed harness output and resamples it. No measurement was
taken.

Run:  python3 p8_the_tail_coordinate_is_the_wrong_statistic.py
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
RESAMPLES = 2000
CI_RESAMPLES = 2000
SEED = 20260814

ARMS = [
    "bitpack-carrier-d16",
    "bitpack-carrier-d32",
    "bitpack-carrier-d64",
    "bitpack-carrier-packed",
    "bitpack-carrier-packed-simd",
]


def pct(xs, q):
    """Linear-interpolated percentile on a sorted list."""
    if not xs:
        return 0.0
    k = (len(xs) - 1) * q
    lo, hi = int(k), min(int(k) + 1, len(xs) - 1)
    return xs[lo] + (xs[hi] - xs[lo]) * (k - lo)


def est_iqr(xs):
    return pct(xs, 0.75) - pct(xs, 0.25)


def est_idr(xs):
    return pct(xs, 0.90) - pct(xs, 0.10)


def est_mad(xs):
    m = pct(xs, 0.5)
    return pct(sorted(abs(x - m) for x in xs), 0.5)


def est_p95(xs):
    return pct(xs, 0.95)


def est_p99(xs):
    return pct(xs, 0.99)


def est_trimsd(xs):
    k = max(1, int(0.10 * len(xs)))
    core = xs[k : len(xs) - k]
    return statistics.pstdev(core) if len(core) > 1 else 0.0


ESTIMATORS = [
    ("iqr", est_iqr),
    ("idr", est_idr),
    ("mad", est_mad),
    ("p95", est_p95),
    ("p99", est_p99),
    ("trimsd", est_trimsd),
]


def samples():
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = {}
        with open(path) as fh:
            for row in csv.DictReader(fh):
                if row["variant"] in ARMS:
                    per.setdefault(row["variant"], []).append(float(row["algo_ns"]))
        out[n] = {a: sorted(v) for a, v in per.items()}
    return out


def section(per, est, rng=None):
    """Argmin of the estimator alone, per region. One coordinate, no weighting."""
    out = []
    for n in sorted(per):
        best, bv = None, None
        for a in ARMS:
            xs = per[n][a]
            if rng is not None:
                xs = sorted(xs[rng.randrange(len(xs))] for _ in range(len(xs)))
            v = est(xs)
            if bv is None or v < bv or (v == bv and a < best):
                bv, best = v, a
        out.append(best)
    return tuple(out)


def separation(per, est, rng):
    """Arm pairs at each region whose bootstrap CI on the difference excludes
    zero. `98`'s p12 question, asked of each estimator."""
    sep = tot = 0
    for n in sorted(per):
        for i in range(len(ARMS)):
            for j in range(i + 1, len(ARMS)):
                a, b = ARMS[i], ARMS[j]
                xa, xb = per[n][a], per[n][b]
                diffs = []
                for _ in range(CI_RESAMPLES):
                    ra = sorted(xa[rng.randrange(len(xa))] for _ in range(len(xa)))
                    rb = sorted(xb[rng.randrange(len(xb))] for _ in range(len(xb)))
                    diffs.append(est(ra) - est(rb))
                diffs.sort()
                lo = diffs[int(0.025 * len(diffs))]
                hi = diffs[int(0.975 * len(diffs))]
                tot += 1
                if lo > 0 or hi < 0:
                    sep += 1
    return sep, tot


BYTES = {
    "bitpack-carrier-d16": 2.0,
    "bitpack-carrier-d32": 4.0,
    "bitpack-carrier-d64": 8.0,
    "bitpack-carrier-packed": 13.0 / 8.0,
    "bitpack-carrier-packed-simd": 13.0 / 8.0,
}
BANDS = (0.0, 0.005, 0.01, 0.02, 0.05, 0.10)


def three_coord_table(per, est, rng=None):
    t = {}
    for n in sorted(per):
        row = {}
        for a in ARMS:
            xs = per[n][a]
            if rng is not None:
                xs = sorted(xs[rng.randrange(len(xs))] for _ in range(len(xs)))
            row[a] = (pct(xs, 0.5), BYTES[a], est(xs))
        t[n] = row
    return t


def normalise3(t):
    rs = sorted(t)
    lo = [min(t[r][a][k] for r in rs for a in ARMS) for k in range(3)]
    hi = [max(t[r][a][k] for r in rs for a in ARMS) for k in range(3)]
    return {
        r: {
            a: tuple(
                (t[r][a][k] - lo[k]) / (hi[k] - lo[k]) if hi[k] > lo[k] else 0.0
                for k in range(3)
            )
            for a in ARMS
        }
        for r in rs
    }


TAIL_W = (1.0 / 32, 1.0 / 32, 1.0)


def sec3(t):
    nt = normalise3(t)
    out = []
    for r in sorted(t):
        v = {a: sum(w * c for w, c in zip(TAIL_W, nt[r][a])) for a in ARMS}
        out.append(min(ARMS, key=lambda a: (v[a], a)))
    return out


def fires3(t, winner, band):
    nt = normalise3(t)
    for i, r in enumerate(sorted(t)):
        v = {a: sum(w * c for w, c in zip(TAIL_W, nt[r][a])) for a in ARMS}
        best, worst = min(v.values()), max(v.values())
        if v[winner[i]] > best + band * (worst - best):
            return True
    return False


def band_sweep(per):
    """Section 7's false alarm measurement, with the tail coordinate swapped."""
    print()
    print("=" * 78)
    print("2. THE BAND TENSION, WITH THE TAIL COORDINATE SWAPPED")
    print("=" * 78)
    print("  the tail-weighing weighting from `100` section 7, three coordinates")
    print("  (median time, declared bytes, TAIL STATISTIC), false alarm rate over")
    print("  2000 regenerations")
    hdr = "  ".join(f"{b * 100:5.1f}%" for b in BANDS)
    print(f"    {'tail statistic':<16} {hdr}")
    for name, est in ESTIMATORS:
        rng = random.Random(SEED)
        base = three_coord_table(per, est)
        winner = sec3(base)
        tabs = [three_coord_table(per, est, rng) for _ in range(RESAMPLES)]
        row = []
        for band in BANDS:
            n = sum(1 for t in tabs if fires3(t, winner, band))
            row.append(f"{100.0 * n / RESAMPLES:5.1f}%")
        print(f"    {name:<16} {'  '.join(row)}")

    print()
    print("  DETECTION: the widest band at which each generator defect is still")
    print("  refused. A statistic is usable only if its false-alarm band above is")
    print("  narrower than its detection band here.")
    print(f"    {'tail statistic':<16} {'G1 unit':>10} {'G2 swap':>10} {'G4 offbyone':>13}")

    def g_unit(t):
        return {r: {a: (c[0], c[1] * 1000, c[2]) for a, c in t[r].items()} for r in t}

    def g_swap(t):
        return {r: {a: (c[1], c[0], c[2]) for a, c in t[r].items()} for r in t}

    def g_off(t):
        rs = sorted(t)
        return {rs[i]: t[rs[(i + 1) % len(rs)]] for i in range(len(rs))}

    for name, est in ESTIMATORS:
        base = three_coord_table(per, est)
        correct = sec3(base)
        cells = []
        for g in (g_unit, g_swap, g_off):
            bad = sec3(g(base))
            if all(x == y for x, y in zip(bad, correct)):
                cells.append("no change")
            else:
                caught = [b for b in BANDS if fires3(base, bad, b)]
                cells.append(f"{max(caught) * 100:.1f}%" if caught else "NEVER")
        print(f"    {name:<16} {cells[0]:>10} {cells[1]:>10} {cells[2]:>13}")


def main():
    print(__doc__.split("Run:")[0].strip())
    print()
    per = samples()
    regions = sorted(per)
    print(
        f"regions {len(regions)}, arms {len(ARMS)}, "
        f"samples per arm per region {len(per[regions[0]][ARMS[0]])}"
    )
    print(f"resamples {RESAMPLES}, CI resamples {CI_RESAMPLES}, seed {SEED}")
    print("noise-floor control arm excluded, per p1")
    print()
    print(
        f"  {'estimator':<10} {'distinct':>9} {'modal':>16}   "
        f"{'arm pairs separated':>20}"
    )
    rows = []
    for name, est in ESTIMATORS:
        rng = random.Random(SEED)
        secs = Counter(section(per, est, rng) for _ in range(RESAMPLES))
        modal, freq = secs.most_common(1)[0]
        rng2 = random.Random(SEED)
        sep, tot = separation(per, est, rng2)
        rows.append((name, len(secs), freq, sep, tot))
        print(
            f"  {name:<10} {len(secs):>9} "
            f"{freq:>7}/{RESAMPLES} ({100.0 * freq / RESAMPLES:5.1f}%)   "
            f"{sep:>10} of {tot:<6} ({100.0 * sep / tot:5.1f}%)"
        )

    band_sweep(per)

    print()
    print("=" * 78)
    print("READING")
    print("=" * 78)
    base = [r for r in rows if r[0] == "iqr"][0]
    best_stab = min(rows, key=lambda r: r[1])
    best_sep = max(rows, key=lambda r: r[3])
    print(
        f"""
The interquartile range is the estimator every prior file in this unit used, and
it produced {base[1]} distinct sections and separated {base[3]} of {base[4]} arm pairs.

The most stable estimator here is {best_stab[0]!r} at {best_stab[1]} distinct sections, and the most
separating is {best_sep[0]!r} at {best_sep[3]} of {best_sep[4]}.

If a tail-keeping estimator is both more stable and more separating than the
interquartile range, then section 7's band tension and `98`'s F-98-12 are facts
about the IQR rather than about tail behaviour, and the fix is to change the
statistic rather than to drop the coordinate or widen the band.

If the IQR is competitive, the conclusion is the harder one: the instrument
cannot resolve the tail at all on this workload, and no choice of statistic
rescues it.
"""
    )


if __name__ == "__main__":
    main()
