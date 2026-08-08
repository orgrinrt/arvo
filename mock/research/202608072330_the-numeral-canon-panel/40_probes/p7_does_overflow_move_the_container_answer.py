#!/usr/bin/env python3
"""
p7. Does the overflow policy change which container a time objective prefers?

The register's Q5 asks whether the arithmetic column is one axis or two, and 25
section 4.2 argues two because the presets' cells answer different questions.
That is a claim about what the cells SAY. It leaves a different question open:

  independently STATEABLE   the axes are different questions, so a table needs a
                            value on each. 25 section 4.2 establishes this.

  independently RESOLVABLE  the best value on one axis does not depend on the
                            value taken on another, so a resolver may settle them
                            one at a time. Nobody has tested this.

If the containers in contention under wrapping differ from those in contention
under saturating, an unobservable axis's resolution depends on an observable
axis's value, and the two cannot be resolved independently however independently
they are stated.

mock/benches/ carries two matched pairs nobody in this panel has cited:

  warm-elementwise-width-l1     "... elementwise transform with no loop-carried
                                 value, declared-width sweep (8192 elements,
                                 4 ops/element, wrapping)"
  precise-elementwise-width-l1  "Container fork under saturating semantics,
                                 elementwise, declared-width sweep (8192
                                 elements, 4 ops/element)"

  warm-container-width-l1       "... declared-width sweep, cache-resident (8192
                                 elements, 3 ops/element, wrapping)"
  precise-container-width-l1    "Container fork under saturating semantics,
                                 declared-width sweep (8192 elements,
                                 3 ops/element)"

Same workload description, same arm names, same declared widths, one coordinate
changed. That is the controlled experiment and it is already committed.

THREE THINGS THIS PROBE HAS TO GET RIGHT, and the first two were found by
running it wrongly first.

  1. Absolute times across the two families are NOT comparable. Separate runs,
     separate thermal state. Only rankings within a run are, so this probe
     compares rankings and never magnitudes.

  2. Ties are noise. The first version compared strict argmins and reported
     disagreement at widths where three arms sat within 1 ns of each other.
     A strict argmin over tied arms measures the noise. So the comparison is
     between the SET of arms in contention, at a stated tolerance, reported at
     three tolerances so its sensitivity is visible.

  3. A collapsed arm is not a fast arm. If one arm's median is orders below the
     rest of its own family at the same width, the likely cause is that the
     compiler removed the work, not that the kernel is that good. The probe
     flags any arm more than DEAD_RATIO times faster than the second-fastest in
     its own run, reports it, and recomputes without it. It does not silently
     drop anything.

This is an ANALYSIS OF COMMITTED HARNESS OUTPUT. It is not a bench and it prices
nothing.
"""

import csv
import glob
import os
import re
import statistics
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.abspath(os.path.join(HERE, "..", "..", "..", "benches"))

PAIRS = [
    ("elementwise, 4 ops/element",
     "warm-elementwise-width-l1", "precise-elementwise-width-l1"),
    ("container fork, 3 ops/element",
     "warm-container-width-l1", "precise-container-width-l1"),
]

# An arm this many times faster than the second-fastest in its own run is
# flagged rather than believed. Chosen well above any plausible kernel gap on
# one host for one workload, so a flag is a strong signal rather than a
# borderline call.
DEAD_RATIO = 20.0

TOLERANCES = (0.02, 0.05, 0.10)


def load(family):
    """{declared_width: {arm: median warm-mode e2e_ns}}"""
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, f"{family}_n*.csv"))):
        size = re.search(r"_n(\d+)\.csv$", path).group(1)
        # size = <declared width><family id, 2 digits><op count, 2 digits>
        width = int(size[:-4])
        per = {}
        with open(path, newline="") as fh:
            for row in csv.DictReader(fh):
                if row["mode"] != "warm":
                    continue
                per.setdefault(row["variant"], []).append(float(row["e2e_ns"]))
        out[width] = {k: statistics.median(v) for k, v in per.items()}
    return out


def stated(family, size):
    path = os.path.join(BENCH, f"{family}_n{size}_findings.md")
    if not os.path.exists(path):
        return {}
    got, section = {}, None
    with open(path) as fh:
        for line in fh:
            if line.startswith("## "):
                section = line.strip()
                continue
            if section != "## End-to-end (all cooldowns combined)":
                continue
            cells = [c.strip() for c in line.split("|")]
            if len(cells) < 5:
                continue
            m = re.match(r"^(\d+)ns$", cells[3])
            if m and cells[1].startswith(("warm-", "precise-", "bitpack-")):
                got[cells[1]] = float(m.group(1))
    return got


def self_check():
    print("SELF-CHECK: recomputed medians against the committed findings files")
    checked = bad = 0
    for _, a, b in PAIRS:
        for family in (a, b):
            for path in sorted(glob.glob(os.path.join(BENCH, f"{family}_n*.csv"))):
                size = re.search(r"_n(\d+)\.csv$", path).group(1)
                want = stated(family, size)
                per = {}
                with open(path, newline="") as fh:
                    for row in csv.DictReader(fh):
                        if row["mode"] != "warm":
                            continue
                        per.setdefault(row["variant"], []).append(float(row["e2e_ns"]))
                for arm, samples in per.items():
                    if arm not in want:
                        continue
                    checked += 1
                    if abs(statistics.median(samples) - want[arm]) > 1.0:
                        bad += 1
                        print(f"  MISMATCH {family} n={size} {arm}")
    print(f"  medians compared: {checked}, mismatches: {bad}")
    return bad == 0


def flag_dead(medians):
    """Return the set of arms whose time is implausibly far below the field."""
    if len(medians) < 2:
        return set()
    order = sorted(medians, key=lambda a: medians[a])
    dead = set()
    for i, arm in enumerate(order[:-1]):
        rest = order[i + 1:]
        second = medians[rest[0]]
        if medians[arm] > 0 and second / medians[arm] >= DEAD_RATIO:
            dead.add(arm)
        else:
            break
    return dead


def contention(medians, tol):
    """Arms within tol of the fastest, as a set."""
    best = min(medians.values())
    return frozenset(a for a in medians if medians[a] <= best * (1.0 + tol))


def short(a):
    return a.replace("warm-container-", "")


def main():
    if not self_check():
        print("  parse disagrees with the committed findings; stopping.")
        return 1
    print()

    for label, wrap_fam, sat_fam in PAIRS:
        print("=" * 78)
        print(f"PAIR: {label}")
        print(f"  wrapping:   {wrap_fam}")
        print(f"  saturating: {sat_fam}")
        w, s = load(wrap_fam), load(sat_fam)
        arms = set.intersection(*[set(d) for d in list(w.values()) + list(s.values())])
        widths = sorted(set(w) & set(s))
        print(f"  arms in every run of both: {len(arms)} {sorted(short(a) for a in arms)}")
        print(f"  declared widths in both:   {widths}")
        print()

        # Step one: flag arms that look eliminated rather than fast.
        flagged = {}
        for width in widths:
            for pol, d in (("wrap", w), ("saturate", s)):
                m = {a: d[width][a] for a in arms}
                dead = flag_dead(m)
                if dead:
                    order = sorted(m, key=lambda a: m[a])
                    for a in dead:
                        flagged.setdefault(a, []).append(
                            (pol, width, m[a], m[order[len(dead)]]))
        if flagged:
            print("  ARMS FLAGGED AS ELIMINATED RATHER THAN FAST")
            for arm, rows in sorted(flagged.items()):
                print(f"    {short(arm)}: flagged in {len(rows)} of {2*len(widths)} runs")
                for pol, width, t, nxt in rows:
                    print(f"      {pol:<9} width {width:<3} {t:>10.0f} ns against "
                          f"{nxt:>10.0f} ns for the next arm  ({nxt/t:.0f}x)")
            print("    A ratio this size within one run at one width is not a kernel")
            print("    difference. The digest column of every CSV in this directory is")
            print("    0 for every arm, so nothing in the harness output cross-checks")
            print("    that an arm computed anything. Reported, then excluded below.")
            print()
        else:
            print("  no arm flagged as eliminated.")
            print()

        usable = arms - set(flagged)
        if usable != arms:
            print(f"  proceeding with {len(usable)} arms: {sorted(short(a) for a in usable)}")
            print()

        for tol in TOLERANCES:
            agree = 0
            rows = []
            for width in widths:
                wm = {a: w[width][a] for a in usable}
                sm = {a: s[width][a] for a in usable}
                cw, cs = contention(wm, tol), contention(sm, tol)
                same = cw == cs
                agree += same
                rows.append((width, cw, cs, same))
            print(f"  ARMS IN CONTENTION, tolerance {tol:.0%} of the fastest")
            print("  {:>6}  {:<34}  {:<34}  {:>6}".format(
                "width", "under wrapping", "under saturating", "same?"))
            for width, cw, cs, same in rows:
                print("  {:>6}  {:<34}  {:<34}  {:>6}".format(
                    width,
                    ", ".join(sorted(short(a) for a in cw)),
                    ", ".join(sorted(short(a) for a in cs)),
                    "yes" if same else "NO"))
            print(f"  widths where the contending set is identical: {agree} of {len(widths)}")
            print()

        print("  full rankings, fastest first, flagged arms included and marked,")
        print("  so every judgement above is inspectable:")
        for width in widths:
            wm = {a: w[width][a] for a in arms}
            sm = {a: s[width][a] for a in arms}
            mark = lambda a: short(a) + ("*" if a in flagged else "")
            print(f"    width {width}")
            print("      wrap:     " + " > ".join(
                f"{mark(a)}({wm[a]:.0f})" for a in sorted(arms, key=lambda a: wm[a])))
            print("      saturate: " + " > ".join(
                f"{mark(a)}({sm[a]:.0f})" for a in sorted(arms, key=lambda a: sm[a])))
        print()
    return 0


if __name__ == "__main__":
    sys.exit(main())
