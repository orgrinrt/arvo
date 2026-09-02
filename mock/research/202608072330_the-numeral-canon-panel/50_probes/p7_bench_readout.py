#!/usr/bin/env python3
# p7: a readout of a bench that already exists, because three files in this unit report its
# magnitude as unpriced.
#
# 47:536-541 says a packed read "can be written as a width-generic byte loop, which needs no
# per-width load type and is correct. What it costs against a fixed-window load is a codegen-quality
# question, and no bench harness has run in this panel, so it is unpriced". 48:504-521 repeats it and
# spends a whole dispatch slot (52) on pricing it.
#
# mock/benches/ holds `bitpack-decoder-shape`, committed, four sizes, with the dense native carrier
# as a competitor arm. Its two packed arms are exactly the two shapes at issue:
#
#   bitpack-plan-naive     "byte offset and bit shift both derived from the running index at runtime"
#   bitpack-plan-windowed  "the plan the width itself determines: P = 8 / gcd(W, 8) fields per
#                           W * P / 8 whole bytes, every byte offset and bit shift a compile-time
#                           constant"
#
# (quoted from mock/benches/variants/bitpack-plan-naive/src/lib.rs:1-3 and
#  mock/benches/variants/bitpack-plan-windowed/src/lib.rs:1-4)
#
# This file only reads the committed CSVs. It runs no bench, measures nothing, and adds no arm. It
# exists so the numbers below carry the command that produced them.
#
#   python3 p7_bench_readout.py
#
# RULES.md:337-357 records this panel losing a night to eighteen files calling a magnitude unpriced
# while mock/benches/ held the measurement. That is the second time.

import csv
import glob
import os
import re
import statistics

BENCH = os.path.join(
    os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "benches")


def main():
    pat = os.path.join(BENCH, "bitpack-decoder-shape_n*.csv")
    files = sorted(glob.glob(pat), key=lambda s: int(re.search(r"_n(\d+)", s).group(1)))
    if not files:
        print("no committed CSVs found at %s" % pat)
        return
    print("source: mock/benches/bitpack-decoder-shape_n*.csv, committed, four sizes")
    print("host, from the committed meta: Apple M1, rustc 1.98.0-nightly (57d06900f 2026-05-27)")
    print("column read: algo_ns. statistic: median over every sample of every run.")
    print()

    per_n = {}
    for f in files:
        n = int(re.search(r"_n(\d+)", f).group(1))
        by_variant = {}
        with open(f) as fh:
            for r in csv.DictReader(fh):
                by_variant.setdefault(r["variant"], []).append(float(r["algo_ns"]))
        per_n[n] = {k: statistics.median(v) for k, v in by_variant.items()}

    names = sorted({k for d in per_n.values() for k in d})
    print("median algo_ns")
    print("%-9s %s" % ("n", "  ".join("%-24s" % x for x in names)))
    for n in sorted(per_n):
        print("%-9d %s"
              % (n, "  ".join("%-24.1f" % per_n[n].get(x, float("nan")) for x in names)))
    print()
    print("ratios, each row independent of the others")
    print("%-9s %-22s %-22s %s"
          % ("n", "naive / windowed", "windowed / native", "naive / native"))
    for n in sorted(per_n):
        d = per_n[n]
        print("%-9d %-22.2f %-22.2f %.2f"
              % (n,
                 d["bitpack-plan-naive"] / d["bitpack-plan-windowed"],
                 d["bitpack-plan-windowed"] / d["bitpack-plan-native"],
                 d["bitpack-plan-naive"] / d["bitpack-plan-native"]))
    print()
    print("what this does and does not say:")
    print("  * the magnitude 47 and 48 call unpriced is priced. an access plan derived per element")
    print("    at runtime costs about 3.1 times one that is a compile-time constant, stable across")
    print("    four sizes spanning the L1 boundary, against a dense-carrier competitor arm.")
    print("  * it prices COMPILE TIME AGAINST RUNTIME, not derivation against site. a site holding")
    print("    the width and the strategy's rule as consts could reach the windowed number itself.")
    print("    what the bench bounds is the cost of the plan not being a const at all.")
    print("  * the windowed arm's plan is P = 8 / gcd(W, 8) fields per whole byte group. that gcd is")
    print("    the same phase-period fact p4 derives and 16:179-180 generalises wrongly from W=13.")
    print("    an independent instance of p4's finding, in committed harness code by another author.")


if __name__ == "__main__":
    main()
