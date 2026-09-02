#!/usr/bin/env python3
"""p1. Extract a real cost table from committed harness output.

Every later probe in this set needs a cost table: a function from (region, arm)
to a vector of measurements, lower being better on each coordinate. Rather than
invent one, this reads the one the repository already holds.

Source: mock/benches/bitpack-carrier-width_n*.csv, committed harness runs of the
same 13-bit column against six carriers at six column sizes spanning L1 to past
a 12 MB L2. Six regions, six arms.

Three coordinates, and the third is the point of doing this against real data
rather than a made-up table:

  time    median per-batch algo_ns over the run's samples. The obvious one.
  bytes   bytes of storage per element. A declared property of the arm, not a
          measurement: d16 stores each 13-bit field in a u16, d32 in a u32,
          d64 in a u64, and the two packed arms store 13 bits.
  spread  the interquartile range of the per-batch samples, in ns. A proxy for
          tail behaviour. The harness's own findings file for n=16384 raises
          exactly this as a decision axis: "Speed leader bitpack-carrier-d64 vs
          stability leader bitpack-carrier-d32 (+3% speed for 4.1x steadier)".

The claim this probe supports is only that the table below is what the
committed CSVs say. It decides nothing on its own.
"""

import csv
import glob
import json
import os
import re
import statistics

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

# Bytes per element, per arm. Declared, not measured: it is the carrier width
# the variant name states. The two packed arms hold 13 bits.
BYTES_PER_ELEM = {
    "bitpack-carrier-d16": 2.0,
    "bitpack-carrier-d16-control": 2.0,
    "bitpack-carrier-d32": 4.0,
    "bitpack-carrier-d64": 8.0,
    "bitpack-carrier-packed": 13.0 / 8.0,
    "bitpack-carrier-packed-simd": 13.0 / 8.0,
}


def load():
    table = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per_variant = {}
        for row in csv.DictReader(open(path)):
            per_variant.setdefault(row["variant"], []).append(float(row["algo_ns"]))
        region = {}
        for variant, samples in per_variant.items():
            samples.sort()
            q1 = statistics.quantiles(samples, n=4)[0]
            q3 = statistics.quantiles(samples, n=4)[2]
            region[variant] = {
                "time": statistics.median(samples),
                "bytes": BYTES_PER_ELEM[variant],
                "spread": q3 - q1,
                "samples": len(samples),
            }
        table[n] = region
    return table


def main():
    table = load()
    regions = sorted(table)
    arms = sorted(BYTES_PER_ELEM)

    print(f"regions (column size n): {regions}")
    print(f"arms: {len(arms)}")
    for a in arms:
        print(f"  {a}")
    print(f"sections over this table: {len(arms)} ** {len(regions)} = "
          f"{len(arms) ** len(regions)}")
    print()

    for n in regions:
        print(f"n = {n}")
        print(f"  {'arm':32s} {'time ns':>10s} {'bytes/el':>9s} {'IQR ns':>10s} {'samples':>8s}")
        for a in arms:
            c = table[n][a]
            print(f"  {a:32s} {c['time']:10.1f} {c['bytes']:9.3f} "
                  f"{c['spread']:10.1f} {c['samples']:8d}")
        print()

    # Pareto structure. An arm dominated at a region can never be chosen by any
    # selection rule that is monotone in the coordinates, so the Pareto front
    # per region bounds every count that follows.
    print("Pareto front per region (no other arm is <= on all three and < on one):")
    product = 1
    for n in regions:
        front = []
        for a in arms:
            ca = table[n][a]
            dominated = False
            for b in arms:
                if b == a:
                    continue
                cb = table[n][b]
                le = all(cb[k] <= ca[k] for k in ("time", "bytes", "spread"))
                lt = any(cb[k] < ca[k] for k in ("time", "bytes", "spread"))
                if le and lt:
                    dominated = True
                    break
            if not dominated:
                front.append(a)
        product *= len(front)
        print(f"  n = {n:8d}: {len(front)} of {len(arms)}  {front}")
    print(f"\nupper bound on Pareto-admissible sections: {product}")

    # Arms dominated in EVERY region. These cannot be selected anywhere by any
    # monotone rule, so no strategy expressible as a preference over these
    # three coordinates can ever reach for them.
    print("\narms dominated in every region:")
    any_dom = False
    for a in arms:
        everywhere = True
        for n in regions:
            ca = table[n][a]
            dominated = any(
                all(table[n][b][k] <= ca[k] for k in ("time", "bytes", "spread"))
                and any(table[n][b][k] < ca[k] for k in ("time", "bytes", "spread"))
                for b in arms if b != a
            )
            if not dominated:
                everywhere = False
                break
        if everywhere:
            any_dom = True
            print(f"  {a}")
    if not any_dom:
        print("  none")

    out = os.path.join(HERE, "p1_cost_table.json")
    with open(out, "w") as f:
        json.dump({str(n): table[n] for n in regions}, f, indent=1, sort_keys=True)
    print(f"\nwritten: {os.path.basename(out)}")


if __name__ == "__main__":
    main()
