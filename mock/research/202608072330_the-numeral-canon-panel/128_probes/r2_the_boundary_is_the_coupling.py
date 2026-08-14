#!/usr/bin/env python3
# r2 (128): where the boundary F8 misplaced actually sits. Exact Fractions throughout.
#
# The frame: an unbiased stochastic rounding has FORCED marginals (B(x) = [Q(x)=ceil] must be
# Bernoulli(frac(x)), else E[Q(x)] != x) and a FREE joint distribution (the coupling). Monotonicity
# on every realisation is a property of the coupling. The claims, predicted before running:
#   1. For a same-cell pair x < y with fracs f_x < f_y, the joint law has one free parameter
#      p11 = P[B_x=1, B_y=1] in the Frechet interval [max(0, f_x+f_y-1), min(f_x, f_y)], and the
#      inversion probability is exactly f_x - p11: zero at the comonotone corner p11 = f_x and
#      nowhere else. The independent coupling p11 = f_x*f_y sits strictly inside whenever
#      0 < f_x, f_y < 1, so it forces inversions. This is the impossibility half: unbiased +
#      per-realisation monotone + within-cell independence cannot coexist.
#   2. For a cross-cell pair, inversion probability is 0 at EVERY point of the Frechet interval:
#      the monotonicity constraint binds within cells only, so the coupling across cells is free.
#   3. Uniqueness within a cell: on a discrete cell with m subpoints, realisation-monotonicity
#      forces every realisation's round-up set to be a suffix (an upper set), suffixes are indexed
#      by a threshold, and the forced marginals then determine the threshold distribution uniquely:
#      uniform. So the shared-uniform-threshold rule is THE realisation-monotone unbiased rounding
#      within a cell, and the whole remaining design space is the coupling of thresholds across
#      cells. Controls: a non-suffix up-set family is non-monotone on its own realisation; a
#      non-uniform threshold distribution breaks a marginal.
from fractions import Fraction

print("--- 1. same-cell pair: inversion probability across the whole Frechet interval ---")
pairs = [(Fraction(1, 5), Fraction(4, 5)), (Fraction(1, 3), Fraction(1, 2)),
         (Fraction(3, 7), Fraction(6, 7)), (Fraction(9, 10), Fraction(19, 20))]
for fx, fy in pairs:
    lo = max(Fraction(0), fx + fy - 1)
    hi = min(fx, fy)
    steps = 12
    zero_points = []
    for i in range(steps + 1):
        p11 = lo + (hi - lo) * Fraction(i, steps)
        p_inv = fx - p11  # P[B_x=1, B_y=0]
        assert p_inv >= 0
        if p_inv == 0:
            zero_points.append(p11)
    p11_ind = fx * fy
    inv_ind = fx - p11_ind
    print(f"f_x={fx}, f_y={fy}: inversion=0 at {len(zero_points)} of {steps+1} sweep points, "
          f"at p11={zero_points} (must be exactly [min(f_x,f_y)]={[hi]}); "
          f"independent coupling inversion={inv_ind} (must be > 0)")

print("--- 2. cross-cell pair: inversion impossible at every coupling ---")
# x in cell k, y in cell k+1, x < y. Q(x) <= k+1 <= Q(y) always: inversion prob is 0 regardless
# of the joint law. Enumerate all four joint outcomes at every Frechet point and count inversions.
fx, fy = Fraction(2, 3), Fraction(1, 5)  # fracs need not be ordered across cells
kx, ky = 0, 1
lo = max(Fraction(0), fx + fy - 1); hi = min(fx, fy)
bad = 0
for i in range(13):
    p11 = lo + (hi - lo) * Fraction(i, 12)
    joint = {(1, 1): p11, (1, 0): fx - p11, (0, 1): fy - p11, (0, 0): 1 - fx - fy + p11}
    for (bx, by), p in joint.items():
        if p > 0 and (kx + bx) > (ky + by):
            bad += 1
print(f"cross-cell (cells {kx},{ky}): {bad} positive-probability inversions across 13 Frechet "
      f"points x 4 outcomes (must be 0)")

print("--- 3. uniqueness within a cell: suffix + forced marginals => uniform threshold ---")
m = 8  # subpoints j = 1..m-1 at fracs j/m, plus the grid point at j=0
# realisation-monotone => the round-up set is a suffix {j : j > t}, t in {0..m-1}.
# marginals: P[j rounded up] = j/m ... wait: unbiasedness at frac f needs P[up] = f. For frac j/m
# the requirement is P[T < j] = j/m. Solve the triangular system for P[T = t].
probs = []
prev = Fraction(0)
for j in range(1, m):
    cum = Fraction(j, m)          # required P[T < j]
    probs.append(cum - prev)      # P[T = j-1]
    prev = cum
probs.append(Fraction(1) - prev)  # P[T = m-1]
uniform = all(p == Fraction(1, m) for p in probs)
print(f"m={m}: solved threshold distribution = {probs[0]} each x {len(probs)} "
      f"(uniform: {uniform}, must be True)")

# control A: a non-suffix up-set family (round up only the middle point) is non-monotone on its
# own realisation, whatever its probability weights are.
up_set = {m // 2}  # round up only j = m/2
vals = [(j // m) + (1 if j in up_set else 0) for j in range(0, m)]
inv = sum(1 for i in range(len(vals) - 1) if vals[i] > vals[i + 1])
print(f"control A: non-suffix up-set {{{m//2}}} realisation has {inv} inversions (must be > 0)")

# control B: a non-uniform threshold distribution breaks a marginal.
bad_probs = [Fraction(1, m)] * m
bad_probs[0] += Fraction(1, 2 * m); bad_probs[1] -= Fraction(1, 2 * m)
j = 1
got = sum(bad_probs[t] for t in range(j))  # P[T < 1]
print(f"control B: perturbed distribution gives P[up at frac {j}/{m}] = {got}, "
      f"required {Fraction(j, m)} (must differ)")
