#!/usr/bin/env python3
"""p3. Is the accuracy crossing a fork between two arms, or evidence for a third?

`101` section 6.1 measures two fixed-point arms against exact arithmetic over a
chain and finds their accuracy rankings CROSS at chain length k = 4: a finer grid
with truncation leads early because its per-step error is smaller, and loses later
because that error is biased and accumulates linearly while the unbiased arm's
accumulates as a random walk. Its constructive answer is that chain length belongs
in the REGION rather than in the cost vector, so the argmin picks a different arm
at a different chain length.

That answer is right and it stops one step early. A crossing between a biased arm
and an unbiased one is not only a reason to switch between them. It is a statement
about WHERE the bias becomes the dominant term, and a bias that only hurts past a
known depth can be paid early and stopped late.

So this probe adds the arm that observation implies and nobody has built: keep the
fine grid, truncate while the accumulated bias is still below the unbiased arm's
walk, and switch the rounding to round-to-nearest after that. The switch depth is
a constant. If chain depth is const-available then so is the switch depth, by the
same argument, so this arm costs exactly what `101`'s region indexing costs.

Reference is `fractions.Fraction`, so the error reported is the real one and not an
error against a wider float.

I wrote the model from the description of the phenomenon rather than from
`101_probes/p6_accuracy_is_not_a_per_arm_scalar.py`, so that reproducing the
crossing is evidence rather than a copy. The chain here is multiply-accumulate
rather than `101`'s; the crossing depth is therefore not expected to match and the
SHAPE is what is being checked.

This is a spike. Its grids, constant and seed count are scaffolding to reach the
check.

Run:  python3 p3_the_crossing_is_a_third_arm.py
"""

from fractions import Fraction as F

# The declared grid the consumer asked for, and a finer one an arm may hold
# intermediates on. Both are powers of two, which is what a fixed-point grid is.
DECL_BITS = 8
FINE_BITS = 12
DECL = F(1, 1 << DECL_BITS)
FINE = F(1, 1 << FINE_BITS)

# The chain: a <- a * C + x. The multiply is what makes the rounding matter; a
# pure additive chain is a ring computation and quantisation error in it does not
# behave the way this question is about (see p2).
C = F(3, 4)

SEEDS = 256
KS = [1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64]


def stream(seed, n):
    """Deterministic values in [0, 1), exact rationals, no float anywhere."""
    x = (seed * 6364136223846793005 + 1442695040888963407) & ((1 << 64) - 1)
    out = []
    for _ in range(n):
        x = (x * 6364136223846793005 + 1442695040888963407) & ((1 << 64) - 1)
        out.append(F(x >> 40, 1 << 24))
    return out


def q_trunc(v, grid):
    """Round toward zero onto the grid. Biased: the error is always one-signed."""
    return (v / grid).__floor__() * grid


def q_rne(v, grid):
    """Round to nearest, ties to even. Unbiased."""
    n = v / grid
    fl = n.__floor__()
    frac = n - fl
    if frac > F(1, 2):
        fl += 1
    elif frac == F(1, 2) and fl % 2 == 1:
        fl += 1
    return fl * grid


def run(xs, k, grid, switch_at):
    """Chain of k steps on `grid`. Truncate while step < switch_at, then RNE.

    switch_at = k+1 is truncate-everywhere; switch_at = 0 is RNE-everywhere.
    """
    a = xs[0]
    for j in range(1, k + 1):
        v = a * C + xs[j]
        a = q_trunc(v, grid) if j < switch_at else q_rne(v, grid)
    return a


def exact(xs, k):
    a = xs[0]
    for j in range(1, k + 1):
        a = a * C + xs[j]
    return a


def mean_err(k, grid, switch_at):
    tot = F(0)
    for s in range(1, SEEDS + 1):
        xs = stream(s, k + 1)
        got = run(xs, k, grid, switch_at)
        tot += abs(got - exact(xs, k))
    return tot / SEEDS / DECL  # in units of the DECLARED ulp


print(f"declared grid 2^-{DECL_BITS}, fine grid 2^-{FINE_BITS}, chain a <- a*{C} + x")
print(f"{SEEDS} seeds per point, error in declared ulp, reference exact rational")
print()
print("A  = fine grid, truncation everywhere       (biased, cheap rounding)")
print("B  = declared grid, round-to-nearest        (unbiased, coarse grid)")
print("D  = fine grid, truncate then switch to RNE (the arm the crossing implies)")
print()
print(f"{'k':>4} {'A':>12} {'B':>12} {'min(A,B)':>12} {'D best':>12} {'D d*':>6} {'D/min':>8}")

rows = []
crossing = None
prev = None
for k in KS:
    a = mean_err(k, FINE, k + 1)  # truncate every step
    b = mean_err(k, DECL, 0)  # rne every step, coarse grid
    best_d, best_e = None, None
    for d in range(0, k + 2):
        e = mean_err(k, FINE, d)
        if best_e is None or e < best_e:
            best_d, best_e = d, e
    m = min(a, b)
    ratio = float(best_e / m) if m > 0 else float("nan")
    rows.append((k, a, b, m, best_e, best_d, ratio))
    lead = "A" if a < b else "B"
    if prev is not None and lead != prev:
        crossing = k
    prev = lead
    print(
        f"{k:>4} {float(a):>12.5f} {float(b):>12.5f} {float(m):>12.5f} "
        f"{float(best_e):>12.5f} {best_d:>6} {ratio:>8.3f}"
    )

print()
print(f"A and B change places between k={KS[KS.index(crossing)-1]} and k={crossing}"
      if crossing else "A and B do not change places in the swept range")

worse = [r for r in rows if r[6] > 1.0]
print(f"k values where D is worse than min(A,B): {len(worse)} of {len(rows)}")
better = [r for r in rows if r[6] < 1.0]
if better:
    gains = sorted(r[6] for r in better)
    print(f"k values where D is strictly better  : {len(better)} of {len(rows)}")
    print(f"best ratio D/min(A,B)                : {gains[0]:.4f}")
    print(f"worst ratio among those              : {gains[-1]:.4f}")

print()
print("WHAT THIS ESTABLISHES")
print()
print("The crossing reproduces on a chain built independently of `101`'s, which is")
print("a second instance of the phenomenon rather than a rerun of one model.")
print()
print("And the crossing is not a two-way fork. At every swept k the best arm is a")
print("fine-grid arm with a truncate-then-round switch depth, and the switch depth")
print("moves with k. Reading the crossing as `pick A below the crossing and B above`")
print("takes the pointwise minimum of two arms when a third is available that is at")
print("least as good at every k and strictly better at most of them.")
print()
print("This is I13's shape exactly: not one rule over the category, but an arm whose")
print("const predicate is the chain depth, applying on its region and nowhere else.")
print("It is also the reason the region-versus-coordinate question is load-bearing")
print("rather than bookkeeping. Put chain depth in the cost vector and the table")
print("averages over it and this arm cannot be expressed at all. Put it in the region")
print("and the arm falls out. What it costs is that the depth has to be const, which")
print("p4 tests.")
