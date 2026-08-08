#!/usr/bin/env python3
"""
p4. Do two strategy objectives pick the same mechanism, and how many
objectives can this space usefully distinguish?

38 states the model and names the test nobody has run:

    "All of them should be decided by measurement, just measuring different
     things, and, this is I think the mental unlock: They weigh different
     measurements differently. For the most part, they probably agree, because
     in general, the best answer fits all, because it fights none of their
     intent. But perhaps my instinct is wrong there, and all truly differ for
     the most part."

That is a testable claim about the design and it has never been tested. This
probe tests it on the one place in the repository where both quantities are
available for the same arms:

  TIME  is measured, from the committed harness family
        mock/benches/bitpack-carrier-width_n*, six record counts, six variants,
        40 warm-mode samples per variant, framework mockspace-bench-harness,
        rustc 1.98.0-nightly (57d06900f 2026-05-27), Apple M1. It is NOT
        re-measured here; the committed CSVs are read. Warm-mode rows are the
        steady-state measurement; the cold-mode rows in the same CSV are
        first-touch and are three orders of magnitude larger, which the
        self-check below is what caught when the first version of this probe
        pooled them.

  SPACE is arithmetic, not a measurement. The family's own title says
        "Packed 13-bit against u16, u32 and u64 dense carriers", so the bytes
        per element follow from the arm name: a dense u16 arm stores 2, u32
        stores 4, u64 stores 8, and a packed 13-bit arm stores 13/8.

This is an ANALYSIS OF COMMITTED HARNESS OUTPUT plus arithmetic. It is not a
bench, it prices nothing new, and every timing number in it belongs to the run
that produced it.

Three quantities are reported per record count, and they answer different
questions:

  argmin agreement      does a pure-time objective pick the same arm as a
                        pure-space objective? This is 38's instinct, directly.

  Pareto set            the arms no other arm beats on both objectives at once.
                        Any objective monotone in both can pick any of these
                        and no others, so its size is an upper bound on how many
                        mechanism answers the objective space can distinguish
                        here.

  hull set              the arms attainable as the argmin of some non-negative
                        LINEAR weighting. A subset of the Pareto set, and the
                        right bound if a strategy is a linear weighting rather
                        than a general preference.

The hull and Pareto SETS are invariant under rescaling either axis by a positive
affine map, so they do not depend on how the probe normalises. The weighting at
which the winner SWITCHES does depend on it, so switch points are reported as
what they are and are not carried as findings.

Self-validation: the probe recomputes medians the committed findings files
already state, and compares. A parse that disagrees with the committed findings
is a parse bug, and the check is printed rather than assumed.
"""

import csv
import glob
import json
import os
import re
import statistics
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.abspath(os.path.join(HERE, "..", "..", "..", "benches"))

# Bytes per element, from the family title's own statement of what each arm
# stores. 13-bit elements.
ELEMENT_BITS = 13
BYTES_PER_ELEMENT = {
    "bitpack-carrier-d16": 2.0,
    "bitpack-carrier-d16-control": 2.0,
    "bitpack-carrier-d32": 4.0,
    "bitpack-carrier-d64": 8.0,
    "bitpack-carrier-packed": ELEMENT_BITS / 8.0,
    "bitpack-carrier-packed-simd": ELEMENT_BITS / 8.0,
}


def load_family(pattern):
    """{n: {variant: [e2e_ns, ...]}} from the committed CSVs."""
    out = {}
    files = sorted(glob.glob(os.path.join(BENCH, pattern)))
    for path in files:
        m = re.search(r"_n(\d+)\.csv$", path)
        if not m:
            continue
        n = int(m.group(1))
        per = {}
        with open(path, newline="") as fh:
            for row in csv.DictReader(fh):
                if row["mode"] != "warm":
                    continue
                per.setdefault(row["variant"], []).append(float(row["e2e_ns"]))
        out[n] = per
    return out, files


def stated_medians(n):
    """Medians the committed findings file states, for the self-check."""
    path = os.path.join(BENCH, f"bitpack-carrier-width_n{n}_findings.md")
    if not os.path.exists(path):
        return {}
    stated = {}
    with open(path) as fh:
        body = fh.read()
    # Scope to one named table. The findings file carries several tables whose
    # rows have the same shape (function-under-test only, per-cooldown splits),
    # and taking the last match silently reads a different quantity. The first
    # version of this probe did exactly that.
    section = None
    for line in body.splitlines():
        if line.startswith("## "):
            section = line.strip()
            continue
        if section != "## End-to-end (all cooldowns combined)":
            continue
        cells = [c.strip() for c in line.split("|")]
        # | <name> | mean | median | best 20% | mid 60% | worst 20% | delta |
        if len(cells) < 5:
            continue
        name = cells[1]
        if name not in BYTES_PER_ELEMENT:
            continue
        m = re.match(r"^(\d+)ns$", cells[3])
        if m:
            stated[name] = float(m.group(1))
    return stated


def pareto(points):
    """points: {name: (time, space)}. Lower is better on both."""
    keep = []
    for a, (ta, sa) in points.items():
        dominated = False
        for b, (tb, sb) in points.items():
            if b == a:
                continue
            if tb <= ta and sb <= sa and (tb < ta or sb < sa):
                dominated = True
                break
        if not dominated:
            keep.append(a)
    return sorted(keep)


def hull_winners(points, steps=100001):
    """Arms attainable as argmin of w*time_norm + (1-w)*space_norm.

    Min-max normalisation per axis; the winner SET is invariant to it, the
    switch points are not.
    """
    names = sorted(points)
    ts = [points[n][0] for n in names]
    ss = [points[n][1] for n in names]
    tlo, thi = min(ts), max(ts)
    slo, shi = min(ss), max(ss)
    tn = {n: (points[n][0] - tlo) / (thi - tlo) if thi > tlo else 0.0 for n in names}
    sn = {n: (points[n][1] - slo) / (shi - slo) if shi > slo else 0.0 for n in names}
    winners = []
    switches = []
    prev = None
    for i in range(steps):
        w = i / (steps - 1)
        # Tie-break on time so a weakly dominated arm (identical space, worse
        # time) cannot appear as a winner. Without this, w = 0 returns whichever
        # of two equal-space arms sorts first, which put a dominated arm in the
        # hull set on the probe's first run.
        best = min(names, key=lambda n: (w * tn[n] + (1.0 - w) * sn[n], points[n][0]))
        if best != prev:
            switches.append((w, best))
            winners.append(best)
            prev = best
    return winners, switches


def main():
    fam, files = load_family("bitpack-carrier-width_n*.csv")
    if not fam:
        print(f"no committed family found under {BENCH}", file=sys.stderr)
        return 1

    print("SOURCE")
    print(f"  bench directory: {BENCH}")
    print(f"  csv files read:  {len(files)}")
    meta_path = os.path.join(BENCH, "bitpack-carrier-width_n1048576.meta.json")
    with open(meta_path) as fh:
        meta = json.load(fh)
    print(f"  harness: {meta['framework']}, {meta['rustc']}, {meta['cpu']}, {meta['os']}")
    print()

    print("SELF-CHECK: recomputed medians against the committed findings files")
    bad = 0
    checked = 0
    for n in sorted(fam):
        stated = stated_medians(n)
        for variant, samples in sorted(fam[n].items()):
            if variant not in stated:
                continue
            mine = statistics.median(samples)
            checked += 1
            # The findings round to whole nanoseconds.
            if abs(mine - stated[variant]) > 1.0:
                bad += 1
                print(f"  MISMATCH n={n} {variant}: mine {mine:.1f}, stated {stated[variant]:.1f}")
    print(f"  medians compared: {checked}, mismatches: {bad}")
    if bad:
        print("  parse disagrees with the committed findings; stopping.")
        return 1
    print()

    print("PER RECORD COUNT: does a pure-time objective pick what a pure-space one picks?")
    print()
    print("  {:>10}  {:<28} {:<28} {:>7}".format("records", "time-argmin (fastest)", "space-argmin (smallest)", "agree?"))
    agree_count = 0
    all_points = {}
    for n in sorted(fam):
        points = {}
        for variant, samples in fam[n].items():
            if variant not in BYTES_PER_ELEMENT:
                continue
            points[variant] = (statistics.median(samples), BYTES_PER_ELEMENT[variant])
        all_points[n] = points
        tmin = min(points, key=lambda v: points[v][0])
        smin = min(points, key=lambda v: (points[v][1], points[v][0]))
        agree = tmin == smin
        agree_count += 1 if agree else 0
        print("  {:>10}  {:<28} {:<28} {:>7}".format(n, tmin, smin, "yes" if agree else "NO"))
    print()
    print(f"  record counts where the two objectives agree: {agree_count} of {len(fam)}")
    print()

    print("DOES ONE OBJECTIVE EVEN GIVE ONE ANSWER ACROSS THE SWEEP?")
    print()
    t_ans, s_ans = [], []
    for n in sorted(fam):
        points = all_points[n]
        t_ans.append(min(points, key=lambda v: points[v][0]))
        s_ans.append(min(points, key=lambda v: (points[v][1], points[v][0])))
    print(f"  pure-time objective, distinct winning arms across the six record counts: "
          f"{len(set(t_ans))}  {sorted(set(a.replace('bitpack-carrier-','') for a in t_ans))}")
    print(f"  pure-space objective, distinct winning arms across the six record counts: "
          f"{len(set(s_ans))}  {sorted(set(a.replace('bitpack-carrier-','') for a in s_ans))}")
    print("  -> a single objective's mechanism answer is not a constant either.")
    print("     It is a function of the workload, which is 25 section 4.4's claim")
    print("     that a strategy assigns functions rather than values, holding on")
    print("     an axis 25 did not measure.")
    print()

    print("HOW MANY MECHANISM ANSWERS THIS OBJECTIVE SPACE DISTINGUISHES")
    print()
    print("  {:>10}  {:>6}  {:<44}  {:>4}  {}".format("records", "|par|", "Pareto set", "|hul|", "linear-weighting winners, in order"))
    for n in sorted(fam):
        points = all_points[n]
        par = pareto(points)
        win, sw = hull_winners(points)
        assert set(win) <= set(par), (n, set(win) - set(par))
        short = lambda s: s.replace("bitpack-carrier-", "")
        print("  {:>10}  {:>6}  {:<44}  {:>4}  {}".format(
            n, len(par), ", ".join(short(p) for p in par), len(win),
            " -> ".join(short(w) for w in win)))
    print()

    print("  switch points, reported as an artifact of min-max normalisation")
    print("  rather than as a finding (the winner SET is normalisation-free,")
    print("  the switch weight is not):")
    for n in sorted(fam):
        _, sw = hull_winners(all_points[n])
        pretty = ", ".join(f"w>={w:.4f}: {b.replace('bitpack-carrier-','')}" for w, b in sw)
        print(f"    n={n}: {pretty}")
    print()

    print("MEDIANS AND BYTES, so every number above can be re-derived")
    print()
    for n in sorted(fam):
        print(f"  n = {n}")
        for variant in sorted(all_points[n]):
            t, s = all_points[n][variant]
            print(f"    {variant:<30} {t:>12.1f} ns   {s:>6.3f} B/elem")
    return 0


if __name__ == "__main__":
    sys.exit(main())
