#!/usr/bin/env python3
# x1: two things. Part A reproduces 127's shared-threshold construction on my own instrument
# (independent code, same Q_U definition, before conceding or contesting). Part B asks the question
# my brief says only I am positioned to answer: dithering exists to decorrelate rounding error ACROSS
# POSITIONS WITHIN ONE EVALUATION PASS (that is the whole point: break up visible banding in one
# frame/array, not across repeated evaluations of it). 127's F127-1 measures monotonicity (global,
# one pass) and unbiasedness (over the draw, i.e. across MANY passes). Neither measures within-pass
# decorrelation, which is the property dithering is normally chosen for. This checks it directly.
#
# Prediction, stated before running:
#   A) Reproduces 127's numbers: shared-threshold monotone at every fixed U, additive-off-grid still
#      fails (T1/Finding 2 unaffected), unbiased in expectation.
#   B) A CONSTANT value rounded at many positions in ONE pass: shared-threshold gives the IDENTICAL
#      output at every position (zero decorrelation, because one U governs the whole pass). An
#      independent-per-element draw gives a MIX of outputs (decorrelation, at the cost of
#      monotonicity, already known). A position-keyed low-discrepancy sequence (golden-ratio ordered
#      dithering, a real deterministic dithering technique) ALSO gives a mix of outputs (decorrelation)
#      while being fully deterministic and reproducible given position, unlike independent random.
#      So shared-threshold is predicted to fail at the ONE property dithering exists for, while both
#      other constructions succeed at it by different mechanisms.

from fractions import Fraction

def q_floor(x):
    return x.numerator // x.denominator

def q_shared(x, U):
    k = q_floor(x)
    f = x - k
    return k if f <= U else k + 1

def q_position_keyed(x, i, golden=Fraction(2654435761, 4294967296)):
    # ordered dithering by a low-discrepancy per-position threshold: frac(i * golden_ratio_approx),
    # a standard deterministic dithering construction (equivalent in spirit to a Bayer/blue-noise
    # ordered-dither pattern; golden-ratio sequencing is the simplest 1D low-discrepancy choice).
    Ui = (i * golden) - q_floor(i * golden)
    return q_shared(x, Ui)

print("=== Part A: reproduce 127's shared-threshold construction, own instrument ===")
E = 5
pts = [Fraction(u, 1 << E) for u in range(-4 * (1 << E), 4 * (1 << E) + 1)]
Us = [Fraction(1, 5), Fraction(1, 2), Fraction(4, 5)]
mono_viol = 0
for U in Us:
    vals = [q_shared(x, U) for x in pts]
    v = sum(1 for i in range(len(vals) - 1) if vals[i] > vals[i + 1])
    mono_viol += v
    print(f"  U={U}: {v} monotonicity violations over {len(pts)} points (predicted 0)")
print(f"  total: {mono_viol} (predicted 0, reproducing 127 Part 1)")

# unbiasedness, exact, small set
for x in [Fraction(1, 3), Fraction(-7, 4), Fraction(5, 2)]:
    k = q_floor(x); f = x - k
    E_Q = k * (1 - f) + (k + 1) * f
    print(f"  E_U[Q_U({x})] = {E_Q}, matches x: {E_Q == x} (predicted True, reproducing 127 Part 2)")

# additivity control, small set, same shape as 127 Part 3
U = Fraction(1, 3)
viol = 0; total = 0
for a in pts[:60]:
    for b in pts[:4]:
        total += 1
        if q_shared(a + b, U) != q_shared(a, U) + q_shared(b, U):
            viol += 1
print(f"  additivity violations at U={U}: {viol} of {total} (predicted > 0, reproducing 127 Part 3 control)")

print()
print("=== Part B: within-pass decorrelation, a property neither 127 nor my own phase one measured ===")
print("=== A CONSTANT value x=0.5 (a maximal tie) rounded at 40 different positions in ONE pass. ===")
print("=== Dithering exists to break up exactly this case: a flat region should not band. ===")

x_const = Fraction(1, 2)
N = 40

# shared-threshold: one U for the whole pass
U_shared = Fraction(37, 100)  # arbitrary single draw, held fixed across the pass
shared_outputs = [q_shared(x_const, U_shared) for _ in range(N)]
distinct_shared = len(set(shared_outputs))
print(f"  shared-threshold (one U for the whole pass): {distinct_shared} distinct output value(s) "
      f"across {N} positions (predicted 1: every position gets the identical decision)")

# independent per-element draw (both files' original construction), using a fixed PRNG for reproducibility
import random
random.seed(42)
indep_outputs = [q_shared(x_const, Fraction(random.randrange(0, 1000), 1000)) for _ in range(N)]
distinct_indep = len(set(indep_outputs))
print(f"  independent-per-element draw: {distinct_indep} distinct output value(s) across {N} "
      f"positions (predicted 2: decorrelates, at the known cost of monotonicity)")

# position-keyed low-discrepancy (deterministic dithering)
posk_outputs = [q_position_keyed(x_const, i) for i in range(N)]
distinct_posk = len(set(posk_outputs))
print(f"  position-keyed golden-ratio ordered dither: {distinct_posk} distinct output value(s) "
      f"across {N} positions (predicted 2: decorrelates, deterministically, without a runtime PRNG)")

# does the position-keyed variant give a roughly even split at a tie, the property that makes
# dithering visually neutral (half up, half down, no directional bias at a maximal tie)?
n_up = sum(1 for v in posk_outputs if v == 1)
n_down = sum(1 for v in posk_outputs if v == 0)
print(f"  position-keyed split at the tie: {n_down} down, {n_up} up, of {N} "
      f"(near-even split is the point: no visible band, no directional bias)")

print()
print("=== Part B continued: is the position-keyed variant globally monotone the way shared-threshold is? ===")
# a smoothly increasing ramp, one value per position, values and positions coupled the way a real
# rounded sequence would be (unlike the constant-value probe above)
ramp = [Fraction(i, 7) for i in range(-20, 21)]
ramp_outputs = [q_position_keyed(x, i) for i, x in enumerate(ramp)]
ramp_viol = sum(1 for i in range(len(ramp_outputs) - 1) if ramp_outputs[i] > ramp_outputs[i + 1])
print(f"  position-keyed on an increasing ramp: {ramp_viol} monotonicity violations over "
      f"{len(ramp_outputs) - 1} consecutive pairs (predicted > 0: position-keying decorrelates "
      f"threshold from value order the same way independent draws do, just deterministically instead "
      f"of randomly; decorrelation and monotonicity are the same tension either way it is achieved)")

print()
print("=== Summary ===")
print("  Shared-threshold: monotone within a pass, unbiased across passes, ZERO within-pass")
print("  decorrelation (a flat region stays flat: one arbitrary deterministic mode per pass).")
print("  Independent and position-keyed: within-pass decorrelation (the property dithering exists")
print("  for), at the cost of monotonicity within the pass, for BOTH mechanisms alike.")
print("  So the real fork is not deterministic-vs-stochastic and it is not independent-vs-correlated.")
print("  It is WITHIN-PASS DECORRELATION versus WITHIN-PASS MONOTONICITY, and shared-threshold buys")
print("  monotonicity by giving up the one property that motivates reaching for a stochastic or")
print("  dithered rounding scheme in the first place.")
