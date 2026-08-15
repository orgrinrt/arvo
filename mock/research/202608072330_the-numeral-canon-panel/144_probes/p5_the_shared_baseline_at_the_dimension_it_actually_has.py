"""p5: the shared-baseline obligation, at the dimension the cost actually has.

NOT A BENCHMARK. Exact arithmetic over synthetic cost tables, no timing, prices
nothing.

140 section 11.4 states an obligation nobody has instrumented since: every
strategy's cost claim is stated against the same named arm rather than each
against its own naive version. 141 declined the second read explicitly
(`141:776-779`), 143 lists it as "the oldest unexamined thing in my file"
(`143:418-420`), and 142 did not touch the weighting side at all. So it stands
at one expert across three files.

Its evidence is `140_probes/p4_baseline_rebase.py`, which sweeps triples of
SCALAR costs and finds that a shared baseline never reorders anything, 0 of 840,
while per-arm baselines reorder 56.3% of the time. Both halves are right. The
scalar half is a theorem: reported = base / cost is a strictly decreasing
function of cost for a fixed positive base, so ranking by reported figure is
ranking by cost. Nothing in 840 comparisons could have come out otherwise.

The gap is the dimension. A strategy's second component is a weighting over cost
COORDINATES, so a cost is a vector, and "the ranking by absolute cost" is not
defined until a weighting fixes it. That leaves two inequivalent readings of
what a shared baseline does, and 140's probe cannot distinguish them because it
has one coordinate:

  PER-COORDINATE, BEFORE THE WEIGHTING. Divide each coordinate by the baseline
  arm's value on that coordinate, then weight. Dividing coordinate k by b_k and
  weighting by w_k is weighting the raw cost by w_k / b_k, so this is a CHANGE
  OF WEIGHTING wearing a normalisation's clothes.

  SCALAR, AFTER THE WEIGHTING. Weight first, then divide the one resulting
  number by the baseline arm's weighted cost. That divides every arm's figure by
  one positive constant, which is the scalar case again and preserves the order.

PREDICTIONS, before running:
  N1 the per-coordinate-before form changes which arm a fixed weighting picks,
     on a single fixed target, at a substantial fraction of weights. At least
     10%.
  N2 the scalar-after form never changes it, at any weight, on any target.
  N3 140's scalar result reproduces exactly: 0 disagreements shared, non-zero
     per-arm.

CONTROLS:
  P1 THE MUTUAL CONTROL, which is 140's own shape. N1 and N2 control each other.
     If BOTH come out zero the comparator is not comparing and neither claim is
     established; the probe says so and exits non-zero.
  P2 THE CASE THAT MUST FAIL for N1. When the baseline arm's cost vector is a
     uniform multiple of (1, 1, 1), the per-coordinate form must change NOTHING,
     because a uniform rescale is a scalar rescale. If it still reorders there,
     the implementation is not doing what it claims.
  P3 the per-arm baseline must reorder under vector cost too, so the hazard 140
     names survives the dimension change rather than being a scalar artifact.
"""

from fractions import Fraction as F
from itertools import product
import random

D = 3
RES = 40


def simplex(res):
    for i in range(res + 1):
        for j in range(res - i + 1):
            yield (F(i, res), F(j, res), F(res - i - j, res))


def wcost(c, w):
    return sum(w[k] * c[k] for k in range(D))


def argmin_by(vals):
    best, bv = 0, None
    for i, v in enumerate(vals):
        if bv is None or v < bv:
            bv, best = v, i
    return best


fail = []
rng = random.Random(20260815 + 31)
grid = list(simplex(RES))

print("=" * 78)
print("N3. 140's scalar result, reproduced")
print("=" * 78)
COSTS = [1, 2, 3, 4, 5, 6]
BASES = [1, 2, 3, 4, 6, 8, 12]


def rank(vals, higher_better):
    ix = list(range(len(vals)))
    ix.sort(key=lambda i: (-vals[i] if higher_better else vals[i], i))
    return tuple(ix)


sh_tot = sh_bad = pa_tot = pa_bad = 0
for costs in product(COSTS, repeat=3):
    if len(set(costs)) != 3:
        continue
    truth = rank([F(c) for c in costs], False)
    for b in BASES:
        sh_tot += 1
        if rank([F(b, c) for c in costs], True) != truth:
            sh_bad += 1
    for bs in product(BASES, repeat=3):
        rep = [F(bs[i], costs[i]) for i in range(3)]
        if len(set(rep)) != 3:
            continue
        pa_tot += 1
        if rank(rep, True) != truth:
            pa_bad += 1
print(f"  shared baseline: {sh_bad} of {sh_tot} reorder")
print(f"  per-arm baseline: {pa_bad} of {pa_tot} reorder ({100 * pa_bad / pa_tot:.1f}%)")
n3 = sh_bad == 0 and pa_bad > 0
print(f"  N3 -> {'REPRODUCED' if n3 else 'DISAGREES with 140'}")
print("  140 reports 0 of 840 and 20106 of 35724 (56.3%). The totals differ because")
print("  140 skips tied reported figures on the per-arm side only; the two verdicts")
print("  agree and the scalar half is a theorem either way.")
if not n3:
    fail.append("N3")

print()
print("=" * 78)
print("N1/N2/P1. the same obligation under a vector cost and a weighting")
print("=" * 78)
percoord_moved = scalar_moved = total = 0
witness = None
for _ in range(120):
    arms = [tuple(F(rng.randint(1, 60)) for _ in range(D)) for _ in range(7)]
    if len({a for a in arms}) != 7:
        continue
    b = rng.randrange(7)
    base = arms[b]
    norm = [tuple(F(c[k]) / base[k] for k in range(D)) for c in arms]
    for w in grid[::3]:
        total += 1
        raw_pick = argmin_by([wcost(c, w) for c in arms])
        pc_pick = argmin_by([wcost(c, w) for c in norm])
        # scalar-after: every arm's weighted cost divided by the baseline's
        denom = wcost(base, w)
        sc_pick = argmin_by([wcost(c, w) / denom for c in arms])
        if pc_pick != raw_pick:
            percoord_moved += 1
            if witness is None:
                witness = (arms, b, w, raw_pick, pc_pick)
        if sc_pick != raw_pick:
            scalar_moved += 1
print(f"  {total} (arm set, weight) cases")
print(f"  per-coordinate normalisation BEFORE the weighting changes the pick at "
      f"{percoord_moved} ({100 * percoord_moved / total:.1f}%)")
print(f"  scalar normalisation AFTER the weighting changes the pick at "
      f"{scalar_moved} ({100 * scalar_moved / total:.1f}%)")
n1 = percoord_moved / total >= 0.10
n2 = scalar_moved == 0
print(f"  N1 per-coordinate form moves the pick at least 10% of the time: "
      f"{'CONFIRMED' if n1 else 'REFUTED'}")
print(f"  N2 scalar form never moves it: {'CONFIRMED' if n2 else 'REFUTED'}")
if not n2:
    fail.append("N2")
print(f"  P1 mutual control: the two forms disagree with each other -> "
      f"{'PASS' if (percoord_moved > 0) != (scalar_moved > 0) else 'FAIL, nothing is being compared'}")
if (percoord_moved > 0) == (scalar_moved > 0):
    fail.append("P1")
if witness:
    arms, b, w, rp, pp = witness
    print(f"  a witness: baseline arm {b} = {tuple(str(x) for x in arms[b])}, "
          f"w = {tuple(str(x) for x in w)}")
    print(f"    raw cost picks arm {rp} = {tuple(str(x) for x in arms[rp])}")
    print(f"    baseline-normalised cost picks arm {pp} = {tuple(str(x) for x in arms[pp])}")
    print(f"    same arms, same weighting, one named shared baseline, two answers.")

print()
print("=" * 78)
print("P2. the case that must fail: a uniform baseline vector")
print("=" * 78)
uni_moved = uni_total = 0
for _ in range(60):
    arms = [tuple(F(rng.randint(1, 60)) for _ in range(D)) for _ in range(7)]
    if len({a for a in arms}) != 7:
        continue
    s = F(rng.randint(1, 9))
    base = (s, s, s)  # uniform: the same number on every coordinate
    norm = [tuple(F(c[k]) / base[k] for k in range(D)) for c in arms]
    for w in grid[::5]:
        uni_total += 1
        if argmin_by([wcost(c, w) for c in norm]) != argmin_by([wcost(c, w) for c in arms]):
            uni_moved += 1
print(f"  uniform baseline: {uni_moved} of {uni_total} picks moved -> "
      f"{'PASS' if uni_moved == 0 else 'FAIL, the normaliser is not a rescale'}")
if uni_moved:
    fail.append("P2")
print("  so the per-coordinate form is harmless exactly when the baseline arm costs")
print("  the same on every coordinate, which is the case where it is not a vector.")

print()
print("=" * 78)
print("P3. the hazard 140 names, under vector cost")
print("=" * 78)
pa_moved = pa_total = 0
for _ in range(120):
    arms = [tuple(F(rng.randint(1, 60)) for _ in range(D)) for _ in range(5)]
    if len({a for a in arms}) != 5:
        continue
    # each arm normalised by its OWN naive version, drawn independently
    own = [tuple(F(rng.randint(1, 60)) for _ in range(D)) for _ in range(5)]
    for w in grid[::5]:
        pa_total += 1
        truth = argmin_by([wcost(c, w) for c in arms])
        # reported: each arm's own speedup against its own baseline, higher better
        rep = [wcost(own[i], w) / wcost(arms[i], w) for i in range(5)]
        best_rep = max(range(5), key=lambda i: (rep[i], -i))
        if best_rep != truth:
            pa_moved += 1
print(f"  per-arm baselines under vector cost: {pa_moved} of {pa_total} "
      f"({100 * pa_moved / pa_total:.1f}%) report a winner that is not the winner")
p3 = pa_moved > 0
print(f"  P3 -> {'PASS, the hazard survives the dimension change' if p3 else 'FAIL'}")
if not p3:
    fail.append("P3")

print()
print("the repair, stated as what the measurement supports:")
print("  the obligation is right and it is under-specified. One named baseline arm")
print("  is necessary and it is not sufficient: WHERE the division happens decides")
print("  whether it is a normalisation or a reweighting. Applied per coordinate")
print("  before the weighting it silently changes the weighting, and the arm a")
print("  consumer asked for is not the arm they get. Applied once to the weighted")
print("  scalar it is 140's own theorem and preserves everything.")

print()
print("=" * 78)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 78)
raise SystemExit(1 if fail else 0)
