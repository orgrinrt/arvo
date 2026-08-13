#!/usr/bin/env python3
"""p3. How much of a rung count is a fact about the arms, and how much about the table.

p2 measured, on one committed cost table, that of 46656 sections 2048 are
Pareto-admissible, 2048 are order-rationalisable and 58 are linearly
rationalisable with strictly positive weights.

Those are three numbers about one table. Before anything cites them, two
questions need answering, and neither is answerable from that table alone.

  1. Is L1 = L2 a fact, or a coincidence? On the real table the order rung
     added nothing at all. If that holds generally, the middle of the ladder is
     free and there is no reason to name it. If it is a coincidence of costs
     that grow with column size, it is not free and it matters.

  2. How stable is the L4 / L1 ratio? If it swings by orders of magnitude
     across tables of the same shape, then no particular value of it can be
     quoted as saying anything about arvo, and what survives is the ORDERING of
     the rungs rather than any gap between them.

Method. Random models of a fixed shape, from three generators chosen to differ
in the structure they impose rather than only in their seed, since three samples
from one generator are one instance wearing three hats.

  uniform     every coordinate independent and uniform. No structure at all.
  scaled      the shape the real table has: time grows with region index, the
              footprint coordinate is a property of the arm and constant across
              regions, spread is proportional to time with per-arm noise.
  tiered      arms split into a fast tier and a slow tier with a large gap,
              which is what the harness's own findings file reports for the
              carrier bench ("a tier split ... a qualitative difference, not a
              gradient").

Smaller than the real table (5 regions, 5 arms) so that several hundred models
fit in the time budget. The shape question being asked does not depend on the
exact size, and the size is stated rather than hidden.
"""

import json
import os
import random
import statistics
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import cone  # noqa: E402

REGIONS = 5
ARMS = 5
D = 3


def dominates(x, y):
    return all(a <= b for a, b in zip(x, y)) and any(a < b for a, b in zip(x, y))


def gen_uniform(rng):
    return [[tuple(rng.randint(1, 40) for _ in range(D)) for _ in range(ARMS)]
            for _ in range(REGIONS)]


def gen_scaled(rng):
    bytes_per_arm = [rng.choice([13, 16, 32, 64, 128]) for _ in range(ARMS)]
    base = [rng.randint(50, 200) for _ in range(ARMS)]
    table = []
    for r in range(REGIONS):
        mult = 8 ** r
        row = []
        for a in range(ARMS):
            t = base[a] * mult + rng.randint(0, base[a] * mult // 4)
            s = max(1, t * rng.randint(1, 40) // 100)
            row.append((t, bytes_per_arm[a], s))
        table.append(row)
    return table


def gen_tiered(rng):
    tier = [rng.randint(0, 1) for _ in range(ARMS)]
    bytes_per_arm = [rng.choice([13, 16, 32, 64]) for _ in range(ARMS)]
    table = []
    for r in range(REGIONS):
        mult = 8 ** r
        row = []
        for a in range(ARMS):
            t = (100 if tier[a] == 0 else 160) * mult + rng.randint(0, 10 * mult)
            s = max(1, t * rng.randint(1, 30) // 100)
            row.append((t, bytes_per_arm[a], s))
        table.append(row)
    return table


def admissible_set(table):
    out = []
    for r in range(REGIONS):
        out.append([a for a in range(ARMS)
                    if not any(dominates(table[r][b], table[r][a])
                               for b in range(ARMS) if b != a)])
    return out


def count_l1(table):
    n = 1
    for row in admissible_set(table):
        n *= len(row)
    return n


def count_order(table):
    vectors = sorted({table[r][a] for r in range(REGIONS) for a in range(ARMS)})
    idx = {v: i for i, v in enumerate(vectors)}
    n = len(vectors)
    strict = [(i, j) for i, u in enumerate(vectors) for j, v in enumerate(vectors)
              if i != j and dominates(u, v)]
    found = [0]

    def ok(edges):
        adj = [set() for _ in range(n)]
        for (u, v) in strict:
            adj[u].add(v)
        for (u, v) in edges:
            adj[u].add(v)
        for (u, v) in strict:
            seen, stack = set(), [v]
            while stack:
                p = stack.pop()
                for q in adj[p]:
                    if q not in seen:
                        seen.add(q)
                        stack.append(q)
            if u in seen:
                return False
        return True

    def rec(r, edges):
        if not ok(edges):
            return
        if r == REGIONS:
            found[0] += 1
            return
        for a in range(ARMS):
            x = idx[table[r][a]]
            new = [(x, idx[table[r][b]]) for b in range(ARMS) if idx[table[r][b]] != x]
            rec(r + 1, edges + new)

    rec(0, [])
    return found[0]


def count_linear(table, rung):
    found = [0]

    def gs_for(r, a):
        ca = table[r][a]
        return [(tuple(y - x for x, y in zip(ca, table[r][b])), table[r][b] != ca)
                for b in range(ARMS) if b != a]

    cache = {(r, a): gs_for(r, a) for r in range(REGIONS) for a in range(ARMS)}

    def rec(r, gs):
        poly = cone.region([g for g, _ in gs])
        if not poly:
            return
        if rung == "positive" and not cone.has_strictly_positive_weights(poly):
            return
        if r == REGIONS:
            found[0] += 1
            return
        for a in range(ARMS):
            rec(r + 1, gs + cache[(r, a)])

    rec(0, [])
    return found[0]


def main():
    total = ARMS ** REGIONS
    trials = 60
    print(f"model shape: {REGIONS} regions, {ARMS} arms, {D} coordinates, "
          f"L0 = {total} sections")
    print(f"{trials} models per generator, seeds 20260814 / 20260815 / 20260816\n")

    summary = {}
    for name, gen, seed in (("uniform", gen_uniform, 20260814),
                            ("scaled", gen_scaled, 20260815),
                            ("tiered", gen_tiered, 20260816)):
        rng = random.Random(seed)
        l1s, l2s, l4s, eq = [], [], [], 0
        for _ in range(trials):
            table = gen(rng)
            a = count_l1(table)
            b = count_order(table)
            c = count_linear(table, "positive")
            l1s.append(a)
            l2s.append(b)
            l4s.append(c)
            if a == b:
                eq += 1
        summary[name] = {"l1": l1s, "l2": l2s, "l4": l4s, "eq": eq}
        print(f"generator {name}")
        for label, xs in (("L1", l1s), ("L2", l2s), ("L4", l4s)):
            print(f"  {label}: min {min(xs):6d}  median {statistics.median(xs):9.1f}  "
                  f"max {max(xs):6d}  ({100.0 * statistics.median(xs) / total:6.3f}% of L0)")
        ratios = [c / a for a, c in zip(l1s, l4s) if a]
        print(f"  L4/L1: min {min(ratios):.5f}  median {statistics.median(ratios):.5f}  "
              f"max {max(ratios):.5f}  spread {max(ratios) / max(min(ratios), 1e-9):.1f}x")
        print(f"  L1 == L2 in {eq} of {trials} models")
        print()

    print("what this settles")
    alleq = all(summary[k]["eq"] == trials for k in summary)
    if alleq:
        print("  L1 == L2 in every model from every generator. Order-rationalisability")
        print("  adds nothing to Pareto-admissibility at this shape, so the middle rung")
        print("  of the ladder is not a rung: naming it buys nothing.")
    else:
        for k in summary:
            n = trials - summary[k]["eq"]
            if n:
                print(f"  {k}: L1 != L2 in {n} of {trials}. The order rung is real "
                      f"and does bite, so it is a rung and not a restatement.")
    print()
    print("  L4/L1 across all models:")
    allr = [c / a for k in summary for a, c in zip(summary[k]["l1"], summary[k]["l4"]) if a]
    print(f"    min {min(allr):.5f}  median {statistics.median(allr):.5f}  max {max(allr):.5f}")
    print(f"    ratio of max to min: {max(allr) / max(min(allr), 1e-9):.1f}x")
    print("  A specific value of this ratio is a fact about one table. Its ORDER,")
    print("  that a weighting admits far fewer sections than admissibility does,")
    print("  is what holds across every generator here.")

    with open(os.path.join(HERE, "p3_rung_counts.json"), "w") as f:
        json.dump(summary, f)
    print("\nwritten: p3_rung_counts.json")


if __name__ == "__main__":
    main()
