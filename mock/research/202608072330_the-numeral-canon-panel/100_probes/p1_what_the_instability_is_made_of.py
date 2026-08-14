#!/usr/bin/env python3
"""p1. What `98`'s section instability is actually made of.

`98`'s F-98-10 reports that a fixed weighting produces 30, 8 and 77 distinct
sections across 2000 bootstrap resamples of the committed carrier run, and that
for one weighting the committed section is not the modal one. `98` section 3
draws from that: "a section is not stable enough to be the object a check is
applied to", and proposes generating the table from the weighting instead.

The finding reproduces. The inference is what this probe tests, by asking a
question `98` did not: WHICH comparisons are moving, and does moving cost
anything.

Three measurements, all on the same committed samples `98` used, with an
independently written extraction and bootstrap:

  1. Per-region flip rate, and the identity of the arms that trade places.
  2. REGRET of holding the committed section instead of each resample's own
     argmin, as a fraction of the region's achievable objective RANGE. A region
     where two arms score a hair apart flips constantly and costs a hair when it
     flips, and a statistic that counts that identically to a real reversal is
     measuring the wrong thing.
  3. The same, with `bitpack-carrier-d16-control` dropped. That arm is a
     NOISE-FLOOR CONTROL: its own doc comment says it "calls the identical
     `sum_d16` on the identical region with the identical arguments as
     `bitpack-carrier-d16`, so the two arms differ only in the exported symbol
     name and must compile to the same machine code. Any measured gap between
     them is the harness's own resolution on this workload."
     (`mock/benches/variants/bitpack-carrier-d16-control/src/lib.rs:1-8`)

Point 3 is the one that decides the reading. If the section's instability is
carried by an arm the bench declares to be a duplicate, then the instability is
the harness's declared noise floor appearing in the argmin, not a property of
the table.

WHICH REGRET STATISTIC, AND A DEFECT IN THE FIRST VERSION OF THIS PROBE.

The first run reported regret as (held - best) / best on the min-max normalised
objective. That is meaningless and its output is kept at
`p1_v1_relative_regret_is_meaningless.out` rather than deleted. Min-max
normalisation maps the global minimum of each coordinate to zero, so the
objective's ORIGIN is an artifact of the arm set: a region whose best arm scores
near zero produces a near-zero denominator and a regret in the thousands of
percent, which is a fact about the normalisation and not about the choice. The
run reported a maximum regret of 5773% for exactly that reason.

What replaces it is invariant to the origin. `w . c` under min-max normalisation
is affine in the raw costs, so differences of objective values are meaningful
and levels are not. Regret is therefore reported as

    (held - best) / (worst - best)

at that region: 0 means the held arm is the best available, 1 means it is the
worst. The contending band is defined the same way, as a fraction of the
region's achievable range, which is what makes it comparable across regions and
across weightings. `97` section 5 used a contending set at a tolerance on raw
times for the same reason, on the ground that "at arity 8 three arms sit within
two percent of each other and a strict argmin there measures the noise"; that
instrument was never turned on this question.

This is an uncertainty estimate over a committed artifact. It is NOT a bench,
no measurement was taken, and no number here prices anything.

Run:  python3 p1_what_the_instability_is_made_of.py
"""

import csv
import glob
import os
import random
import re
import statistics
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

# Bytes per element. A declared property of the arm, not a measurement:
# d16/d32/d64 are dense native carriers, packed carries 13 bits.
BYTES = {
    "bitpack-carrier-d16": 2.0,
    "bitpack-carrier-d16-control": 2.0,
    "bitpack-carrier-d32": 4.0,
    "bitpack-carrier-d64": 8.0,
    "bitpack-carrier-packed": 13.0 / 8.0,
    "bitpack-carrier-packed-simd": 13.0 / 8.0,
}

CONTROL = "bitpack-carrier-d16-control"

# The same three weightings `98` p10 instantiated, so the comparison is like for
# like. The numbers are scaffolding chosen to reach the check and are not
# proposals about what any strategy should weigh.
WEIGHTINGS = [
    ("speed-first", (1.0, 1.0 / 32, 1.0 / 32)),
    ("storage-first", (1.0 / 32, 1.0, 1.0 / 32)),
    ("tail-first", (1.0 / 32, 1.0 / 32, 1.0)),
]

RESAMPLES = 2000
SEED = 20260814


def samples():
    """Raw algo_ns per arm per region, read from the committed CSVs."""
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = {}
        with open(path) as fh:
            for row in csv.DictReader(fh):
                per.setdefault(row["variant"], []).append(float(row["algo_ns"]))
        out[n] = per
    return out


def cost_table(per_region, arms, rng=None):
    """(median time, bytes, IQR) per arm per region, optionally bootstrapped."""
    table = {}
    for n, per in per_region.items():
        row = {}
        for arm in arms:
            xs = per[arm]
            if rng is not None:
                xs = [xs[rng.randrange(len(xs))] for _ in range(len(xs))]
            xs = sorted(xs)
            q = statistics.quantiles(xs, n=4)
            row[arm] = (statistics.median(xs), BYTES[arm], q[2] - q[0])
        table[n] = row
    return table


def normalise(table, arms):
    """Min-max each coordinate over the whole table, as `98` p10 does."""
    regions = sorted(table)
    lo = [min(table[r][a][k] for r in regions for a in arms) for k in range(3)]
    hi = [max(table[r][a][k] for r in regions for a in arms) for k in range(3)]
    return {
        r: {
            a: tuple(
                (table[r][a][k] - lo[k]) / (hi[k] - lo[k]) if hi[k] > lo[k] else 0.0
                for k in range(3)
            )
            for a in arms
        }
        for r in regions
    }


def scores(nt, w, r, arms):
    return {a: sum(wi * ci for wi, ci in zip(w, nt[r][a])) for a in arms}


def argmin_section(table, arms, w):
    nt = normalise(table, arms)
    out = []
    for r in sorted(table):
        v = scores(nt, w, r, arms)
        out.append(min(arms, key=lambda a: (v[a], a)))
    return tuple(out)


def contending(table, arms, w, tol):
    """Per region, every arm within `tol` of the best, as a fraction of the
    region's achievable objective range. Origin-invariant, unlike a relative
    band on a min-max normalised level."""
    nt = normalise(table, arms)
    out = []
    for r in sorted(table):
        v = scores(nt, w, r, arms)
        best, worst = min(v.values()), max(v.values())
        span = worst - best
        cut = best + tol * span
        out.append(tuple(sorted(a for a in arms if v[a] <= cut)))
    return tuple(out)


def short(x):
    return x.replace("bitpack-carrier-", "")


def run(label, arms, per_region):
    print("=" * 78)
    print(f"ARM SET: {label}  ({len(arms)} arms)")
    print("=" * 78)
    rng = random.Random(SEED)
    base = cost_table(per_region, arms)
    tables = [cost_table(per_region, arms, rng) for _ in range(RESAMPLES)]
    regions = sorted(base)
    norms = [normalise(t, arms) for t in tables]

    for name, w in WEIGHTINGS:
        obs = argmin_section(base, arms, w)
        secs = Counter(argmin_section(t, arms, w) for t in tables)
        modal, freq = secs.most_common(1)[0]
        print(f"\n{name}")
        print(f"  committed section : {[short(a) for a in obs]}")
        print(f"  distinct sections : {len(secs)}")
        print(
            f"  modal appears     : {freq}/{RESAMPLES} "
            f"({100.0 * freq / RESAMPLES:.1f}%)"
            f"{'  == committed' if modal == obs else '  != committed'}"
        )

        print("  per region:  regret = (held - best) / (worst - best), 0 is best")
        allreg = []
        for i, r in enumerate(regions):
            dist = Counter(s[i] for s in secs.elements())
            flip = 100.0 * (1.0 - dist.most_common(1)[0][1] / RESAMPLES)
            regrets = []
            for nt in norms:
                v = scores(nt, w, r, arms)
                best, worst = min(v.values()), max(v.values())
                span = worst - best
                regrets.append((v[obs[i]] - best) / span if span > 0 else 0.0)
            allreg += regrets
            mean_reg = sum(regrets) / len(regrets)
            p95 = sorted(regrets)[int(0.95 * len(regrets))]
            movers = ", ".join(f"{short(a)}:{c}" for a, c in dist.most_common())
            print(
                f"    n={r:>8}  flip {flip:5.1f}%  "
                f"regret mean {mean_reg:.5f}  p95 {p95:.5f}   [{movers}]"
            )
        print(
            f"  regret of holding the committed section, over every region and "
            f"resample: mean {sum(allreg) / len(allreg):.5f}, max {max(allreg):.5f}"
        )

        for tol in (0.0, 0.02, 0.05, 0.10):
            csets = Counter(contending(t, arms, w, tol) for t in tables)
            cmod, cfreq = csets.most_common(1)[0]
            print(
                f"  contending set within {tol * 100:4.1f}% of the range: "
                f"{len(csets):4d} distinct, modal {cfreq}/{RESAMPLES} "
                f"({100.0 * cfreq / RESAMPLES:.1f}%)"
            )


def main():
    per_region = samples()
    arms_all = sorted(next(iter(per_region.values())))
    arms_nocontrol = [a for a in arms_all if a != CONTROL]

    print(__doc__.split("Run:")[0].strip())
    print()
    print(
        f"regions: {len(per_region)}   samples per arm per region: "
        f"{len(next(iter(next(iter(per_region.values())).values())))}"
    )
    print(f"resamples: {RESAMPLES}, seed {SEED}")
    print()

    run("all six, as 98 p10 used", arms_all, per_region)
    print()
    run("five, noise-floor control dropped", arms_nocontrol, per_region)

    print()
    print("=" * 78)
    print("READING")
    print("=" * 78)
    print(
        """
The distinct-section count is a product over regions. If each of six regions
holds its pick with probability p independently, the whole section reproduces
with probability p^6: p = 0.95 gives 0.74, p = 0.80 gives 0.26. So a 26.6%
modal rate is what roughly 80% per-region stability looks like, and the
statistic compounds the region count into what reads as a statement about the
table.

Regret says whether a flip matters. The control arm says whether it is even a
comparison between two things.
"""
    )


if __name__ == "__main__":
    main()
