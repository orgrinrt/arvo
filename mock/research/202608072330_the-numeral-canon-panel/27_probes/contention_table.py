#!/usr/bin/env python3
"""Read the committed contention csvs and produce every derived figure.

Nothing here is copied from prose. Every number is computed from
`mock/benches/bitpack-contention_n<KEY>.csv`, where `KEY = N * 10 + T`.

## Two statistics, and why both are printed

The single-core carrier sweep reports warm-mode medians and its arms have tight
distributions, so the median is a fine estimator there. A threaded arm's sample
is the wall time of a pass whose duration is the maximum over `T` threads, so any
scheduling interference on any one thread inflates that sample and none can
deflate it. The distribution is one-sided, its median drifts with how much
interference a particular process happened to meet, and the measured consequence
is visible in the noise floor: at `n = 4194304`, `t = 2`, the two byte-identical
`d16` arms differ by 10 percent on medians and by 0.8 percent on minima.

So the floor of the distribution is the estimator of the uninterfered cost, and
the tenth percentile is used for it rather than the strict minimum, because a
strict minimum is one sample and a percentile is not. Medians are printed
alongside throughout, so a reader can see where the two disagree and by how much.

Run: python3 27_probes/contention_table.py [--stat p10|median|min]
"""

import csv
import os
import statistics
import sys

BENCH = os.path.normpath(
    os.path.join(os.path.dirname(__file__), "..", "..", "..", "benches")
)

NS = [16384, 1048576, 4194304, 8388608]
TS = [1, 2, 4, 8]
ARMS = ["d16", "d16-control", "d32", "d64", "packed", "packed-simd"]
WIDTH = {
    "d16": 2.0,
    "d16-control": 2.0,
    "d32": 4.0,
    "d64": 8.0,
    "packed": 13.0 / 8.0,
    "packed-simd": 13.0 / 8.0,
}


def p10(vals):
    v = sorted(vals)
    return v[max(0, (len(v) * 10) // 100)]


STATS = {"p10": p10, "median": statistics.median, "min": min}


def row(key, n, stat):
    """Warm-mode per-element picoseconds per arm, or None if the row is absent."""
    path = os.path.join(BENCH, f"bitpack-contention_n{key}.csv")
    if not os.path.exists(path):
        return None
    per_arm = {}
    with open(path) as fh:
        for r in csv.DictReader(fh):
            if r["mode"] != "warm":
                continue
            per_arm.setdefault(r["variant"], []).append(float(r["algo_ns"]))
    return {
        v.replace("bitpack-contend-", ""): stat(vals) * 1000.0 / n
        for v, vals in per_arm.items()
    }


def load(stat):
    table = {}
    for n in NS:
        for t in TS:
            r = row(n * 10 + t, n, stat)
            if r is not None:
                table[(n, t)] = r
    return table


def show(table, label):
    keys = sorted(table)
    hdr = f"{'n':>9} {'t':>2} " + " ".join(f"{a:>12}" for a in ARMS)

    print(f"### {label}: picoseconds per element of the whole column")
    print("(wall time over N, so a perfectly scaling arm quarters from t=1 to t=4)")
    print()
    print(hdr)
    for n, t in keys:
        r = table[(n, t)]
        print(
            f"{n:>9} {t:>2} "
            + " ".join(f"{r[a]:12.1f}" if a in r else f"{'-':>12}" for a in ARMS)
        )
    print()

    print(f"### {label}: speedup against the same arm's own t=1 row (4.00 ideal at t=4)")
    print()
    print(hdr)
    for n, t in keys:
        if (n, 1) not in table:
            continue
        base, r = table[(n, 1)], table[(n, t)]
        print(
            f"{n:>9} {t:>2} "
            + " ".join(
                f"{base[a]/r[a]:12.2f}" if a in r and a in base else f"{'-':>12}"
                for a in ARMS
            )
        )
    print()

    print(f"### {label}: implied aggregate read bandwidth, GB/s (the machine, not one core)")
    print()
    print(hdr)
    for n, t in keys:
        r = table[(n, t)]
        print(
            f"{n:>9} {t:>2} "
            + " ".join(
                f"{WIDTH[a]/(r[a]*1e-12)/1e9:12.1f}" if a in r else f"{'-':>12}"
                for a in ARMS
            )
        )
    print()

    print(f"### {label}: break-even carrier width, bytes per element")
    print("(linear between the measured d32 and d64 points; below 2 means packing")
    print(" beats every native carrier a 13-bit field could use)")
    print()
    print(f"{'n':>9} {'t':>2} {'vs packed':>12} {'vs packed-simd':>15}")
    for n, t in keys:
        r = table[(n, t)]
        if r.get("d64") == r.get("d32"):
            continue
        cells = [
            4.0 + (r[a] - r["d32"]) * 4.0 / (r["d64"] - r["d32"])
            for a in ("packed", "packed-simd")
        ]
        print(f"{n:>9} {t:>2} {cells[0]:12.2f} {cells[1]:15.2f}")
    print()

    print(f"### {label}: noise floor, d16-control against d16, per cent")
    print()
    for n, t in keys:
        r = table[(n, t)]
        print(f"  n={n:>9} t={t}: {100.0*(r['d16-control']/r['d16']-1.0):+6.2f}%")
    print()

    print(f"### {label}: packed-simd against each dense carrier, per cent")
    print("(negative means packing wins)")
    print()
    print(f"{'n':>9} {'t':>2} " + " ".join(f"{c:>12}" for c in ("vs d16", "vs d32", "vs d64")))
    for n, t in keys:
        r = table[(n, t)]
        print(
            f"{n:>9} {t:>2} "
            + " ".join(
                f"{100.0*(r['packed-simd']/r[c]-1.0):+12.1f}" for c in ("d16", "d32", "d64")
            )
        )
    print()


def main():
    which = "p10"
    if "--stat" in sys.argv:
        which = sys.argv[sys.argv.index("--stat") + 1]
    table = load(STATS[which])
    if not table:
        print("no contention csvs found yet")
        return 1
    show(table, which)

    # the two estimators side by side, so the gap between them is visible rather
    # than a choice a reader has to take on trust
    med = load(statistics.median)
    print("### tenth percentile against median, per cent (how much interference the")
    print("    median is carrying on each row)")
    print()
    print(f"{'n':>9} {'t':>2} " + " ".join(f"{a:>12}" for a in ARMS))
    for n, t in sorted(table):
        a10, am = table[(n, t)], med[(n, t)]
        print(
            f"{n:>9} {t:>2} "
            + " ".join(f"{100.0*(am[a]/a10[a]-1.0):12.1f}" for a in ARMS)
        )
    return 0


if __name__ == "__main__":
    sys.exit(main())
