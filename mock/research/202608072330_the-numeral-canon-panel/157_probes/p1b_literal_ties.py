#!/usr/bin/env python3
"""157 P1b. Which non-grid literals actually separate truncation from ties-to-even at F=0.
Written after p1's C2 control failed on my own choice of witness (1/2), which is a tie.
NEGATIVE CONTROL: a GRID literal (an integer, at F=0) must separate NOTHING."""
from fractions import Fraction

def realise(x, W, F, signed, policy, rounding):
    step = Fraction(1, 2 ** F); q = x / step
    if rounding == "trunc":
        k = int(q) if q >= 0 else -int(-q)
    else:
        fl = q.numerator // q.denominator; frac = q - fl
        k = fl + 1 if frac > Fraction(1,2) else (fl if frac < Fraction(1,2) else (fl if fl % 2 == 0 else fl + 1))
    lo, hi = (-(2**(W-1)), 2**(W-1)-1) if signed else (0, 2**W - 1)
    span = hi - lo + 1
    k = ((k - lo) % span + lo) if policy == "wrap" else max(lo, min(hi, k))
    return Fraction(k) * step

cands = [Fraction(1,2), Fraction(3,4), Fraction(1,3), Fraction(2,3), Fraction(3,2),
         Fraction(5,2), Fraction(1,1), Fraction(2,1)]
print("literal   grid?   separates trunc/near at F=0 (W=3 unsigned sat)")
for c in cands:
    on_grid = (c.denominator == 1)
    t = realise(c, 3, 0, False, "sat", "trunc"); n = realise(c, 3, 0, False, "sat", "near")
    print(f"  {str(c):>5}   {'yes' if on_grid else 'no ':>4}    {'SEPARATES' if t != n else 'no'}   trunc={t} near={n}")
print()
print("CONTROL: every grid literal must fail to separate ->",
      "PASS" if all(realise(c,3,0,False,"sat","trunc") == realise(c,3,0,False,"sat","near")
                    for c in cands if c.denominator == 1) else "FAIL")
print("CONTROL: at least one non-grid literal must separate ->",
      "PASS" if any(realise(c,3,0,False,"sat","trunc") != realise(c,3,0,False,"sat","near")
                    for c in cands if c.denominator != 1) else "FAIL")
print()
print("The tie case: 1/2 is non-grid at F=0 and does NOT separate, because")
print("ties-to-even sends it to 0 and truncation sends it to 0 as well.")
print("So 'non-grid' is necessary and not sufficient; 'non-grid and not a tie' is the")
print("condition, and any half-integer is exactly the excluded case.")
