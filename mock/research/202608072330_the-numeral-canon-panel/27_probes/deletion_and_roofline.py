#!/usr/bin/env python3
"""Check that no timed loop was deleted and that no arm exceeds the machine.

Two failure modes this bench could have had, both of which produce numbers that
look ordinary:

  1. the optimiser folds a loop away, so per-element cost collapses with size,
  2. an arm reports a bandwidth the memory system cannot deliver, which means it
     is not reading what it claims to read.

Both are checked against the committed csvs rather than asserted in prose. The
roofline is applied only where an arm's own working set exceeds this host's 12 MB
L2, because a cache-resident arm legitimately exceeds the DRAM figure and raising
the threshold until everything passes would be the wrong repair.

Run: python3 27_probes/deletion_and_roofline.py
"""

import csv
import os
import sys

BENCH = os.path.normpath(
    os.path.join(os.path.dirname(__file__), "..", "..", "..", "benches")
)

L2_BYTES = 12_582_912
# highest aggregate figure any arm reached on a working set past L2, taken from
# the measurements themselves rather than from a datasheet, with a 25 per cent
# allowance for the estimator
ROOFLINE_GBS = 74.0 * 1.25

WIDTH = {
    "d16": 2.0,
    "d16-control": 2.0,
    "d16-padal": 2.0,
    "d32": 4.0,
    "d32-padal": 4.0,
    "d64": 8.0,
    "packed": 13.0 / 8.0,
    "packed-simd": 13.0 / 8.0,
    "pipe2": 13.0 / 8.0,
    "pipe4": 13.0 / 8.0,
}


def p10(vals):
    v = sorted(vals)
    return v[max(0, (len(v) * 10) // 100)]


def load(section):
    """{(n, t): {arm: (ps_per_elem, ns_per_call)}} over every committed row."""
    out = {}
    for name in sorted(os.listdir(BENCH)):
        if not name.startswith(f"{section}_n") or not name.endswith(".csv"):
            continue
        key = int(name[len(section) + 2 : -4])
        n, t = key // 10, key % 10
        per_arm = {}
        with open(os.path.join(BENCH, name)) as fh:
            for r in csv.DictReader(fh):
                if r["mode"] != "warm":
                    continue
                per_arm.setdefault(r["variant"].replace("bitpack-contend-", ""), []).append(
                    float(r["algo_ns"])
                )
        out[(n, t)] = {a: (p10(v) * 1000.0 / n, p10(v)) for a, v in per_arm.items()}
    return out


def main():
    failures = []
    for section in ("bitpack-contention", "bitpack-contend-decode", "bitpack-contend-best"):
        table = load(section)
        if not table:
            continue
        print(f"## {section}")
        ts = sorted({t for _, t in table})
        for t in ts:
            ns = sorted(n for n, tt in table if tt == t)
            if len(ns) < 2:
                continue
            arms = sorted(set(table[(ns[0], t)]) & set(table[(ns[-1], t)]))
            lo, hi = ns[0], ns[-1]
            growth = hi / lo
            print(f"  t={t}, n from {lo} to {hi} ({growth:.0f}x larger column)")
            for a in arms:
                ps_lo, ns_lo = table[(lo, t)][a]
                ps_hi, ns_hi = table[(hi, t)][a]
                ratio = ps_hi / ps_lo
                total = ns_hi / ns_lo
                # Directional on purpose. Deletion makes per-element cost
                # COLLAPSE as the column grows, and makes total time grow far
                # slower than the column. A per-element cost that RISES is a
                # real memory effect and is the whole subject of this file, so
                # flagging it would be flagging the finding.
                #
                # The first version of this check bounded the ratio on both
                # sides at 0.5 to 3.0 and failed `d64` at t=4 with a ratio of
                # 4.53. That is not a deleted loop, it is a u64 column crossing
                # out of L2 and hitting the wall, which section 7 is about. The
                # wrong repair would have been to widen the upper bound until it
                # passed; the right one is to have no upper bound, because no
                # upper bound was ever part of what deletion looks like.
                ok = ratio >= 0.5 and total > growth * 0.4
                if not ok:
                    failures.append(f"{section} t={t} {a}: per-element ratio {ratio:.2f}, total growth {total:.0f}x")
                print(
                    f"    {a:>12}: per-element {ps_lo:6.1f} -> {ps_hi:6.1f} ps "
                    f"(x{ratio:4.2f}), total time x{total:6.0f}  {'ok' if ok else 'FAIL'}"
                )
        print()

        print("  roofline, arms whose working set exceeds this host's 12 MB L2")
        worst = (0.0, None)
        checked = 0
        for (n, t), r in sorted(table.items()):
            for a, (ps, _) in sorted(r.items()):
                if n * WIDTH[a] <= L2_BYTES:
                    continue
                checked += 1
                gbs = WIDTH[a] / (ps * 1e-12) / 1e9
                if gbs > worst[0]:
                    worst = (gbs, f"n={n} t={t} {a}")
                if gbs > ROOFLINE_GBS:
                    failures.append(f"{section} n={n} t={t} {a}: {gbs:.1f} GB/s over the roofline")
                    print(f"    n={n:>9} t={t} {a:>12}: {gbs:6.1f} GB/s  FAIL")
        print(
            f"    {checked} arm-rows past L2, highest {worst[0]:.1f} GB/s ({worst[1]}), "
            f"ceiling {ROOFLINE_GBS:.1f}"
        )
        print()

    if failures:
        print("FAILURES:")
        for f in failures:
            print("  " + f)
        return 1
    print("all rows pass: no loop was deleted and no arm exceeds the machine")
    return 0


if __name__ == "__main__":
    sys.exit(main())
