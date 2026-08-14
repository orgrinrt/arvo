#!/usr/bin/env python3
# s2 (133): two sentences of the candidate checked against the sources they compress.
#
# Predictions, stated before running:
#   A. Candidate 5.2 says "every deterministic member is order-preserving." Unscoped, that is
#      false: a deterministic grid-fixing retraction need not be monotone, and 125_probes/p1's
#      parity control is a committed counterexample. The exact characterisation, both directions:
#      a deterministic quantisation is monotone IFF it is a per-cell threshold rule (its round-up
#      set within each cell is a suffix). Brute force over ALL retraction restrictions on a
#      discrete cell at m in {3, 4, 5} subpoints per quantum: monotone count = suffix count = m,
#      out of 2^(m-1) retractions per cell.
#   B. Candidate 5.3 says "no member carries more than one of the first three [exact laws]."
#      False against 125 F3: floor carries the order bound AND exact staged composition;
#      toward_zero carries exact composition AND negation symmetry; half_even carries negation
#      symmetry. The one true exclusivity: no member carries the order bound and negation
#      symmetry at once, because negation swaps the adjoints. Verified by direct check with
#      controls (half_up fails composition and negation; floor fails negation).
from fractions import Fraction
from itertools import product

def q_floor(x): return x.numerator // x.denominator
def q_ceil(x): return -((-x).numerator // (-x).denominator)
def q_tz(x): return q_floor(x) if x >= 0 else q_ceil(x)
def q_half_up(x): return q_floor(x + Fraction(1, 2))
def q_half_even(x):
    k = q_floor(x); r = x - k
    if r < Fraction(1, 2): return k
    if r > Fraction(1, 2): return k + 1
    return k if k % 2 == 0 else k + 1

print("--- A. monotone deterministic retraction == per-cell threshold rule, exhaustively ---")
for m in (3, 4, 5):
    # a retraction restricted to one cell: each of the m-1 off-grid subpoints goes to k or k+1.
    # monotone over the cell (with the grid endpoints fixed at k and k+1) iff the round-up set
    # is a suffix of the subpoints.
    total = mono = suffix = 0
    for bits in product((0, 1), repeat=m - 1):
        total += 1
        vals = [0] + [b for b in bits] + [1]  # cell endpoints fixed: k -> 0-offset, k+1 -> 1
        is_mono = all(vals[i] <= vals[i + 1] for i in range(len(vals) - 1))
        is_suffix = all(bits[i] <= bits[i + 1] for i in range(len(bits) - 1))
        if is_mono: mono += 1
        if is_suffix: suffix += 1
        assert is_mono == is_suffix  # the characterisation, both directions, per retraction
    print(f"m={m}: {total} retractions per cell, {mono} monotone, {suffix} suffix rules "
          f"(equal={mono == suffix}, count must be m={m}: {mono == m})")
# the committed counterexample class: parity keying is not a suffix, hence not monotone
bits_parity = tuple((i % 2) for i in range(1, 4))  # m=4: subpoints 1..3, round up odd ones
vals = [0] + list(bits_parity) + [1]
print(f"parity control at m=4: round-up set {bits_parity} monotone="
      f"{all(vals[i] <= vals[i+1] for i in range(len(vals)-1))} (must be False)")

print("--- B. the law overlap, checked per member ---")
E = 4
pts = [Fraction(u, 1 << E) for u in range(-2000, 2001)]
grid = range(-130, 131)

def adjunction_ok(f, right):  # right adjoint: g <= x <=> g <= f(x); left: x <= g <=> f(x) <= g
    for x in pts[::7]:
        for g in list(grid)[::11]:
            if right and (g <= x) != (g <= f(x)): return False
            if not right and (x <= g) != (f(x) <= g): return False
    return True

def staged_ok(f):  # fine -> quarters -> integers equals fine -> integers
    return all(f(Fraction(f(x * 4), 4)) == f(x) for x in pts)

def negation_ok(f):
    return all(f(-x) == -f(x) for x in pts)

members = [("floor", q_floor), ("ceil", q_ceil), ("toward_zero", q_tz),
           ("half_up", q_half_up), ("half_even", q_half_even)]
print(f"{'member':<12} {'order bound':<12} {'staged comp':<12} {'negation sym':<12}")
rows = {}
for name, f in members:
    ob = adjunction_ok(f, right=(name == "floor")) if name in ("floor", "ceil") else False
    sc = staged_ok(f)
    ns = negation_ok(f)
    rows[name] = (ob, sc, ns)
    print(f"{name:<12} {str(ob):<12} {str(sc):<12} {str(ns):<12}")

print()
print("checks against candidate 5.3:")
print(f"floor carries order bound AND composition: {rows['floor'][0] and rows['floor'][1]} "
      f"(must be True: refutes 'no member carries more than one')")
print(f"toward_zero carries composition AND negation: {rows['toward_zero'][1] and rows['toward_zero'][2]} "
      f"(must be True: second refutation)")
print(f"half_even carries negation symmetry: {rows['half_even'][2]} "
      f"(must be True: 5.3 attributes it to toward_zero alone)")
print(f"controls: half_up composition {rows['half_up'][1]} (must be False), "
      f"half_up negation {rows['half_up'][2]} (must be False), "
      f"floor negation {rows['floor'][2]} (must be False)")
excl = not any((n in ('floor', 'ceil')) and rows[n][2] for n in rows)
print(f"the true exclusivity, order bound never with negation symmetry: {excl} (must be True)")
