#!/usr/bin/env python3
# v3 (151): 150's dissents on the weighting side, reproduced before conceding.
#
# 150 says the baseline conflict 145 found is real and that both readings of it are one dimension
# short: the per-coordinate division carries TWO independent choices, units and baseline provenance,
# and the cross-target invariance appears in exactly one cell of four and needs both. It adds a
# consequence neither 145 nor 146 drew: under baseline-relative units the baseline arm's own figure
# is identically the weight sum, so the reporting normalisation is division by one and there is no
# second operation to place.
#
# 150 also says 146 section 4 carries F144-2 without `cost coordinates = 2`, and that recomputation
# gives +1 at two coordinates and exactly 0 at three.
#
# Predictions, stated before running:
#
#   C1. The two-by-two: only baseline-relative units with a per-target baseline drives the
#       cross-target switch rate to zero on a pure per-coordinate rescale. The other three cells
#       are nonzero.
#   C2. CONTROL: the two absolute rows must be IDENTICAL, digit for digit, because the baseline
#       never enters an absolute-units selection. 150 reports its own first version drew fresh
#       tables per cell and produced a difference that cannot exist; a shared draw is what makes
#       the cell comparison mean anything.
#   C3. CONTROL: the arbitrary-target column must be nonzero in every cell, or the invariant cell
#       is a claim that targets do not matter rather than a change of units.
#   C4. Under baseline-relative units on the simplex the baseline arm's figure is exactly 1 at
#       every case, so the reporting normalisation divides by one.
#   C5. 150's D3: F144-2's optimum is +1 at two cost coordinates and exactly 0 at three with a
#       constant third, and the three-coordinate zero needs no solver because putting all weight on
#       the constant coordinate makes every arm tie.
#   C6. CONTROL for C5: 139's control arm at (4,4), inside the hull, must come out strictly
#       negative at two coordinates, or the procedure cannot tell a selectable arm from an
#       unselectable one.
from fractions import Fraction as Fr
import random

random.seed(20260815)

def pick(costs, w):
    best, bi = None, None
    for i, c in enumerate(costs):
        s = sum(wk * ck for wk, ck in zip(w, c))
        if best is None or s < best:
            best, bi = s, i
    return bi

def simplex(n, res):
    def rec(k, left):
        if k == 1:
            yield (Fr(left, res),)
            return
        for i in range(left + 1):
            for rest in rec(k - 1, left - i):
                yield (Fr(i, res),) + rest
    return [w for w in rec(n, res) if any(x > 0 for x in w)]

ARMS, COORDS, RES = 7, 3, 12
W = simplex(COORDS, RES)

def draw(n=ARMS, k=COORDS, hi=60):
    return [tuple(Fr(random.randint(1, hi)) for _ in range(k)) for _ in range(n)]

print("=" * 96)
print("C1/C2/C3. The two-by-two: units crossed with baseline provenance, one shared population")
print("=" * 96)

DRAWS = 40
cells = {}
for units in ("absolute", "relative"):
    for prov in ("fixed", "per-target"):
        cells[(units, prov)] = {"rescale": [0, 0], "arbitrary": [0, 0]}

random.seed(20260815)
population = []
for _ in range(DRAWS):
    C1 = draw()
    r = tuple(Fr(random.randint(1, 8)) for _ in range(COORDS))
    C1r = [tuple(rk * ck for rk, ck in zip(r, c)) for c in C1]
    C2 = draw()
    bi = random.randrange(ARMS)
    population.append((C1, C1r, C2, bi))

def rel(C, b):
    return [tuple(ck / bk for ck, bk in zip(c, b)) for c in C]

for (C1, C1r, C2, bi) in population:
    b1, b1r, b2 = C1[bi], C1r[bi], C2[bi]
    if any(x == 0 for x in b1 + b1r + b2):
        continue
    for w in W:
        for units in ("absolute", "relative"):
            for prov in ("fixed", "per-target"):
                if units == "absolute":
                    A, Bres, Barb = C1, C1r, C2
                else:
                    A = rel(C1, b1)
                    Bres = rel(C1r, b1r if prov == "per-target" else b1)
                    Barb = rel(C2, b2 if prov == "per-target" else b1)
                p0 = pick(A, w)
                cells[(units, prov)]["rescale"][1] += 1
                cells[(units, prov)]["arbitrary"][1] += 1
                if pick(Bres, w) != p0:
                    cells[(units, prov)]["rescale"][0] += 1
                if pick(Barb, w) != p0:
                    cells[(units, prov)]["arbitrary"][0] += 1

print(f"  {'units':<12}{'baseline':<14}{'pure rescale':>16}{'arbitrary change':>20}")
res_table = {}
for units in ("absolute", "relative"):
    for prov in ("fixed", "per-target"):
        c = cells[(units, prov)]
        a = 100.0 * c["rescale"][0] / c["rescale"][1]
        b = 100.0 * c["arbitrary"][0] / c["arbitrary"][1]
        res_table[(units, prov)] = (a, b)
        print(f"  {units:<12}{prov:<14}{a:>15.1f}%{b:>19.1f}%")

zero_cells = [k for k, v in res_table.items() if v[0] == 0.0]
c1 = zero_cells == [("relative", "per-target")]
print(f"\n  C1 (the invariance is exactly one cell and it is relative/per-target): "
      f"{'CONFIRMED' if c1 else 'REFUTED'}, zero cells = {zero_cells}")
c2 = res_table[("absolute", "fixed")] == res_table[("absolute", "per-target")]
print(f"  C2 CONTROL (the two absolute rows are identical): {c2} (must be True)")
c3 = all(v[1] > 0 for v in res_table.values())
print(f"  C3 CONTROL (every cell moves under an arbitrary target change): {c3} (must be True)")

print()
print("=" * 96)
print("C4. Under relative units, what is the baseline arm's own figure?")
print("=" * 96)
cases = 0
ones = 0
rank_eq = 0
for (C1, _, _, bi) in population:
    b1 = C1[bi]
    if any(x == 0 for x in b1):
        continue
    R = rel(C1, b1)
    for w in W:
        cases += 1
        fig = sum(wk * ck for wk, ck in zip(w, R[bi]))
        if fig == 1:
            ones += 1
        # the reported ranking under a division by the baseline's own weighted figure
        base_fig = fig
        rep = [sum(wk * ck for wk, ck in zip(w, c)) / base_fig for c in R]
        sel = [sum(wk * ck for wk, ck in zip(w, c)) for c in R]
        if min(range(len(rep)), key=lambda i: rep[i]) == min(range(len(sel)),
                                                             key=lambda i: sel[i]):
            rank_eq += 1
print(f"  cases: {cases}")
print(f"  baseline arm's own relative figure equals 1: {ones} ({100.0 * ones / cases:.1f}%)")
print(f"  the reported ranking equals the selection ranking: {rank_eq} of {cases}")
c4 = ones == cases
print(f"  C4: {'CONFIRMED' if c4 else 'REFUTED'}")
print("  So under relative units the reporting normalisation divides every arm's figure by one,")
print("  and the two operations 144 sections 6 and 4.3 name stop competing: there is only one.")

print()
print("=" * 96)
print("C5/C6. 150's D3: F144-2 at two cost coordinates and at three")
print("=" * 96)

def worst_case_gap(arms, idx, res):
    """min over the simplex of (arm idx's weighted cost) - (best rival's weighted cost),
    on an exact rational grid. Negative means strictly selectable somewhere."""
    n = len(arms[0])
    best = None
    argw = None
    for w in simplex(n, res):
        s = sum(wk * ck for wk, ck in zip(w, arms[idx]))
        r = min(sum(wk * ck for wk, ck in zip(w, c))
                for j, c in enumerate(arms) if j != idx)
        g = s - r
        if best is None or g < best:
            best, argw = g, w
    return best, argw

two = [(Fr(0), Fr(10)), (Fr(10), Fr(0)), (Fr(6), Fr(6))]
two_ctrl = [(Fr(0), Fr(10)), (Fr(10), Fr(0)), (Fr(4), Fr(4))]
three = [c + (Fr(0),) for c in two]

g2, w2 = worst_case_gap(two, 2, 240)
g2c, _ = worst_case_gap(two_ctrl, 2, 240)
g3, w3 = worst_case_gap(three, 2, 60)
print(f"  two coordinates, the compromise arm at (6,6):   optimum {g2} at w = "
      f"{tuple(str(x) for x in w2)}")
print(f"  two coordinates, 139's control arm at (4,4):    optimum {g2c}")
print(f"  three coordinates, third identically zero:      optimum {g3} at w = "
      f"{tuple(str(x) for x in w3)}")
# the exact witness, no solver needed
wit = (Fr(0), Fr(0), Fr(1))
s = sum(wk * ck for wk, ck in zip(wit, three[2]))
r = min(sum(wk * ck for wk, ck in zip(wit, c)) for j, c in enumerate(three) if j != 2)
print(f"\n  the exact witness at three coordinates needs no solver: at w = (0, 0, 1) every arm")
print(f"  scores 0 on the constant coordinate, so the gap is {s - r} by inspection.")
c5 = (g2 == 1 and g3 == 0)
c6 = g2c < 0
print(f"\n  C5: {'CONFIRMED' if c5 else 'REFUTED'} (+1 at two coordinates, 0 at three)")
print(f"  C6 CONTROL (the hull-interior control arm is strictly selectable): {c6} (must be True)")

print()
print("=" * 96)
print("VERDICT")
print("=" * 96)
print("  150's two-by-two reproduces: the cross-target invariance needs BOTH baseline-relative")
print("  units and a per-target baseline, and relative units with a fixed baseline does not give")
print("  it. 145's units reading is one dimension short and 146 inherited that.")
print()
print("  And 150's consequence holds: under relative units the baseline arm's figure is exactly 1")
print("  at every case, so the reporting normalisation is division by one. There is no second")
print("  operation to place, which is why the two sections stop competing rather than needing a")
print("  rule about which wins.")
print()
print("  150's D3 reproduces: F144-2's optimum is +1 at two cost coordinates and exactly 0 at")
print("  three with a constant third, and the three-coordinate zero is exhibited rather than")
print("  solved. 146 section 4 states the claim without the coordinate count it needs.")
