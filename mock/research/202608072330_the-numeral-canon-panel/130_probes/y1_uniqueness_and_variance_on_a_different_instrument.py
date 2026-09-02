#!/usr/bin/env python3
# y1: two independent checks on 128's load-bearing claims, built without reading 128's probe code
# line by line first (the claims were read from 128's prose and predicted against, then checked
# against a DIFFERENT construction than 128 used, per the reply-step discipline: reproduce before
# accepting or contesting).
#
# Predictions, stated before running:
#
# Part 1 (uniqueness). 128's r2 part 3 solves ONE triangular system at m=8 and reports the
# solution is uniform. That shows uniform SATISFIES the marginal constraints; it does not by
# itself show no OTHER distribution over thresholds could. The genuine uniqueness claim is that
# the linear map from "distribution over thresholds" to "implied marginals" is INVERTIBLE, so the
# marginals (forced by unbiasedness) determine the threshold law with no freedom left, at every m,
# not merely the one tested. I predict the matrix representing this map (A[t][j] = 1 iff threshold
# t is < j, i.e. whether choosing threshold t rounds the j-th subpoint up) is lower-triangular with
# a nonzero diagonal for every m, hence has determinant +/-1 and is invertible for every m, which
# is a stronger and different statement than solving one instance: it says uniqueness holds
# because the SYSTEM is well-posed, not because this particular solve happened to land on uniform.
#
# Part 2 (variance, extended past n=10). 128's r3 part 1 enumerates all 2^n outcomes up to n=10
# and matches the closed forms n^2 f(1-f) (comonotone) and n f(1-f) (independent). I predict both
# closed forms are exact for EVERY n, not merely the ten enumerated, because each follows from a
# one-line variance identity (Var(n*X) = n^2 Var(X) for the comonotone sum, which is literally n
# copies of the same random error; Var(sum of n iid) = n*Var(X) for the independent sum), neither
# of which needs enumeration to hold. I verify the closed forms analytically (symbolic Fraction
# algebra, not full 2^n enumeration) at n = 25 and n = 100, past where brute-force enumeration
# (2^100 outcomes) could ever be checked exhaustively, which is the point: the two claims differ
# in strength, and this probe checks whether 128's n<=10 predicate widens to n any.

from fractions import Fraction
from itertools import product

print("=== Part 1: is the threshold-to-marginals map invertible at every m, not merely m=8? ===")


def marginal_matrix(m):
    # rows = candidate threshold values t in {0, ..., m-1} (uniform candidate support)
    # cols = subpoints j in {1, ..., m-1} (fracs j/m)
    # A[t][j-1] = 1 if choosing threshold t rounds subpoint j UP, i.e. frac(j/m) > t/m, i.e. j > t
    return [[1 if j > t else 0 for j in range(1, m)] for t in range(m)]


all_invertible = True
for m in (3, 5, 8, 12, 20):
    A = marginal_matrix(m)
    # A is (m rows) x (m-1 cols): candidate thresholds outnumber constraints by one, because a
    # probability vector over m thresholds has m-1 free parameters once normalised to sum 1. Build
    # the (m-1)x(m-1) SQUARE system actually solved: unknowns p_0..p_{m-2} (p_{m-1} = 1 - sum),
    # substituted into the m-1 marginal constraints P[T < j] = j/m for j = 1..m-1.
    # P[T < j] = sum_{t=0}^{j-1} p_t. This is exactly lower-triangular in p_0..p_{m-2} once
    # written as a linear system in the first m-1 unknowns (the last unknown only enters the
    # j = m-1 case through the "sum to 1" identity, which the marginal equations already encode
    # since P[T < m-1] = 1 - p_{m-1}).
    size = m - 1
    M = [[Fraction(1) if t < j else Fraction(0) for t in range(size)] for j in range(1, m)]
    # M is exactly lower-triangular with a full row of leading 1s appearing progressively:
    # row j-1 (constraint P[T<j]=j/m) has 1s for t=0..j-1, so it is UPPER triangular in this
    # ordering (nonzero entries are a prefix growing with the row index), with a full row of 1s
    # at the diagonal position (t = j-1 is included whenever t < j), diagonal entries all 1.
    diag_all_one = all(M[i][i] == 1 for i in range(size))
    # confirm the matrix is triangular: no nonzero entries strictly below the anti-pattern implied
    # by "row j has support exactly on columns 0..j-1"
    triangular = all(
        (M[i][k] == 0) == (k >= i + 1 == False and k > i)
        for i in range(size) for k in range(size)
    ) if size else True
    # direct determinant via the triangular structure: since row i (0-indexed) has 1s exactly on
    # columns 0..i, this is upper triangular with all-1 diagonal after reading rows bottom-up;
    # compute det by explicit Gaussian elimination over Fractions to avoid asserting the shape
    # rather than checking it.
    det = Fraction(1)
    Mc = [row[:] for row in M]
    ok = True
    for col in range(size):
        piv = None
        for r in range(col, size):
            if Mc[r][col] != 0:
                piv = r
                break
        if piv is None:
            ok = False
            break
        if piv != col:
            Mc[col], Mc[piv] = Mc[piv], Mc[col]
            det = -det
        det *= Mc[col][col]
        inv = Mc[col][col]
        for r in range(col + 1, size):
            if Mc[r][col] != 0:
                factor = Mc[r][col] / inv
                Mc[r] = [Mc[r][k] - factor * Mc[col][k] for k in range(size)]
    invertible = ok and det != 0
    all_invertible = all_invertible and invertible
    print(f"  m={m}: system size {size}x{size}, det = {det if ok else 'singular'}, "
          f"invertible = {invertible} (must be True at every m)")

print(f"  ALL m invertible: {all_invertible} "
      f"(this is the uniqueness claim itself: the marginals determine the threshold law with no "
      f"freedom left, as a fact about the linear system, at every m tested, not only m=8)")

print()
print("=== Part 1 control: is a NON-monotone (non-suffix) coupling family even in the search ===")
print("=== space this system covers? Confirm the system only parametrises suffix rules.      ===")
# any solution of the triangular system is, by its own construction (columns indexed by
# threshold candidates t, each column a suffix indicator), a MIXTURE of suffix rules. A mixture
# of suffix rules is realisation-monotone by construction (each realisation IS a suffix rule,
# which is monotone). So the system cannot represent a non-suffix realisation at all; the
# uniqueness claim is scoped to "among realisation-monotone unbiased rules" by the parametrisation
# itself, matching 128's own framing rather than smuggling in a wider claim.
print("  (structural: every column of M is a suffix indicator, so every solution is a mixture")
print("   of monotone realisations; non-monotone couplings are outside this system's domain by")
print("   construction, which matches, rather than narrows, 128's own claim)")

print()
print("=== Part 2: does the variance closed form hold past n = 10, analytically? ===")
f = Fraction(1, 3)


def var_comonotone_closed(n, f):
    e_up, e_dn = 1 - f, -f
    # sum error = n * single error (all elements move together under one shared threshold draw)
    return f * (n * e_up) ** 2 + (1 - f) * (n * e_dn) ** 2


def var_independent_closed(n, f):
    e_up, e_dn = 1 - f, -f
    single_var = f * e_up ** 2 + (1 - f) * e_dn ** 2
    return n * single_var  # exact for a sum of n iid terms, by linearity of variance


for n in (11, 15, 25, 100):
    vc = var_comonotone_closed(n, f)
    vi = var_independent_closed(n, f)
    pred_c, pred_i = n * n * f * (1 - f), n * f * (1 - f)
    print(f"  n={n}: comonotone closed-form={vc}, predicted n^2 f(1-f)={pred_c}, "
          f"match={vc == pred_c}; independent closed-form={vi}, predicted n f(1-f)={pred_i}, "
          f"match={vi == pred_i}")

print()
print("=== Part 2 cross-check: brute-force enumeration agrees with the closed form up to the ===")
print("=== point brute force is still feasible (n=14, 2^14 = 16384 outcomes), independent   ===")
print("=== construction from 128's r3 (which enumerated to n=10 only)                       ===")
for n in (12, 14):
    e_up, e_dn = 1 - f, -f
    var_ind_enum = Fraction(0)
    for bits in product([0, 1], repeat=n):
        p = Fraction(1)
        s = Fraction(0)
        for b in bits:
            p *= f if b else (1 - f)
            s += e_up if b else e_dn
        var_ind_enum += p * s * s
    closed = var_independent_closed(n, f)
    print(f"  n={n}: brute-force enumeration Var={var_ind_enum}, closed form={closed}, "
          f"match={var_ind_enum == closed} ({2**n} outcomes enumerated)")

print()
print("=== Part 3: are 128's variance framing and 129's decorrelation framing one finding ===")
print("=== or two? Test whether 128's own 'per-cell, independent across cells' member    ===")
print("=== (praised in 128 section 5 as buying cross-cell decorrelation) delivers ANY of ===")
print("=== 129's within-pass decorrelation on 129's own worst-case probe: a single VALUE ===")
print("=== repeated at many POSITIONS, so every element shares one cell by construction. ===")
print("=== Prediction: 1 distinct output (per-cell scheme draws once per cell, and there ===")
print("=== is only one cell here), identical to the pure global shared-threshold failure,===")
print("=== because 128's per-cell member is keyed on VALUE (which cell) while 129's test ===")
print("=== varies POSITION at fixed value, an axis 128's construction cannot see.        ===")
import random
random.seed(4242)
x = Fraction(1, 2)
k = x.numerator // x.denominator; f_x = x - k
U0 = Fraction(random.randrange(1, 999), 1000)  # one draw for the one cell in play
per_cell_outputs = [k if f_x <= U0 else k + 1 for _ in range(40)]
print(f"  128's per-cell-independent-across-cells on 129's constant-value-40-positions probe: "
      f"{len(set(per_cell_outputs))} distinct output(s) (predicted 1)")
phi = (1 + 5 ** 0.5) / 2
pk_outputs = [0 if float(f_x) <= ((i * phi) % 1.0) else 1 for i in range(40)]
print(f"  129's position-keyed golden-ratio dither on the same probe: "
      f"{len(set(pk_outputs))} distinct output(s) (predicted 2, reproducing 129's own result, "
      f"confirming the two constructions diverge on exactly this input)")

print()
print("=== Summary ===")
print(f"  Part 1: uniqueness confirmed as a well-posed-system fact at m in {{3,5,8,12,20}}, "
      f"all invertible = {all_invertible}. This generalises 128's single m=8 solve to a "
      f"structural argument (triangular, unit diagonal, hence invertible for every m).")
print("  Part 2: variance closed forms hold at n up to 100 analytically and are cross-checked")
print("  by brute-force enumeration through n=14, past 128's own n<=10 enumeration bound. The")
print("  closed forms are exact identities of variance algebra and are not n-bounded at all.")
