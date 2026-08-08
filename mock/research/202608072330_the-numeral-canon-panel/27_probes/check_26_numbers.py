#!/usr/bin/env python3
"""Recompute file 26's headline numbers straight from the committed csvs.

The dispatch brief asserts 26's conclusions as the reference this file extends,
and `RULES.md` says a brief's cheap factual claims get checked before anything
is reasoned from them. Everything below is derived from
`mock/benches/bitpack-carrier-width_n*.csv` alone; no number is copied from 26's
prose.

Three claims are tested:

  1. the warm-mode median per-element cost table in 26 section 7,
  2. the break-even carrier width of 5.8 to 7.0 bytes in 26 section 10,
  3. the flatness of the dense arms across the size sweep (26 section 5.1).

Run: python3 27_probes/check_26_numbers.py
"""

import csv
import os
import statistics
import sys

BENCH = os.path.join(os.path.dirname(__file__), "..", "..", "..", "benches")
BENCH = os.path.normpath(BENCH)

SIZES = [16384, 131072, 1048576, 2097152, 4194304, 8388608]
ARMS = [
    "bitpack-carrier-d16",
    "bitpack-carrier-d16-control",
    "bitpack-carrier-d32",
    "bitpack-carrier-d64",
    "bitpack-carrier-packed",
    "bitpack-carrier-packed-simd",
]
BYTES_PER_ELEM = {
    "bitpack-carrier-d16": 2.0,
    "bitpack-carrier-d16-control": 2.0,
    "bitpack-carrier-d32": 4.0,
    "bitpack-carrier-d64": 8.0,
    "bitpack-carrier-packed": 13.0 / 8.0,
    "bitpack-carrier-packed-simd": 13.0 / 8.0,
}


def warm_medians(path, n):
    """Median algo_ns per arm over warm rows only, and the per-element ps."""
    per_arm = {}
    with open(path) as fh:
        for row in csv.DictReader(fh):
            if row["mode"] != "warm":
                continue
            per_arm.setdefault(row["variant"], []).append(float(row["algo_ns"]))
    out = {}
    for arm, vals in per_arm.items():
        med_ns = statistics.median(vals)
        out[arm] = (med_ns, med_ns * 1000.0 / n, len(vals))
    return out


def main():
    table = {}
    for n in SIZES:
        path = os.path.join(BENCH, f"bitpack-carrier-width_n{n}.csv")
        if not os.path.exists(path):
            print(f"MISSING {path}")
            return 1
        table[n] = warm_medians(path, n)

    print("warm-mode median picoseconds per element, from the committed csvs")
    print()
    header = f"{'n':>9} " + " ".join(f"{a.split('carrier-')[1]:>13}" for a in ARMS)
    print(header)
    for n in SIZES:
        cells = []
        for a in ARMS:
            v = table[n].get(a)
            cells.append(f"{v[1]:13.1f}" if v else f"{'-':>13}")
        print(f"{n:>9} " + " ".join(cells))
    print()

    # sample counts, so a reader can see the medians are not over three rows
    counts = {a: table[SIZES[0]][a][2] for a in ARMS if a in table[SIZES[0]]}
    print(f"warm samples per arm per size: {sorted(set(counts.values()))}")
    print()

    # claim 2: break-even carrier width, interpolating dense cost between the
    # d32 and d64 points and asking where it equals the packed cost.
    print("break-even carrier width, linear between the d32 and d64 points")
    for n in SIZES:
        t = table[n]
        c32 = t["bitpack-carrier-d32"][1]
        c64 = t["bitpack-carrier-d64"][1]
        for packed_arm in ("bitpack-carrier-packed", "bitpack-carrier-packed-simd"):
            cp = t[packed_arm][1]
            if c64 == c32:
                continue
            b = 4.0 + (cp - c32) * 4.0 / (c64 - c32)
            tag = packed_arm.split("carrier-")[1]
            print(f"  n={n:>9} {tag:>12}: break-even = {b:6.2f} bytes/elem")
    print()

    # claim 3: flatness of each arm across the sweep
    print("spread of per-element cost across the whole size sweep, per arm")
    for a in ARMS:
        vals = [table[n][a][1] for n in SIZES if a in table[n]]
        lo, hi = min(vals), max(vals)
        print(f"  {a:>30}: {lo:6.1f} to {hi:6.1f} ps  ({100*(hi/lo-1):5.1f}% spread)")
    print()

    # implied per-core read bandwidth, which is what the contention question
    # turns on: how close is a single core to the machine's ceiling.
    print("implied single-core read bandwidth, GB/s")
    header = f"{'n':>9} " + " ".join(f"{a.split('carrier-')[1]:>13}" for a in ARMS)
    print(header)
    for n in SIZES:
        cells = []
        for a in ARMS:
            ps = table[n][a][1]
            gbs = BYTES_PER_ELEM[a] / (ps * 1e-12) / 1e9
            cells.append(f"{gbs:13.1f}")
        print(f"{n:>9} " + " ".join(cells))
    return 0


if __name__ == "__main__":
    sys.exit(main())
