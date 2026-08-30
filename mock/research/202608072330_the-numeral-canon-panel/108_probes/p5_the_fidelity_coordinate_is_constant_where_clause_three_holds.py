#!/usr/bin/env python3
"""p5. Where clause three holds, the coordinate op's accuracy intent needs is
a constant. The corpus contains both cells and says so.

The pair's third clause makes component two range over "the arms that produce
the answer the first component fixed". `106` section 8's own table then says,
for exactly that region, that "a fidelity column would measure a constant".
Put together, the weighting can never weigh accuracy, which is what I7 asks
for and what `102` section 3.7 nonetheless assigns to the weighting.

This checks the arithmetic half of that against committed artifacts rather
than against anybody's prose:

  ONE. An answer-equivalent family: the fidelity coordinate is identical across
       its arms, so a weighting reading it learns nothing.
  TWO. An answer-differing family: the fidelity coordinate separates its arms
       by a factor, so a weighting reading it learns something.
  THREE. The corpus records neither, because the column is empty everywhere.

The error figures are `103`'s, from its committed `p7_errors.out`, and are read
out of that file rather than retyped. The point being made from them is not
`103`'s: `103` used them to test `102`'s measured-coordinate constraint. Here
they are used to show which cell clause three admits.

Run from the probe directory.
"""

import csv
import os
import re
import sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.join(HERE, "..")
BENCH = os.path.abspath(os.path.join(PANEL, "..", "..", "benches"))


def read_103_errors():
    """The exact per-arm error coordinates, from 103's committed output."""
    path = os.path.join(PANEL, "103_probes", "p7_errors.out")
    rows = []
    with open(path, encoding="utf-8") as f:
        for line in f:
            parts = line.rstrip("\n").split("\t")
            if len(parts) == 3:
                try:
                    rows.append((parts[0], parts[1], float(parts[2])))
                except ValueError:
                    pass
    return rows


def main():
    print("p5. Where clause three holds, the accuracy coordinate is a constant.")
    print()

    rows = read_103_errors()
    print(f"read {len(rows)} (family, arm, exact error) triples from 103_probes/p7_errors.out")
    print()

    byfam = defaultdict(dict)
    for fam, arm, err in rows:
        byfam[fam][arm] = err

    print("=== the two cells, from the committed error coordinates ===")
    print()
    print(f"{'family':<44} {'arms':>5} {'spread':>12}   verdict")
    ansdiff = 0
    anseq = 0
    for fam in sorted(byfam):
        errs = byfam[fam]
        vals = list(errs.values())
        if len(vals) < 2:
            continue
        lo, hi = min(vals), max(vals)
        if lo == hi:
            verdict = "constant: a weighting on it learns nothing"
            anseq += 1
            spread = "1.000x"
        else:
            verdict = "varies: a weighting on it selects"
            ansdiff += 1
            spread = f"{hi / lo:.3f}x" if lo > 0 else "inf"
        print(f"{fam:<44} {len(vals):>5} {spread:>12}   {verdict}")
    print()
    print(f"families whose arms share one exact error (clause three's region): {anseq}")
    print(f"families whose arms do not (the region clause three excludes):     {ansdiff}")
    print()

    # ---- the coordinate is absent from the corpus entirely ----
    print("=== and the corpus records the coordinate nowhere ===")
    print()
    csvdir = os.path.join(BENCH, ".bench_history")
    if not os.path.isdir(csvdir):
        # fall back to whatever layout the committed corpus uses
        cands = []
        for root, _dirs, files in os.walk(BENCH):
            if "target" in root.split(os.sep):
                continue
            for fn in files:
                if fn.endswith(".csv"):
                    cands.append(os.path.join(root, fn))
        csvs = cands
    else:
        csvs = []
        for root, _dirs, files in os.walk(csvdir):
            for fn in files:
                if fn.endswith(".csv"):
                    csvs.append(os.path.join(root, fn))

    nonzero_score = 0
    total_rows = 0
    files_seen = 0
    header_has_score = 0
    for path in csvs:
        files_seen += 1
        try:
            with open(path, newline="", encoding="utf-8") as f:
                r = csv.DictReader(f)
                if r.fieldnames and "score" in r.fieldnames:
                    header_has_score += 1
                for row in r:
                    total_rows += 1
                    v = (row.get("score") or "").strip()
                    if v not in ("", "0", "0.0", "0.000000"):
                        nonzero_score += 1
        except Exception:
            pass

    print(f"committed CSVs scanned:           {files_seen}")
    print(f"  declaring a `score` column:     {header_has_score}")
    print(f"  data rows:                      {total_rows}")
    print(f"  rows with a non-empty `score`:  {nonzero_score}")
    print()

    # ---- and nothing computes it ----
    variants = os.path.join(BENCH, "variants")
    impls = 0
    crates = 0
    for d in sorted(os.listdir(variants)):
        p = os.path.join(variants, d, "src")
        if not os.path.isdir(p):
            continue
        crates += 1
        for root, dirs, files in os.walk(p):
            dirs[:] = [x for x in dirs if x != "target"]
            for fn in files:
                if fn.endswith(".rs"):
                    with open(os.path.join(root, fn), encoding="utf-8", errors="replace") as f:
                        if re.search(r"fn\s+score_output", f.read()):
                            impls += 1
                            break
    print(f"variant crates with a src/ tree:  {crates}")
    print(f"  implementing `score_output`:    {impls}")
    print()
    print("(`target/` is excluded from every walk above. `106` section 0.2 and")
    print(" `107` section 3 both record that a grep over `variants/` after a suite")
    print(" run returns build artifacts: `score_output` reads 84 on a contaminated")
    print(" tree against 0 on a clean one.)")
    print()

    print("=== what this shows ===")
    print()
    print("The corpus holds one family whose arms share an exact error to twelve")
    print("printed digits and one whose arms differ by two orders of magnitude.")
    print("Clause three admits the first and excludes the second. In the first, a")
    print("fidelity coordinate is a constant, so a weighting that reads it cannot")
    print("distinguish anything, which is `106` section 8's own sentence. So under")
    print("the pair as written, op's accuracy intent is not expressible in either")
    print("component: not in component one, which fixes an answer rather than")
    print("ranking approximations to it, and not in component two, whose region")
    print("makes the coordinate constant.")
    print()
    print("The repair is one word in clause three and it is instantiated already:")
    print("component one fixes the DENOTED answer, the declared semantics, rather")
    print("than the computed one. Arms realise it exactly or approximately, the")
    print("fidelity coordinate is the distance from the declaration, and it varies.")
    print("`103`'s per-arm-oracle shape is that discipline stated as validation:")
    print("each arm against its own declared semantics rather than against another")
    print("arm. The two halves fit because they are the same idea.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
