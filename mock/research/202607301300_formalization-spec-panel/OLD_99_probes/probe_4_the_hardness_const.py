# Probe 4: the transcendental hardness const exists, is computable by exhaustion,
# and has no visible closed form.
#
# Claim under test: for a transcendental elementary function over an arvo numeral,
# the working precision that decides correct rounding for EVERY operand is a
# well-defined constant of the type (the value set is finite, so the maximum over
# it exists), computable by exhaustion at model widths, and its value is an
# empirical fact rather than a formula of (P, F).
#
# Model: exp over the numeral P = F = 8 (operands k/256, k = 1..255; k = 0 is the
# removable exact point exp(0) = 1). Result read on the same 1/256 index grid.
# For each operand the distance of exp(x)*256 from the nearest rounding boundary
# (half-integer) is computed with the decimal module at 60 digits, which is
# orders of magnitude past the discrimination needed at this width; the hardness
# is the worst such distance over the whole value set.
#
# Sub-claims:
#   A: no distance is exactly zero (no ties), the model-width shadow of
#      Lindemann-Weierstrass (exp of a nonzero rational is transcendental,
#      hence never equal to the rational boundary).
#   B: the hardness const exists and is reported with its argmax operand.
#   C: repeated at a second width (P = F = 6) to show the const moves with the
#      type and follows no visible formula.

from decimal import Decimal, getcontext
import math

getcontext().prec = 60

def hardness(P, F):
    worst = (None, Decimal(1))
    ties = 0
    scale = Decimal(2) ** F
    for k in range(1, 2 ** P):
        x = Decimal(k) / (2 ** F)
        t = x.exp() * scale
        u = t % 1
        d = abs(u - Decimal("0.5"))
        if d == 0:
            ties += 1
        if d < worst[1]:
            worst = (k, d)
    k, d = worst
    bits = math.ceil(-math.log2(float(d)))
    return ties, k, float(d), bits

for (P, F) in [(8, 8), (6, 6), (8, 4)]:
    ties, k, d, bits = hardness(P, F)
    print(f"P={P} F={F}: ties={ties}  hardest operand k={k}  boundary distance={d:.3e}"
          f"  extra bits to decide rounding={bits}")
