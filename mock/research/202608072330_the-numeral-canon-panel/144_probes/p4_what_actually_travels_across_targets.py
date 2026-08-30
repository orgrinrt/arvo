"""p4: 139's 44.3% says a selection is not portable. What IS portable?

NOT A BENCHMARK. Exact arithmetic over synthetic cost tables, no timing, prices
nothing.

139 section 5 reports that the same weight vector picks a different arm at 838
of 1891 simplex points across its two cost tables, and reads that as the
weighting structure being target-dependent in the way that matters. p1
reproduced the number exactly. This probe asks the two questions the number
leaves open.

FIRST, is 44.3% informative? It is a property of one invented pair of tables. If
an ordinary random pair gives the same figure, the number carries no content
beyond "the tables differ", and the design learns nothing from its size.

SECOND, and this is the part that decides what the canon owes. A weighting and
an arm are two different things to carry across a target boundary, and only one
of them can be carried without loss. Carrying the WEIGHTING means re-resolving
the selection on the new target, and by construction it lands on that target's
optimum for that weighting. Carrying the ARM means freezing target one's answer,
and that has a measurable cost. So the 44.3% is not a portability failure. It is
what a portable weighting LOOKS LIKE from the arm's side, and the design owes a
statement about which of the two travels.

THIRD, a composition with 140's shared-baseline obligation, which is the piece
143 says nobody has built an instrument against. Normalising every arm's cost by
one named baseline arm's cost, per target, is exactly a per-coordinate rescaling
of the cost table. So it should remove precisely the part of a target difference
that IS a per-coordinate rescaling and nothing else. That is a sharp conditional
and both halves are measured.

PREDICTIONS, before running:
  L1 44.3% is unremarkable: the mean switch rate over random independent target
     pairs is at least 40%.
  L2 carrying the arm costs real regret: mean relative regret at least 20% over
     random target pairs, at weights where the selection differs.
  L3a on target pairs differing by a pure per-coordinate rescaling, normalising
     by a shared baseline arm drives the switch rate to exactly zero.
  L3b on arbitrary target pairs it does not, and the residual switch rate stays
     within a few points of the un-normalised one.

CONTROLS, each must fire or the number beside it is void:
  M1 THE CASE THAT MUST FAIL. On identical targets the switch rate and the
     arm-carrying regret must both be exactly zero. Anything else means the
     instrument is measuring itself.
  M2 THE SECOND CASE THAT MUST FAIL. On the pure-rescale pairs, the
     UN-normalised switch rate must be non-zero. If a rescale does not move the
     selection to begin with, L3a is removing nothing and proves nothing.
  M3 carrying the weighting must show exactly zero regret at every weight on
     every target. This is definitional, so a non-zero here is a defect in the
     regret code rather than a finding.
"""

from fractions import Fraction as F
import random

RES = 40  # simplex grid resolution; 861 points at d = 3
D = 3
NARMS = 7


def simplex(res, d):
    if d == 3:
        for i in range(res + 1):
            for j in range(res - i + 1):
                yield (F(i, res), F(j, res), F(res - i - j, res))
    else:
        raise ValueError


def argmin(arms, w):
    best, bv = 0, None
    for i, c in enumerate(arms):
        v = sum(w[k] * c[k] for k in range(len(c)))
        if bv is None or v < bv:
            bv, best = v, i
    return best


def value(arms, i, w):
    return sum(w[k] * arms[i][k] for k in range(len(arms[i])))


def normalise(arms, b):
    """Every arm's cost divided by the baseline arm's cost, per coordinate."""
    base = arms[b]
    return [tuple(F(c[k]) / F(base[k]) for k in range(len(c))) for c in arms]


def rand_table(rng, n=NARMS, d=D, lo=1, hi=60):
    return [tuple(F(rng.randint(lo, hi)) for _ in range(d)) for _ in range(n)]


fail = []
rng = random.Random(20260815 + 23)
grid = list(simplex(RES, D))

print("=" * 78)
print("L1. is 44.3% informative? the switch rate over random target pairs")
print("=" * 78)
rates = []
for _ in range(200):
    t1, t2 = rand_table(rng), rand_table(rng)
    moved = sum(1 for w in grid if argmin(t1, w) != argmin(t2, w))
    rates.append(moved / len(grid))
rates.sort()
mean = sum(rates) / len(rates)
print(f"  200 random independent target pairs, {len(grid)} weights each")
print(f"  switch rate: min {100 * rates[0]:.1f}%  median {100 * rates[100]:.1f}%  "
      f"mean {100 * mean:.1f}%  max {100 * rates[-1]:.1f}%")
below = sum(1 for r in rates if r <= 0.443)
print(f"  139's 44.3% sits at percentile {100 * below / len(rates):.0f} of this distribution")
l1 = mean >= 0.40
print(f"  L1 mean at least 40%: {'CONFIRMED' if l1 else 'REFUTED'}")
print("  reading: the size of the number is a fact about how different two cost")
print("  tables are. It is not a fact about weightings, and nothing in the design")
print("  should be gated on it.")

print()
print("=" * 78)
print("L2/M3. carrying the arm against carrying the weighting")
print("=" * 78)
regrets = []
worst = F(0)
carry_w_nonzero = 0
for _ in range(200):
    t1, t2 = rand_table(rng), rand_table(rng)
    for w in grid[::7]:
        a1 = argmin(t1, w)          # the arm target one chose
        best2 = min(value(t2, i, w) for i in range(NARMS))
        frozen = value(t2, a1, w)   # carrying the ARM
        reresolved = value(t2, argmin(t2, w), w)  # carrying the WEIGHTING
        if reresolved != best2:
            carry_w_nonzero += 1
        if frozen != best2:
            r = (frozen - best2) / best2
            regrets.append(r)
            worst = max(worst, r)
regrets.sort()
n = len(regrets)
mean_r = sum(regrets) / n if n else F(0)
print(f"  {n} weight-and-target-pair cases where the frozen arm is not optimal")
print(f"  relative regret of carrying the ARM: mean {100 * float(mean_r):.1f}%  "
      f"median {100 * float(regrets[n // 2]):.1f}%  worst {100 * float(worst):.1f}%")
print(f"  M3 regret of carrying the WEIGHTING, non-zero cases: {carry_w_nonzero} "
      f"-> {'PASS' if carry_w_nonzero == 0 else 'FAIL, the regret code is wrong'}")
if carry_w_nonzero:
    fail.append("M3")
l2 = mean_r >= F(1, 5)
print(f"  L2 mean regret at least 20%: {'CONFIRMED' if l2 else 'REFUTED'}")
print("  reading: a weighting travels with zero loss and an arm does not. So a")
print("  predicate that names an ARM is bound to the target it was measured on,")
print("  and a predicate that names a WEIGHTING is not.")

print()
print("=" * 78)
print("M1. the case that must fail: identical targets")
print("=" * 78)
t = rand_table(rng)
moved = sum(1 for w in grid if argmin(t, w) != argmin(t, w))
badreg = 0
for w in grid[::7]:
    a1 = argmin(t, w)
    best = min(value(t, i, w) for i in range(NARMS))
    if value(t, a1, w) != best:
        badreg += 1
print(f"  identical targets: switch rate {moved}, arm-carrying regret cases {badreg}")
print(f"  M1 -> {'PASS' if moved == 0 and badreg == 0 else 'FAIL'}")
if moved or badreg:
    fail.append("M1")

print()
print("=" * 78)
print("L3. does normalising by one shared baseline arm make a weighting travel?")
print("=" * 78)
BASE = 0  # the named baseline arm, 140's obligation made concrete

print("  (a) target pairs differing by a PURE per-coordinate rescaling")
raw_rates, norm_rates = [], []
for _ in range(100):
    t1 = rand_table(rng)
    scale = [F(rng.randint(1, 8)) for _ in range(D)]
    t2 = [tuple(scale[k] * c[k] for k in range(D)) for c in t1]
    raw = sum(1 for w in grid if argmin(t1, w) != argmin(t2, w)) / len(grid)
    n1, n2 = normalise(t1, BASE), normalise(t2, BASE)
    nor = sum(1 for w in grid if argmin(n1, w) != argmin(n2, w)) / len(grid)
    raw_rates.append(raw)
    norm_rates.append(nor)
raw_mean = sum(raw_rates) / len(raw_rates)
nor_mean = sum(norm_rates) / len(norm_rates)
print(f"      un-normalised switch rate: mean {100 * raw_mean:.1f}%  "
      f"max {100 * max(raw_rates):.1f}%")
print(f"      normalised switch rate:    mean {100 * nor_mean:.1f}%  "
      f"max {100 * max(norm_rates):.1f}%")
m2 = raw_mean > 0
print(f"      M2 the rescale moves the selection before normalising: "
      f"{'PASS' if m2 else 'FAIL, nothing to remove'}")
if not m2:
    fail.append("M2")
l3a = max(norm_rates) == 0
print(f"      L3a normalising drives it to exactly zero: "
      f"{'CONFIRMED' if l3a else 'REFUTED'}")
if not l3a:
    fail.append("L3a")

print("  (b) arbitrary target pairs")
raw_b, nor_b = [], []
for _ in range(100):
    t1, t2 = rand_table(rng), rand_table(rng)
    raw_b.append(sum(1 for w in grid if argmin(t1, w) != argmin(t2, w)) / len(grid))
    n1, n2 = normalise(t1, BASE), normalise(t2, BASE)
    nor_b.append(sum(1 for w in grid if argmin(n1, w) != argmin(n2, w)) / len(grid))
rb, nb = sum(raw_b) / len(raw_b), sum(nor_b) / len(nor_b)
print(f"      un-normalised switch rate: mean {100 * rb:.1f}%")
print(f"      normalised switch rate:    mean {100 * nb:.1f}%")
print(f"      change: {100 * (nb - rb):+.1f} points")
l3b = abs(nb - rb) < 0.10
print(f"      L3b normalising does not help here: {'CONFIRMED' if l3b else 'REFUTED'}")

print()
print("  the composed statement, which is what the two halves are for:")
print("    a weighting travels across a target change exactly to the extent that")
print("    the change is a per-coordinate rescaling of the cost table, and one")
print("    shared baseline arm, normalised per target, is what makes that part")
print("    travel. The rest of a target change reorders arms genuinely and no")
print("    normalisation can recover it.")

print()
print("=" * 78)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 78)
raise SystemExit(1 if fail else 0)
