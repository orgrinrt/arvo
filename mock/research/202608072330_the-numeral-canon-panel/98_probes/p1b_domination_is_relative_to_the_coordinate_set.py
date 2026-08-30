#!/usr/bin/env python3
"""p1b. "Dominated in every region" is a claim about the coordinate set.

An arm Pareto-dominated at every region cannot be selected anywhere by any
selection rule monotone in the coordinates, so it is out of reach of every
strategy expressible as a preference over them. That is a strong statement and
it is worth knowing which arms it catches.

It is also entirely relative to which coordinates are in the model. This probe
recomputes the same structure over every non-empty subset of the three
coordinates in p1, and reports which arms are dominated everywhere under each.

If the set of everywhere-dominated arms shrinks when a coordinate is added, then
the earlier, smaller model was not reporting a fact about the arms. It was
reporting a fact about what it had measured. The finding then is not "drop the
arm" but "the arm wins on a coordinate the model does not carry".
"""

import itertools
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
COORDS = ("time", "bytes", "spread")


def load():
    with open(os.path.join(HERE, "p1_cost_table.json")) as f:
        raw = json.load(f)
    return {int(k): v for k, v in raw.items()}


def dominated(table, n, a, ks):
    ca = table[n][a]
    for b in table[n]:
        if b == a:
            continue
        cb = table[n][b]
        if all(cb[k] <= ca[k] for k in ks) and any(cb[k] < ca[k] for k in ks):
            return True
    return False


def main():
    table = load()
    regions = sorted(table)
    arms = sorted(table[regions[0]])

    print("Arms dominated in EVERY region, per coordinate subset.")
    print("An arm listed here is unreachable by any monotone selection rule")
    print("over those coordinates, at every region measured.\n")

    rows = []
    for r in range(1, len(COORDS) + 1):
        for ks in itertools.combinations(COORDS, r):
            everywhere = [
                a for a in arms
                if all(dominated(table, n, a, ks) for n in regions)
            ]
            fronts = [
                sum(1 for a in arms if not dominated(table, n, a, ks))
                for n in regions
            ]
            bound = 1
            for f in fronts:
                bound *= f
            rows.append((ks, everywhere, fronts, bound))
            print(f"  coords {str(list(ks)):34s} dominated everywhere: "
                  f"{everywhere if everywhere else ['none']}")
            print(f"  {'':41s} Pareto fronts per region: {fronts}, "
                  f"admissible sections <= {bound}")
    print()

    two = next(r for r in rows if r[0] == ("time", "bytes"))
    three = next(r for r in rows if r[0] == ("time", "bytes", "spread"))
    lost = [a for a in two[1] if a not in three[1]]
    print(f"under (time, bytes):          {two[1]}")
    print(f"under (time, bytes, spread):  {three[1]}")
    if lost:
        print(f"\nRESCUED by adding 'spread': {lost}")
        for a in lost:
            where = [n for n in regions if not dominated(table, n, a, COORDS)]
            print(f"  {a} is on the front at {len(where)} of {len(regions)} "
                  f"regions once spread is a coordinate: {where}")
        print("\nSo the everywhere-dominated verdict on these arms was a")
        print("property of the two-coordinate model, not of the arms.")
    else:
        print("\nNo arm changes verdict when 'spread' is added.")

    print("\nAnd the converse direction, which is the honest caveat on this")
    print("probe itself: adding a fourth coordinate could rescue the rest.")
    for a in three[1]:
        print(f"  {a} is dominated everywhere on all three coordinates.")
        print("    That is a licence to drop it only if the three are the whole")
        print("    of what any strategy weighs, which nothing has established.")


if __name__ == "__main__":
    main()
