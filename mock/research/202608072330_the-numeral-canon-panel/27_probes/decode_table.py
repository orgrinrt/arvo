#!/usr/bin/env python3
"""Read the committed decode-attack csvs.

Same estimator discipline as `contention_table.py`: tenth percentile of the warm
samples as the primary, median printed beside it, because a threaded sample is
the maximum over `T` threads and its distribution is one-sided.

The `d16` and `d16-control` arms appear in both bench sections. The contention
section's csvs were produced before the shared crate was split into modules and
this section's after it, so comparing those two arms at the same key is the check
that the split changed nothing that matters. That comparison is printed last.

Run: python3 27_probes/decode_table.py
"""

import csv
import os
import statistics
import sys

BENCH = os.path.normpath(
    os.path.join(os.path.dirname(__file__), "..", "..", "..", "benches")
)

KEYS = [163841, 163844, 41943041, 41943044, 83886081, 83886084]
ARMS = ["d16", "d16-control", "packed-simd", "pipe2", "pipe4"]


def p10(vals):
    v = sorted(vals)
    return v[max(0, (len(v) * 10) // 100)]


def read(section, key, n, mode="warm"):
    path = os.path.join(BENCH, f"{section}_n{key}.csv")
    if not os.path.exists(path):
        return None
    per_arm = {}
    with open(path) as fh:
        for r in csv.DictReader(fh):
            if r["mode"] != mode:
                continue
            per_arm.setdefault(r["variant"].replace("bitpack-contend-", ""), []).append(
                float(r["algo_ns"])
            )
    return {
        a: (p10(v) * 1000.0 / n, statistics.median(v) * 1000.0 / n)
        for a, v in per_arm.items()
    }


def main():
    for mode in ("warm", "cold"):
        print(f"### {mode}: picoseconds per element, tenth percentile (median)")
        print()
        print(f"{'n':>9} {'t':>2} " + " ".join(f"{a:>18}" for a in ARMS))
        rows = {}
        for key in KEYS:
            n, t = key // 10, key % 10
            r = read("bitpack-contend-decode", key, n, mode)
            if r is None:
                continue
            rows[(n, t)] = r
            print(
                f"{n:>9} {t:>2} "
                + " ".join(
                    f"{r[a][0]:9.1f} ({r[a][1]:6.1f})" if a in r else f"{'-':>18}"
                    for a in ARMS
                )
            )
        print()
        print(f"### {mode}: each packed decode against packed-simd, and against d16")
        print("(negative means faster)")
        print()
        print(f"{'n':>9} {'t':>2} " + " ".join(f"{c:>16}" for c in
              ("pipe2 vs simd", "pipe4 vs simd", "best vs d16")))
        for (n, t), r in sorted(rows.items()):
            base = r["packed-simd"][0]
            best = min(r[a][0] for a in ("packed-simd", "pipe2", "pipe4"))
            print(
                f"{n:>9} {t:>2} "
                + f"{100*(r['pipe2'][0]/base-1):16.1f} "
                + f"{100*(r['pipe4'][0]/base-1):16.1f} "
                + f"{100*(best/r['d16'][0]-1):16.1f}"
            )
        print()

    print("### the split check: d16 in this section against d16 in the contention")
    print("    section, at the same key, per cent (warm, tenth percentile)")
    print()
    for key in KEYS:
        n, t = key // 10, key % 10
        a = read("bitpack-contend-decode", key, n)
        b = read("bitpack-contention", key, n)
        if a is None or b is None:
            continue
        for arm in ("d16", "d16-control"):
            print(
                f"  n={n:>9} t={t} {arm:>12}: {a[arm][0]:6.1f} against {b[arm][0]:6.1f}"
                f"  ({100*(a[arm][0]/b[arm][0]-1):+6.1f}%)"
            )
    return 0


if __name__ == "__main__":
    sys.exit(main())
