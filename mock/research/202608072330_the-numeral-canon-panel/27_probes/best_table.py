#!/usr/bin/env python3
"""Read the both-sides-attacked csvs.

Same estimator discipline as the sibling probes: tenth percentile of the warm
samples as the primary, median beside it, because a threaded sample is the
maximum over `T` threads and its distribution is one-sided.

Six arms: three dense carriers in their committed form and two of them with the
same `UADALP` attack the packed side received, plus the packed decode before and
after that attack. The point of the pairing is that a kernel improvement on an
arm that is already limited by the machine should change nothing, and one on an
arm that is not should change everything, so the two pairs are a direct test of
which arms are bandwidth-bound at which thread count.

Run: python3 27_probes/best_table.py
"""

import csv
import os
import statistics
import sys

BENCH = os.path.normpath(
    os.path.join(os.path.dirname(__file__), "..", "..", "..", "benches")
)

KEYS = [41943041, 41943044, 83886081, 83886084]
ARMS = ["d16", "d16-padal", "d32", "d32-padal", "packed-simd", "pipe4"]
WIDTH = {
    "d16": 2.0,
    "d16-padal": 2.0,
    "d32": 4.0,
    "d32-padal": 4.0,
    "packed-simd": 13.0 / 8.0,
    "pipe4": 13.0 / 8.0,
}


def p10(vals):
    v = sorted(vals)
    return v[max(0, (len(v) * 10) // 100)]


def read(key, n, mode):
    path = os.path.join(BENCH, f"bitpack-contend-best_n{key}.csv")
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
    if not per_arm:
        return None
    return {
        a: (p10(v) * 1000.0 / n, statistics.median(v) * 1000.0 / n)
        for a, v in per_arm.items()
    }


def main():
    for mode in ("warm", "cold"):
        table = {}
        for key in KEYS:
            n, t = key // 10, key % 10
            r = read(key, n, mode)
            if r is not None:
                table[(n, t)] = r
        if not table:
            continue

        print(f"### {mode}: picoseconds per element, tenth percentile (median)")
        print()
        print(f"{'n':>9} {'t':>2} " + " ".join(f"{a:>18}" for a in ARMS))
        for (n, t), r in sorted(table.items()):
            print(
                f"{n:>9} {t:>2} "
                + " ".join(f"{r[a][0]:9.1f} ({r[a][1]:6.1f})" for a in ARMS)
            )
        print()

        print(f"### {mode}: implied aggregate read bandwidth, GB/s")
        print()
        print(f"{'n':>9} {'t':>2} " + " ".join(f"{a:>12}" for a in ARMS))
        for (n, t), r in sorted(table.items()):
            print(
                f"{n:>9} {t:>2} "
                + " ".join(f"{WIDTH[a]/(r[a][0]*1e-12)/1e9:12.1f}" for a in ARMS)
            )
        print()

        print(f"### {mode}: what the kernel attack bought, per arm and per thread count")
        print("(an arm limited by the machine should gain nothing from a better kernel)")
        print()
        print(f"{'n':>9} {'t':>2} {'d16-padal/d16':>16} {'d32-padal/d32':>16} {'pipe4/packed-simd':>19}")
        for (n, t), r in sorted(table.items()):
            print(
                f"{n:>9} {t:>2} "
                f"{100*(r['d16-padal'][0]/r['d16'][0]-1):16.1f} "
                f"{100*(r['d32-padal'][0]/r['d32'][0]-1):16.1f} "
                f"{100*(r['pipe4'][0]/r['packed-simd'][0]-1):19.1f}"
            )
        print()

        print(f"### {mode}: the packed arm against the best dense arm of each width")
        print("(negative means packing wins)")
        print()
        print(f"{'n':>9} {'t':>2} {'pipe4 vs best u16':>20} {'pipe4 vs best u32':>20}")
        for (n, t), r in sorted(table.items()):
            b16 = min(r["d16"][0], r["d16-padal"][0])
            b32 = min(r["d32"][0], r["d32-padal"][0])
            print(
                f"{n:>9} {t:>2} {100*(r['pipe4'][0]/b16-1):20.1f} {100*(r['pipe4'][0]/b32-1):20.1f}"
            )
        print()

        print(f"### {mode}: speedup of each arm against its own t=1 row")
        print()
        print(f"{'n':>9} {'t':>2} " + " ".join(f"{a:>12}" for a in ARMS))
        for (n, t), r in sorted(table.items()):
            if (n, 1) not in table:
                continue
            b = table[(n, 1)]
            print(
                f"{n:>9} {t:>2} " + " ".join(f"{b[a][0]/r[a][0]:12.2f}" for a in ARMS)
            )
        print()
    return 0


if __name__ == "__main__":
    sys.exit(main())
