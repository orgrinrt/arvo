#!/usr/bin/env python3
"""p3b. The mechanism behind L1 = L2, and an exact criterion for it.

p3 found that Pareto-admissibility and order-rationalisability coincide in 60 of
60 models from each structured generator and in only 18 of 60 from the
unstructured one, and p2 found them equal on the real carrier table at 2048.
That difference deserves a mechanism rather than a note, because if the middle
rung of the ladder is empty in the regime arvo's measurements inhabit, then the
canon has two rungs to choose between and not three.

A first hypothesis, and it fails. Order-rationalisability can only differ from
admissibility through a cycle that leaves a region and returns, since revealed
edges never leave a region and a within-region cycle through a strict edge says
exactly that the chosen arm was dominated, which admissibility already caught.
So one might expect the real table to have no backward cross-region Pareto edge.
It has two. The hypothesis is refuted by the data it was meant to explain, and
it is recorded here rather than deleted.

The criterion that does hold, and it is exact rather than a proxy. Build the
UNION graph: every Pareto edge, plus every revealed edge that any ADMISSIBLE
choice could contribute. Every admissible section's own graph is a subgraph of
it. So if no strict edge of the union graph lies on a cycle, no admissible
section can be order-irrational, and L1 = L2 follows. This is sufficient, it is
checkable in one reachability sweep, and it is what the real table satisfies:
its two backward edges both point at an arm that is dominated inside its own
region, so nothing admissible ever chooses that vector and it contributes no
outgoing revealed edge to close a cycle.

The criterion is sufficient and not necessary, and that is stated rather than
assumed: a union graph with a strict cycle only says SOME section could be
irrational, and whether one is admissible as well is a further question.
"""

import json
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import p3_a_rung_count_is_a_fact_about_one_table as p3  # noqa: E402

COORDS = ("time", "bytes", "spread")


def load_real():
    with open(os.path.join(HERE, "p1_cost_table.json")) as f:
        raw = json.load(f)
    regions = sorted(int(k) for k in raw)
    arms = sorted(raw[str(regions[0])])
    table = [[tuple(raw[str(r)][a][c] for c in COORDS) for a in arms] for r in regions]
    return table, regions, arms


def admissible_arms(table):
    A = len(table[0])
    return [[a for a in range(A)
             if not any(p3.dominates(table[r][b], table[r][a]) for b in range(A) if b != a)]
            for r in range(len(table))]


def union_graph(table):
    R, A = len(table), len(table[0])
    vs = sorted({table[r][a] for r in range(R) for a in range(A)})
    idx = {v: i for i, v in enumerate(vs)}
    n = len(vs)
    strict = [(i, j) for i, u in enumerate(vs) for j, v in enumerate(vs)
              if i != j and p3.dominates(u, v)]
    adj = [set() for _ in range(n)]
    for u, v in strict:
        adj[u].add(v)
    for r, choices in enumerate(admissible_arms(table)):
        for a in choices:
            x = idx[table[r][a]]
            for b in range(A):
                y = idx[table[r][b]]
                if x != y:
                    adj[x].add(y)
    return vs, idx, adj, strict


def strict_edges_on_a_cycle(adj, strict, n):
    out = []
    for (u, v) in strict:
        seen, stack = set(), [v]
        while stack:
            p = stack.pop()
            for q in adj[p]:
                if q not in seen:
                    seen.add(q)
                    stack.append(q)
        if u in seen:
            out.append((u, v))
    return out


def backward_cross_edges(table):
    first = {}
    for r, row in enumerate(table):
        for v in row:
            first[v] = min(first.get(v, r), r)
    vs = list(first)
    return [(u, v) for u in vs for v in vs
            if u != v and p3.dominates(u, v) and first[u] > first[v]]


def main():
    table, regions, arms = load_real()
    print("the real carrier table")

    back = backward_cross_edges(table)
    print(f"  backward cross-region Pareto edges: {len(back)}")
    print("  the refuted hypothesis predicted zero of these. Each one, and where")
    print("  its endpoints live:")
    A = len(arms)
    adm = admissible_arms(table)
    for u, v in back:
        for r in range(len(table)):
            for a in range(A):
                if table[r][a] == u:
                    print(f"    dominating: n={regions[r]:8d} {arms[a]}")
                if table[r][a] == v:
                    ok = a in adm[r]
                    print(f"    dominated : n={regions[r]:8d} {arms[a]}  "
                          f"admissible in its own region: {ok}")
        print()

    vs, idx, adj, strict = union_graph(table)
    cyc = strict_edges_on_a_cycle(adj, strict, len(vs))
    print(f"  union graph over every admissible choice: {len(vs)} vertices, "
          f"{len(strict)} strict Pareto edges")
    print(f"  strict edges lying on a cycle: {len(cyc)}")
    print(f"  => no admissible section can be order-irrational: {len(cyc) == 0}")
    print(f"  which predicts L1 == L2, and p2 measured 2048 == 2048.")
    print()

    print("the same criterion against the three generators from p3")
    for gname, gen, seed in (("uniform", p3.gen_uniform, 20260814),
                             ("scaled", p3.gen_scaled, 20260815),
                             ("tiered", p3.gen_tiered, 20260816)):
        rng = random.Random(seed)
        pred_eq = agree = 0
        mismatch = 0
        eq_total = 0
        for _ in range(40):
            t = gen(rng)
            vs2, _, adj2, st2 = union_graph(t)
            clean = not strict_edges_on_a_cycle(adj2, st2, len(vs2))
            a1, a2 = p3.count_l1(t), p3.count_order(t)
            if a1 == a2:
                eq_total += 1
            if clean:
                pred_eq += 1
                if a1 == a2:
                    agree += 1
                else:
                    mismatch += 1
        print(f"  {gname}: criterion clean in {pred_eq} of 40, and of those "
              f"L1 == L2 in {agree}"
              + (f"   SOUNDNESS FAILURE on {mismatch}" if mismatch else ""))
        print(f"           L1 == L2 actually held in {eq_total} of 40, so the "
              f"criterion is {'exact here' if eq_total == pred_eq else 'sufficient, not necessary'}")
    print()

    print("what this establishes")
    print("  The middle rung of the ladder is a real rung: the unstructured")
    print("  generator separates it from admissibility in most of its models.")
    print("  It is EMPTY on the real table and on both structured generators, and")
    print("  the union-graph criterion says why in a way that can be rechecked on")
    print("  any future cost table in one sweep.")
    print()
    print("  The design consequence, stated as a consequence and not as a rule:")
    print("  where the criterion is clean, a canon that required only 'never")
    print("  select a dominated arm' would get order-consistency for free, and a")
    print("  canon that required order-consistency would be asking for nothing")
    print("  extra. The two are only worth distinguishing where it is dirty.")


if __name__ == "__main__":
    main()
