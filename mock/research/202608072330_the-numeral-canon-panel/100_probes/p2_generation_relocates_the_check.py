#!/usr/bin/env python3
"""p2. Does generating the table from the weighting eliminate the check.

`98` section 3 proposes generating the region-to-arm table from a stated
weighting rather than writing it and checking it, and says of the result:
"rationalisability is TRUE BY CONSTRUCTION rather than a property to be
verified, so there is NOTHING TO CHECK and nothing to police".

The first clause is right and the second does not follow, and this probe is
built to establish the gap rather than to argue it.

THE CLAIM UNDER TEST. Generating replaces a human writing a table with a tool
computing one. It removes every defect a human writing a table can introduce and
it introduces a defect class a human cannot: the tool computes the wrong argmin.
A tool's wrong argmin is, in general, the RIGHT argmin of something else, so it
is still rationalisable, and rationalisability cannot see it. So the check does
not vanish; it relocates onto the generator, and the question is what detects it
there.

THE EXPERIMENT. Take a cost table, a stated weighting `w`, and the section a
correct generator emits. Then run four generators carrying defects that are
ordinary rather than exotic, each one a mistake somebody makes reading a CSV:

  G1  unit          one coordinate read in the wrong unit (scaled by 1000)
  G2  swap          two coordinates read in the wrong order
  G3  tiebreak      ties broken by the last arm rather than the first
  G4  offbyone      row r's costs attributed to region r+1, cyclically
  G5  dropped       one coordinate never read at all

and three detectors:

  D1  rationalisable  is the emitted section an argmin of SOME weighting
  D2  cone membership is the STATED `w` one of the weightings that explain it
  D3  recomputation   does an independent argmin under `w` agree, arm for arm

PREDICTIONS, WRITTEN BEFORE RUNNING, so a surprise is visible as one.

  D1 passes on G1 and G2, because a rescaled or permuted read is the exact
     argmin of the correspondingly rescaled or permuted weighting. It passes on
     G3, because a tie admits either arm weakly. On G4 it may go either way,
     since a permuted assignment need not be any weighting's argmin. G5 is the
     interesting one: a dropped coordinate is a ZERO WEIGHT, so D1 at `w >= 0`
     passes by construction and D1 at `w > 0` should catch it. That is the one
     place `98`'s strict-positivity rung has detection power against a
     generator, and it is worth separating from the guarantee that rung was
     proposed for.
  D2 catches G1, G2 and G4, and does NOT catch G3, because both tie-breaks are
     weak argmins under the stated `w`.
  D3 catches all four, and reports G3 as a defect when it is not one, because
     the two tie-break policies disagree about a choice the weighting does not
     make.

If those hold, the ordering is exact: D1 is nearly vacuous on generated output,
D3 is sound but not tie-break invariant, and D2 is the differential a generated
table actually wants. D2 is `97`'s decider, unchanged, in a different role: it
stops being a constraint on a human's table and becomes the acceptance test on a
tool's output.

Two models, because one instance decides nothing: the committed carrier table
converted to exact rationals, and 400 random models across three generators.

Exact rational arithmetic throughout. No measurement is taken and this is not a
bench.

Run:  python3 p2_generation_relocates_the_check.py
"""

import csv
import glob
import os
import random
import re
import statistics
from fractions import Fraction

import cone

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

BYTES = {
    "bitpack-carrier-d16": Fraction(2),
    "bitpack-carrier-d32": Fraction(4),
    "bitpack-carrier-d64": Fraction(8),
    "bitpack-carrier-packed": Fraction(13, 8),
    "bitpack-carrier-packed-simd": Fraction(13, 8),
}
D = 3


def carrier_table():
    """The committed carrier run as an exact rational cost table.

    The noise-floor control arm is dropped: `p1` establishes that it is
    byte-identical to `bitpack-carrier-d16` by the bench's own construction, so
    including it puts two copies of one arm in the model.
    """
    costs, regions = {}, []
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = {}
        with open(path) as fh:
            for row in csv.DictReader(fh):
                per.setdefault(row["variant"], []).append(float(row["algo_ns"]))
        row = {}
        for arm in BYTES:
            xs = sorted(per[arm])
            q = statistics.quantiles(xs, n=4)
            row[arm] = (
                Fraction(statistics.median(xs)).limit_denominator(10**6),
                BYTES[arm],
                Fraction(q[2] - q[0]).limit_denominator(10**6),
            )
        costs[n] = row
        regions.append(n)
    return costs, regions, sorted(BYTES)


def random_model(rng, nr=5, na=5):
    regions = list(range(nr))
    arms = list(range(na))
    costs = {
        r: {a: tuple(Fraction(rng.randrange(1, 400)) for _ in range(D)) for a in arms}
        for r in regions
    }
    return costs, regions, arms


def with_tie(rng, nr=5, na=5):
    """A model carrying a deliberate tie at one region, so G3 has something to
    do. Without this the tie-break defect is unreachable and the probe would
    report it clean for the wrong reason."""
    costs, regions, arms = random_model(rng, nr, na)
    r = regions[0]
    costs[r][arms[1]] = costs[r][arms[0]]
    return costs, regions, arms


# --------------------------------------------------------------------------
# The generators. G0 is correct; the rest carry one ordinary defect each.
# --------------------------------------------------------------------------


def g0(w, costs, regions, arms):
    return cone.argmin_section(w, costs, regions, arms, "first")


def g1_unit(w, costs, regions, arms):
    """Coordinate 1 read in the wrong unit."""
    bad = {
        r: {a: (c[0], c[1] * 1000, c[2]) for a, c in costs[r].items()} for r in regions
    }
    return cone.argmin_section(w, bad, regions, arms, "first")


def g2_swap(w, costs, regions, arms):
    """Coordinates 0 and 1 read in the wrong order."""
    bad = {r: {a: (c[1], c[0], c[2]) for a, c in costs[r].items()} for r in regions}
    return cone.argmin_section(w, bad, regions, arms, "first")


def g3_tiebreak(w, costs, regions, arms):
    """Ties broken by the last arm rather than the first."""
    return cone.argmin_section(w, costs, regions, arms, "last")


def g5_dropped(w, costs, regions, arms):
    """Coordinate 2 never read. Equivalent to a zero weight on it, so it is the
    one defect the strictly-positive rung can see."""
    w2 = (w[0], w[1], Fraction(0))
    return cone.argmin_section(w2, costs, regions, arms, "first")


def g4_offbyone(w, costs, regions, arms):
    """Row r's costs attributed to region r+1, cyclically."""
    sh = {regions[i]: costs[regions[(i + 1) % len(regions)]] for i in range(len(regions))}
    return cone.argmin_section(w, sh, regions, arms, "first")


GENERATORS = [
    ("G0 correct", g0),
    ("G1 unit", g1_unit),
    ("G2 swap", g2_swap),
    ("G3 tiebreak", g3_tiebreak),
    ("G4 offbyone", g4_offbyone),
    ("G5 dropped", g5_dropped),
]


def detect(w, section, costs, regions, arms, ref):
    d1 = cone.nonempty(section, costs, regions, arms, D, strict=False)
    d1s = cone.nonempty(section, costs, regions, arms, D, strict=True)
    d2 = cone.admits(w, section, costs, regions, arms)
    d3 = all(section[r] == ref[r] for r in regions)
    return d1, d1s, d2, d3


def report(title, w, costs, regions, arms):
    print(f"\n{title}")
    print(f"  stated weighting w = {[str(x) for x in w]}")
    ref = g0(w, costs, regions, arms)
    print(
        f"  {'generator':<14} {'D1 rat>=0':>9} {'D1 rat>0':>9} "
        f"{'D2 w in cone':>13} {'D3 recompute':>13}   verdict"
    )
    for name, gen in GENERATORS:
        sec = gen(w, costs, regions, arms)
        d1, d1s, d2, d3 = detect(w, sec, costs, regions, arms, ref)
        same = all(sec[r] == ref[r] for r in regions)
        verdict = "identical to correct" if same else "DIFFERS from correct"
        print(
            f"  {name:<14} {str(d1):>9} {str(d1s):>9} "
            f"{str(d2):>13} {str(d3):>13}   {verdict}"
        )


def sweep(rng, maker, n, label):
    """How often each detector fires, over many models."""
    tally = {name: [0, 0, 0, 0, 0] for name, _ in GENERATORS}
    for _ in range(n):
        costs, regions, arms = maker(rng)
        w = tuple(Fraction(rng.randrange(1, 60)) for _ in range(D))
        ref = g0(w, costs, regions, arms)
        for name, gen in GENERATORS:
            sec = gen(w, costs, regions, arms)
            differs = any(sec[r] != ref[r] for r in regions)
            d1, d1s, d2, d3 = detect(w, sec, costs, regions, arms, ref)
            t = tally[name]
            t[0] += 1 if differs else 0
            t[1] += 1 if (differs and not d1) else 0
            t[2] += 1 if (differs and not d1s) else 0
            t[3] += 1 if (differs and not d2) else 0
            t[4] += 1 if (differs and not d3) else 0
    print(f"\n{label}: {n} models")
    print(
        f"  {'generator':<14} {'differs':>8} "
        f"{'D1 caught':>10} {'D1s caught':>11} {'D2 caught':>10} {'D3 caught':>10}"
    )
    for name, _ in GENERATORS:
        d, c1, c1s, c2, c3 = tally[name]
        pct = lambda x: f"{x:>4} ({100.0 * x / d:5.1f}%)" if d else f"{x:>4}   (n/a)"
        print(
            f"  {name:<14} {d:>8} {pct(c1):>10} {pct(c1s):>11} "
            f"{pct(c2):>10} {pct(c3):>10}"
        )
    return tally


def main():
    print(__doc__.split("Run:")[0].strip())
    print()

    costs, regions, arms = carrier_table()
    print("=" * 78)
    print("PART 1. The committed carrier table, as exact rationals")
    print("=" * 78)
    print(f"regions {len(regions)}, arms {len(arms)}, coordinates {D}")
    print("cost coordinates: (median algo_ns, declared bytes per element, IQR)")
    for label, w in [
        ("speed-first", (Fraction(32), Fraction(1), Fraction(1))),
        ("storage-first", (Fraction(1), Fraction(32), Fraction(1))),
    ]:
        report(f"weighting: {label}", w, costs, regions, arms)

    print()
    print("=" * 78)
    print("PART 2. Random models, so no verdict rests on one table")
    print("=" * 78)
    rng = random.Random(20260814)
    sweep(rng, random_model, 200, "generic models, ties improbable")
    sweep(rng, with_tie, 200, "models carrying a deliberate tie at one region")

    print()
    print("=" * 78)
    print("READING")
    print("=" * 78)
    print(
        """
A detector "catches" a generator when the generator's section differs from the
correct one AND the detector reports it. The `differs` column is the number of
models where there was anything to catch.

What the numbers are expected to say, and the check on this probe is whether
they say it: rationalisability is close to blind on generated output, because a
generator's mistake produces the correct answer to a different question and a
different question still has a weighting. Cone membership of the STATED
weighting is the discriminating test, and it is invariant to how ties are
broken, which direct recomputation is not.

The consequence for `98` section 3 is narrow and it does not overturn the
proposal. Generating removes one class of defect and admits another, so the
check relocates onto the generator rather than disappearing. What it relocates
to is the decider `97` already built.
"""
    )


if __name__ == "__main__":
    main()
