#!/usr/bin/env python3
"""p4. How wide the compile-time differential's tolerance has to be, and what
normalising the cost coordinates does to the defects that can occur.

`p3` puts the check between a generated winner table and the weighting that
generated it into a compile-time assertion, and shows it refuses a generator bug
at build time for no runtime cost. Stated as EQUALITY, that assertion has a
suspected defect: at a region where two arms sit a hair apart, the argmin moves
under the noise of the run, so regenerating the table from a fresh measurement
changes the committed winner and the assertion fires on a design nobody changed.

This probe measures whether that suspicion is true, over every combination of the
three weightings and the two arm sets `p1` distinguishes, rather than over the one
configuration that happens to be stable. The first version of this file ran
speed-first on the five-arm set only, got a 0% false alarm rate, and would have
reported the suspicion refuted on a setup that could not have shown it. That is
`the-test-gate.md`'s "setup that helps" and the fix is the whole matrix.

FOUR THINGS MEASURED.

  1. THE INSTRUMENT'S OWN RESOLUTION. `bitpack-carrier-d16-control` is
     byte-identical to `bitpack-carrier-d16` by construction, and its doc comment
     says "any measured gap between them is the harness's own resolution on this
     workload" (`variants/bitpack-carrier-d16-control/src/lib.rs:1-8`). The gap
     between those two medians is therefore a MEASURED calibration of how much
     apparent difference means nothing, sitting in the committed CSVs of every
     run. It is the natural floor for the differential's band.

  2. FALSE ALARM RATE over 2000 bootstrap resamples, standing in for somebody
     rerunning the bench and regenerating, at six band widths, over all six
     weighting-by-arm-set combinations.

  3. DETECTION: whether the same bands still refuse `p2`'s generator defects.

  4. NORMALISATION. `98`'s p10 and this panel's `p1` both min-max normalise each
     coordinate before weighting. Min-max normalisation is SCALE INVARIANT, so a
     generator that reads a coordinate in the wrong unit emits the identical
     table under it and a different one without it. Both sides are run here
     rather than one being picked, because the difference is a representation
     choice nobody has named and it changes which defects can even occur.

Not a bench. No measurement was taken; this reads committed harness output and
resamples it.

Run:  python3 p4_the_differential_wants_a_band.py
"""

import csv
import glob
import os
import random
import re
import statistics

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

BYTES = {
    "bitpack-carrier-d16": 2.0,
    "bitpack-carrier-d16-control": 2.0,
    "bitpack-carrier-d32": 4.0,
    "bitpack-carrier-d64": 8.0,
    "bitpack-carrier-packed": 13.0 / 8.0,
    "bitpack-carrier-packed-simd": 13.0 / 8.0,
}
CONTROL = "bitpack-carrier-d16-control"
PAIR = ("bitpack-carrier-d16", CONTROL)

WEIGHTINGS = [
    ("speed-first", (1.0, 1.0 / 32, 1.0 / 32)),
    ("storage-first", (1.0 / 32, 1.0, 1.0 / 32)),
    ("tail-first", (1.0 / 32, 1.0 / 32, 1.0)),
]
RESAMPLES = 2000
SEED = 20260814
BANDS = (0.0, 0.005, 0.01, 0.02, 0.05, 0.10)


def samples():
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = {}
        with open(path) as fh:
            for row in csv.DictReader(fh):
                per.setdefault(row["variant"], []).append(float(row["algo_ns"]))
        out[n] = per
    return out


def table(per_region, arms, rng=None):
    t = {}
    for n, per in per_region.items():
        row = {}
        for a in arms:
            xs = per[a]
            if rng is not None:
                xs = [xs[rng.randrange(len(xs))] for _ in range(len(xs))]
            xs = sorted(xs)
            q = statistics.quantiles(xs, n=4)
            row[a] = (statistics.median(xs), BYTES[a], q[2] - q[0])
        t[n] = row
    return t


def prep(t, arms, normalised):
    """Min-max normalised, or raw. The whole point of carrying both."""
    if not normalised:
        return t
    rs = sorted(t)
    lo = [min(t[r][a][k] for r in rs for a in arms) for k in range(3)]
    hi = [max(t[r][a][k] for r in rs for a in arms) for k in range(3)]
    return {
        r: {
            a: tuple(
                (t[r][a][k] - lo[k]) / (hi[k] - lo[k]) if hi[k] > lo[k] else 0.0
                for k in range(3)
            )
            for a in arms
        }
        for r in rs
    }


def section(t, arms, w, normalised):
    nt = prep(t, arms, normalised)
    out = {}
    for r in sorted(t):
        v = {a: sum(wi * ci for wi, ci in zip(w, nt[r][a])) for a in arms}
        out[r] = min(arms, key=lambda a: (v[a], a))
    return out


def fires(t, arms, w, winner, band, normalised):
    nt = prep(t, arms, normalised)
    for r in sorted(t):
        v = {a: sum(wi * ci for wi, ci in zip(w, nt[r][a])) for a in arms}
        best, worst = min(v.values()), max(v.values())
        if v[winner[r]] > best + band * (worst - best):
            return True
    return False


def short(a):
    return a.replace("bitpack-carrier-", "")


def main():
    print(__doc__.split("Run:")[0].strip())
    print()
    per = samples()
    regions = sorted(per)
    all_arms = sorted(BYTES)
    five = [a for a in all_arms if a != CONTROL]

    print("=" * 78)
    print("1. THE INSTRUMENT'S OWN RESOLUTION, from the noise-floor control pair")
    print("=" * 78)
    gaps = []
    print(f"  {'region':>9} {'d16 median':>13} {'control median':>15} {'gap':>10}")
    for r in regions:
        a = statistics.median(per[r][PAIR[0]])
        b = statistics.median(per[r][PAIR[1]])
        g = abs(a - b) / min(a, b)
        gaps.append(g)
        print(f"  {r:>9} {a:>13.1f} {b:>15.1f} {100.0 * g:>9.3f}%")
    print(
        f"\n  apparent difference between two byte-identical arms: "
        f"median {100.0 * statistics.median(gaps):.3f}%, "
        f"max {100.0 * max(gaps):.3f}%"
    )

    print()
    print("=" * 78)
    print("2. FALSE ALARM RATE, all six weighting-by-arm-set combinations")
    print("=" * 78)
    print("  min-max normalised, which is what `98` p10 and `p1` both do")
    for setname, arms in (("six arms", all_arms), ("five, control dropped", five)):
        rng = random.Random(SEED)
        base = table(per, arms)
        tabs = [table(per, arms, rng) for _ in range(RESAMPLES)]
        print(f"\n  arm set: {setname}")
        hdr = "  ".join(f"{b * 100:5.1f}%" for b in BANDS)
        print(f"    {'weighting':<15} {hdr}")
        for name, w in WEIGHTINGS:
            winner = section(base, arms, w, True)
            row = []
            for band in BANDS:
                n = sum(1 for t in tabs if fires(t, arms, w, winner, band, True))
                row.append(f"{100.0 * n / RESAMPLES:5.1f}%")
            print(f"    {name:<15} {'  '.join(row)}")

    print()
    print("=" * 78)
    print("3. DETECTION, and 4. WHAT NORMALISATION DOES TO IT")
    print("=" * 78)

    def gen_unit(t):
        return {r: {a: (c[0], c[1] * 1000, c[2]) for a, c in t[r].items()} for r in t}

    def gen_swap(t):
        return {r: {a: (c[1], c[0], c[2]) for a, c in t[r].items()} for r in t}

    def gen_off(t):
        rs = sorted(t)
        return {rs[i]: t[rs[(i + 1) % len(rs)]] for i in range(len(rs))}

    defects = [("G1 unit", gen_unit), ("G2 swap", gen_swap), ("G4 offbyone", gen_off)]
    arms = five
    base = table(per, arms)
    for normalised in (True, False):
        label = "min-max normalised" if normalised else "raw coordinates"
        print(f"\n  {label}")
        for name, w in WEIGHTINGS:
            correct = section(base, arms, w, normalised)
            for dname, g in defects:
                bad = section(g(base), arms, w, normalised)
                if all(bad[r] == correct[r] for r in regions):
                    verdict = "emits the CORRECT table, so nothing can catch it"
                else:
                    caught = [b for b in BANDS if fires(base, arms, w, bad, b, normalised)]
                    verdict = (
                        f"refused up to band {max(caught) * 100:.1f}%"
                        if caught
                        else "NOT REFUSED at any band"
                    )
                print(f"    {name:<15} {dname:<13} {verdict}")

    print()
    print("=" * 78)
    print("READING")
    print("=" * 78)
    print(
        """
Section 2's band 0.0% column is the equality form `p3` compiles, and its number
is the probability that regenerating the table from a rerun refuses a design
nobody changed. Read it per row: the configurations `p1` shows are stable have
nothing to absorb, and the ones it shows are not are where a band earns its keep.

Section 4 is a correction to this probe's own framing, kept because the mistake
is instructive. Min-max normalisation is scale invariant by construction, so a
generator reading a coordinate in the wrong unit produces the identical
normalised table, the identical section and the identical shipped code. The first
version of this reading called that "invisible to every instrument", which is
wrong in the way that matters: the defect is NEUTRALISED, not concealed. There is
nothing downstream that differs, and a check that cannot see a difference which
does not exist has lost nothing.

What normalisation does cost is elsewhere, and `p4b` measures it: the weights
read normalised units whose scale depends on which arms are present, so the
section stops being independent of arms nobody selects.

The detection rows above stand as written for the defects that do change the
answer, G2 and G4, and those are what the band has to keep refusing.
"""
    )


if __name__ == "__main__":
    main()
