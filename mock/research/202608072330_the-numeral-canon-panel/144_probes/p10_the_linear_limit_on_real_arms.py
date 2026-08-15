"""p10: 139's O-139-C, answered on real arms instead of invented ones.

THIS IS NOT A BENCH RUN. It takes no measurement. It reads the CSV files the
mockspace bench harness already committed under `mock/benches/` and does exact
arithmetic on them. The numbers are the harness's; the analysis is mine, and
where I say "measured" about a time I mean the harness measured it.

139's O-139-C closes on this and says so: "finding one real arm in
`mock/benches/variants/` that is Pareto-optimal and inside the convex hull of
its neighbours. If none exists in the arm sets the library actually has, linear
is enough and the limit is theoretical." Every arm set in p1 through p9 is
synthetic, so that question is still open after all of them.

The cost table is built without inventing a coordinate. Each bench family runs
the same variants at several input sizes, so a variant's cost vector is its time
at each size, and a weighting over those coordinates is a consumer saying which
working-set sizes they care about. That is a real weighting over real costs and
every entry traces to a committed CSV.

PREDICTIONS, before running:
  AA1 at least one real family contains a Pareto-optimal arm that no linear
      weighting can select. If none does, O-139-C closes the other way and the
      linear limit is theoretical for the arm sets arvo actually has.
  AA2 in the majority of families the linear image is strictly smaller than the
      non-dominated count, matching p3's synthetic result.
  AA3 in at least one family the arm chosen with all the weight on the smallest
      size differs from the arm chosen with all the weight on the largest, so
      the coordinate axis genuinely reorders arms on committed data.

CONTROLS:
  BB1 no dominated arm may be strictly selectable, in any family.
  BB2 THE ROBUSTNESS CONTROL. Every verdict is recomputed with the per-variant
      MEAN in place of the MEDIAN. A family whose Pareto set or linear image
      changes between the two has a verdict that is an artifact of the summary
      statistic, and it is reported and excluded from the headline counts.
  BB3 THE NON-VACUITY CONTROL. For each family the arm-to-arm spread is compared
      against the within-arm batch spread. A family whose arms differ by less
      than the noise of a single arm is not an arm set and is excluded.
"""

import csv
import glob
import os
import re
import statistics
from fractions import Fraction as F

from exact_lp import strictly_selectable, dominated

BENCH = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                     "..", "..", "..", "benches")
BENCH = os.path.normpath(BENCH)

fail = []


def load_families():
    fam = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "*_n*.csv"))):
        m = re.match(r"(.+)_n(\d+)\.csv$", os.path.basename(path))
        if not m:
            continue
        name, size = m.group(1), int(m.group(2))
        rows = list(csv.DictReader(open(path)))
        per = {}
        for r in rows:
            per.setdefault(r["variant"], []).append(float(r["algo_ns"]))
        fam.setdefault(name, {})[size] = per
    out = {}
    for name, sizes in fam.items():
        keys = sorted(sizes)
        if len(keys) < 3:
            continue
        vs = sorted(sizes[keys[0]])
        if len(vs) < 4 or any(sorted(sizes[k]) != vs for k in keys):
            continue
        out[name] = (keys, vs, sizes)
    return out


def cost_matrix(keys, vs, sizes, stat):
    """One row per variant, one column per size point, in exact Fractions."""
    mat = []
    for v in vs:
        row = []
        for k in keys:
            samples = sizes[k][v]
            val = stat(samples)
            # scale to integers so the LP stays in small rationals
            row.append(F(int(round(val * 10)), 10))
        mat.append(tuple(row))
    return mat


def spreads(keys, vs, sizes):
    """(arm-to-arm spread, worst within-arm spread), per size, as ratios."""
    ratios = []
    for k in keys:
        meds = [statistics.median(sizes[k][v]) for v in vs]
        arm_spread = max(meds) - min(meds)
        within = 0.0
        for v in vs:
            s = sorted(sizes[k][v])
            q1, q3 = s[len(s) // 4], s[(3 * len(s)) // 4]
            within = max(within, q3 - q1)
        ratios.append(arm_spread / within if within > 0 else float("inf"))
    return min(ratios), max(ratios)


def analyse(mat):
    n = len(mat)
    par = [i for i in range(n) if not dominated(mat, i)]
    lin = [i for i in range(n) if strictly_selectable(mat, i)]
    return set(par), set(lin)


families = load_families()
print(f"read {len(families)} bench families from {BENCH}")
print()
print(f"{'family':<30} {'arms':>4} {'coords':>6} {'pareto':>6} {'linear':>6} "
      f"{'unreach':>7} {'noise ratio':>12} {'stable':>7}")

unreachable_total = 0
families_with_unreachable = []
strictly_smaller = 0
counted = 0
excluded_noise = []
excluded_unstable = []
reorder_families = []

for name in sorted(families):
    keys, vs, sizes = families[name]
    lo_ratio, hi_ratio = spreads(keys, vs, sizes)
    med = cost_matrix(keys, vs, sizes, statistics.median)
    mean = cost_matrix(keys, vs, sizes, statistics.fmean)
    par_m, lin_m = analyse(med)
    par_a, lin_a = analyse(mean)
    stable = (par_m == par_a) and (lin_m == lin_a)
    unreach = sorted(par_m - lin_m)
    bad = [i for i in lin_m if dominated(med, i)]
    if bad:
        fail.append(f"BB1 {name}")
    noisy = lo_ratio < 1.0
    print(f"{name:<30} {len(vs):>4} {len(keys):>6} {len(par_m):>6} {len(lin_m):>6} "
          f"{len(unreach):>7} {lo_ratio:>12.1f} {str(stable):>7}")
    if noisy:
        excluded_noise.append(name)
        continue
    if not stable:
        excluded_unstable.append(name)
        continue
    counted += 1
    if len(lin_m) < len(par_m):
        strictly_smaller += 1
    if unreach:
        unreachable_total += len(unreach)
        families_with_unreachable.append((name, [vs[i] for i in unreach]))
    # AA3: does the extreme-weight choice move across the coordinate axis?
    first = min(range(len(vs)), key=lambda i: (med[i][0], i))
    last = min(range(len(vs)), key=lambda i: (med[i][-1], i))
    if first != last:
        reorder_families.append((name, vs[first], vs[last]))

print()
print(f"BB3 families excluded for arm spread below within-arm spread: "
      f"{len(excluded_noise)} {excluded_noise}")
print(f"BB2 families excluded because median and mean disagree on the verdict: "
      f"{len(excluded_unstable)} {excluded_unstable}")
print(f"BB1 dominated arms reported strictly selectable: "
      f"{'0, PASS' if not fail else 'FAIL ' + str(fail)}")
print(f"families surviving both controls: {counted}")

print()
print("=" * 78)
print("AA1. is there a real Pareto arm no linear weighting can select?")
print("=" * 78)
if families_with_unreachable:
    print(f"  YES. {unreachable_total} such arms across "
          f"{len(families_with_unreachable)} families:")
    for name, arms in families_with_unreachable:
        print(f"    {name}: {arms}")
    print("  O-139-C closes in the direction 139 flagged as the live one: the limit")
    print("  is not theoretical, it is present in the arm sets arvo has today.")
else:
    print("  NO. Every non-dominated arm in every surviving family is reachable by")
    print("  some linear weighting. O-139-C closes the other way for this corpus,")
    print("  and the linear limit is theoretical HERE while remaining real in")
    print("  general, per p2's 11.7% over random arm sets.")
print(f"  AA1 -> {'CONFIRMED' if families_with_unreachable else 'REFUTED'}")

print()
print("=" * 78)
print("AA2. is the linear image smaller than the Pareto count on real arms?")
print("=" * 78)
print(f"  strictly smaller in {strictly_smaller} of {counted} surviving families "
      f"({100.0 * strictly_smaller / counted:.0f}%)")
aa2 = strictly_smaller * 2 > counted
print(f"  AA2 -> {'CONFIRMED' if aa2 else 'REFUTED'}")

print()
print("=" * 78)
print("AA3. does the coordinate axis reorder arms on committed data?")
print("=" * 78)
print(f"  families where the smallest-size winner is not the largest-size winner: "
      f"{len(reorder_families)} of {counted}")
for name, a, b in reorder_families[:12]:
    print(f"    {name}: smallest -> {a}, largest -> {b}")
aa3 = bool(reorder_families)
print(f"  AA3 -> {'CONFIRMED' if aa3 else 'REFUTED'}")
print("  this is p4's portability phenomenon on committed measurements rather than")
print("  invented cost tables: one weighting, two answers, and the difference is a")
print("  fact about the workload rather than about the design.")

print()
print("=" * 78)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 78)
raise SystemExit(1 if fail else 0)
