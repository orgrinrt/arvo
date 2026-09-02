#!/usr/bin/env python3
"""p2. The ladder between "any table" and "one weighting", derived and counted.

The question this unit is on is what a strategy IS. Two answers are in play and
they are usually stated as alternatives: a strategy picks a mechanism per region
(a table), or a strategy is a weighting over measurements and the picks fall out
of minimising under it.

They are not alternatives. They are the two ends of a ladder, and the rungs
between them are what a canon can name. This probe defines the rungs, computes
each over the real cost table from p1, and exhibits a witness separating
neighbours wherever the counts allow.

The rungs, on a finite model with regions R, arms A, and a cost map
c : R x A -> Q^d, lower better on every coordinate. A SECTION is any
sigma : R -> A.

  L0  any section.                                       |A|^|R|
  L1  Pareto-admissible: sigma(r) is not strictly dominated at r, for all r.
  L2  order-rationalisable: one total preorder over cost vectors, monotone with
      respect to the componentwise order, makes sigma(r) a minimum among the
      arms available at r, simultaneously at every r.
  L3  linearly rationalisable, weights w >= 0 not all zero.
  L4  as L3 with every weight strictly positive: every coordinate counts.
  L5  as L4 and sigma(r) the UNIQUE minimiser at every r, so the choice is
      forced rather than merely permitted.

Why a ladder and not a binary. "A strategy is a weighting" is L4. "A strategy is
a table" is L0. Op's answer at `88` section 1 was "mostly option 1, but a little
bit of option 3 with it", which has no reading on a binary and an obvious one on
a ladder: a rung strictly between the ends.

What this probe does not do. It does not say which rung arvo should require. It
measures how far apart the rungs sit on one real table, so a proposal to require
one of them can be priced. Choosing is op's.

Method. Exact rational arithmetic throughout, via polygon clipping on the weight
simplex in cone.py, cross-checked against Fourier-Motzkin in lp.py on every
section of a sample. Counts are measurements.

Scaling. Each coordinate is multiplied by a positive rational that clears its
denominators, which keeps every number an integer. This changes nothing that is
measured here: replacing coordinate k by lambda_k * coordinate k for
lambda_k > 0 is the reparameterisation w_k -> w_k / lambda_k, a bijection of the
positive orthant onto itself and of the non-negative orthant onto itself. Every
rung above is therefore invariant under it, and Pareto order is invariant too.

Enumeration. Each rung is counted by depth-first search over regions with
pruning, sound because every rung's condition is monotone in the set of regions
committed so far: a prefix that already fails cannot be rescued later. L1 is
enumerated exhaustively as a control on the pruned counts.
"""

import itertools
import json
import os
import random
import sys
from fractions import Fraction
from math import gcd

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import cone  # noqa: E402
from lp import feasible  # noqa: E402

COORDS = ("time", "bytes", "spread")


def load_scaled():
    with open(os.path.join(HERE, "p1_cost_table.json")) as f:
        raw = json.load(f)
    regions = sorted(int(k) for k in raw)
    arms = sorted(raw[str(regions[0])])
    exact = {r: {a: tuple(Fraction(raw[str(r)][a][c]) for c in COORDS) for a in arms}
             for r in regions}
    scale = []
    for k in range(len(COORDS)):
        lcm = 1
        for r in regions:
            for a in arms:
                den = exact[r][a][k].denominator
                lcm = lcm * den // gcd(lcm, den)
        scale.append(Fraction(lcm))
    table = {r: {a: tuple(int(exact[r][a][k] * scale[k]) for k in range(len(COORDS)))
                 for a in arms} for r in regions}
    return table, regions, arms, scale


def dominates(x, y):
    return all(a <= b for a, b in zip(x, y)) and any(a < b for a, b in zip(x, y))


def admissible(table, r, a):
    ca = table[r][a]
    return not any(dominates(table[r][b], ca) for b in table[r] if b != a)


def gvecs(table, r, a):
    """Difference vectors g = c(r,m) - c(r,a); the section demands <w,g> >= 0."""
    ca = table[r][a]
    return [(tuple(y - x for x, y in zip(ca, table[r][b])), table[r][b] != ca)
            for b in table[r] if b != a]


def count_linear(table, regions, arms, rung):
    """rung in {'nonneg', 'positive', 'strict'}."""
    cache = {(r, a): gvecs(table, r, a) for r in regions for a in arms}
    found = []

    def rec(i, gs, chosen):
        poly = cone.region([g for g, _ in gs])
        if not poly:
            return
        if rung == "positive" and not cone.has_strictly_positive_weights(poly):
            return
        if rung == "strict" and not cone.has_point_strict_on(
                poly, [g for g, differs in gs if differs]):
            return
        if i == len(regions):
            found.append(tuple(chosen))
            return
        r = regions[i]
        for a in arms:
            rec(i + 1, gs + cache[(r, a)], chosen + [a])

    rec(0, [], [])
    return found


def build_order_context(table, regions, arms):
    vectors = sorted({table[r][a] for r in regions for a in arms})
    idx = {v: i for i, v in enumerate(vectors)}
    strict = [(i, j) for i, u in enumerate(vectors) for j, v in enumerate(vectors)
              if i != j and dominates(u, v)]
    return vectors, idx, strict


def order_ok(n, adj, strict):
    reach = []
    for s in range(n):
        seen, stack = set(), [s]
        while stack:
            u = stack.pop()
            for v in adj[u]:
                if v not in seen:
                    seen.add(v)
                    stack.append(v)
        reach.append(seen)
    return all(u not in reach[v] for (u, v) in strict)


def count_order(table, regions, arms, ctx):
    vectors, idx, strict = ctx
    n = len(vectors)
    found = []

    def rec(i, edges, chosen):
        adj = [set() for _ in range(n)]
        for (u, v) in strict:
            adj[u].add(v)
        for (u, v) in edges:
            adj[u].add(v)
        if not order_ok(n, adj, strict):
            return
        if i == len(regions):
            found.append(tuple(chosen))
            return
        r = regions[i]
        for a in arms:
            x = idx[table[r][a]]
            new = [(x, idx[table[r][b]]) for b in table[r] if idx[table[r][b]] != x]
            rec(i + 1, edges + new, chosen + [a])

    rec(0, [], [])
    return found


def fm_check(table, regions, combo, rung):
    """Independent verdict via Fourier-Motzkin, used to cross-check cone.py."""
    d = len(COORDS)
    if rung == "nonneg":
        rows = [(tuple(Fraction(k == j) for k in range(d)), Fraction(0)) for j in range(d)]
        rows.append((tuple(Fraction(1) for _ in range(d)), Fraction(1)))
        rows.append((tuple(Fraction(-1) for _ in range(d)), Fraction(-1)))
    else:
        rows = [(tuple(Fraction(k == j) for k in range(d)), Fraction(1)) for j in range(d)]
    for r, a in zip(regions, combo):
        for g, differs in gvecs(table, r, a):
            rhs = Fraction(1) if (rung == "strict" and differs) else Fraction(0)
            rows.append((tuple(Fraction(v) for v in g), rhs))
    return feasible(rows, d)


def main():
    table, regions, arms, scale = load_scaled()
    total = len(arms) ** len(regions)
    ctx = build_order_context(table, regions, arms)

    print(f"regions {len(regions)}, arms {len(arms)}, coordinates {len(COORDS)} {COORDS}")
    print(f"coordinate scale factors applied (feasibility-invariant): "
          f"{[str(s) for s in scale]}")
    print(f"distinct cost vectors {len(ctx[0])}, strict Pareto edges among them {len(ctx[2])}")
    print(f"L0 (all sections): {total}\n")

    l1 = [c for c in itertools.product(arms, repeat=len(regions))
          if all(admissible(table, r, a) for r, a in zip(regions, c))]
    l2 = count_order(table, regions, arms, ctx)
    l3 = count_linear(table, regions, arms, "nonneg")
    l4 = count_linear(table, regions, arms, "positive")
    l5 = count_linear(table, regions, arms, "strict")
    sets = {"L1": set(l1), "L2": set(l2), "L3": set(l3), "L4": set(l4), "L5": set(l5)}

    print("counts")
    print(f"  L0 any section                       {total:8d}   100.000%")
    for name, label in (("L1", "Pareto-admissible"),
                        ("L2", "order-rationalisable"),
                        ("L3", "linear, w >= 0"),
                        ("L4", "linear, w > 0"),
                        ("L5", "linear, w > 0, unique argmin")):
        c = len(sets[name])
        print(f"  {name} {label:34s} {c:8d}   {100.0 * c / total:7.3f}%")
    print()

    print("containments observed (a failure here means a classifier is wrong)")
    for an, bn in (("L5", "L4"), ("L4", "L3"), ("L4", "L2"), ("L4", "L1"),
                   ("L2", "L1"), ("L5", "L1")):
        a, b = sets[an], sets[bn]
        ok = a <= b
        print(f"  {an} subset of {bn}: {ok}" + ("" if ok else f"   ({len(a - b)} exceptions)"))
    l3_not_l1 = sets["L3"] - sets["L1"]
    print(f"  L3 subset of L1: {not l3_not_l1}"
          + (f"   ({len(l3_not_l1)} exceptions, expected: a zero weight can "
             f"hide a dominated pick)" if l3_not_l1 else ""))
    print()

    def show(combo, why):
        print(f"  {why}")
        for r, a in zip(regions, combo):
            c = table[r][a]
            print(f"    n={r:8d} -> {a:30s} scaled cost {c}")

    print("witnesses separating the rungs")
    for an, bn, why in (
        ("L1", "L2", "in L1 but not L2: admissible at each region on its own, yet no "
                     "single consistent preference explains all six picks at once"),
        ("L2", "L4", "in L2 but not L4: a consistent preference order exists, but no "
                     "linear weighting realises it"),
        ("L3", "L4", "in L3 but not L4: rationalisable only by zeroing a coordinate, so "
                     "it is a strategy that does not weigh one of the measurements"),
        ("L4", "L5", "in L4 but not L5: a weighting permits this section without forcing "
                     "it; some region has a tie under every witnessing weight"),
    ):
        sep = sorted(sets[an] - sets[bn])
        if sep:
            show(sep[0], why)
        else:
            print(f"  {an} \\ {bn} is empty on this table")
    print()

    front = 1
    for r in regions:
        front *= sum(1 for a in arms if admissible(table, r, a))
    print(f"L1 predicted from per-region Pareto fronts: {front}, measured "
          f"{len(sets['L1'])}, agree: {front == len(sets['L1'])}")

    print("\ncross-check: cone.py against Fourier-Motzkin on a random sample")
    random.seed(20260814)
    pool = list(itertools.product(arms, repeat=len(regions)))
    sample = random.sample(pool, 300)
    for rung, key in (("nonneg", "L3"), ("positive", "L4"), ("strict", "L5")):
        bad = [c for c in sample if (c in sets[key]) != fm_check(table, regions, c, rung)]
        print(f"  {rung:9s} ({key}): {len(sample) - len(bad)}/{len(sample)} agree"
              + ("" if not bad else f"   MISMATCH on {bad[:2]}"))

    print("\nratios")
    print(f"  L1 / L0 = {len(sets['L1'])}/{total} = {len(sets['L1']) / total:.6f}")
    if sets["L1"]:
        print(f"  L4 / L1 = {len(sets['L4'])}/{len(sets['L1'])} = "
              f"{len(sets['L4']) / len(sets['L1']):.6f}")
    print(f"  L4 / L0 = {len(sets['L4'])}/{total} = {len(sets['L4']) / total:.6f}")
    print(f"  L5 / L0 = {len(sets['L5'])}/{total} = {len(sets['L5']) / total:.6f}")

    with open(os.path.join(HERE, "p2_rungs.json"), "w") as f:
        json.dump({k: sorted(map(list, v)) for k, v in sets.items()}, f)
    print("\nwritten: p2_rungs.json")


if __name__ == "__main__":
    main()
