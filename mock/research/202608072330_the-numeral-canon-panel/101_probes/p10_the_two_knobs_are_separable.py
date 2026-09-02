#!/usr/bin/env python3
"""Q44 asks whether a canon-level weighting must be strictly positive. The
options as registered assume the guarantee comes from positivity. It does not.

`98` measured that at a non-negative weighting 72 sections are rationalisable and
63 of them select an arm no weighting can select, while at a strictly positive
weighting 9 are and none does
(`98_probes/p6_reproduce_the_predecessors_count_and_rung_it.out`). `OPTIONS.md`
Q44 turns that into three options, and (a), requiring strict positivity, is
recorded as costing "the ability to express a strategy that genuinely does not
care about a coordinate".

Building `p4_what_a_coordinate_buys.py` turned up that the property being
measured has **two independent knobs**, and its first version welded them
together:

  WEIGHTS. May a weight be zero, or must every coordinate carry some weight?
  MINIMA.  Must the named arm be the UNIQUE argmin at each region, or merely one
           of the minima?

`97`'s 72 is the first knob open and the second open. Its 9 is the first closed.
This probe runs all four cells, and the question it answers is whether the second
knob alone buys the same guarantee, because if it does then Q44 has a fourth
option that costs nothing it names.

Block B asks the other half: at two coordinates, is the section a zero weight
selects reachable by a strictly positive weighting? If it is, the "cannot express
indifference" cost is a cost of expressing indifference EXACTLY rather than of
expressing it at all.

This reads committed artifacts. It is NOT a bench, no measurement was taken,
and no number here prices anything.

Run:  python3 p10_the_two_knobs_are_separable.py
"""

import csv
import glob
import itertools
import os
import re
import statistics
from collections import defaultdict
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

BITS = {
    "bitpack-carrier-d16": 16,
    "bitpack-carrier-d32": 32,
    "bitpack-carrier-d64": 64,
    "bitpack-carrier-packed": 13,
    "bitpack-carrier-packed-simd": 13,
}


def samples():
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = defaultdict(list)
        with open(path) as fh:
            for row in csv.DictReader(fh):
                per[row["variant"]].append(float(row["algo_ns"]))
        out[n] = dict(per)
    return out


def feasible2(constraints, weights_strict, cons_strict):
    """w = (x, 1-x). <w,g> >= 0, strictly when cons_strict; x in [0,1], open
    when weights_strict."""
    lo, hi = Fraction(0), Fraction(1)
    lo_open = hi_open = weights_strict
    for g in constraints:
        a, c = g[0] - g[1], g[1]
        if a == 0:
            if c < 0 or (cons_strict and c == 0):
                return False
            continue
        b = -Fraction(c, a)
        if a > 0:
            if b > lo:
                lo, lo_open = b, cons_strict
            elif b == lo:
                lo_open = lo_open or cons_strict
        else:
            if b < hi:
                hi, hi_open = b, cons_strict
            elif b == hi:
                hi_open = hi_open or cons_strict
    if lo > hi:
        return False
    if lo == hi and (lo_open or hi_open):
        return False
    return True


def dominated_everywhere(t, arms):
    """Arms no strictly positive weighting can select: at every region some
    other arm is no worse on every coordinate and better on one.

    WEAK domination is the right notion here and the first version of this probe
    used the strict one, which found only `bitpack-carrier-d64` where `97` and
    `98` name two arms. `bitpack-carrier-packed` carries the same 13 bits as
    `bitpack-carrier-packed-simd` and is slower, so it is dominated only if
    equality on a coordinate counts. It does: if b <= a everywhere and b < a
    somewhere then <w, a - b> > 0 for every w > 0, so a is never a strict argmin.
    That output is kept at `p10_first_version_used_strict_domination.out`, and
    with it the dominated-arm column read 0 in all four cells, which is the
    predecessors' 63 silently lost to a definition."""
    out = []
    for a in arms:
        for b in arms:
            if a == b:
                continue
            if all(
                all(t[n][b][i] <= t[n][a][i] for i in range(2))
                and any(t[n][b][i] < t[n][a][i] for i in range(2))
                for n in t
            ):
                out.append(a)
                break
    return out


def main():
    per = samples()
    arms = sorted(BITS)
    t = {
        n: {
            a: (
                Fraction(statistics.median(per[n][a])).limit_denominator(10**12),
                Fraction(BITS[a]),
            )
            for a in arms
        }
        for n in per
    }
    regions = sorted(t)
    dom = set(dominated_everywhere(t, arms))

    print("=" * 78)
    print("A. THE FOUR CELLS, ON `97`'S MODEL")
    print("=" * 78)
    print("  6 regions, 5 arms, 2 coordinates (median algo_ns, bits per element)")
    print(f"  arms dominated at every region: {sorted(dom)}")
    print()
    print(f"  {'weights':>10s} {'minima':>10s} {'sections':>9s}   selecting a dominated arm")
    for ws in (False, True):
        for cs in (False, True):
            n_ok = 0
            n_dom = 0
            for sec in itertools.product(arms, repeat=len(regions)):
                cons = []
                for r, a in zip(regions, sec):
                    for b in arms:
                        if b != a:
                            cons.append(tuple(t[r][b][i] - t[r][a][i] for i in range(2)))
                if feasible2(cons, ws, cs):
                    n_ok += 1
                    if dom & set(sec):
                        n_dom += 1
            print(
                f"  {'w > 0' if ws else 'w >= 0':>10s} {'unique' if cs else 'weak':>10s}"
                f" {n_ok:9d}   {n_dom}"
            )
    print()
    print("  `97` and `98` report 72 at the top-left cell and 9 at the bottom-left.")
    print("  Whether the OTHER knob alone reaches 9 is what Q44 does not ask.")

    print()
    print("=" * 78)
    print("B. IS A ZERO WEIGHT'S SECTION REACHABLE BY A STRICTLY POSITIVE ONE")
    print("=" * 78)
    print("  At two coordinates the weighting is one number r = w2/w1, so a zero")
    print("  weight on coordinate 2 is the limit r -> 0. If the section at r = 0")
    print("  equals the section at every sufficiently small positive r, then")
    print("  indifference is expressible to any tolerance without a zero weight.")
    print()

    def section(r, tie_to_second):
        out = []
        for n in regions:
            best, bestv = None, None
            for a in arms:
                v = t[n][a][0] + r * t[n][a][1]
                if bestv is None or v < bestv:
                    best, bestv = a, v
            out.append(best)
        return tuple(out)

    short = lambda sec: ",".join(a.split("-")[-1] for a in sec)

    zero = section(Fraction(0), False)
    print("  END ONE: zero weight on the SIZE coordinate, r -> 0.")
    for e in [Fraction(1, 10**k) for k in (1, 3, 6, 12)]:
        sec = section(e, False)
        print(f"    r = 1e-{len(str(e.denominator)) - 1:<3d} {'==' if sec == zero else '!='} the r = 0 section")
    print(f"    r = 0     {short(zero)}")
    print()
    print("  END TWO: zero weight on the TIME coordinate, r -> infinity. This is")
    print("  the end where the gap lives, because two arms carry 13 bits each.")
    big = [Fraction(10**k) for k in (1, 3, 6, 12)]
    limit = section(big[-1], False)
    for e in big:
        sec = section(e, False)
        print(f"    r = 1e{len(str(e.numerator)) - 1:<4d} {'==' if sec == limit else '!='} the large-r section   {short(sec)}")
    # The weak argmin SET at a pure-size weighting, which is what a zero weight
    # on time admits and what the limit does not.
    print()
    print("  weak argmin set at a pure-size weighting, per region:")
    for n in regions:
        m = min(t[n][a][1] for a in arms)
        tied = [a for a in arms if t[n][a][1] == m]
        print(f"    n = {n:<10d} {short(tied)}")
    print()
    print("  Ties are the whole content of the gap. A zero weight lets two arms tie")
    print("  on the coordinate that is still being weighed, and a tie is where a")
    print("  dominated arm gets in. A strictly positive r breaks the tie with the")
    print("  second coordinate rather than admitting both.")


if __name__ == "__main__":
    main()
