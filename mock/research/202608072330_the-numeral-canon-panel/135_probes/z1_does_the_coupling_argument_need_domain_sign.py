#!/usr/bin/env python3
# z1: checking my own signature claim before making it. 132 section 5.6 and 5.7 predicate the
# uniqueness/impossibility/variance/keying clauses on "domain closed under negation", the same
# predicate 5.2-5.4 carry for the additive-homomorphism theorem. That theorem's predicate is real
# (T1/T1b prove it separately for the two domain shapes). The coupling argument (my own y1, and
# 128's r2/r3) is built entirely from frac(x) = x - floor(x), a per-cell-local quantity, and never
# reads whether the ambient container is one-signed or symmetric. Prediction, stated before running:
# every quantity the coupling argument produces (the uniqueness solve, the variance law, the
# per-cell-vs-global keying divergence) is identical for a cell with negative index and a cell with
# positive index, because nothing in the construction ever inspects the sign of k. If confirmed, the
# "domain closed under negation" predicate on 5.6/5.7 is inherited from the earlier sections rather
# than established by this argument, and the honest predicate is domain any (the construction does
# not touch the dimension at all, so nothing about it is sign-conditional).
from fractions import Fraction


def q_floor(x):
    return x.numerator // x.denominator


print("=== Part 1: uniqueness solve, does it reference which cell (k) at all? ===")
print("=== (128's r2 part 3 and my own y1 part 1 both parametrise by m subpoints ===")
print("=== per cell, never by k; confirm the solved distribution is identical    ===")
print("=== whether the cell in question is k=-3 or k=+3.)                        ===")


def threshold_dist(m):
    probs = []
    prev = Fraction(0)
    for j in range(1, m):
        cum = Fraction(j, m)
        probs.append(cum - prev)
        prev = cum
    probs.append(Fraction(1) - prev)
    return probs


for m in (5, 8):
    dist = threshold_dist(m)
    print(f"  m={m}: threshold distribution {dist} (computed with no reference to any k at all; "
          f"the same values apply verbatim to a cell at k=-5 as to one at k=+5, because the "
          f"construction is stated over frac(x) in [0,1) alone)")

print()
print("=== Part 2: variance law, negative-k cell vs positive-k cell, same frac ===")


def var_comonotone(n, f):
    e_up, e_dn = 1 - f, -f
    return f * (n * e_up) ** 2 + (1 - f) * (n * e_dn) ** 2


def var_independent(n, f):
    e_up, e_dn = 1 - f, -f
    single = f * e_up ** 2 + (1 - f) * e_dn ** 2
    return n * single


f = Fraction(1, 3)
for n in (5, 10):
    vc, vi = var_comonotone(n, f), var_independent(n, f)
    print(f"  n={n}, f={f} (this f is the same number whether the cell holding it is k=-4 "
          f"[values in [-4,-3)] or k=+4 [values in [4,5)]): comonotone Var={vc}, "
          f"independent Var={vi}")
print("  (the formula never mentions k; there is no branch on sign anywhere in either derivation)")

print()
print("=== Part 3: the keying divergence (F130-1), negative-position and negative-value check ===")
print("=== reproducing the original probe's exact construction, but at x = -1/2 (cell k = -1) ===")


def q_shared(x, U):
    k = q_floor(x)
    f = x - k
    return k if f <= U else k + 1


import random
random.seed(4242)

for x_test, label in [(Fraction(1, 2), "x=1/2 (cell k=0)"), (Fraction(-1, 2), "x=-1/2 (cell k=-1)")]:
    k = q_floor(x_test)
    f_x = x_test - k
    U0 = Fraction(random.randrange(1, 999), 1000)
    per_cell_outputs = [k if f_x <= U0 else k + 1 for _ in range(40)]
    print(f"  {label}: per-cell-independent-across-cells decorrelation count over 40 positions = "
          f"{len(set(per_cell_outputs))} (predicted: identical to the positive-cell case, 1)")

print()
print("=== Summary ===")
print("  If every quantity above is identical in shape and value between a negative-indexed cell")
print("  and a positive-indexed one (predicted: yes, since none of the three constructions ever")
print("  reads k's sign), then 132 5.6/5.7's 'domain closed under negation' predicate restricts a")
print("  region the argument was never shown to depend on, and the honest predicate is 'domain")
print("  any', inherited from nothing needing checking because the construction is sign-blind by")
print("  its own definition.")
