#!/usr/bin/env python3
"""p4b. Under min-max normalisation, an arm no weighting can ever select still
changes what every weighting selects.

`98`'s p10 and this panel's `p1` both min-max normalise each cost coordinate over
the whole table before applying the weighting, so that coordinates measured in
nanoseconds and in bytes can be added. That is the obvious move and it has a
consequence nobody in the unit has named.

THE CLAIM UNDER TEST. Min-max normalisation reads its scale off the arm set. Add
an arm and the range of a coordinate can widen; widen it and every other arm's
normalised value on that coordinate shrinks; shrink it and the effective weight
on that coordinate falls. So the argmin at a region can move because of an arm
that is not, and could never be, the argmin anywhere.

Under raw coordinates that cannot happen: an argmin is unaffected by an
alternative that is never the minimum. Under normalisation it can, and this probe
measures whether it does on the committed carrier table.

WHY THIS IS NOT HYPOTHETICAL. `97` section 10 establishes that the committed
carrier family contains arms Pareto-dominated in EVERY region, so no weighting
can select them anywhere; on the three-coordinate model used here that set is
`{bitpack-carrier-d64}` and the probe recomputes it rather than importing it.
`98` p10 normalises over a table containing it, and so does `p1`. If this effect
is real, every number those files report depends on the presence of an arm whose
entire role is to be unselectable.

FOUR EXPERIMENTS.

  A. Drop every arm dominated in every region, computed from the model in use
     rather than taken from `97`'s two-coordinate figure, and see whether the
     section over the remaining arms moves.
  B. Add a synthetic arm that is strictly dominated at every region and extreme
     on one coordinate, the shape of a bench arm added as a negative control, and
     see whether the section moves.
  C. The same two, under raw coordinates, where the prediction is that nothing
     moves and a move would refute the reasoning above rather than confirm it,
     and under a normalisation whose range is FROZEN as declared constants rather
     than read off the arm set, which is the remedy this probe proposes and
     therefore has to test rather than assert.
  D. How far the effect can be pushed: sweep the synthetic arm's extremity and
     count how many regions change their pick.

Not a bench. Reads committed harness output. No measurement taken.

Run:  python3 p4b_normalisation_breaks_independence.py
"""

import csv
import glob
import os
import re
import statistics

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

BYTES = {
    "bitpack-carrier-d16": 2.0,
    "bitpack-carrier-d32": 4.0,
    "bitpack-carrier-d64": 8.0,
    "bitpack-carrier-packed": 13.0 / 8.0,
    "bitpack-carrier-packed-simd": 13.0 / 8.0,
}
# NOT hardcoded. `97` section 10 reports `d64` and `packed` dominated in every
# region on its TWO-coordinate model; on the three-coordinate model used here
# only `d64` is, and the first version of this probe dropped both, removed an arm
# that is genuinely selected, and made its own raw-coordinate control fire. The
# dominated set is computed from the model actually in use.

WEIGHTINGS = [
    ("speed-first", (1.0, 1.0 / 32, 1.0 / 32)),
    ("storage-first", (1.0 / 32, 1.0, 1.0 / 32)),
    ("tail-first", (1.0 / 32, 1.0 / 32, 1.0)),
]


def base_table():
    t = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = {}
        with open(path) as fh:
            for row in csv.DictReader(fh):
                per.setdefault(row["variant"], []).append(float(row["algo_ns"]))
        row = {}
        for a in BYTES:
            xs = sorted(per[a])
            q = statistics.quantiles(xs, n=4)
            row[a] = (statistics.median(xs), BYTES[a], q[2] - q[0])
        t[n] = row
    return t


# A frozen normalisation range, declared once rather than read off whichever arms
# happen to be in the table. The numbers are the ranges the full five-arm table
# happens to span, rounded outward, and they are scaffolding: what matters is
# that they are CONSTANTS rather than a function of the arm set.
FROZEN = ((0.0, 1_000_000.0), (0.0, 16.0), (0.0, 100_000.0))


def prep(t, arms, normalised, frozen=False):
    if not normalised:
        return t
    rs = sorted(t)
    if frozen:
        lo = [FROZEN[k][0] for k in range(3)]
        hi = [FROZEN[k][1] for k in range(3)]
    else:
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


def section(t, arms, w, normalised, frozen=False):
    nt = prep(t, arms, normalised, frozen)
    out = []
    for r in sorted(t):
        v = {a: sum(wi * ci for wi, ci in zip(w, nt[r][a])) for a in arms}
        out.append(min(arms, key=lambda a: (v[a], a)))
    return tuple(out)


def dominated_everywhere(t, arms, a):
    for r in sorted(t):
        ca = t[r][a]
        if not any(
            all(x <= y for x, y in zip(t[r][b], ca))
            and any(x < y for x, y in zip(t[r][b], ca))
            for b in arms
            if b != a
        ):
            return False
    return True


def with_synthetic(t, arms, factor):
    """A synthetic arm strictly worse than every real arm on every coordinate at
    every region, so no weighting can select it, and `factor` times as extreme on
    the time coordinate so it stretches that coordinate's range."""
    t2 = {r: dict(t[r]) for r in t}
    for r in t:
        worst = [max(t[r][a][k] for a in arms) for k in range(3)]
        t2[r]["synthetic-control"] = (
            worst[0] * factor,
            worst[1] + 1.0,
            worst[2] + 1.0,
        )
    return t2, arms + ["synthetic-control"]


def short(s):
    return [x.replace("bitpack-carrier-", "").replace("synthetic-", "") for x in s]


def main():
    print(__doc__.split("Run:")[0].strip())
    print()
    t = base_table()
    arms = sorted(BYTES)
    regions = sorted(t)
    print(f"regions {len(regions)}, arms {len(arms)}, coordinates 3")
    print("(median algo_ns, declared bytes per element, IQR of the samples)")
    print()
    print("  arms dominated in every region, recomputed here rather than taken "
          "from `97`:")
    for a in arms:
        if dominated_everywhere(t, arms, a):
            print(f"    {a}")
    print()

    dominated = [a for a in arms if dominated_everywhere(t, arms, a)]
    kept = [a for a in arms if a not in dominated]

    for normalised, frozen, label in (
        (True, False, "MIN-MAX NORMALISED, range read off the arm set"),
        (False, False, "RAW COORDINATES"),
        (True, True, "NORMALISED, range FROZEN as declared constants"),
    ):
        print("=" * 78)
        print(label)
        print("=" * 78)
        for name, w in WEIGHTINGS:
            full = section(t, arms, w, normalised, frozen)
            # A. drop every arm dominated in every region
            dropped = section(t, kept, w, normalised, frozen)
            # B. add a synthetic arm nothing can select
            t2, arms2 = with_synthetic(t, arms, 4.0)
            added = section(t2, arms2, w, normalised, frozen)
            print(f"\n  {name}")
            print(f"    all {len(arms)} arms                : {short(full)}")
            print(
                f"    A. {len(dominated)} dominated arm(s) dropped : {short(dropped)}"
                f"   {'SAME' if dropped == full else 'MOVED'}"
            )
            print(
                f"    B. unselectable arm added    : {short(added)}"
                f"   {'SAME' if added == full else 'MOVED'}"
            )
            assert "synthetic-control" not in added, (
                "the synthetic arm was selected, so it is not unselectable and "
                "this experiment proves nothing"
            )

    print()
    print("=" * 78)
    print("D. HOW FAR IT GOES: sweeping the unselectable arm's extremity")
    print("=" * 78)
    print("  min-max normalised. The added arm is never selected at any factor.")
    print(f"  {'factor':>8}  {'regions whose pick moved, per weighting'}")
    for factor in (1.01, 1.5, 2.0, 4.0, 8.0, 32.0, 128.0):
        t2, arms2 = with_synthetic(t, arms, factor)
        cells = []
        for name, w in WEIGHTINGS:
            full = section(t, arms, w, True)
            added = section(t2, arms2, w, True)
            assert "synthetic-control" not in added
            moved = sum(1 for x, y in zip(full, added) if x != y)
            cells.append(f"{name} {moved}/{len(regions)}")
        print(f"  {factor:>8.2f}  {'   '.join(cells)}")

    print()
    print("=" * 78)
    print("READING")
    print("=" * 78)
    print(
        """
The raw and normalised sections are not comparable to each other: the same weight
numbers mean different things applied to nanoseconds and to normalised units, so
the raw half is a control for the MOVED/SAME column and for nothing else.

Whatever the rows above say, the raw-coordinate half is the control and it must
show nothing moving. An argmin cannot be changed by an alternative that is never
the minimum, so a MOVED there would mean this probe is broken rather than that
something was found.

If the normalised half moves where the raw half does not, then the effect is
exactly what it looks like: a weighting applied to normalised coordinates is not
a weighting over the arms, it is a weighting over the arms AND whatever else
happens to be in the table. Two consequences follow and neither is about noise.

A design shipping normalised costs has to say what the normalisation range is,
because it is part of the semantics rather than a presentation detail. Freezing
it as declared constants restores independence; reading it off the arm set does
not.

And a bench arm added as a negative control, which is measurement hygiene and
exactly what `bitpack-carrier-d16-control` is for, would then be able to change
what every strategy selects. That is a coupling between the instrument and the
answer, and it is worth knowing about before anything is built on a normalised
cost table.
"""
    )


if __name__ == "__main__":
    main()
