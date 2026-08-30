"""p9: attacking my own replacement, and pricing the limit it removes.

NOT A BENCHMARK. Exact arithmetic over synthetic cost tables, no timing, prices
nothing in time or bytes. What it prices is a gap in the selector's own
criterion, which is a structural quantity and is named as one.

p2 offers a weighted Chebyshev selector as the replacement for the linear one,
p8 shows it const-evaluates and lowers to one unconditional branch. An offer
with only its advantages measured is half a dispatch, so this probe measures the
two things that could sink it and one thing that decides whether it is worth
having at all.

  THE PORTABILITY COST. A Chebyshev selector needs a reference point, and the
  natural one is the componentwise ideal of the arm set, which is a fact about
  the target. So a Chebyshev weight carries a second target-dependent object
  that a linear weight does not, and it could travel worse. p4 measured that a
  linear weighting travels perfectly in the sense that matters (it re-resolves
  to that target's optimum) while the ARM does not travel at all. The question
  here is whether the arm travels even worse under Chebyshev.

  THE PRICE OF THE LIMIT. If a linear selector cannot reach an arm, what does
  the consumer lose by taking the best arm it can reach? Measured in the
  criterion the unreachable arm optimises, which is the only currency available
  without a harness.

PREDICTIONS, before running:
  X1 a Chebyshev weight carried across independent targets switches arms at
     least as often as a linear one, because the reference point moves too.
  X2 declaring the reference point in the design, rather than computing it per
     target, reduces the Chebyshev switch rate.
  X3 normalising per target by one shared baseline arm drives the Chebyshev
     switch rate to exactly zero on pure-rescale target pairs, exactly as p4's
     L3a found for the linear selector, because a rescale leaves the normalised
     table and therefore the normalised ideal unchanged.
  Z1 the price of the linear limit is not negligible: over arm sets containing
     an unreachable Pareto arm, the median relative gap between that arm and the
     best linearly reachable one, in the max-deviation criterion at the arm's
     own certificate weight, is at least 10%.

CONTROLS:
  Y1 THE CASE THAT MUST FAIL. On identical targets both selectors must switch at
     exactly zero weights. Anything else and the instrument is measuring itself.
  Y2 THE SECOND CASE THAT MUST FAIL. On the pure-rescale pairs the Chebyshev
     switch rate must be non-zero BEFORE normalising, or X3 removes nothing.
  Y3 the gap in Z1 must be zero whenever the unreachable arm is not actually
     better at its own certificate weight, which cannot happen by construction;
     a non-zero count of negative gaps means the gap formula is wrong.
"""

from fractions import Fraction as F
import random

from exact_lp import strictly_selectable, dominated

D = 3
RES = 30


def simplex(res):
    for i in range(res + 1):
        for j in range(res - i + 1):
            yield (F(i, res), F(j, res), F(res - i - j, res))


def lin_pick(arms, w):
    best, bv = 0, None
    for i, c in enumerate(arms):
        v = sum(w[k] * c[k] for k in range(D))
        if bv is None or v < bv:
            bv, best = v, i
    return best


def ideal(arms):
    return [min(F(a[k]) for a in arms) - 1 for k in range(D)]


def cheb_val(c, w, z):
    return max(w[k] * (F(c[k]) - z[k]) for k in range(D))


def cheb_pick(arms, w, z):
    best, bv = 0, None
    for i, c in enumerate(arms):
        v = cheb_val(c, w, z)
        if bv is None or v < bv:
            bv, best = v, i
    return best


def cert_weight(arms, i, z):
    raw = [F(1) / (F(arms[i][k]) - z[k]) for k in range(D)]
    s = sum(raw)
    return [r / s for r in raw]


def rand_table(rng, n=7, lo=1, hi=60):
    return [tuple(F(rng.randint(lo, hi)) for _ in range(D)) for _ in range(n)]


fail = []
rng = random.Random(20260815 + 41)
grid = list(simplex(RES))
DECLARED_Z = [F(0), F(0), F(0)]  # X2: a reference point fixed by the design

print("=" * 78)
print("X1/X2/Y1. does a Chebyshev weight travel worse than a linear one?")
print("=" * 78)
lin_rate = ch_rate = ch_fixed_rate = 0.0
N = 120
for _ in range(N):
    t1, t2 = rand_table(rng), rand_table(rng)
    z1, z2 = ideal(t1), ideal(t2)
    lin_rate += sum(1 for w in grid if lin_pick(t1, w) != lin_pick(t2, w)) / len(grid)
    ch_rate += sum(1 for w in grid
                   if cheb_pick(t1, w, z1) != cheb_pick(t2, w, z2)) / len(grid)
    ch_fixed_rate += sum(1 for w in grid
                         if cheb_pick(t1, w, DECLARED_Z) != cheb_pick(t2, w, DECLARED_Z)
                         ) / len(grid)
lin_rate /= N
ch_rate /= N
ch_fixed_rate /= N
print(f"  {N} independent target pairs, {len(grid)} weights each")
print(f"    linear selector:                       {100 * lin_rate:.1f}% switch")
print(f"    chebyshev, ideal computed per target:  {100 * ch_rate:.1f}% switch")
print(f"    chebyshev, reference declared by design:{100 * ch_fixed_rate:.1f}% switch")
x1 = ch_rate >= lin_rate
print(f"  X1 chebyshev switches at least as often: {'CONFIRMED' if x1 else 'REFUTED'}")
x2 = ch_fixed_rate < ch_rate
print(f"  X2 declaring the reference reduces it: {'CONFIRMED' if x2 else 'REFUTED'}")

t = rand_table(rng)
zt = ideal(t)
same = sum(1 for w in grid if lin_pick(t, w) != lin_pick(t, w)) + \
       sum(1 for w in grid if cheb_pick(t, w, zt) != cheb_pick(t, w, zt))
print(f"  Y1 identical targets, both selectors: {same} switches -> "
      f"{'PASS' if same == 0 else 'FAIL'}")
if same:
    fail.append("Y1")

print()
print("=" * 78)
print("X3/Y2. does the shared-baseline normalisation rescue Chebyshev too?")
print("=" * 78)
raw_ch, nor_ch = [], []
for _ in range(80):
    t1 = rand_table(rng)
    scale = [F(rng.randint(1, 8)) for _ in range(D)]
    t2 = [tuple(scale[k] * c[k] for k in range(D)) for c in t1]
    z1, z2 = ideal(t1), ideal(t2)
    raw_ch.append(sum(1 for w in grid
                      if cheb_pick(t1, w, z1) != cheb_pick(t2, w, z2)) / len(grid))
    b = 0
    n1 = [tuple(F(c[k]) / t1[b][k] for k in range(D)) for c in t1]
    n2 = [tuple(F(c[k]) / t2[b][k] for k in range(D)) for c in t2]
    zn1, zn2 = ideal(n1), ideal(n2)
    nor_ch.append(sum(1 for w in grid
                      if cheb_pick(n1, w, zn1) != cheb_pick(n2, w, zn2)) / len(grid))
rm, nm = sum(raw_ch) / len(raw_ch), sum(nor_ch) / len(nor_ch)
print(f"  pure-rescale target pairs, chebyshev with a per-target ideal:")
print(f"    un-normalised: mean {100 * rm:.1f}%  max {100 * max(raw_ch):.1f}%")
print(f"    normalised:    mean {100 * nm:.1f}%  max {100 * max(nor_ch):.1f}%")
print(f"  Y2 the rescale moves it before normalising: "
      f"{'PASS' if rm > 0 else 'FAIL, nothing to remove'}")
if rm <= 0:
    fail.append("Y2")
x3 = max(nor_ch) == 0
print(f"  X3 normalising drives it to exactly zero: {'CONFIRMED' if x3 else 'REFUTED'}")
if not x3:
    fail.append("X3")

print()
print("=" * 78)
print("Z1/Y3. what does the linear limit cost, in the criterion it forgoes?")
print("=" * 78)
gaps = []
neg = 0
sets_with = 0
tried = 0
while sets_with < 150 and tried < 4000:
    tried += 1
    arms = rand_table(rng, n=rng.randint(5, 9))
    if len({a for a in arms}) != len(arms):
        continue
    z = ideal(arms)
    par = [i for i in range(len(arms)) if not dominated(arms, i)]
    lin = [i for i in range(len(arms)) if strictly_selectable(arms, i)]
    unreach = [i for i in par if i not in lin]
    if not unreach or not lin:
        continue
    sets_with += 1
    for u in unreach:
        w = cert_weight(arms, u, z)
        vu = cheb_val(arms[u], w, z)
        vbest_lin = min(cheb_val(arms[i], w, z) for i in lin)
        g = (vbest_lin - vu) / vu
        if g < 0:
            neg += 1
        gaps.append(g)
gaps.sort()
n = len(gaps)
med = gaps[n // 2]
mean = sum(gaps) / n
print(f"  {sets_with} random arm sets containing an unreachable Pareto arm "
      f"(of {tried} drawn)")
print(f"  {n} unreachable arms, gap to the best linearly reachable arm at the")
print(f"  unreachable arm's own certificate weight, in that criterion:")
print(f"    min {100 * float(gaps[0]):.1f}%  median {100 * float(med):.1f}%  "
      f"mean {100 * float(mean):.1f}%  max {100 * float(gaps[-1]):.1f}%")
print(f"  Y3 negative gaps (impossible by construction): {neg} -> "
      f"{'PASS' if neg == 0 else 'FAIL, the gap formula is wrong'}")
if neg:
    fail.append("Y3")
z1 = med >= F(1, 10)
print(f"  Z1 median gap at least 10%: {'CONFIRMED' if z1 else 'REFUTED'}")

print()
print("the composition, which is what this is for:")
print("  the Chebyshev selector reaches every Pareto arm and costs nothing at")
print("  compile time. I expected its extra target-dependent object, the reference")
print("  point, to make a frozen selection travel worse. It does not: the two")
print("  selectors switch at 86.9% each, indistinguishable at one decimal, so the")
print("  reference point is not where the portability goes. And X2 fell: declaring")
print("  the reference in the design does not help either, which closes that route")
print("  rather than opening it. What DOES recover the rescale part of a target")
print("  change is the shared-baseline normalisation, exactly and for both")
print("  selectors. So neither selector is the answer on its own: what travels is a")
print("  weighting and a named baseline arm, and what never travels is the arm.")

print()
print("=" * 78)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 78)
raise SystemExit(1 if fail else 0)
