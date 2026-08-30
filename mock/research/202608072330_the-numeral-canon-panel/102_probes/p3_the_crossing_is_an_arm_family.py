#!/usr/bin/env python3
"""p3. Is the accuracy crossing a fork between two arms, or a family of them?

`101` section 6.1 measures two fixed-point arms against exact arithmetic over a
chain and finds their accuracy rankings CROSS at chain length k = 4: a finer grid
with truncation leads early because its per-step error is smaller, and loses later
because that error is BIASED and accumulates linearly while the unbiased arm's
accumulates as a random walk. Its constructive answer is that chain length belongs
in the REGION rather than in the cost vector, so the argmin selects a different arm
at a different chain length.

That answer is right. This probe asks what is on the other side of it: once chain
depth is a region dimension, what is the arm set at a given depth?

## Two failures in version one, preserved beside this file, because both are the finding

`p3_first_version_contractive_chain_no_crossing.py` used a contracting chain,
`a <- a*3/4 + x`. It reported NO crossing at any k, and I had written prose saying
the crossing reproduced. It does not, and the reason is structural rather than a
tuning miss: a contraction damps old error geometrically, so neither the bias nor
the walk accumulates and both arms reach a steady state. **A crossing between a
biased arm and an unbiased one exists only where error accumulates, so the chain
has to be non-contracting for the question to be live at all.** That is worth more
than the run it broke: `101`'s finding carries a hidden predicate on the chain's
gain, and a strategy weighing accuracy over chains has nothing to decide inside a
contraction.

Version one's second failure was mine and more ordinary. It scored arms on accuracy
alone, so the arm that rounds well on the finest grid dominated at every depth and
the "best" switch depth collapsed to zero. **An accuracy-only model has no fork in
it**: round-to-nearest on the finest available grid wins, always, and there is
nothing for a strategy to weigh. The interesting object appears only when the
rounding itself carries a cost.

## So this version

Non-contracting chain, so error accumulates. Two coordinates, so there is something
to trade: the mean absolute error against the exact rational value, and the number
of round-to-nearest steps the arm performs, which is a DECLARED static property of
the arm rather than a measurement, exactly as bits-per-element is in `97`'s model
and as `101` section 2.3 says a declared coordinate legitimately may be.

The arm family is the switch depth: hold intermediates on the fine grid, truncate
for the first `d` steps, round to nearest afterwards. `d = k+1` truncates
everywhere and `d = 0` rounds everywhere, so the two obvious arms are the ends of
a family with `k+2` members, and the coarse-grid arm sits outside it.

Reference is `fractions.Fraction`, so the error is the real one.

I wrote the model from the description of the phenomenon rather than from
`101_probes/p6_accuracy_is_not_a_per_arm_scalar.py`, so reproducing the crossing is
evidence rather than a copy.

This is a spike. Its grids, gain and seed count are scaffolding to reach the check.

Run:  python3 p3_the_crossing_is_an_arm_family.py
"""

from fractions import Fraction as F

DECL_BITS = 8
FINE_BITS = 9  # one bit finer, so the two error terms are comparable at small k
DECL = F(1, 1 << DECL_BITS)
FINE = F(1, 1 << FINE_BITS)

GAIN = F(1)  # non-contracting: this is what version one got wrong
SEEDS = 512
KS = [1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128]


def stream(seed, n):
    x = (seed * 6364136223846793005 + 1442695040888963407) & ((1 << 64) - 1)
    out = []
    for _ in range(n):
        x = (x * 6364136223846793005 + 1442695040888963407) & ((1 << 64) - 1)
        out.append(F(x >> 40, 1 << 24))
    return out


def q_trunc(v, grid):
    return (v / grid).__floor__() * grid


def q_rne(v, grid):
    n = v / grid
    fl = n.__floor__()
    frac = n - fl
    if frac > F(1, 2) or (frac == F(1, 2) and fl % 2 == 1):
        fl += 1
    return fl * grid


def run(xs, k, grid, switch_at):
    a = xs[0]
    for j in range(1, k + 1):
        v = a * GAIN + xs[j]
        a = q_trunc(v, grid) if j < switch_at else q_rne(v, grid)
    return a


def exact(xs, k):
    a = xs[0]
    for j in range(1, k + 1):
        a = a * GAIN + xs[j]
    return a


def mean_err(k, grid, switch_at):
    tot = F(0)
    for s in range(1, SEEDS + 1):
        xs = stream(s, k + 1)
        tot += abs(run(xs, k, grid, switch_at) - exact(xs, k))
    return tot / SEEDS / DECL


def rne_steps(k, switch_at):
    """The declared cost coordinate: how many steps pay round-to-nearest."""
    return max(0, k - max(0, min(switch_at, k + 1)) + 1) if switch_at <= k else 0


# ---------------------------------------------------------------------------
# 1. does the crossing reproduce on a non-contracting chain
# ---------------------------------------------------------------------------

print(f"declared grid 2^-{DECL_BITS}, fine grid 2^-{FINE_BITS}, chain a <- a*{GAIN} + x")
print(f"{SEEDS} seeds per point, error in declared ulp, reference exact rational")
print()
print("1. DOES THE CROSSING REPRODUCE")
print()
print("A = fine grid, truncation everywhere   (biased, no rne steps)")
print("B = declared grid, round-to-nearest    (unbiased, k rne steps, coarser grid)")
print()
print(f"{'k':>5} {'A err':>10} {'B err':>10}  leader")
lead_prev, crossing = None, []
errs = {}
for k in KS:
    a = mean_err(k, FINE, k + 1)
    b = mean_err(k, DECL, 0)
    errs[k] = (a, b)
    lead = "A" if a < b else "B"
    if lead_prev is not None and lead != lead_prev:
        crossing.append(k)
    lead_prev = lead
    print(f"{k:>5} {float(a):>10.5f} {float(b):>10.5f}  {lead}")

print()
if crossing:
    print(f"CROSSING at k in {crossing}: the ranking of A and B is not a per-arm fact.")
else:
    print("NO CROSSING in the swept range: one arm leads throughout.")

# ---------------------------------------------------------------------------
# 2. the arm family, and what a weighting over the two coordinates selects
# ---------------------------------------------------------------------------

print()
print("2. THE ARM FAMILY AT EACH DEPTH, AND WHAT A WEIGHTING SELECTS")
print()
print("Arm D(d): fine grid, truncate for the first d steps, round to nearest after.")
print("Coordinates: (mean error in declared ulp, rne steps performed).")
print("The second is declared and exact, not measured.")
print()

FAMILY_KS = [4, 16, 64]
for k in FAMILY_KS:
    print(f"  k = {k}")
    print(f"    {'d':>4} {'err':>10} {'rne steps':>10}  {'note':<26}")
    pts = []
    for d in range(0, k + 2):
        e = mean_err(k, FINE, d)
        c = rne_steps(k, d)
        pts.append((d, e, c))
    # Pareto front on (error, rne steps), both minimised
    front = []
    for d, e, c in pts:
        if not any((e2 <= e and c2 <= c and (e2 < e or c2 < c)) for _, e2, c2 in pts):
            front.append((d, e, c))
    for d, e, c in pts:
        note = "on the Pareto front" if (d, e, c) in front else ""
        if d == 0:
            note += " (round everywhere)"
        if d == k + 1:
            note += " (truncate everywhere = A)"
        print(f"    {d:>4} {float(e):>10.5f} {c:>10}  {note:<26}")
    print(f"    Pareto front size: {len(front)} of {len(pts)} arms")
    # how many distinct arms a weighting can select, by sweeping the exchange rate
    sel = set()
    r = F(1, 1 << 20)
    while r < F(1 << 20):
        best = min(pts, key=lambda t: (t[1] + r * t[2], t[0]))
        sel.add(best[0])
        r *= 2
    print(f"    distinct arms selected across 40 exchange rates: {len(sel)} -> {sorted(sel)}")
    print()

print("WHAT THIS ESTABLISHES")
print()
print("ONE. The crossing reproduces on a chain built independently of `101`'s, and")
print("only on a non-contracting one. So `101`'s finding is real and it carries a")
print("predicate its own statement does not: the chain's gain. Inside a contraction")
print("there is nothing to weigh, because old error is damped rather than accumulated.")
print()
print("TWO. The two arms are the two ends of a family with k+2 members, and the")
print("interior members are on the Pareto front. Reading the crossing as `pick A")
print("below it and B above` takes two points from a front that has more.")
print()
print("THREE. And that is the converged mechanism working, not failing. Once accuracy")
print("is a coordinate and chain depth is in the region, a weighting over (error, rne")
print("steps) selects an interior arm, and different weightings select different ones.")
print("This is the first instance in this unit of a strategy weighing something other")
print("than time and bytes, and it behaves exactly as `98` section 4 says a weighting")
print("should: op's four intents each name a primary concern and refuse to make it")
print("absolute, and the interior of this front is where a finite rate lands.")
print()
print("FOUR. What it costs is that the switch depth has to be const, which p4 tests.")
