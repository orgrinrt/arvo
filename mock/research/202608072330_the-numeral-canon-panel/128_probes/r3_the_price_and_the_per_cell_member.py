#!/usr/bin/env python3
# r3 (128): the price 127 did not measure, and the family member neither 126, 127 nor I named.
# Exact Fractions; distributions enumerated in full, no sampling.
#
# Predictions, stated before running:
#   1. Accumulation error variance for n same-cell elements at frac f: comonotone (shared U)
#      gives exactly n^2 f(1-f); independent gives exactly n f(1-f). Enumerated for n up to 10.
#      Control: at n = 1 the two coincide (the distinction needs at least a pair).
#   2. The PER-CELL threshold member (independent U_k per grid cell): monotone on every
#      realisation over a multi-cell window, for every threshold tuple in a sweep; exactly
#      unbiased; cross-cell error covariance exactly 0; within-cell covariance positive (the
#      irreducible coherence realisation-monotonicity forces).
#   3. The price of that freedom: a per-cell realisation with unequal thresholds is NOT
#      q-translation-equivariant, and fails wrap commutation mod span at some points. So the
#      family trades along: global U keeps wrap compatibility and pays full error coherence;
#      per-cell U buys cross-cell decorrelation and pays the translation structure.
from fractions import Fraction
from itertools import product

def q_floor(x): return x.numerator // x.denominator

print("--- 1. variance of the summed error, n same-cell elements at frac f ---")
f = Fraction(1, 3)
for n in (1, 2, 4, 8, 10):
    # comonotone: one shared U; error of each element = (1-f)*[U<f] ... value error e_i =
    # Q(x)-x = (1-f) if rounded up (prob f), else -f (prob 1-f). Shared U: all together.
    e_up, e_dn = (1 - f), -f
    # E[sum e] = n*(f*e_up + (1-f)*e_dn) = 0
    var_com = f * (n * e_up) ** 2 + (1 - f) * (n * e_dn) ** 2
    # independent: enumerate all 2^n outcomes exactly
    var_ind = Fraction(0)
    for bits in product([0, 1], repeat=n):
        p = Fraction(1)
        s = Fraction(0)
        for b in bits:
            p *= f if b else (1 - f)
            s += e_up if b else e_dn
        var_ind += p * s * s
    pred_com, pred_ind = n * n * f * (1 - f), n * f * (1 - f)
    print(f"n={n}: comonotone Var={var_com} (formula n^2 f(1-f)={pred_com}, "
          f"match={var_com == pred_com}); independent Var={var_ind} "
          f"(formula n f(1-f)={pred_ind}, match={var_ind == pred_ind})"
          + ("  <- control: identical at n=1" if n == 1 else ""))

print("--- 2. the per-cell threshold member on a multi-cell window ---")
E = 4
cells = range(-2, 3)  # five cells
window = [Fraction(u, 1 << E) for u in range(-2 * (1 << E), 3 * (1 << E))]
def q_percell(x, thresholds):  # thresholds: dict cell -> U_k
    k = q_floor(x); frc = x - k
    return k if frc <= thresholds[k] else k + 1
sweep = [Fraction(1, 5), Fraction(1, 2), Fraction(9, 10)]
tuples_checked = 0; mono_bad = 0
for tup in product(sweep, repeat=len(list(cells))):
    th = dict(zip(cells, tup))
    vals = [q_percell(x, th) for x in window]
    mono_bad += sum(1 for i in range(len(vals) - 1) if vals[i] > vals[i + 1])
    tuples_checked += 1
print(f"monotonicity: {mono_bad} violations across {tuples_checked} threshold tuples x "
      f"{len(window)-1} adjacent pairs (must be 0)")

# unbiasedness and covariances, exactly, for two same-cell and two cross-cell points, with
# independent uniform U_k per cell: enumerate by interval decomposition of (U_a, U_b) plane.
xa, xb = Fraction(5, 16), Fraction(11, 16)        # same cell 0: fracs 5/16, 11/16
yc = Fraction(1) + Fraction(7, 16)                # cell 1: frac 7/16
fa, fb, fc = Fraction(5, 16), Fraction(11, 16), Fraction(7, 16)
ea_up, ea_dn = 1 - fa, -fa
eb_up, eb_dn = 1 - fb, -fb
ec_up, ec_dn = 1 - fc, -fc
# same cell: shared U_0. E[e_a e_b] over U_0: up iff U_0 < f.
# regions of U_0: [0, fa): both up; [fa, fb): a down, b up; [fb, 1): both down.
cov_same = (fa * ea_up * eb_up + (fb - fa) * ea_dn * eb_up + (1 - fb) * ea_dn * eb_dn)
# means are zero (unbiased), so covariance = E[e_a e_b]
# cross cell: independent U_0, U_1: E[e_a e_c] = E[e_a] E[e_c] = 0
mean_a = fa * ea_up + (1 - fa) * ea_dn
mean_c = fc * ec_up + (1 - fc) * ec_dn
cov_cross = mean_a * mean_c
print(f"unbiased: E[e_a]={mean_a}, E[e_c]={mean_c} (must be 0, 0)")
print(f"within-cell covariance E[e_a e_b] = {cov_same} (must be > 0: forced coherence)")
print(f"cross-cell covariance = {cov_cross} (must be 0: the freedom the per-cell member buys)")

print("--- 3. the price: translation equivariance and wrap compatibility are lost ---")
th = {k: (Fraction(1, 5) if k % 2 == 0 else Fraction(9, 10)) for k in range(-17, 18)}
x = Fraction(1, 2)  # frac 1/2: cell 0 threshold 1/5 -> up; cell 1 threshold 9/10 -> down
lhs = q_percell(x + 1, th)
rhs = q_percell(x, th) + 1
print(f"translation: Q(x+q)={lhs}, Q(x)+q={rhs} (must differ for this tuple)")
W = 3; span = 1 << W; lo_r = 0
def wrap_exact(v): return v - span * ((v - lo_r) // span)
def wrap_grid(k): return (k - lo_r) % span + lo_r
wrap_window = [Fraction(u, 1 << E) for u in range(-span * (1 << E), 2 * span * (1 << E))]
qm = sum(1 for v in wrap_window
         if (wrap_grid(q_percell(v, th)) - q_percell(wrap_exact(v), th)) % span != 0)
print(f"wrap mod-span mismatches for the same tuple: {qm} of {len(wrap_window)} (must be > 0)")

# --- follow-up after run 1 refuted prediction 3's wrap half: the tuple used alternated by cell
# PARITY, and the span (8) is even, so the assignment was span-periodic and commuted by
# construction. The sharper statement, predicted before this second run: a per-cell threshold
# assignment commutes with wrap mod span IFF it is constant on residue classes mod span. The
# span-periodic sub-member therefore keeps wrap compatibility (0 mismatches, run 1's observed
# zero reclassified as this case), and an APERIODIC assignment, differing within one residue
# class, must fail (> 0 mismatches).
print("--- 3b. wrap commutation is exactly span-periodicity of the threshold table ---")
th_aperiodic = dict(th)
th_aperiodic[8] = Fraction(9, 10)   # cell 8 is residue 0 mod 8; cell 0 keeps 1/5: class broken
qm2 = sum(1 for v in wrap_window
          if (wrap_grid(q_percell(v, th_aperiodic)) - q_percell(wrap_exact(v), th_aperiodic))
          % span != 0)
print(f"aperiodic table (th[0]=1/5, th[8]=9/10, same residue class): {qm2} mod-span mismatches "
      f"of {len(wrap_window)} (must be > 0)")
th_periodic = {k: th[((k % span) + span) % span] for k in range(-17, 18)}
qm3 = sum(1 for v in wrap_window
          if (wrap_grid(q_percell(v, th_periodic)) - q_percell(wrap_exact(v), th_periodic))
          % span != 0)
print(f"residue-keyed table (constant on classes mod span): {qm3} mismatches (must be 0)")
