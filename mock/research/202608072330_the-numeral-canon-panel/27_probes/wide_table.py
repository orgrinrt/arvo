#!/usr/bin/env python3
"""Read the wide-column csvs: the `u16` question with nothing cache-resident.

The contention sweep's largest row holds a 16 MiB `u16` column and a 13.0 MiB
packed one against a 12 MiB L2, so both are past it only barely and the two arms
land within the threaded noise floor of each other. This section carries only
those two regions, which buys four times the record count inside the same
allocation and puts both several times past the cache.

Three arms plus a byte-identical control: the `u16` carrier as committed, the
`u16` carrier with its widening chain attacked, and the packed decode with four
accumulators. The packed arm is compared against the **better** of the two dense
arms at every row, because comparing an attacked kernel against an unattacked one
is the failure this whole section exists to avoid repeating.

Same estimator discipline as its siblings: tenth percentile of the samples, with
the median printed beside it.

Run: python3 27_probes/wide_table.py
"""

import csv
import os
import statistics
import sys

BENCH = os.path.normpath(
    os.path.join(os.path.dirname(__file__), "..", "..", "..", "benches")
)

L2_BYTES = 12_582_912
KEYS = [83886081, 83886084, 167772161, 167772164, 335544321, 335544324]
ARMS = ["d16", "d16-control", "d16-padal", "pipe4"]
WIDTH = {"d16": 2.0, "d16-control": 2.0, "d16-padal": 2.0, "pipe4": 13.0 / 8.0}


def p10(vals):
    v = sorted(vals)
    return v[max(0, (len(v) * 10) // 100)]


def read(key, n, mode):
    path = os.path.join(BENCH, f"bitpack-wide_n{key}.csv")
    if not os.path.exists(path):
        return None
    per_arm = {}
    with open(path) as fh:
        for r in csv.DictReader(fh):
            if r["mode"] != mode:
                continue
            per_arm.setdefault(r["variant"].replace("bitpack-wide-", ""), []).append(
                float(r["algo_ns"])
            )
    if not per_arm:
        return None
    return {
        a: (p10(v) * 1000.0 / n, statistics.median(v) * 1000.0 / n)
        for a, v in per_arm.items()
    }


def main():
    print("region footprints against this host's 12 MiB L2")
    print()
    print(f"{'n':>10} {'u16 region':>14} {'packed region':>16} {'u16 / L2':>10} {'packed / L2':>12}")
    for n in sorted({k // 10 for k in KEYS}):
        d16 = n * 2
        pk = (n * 13) // 8
        print(
            f"{n:>10} {d16/2**20:11.0f} MiB {pk/2**20:13.0f} MiB "
            f"{d16/L2_BYTES:10.1f} {pk/L2_BYTES:12.1f}"
        )
    print()

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
        print(f"{'n':>10} {'t':>2} " + " ".join(f"{a:>18}" for a in ARMS))
        for (n, t), r in sorted(table.items()):
            print(
                f"{n:>10} {t:>2} "
                + " ".join(f"{r[a][0]:9.1f} ({r[a][1]:6.1f})" for a in ARMS)
            )
        print()

        print(f"### {mode}: implied aggregate read bandwidth, GB/s")
        print()
        print(f"{'n':>10} {'t':>2} " + " ".join(f"{a:>12}" for a in ARMS))
        for (n, t), r in sorted(table.items()):
            print(
                f"{n:>10} {t:>2} "
                + " ".join(f"{WIDTH[a]/(r[a][0]*1e-12)/1e9:12.1f}" for a in ARMS)
            )
        print()

        print(f"### {mode}: the answer, and the floor it has to clear")
        print()
        print(
            f"{'n':>10} {'t':>2} {'control gap':>13} {'padal bought':>14} "
            f"{'pipe4 vs best u16':>19}"
        )
        for (n, t), r in sorted(table.items()):
            best = min(r["d16"][0], r["d16-padal"][0])
            print(
                f"{n:>10} {t:>2} "
                f"{100*(r['d16-control'][0]/r['d16'][0]-1):12.2f}% "
                f"{100*(r['d16-padal'][0]/r['d16'][0]-1):13.1f}% "
                f"{100*(r['pipe4'][0]/best-1):18.1f}%"
            )
        print()

        print(f"### {mode}: speedup of each arm against its own t=1 row")
        print()
        print(f"{'n':>10} {'t':>2} " + " ".join(f"{a:>12}" for a in ARMS))
        for (n, t), r in sorted(table.items()):
            if (n, 1) not in table:
                continue
            b = table[(n, 1)]
            print(f"{n:>10} {t:>2} " + " ".join(f"{b[a][0]/r[a][0]:12.2f}" for a in ARMS))
        print()

    # the cross-check that the erased base pointer and the two-region layout
    # changed nothing: this section's d16 against the contention section's, at
    # the one key both declare
    print("### cross-check: d16 here against d16 in the contention section, n=8388608")
    print()
    for t in (1, 4):
        key = 83886081 + (t - 1) * 3
        a = read(key, 8388608, "warm")
        path = os.path.join(BENCH, f"bitpack-contention_n{key}.csv")
        if a is None or not os.path.exists(path):
            continue
        vals = [
            float(r["algo_ns"])
            for r in csv.DictReader(open(path))
            if r["mode"] == "warm" and r["variant"].endswith("d16")
        ]
        b = p10(vals) * 1000.0 / 8388608
        print(f"  t={t}: wide {a['d16'][0]:6.1f} against contention {b:6.1f}  ({100*(a['d16'][0]/b-1):+5.1f}%)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
