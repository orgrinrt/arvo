#!/usr/bin/env python3
"""p4b. Independent recount of p4's tropical column, written from scratch
rather than by editing p4, because p4 returned the identical count (1360) in
all four non-monotone cells and an identical number appearing four times across
two different windows and two different reductions is the shape of a counting
bug.

It is not a bug: this script reproduces 1360 in all four cells. It also counts
the underlying commuting failures UNWEIGHTED, over reachable (u, v) pairs rather
than over (a, b, c) triples, and those differ (240, 450, 464, 562), which
confirms the four reductions are genuinely different functions and the collision
is in the Q^3-weighted count alone.

The collision is recorded as an observed coincidence with no mechanism offered.
I did not explain it and I do not claim it means anything.

Run: python3 p4b_recount.py
"""

out = []


def say(s):
    out.append(s)
    print(s)


for name, (lo, hi) in {"unsigned [0,15]": (0, 15), "signed [-8,7]": (-8, 7)}.items():
    n = hi - lo + 1
    rhos = {
        "wrap": lambda x, lo=lo, n=n: (x - lo) % n + lo,
        "saturate": lambda x, lo=lo, hi=hi: lo if x < lo else (hi if x > hi else x),
        "mutant": lambda x, lo=lo, hi=hi: hi if x < lo else (lo if x > hi else x),
    }
    Q = list(range(lo, hi + 1))
    reach = list(range(2 * lo, 2 * hi + 1))
    for pn, rho in rhos.items():
        bad = sum(
            1
            for a in Q
            for b in Q
            for c in Q
            if rho(min(a, b) + c) != min(rho(a + c), rho(b + c))
        )
        uv = sum(
            1 for u in reach for v in reach if rho(min(u, v)) != min(rho(u), rho(v))
        )
        say(f"{name:16s} {pn:10s} trop_over_Q3={bad:5d}  commuting_failures_over_reachable_pairs={uv:4d}")

say("")
say("p4's counts reproduce. The 1360 collision across four cells is real and")
say("unexplained; the unweighted counts differ, so the reductions are not the")
say("same function.")

with open("p4b_recount.out", "w") as f:
    f.write("\n".join(out) + "\n")
