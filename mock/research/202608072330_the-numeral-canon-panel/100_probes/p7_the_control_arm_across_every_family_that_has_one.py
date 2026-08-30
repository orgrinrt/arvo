#!/usr/bin/env python3
"""p7. Does the noise-floor control explain the section instability in every
committed family that has one, or only in the one `98` happened to measure.

`p1` establishes, on `bitpack-carrier-width`, that every flip `98`'s F-98-10
reports at speed-first is between `bitpack-carrier-d16` and its own
byte-identical noise-floor control, and that dropping the control takes the
section from 31 distinct across 2000 resamples to 1. That is one family, and
`RULES.md` puts the bar at three independent instances.

Three more committed families declare a noise-floor control arm, each in its own
words:

  bitpack-contend-d16-control  "The noise floor: byte-identical to
                                `bitpack-contend-d16`. Same kernel, same region,
                                same arguments, same thread count."
  bitpack-wide-d16-control     "The noise floor: byte-identical to
                                `bitpack-wide-d16`."
  bitpack-carrier-d16-control  the one `p1` uses.

They appear across four committed bench families: `bitpack-carrier-width`,
`bitpack-contention`, `bitpack-contend-decode` and `bitpack-wide`.

THE MEASUREMENT is deliberately narrower than `p1`'s so that nothing depends on
a footprint declaration for arms this probe has not studied. It uses ONE cost
coordinate, the median `algo_ns`, which is the pure speed-first case and the
exact case `98` reports thirty distinct sections for. With one coordinate the
weighting drops out entirely: the section is just the per-region fastest arm, so
any instability is measurement noise and nothing else.

For each family: how many distinct sections across 2000 bootstrap resamples with
the control arm present, how many with it dropped, and which arms trade places.

A family where the controlled arm is never the fastest anywhere is a family where
dropping the control changes nothing, and that outcome is reported rather than
treated as a failure: the effect can only bite where the controlled arm is
competitive.

Not a bench. Reads committed harness output and resamples it. No measurement was
taken.

Run:  python3 p7_the_control_arm_across_every_family_that_has_one.py
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

RESAMPLES = 2000
SEED = 20260814


def families():
    """Every committed family whose rows include an arm named *-control."""
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "*.csv"))):
        base = os.path.basename(path)
        m = re.match(r"(.+)_n(\d+)\.csv$", base)
        if not m:
            continue
        fam, n = m.group(1), int(m.group(2))
        per = {}
        with open(path) as fh:
            for row in csv.DictReader(fh):
                v = row.get("variant", "")
                a = row.get("algo_ns", "")
                if not v or not a:
                    continue
                try:
                    per.setdefault(v, []).append(float(a))
                except ValueError:
                    continue
        if not any("control" in v for v in per):
            continue
        out.setdefault(fam, {})[n] = per
    # keep only families where every region carries the same arm set
    keep = {}
    for fam, regions in out.items():
        armsets = {tuple(sorted(p)) for p in regions.values()}
        if len(armsets) == 1 and len(regions) >= 2:
            keep[fam] = regions
    return keep


def section(regions, arms, rng=None):
    """Per-region fastest arm by median algo_ns. One coordinate, so no weighting."""
    out = []
    for n in sorted(regions):
        per = regions[n]
        best, bv = None, None
        for a in arms:
            xs = per[a]
            if rng is not None:
                xs = [xs[rng.randrange(len(xs))] for _ in range(len(xs))]
            v = statistics.median(xs)
            if bv is None or v < bv or (v == bv and a < best):
                bv, best = v, a
        out.append(best)
    return tuple(out)


def short(fam, a):
    return a.replace(fam.rsplit("-", 1)[0] + "-", "").replace("bitpack-", "")


def regret(regions, arms, held, rng):
    """With one raw coordinate the regret is a clean time ratio: how much slower
    the held arm is than the per-resample fastest, as a fraction of the fastest.
    No normalisation, so no arbitrary origin and no meaningless denominator."""
    out = []
    for _ in range(RESAMPLES):
        tot = []
        for i, n in enumerate(sorted(regions)):
            per = regions[n]
            vals = {}
            for a in arms:
                xs = per[a]
                xs = [xs[rng.randrange(len(xs))] for _ in range(len(xs))]
                vals[a] = statistics.median(xs)
            best = min(vals.values())
            tot.append((vals[held[i]] - best) / best if best > 0 else 0.0)
        out.append(tot)
    flat = [x for row in out for x in row]
    return sum(flat) / len(flat), max(flat)


def run(fam, regions):
    arms = sorted(next(iter(regions.values())))
    controls = [a for a in arms if "control" in a]
    nocontrol = [a for a in arms if "control" not in a]
    print(f"\n{fam}")
    print(f"  regions {len(regions)}, arms {len(arms)}, control arm(s) {controls}")
    print(f"  samples per arm per region: "
          f"{sorted({len(v) for p in regions.values() for v in p.values()})}")

    results = {}
    for label, aset in (("with control", arms), ("control dropped", nocontrol)):
        rng = random.Random(SEED)
        base = section(regions, aset)
        secs = Counter(section(regions, aset, rng) for _ in range(RESAMPLES))
        modal, freq = secs.most_common(1)[0]
        results[label] = (len(secs), freq, base, secs)
        print(
            f"    {label:<16} distinct {len(secs):>4}   "
            f"modal {freq}/{RESAMPLES} ({100.0 * freq / RESAMPLES:5.1f}%)   "
            f"{[short(fam, a) for a in base]}"
        )

    # what the instability costs, with the control dropped, which is the
    # configuration the finding is about
    rng = random.Random(SEED)
    held = results["control dropped"][2]
    mean_r, max_r = regret(regions, nocontrol, held, rng)
    print(
        f"    cost of holding the committed section, control dropped: "
        f"mean {100.0 * mean_r:.4f}% slower, worst {100.0 * max_r:.4f}%"
    )

    # which arms trade places, with the control present
    _, _, base, secs = results["with control"]
    involved = Counter()
    for i in range(len(base)):
        d = Counter(s[i] for s in secs.elements())
        if len(d) > 1:
            for a in d:
                involved[a] += 1
    if involved:
        print(
            "    arms that trade places: "
            + ", ".join(f"{short(fam, a)}({c} regions)" for a, c in involved.most_common())
        )
        ctrl_in = any("control" in a for a in involved)
        print(f"    a control arm is one of them: {ctrl_in}")
    else:
        print("    no region's pick moves even with the control present")
    return results


def main():
    print(__doc__.split("Run:")[0].strip())
    print()
    fams = families()
    print(f"committed families carrying a noise-floor control arm: {len(fams)}")
    print(f"resamples {RESAMPLES}, seed {SEED}, one cost coordinate (median algo_ns)")
    all_res = {}
    for fam in sorted(fams):
        all_res[fam] = run(fam, fams[fam])

    print()
    print("=" * 78)
    print("SUMMARY")
    print("=" * 78)
    print(f"  {'family':<26} {'distinct with':>14} {'distinct without':>17}")
    for fam in sorted(all_res):
        a = all_res[fam]["with control"][0]
        b = all_res[fam]["control dropped"][0]
        print(f"  {fam:<26} {a:>14} {b:>17}")
    print()
    print(
        """
The regret line is the one that says whether any of it matters. With a single raw
cost coordinate it is directly interpretable: it is how much slower the committed
section runs than the fastest arms that resample would have picked.

A family whose count falls when the control is dropped is one where the argmin
was choosing between two copies of one arm. A family whose count does not fall is
one where the controlled arm was not competitive, so the control never reached
the section and there was nothing for it to destabilise; that is a fact about
which arm the control shadows rather than a counterexample.

With one cost coordinate there is no weighting to blame: the section is the
per-region fastest arm, so every distinct section beyond the first is measurement
noise, and every flip involving a control arm is the harness's own declared
resolution deciding a bench result.
"""
    )


if __name__ == "__main__":
    main()
