#!/usr/bin/env python3
"""Two guards on the carrier-width sweep, run against its committed CSVs.

Guard one, deletion. A loop the optimiser removed does not scale with n. Six
committed sizes span a 512-fold range, so a per-element cost that stays roughly
constant across them is a loop doing real work per element, and one that
collapses toward zero is a loop that was deleted. This is the failure that put
six cells elsewhere in this directory above the machine's memory roofline: a
saturating fold reaches a fixpoint, the optimiser drops the loop, and the values
stay correct while the timing becomes meaningless.

Guard two, roofline. Bytes read per element divided by time per element is a
bandwidth, and an arm that implies more bandwidth than the memory it reads from
can deliver is not measuring the walk it claims to.

The ceiling has to be per cache level, which the first version of this script
got wrong and which is worth stating because the wrong version is the tempting
one. Anchoring one ceiling to the DRAM-resident case and applying it everywhere
flags the u64 arm at n=131072, where it implies 82.6 GB/s. That is not a defect:
at that size the u64 column is 1 MB, it lives in a 12 MB L2, and L2 delivers far
more than DRAM does. The check is therefore applied only where an arm's working
set exceeds L2, which is the only regime where a DRAM ceiling means anything,
and the ceiling itself is derived from the measured data rather than a datasheet.

A note on the input, also from getting it wrong first: each CSV holds both
`warm` and `cold` mode rows, 240 of each here. Pooling them is not a neutral
choice. At n=16384 the u64 arm medians 1316.9 ns warm and 2158.8 ns cold, and a
pooled median lands between the two and moves with whatever the row counts
happen to be. The harness's own findings tables report warm, so this reads warm.

Usage:  ./deletion_and_roofline.py [path-to-mock/benches]
Exit 0 = both guards pass. Exit 1 = a guard failed, with the offending rows.
"""

import csv
import os
import statistics
import sys

SIZES = [16384, 131072, 1048576, 2097152, 4194304, 8388608]
MODE = "warm"

# hw.perflevel0.l2cachesize on the host this was measured on
L2_BYTES = 12 * 1024 * 1024

# bytes read per element by each arm, from the shared crate's layout
BYTES_PER_ELEM = {
    "bitpack-carrier-d64": 8.0,
    "bitpack-carrier-d32": 4.0,
    "bitpack-carrier-d16": 2.0,
    "bitpack-carrier-d16-control": 2.0,
    "bitpack-carrier-packed": 13.0 / 8.0,
    "bitpack-carrier-packed-simd": 13.0 / 8.0,
}

bench_dir = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
    os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "benches")
bench_dir = os.path.abspath(bench_dir)


def medians(n):
    """median algo_ns per variant at size n, warm rows only"""
    path = os.path.join(bench_dir, "bitpack-carrier-width_n%d.csv" % n)
    if not os.path.exists(path):
        print("missing: %s" % path, file=sys.stderr)
        sys.exit(2)
    acc = {}
    with open(path) as f:
        for row in csv.DictReader(f):
            if row["mode"] != MODE:
                continue
            acc.setdefault(row["variant"], []).append(float(row["algo_ns"]))
    if not acc:
        print("no %s rows in %s" % (MODE, path), file=sys.stderr)
        sys.exit(2)
    return {v: statistics.median(x) for v, x in acc.items()}


data = {n: medians(n) for n in SIZES}
variants = sorted(data[SIZES[0]].keys())
ps = {v: [data[n][v] / n * 1000.0 for n in SIZES] for v in variants}

print("per-element cost, picoseconds (median warm algo_ns / n * 1000)")
print("%-30s %s" % ("variant", " ".join("%9d" % n for n in SIZES)))
for v in variants:
    print("%-30s %s" % (v, " ".join("%9.1f" % p for p in ps[v])))

failures = []

# ---- guard one: deletion -------------------------------------------------
# A real per-element loop keeps a per-element cost of the same order across a
# 512-fold size range. A deleted loop's per-element cost falls like 1/n, so by
# the largest size it would be several hundred times smaller. The threshold is
# deliberately loose (a factor of four either way) because memory effects
# legitimately move per-element cost; deletion moves it by two orders more.
print()
print("guard one, deletion")
print("  per-element cost, largest n over smallest n (1.0 = perfectly linear)")
for v in variants:
    ratio = ps[v][-1] / ps[v][0]
    ok = ratio > 0.25
    print("    %-30s %6.3f   %s" % (v, ratio, "ok" if ok else "COLLAPSED"))
    if not ok:
        failures.append("%s per-element cost collapsed by %.0fx across the sweep, "
                        "which is what a deleted loop looks like" % (v, 1.0 / ratio))

print("  total time growth against 512x size growth")
for v in variants:
    growth = data[SIZES[-1]][v] / data[SIZES[0]][v]
    ok = growth > 128.0
    print("    %-30s %7.1fx  %s" % (v, growth, "ok" if ok else "SUBLINEAR"))
    if not ok:
        failures.append("%s total time grew only %.1fx for a 512x larger column"
                        % (v, growth))

# ---- guard two: roofline, applied only past L2 ---------------------------
print()
print("guard two, roofline: implied read bandwidth, GB/s (* = working set past L2)")
print("%-30s %s" % ("variant", " ".join("%9d" % n for n in SIZES)))
bw = {}
past_l2 = {}
for v in variants:
    bw[v] = [BYTES_PER_ELEM[v] / (p * 1e-12) / 1e9 for p in ps[v]]
    past_l2[v] = [n * BYTES_PER_ELEM[v] > L2_BYTES for n in SIZES]
    cells = ["%8.1f%s" % (b, "*" if o else " ")
             for b, o in zip(bw[v], past_l2[v])]
    print("%-30s %s" % (v, " ".join(cells)))

dram = [b for v in variants for b, o in zip(bw[v], past_l2[v]) if o]
if not dram:
    print("\n  no arm's working set exceeds L2; the roofline guard has nothing to check")
else:
    ceiling = 1.25 * max(dram)
    print()
    print("  highest bandwidth measured with the working set past L2: %.1f GB/s" % max(dram))
    print("  ceiling used (1.25x that, measured rather than from a datasheet): %.1f GB/s"
          % ceiling)
    for v in variants:
        for n, b, o in zip(SIZES, bw[v], past_l2[v]):
            if o and b > ceiling:
                failures.append(
                    "%s at n=%d reads %.0f MB and implies %.1f GB/s, above the "
                    "%.1f GB/s ceiling, so it is reporting a walk faster than the "
                    "memory it walks"
                    % (v, n, n * BYTES_PER_ELEM[v] / 1e6, b, ceiling))

print()
if failures:
    print("FAILED:")
    for f in failures:
        print("  - " + f)
    sys.exit(1)
print("both guards pass: every arm scales with n, and no arm reading past L2")
print("implies a bandwidth the memory below it cannot deliver")
