#!/usr/bin/env python3
"""What cost coordinates does this repository actually carry?

The unit has settled on "a weighting over cost coordinates plus a cost table"
as the object a strategy is. Nobody has asked what the coordinates ARE. Three
members have used exactly three: the median of `algo_ns`, a bytes-per-element
number typed into a probe by hand, and the interquartile range of the same
`algo_ns` samples the first coordinate is a median of.

This census answers three questions mechanically, and the interesting part is
that they have different answers:

  1. Which of the harness CSV's 17 columns carry information in the committed
     corpus? (Realised.)
  2. Which columns exist in the schema, are filled by a mechanism the harness
     already ships, and are empty here because no arvo variant uses that
     mechanism? (Reachable, and the distance to it is a code change in the
     variants or a flag on the run.)
  3. Which of the coordinates op's stated strategy intents name has no column
     at all? (Absent, and reaching it is instrument work.)

Method. Read every committed `*.csv` under `mock/benches/`, count for each
column how many rows carry something other than empty/zero, and how many files
carry any such row. Then read every variant crate's source and count which
`Routine` hooks and which measurement constructors appear. Then group the CSVs
into families and report arms, regions and whether a byte-identical noise-floor
control arm is present.

This is a census of committed artifacts. It is NOT a bench, no measurement was
taken, and no number here prices anything.

Run:  python3 p1_the_coordinate_census.py
"""

import collections
import csv
import glob
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))
VAR = os.path.join(BENCH, "variants")

# The `Routine` surface, from the pinned mockspace checkout
# `bench-core/src/lib.rs`. Each is a hook a variant crate MAY implement; the
# default of every one of them is "no coordinate".
ROUTINE_HOOKS = [
    "validate_output",
    "score_output",
    "score_label",
    "score_dimensions",
    "outputs_may_differ",
    "compare_epsilon",
]

# The measurement constructors. `timed!` fills `run_ticks` only; the harness's
# own doc says the other three fields are "populated by the matrix scaffold" and
# that "a zero in any of the three latter fields means not measured by this
# constructor, never a measured zero" (bench-core/src/lib.rs:420-447).
CONSTRUCTORS = ["timed!", "timed_calibrated!", "bench_matrix", "setup_ticks", "first_ticks", "digest"]


def field_census():
    files = sorted(glob.glob(os.path.join(BENCH, "*.csv")))
    fields = None
    informative = collections.Counter()
    total = collections.Counter()
    files_with = collections.Counter()
    distinct = collections.defaultdict(set)
    for path in files:
        with open(path) as fh:
            rdr = csv.DictReader(fh)
            fields = rdr.fieldnames
            seen = set()
            for row in rdr:
                for k, v in row.items():
                    total[k] += 1
                    if len(distinct[k]) < 6:
                        distinct[k].add(v)
                    if v not in ("", "0", "0.0", "0.00"):
                        informative[k] += 1
                        seen.add(k)
            for k in seen:
                files_with[k] += 1
    return files, fields, informative, total, files_with, distinct


def varies_between_arms():
    """For each numeric column: does it ever differ between two arms at one region?"""
    cols = ["run", "pass", "batch_idx", "e2e_ns", "algo_ns", "bridge_ns", "batch_count"]
    out = {c: [False, 0] for c in cols}
    for path in sorted(glob.glob(os.path.join(BENCH, "*.csv"))):
        per = collections.defaultdict(lambda: collections.defaultdict(list))
        with open(path) as fh:
            for row in csv.DictReader(fh):
                for c in cols:
                    try:
                        per[c][row["variant"]].append(float(row[c]))
                    except ValueError:
                        pass
        for c in cols:
            arms = per[c]
            if len(arms) < 2:
                continue
            out[c][1] += 1
            means = [sum(v) / len(v) for v in arms.values()]
            if max(means) != min(means):
                out[c][0] = True
    return {k: tuple(v) for k, v in out.items()}


def source_census():
    counts = collections.Counter()
    files = collections.defaultdict(set)
    for path in glob.glob(os.path.join(VAR, "*", "src", "*.rs")):
        src = open(path).read()
        crate = path.split(os.sep)[-3]
        for h in ROUTINE_HOOKS:
            if re.search(r"fn\s+" + h + r"\b", src):
                counts[h] += 1
                files[h].add(crate)
        for c in CONSTRUCTORS:
            n = src.count(c)
            if n:
                counts[c] += n
                files[c].add(crate)
    return counts, files


def family_census():
    fams = collections.defaultdict(lambda: {"regions": set(), "arms": set()})
    for path in sorted(glob.glob(os.path.join(BENCH, "*.csv"))):
        base = os.path.basename(path)
        m = re.match(r"(.+)_n(\d+)\.csv$", base)
        if not m:
            continue
        fam, n = m.group(1), int(m.group(2))
        fams[fam]["regions"].add(n)
        with open(path) as fh:
            for row in csv.DictReader(fh):
                fams[fam]["arms"].add(row["variant"])
    return fams


def main():
    files, fields, informative, total, files_with, distinct = field_census()
    print("=" * 78)
    print("1. THE HARNESS SCHEMA AGAINST THE COMMITTED CORPUS")
    print("=" * 78)
    print(f"committed CSV files: {len(files)}   data rows: {total[fields[0]]}")
    print()
    print(f"{'column':14s} {'rows w/ info':>13s} {'files w/ any':>13s}  distinct values seen")
    for k in fields:
        vals = sorted(distinct[k])[:4]
        show = ",".join(v if v != "" else "<empty>" for v in vals)
        if len(distinct[k]) > 4:
            show += ",..."
        print(f"{k:14s} {informative[k]:13d} {files_with[k]:13d}  {show}")
    # A column carries a COORDINATE only if it can separate two arms measured at
    # the same region. A column that is constant across the whole corpus, or that
    # indexes the sample rather than measuring it, cannot. Measure that rather
    # than eyeballing it.
    varying = varies_between_arms()
    print()
    print("of the columns carrying information, which VARY BETWEEN ARMS at a fixed")
    print("region (the only ones a cost coordinate can be built from):")
    for k, (v, tested) in sorted(varying.items()):
        verdict = "varies" if v else "constant across arms"
        print(f"    {k:14s} {verdict}   ({tested} (family, region) groups tested)")

    dead = [k for k in fields if informative[k] == 0]
    live = [k for k in fields if informative[k] > 0]
    print()
    print(f"columns carrying information : {len(live):2d}  {live}")
    print(f"columns identically empty/zero: {len(dead):2d}  {dead}")

    counts, srcfiles = source_census()
    print()
    print("=" * 78)
    print("2. WHICH HOOKS THE VARIANTS IMPLEMENT")
    print("=" * 78)
    nvar = len(glob.glob(os.path.join(VAR, "*", "Cargo.toml")))
    nsrc = len(glob.glob(os.path.join(VAR, "*", "src", "*.rs")))
    print(f"variant crates: {nvar}   source files scanned: {nsrc}")
    print()
    print(f"{'Routine hook':22s} {'crates implementing':>20s}")
    for h in ROUTINE_HOOKS:
        print(f"{h:22s} {len(srcfiles[h]):20d}")
    print()
    print(f"{'constructor / field':22s} {'occurrences':>12s} {'crates':>8s}")
    for c in CONSTRUCTORS:
        print(f"{c:22s} {counts[c]:12d} {len(srcfiles[c]):8d}")

    fams = family_census()
    print()
    print("=" * 78)
    print("3. FAMILIES, ARMS, REGIONS, AND WHICH CARRY A NOISE-FLOOR CONTROL")
    print("=" * 78)
    ctl = 0
    print(f"{'family':34s} {'regions':>8s} {'arms':>5s}  control arm")
    for fam in sorted(fams):
        r = len(fams[fam]["regions"])
        arms = sorted(fams[fam]["arms"])
        has = [a for a in arms if a.endswith("-control")]
        if has:
            ctl += 1
        print(f"{fam:34s} {r:8d} {len(arms):5d}  {has[0] if has else ''}")
    print()
    print(f"families: {len(fams)}   with a byte-identical control arm: {ctl}")

    print()
    print("=" * 78)
    print("4. THE THREE TIERS")
    print("=" * 78)
    print("REALISED  (a coordinate can be computed from the committed corpus today):")
    for k in live:
        print(f"            {k}")
    print()
    print("REACHABLE (the column exists, the harness fills it, no arvo variant asks):")
    print("            setup_ns, first_ns, digest  <- the matrix scaffold constructor;")
    print("                                           every arvo variant uses timed!")
    print("            score                       <- Routine::score_output / score_dimensions;")
    print("                                           implemented by 0 arvo variants")
    print("            instructions, cycles        <- --perf-counters, needs sudo on an")
    print("                                           Apple-Silicon host (bench-harness/src/perf.rs)")
    print()
    print("ABSENT    (no column, reaching it is instrument work):")
    print("            bytes per element           <- declared by hand in every probe that")
    print("                                           weighs storage; no bench measures a size")
    print("            accuracy / divergence       <- see the digest and score rows above:")
    print("                                           a slot exists, the QUANTITY does not")
    print("            compile time                <- schema is entirely runtime fields")


if __name__ == "__main__":
    main()
